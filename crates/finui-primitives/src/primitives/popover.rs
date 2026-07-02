use std::hash::Hash;

use eframe::egui::{self, Align2, FontId, Pos2, Rect, Response, Sense, Shape, Stroke, Vec2};

use super::{
    DropdownMenuAlign, DropdownMenuDataState, DropdownMenuSide, PrimitiveLayerOptions,
    PrimitiveLayerOutput, PrimitiveTheme, dropdown_menu_align_from_layer_align,
    dropdown_menu_placement_parts, dropdown_menu_side_from_layer_side,
    primitive_dismissable_layer_options, primitive_mounted_content_text_colors,
    show_primitive_layer,
};
use crate::{DismissPolicy, LayerPlacement};

pub type PopoverDataState = DropdownMenuDataState;
pub type PopoverSide = DropdownMenuSide;
pub type PopoverAlign = DropdownMenuAlign;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopoverRootOptions {
    pub open: bool,
    pub default_open: Option<bool>,
    pub modal: bool,
}

impl Default for PopoverRootOptions {
    fn default() -> Self {
        Self {
            open: false,
            default_open: None,
            modal: false,
        }
    }
}

impl PopoverRootOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PopoverRootOutput {
    pub open: bool,
    pub default_open: Option<bool>,
    pub modal: bool,
    pub data_state: PopoverDataState,
}

pub fn primitive_popover_root_output(options: PopoverRootOptions) -> PopoverRootOutput {
    PopoverRootOutput {
        open: options.open,
        default_open: options.default_open,
        modal: options.modal,
        data_state: if options.open {
            PopoverDataState::Open
        } else {
            PopoverDataState::Closed
        },
    }
}

