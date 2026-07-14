mod gpu_preview;

use std::{path::PathBuf, time::Duration};

use eframe::egui;
use finui_media_surface::{
    MediaSurfaceAction, MediaSurfaceInteractionState, MediaSurfaceSnapshot, MediaSurfaceUxReceipt,
    MediaTransform, MediaTransformKind, show_media_surface,
};
use finui_primitives::{PrimitiveTheme, ThemeMode};
use finui_timeline::{
    TimelineClip, TimelineGeometryCache, TimelineInteractionState, TimelineSnapshot, TimelineTrack,
    TimelineUxReceipt, TimelineViewport, show_timeline,
};
use finui_workbench::{
    CommandScopeOutput, PaneConstraints, PanelId, PanelTab, SplitAxis,
    WORKBENCH_LAYOUT_SCHEMA_VERSION, WorkbenchNode, WorkbenchState, show_workbench,
};
use gpu_preview::{GpuPreview, RecreateReason};
use serde_json::json;

const MEDIA_PANEL: &str = "media";
const PREVIEW_PANEL: &str = "preview";
const INSPECTOR_PANEL: &str = "inspector";
const TIMELINE_PANEL: &str = "timeline";
const AGENT_PANEL: &str = "agent";

fn main() -> eframe::Result {
    let smoke = SmokeConfig::from_env();
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([960.0, 600.0])
            .with_visible(smoke.is_none()),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Finui Editor Lab",
        options,
        Box::new(move |cc| Ok(Box::new(EditorLabApp::new(cc, smoke)))),
    )
}

struct EditorLabApp {
    preview: Option<GpuPreview>,
    startup_error: Option<String>,
    workbench: WorkbenchState,
    timeline_cache: TimelineGeometryCache,
    timeline_viewport: TimelineViewport,
    timeline_interaction: TimelineInteractionState,
    timeline_playhead_tick: i64,
    media_transform: MediaTransform,
    media_interaction: MediaSurfaceInteractionState,
    last_timeline_receipt: Option<TimelineUxReceipt>,
    last_media_receipt: Option<MediaSurfaceUxReceipt>,
    persistence_status: String,
    last_divider_count: usize,
    last_region_count: usize,
    last_panel_tab_count: usize,
    last_rendered_panel_count: usize,
    last_command_scope: CommandScopeOutput,
    smoke: Option<SmokeConfig>,
    app_frame: u64,
    smoke_finished: bool,
}

impl EditorLabApp {
    fn new(cc: &eframe::CreationContext<'_>, smoke: Option<SmokeConfig>) -> Self {
        let (preview, startup_error) = match cc.wgpu_render_state.as_ref() {
            Some(render_state) => (Some(GpuPreview::new(render_state.clone())), None),
            None => (
                None,
                Some("editor_lab requires eframe's wgpu renderer".to_owned()),
            ),
        };
        let workbench = default_editor_workbench();
        let timeline_fixture = default_timeline_fixture();
        let timeline_cache = TimelineGeometryCache::build(&timeline_fixture);
        let last_command_scope = workbench.command_scope();

        Self {
            preview,
            startup_error,
            workbench,
            timeline_cache,
            timeline_viewport: TimelineViewport::new(0, 2.0),
            timeline_interaction: TimelineInteractionState::default(),
            timeline_playhead_tick: 240,
            media_transform: MediaTransform::default(),
            media_interaction: MediaSurfaceInteractionState::default(),
            last_timeline_receipt: None,
            last_media_receipt: None,
            persistence_status: "layout schema v1".to_owned(),
            last_divider_count: 0,
            last_region_count: 0,
            last_panel_tab_count: 0,
            last_rendered_panel_count: 0,
            last_command_scope,
            smoke,
            app_frame: 0,
            smoke_finished: false,
        }
    }

    fn round_trip_layout(&mut self) {
        let result = self
            .workbench
            .to_json_pretty()
            .map_err(|error| error.to_string())
            .and_then(|json| WorkbenchState::from_json(&json).map_err(|error| error.to_string()));
        match result {
            Ok(restored) => {
                self.workbench = restored;
                self.persistence_status = "layout JSON round-trip: PASS".to_owned();
            }
            Err(error) => self.persistence_status = format!("layout restore failed: {error}"),
        }
    }

