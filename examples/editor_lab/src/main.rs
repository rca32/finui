mod gpu_preview;

use std::{path::PathBuf, time::Duration};

use eframe::egui;
use finui_primitives::{PrimitiveTheme, ThemeMode};
use gpu_preview::{GpuPreview, RecreateReason};

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

        Self {
            preview,
            startup_error,
            smoke,
            app_frame: 0,
            smoke_finished: false,
        }
    }

    fn finish_smoke_if_ready(&mut self, ctx: &egui::Context) {
        let Some(smoke) = self.smoke.as_ref() else {
            return;
        };
        if self.smoke_finished || self.app_frame < smoke.frames {
            return;
        }

        let result = self
            .preview
            .as_ref()
            .ok_or_else(|| {
                self.startup_error
                    .clone()
                    .unwrap_or_else(|| "GPU preview did not start".to_owned())
            })
            .and_then(|preview| preview.write_smoke_receipt(&smoke.receipt_path));

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

        ui.heading("Finui Editor Lab");
        ui.label("wgpu application runtime");
        ui.colored_label(theme.text, "GPU-native texture bridge");

        let mut recreate_requested = false;
        ui.horizontal(|ui| {
            if ui.button("Recreate GPU texture").clicked() {
                recreate_requested = true;
            }
            ui.label("Resize the window to exercise deferred texture retirement.");
        });

        let width = ui.available_width().clamp(320.0, 1600.0);
        let display_size = egui::vec2(width, width * 9.0 / 16.0);

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

            let elapsed_seconds = ui.input(|input| input.time) as f32;
            let pixels_per_point = ui.ctx().pixels_per_point();
            let requested_pixels = [
                (display_size.x * pixels_per_point).round() as u32,
                (display_size.y * pixels_per_point).round() as u32,
            ];
            let texture_id = preview.render(requested_pixels, elapsed_seconds);

            ui.add(
                egui::Image::new((texture_id, display_size))
                    .fit_to_exact_size(display_size)
                    .corner_radius(6.0),
            );

            let stats = preview.stats();
            ui.monospace(format!(
                "{}x{} px | generation {} | {} GPU frames | {} retired",
                stats.texture_size[0],
                stats.texture_size[1],
                stats.generation,
                stats.render_passes,
                stats.texture_releases
            ));
            ui.small(format!(
                "{} | CPU pixel readbacks: {} | CPU pixel uploads: {} bytes",
                preview.adapter_summary(),
                stats.cpu_pixel_readbacks,
                stats.cpu_pixel_upload_bytes
            ));
        } else if let Some(error) = self.startup_error.as_ref() {
            ui.colored_label(theme.text, error);
        }

        ui.ctx().request_repaint_after(Duration::from_millis(16));
        self.finish_smoke_if_ready(ui.ctx());
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
