use eframe::egui::{self, Align2, FontId, Rect, Response, Sense, Vec2};

use super::{PrimitiveTheme, paint_radix_icon, radix_icon_visual};

#[derive(Debug, Clone, Copy)]
pub struct AccessibleIconOptions {
    pub size: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for AccessibleIconOptions {
    fn default() -> Self {
        Self {
            size: 32.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl AccessibleIconOptions {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.enabled = !disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessibleIconRootOptions {
    pub label: String,
    pub decorative: bool,
    pub size: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl AccessibleIconRootOptions {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            decorative: false,
            size: 32.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
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

#[derive(Debug, Clone, PartialEq)]
pub struct AccessibleIconRootOutput {
    pub label: Option<String>,
    pub decorative: bool,
    pub size: f32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveAccessibilityRole {
    Generic,
    Button,
    Checkbox,
    Dialog,
    AlertDialog,
    Group,
    Menu,
    MenuItem,
    Slider,
    Status,
    Textbox,
    Tooltip,
    Toolbar,
}

impl PrimitiveAccessibilityRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Button => "button",
            Self::Checkbox => "checkbox",
            Self::Dialog => "dialog",
            Self::AlertDialog => "alertdialog",
            Self::Group => "group",
            Self::Menu => "menu",
            Self::MenuItem => "menuitem",
            Self::Slider => "slider",
            Self::Status => "status",
            Self::Textbox => "textbox",
            Self::Tooltip => "tooltip",
            Self::Toolbar => "toolbar",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveAccessibilityLive {
    Off,
    Polite,
    Assertive,
}

impl PrimitiveAccessibilityLive {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveAccessibilityState {
    pub key: &'static str,
    pub value: String,
}

impl PrimitiveAccessibilityState {
    pub fn new(key: &'static str, value: impl Into<String>) -> Self {
        Self {
            key,
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveAccessibilityNodeOptions {
    pub id: String,
    pub role: PrimitiveAccessibilityRole,
    pub name: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub live: PrimitiveAccessibilityLive,
    pub states: Vec<PrimitiveAccessibilityState>,
}

impl PrimitiveAccessibilityNodeOptions {
    pub fn new(id: impl Into<String>, role: PrimitiveAccessibilityRole) -> Self {
        Self {
            id: id.into(),
            role,
            name: None,
            description: None,
            value: None,
            live: PrimitiveAccessibilityLive::Off,
            states: Vec::new(),
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn live(mut self, live: PrimitiveAccessibilityLive) -> Self {
        self.live = live;
        self
    }

    pub fn state(mut self, key: &'static str, value: impl Into<String>) -> Self {
        self.states
            .push(PrimitiveAccessibilityState::new(key, value));
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveAccessibilityNodeOutput {
    pub id: String,
    pub role: PrimitiveAccessibilityRole,
    pub role_name: &'static str,
    pub name: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub live: PrimitiveAccessibilityLive,
    pub live_name: &'static str,
    pub states: Vec<PrimitiveAccessibilityState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveAccessibilityTreeOutput {
    pub nodes: Vec<PrimitiveAccessibilityNodeOutput>,
}

pub fn primitive_accessible_icon_root_output(
    options: AccessibleIconRootOptions,
) -> AccessibleIconRootOutput {
    AccessibleIconRootOutput {
        label: if options.decorative {
            None
        } else {
            Some(options.label)
        },
        decorative: options.decorative,
        size: options.size.max(16.0),
        enabled: options.enabled,
    }
}

pub fn primitive_accessibility_node_output(
    options: PrimitiveAccessibilityNodeOptions,
) -> PrimitiveAccessibilityNodeOutput {
    PrimitiveAccessibilityNodeOutput {
        id: options.id,
        role: options.role,
        role_name: options.role.as_str(),
        name: options.name,
        description: options.description,
        value: options.value,
        live: options.live,
        live_name: options.live.as_str(),
        states: options.states,
    }
}

pub fn primitive_accessibility_tree_output(
    nodes: impl IntoIterator<Item = PrimitiveAccessibilityNodeOutput>,
) -> PrimitiveAccessibilityTreeOutput {
    PrimitiveAccessibilityTreeOutput {
        nodes: nodes.into_iter().collect(),
    }
}

pub fn primitive_accessibility_tree_json_snapshot(
    tree: &PrimitiveAccessibilityTreeOutput,
) -> String {
    let nodes = tree
        .nodes
        .iter()
        .map(primitive_accessibility_node_json)
        .collect::<Vec<_>>()
        .join(",");
    format!("{{\"nodes\":[{nodes}]}}")
}

fn primitive_accessibility_node_json(node: &PrimitiveAccessibilityNodeOutput) -> String {
    let states = node
        .states
        .iter()
        .map(|state| {
            format!(
                "{{\"key\":\"{}\",\"value\":\"{}\"}}",
                json_escape(state.key),
                json_escape(&state.value)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"id\":\"{}\",\"role\":\"{}\",\"name\":{},\"description\":{},\"value\":{},\"live\":\"{}\",\"states\":[{}]}}",
        json_escape(&node.id),
        json_escape(node.role_name),
        json_option(node.name.as_deref()),
        json_option(node.description.as_deref()),
        json_option(node.value.as_deref()),
        json_escape(node.live_name),
        states
    )
}

fn json_option(value: Option<&str>) -> String {
    match value {
        Some(value) => format!("\"{}\"", json_escape(value)),
        None => "null".to_owned(),
    }
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if ch.is_control() => escaped.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => escaped.push(ch),
        }
    }
    escaped
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveDirection {
    Ltr,
    Rtl,
}

impl PrimitiveDirection {
    pub fn is_rtl(self) -> bool {
        matches!(self, Self::Rtl)
    }
}

pub struct PrimitiveDirectionProviderOutput<T> {
    pub inner: T,
    pub direction: PrimitiveDirection,
}

pub fn accessible_icon_label(label: &str) -> egui::WidgetText {
    egui::WidgetText::from(label.to_owned())
}

pub fn primitive_accessible_icon(
    ui: &mut egui::Ui,
    _id_source: impl std::hash::Hash,
    visual: &str,
    label: &str,
    options: AccessibleIconOptions,
) -> Response {
    let root = primitive_accessible_icon_root_output(
        AccessibleIconRootOptions::new(label)
            .size(options.size)
            .disabled(!options.enabled)
            .theme(options.theme),
    );
    let sense = if root.enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(root.size), sense);
    let response = if let Some(label) = root.label.as_deref() {
        response.on_hover_text(accessible_icon_label(label))
    } else {
        response
    };
    let fill = if response.hovered() && root.enabled {
        options.theme.item_hover_fill
    } else {
        options.theme.content_fill
    };
    let text_color = if root.enabled {
        options.theme.text
    } else {
        options.theme.disabled_text
    };

    ui.painter().rect(
        rect,
        options.theme.radius,
        fill,
        options.theme.content_stroke,
        egui::StrokeKind::Inside,
    );
    let visual = radix_icon_visual(visual);
    if let Some(icon) = visual.icon {
        paint_radix_icon(ui, icon, rect.shrink(root.size * 0.25), text_color);
    } else if let Some(fallback_text) = visual.fallback_text {
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            fallback_text,
            crate::scaled_proportional_font(ui, root.size * 0.5),
            text_color,
        );
    }
    response
}

pub fn primitive_direction_provider<T>(
    ui: &mut egui::Ui,
    direction: PrimitiveDirection,
    add_contents: impl FnOnce(&mut egui::Ui) -> T,
) -> PrimitiveDirectionProviderOutput<T> {
    let layout = if direction.is_rtl() {
        egui::Layout::right_to_left(egui::Align::Center)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
    let inner = ui.with_layout(layout, add_contents).inner;
    PrimitiveDirectionProviderOutput { inner, direction }
}

pub fn primitive_slot(
    ui: &mut egui::Ui,
    parent: egui::Id,
    part: &'static str,
    rect: Rect,
    sense: Sense,
) -> Response {
    ui.interact(rect, slot_id(parent, part), sense)
}

pub fn slot_id(parent: egui::Id, part: &'static str) -> egui::Id {
    parent.with(part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_direction_reports_rtl() {
        assert!(!PrimitiveDirection::Ltr.is_rtl());
        assert!(PrimitiveDirection::Rtl.is_rtl());
    }

    #[test]
    fn accessible_icon_options_preserve_disabled_state() {
        let options = AccessibleIconOptions::default().size(28.0).disabled(true);

        assert_eq!(options.size, 28.0);
        assert!(!options.enabled);
    }

    #[test]
    fn accessible_icon_root_output_preserves_label_contract() {
        let output = primitive_accessible_icon_root_output(
            AccessibleIconRootOptions::new("도움말").size(30.0),
        );

        assert_eq!(output.label.as_deref(), Some("도움말"));
        assert!(!output.decorative);
        assert_eq!(output.size, 30.0);
        assert!(output.enabled);
    }

    #[test]
    fn accessible_icon_root_output_allows_decorative_icon() {
        let output = primitive_accessible_icon_root_output(
            AccessibleIconRootOptions::new("ignored")
                .decorative(true)
                .disabled(true),
        );

        assert_eq!(output.label, None);
        assert!(output.decorative);
        assert!(!output.enabled);
    }

    #[test]
    fn accessibility_node_output_preserves_agent_readable_contract() {
        let node = primitive_accessibility_node_output(
            PrimitiveAccessibilityNodeOptions::new("volume", PrimitiveAccessibilityRole::Slider)
                .name("Volume")
                .description("Audio output level")
                .value("42")
                .live(PrimitiveAccessibilityLive::Polite)
                .state("disabled", "false")
                .state("orientation", "horizontal"),
        );

        assert_eq!(node.id, "volume");
        assert_eq!(node.role, PrimitiveAccessibilityRole::Slider);
        assert_eq!(node.role_name, "slider");
        assert_eq!(node.name.as_deref(), Some("Volume"));
        assert_eq!(node.description.as_deref(), Some("Audio output level"));
        assert_eq!(node.value.as_deref(), Some("42"));
        assert_eq!(node.live, PrimitiveAccessibilityLive::Polite);
        assert_eq!(node.live_name, "polite");
        assert_eq!(
            node.states,
            vec![
                PrimitiveAccessibilityState::new("disabled", "false"),
                PrimitiveAccessibilityState::new("orientation", "horizontal")
            ]
        );
    }

    #[test]
    fn accessibility_tree_output_collects_snapshot_nodes() {
        let dialog = primitive_accessibility_node_output(
            PrimitiveAccessibilityNodeOptions::new("confirm", PrimitiveAccessibilityRole::Dialog)
                .name("Confirm order")
                .description("Review order details"),
        );
        let status = primitive_accessibility_node_output(
            PrimitiveAccessibilityNodeOptions::new("toast", PrimitiveAccessibilityRole::Status)
                .name("Saved")
                .live(PrimitiveAccessibilityLive::Assertive),
        );
        let tree = primitive_accessibility_tree_output([dialog, status]);

        assert_eq!(tree.nodes.len(), 2);
        assert_eq!(tree.nodes[0].role_name, "dialog");
        assert_eq!(tree.nodes[0].name.as_deref(), Some("Confirm order"));
        assert_eq!(tree.nodes[1].role_name, "status");
        assert_eq!(tree.nodes[1].live_name, "assertive");
    }

    #[test]
    fn accessibility_tree_json_snapshot_is_stable_and_agent_readable() {
        let tree = primitive_accessibility_tree_output([
            primitive_accessibility_node_output(
                PrimitiveAccessibilityNodeOptions::new(
                    "confirm-dialog",
                    PrimitiveAccessibilityRole::Dialog,
                )
                .name("Confirm \"Order\"")
                .description("Review\nOrder")
                .state("open", "true"),
            ),
            primitive_accessibility_node_output(
                PrimitiveAccessibilityNodeOptions::new(
                    "price-input",
                    PrimitiveAccessibilityRole::Textbox,
                )
                .value("103.25")
                .state("invalid", "false"),
            ),
        ]);

        assert_eq!(
            primitive_accessibility_tree_json_snapshot(&tree),
            "{\"nodes\":[{\"id\":\"confirm-dialog\",\"role\":\"dialog\",\"name\":\"Confirm \\\"Order\\\"\",\"description\":\"Review\\nOrder\",\"value\":null,\"live\":\"off\",\"states\":[{\"key\":\"open\",\"value\":\"true\"}]},{\"id\":\"price-input\",\"role\":\"textbox\",\"name\":null,\"description\":null,\"value\":\"103.25\",\"live\":\"off\",\"states\":[{\"key\":\"invalid\",\"value\":\"false\"}]}]}"
        );
    }

    #[test]
    fn direction_provider_output_preserves_direction() {
        let output = PrimitiveDirectionProviderOutput {
            inner: 7,
            direction: PrimitiveDirection::Rtl,
        };

        assert_eq!(output.inner, 7);
        assert_eq!(output.direction, PrimitiveDirection::Rtl);
    }

    #[test]
    fn slot_id_is_stable_for_parent_and_part() {
        let parent = egui::Id::new("parent");

        assert_eq!(slot_id(parent, "trigger"), slot_id(parent, "trigger"));
        assert_ne!(slot_id(parent, "trigger"), slot_id(parent, "content"));
    }
}