    fn finish_smoke_if_ready(&mut self, ctx: &egui::Context) {
        let Some(smoke) = self.smoke.as_ref() else {
            return;
        };
        if self.smoke_finished || self.app_frame < smoke.frames {
            return;
        }

        let workbench_receipt = json!({
            "active_panels_rendered": self.last_rendered_panel_count,
            "command_scope": command_scope_label(&self.last_command_scope),
            "divider_count": self.last_divider_count,
            "focused_panel": self.workbench.focused_panel.as_ref().map(PanelId::as_str),
            "panel_tab_count": self.last_panel_tab_count,
            "region_count": self.last_region_count,
            "schema_version": WORKBENCH_LAYOUT_SCHEMA_VERSION,
            "timeline": self.last_timeline_receipt.as_ref(),
            "media_surface": self.last_media_receipt.as_ref(),
        });
        let result = self
            .preview
            .as_ref()
            .ok_or_else(|| {
                self.startup_error
                    .clone()
                    .unwrap_or_else(|| "GPU preview did not start".to_owned())
            })
            .and_then(|preview| {
                preview.write_smoke_receipt(&smoke.receipt_path, workbench_receipt)
            });

        if let Err(error) = result {
            eprintln!("editor_lab smoke receipt failed: {error}");
        }
        self.smoke_finished = true;
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
}

impl eframe::App for EditorLabApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.app_frame += 1;
        let mode = if ui.visuals().dark_mode {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        let theme = PrimitiveTheme::for_mode(mode);

        let mut recreate_requested = false;
        let mut round_trip_requested = false;
        let mut reset_requested = false;
        ui.horizontal(|ui| {
            ui.heading("Finui Editor Lab");
            ui.separator();
            ui.label("wgpu + caller-owned workbench");
            if ui.small_button("Recreate texture").clicked() {
                recreate_requested = true;
            }
            if ui.small_button("Round-trip layout").clicked() {
                round_trip_requested = true;
            }
            if ui.small_button("Reset layout").clicked() {
                reset_requested = true;
            }
            ui.separator();
            ui.small(command_scope_label(&self.workbench.command_scope()));
            ui.small(&self.persistence_status);
        });
        ui.separator();

        if round_trip_requested {
            self.round_trip_layout();
        }
        if reset_requested {
            self.workbench = default_editor_workbench();
            self.persistence_status = "layout reset".to_owned();
        }
        if let Some(preview) = self.preview.as_mut() {
            if recreate_requested {
                preview.force_recreate(RecreateReason::Manual);
            }
            if self
                .smoke
                .as_ref()
                .is_some_and(|smoke| self.app_frame == smoke.recreate_frame)
            {
                preview.force_recreate(RecreateReason::DeviceRecoverySimulation);
            }
        }

        let elapsed_seconds = ui.input(|input| input.time) as f32;
        let pixels_per_point = ui.ctx().pixels_per_point();
        let (output, rendered_panel_count) = {
            let mut runtime = EditorPanelRuntime {
                preview: &mut self.preview,
                startup_error: self.startup_error.as_deref(),
                theme,
                elapsed_seconds,
                pixels_per_point,
                timeline_cache: &self.timeline_cache,
                timeline_viewport: self.timeline_viewport,
                timeline_interaction: &mut self.timeline_interaction,
                timeline_playhead_tick: self.timeline_playhead_tick,
                media_transform: &mut self.media_transform,
                media_interaction: &mut self.media_interaction,
                last_timeline_receipt: &mut self.last_timeline_receipt,
                last_media_receipt: &mut self.last_media_receipt,
                rendered_panel_count: 0,
            };
            let output = show_workbench(ui, &self.workbench, |panel_ui, panel_id| {
                runtime.show(panel_ui, panel_id);
            });
            (output, runtime.rendered_panel_count)
        };

        match output {
            Ok(output) => {
                self.workbench.apply_all(&output.actions);
                self.last_divider_count = output.geometry.dividers.len();
                self.last_region_count = output.geometry.tab_regions.len();
                self.last_panel_tab_count = output
                    .geometry
                    .tab_regions
                    .iter()
                    .map(|region| region.tabs.len())
                    .sum();
                self.last_rendered_panel_count = rendered_panel_count;
                self.last_command_scope = output.command_scope;
            }
            Err(error) => {
                ui.colored_label(theme.text, format!("Invalid workbench layout: {error}"));
            }
        }

        ui.ctx().request_repaint_after(Duration::from_millis(16));
        self.finish_smoke_if_ready(ui.ctx());
    }
}

