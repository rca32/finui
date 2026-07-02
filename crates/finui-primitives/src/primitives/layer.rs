use std::hash::Hash;

use eframe::egui::{self, Rect};

use super::theme::PrimitiveTheme;
use crate::{
    AnchoredLayerOptions, DismissLayerEvent, DismissLayerFilter, DismissPolicy, LayerAlign,
    LayerOutput, LayerPlacement, LayerResolvedPlacement, LayerSide, show_anchored_layer,
};

pub struct PrimitiveLayerOptions {
    pub id: egui::Id,
    pub anchor_rect: Option<Rect>,
    pub placement: LayerPlacement,
    pub width: f32,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub inner_margin: egui::Margin,
    pub order: egui::Order,
    pub dismiss_policy: DismissPolicy,
    pub dismiss_filter: Option<DismissLayerFilter>,
    pub theme: PrimitiveTheme,
}

impl PrimitiveLayerOptions {
    pub fn new(id: impl Hash, width: f32) -> Self {
        Self {
            id: egui::Id::new(id),
            anchor_rect: None,
            placement: LayerPlacement::Fixed(egui::Pos2::ZERO),
            width,
            min_height: None,
            max_height: None,
            inner_margin: egui::Margin::same(8),
            order: egui::Order::Foreground,
            dismiss_policy: DismissPolicy::OutsideClickAndEscape,
            dismiss_filter: None,
            theme: PrimitiveTheme::default(),
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

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = Some(min_height);
        self
    }

    pub fn inner_margin(mut self, inner_margin: egui::Margin) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    pub fn order(mut self, order: egui::Order) -> Self {
        self.order = order;
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
}

pub struct PrimitiveLayerOutput<T> {
    pub action: Option<T>,
    pub should_close: bool,
    pub dismiss_event: Option<DismissLayerEvent>,
    pub content_rect: Rect,
    pub resolved_placement: LayerResolvedPlacement,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveLayerAnimationOutput {
    pub open_progress: f32,
    pub close_progress: f32,
    pub transform_origin: egui::Pos2,
    pub data_side: &'static str,
    pub data_align: &'static str,
    pub collision_flipped: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitivePortalOutput {
    pub content_rect: Rect,
}

pub fn primitive_portal_output(content_rect: Rect) -> PrimitivePortalOutput {
    PrimitivePortalOutput { content_rect }
}

pub fn primitive_dismissable_layer_options(
    options: PrimitiveLayerOptions,
    dismiss_policy: DismissPolicy,
) -> PrimitiveLayerOptions {
    options.dismiss_policy(dismiss_policy)
}

pub fn primitive_layer_animation_output(
    open: bool,
    placement: LayerResolvedPlacement,
    progress: f32,
) -> PrimitiveLayerAnimationOutput {
    let open_progress = if open { progress.clamp(0.0, 1.0) } else { 0.0 };
    PrimitiveLayerAnimationOutput {
        open_progress,
        close_progress: if open { 1.0 - open_progress } else { 1.0 },
        transform_origin: primitive_layer_transform_origin(placement.side, placement.align),
        data_side: placement.side.as_str(),
        data_align: placement.align.as_str(),
        collision_flipped: placement.flipped,
    }
}

pub fn primitive_layer_transform_origin(side: LayerSide, align: LayerAlign) -> egui::Pos2 {
    let align_fraction = match align {
        LayerAlign::Start => 0.0,
        LayerAlign::Center => 0.5,
        LayerAlign::End => 1.0,
    };
    match side {
        LayerSide::Top => egui::pos2(align_fraction, 1.0),
        LayerSide::Bottom => egui::pos2(align_fraction, 0.0),
        LayerSide::Left => egui::pos2(1.0, align_fraction),
        LayerSide::Right => egui::pos2(0.0, align_fraction),
    }
}

pub fn show_primitive_layer<T>(
    ctx: &egui::Context,
    options: PrimitiveLayerOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> Option<T>,
) -> PrimitiveLayerOutput<T> {
    let mut layer = AnchoredLayerOptions::new(options.id, options.width)
        .placement(options.placement)
        .inner_margin(options.inner_margin)
        .order(options.order)
        .dismiss_policy(options.dismiss_policy);
    if let Some(dismiss_filter) = options.dismiss_filter {
        layer = layer.dismiss_filter(dismiss_filter);
    }
    if let Some(anchor) = options.anchor_rect {
        layer = layer.anchor_rect(anchor);
    }
    if let Some(min_height) = options.min_height {
        layer = layer.min_height(min_height);
    }
    if let Some(max_height) = options.max_height {
        layer = layer.max_height(max_height);
    }
    layer.fill = options.theme.content_fill;
    layer.stroke = options.theme.content_stroke;
    layer.radius = options.theme.radius;

    let output: LayerOutput<T> = show_anchored_layer(ctx, layer, add_contents);
    let portal = primitive_portal_output(output.panel_rect);
    PrimitiveLayerOutput {
        action: output.action,
        should_close: output.should_close,
        dismiss_event: output.dismiss_event,
        content_rect: portal.content_rect,
        resolved_placement: output.resolved_placement,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portal_output_preserves_content_rect() {
        let rect = Rect::from_min_size(egui::pos2(12.0, 20.0), egui::vec2(120.0, 80.0));
        let output = primitive_portal_output(rect);

        assert_eq!(output.content_rect, rect);
    }

    #[test]
    fn dismissable_layer_options_preserve_policy() {
        let options = PrimitiveLayerOptions::new("layer-options-test", 180.0);
        let options =
            primitive_dismissable_layer_options(options, DismissPolicy::OutsideClickAndEscape);

        assert_eq!(options.dismiss_policy, DismissPolicy::OutsideClickAndEscape);
    }

    #[test]
    fn layer_options_preserve_dismiss_filter_contract() {
        fn prevent(event: DismissLayerEvent) -> DismissLayerEvent {
            event.prevent_default()
        }

        let options =
            PrimitiveLayerOptions::new("layer-filter-test", 180.0).dismiss_filter(prevent);

        assert!(options.dismiss_filter.is_some());
    }

    #[test]
    fn layer_animation_output_reports_progress_origin_and_collision_parts() {
        let output = primitive_layer_animation_output(
            true,
            LayerResolvedPlacement {
                side: LayerSide::Top,
                align: LayerAlign::End,
                flipped: true,
            },
            0.35,
        );
        let closed = primitive_layer_animation_output(
            false,
            LayerResolvedPlacement {
                side: LayerSide::Right,
                align: LayerAlign::Center,
                flipped: false,
            },
            0.75,
        );

        assert_eq!(output.open_progress, 0.35);
        assert_eq!(output.close_progress, 0.65);
        assert_eq!(output.transform_origin, egui::pos2(1.0, 1.0));
        assert_eq!(output.data_side, "top");
        assert_eq!(output.data_align, "end");
        assert!(output.collision_flipped);
        assert_eq!(closed.open_progress, 0.0);
        assert_eq!(closed.close_progress, 1.0);
        assert_eq!(closed.transform_origin, egui::pos2(0.0, 0.5));
    }
}
