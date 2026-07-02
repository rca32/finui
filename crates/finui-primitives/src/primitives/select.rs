use std::hash::Hash;

use eframe::egui::{self, Align2, Color32, FontId, Rect, Response, Sense, Stroke, Vec2};

use super::{
    DropdownMenuAlign, DropdownMenuDataState, DropdownMenuDirection, DropdownMenuOptions,
    DropdownMenuOutput, DropdownMenuSide, MenuItemOptions, PrimitiveTheme, RadixIcon,
    dropdown_menu_placement_parts, paint_radix_icon, primitive_horizontal_arrow_step,
    primitive_menu_item, primitive_menu_label, primitive_menu_separator, radix_colors,
    show_dropdown_menu,
};
use crate::LayerPlacement;

pub type SelectDataState = DropdownMenuDataState;
pub type SelectDirection = DropdownMenuDirection;
pub type SelectSide = DropdownMenuSide;
pub type SelectAlign = DropdownMenuAlign;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectPosition {
    ItemAligned,
    Popper,
}

impl SelectPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ItemAligned => "item-aligned",
            Self::Popper => "popper",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectRootOptions {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub open: bool,
    pub default_open: Option<bool>,
    pub direction: Option<SelectDirection>,
    pub name: Option<String>,
    pub disabled: bool,
    pub required: bool,
}

impl Default for SelectRootOptions {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            open: false,
            default_open: None,
            direction: None,
            name: None,
            disabled: false,
            required: false,
        }
    }
}

