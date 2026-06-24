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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DismissPolicy {
    None,
    OutsideClick,
    OutsideClickAndEscape,
    EscapeOnly,
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

    pub fn order(mut self, order: egui::Order) -> Self {
        self.order = order;
        self
    }
}

pub struct LayerOutput<T> {
    pub action: Option<T>,
    pub should_close: bool,
    pub panel_rect: Rect,
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
    let pos = resolve_layer_pos(ctx, &options, estimated_size);
    let shown = egui::Area::new(options.id)
        .order(options.order)
        .fixed_pos(pos)
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
    let should_close = should_dismiss(ctx, rect, options.anchor_rect, options.dismiss_policy);
    LayerOutput {
        action,
        should_close,
        panel_rect: rect,
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

fn resolve_layer_pos(ctx: &egui::Context, options: &AnchoredLayerOptions, size: Vec2) -> Pos2 {
    let screen = ctx.content_rect();
    let mut desired = match (options.placement, options.anchor_rect) {
        (LayerPlacement::Fixed(pos), _) => pos,
        (LayerPlacement::BelowStart { offset }, Some(anchor)) => {
            let below = anchor.left_bottom() + offset;
            if below.y + size.y > screen.bottom() - options.margin {
                egui::pos2(anchor.left() + offset.x, anchor.top() - offset.y - size.y)
            } else {
                below
            }
        }
        (LayerPlacement::BelowEnd { offset }, Some(anchor)) => {
            let below = egui::pos2(
                anchor.right() - size.x + offset.x,
                anchor.bottom() + offset.y,
            );
            if below.y + size.y > screen.bottom() - options.margin {
                egui::pos2(
                    anchor.right() - size.x + offset.x,
                    anchor.top() - offset.y - size.y,
                )
            } else {
                below
            }
        }
        (LayerPlacement::AboveStart { offset }, Some(anchor)) => {
            let above = egui::pos2(anchor.left() + offset.x, anchor.top() - offset.y - size.y);
            if above.y < screen.top() + options.margin {
                anchor.left_bottom() + offset
            } else {
                above
            }
        }
        (LayerPlacement::RightStart { offset }, Some(anchor)) => {
            let right = anchor.right_top() + offset;
            if right.x + size.x > screen.right() - options.margin {
                egui::pos2(anchor.left() - offset.x - size.x, anchor.top() + offset.y)
            } else {
                right
            }
        }
        (LayerPlacement::Centered { top_margin }, _) => egui::pos2(
            screen.center().x - size.x * 0.5,
            (screen.center().y - size.y * 0.5).max(screen.top() + top_margin),
        ),
        (_, None) => screen.center() - size * 0.5,
    };

    if desired.y < screen.top() + options.margin {
        desired.y = screen.top() + options.margin;
    }
    clamp_layer_pos(ctx, desired, size, options.margin)
}

fn should_dismiss(
    ctx: &egui::Context,
    panel_rect: Rect,
    trigger_rect: Option<Rect>,
    policy: DismissPolicy,
) -> bool {
    let escape = ctx.input(|input| input.key_pressed(egui::Key::Escape));
    if matches!(
        policy,
        DismissPolicy::EscapeOnly | DismissPolicy::OutsideClickAndEscape
    ) && escape
    {
        return true;
    }
    if !matches!(
        policy,
        DismissPolicy::OutsideClick | DismissPolicy::OutsideClickAndEscape
    ) {
        return false;
    }
    ctx.input(|input| {
        if input.pointer.button_pressed(egui::PointerButton::Primary)
            || input.pointer.button_pressed(egui::PointerButton::Secondary)
        {
            input.pointer.press_origin().is_some_and(|pos| {
                !panel_rect.contains(pos) && !trigger_rect.is_some_and(|rect| rect.contains(pos))
            })
        } else {
            false
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

        let pos = resolve_layer_pos(&ctx, &options, Vec2::new(136.0, 120.0));

        assert!(pos.y < anchor.top());
        assert!(pos.x >= 8.0);
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

        let pos = resolve_layer_pos(&ctx, &options, Vec2::new(136.0, 80.0));

        assert!(pos.y > anchor.bottom());
        assert!(pos.x >= 8.0);
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

        let pos = resolve_layer_pos(&ctx, &options, Vec2::new(196.0, 120.0));

        assert!(pos.x <= anchor.right() - 196.0);
        assert!(pos.x >= 8.0);
        assert_eq!(pos.y, anchor.bottom() + 6.0);
    }
}
