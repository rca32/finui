use std::hash::Hash;

use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

use crate::config::{TV_LIGHT, tv_theme_for_dark_mode};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayerPlacement {
    Fixed(Pos2),
    BelowStart { offset: Vec2 },
    BelowEnd { offset: Vec2 },
    AboveStart { offset: Vec2 },
    RightStart { offset: Vec2 },
    Centered { top_margin: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl LayerSide {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerAlign {
    Start,
    Center,
    End,
}

impl LayerAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayerResolvedPlacement {
    pub side: LayerSide,
    pub align: LayerAlign,
    pub flipped: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DismissPolicy {
    None,
    OutsideClick,
    OutsideClickAndEscape,
    EscapeOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DismissLayerEventKind {
    EscapeKeyDown,
    PointerDownOutside,
    PointerDownOnTrigger,
}

impl DismissLayerEventKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EscapeKeyDown => "escapeKeyDown",
            Self::PointerDownOutside => "pointerDownOutside",
            Self::PointerDownOnTrigger => "pointerDownOnTrigger",
        }
    }

    pub fn is_interact_outside(self) -> bool {
        matches!(self, Self::PointerDownOutside | Self::PointerDownOnTrigger)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DismissLayerEvent {
    pub kind: DismissLayerEventKind,
    pub pointer_button: Option<egui::PointerButton>,
    pub position: Option<Pos2>,
    pub default_prevented: bool,
}

impl DismissLayerEvent {
    pub fn escape_key_down() -> Self {
        Self {
            kind: DismissLayerEventKind::EscapeKeyDown,
            pointer_button: None,
            position: None,
            default_prevented: false,
        }
    }

    pub fn pointer_down(
        kind: DismissLayerEventKind,
        pointer_button: egui::PointerButton,
        position: Pos2,
    ) -> Self {
        Self {
            kind,
            pointer_button: Some(pointer_button),
            position: Some(position),
            default_prevented: false,
        }
    }

    pub fn prevent_default(mut self) -> Self {
        self.default_prevented = true;
        self
    }

    pub fn should_close(self) -> bool {
        !self.default_prevented
    }
}

pub type DismissLayerFilter = fn(DismissLayerEvent) -> DismissLayerEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DismissLayerCandidate {
    pub layer_depth: usize,
    pub event: DismissLayerEvent,
}

pub fn nested_dismiss_close_order(
    candidates: impl IntoIterator<Item = DismissLayerCandidate>,
) -> Vec<DismissLayerCandidate> {
    let mut candidates = candidates
        .into_iter()
        .filter(|candidate| candidate.event.should_close())
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| right.layer_depth.cmp(&left.layer_depth));
    candidates
}

pub struct AnchoredLayerOptions {
    pub id: egui::Id,
    pub anchor_rect: Option<Rect>,
    pub placement: LayerPlacement,
    pub width: f32,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub inner_margin: egui::Margin,
    pub margin: f32,
    pub order: egui::Order,
    pub dismiss_policy: DismissPolicy,
    pub dismiss_filter: Option<DismissLayerFilter>,
    pub fill: Color32,
    pub stroke: Stroke,
    pub radius: u8,
}

impl AnchoredLayerOptions {
    pub fn new(id: impl Hash, width: f32) -> Self {
        Self {
            id: egui::Id::new(id),
            anchor_rect: None,
            placement: LayerPlacement::Fixed(Pos2::ZERO),
            width,
            min_height: None,
            max_height: None,
            inner_margin: egui::Margin::same(8),
            margin: 8.0,
            order: egui::Order::Foreground,
            dismiss_policy: DismissPolicy::OutsideClickAndEscape,
            dismiss_filter: None,
            fill: TV_LIGHT.popup_background,
            stroke: Stroke::new(1.0, TV_LIGHT.toolbar_border),
            radius: 6,
        }
    }

    pub fn anchor_rect(mut self, rect: Rect) -> Self {
        self.anchor_rect = Some(rect);
        self
    }

    pub fn placement(mut self, placement: LayerPlacement) -> Self {
        self.placement = placement;
        self
    }

    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = Some(min_height);
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn inner_margin(mut self, inner_margin: egui::Margin) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    pub fn dismiss_policy(mut self, dismiss_policy: DismissPolicy) -> Self {
        self.dismiss_policy = dismiss_policy;
        self
    }

    pub fn dismiss_filter(mut self, dismiss_filter: DismissLayerFilter) -> Self {
        self.dismiss_filter = Some(dismiss_filter);
        self
    }

    pub fn order(mut self, order: egui::Order) -> Self {
        self.order = order;
        self
    }
}

pub struct LayerOutput<T> {
    pub action: Option<T>,
    pub should_close: bool,
    pub dismiss_event: Option<DismissLayerEvent>,
    pub panel_rect: Rect,
    pub resolved_placement: LayerResolvedPlacement,
}

pub fn show_anchored_layer<T>(
    ctx: &egui::Context,
    mut options: AnchoredLayerOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> Option<T>,
) -> LayerOutput<T> {
    if ctx.global_style().visuals.dark_mode
        && options.fill == TV_LIGHT.popup_background
        && options.stroke.color == TV_LIGHT.toolbar_border
    {
        let tv = tv_theme_for_dark_mode(true);
        options.fill = tv.popup_background;
        options.stroke = Stroke::new(1.0, tv.toolbar_border);
    }
    let estimated_size = Vec2::new(
        options.width + f32::from(options.inner_margin.left + options.inner_margin.right),
        options
            .max_height
            .or(options.min_height)
            .unwrap_or(220.0)
            .max(24.0),
    );
    let resolved = resolve_layer_position(ctx, &options, estimated_size);
    let shown = egui::Area::new(options.id)
        .order(options.order)
        .fixed_pos(resolved.pos)
        .show(ctx, |ui| {
            let panel = egui::Frame::new()
                .fill(options.fill)
                .stroke(options.stroke)
                .corner_radius(egui::CornerRadius::same(options.radius))
                .inner_margin(options.inner_margin)
                .show(ui, |ui| {
                    ui.set_width(options.width);
                    if let Some(min_height) = options.min_height {
                        ui.set_min_height(min_height);
                    }
                    match options.max_height {
                        Some(max_height) => {
                            ui.set_max_height(max_height);
                            egui::ScrollArea::vertical()
                                .max_height(max_height)
                                .auto_shrink([false, false])
                                .show(ui, add_contents)
                                .inner
                        }
                        None => add_contents(ui),
                    }
                });
            (panel.response.rect, panel.inner)
        });
    let (rect, action) = shown.inner;
    let dismiss_event =
        resolve_dismiss_event(ctx, rect, options.anchor_rect, options.dismiss_policy).map(
            |event| {
                options
                    .dismiss_filter
                    .map_or(event, |dismiss_filter| dismiss_filter(event))
            },
        );
    let should_close = dismiss_event.is_some_and(DismissLayerEvent::should_close);
    LayerOutput {
        action,
        should_close,
        dismiss_event,
        panel_rect: rect,
        resolved_placement: resolved.placement,
    }
}

pub fn clamp_layer_pos(ctx: &egui::Context, desired: Pos2, size: Vec2, margin: f32) -> Pos2 {
    let screen = ctx.content_rect();
    egui::pos2(
        desired
            .x
            .min(screen.right() - size.x - margin)
            .max(screen.left() + margin),
        desired
            .y
            .min(screen.bottom() - size.y - margin)
            .max(screen.top() + margin),
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ResolvedLayerPosition {
    pos: Pos2,
    placement: LayerResolvedPlacement,
}

fn resolve_layer_position(
    ctx: &egui::Context,
    options: &AnchoredLayerOptions,
    size: Vec2,
) -> ResolvedLayerPosition {
    let screen = ctx.content_rect();
    let (mut desired, placement) = match (options.placement, options.anchor_rect) {
        (LayerPlacement::Fixed(pos), _) => (
            pos,
            LayerResolvedPlacement {
                side: LayerSide::Bottom,
                align: LayerAlign::Start,
                flipped: false,
            },
        ),
        (LayerPlacement::BelowStart { offset }, Some(anchor)) => {
            let below = anchor.left_bottom() + offset;
            if below.y + size.y > screen.bottom() - options.margin {
                (
                    egui::pos2(anchor.left() + offset.x, anchor.top() - offset.y - size.y),
                    LayerResolvedPlacement {
                        side: LayerSide::Top,
                        align: LayerAlign::Start,
                        flipped: true,
                    },
                )
            } else {
                (
                    below,
                    LayerResolvedPlacement {
                        side: LayerSide::Bottom,
                        align: LayerAlign::Start,
                        flipped: false,
                    },
                )
            }
        }
        (LayerPlacement::BelowEnd { offset }, Some(anchor)) => {
            let below = egui::pos2(
                anchor.right() - size.x + offset.x,
                anchor.bottom() + offset.y,
            );
            if below.y + size.y > screen.bottom() - options.margin {
                (
                    egui::pos2(
                        anchor.right() - size.x + offset.x,
                        anchor.top() - offset.y - size.y,
                    ),
                    LayerResolvedPlacement {
                        side: LayerSide::Top,
                        align: LayerAlign::End,
                        flipped: true,
                    },
                )
            } else {
                (
                    below,
                    LayerResolvedPlacement {
                        side: LayerSide::Bottom,
                        align: LayerAlign::End,
                        flipped: false,
                    },
                )
            }
        }
        (LayerPlacement::AboveStart { offset }, Some(anchor)) => {
            let above = egui::pos2(anchor.left() + offset.x, anchor.top() - offset.y - size.y);
            if above.y < screen.top() + options.margin {
                (
                    anchor.left_bottom() + offset,
                    LayerResolvedPlacement {
                        side: LayerSide::Bottom,
                        align: LayerAlign::Start,
                        flipped: true,
                    },
                )
            } else {
                (
                    above,
                    LayerResolvedPlacement {
                        side: LayerSide::Top,
                        align: LayerAlign::Start,
                        flipped: false,
                    },
                )
            }
        }
        (LayerPlacement::RightStart { offset }, Some(anchor)) => {
            let right = anchor.right_top() + offset;
            if right.x + size.x > screen.right() - options.margin {
                (
                    egui::pos2(anchor.left() - offset.x - size.x, anchor.top() + offset.y),
                    LayerResolvedPlacement {
                        side: LayerSide::Left,
                        align: LayerAlign::Start,
                        flipped: true,
                    },
                )
            } else {
                (
                    right,
                    LayerResolvedPlacement {
                        side: LayerSide::Right,
                        align: LayerAlign::Start,
                        flipped: false,
                    },
                )
            }
        }
        (LayerPlacement::Centered { top_margin }, _) => (
            egui::pos2(
                screen.center().x - size.x * 0.5,
                (screen.center().y - size.y * 0.5).max(screen.top() + top_margin),
            ),
            LayerResolvedPlacement {
                side: LayerSide::Bottom,
                align: LayerAlign::Center,
                flipped: false,
            },
        ),
        (_, None) => (
            screen.center() - size * 0.5,
            LayerResolvedPlacement {
                side: LayerSide::Bottom,
                align: LayerAlign::Center,
                flipped: false,
            },
        ),
    };

    if desired.y < screen.top() + options.margin {
        desired.y = screen.top() + options.margin;
    }
    ResolvedLayerPosition {
        pos: clamp_layer_pos(ctx, desired, size, options.margin),
        placement,
    }
}

fn resolve_dismiss_event(
    ctx: &egui::Context,
    panel_rect: Rect,
    trigger_rect: Option<Rect>,
    policy: DismissPolicy,
) -> Option<DismissLayerEvent> {
    let escape = ctx.input(|input| input.key_pressed(egui::Key::Escape));
    let pointer = ctx.input(|input| {
        [egui::PointerButton::Primary, egui::PointerButton::Secondary]
            .into_iter()
            .find_map(|button| {
                input
                    .pointer
                    .button_pressed(button)
                    .then(|| {
                        input
                            .pointer
                            .press_origin()
                            .map(|position| (button, position))
                    })
                    .flatten()
            })
    });
    dismiss_event_for_interaction(policy, panel_rect, trigger_rect, escape, pointer)
}

pub fn dismiss_event_for_interaction(
    policy: DismissPolicy,
    panel_rect: Rect,
    trigger_rect: Option<Rect>,
    escape: bool,
    pointer: Option<(egui::PointerButton, Pos2)>,
) -> Option<DismissLayerEvent> {
    if matches!(
        policy,
        DismissPolicy::EscapeOnly | DismissPolicy::OutsideClickAndEscape
    ) && escape
    {
        return Some(DismissLayerEvent::escape_key_down());
    }
    if !matches!(
        policy,
        DismissPolicy::OutsideClick | DismissPolicy::OutsideClickAndEscape
    ) {
        return None;
    }
    pointer.and_then(|(button, position)| {
        if panel_rect.contains(position) {
            None
        } else if trigger_rect.is_some_and(|rect| rect.contains(position)) {
            Some(DismissLayerEvent::pointer_down(
                DismissLayerEventKind::PointerDownOnTrigger,
                button,
                position,
            ))
        } else {
            Some(DismissLayerEvent::pointer_down(
                DismissLayerEventKind::PointerDownOutside,
                button,
                position,
            ))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_layer_pos_keeps_popup_inside_screen() {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0))),
                ..Default::default()
            },
            |_| {},
        );

        let pos = clamp_layer_pos(&ctx, Pos2::new(500.0, 500.0), Vec2::new(100.0, 80.0), 8.0);

        assert!(pos.x <= 212.0);
        assert!(pos.y <= 152.0);
    }

    #[test]
    fn anchored_below_start_flips_above_when_bottom_would_overflow() {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0))),
                ..Default::default()
            },
            |_| {},
        );
        let anchor = Rect::from_min_size(Pos2::new(24.0, 204.0), Vec2::new(42.0, 28.0));
        let options = AnchoredLayerOptions::new("flip_test", 120.0)
            .anchor_rect(anchor)
            .placement(LayerPlacement::BelowStart {
                offset: Vec2::new(0.0, 6.0),
            })
            .max_height(120.0);

        let resolved = resolve_layer_position(&ctx, &options, Vec2::new(136.0, 120.0));

        assert!(resolved.pos.y < anchor.top());
        assert!(resolved.pos.x >= 8.0);
        assert_eq!(resolved.placement.side, LayerSide::Top);
        assert_eq!(resolved.placement.align, LayerAlign::Start);
        assert!(resolved.placement.flipped);
    }

    #[test]
    fn anchored_above_start_flips_below_when_top_would_overflow() {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0))),
                ..Default::default()
            },
            |_| {},
        );
        let anchor = Rect::from_min_size(Pos2::new(24.0, 16.0), Vec2::new(42.0, 28.0));
        let options = AnchoredLayerOptions::new("above_flip_test", 120.0)
            .anchor_rect(anchor)
            .placement(LayerPlacement::AboveStart {
                offset: Vec2::new(0.0, 6.0),
            })
            .max_height(80.0);

        let resolved = resolve_layer_position(&ctx, &options, Vec2::new(136.0, 80.0));

        assert!(resolved.pos.y > anchor.bottom());
        assert!(resolved.pos.x >= 8.0);
        assert_eq!(resolved.placement.side, LayerSide::Bottom);
        assert_eq!(resolved.placement.align, LayerAlign::Start);
        assert!(resolved.placement.flipped);
    }

    #[test]
    fn anchored_below_end_aligns_to_anchor_right_edge_and_clamps() {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0))),
                ..Default::default()
            },
            |_| {},
        );
        let anchor = Rect::from_min_size(Pos2::new(264.0, 32.0), Vec2::new(36.0, 28.0));
        let options = AnchoredLayerOptions::new("below_end_test", 180.0)
            .anchor_rect(anchor)
            .placement(LayerPlacement::BelowEnd {
                offset: Vec2::new(0.0, 6.0),
            })
            .max_height(120.0);

        let resolved = resolve_layer_position(&ctx, &options, Vec2::new(196.0, 120.0));

        assert!(resolved.pos.x <= anchor.right() - 196.0);
        assert!(resolved.pos.x >= 8.0);
        assert_eq!(resolved.pos.y, anchor.bottom() + 6.0);
        assert_eq!(resolved.placement.side, LayerSide::Bottom);
        assert_eq!(resolved.placement.align, LayerAlign::End);
        assert!(!resolved.placement.flipped);
    }

    #[test]
    fn anchored_right_start_reports_left_when_right_would_overflow() {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0))),
                ..Default::default()
            },
            |_| {},
        );
        let anchor = Rect::from_min_size(Pos2::new(260.0, 32.0), Vec2::new(40.0, 28.0));
        let options = AnchoredLayerOptions::new("right_flip_test", 160.0)
            .anchor_rect(anchor)
            .placement(LayerPlacement::RightStart {
                offset: Vec2::new(6.0, 0.0),
            })
            .max_height(80.0);

        let resolved = resolve_layer_position(&ctx, &options, Vec2::new(176.0, 80.0));

        assert!(resolved.pos.x < anchor.left());
        assert_eq!(resolved.placement.side, LayerSide::Left);
        assert_eq!(resolved.placement.align, LayerAlign::Start);
        assert!(resolved.placement.flipped);
    }

    #[test]
    fn dismiss_event_reports_escape_pointer_outside_and_trigger_press() {
        let panel = Rect::from_min_size(Pos2::new(20.0, 20.0), Vec2::new(100.0, 80.0));
        let trigger = Rect::from_min_size(Pos2::new(20.0, 0.0), Vec2::new(80.0, 18.0));

        let escape = dismiss_event_for_interaction(
            DismissPolicy::OutsideClickAndEscape,
            panel,
            Some(trigger),
            true,
            None,
        )
        .expect("escape should dismiss");
        assert_eq!(escape.kind, DismissLayerEventKind::EscapeKeyDown);
        assert_eq!(escape.kind.as_str(), "escapeKeyDown");
        assert!(escape.should_close());

        let outside = dismiss_event_for_interaction(
            DismissPolicy::OutsideClickAndEscape,
            panel,
            Some(trigger),
            false,
            Some((egui::PointerButton::Secondary, Pos2::new(180.0, 40.0))),
        )
        .expect("outside pointer should dismiss");
        assert_eq!(outside.kind, DismissLayerEventKind::PointerDownOutside);
        assert_eq!(outside.pointer_button, Some(egui::PointerButton::Secondary));
        assert!(outside.kind.is_interact_outside());

        let trigger_press = dismiss_event_for_interaction(
            DismissPolicy::OutsideClickAndEscape,
            panel,
            Some(trigger),
            false,
            Some((egui::PointerButton::Primary, Pos2::new(30.0, 10.0))),
        )
        .expect("trigger press should surface as a dismiss event");
        assert_eq!(
            trigger_press.kind,
            DismissLayerEventKind::PointerDownOnTrigger
        );
        assert!(trigger_press.should_close());
    }

    #[test]
    fn dismiss_event_respects_policy_and_prevent_default() {
        let panel = Rect::from_min_size(Pos2::new(20.0, 20.0), Vec2::new(100.0, 80.0));
        let pointer = Some((egui::PointerButton::Primary, Pos2::new(180.0, 40.0)));

        assert_eq!(
            dismiss_event_for_interaction(DismissPolicy::EscapeOnly, panel, None, false, pointer),
            None
        );
        assert_eq!(
            dismiss_event_for_interaction(DismissPolicy::OutsideClick, panel, None, true, None),
            None
        );

        let event =
            dismiss_event_for_interaction(DismissPolicy::OutsideClick, panel, None, false, pointer)
                .expect("outside pointer should produce an event")
                .prevent_default();
        assert!(event.default_prevented);
        assert!(!event.should_close());
    }

    #[test]
    fn nested_dismiss_order_closes_topmost_unprevented_layer_first() {
        let outside = DismissLayerEvent::pointer_down(
            DismissLayerEventKind::PointerDownOutside,
            egui::PointerButton::Primary,
            Pos2::new(200.0, 40.0),
        );
        let order = nested_dismiss_close_order([
            DismissLayerCandidate {
                layer_depth: 0,
                event: outside,
            },
            DismissLayerCandidate {
                layer_depth: 2,
                event: outside.prevent_default(),
            },
            DismissLayerCandidate {
                layer_depth: 1,
                event: outside,
            },
        ]);

        assert_eq!(order.len(), 2);
        assert_eq!(order[0].layer_depth, 1);
        assert_eq!(order[1].layer_depth, 0);
    }
}