impl SelectRootOptions {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    pub fn direction(mut self, direction: SelectDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectRootOutput {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub open: bool,
    pub default_open: Option<bool>,
    pub direction: Option<SelectDirection>,
    pub name: Option<String>,
    pub disabled: bool,
    pub required: bool,
    pub data_state: SelectDataState,
}

pub fn primitive_select_root_output(options: SelectRootOptions) -> SelectRootOutput {
    SelectRootOutput {
        value: options.value,
        default_value: options.default_value,
        open: options.open,
        default_open: options.default_open,
        direction: options.direction,
        name: options.name,
        disabled: options.disabled,
        required: options.required,
        data_state: if options.open {
            SelectDataState::Open
        } else {
            SelectDataState::Closed
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectPortalOptions {
    pub container: Option<String>,
}

impl Default for SelectPortalOptions {
    fn default() -> Self {
        Self { container: None }
    }
}

impl SelectPortalOptions {
    pub fn container(mut self, container: impl Into<String>) -> Self {
        self.container = Some(container.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectPortalOutput {
    pub container: Option<String>,
}

pub fn primitive_select_portal_output(options: SelectPortalOptions) -> SelectPortalOutput {
    SelectPortalOutput {
        container: options.container,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectItem<T> {
    pub value: T,
    pub label: &'static str,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectKeyboardAction {
    None,
    Open,
    FocusNext,
    FocusPrevious,
    FocusFirst,
    FocusLast,
    Commit,
    Cancel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectKeyboardOutput<T> {
    pub open: bool,
    pub active_value: Option<T>,
    pub committed_value: Option<T>,
    pub committed: bool,
    pub cancelled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectValueTextOutput {
    pub text: String,
    pub placeholder: bool,
}

pub struct SelectOptions {
    pub id: egui::Id,
    pub trigger_rect: Rect,
    pub width: f32,
    pub placement: LayerPlacement,
    pub max_height: Option<f32>,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectContentOptions {
    pub width: f32,
    pub max_height: Option<f32>,
    pub inner_margin: egui::Margin,
    pub open: bool,
    pub position: SelectPosition,
    pub side: SelectSide,
    pub align: SelectAlign,
    pub data_state: SelectDataState,
    pub theme: PrimitiveTheme,
}

impl SelectContentOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self.data_state = if open {
            SelectDataState::Open
        } else {
            SelectDataState::Closed
        };
        self
    }

    pub fn position(mut self, position: SelectPosition) -> Self {
        self.position = position;
        self
    }

    pub fn side_align(mut self, side: SelectSide, align: SelectAlign) -> Self {
        self.side = side;
        self.align = align;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectContentOutput {
    pub width: f32,
    pub max_height: Option<f32>,
    pub open: bool,
    pub mounted: bool,
    pub position: SelectPosition,
    pub side: SelectSide,
    pub align: SelectAlign,
    pub data_state: SelectDataState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectViewportOptions {
    pub width: f32,
    pub item_gap: f32,
    pub theme: PrimitiveTheme,
}

impl SelectViewportOptions {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            item_gap: 0.0,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn item_gap(mut self, item_gap: f32) -> Self {
        self.item_gap = item_gap;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SelectTriggerOptions {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub enabled: bool,
    pub placeholder: bool,
    pub theme: PrimitiveTheme,
}

impl Default for SelectTriggerOptions {
    fn default() -> Self {
        Self {
            width: 112.0,
            height: 32.0,
            open: false,
            enabled: true,
            placeholder: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl SelectTriggerOptions {
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
        self.enabled = !disabled;
        self
    }

    pub fn placeholder(mut self, placeholder: bool) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug)]
pub struct SelectTriggerOutput {
    pub response: Response,
    pub rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectItemOptions {
    pub width: f32,
    pub selected: bool,
    pub highlighted: bool,
    pub checked: bool,
    pub disabled: bool,
    pub theme: PrimitiveTheme,
}

impl SelectItemOptions {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectLabelOptions {
    pub width: f32,
}

impl SelectLabelOptions {
    pub fn new(width: f32) -> Self {
        Self { width }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelectSeparatorOptions {
    pub width: f32,
}

impl SelectSeparatorOptions {
    pub fn new(width: f32) -> Self {
        Self { width }
    }
}

impl SelectOptions {
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

    pub fn content_options(&self) -> SelectContentOptions {
        let (side, align) = dropdown_menu_placement_parts(self.placement);
        SelectContentOptions {
            width: self.width,
            max_height: self.max_height,
            inner_margin: self.inner_margin,
            open: true,
            position: SelectPosition::Popper,
            side,
            align,
            data_state: SelectDataState::Open,
            theme: self.theme,
        }
    }
}

pub fn show_select<T: Copy + PartialEq>(
    ctx: &egui::Context,
    options: SelectOptions,
    add_items: impl FnOnce(&mut egui::Ui) -> Option<T>,
) -> DropdownMenuOutput<T> {
    let menu = primitive_select_content_options(&options);
    show_dropdown_menu(ctx, menu, add_items)
}

pub fn primitive_select_content_output(options: SelectContentOptions) -> SelectContentOutput {
    SelectContentOutput {
        width: options.width,
        max_height: options.max_height,
        open: options.open,
        mounted: options.open,
        position: options.position,
        side: options.side,
        align: options.align,
        data_state: options.data_state,
    }
}

pub fn primitive_select_content_options(options: &SelectOptions) -> DropdownMenuOptions {
    let content = options.content_options();
    let mut menu = DropdownMenuOptions::anchored(
        options.id,
        options.trigger_rect,
        content.width,
        options.placement,
    )
    .inner_margin(content.inner_margin);
    if let Some(max_height) = content.max_height {
        menu = menu.max_height(max_height);
    }
    menu.theme = content.theme;
    menu
}

pub fn primitive_select_viewport<T>(
    ui: &mut egui::Ui,
    options: SelectViewportOptions,
    add_items: impl FnOnce(&mut egui::Ui, SelectViewportOptions) -> Option<T>,
) -> Option<T> {
    ui.set_min_width(options.width);
    if options.item_gap > 0.0 {
        ui.spacing_mut().item_spacing.y = options.item_gap;
    }
    add_items(ui, options)
}

pub fn primitive_select_item(
    ui: &mut egui::Ui,
    label: &'static str,
    options: SelectItemOptions,
) -> Response {
    let mut item_options = MenuItemOptions::new(options.width)
        .selected(options.selected)
        .highlighted(options.highlighted)
        .checked(options.checked)
        .disabled(options.disabled);
    item_options.theme = options.theme;
    primitive_menu_item(ui, label, item_options)
}

pub fn primitive_select_label(ui: &mut egui::Ui, label: &str, options: SelectLabelOptions) {
    primitive_menu_label(ui, label, options.width);
}

pub fn primitive_select_separator(ui: &mut egui::Ui, options: SelectSeparatorOptions) {
    primitive_menu_separator(ui, options.width);
}

pub fn primitive_select_trigger(
    ui: &mut egui::Ui,
    _id_source: impl Hash,
    value: &str,
    options: SelectTriggerOptions,
) -> SelectTriggerOutput {
    let root = primitive_select_root_output(
        SelectRootOptions::default()
            .open(options.open)
            .disabled(!options.enabled),
    );
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
    let open = root.data_state == SelectDataState::Open;
    let fill = if !options.enabled {
        select_disabled_fill(options.theme)
    } else if open {
        select_active_fill(options.theme)
    } else if response.hovered() {
        select_hover_fill(options.theme)
    } else {
        options.theme.content_fill
    };
    let stroke_color = if !options.enabled {
        select_idle_stroke(options.theme)
    } else if open {
        select_active_stroke(options.theme)
    } else if response.hovered() {
        select_hover_stroke(options.theme)
    } else {
        options.theme.content_stroke.color
    };
    ui.painter().rect(
        rect,
        options.theme.row_radius,
        fill,
        Stroke::new(1.0, stroke_color),
        egui::StrokeKind::Inside,
    );
    primitive_select_value(
        ui,
        Rect::from_min_max(
            rect.left_top() + Vec2::new(10.0, 0.0),
            rect.right_bottom() - Vec2::new(30.0, 0.0),
        ),
        value,
        options.placeholder,
        options.theme,
    );
    primitive_select_icon(ui, rect, options.open, options.enabled, options.theme);
    SelectTriggerOutput { response, rect }
}

pub fn primitive_select_value(
    ui: &egui::Ui,
    rect: Rect,
    value: &str,
    placeholder: bool,
    theme: PrimitiveTheme,
) {
    let color = if placeholder {
        theme.muted_text
    } else {
        theme.text
    };
    ui.painter().with_clip_rect(rect).text(
        rect.left_center(),
        Align2::LEFT_CENTER,
        value,
        crate::scaled_proportional_font(ui, 13.0),
        color,
    );
}

pub fn primitive_select_icon(
    ui: &egui::Ui,
    trigger_rect: Rect,
    open: bool,
    enabled: bool,
    theme: PrimitiveTheme,
) {
    let center = trigger_rect.right_center() - Vec2::new(14.0, 0.0);
    let color = if !enabled {
        theme.disabled_text
    } else if open {
        select_active_text(theme)
    } else {
        theme.muted_text
    };
    let icon = if open {
        RadixIcon::ChevronUp
    } else {
        RadixIcon::ChevronDown
    };
    let icon_rect = Rect::from_center_size(center, Vec2::splat(15.0));
    paint_radix_icon(ui, icon, icon_rect, color);
}

fn is_dark_primitive_theme(theme: PrimitiveTheme) -> bool {
    let fill = theme.content_fill;
    u16::from(fill.r()) + u16::from(fill.g()) + u16::from(fill.b()) < 160
}

fn select_disabled_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x10, 0x14, 0x1b)
    } else {
        radix_colors::SLATE_2
    }
}

fn select_hover_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x1f, 0x28, 0x36)
    } else {
        radix_colors::SLATE_2
    }
}

fn select_active_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x23, 0x34, 0x70)
    } else {
        radix_colors::INDIGO_3
    }
}

fn select_idle_stroke(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x4a, 0x55, 0x68)
    } else {
        radix_colors::SLATE_5
    }
}

fn select_hover_stroke(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x68, 0x78, 0x95)
    } else {
        radix_colors::SLATE_7
    }
}

fn select_active_stroke(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x8a, 0xa2, 0xff)
    } else {
        radix_colors::INDIGO_8
    }
}

fn select_active_text(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0xee, 0xf3, 0xff)
    } else {
        radix_colors::INDIGO_11
    }
}

pub fn select_next_enabled<T: Copy + PartialEq>(
    items: &[SelectItem<T>],
    current: Option<T>,
    direction: isize,
) -> Option<T> {
    if items.is_empty() || !items.iter().any(|item| item.enabled) {
        return None;
    }
    let len = items.len() as isize;
    let start = current
        .and_then(|value| items.iter().position(|item| item.value == value))
        .map(|index| index as isize)
        .unwrap_or(if direction >= 0 { -1 } else { len });
    for step in 1..=items.len() {
        let index = (start + direction * step as isize).rem_euclid(len) as usize;
        if items[index].enabled {
            return Some(items[index].value);
        }
    }
    None
}

pub fn select_horizontal_next_enabled<T: Copy + PartialEq>(
    items: &[SelectItem<T>],
    current: Option<T>,
    direction: Option<SelectDirection>,
    arrow_left_pressed: bool,
    arrow_right_pressed: bool,
) -> Option<T> {
    let primitive_direction = direction.map(|direction| match direction {
        SelectDirection::Ltr => super::PrimitiveDirection::Ltr,
        SelectDirection::Rtl => super::PrimitiveDirection::Rtl,
    });
    primitive_horizontal_arrow_step(primitive_direction, arrow_left_pressed, arrow_right_pressed)
        .and_then(|step| select_next_enabled(items, current, step))
}

#[allow(clippy::too_many_arguments)]
pub fn select_keyboard_action(
    open: bool,
    disabled: bool,
    enter_pressed: bool,
    space_pressed: bool,
    escape_pressed: bool,
    arrow_up_pressed: bool,
    arrow_down_pressed: bool,
    home_pressed: bool,
    end_pressed: bool,
) -> SelectKeyboardAction {
    if disabled {
        return SelectKeyboardAction::None;
    }
    if open {
        if escape_pressed {
            SelectKeyboardAction::Cancel
        } else if enter_pressed || space_pressed {
            SelectKeyboardAction::Commit
        } else if home_pressed {
            SelectKeyboardAction::FocusFirst
        } else if end_pressed {
            SelectKeyboardAction::FocusLast
        } else if arrow_down_pressed {
            SelectKeyboardAction::FocusNext
        } else if arrow_up_pressed {
            SelectKeyboardAction::FocusPrevious
        } else {
            SelectKeyboardAction::None
        }
    } else if enter_pressed || space_pressed || arrow_down_pressed || arrow_up_pressed {
        SelectKeyboardAction::Open
    } else {
        SelectKeyboardAction::None
    }
}

pub fn select_open_active_value<T: Copy + PartialEq>(
    items: &[SelectItem<T>],
    selected: Option<T>,
) -> Option<T> {
    selected
        .filter(|value| {
            items
                .iter()
                .any(|item| item.enabled && item.value == *value)
        })
        .or_else(|| {
            items
                .iter()
                .find(|item| item.enabled)
                .map(|item| item.value)
        })
}

pub fn select_keyboard_target_value<T: Copy + PartialEq>(
    items: &[SelectItem<T>],
    active: Option<T>,
    action: SelectKeyboardAction,
    loop_focus: bool,
) -> Option<T> {
    match action {
        SelectKeyboardAction::FocusNext => {
            select_next_enabled_with_loop(items, active, 1, loop_focus)
        }
        SelectKeyboardAction::FocusPrevious => {
            select_next_enabled_with_loop(items, active, -1, loop_focus)
        }
        SelectKeyboardAction::FocusFirst => items
            .iter()
            .find(|item| item.enabled)
            .map(|item| item.value),
        SelectKeyboardAction::FocusLast => items
            .iter()
            .rev()
            .find(|item| item.enabled)
            .map(|item| item.value),
        _ => active,
    }
}

pub fn select_keyboard_output<T: Copy + PartialEq>(
    items: &[SelectItem<T>],
    current_value: Option<T>,
    active_value: Option<T>,
    open: bool,
    action: SelectKeyboardAction,
    loop_focus: bool,
) -> SelectKeyboardOutput<T> {
    match action {
        SelectKeyboardAction::Open => SelectKeyboardOutput {
            open: true,
            active_value: select_open_active_value(items, current_value),
            committed_value: current_value,
            committed: false,
            cancelled: false,
        },
        SelectKeyboardAction::FocusNext
        | SelectKeyboardAction::FocusPrevious
        | SelectKeyboardAction::FocusFirst
        | SelectKeyboardAction::FocusLast => SelectKeyboardOutput {
            open,
            active_value: select_keyboard_target_value(items, active_value, action, loop_focus),
            committed_value: current_value,
            committed: false,
            cancelled: false,
        },
        SelectKeyboardAction::Commit => SelectKeyboardOutput {
            open: false,
            active_value,
            committed_value: active_value.or(current_value),
            committed: active_value.is_some(),
            cancelled: false,
        },
        SelectKeyboardAction::Cancel => SelectKeyboardOutput {
            open: false,
            active_value: current_value,
            committed_value: current_value,
            committed: false,
            cancelled: true,
        },
        SelectKeyboardAction::None => SelectKeyboardOutput {
            open,
            active_value,
            committed_value: current_value,
            committed: false,
            cancelled: false,
        },
    }
}

fn select_next_enabled_with_loop<T: Copy + PartialEq>(
    items: &[SelectItem<T>],
    current: Option<T>,
    direction: isize,
    loop_focus: bool,
) -> Option<T> {
    if loop_focus {
        return select_next_enabled(items, current, direction);
    }
    if items.is_empty() || !items.iter().any(|item| item.enabled) {
        return None;
    }
    let len = items.len() as isize;
    let start = current
        .and_then(|value| items.iter().position(|item| item.value == value))
        .map(|index| index as isize)
        .unwrap_or(if direction >= 0 { -1 } else { len });
    for step in 1..=items.len() {
        let raw = start + direction * step as isize;
        if raw < 0 || raw >= len {
            return None;
        }
        let index = raw as usize;
        if items[index].enabled {
            return Some(items[index].value);
        }
    }
    None
}

pub fn select_value_text_output(
    value: Option<impl Into<String>>,
    placeholder: impl Into<String>,
) -> SelectValueTextOutput {
    match value {
        Some(value) => SelectValueTextOutput {
            text: value.into(),
            placeholder: false,
        },
        None => SelectValueTextOutput {
            text: placeholder.into(),
            placeholder: true,
        },
    }
}

pub fn select_typeahead_match<T: Copy + PartialEq>(
    items: &[SelectItem<T>],
    current: Option<T>,
    query: &str,
) -> Option<T> {
    let query = query.trim();
    if query.is_empty() {
        return None;
    }
    let query = query.to_ascii_lowercase();
    let start_index = current
        .and_then(|value| items.iter().position(|item| item.value == value))
        .unwrap_or(items.len().saturating_sub(1));
    for step in 1..=items.len() {
        let index = (start_index + step) % items.len();
        let item = &items[index];
        if item.enabled && item.label.to_ascii_lowercase().starts_with(&query) {
            return Some(item.value);
        }
    }
    None
}

pub fn select_apply_open(current: &mut bool, next: bool, options: &SelectRootOptions) -> bool {
    if options.disabled || *current == next {
        return false;
    }
    *current = next;
    true
}

pub fn select_apply_value<T: Copy + PartialEq>(
    current: &mut T,
    next: T,
    items: &[SelectItem<T>],
    options: &SelectRootOptions,
) -> bool {
    if options.disabled || *current == next {
        return false;
    }
    if !items.iter().any(|item| item.enabled && item.value == next) {
        return false;
    }
    *current = next;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_next_enabled_skips_disabled_items() {
        let items = [
            SelectItem {
                value: 1,
                label: "A",
                enabled: false,
            },
            SelectItem {
                value: 2,
                label: "B",
                enabled: true,
            },
            SelectItem {
                value: 3,
                label: "C",
                enabled: true,
            },
        ];

        assert_eq!(select_next_enabled(&items, None, 1), Some(2));
        assert_eq!(select_next_enabled(&items, Some(3), 1), Some(2));
    }

    #[test]
    fn select_horizontal_next_enabled_reverses_arrow_direction_in_rtl() {
        let items = [
            SelectItem {
                value: 1,
                label: "A",
                enabled: true,
            },
            SelectItem {
                value: 2,
                label: "B",
                enabled: true,
            },
            SelectItem {
                value: 3,
                label: "C",
                enabled: true,
            },
        ];

        assert_eq!(
            select_horizontal_next_enabled(
                &items,
                Some(2),
                Some(SelectDirection::Ltr),
                false,
                true
            ),
            Some(3)
        );
        assert_eq!(
            select_horizontal_next_enabled(
                &items,
                Some(2),
                Some(SelectDirection::Rtl),
                false,
                true
            ),
            Some(1)
        );
        assert_eq!(
            select_horizontal_next_enabled(
                &items,
                Some(2),
                Some(SelectDirection::Rtl),
                true,
                false
            ),
            Some(3)
        );
    }

    #[test]
    fn select_keyboard_output_focuses_selected_item_when_opening() {
        let items = [
            SelectItem {
                value: "1m",
                label: "1M",
                enabled: true,
            },
            SelectItem {
                value: "3m",
                label: "3M",
                enabled: true,
            },
            SelectItem {
                value: "6m",
                label: "6M",
                enabled: true,
            },
        ];

        let action =
            select_keyboard_action(false, false, false, true, false, false, false, false, false);
        let output = select_keyboard_output(&items, Some("3m"), None, false, action, true);

        assert_eq!(action, SelectKeyboardAction::Open);
        assert!(output.open);
        assert_eq!(output.active_value, Some("3m"));
        assert_eq!(output.committed_value, Some("3m"));
        assert!(!output.committed);
    }

    #[test]
    fn select_keyboard_output_skips_disabled_items_for_arrow_home_and_end() {
        let items = [
            SelectItem {
                value: "1m",
                label: "1M",
                enabled: true,
            },
            SelectItem {
                value: "3m",
                label: "3M",
                enabled: false,
            },
            SelectItem {
                value: "6m",
                label: "6M",
                enabled: true,
            },
        ];

        let next = select_keyboard_output(
            &items,
            Some("1m"),
            Some("1m"),
            true,
            SelectKeyboardAction::FocusNext,
            true,
        );
        let previous_no_loop = select_keyboard_output(
            &items,
            Some("1m"),
            Some("1m"),
            true,
            SelectKeyboardAction::FocusPrevious,
            false,
        );
        let last = select_keyboard_output(
            &items,
            Some("1m"),
            Some("1m"),
            true,
            SelectKeyboardAction::FocusLast,
            true,
        );

        assert_eq!(next.active_value, Some("6m"));
        assert_eq!(previous_no_loop.active_value, None);
        assert_eq!(last.active_value, Some("6m"));
    }

    #[test]
    fn select_keyboard_output_commits_enter_and_cancels_escape() {
        let items = [
            SelectItem {
                value: "1m",
                label: "1M",
                enabled: true,
            },
            SelectItem {
                value: "3m",
                label: "3M",
                enabled: true,
            },
        ];
        let commit_action =
            select_keyboard_action(true, false, true, false, false, false, false, false, false);
        let cancel_action =
            select_keyboard_action(true, false, false, false, true, false, false, false, false);
        let committed =
            select_keyboard_output(&items, Some("1m"), Some("3m"), true, commit_action, true);
        let cancelled =
            select_keyboard_output(&items, Some("1m"), Some("3m"), true, cancel_action, true);

        assert_eq!(commit_action, SelectKeyboardAction::Commit);
        assert!(!committed.open);
        assert_eq!(committed.committed_value, Some("3m"));
        assert!(committed.committed);
        assert_eq!(cancel_action, SelectKeyboardAction::Cancel);
        assert!(!cancelled.open);
        assert_eq!(cancelled.committed_value, Some("1m"));
        assert_eq!(cancelled.active_value, Some("1m"));
        assert!(cancelled.cancelled);
    }

    #[test]
    fn select_value_text_output_separates_placeholder_from_value() {
        let placeholder = select_value_text_output(None::<String>, "기간 선택");
        let value = select_value_text_output(Some("3M"), "기간 선택");

        assert_eq!(placeholder.text, "기간 선택");
        assert!(placeholder.placeholder);
        assert_eq!(value.text, "3M");
        assert!(!value.placeholder);
    }

    #[test]
    fn select_typeahead_match_wraps_from_current_and_skips_disabled_items() {
        let items = [
            SelectItem {
                value: "1m",
                label: "1분",
                enabled: true,
            },
            SelectItem {
                value: "5m",
                label: "5분",
                enabled: true,
            },
            SelectItem {
                value: "1h",
                label: "1시간",
                enabled: false,
            },
            SelectItem {
                value: "1d",
                label: "1일",
                enabled: true,
            },
        ];

        assert_eq!(select_typeahead_match(&items, Some("1m"), "5"), Some("5m"));
        assert_eq!(select_typeahead_match(&items, Some("5m"), "1"), Some("1d"));
        assert_eq!(select_typeahead_match(&items, Some("1d"), "1"), Some("1m"));
    }

    #[test]
    fn select_apply_open_respects_disabled_and_noop_state() {
        let mut open = false;

        assert!(select_apply_open(
            &mut open,
            true,
            &SelectRootOptions::default()
        ));
        assert!(open);
        assert!(!select_apply_open(
            &mut open,
            true,
            &SelectRootOptions::default().open(true)
        ));
        assert!(!select_apply_open(
            &mut open,
            false,
            &SelectRootOptions::default().disabled(true)
        ));
        assert!(open);
    }

    #[test]
    fn select_apply_value_respects_enabled_items_disabled_root_and_noop_state() {
        let items = [
            SelectItem {
                value: "1m",
                label: "1분",
                enabled: true,
            },
            SelectItem {
                value: "5m",
                label: "5분",
                enabled: true,
            },
            SelectItem {
                value: "1h",
                label: "1시간",
                enabled: false,
            },
        ];
        let mut value = "1m";

        assert!(select_apply_value(
            &mut value,
            "5m",
            &items,
            &SelectRootOptions::default()
        ));
        assert_eq!(value, "5m");
        assert!(!select_apply_value(
            &mut value,
            "5m",
            &items,
            &SelectRootOptions::default()
        ));
        assert!(!select_apply_value(
            &mut value,
            "1h",
            &items,
            &SelectRootOptions::default()
        ));
        assert_eq!(value, "5m");
        assert!(!select_apply_value(
            &mut value,
            "1m",
            &items,
            &SelectRootOptions::default().disabled(true)
        ));
        assert_eq!(value, "5m");
    }

    #[test]
    fn select_options_preserve_popup_sizing_contract() {
        let options = SelectOptions::anchored(
            "select_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        )
        .inner_margin(egui::Margin::symmetric(4, 5))
        .max_height(320.0);

        assert_eq!(options.width, 180.0);
        assert_eq!(options.max_height, Some(320.0));
        assert_eq!(options.inner_margin.left, 4);
        assert_eq!(options.inner_margin.top, 5);
        assert_eq!(options.content_options().width, 180.0);
        assert_eq!(options.content_options().max_height, Some(320.0));
        assert_eq!(options.content_options().position, SelectPosition::Popper);
        assert_eq!(options.content_options().side, SelectSide::Bottom);
        assert_eq!(options.content_options().align, SelectAlign::Start);
        assert_eq!(options.content_options().data_state, SelectDataState::Open);
    }

    #[test]
    fn select_content_options_convert_to_dropdown_contract() {
        let options = SelectOptions::anchored(
            "select_content_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        )
        .inner_margin(egui::Margin::symmetric(4, 5))
        .max_height(320.0);

        let menu = primitive_select_content_options(&options);

        assert_eq!(menu.width, 180.0);
        assert_eq!(menu.max_height, Some(320.0));
        assert_eq!(menu.inner_margin.left, 4);
        assert_eq!(menu.inner_margin.top, 5);
    }

    #[test]
    fn select_root_output_preserves_radix_contract() {
        let output = primitive_select_root_output(
            SelectRootOptions::default()
                .value("5m")
                .default_value("1m")
                .open(true)
                .default_open(false)
                .direction(SelectDirection::Rtl)
                .name("timeframe")
                .disabled(true)
                .required(true),
        );

        assert_eq!(output.value.as_deref(), Some("5m"));
        assert_eq!(output.default_value.as_deref(), Some("1m"));
        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert_eq!(output.direction, Some(SelectDirection::Rtl));
        assert_eq!(output.name.as_deref(), Some("timeframe"));
        assert!(output.disabled);
        assert!(output.required);
        assert_eq!(output.data_state, SelectDataState::Open);
        assert_eq!(output.data_state.as_str(), "open");
    }

    #[test]
    fn select_portal_output_preserves_container_contract() {
        let output = primitive_select_portal_output(
            SelectPortalOptions::default().container("select-layer"),
        );

        assert_eq!(output.container.as_deref(), Some("select-layer"));
    }

    #[test]
    fn select_content_output_preserves_position_side_align_and_state() {
        let options = SelectOptions::anchored(
            "select_content_output_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            180.0,
            LayerPlacement::RightStart {
                offset: egui::vec2(8.0, 0.0),
            },
        )
        .max_height(320.0);

        let content = options
            .content_options()
            .open(false)
            .position(SelectPosition::ItemAligned)
            .side_align(SelectSide::Right, SelectAlign::End);
        let output = primitive_select_content_output(content);

        assert_eq!(output.width, 180.0);
        assert_eq!(output.max_height, Some(320.0));
        assert!(!output.open);
        assert!(!output.mounted);
        assert_eq!(output.position, SelectPosition::ItemAligned);
        assert_eq!(output.position.as_str(), "item-aligned");
        assert_eq!(output.side, SelectSide::Right);
        assert_eq!(output.side.as_str(), "right");
        assert_eq!(output.align, SelectAlign::End);
        assert_eq!(output.align.as_str(), "end");
        assert_eq!(output.data_state, SelectDataState::Closed);
        assert_eq!(output.data_state.as_str(), "closed");
    }

    #[test]
    fn select_trigger_options_preserve_part_state() {
        let options = SelectTriggerOptions::default()
            .size(96.0, 28.0)
            .open(true)
            .placeholder(true)
            .disabled(true);

        assert_eq!(options.width, 96.0);
        assert_eq!(options.height, 28.0);
        assert!(options.open);
        assert!(options.placeholder);
        assert!(!options.enabled);
    }

    #[test]
    fn select_item_options_preserve_item_part_state() {
        let options = SelectItemOptions::new(144.0)
            .selected(true)
            .highlighted(true)
            .checked(true)
            .disabled(true);

        assert_eq!(options.width, 144.0);
        assert!(options.selected);
        assert!(options.highlighted);
        assert!(options.checked);
        assert!(options.disabled);
    }

    #[test]
    fn select_item_options_preserve_custom_theme() {
        let theme = PrimitiveTheme {
            menu_row_height: 24.0,
            ..PrimitiveTheme::default()
        };
        let options = SelectItemOptions::new(144.0).theme(theme);

        assert_eq!(options.theme.menu_row_height, 24.0);
    }

    #[test]
    fn select_viewport_options_preserve_width_gap_and_theme() {
        let theme = PrimitiveTheme {
            menu_row_height: 26.0,
            ..PrimitiveTheme::default()
        };
        let options = SelectViewportOptions::new(144.0).item_gap(2.0).theme(theme);

        assert_eq!(options.width, 144.0);
        assert_eq!(options.item_gap, 2.0);
        assert_eq!(options.theme.menu_row_height, 26.0);
    }

    #[test]
    fn select_label_options_preserve_part_width() {
        let options = SelectLabelOptions::new(144.0);

        assert_eq!(options.width, 144.0);
    }

    #[test]
    fn select_separator_options_preserve_part_width() {
        let options = SelectSeparatorOptions::new(144.0);

        assert_eq!(options.width, 144.0);
    }
}
