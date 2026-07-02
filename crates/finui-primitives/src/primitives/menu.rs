use std::hash::Hash;

use eframe::egui::{self, Align2, Color32, FontId, Rect, Response, Sense, Stroke, Vec2};

use super::{
    PrimitiveLayerAnimationOutput, PrimitiveLayerOptions, PrimitiveLayerOutput, PrimitiveTheme,
    RadixIcon, RovingFocusAction, RovingFocusKey, RovingFocusOptions, RovingFocusOrientation,
    RovingFocusOutput, paint_radix_icon, primitive_layer_animation_output,
    primitive_roving_focus_output, radix_colors, show_primitive_layer,
};
use crate::{DismissPolicy, LayerAlign, LayerPlacement, LayerResolvedPlacement, LayerSide};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownMenuDataState {
    Open,
    Closed,
}

impl DropdownMenuDataState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownMenuDirection {
    Ltr,
    Rtl,
}

impl DropdownMenuDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropdownMenuSide {
    Top,
    Right,
    Bottom,
    Left,
}

impl DropdownMenuSide {
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
pub enum DropdownMenuAlign {
    Start,
    Center,
    End,
}

impl DropdownMenuAlign {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropdownMenuRootOptions {
    pub open: bool,
    pub default_open: Option<bool>,
    pub modal: bool,
    pub direction: Option<DropdownMenuDirection>,
}

impl Default for DropdownMenuRootOptions {
    fn default() -> Self {
        Self {
            open: false,
            default_open: None,
            modal: true,
            direction: None,
        }
    }
}

impl DropdownMenuRootOptions {
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

