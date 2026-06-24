use std::hash::Hash;

use eframe::egui::{self, FontId, Rect, Response, Stroke, Vec2};

use super::{PrimitiveTheme, radix_colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupMode {
    Single,
    Multiple,
}

impl ToggleGroupMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupOrientation {
    Horizontal,
    Vertical,
}

impl ToggleGroupOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleGroupDirection {
    Ltr,
    Rtl,
}

impl ToggleGroupDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToggleDataState {
    On,
    Off,
}

impl ToggleDataState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::On => "on",
            Self::Off => "off",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToggleButtonOptions {
    pub width: f32,
    pub height: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for ToggleButtonOptions {
    fn default() -> Self {
        Self {
            width: 34.0,
            height: 28.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

pub struct ToggleButtonOutput {
    pub response: Response,
    pub changed: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleGroupRootOptions {
    pub mode: ToggleGroupMode,
    pub value: Vec<String>,
    pub default_value: Vec<String>,
    pub disabled: bool,
    pub roving_focus: bool,
    pub orientation: ToggleGroupOrientation,
    pub direction: Option<ToggleGroupDirection>,
    pub loop_focus: bool,
    pub aria_label: Option<String>,
    pub theme: PrimitiveTheme,
}

impl ToggleGroupRootOptions {
    pub fn new(mode: ToggleGroupMode) -> Self {
        Self {
            mode,
            value: Vec::new(),
            default_value: Vec::new(),
            disabled: false,
            roving_focus: true,
            orientation: ToggleGroupOrientation::Horizontal,
            direction: None,
            loop_focus: true,
            aria_label: None,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = vec![value.into()];
        self
    }

    pub fn values(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.value = values.into_iter().map(Into::into).collect();
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = vec![value.into()];
        self
    }

    pub fn default_values(mut self, values: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.default_value = values.into_iter().map(Into::into).collect();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn roving_focus(mut self, roving_focus: bool) -> Self {
        self.roving_focus = roving_focus;
        self
    }

    pub fn orientation(mut self, orientation: ToggleGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn direction(mut self, direction: ToggleGroupDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleGroupRootOutput {
    pub mode: ToggleGroupMode,
    pub value: Vec<String>,
    pub default_value: Vec<String>,
    pub disabled: bool,
    pub roving_focus: bool,
    pub orientation: ToggleGroupOrientation,
    pub data_orientation: &'static str,
    pub direction: Option<ToggleGroupDirection>,
    pub loop_focus: bool,
    pub aria_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleRootOptions {
    pub pressed: bool,
    pub default_pressed: Option<bool>,
    pub disabled: bool,
    pub aria_label: Option<String>,
    pub theme: PrimitiveTheme,
}

impl ToggleRootOptions {
    pub fn new(pressed: bool) -> Self {
        Self {
            pressed,
            default_pressed: None,
            disabled: false,
            aria_label: None,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn default_pressed(mut self, default_pressed: bool) -> Self {
        self.default_pressed = Some(default_pressed);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleRootOutput {
    pub pressed: bool,
    pub default_pressed: Option<bool>,
    pub disabled: bool,
    pub data_state: ToggleDataState,
    pub aria_pressed: bool,
    pub aria_label: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToggleGroupItemState {
    pub selected: bool,
    pub enabled: bool,
}

impl ToggleGroupItemState {
    pub fn from_item(selected: bool, group_enabled: bool, item: ToggleGroupItem) -> Self {
        Self {
            selected,
            enabled: group_enabled && item.enabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToggleRootState {
    pub pressed: bool,
    pub hovered: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToggleGroupItem {
    pub label: &'static str,
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleGroupItemOptions {
    pub value: String,
    pub selected: bool,
    pub disabled: bool,
    pub orientation: ToggleGroupOrientation,
    pub aria_label: Option<String>,
    pub theme: PrimitiveTheme,
}

impl ToggleGroupItemOptions {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            selected: false,
            disabled: false,
            orientation: ToggleGroupOrientation::Horizontal,
            aria_label: None,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn orientation(mut self, orientation: ToggleGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn aria_label(mut self, aria_label: impl Into<String>) -> Self {
        self.aria_label = Some(aria_label.into());
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleGroupItemOutput {
    pub value: String,
    pub selected: bool,
    pub disabled: bool,
    pub data_state: ToggleDataState,
    pub data_orientation: &'static str,
    pub aria_pressed: bool,
    pub aria_label: Option<String>,
}

pub struct ToggleGroupOutput {
    pub changed: bool,
}

pub fn primitive_toggle_group_root_output(
    options: ToggleGroupRootOptions,
) -> ToggleGroupRootOutput {
    ToggleGroupRootOutput {
        mode: options.mode,
        value: options.value,
        default_value: options.default_value,
        disabled: options.disabled,
        roving_focus: options.roving_focus,
        orientation: options.orientation,
        data_orientation: options.orientation.as_str(),
        direction: options.direction,
        loop_focus: options.loop_focus,
        aria_label: options.aria_label,
    }
}

pub fn primitive_toggle_group_item_output(
    options: ToggleGroupItemOptions,
) -> ToggleGroupItemOutput {
    ToggleGroupItemOutput {
        value: options.value,
        selected: options.selected,
        disabled: options.disabled,
        data_state: if options.selected {
            ToggleDataState::On
        } else {
            ToggleDataState::Off
        },
        data_orientation: options.orientation.as_str(),
        aria_pressed: options.selected,
        aria_label: options.aria_label,
    }
}

pub fn primitive_toggle_root_output(options: ToggleRootOptions) -> ToggleRootOutput {
    ToggleRootOutput {
        pressed: options.pressed,
        default_pressed: options.default_pressed,
        disabled: options.disabled,
        data_state: if options.pressed {
            ToggleDataState::On
        } else {
            ToggleDataState::Off
        },
        aria_pressed: options.pressed,
        aria_label: options.aria_label,
    }
}

pub fn primitive_toggle_root_state(root: &ToggleRootOutput, hovered: bool) -> ToggleRootState {
    ToggleRootState {
        pressed: root.pressed,
        hovered,
        enabled: !root.disabled,
    }
}

pub fn toggle_apply_pressed(current: &mut bool, next: bool, options: ToggleRootOptions) -> bool {
    if options.disabled || *current == next {
        return false;
    }
    *current = next;
    true
}

pub fn primitive_toggle(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    pressed: &mut bool,
    label: &str,
    options: ToggleButtonOptions,
) -> ToggleButtonOutput {
    let root_options = ToggleRootOptions::new(*pressed)
        .disabled(!options.enabled)
        .aria_label(label)
        .theme(options.theme);
    let root = primitive_toggle_root_output(root_options);
    let sense = if !root.disabled {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };
    let (rect, _) = ui.allocate_exact_size(Vec2::new(options.width, options.height), sense);
    let response = ui.interact(rect, ui.id().with(id_source), sense);
    let response = if root.disabled {
        response
    } else {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    };
    let changed = !root.disabled && response.clicked();
    if changed {
        *pressed = !*pressed;
    }
    let root = primitive_toggle_root_output(
        ToggleRootOptions::new(*pressed)
            .disabled(!options.enabled)
            .aria_label(label)
            .theme(options.theme),
    );
    primitive_toggle_root(
        ui,
        rect,
        primitive_toggle_root_state(&root, response.hovered()),
        options,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        toggle_root_text_color(
            primitive_toggle_root_state(&root, response.hovered()),
            options.theme,
        ),
    );
    ToggleButtonOutput { response, changed }
}

pub fn primitive_toggle_root(
    ui: &egui::Ui,
    rect: Rect,
    state: ToggleRootState,
    options: ToggleButtonOptions,
) {
    let target = rect.shrink(1.0);
    ui.painter().rect(
        target,
        options.theme.row_radius,
        toggle_root_fill(state, options.theme),
        Stroke::new(
            1.0,
            if state.pressed {
                options.theme.text
            } else {
                options.theme.content_stroke.color
            },
        ),
        egui::StrokeKind::Inside,
    );
}

pub fn toggle_root_fill(state: ToggleRootState, theme: PrimitiveTheme) -> egui::Color32 {
    if state.pressed {
        theme.item_selected_fill
    } else if state.hovered && state.enabled {
        theme.item_hover_fill
    } else {
        theme.content_fill
    }
}

pub fn toggle_root_text_color(state: ToggleRootState, theme: PrimitiveTheme) -> egui::Color32 {
    if !state.enabled {
        theme.disabled_text
    } else if state.pressed {
        toggle_pressed_text_color(theme)
    } else {
        theme.text
    }
}

fn toggle_pressed_text_color(theme: PrimitiveTheme) -> egui::Color32 {
    if theme == PrimitiveTheme::light() {
        radix_colors::INDIGO_11
    } else {
        theme.text
    }
}

pub fn primitive_toggle_group_item(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    label: &str,
    state: ToggleGroupItemState,
    options: ToggleButtonOptions,
) -> ToggleButtonOutput {
    let mut pressed = state.selected;
    let item_output = primitive_toggle_group_item_output(
        ToggleGroupItemOptions::new(label)
            .selected(state.selected)
            .disabled(!state.enabled),
    );
    primitive_toggle(
        ui,
        id_source,
        &mut pressed,
        label,
        ToggleButtonOptions {
            enabled: !item_output.disabled,
            ..options
        },
    )
}

pub fn primitive_toggle_group(
    ui: &mut egui::Ui,
    id_source: impl Hash + Clone,
    selected: &mut Vec<usize>,
    mode: ToggleGroupMode,
    items: &[ToggleGroupItem],
    options: ToggleButtonOptions,
) -> ToggleGroupOutput {
    let mut changed = false;
    let root = primitive_toggle_group_root_output(
        ToggleGroupRootOptions::new(mode)
            .values(selected.iter().map(|index| index.to_string()))
            .disabled(!options.enabled),
    );
    ui.horizontal(|ui| {
        for (index, item) in items.iter().enumerate() {
            let state =
                ToggleGroupItemState::from_item(selected.contains(&index), !root.disabled, *item);
            let output =
                primitive_toggle_group_item(ui, (&id_source, index), item.label, state, options);
            if output.changed && toggle_group_apply_item(selected, index, mode, items) {
                changed = true;
            }
        }
    });
    ToggleGroupOutput { changed }
}

pub fn toggle_group_apply_item(
    current: &mut Vec<usize>,
    clicked: usize,
    mode: ToggleGroupMode,
    items: &[ToggleGroupItem],
) -> bool {
    if !items.get(clicked).is_some_and(|item| item.enabled) {
        return false;
    }
    apply_toggle_group_action(current, clicked, mode)
}

pub fn apply_toggle_group_action(
    current: &mut Vec<usize>,
    clicked: usize,
    mode: ToggleGroupMode,
) -> bool {
    match mode {
        ToggleGroupMode::Single => {
            if current.len() == 1 && current[0] == clicked {
                return false;
            }
            current.clear();
            current.push(clicked);
            true
        }
        ToggleGroupMode::Multiple => {
            if let Some(pos) = current.iter().position(|index| *index == clicked) {
                current.remove(pos);
            } else {
                current.push(clicked);
                current.sort_unstable();
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_toggle_group_replaces_selection() {
        let mut selected = vec![0];

        assert!(apply_toggle_group_action(
            &mut selected,
            2,
            ToggleGroupMode::Single
        ));
        assert_eq!(selected, vec![2]);
        assert!(!apply_toggle_group_action(
            &mut selected,
            2,
            ToggleGroupMode::Single
        ));
    }

    #[test]
    fn multiple_toggle_group_toggles_membership() {
        let mut selected = vec![1, 3];

        assert!(apply_toggle_group_action(
            &mut selected,
            2,
            ToggleGroupMode::Multiple
        ));
        assert_eq!(selected, vec![1, 2, 3]);
        assert!(apply_toggle_group_action(
            &mut selected,
            1,
            ToggleGroupMode::Multiple
        ));
        assert_eq!(selected, vec![2, 3]);
    }

    #[test]
    fn toggle_group_apply_item_rejects_disabled_and_missing_items() {
        let items = [
            ToggleGroupItem {
                label: "A",
                enabled: true,
            },
            ToggleGroupItem {
                label: "B",
                enabled: false,
            },
        ];
        let mut selected = vec![0];

        assert!(!toggle_group_apply_item(
            &mut selected,
            1,
            ToggleGroupMode::Single,
            &items
        ));
        assert_eq!(selected, vec![0]);
        assert!(!toggle_group_apply_item(
            &mut selected,
            9,
            ToggleGroupMode::Single,
            &items
        ));
        assert_eq!(selected, vec![0]);
    }

    #[test]
    fn toggle_group_items_expose_enabled_state() {
        let item = ToggleGroupItem {
            label: "A",
            enabled: false,
        };

        assert_eq!(item.label, "A");
        assert!(!item.enabled);
    }

    #[test]
    fn toggle_group_item_state_combines_selected_item_and_group_state() {
        let item = ToggleGroupItem {
            label: "A",
            enabled: true,
        };

        assert_eq!(
            ToggleGroupItemState::from_item(true, true, item),
            ToggleGroupItemState {
                selected: true,
                enabled: true,
            }
        );
        assert_eq!(
            ToggleGroupItemState::from_item(false, false, item),
            ToggleGroupItemState {
                selected: false,
                enabled: false,
            }
        );
    }

    #[test]
    fn toggle_group_root_output_preserves_radix_group_contract() {
        let output = primitive_toggle_group_root_output(
            ToggleGroupRootOptions::new(ToggleGroupMode::Multiple)
                .values(["bold", "italic"])
                .default_values(["bold"])
                .disabled(true)
                .roving_focus(false)
                .orientation(ToggleGroupOrientation::Vertical)
                .direction(ToggleGroupDirection::Rtl)
                .loop_focus(false)
                .aria_label("텍스트 스타일"),
        );

        assert_eq!(output.mode, ToggleGroupMode::Multiple);
        assert_eq!(output.mode.as_str(), "multiple");
        assert_eq!(output.value, vec!["bold".to_owned(), "italic".to_owned()]);
        assert_eq!(output.default_value, vec!["bold".to_owned()]);
        assert!(output.disabled);
        assert!(!output.roving_focus);
        assert_eq!(output.data_orientation, "vertical");
        assert_eq!(output.direction, Some(ToggleGroupDirection::Rtl));
        assert_eq!(output.direction.unwrap().as_str(), "rtl");
        assert!(!output.loop_focus);
        assert_eq!(output.aria_label.as_deref(), Some("텍스트 스타일"));
    }

    #[test]
    fn toggle_group_item_output_preserves_value_state_and_orientation() {
        let output = primitive_toggle_group_item_output(
            ToggleGroupItemOptions::new("center")
                .selected(true)
                .orientation(ToggleGroupOrientation::Horizontal)
                .aria_label("가운데 정렬"),
        );

        assert_eq!(output.value, "center");
        assert_eq!(output.data_state, ToggleDataState::On);
        assert_eq!(output.data_orientation, "horizontal");
        assert!(output.aria_pressed);
        assert_eq!(output.aria_label.as_deref(), Some("가운데 정렬"));
    }

    #[test]
    fn toggle_root_output_preserves_radix_state_contract() {
        let output = primitive_toggle_root_output(
            ToggleRootOptions::new(true)
                .default_pressed(false)
                .aria_label("기울임"),
        );

        assert!(output.pressed);
        assert_eq!(output.default_pressed, Some(false));
        assert_eq!(output.data_state, ToggleDataState::On);
        assert_eq!(output.data_state.as_str(), "on");
        assert!(output.aria_pressed);
        assert_eq!(output.aria_label.as_deref(), Some("기울임"));
    }

    #[test]
    fn toggle_apply_pressed_respects_disabled_and_noop_state() {
        let mut pressed = false;

        assert!(toggle_apply_pressed(
            &mut pressed,
            true,
            ToggleRootOptions::new(false)
        ));
        assert!(pressed);
        assert!(!toggle_apply_pressed(
            &mut pressed,
            true,
            ToggleRootOptions::new(true)
        ));
        assert!(!toggle_apply_pressed(
            &mut pressed,
            false,
            ToggleRootOptions::new(true).disabled(true)
        ));
        assert!(pressed);
    }

    #[test]
    fn toggle_root_state_maps_disabled_output_to_render_state() {
        let output = primitive_toggle_root_output(ToggleRootOptions::new(false).disabled(true));

        assert_eq!(output.data_state, ToggleDataState::Off);
        assert_eq!(
            primitive_toggle_root_state(&output, true),
            ToggleRootState {
                pressed: false,
                hovered: true,
                enabled: false,
            }
        );
    }

    #[test]
    fn toggle_root_fill_tracks_pressed_hovered_and_disabled_states() {
        let theme = PrimitiveTheme::default();

        assert_eq!(
            toggle_root_fill(
                ToggleRootState {
                    pressed: true,
                    hovered: false,
                    enabled: true,
                },
                theme,
            ),
            theme.item_selected_fill
        );
        assert_eq!(
            toggle_root_fill(
                ToggleRootState {
                    pressed: false,
                    hovered: true,
                    enabled: true,
                },
                theme,
            ),
            theme.item_hover_fill
        );
        assert_eq!(
            toggle_root_fill(
                ToggleRootState {
                    pressed: false,
                    hovered: true,
                    enabled: false,
                },
                theme,
            ),
            theme.content_fill
        );
    }

    #[test]
    fn toggle_root_text_color_makes_pressed_state_distinct() {
        let theme = PrimitiveTheme::default();

        assert_eq!(
            toggle_root_text_color(
                ToggleRootState {
                    pressed: true,
                    hovered: false,
                    enabled: true,
                },
                theme,
            ),
            radix_colors::INDIGO_11
        );
        assert_eq!(
            toggle_root_text_color(
                ToggleRootState {
                    pressed: false,
                    hovered: false,
                    enabled: false,
                },
                theme,
            ),
            theme.disabled_text
        );
    }
}
