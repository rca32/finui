use std::hash::Hash;

use eframe::egui::{self, Color32, FontId, Rect, Response, Vec2};

use super::{
    PrimitiveDirection, PrimitiveTheme, RovingFocusState, primitive_horizontal_arrow_step,
    radix_colors,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabItem {
    pub label: &'static str,
    pub enabled: bool,
}

pub struct TabsHeaderOutput {
    pub response: Option<Response>,
    pub changed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsActivationMode {
    Automatic,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsKeyboardAction {
    None,
    FocusFirst,
    FocusLast,
    FocusNext,
    FocusPrevious,
    Activate,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabsHeaderOptions {
    pub theme: PrimitiveTheme,
    pub orientation: TabsOrientation,
    pub direction: Option<PrimitiveDirection>,
    pub activation_mode: TabsActivationMode,
    pub loop_focus: bool,
}

impl Default for TabsHeaderOptions {
    fn default() -> Self {
        Self {
            theme: PrimitiveTheme::default(),
            orientation: TabsOrientation::Horizontal,
            direction: None,
            activation_mode: TabsActivationMode::Automatic,
            loop_focus: true,
        }
    }
}

impl TabsHeaderOptions {
    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn orientation(mut self, orientation: TabsOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn direction(mut self, direction: PrimitiveDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn activation_mode(mut self, activation_mode: TabsActivationMode) -> Self {
        self.activation_mode = activation_mode;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }
}

pub struct TabsContentOutput<T> {
    pub inner: T,
    pub rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabsTriggerState {
    pub active: bool,
    pub enabled: bool,
    pub hovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabsContentOptions {
    pub min_height: f32,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
}

impl Default for TabsContentOptions {
    fn default() -> Self {
        Self {
            min_height: 0.0,
            inner_margin: egui::Margin::symmetric(10, 8),
            theme: PrimitiveTheme::default(),
        }
    }
}

impl TabsContentOptions {
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
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

pub fn primitive_tabs_header(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    selected: &mut usize,
    focus: &mut RovingFocusState,
    items: &[TabItem],
    theme: PrimitiveTheme,
) -> TabsHeaderOutput {
    primitive_tabs_header_with_options(
        ui,
        id_source,
        selected,
        focus,
        items,
        TabsHeaderOptions::default().theme(theme),
    )
}

pub fn primitive_tabs_header_with_options(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    selected: &mut usize,
    focus: &mut RovingFocusState,
    items: &[TabItem],
    options: TabsHeaderOptions,
) -> TabsHeaderOutput {
    let enabled = items.iter().map(|item| item.enabled).collect::<Vec<_>>();
    let keyboard_action = ui.input(|input| {
        tabs_keyboard_action(
            options.orientation,
            options.direction,
            input.key_pressed(egui::Key::Enter),
            input.key_pressed(egui::Key::Space),
            input.key_pressed(egui::Key::ArrowUp),
            input.key_pressed(egui::Key::ArrowDown),
            input.key_pressed(egui::Key::ArrowLeft),
            input.key_pressed(egui::Key::ArrowRight),
            input.key_pressed(egui::Key::Home),
            input.key_pressed(egui::Key::End),
        )
    });
    let mut changed = tabs_apply_keyboard_action(
        selected,
        focus,
        &enabled,
        keyboard_action,
        options.activation_mode,
        options.loop_focus,
    );

    let bounds = ui.available_rect_before_wrap();
    let rects = tab_rects_with_orientation(bounds, items, options.orientation);
    let mut last_response = None;
    for (index, (item, rect)) in items.iter().zip(rects.into_iter()).enumerate() {
        let mut response = ui.interact(
            rect,
            ui.id().with((&id_source, index)),
            if item.enabled {
                egui::Sense::click()
            } else {
                egui::Sense::hover()
            },
        );
        if item.enabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        if response.clicked() && item.enabled && *selected != index {
            *selected = index;
            focus.active_index = Some(index);
            changed = true;
        }
        primitive_tabs_trigger(
            ui,
            rect,
            item.label,
            TabsTriggerState {
                active: *selected == index,
                enabled: item.enabled,
                hovered: response.hovered(),
            },
            options.theme,
        );
        last_response = Some(response);
    }
    if let Some(last) =
        primitive_tabs_list_rect_with_orientation(bounds, items, options.orientation)
    {
        ui.allocate_rect(last, egui::Sense::hover());
    }
    TabsHeaderOutput {
        response: last_response,
        changed,
    }
}

pub fn primitive_tabs_content<T>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    options: TabsContentOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> T,
) -> TabsContentOutput<T> {
    let frame = egui::Frame::new()
        .fill(options.theme.content_fill)
        .stroke(options.theme.content_stroke)
        .corner_radius(options.theme.row_radius)
        .inner_margin(options.inner_margin);
    let output = frame.show(ui, |ui| {
        ui.set_min_height(options.min_height);
        add_contents(ui)
    });
    let response = ui.interact(
        output.response.rect,
        ui.id().with(id_source),
        egui::Sense::hover(),
    );
    TabsContentOutput {
        inner: output.inner,
        rect: response.rect,
    }
}

pub fn primitive_tabs_list_rect(bounds: Rect, items: &[TabItem]) -> Option<Rect> {
    primitive_tabs_list_rect_with_orientation(bounds, items, TabsOrientation::Horizontal)
}

pub fn primitive_tabs_list_rect_with_orientation(
    bounds: Rect,
    items: &[TabItem],
    orientation: TabsOrientation,
) -> Option<Rect> {
    let rects = tab_rects_with_orientation(bounds, items, orientation);
    let first = rects.first()?;
    let mut list = *first;
    for rect in rects.iter().skip(1) {
        list = list.union(*rect);
    }
    Some(list)
}

#[allow(clippy::too_many_arguments)]
pub fn tabs_keyboard_action(
    orientation: TabsOrientation,
    direction: Option<PrimitiveDirection>,
    enter_pressed: bool,
    space_pressed: bool,
    arrow_up_pressed: bool,
    arrow_down_pressed: bool,
    arrow_left_pressed: bool,
    arrow_right_pressed: bool,
    home_pressed: bool,
    end_pressed: bool,
) -> TabsKeyboardAction {
    if enter_pressed || space_pressed {
        return TabsKeyboardAction::Activate;
    }
    if home_pressed {
        return TabsKeyboardAction::FocusFirst;
    }
    if end_pressed {
        return TabsKeyboardAction::FocusLast;
    }
    match orientation {
        TabsOrientation::Horizontal => match primitive_horizontal_arrow_step(
            direction,
            arrow_left_pressed,
            arrow_right_pressed,
        ) {
            Some(step) if step > 0 => TabsKeyboardAction::FocusNext,
            Some(_) => TabsKeyboardAction::FocusPrevious,
            None => TabsKeyboardAction::None,
        },
        TabsOrientation::Vertical => {
            if arrow_down_pressed {
                TabsKeyboardAction::FocusNext
            } else if arrow_up_pressed {
                TabsKeyboardAction::FocusPrevious
            } else {
                TabsKeyboardAction::None
            }
        }
    }
}

pub fn tabs_apply_keyboard_action(
    selected: &mut usize,
    focus: &mut RovingFocusState,
    enabled: &[bool],
    action: TabsKeyboardAction,
    activation_mode: TabsActivationMode,
    loop_focus: bool,
) -> bool {
    let Some(target) = tabs_keyboard_target_index(enabled, focus.active_index, action, loop_focus)
    else {
        return false;
    };
    focus.active_index = Some(target);
    if action == TabsKeyboardAction::Activate || activation_mode == TabsActivationMode::Automatic {
        if *selected != target {
            *selected = target;
            return true;
        }
    }
    false
}

pub fn tabs_keyboard_target_index(
    enabled: &[bool],
    current: Option<usize>,
    action: TabsKeyboardAction,
    loop_focus: bool,
) -> Option<usize> {
    match action {
        TabsKeyboardAction::None => None,
        TabsKeyboardAction::Activate => {
            current.filter(|index| enabled.get(*index).copied().unwrap_or(false))
        }
        TabsKeyboardAction::FocusFirst => enabled.iter().position(|enabled| *enabled),
        TabsKeyboardAction::FocusLast => enabled.iter().rposition(|enabled| *enabled),
        TabsKeyboardAction::FocusNext => tabs_next_keyboard_index(enabled, current, 1, loop_focus),
        TabsKeyboardAction::FocusPrevious => {
            tabs_next_keyboard_index(enabled, current, -1, loop_focus)
        }
    }
}

fn tabs_next_keyboard_index(
    enabled: &[bool],
    current: Option<usize>,
    direction: isize,
    loop_focus: bool,
) -> Option<usize> {
    if enabled.is_empty() || !enabled.iter().any(|enabled| *enabled) {
        return None;
    }
    let len = enabled.len() as isize;
    let start = current
        .map(|index| index as isize)
        .unwrap_or(if direction >= 0 { -1 } else { len });
    for step in 1..=enabled.len() {
        let raw = start + direction * step as isize;
        if !loop_focus && (raw < 0 || raw >= len) {
            return None;
        }
        let next = raw.rem_euclid(len) as usize;
        if enabled[next] {
            return Some(next);
        }
    }
    None
}

pub fn tabs_index_by_label(items: &[TabItem], query: &str) -> Option<usize> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(index) = trimmed.parse::<usize>() {
        return items.get(index).filter(|item| item.enabled).map(|_| index);
    }

    let query_key = tab_label_key(trimmed);
    items.iter().enumerate().find_map(|(index, item)| {
        if !item.enabled {
            return None;
        }
        let label_key = tab_label_key(item.label);
        let singular_label_key = label_key.strip_suffix('s').unwrap_or(&label_key);
        (label_key == query_key
            || singular_label_key == query_key
            || label_key.ends_with(&query_key))
        .then_some(index)
    })
}

pub fn primitive_tabs_trigger(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    state: TabsTriggerState,
    theme: PrimitiveTheme,
) {
    let chip_rect = rect.shrink2(Vec2::new(2.0, 3.0));
    let fill = if state.active {
        theme.content_fill
    } else if state.hovered && state.enabled {
        theme.item_hover_fill
    } else {
        Color32::TRANSPARENT
    };
    let stroke = if state.active {
        theme.content_stroke
    } else {
        egui::Stroke::NONE
    };
    ui.painter().rect(
        chip_rect,
        theme.row_radius,
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    if state.active {
        let underline = Rect::from_min_max(
            chip_rect.left_bottom() + Vec2::new(8.0, -2.0),
            chip_rect.right_bottom() - Vec2::new(8.0, 0.0),
        );
        ui.painter()
            .rect_filled(underline, 2.0, radix_colors::INDIGO_9);
    }
    ui.painter().text(
        chip_rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        if state.active {
            theme.text
        } else if state.enabled {
            theme.text
        } else {
            theme.disabled_text
        },
    );
}

pub fn tab_rects(bounds: Rect, items: &[TabItem]) -> Vec<Rect> {
    tab_rects_with_orientation(bounds, items, TabsOrientation::Horizontal)
}

pub fn tab_rects_with_orientation(
    bounds: Rect,
    items: &[TabItem],
    orientation: TabsOrientation,
) -> Vec<Rect> {
    match orientation {
        TabsOrientation::Horizontal => horizontal_tab_rects(bounds, items),
        TabsOrientation::Vertical => vertical_tab_rects(bounds, items),
    }
}

fn horizontal_tab_rects(bounds: Rect, items: &[TabItem]) -> Vec<Rect> {
    let mut x = bounds.left();
    let mut y = bounds.top();
    let right = bounds.right().max(bounds.left() + 1.0);
    items
        .iter()
        .map(|item| {
            let width = tab_width(item.label);
            if x > bounds.left() && x + width > right {
                x = bounds.left();
                y += 30.0;
            }
            let rect = Rect::from_min_size(egui::pos2(x, y), Vec2::new(width, 30.0));
            x += width;
            rect
        })
        .collect()
}

fn vertical_tab_rects(bounds: Rect, items: &[TabItem]) -> Vec<Rect> {
    let width = items
        .iter()
        .map(|item| tab_width(item.label))
        .fold(54.0, f32::max)
        .min(bounds.width().max(54.0));
    let mut y = bounds.top();
    items
        .iter()
        .map(|_| {
            let rect = Rect::from_min_size(egui::pos2(bounds.left(), y), Vec2::new(width, 30.0));
            y += 30.0;
            rect
        })
        .collect()
}

fn tab_width(label: &str) -> f32 {
    (label.chars().count() as f32 * 8.0 + 24.0).max(54.0)
}

fn tab_label_key(label: &str) -> String {
    label
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_rects_keep_order_and_minimum_width() {
        let rects = tab_rects(
            Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(400.0, 30.0)),
            &[
                TabItem {
                    label: "A",
                    enabled: true,
                },
                TabItem {
                    label: "Long Tab",
                    enabled: true,
                },
            ],
        );

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].left(), 10.0);
        assert!(rects[0].width() >= 54.0);
        assert_eq!(rects[1].left(), rects[0].right());
    }

    #[test]
    fn tabs_list_rect_covers_all_trigger_widths() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(400.0, 30.0));
        let items = [
            TabItem {
                label: "A",
                enabled: true,
            },
            TabItem {
                label: "Long Tab",
                enabled: true,
            },
        ];
        let rects = tab_rects(bounds, &items);
        let list = primitive_tabs_list_rect(bounds, &items).expect("non-empty tab list");

        assert_eq!(list.left(), bounds.left());
        assert_eq!(list.right(), rects.last().unwrap().right());
        assert_eq!(list.height(), 30.0);
    }

    #[test]
    fn vertical_tab_rects_stack_items_and_list_rect_covers_height() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(400.0, 300.0));
        let items = [
            TabItem {
                label: "A",
                enabled: true,
            },
            TabItem {
                label: "Long Tab",
                enabled: true,
            },
        ];
        let rects = tab_rects_with_orientation(bounds, &items, TabsOrientation::Vertical);
        let list =
            primitive_tabs_list_rect_with_orientation(bounds, &items, TabsOrientation::Vertical)
                .expect("non-empty tab list");

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].top(), bounds.top());
        assert_eq!(rects[1].top(), rects[0].bottom());
        assert_eq!(rects[0].left(), bounds.left());
        assert_eq!(list.top(), bounds.top());
        assert_eq!(list.bottom(), rects.last().unwrap().bottom());
    }

    #[test]
    fn tabs_keyboard_action_respects_orientation_and_activation_keys() {
        assert_eq!(
            tabs_keyboard_action(
                TabsOrientation::Horizontal,
                None,
                false,
                false,
                false,
                false,
                false,
                true,
                false,
                false,
            ),
            TabsKeyboardAction::FocusNext
        );
        assert_eq!(
            tabs_keyboard_action(
                TabsOrientation::Horizontal,
                None,
                false,
                false,
                false,
                true,
                false,
                false,
                false,
                false,
            ),
            TabsKeyboardAction::None
        );
        assert_eq!(
            tabs_keyboard_action(
                TabsOrientation::Vertical,
                None,
                false,
                false,
                false,
                true,
                false,
                false,
                false,
                false,
            ),
            TabsKeyboardAction::FocusNext
        );
        assert_eq!(
            tabs_keyboard_action(
                TabsOrientation::Vertical,
                None,
                false,
                true,
                false,
                false,
                false,
                false,
                false,
                false,
            ),
            TabsKeyboardAction::Activate
        );
    }

    #[test]
    fn tabs_keyboard_action_reverses_horizontal_arrows_in_rtl() {
        assert_eq!(
            tabs_keyboard_action(
                TabsOrientation::Horizontal,
                Some(PrimitiveDirection::Rtl),
                false,
                false,
                false,
                false,
                false,
                true,
                false,
                false,
            ),
            TabsKeyboardAction::FocusPrevious
        );
        assert_eq!(
            tabs_keyboard_action(
                TabsOrientation::Horizontal,
                Some(PrimitiveDirection::Rtl),
                false,
                false,
                false,
                false,
                true,
                false,
                false,
                false,
            ),
            TabsKeyboardAction::FocusNext
        );
    }

    #[test]
    fn tabs_keyboard_target_skips_disabled_and_respects_loop_focus() {
        let enabled = [true, false, true];

        assert_eq!(
            tabs_keyboard_target_index(&enabled, Some(0), TabsKeyboardAction::FocusNext, true),
            Some(2)
        );
        assert_eq!(
            tabs_keyboard_target_index(&enabled, Some(2), TabsKeyboardAction::FocusNext, true),
            Some(0)
        );
        assert_eq!(
            tabs_keyboard_target_index(&enabled, Some(2), TabsKeyboardAction::FocusNext, false),
            None
        );
        assert_eq!(
            tabs_keyboard_target_index(&enabled, Some(0), TabsKeyboardAction::FocusPrevious, true),
            Some(2)
        );
        assert_eq!(
            tabs_keyboard_target_index(&enabled, Some(1), TabsKeyboardAction::Activate, true),
            None
        );
    }

    #[test]
    fn tabs_apply_keyboard_action_separates_manual_focus_from_selected_value() {
        let enabled = [true, true, true];
        let mut selected = 0;
        let mut focus = RovingFocusState {
            active_index: Some(0),
        };

        let changed = tabs_apply_keyboard_action(
            &mut selected,
            &mut focus,
            &enabled,
            TabsKeyboardAction::FocusNext,
            TabsActivationMode::Manual,
            true,
        );

        assert!(!changed);
        assert_eq!(selected, 0);
        assert_eq!(focus.active_index, Some(1));

        let changed = tabs_apply_keyboard_action(
            &mut selected,
            &mut focus,
            &enabled,
            TabsKeyboardAction::Activate,
            TabsActivationMode::Manual,
            true,
        );

        assert!(changed);
        assert_eq!(selected, 1);
        assert_eq!(focus.active_index, Some(1));
    }

    #[test]
    fn tabs_apply_keyboard_action_selects_focused_tab_in_automatic_mode() {
        let enabled = [true, true, true];
        let mut selected = 0;
        let mut focus = RovingFocusState {
            active_index: Some(0),
        };

        let changed = tabs_apply_keyboard_action(
            &mut selected,
            &mut focus,
            &enabled,
            TabsKeyboardAction::FocusNext,
            TabsActivationMode::Automatic,
            true,
        );

        assert!(changed);
        assert_eq!(selected, 1);
        assert_eq!(focus.active_index, Some(1));
    }

    #[test]
    fn tabs_index_by_label_matches_normalized_labels_and_numeric_indices() {
        let items = [
            TabItem {
                label: "Controls",
                enabled: true,
            },
            TabItem {
                label: "Overlays",
                enabled: true,
            },
            TabItem {
                label: "New Parts",
                enabled: true,
            },
        ];

        assert_eq!(tabs_index_by_label(&items, "control"), Some(0));
        assert_eq!(tabs_index_by_label(&items, "overlay"), Some(1));
        assert_eq!(tabs_index_by_label(&items, "new_parts"), Some(2));
        assert_eq!(tabs_index_by_label(&items, "newparts"), Some(2));
        assert_eq!(tabs_index_by_label(&items, "parts"), Some(2));
        assert_eq!(tabs_index_by_label(&items, "2"), Some(2));
    }

    #[test]
    fn tabs_index_by_label_skips_disabled_or_unknown_tabs() {
        let items = [
            TabItem {
                label: "Controls",
                enabled: true,
            },
            TabItem {
                label: "Overlays",
                enabled: false,
            },
        ];

        assert_eq!(tabs_index_by_label(&items, "overlays"), None);
        assert_eq!(tabs_index_by_label(&items, "1"), None);
        assert_eq!(tabs_index_by_label(&items, "missing"), None);
    }

    #[test]
    fn tabs_content_options_preserve_part_contract() {
        let options = TabsContentOptions::default()
            .min_height(120.0)
            .inner_margin(egui::Margin::symmetric(6, 7));

        assert_eq!(options.min_height, 120.0);
        assert_eq!(options.inner_margin.left, 6);
        assert_eq!(options.inner_margin.top, 7);
    }
}