fn default_editor_workbench() -> WorkbenchState {
    let media = WorkbenchNode::tabs(
        "media-region",
        vec![PanelTab::new(MEDIA_PANEL, "Media")],
        MEDIA_PANEL,
    );
    let preview = WorkbenchNode::tabs(
        "preview-region",
        vec![PanelTab::new(PREVIEW_PANEL, "Preview")],
        PREVIEW_PANEL,
    );
    let inspector_agent = WorkbenchNode::tabs(
        "detail-region",
        vec![
            PanelTab::new(INSPECTOR_PANEL, "Inspector"),
            PanelTab::new(AGENT_PANEL, "Agent"),
        ],
        INSPECTOR_PANEL,
    );
    let timeline = WorkbenchNode::tabs(
        "timeline-region",
        vec![PanelTab::new(TIMELINE_PANEL, "Timeline")],
        TIMELINE_PANEL,
    );
    let preview_and_detail = WorkbenchNode::split(
        "preview-detail-split",
        SplitAxis::Horizontal,
        0.72,
        PaneConstraints::minimum(360.0),
        PaneConstraints::bounded(220.0, 420.0),
        preview,
        inspector_agent,
    );
    let main_row = WorkbenchNode::split(
        "media-main-split",
        SplitAxis::Horizontal,
        0.20,
        PaneConstraints::bounded(180.0, 320.0),
        PaneConstraints::minimum(580.0),
        media,
        preview_and_detail,
    );
    let root = WorkbenchNode::split(
        "main-timeline-split",
        SplitAxis::Vertical,
        0.68,
        PaneConstraints::minimum(300.0),
        PaneConstraints::bounded(160.0, 360.0),
        main_row,
        timeline,
    );
    WorkbenchState::new(root).with_focused_panel(PREVIEW_PANEL)
}

fn default_timeline_fixture() -> TimelineSnapshot {
    TimelineSnapshot {
        revision: 1,
        tracks: (0..100)
            .map(|track_index| {
                TimelineTrack::new(
                    format!("track-{track_index}"),
                    format!("Track {track_index:03}"),
                    (0..100)
                        .map(|clip_index| {
                            let mut clip = TimelineClip::new(
                                format!("clip-{track_index}-{clip_index}"),
                                format!("C{clip_index:02}"),
                                clip_index as i64 * 120,
                                100,
                            );
                            clip.selected = track_index == 0 && clip_index == 0;
                            clip
                        })
                        .collect(),
                )
            })
            .collect(),
    }
}

struct EditorPanelRuntime<'a> {
    preview: &'a mut Option<GpuPreview>,
    startup_error: Option<&'a str>,
    theme: PrimitiveTheme,
    elapsed_seconds: f32,
    pixels_per_point: f32,
    timeline_cache: &'a TimelineGeometryCache,
    timeline_viewport: TimelineViewport,
    timeline_interaction: &'a mut TimelineInteractionState,
    timeline_playhead_tick: i64,
    media_transform: &'a mut MediaTransform,
    media_interaction: &'a mut MediaSurfaceInteractionState,
    last_timeline_receipt: &'a mut Option<TimelineUxReceipt>,
    last_media_receipt: &'a mut Option<MediaSurfaceUxReceipt>,
    rendered_panel_count: usize,
}

impl EditorPanelRuntime<'_> {
    fn show(&mut self, ui: &mut egui::Ui, panel_id: &PanelId) {
        self.rendered_panel_count += 1;
        match panel_id.as_str() {
            MEDIA_PANEL => show_media_panel(ui),
            PREVIEW_PANEL => self.show_preview(ui),
            INSPECTOR_PANEL => show_inspector_panel(ui),
            TIMELINE_PANEL => self.show_timeline(ui),
            AGENT_PANEL => show_agent_panel(ui),
            unknown => {
                ui.label(format!("Unknown panel: {unknown}"));
            }
        }
    }

    fn show_preview(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        let render_size = fit_16_by_9(available);

        let Some(preview) = self.preview.as_mut() else {
            ui.colored_label(
                self.theme.text,
                self.startup_error.unwrap_or("GPU preview unavailable"),
            );
            return;
        };
        let requested_pixels = [
            (render_size.x * self.pixels_per_point).round() as u32,
            (render_size.y * self.pixels_per_point).round() as u32,
        ];
        let texture_id = preview.render(requested_pixels, self.elapsed_seconds);
        let stats = preview.stats();
        let output = show_media_surface(
            ui,
            &MediaSurfaceSnapshot {
                texture_id,
                source_size: stats.texture_size,
                transform: *self.media_transform,
                selected: true,
            },
            self.media_interaction,
        );
        apply_media_actions(
            self.media_transform,
            self.media_interaction,
            &output.actions,
        );
        *self.last_media_receipt = Some(output.receipt);
    }

    fn show_timeline(&mut self, ui: &mut egui::Ui) {
        let output = show_timeline(
            ui,
            self.timeline_cache,
            self.timeline_viewport,
            self.timeline_playhead_tick,
            self.timeline_interaction,
        );
        self.timeline_interaction.apply_all(&output.actions);
        *self.last_timeline_receipt = Some(output.receipt);
    }
}

fn apply_media_actions(
    transform: &mut MediaTransform,
    interaction: &mut MediaSurfaceInteractionState,
    actions: &[MediaSurfaceAction],
) {
    for action in actions {
        if let MediaSurfaceAction::CommitTransform {
            kind,
            delta_points,
            scale_multiplier,
        } = action
        {
            match kind {
                MediaTransformKind::Move => transform.offset_points += *delta_points,
                MediaTransformKind::Scale => {
                    transform.scale = (transform.scale * scale_multiplier).max(0.01);
                }
            }
        }
        interaction.apply(action);
    }
}

