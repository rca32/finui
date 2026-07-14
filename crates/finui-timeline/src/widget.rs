use egui::{Align2, Color32, FontId, Rect, Sense, Stroke, StrokeKind, Ui, pos2};
use serde::{Deserialize, Serialize};

use crate::{
    TimelineAction, TimelineDragKind, TimelineGeometry, TimelineGeometryCache,
    TimelineInteractionState, TimelineViewport,
};

#[derive(Clone, Debug)]
pub struct TimelineOutput {
    pub actions: Vec<TimelineAction>,
    pub geometry: TimelineGeometry,
    pub receipt: TimelineUxReceipt,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineUxReceipt {
    pub primitive: String,
    pub total_clip_count: usize,
    pub candidate_clip_count: usize,
    pub painted_clip_ids: Vec<String>,
    pub hit_test_clip_ids: Vec<String>,
    pub selected_visible_clip_ids: Vec<String>,
    pub visible_track_range: [usize; 2],
    pub visible_tick_range: [i64; 2],
    pub drag_phase: String,
    pub accessibility_label: String,
    pub focus_order_clip_ids: Vec<String>,
    pub keyboard_selection_enabled: bool,
}

pub fn timeline_receipt_json(receipt: &TimelineUxReceipt) -> String {
    serde_json::to_string_pretty(receipt).expect("timeline receipt is serializable")
}

pub fn show_timeline(
    ui: &mut Ui,
    cache: &TimelineGeometryCache,
    viewport: TimelineViewport,
    playhead_tick: i64,
    interaction: &TimelineInteractionState,
) -> TimelineOutput {
    let rect = ui.available_rect_before_wrap();
    let root_response = ui.allocate_rect(rect, Sense::hover());
    root_response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Other, true, "Timeline editor")
    });
    let geometry = cache.geometry(viewport, rect, playhead_tick);
    let painter = ui.painter().clone();
    let visuals = ui.visuals().clone();

    painter.rect_filled(rect, 0.0, visuals.extreme_bg_color);
    painter.rect_filled(
        Rect::from_min_max(rect.min, pos2(rect.max.x, geometry.content_rect.min.y)),
        0.0,
        visuals.faint_bg_color,
    );
    for mark in &geometry.ruler_marks {
        let x = rect.min.x + mark.x;
        painter.line_segment(
            [pos2(x, rect.min.y + 14.0), pos2(x, rect.min.y + 23.0)],
            visuals.widgets.noninteractive.bg_stroke,
        );
        painter.text(
            pos2(x + 3.0, rect.min.y + 8.0),
            Align2::LEFT_CENTER,
            mark.tick.to_string(),
            FontId::monospace(9.0),
            visuals.weak_text_color(),
        );
    }

    let mut actions = Vec::new();
    let ruler_rect = Rect::from_min_max(rect.min, pos2(rect.max.x, geometry.content_rect.min.y));
    let ruler_response = ui.interact(
        ruler_rect,
        ui.make_persistent_id("finui-timeline-ruler"),
        Sense::click(),
    );
    ruler_response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Timeline ruler")
    });
    if ruler_response.clicked()
        && let Some(pointer) = ruler_response.interact_pointer_pos()
    {
        actions.push(TimelineAction::SetPlayhead {
            tick: pointer_tick(rect, viewport, pointer.x),
        });
    }
    for clip in &geometry.visible_clips {
        let fill = if clip.selected {
            visuals.selection.bg_fill
        } else {
            Color32::from_rgb(45, 103, 126)
        };
        painter.rect_filled(clip.rect, 3.0, fill);
        painter.rect_stroke(
            clip.rect,
            3.0,
            Stroke::new(1.0, visuals.widgets.noninteractive.bg_stroke.color),
            StrokeKind::Inside,
        );
        painter.text(
            pos2(clip.rect.min.x + 6.0, clip.rect.center().y),
            Align2::LEFT_CENTER,
            &clip.label,
            FontId::proportional(11.0),
            Color32::WHITE,
        );

        if interaction.drag.is_none() {
            let response = ui.interact(
                clip.rect,
                ui.make_persistent_id(("finui-timeline-clip", clip.clip_id.as_str())),
                Sense::click_and_drag(),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Button, true, &clip.label)
            });
            if response.clicked() {
                response.request_focus();
                actions.push(TimelineAction::SelectClip {
                    clip_id: clip.clip_id.clone(),
                    additive: ui.input(|input| input.modifiers.command),
                });
            }
            if response.has_focus()
                && ui.input(|input| {
                    input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                })
            {
                actions.push(TimelineAction::SelectClip {
                    clip_id: clip.clip_id.clone(),
                    additive: ui.input(|input| input.modifiers.command),
                });
            }
            if response.drag_started()
                && let Some(pointer) = response.interact_pointer_pos()
                && let Some(hit) = geometry.hit_test(pointer)
            {
                actions.push(TimelineAction::BeginDrag {
                    clip_id: clip.clip_id.clone(),
                    kind: TimelineDragKind::from(hit.zone),
                    pointer_tick: pointer_tick(rect, viewport, pointer.x),
                    origin_start_tick: clip.start_tick,
                    origin_duration_ticks: clip.duration_ticks,
                });
            }
        }
    }

    if let Some(drag) = interaction.drag.as_ref() {
        let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
        if escape {
            actions.push(TimelineAction::CancelDrag {
                clip_id: drag.clip_id.clone(),
                kind: drag.kind,
            });
        } else if ui.input(|input| input.pointer.any_released()) {
            actions.push(TimelineAction::CommitDrag {
                clip_id: drag.clip_id.clone(),
                kind: drag.kind,
                delta_ticks: drag.delta_ticks,
            });
        } else if let Some(pointer) = ui.input(|input| input.pointer.interact_pos()) {
            let tick = pointer_tick(rect, viewport, pointer.x);
            actions.push(TimelineAction::UpdateDrag {
                clip_id: drag.clip_id.clone(),
                kind: drag.kind,
                delta_ticks: tick - drag.origin_pointer_tick,
            });
        }
    }

    if let Some(x) = geometry.playhead_x {
        painter.line_segment(
            [pos2(x, rect.min.y), pos2(x, rect.max.y)],
            Stroke::new(1.5, Color32::from_rgb(245, 88, 88)),
        );
    }

    let receipt = TimelineUxReceipt {
        primitive: "timeline".to_owned(),
        total_clip_count: geometry.total_clip_count,
        candidate_clip_count: geometry.candidate_clip_count,
        painted_clip_ids: clip_ids(&geometry),
        hit_test_clip_ids: clip_ids(&geometry),
        selected_visible_clip_ids: geometry
            .visible_clips
            .iter()
            .filter(|clip| clip.selected)
            .map(|clip| clip.clip_id.to_string())
            .collect(),
        visible_track_range: [
            geometry.visible_track_range.0,
            geometry.visible_track_range.1,
        ],
        visible_tick_range: [geometry.visible_tick_range.0, geometry.visible_tick_range.1],
        drag_phase: if interaction.drag.is_some() {
            "active".to_owned()
        } else {
            "idle".to_owned()
        },
        accessibility_label: "Timeline editor".to_owned(),
        focus_order_clip_ids: clip_ids(&geometry),
        keyboard_selection_enabled: true,
    };

    TimelineOutput {
        actions,
        geometry,
        receipt,
    }
}

