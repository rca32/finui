use std::hash::Hash;

use eframe::egui::{self, Align2, FontId, Pos2, Rect, Response, Sense, Stroke, Vec2};

use super::{
    DropdownMenuAlign, DropdownMenuDataState, DropdownMenuDirection, DropdownMenuOutput,
    DropdownMenuSide, DropdownMenuSubContentOptions, DropdownMenuSubContentOutput,
    DropdownMenuSubDelayOutput, DropdownMenuSubDismissOutput, DropdownMenuSubKeyboardAction,
    DropdownMenuSubPointerGraceOutput, DropdownMenuSubTriggerOptions, DropdownMenuSubTriggerOutput,
    MenuItem, MenuItemOptions, PrimitiveLayerOptions, PrimitiveTheme, RovingFocusAction,
    RovingFocusKey, RovingFocusOutput, dropdown_menu_align_from_layer_align,
    dropdown_menu_roving_focus_output, dropdown_menu_side_from_layer_side,
    dropdown_menu_sub_delay_output, dropdown_menu_sub_dismiss_output,
    dropdown_menu_sub_keyboard_action, dropdown_menu_sub_pointer_grace_output,
    menu_typeahead_match, primitive_dropdown_menu_sub_content_output,
    primitive_dropdown_menu_sub_trigger_output, primitive_layer_animation_output,
    primitive_menu_checkbox_item, primitive_menu_item, primitive_menu_label,
    primitive_menu_radio_item, primitive_menu_separator, radix_colors, show_primitive_layer,
};
use crate::{DismissPolicy, LayerPlacement};

pub type ContextMenuDataState = DropdownMenuDataState;
pub type ContextMenuDirection = DropdownMenuDirection;
pub type ContextMenuSide = DropdownMenuSide;
pub type ContextMenuAlign = DropdownMenuAlign;
pub type ContextMenuSubTriggerOptions = DropdownMenuSubTriggerOptions;
pub type ContextMenuSubTriggerOutput = DropdownMenuSubTriggerOutput;
pub type ContextMenuSubContentOptions = DropdownMenuSubContentOptions;
pub type ContextMenuSubContentOutput = DropdownMenuSubContentOutput;
pub type ContextMenuSubKeyboardAction = DropdownMenuSubKeyboardAction;
pub type ContextMenuSubDelayOutput = DropdownMenuSubDelayOutput;
pub type ContextMenuSubPointerGraceOutput = DropdownMenuSubPointerGraceOutput;
pub type ContextMenuSubDismissOutput = DropdownMenuSubDismissOutput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextMenuOpenOrigin {
    Pointer,
    TouchLongPress,
    Keyboard,
}

impl ContextMenuOpenOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pointer => "pointer",
            Self::TouchLongPress => "touch-long-press",
            Self::Keyboard => "keyboard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextMenuOpenOutput {
    pub open: bool,
    pub position: Option<Pos2>,
    pub origin: Option<ContextMenuOpenOrigin>,
    pub data_origin: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextMenuRootOptions {
    pub open: bool,
    pub modal: bool,
    pub direction: Option<ContextMenuDirection>,
}

impl Default for ContextMenuRootOptions {
    fn default() -> Self {
        Self {
            open: false,
            modal: true,
            direction: None,
        }
    }
}

