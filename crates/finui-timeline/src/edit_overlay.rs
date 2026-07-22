use egui::{Pos2, Rect, Response, Sense, Ui, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineOverlayTarget {
    Playhead,
    Marker,
    RangeIn,
    RangeOut,
    TrimStart,
    TrimEnd,
    Slip,
    FadeStart,
    FadeEnd,
    Keyframe,
    DragGhost,
}

impl TimelineOverlayTarget {
    pub const ALL_ACCESSIBLE: [Self; 11] = [
        Self::Playhead,
        Self::Marker,
        Self::RangeIn,
        Self::RangeOut,
        Self::TrimStart,
        Self::TrimEnd,
        Self::Slip,
        Self::FadeStart,
        Self::FadeEnd,
        Self::Keyframe,
        Self::DragGhost,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Playhead => "playhead",
            Self::Marker => "marker",
            Self::RangeIn => "range_in",
            Self::RangeOut => "range_out",
            Self::TrimStart => "trim_start",
            Self::TrimEnd => "trim_end",
            Self::Slip => "slip",
            Self::FadeStart => "fade_start",
            Self::FadeEnd => "fade_end",
            Self::Keyframe => "keyframe",
            Self::DragGhost => "drag_ghost",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimelineOverlayActionPhase {
    Begin,
    Update,
    Commit,
    Cancel,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TimelineOverlayAction {
    pub interaction_id: u64,
    pub phase: TimelineOverlayActionPhase,
    pub target: TimelineOverlayTarget,
    pub delta_ticks: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimelineOverlayGeometryInput {
    pub start_tick: i64,
    pub end_tick: i64,
    pub playhead_tick: i64,
    pub in_tick: i64,
    pub out_tick: i64,
    pub marker_tick: i64,
    pub snap_tick: i64,
    pub compact: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimelineOverlayGeometry {
    pub rect: Rect,
    pub ruler: Rect,
    pub track: Rect,
    pub range: Rect,
    pub playhead_x: f32,
    pub in_x: f32,
    pub out_x: f32,
    pub marker_x: f32,
    pub snap_x: f32,
    pub visible_tick_range: [i64; 2],
}

impl TimelineOverlayGeometry {
    pub fn build(rect: Rect, input: TimelineOverlayGeometryInput) -> Self {
        let ruler_height = if input.compact { 24.0 } else { 38.0 };
        let ruler =
            Rect::from_min_max(rect.min, Pos2::new(rect.right(), rect.top() + ruler_height));
        let track = Rect::from_min_max(
            Pos2::new(rect.left() + 8.0, ruler.bottom() + 10.0),
            Pos2::new(rect.right() - 8.0, rect.bottom() - 10.0),
        );
        let start_tick = input.start_tick;
        let end_tick = input.end_tick.max(start_tick.saturating_add(1));
        let span = (end_tick - start_tick).max(1);
        let x_for = |tick: i64| {
            rect.left() + ((tick - start_tick) as f32 / span as f32).clamp(0.0, 1.0) * rect.width()
        };
        let in_x = x_for(input.in_tick);
        let out_x = x_for(input.out_tick);
        Self {
            rect,
            ruler,
            track,
            range: Rect::from_min_max(
                Pos2::new(in_x, ruler.bottom()),
                Pos2::new(out_x.max(in_x + 2.0), track.bottom()),
            ),
            playhead_x: x_for(input.playhead_tick),
            in_x,
            out_x,
            marker_x: x_for(input.marker_tick),
            snap_x: x_for(input.snap_tick),
            visible_tick_range: [start_tick, end_tick],
        }
    }

    pub fn tick_for_x(self, x: f32) -> i64 {
        let fraction = ((x - self.rect.left()) / self.rect.width().max(1.0)).clamp(0.0, 1.0);
        self.visible_tick_range[0]
            + ((self.visible_tick_range[1] - self.visible_tick_range[0]) as f32 * fraction).round()
                as i64
    }

    pub fn x_for_tick(self, tick: i64) -> f32 {
        let span = (self.visible_tick_range[1] - self.visible_tick_range[0]).max(1);
        self.rect.left()
            + ((tick - self.visible_tick_range[0]) as f32 / span as f32).clamp(0.0, 1.0)
                * self.rect.width()
    }

    pub fn target_rect(self, target: TimelineOverlayTarget) -> Rect {
        let x = match target {
            TimelineOverlayTarget::Playhead => self.playhead_x,
            TimelineOverlayTarget::Marker => self.marker_x,
            TimelineOverlayTarget::RangeIn
            | TimelineOverlayTarget::TrimStart
            | TimelineOverlayTarget::FadeStart => self.in_x,
            TimelineOverlayTarget::RangeOut
            | TimelineOverlayTarget::TrimEnd
            | TimelineOverlayTarget::FadeEnd => self.out_x,
            TimelineOverlayTarget::Slip | TimelineOverlayTarget::Keyframe => self.track.center().x,
            TimelineOverlayTarget::DragGhost => self.range.center().x,
        };
        Rect::from_center_size(
            Pos2::new(x, self.track.center().y),
            Vec2::new(16.0, self.track.height().max(24.0)),
        )
    }
}

#[derive(Clone, Debug)]
pub struct TimelineOverlayAccessibility<'a> {
    pub root_label: &'a str,
    pub state_label: &'a str,
    pub playhead_time: &'a str,
    pub signed_snap_delta_label: &'a str,
    pub invalid_reason: Option<&'a str>,
    pub cancelled_status: Option<&'a str>,
    pub target_labels: [&'a str; 11],
    pub reservation_progress_percent: Option<u8>,
}

#[derive(Clone, Copy, Debug)]
pub struct TimelineOverlayInteractionInput {
    pub interaction_id: u64,
    pub target: TimelineOverlayTarget,
    pub ticks_per_point: f32,
    pub enabled: bool,
    pub keyboard_active: bool,
    pub active_interaction_id: Option<u64>,
}

pub struct TimelineOverlayBehaviorOutput {
    pub response: Response,
    pub actions: Vec<TimelineOverlayAction>,
    pub accessibility_node_count: usize,
}

pub fn show_timeline_edit_overlay_behavior(
    ui: &mut Ui,
    geometry: TimelineOverlayGeometry,
    input: TimelineOverlayInteractionInput,
    accessibility: TimelineOverlayAccessibility<'_>,
) -> TimelineOverlayBehaviorOutput {
    let response = ui.interact(
        geometry.rect,
        ui.make_persistent_id("finui-timeline-edit-overlay-root"),
        Sense::click_and_drag(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            input.enabled,
            accessibility.root_label,
        )
    });
    let status = [
        accessibility.root_label,
        accessibility.state_label,
        accessibility.playhead_time,
        accessibility.signed_snap_delta_label,
        accessibility.invalid_reason.unwrap_or_default(),
        accessibility.cancelled_status.unwrap_or_default(),
    ]
    .into_iter()
    .filter(|value| !value.is_empty())
    .collect::<Vec<_>>()
    .join(" · ");
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Application);
        node.set_label(status);
    });
    if input.enabled && (response.clicked() || response.drag_started()) {
        response.request_focus();
    }

    let mut accessibility_node_count = 1;
    for (target, label) in TimelineOverlayTarget::ALL_ACCESSIBLE
        .into_iter()
        .zip(accessibility.target_labels)
    {
        let target_response = ui.interact(
            geometry.target_rect(target),
            ui.make_persistent_id(("finui-timeline-edit-overlay-target", target.as_str())),
            Sense::click_and_drag(),
        );
        target_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Slider, input.enabled, label)
        });
        ui.ctx().accesskit_node_builder(target_response.id, |node| {
            node.set_role(egui::accesskit::Role::Slider);
            node.set_label(label);
            node.set_numeric_value(match target {
                TimelineOverlayTarget::Playhead => geometry.tick_for_x(geometry.playhead_x) as f64,
                TimelineOverlayTarget::Marker => geometry.tick_for_x(geometry.marker_x) as f64,
                TimelineOverlayTarget::RangeIn
                | TimelineOverlayTarget::TrimStart
                | TimelineOverlayTarget::FadeStart => geometry.tick_for_x(geometry.in_x) as f64,
                TimelineOverlayTarget::RangeOut
                | TimelineOverlayTarget::TrimEnd
                | TimelineOverlayTarget::FadeEnd => geometry.tick_for_x(geometry.out_x) as f64,
                TimelineOverlayTarget::Slip
                | TimelineOverlayTarget::Keyframe
                | TimelineOverlayTarget::DragGhost => {
                    geometry.tick_for_x(geometry.track.center().x) as f64
                }
            });
            node.set_numeric_value_step(1.0);
        });
        accessibility_node_count += 1;
    }
    if let Some(progress) = accessibility.reservation_progress_percent {
        let progress_id = ui.make_persistent_id("finui-timeline-edit-overlay-progress");
        ui.ctx().accesskit_node_builder(progress_id, |node| {
            node.set_role(egui::accesskit::Role::ProgressIndicator);
            node.set_label(accessibility.target_labels[10]);
            node.set_numeric_value(f64::from(progress));
            node.set_min_numeric_value(0.0);
            node.set_max_numeric_value(100.0);
        });
        accessibility_node_count += 1;
    }

    let mut actions = Vec::new();
    if input.enabled && response.drag_started() {
        actions.push(action(input, TimelineOverlayActionPhase::Begin, 0));
    }
    if input.enabled && response.dragged() {
        actions.push(action(
            input,
            TimelineOverlayActionPhase::Update,
            (response.drag_delta().x * input.ticks_per_point).round() as i64,
        ));
    }
    if input.enabled && response.drag_stopped() {
        actions.push(action(input, TimelineOverlayActionPhase::Commit, 0));
    }
    if input.enabled && input.keyboard_active && response.has_focus() {
        ui.input(|raw| {
            let step = if raw.modifiers.shift { 10 } else { 1 };
            let delta = if raw.key_pressed(egui::Key::ArrowLeft) {
                Some(-step)
            } else if raw.key_pressed(egui::Key::ArrowRight) {
                Some(step)
            } else {
                None
            };
            if let Some(delta) = delta {
                actions.extend([
                    action(input, TimelineOverlayActionPhase::Begin, 0),
                    action(input, TimelineOverlayActionPhase::Update, delta),
                    action(input, TimelineOverlayActionPhase::Commit, delta),
                ]);
            }
            if raw.key_pressed(egui::Key::Escape)
                && input.active_interaction_id == Some(input.interaction_id)
            {
                actions.push(action(input, TimelineOverlayActionPhase::Cancel, 0));
            }
        });
    }
    if response.lost_focus() && input.active_interaction_id == Some(input.interaction_id) {
        actions.push(action(input, TimelineOverlayActionPhase::Cancel, 0));
    }

    TimelineOverlayBehaviorOutput {
        response,
        actions,
        accessibility_node_count,
    }
}

