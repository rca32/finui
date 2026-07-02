use std::hash::Hash;

use eframe::egui::{self, Align2, Color32, FontId, Rect, Response, Sense, Stroke, Vec2};

use super::{
    DropdownMenuAlign, DropdownMenuSide, PopoverArrowSide, PrimitiveLayerAnimationOutput,
    PrimitiveLayerOptions, PrimitiveLayerOutput, PrimitiveTheme,
    dropdown_menu_align_from_layer_align, dropdown_menu_layer_align, dropdown_menu_layer_side,
    dropdown_menu_placement_parts, dropdown_menu_side_from_layer_side, popover_arrow_side,
    primitive_layer_animation_output, primitive_mounted_content_policy, primitive_popover_arrow,
    show_primitive_layer,
};
use crate::{DismissPolicy, LayerPlacement, LayerResolvedPlacement};

pub type TooltipSide = DropdownMenuSide;
pub type TooltipAlign = DropdownMenuAlign;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipDataState {
    Closed,
    DelayedOpen,
    InstantOpen,
}

impl TooltipDataState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::DelayedOpen => "delayed-open",
            Self::InstantOpen => "instant-open",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooltipProviderOptions {
    pub delay_duration_ms: u64,
    pub skip_delay_duration_ms: u64,
    pub disable_hoverable_content: bool,
}

impl Default for TooltipProviderOptions {
    fn default() -> Self {
        Self {
            delay_duration_ms: 700,
            skip_delay_duration_ms: 300,
            disable_hoverable_content: false,
        }
    }
}

impl TooltipProviderOptions {
    pub fn delay_duration_ms(mut self, delay_duration_ms: u64) -> Self {
        self.delay_duration_ms = delay_duration_ms;
        self
    }

    pub fn skip_delay_duration_ms(mut self, skip_delay_duration_ms: u64) -> Self {
        self.skip_delay_duration_ms = skip_delay_duration_ms;
        self
    }