    pub fn direction(mut self, direction: DropdownMenuDirection) -> Self {
        self.direction = Some(direction);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropdownMenuRootOutput {
    pub open: bool,
    pub default_open: Option<bool>,
    pub modal: bool,
    pub direction: Option<DropdownMenuDirection>,
    pub data_state: DropdownMenuDataState,
}

pub fn primitive_dropdown_menu_root_output(
    options: DropdownMenuRootOptions,
) -> DropdownMenuRootOutput {
    DropdownMenuRootOutput {
        open: options.open,
        default_open: options.default_open,
        modal: options.modal,
        direction: options.direction,
        data_state: if options.open {
            DropdownMenuDataState::Open
        } else {
            DropdownMenuDataState::Closed
        },
    }
}

pub fn dropdown_menu_apply_open(
    current: &mut bool,
    next: bool,
    _options: &DropdownMenuRootOptions,
) -> bool {
    if *current == next {
        return false;
    }
    *current = next;
    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropdownMenuPortalOptions {
    pub force_mount: bool,
    pub container: Option<String>,
}

impl Default for DropdownMenuPortalOptions {
    fn default() -> Self {
        Self {
            force_mount: false,
            container: None,
        }
    }
}

impl DropdownMenuPortalOptions {
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
pub struct DropdownMenuPortalOutput {
    pub force_mount: bool,
    pub container: Option<String>,
}

pub fn primitive_dropdown_menu_portal_output(
    options: DropdownMenuPortalOptions,
) -> DropdownMenuPortalOutput {
    DropdownMenuPortalOutput {
        force_mount: options.force_mount,
        container: options.container,
    }
}

pub struct DropdownMenuOptions {
    pub id: egui::Id,
    pub trigger_rect: Rect,
    pub placement: LayerPlacement,
    pub width: f32,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
}

impl DropdownMenuOptions {
    pub fn anchored(
        id: impl Hash,
        trigger_rect: Rect,
        width: f32,
        placement: LayerPlacement,
    ) -> Self {
        Self {
            id: egui::Id::new(id),
            trigger_rect,
            placement,
            width,
            min_height: None,
            max_height: None,
            inner_margin: egui::Margin::same(8),
            theme: PrimitiveTheme::default(),
        }
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

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropdownMenuTriggerOptions {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub theme: PrimitiveTheme,
}

impl Default for DropdownMenuTriggerOptions {
    fn default() -> Self {
        Self {
            width: 112.0,
            height: 32.0,
            open: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl DropdownMenuTriggerOptions {
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
pub struct DropdownMenuContentOptions {
    pub width: f32,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub placement: LayerPlacement,
    pub inner_margin: egui::Margin,
    pub loop_focus: bool,
    pub force_mount: bool,
    pub open: bool,
    pub side: DropdownMenuSide,
    pub align: DropdownMenuAlign,
    pub data_state: DropdownMenuDataState,
    pub theme: PrimitiveTheme,
}

impl DropdownMenuContentOptions {
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
            DropdownMenuDataState::Open
        } else {
            DropdownMenuDataState::Closed
        };
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DropdownMenuContentOutput {
    pub width: f32,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub open: bool,
    pub loop_focus: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub side: DropdownMenuSide,
    pub align: DropdownMenuAlign,
    pub data_state: DropdownMenuDataState,
    pub animation: PrimitiveLayerAnimationOutput,
}

pub struct DropdownMenuOutput<T> {
    pub action: Option<T>,
    pub should_close: bool,
    pub content_rect: Rect,
    pub side: DropdownMenuSide,
    pub align: DropdownMenuAlign,
    pub animation: PrimitiveLayerAnimationOutput,
}

pub fn primitive_dropdown_menu_trigger(
    ui: &mut egui::Ui,
    label: &str,
    options: DropdownMenuTriggerOptions,
) -> Response {
    let root =
        primitive_dropdown_menu_root_output(DropdownMenuRootOptions::default().open(options.open));
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(options.width, options.height), Sense::click());
    let response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    let open = root.data_state == DropdownMenuDataState::Open;
    let fill = if open {
        menu_selected_fill(options.theme)
    } else if response.hovered() {
        menu_hover_fill(options.theme)
    } else {
        options.theme.content_fill
    };
    let stroke_color = if open {
        menu_selected_stroke(options.theme)
    } else if response.hovered() {
        menu_hover_stroke(options.theme)
    } else {
        options.theme.content_stroke.color
    };
    let text_color = if open {
        menu_selected_text(options.theme)
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
    let text_rect = Rect::from_min_max(
        rect.left_top() + Vec2::new(10.0, 0.0),
        rect.right_bottom() - Vec2::new(30.0, 0.0),
    );
    ui.painter().with_clip_rect(text_rect).text(
        text_rect.left_center(),
        Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        text_color,
    );
    let icon = if open {
        RadixIcon::ChevronUp
    } else {
        RadixIcon::ChevronDown
    };
    let icon_rect = Rect::from_center_size(
        rect.right_center() - Vec2::new(14.0, 0.0),
        Vec2::splat(15.0),
    );
    paint_radix_icon(ui, icon, icon_rect, text_color);
    response
}

pub fn dropdown_menu_placement_parts(
    placement: LayerPlacement,
) -> (DropdownMenuSide, DropdownMenuAlign) {
    match placement {
        LayerPlacement::BelowStart { .. } => (DropdownMenuSide::Bottom, DropdownMenuAlign::Start),
        LayerPlacement::BelowEnd { .. } => (DropdownMenuSide::Bottom, DropdownMenuAlign::End),
        LayerPlacement::AboveStart { .. } => (DropdownMenuSide::Top, DropdownMenuAlign::Start),
        LayerPlacement::RightStart { .. } => (DropdownMenuSide::Right, DropdownMenuAlign::Start),
        LayerPlacement::Centered { .. } => (DropdownMenuSide::Bottom, DropdownMenuAlign::Center),
        LayerPlacement::Fixed(_) => (DropdownMenuSide::Bottom, DropdownMenuAlign::Start),
    }
}

pub fn dropdown_menu_side_from_layer_side(side: LayerSide) -> DropdownMenuSide {
    match side {
        LayerSide::Top => DropdownMenuSide::Top,
        LayerSide::Right => DropdownMenuSide::Right,
        LayerSide::Bottom => DropdownMenuSide::Bottom,
        LayerSide::Left => DropdownMenuSide::Left,
    }
}

pub fn dropdown_menu_align_from_layer_align(align: LayerAlign) -> DropdownMenuAlign {
    match align {
        LayerAlign::Start => DropdownMenuAlign::Start,
        LayerAlign::Center => DropdownMenuAlign::Center,
        LayerAlign::End => DropdownMenuAlign::End,
    }
}

pub fn dropdown_menu_layer_side(side: DropdownMenuSide) -> LayerSide {
    match side {
        DropdownMenuSide::Top => LayerSide::Top,
        DropdownMenuSide::Right => LayerSide::Right,
        DropdownMenuSide::Bottom => LayerSide::Bottom,
        DropdownMenuSide::Left => LayerSide::Left,
    }
}

pub fn dropdown_menu_layer_align(align: DropdownMenuAlign) -> LayerAlign {
    match align {
        DropdownMenuAlign::Start => LayerAlign::Start,
        DropdownMenuAlign::Center => LayerAlign::Center,
        DropdownMenuAlign::End => LayerAlign::End,
    }
}

pub fn primitive_dropdown_menu_content_options(
    options: &DropdownMenuOptions,
) -> DropdownMenuContentOptions {
    let (side, align) = dropdown_menu_placement_parts(options.placement);
    DropdownMenuContentOptions {
        width: options.width,
        min_height: options.min_height,
        max_height: options.max_height,
        placement: options.placement,
        inner_margin: options.inner_margin,
        loop_focus: true,
        force_mount: false,
        open: true,
        side,
        align,
        data_state: DropdownMenuDataState::Open,
        theme: options.theme,
    }
}

pub fn primitive_dropdown_menu_content_output(
    options: DropdownMenuContentOptions,
) -> DropdownMenuContentOutput {
    DropdownMenuContentOutput {
        width: options.width,
        min_height: options.min_height,
        max_height: options.max_height,
        open: options.open,
        loop_focus: options.loop_focus,
        force_mount: options.force_mount,
        mounted: options.open || options.force_mount,
        side: options.side,
        align: options.align,
        data_state: options.data_state,
        animation: primitive_layer_animation_output(
            options.data_state == DropdownMenuDataState::Open,
            LayerResolvedPlacement {
                side: dropdown_menu_layer_side(options.side),
                align: dropdown_menu_layer_align(options.align),
                flipped: false,
            },
            1.0,
        ),
    }
}

pub fn primitive_dropdown_menu_layer_options(
    options: &DropdownMenuOptions,
) -> PrimitiveLayerOptions {
    let content = primitive_dropdown_menu_content_options(options);
    let mut layer = PrimitiveLayerOptions::new(options.id, content.width)
        .anchor_rect(options.trigger_rect)
        .placement(content.placement)
        .order(egui::Order::Tooltip)
        .inner_margin(content.inner_margin)
        .dismiss_policy(DismissPolicy::OutsideClickAndEscape);
    layer.theme = content.theme;
    if let Some(min_height) = content.min_height {
        layer = layer.min_height(min_height);
    }
    if let Some(max_height) = content.max_height {
        layer = layer.max_height(max_height);
    }
    layer
}

pub fn show_dropdown_menu<T>(
    ctx: &egui::Context,
    options: DropdownMenuOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> Option<T>,
) -> DropdownMenuOutput<T> {
    let layer = primitive_dropdown_menu_layer_options(&options);
    let output: PrimitiveLayerOutput<T> = show_primitive_layer(ctx, layer, add_contents);
    DropdownMenuOutput {
        action: output.action,
        should_close: output.should_close,
        content_rect: output.content_rect,
        side: dropdown_menu_side_from_layer_side(output.resolved_placement.side),
        align: dropdown_menu_align_from_layer_align(output.resolved_placement.align),
        animation: primitive_layer_animation_output(true, output.resolved_placement, 1.0),
    }
}

pub struct MenuItemOptions {
    pub width: f32,
    pub selected: bool,
    pub highlighted: bool,
    pub checked: bool,
    pub disabled: bool,
    pub trailing: Option<&'static str>,
    pub theme: PrimitiveTheme,
}

impl MenuItemOptions {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            selected: false,
            highlighted: false,
            checked: false,
            disabled: false,
            trailing: None,
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

    pub fn trailing(mut self, trailing: &'static str) -> Self {
        self.trailing = Some(trailing);
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

pub type DropdownMenuItemOptions = MenuItemOptions;
pub type DropdownMenuLabelOptions = f32;
pub type DropdownMenuSeparatorOptions = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenuItem<T> {
    pub value: T,
    pub label: &'static str,
    pub enabled: bool,
}

pub fn menu_roving_focus_output<T>(
    items: &[MenuItem<T>],
    current: Option<usize>,
    key: Option<RovingFocusKey>,
    loop_focus: bool,
) -> RovingFocusOutput {
    let enabled = items.iter().map(|item| item.enabled).collect::<Vec<_>>();
    primitive_roving_focus_output(
        &enabled,
        current,
        key,
        RovingFocusOptions::default()
            .orientation(RovingFocusOrientation::Vertical)
            .loop_focus(loop_focus),
    )
}

pub fn dropdown_menu_roving_focus_output<T>(
    items: &[MenuItem<T>],
    current: Option<usize>,
    key: Option<RovingFocusKey>,
    loop_focus: bool,
) -> RovingFocusOutput {
    menu_roving_focus_output(items, current, key, loop_focus)
}

pub fn primitive_menu_item(ui: &mut egui::Ui, label: &str, options: MenuItemOptions) -> Response {
    let sense = if options.disabled {
        Sense::hover()
    } else {
        Sense::click()
    };
    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(options.width, options.theme.menu_row_height),
        sense,
    );
    let response = if options.disabled {
        response
    } else {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    };
    let row_rect = rect.shrink2(Vec2::new(2.0, 1.0));
    let active = options.highlighted || response.hovered() && !options.disabled;
    let fill = if options.disabled {
        options.theme.content_fill
    } else if options.selected && active {
        menu_selected_active_fill(options.theme)
    } else if options.selected {
        menu_selected_fill(options.theme)
    } else if options.highlighted {
        menu_highlighted_fill(options.theme)
    } else if active {
        menu_hover_fill(options.theme)
    } else {
        options.theme.content_fill
    };
    let stroke_color = if options.disabled {
        None
    } else if options.selected && active {
        Some(menu_selected_stroke(options.theme))
    } else if options.highlighted {
        Some(menu_highlighted_stroke(options.theme))
    } else if response.hovered() && !options.disabled {
        Some(menu_hover_stroke(options.theme))
    } else {
        None
    };
    ui.painter()
        .rect_filled(row_rect, options.theme.row_radius, fill);
    if let Some(stroke_color) = stroke_color {
        ui.painter().rect_stroke(
            row_rect,
            options.theme.row_radius,
            Stroke::new(1.0, stroke_color),
            egui::StrokeKind::Inside,
        );
    }

    let text_color = if options.disabled {
        options.theme.disabled_text
    } else if options.selected || options.checked {
        menu_selected_text(options.theme)
    } else {
        options.theme.text
    };
    if options.checked {
        let icon_rect = Rect::from_center_size(
            row_rect.left_center() + Vec2::new(10.0, 0.0),
            Vec2::splat(15.0),
        );
        paint_radix_icon(ui, RadixIcon::Check, icon_rect, text_color);
    }
    let label_right_inset = if options.trailing.is_some() {
        52.0
    } else {
        8.0
    };
    let label_rect = Rect::from_min_max(
        row_rect.left_top() + Vec2::new(24.0, 0.0),
        row_rect.right_bottom() - Vec2::new(label_right_inset, 0.0),
    );
    ui.painter().with_clip_rect(label_rect).text(
        label_rect.left_center(),
        Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        text_color,
    );
    if let Some(trailing) = options.trailing {
        ui.painter().text(
            row_rect.right_center() - Vec2::new(10.0, 0.0),
            Align2::RIGHT_CENTER,
            trailing,
            crate::scaled_proportional_font(ui, 12.0),
            if options.disabled {
                options.theme.disabled_text
            } else if options.selected {
                menu_selected_text(options.theme)
            } else {
                options.theme.muted_text
            },
        );
    }
    response
}

fn is_dark_primitive_theme(theme: PrimitiveTheme) -> bool {
    let fill = theme.content_fill;
    u16::from(fill.r()) + u16::from(fill.g()) + u16::from(fill.b()) < 160
}

fn menu_selected_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x23, 0x34, 0x70)
    } else {
        radix_colors::INDIGO_3
    }
}

fn menu_selected_active_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x2b, 0x40, 0x86)
    } else {
        radix_colors::INDIGO_4
    }
}

fn menu_highlighted_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x1e, 0x2b, 0x52)
    } else {
        radix_colors::INDIGO_2
    }
}

fn menu_hover_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x1f, 0x28, 0x36)
    } else {
        radix_colors::SLATE_3
    }
}

fn menu_selected_stroke(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x8a, 0xa2, 0xff)
    } else {
        radix_colors::INDIGO_8
    }
}

