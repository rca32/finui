use std::hash::Hash;

use eframe::egui::{self, Align2, FontId, Pos2, Rect, Response, Sense, Vec2};

use super::{
    DropdownMenuAlign, DropdownMenuDataState, DropdownMenuDirection, DropdownMenuSide,
    MenuItemOptions, PrimitiveTheme, RovingFocusAction, RovingFocusKey, RovingFocusOptions,
    RovingFocusOrientation, RovingFocusOutput, dropdown_menu_placement_parts,
    primitive_menu_checkbox_item, primitive_menu_item, primitive_menu_label,
    primitive_menu_radio_item, primitive_menu_separator, primitive_roving_focus_output,
};
use crate::LayerPlacement;

pub type MenubarDataState = DropdownMenuDataState;
pub type MenubarDirection = DropdownMenuDirection;
pub type MenubarSide = DropdownMenuSide;
pub type MenubarAlign = DropdownMenuAlign;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenubarRootOptions {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub direction: Option<MenubarDirection>,
    pub loop_focus: bool,
}

impl Default for MenubarRootOptions {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            direction: None,
            loop_focus: false,
        }
    }
}

impl MenubarRootOptions {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, default_value: impl Into<String>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn direction(mut self, direction: MenubarDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenubarRootOutput {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub direction: Option<MenubarDirection>,
    pub loop_focus: bool,
}

pub fn primitive_menubar_root_output(options: MenubarRootOptions) -> MenubarRootOutput {
    MenubarRootOutput {
        value: options.value,
        default_value: options.default_value,
        direction: options.direction,
        loop_focus: options.loop_focus,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenubarMenuOptions {
    pub value: Option<String>,
}

impl Default for MenubarMenuOptions {
    fn default() -> Self {
        Self { value: None }
    }
}

impl MenubarMenuOptions {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenubarMenuOutput {
    pub value: Option<String>,
}

pub fn primitive_menubar_menu_output(options: MenubarMenuOptions) -> MenubarMenuOutput {
    MenubarMenuOutput {
        value: options.value,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenubarPortalOptions {
    pub force_mount: bool,
    pub container: Option<String>,
}

impl Default for MenubarPortalOptions {
    fn default() -> Self {
        Self {
            force_mount: false,
            container: None,
        }
    }
}

impl MenubarPortalOptions {
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
pub struct MenubarPortalOutput {
    pub force_mount: bool,
    pub container: Option<String>,
}

pub fn primitive_menubar_portal_output(options: MenubarPortalOptions) -> MenubarPortalOutput {
    MenubarPortalOutput {
        force_mount: options.force_mount,
        container: options.container,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenubarContentOptions {
    pub width: f32,
    pub placement: LayerPlacement,
    pub open: bool,
    pub loop_focus: bool,
    pub force_mount: bool,
    pub side: MenubarSide,
    pub align: MenubarAlign,
    pub data_state: MenubarDataState,
}

impl MenubarContentOptions {
    pub fn new(width: f32, placement: LayerPlacement) -> Self {
        let (side, align) = dropdown_menu_placement_parts(placement);
        Self {
            width,
            placement,
            open: true,
            loop_focus: false,
            force_mount: false,
            side,
            align,
            data_state: MenubarDataState::Open,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self.data_state = if open {
            MenubarDataState::Open
        } else {
            MenubarDataState::Closed
        };
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn side_align(mut self, side: MenubarSide, align: MenubarAlign) -> Self {
        self.side = side;
        self.align = align;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenubarContentOutput {
    pub width: f32,
    pub open: bool,
    pub loop_focus: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub side: MenubarSide,
    pub align: MenubarAlign,
    pub data_state: MenubarDataState,
}

pub fn primitive_menubar_content_output(options: MenubarContentOptions) -> MenubarContentOutput {
    MenubarContentOutput {
        width: options.width,
        open: options.open,
        loop_focus: options.loop_focus,
        force_mount: options.force_mount,
        mounted: options.open || options.force_mount,
        side: options.side,
        align: options.align,
        data_state: options.data_state,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuNavigationAction {
    None,
    Open(usize),
    Close,
}

pub fn menubar_next_index(len: usize, current: Option<usize>, direction: isize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let len = len as isize;
    let start = current
        .map(|index| index as isize)
        .unwrap_or(if direction >= 0 { -1 } else { len });
    Some((start + direction).rem_euclid(len) as usize)
}

pub fn menubar_next_enabled_index(
    items: &[MenubarItem],
    current: Option<usize>,
    direction: isize,
) -> Option<usize> {
    if items.is_empty() {
        return None;
    }
    let direction = if direction < 0 { -1 } else { 1 };
    let mut index = menubar_next_index(items.len(), current, direction)?;
    for _ in 0..items.len() {
        if items[index].enabled {
            return Some(index);
        }
        index = menubar_next_index(items.len(), Some(index), direction)?;
    }
    None
}

pub fn menubar_typeahead_index(
    items: &[MenubarItem],
    current: Option<usize>,
    query: &str,
) -> Option<usize> {
    let query = query.trim().to_lowercase();
    if items.is_empty() || query.is_empty() {
        return None;
    }
    let mut index = menubar_next_index(items.len(), current, 1)?;
    for _ in 0..items.len() {
        let item = items[index];
        if item.enabled && item.label.to_lowercase().starts_with(&query) {
            return Some(index);
        }
        index = menubar_next_index(items.len(), Some(index), 1)?;
    }
    None
}

pub fn menubar_apply_open(
    current: &mut Option<usize>,
    next: Option<usize>,
    items: &[MenubarItem],
    _options: &MenubarRootOptions,
) -> bool {
    let next = match next {
        Some(index) if items.get(index).is_some_and(|item| item.enabled) => Some(index),
        Some(_) => return false,
        None => None,
    };
    if *current == next {
        return false;
    }
    *current = next;
    true
}

pub fn navigation_menu_panel_rect(trigger: Rect, width: f32, height: f32) -> Rect {
    Rect::from_min_size(trigger.left_bottom(), eframe::egui::vec2(width, height))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenubarItem {
    pub label: &'static str,
    pub enabled: bool,
}

pub fn menubar_roving_focus_output(
    items: &[MenubarItem],
    current: Option<usize>,
    key: Option<RovingFocusKey>,
    options: &MenubarRootOptions,
) -> RovingFocusOutput {
    let enabled = items.iter().map(|item| item.enabled).collect::<Vec<_>>();
    primitive_roving_focus_output(
        &enabled,
        current,
        key,
        RovingFocusOptions::default()
            .orientation(RovingFocusOrientation::Horizontal)
            .loop_focus(options.loop_focus)
            .rtl(options.direction == Some(MenubarDirection::Rtl)),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenubarNavigationLevel {
    TopLevel,
    Content,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenubarStateMachineOutput {
    pub level: MenubarNavigationLevel,
    pub top_level_focus: Option<usize>,
    pub open_menu: Option<usize>,
    pub content_focus: Option<usize>,
    pub top_level_action: RovingFocusAction,
    pub content_action: RovingFocusAction,
    pub content_activated: bool,
}

pub fn menubar_state_machine_output(
    items: &[MenubarItem],
    top_level_focus: Option<usize>,
    open_menu: Option<usize>,
    content_enabled: &[bool],
    content_focus: Option<usize>,
    level: MenubarNavigationLevel,
    key: Option<RovingFocusKey>,
    options: &MenubarRootOptions,
) -> MenubarStateMachineOutput {
    match level {
        MenubarNavigationLevel::TopLevel => {
            let top = menubar_roving_focus_output(items, top_level_focus, key, options);
            let mut next_open_menu = open_menu;
            let mut next_content_focus = content_focus;
            if top.action == RovingFocusAction::Activate {
                next_open_menu = top.active_index;
                next_content_focus = first_enabled_index(content_enabled);
            } else if top.action == RovingFocusAction::Moved {
                if open_menu.is_some() {
                    next_open_menu = top.active_index;
                    next_content_focus = first_enabled_index(content_enabled);
                }
            } else if top.action == RovingFocusAction::Close {
                next_open_menu = None;
                next_content_focus = None;
            }

            MenubarStateMachineOutput {
                level,
                top_level_focus: top.active_index,
                open_menu: next_open_menu,
                content_focus: next_content_focus,
                top_level_action: top.action,
                content_action: RovingFocusAction::None,
                content_activated: false,
            }
        }
        MenubarNavigationLevel::Content => {
            let content = primitive_roving_focus_output(
                content_enabled,
                content_focus,
                key,
                RovingFocusOptions::default()
                    .orientation(RovingFocusOrientation::Vertical)
                    .loop_focus(options.loop_focus),
            );
            let close = content.action == RovingFocusAction::Close;
            MenubarStateMachineOutput {
                level,
                top_level_focus,
                open_menu: if close { None } else { open_menu },
                content_focus: if close { None } else { content.active_index },
                top_level_action: RovingFocusAction::None,
                content_action: content.action,
                content_activated: content.action == RovingFocusAction::Activate,
            }
        }
    }
}

fn first_enabled_index(enabled: &[bool]) -> Option<usize> {
    enabled.iter().position(|enabled| *enabled)
}

pub struct MenubarOutput {
    pub changed: bool,
    pub trigger_rect: Option<Rect>,
    pub responses: Vec<Response>,
}

pub type MenubarItemOptions = MenuItemOptions;
pub type MenubarLabelOptions = f32;
pub type MenubarSeparatorOptions = f32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationMenuItem {
    pub label: &'static str,
    pub enabled: bool,
}

fn navigation_item_as_menubar_item(item: NavigationMenuItem) -> MenubarItem {
    MenubarItem {
        label: item.label,
        enabled: item.enabled,
    }
}

pub fn navigation_menu_next_enabled_index(
    items: &[NavigationMenuItem],
    current: Option<usize>,
    direction: isize,
) -> Option<usize> {
    let menubar_items = items
        .iter()
        .copied()
        .map(navigation_item_as_menubar_item)
        .collect::<Vec<_>>();
    menubar_next_enabled_index(&menubar_items, current, direction)
}

pub fn navigation_menu_typeahead_index(
    items: &[NavigationMenuItem],
    current: Option<usize>,
    query: &str,
) -> Option<usize> {
    let menubar_items = items
        .iter()
        .copied()
        .map(navigation_item_as_menubar_item)
        .collect::<Vec<_>>();
    menubar_typeahead_index(&menubar_items, current, query)
}

pub fn navigation_menu_apply_open(
    current: &mut Option<usize>,
    next: Option<usize>,
    items: &[NavigationMenuItem],
    _options: &NavigationMenuRootOptions,
) -> bool {
    let next = match next {
        Some(index) if items.get(index).is_some_and(|item| item.enabled) => Some(index),
        Some(_) => return false,
        None => None,
    };
    if *current == next {
        return false;
    }
    *current = next;
    true
}

pub struct NavigationMenuRootOutput<T> {
    pub inner: T,
    pub orientation: NavigationMenuOrientation,
    pub active_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMenuOrientation {
    Horizontal,
    Vertical,
}

impl NavigationMenuOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMenuDataState {
    Open,
    Closed,
    Visible,
    Hidden,
}

impl NavigationMenuDataState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
            Self::Visible => "visible",
            Self::Hidden => "hidden",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationMenuMotion {
    ToStart,
    ToEnd,
    FromStart,
    FromEnd,
}

impl NavigationMenuMotion {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ToStart => "to-start",
            Self::ToEnd => "to-end",
            Self::FromStart => "from-start",
            Self::FromEnd => "from-end",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuRootOptions {
    pub orientation: NavigationMenuOrientation,
    pub loop_focus: bool,
    pub value: Option<&'static str>,
    pub default_value: Option<&'static str>,
    pub delay_duration_ms: u64,
    pub skip_delay_duration_ms: u64,
    pub direction: Option<MenubarDirection>,
    pub theme: PrimitiveTheme,
}

impl Default for NavigationMenuRootOptions {
    fn default() -> Self {
        Self {
            orientation: NavigationMenuOrientation::Horizontal,
            loop_focus: true,
            value: None,
            default_value: None,
            delay_duration_ms: 200,
            skip_delay_duration_ms: 300,
            direction: None,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl NavigationMenuRootOptions {
    pub fn orientation(mut self, orientation: NavigationMenuOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn value(mut self, value: &'static str) -> Self {
        self.value = Some(value);
        self
    }

    pub fn default_value(mut self, default_value: &'static str) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn delay_duration_ms(mut self, delay_duration_ms: u64) -> Self {
        self.delay_duration_ms = delay_duration_ms;
        self
    }

    pub fn skip_delay_duration_ms(mut self, skip_delay_duration_ms: u64) -> Self {
        self.skip_delay_duration_ms = skip_delay_duration_ms;
        self
    }

    pub fn direction(mut self, direction: MenubarDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationMenuRootPartOutput {
    pub value: Option<&'static str>,
    pub default_value: Option<&'static str>,
    pub delay_duration_ms: u64,
    pub skip_delay_duration_ms: u64,
    pub direction: Option<MenubarDirection>,
    pub orientation: NavigationMenuOrientation,
}

pub fn primitive_navigation_root_output(
    options: NavigationMenuRootOptions,
) -> NavigationMenuRootPartOutput {
    NavigationMenuRootPartOutput {
        value: options.value,
        default_value: options.default_value,
        delay_duration_ms: options.delay_duration_ms,
        skip_delay_duration_ms: options.skip_delay_duration_ms,
        direction: options.direction,
        orientation: options.orientation,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationMenuListOutput {
    pub orientation: NavigationMenuOrientation,
    pub item_count: usize,
}

pub fn primitive_navigation_list_output(
    orientation: NavigationMenuOrientation,
    item_count: usize,
) -> NavigationMenuListOutput {
    NavigationMenuListOutput {
        orientation,
        item_count,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationMenuItemOutput {
    pub value: Option<&'static str>,
    pub enabled: bool,
}

pub fn primitive_navigation_item_output(
    item: NavigationMenuItem,
    value: Option<&'static str>,
) -> NavigationMenuItemOutput {
    NavigationMenuItemOutput {
        value: value.or(Some(item.label)),
        enabled: item.enabled,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationMenuContentOutput {
    pub open: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub orientation: NavigationMenuOrientation,
    pub data_state: NavigationMenuDataState,
    pub motion: Option<NavigationMenuMotion>,
}

pub fn primitive_navigation_content_output(
    open: bool,
    force_mount: bool,
    orientation: NavigationMenuOrientation,
    motion: Option<NavigationMenuMotion>,
) -> NavigationMenuContentOutput {
    NavigationMenuContentOutput {
        open,
        force_mount,
        mounted: open || force_mount,
        orientation,
        data_state: if open {
            NavigationMenuDataState::Open
        } else {
            NavigationMenuDataState::Closed
        },
        motion,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MenubarTriggerState {
    pub open: bool,
    pub enabled: bool,
    pub hovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenubarTriggerOutput {
    pub data_state: MenubarDataState,
    pub highlighted: bool,
    pub disabled: bool,
}

pub fn primitive_menubar_trigger_output(state: MenubarTriggerState) -> MenubarTriggerOutput {
    MenubarTriggerOutput {
        data_state: if state.open {
            MenubarDataState::Open
        } else {
            MenubarDataState::Closed
        },
        highlighted: state.hovered && state.enabled,
        disabled: !state.enabled,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuLinkState {
    pub active: bool,
    pub enabled: bool,
    pub hovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuIndicatorOptions {
    pub size: f32,
    pub open: bool,
    pub force_mount: bool,
    pub orientation: NavigationMenuOrientation,
    pub theme: PrimitiveTheme,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuTriggerOptions {
    pub width: f32,
    pub height: f32,
    pub state: MenubarTriggerState,
    pub theme: PrimitiveTheme,
}

impl Default for NavigationMenuTriggerOptions {
    fn default() -> Self {
        Self {
            width: 112.0,
            height: 32.0,
            state: MenubarTriggerState {
                open: false,
                enabled: true,
                hovered: false,
            },
            theme: PrimitiveTheme::default(),
        }
    }
}

impl NavigationMenuTriggerOptions {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn state(mut self, state: MenubarTriggerState) -> Self {
        self.state = state;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuViewportOptions {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub force_mount: bool,
    pub theme: PrimitiveTheme,
}

impl Default for NavigationMenuViewportOptions {
    fn default() -> Self {
        Self {
            width: 220.0,
            height: 78.0,
            open: false,
            force_mount: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl NavigationMenuViewportOptions {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

impl Default for NavigationMenuIndicatorOptions {
    fn default() -> Self {
        Self {
            size: 10.0,
            open: false,
            force_mount: false,
            orientation: NavigationMenuOrientation::Horizontal,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl NavigationMenuIndicatorOptions {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn orientation(mut self, orientation: NavigationMenuOrientation) -> Self {
        self.orientation = orientation;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuIndicatorOutput {
    pub open: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub orientation: NavigationMenuOrientation,
    pub data_state: NavigationMenuDataState,
}

pub fn primitive_navigation_indicator_output(
    options: NavigationMenuIndicatorOptions,
) -> NavigationMenuIndicatorOutput {
    NavigationMenuIndicatorOutput {
        open: options.open,
        force_mount: options.force_mount,
        mounted: options.open || options.force_mount,
        orientation: options.orientation,
        data_state: if options.open {
            NavigationMenuDataState::Visible
        } else {
            NavigationMenuDataState::Hidden
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NavigationMenuViewportOutput {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub orientation: NavigationMenuOrientation,
    pub data_state: NavigationMenuDataState,
}

pub fn primitive_navigation_viewport_output(
    options: NavigationMenuViewportOptions,
    orientation: NavigationMenuOrientation,
) -> NavigationMenuViewportOutput {
    NavigationMenuViewportOutput {
        width: options.width,
        height: options.height,
        open: options.open,
        force_mount: options.force_mount,
        mounted: options.open || options.force_mount,
        orientation,
        data_state: if options.open {
            NavigationMenuDataState::Open
        } else {
            NavigationMenuDataState::Closed
        },
    }
}

pub fn primitive_menubar(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    open_index: &mut Option<usize>,
    items: &[MenubarItem],
    item_width: f32,
    height: f32,
    theme: PrimitiveTheme,
) -> MenubarOutput {
    let mut changed = false;
    if ui.input(|input| input.key_pressed(egui::Key::ArrowRight)) {
        *open_index = menubar_next_enabled_index(items, *open_index, 1);
        changed = true;
    }
    if ui.input(|input| input.key_pressed(egui::Key::ArrowLeft)) {
        *open_index = menubar_next_enabled_index(items, *open_index, -1);
        changed = true;
    }

    let root_id = ui.id().with(id_source);
    let mut trigger_rect = None;
    let mut responses = Vec::with_capacity(items.len());
    ui.horizontal(|ui| {
        for (index, item) in items.iter().enumerate() {
            let (rect, _) = ui.allocate_exact_size(Vec2::new(item_width, height), Sense::hover());
            let sense = if item.enabled {
                Sense::click()
            } else {
                Sense::hover()
            };
            let response = ui.interact(rect, root_id.with(index), sense);
            if *open_index == Some(index) {
                trigger_rect = Some(rect);
            }
            if item.enabled && response.clicked() {
                if *open_index == Some(index) {
                    *open_index = None;
                    trigger_rect = None;
                } else {
                    *open_index = Some(index);
                    trigger_rect = Some(rect);
                }
                changed = true;
            }
            primitive_menubar_trigger(
                ui,
                rect,
                item.label,
                MenubarTriggerState {
                    open: *open_index == Some(index),
                    enabled: item.enabled,
                    hovered: response.hovered(),
                },
                theme,
            );
            responses.push(response);
        }
    });
    MenubarOutput {
        changed,
        trigger_rect,
        responses,
    }
}

pub fn primitive_navigation_list(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    open_index: &mut Option<usize>,
    items: &[NavigationMenuItem],
    item_width: f32,
    height: f32,
    theme: PrimitiveTheme,
) -> MenubarOutput {
    let menubar_items: Vec<MenubarItem> = items
        .iter()
        .map(|item| MenubarItem {
            label: item.label,
            enabled: item.enabled,
        })
        .collect();
    primitive_menubar(
        ui,
        id_source,
        open_index,
        &menubar_items,
        item_width,
        height,
        theme,
    )
}

pub fn primitive_navigation_root<T>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    active_index: &mut Option<usize>,
    options: NavigationMenuRootOptions,
    add_parts: impl FnOnce(&mut egui::Ui, &mut Option<usize>, NavigationMenuRootOptions) -> T,
) -> NavigationMenuRootOutput<T> {
    let inner = ui
        .push_id(id_source, |ui| add_parts(ui, active_index, options))
        .inner;
    NavigationMenuRootOutput {
        inner,
        orientation: options.orientation,
        active_index: *active_index,
    }
}

pub fn primitive_navigation_trigger(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    label: &str,
    options: NavigationMenuTriggerOptions,
) -> Response {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(options.width, options.height), Sense::hover());
    let sense = if options.state.enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let response = ui.interact(rect, ui.id().with(id_source), sense);
    let state = MenubarTriggerState {
        hovered: options.state.hovered || response.hovered(),
        ..options.state
    };
    primitive_menubar_trigger(ui, rect, label, state, options.theme);
    response
}

pub fn primitive_menubar_trigger(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    state: MenubarTriggerState,
    theme: PrimitiveTheme,
) {
    let fill = if state.open {
        theme.item_selected_fill
    } else if state.hovered && state.enabled {
        theme.item_hover_fill
    } else {
        theme.content_fill
    };
    ui.painter()
        .rect_filled(rect.shrink(1.0), theme.row_radius, fill);
    ui.painter().rect_stroke(
        rect.shrink(1.0),
        theme.row_radius,
        theme.content_stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        if state.enabled {
            theme.text
        } else {
            theme.disabled_text
        },
    );
}

pub fn primitive_menubar_item(
    ui: &mut egui::Ui,
    label: &'static str,
    options: MenubarItemOptions,
) -> Response {
    primitive_menu_item(ui, label, options)
}

pub fn primitive_menubar_checkbox_item(
    ui: &mut egui::Ui,
    label: &'static str,
    checked: &mut bool,
    options: MenubarItemOptions,
) -> Response {
    primitive_menu_checkbox_item(ui, label, checked, options)
}

pub fn primitive_menubar_radio_item(
    ui: &mut egui::Ui,
    label: &'static str,
    value: &'static str,
    current_value: &mut &'static str,
    options: MenubarItemOptions,
) -> Response {
    primitive_menu_radio_item(ui, label, value, current_value, options)
}

pub fn primitive_menubar_label(ui: &mut egui::Ui, label: &str, width: MenubarLabelOptions) {
    primitive_menu_label(ui, label, width);
}

pub fn primitive_menubar_separator(ui: &mut egui::Ui, width: MenubarSeparatorOptions) {
    primitive_menu_separator(ui, width);
}

pub fn primitive_navigation_link(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    rect: Rect,
    label: &str,
    state: NavigationMenuLinkState,
    theme: PrimitiveTheme,
) -> Response {
    let sense = if state.enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let response = ui.interact(rect, ui.id().with(id_source), sense);
    let paint_state = NavigationMenuLinkState {
        hovered: state.hovered || response.hovered(),
        ..state
    };
    paint_navigation_link(ui, rect, label, paint_state, theme);
    response
}

fn paint_navigation_link(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    state: NavigationMenuLinkState,
    theme: PrimitiveTheme,
) {
    let fill = if state.active {
        theme.item_selected_fill
    } else if state.hovered && state.enabled {
        theme.item_hover_fill
    } else {
        theme.content_fill
    };
    ui.painter()
        .rect_filled(rect.shrink(1.0), theme.row_radius, fill);
    ui.painter().text(
        rect.left_center() + Vec2::new(10.0, 0.0),
        Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        if state.enabled {
            theme.text
        } else {
            theme.disabled_text
        },
    );
}

pub fn primitive_navigation_panel(
    ui: &mut egui::Ui,
    trigger_rect: Rect,
    title: &str,
    body: &str,
    width: f32,
    height: f32,
    theme: PrimitiveTheme,
) -> Rect {
    let panel = navigation_menu_panel_rect(trigger_rect, width, height);
    primitive_navigation_content(ui, panel, title, body, theme);
    panel
}

pub fn primitive_navigation_viewport(
    ui: &egui::Ui,
    anchor: Pos2,
    title: &str,
    body: &str,
    options: NavigationMenuViewportOptions,
) -> Option<Rect> {
    if !options.open && !options.force_mount {
        return None;
    }
    let rect = Rect::from_min_size(anchor, Vec2::new(options.width, options.height));
    primitive_navigation_content(ui, rect, title, body, options.theme);
    Some(rect)
}

pub fn primitive_navigation_content(
    ui: &egui::Ui,
    panel: Rect,
    title: &str,
    body: &str,
    theme: PrimitiveTheme,
) {
    ui.painter().rect(
        panel,
        theme.radius,
        theme.content_fill,
        theme.content_stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        panel.left_top() + Vec2::new(12.0, 14.0),
        Align2::LEFT_TOP,
        title,
        crate::scaled_proportional_font(ui, 13.0),
        theme.text,
    );
    ui.painter().text(
        panel.left_top() + Vec2::new(12.0, 40.0),
        Align2::LEFT_TOP,
        body,
        crate::scaled_proportional_font(ui, 12.0),
        theme.muted_text,
    );
}

pub fn navigation_menu_indicator_points(
    trigger_rect: Rect,
    content_rect: Rect,
    size: f32,
) -> [Pos2; 3] {
    let half = size * 0.5;
    let center_x = trigger_rect.center().x.clamp(
        content_rect.left() + half,
        (content_rect.right() - half).max(content_rect.left() + half),
    );
    [
        Pos2::new(center_x, content_rect.top()),
        Pos2::new(center_x - half, content_rect.top() + size),
        Pos2::new(center_x + half, content_rect.top() + size),
    ]
}

pub fn primitive_navigation_indicator(
    ui: &egui::Ui,
    trigger_rect: Rect,
    content_rect: Rect,
    options: NavigationMenuIndicatorOptions,
) -> [Pos2; 3] {
    let points = navigation_menu_indicator_points(trigger_rect, content_rect, options.size);
    ui.painter().add(egui::Shape::convex_polygon(
        points.to_vec(),
        options.theme.content_fill,
        options.theme.content_stroke,
    ));
    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menubar_next_index_wraps() {
        assert_eq!(menubar_next_index(3, None, 1), Some(0));
        assert_eq!(menubar_next_index(3, Some(2), 1), Some(0));
        assert_eq!(menubar_next_index(3, Some(0), -1), Some(2));
    }

    #[test]
    fn menubar_next_enabled_index_wraps_and_skips_disabled_items() {
        let items = [
            MenubarItem {
                label: "File",
                enabled: false,
            },
            MenubarItem {
                label: "View",
                enabled: true,
            },
            MenubarItem {
                label: "Help",
                enabled: true,
            },
        ];

        assert_eq!(menubar_next_enabled_index(&items, None, 1), Some(1));
        assert_eq!(menubar_next_enabled_index(&items, Some(1), 1), Some(2));
        assert_eq!(menubar_next_enabled_index(&items, Some(1), -1), Some(2));
        assert_eq!(menubar_next_enabled_index(&items[..1], None, 1), None);
    }

    #[test]
    fn menubar_roving_focus_output_uses_horizontal_menu_contract() {
        let items = [
            MenubarItem {
                label: "File",
                enabled: true,
            },
            MenubarItem {
                label: "Edit",
                enabled: false,
            },
            MenubarItem {
                label: "Help",
                enabled: true,
            },
        ];
        let options = MenubarRootOptions::default().loop_focus(true);

        let output = menubar_roving_focus_output(
            &items,
            Some(0),
            Some(RovingFocusKey::ArrowRight),
            &options,
        );
        let activation =
            menubar_roving_focus_output(&items, Some(2), Some(RovingFocusKey::Enter), &options);
        let close =
            menubar_roving_focus_output(&items, Some(2), Some(RovingFocusKey::Escape), &options);

        assert_eq!(output.active_index, Some(2));
        assert_eq!(output.action, RovingFocusAction::Moved);
        assert_eq!(output.item_tab_indices, vec![-1, -1, 0]);
        assert_eq!(activation.action, RovingFocusAction::Activate);
        assert_eq!(close.action, RovingFocusAction::Close);
    }

    #[test]
    fn menubar_roving_focus_output_respects_rtl_arrow_direction() {
        let items = [
            MenubarItem {
                label: "File",
                enabled: true,
            },
            MenubarItem {
                label: "Edit",
                enabled: true,
            },
            MenubarItem {
                label: "Help",
                enabled: true,
            },
        ];
        let options = MenubarRootOptions::default()
            .direction(MenubarDirection::Rtl)
            .loop_focus(true);

        let output = menubar_roving_focus_output(
            &items,
            Some(1),
            Some(RovingFocusKey::ArrowRight),
            &options,
        );

        assert_eq!(output.active_index, Some(0));
    }

    #[test]
    fn menubar_state_machine_integrates_top_level_and_open_content_navigation() {
        let items = [
            MenubarItem {
                label: "File",
                enabled: true,
            },
            MenubarItem {
                label: "Edit",
                enabled: true,
            },
            MenubarItem {
                label: "Help",
                enabled: true,
            },
        ];
        let content_enabled = [true, false, true];
        let options = MenubarRootOptions::default().loop_focus(true);

        let open = menubar_state_machine_output(
            &items,
            Some(0),
            None,
            &content_enabled,
            None,
            MenubarNavigationLevel::TopLevel,
            Some(RovingFocusKey::Enter),
            &options,
        );
        let switch_open_menu = menubar_state_machine_output(
            &items,
            open.top_level_focus,
            open.open_menu,
            &content_enabled,
            open.content_focus,
            MenubarNavigationLevel::TopLevel,
            Some(RovingFocusKey::ArrowRight),
            &options,
        );
        let move_content = menubar_state_machine_output(
            &items,
            switch_open_menu.top_level_focus,
            switch_open_menu.open_menu,
            &content_enabled,
            switch_open_menu.content_focus,
            MenubarNavigationLevel::Content,
            Some(RovingFocusKey::ArrowDown),
            &options,
        );
        let activate_content = menubar_state_machine_output(
            &items,
            move_content.top_level_focus,
            move_content.open_menu,
            &content_enabled,
            move_content.content_focus,
            MenubarNavigationLevel::Content,
            Some(RovingFocusKey::Enter),
            &options,
        );
        let close = menubar_state_machine_output(
            &items,
            activate_content.top_level_focus,
            activate_content.open_menu,
            &content_enabled,
            activate_content.content_focus,
            MenubarNavigationLevel::Content,
            Some(RovingFocusKey::Escape),
            &options,
        );

        assert_eq!(open.level, MenubarNavigationLevel::TopLevel);
        assert_eq!(open.open_menu, Some(0));
        assert_eq!(open.content_focus, Some(0));
        assert_eq!(open.top_level_action, RovingFocusAction::Activate);
        assert_eq!(switch_open_menu.open_menu, Some(1));
        assert_eq!(switch_open_menu.content_focus, Some(0));
        assert_eq!(move_content.level, MenubarNavigationLevel::Content);
        assert_eq!(move_content.content_focus, Some(2));
        assert_eq!(move_content.content_action, RovingFocusAction::Moved);
        assert!(activate_content.content_activated);
        assert_eq!(activate_content.open_menu, Some(1));
        assert_eq!(close.open_menu, None);
        assert_eq!(close.content_focus, None);
        assert_eq!(close.content_action, RovingFocusAction::Close);
    }

    #[test]
    fn menubar_typeahead_index_wraps_from_current_and_skips_disabled_items() {
        let items = [
            MenubarItem {
                label: "File",
                enabled: true,
            },
            MenubarItem {
                label: "View",
                enabled: true,
            },
            MenubarItem {
                label: "Help",
                enabled: false,
            },
        ];

        assert_eq!(menubar_typeahead_index(&items, Some(0), "v"), Some(1));
        assert_eq!(menubar_typeahead_index(&items, Some(1), "f"), Some(0));
        assert_eq!(menubar_typeahead_index(&items, Some(0), "h"), None);
        assert_eq!(menubar_typeahead_index(&items, Some(0), ""), None);
    }

    #[test]
    fn menubar_apply_open_skips_disabled_invalid_and_noop_state() {
        let items = [
            MenubarItem {
                label: "File",
                enabled: true,
            },
            MenubarItem {
                label: "View",
                enabled: false,
            },
            MenubarItem {
                label: "Help",
                enabled: true,
            },
        ];
        let options = MenubarRootOptions::default()
            .value("File")
            .default_value("File")
            .loop_focus(true);
        let mut open = None;

        assert!(!menubar_apply_open(&mut open, Some(1), &items, &options));
        assert_eq!(open, None);
        assert!(!menubar_apply_open(&mut open, Some(9), &items, &options));
        assert_eq!(open, None);
        assert!(menubar_apply_open(&mut open, Some(2), &items, &options));
        assert_eq!(open, Some(2));
        assert!(!menubar_apply_open(&mut open, Some(2), &items, &options));
        assert!(menubar_apply_open(&mut open, None, &items, &options));
        assert_eq!(open, None);
    }

    #[test]
    fn menubar_items_preserve_enabled_state() {
        let item = MenubarItem {
            label: "File",
            enabled: false,
        };

        assert_eq!(item.label, "File");
        assert!(!item.enabled);
    }

    #[test]
    fn menubar_root_output_preserves_radix_contract() {
        let output = primitive_menubar_root_output(
            MenubarRootOptions::default()
                .value("file")
                .default_value("edit")
                .direction(MenubarDirection::Rtl)
                .loop_focus(true),
        );

        assert_eq!(output.value.as_deref(), Some("file"));
        assert_eq!(output.default_value.as_deref(), Some("edit"));
        assert_eq!(output.direction, Some(MenubarDirection::Rtl));
        assert!(output.loop_focus);
        assert_eq!(MenubarDirection::Rtl.as_str(), "rtl");
    }

    #[test]
    fn menubar_menu_output_preserves_value_contract() {
        let output = primitive_menubar_menu_output(MenubarMenuOptions::default().value("view"));

        assert_eq!(output.value.as_deref(), Some("view"));
    }

    #[test]
    fn menubar_portal_output_preserves_force_mount_and_container() {
        let output = primitive_menubar_portal_output(
            MenubarPortalOptions::default()
                .force_mount(true)
                .container("menubar-layer"),
        );

        assert!(output.force_mount);
        assert_eq!(output.container.as_deref(), Some("menubar-layer"));
    }

    #[test]
    fn menubar_content_output_preserves_state_side_align_and_mount() {
        let content = MenubarContentOptions::new(
            180.0,
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            },
        )
        .open(false)
        .loop_focus(true)
        .force_mount(true)
        .side_align(MenubarSide::Bottom, MenubarAlign::Center);
        let output = primitive_menubar_content_output(content);

        assert_eq!(output.width, 180.0);
        assert!(!output.open);
        assert!(output.loop_focus);
        assert!(output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.side, MenubarSide::Bottom);
        assert_eq!(output.side.as_str(), "bottom");
        assert_eq!(output.align, MenubarAlign::Center);
        assert_eq!(output.align.as_str(), "center");
        assert_eq!(output.data_state, MenubarDataState::Closed);
        assert_eq!(output.data_state.as_str(), "closed");
    }

    #[test]
    fn menubar_named_item_options_share_menu_item_contract() {
        let options = MenubarItemOptions::new(148.0)
            .selected(true)
            .checked(true)
            .disabled(true)
            .trailing("Ctrl+N");

        assert_eq!(options.width, 148.0);
        assert!(options.selected);
        assert!(options.checked);
        assert!(options.disabled);
        assert_eq!(options.trailing, Some("Ctrl+N"));
    }

    #[test]
    fn menubar_label_and_separator_options_preserve_width() {
        let label_width: MenubarLabelOptions = 148.0;
        let separator_width: MenubarSeparatorOptions = 148.0;

        assert_eq!(label_width, 148.0);
        assert_eq!(separator_width, 148.0);
    }

    #[test]
    fn navigation_menu_items_preserve_enabled_state() {
        let item = NavigationMenuItem {
            label: "Products",
            enabled: false,
        };

        assert_eq!(item.label, "Products");
        assert!(!item.enabled);
    }

    #[test]
    fn navigation_menu_next_enabled_index_uses_navigation_items() {
        let items = [
            NavigationMenuItem {
                label: "File",
                enabled: false,
            },
            NavigationMenuItem {
                label: "View",
                enabled: true,
            },
            NavigationMenuItem {
                label: "Help",
                enabled: true,
            },
        ];

        assert_eq!(navigation_menu_next_enabled_index(&items, None, 1), Some(1));
        assert_eq!(
            navigation_menu_next_enabled_index(&items, Some(1), -1),
            Some(2)
        );
    }

    #[test]
    fn navigation_menu_typeahead_index_wraps_and_skips_disabled_items() {
        let items = [
            NavigationMenuItem {
                label: "File",
                enabled: true,
            },
            NavigationMenuItem {
                label: "View",
                enabled: false,
            },
            NavigationMenuItem {
                label: "Help",
                enabled: true,
            },
        ];

        assert_eq!(navigation_menu_typeahead_index(&items, None, "h"), Some(2));
        assert_eq!(
            navigation_menu_typeahead_index(&items, Some(2), "f"),
            Some(0)
        );
        assert_eq!(navigation_menu_typeahead_index(&items, Some(0), "v"), None);
    }

    #[test]
    fn navigation_menu_apply_open_skips_disabled_invalid_and_noop_state() {
        let items = [
            NavigationMenuItem {
                label: "File",
                enabled: true,
            },
            NavigationMenuItem {
                label: "View",
                enabled: false,
            },
            NavigationMenuItem {
                label: "Help",
                enabled: true,
            },
        ];
        let options = NavigationMenuRootOptions::default().value("File");
        let mut open = None;

        assert!(!navigation_menu_apply_open(
            &mut open,
            Some(1),
            &items,
            &options
        ));
        assert_eq!(open, None);
        assert!(!navigation_menu_apply_open(
            &mut open,
            Some(9),
            &items,
            &options
        ));
        assert_eq!(open, None);
        assert!(navigation_menu_apply_open(
            &mut open,
            Some(2),
            &items,
            &options
        ));
        assert_eq!(open, Some(2));
        assert!(!navigation_menu_apply_open(
            &mut open,
            Some(2),
            &items,
            &options
        ));
        assert!(navigation_menu_apply_open(
            &mut open, None, &items, &options
        ));
        assert_eq!(open, None);
    }

    #[test]
    fn navigation_menu_root_options_preserve_orientation_loop_and_theme() {
        let theme = PrimitiveTheme {
            row_radius: 7.0,
            ..PrimitiveTheme::default()
        };
        let options = NavigationMenuRootOptions::default()
            .orientation(NavigationMenuOrientation::Vertical)
            .loop_focus(false)
            .value("learn")
            .default_value("home")
            .delay_duration_ms(120)
            .skip_delay_duration_ms(80)
            .direction(MenubarDirection::Rtl)
            .theme(theme);

        assert_eq!(options.orientation, NavigationMenuOrientation::Vertical);
        assert!(!options.loop_focus);
        assert_eq!(options.value, Some("learn"));
        assert_eq!(options.default_value, Some("home"));
        assert_eq!(options.delay_duration_ms, 120);
        assert_eq!(options.skip_delay_duration_ms, 80);
        assert_eq!(options.direction, Some(MenubarDirection::Rtl));
        assert_eq!(options.theme.row_radius, 7.0);
    }

    #[test]
    fn navigation_menu_root_part_output_preserves_radix_contract() {
        let output = primitive_navigation_root_output(
            NavigationMenuRootOptions::default()
                .value("learn")
                .default_value("home")
                .delay_duration_ms(120)
                .skip_delay_duration_ms(80)
                .direction(MenubarDirection::Rtl)
                .orientation(NavigationMenuOrientation::Vertical),
        );

        assert_eq!(output.value, Some("learn"));
        assert_eq!(output.default_value, Some("home"));
        assert_eq!(output.delay_duration_ms, 120);
        assert_eq!(output.skip_delay_duration_ms, 80);
        assert_eq!(output.direction, Some(MenubarDirection::Rtl));
        assert_eq!(output.orientation, NavigationMenuOrientation::Vertical);
        assert_eq!(output.orientation.as_str(), "vertical");
    }

    #[test]
    fn navigation_menu_list_and_item_outputs_preserve_orientation_and_value() {
        let list = primitive_navigation_list_output(NavigationMenuOrientation::Horizontal, 3);
        let item = primitive_navigation_item_output(
            NavigationMenuItem {
                label: "Learn",
                enabled: true,
            },
            Some("learn"),
        );

        assert_eq!(list.orientation, NavigationMenuOrientation::Horizontal);
        assert_eq!(list.orientation.as_str(), "horizontal");
        assert_eq!(list.item_count, 3);
        assert_eq!(item.value, Some("learn"));
        assert!(item.enabled);
    }

    #[test]
    fn navigation_menu_content_output_preserves_state_motion_and_orientation() {
        let output = primitive_navigation_content_output(
            true,
            false,
            NavigationMenuOrientation::Horizontal,
            Some(NavigationMenuMotion::FromEnd),
        );

        assert!(output.open);
        assert!(!output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.orientation, NavigationMenuOrientation::Horizontal);
        assert_eq!(output.data_state, NavigationMenuDataState::Open);
        assert_eq!(output.data_state.as_str(), "open");
        assert_eq!(output.motion, Some(NavigationMenuMotion::FromEnd));
        assert_eq!(NavigationMenuMotion::FromEnd.as_str(), "from-end");
    }

    #[test]
    fn navigation_menu_root_output_preserves_active_index() {
        let output = NavigationMenuRootOutput {
            inner: "inner",
            orientation: NavigationMenuOrientation::Horizontal,
            active_index: Some(2),
        };

        assert_eq!(output.inner, "inner");
        assert_eq!(output.orientation, NavigationMenuOrientation::Horizontal);
        assert_eq!(output.active_index, Some(2));
    }

    #[test]
    fn navigation_menu_trigger_options_preserve_state_size_and_theme() {
        let state = MenubarTriggerState {
            open: true,
            enabled: false,
            hovered: true,
        };
        let options = NavigationMenuTriggerOptions::default()
            .size(140.0, 34.0)
            .state(state)
            .theme(PrimitiveTheme::default());

        assert_eq!(options.width, 140.0);
        assert_eq!(options.height, 34.0);
        assert_eq!(options.state, state);
    }

    #[test]
    fn navigation_menu_viewport_options_preserve_open_force_mount_and_size() {
        let options = NavigationMenuViewportOptions::default()
            .size(240.0, 96.0)
            .open(false)
            .force_mount(true);

        assert_eq!(options.width, 240.0);
        assert_eq!(options.height, 96.0);
        assert!(!options.open);
        assert!(options.force_mount);
    }

    #[test]
    fn navigation_menu_viewport_output_preserves_state_orientation_and_size() {
        let output = primitive_navigation_viewport_output(
            NavigationMenuViewportOptions::default()
                .size(240.0, 96.0)
                .open(false)
                .force_mount(true),
            NavigationMenuOrientation::Vertical,
        );

        assert_eq!(output.width, 240.0);
        assert_eq!(output.height, 96.0);
        assert!(!output.open);
        assert!(output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.orientation, NavigationMenuOrientation::Vertical);
        assert_eq!(output.data_state, NavigationMenuDataState::Closed);
        assert_eq!(output.data_state.as_str(), "closed");
    }

    #[test]
    fn navigation_menu_panel_rect_anchors_below_trigger() {
        let trigger = Rect::from_min_size(egui::pos2(20.0, 12.0), Vec2::new(80.0, 30.0));
        let panel = navigation_menu_panel_rect(trigger, 220.0, 78.0);

        assert_eq!(panel.left_top(), trigger.left_bottom());
        assert_eq!(panel.width(), 220.0);
        assert_eq!(panel.height(), 78.0);
    }

    #[test]
    fn menubar_trigger_state_preserves_open_enabled_hovered_parts() {
        let state = MenubarTriggerState {
            open: true,
            enabled: false,
            hovered: true,
        };

        assert!(state.open);
        assert!(!state.enabled);
        assert!(state.hovered);
    }

    #[test]
    fn menubar_trigger_output_maps_radix_data_attributes() {
        let output = primitive_menubar_trigger_output(MenubarTriggerState {
            open: true,
            enabled: true,
            hovered: true,
        });

        assert_eq!(output.data_state, MenubarDataState::Open);
        assert_eq!(output.data_state.as_str(), "open");
        assert!(output.highlighted);
        assert!(!output.disabled);
    }

    #[test]
    fn navigation_menu_link_state_preserves_active_enabled_hovered_parts() {
        let state = NavigationMenuLinkState {
            active: true,
            enabled: false,
            hovered: true,
        };

        assert!(state.active);
        assert!(!state.enabled);
        assert!(state.hovered);
    }

    #[test]
    fn navigation_menu_indicator_clamps_to_content_rect() {
        let trigger = Rect::from_min_size(egui::pos2(0.0, 0.0), Vec2::new(20.0, 28.0));
        let content = Rect::from_min_size(egui::pos2(80.0, 40.0), Vec2::new(100.0, 80.0));
        let points = navigation_menu_indicator_points(trigger, content, 12.0);

        assert_eq!(points[0].x, 86.0);
        assert_eq!(points[0].y, 40.0);
        assert_eq!(points[1].y, 52.0);
        assert_eq!(points[2].y, 52.0);
    }

    #[test]
    fn navigation_menu_indicator_output_preserves_visible_state() {
        let output = primitive_navigation_indicator_output(
            NavigationMenuIndicatorOptions::default()
                .open(true)
                .force_mount(false)
                .orientation(NavigationMenuOrientation::Vertical),
        );

        assert!(output.open);
        assert!(!output.force_mount);
        assert!(output.mounted);
        assert_eq!(output.orientation, NavigationMenuOrientation::Vertical);
        assert_eq!(output.data_state, NavigationMenuDataState::Visible);
        assert_eq!(output.data_state.as_str(), "visible");
    }
}