    pub fn disable_hoverable_content(mut self, disabled: bool) -> Self {
        self.disable_hoverable_content = disabled;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooltipProviderOutput {
    pub delay_duration_ms: u64,
    pub skip_delay_duration_ms: u64,
    pub disable_hoverable_content: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TooltipDelayEvent {
    PointerEnter {
        hovered_ms: u64,
        since_last_close_ms: Option<u64>,
    },
    PointerLeave {
        grace_ms: u64,
    },
    ContentPointerEnter,
    Focus,
    Blur,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooltipDelayOutput {
    pub open: bool,
    pub instant: bool,
    pub data_state: TooltipDataState,
    pub next_delay_ms: Option<u64>,
    pub close_after_ms: Option<u64>,
    pub hoverable_content: bool,
    pub pointer_grace_active: bool,
}

pub fn primitive_tooltip_provider_output(options: TooltipProviderOptions) -> TooltipProviderOutput {
    TooltipProviderOutput {
        delay_duration_ms: options.delay_duration_ms,
        skip_delay_duration_ms: options.skip_delay_duration_ms,
        disable_hoverable_content: options.disable_hoverable_content,
    }
}

pub fn primitive_tooltip_delay_output(
    currently_open: bool,
    provider: TooltipProviderOptions,
    event: TooltipDelayEvent,
) -> TooltipDelayOutput {
    let hoverable_content = !provider.disable_hoverable_content;
    let (open, instant, next_delay_ms, close_after_ms, pointer_grace_active) = match event {
        TooltipDelayEvent::PointerEnter {
            hovered_ms,
            since_last_close_ms,
        } => {
            let skip_delay = since_last_close_ms
                .is_some_and(|elapsed| elapsed <= provider.skip_delay_duration_ms);
            if skip_delay {
                (true, true, None, None, false)
            } else if hovered_ms >= provider.delay_duration_ms {
                (true, false, None, None, false)
            } else {
                (
                    false,
                    false,
                    Some(provider.delay_duration_ms - hovered_ms),
                    None,
                    false,
                )
            }
        }
        TooltipDelayEvent::PointerLeave { grace_ms } => {
            if currently_open && hoverable_content && grace_ms > 0 {
                (true, false, None, Some(grace_ms), true)
            } else {
                (false, false, None, None, false)
            }
        }
        TooltipDelayEvent::ContentPointerEnter => (
            currently_open && hoverable_content,
            false,
            None,
            None,
            false,
        ),
        TooltipDelayEvent::Focus => (true, true, None, None, false),
        TooltipDelayEvent::Blur => (false, false, None, None, false),
    };
    TooltipDelayOutput {
        open,
        instant,
        data_state: tooltip_data_state(open, instant),
        next_delay_ms,
        close_after_ms,
        hoverable_content,
        pointer_grace_active,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooltipRootOptions {
    pub open: bool,
    pub default_open: Option<bool>,
    pub delay_duration_ms: u64,
    pub disable_hoverable_content: bool,
    pub instant: bool,
}

impl Default for TooltipRootOptions {
    fn default() -> Self {
        Self {
            open: false,
            default_open: None,
            delay_duration_ms: 700,
            disable_hoverable_content: false,
            instant: false,
        }
    }
}

impl TooltipRootOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    pub fn delay_duration_ms(mut self, delay_duration_ms: u64) -> Self {
        self.delay_duration_ms = delay_duration_ms;
        self
    }

    pub fn disable_hoverable_content(mut self, disabled: bool) -> Self {
        self.disable_hoverable_content = disabled;
        self
    }

    pub fn instant(mut self, instant: bool) -> Self {
        self.instant = instant;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TooltipRootOutput {
    pub open: bool,
    pub default_open: Option<bool>,
    pub delay_duration_ms: u64,
    pub disable_hoverable_content: bool,
    pub data_state: TooltipDataState,
}

pub fn primitive_tooltip_root_output(options: TooltipRootOptions) -> TooltipRootOutput {
    TooltipRootOutput {
        open: options.open,
        default_open: options.default_open,
        delay_duration_ms: options.delay_duration_ms,
        disable_hoverable_content: options.disable_hoverable_content,
        data_state: tooltip_data_state(options.open, options.instant),
    }
}

pub fn tooltip_apply_open(current: &mut bool, next: bool, options: &TooltipRootOptions) -> bool {
    let output = primitive_tooltip_root_output((*options).open(*current));
    if output.open == next {
        return false;
    }
    *current = next;
    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TooltipPortalOptions {
    pub force_mount: bool,
    pub container: Option<String>,
}

impl Default for TooltipPortalOptions {
    fn default() -> Self {
        Self {
            force_mount: false,
            container: None,
        }
    }
}

impl TooltipPortalOptions {
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
pub struct TooltipPortalOutput {
    pub force_mount: bool,
    pub container: Option<String>,
}

pub fn primitive_tooltip_portal_output(options: TooltipPortalOptions) -> TooltipPortalOutput {
    TooltipPortalOutput {
        force_mount: options.force_mount,
        container: options.container,
    }
}

pub fn tooltip_data_state(open: bool, instant: bool) -> TooltipDataState {
    if !open {
        TooltipDataState::Closed
    } else if instant {
        TooltipDataState::InstantOpen
    } else {
        TooltipDataState::DelayedOpen
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TooltipOptions {
    pub id: egui::Id,
    pub trigger_rect: Rect,
    pub width: f32,
    pub placement: LayerPlacement,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
}

impl TooltipOptions {
    pub fn new(id: impl Hash, trigger_rect: Rect) -> Self {
        Self {
            id: egui::Id::new(id),
            trigger_rect,
            width: 220.0,
            placement: LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
            inner_margin: egui::Margin::symmetric(10, 6),
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn placement(mut self, placement: LayerPlacement) -> Self {
        self.placement = placement;
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
pub struct TooltipTriggerOptions {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub theme: PrimitiveTheme,
}

impl Default for TooltipTriggerOptions {
    fn default() -> Self {
        Self {
            width: 112.0,
            height: 32.0,
            open: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl TooltipTriggerOptions {
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
pub struct TooltipContentOptions {
    pub width: f32,
    pub placement: LayerPlacement,
    pub inner_margin: egui::Margin,
    pub open: bool,
    pub instant: bool,
    pub force_mount: bool,
    pub aria_label: Option<&'static str>,
    pub side: TooltipSide,
    pub align: TooltipAlign,
    pub data_state: TooltipDataState,
    pub theme: PrimitiveTheme,
}

impl TooltipContentOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self.data_state = tooltip_data_state(open, self.instant);
        self
    }

    pub fn instant(mut self, instant: bool) -> Self {
        self.instant = instant;
        self.data_state = tooltip_data_state(self.open, instant);
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn aria_label(mut self, aria_label: &'static str) -> Self {
        self.aria_label = Some(aria_label);
        self
    }

    pub fn side_align(mut self, side: TooltipSide, align: TooltipAlign) -> Self {
        self.side = side;
        self.align = align;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TooltipContentOutput {
    pub width: f32,
    pub open: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub aria_label: Option<&'static str>,
    pub side: TooltipSide,
    pub align: TooltipAlign,
    pub data_state: TooltipDataState,
    pub animation: PrimitiveLayerAnimationOutput,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TooltipOutput {
    pub content_rect: Rect,
    pub arrow_side: PopoverArrowSide,
    pub side: TooltipSide,
    pub align: TooltipAlign,
    pub animation: PrimitiveLayerAnimationOutput,
}

pub fn primitive_tooltip_trigger(
    ui: &mut egui::Ui,
    label: &str,
    options: TooltipTriggerOptions,
) -> Response {
    let root = primitive_tooltip_root_output(TooltipRootOptions::default().open(options.open));
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(options.width, options.height), Sense::click());
    let fill = if root.open || response.hovered() {
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

pub fn primitive_tooltip_content_options(options: &TooltipOptions) -> TooltipContentOptions {
    let (side, align) = dropdown_menu_placement_parts(options.placement);
    TooltipContentOptions {
        width: options.width,
        placement: options.placement,
        inner_margin: options.inner_margin,
        open: true,
        instant: false,
        force_mount: false,
        aria_label: None,
        side,
        align,
        data_state: TooltipDataState::DelayedOpen,
        theme: options.theme,
    }
}

pub fn primitive_tooltip_content_output(options: TooltipContentOptions) -> TooltipContentOutput {
    TooltipContentOutput {
        width: options.width,
        open: options.open,
        force_mount: options.force_mount,
        mounted: options.open || options.force_mount,
        aria_label: options.aria_label,
        side: options.side,
        align: options.align,
        data_state: options.data_state,
        animation: primitive_layer_animation_output(
            options.data_state != TooltipDataState::Closed,
            LayerResolvedPlacement {
                side: dropdown_menu_layer_side(options.side),
                align: dropdown_menu_layer_align(options.align),
                flipped: false,
            },
            1.0,
        ),
    }
}

pub fn primitive_tooltip_layer_options(options: &TooltipOptions) -> PrimitiveLayerOptions {
    let content = primitive_tooltip_content_options(options);
    let mut layer = PrimitiveLayerOptions::new(options.id, content.width)
        .anchor_rect(options.trigger_rect)
        .placement(content.placement)
        .order(egui::Order::Tooltip)
        .min_height(32.0)
        .inner_margin(content.inner_margin)
        .dismiss_policy(DismissPolicy::None);
    layer.theme = content.theme;
    layer
}

pub fn tooltip_content_text_color(output: &TooltipContentOutput, theme: PrimitiveTheme) -> Color32 {
    primitive_mounted_content_policy(
        output.data_state != TooltipDataState::Closed,
        output.force_mount,
        theme,
    )
    .text_colors
    .title
}

pub fn primitive_tooltip_content(ui: &mut egui::Ui, text: &str, theme: PrimitiveTheme) {
    let output = primitive_tooltip_content_output(
        primitive_tooltip_content_options(&TooltipOptions::new(
            "tooltip_content_default",
            ui.min_rect(),
        ))
        .open(true)
        .theme(theme),
    );
    primitive_tooltip_content_text(ui, text, tooltip_content_text_color(&output, theme));
}

pub fn primitive_tooltip_content_with_options(
    ui: &mut egui::Ui,
    text: &str,
    options: TooltipContentOptions,
) {
    let output = primitive_tooltip_content_output(options);
    primitive_tooltip_content_text(ui, text, tooltip_content_text_color(&output, options.theme));
}

fn primitive_tooltip_content_text(ui: &mut egui::Ui, text: &str, color: Color32) {
    ui.label(egui::RichText::new(text).color(color));
}

pub fn primitive_tooltip_arrow(
    ui: &egui::Ui,
    trigger_rect: Rect,
    content_rect: Rect,
    size: f32,
    theme: PrimitiveTheme,
) {
    primitive_popover_arrow(ui, trigger_rect, content_rect, size, theme);
}

pub fn show_tooltip(
    ctx: &egui::Context,
    open: bool,
    options: TooltipOptions,
    text: &str,
) -> Option<TooltipOutput> {
    if !open {
        return None;
    }
    let trigger_rect = options.trigger_rect;
    let theme = options.theme;
    let layer = primitive_tooltip_layer_options(&options);
    let output: PrimitiveLayerOutput<()> = show_primitive_layer(ctx, layer, |ui| {
        primitive_tooltip_content(ui, text, theme);
        None::<()>
    });
    Some(TooltipOutput {
        content_rect: output.content_rect,
        arrow_side: popover_arrow_side(trigger_rect, output.content_rect),
        side: dropdown_menu_side_from_layer_side(output.resolved_placement.side),
        align: dropdown_menu_align_from_layer_align(output.resolved_placement.align),
        animation: primitive_layer_animation_output(true, output.resolved_placement, 1.0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tooltip_options_preserve_content_contract() {
        let options = TooltipOptions::new(
            "tooltip_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
        )
        .width(180.0)
        .placement(LayerPlacement::RightStart {
            offset: egui::vec2(6.0, 0.0),
        })
        .inner_margin(egui::Margin::symmetric(4, 5));

        let content = primitive_tooltip_content_options(&options);

        assert_eq!(content.width, 180.0);
        assert_eq!(content.inner_margin.left, 4);
        assert_eq!(content.inner_margin.top, 5);
        assert_eq!(content.side, TooltipSide::Right);
        assert_eq!(content.align, TooltipAlign::Start);
        assert_eq!(content.data_state, TooltipDataState::DelayedOpen);
    }

    #[test]
    fn tooltip_layer_options_use_non_dismissable_hover_policy() {
        let options = TooltipOptions::new(
            "tooltip_layer_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
        );

        let layer = primitive_tooltip_layer_options(&options);

        assert_eq!(layer.dismiss_policy, DismissPolicy::None);
        assert_eq!(layer.order, egui::Order::Tooltip);
        assert_eq!(layer.min_height, Some(32.0));
        assert_eq!(layer.anchor_rect, Some(options.trigger_rect));
    }

    #[test]
    fn tooltip_trigger_options_preserve_root_state() {
        let options = TooltipTriggerOptions::default().size(96.0, 28.0).open(true);

        assert_eq!(options.width, 96.0);
        assert_eq!(options.height, 28.0);
        assert!(options.open);
    }

    #[test]
    fn tooltip_provider_output_preserves_radix_contract() {
        let output = primitive_tooltip_provider_output(
            TooltipProviderOptions::default()
                .delay_duration_ms(800)
                .skip_delay_duration_ms(500)
                .disable_hoverable_content(true),
        );

        assert_eq!(output.delay_duration_ms, 800);
        assert_eq!(output.skip_delay_duration_ms, 500);
        assert!(output.disable_hoverable_content);
    }

    #[test]
    fn tooltip_root_output_preserves_radix_contract() {
        let output = primitive_tooltip_root_output(
            TooltipRootOptions::default()
                .open(true)
                .default_open(false)
                .delay_duration_ms(0)
                .disable_hoverable_content(true)
                .instant(true),
        );

        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert_eq!(output.delay_duration_ms, 0);
        assert!(output.disable_hoverable_content);
        assert_eq!(output.data_state, TooltipDataState::InstantOpen);
        assert_eq!(output.data_state.as_str(), "instant-open");
    }

    #[test]
    fn tooltip_apply_open_respects_noop_state_and_root_contract() {
        let options = TooltipRootOptions::default()
            .open(false)
            .default_open(false)
            .delay_duration_ms(120)
            .instant(true);
        let mut open = false;

        assert!(!tooltip_apply_open(&mut open, false, &options));
        assert!(!open);
        assert!(tooltip_apply_open(&mut open, true, &options));
        assert!(open);

        let output = primitive_tooltip_root_output(options.open(open));
        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert_eq!(output.delay_duration_ms, 120);
        assert_eq!(output.data_state, TooltipDataState::InstantOpen);
    }

    #[test]
    fn tooltip_portal_output_preserves_force_mount_and_container() {
        let output = primitive_tooltip_portal_output(
            TooltipPortalOptions::default()
                .force_mount(true)
                .container("tooltip-layer"),
        );

        assert!(output.force_mount);
        assert_eq!(output.container.as_deref(), Some("tooltip-layer"));
    }

    #[test]
    fn tooltip_content_output_preserves_state_side_align_aria_and_mount() {
        let options = TooltipOptions::new(
            "tooltip_content_output_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
        )
        .placement(LayerPlacement::BelowStart {
            offset: egui::vec2(0.0, 6.0),
        })
        .width(180.0);
        let content = primitive_tooltip_content_options(&options)
            .open(false)
            .instant(true)
            .force_mount(true)
            .aria_label("Tooltip label")
            .side_align(TooltipSide::Bottom, TooltipAlign::Center);
        let output = primitive_tooltip_content_output(content);

        assert_eq!(output.width, 180.0);
        assert!(!output.open);
        assert!(output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.aria_label, Some("Tooltip label"));
        assert_eq!(output.side, TooltipSide::Bottom);
        assert_eq!(output.side.as_str(), "bottom");
        assert_eq!(output.align, TooltipAlign::Center);
        assert_eq!(output.align.as_str(), "center");
        assert_eq!(output.data_state, TooltipDataState::Closed);
        assert_eq!(output.data_state.as_str(), "closed");
        assert_eq!(output.animation.open_progress, 0.0);
        assert_eq!(output.animation.data_side, "bottom");
        assert_eq!(output.animation.data_align, "center");
        assert_eq!(output.animation.transform_origin, egui::pos2(0.5, 0.0));
    }

    #[test]
    fn tooltip_output_reports_collision_resolved_side_align() {
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
        let options = TooltipOptions::new(
            "tooltip_collision_output_test",
            Rect::from_min_size(egui::pos2(24.0, 204.0), egui::vec2(42.0, 28.0)),
        )
        .placement(LayerPlacement::BelowStart {
            offset: egui::vec2(0.0, 6.0),
        })
        .width(120.0);

        let output = show_tooltip(&ctx, true, options, "Collision tooltip")
            .expect("open tooltip should render");

        assert_eq!(output.side, TooltipSide::Top);
        assert_eq!(output.align, TooltipAlign::Start);
        assert_eq!(output.animation.data_side, "top");
        assert_eq!(output.animation.data_align, "start");
        assert_eq!(output.animation.transform_origin, egui::pos2(0.0, 1.0));
        assert!(output.animation.collision_flipped);
    }

    #[test]
    fn tooltip_content_text_color_mutes_closed_force_mounted_content() {
        let theme = PrimitiveTheme::default();
        let closed = primitive_tooltip_content_output(
            primitive_tooltip_content_options(&TooltipOptions::new(
                "tooltip_closed_text_color_test",
                Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            ))
            .open(false)
            .force_mount(true),
        );
        let open = primitive_tooltip_content_output(
            primitive_tooltip_content_options(&TooltipOptions::new(
                "tooltip_open_text_color_test",
                Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            ))
            .open(true),
        );

        assert_eq!(tooltip_content_text_color(&closed, theme), theme.muted_text);
        assert_eq!(tooltip_content_text_color(&open, theme), theme.text);
    }

    #[test]
    fn tooltip_data_state_tracks_delayed_and_instant_open() {
        assert_eq!(tooltip_data_state(false, false), TooltipDataState::Closed);
        assert_eq!(
            tooltip_data_state(true, false),
            TooltipDataState::DelayedOpen
        );
        assert_eq!(
            tooltip_data_state(true, true),
            TooltipDataState::InstantOpen
        );
    }

    #[test]
    fn tooltip_delay_output_waits_then_opens_after_delay() {
        let provider = TooltipProviderOptions::default().delay_duration_ms(700);

        let waiting = primitive_tooltip_delay_output(
            false,
            provider,
            TooltipDelayEvent::PointerEnter {
                hovered_ms: 250,
                since_last_close_ms: None,
            },
        );
        let open = primitive_tooltip_delay_output(
            false,
            provider,
            TooltipDelayEvent::PointerEnter {
                hovered_ms: 700,
                since_last_close_ms: None,
            },
        );

        assert!(!waiting.open);
        assert_eq!(waiting.next_delay_ms, Some(450));
        assert_eq!(waiting.data_state, TooltipDataState::Closed);
        assert!(open.open);
        assert!(!open.instant);
        assert_eq!(open.data_state, TooltipDataState::DelayedOpen);
    }

    #[test]
    fn tooltip_delay_output_uses_skip_delay_for_instant_reopen() {
        let provider = TooltipProviderOptions::default()
            .delay_duration_ms(700)
            .skip_delay_duration_ms(300);

        let output = primitive_tooltip_delay_output(
            false,
            provider,
            TooltipDelayEvent::PointerEnter {
                hovered_ms: 0,
                since_last_close_ms: Some(200),
            },
        );

        assert!(output.open);
        assert!(output.instant);
        assert_eq!(output.next_delay_ms, None);
        assert_eq!(output.data_state, TooltipDataState::InstantOpen);
    }

    #[test]
    fn tooltip_delay_output_respects_hoverable_content_grace_and_disable_flag() {
        let hoverable = TooltipProviderOptions::default().disable_hoverable_content(false);
        let disabled = TooltipProviderOptions::default().disable_hoverable_content(true);

        let grace = primitive_tooltip_delay_output(
            true,
            hoverable,
            TooltipDelayEvent::PointerLeave { grace_ms: 120 },
        );
        let closed = primitive_tooltip_delay_output(
            true,
            disabled,
            TooltipDelayEvent::PointerLeave { grace_ms: 120 },
        );
        let content =
            primitive_tooltip_delay_output(true, hoverable, TooltipDelayEvent::ContentPointerEnter);

        assert!(grace.open);
        assert!(grace.pointer_grace_active);
        assert_eq!(grace.close_after_ms, Some(120));
        assert!(!closed.open);
        assert!(!closed.hoverable_content);
        assert!(content.open);
    }

    #[test]
    fn tooltip_delay_output_opens_on_focus_and_closes_on_blur() {
        let provider = TooltipProviderOptions::default();

        let focused = primitive_tooltip_delay_output(false, provider, TooltipDelayEvent::Focus);
        let blurred = primitive_tooltip_delay_output(true, provider, TooltipDelayEvent::Blur);

        assert!(focused.open);
        assert!(focused.instant);
        assert_eq!(focused.data_state, TooltipDataState::InstantOpen);
        assert!(!blurred.open);
        assert_eq!(blurred.data_state, TooltipDataState::Closed);
    }
}