fn pointer_tick(rect: Rect, viewport: TimelineViewport, pointer_x: f32) -> i64 {
    viewport.start_tick.saturating_add(
        (f64::from(pointer_x - rect.min.x) * viewport.ticks_per_point).round() as i64,
    )
}

fn clip_ids(geometry: &TimelineGeometry) -> Vec<String> {
    geometry
        .visible_clips
        .iter()
        .map(|clip| clip.clip_id.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{TimelineClip, TimelineSnapshot, TimelineTrack, TimelineViewport};

    use super::*;

    #[test]
    fn receipt_lists_only_painted_and_hit_tested_clips() {
        let snapshot = TimelineSnapshot {
            revision: 1,
            tracks: vec![TimelineTrack::new(
                "v1",
                "V1",
                vec![
                    TimelineClip::new("offscreen", "Off", 0, 10),
                    TimelineClip::new("visible", "Visible", 100, 50),
                ],
            )],
        };
        let cache = TimelineGeometryCache::build(&snapshot);
        egui::__run_test_ui(|ui| {
            ui.set_min_size(egui::vec2(400.0, 100.0));
            let output = show_timeline(
                ui,
                &cache,
                TimelineViewport::new(90, 1.0),
                110,
                &TimelineInteractionState::default(),
            );
            assert_eq!(output.receipt.painted_clip_ids, ["visible"]);
            assert_eq!(
                output.receipt.hit_test_clip_ids,
                output.receipt.painted_clip_ids
            );
            assert!(timeline_receipt_json(&output.receipt).contains("\"primitive\": \"timeline\""));
        });
    }
}