impl ContextMenuRootOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    pub fn direction(mut self, direction: ContextMenuDirection) -> Self {
        self.direction = Some(direction);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextMenuRootOutput {
    pub open: bool,
    pub modal: bool,
    pub direction: Option<ContextMenuDirection>,
    pub data_state: ContextMenuDataState,
}

pub fn primitive_context_menu_root_output(
    options: ContextMenuRootOptions,
) -> ContextMenuRootOutput {
    ContextMenuRootOutput {
        open: options.open,
        modal: options.modal,
        direction: options.direction,
        data_state: if options.open {
            ContextMenuDataState::Open
        } else {
            ContextMenuDataState::Closed
        },
    }
}

pub fn context_menu_apply_open(
    current_position: &mut Option<Pos2>,
    next_position: Option<Pos2>,
    options: &ContextMenuRootOptions,
) -> bool {
    let output = primitive_context_menu_root_output((*options).open(current_position.is_some()));
    if output.open == next_position.is_some() && *current_position == next_position {
        return false;
    }
    *current_position = next_position;
    true
}

pub fn context_menu_pointer_open_output(position: Pos2, disabled: bool) -> ContextMenuOpenOutput {
    context_menu_open_output(!disabled, Some(position), ContextMenuOpenOrigin::Pointer)
}

pub fn context_menu_long_press_open_output(
    start_position: Pos2,
    current_position: Pos2,
    elapsed_ms: u64,
    threshold_ms: u64,
    max_movement: f32,
    disabled: bool,
) -> ContextMenuOpenOutput {
    let movement = start_position.distance(current_position);
    let open = !disabled && elapsed_ms >= threshold_ms && movement <= max_movement.max(0.0);
    context_menu_open_output(
        open,
        Some(start_position),
        ContextMenuOpenOrigin::TouchLongPress,
    )
}

pub fn context_menu_keyboard_open_output(
    trigger_rect: Rect,
    context_key_pressed: bool,
    shift_f10_pressed: bool,
    disabled: bool,
) -> ContextMenuOpenOutput {
    let open = !disabled && (context_key_pressed || shift_f10_pressed);
    context_menu_open_output(
        open,
        Some(trigger_rect.center()),
        ContextMenuOpenOrigin::Keyboard,
    )
}

fn context_menu_open_output(
    open: bool,
    position: Option<Pos2>,
    origin: ContextMenuOpenOrigin,
) -> ContextMenuOpenOutput {
    ContextMenuOpenOutput {
        open,
        position: open.then_some(position).flatten(),
        origin: open.then_some(origin),
        data_origin: open.then_some(origin.as_str()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextMenuPortalOptions {
    pub force_mount: bool,
    pub container: Option<String>,
}

impl Default for ContextMenuPortalOptions {
    fn default() -> Self {
        Self {
            force_mount: false,
            container: None,
        }
    }
}

impl ContextMenuPortalOptions {
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
pub struct ContextMenuPortalOutput {
    pub force_mount: bool,
    pub container: Option<String>,
}

pub fn primitive_context_menu_portal_output(
    options: ContextMenuPortalOptions,
) -> ContextMenuPortalOutput {
    ContextMenuPortalOutput {
        force_mount: options.force_mount,
        container: options.container,
    }
}

pub struct ContextMenuOptions {
    pub id: egui::Id,
    pub position: Pos2,
    pub width: f32,
    pub max_height: Option<f32>,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
}

impl ContextMenuOptions {
    pub fn at(id: impl Hash, position: Pos2, width: f32) -> Self {
        Self {
            id: egui::Id::new(id),
            position,
            width,
            max_height: None,
            inner_margin: egui::Margin::same(8),
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
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
pub struct ContextMenuTriggerOptions {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub disabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for ContextMenuTriggerOptions {
    fn default() -> Self {
        Self {
            width: 150.0,
            height: 32.0,
            open: false,
            disabled: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl ContextMenuTriggerOptions {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextMenuContentOptions {
    pub width: f32,
    pub max_height: Option<f32>,
    pub inner_margin: egui::Margin,
    pub loop_focus: bool,
    pub force_mount: bool,
    pub open: bool,
    pub side: ContextMenuSide,
    pub align: ContextMenuAlign,
    pub data_state: ContextMenuDataState,
    pub theme: PrimitiveTheme,
}

impl ContextMenuContentOptions {
    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self.data_state = if open {
            ContextMenuDataState::Open
        } else {
            ContextMenuDataState::Closed
        };
        self
    }

    pub fn side_align(mut self, side: ContextMenuSide, align: ContextMenuAlign) -> Self {
        self.side = side;
        self.align = align;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextMenuContentOutput {
    pub width: f32,
    pub max_height: Option<f32>,
    pub open: bool,
    pub loop_focus: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub side: ContextMenuSide,
    pub align: ContextMenuAlign,
    pub data_state: ContextMenuDataState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextMenuItemOptions {
    pub width: f32,
    pub selected: bool,
    pub highlighted: bool,
    pub checked: bool,
    pub disabled: bool,
    pub theme: PrimitiveTheme,
}

impl ContextMenuItemOptions {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            selected: false,
            highlighted: false,
            checked: false,
            disabled: false,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn highlighted(mut self, highlighted: bool) -> Self {
        self.highlighted = highlighted;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

pub type ContextMenuLabelOptions = f32;
pub type ContextMenuSeparatorOptions = f32;
pub type ContextMenuItem<T> = MenuItem<T>;

pub fn context_menu_roving_focus_output<T>(
    items: &[ContextMenuItem<T>],
    current: Option<usize>,
    key: Option<RovingFocusKey>,
    loop_focus: bool,
) -> RovingFocusOutput {
    dropdown_menu_roving_focus_output(items, current, key, loop_focus)
}

pub fn primitive_context_menu_sub_trigger_output(
    options: ContextMenuSubTriggerOptions,
) -> ContextMenuSubTriggerOutput {
    primitive_dropdown_menu_sub_trigger_output(options)
}

pub fn primitive_context_menu_sub_content_output(
    options: ContextMenuSubContentOptions,
) -> ContextMenuSubContentOutput {
    primitive_dropdown_menu_sub_content_output(options)
}

pub fn context_menu_sub_keyboard_action(
    direction: ContextMenuDirection,
    arrow_left_pressed: bool,
    arrow_right_pressed: bool,
    escape_pressed: bool,
) -> ContextMenuSubKeyboardAction {
    dropdown_menu_sub_keyboard_action(
        direction,
        arrow_left_pressed,
        arrow_right_pressed,
        escape_pressed,
    )
}

pub fn context_menu_sub_delay_output(
    requested_open: bool,
    elapsed_ms: u64,
    open_delay_ms: u64,
) -> ContextMenuSubDelayOutput {
    dropdown_menu_sub_delay_output(requested_open, elapsed_ms, open_delay_ms)
}

pub fn context_menu_sub_pointer_grace_output(
    trigger_rect: Rect,
    content_rect: Rect,
    pointer_pos: Pos2,
    side: ContextMenuSide,
    grace: f32,
) -> ContextMenuSubPointerGraceOutput {
    dropdown_menu_sub_pointer_grace_output(trigger_rect, content_rect, pointer_pos, side, grace)
}

pub fn context_menu_sub_dismiss_output(
    parent_content_rect: Rect,
    submenu_content_rect: Rect,
    pointer_pos: Pos2,
) -> ContextMenuSubDismissOutput {
    dropdown_menu_sub_dismiss_output(parent_content_rect, submenu_content_rect, pointer_pos)
}

pub fn context_menu_typeahead_match<T: Copy + PartialEq>(
    items: &[ContextMenuItem<T>],
    current: Option<T>,
    query: &str,
) -> Option<T> {
    menu_typeahead_match(items, current, query)
}

pub fn show_context_menu<T>(
    ctx: &egui::Context,
    options: ContextMenuOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> Option<T>,
) -> DropdownMenuOutput<T> {
    let layer = primitive_context_menu_layer_options(&options);
    let output = show_primitive_layer(ctx, layer, add_contents);
    DropdownMenuOutput {
        action: output.action,
        should_close: output.should_close,
        content_rect: output.content_rect,
        side: dropdown_menu_side_from_layer_side(output.resolved_placement.side),
        align: dropdown_menu_align_from_layer_align(output.resolved_placement.align),
        animation: primitive_layer_animation_output(true, output.resolved_placement, 1.0),
    }
}

pub fn primitive_context_menu_trigger(
    ui: &mut egui::Ui,
    label: &str,
    options: ContextMenuTriggerOptions,
) -> Response {
    let root =
        primitive_context_menu_root_output(ContextMenuRootOptions::default().open(options.open));
    let sense = if options.disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(options.width, options.height), sense);
    let open = root.data_state == ContextMenuDataState::Open;
    let fill = if options.disabled {
        radix_colors::SLATE_2
    } else if open {
        radix_colors::INDIGO_3
    } else if response.hovered() {
        radix_colors::SLATE_2
    } else {
        options.theme.content_fill
    };
    let stroke_color = if options.disabled {
        radix_colors::SLATE_5
    } else if open {
        radix_colors::INDIGO_8
    } else if response.hovered() {
        radix_colors::SLATE_7
    } else {
        options.theme.content_stroke.color
    };
    let text_color = if options.disabled {
        options.theme.disabled_text
    } else if open {
        radix_colors::INDIGO_11
    } else {
        options.theme.text
    };
    ui.painter().rect(
        rect,
        options.theme.row_radius,
        fill,
        Stroke::new(1.0, stroke_color),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        format!("Right click: {label}"),
        crate::scaled_proportional_font(ui, 12.0),
        text_color,
    );
    response
}

pub fn primitive_context_menu_content_options(
    options: &ContextMenuOptions,
) -> ContextMenuContentOptions {
    ContextMenuContentOptions {
        width: options.width,
        max_height: options.max_height,
        inner_margin: options.inner_margin,
        loop_focus: false,
        force_mount: false,
        open: true,
        side: ContextMenuSide::Bottom,
        align: ContextMenuAlign::Start,
        data_state: ContextMenuDataState::Open,
        theme: options.theme,
    }
}

pub fn primitive_context_menu_content_output(
    options: ContextMenuContentOptions,
) -> ContextMenuContentOutput {
    ContextMenuContentOutput {
        width: options.width,
        max_height: options.max_height,
        open: options.open,
        loop_focus: options.loop_focus,
        force_mount: options.force_mount,
        mounted: options.open || options.force_mount,
        side: options.side,
        align: options.align,
        data_state: options.data_state,
    }
}

pub fn primitive_context_menu_layer_options(options: &ContextMenuOptions) -> PrimitiveLayerOptions {
    let content = primitive_context_menu_content_options(options);
    let mut layer = PrimitiveLayerOptions::new(options.id, content.width)
        .placement(LayerPlacement::Fixed(options.position))
        .order(egui::Order::Tooltip)
        .inner_margin(content.inner_margin)
        .dismiss_policy(DismissPolicy::OutsideClickAndEscape);
    layer.theme = content.theme;
    if let Some(max_height) = content.max_height {
        layer = layer.max_height(max_height);
    }
    layer
}

pub fn primitive_context_menu_item(
    ui: &mut egui::Ui,
    label: &'static str,
    options: ContextMenuItemOptions,
) -> Response {
    primitive_menu_item(ui, label, context_menu_item_to_menu_options(options))
}

pub fn primitive_context_menu_checkbox_item(
    ui: &mut egui::Ui,
    label: &'static str,
    checked: &mut bool,
    options: ContextMenuItemOptions,
) -> Response {
    primitive_menu_checkbox_item(
        ui,
        label,
        checked,
        context_menu_item_to_menu_options(options),
    )
}

pub fn primitive_context_menu_radio_item(
    ui: &mut egui::Ui,
    label: &'static str,
    value: &'static str,
    current_value: &mut &'static str,
    options: ContextMenuItemOptions,
) -> Response {
    primitive_menu_radio_item(
        ui,
        label,
        value,
        current_value,
        context_menu_item_to_menu_options(options),
    )
}

pub fn primitive_context_menu_label(
    ui: &mut egui::Ui,
    label: &str,
    width: ContextMenuLabelOptions,
) {
    primitive_menu_label(ui, label, width);
}

pub fn primitive_context_menu_separator(ui: &mut egui::Ui, width: ContextMenuSeparatorOptions) {
    primitive_menu_separator(ui, width);
}

fn context_menu_item_to_menu_options(options: ContextMenuItemOptions) -> MenuItemOptions {
    let mut menu_options = MenuItemOptions::new(options.width)
        .selected(options.selected)
        .highlighted(options.highlighted)
        .checked(options.checked)
        .disabled(options.disabled);
    menu_options.theme = options.theme;
    menu_options
}

pub fn context_menu_anchor_rect(position: Pos2) -> Rect {
    Rect::from_min_size(position, egui::Vec2::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_menu_anchor_is_zero_sized_at_pointer() {
        let rect = context_menu_anchor_rect(egui::pos2(12.0, 34.0));

        assert_eq!(rect.min, egui::pos2(12.0, 34.0));
        assert_eq!(rect.size(), egui::Vec2::ZERO);
    }

    #[test]
    fn context_menu_submenu_reuses_dropdown_submenu_contract() {
        let trigger = primitive_context_menu_sub_trigger_output(
            ContextMenuSubTriggerOptions::new(160.0)
                .open(true)
                .direction(ContextMenuDirection::Ltr),
        );
        let content = primitive_context_menu_sub_content_output(
            ContextMenuSubContentOptions::new(200.0)
                .open(true)
                .side_align(ContextMenuSide::Right, ContextMenuAlign::Start),
        );
        let action =
            context_menu_sub_keyboard_action(ContextMenuDirection::Ltr, false, true, false);
        let delay = context_menu_sub_delay_output(true, 150, 100);
        let grace = context_menu_sub_pointer_grace_output(
            Rect::from_min_size(egui::pos2(10.0, 10.0), egui::vec2(120.0, 28.0)),
            Rect::from_min_size(egui::pos2(150.0, 8.0), egui::vec2(180.0, 140.0)),
            egui::pos2(140.0, 40.0),
            ContextMenuSide::Right,
            8.0,
        );
        let dismiss = context_menu_sub_dismiss_output(
            Rect::from_min_size(egui::pos2(10.0, 10.0), egui::vec2(120.0, 160.0)),
            Rect::from_min_size(egui::pos2(140.0, 20.0), egui::vec2(160.0, 120.0)),
            egui::pos2(400.0, 40.0),
        );

        assert_eq!(trigger.data_state, ContextMenuDataState::Open);
        assert_eq!(content.side, ContextMenuSide::Right);
        assert_eq!(content.data_state, ContextMenuDataState::Open);
        assert_eq!(action, ContextMenuSubKeyboardAction::OpenSubmenu);
        assert!(delay.should_open);
        assert!(grace.pointer_in_grace_area);
        assert!(dismiss.should_close_submenu);
        assert!(dismiss.should_close_root);
    }

    #[test]
    fn context_menu_options_preserve_content_contract() {
        let options = ContextMenuOptions::at("context_options_test", egui::pos2(12.0, 34.0), 156.0)
            .max_height(220.0)
            .inner_margin(egui::Margin::symmetric(4, 5));

        let content = primitive_context_menu_content_options(&options);

        assert_eq!(content.width, 156.0);
        assert_eq!(content.max_height, Some(220.0));
        assert_eq!(content.inner_margin.left, 4);
        assert_eq!(content.inner_margin.top, 5);
        assert!(!content.loop_focus);
        assert!(!content.force_mount);
        assert_eq!(content.side, ContextMenuSide::Bottom);
        assert_eq!(content.align, ContextMenuAlign::Start);
        assert_eq!(content.data_state, ContextMenuDataState::Open);
    }

    #[test]
    fn context_menu_layer_options_use_pointer_position_and_dismiss_policy() {
        let options = ContextMenuOptions::at("context_layer_test", egui::pos2(12.0, 34.0), 156.0);

        let layer = primitive_context_menu_layer_options(&options);

        assert_eq!(
            layer.placement,
            LayerPlacement::Fixed(egui::pos2(12.0, 34.0))
        );
        assert_eq!(layer.order, egui::Order::Tooltip);
        assert_eq!(layer.dismiss_policy, DismissPolicy::OutsideClickAndEscape);
    }

    #[test]
    fn context_menu_item_options_preserve_item_state() {
        let options = ContextMenuItemOptions::new(132.0)
            .selected(true)
            .highlighted(true)
            .checked(true)
            .disabled(true);

        assert_eq!(options.width, 132.0);
        assert!(options.selected);
        assert!(options.highlighted);
        assert!(options.checked);
        assert!(options.disabled);
    }

    #[test]
    fn context_menu_typeahead_match_uses_shared_menu_item_contract() {
        let items = [
            ContextMenuItem {
                value: "copy",
                label: "복사",
                enabled: true,
            },
            ContextMenuItem {
                value: "hide",
                label: "숨기기",
                enabled: true,
            },
            ContextMenuItem {
                value: "delete",
                label: "삭제",
                enabled: false,
            },
        ];

        assert_eq!(
            context_menu_typeahead_match(&items, Some("copy"), "숨"),
            Some("hide")
        );
        assert_eq!(
            context_menu_typeahead_match(&items, Some("hide"), "복"),
            Some("copy")
        );
        assert_eq!(
            context_menu_typeahead_match(&items, Some("copy"), "삭"),
            None
        );
    }

    #[test]
    fn context_menu_roving_focus_output_uses_shared_vertical_menu_contract() {
        let items = [
            ContextMenuItem {
                value: "copy",
                label: "복사",
                enabled: true,
            },
            ContextMenuItem {
                value: "paste",
                label: "붙여넣기",
                enabled: false,
            },
            ContextMenuItem {
                value: "delete",
                label: "삭제",
                enabled: true,
            },
        ];

        let output = context_menu_roving_focus_output(
            &items,
            Some(0),
            Some(RovingFocusKey::ArrowDown),
            true,
        );
        let activation =
            context_menu_roving_focus_output(&items, Some(2), Some(RovingFocusKey::Space), true);
        let close =
            context_menu_roving_focus_output(&items, Some(2), Some(RovingFocusKey::Escape), true);

        assert_eq!(output.active_index, Some(2));
        assert_eq!(output.action, RovingFocusAction::Moved);
        assert_eq!(output.item_tab_indices, vec![-1, -1, 0]);
        assert_eq!(activation.action, RovingFocusAction::Activate);
        assert_eq!(close.action, RovingFocusAction::Close);
    }

    #[test]
    fn context_menu_root_output_preserves_radix_contract() {
        let output = primitive_context_menu_root_output(
            ContextMenuRootOptions::default()
                .open(true)
                .modal(false)
                .direction(ContextMenuDirection::Rtl),
        );

        assert!(output.open);
        assert!(!output.modal);
        assert_eq!(output.direction, Some(ContextMenuDirection::Rtl));
        assert_eq!(output.data_state, ContextMenuDataState::Open);
        assert_eq!(output.data_state.as_str(), "open");
        assert_eq!(ContextMenuDirection::Rtl.as_str(), "rtl");
    }

    #[test]
    fn context_menu_apply_open_preserves_pointer_position_and_noop_state() {
        let options = ContextMenuRootOptions::default().open(false).modal(true);
        let mut position = None;

        assert!(!context_menu_apply_open(&mut position, None, &options));
        assert_eq!(position, None);
        assert!(context_menu_apply_open(
            &mut position,
            Some(egui::pos2(12.0, 34.0)),
            &options,
        ));
        assert_eq!(position, Some(egui::pos2(12.0, 34.0)));
        assert!(!context_menu_apply_open(
            &mut position,
            Some(egui::pos2(12.0, 34.0)),
            &options,
        ));
        assert!(context_menu_apply_open(&mut position, None, &options));
        assert_eq!(position, None);
    }

    #[test]
    fn context_menu_pointer_open_output_preserves_pointer_origin() {
        let output = context_menu_pointer_open_output(egui::pos2(12.0, 34.0), false);
        let disabled = context_menu_pointer_open_output(egui::pos2(12.0, 34.0), true);

        assert!(output.open);
        assert_eq!(output.position, Some(egui::pos2(12.0, 34.0)));
        assert_eq!(output.origin, Some(ContextMenuOpenOrigin::Pointer));
        assert_eq!(output.data_origin, Some("pointer"));
        assert!(!disabled.open);
        assert_eq!(disabled.position, None);
    }

    #[test]
    fn context_menu_long_press_open_output_respects_threshold_and_movement() {
        let start = egui::pos2(12.0, 34.0);

        let waiting = context_menu_long_press_open_output(
            start,
            egui::pos2(13.0, 35.0),
            399,
            400,
            8.0,
            false,
        );
        let moved_too_far = context_menu_long_press_open_output(
            start,
            egui::pos2(24.0, 34.0),
            400,
            400,
            8.0,
            false,
        );
        let opened = context_menu_long_press_open_output(
            start,
            egui::pos2(13.0, 35.0),
            400,
            400,
            8.0,
            false,
        );

        assert!(!waiting.open);
        assert!(!moved_too_far.open);
        assert!(opened.open);
        assert_eq!(opened.position, Some(start));
        assert_eq!(opened.origin, Some(ContextMenuOpenOrigin::TouchLongPress));
        assert_eq!(opened.data_origin, Some("touch-long-press"));
    }

    #[test]
    fn context_menu_keyboard_open_output_uses_trigger_center_for_context_key_or_shift_f10() {
        let trigger = Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(80.0, 24.0));
        let context_key = context_menu_keyboard_open_output(trigger, true, false, false);
        let shift_f10 = context_menu_keyboard_open_output(trigger, false, true, false);
        let disabled = context_menu_keyboard_open_output(trigger, true, true, true);

        assert!(context_key.open);
        assert_eq!(context_key.position, Some(trigger.center()));
        assert_eq!(context_key.origin, Some(ContextMenuOpenOrigin::Keyboard));
        assert_eq!(context_key.data_origin, Some("keyboard"));
        assert_eq!(shift_f10.position, Some(trigger.center()));
        assert!(!disabled.open);
        assert_eq!(disabled.position, None);
    }

    #[test]
    fn context_menu_portal_output_preserves_force_mount_and_container() {
        let output = primitive_context_menu_portal_output(
            ContextMenuPortalOptions::default()
                .force_mount(true)
                .container("context-menu-layer"),
        );

        assert!(output.force_mount);
        assert_eq!(output.container.as_deref(), Some("context-menu-layer"));
    }

    #[test]
    fn context_menu_content_output_preserves_state_side_align_and_mount() {
        let options =
            ContextMenuOptions::at("context_content_output_test", egui::pos2(12.0, 34.0), 156.0)
                .max_height(220.0);
        let content = primitive_context_menu_content_options(&options)
            .open(false)
            .force_mount(true)
            .loop_focus(true)
            .side_align(ContextMenuSide::Right, ContextMenuAlign::End);
        let output = primitive_context_menu_content_output(content);

        assert_eq!(output.width, 156.0);
        assert_eq!(output.max_height, Some(220.0));
        assert!(!output.open);
        assert!(output.loop_focus);
        assert!(output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.side, ContextMenuSide::Right);
        assert_eq!(output.align, ContextMenuAlign::End);
        assert_eq!(output.data_state, ContextMenuDataState::Closed);
        assert_eq!(output.data_state.as_str(), "closed");
        assert_eq!(output.side.as_str(), "right");
        assert_eq!(output.align.as_str(), "end");
    }

    #[test]
    fn context_menu_trigger_options_preserve_root_state_and_disabled() {
        let options = ContextMenuTriggerOptions::default()
            .size(96.0, 28.0)
            .open(true)
            .disabled(true);

        assert_eq!(options.width, 96.0);
        assert_eq!(options.height, 28.0);
        assert!(options.open);
        assert!(options.disabled);
    }

    #[test]
    fn context_menu_item_options_convert_to_shared_menu_options() {
        let theme = PrimitiveTheme {
            menu_row_height: 36.0,
            ..PrimitiveTheme::default()
        };
        let mut options = ContextMenuItemOptions::new(132.0)
            .selected(true)
            .highlighted(true)
            .checked(true)
            .disabled(true);
        options.theme = theme;

        let menu_options = context_menu_item_to_menu_options(options);

        assert_eq!(menu_options.width, 132.0);
        assert!(menu_options.selected);
        assert!(menu_options.highlighted);
        assert!(menu_options.checked);
        assert!(menu_options.disabled);
        assert_eq!(menu_options.theme.menu_row_height, 36.0);
    }

    #[test]
    fn context_menu_label_and_separator_options_preserve_width() {
        let label_width: ContextMenuLabelOptions = 132.0;
        let separator_width: ContextMenuSeparatorOptions = 132.0;

        assert_eq!(label_width, 132.0);
        assert_eq!(separator_width, 132.0);
    }
}