fn menu_highlighted_stroke(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x6f, 0x87, 0xd6)
    } else {
        radix_colors::INDIGO_7
    }
}

fn menu_hover_stroke(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x4a, 0x55, 0x68)
    } else {
        radix_colors::SLATE_6
    }
}

fn menu_selected_text(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0xee, 0xf3, 0xff)
    } else {
        radix_colors::INDIGO_11
    }
}

pub fn primitive_dropdown_menu_item(
    ui: &mut egui::Ui,
    label: &str,
    options: DropdownMenuItemOptions,
) -> Response {
    primitive_menu_item(ui, label, options)
}

pub fn primitive_dropdown_menu_checkbox_item(
    ui: &mut egui::Ui,
    label: &'static str,
    checked: &mut bool,
    options: DropdownMenuItemOptions,
) -> Response {
    primitive_menu_checkbox_item(ui, label, checked, options)
}

pub fn primitive_dropdown_menu_radio_item(
    ui: &mut egui::Ui,
    label: &'static str,
    value: &'static str,
    current_value: &mut &'static str,
    options: DropdownMenuItemOptions,
) -> Response {
    primitive_menu_radio_item(ui, label, value, current_value, options)
}

pub fn primitive_dropdown_menu_label(
    ui: &mut egui::Ui,
    label: &str,
    width: DropdownMenuLabelOptions,
) {
    primitive_menu_label(ui, label, width);
}

