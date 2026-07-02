use std::hash::Hash;

use eframe::egui::{self, Align2, FontId, Rect, Response, Sense, Stroke, Vec2};

use super::{
    DropdownMenuAlign, DropdownMenuDataState, DropdownMenuSide, PopoverArrowSide,
    PrimitiveLayerOptions, PrimitiveLayerOutput, PrimitiveTheme,
    dropdown_menu_align_from_layer_align, dropdown_menu_placement_parts,
    dropdown_menu_side_from_layer_side, popover_arrow_side, primitive_mounted_content_policy,
    primitive_popover_arrow, show_primitive_layer,
};
use crate::{DismissPolicy, LayerPlacement};

pub type HoverCardDataState = DropdownMenuDataState;
pub type HoverCardSide = DropdownMenuSide;
pub type HoverCardAlign = DropdownMenuAlign;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoverCardRootOptions {
    pub open: bool,
    pub default_open: Option<bool>,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
}

impl Default for HoverCardRootOptions {
    fn default() -> Self {
        Self {
            open: false,
            default_open: None,
            open_delay_ms: 700,
            close_delay_ms: 300,
        }
    }
}

impl HoverCardRootOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    pub fn open_delay_ms(mut self, open_delay_ms: u64) -> Self {
        self.open_delay_ms = open_delay_ms;
        self
    }

    pub fn close_delay_ms(mut self, close_delay_ms: u64) -> Self {
        self.close_delay_ms = close_delay_ms;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoverCardRootOutput {
    pub open: bool,
    pub default_open: Option<bool>,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
    pub data_state: HoverCardDataState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverCardDelayEvent {
    PointerEnter { hovered_ms: u64 },
    PointerLeave { elapsed_leave_ms: u64 },
    Focus,
    Blur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HoverCardDelayOutput {
    pub open: bool,
    pub data_state: HoverCardDataState,
    pub next_open_delay_ms: Option<u64>,
    pub close_after_ms: Option<u64>,
}

pub fn primitive_hover_card_root_output(options: HoverCardRootOptions) -> HoverCardRootOutput {
    HoverCardRootOutput {
        open: options.open,
        default_open: options.default_open,
        open_delay_ms: options.open_delay_ms,
        close_delay_ms: options.close_delay_ms,
        data_state: if options.open {
            HoverCardDataState::Open
        } else {
            HoverCardDataState::Closed
        },
    }
}

pub fn primitive_hover_card_delay_output(
    currently_open: bool,
    options: HoverCardRootOptions,
    event: HoverCardDelayEvent,
) -> HoverCardDelayOutput {
    let (open, next_open_delay_ms, close_after_ms) = match event {
        HoverCardDelayEvent::PointerEnter { hovered_ms } => {
            if hovered_ms >= options.open_delay_ms {
                (true, None, None)
            } else {
                (false, Some(options.open_delay_ms - hovered_ms), None)
            }
        }
        HoverCardDelayEvent::PointerLeave { elapsed_leave_ms } => {
            if !currently_open || elapsed_leave_ms >= options.close_delay_ms {
                (false, None, None)
            } else {
                (true, None, Some(options.close_delay_ms - elapsed_leave_ms))
            }
        }
        HoverCardDelayEvent::Focus => (true, None, None),
        HoverCardDelayEvent::Blur => (false, None, None),
    };
    HoverCardDelayOutput {
        open,
        data_state: if open {
            HoverCardDataState::Open
        } else {
            HoverCardDataState::Closed
        },
        next_open_delay_ms,
        close_after_ms,
    }
}

pub fn hover_card_apply_open(
    current: &mut bool,
    next: bool,
    options: &HoverCardRootOptions,
) -> bool {
    let output = primitive_hover_card_root_output((*options).open(*current));
    if output.open == next {
        return false;
    }
    *current = next;
    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverCardPortalOptions {
    pub force_mount: bool,
    pub container: Option<String>,
}

impl Default for HoverCardPortalOptions {
    fn default() -> Self {
        Self {
            force_mount: false,
            container: None,
        }
    }
}

impl HoverCardPortalOptions {
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
pub struct HoverCardPortalOutput {
    pub force_mount: bool,
    pub container: Option<String>,
}

pub fn primitive_hover_card_portal_output(
    options: HoverCardPortalOptions,
) -> HoverCardPortalOutput {
    HoverCardPortalOutput {
        force_mount: options.force_mount,
        container: options.container,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HoverCardOptions {
    pub id: egui::Id,
    pub trigger_rect: Rect,
    pub width: f32,
    pub placement: LayerPlacement,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
}

impl HoverCardOptions {
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
            open_delay_ms: 0,
            close_delay_ms: 300,
        }
    }

    pub fn open_delay_ms(mut self, delay_ms: u64) -> Self {
        self.open_delay_ms = delay_ms;
        self
    }

    pub fn close_delay_ms(mut self, delay_ms: u64) -> Self {
        self.close_delay_ms = delay_ms;
        self
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
pub struct HoverCardTriggerOptions {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub theme: PrimitiveTheme,
}

impl Default for HoverCardTriggerOptions {
    fn default() -> Self {
        Self {
            width: 112.0,
            height: 32.0,
            open: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl HoverCardTriggerOptions {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoverCardContentOptions {
    pub width: f32,
    pub placement: LayerPlacement,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
    pub open: bool,
    pub force_mount: bool,
    pub side: HoverCardSide,
    pub align: HoverCardAlign,
    pub data_state: HoverCardDataState,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
}

impl HoverCardContentOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self.data_state = if open {
            HoverCardDataState::Open
        } else {
            HoverCardDataState::Closed
        };
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn side_align(mut self, side: HoverCardSide, align: HoverCardAlign) -> Self {
        self.side = side;
        self.align = align;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HoverCardContentOutput {
    pub width: f32,
    pub open: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub side: HoverCardSide,
    pub align: HoverCardAlign,
    pub data_state: HoverCardDataState,
    pub open_delay_ms: u64,
    pub close_delay_ms: u64,
}

pub struct HoverCardOutput<T> {
    pub action: Option<T>,
    pub content_rect: Rect,
    pub arrow_side: PopoverArrowSide,
    pub side: HoverCardSide,
    pub align: HoverCardAlign,
}

pub fn primitive_hover_card_trigger(
    ui: &mut egui::Ui,
    label: &str,
    options: HoverCardTriggerOptions,
) -> Response {
    let root = primitive_hover_card_root_output(HoverCardRootOptions::default().open(options.open));
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(options.width, options.height), Sense::click());
    let fill = if root.data_state == HoverCardDataState::Open || response.hovered() {
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

pub fn primitive_hover_card_content_options(options: &HoverCardOptions) -> HoverCardContentOptions {
    let (side, align) = dropdown_menu_placement_parts(options.placement);
    HoverCardContentOptions {
        width: options.width,
        placement: options.placement,
        inner_margin: options.inner_margin,
        theme: options.theme,
        open: true,
        force_mount: false,
        side,
        align,
        data_state: HoverCardDataState::Open,
        open_delay_ms: options.open_delay_ms,
        close_delay_ms: options.close_delay_ms,
    }
}

pub fn primitive_hover_card_content_output(
    options: HoverCardContentOptions,
) -> HoverCardContentOutput {
    HoverCardContentOutput {
        width: options.width,
        open: options.open,
        force_mount: options.force_mount,
        mounted: options.open || options.force_mount,
        side: options.side,
        align: options.align,
        data_state: options.data_state,
        open_delay_ms: options.open_delay_ms,
        close_delay_ms: options.close_delay_ms,
    }
}

pub fn primitive_hover_card_layer_options(options: &HoverCardOptions) -> PrimitiveLayerOptions {
    let content = primitive_hover_card_content_options(options);
    let mut layer = PrimitiveLayerOptions::new(options.id, content.width)
        .anchor_rect(options.trigger_rect)
        .placement(content.placement)
        .order(egui::Order::Tooltip)
        .min_height(48.0)
        .inner_margin(content.inner_margin)
        .dismiss_policy(DismissPolicy::None);
    layer.theme = content.theme;
    layer
}

pub fn primitive_hover_card_content(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    theme: PrimitiveTheme,
) {
    let output = HoverCardContentOutput {
        width: 0.0,
        open: true,
        force_mount: false,
        mounted: true,
        side: HoverCardSide::Bottom,
        align: HoverCardAlign::Start,
        data_state: HoverCardDataState::Open,
        open_delay_ms: 0,
        close_delay_ms: 0,
    };
    let (title_color, description_color) = hover_card_content_text_colors(&output, theme);
    primitive_hover_card_content_text(ui, title, description, title_color, description_color);
}

pub fn hover_card_content_text_colors(
    output: &HoverCardContentOutput,
    theme: PrimitiveTheme,
) -> (egui::Color32, egui::Color32) {
    let colors = primitive_mounted_content_policy(
        output.data_state != HoverCardDataState::Closed,
        output.force_mount,
        theme,
    )
    .text_colors;
    (colors.title, colors.detail)
}

pub fn primitive_hover_card_content_with_options(
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    options: HoverCardContentOptions,
) {
    let output = primitive_hover_card_content_output(options);
    let (title_color, description_color) = hover_card_content_text_colors(&output, options.theme);
    primitive_hover_card_content_text(ui, title, description, title_color, description_color);
}

fn primitive_hover_card_content_text(
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

pub fn primitive_hover_card_arrow(
    ui: &egui::Ui,
    trigger_rect: Rect,
    content_rect: Rect,
    size: f32,
    theme: PrimitiveTheme,
) {
    primitive_popover_arrow(ui, trigger_rect, content_rect, size, theme);
}

pub fn show_hover_card<T>(
    ctx: &egui::Context,
    open: bool,
    options: HoverCardOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> Option<T>,
) -> Option<HoverCardOutput<T>> {
    if !open {
        return None;
    }
    let _delay_ms = options.open_delay_ms;
    let trigger_rect = options.trigger_rect;
    let layer = primitive_hover_card_layer_options(&options);
    let output: PrimitiveLayerOutput<T> = show_primitive_layer(ctx, layer, add_contents);
    Some(HoverCardOutput {
        action: output.action,
        content_rect: output.content_rect,
        arrow_side: popover_arrow_side(trigger_rect, output.content_rect),
        side: dropdown_menu_side_from_layer_side(output.resolved_placement.side),
        align: dropdown_menu_align_from_layer_align(output.resolved_placement.align),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hover_card_options_preserve_content_contract() {
        let options = HoverCardOptions::anchored(
            "hover_card_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::RightStart {
                offset: egui::vec2(6.0, 0.0),
            },
        )
        .inner_margin(egui::Margin::symmetric(4, 5))
        .open_delay_ms(120)
        .close_delay_ms(240);

        let content = primitive_hover_card_content_options(&options);

        assert_eq!(content.width, 180.0);
        assert_eq!(content.inner_margin.left, 4);
        assert_eq!(content.inner_margin.top, 5);
        assert_eq!(content.side, HoverCardSide::Right);
        assert_eq!(content.align, HoverCardAlign::Start);
        assert_eq!(content.data_state, HoverCardDataState::Open);
        assert_eq!(content.open_delay_ms, 120);
        assert_eq!(content.close_delay_ms, 240);
    }

    #[test]
    fn hover_card_layer_options_use_non_dismissable_hover_policy() {
        let options = HoverCardOptions::anchored(
            "hover_card_layer_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        );

        let layer = primitive_hover_card_layer_options(&options);

        assert_eq!(layer.dismiss_policy, DismissPolicy::None);
        assert_eq!(layer.order, egui::Order::Tooltip);
        assert_eq!(layer.min_height, Some(48.0));
        assert_eq!(layer.anchor_rect, Some(options.trigger_rect));
    }

    #[test]
    fn hover_card_trigger_options_preserve_root_state() {
        let options = HoverCardTriggerOptions::default()
            .size(96.0, 28.0)
            .open(true);

        assert_eq!(options.width, 96.0);
        assert_eq!(options.height, 28.0);
        assert!(options.open);
    }

    #[test]
    fn hover_card_root_output_preserves_radix_contract() {
        let output = primitive_hover_card_root_output(
            HoverCardRootOptions::default()
                .open(true)
                .default_open(false)
                .open_delay_ms(120)
                .close_delay_ms(240),
        );

        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert_eq!(output.open_delay_ms, 120);
        assert_eq!(output.close_delay_ms, 240);
        assert_eq!(output.data_state, HoverCardDataState::Open);
        assert_eq!(output.data_state.as_str(), "open");
    }

    #[test]
    fn hover_card_apply_open_respects_noop_state_and_delay_contract() {
        let options = HoverCardRootOptions::default()
            .open(false)
            .default_open(false)
            .open_delay_ms(120)
            .close_delay_ms(240);
        let mut open = false;

        assert!(!hover_card_apply_open(&mut open, false, &options));
        assert!(!open);
        assert!(hover_card_apply_open(&mut open, true, &options));
        assert!(open);

        let output = primitive_hover_card_root_output(options.open(open));
        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert_eq!(output.open_delay_ms, 120);
        assert_eq!(output.close_delay_ms, 240);
        assert_eq!(output.data_state, HoverCardDataState::Open);
    }

    #[test]
    fn hover_card_delay_output_waits_then_opens_after_delay() {
        let options = HoverCardRootOptions::default().open_delay_ms(700);

        let waiting = primitive_hover_card_delay_output(
            false,
            options,
            HoverCardDelayEvent::PointerEnter { hovered_ms: 250 },
        );
        let open = primitive_hover_card_delay_output(
            false,
            options,
            HoverCardDelayEvent::PointerEnter { hovered_ms: 700 },
        );

        assert!(!waiting.open);
        assert_eq!(waiting.next_open_delay_ms, Some(450));
        assert_eq!(waiting.data_state, HoverCardDataState::Closed);
        assert!(open.open);
        assert_eq!(open.next_open_delay_ms, None);
        assert_eq!(open.data_state, HoverCardDataState::Open);
    }

    #[test]
    fn hover_card_delay_output_uses_close_delay_before_closing() {
        let options = HoverCardRootOptions::default().close_delay_ms(300);

        let grace = primitive_hover_card_delay_output(
            true,
            options,
            HoverCardDelayEvent::PointerLeave {
                elapsed_leave_ms: 120,
            },
        );
        let closed = primitive_hover_card_delay_output(
            true,
            options,
            HoverCardDelayEvent::PointerLeave {
                elapsed_leave_ms: 300,
            },
        );

        assert!(grace.open);
        assert_eq!(grace.close_after_ms, Some(180));
        assert_eq!(grace.data_state, HoverCardDataState::Open);
        assert!(!closed.open);
        assert_eq!(closed.close_after_ms, None);
        assert_eq!(closed.data_state, HoverCardDataState::Closed);
    }

    #[test]
    fn hover_card_delay_output_opens_on_focus_and_closes_on_blur() {
        let options = HoverCardRootOptions::default();

        let focused = primitive_hover_card_delay_output(false, options, HoverCardDelayEvent::Focus);
        let blurred = primitive_hover_card_delay_output(true, options, HoverCardDelayEvent::Blur);

        assert!(focused.open);
        assert_eq!(focused.data_state, HoverCardDataState::Open);
        assert!(!blurred.open);
        assert_eq!(blurred.data_state, HoverCardDataState::Closed);
    }

    #[test]
    fn hover_card_portal_output_preserves_force_mount_and_container() {
        let output = primitive_hover_card_portal_output(
            HoverCardPortalOptions::default()
                .force_mount(true)
                .container("hover-card-layer"),
        );

        assert!(output.force_mount);
        assert_eq!(output.container.as_deref(), Some("hover-card-layer"));
    }

    #[test]
    fn hover_card_content_output_preserves_state_side_align_delay_and_mount() {
        let options = HoverCardOptions::anchored(
            "hover_card_content_output_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        )
        .open_delay_ms(80)
        .close_delay_ms(160);
        let content = primitive_hover_card_content_options(&options)
            .open(false)
            .force_mount(true)
            .side_align(HoverCardSide::Bottom, HoverCardAlign::Center);
        let output = primitive_hover_card_content_output(content);

        assert_eq!(output.width, 180.0);
        assert!(!output.open);
        assert!(output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.side, HoverCardSide::Bottom);
        assert_eq!(output.side.as_str(), "bottom");
        assert_eq!(output.align, HoverCardAlign::Center);
        assert_eq!(output.align.as_str(), "center");
        assert_eq!(output.data_state, HoverCardDataState::Closed);
        assert_eq!(output.data_state.as_str(), "closed");
        assert_eq!(output.open_delay_ms, 80);
        assert_eq!(output.close_delay_ms, 160);
    }

    #[test]
    fn hover_card_content_text_colors_mute_closed_force_mounted_content() {
        let theme = PrimitiveTheme::default();
        let open = HoverCardContentOutput {
            width: 180.0,
            open: true,
            force_mount: true,
            mounted: true,
            side: HoverCardSide::Bottom,
            align: HoverCardAlign::Start,
            data_state: HoverCardDataState::Open,
            open_delay_ms: 120,
            close_delay_ms: 300,
        };
        let closed = HoverCardContentOutput {
            data_state: HoverCardDataState::Closed,
            open: false,
            ..open
        };

        assert_eq!(
            hover_card_content_text_colors(&open, theme),
            (theme.text, theme.muted_text)
        );
        assert_eq!(
            hover_card_content_text_colors(&closed, theme),
            (theme.muted_text, theme.disabled_text)
        );
    }
}
