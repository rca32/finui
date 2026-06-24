use eframe::egui::{self, Align2, FontId, Rect, Response, Sense, Vec2, pos2};

use super::{PrimitiveDirection, PrimitiveTheme, radix_colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarOrientation {
    Horizontal,
    Vertical,
}

impl ToolbarOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolbarItemKind {
    Button,
    Toggle,
    Separator,
    Gap,
    FlexibleSpace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToolbarItemSpec {
    pub kind: ToolbarItemKind,
    pub width: f32,
}

impl ToolbarItemSpec {
    pub fn button(width: f32) -> Self {
        Self {
            kind: ToolbarItemKind::Button,
            width,
        }
    }

    pub fn toggle(width: f32) -> Self {
        Self {
            kind: ToolbarItemKind::Toggle,
            width,
        }
    }

    pub fn separator() -> Self {
        Self {
            kind: ToolbarItemKind::Separator,
            width: 1.0,
        }
    }

    pub fn gap(width: f32) -> Self {
        Self {
            kind: ToolbarItemKind::Gap,
            width,
        }
    }

    pub fn flexible_space() -> Self {
        Self {
            kind: ToolbarItemKind::FlexibleSpace,
            width: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToolbarButtonSpec {
    pub label: &'static str,
    pub kind: ToolbarItemKind,
    pub pressed: bool,
    pub enabled: bool,
    pub width: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarToggleGroupMode {
    Single,
    Multiple,
}

impl ToolbarToggleGroupMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
}

impl ToolbarButtonSpec {
    pub fn button(label: &'static str, width: f32) -> Self {
        Self {
            label,
            kind: ToolbarItemKind::Button,
            pressed: false,
            enabled: true,
            width,
        }
    }

    pub fn toggle(label: &'static str, pressed: bool, width: f32) -> Self {
        Self {
            label,
            kind: ToolbarItemKind::Toggle,
            pressed,
            enabled: true,
            width,
        }
    }

    pub fn separator() -> Self {
        Self {
            label: "",
            kind: ToolbarItemKind::Separator,
            pressed: false,
            enabled: false,
            width: 1.0,
        }
    }

    pub fn gap(width: f32) -> Self {
        Self {
            label: "",
            kind: ToolbarItemKind::Gap,
            pressed: false,
            enabled: false,
            width,
        }
    }

    pub fn flexible_space() -> Self {
        Self {
            label: "",
            kind: ToolbarItemKind::FlexibleSpace,
            pressed: false,
            enabled: false,
            width: 0.0,
        }
    }

    fn item_spec(self) -> ToolbarItemSpec {
        ToolbarItemSpec {
            kind: self.kind,
            width: self.width,
        }
    }
}

pub struct ToolbarOutput {
    pub clicked: Option<usize>,
    pub responses: Vec<Response>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToolbarActionOutput {
    pub index: usize,
    pub label: &'static str,
    pub kind: ToolbarItemKind,
    pub pressed: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToolbarButtonState {
    pub pressed: bool,
    pub enabled: bool,
    pub hovered: bool,
}

impl ToolbarButtonState {
    pub fn from_spec(spec: ToolbarButtonSpec, hovered: bool) -> Self {
        Self {
            pressed: spec.pressed,
            enabled: spec.enabled,
            hovered,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolbarRootOptions {
    pub orientation: ToolbarOrientation,
    pub direction: Option<PrimitiveDirection>,
    pub loop_focus: bool,
}

impl Default for ToolbarRootOptions {
    fn default() -> Self {
        Self {
            orientation: ToolbarOrientation::Horizontal,
            direction: None,
            loop_focus: true,
        }
    }
}

impl ToolbarRootOptions {
    pub fn orientation(mut self, orientation: ToolbarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn direction(mut self, direction: Option<PrimitiveDirection>) -> Self {
        self.direction = direction;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolbarRootOutput {
    pub orientation: ToolbarOrientation,
    pub direction: Option<PrimitiveDirection>,
    pub loop_focus: bool,
    pub data_orientation: &'static str,
    pub role: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolbarButtonOutput {
    pub data_orientation: &'static str,
    pub disabled: bool,
    pub data_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolbarLinkOutput {
    pub href: &'static str,
    pub target: Option<&'static str>,
    pub data_orientation: &'static str,
    pub disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolbarToggleGroupOutput {
    pub mode: ToolbarToggleGroupMode,
    pub value: Vec<&'static str>,
    pub default_value: Vec<&'static str>,
    pub disabled: bool,
    pub data_orientation: &'static str,
    pub data_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolbarToggleItemOutput {
    pub value: &'static str,
    pub pressed: bool,
    pub disabled: bool,
    pub data_state: &'static str,
    pub data_disabled: bool,
    pub data_orientation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolbarSeparatorOutput {
    pub data_orientation: &'static str,
    pub aria_orientation: &'static str,
}

pub fn primitive_toolbar_root_output(options: ToolbarRootOptions) -> ToolbarRootOutput {
    ToolbarRootOutput {
        orientation: options.orientation,
        direction: options.direction,
        loop_focus: options.loop_focus,
        data_orientation: options.orientation.as_str(),
        role: "toolbar",
    }
}

pub fn primitive_toolbar_button_output(
    root: ToolbarRootOptions,
    disabled: bool,
) -> ToolbarButtonOutput {
    ToolbarButtonOutput {
        data_orientation: root.orientation.as_str(),
        disabled,
        data_disabled: disabled,
    }
}

pub fn primitive_toolbar_link_output(
    root: ToolbarRootOptions,
    href: &'static str,
    target: Option<&'static str>,
    disabled: bool,
) -> ToolbarLinkOutput {
    ToolbarLinkOutput {
        href,
        target,
        data_orientation: root.orientation.as_str(),
        disabled,
    }
}

pub fn primitive_toolbar_toggle_group_output(
    root: ToolbarRootOptions,
    mode: ToolbarToggleGroupMode,
    value: Vec<&'static str>,
    default_value: Vec<&'static str>,
    disabled: bool,
) -> ToolbarToggleGroupOutput {
    ToolbarToggleGroupOutput {
        mode,
        value,
        default_value,
        disabled,
        data_orientation: root.orientation.as_str(),
        data_disabled: disabled,
    }
}

pub fn primitive_toolbar_toggle_item_output(
    root: ToolbarRootOptions,
    value: &'static str,
    pressed: bool,
    disabled: bool,
) -> ToolbarToggleItemOutput {
    ToolbarToggleItemOutput {
        value,
        pressed,
        disabled,
        data_state: if pressed { "on" } else { "off" },
        data_disabled: disabled,
        data_orientation: root.orientation.as_str(),
    }
}

pub fn primitive_toolbar_separator_output(root: ToolbarRootOptions) -> ToolbarSeparatorOutput {
    ToolbarSeparatorOutput {
        data_orientation: root.orientation.as_str(),
        aria_orientation: root.orientation.as_str(),
    }
}

pub fn primitive_toolbar(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    items: &[ToolbarButtonSpec],
    height: f32,
    gap: f32,
    theme: PrimitiveTheme,
) -> ToolbarOutput {
    let desired_width = ui.available_width();
    let (bounds, _) = ui.allocate_exact_size(Vec2::new(desired_width, height), Sense::hover());
    let specs: Vec<_> = items.iter().map(|item| item.item_spec()).collect();
    let rects = toolbar_item_rects(bounds, &specs, gap);
    let root_id = ui.id().with(id_source);
    let mut clicked = None;
    let mut responses = Vec::with_capacity(items.len());
    for (index, (item, rect)) in items.iter().zip(rects.iter()).enumerate() {
        match item.kind {
            ToolbarItemKind::Button | ToolbarItemKind::Toggle => {
                let sense = if item.enabled {
                    Sense::click()
                } else {
                    Sense::hover()
                };
                let response = ui.interact(*rect, root_id.with(index), sense);
                let response = if item.enabled {
                    response.on_hover_cursor(egui::CursorIcon::PointingHand)
                } else {
                    response
                };
                primitive_toolbar_button(
                    ui,
                    *rect,
                    item.label,
                    ToolbarButtonState::from_spec(*item, response.hovered()),
                    theme,
                );
                if item.enabled && response.clicked() {
                    clicked = Some(index);
                }
                responses.push(response);
            }
            ToolbarItemKind::Separator => {
                let response = ui.interact(*rect, root_id.with(index), Sense::hover());
                primitive_toolbar_separator(ui, *rect, theme);
                responses.push(response);
            }
            ToolbarItemKind::Gap | ToolbarItemKind::FlexibleSpace => {
                responses.push(ui.interact(*rect, root_id.with(index), Sense::hover()));
            }
        }
    }
    ToolbarOutput { clicked, responses }
}

pub fn primitive_toolbar_button(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    state: ToolbarButtonState,
    theme: PrimitiveTheme,
) {
    let fill = if !state.enabled {
        theme.content_fill
    } else if state.pressed && state.hovered {
        theme.item_selected_fill
    } else if state.pressed {
        theme.item_selected_fill
    } else if state.hovered && state.enabled {
        theme.item_hover_fill
    } else {
        theme.content_fill
    };
    let stroke = if !state.enabled {
        theme.content_stroke
    } else if state.pressed && state.hovered {
        egui::Stroke::new(1.0, theme.text)
    } else if state.pressed {
        egui::Stroke::new(1.0, theme.text)
    } else if state.hovered {
        theme.content_stroke
    } else {
        theme.content_stroke
    };
    let text_color = if !state.enabled {
        theme.disabled_text
    } else if state.pressed {
        theme.text
    } else {
        theme.text
    };
    ui.painter()
        .rect_filled(rect.shrink(1.0), theme.row_radius, fill);
    ui.painter().rect_stroke(
        rect.shrink(1.0),
        theme.row_radius,
        stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        text_color,
    );
}

pub fn primitive_toolbar_toggle_item(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    pressed: bool,
    enabled: bool,
    hovered: bool,
    theme: PrimitiveTheme,
) {
    primitive_toolbar_button(
        ui,
        rect,
        label,
        ToolbarButtonState {
            pressed,
            enabled,
            hovered,
        },
        theme,
    );
}

pub fn primitive_toolbar_separator(ui: &egui::Ui, rect: Rect, theme: PrimitiveTheme) {
    ui.painter().vline(
        rect.center().x,
        rect.top() + 6.0..=rect.bottom() - 6.0,
        theme.content_stroke,
    );
}

pub fn toolbar_apply_action(
    items: &[ToolbarButtonSpec],
    index: usize,
) -> Option<ToolbarActionOutput> {
    let item = items.get(index)?;
    if !item.enabled {
        return None;
    }
    match item.kind {
        ToolbarItemKind::Button => Some(ToolbarActionOutput {
            index,
            label: item.label,
            kind: item.kind,
            pressed: None,
        }),
        ToolbarItemKind::Toggle => Some(ToolbarActionOutput {
            index,
            label: item.label,
            kind: item.kind,
            pressed: Some(!item.pressed),
        }),
        ToolbarItemKind::Separator | ToolbarItemKind::Gap | ToolbarItemKind::FlexibleSpace => None,
    }
}

pub fn toolbar_item_rects(bounds: Rect, items: &[ToolbarItemSpec], gap: f32) -> Vec<Rect> {
    let fixed_width = items
        .iter()
        .filter(|item| item.kind != ToolbarItemKind::FlexibleSpace)
        .map(|item| item.width.max(0.0))
        .sum::<f32>();
    let gaps = gap.max(0.0) * items.len().saturating_sub(1) as f32;
    let flex_count = items
        .iter()
        .filter(|item| item.kind == ToolbarItemKind::FlexibleSpace)
        .count();
    let flex_width = if flex_count > 0 {
        ((bounds.width() - fixed_width - gaps).max(0.0)) / flex_count as f32
    } else {
        0.0
    };

    let mut x = bounds.left();
    items
        .iter()
        .map(|item| {
            let width = match item.kind {
                ToolbarItemKind::FlexibleSpace => flex_width,
                _ => item.width.max(0.0),
            };
            let rect =
                Rect::from_min_size(pos2(x, bounds.top()), Vec2::new(width, bounds.height()));
            x += width + gap.max(0.0);
            rect
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolbar_item_rects_assign_remaining_width_to_flexible_space() {
        let rects = toolbar_item_rects(
            Rect::from_min_size(pos2(0.0, 0.0), Vec2::new(200.0, 32.0)),
            &[
                ToolbarItemSpec::button(32.0),
                ToolbarItemSpec::flexible_space(),
                ToolbarItemSpec::toggle(32.0),
            ],
            4.0,
        );

        assert_eq!(rects[0].width(), 32.0);
        assert_eq!(rects[1].width(), 128.0);
        assert_eq!(rects[2].right(), 200.0);
    }

    #[test]
    fn toolbar_item_rects_preserve_explicit_gap_items() {
        let rects = toolbar_item_rects(
            Rect::from_min_size(pos2(10.0, 20.0), Vec2::new(73.0, 26.0)),
            &[
                ToolbarItemSpec::button(26.0),
                ToolbarItemSpec::gap(5.0),
                ToolbarItemSpec::button(42.0),
            ],
            0.0,
        );

        assert_eq!(rects[0].left(), 10.0);
        assert_eq!(rects[1].width(), 5.0);
        assert_eq!(rects[2].left(), 41.0);
    }

    #[test]
    fn toolbar_button_specs_convert_to_layout_specs() {
        let button = ToolbarButtonSpec::toggle("B", true, 28.0);
        let spec = button.item_spec();

        assert_eq!(spec.kind, ToolbarItemKind::Toggle);
        assert_eq!(spec.width, 28.0);
        assert!(button.pressed);
    }

    #[test]
    fn toolbar_button_state_preserves_spec_state() {
        let state = ToolbarButtonState::from_spec(ToolbarButtonSpec::toggle("B", true, 28.0), true);

        assert!(state.pressed);
        assert!(state.enabled);
        assert!(state.hovered);
    }

    #[test]
    fn toolbar_apply_action_respects_kind_enabled_and_toggle_state() {
        let mut disabled = ToolbarButtonSpec::button("Off", 32.0);
        disabled.enabled = false;
        let items = [
            ToolbarButtonSpec::button("New", 32.0),
            ToolbarButtonSpec::toggle("Pin", false, 32.0),
            ToolbarButtonSpec::separator(),
            disabled,
        ];

        assert_eq!(
            toolbar_apply_action(&items, 0),
            Some(ToolbarActionOutput {
                index: 0,
                label: "New",
                kind: ToolbarItemKind::Button,
                pressed: None
            })
        );
        assert_eq!(
            toolbar_apply_action(&items, 1),
            Some(ToolbarActionOutput {
                index: 1,
                label: "Pin",
                kind: ToolbarItemKind::Toggle,
                pressed: Some(true)
            })
        );
        assert_eq!(toolbar_apply_action(&items, 2), None);
        assert_eq!(toolbar_apply_action(&items, 3), None);
        assert_eq!(toolbar_apply_action(&items, 9), None);
    }

    #[test]
    fn toolbar_root_output_preserves_radix_contract() {
        let output = primitive_toolbar_root_output(
            ToolbarRootOptions::default()
                .orientation(ToolbarOrientation::Vertical)
                .direction(Some(PrimitiveDirection::Rtl))
                .loop_focus(false),
        );

        assert_eq!(output.orientation, ToolbarOrientation::Vertical);
        assert_eq!(output.direction, Some(PrimitiveDirection::Rtl));
        assert!(!output.loop_focus);
        assert_eq!(output.data_orientation, "vertical");
        assert_eq!(output.role, "toolbar");
    }

    #[test]
    fn toolbar_item_outputs_expose_orientation_and_disabled_contracts() {
        let root = ToolbarRootOptions::default().orientation(ToolbarOrientation::Horizontal);
        let button = primitive_toolbar_button_output(root, true);
        let link = primitive_toolbar_link_output(root, "#chart", Some("_blank"), false);
        let separator = primitive_toolbar_separator_output(root);

        assert_eq!(button.data_orientation, "horizontal");
        assert!(button.disabled);
        assert!(button.data_disabled);
        assert_eq!(link.href, "#chart");
        assert_eq!(link.target, Some("_blank"));
        assert_eq!(link.data_orientation, "horizontal");
        assert_eq!(separator.data_orientation, "horizontal");
        assert_eq!(separator.aria_orientation, "horizontal");
    }

    #[test]
    fn toolbar_toggle_outputs_match_group_and_item_data_attributes() {
        let root = ToolbarRootOptions::default();
        let group = primitive_toolbar_toggle_group_output(
            root,
            ToolbarToggleGroupMode::Multiple,
            vec!["bold", "italic"],
            vec!["bold"],
            false,
        );
        let on_item = primitive_toolbar_toggle_item_output(root, "bold", true, false);
        let disabled_item = primitive_toolbar_toggle_item_output(root, "strike", false, true);

        assert_eq!(group.mode.as_str(), "multiple");
        assert_eq!(group.value, vec!["bold", "italic"]);
        assert_eq!(group.default_value, vec!["bold"]);
        assert_eq!(group.data_orientation, "horizontal");
        assert_eq!(on_item.value, "bold");
        assert_eq!(on_item.data_state, "on");
        assert_eq!(disabled_item.data_state, "off");
        assert!(disabled_item.data_disabled);
    }
}