pub fn primitive_dropdown_menu_separator(ui: &mut egui::Ui, width: DropdownMenuSeparatorOptions) {
    primitive_menu_separator(ui, width);
}

pub fn primitive_menu_checkbox_item(
    ui: &mut egui::Ui,
    label: &str,
    checked: &mut bool,
    options: MenuItemOptions,
) -> Response {
    let response = primitive_menu_item(ui, label, options.checked(*checked));
    if response.clicked() {
        *checked = menu_checkbox_next_state(*checked);
    }
    response
}

pub fn primitive_menu_radio_item(
    ui: &mut egui::Ui,
    label: &str,
    value: &'static str,
    current_value: &mut &'static str,
    options: MenuItemOptions,
) -> Response {
    let selected = *current_value == value;
    let response = primitive_menu_item(ui, label, options.selected(selected).checked(selected));
    if response.clicked() {
        *current_value = menu_radio_next_value(value);
    }
    response
}

pub fn menu_checkbox_next_state(current: bool) -> bool {
    !current
}

pub fn menu_radio_next_value(value: &'static str) -> &'static str {
    value
}

pub fn menu_apply_value<T: Copy + PartialEq>(
    current: &mut T,
    next: T,
    items: &[MenuItem<T>],
) -> bool {
    if *current == next {
        return false;
    }
    if !items.iter().any(|item| item.enabled && item.value == next) {
        return false;
    }
    *current = next;
    true
}

pub fn menu_typeahead_match<T: Copy + PartialEq>(
    items: &[MenuItem<T>],
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

pub fn primitive_menu_label(ui: &mut egui::Ui, label: &str, width: f32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 24.0), Sense::hover());
    ui.painter().text(
        rect.left_center() + Vec2::new(10.0, 1.0),
        Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 12.0),
        radix_colors::SLATE_10,
    );
}