fn action(
    input: TimelineOverlayInteractionInput,
    phase: TimelineOverlayActionPhase,
    delta_ticks: i64,
) -> TimelineOverlayAction {
    TimelineOverlayAction {
        interaction_id: input.interaction_id,
        phase,
        target: input.target,
        delta_ticks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn geometry() -> TimelineOverlayGeometry {
        TimelineOverlayGeometry::build(
            Rect::from_min_size(Pos2::ZERO, Vec2::new(800.0, 160.0)),
            TimelineOverlayGeometryInput {
                start_tick: 0,
                end_tick: 35_000,
                playhead_tick: 18_120,
                in_tick: 5_000,
                out_tick: 30_000,
                marker_tick: 8_000,
                snap_tick: 20_000,
                compact: false,
            },
        )
    }

    fn accessibility<'a>() -> TimelineOverlayAccessibility<'a> {
        TimelineOverlayAccessibility {
            root_label: "Timeline edit overlay",
            state_label: "invalid",
            playhead_time: "00:00:18:03",
            signed_snap_delta_label: "-12 ticks",
            invalid_reason: Some("Cannot overlap clips"),
            cancelled_status: None,
            target_labels: [
                "Playhead",
                "Marker",
                "In point",
                "Out point",
                "Trim start",
                "Trim end",
                "Slip handle",
                "Fade start",
                "Fade end",
                "Keyframe",
                "Reservation 62%",
            ],
            reservation_progress_percent: Some(62),
        }
    }

    #[test]
    fn geometry_uses_exact_integer_tick_projection() {
        let geometry = geometry();
        assert_eq!(geometry.tick_for_x(geometry.playhead_x), 18_120);
        assert_eq!(geometry.visible_tick_range, [0, 35_000]);
        assert!(geometry.in_x < geometry.out_x);
    }

    #[test]
    fn accesskit_exposes_root_targets_status_and_progress() {
        let context = egui::Context::default();
        context.enable_accesskit();
        let mut count = 0;
        let output = context.run_ui(Default::default(), |ui| {
            ui.set_min_size(Vec2::new(800.0, 160.0));
            count = show_timeline_edit_overlay_behavior(
                ui,
                geometry(),
                TimelineOverlayInteractionInput {
                    interaction_id: 22,
                    target: TimelineOverlayTarget::TrimEnd,
                    ticks_per_point: 10.0,
                    enabled: true,
                    keyboard_active: true,
                    active_interaction_id: None,
                },
                accessibility(),
            )
            .accessibility_node_count;
        });
        assert_eq!(count, 13);
        let update = output
            .platform_output
            .accesskit_update
            .expect("accesskit update");
        assert!(update.nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Application
                && node
                    .label()
                    .is_some_and(|label| label.contains("Cannot overlap clips"))
        }));
        assert!(update.nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Slider && node.label() == Some("Slip handle")
        }));
        assert!(update.nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::ProgressIndicator
                && node.numeric_value() == Some(62.0)
        }));
    }

    #[test]
    fn keyboard_nudge_is_a_complete_transaction_and_background_input_is_ignored() {
        let context = egui::Context::default();
        let mut id = None;
        let _ = context.run_ui(Default::default(), |ui| {
            ui.set_min_size(Vec2::new(800.0, 160.0));
            id = Some(
                show_timeline_edit_overlay_behavior(
                    ui,
                    geometry(),
                    TimelineOverlayInteractionInput {
                        interaction_id: 22,
                        target: TimelineOverlayTarget::Keyframe,
                        ticks_per_point: 10.0,
                        enabled: true,
                        keyboard_active: true,
                        active_interaction_id: None,
                    },
                    accessibility(),
                )
                .response
                .id,
            );
        });
        let id = id.expect("root response id");
        context.memory_mut(|memory| memory.request_focus(id));
        let input = egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::ArrowRight,
                physical_key: Some(egui::Key::ArrowRight),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::SHIFT,
            }],
            modifiers: egui::Modifiers::SHIFT,
            ..Default::default()
        };
        let mut actions = Vec::new();
        let _ = context.run_ui(input, |ui| {
            ui.set_min_size(Vec2::new(800.0, 160.0));
            actions = show_timeline_edit_overlay_behavior(
                ui,
                geometry(),
                TimelineOverlayInteractionInput {
                    interaction_id: 22,
                    target: TimelineOverlayTarget::Keyframe,
                    ticks_per_point: 10.0,
                    enabled: true,
                    keyboard_active: true,
                    active_interaction_id: None,
                },
                accessibility(),
            )
            .actions;
        });
        assert_eq!(actions.len(), 3);
        assert_eq!(actions[0].phase, TimelineOverlayActionPhase::Begin);
        assert_eq!(actions[1].delta_ticks, 10);
        assert_eq!(actions[2].phase, TimelineOverlayActionPhase::Commit);

        let mut background_actions = vec![TimelineOverlayAction {
            interaction_id: 0,
            phase: TimelineOverlayActionPhase::Cancel,
            target: TimelineOverlayTarget::Playhead,
            delta_ticks: 0,
        }];
        let _ = context.run_ui(Default::default(), |ui| {
            ui.set_min_size(Vec2::new(800.0, 160.0));
            background_actions = show_timeline_edit_overlay_behavior(
                ui,
                geometry(),
                TimelineOverlayInteractionInput {
                    interaction_id: 22,
                    target: TimelineOverlayTarget::Keyframe,
                    ticks_per_point: 10.0,
                    enabled: true,
                    keyboard_active: false,
                    active_interaction_id: None,
                },
                accessibility(),
            )
            .actions;
        });
        assert!(background_actions.is_empty());
    }
}