pub fn popover_apply_open(current: &mut bool, next: bool, _options: &PopoverRootOptions) -> bool {
    if *current == next {
        return false;
    }
    *current = next;
    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopoverPortalOptions {
    pub force_mount: bool,
    pub container: Option<String>,
}

impl Default for PopoverPortalOptions {
    fn default() -> Self {
        Self {
            force_mount: false,
            container: None,
        }
    }
}

impl PopoverPortalOptions {
    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn container(mut self, container: impl Into<String>) -> Self {
        self.container = Some(container.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopoverPortalOutput {
    pub force_mount: bool,
    pub container: Option<String>,
}

pub fn primitive_popover_portal_output(options: PopoverPortalOptions) -> PopoverPortalOutput {
    PopoverPortalOutput {
        force_mount: options.force_mount,
        container: options.container,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopoverArrowSide {
    Top,
    Bottom,
    Left,
    Right,
}

pub struct PopoverOptions {
    pub id: egui::Id,
    pub trigger_rect: Rect,
    pub width: f32,
    pub placement: LayerPlacement,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
}

impl PopoverOptions {
    pub fn anchored(
        id: impl Hash,
        trigger_rect: Rect,
        width: f32,
        placement: LayerPlacement,
    ) -> Self {
        Self {
            id: egui::Id::new(id),
            trigger_rect,
            width,
            placement,
            inner_margin: egui::Margin::same(12),
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn inner_margin(mut self, inner_margin: egui::Margin) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverTriggerOptions {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub theme: PrimitiveTheme,
}

impl Default for PopoverTriggerOptions {
    fn default() -> Self {
        Self {
            width: 112.0,
            height: 32.0,
            open: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl PopoverTriggerOptions {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverAnchorOptions {
    pub size: Vec2,
    pub visible: bool,
    pub theme: PrimitiveTheme,
}

impl Default for PopoverAnchorOptions {
    fn default() -> Self {
        Self {
            size: Vec2::new(20.0, 32.0),
            visible: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl PopoverAnchorOptions {
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverAnchorOutput {
    pub rect: Rect,
}

pub fn primitive_popover_anchor_output(rect: Rect) -> PopoverAnchorOutput {
    PopoverAnchorOutput { rect }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverContentOptions {
    pub width: f32,
    pub placement: LayerPlacement,
    pub inner_margin: egui::Margin,
    pub open: bool,
    pub force_mount: bool,
    pub side: PopoverSide,
    pub align: PopoverAlign,
    pub data_state: PopoverDataState,
    pub theme: PrimitiveTheme,
}

impl PopoverContentOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self.data_state = if open {
            PopoverDataState::Open
        } else {
            PopoverDataState::Closed
        };
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn side_align(mut self, side: PopoverSide, align: PopoverAlign) -> Self {
        self.side = side;
        self.align = align;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverContentOutput {
    pub width: f32,
    pub open: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub side: PopoverSide,
    pub align: PopoverAlign,
    pub data_state: PopoverDataState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverCloseOptions {
    pub width: f32,
    pub height: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for PopoverCloseOptions {
    fn default() -> Self {
        Self {
            width: 84.0,
            height: 30.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl PopoverCloseOptions {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.enabled = !disabled;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

pub struct PopoverOutput<T> {
    pub action: Option<T>,
    pub should_close: bool,
    pub content_rect: Rect,
    pub side: PopoverSide,
    pub align: PopoverAlign,
}

pub fn primitive_popover_trigger(
    ui: &mut egui::Ui,
    label: &str,
    options: PopoverTriggerOptions,
) -> Response {
    let root = primitive_popover_root_output(PopoverRootOptions::default().open(options.open));
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(options.width, options.height), Sense::click());
    let fill = if root.data_state == PopoverDataState::Open || response.hovered() {
        options.theme.item_hover_fill
    } else {
        options.theme.content_fill
    };
    ui.painter().rect(
        rect,
        options.theme.row_radius,
        fill,
        Stroke::new(1.0, options.theme.content_stroke.color),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.left_center() + Vec2::new(10.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        options.theme.text,
    );
    response
}

pub fn primitive_popover_anchor(
    ui: &mut egui::Ui,
    options: PopoverAnchorOptions,
) -> PopoverAnchorOutput {
    let (rect, _) = ui.allocate_exact_size(options.size, Sense::hover());
    let output = primitive_popover_anchor_output(rect);
    if options.visible {
        let center = output.rect.center();
        let stroke = Stroke::new(1.0, options.theme.muted_text);
        ui.painter().rect(
            output.rect.shrink(5.0),
            options.theme.row_radius,
            egui::Color32::TRANSPARENT,
            Stroke::new(1.0, options.theme.content_stroke.color),
            egui::StrokeKind::Inside,
        );
        ui.painter().line_segment(
            [
                egui::pos2(center.x - 4.0, center.y),
                egui::pos2(center.x + 4.0, center.y),
            ],
            stroke,
        );
        ui.painter().line_segment(
            [
                egui::pos2(center.x, center.y - 4.0),
                egui::pos2(center.x, center.y + 4.0),
            ],
            stroke,
        );
    }
    output
}

pub fn primitive_popover_content_options(options: &PopoverOptions) -> PopoverContentOptions {
    let (side, align) = dropdown_menu_placement_parts(options.placement);
    PopoverContentOptions {
        width: options.width,
        placement: options.placement,
        inner_margin: options.inner_margin,
        open: true,
        force_mount: false,
        side,
        align,
        data_state: PopoverDataState::Open,
        theme: options.theme,
    }
}

pub fn primitive_popover_content_output(options: PopoverContentOptions) -> PopoverContentOutput {
    PopoverContentOutput {
        width: options.width,
        open: options.open,
        force_mount: options.force_mount,
        mounted: options.open || options.force_mount,
        side: options.side,
        align: options.align,
        data_state: options.data_state,
    }
}

pub fn primitive_popover_layer_options(options: &PopoverOptions) -> PrimitiveLayerOptions {
    let content = primitive_popover_content_options(options);
    let mut layer = PrimitiveLayerOptions::new(options.id, content.width)
        .anchor_rect(options.trigger_rect)
        .placement(content.placement)
        .order(egui::Order::Tooltip)
        .min_height(96.0)
        .inner_margin(content.inner_margin);
    layer.theme = content.theme;
    primitive_dismissable_layer_options(layer, DismissPolicy::OutsideClickAndEscape)
}

pub fn primitive_popover_content(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    theme: PrimitiveTheme,
) {
    let output = PopoverContentOutput {
        width: 0.0,
        open: true,
        force_mount: false,
        mounted: true,
        side: PopoverSide::Bottom,
        align: PopoverAlign::Start,
        data_state: PopoverDataState::Open,
    };
    let (title_color, description_color) = popover_content_text_colors(&output, theme);
    primitive_popover_content_text(ui, title, description, title_color, description_color);
}

pub fn popover_content_text_colors(
    output: &PopoverContentOutput,
    theme: PrimitiveTheme,
) -> (egui::Color32, egui::Color32) {
    let colors =
        primitive_mounted_content_text_colors(output.data_state != PopoverDataState::Closed, theme);
    (colors.title, colors.detail)
}

pub fn primitive_popover_content_with_options(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    options: PopoverContentOptions,
) {
    let output = primitive_popover_content_output(options);
    let (title_color, description_color) = popover_content_text_colors(&output, options.theme);
    primitive_popover_content_text(ui, title, description, title_color, description_color);
}

fn primitive_popover_content_text(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    title_color: egui::Color32,
    description_color: egui::Color32,
) {
    ui.label(
        egui::RichText::new(title)
            .font(crate::scaled_proportional_font(ui, 13.0))
            .color(title_color),
    );
    ui.label(
        egui::RichText::new(description)
            .font(crate::scaled_proportional_font(ui, 12.0))
            .color(description_color),
    );
}

pub fn primitive_popover_close(
    ui: &mut egui::Ui,
    label: &str,
    options: PopoverCloseOptions,
) -> Response {
    let sense = if options.enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(options.width, options.height), sense);
    let response = if options.enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    };
    let fill = if response.hovered() && options.enabled {
        options.theme.item_hover_fill
    } else {
        options.theme.content_fill
    };
    let text_color = if options.enabled {
        options.theme.text
    } else {
        options.theme.disabled_text
    };
    ui.painter().rect(
        rect,
        options.theme.row_radius,
        fill,
        options.theme.content_stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        crate::scaled_proportional_font(ui, 12.0),
        text_color,
    );
    response
}

pub fn show_popover<T>(
    ctx: &egui::Context,
    options: PopoverOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> Option<T>,
) -> PopoverOutput<T> {
    let layer = primitive_popover_layer_options(&options);
    let output: PrimitiveLayerOutput<T> = show_primitive_layer(ctx, layer, add_contents);
    PopoverOutput {
        action: output.action,
        should_close: output.should_close,
        content_rect: output.content_rect,
        side: dropdown_menu_side_from_layer_side(output.resolved_placement.side),
        align: dropdown_menu_align_from_layer_align(output.resolved_placement.align),
    }
}

pub fn popover_arrow_side(trigger_rect: Rect, content_rect: Rect) -> PopoverArrowSide {
    if content_rect.bottom() <= trigger_rect.top() {
        PopoverArrowSide::Bottom
    } else if content_rect.top() >= trigger_rect.bottom() {
        PopoverArrowSide::Top
    } else if content_rect.right() <= trigger_rect.left() {
        PopoverArrowSide::Right
    } else if content_rect.left() >= trigger_rect.right() {
        PopoverArrowSide::Left
    } else {
        PopoverArrowSide::Top
    }
}

pub fn popover_arrow_points(trigger_rect: Rect, content_rect: Rect, size: f32) -> [Pos2; 3] {
    let size = size.max(2.0);
    let side = popover_arrow_side(trigger_rect, content_rect);
    let x = trigger_rect
        .center()
        .x
        .clamp(content_rect.left() + size, content_rect.right() - size);
    let y = trigger_rect
        .center()
        .y
        .clamp(content_rect.top() + size, content_rect.bottom() - size);
    match side {
        PopoverArrowSide::Top => [
            egui::pos2(x, content_rect.top()),
            egui::pos2(x - size, content_rect.top() + size),
            egui::pos2(x + size, content_rect.top() + size),
        ],
        PopoverArrowSide::Bottom => [
            egui::pos2(x, content_rect.bottom()),
            egui::pos2(x - size, content_rect.bottom() - size),
            egui::pos2(x + size, content_rect.bottom() - size),
        ],
        PopoverArrowSide::Left => [
            egui::pos2(content_rect.left(), y),
            egui::pos2(content_rect.left() + size, y - size),
            egui::pos2(content_rect.left() + size, y + size),
        ],
        PopoverArrowSide::Right => [
            egui::pos2(content_rect.right(), y),
            egui::pos2(content_rect.right() - size, y - size),
            egui::pos2(content_rect.right() - size, y + size),
        ],
    }
}

pub fn primitive_popover_arrow(
    ui: &egui::Ui,
    trigger_rect: Rect,
    content_rect: Rect,
    size: f32,
    theme: PrimitiveTheme,
) {
    let points = popover_arrow_points(trigger_rect, content_rect, size);
    ui.painter().add(Shape::convex_polygon(
        points.to_vec(),
        theme.content_fill,
        Stroke::new(1.0, theme.content_stroke.color),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popover_arrow_side_tracks_content_relation_to_trigger() {
        let trigger = Rect::from_min_size(egui::pos2(20.0, 20.0), egui::vec2(40.0, 24.0));
        let below = Rect::from_min_size(egui::pos2(12.0, 52.0), egui::vec2(120.0, 80.0));
        let above = Rect::from_min_size(egui::pos2(12.0, 0.0), egui::vec2(120.0, 16.0));
        let right = Rect::from_min_size(egui::pos2(72.0, 16.0), egui::vec2(120.0, 80.0));

        assert_eq!(popover_arrow_side(trigger, below), PopoverArrowSide::Top);
        assert_eq!(popover_arrow_side(trigger, above), PopoverArrowSide::Bottom);
        assert_eq!(popover_arrow_side(trigger, right), PopoverArrowSide::Left);
    }

    #[test]
    fn popover_arrow_points_clamp_to_content_bounds() {
        let trigger = Rect::from_min_size(egui::pos2(0.0, 20.0), egui::vec2(10.0, 24.0));
        let content = Rect::from_min_size(egui::pos2(30.0, 52.0), egui::vec2(120.0, 80.0));

        let points = popover_arrow_points(trigger, content, 8.0);

        assert!(points.iter().all(|point| point.x >= content.left()));
        assert!(points.iter().all(|point| point.x <= content.right()));
    }

    #[test]
    fn popover_options_preserve_content_contract() {
        let options = PopoverOptions::anchored(
            "popover_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::RightStart {
                offset: egui::vec2(6.0, 0.0),
            },
        )
        .inner_margin(egui::Margin::symmetric(4, 5));

        let content = primitive_popover_content_options(&options);

        assert_eq!(content.width, 180.0);
        assert_eq!(content.inner_margin.left, 4);
        assert_eq!(content.inner_margin.top, 5);
        assert_eq!(content.side, PopoverSide::Right);
        assert_eq!(content.align, PopoverAlign::Start);
        assert_eq!(content.data_state, PopoverDataState::Open);
    }

    #[test]
    fn popover_layer_options_use_dismissable_policy() {
        let options = PopoverOptions::anchored(
            "popover_layer_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        );

        let layer = primitive_popover_layer_options(&options);

        assert_eq!(layer.dismiss_policy, DismissPolicy::OutsideClickAndEscape);
        assert_eq!(layer.order, egui::Order::Tooltip);
        assert_eq!(layer.min_height, Some(96.0));
        assert_eq!(layer.anchor_rect, Some(options.trigger_rect));
    }

    #[test]
    fn popover_trigger_options_preserve_root_state() {
        let options = PopoverTriggerOptions::default().size(96.0, 28.0).open(true);

        assert_eq!(options.width, 96.0);
        assert_eq!(options.height, 28.0);
        assert!(options.open);
    }

    #[test]
    fn popover_root_output_preserves_radix_contract() {
        let output = primitive_popover_root_output(
            PopoverRootOptions::default()
                .open(true)
                .default_open(false)
                .modal(true),
        );

        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert!(output.modal);
        assert_eq!(output.data_state, PopoverDataState::Open);
        assert_eq!(output.data_state.as_str(), "open");
    }

    #[test]
    fn popover_apply_open_respects_noop_state() {
        let mut open = false;

        assert!(popover_apply_open(
            &mut open,
            true,
            &PopoverRootOptions::default()
        ));
        assert!(open);
        assert!(!popover_apply_open(
            &mut open,
            true,
            &PopoverRootOptions::default().open(true)
        ));
        assert!(popover_apply_open(
            &mut open,
            false,
            &PopoverRootOptions::default().open(true)
        ));
        assert!(!open);
    }

    #[test]
    fn popover_portal_output_preserves_force_mount_and_container() {
        let output = primitive_popover_portal_output(
            PopoverPortalOptions::default()
                .force_mount(true)
                .container("popover-layer"),
        );

        assert!(output.force_mount);
        assert_eq!(output.container.as_deref(), Some("popover-layer"));
    }

    #[test]
    fn popover_content_output_preserves_state_side_align_and_mount() {
        let options = PopoverOptions::anchored(
            "popover_content_output_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        );
        let content = primitive_popover_content_options(&options)
            .open(false)
            .force_mount(true)
            .side_align(PopoverSide::Bottom, PopoverAlign::Center);
        let output = primitive_popover_content_output(content);

        assert_eq!(output.width, 180.0);
        assert!(!output.open);
        assert!(output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.side, PopoverSide::Bottom);
        assert_eq!(output.side.as_str(), "bottom");
        assert_eq!(output.align, PopoverAlign::Center);
        assert_eq!(output.align.as_str(), "center");
        assert_eq!(output.data_state, PopoverDataState::Closed);
        assert_eq!(output.data_state.as_str(), "closed");
    }

    #[test]
    fn popover_output_reports_collision_resolved_side_align() {
        let ctx = egui::Context::default();
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::pos2(0.0, 0.0),
                    egui::vec2(320.0, 240.0),
                )),
                ..Default::default()
            },
            |_| {},
        );
        let options = PopoverOptions::anchored(
            "popover_collision_output_test",
            Rect::from_min_size(egui::pos2(24.0, 204.0), egui::vec2(42.0, 28.0)),
            120.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        );

        let output = show_popover(&ctx, options, |ui| {
            primitive_popover_content(ui, "Title", "Description", PrimitiveTheme::default());
            None::<()>
        });

        assert_eq!(output.side, PopoverSide::Top);
        assert_eq!(output.align, PopoverAlign::Start);
    }

    #[test]
    fn popover_content_text_colors_mute_closed_force_mounted_content() {
        let theme = PrimitiveTheme::default();
        let open = PopoverContentOutput {
            width: 180.0,
            open: true,
            force_mount: true,
            mounted: true,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Start,
            data_state: PopoverDataState::Open,
        };
        let closed = PopoverContentOutput {
            data_state: PopoverDataState::Closed,
            open: false,
            ..open
        };

        assert_eq!(
            popover_content_text_colors(&open, theme),
            (theme.text, theme.muted_text)
        );
        assert_eq!(
            popover_content_text_colors(&closed, theme),
            (theme.muted_text, theme.disabled_text)
        );
    }

    #[test]
    fn popover_anchor_output_preserves_anchor_rect() {
        let rect = Rect::from_min_size(egui::pos2(12.0, 18.0), egui::vec2(20.0, 32.0));
        let output = primitive_popover_anchor_output(rect);

        assert_eq!(output.rect, rect);
    }

    #[test]
    fn popover_anchor_options_preserve_size_visibility_and_theme() {
        let theme = PrimitiveTheme::default();
        let options = PopoverAnchorOptions::default()
            .size(Vec2::new(16.0, 24.0))
            .visible(false)
            .theme(theme);

        assert_eq!(options.size, Vec2::new(16.0, 24.0));
        assert!(!options.visible);
        assert_eq!(options.theme, theme);
    }

    #[test]
    fn popover_close_options_preserve_action_button_state() {
        let theme = PrimitiveTheme::default();
        let options = PopoverCloseOptions::default()
            .size(72.0, 26.0)
            .disabled(true)
            .theme(theme);

        assert_eq!(options.width, 72.0);
        assert_eq!(options.height, 26.0);
        assert!(!options.enabled);
        assert_eq!(options.theme, theme);
    }
}