pub fn primitive_menu_separator(ui: &mut egui::Ui, width: f32) {
    let theme = PrimitiveTheme::default();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, 9.0), Sense::hover());
    ui.painter().hline(
        rect.left() + 8.0..=rect.right() - 8.0,
        rect.center().y,
        theme.content_stroke,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_item_options_preserve_radix_item_parts() {
        let options = MenuItemOptions::new(160.0)
            .selected(true)
            .highlighted(true)
            .checked(true)
            .disabled(true)
            .trailing("Ctrl+S");

        assert_eq!(options.width, 160.0);
        assert!(options.selected);
        assert!(options.highlighted);
        assert!(options.checked);
        assert!(options.disabled);
        assert_eq!(options.trailing, Some("Ctrl+S"));
    }

    #[test]
    fn menu_checkbox_and_radio_item_state_rules_match_radix_parts() {
        assert!(menu_checkbox_next_state(false));
        assert!(!menu_checkbox_next_state(true));
        assert_eq!(menu_radio_next_value("expanded"), "expanded");
    }

    #[test]
    fn menu_apply_value_respects_enabled_items_and_noop_state() {
        let items = [
            MenuItem {
                value: "view",
                label: "보기",
                enabled: true,
            },
            MenuItem {
                value: "edit",
                label: "편집",
                enabled: true,
            },
            MenuItem {
                value: "save",
                label: "저장",
                enabled: false,
            },
        ];
        let mut value = "view";

        assert!(menu_apply_value(&mut value, "edit", &items));
        assert_eq!(value, "edit");
        assert!(!menu_apply_value(&mut value, "edit", &items));
        assert!(!menu_apply_value(&mut value, "save", &items));
        assert!(!menu_apply_value(&mut value, "missing", &items));
        assert_eq!(value, "edit");
    }

    #[test]
    fn menu_typeahead_match_wraps_from_current_and_skips_disabled_items() {
        let items = [
            MenuItem {
                value: "view",
                label: "보기",
                enabled: true,
            },
            MenuItem {
                value: "edit",
                label: "편집",
                enabled: true,
            },
            MenuItem {
                value: "save",
                label: "저장",
                enabled: false,
            },
        ];

        assert_eq!(
            menu_typeahead_match(&items, Some("view"), "편"),
            Some("edit")
        );
        assert_eq!(
            menu_typeahead_match(&items, Some("edit"), "보"),
            Some("view")
        );
        assert_eq!(menu_typeahead_match(&items, Some("view"), "저"), None);
    }

    #[test]
    fn dropdown_menu_roving_focus_output_skips_disabled_items_and_wraps() {
        let items = [
            MenuItem {
                value: "view",
                label: "보기",
                enabled: true,
            },
            MenuItem {
                value: "edit",
                label: "편집",
                enabled: false,
            },
            MenuItem {
                value: "save",
                label: "저장",
                enabled: true,
            },
        ];

        let output = dropdown_menu_roving_focus_output(
            &items,
            Some(0),
            Some(RovingFocusKey::ArrowDown),
            true,
        );
        let activation =
            dropdown_menu_roving_focus_output(&items, Some(2), Some(RovingFocusKey::Enter), true);
        let close =
            dropdown_menu_roving_focus_output(&items, Some(2), Some(RovingFocusKey::Escape), true);

        assert_eq!(output.active_index, Some(2));
        assert_eq!(output.action, RovingFocusAction::Moved);
        assert_eq!(output.item_tab_indices, vec![-1, -1, 0]);
        assert_eq!(output.item_highlighted, vec![false, false, true]);
        assert_eq!(activation.action, RovingFocusAction::Activate);
        assert_eq!(close.action, RovingFocusAction::Close);
    }

    #[test]
    fn dropdown_menu_roving_focus_output_respects_non_looping_content() {
        let items = [
            MenuItem {
                value: "view",
                label: "보기",
                enabled: true,
            },
            MenuItem {
                value: "save",
                label: "저장",
                enabled: true,
            },
        ];

        let output = dropdown_menu_roving_focus_output(
            &items,
            Some(1),
            Some(RovingFocusKey::ArrowDown),
            false,
        );

        assert_eq!(output.action, RovingFocusAction::None);
        assert_eq!(output.active_index, Some(1));
        assert_eq!(output.item_tab_indices, vec![-1, 0]);
    }

    #[test]
    fn dropdown_menu_apply_open_respects_noop_state() {
        let mut open = false;

        assert!(dropdown_menu_apply_open(
            &mut open,
            true,
            &DropdownMenuRootOptions::default()
        ));
        assert!(open);
        assert!(!dropdown_menu_apply_open(
            &mut open,
            true,
            &DropdownMenuRootOptions::default().open(true)
        ));
        assert!(dropdown_menu_apply_open(
            &mut open,
            false,
            &DropdownMenuRootOptions::default().open(true)
        ));
        assert!(!open);
    }

    #[test]
    fn dropdown_menu_options_preserve_content_contract() {
        let options = DropdownMenuOptions::anchored(
            "dropdown_options_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            168.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        )
        .min_height(120.0)
        .max_height(220.0)
        .inner_margin(egui::Margin::symmetric(4, 5));

        let content = primitive_dropdown_menu_content_options(&options);

        assert_eq!(content.width, 168.0);
        assert_eq!(content.min_height, Some(120.0));
        assert_eq!(content.max_height, Some(220.0));
        assert_eq!(content.inner_margin.left, 4);
        assert_eq!(content.inner_margin.top, 5);
        assert!(content.loop_focus);
        assert!(!content.force_mount);
        assert_eq!(content.side, DropdownMenuSide::Bottom);
        assert_eq!(content.align, DropdownMenuAlign::Start);
        assert_eq!(content.data_state, DropdownMenuDataState::Open);
    }

    #[test]
    fn dropdown_menu_layer_options_use_anchor_and_dismiss_policy() {
        let trigger_rect = Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0));
        let options = DropdownMenuOptions::anchored(
            "dropdown_layer_test",
            trigger_rect,
            168.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        );

        let layer = primitive_dropdown_menu_layer_options(&options);

        assert_eq!(layer.anchor_rect, Some(trigger_rect));
        assert_eq!(layer.order, egui::Order::Tooltip);
        assert_eq!(layer.dismiss_policy, DismissPolicy::OutsideClickAndEscape);
    }

    #[test]
    fn dropdown_menu_trigger_options_preserve_root_state() {
        let options = DropdownMenuTriggerOptions::default()
            .size(96.0, 28.0)
            .open(true);

        assert_eq!(options.width, 96.0);
        assert_eq!(options.height, 28.0);
        assert!(options.open);
    }

    #[test]
    fn dropdown_menu_root_output_preserves_radix_contract() {
        let output = primitive_dropdown_menu_root_output(
            DropdownMenuRootOptions::default()
                .open(true)
                .default_open(false)
                .modal(false)
                .direction(DropdownMenuDirection::Rtl),
        );

        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert!(!output.modal);
        assert_eq!(output.direction, Some(DropdownMenuDirection::Rtl));
        assert_eq!(output.data_state, DropdownMenuDataState::Open);
        assert_eq!(output.data_state.as_str(), "open");
        assert_eq!(DropdownMenuDirection::Rtl.as_str(), "rtl");
    }

    #[test]
    fn dropdown_menu_portal_output_preserves_force_mount_and_container() {
        let output = primitive_dropdown_menu_portal_output(
            DropdownMenuPortalOptions::default()
                .force_mount(true)
                .container("primitive-menu-root"),
        );

        assert!(output.force_mount);
        assert_eq!(output.container.as_deref(), Some("primitive-menu-root"));
    }

    #[test]
    fn dropdown_menu_content_output_preserves_state_side_align_and_mount() {
        let options = DropdownMenuOptions::anchored(
            "dropdown_content_output_test",
            Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(30.0, 40.0)),
            168.0,
            LayerPlacement::RightStart {
                offset: egui::vec2(8.0, 0.0),
            },
        )
        .min_height(100.0)
        .max_height(220.0);

        let content = primitive_dropdown_menu_content_options(&options)
            .open(false)
            .force_mount(true)
            .loop_focus(false);
        let output = primitive_dropdown_menu_content_output(content);

        assert_eq!(output.width, 168.0);
        assert_eq!(output.min_height, Some(100.0));
        assert_eq!(output.max_height, Some(220.0));
        assert!(!output.open);
        assert!(!output.loop_focus);
        assert!(output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.side, DropdownMenuSide::Right);
        assert_eq!(output.align, DropdownMenuAlign::Start);
        assert_eq!(output.data_state, DropdownMenuDataState::Closed);
        assert_eq!(output.data_state.as_str(), "closed");
        assert_eq!(output.side.as_str(), "right");
        assert_eq!(output.align.as_str(), "start");
        assert_eq!(output.animation.open_progress, 0.0);
        assert_eq!(output.animation.data_side, "right");
        assert_eq!(output.animation.data_align, "start");
        assert_eq!(output.animation.transform_origin, egui::pos2(0.0, 0.0));
    }

    #[test]
    fn dropdown_menu_output_reports_collision_resolved_side_align() {
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
        let options = DropdownMenuOptions::anchored(
            "dropdown_collision_output_test",
            Rect::from_min_size(egui::pos2(24.0, 204.0), egui::vec2(42.0, 28.0)),
            120.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        )
        .max_height(120.0);

        let output = show_dropdown_menu(&ctx, options, |ui| {
            ui.label("Collision item");
            None::<()>
        });

        assert_eq!(output.side, DropdownMenuSide::Top);
        assert_eq!(output.align, DropdownMenuAlign::Start);
        assert_eq!(output.animation.data_side, "top");
        assert_eq!(output.animation.data_align, "start");
        assert_eq!(output.animation.transform_origin, egui::pos2(0.0, 1.0));
        assert!(output.animation.collision_flipped);
    }

    #[test]
    fn dropdown_menu_named_item_options_share_menu_item_contract() {
        let options = DropdownMenuItemOptions::new(144.0)
            .selected(true)
            .highlighted(true)
            .checked(true)
            .trailing("Ctrl+S");

        assert_eq!(options.width, 144.0);
        assert!(options.selected);
        assert!(options.highlighted);
        assert!(options.checked);
        assert_eq!(options.trailing, Some("Ctrl+S"));
    }

    #[test]
    fn dropdown_menu_label_and_separator_options_preserve_width() {
        let label_width: DropdownMenuLabelOptions = 144.0;
        let separator_width: DropdownMenuSeparatorOptions = 144.0;

        assert_eq!(label_width, 144.0);
        assert_eq!(separator_width, 144.0);
    }
}
