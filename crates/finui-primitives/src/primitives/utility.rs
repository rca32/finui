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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveControllableScope {
    DialogOpen,
    AlertDialogOpen,
    PopoverOpen,
    HoverCardOpen,
    TooltipOpen,
    DropdownMenuOpen,
    SelectOpen,
    SelectValue,
    MenuValue,
    MenubarValue,
    NavigationMenuValue,
    RadioGroupValue,
    SliderValue,
    OtpValue,
    PasswordValue,
    ToggleGroupValue,
}

impl PrimitiveControllableScope {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DialogOpen => "dialog.open",
            Self::AlertDialogOpen => "alert_dialog.open",
            Self::PopoverOpen => "popover.open",
            Self::HoverCardOpen => "hover_card.open",
            Self::TooltipOpen => "tooltip.open",
            Self::DropdownMenuOpen => "dropdown_menu.open",
            Self::SelectOpen => "select.open",
            Self::SelectValue => "select.value",
            Self::MenuValue => "menu.value",
            Self::MenubarValue => "menubar.value",
            Self::NavigationMenuValue => "navigation_menu.value",
            Self::RadioGroupValue => "radio_group.value",
            Self::SliderValue => "slider.value",
            Self::OtpValue => "otp.value",
            Self::PasswordValue => "password.value",
            Self::ToggleGroupValue => "toggle_group.value",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveControllableMode {
    Controlled,
    Uncontrolled,
}

impl PrimitiveControllableMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Controlled => "controlled",
            Self::Uncontrolled => "uncontrolled",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveApiStability {
    Stable,
    Experimental,
}

impl PrimitiveApiStability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Experimental => "experimental",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveControllableStateOutput<T> {
    pub scope: PrimitiveControllableScope,
    pub scope_name: &'static str,
    pub mode: PrimitiveControllableMode,
    pub mode_name: &'static str,
    pub value: T,
    pub default_value: Option<T>,
    pub next_value: T,
    pub should_emit_change: bool,
    pub should_update_internal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveDataAttributePair {
    pub name: &'static str,
    pub value: String,
}

impl PrimitiveDataAttributePair {
    pub fn new(name: &'static str, value: impl Into<String>) -> Self {
        Self {
            name,
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveDataAttributesOptions {
    pub component: &'static str,
    pub data_state: Option<&'static str>,
    pub data_side: Option<&'static str>,
    pub data_align: Option<&'static str>,
    pub data_disabled: Option<bool>,
    pub data_orientation: Option<&'static str>,
}

impl PrimitiveDataAttributesOptions {
    pub fn new(component: &'static str) -> Self {
        Self {
            component,
            data_state: None,
            data_side: None,
            data_align: None,
            data_disabled: None,
            data_orientation: None,
        }
    }

    pub fn state(mut self, data_state: &'static str) -> Self {
        self.data_state = Some(data_state);
        self
    }

    pub fn side(mut self, data_side: &'static str) -> Self {
        self.data_side = Some(data_side);
        self
    }

    pub fn align(mut self, data_align: &'static str) -> Self {
        self.data_align = Some(data_align);
        self
    }

    pub fn disabled(mut self, data_disabled: bool) -> Self {
        self.data_disabled = Some(data_disabled);
        self
    }

    pub fn orientation(mut self, data_orientation: &'static str) -> Self {
        self.data_orientation = Some(data_orientation);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveDataAttributesOutput {
    pub component: &'static str,
    pub data_state: Option<&'static str>,
    pub data_side: Option<&'static str>,
    pub data_align: Option<&'static str>,
    pub data_disabled: Option<bool>,
    pub data_orientation: Option<&'static str>,
    pub attributes: Vec<PrimitiveDataAttributePair>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTextOverflowMode {
    Clip,
    Ellipsis,
}

impl PrimitiveTextOverflowMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clip => "clip",
            Self::Ellipsis => "ellipsis",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTextLabelKind {
    Plain,
    DenseFinancial,
    Korean,
    MixedNumeric,
}

impl PrimitiveTextLabelKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::DenseFinancial => "dense_financial",
            Self::Korean => "korean",
            Self::MixedNumeric => "mixed_numeric",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveTextOverflowOptions {
    pub text: String,
    pub max_chars: Option<usize>,
    pub mode: PrimitiveTextOverflowMode,
    pub label_kind: PrimitiveTextLabelKind,
    pub tooltip_when_clipped: bool,
}

impl PrimitiveTextOverflowOptions {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            max_chars: None,
            mode: PrimitiveTextOverflowMode::Ellipsis,
            label_kind: PrimitiveTextLabelKind::Plain,
            tooltip_when_clipped: true,
        }
    }

    pub fn max_chars(mut self, max_chars: usize) -> Self {
        self.max_chars = Some(max_chars);
        self
    }

    pub fn mode(mut self, mode: PrimitiveTextOverflowMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn label_kind(mut self, label_kind: PrimitiveTextLabelKind) -> Self {
        self.label_kind = label_kind;
        self
    }

    pub fn tooltip_when_clipped(mut self, tooltip_when_clipped: bool) -> Self {
        self.tooltip_when_clipped = tooltip_when_clipped;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveTextOverflowOutput {
    pub text: String,
    pub display_text: String,
    pub clipped: bool,
    pub mode: PrimitiveTextOverflowMode,
    pub mode_name: &'static str,
    pub label_kind: PrimitiveTextLabelKind,
    pub label_kind_name: &'static str,
    pub tooltip_text: Option<String>,
    pub accessibility_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveUxStatePair {
    pub key: &'static str,
    pub value: String,
}

impl PrimitiveUxStatePair {
    pub fn new(key: &'static str, value: impl Into<String>) -> Self {
        Self {
            key,
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveUxReceiptOptions {
    pub primitive: String,
    pub part: String,
    pub states: Vec<PrimitiveUxStatePair>,
    pub focused_id: Option<String>,
    pub selected_item: Option<String>,
    pub selected_value: Option<String>,
    pub open_layers: Vec<String>,
}

impl PrimitiveUxReceiptOptions {
    pub fn new(primitive: impl Into<String>, part: impl Into<String>) -> Self {
        Self {
            primitive: primitive.into(),
            part: part.into(),
            states: Vec::new(),
            focused_id: None,
            selected_item: None,
            selected_value: None,
            open_layers: Vec::new(),
        }
    }

    pub fn state(mut self, key: &'static str, value: impl Into<String>) -> Self {
        self.states.push(PrimitiveUxStatePair::new(key, value));
        self
    }

    pub fn focused_id(mut self, focused_id: impl Into<String>) -> Self {
        self.focused_id = Some(focused_id.into());
        self
    }

    pub fn selected_item(mut self, selected_item: impl Into<String>) -> Self {
        self.selected_item = Some(selected_item.into());
        self
    }

    pub fn selected_value(mut self, selected_value: impl Into<String>) -> Self {
        self.selected_value = Some(selected_value.into());
        self
    }

    pub fn open_layer(mut self, open_layer: impl Into<String>) -> Self {
        self.open_layers.push(open_layer.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveUxReceiptOutput {
    pub primitive: String,
    pub part: String,
    pub states: Vec<PrimitiveUxStatePair>,
    pub focused_id: Option<String>,
    pub selected_item: Option<String>,
    pub selected_value: Option<String>,
    pub open_layers: Vec<String>,
}

pub fn primitive_controllable_state_output<T: Clone + PartialEq>(
    scope: PrimitiveControllableScope,
    controlled_value: Option<T>,
    default_value: Option<T>,
    internal_value: T,
    next_value: T,
) -> PrimitiveControllableStateOutput<T> {
    let mode = if controlled_value.is_some() {
        PrimitiveControllableMode::Controlled
    } else {
        PrimitiveControllableMode::Uncontrolled
    };
    let value = controlled_value.unwrap_or(internal_value);
    let should_emit_change = value != next_value;
    let should_update_internal =
        mode == PrimitiveControllableMode::Uncontrolled && should_emit_change;

    PrimitiveControllableStateOutput {
        scope,
        scope_name: scope.as_str(),
        mode,
        mode_name: mode.as_str(),
        value,
        default_value,
        next_value,
        should_emit_change,
        should_update_internal,
    }
}

pub fn primitive_data_attributes_output(
    options: PrimitiveDataAttributesOptions,
) -> PrimitiveDataAttributesOutput {
    let mut attributes = Vec::new();
    if let Some(value) = options.data_state {
        attributes.push(PrimitiveDataAttributePair::new("data-state", value));
    }
    if let Some(value) = options.data_side {
        attributes.push(PrimitiveDataAttributePair::new("data-side", value));
    }
    if let Some(value) = options.data_align {
        attributes.push(PrimitiveDataAttributePair::new("data-align", value));
    }
    if let Some(value) = options.data_disabled {
        attributes.push(PrimitiveDataAttributePair::new(
            "data-disabled",
            value.to_string(),
        ));
    }
    if let Some(value) = options.data_orientation {
        attributes.push(PrimitiveDataAttributePair::new("data-orientation", value));
    }

    PrimitiveDataAttributesOutput {
        component: options.component,
        data_state: options.data_state,
        data_side: options.data_side,
        data_align: options.data_align,
        data_disabled: options.data_disabled,
        data_orientation: options.data_orientation,
        attributes,
    }
}

pub fn primitive_ux_receipt_output(options: PrimitiveUxReceiptOptions) -> PrimitiveUxReceiptOutput {
    PrimitiveUxReceiptOutput {
        primitive: options.primitive,
        part: options.part,
        states: options.states,
        focused_id: options.focused_id,
        selected_item: options.selected_item,
        selected_value: options.selected_value,
        open_layers: options.open_layers,
    }
}

pub fn primitive_ux_receipt_json_snapshot(receipt: &PrimitiveUxReceiptOutput) -> String {
    let states = receipt
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
    let open_layers = receipt
        .open_layers
        .iter()
        .map(|layer| format!("\"{}\"", json_escape(layer)))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"primitive\":\"{}\",\"part\":\"{}\",\"states\":[{}],\"focused_id\":{},\"selected_item\":{},\"selected_value\":{},\"open_layers\":[{}]}}",
        json_escape(&receipt.primitive),
        json_escape(&receipt.part),
        states,
        json_option(receipt.focused_id.as_deref()),
        json_option(receipt.selected_item.as_deref()),
        json_option(receipt.selected_value.as_deref()),
        open_layers
    )
}

pub fn primitive_text_overflow_output(
    options: PrimitiveTextOverflowOptions,
) -> PrimitiveTextOverflowOutput {
    let text_len = options.text.chars().count();
    let clipped = options
        .max_chars
        .is_some_and(|max_chars| text_len > max_chars);
    let display_text = if let Some(max_chars) = options.max_chars {
        primitive_truncated_text(&options.text, max_chars, options.mode)
    } else {
        options.text.clone()
    };
    let tooltip_text = (clipped && options.tooltip_when_clipped).then(|| options.text.clone());

    PrimitiveTextOverflowOutput {
        accessibility_text: options.text.clone(),
        text: options.text,
        display_text,
        clipped,
        mode: options.mode,
        mode_name: options.mode.as_str(),
        label_kind: options.label_kind,
        label_kind_name: options.label_kind.as_str(),
        tooltip_text,
    }
}

fn primitive_truncated_text(
    text: &str,
    max_chars: usize,
    mode: PrimitiveTextOverflowMode,
) -> String {
    let current_len = text.chars().count();
    if current_len <= max_chars {
        return text.to_owned();
    }
    match mode {
        PrimitiveTextOverflowMode::Clip => text.chars().take(max_chars).collect(),
        PrimitiveTextOverflowMode::Ellipsis => {
            if max_chars <= 3 {
                ".".repeat(max_chars)
            } else {
                let prefix = text.chars().take(max_chars - 3).collect::<String>();
                format!("{prefix}...")
            }
        }
    }
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

pub fn primitive_horizontal_arrow_step(
    direction: Option<PrimitiveDirection>,
    arrow_left_pressed: bool,
    arrow_right_pressed: bool,
) -> Option<isize> {
    let rtl = direction == Some(PrimitiveDirection::Rtl);
    if arrow_right_pressed {
        Some(if rtl { -1 } else { 1 })
    } else if arrow_left_pressed {
        Some(if rtl { 1 } else { -1 })
    } else {
        None
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
    fn controllable_open_output_keeps_dialog_popover_and_menu_controlled_by_owner() {
        for scope in [
            PrimitiveControllableScope::DialogOpen,
            PrimitiveControllableScope::PopoverOpen,
            PrimitiveControllableScope::DropdownMenuOpen,
        ] {
            let output =
                primitive_controllable_state_output(scope, Some(false), Some(true), true, true);

            assert_eq!(output.scope, scope);
            assert_eq!(output.scope_name, scope.as_str());
            assert_eq!(output.mode, PrimitiveControllableMode::Controlled);
            assert_eq!(output.mode_name, "controlled");
            assert!(!output.value);
            assert_eq!(output.default_value, Some(true));
            assert_eq!(output.next_value, true);
            assert!(output.should_emit_change);
            assert!(!output.should_update_internal);
        }
    }

    #[test]
    fn controllable_open_output_updates_uncontrolled_local_state() {
        for scope in [
            PrimitiveControllableScope::AlertDialogOpen,
            PrimitiveControllableScope::HoverCardOpen,
            PrimitiveControllableScope::TooltipOpen,
            PrimitiveControllableScope::SelectOpen,
        ] {
            let output = primitive_controllable_state_output(scope, None, Some(false), false, true);

            assert_eq!(output.mode, PrimitiveControllableMode::Uncontrolled);
            assert_eq!(output.mode_name, "uncontrolled");
            assert!(!output.value);
            assert_eq!(output.default_value, Some(false));
            assert_eq!(output.next_value, true);
            assert!(output.should_emit_change);
            assert!(output.should_update_internal);
        }
    }

    #[test]
    fn controllable_value_output_unifies_select_menu_and_form_value_ownership() {
        let controlled_select = primitive_controllable_state_output(
            PrimitiveControllableScope::SelectValue,
            Some("1m"),
            Some("3m"),
            "6m",
            "3m",
        );
        let uncontrolled_menu = primitive_controllable_state_output(
            PrimitiveControllableScope::MenuValue,
            None,
            Some("view"),
            "view",
            "edit",
        );
        let unchanged_radio = primitive_controllable_state_output(
            PrimitiveControllableScope::RadioGroupValue,
            None,
            Some("buy"),
            "buy",
            "buy",
        );

        assert_eq!(
            controlled_select.mode,
            PrimitiveControllableMode::Controlled
        );
        assert_eq!(controlled_select.value, "1m");
        assert!(controlled_select.should_emit_change);
        assert!(!controlled_select.should_update_internal);

        assert_eq!(
            uncontrolled_menu.mode,
            PrimitiveControllableMode::Uncontrolled
        );
        assert_eq!(uncontrolled_menu.value, "view");
        assert!(uncontrolled_menu.should_emit_change);
        assert!(uncontrolled_menu.should_update_internal);

        assert_eq!(
            unchanged_radio.mode,
            PrimitiveControllableMode::Uncontrolled
        );
        assert!(!unchanged_radio.should_emit_change);
        assert!(!unchanged_radio.should_update_internal);
    }

    #[test]
    fn primitive_api_stability_names_match_public_contract_terms() {
        assert_eq!(PrimitiveApiStability::Stable.as_str(), "stable");
        assert_eq!(PrimitiveApiStability::Experimental.as_str(), "experimental");
    }

    #[test]
    fn ux_receipt_output_collects_state_focus_selection_and_open_layers() {
        let receipt = primitive_ux_receipt_output(
            PrimitiveUxReceiptOptions::new("select", "content")
                .state("open", "true")
                .state("highlighted", "1W")
                .focused_id("select-period-item-1w")
                .selected_item("1W")
                .selected_value("1W")
                .open_layer("select.content")
                .open_layer("select.viewport"),
        );

        assert_eq!(receipt.primitive, "select");
        assert_eq!(receipt.part, "content");
        assert_eq!(
            receipt.states,
            vec![
                PrimitiveUxStatePair::new("open", "true"),
                PrimitiveUxStatePair::new("highlighted", "1W")
            ]
        );
        assert_eq!(receipt.focused_id.as_deref(), Some("select-period-item-1w"));
        assert_eq!(receipt.selected_item.as_deref(), Some("1W"));
        assert_eq!(receipt.selected_value.as_deref(), Some("1W"));
        assert_eq!(
            receipt.open_layers,
            vec!["select.content".to_owned(), "select.viewport".to_owned()]
        );
    }

    #[test]
    fn ux_receipt_json_snapshot_is_stable_and_agent_readable() {
        let receipt = primitive_ux_receipt_output(
            PrimitiveUxReceiptOptions::new("dialog", "content")
                .state("open", "true")
                .focused_id("confirm-title")
                .open_layer("dialog.portal")
                .open_layer("dialog.content"),
        );

        assert_eq!(
            primitive_ux_receipt_json_snapshot(&receipt),
            "{\"primitive\":\"dialog\",\"part\":\"content\",\"states\":[{\"key\":\"open\",\"value\":\"true\"}],\"focused_id\":\"confirm-title\",\"selected_item\":null,\"selected_value\":null,\"open_layers\":[\"dialog.portal\",\"dialog.content\"]}"
        );
    }

    #[test]
    fn data_attributes_output_keeps_canonical_radix_names_for_layered_primitives() {
        let dialog = primitive_data_attributes_output(
            PrimitiveDataAttributesOptions::new("dialog.content")
                .state("open")
                .disabled(false),
        );
        let dropdown = primitive_data_attributes_output(
            PrimitiveDataAttributesOptions::new("dropdown_menu.content")
                .state("closed")
                .side("bottom")
                .align("start"),
        );
        let select = primitive_data_attributes_output(
            PrimitiveDataAttributesOptions::new("select.content")
                .state("open")
                .side("top")
                .align("center")
                .disabled(true),
        );

        assert_eq!(dialog.component, "dialog.content");
        assert_eq!(dialog.data_state, Some("open"));
        assert_eq!(
            dialog.attributes,
            vec![
                PrimitiveDataAttributePair::new("data-state", "open"),
                PrimitiveDataAttributePair::new("data-disabled", "false")
            ]
        );
        assert_eq!(dropdown.data_side, Some("bottom"));
        assert_eq!(dropdown.data_align, Some("start"));
        assert_eq!(
            dropdown.attributes,
            vec![
                PrimitiveDataAttributePair::new("data-state", "closed"),
                PrimitiveDataAttributePair::new("data-side", "bottom"),
                PrimitiveDataAttributePair::new("data-align", "start")
            ]
        );
        assert_eq!(select.data_disabled, Some(true));
        assert_eq!(
            select.attributes,
            vec![
                PrimitiveDataAttributePair::new("data-state", "open"),
                PrimitiveDataAttributePair::new("data-side", "top"),
                PrimitiveDataAttributePair::new("data-align", "center"),
                PrimitiveDataAttributePair::new("data-disabled", "true")
            ]
        );
    }

    #[test]
    fn data_attributes_output_covers_orientation_and_disabled_controls() {
        let toolbar = primitive_data_attributes_output(
            PrimitiveDataAttributesOptions::new("toolbar.root")
                .orientation("vertical")
                .disabled(false),
        );
        let toggle = primitive_data_attributes_output(
            PrimitiveDataAttributesOptions::new("toggle_group.item")
                .state("on")
                .orientation("horizontal")
                .disabled(true),
        );

        assert_eq!(toolbar.data_orientation, Some("vertical"));
        assert_eq!(
            toolbar.attributes,
            vec![
                PrimitiveDataAttributePair::new("data-disabled", "false"),
                PrimitiveDataAttributePair::new("data-orientation", "vertical")
            ]
        );
        assert_eq!(toggle.data_state, Some("on"));
        assert_eq!(toggle.data_orientation, Some("horizontal"));
        assert_eq!(toggle.data_disabled, Some(true));
        assert_eq!(
            toggle.attributes,
            vec![
                PrimitiveDataAttributePair::new("data-state", "on"),
                PrimitiveDataAttributePair::new("data-disabled", "true"),
                PrimitiveDataAttributePair::new("data-orientation", "horizontal")
            ]
        );
    }

    #[test]
    fn data_attributes_output_omits_non_applicable_attributes_without_renaming() {
        let label =
            primitive_data_attributes_output(PrimitiveDataAttributesOptions::new("label.root"));

        assert_eq!(label.component, "label.root");
        assert_eq!(label.data_state, None);
        assert_eq!(label.data_side, None);
        assert_eq!(label.data_align, None);
        assert_eq!(label.data_disabled, None);
        assert_eq!(label.data_orientation, None);
        assert!(label.attributes.is_empty());
    }

    #[test]
    fn text_overflow_output_ellipsizes_long_dense_financial_labels() {
        let output = primitive_text_overflow_output(
            PrimitiveTextOverflowOptions::new("KR103502GG38 3Y Treasury 2026-06-30")
                .max_chars(18)
                .label_kind(PrimitiveTextLabelKind::DenseFinancial),
        );

        assert_eq!(output.display_text, "KR103502GG38 3Y...");
        assert!(output.clipped);
        assert_eq!(output.mode_name, "ellipsis");
        assert_eq!(output.label_kind_name, "dense_financial");
        assert_eq!(
            output.tooltip_text.as_deref(),
            Some("KR103502GG38 3Y Treasury 2026-06-30")
        );
        assert_eq!(output.accessibility_text, output.text);
    }

    #[test]
    fn text_overflow_output_clips_korean_labels_on_char_boundaries() {
        let output = primitive_text_overflow_output(
            PrimitiveTextOverflowOptions::new("한국국채 수익률")
                .max_chars(4)
                .mode(PrimitiveTextOverflowMode::Clip)
                .label_kind(PrimitiveTextLabelKind::Korean),
        );

        assert_eq!(output.display_text, "한국국채");
        assert!(output.clipped);
        assert_eq!(output.mode_name, "clip");
        assert_eq!(output.label_kind_name, "korean");
        assert_eq!(output.accessibility_text, "한국국채 수익률");
    }

    #[test]
    fn text_overflow_output_preserves_mixed_numeric_label_when_it_fits() {
        let output = primitive_text_overflow_output(
            PrimitiveTextOverflowOptions::new("KTB 3Y 3.245%")
                .max_chars(16)
                .label_kind(PrimitiveTextLabelKind::MixedNumeric)
                .tooltip_when_clipped(false),
        );

        assert_eq!(output.display_text, "KTB 3Y 3.245%");
        assert!(!output.clipped);
        assert_eq!(output.tooltip_text, None);
        assert_eq!(output.label_kind_name, "mixed_numeric");
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
    fn horizontal_arrow_step_reverses_for_rtl() {
        assert_eq!(primitive_horizontal_arrow_step(None, false, true), Some(1));
        assert_eq!(
            primitive_horizontal_arrow_step(Some(PrimitiveDirection::Ltr), true, false),
            Some(-1)
        );
        assert_eq!(
            primitive_horizontal_arrow_step(Some(PrimitiveDirection::Rtl), false, true),
            Some(-1)
        );
        assert_eq!(
            primitive_horizontal_arrow_step(Some(PrimitiveDirection::Rtl), true, false),
            Some(1)
        );
        assert_eq!(
            primitive_horizontal_arrow_step(Some(PrimitiveDirection::Rtl), false, false),
            None
        );
    }

    #[test]
    fn slot_id_is_stable_for_parent_and_part() {
        let parent = egui::Id::new("parent");

        assert_eq!(slot_id(parent, "trigger"), slot_id(parent, "trigger"));
        assert_ne!(slot_id(parent, "trigger"), slot_id(parent, "content"));
    }
}