fn show_media_panel(ui: &mut egui::Ui) {
    ui.small("PROJECT MEDIA");
    ui.weak("Search media…");
    ui.separator();
    for (name, kind) in [
        ("av-a.mp4", "10s · H.264"),
        ("av-b.mp4", "10s · H.264"),
        ("still.png", "RGBA image"),
    ] {
        ui.strong(name);
        ui.small(kind);
        ui.add_space(6.0);
    }
}

#[cfg(test)]
mod tests {
    use eframe::egui::{Rect, pos2, vec2};
    use finui_workbench::{CommandScopeOutput, PanelId, RegionId, calculate_workbench_geometry};

    use super::{PREVIEW_PANEL, default_editor_workbench, default_timeline_fixture, fit_16_by_9};

    #[test]
    fn editor_shell_contains_required_panels_regions_and_dividers() {
        let state = default_editor_workbench();
        let geometry = calculate_workbench_geometry(
            &state,
            Rect::from_min_size(pos2(0.0, 0.0), vec2(1280.0, 720.0)),
            1.0,
        )
        .unwrap();
        let panel_ids: Vec<_> = geometry
            .tab_regions
            .iter()
            .flat_map(|region| region.tabs.iter().map(|tab| tab.id.as_str()))
            .collect();

        assert_eq!(geometry.dividers.len(), 3);
        assert_eq!(geometry.tab_regions.len(), 4);
        assert_eq!(
            panel_ids,
            ["media", "preview", "inspector", "agent", "timeline"]
        );
        assert_eq!(
            state.command_scope(),
            CommandScopeOutput::Panel {
                region_id: RegionId::from("preview-region"),
                panel_id: PanelId::from(PREVIEW_PANEL),
            }
        );
    }

    #[test]
    fn preview_aspect_fit_stays_inside_panel() {
        assert_eq!(fit_16_by_9(vec2(1600.0, 900.0)), vec2(1600.0, 900.0));
        let fitted = fit_16_by_9(vec2(800.0, 300.0));
        assert!((fitted.x - 533.3333).abs() < 0.001);
        assert_eq!(fitted.y, 300.0);
    }

    #[test]
    fn editor_timeline_fixture_contains_ten_thousand_clips() {
        let fixture = default_timeline_fixture();
        assert_eq!(fixture.tracks.len(), 100);
        assert_eq!(fixture.total_clip_count(), 10_000);
    }
}

fn fit_16_by_9(bounds: egui::Vec2) -> egui::Vec2 {
    let mut width = bounds.x;
    let mut height = width * 9.0 / 16.0;
    if height > bounds.y {
        height = bounds.y;
        width = height * 16.0 / 9.0;
    }
    egui::vec2(width.max(16.0), height.max(16.0))
}

fn show_inspector_panel(ui: &mut egui::Ui) {
    ui.small("CLIP INSPECTOR");
    egui::Grid::new("editor-inspector-grid")
        .num_columns(2)
        .spacing([10.0, 8.0])
        .show(ui, |ui| {
            for (label, value) in [
                ("Position", "0, 0"),
                ("Scale", "100%"),
                ("Rotation", "0°"),
                ("Opacity", "100%"),
                ("Speed", "1.0x"),
            ] {
                ui.label(label);
                ui.monospace(value);
                ui.end_row();
            }
        });
}

fn show_agent_panel(ui: &mut egui::Ui) {
    ui.small("AGENT CONTEXT");
    ui.monospace("selection: clip-av-a");
    ui.monospace("transport: paused");
    ui.monospace("commands: panel-scoped");
}

fn command_scope_label(scope: &CommandScopeOutput) -> String {
    match scope {
        CommandScopeOutput::Global => "scope: global".to_owned(),
        CommandScopeOutput::Panel {
            region_id,
            panel_id,
        } => format!("scope: {region_id}/{panel_id}"),
    }
}

#[derive(Clone)]
struct SmokeConfig {
    frames: u64,
    recreate_frame: u64,
    receipt_path: PathBuf,
}

impl SmokeConfig {
    fn from_env() -> Option<Self> {
        let frames = std::env::var("FINUI_EDITOR_LAB_SMOKE_FRAMES")
            .ok()?
            .parse::<u64>()
            .ok()?
            .max(6);
        let receipt_path = std::env::var_os("FINUI_EDITOR_LAB_SMOKE_RECEIPT")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".tmp/editor-lab-smoke.json"));

        Some(Self {
            frames,
            recreate_frame: (frames / 3).max(2),
            receipt_path,
        })
    }
}
