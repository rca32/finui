use std::hash::Hash;

use eframe::egui::{self, Color32, FontId, Pos2, Rect, Response, RichText, Stroke, Vec2};

use super::{
    PrimitiveDirection, PrimitiveTheme, RadixIcon, paint_radix_icon,
    primitive_horizontal_arrow_step, radix_colors,
};

pub struct PrimitiveControlOutput {
    pub response: Response,
    pub changed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveKeyboardActivation {
    None,
    Activate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioGroupKeyboardAction {
    None,
    Activate,
    First,
    Last,
    Next,
    Previous,
}

pub struct PrimitiveFormControlOutput<T> {
    pub inner: T,
    pub response: Response,
    pub name: Option<&'static str>,
    pub invalid: bool,
    pub valid: bool,
    pub enabled: bool,
    pub data_invalid: bool,
    pub data_valid: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveFormMessageKind {
    Description,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveFormMessageMatch {
    ValueMissing,
    TypeMismatch,
    PatternMismatch,
    TooShort,
    TooLong,
    RangeUnderflow,
    RangeOverflow,
    StepMismatch,
    BadInput,
    CustomError,
    ServerInvalid,
}

impl PrimitiveFormMessageMatch {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ValueMissing => "valueMissing",
            Self::TypeMismatch => "typeMismatch",
            Self::PatternMismatch => "patternMismatch",
            Self::TooShort => "tooShort",
            Self::TooLong => "tooLong",
            Self::RangeUnderflow => "rangeUnderflow",
            Self::RangeOverflow => "rangeOverflow",
            Self::StepMismatch => "stepMismatch",
            Self::BadInput => "badInput",
            Self::CustomError => "customError",
            Self::ServerInvalid => "serverInvalid",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveFormAssociationOptions {
    pub name: String,
    pub invalid: bool,
    pub server_invalid: bool,
    pub has_description: bool,
    pub error_match: Option<PrimitiveFormMessageMatch>,
}

impl PrimitiveFormAssociationOptions {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            invalid: false,
            server_invalid: false,
            has_description: false,
            error_match: None,
        }
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn server_invalid(mut self, server_invalid: bool) -> Self {
        self.server_invalid = server_invalid;
        self
    }

    pub fn description(mut self, has_description: bool) -> Self {
        self.has_description = has_description;
        self
    }

    pub fn error_match(mut self, match_kind: PrimitiveFormMessageMatch) -> Self {
        self.error_match = Some(match_kind);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveFormAssociationOutput {
    pub name: String,
    pub field_id: String,
    pub control_id: String,
    pub label_id: String,
    pub label_for: String,
    pub description_id: Option<String>,
    pub error_id: Option<String>,
    pub described_by: Vec<String>,
    pub error_match: Option<PrimitiveFormMessageMatch>,
    pub invalid: bool,
    pub valid: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveFormFieldOptions {
    pub width: f32,
    pub spacing: f32,
    pub theme: PrimitiveTheme,
}

impl PrimitiveFormFieldOptions {
    pub fn new(width: f32) -> Self {
        Self {
            width,
            spacing: 6.0,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveFormFieldPartOptions {
    pub name: String,
    pub server_invalid: bool,
    pub invalid: bool,
}

impl PrimitiveFormFieldPartOptions {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            server_invalid: false,
            invalid: false,
        }
    }

    pub fn server_invalid(mut self, server_invalid: bool) -> Self {
        self.server_invalid = server_invalid;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveFormFieldOutput {
    pub name: String,
    pub server_invalid: bool,
    pub invalid: bool,
    pub valid: bool,
    pub data_invalid: bool,
    pub data_valid: bool,
}

pub fn primitive_form_field_output(
    options: PrimitiveFormFieldPartOptions,
) -> PrimitiveFormFieldOutput {
    let invalid = options.invalid || options.server_invalid;
    PrimitiveFormFieldOutput {
        name: options.name,
        server_invalid: options.server_invalid,
        invalid,
        valid: !invalid,
        data_invalid: invalid,
        data_valid: !invalid,
    }
}

pub fn primitive_form_association_output(
    options: PrimitiveFormAssociationOptions,
) -> PrimitiveFormAssociationOutput {
    let invalid = options.invalid || options.server_invalid;
    let id_part = primitive_form_id_part(&options.name);
    let field_id = format!("form-field-{id_part}");
    let control_id = format!("{field_id}-control");
    let label_id = format!("{field_id}-label");
    let description_id = options
        .has_description
        .then(|| format!("{field_id}-description"));
    let error_id = invalid.then(|| format!("{field_id}-error"));
    let mut described_by = Vec::new();
    if let Some(description_id) = &description_id {
        described_by.push(description_id.clone());
    }
    if let Some(error_id) = &error_id {
        described_by.push(error_id.clone());
    }

    PrimitiveFormAssociationOutput {
        name: options.name,
        field_id,
        control_id: control_id.clone(),
        label_id,
        label_for: control_id,
        description_id,
        error_id,
        described_by,
        error_match: invalid.then_some(
            options
                .error_match
                .unwrap_or(PrimitiveFormMessageMatch::CustomError),
        ),
        invalid,
        valid: !invalid,
    }
}

fn primitive_form_id_part(name: &str) -> String {
    let mut output = String::new();
    let mut last_was_separator = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator && !output.is_empty() {
            output.push('-');
            last_was_separator = true;
        }
    }
    while output.ends_with('-') {
        output.pop();
    }
    if output.is_empty() {
        "field".to_owned()
    } else {
        output
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveFormControlOptions {
    pub width: Option<f32>,
    pub enabled: bool,
    pub invalid: bool,
    pub name: Option<&'static str>,
    pub theme: PrimitiveTheme,
}

impl Default for PrimitiveFormControlOptions {
    fn default() -> Self {
        Self {
            width: None,
            enabled: true,
            invalid: false,
            name: None,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl PrimitiveFormControlOptions {
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveFormLabelOptions {
    pub name: String,
    pub invalid: bool,
}

impl PrimitiveFormLabelOptions {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            invalid: false,
        }
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveFormLabelOutput {
    pub name: String,
    pub invalid: bool,
    pub valid: bool,
    pub data_invalid: bool,
    pub data_valid: bool,
}

pub fn primitive_form_label_output(options: PrimitiveFormLabelOptions) -> PrimitiveFormLabelOutput {
    PrimitiveFormLabelOutput {
        name: options.name,
        invalid: options.invalid,
        valid: !options.invalid,
        data_invalid: options.invalid,
        data_valid: !options.invalid,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveFormMessageOptions {
    pub name: Option<String>,
    pub match_kind: Option<PrimitiveFormMessageMatch>,
    pub force_match: bool,
    pub field_invalid: bool,
}

impl Default for PrimitiveFormMessageOptions {
    fn default() -> Self {
        Self {
            name: None,
            match_kind: None,
            force_match: false,
            field_invalid: false,
        }
    }
}

impl PrimitiveFormMessageOptions {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn match_kind(mut self, match_kind: PrimitiveFormMessageMatch) -> Self {
        self.match_kind = Some(match_kind);
        self
    }

    pub fn force_match(mut self, force_match: bool) -> Self {
        self.force_match = force_match;
        self
    }

    pub fn field_invalid(mut self, field_invalid: bool) -> Self {
        self.field_invalid = field_invalid;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveFormMessageOutput {
    pub name: Option<String>,
    pub match_kind: Option<PrimitiveFormMessageMatch>,
    pub force_match: bool,
    pub visible: bool,
}

pub fn primitive_form_message_output(
    options: PrimitiveFormMessageOptions,
) -> PrimitiveFormMessageOutput {
    PrimitiveFormMessageOutput {
        name: options.name,
        match_kind: options.match_kind,
        force_match: options.force_match,
        visible: options.force_match || options.field_invalid,
    }
}

pub fn primitive_form_field<T>(
    ui: &mut egui::Ui,
    label: &str,
    description: Option<&str>,
    error: Option<&str>,
    options: PrimitiveFormFieldOptions,
    add_control: impl FnOnce(&mut egui::Ui) -> T,
) -> T {
    ui.set_max_width(options.width);
    primitive_form_label(ui, label, options.theme);
    ui.add_space(options.spacing);
    let output = primitive_form_control(
        ui,
        ("form_control", label),
        PrimitiveFormControlOptions::default()
            .width(options.width)
            .invalid(error.is_some()),
        add_control,
    )
    .inner;
    if let Some(description) = description {
        primitive_form_message(
            ui,
            description,
            PrimitiveFormMessageKind::Description,
            options.theme,
        );
    }
    if let Some(error) = error {
        primitive_form_message(ui, error, PrimitiveFormMessageKind::Error, options.theme);
    }
    output
}

pub fn primitive_form_control<T>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    options: PrimitiveFormControlOptions,
    add_control: impl FnOnce(&mut egui::Ui) -> T,
) -> PrimitiveFormControlOutput<T> {
    let inner = ui.scope(|ui| {
        if !options.enabled {
            ui.disable();
        }
        if let Some(width) = options.width {
            ui.set_max_width(width);
        }
        add_control(ui)
    });
    let response = ui.interact(
        inner.response.rect,
        ui.id().with(id_source),
        egui::Sense::hover(),
    );
    if options.invalid {
        ui.painter().line_segment(
            [
                response.rect.left_bottom() + Vec2::new(0.0, 1.0),
                response.rect.right_bottom() + Vec2::new(0.0, 1.0),
            ],
            Stroke::new(1.0, Color32::from_rgb(196, 40, 40)),
        );
    }
    PrimitiveFormControlOutput {
        inner: inner.inner,
        response,
        name: options.name,
        invalid: options.invalid,
        valid: !options.invalid,
        enabled: options.enabled,
        data_invalid: options.invalid,
        data_valid: !options.invalid,
    }
}

pub fn primitive_form_label(ui: &mut egui::Ui, text: &str, theme: PrimitiveTheme) -> Response {
    ui.label(
        RichText::new(text)
            .font(crate::scaled_proportional_font(ui, 12.0))
            .color(theme.text),
    )
}

pub fn primitive_form_message(
    ui: &mut egui::Ui,
    text: &str,
    kind: PrimitiveFormMessageKind,
    theme: PrimitiveTheme,
) -> Response {
    let color = match kind {
        PrimitiveFormMessageKind::Description => theme.muted_text,
        PrimitiveFormMessageKind::Error => Color32::from_rgb(196, 40, 40),
    };
    ui.label(
        RichText::new(text)
            .font(crate::scaled_proportional_font(ui, 12.0))
            .color(color),
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveCheckboxOptions {
    pub size: f32,
    pub label_gap: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for PrimitiveCheckboxOptions {
    fn default() -> Self {
        Self {
            size: 16.0,
            label_gap: 8.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckboxState {
    Unchecked,
    Checked,
    Indeterminate,
}

impl CheckboxState {
    pub fn from_bool(checked: bool) -> Self {
        if checked {
            Self::Checked
        } else {
            Self::Unchecked
        }
    }

    pub fn is_checked(self) -> bool {
        matches!(self, Self::Checked)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unchecked => "unchecked",
            Self::Checked => "checked",
            Self::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckboxRootOptions {
    pub state: CheckboxState,
    pub default_state: Option<CheckboxState>,
    pub disabled: bool,
    pub required: bool,
    pub name: Option<&'static str>,
    pub value: &'static str,
    pub size: f32,
    pub theme: PrimitiveTheme,
}

impl Default for CheckboxRootOptions {
    fn default() -> Self {
        Self {
            state: CheckboxState::Unchecked,
            default_state: None,
            disabled: false,
            required: false,
            name: None,
            value: "on",
            size: 16.0,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl CheckboxRootOptions {
    pub fn state(mut self, state: CheckboxState) -> Self {
        self.state = state;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.state = CheckboxState::from_bool(checked);
        self
    }

    pub fn default_state(mut self, state: CheckboxState) -> Self {
        self.default_state = Some(state);
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

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn value(mut self, value: &'static str) -> Self {
        self.value = value;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckboxRootOutput {
    pub rect: Rect,
    pub state: CheckboxState,
    pub default_state: Option<CheckboxState>,
    pub disabled: bool,
    pub required: bool,
    pub name: Option<&'static str>,
    pub value: &'static str,
    pub data_state: CheckboxState,
    pub data_disabled: bool,
}

pub fn primitive_checkbox(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    checked: &mut bool,
    label: &str,
    options: PrimitiveCheckboxOptions,
) -> PrimitiveControlOutput {
    let text_width = label.chars().count() as f32 * 8.0;
    let desired = Vec2::new(options.size + options.label_gap + text_width + 4.0, 24.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let response = ui.interact(
        rect,
        ui.id().with(id_source),
        if options.enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let response = if options.enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    };
    if options.enabled && response.clicked() {
        response.request_focus();
    }
    let keyboard_activation = ui.input(|input| {
        primitive_form_keyboard_activation(
            options.enabled,
            response.has_focus(),
            input.key_pressed(egui::Key::Enter),
            input.key_pressed(egui::Key::Space),
        )
    });
    let changed = options.enabled
        && (response.clicked() || keyboard_activation == PrimitiveKeyboardActivation::Activate);
    if changed {
        *checked = !*checked;
    }

    let box_rect = primitive_checkbox_root_rect(rect, options.size);
    primitive_checkbox_root(ui, box_rect, *checked, response.hovered(), options);
    primitive_checkbox_indicator(ui, box_rect, *checked, options);
    if response.has_focus() {
        draw_control_focus_ring(ui, box_rect.expand(3.0), options.theme);
    }

    ui.painter().text(
        egui::pos2(box_rect.right() + options.label_gap, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        if options.enabled {
            options.theme.text
        } else {
            options.theme.disabled_text
        },
    );

    PrimitiveControlOutput { response, changed }
}

pub fn primitive_checkbox_root_rect(bounds: Rect, size: f32) -> Rect {
    Rect::from_center_size(
        egui::pos2(bounds.left() + size * 0.5, bounds.center().y),
        Vec2::splat(size),
    )
}

pub fn primitive_checkbox_root_output(
    bounds: Rect,
    options: CheckboxRootOptions,
) -> CheckboxRootOutput {
    CheckboxRootOutput {
        rect: primitive_checkbox_root_rect(bounds, options.size),
        state: options.state,
        default_state: options.default_state,
        disabled: options.disabled,
        required: options.required,
        name: options.name,
        value: options.value,
        data_state: options.state,
        data_disabled: options.disabled,
    }
}

pub fn primitive_checkbox_root_with_options(
    ui: &egui::Ui,
    rect: Rect,
    hovered: bool,
    options: CheckboxRootOptions,
) -> CheckboxRootOutput {
    primitive_checkbox_root(
        ui,
        rect,
        options.state.is_checked(),
        hovered,
        PrimitiveCheckboxOptions {
            size: options.size,
            enabled: !options.disabled,
            theme: options.theme,
            ..Default::default()
        },
    );
    primitive_checkbox_indicator_with_state(ui, rect, options.state, options.theme);
    CheckboxRootOutput {
        rect,
        state: options.state,
        default_state: options.default_state,
        disabled: options.disabled,
        required: options.required,
        name: options.name,
        value: options.value,
        data_state: options.state,
        data_disabled: options.disabled,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckboxIndicatorOptions {
    pub state: CheckboxState,
    pub disabled: bool,
    pub force_mount: bool,
}

impl CheckboxIndicatorOptions {
    pub fn new(state: CheckboxState) -> Self {
        Self {
            state,
            disabled: false,
            force_mount: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CheckboxIndicatorOutput {
    pub state: CheckboxState,
    pub disabled: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub data_state: CheckboxState,
    pub data_disabled: bool,
}

pub fn primitive_checkbox_indicator_output(
    options: CheckboxIndicatorOptions,
) -> CheckboxIndicatorOutput {
    CheckboxIndicatorOutput {
        state: options.state,
        disabled: options.disabled,
        force_mount: options.force_mount,
        mounted: options.state != CheckboxState::Unchecked || options.force_mount,
        data_state: options.state,
        data_disabled: options.disabled,
    }
}

pub fn primitive_checkbox_root(
    ui: &egui::Ui,
    rect: Rect,
    checked: bool,
    hovered: bool,
    options: PrimitiveCheckboxOptions,
) {
    let stroke = if !options.enabled {
        Stroke::new(1.0, options.theme.disabled_text)
    } else if checked {
        Stroke::new(1.2, control_accent_stroke(options.theme))
    } else {
        Stroke::new(1.2, control_idle_stroke(options.theme))
    };
    let fill = if !options.enabled {
        options.theme.content_fill
    } else if checked {
        control_accent_fill(options.theme)
    } else if hovered && options.enabled {
        options.theme.item_hover_fill
    } else {
        options.theme.content_fill
    };
    ui.painter()
        .rect_filled(rect, options.theme.row_radius, fill);
    draw_rect_outline(ui, rect, stroke);
}

pub fn primitive_checkbox_indicator(
    ui: &egui::Ui,
    rect: Rect,
    checked: bool,
    options: PrimitiveCheckboxOptions,
) {
    primitive_checkbox_indicator_with_state(
        ui,
        rect,
        CheckboxState::from_bool(checked),
        options.theme,
    );
}

pub fn primitive_checkbox_indicator_with_state(
    ui: &egui::Ui,
    rect: Rect,
    state: CheckboxState,
    theme: PrimitiveTheme,
) {
    match state {
        CheckboxState::Checked => {
            draw_checkbox_check_mark(ui, rect, control_mark_color(theme));
        }
        CheckboxState::Indeterminate => {
            draw_indeterminate_mark(ui, rect, control_mark_color(theme))
        }
        CheckboxState::Unchecked => {}
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveSwitchOptions {
    pub width: f32,
    pub height: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for PrimitiveSwitchOptions {
    fn default() -> Self {
        Self {
            width: 34.0,
            height: 18.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchRootOptions {
    pub checked: bool,
    pub default_checked: Option<bool>,
    pub disabled: bool,
    pub required: bool,
    pub name: Option<String>,
    pub value: String,
}

impl Default for SwitchRootOptions {
    fn default() -> Self {
        Self {
            checked: false,
            default_checked: None,
            disabled: false,
            required: false,
            name: None,
            value: "on".to_owned(),
        }
    }
}

impl SwitchRootOptions {
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = Some(default_checked);
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

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwitchDataState {
    Checked,
    Unchecked,
}

impl SwitchDataState {
    pub fn from_checked(checked: bool) -> Self {
        if checked {
            Self::Checked
        } else {
            Self::Unchecked
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checked => "checked",
            Self::Unchecked => "unchecked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwitchRootOutput {
    pub rect: Rect,
    pub thumb_center: Pos2,
    pub checked: bool,
    pub default_checked: Option<bool>,
    pub disabled: bool,
    pub required: bool,
    pub data_state: SwitchDataState,
    pub data_disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwitchThumbOutput {
    pub center: Pos2,
    pub checked: bool,
    pub disabled: bool,
    pub data_state: SwitchDataState,
    pub data_disabled: bool,
}

pub fn primitive_switch_root_output(
    rect: Rect,
    checked: bool,
    options: PrimitiveSwitchOptions,
) -> SwitchRootOutput {
    primitive_switch_root_output_with_options(
        rect,
        SwitchRootOptions::default()
            .checked(checked)
            .disabled(!options.enabled),
        options,
    )
}

pub fn primitive_switch_root_output_with_options(
    rect: Rect,
    root_options: SwitchRootOptions,
    options: PrimitiveSwitchOptions,
) -> SwitchRootOutput {
    SwitchRootOutput {
        rect,
        thumb_center: primitive_switch_thumb_center(rect, root_options.checked, options.height),
        checked: root_options.checked,
        default_checked: root_options.default_checked,
        disabled: root_options.disabled,
        required: root_options.required,
        data_state: SwitchDataState::from_checked(root_options.checked),
        data_disabled: root_options.disabled,
    }
}

pub fn primitive_switch_thumb_output(root: SwitchRootOutput) -> SwitchThumbOutput {
    SwitchThumbOutput {
        center: root.thumb_center,
        checked: root.checked,
        disabled: root.disabled,
        data_state: root.data_state,
        data_disabled: root.data_disabled,
    }
}

pub fn primitive_switch(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    checked: &mut bool,
    options: PrimitiveSwitchOptions,
) -> PrimitiveControlOutput {
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(options.width, options.height),
        egui::Sense::hover(),
    );
    let response = ui.interact(
        rect,
        ui.id().with(id_source),
        if options.enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let response = if options.enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    };
    if options.enabled && response.clicked() {
        response.request_focus();
    }
    let keyboard_activation = ui.input(|input| {
        primitive_form_keyboard_activation(
            options.enabled,
            response.has_focus(),
            input.key_pressed(egui::Key::Enter),
            input.key_pressed(egui::Key::Space),
        )
    });
    let changed = options.enabled
        && (response.clicked() || keyboard_activation == PrimitiveKeyboardActivation::Activate);
    if changed {
        *checked = !*checked;
    }
    paint_switch(
        ui,
        rect,
        *checked,
        response.hovered(),
        response.has_focus(),
        options,
    );

    PrimitiveControlOutput { response, changed }
}

pub fn primitive_switch_at(
    ui: &mut egui::Ui,
    rect: Rect,
    id_source: impl Hash,
    checked: &mut bool,
    options: PrimitiveSwitchOptions,
) -> PrimitiveControlOutput {
    let response = ui.interact(
        rect,
        ui.id().with(id_source),
        if options.enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let response = if options.enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    };
    if options.enabled && response.clicked() {
        response.request_focus();
    }
    let keyboard_activation = ui.input(|input| {
        primitive_form_keyboard_activation(
            options.enabled,
            response.has_focus(),
            input.key_pressed(egui::Key::Enter),
            input.key_pressed(egui::Key::Space),
        )
    });
    let changed = options.enabled
        && (response.clicked() || keyboard_activation == PrimitiveKeyboardActivation::Activate);
    if changed {
        *checked = !*checked;
    }
    paint_switch(
        ui,
        rect,
        *checked,
        response.hovered(),
        response.has_focus(),
        options,
    );

    PrimitiveControlOutput { response, changed }
}

fn paint_switch(
    ui: &mut egui::Ui,
    rect: Rect,
    checked: bool,
    hovered: bool,
    focused: bool,
    options: PrimitiveSwitchOptions,
) {
    let root = primitive_switch_root_output(rect, checked, options);
    primitive_switch_root(ui, root.rect, root.checked, hovered, options);
    primitive_switch_thumb(ui, root, options);
    if focused {
        draw_control_focus_ring(ui, rect.expand(3.0), options.theme);
    }
}

pub fn primitive_switch_root(
    ui: &egui::Ui,
    rect: Rect,
    checked: bool,
    hovered: bool,
    options: PrimitiveSwitchOptions,
) {
    let fill = if !options.enabled {
        options.theme.content_fill
    } else if checked {
        control_accent_fill(options.theme)
    } else if hovered && options.enabled {
        options.theme.item_hover_fill
    } else {
        options.theme.content_fill
    };
    ui.painter().rect_filled(rect, options.height * 0.5, fill);
}

pub fn primitive_switch_thumb(
    ui: &egui::Ui,
    root: SwitchRootOutput,
    options: PrimitiveSwitchOptions,
) {
    let thumb = primitive_switch_thumb_output(root);
    let fill = if thumb.disabled {
        options.theme.content_fill
    } else if is_dark_primitive_theme(options.theme) {
        Color32::from_rgb(0xbf, 0xcb, 0xdc)
    } else {
        options.theme.text
    };
    ui.painter().circle_filled(
        thumb.center + Vec2::new(0.0, 1.0),
        options.height * 0.5 - 3.0,
        Color32::from_rgba_unmultiplied(28, 32, 36, 32),
    );
    ui.painter()
        .circle_filled(thumb.center, options.height * 0.5 - 3.0, fill);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadioItem<T> {
    pub value: T,
    pub label: &'static str,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioGroupOrientation {
    Vertical,
    Horizontal,
}

impl RadioGroupOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioGroupDataState {
    Checked,
    Unchecked,
}

impl RadioGroupDataState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checked => "checked",
            Self::Unchecked => "unchecked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupRootOptions {
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub disabled: bool,
    pub required: bool,
    pub orientation: RadioGroupOrientation,
    pub direction: Option<PrimitiveDirection>,
    pub loop_focus: bool,
    pub name: Option<String>,
}

impl Default for RadioGroupRootOptions {
    fn default() -> Self {
        Self {
            value: None,
            default_value: None,
            disabled: false,
            required: false,
            orientation: RadioGroupOrientation::Vertical,
            direction: None,
            loop_focus: true,
            name: None,
        }
    }
}

impl RadioGroupRootOptions {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
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

    pub fn orientation(mut self, orientation: RadioGroupOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn direction(mut self, direction: PrimitiveDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupRootOutput {
    pub id: egui::Id,
    pub item_count: usize,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub disabled: bool,
    pub required: bool,
    pub orientation: RadioGroupOrientation,
    pub direction: Option<PrimitiveDirection>,
    pub loop_focus: bool,
    pub name: Option<String>,
    pub options: RadioGroupRootOptions,
}

pub fn primitive_radio_group_root(
    ui: &egui::Ui,
    id_source: impl Hash,
    item_count: usize,
) -> RadioGroupRootOutput {
    primitive_radio_group_root_with_options(
        ui,
        id_source,
        item_count,
        RadioGroupRootOptions::default(),
    )
}

pub fn primitive_radio_group_root_with_options(
    ui: &egui::Ui,
    id_source: impl Hash,
    item_count: usize,
    options: RadioGroupRootOptions,
) -> RadioGroupRootOutput {
    radio_group_root_output(ui.id().with(id_source), item_count, options)
}

pub fn radio_group_root_output(
    id: egui::Id,
    item_count: usize,
    options: RadioGroupRootOptions,
) -> RadioGroupRootOutput {
    RadioGroupRootOutput {
        id,
        item_count,
        value: options.value.clone(),
        default_value: options.default_value.clone(),
        disabled: options.disabled,
        required: options.required,
        orientation: options.orientation,
        direction: options.direction,
        loop_focus: options.loop_focus,
        name: options.name.clone(),
        options,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupItemOptions {
    pub value: String,
    pub checked: bool,
    pub disabled: bool,
    pub required: bool,
}

impl RadioGroupItemOptions {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            checked: false,
            disabled: false,
            required: false,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
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
pub struct RadioGroupItemOutput {
    pub value: String,
    pub checked: bool,
    pub disabled: bool,
    pub required: bool,
    pub data_state: RadioGroupDataState,
    pub data_disabled: bool,
}

pub fn primitive_radio_group_item_output(options: RadioGroupItemOptions) -> RadioGroupItemOutput {
    RadioGroupItemOutput {
        value: options.value,
        checked: options.checked,
        disabled: options.disabled,
        required: options.required,
        data_state: if options.checked {
            RadioGroupDataState::Checked
        } else {
            RadioGroupDataState::Unchecked
        },
        data_disabled: options.disabled,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupIndicatorOptions {
    pub checked: bool,
    pub disabled: bool,
    pub force_mount: bool,
}

impl RadioGroupIndicatorOptions {
    pub fn new(checked: bool) -> Self {
        Self {
            checked,
            disabled: false,
            force_mount: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupIndicatorOutput {
    pub checked: bool,
    pub disabled: bool,
    pub force_mount: bool,
    pub mounted: bool,
    pub data_state: RadioGroupDataState,
    pub data_disabled: bool,
}

pub fn primitive_radio_group_indicator_output(
    options: RadioGroupIndicatorOptions,
) -> RadioGroupIndicatorOutput {
    RadioGroupIndicatorOutput {
        checked: options.checked,
        disabled: options.disabled,
        force_mount: options.force_mount,
        mounted: options.checked || options.force_mount,
        data_state: if options.checked {
            RadioGroupDataState::Checked
        } else {
            RadioGroupDataState::Unchecked
        },
        data_disabled: options.disabled,
    }
}

pub fn primitive_radio_group<T: Copy + PartialEq + Hash>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    selected: &mut T,
    items: &[RadioItem<T>],
    theme: PrimitiveTheme,
) -> bool {
    primitive_radio_group_with_options(
        ui,
        id_source,
        selected,
        items,
        RadioGroupRootOptions::default(),
        theme,
    )
}

pub fn primitive_radio_group_with_options<T: Copy + PartialEq + Hash>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    selected: &mut T,
    items: &[RadioItem<T>],
    options: RadioGroupRootOptions,
    theme: PrimitiveTheme,
) -> bool {
    let mut changed = false;
    let root = primitive_radio_group_root_with_options(ui, id_source, items.len(), options);
    let keyboard_action = ui.input(|input| {
        radio_group_keyboard_action(
            root.options.orientation,
            root.options.direction,
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
    let mut render_items = |ui: &mut egui::Ui| {
        for (index, item) in items.iter().enumerate() {
            let mut checked = *selected == item.value;
            let item_id = ui.id().with((root.id, index, item.value));
            let output = primitive_radio_item(
                ui,
                (root.id, index, item.value),
                checked,
                item.label,
                item.enabled && !root.options.disabled,
                theme,
            );
            if output.changed && radio_group_apply_value(selected, item.value, items, &root.options)
            {
                checked = true;
                changed = true;
            }
            if output.response.has_focus()
                && keyboard_action != RadioGroupKeyboardAction::None
                && !root.options.disabled
            {
                let target = radio_group_keyboard_target_index(
                    items,
                    Some(index),
                    keyboard_action,
                    root.options.loop_focus,
                );
                if let Some(target) = target {
                    if radio_group_apply_value(selected, items[target].value, items, &root.options)
                    {
                        changed = true;
                        checked = target == index;
                    }
                    let target_id = ui.id().with((root.id, target, items[target].value));
                    if target_id != item_id {
                        ui.memory_mut(|memory| memory.request_focus(target_id));
                    }
                }
            }
            let _ = checked;
        }
    };
    match root.options.orientation {
        RadioGroupOrientation::Vertical => render_items(ui),
        RadioGroupOrientation::Horizontal => {
            ui.horizontal(|ui| render_items(ui));
        }
    }
    changed
}

pub fn radio_group_apply_value<T: Copy + PartialEq>(
    selected: &mut T,
    next: T,
    items: &[RadioItem<T>],
    options: &RadioGroupRootOptions,
) -> bool {
    if options.disabled || *selected == next {
        return false;
    }
    let Some(item) = items.iter().find(|item| item.value == next) else {
        return false;
    };
    if !item.enabled {
        return false;
    }
    *selected = next;
    true
}

pub fn primitive_radio_item(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    checked: bool,
    label: &str,
    enabled: bool,
    theme: PrimitiveTheme,
) -> PrimitiveControlOutput {
    let text_width = label.chars().count() as f32 * 8.0;
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(22.0 + text_width, 24.0), egui::Sense::hover());
    let response = ui.interact(
        rect,
        ui.id().with(id_source),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let response = if enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    };
    if enabled && response.clicked() {
        response.request_focus();
    }
    let keyboard_activation = ui.input(|input| {
        primitive_form_keyboard_activation(
            enabled,
            response.has_focus(),
            input.key_pressed(egui::Key::Enter),
            input.key_pressed(egui::Key::Space),
        )
    });
    let dot = primitive_radio_item_rect(rect, 14.0);
    primitive_radio_item_part(ui, dot, checked, response.hovered(), enabled, theme);
    primitive_radio_indicator(ui, dot, checked, theme);
    if response.has_focus() {
        draw_control_focus_ring(ui, dot.expand(3.0), theme);
    }
    ui.painter().text(
        egui::pos2(dot.right() + 8.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        if enabled {
            theme.text
        } else {
            theme.disabled_text
        },
    );
    let changed = enabled
        && (response.clicked() || keyboard_activation == PrimitiveKeyboardActivation::Activate);
    PrimitiveControlOutput { response, changed }
}

pub fn primitive_radio_item_rect(bounds: Rect, size: f32) -> Rect {
    primitive_checkbox_root_rect(bounds, size)
}

pub fn primitive_radio_item_part(
    ui: &egui::Ui,
    rect: Rect,
    checked: bool,
    hovered: bool,
    enabled: bool,
    theme: PrimitiveTheme,
) {
    let fill = if checked {
        theme.item_selected_fill
    } else if hovered && enabled {
        theme.item_hover_fill
    } else {
        theme.content_fill
    };
    let stroke = if enabled {
        theme.content_stroke
    } else {
        Stroke::new(1.0, theme.disabled_text)
    };
    ui.painter()
        .circle_filled(rect.center(), rect.width() * 0.5, fill);
    ui.painter()
        .circle_stroke(rect.center(), rect.width() * 0.5, stroke);
}

pub fn primitive_radio_indicator(ui: &egui::Ui, rect: Rect, checked: bool, theme: PrimitiveTheme) {
    if checked {
        ui.painter()
            .circle_filled(rect.center(), rect.width() * 0.5 - 4.0, theme.text);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveSliderOptions {
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub width: f32,
    pub height: f32,
    pub theme: PrimitiveTheme,
}

impl PrimitiveSliderOptions {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
            step: 0.0,
            width: 160.0,
            height: 22.0,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}

impl SliderOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliderKeyboardAction {
    None,
    SmallIncrease,
    SmallDecrease,
    LargeIncrease,
    LargeDecrease,
    Minimum,
    Maximum,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliderRootOptions {
    pub value: Vec<f32>,
    pub default_value: Option<Vec<f32>>,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub disabled: bool,
    pub orientation: SliderOrientation,
    pub direction: Option<PrimitiveDirection>,
    pub inverted: bool,
    pub min_steps_between_thumbs: u32,
    pub name: Option<&'static str>,
    pub form: Option<&'static str>,
    pub theme: PrimitiveTheme,
}

impl SliderRootOptions {
    pub fn new(value: impl Into<Vec<f32>>, min: f32, max: f32) -> Self {
        Self {
            value: value.into(),
            default_value: None,
            min,
            max,
            step: 1.0,
            disabled: false,
            orientation: SliderOrientation::Horizontal,
            direction: None,
            inverted: false,
            min_steps_between_thumbs: 0,
            name: None,
            form: None,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn default_value(mut self, default_value: impl Into<Vec<f32>>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn direction(mut self, direction: PrimitiveDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn inverted(mut self, inverted: bool) -> Self {
        self.inverted = inverted;
        self
    }

    pub fn min_steps_between_thumbs(mut self, min_steps_between_thumbs: u32) -> Self {
        self.min_steps_between_thumbs = min_steps_between_thumbs;
        self
    }

    pub fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn form(mut self, form: &'static str) -> Self {
        self.form = Some(form);
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

pub fn primitive_slider(
    ui: &mut egui::Ui,
    value: &mut f32,
    options: PrimitiveSliderOptions,
) -> PrimitiveControlOutput {
    let before = *value;
    *value = slider_snap_value(*value, options.min, options.max, options.step);

    let (rect, response) = ui.allocate_exact_size(
        Vec2::new(options.width, options.height),
        egui::Sense::click_and_drag(),
    );
    let cursor = if response.dragged() {
        egui::CursorIcon::Grabbing
    } else {
        egui::CursorIcon::Grab
    };
    let response = response.on_hover_cursor(cursor);
    if (response.clicked() || response.dragged())
        && let Some(pos) = response.interact_pointer_pos()
    {
        let track = primitive_slider_track_rect(rect);
        let fraction = if track.width().abs() <= f32::EPSILON {
            0.0
        } else {
            ((pos.x - track.left()) / track.width()).clamp(0.0, 1.0)
        };
        *value = slider_snap_value(
            options.min + (options.max - options.min) * fraction,
            options.min,
            options.max,
            options.step,
        );
    }

    primitive_slider_parts(
        ui,
        rect,
        *value,
        options,
        response.hovered() || response.dragged(),
    );

    PrimitiveControlOutput {
        response,
        changed: (*value - before).abs() > f32::EPSILON,
    }
}

pub fn slider_value_fraction(value: f32, min: f32, max: f32) -> f32 {
    let span = max - min;
    if span.abs() <= f32::EPSILON {
        return 0.0;
    }
    ((value - min) / span).clamp(0.0, 1.0)
}

pub fn slider_keyboard_action(
    orientation: SliderOrientation,
    direction: Option<PrimitiveDirection>,
    inverted: bool,
    key: egui::Key,
) -> SliderKeyboardAction {
    match key {
        egui::Key::Home => SliderKeyboardAction::Minimum,
        egui::Key::End => SliderKeyboardAction::Maximum,
        egui::Key::PageUp => SliderKeyboardAction::LargeIncrease,
        egui::Key::PageDown => SliderKeyboardAction::LargeDecrease,
        egui::Key::ArrowRight => slider_keyboard_step_action(
            SliderKeyboardAction::SmallIncrease,
            SliderKeyboardAction::SmallDecrease,
            orientation,
            direction,
            inverted,
        ),
        egui::Key::ArrowLeft => slider_keyboard_step_action(
            SliderKeyboardAction::SmallDecrease,
            SliderKeyboardAction::SmallIncrease,
            orientation,
            direction,
            inverted,
        ),
        egui::Key::ArrowUp => slider_keyboard_step_action(
            SliderKeyboardAction::SmallIncrease,
            SliderKeyboardAction::SmallDecrease,
            orientation,
            None,
            inverted,
        ),
        egui::Key::ArrowDown => slider_keyboard_step_action(
            SliderKeyboardAction::SmallDecrease,
            SliderKeyboardAction::SmallIncrease,
            orientation,
            None,
            inverted,
        ),
        _ => SliderKeyboardAction::None,
    }
}

fn slider_keyboard_step_action(
    normal: SliderKeyboardAction,
    reversed: SliderKeyboardAction,
    orientation: SliderOrientation,
    direction: Option<PrimitiveDirection>,
    inverted: bool,
) -> SliderKeyboardAction {
    let rtl_reversed =
        orientation == SliderOrientation::Horizontal && direction == Some(PrimitiveDirection::Rtl);
    if rtl_reversed ^ inverted {
        reversed
    } else {
        normal
    }
}

pub fn slider_keyboard_value(
    current: f32,
    options: &SliderRootOptions,
    action: SliderKeyboardAction,
) -> Option<f32> {
    if options.disabled || action == SliderKeyboardAction::None {
        return None;
    }
    let step = slider_positive_step(options.step);
    let large_step = step * 10.0;
    let next = match action {
        SliderKeyboardAction::None => return None,
        SliderKeyboardAction::SmallIncrease => current + step,
        SliderKeyboardAction::SmallDecrease => current - step,
        SliderKeyboardAction::LargeIncrease => current + large_step,
        SliderKeyboardAction::LargeDecrease => current - large_step,
        SliderKeyboardAction::Minimum => options.min,
        SliderKeyboardAction::Maximum => options.max,
    };
    Some(slider_snap_value(
        next,
        options.min,
        options.max,
        options.step,
    ))
}

pub fn slider_value_from_pointer(pos: Pos2, root: SliderRootOutput) -> f32 {
    let mut fraction = match root.orientation {
        SliderOrientation::Horizontal => {
            if root.track.width().abs() <= f32::EPSILON {
                0.0
            } else {
                ((pos.x - root.track.left()) / root.track.width()).clamp(0.0, 1.0)
            }
        }
        SliderOrientation::Vertical => {
            if root.track.height().abs() <= f32::EPSILON {
                0.0
            } else {
                ((root.track.bottom() - pos.y) / root.track.height()).clamp(0.0, 1.0)
            }
        }
    };
    if root.orientation == SliderOrientation::Horizontal
        && root.direction == Some(PrimitiveDirection::Rtl)
    {
        fraction = 1.0 - fraction;
    }
    if root.inverted {
        fraction = 1.0 - fraction;
    }
    slider_snap_value(
        root.min + (root.max - root.min) * fraction,
        root.min,
        root.max,
        root.step,
    )
}

pub fn primitive_slider_track_rect(bounds: Rect) -> Rect {
    Rect::from_center_size(bounds.center(), Vec2::new(bounds.width(), 4.0))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderRootOutput {
    pub rect: Rect,
    pub track: Rect,
    pub value: f32,
    pub values: usize,
    pub default_values: usize,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub fraction: f32,
    pub orientation: SliderOrientation,
    pub direction: Option<PrimitiveDirection>,
    pub disabled: bool,
    pub inverted: bool,
    pub min_steps_between_thumbs: u32,
    pub thumb_count: usize,
    pub name: Option<&'static str>,
    pub form: Option<&'static str>,
    pub data_orientation: SliderOrientation,
    pub data_disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderTrackOutput {
    pub rect: Rect,
    pub orientation: SliderOrientation,
    pub disabled: bool,
    pub data_orientation: SliderOrientation,
    pub data_disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderRangeOutput {
    pub rect: Rect,
    pub orientation: SliderOrientation,
    pub disabled: bool,
    pub data_orientation: SliderOrientation,
    pub data_disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SliderThumbOutput {
    pub center: Pos2,
    pub orientation: SliderOrientation,
    pub disabled: bool,
    pub index: usize,
    pub data_orientation: SliderOrientation,
    pub data_disabled: bool,
}

pub fn primitive_slider_root(
    bounds: Rect,
    value: f32,
    options: PrimitiveSliderOptions,
) -> SliderRootOutput {
    let fraction = slider_value_fraction(value, options.min, options.max);
    let track = primitive_slider_track_rect(bounds);
    SliderRootOutput {
        rect: bounds,
        track,
        value,
        values: 1,
        default_values: 0,
        min: options.min,
        max: options.max,
        step: options.step,
        fraction,
        orientation: SliderOrientation::Horizontal,
        direction: None,
        disabled: false,
        inverted: false,
        min_steps_between_thumbs: 0,
        thumb_count: 1,
        name: None,
        form: None,
        data_orientation: SliderOrientation::Horizontal,
        data_disabled: false,
    }
}

pub fn primitive_slider_root_with_options(
    bounds: Rect,
    options: SliderRootOptions,
) -> SliderRootOutput {
    let value = options.value.first().copied().unwrap_or(options.min);
    let mut fraction = slider_value_fraction(value, options.min, options.max);
    if options.inverted {
        fraction = 1.0 - fraction;
    }
    let track = match options.orientation {
        SliderOrientation::Horizontal => primitive_slider_track_rect(bounds),
        SliderOrientation::Vertical => {
            Rect::from_center_size(bounds.center(), Vec2::new(4.0, bounds.height()))
        }
    };
    SliderRootOutput {
        rect: bounds,
        track,
        value,
        values: options.value.len(),
        default_values: options.default_value.as_ref().map_or(0, Vec::len),
        min: options.min,
        max: options.max,
        step: options.step,
        fraction,
        orientation: options.orientation,
        direction: options.direction,
        disabled: options.disabled,
        inverted: options.inverted,
        min_steps_between_thumbs: options.min_steps_between_thumbs,
        thumb_count: options.value.len().max(1),
        name: options.name,
        form: options.form,
        data_orientation: options.orientation,
        data_disabled: options.disabled,
    }
}

pub fn primitive_slider_track_output(root: SliderRootOutput) -> SliderTrackOutput {
    SliderTrackOutput {
        rect: root.track,
        orientation: root.orientation,
        disabled: root.disabled,
        data_orientation: root.data_orientation,
        data_disabled: root.data_disabled,
    }
}

pub fn primitive_slider_range_rect(track: Rect, fraction: f32) -> Rect {
    let right = egui::lerp(track.left()..=track.right(), fraction.clamp(0.0, 1.0));
    Rect::from_min_max(track.left_top(), Pos2::new(right, track.bottom()))
}

pub fn primitive_slider_range_output(root: SliderRootOutput) -> SliderRangeOutput {
    let rect = match root.orientation {
        SliderOrientation::Horizontal => primitive_slider_range_rect(root.track, root.fraction),
        SliderOrientation::Vertical => {
            let top = egui::lerp(root.track.bottom()..=root.track.top(), root.fraction);
            Rect::from_min_max(Pos2::new(root.track.left(), top), root.track.right_bottom())
        }
    };
    SliderRangeOutput {
        rect,
        orientation: root.orientation,
        disabled: root.disabled,
        data_orientation: root.data_orientation,
        data_disabled: root.data_disabled,
    }
}

pub fn primitive_slider_thumb_center(track: Rect, fraction: f32) -> Pos2 {
    Pos2::new(
        egui::lerp(track.left()..=track.right(), fraction.clamp(0.0, 1.0)),
        track.center().y,
    )
}

pub fn primitive_slider_thumb_output(root: SliderRootOutput, index: usize) -> SliderThumbOutput {
    let center = match root.orientation {
        SliderOrientation::Horizontal => primitive_slider_thumb_center(root.track, root.fraction),
        SliderOrientation::Vertical => Pos2::new(
            root.track.center().x,
            egui::lerp(root.track.bottom()..=root.track.top(), root.fraction),
        ),
    };
    SliderThumbOutput {
        center,
        orientation: root.orientation,
        disabled: root.disabled,
        index,
        data_orientation: root.data_orientation,
        data_disabled: root.data_disabled,
    }
}

pub fn primitive_slider_track(ui: &egui::Ui, track: Rect, theme: PrimitiveTheme) {
    let fill = if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x24, 0x2b, 0x36)
    } else {
        theme.item_hover_fill
    };
    let stroke = if is_dark_primitive_theme(theme) {
        Stroke::new(1.0, Color32::from_rgb(0x4a, 0x55, 0x68))
    } else {
        theme.content_stroke
    };
    ui.painter().rect_filled(track, track.height() * 0.5, fill);
    ui.painter().rect_stroke(
        track,
        track.height() * 0.5,
        stroke,
        egui::StrokeKind::Inside,
    );
}

pub fn primitive_slider_range(ui: &egui::Ui, range: Rect, theme: PrimitiveTheme) {
    let fill = if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x68, 0x86, 0xff)
    } else {
        theme.item_selected_fill
    };
    ui.painter().rect_filled(range, range.height() * 0.5, fill);
}

pub fn primitive_slider_thumb(ui: &egui::Ui, center: Pos2, hovered: bool, theme: PrimitiveTheme) {
    let fill = if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x1b, 0x22, 0x2d)
    } else {
        theme.content_fill
    };
    ui.painter().circle_filled(
        center + Vec2::new(0.0, 1.0),
        if hovered { 8.5 } else { 7.5 },
        Color32::from_rgba_unmultiplied(28, 32, 36, 36),
    );
    ui.painter().circle_filled(center, 7.0, fill);
    ui.painter().circle_stroke(
        center,
        if hovered { 8.0 } else { 7.0 },
        Stroke::new(
            1.2,
            if hovered {
                theme.text
            } else if is_dark_primitive_theme(theme) {
                Color32::from_rgb(0x7d, 0x95, 0xc7)
            } else {
                theme.content_stroke.color
            },
        ),
    );
}

pub fn primitive_slider_parts(
    ui: &egui::Ui,
    bounds: Rect,
    value: f32,
    options: PrimitiveSliderOptions,
    hovered: bool,
) {
    let root = primitive_slider_root(bounds, value, options);
    let track = primitive_slider_track_output(root);
    let range = primitive_slider_range_output(root);
    let thumb = primitive_slider_thumb_output(root, 0);

    primitive_slider_track(ui, track.rect, options.theme);
    primitive_slider_range(ui, range.rect, options.theme);
    primitive_slider_thumb(ui, thumb.center, hovered, options.theme);
}

pub fn slider_snap_value(value: f32, min: f32, max: f32, step: f32) -> f32 {
    let clamped = value.clamp(min.min(max), min.max(max));
    if step <= 0.0 {
        return clamped;
    }
    let snapped = ((clamped - min) / step).round() * step + min;
    snapped.clamp(min.min(max), min.max(max))
}

pub fn checkbox_apply_checked(
    current: &mut bool,
    next: bool,
    options: CheckboxRootOptions,
) -> bool {
    if options.disabled || *current == next {
        return false;
    }
    *current = next;
    true
}

pub fn primitive_form_keyboard_activation(
    enabled: bool,
    focused: bool,
    enter_pressed: bool,
    space_pressed: bool,
) -> PrimitiveKeyboardActivation {
    if enabled && focused && (enter_pressed || space_pressed) {
        PrimitiveKeyboardActivation::Activate
    } else {
        PrimitiveKeyboardActivation::None
    }
}

pub fn switch_apply_checked(current: &mut bool, next: bool, options: SwitchRootOptions) -> bool {
    if options.disabled || *current == next {
        return false;
    }
    *current = next;
    true
}

#[allow(clippy::too_many_arguments)]
pub fn radio_group_keyboard_action(
    orientation: RadioGroupOrientation,
    direction: Option<PrimitiveDirection>,
    enter_pressed: bool,
    space_pressed: bool,
    arrow_up_pressed: bool,
    arrow_down_pressed: bool,
    arrow_left_pressed: bool,
    arrow_right_pressed: bool,
    home_pressed: bool,
    end_pressed: bool,
) -> RadioGroupKeyboardAction {
    if enter_pressed || space_pressed {
        return RadioGroupKeyboardAction::Activate;
    }
    if home_pressed {
        return RadioGroupKeyboardAction::First;
    }
    if end_pressed {
        return RadioGroupKeyboardAction::Last;
    }
    match orientation {
        RadioGroupOrientation::Vertical => {
            if arrow_down_pressed {
                RadioGroupKeyboardAction::Next
            } else if arrow_up_pressed {
                RadioGroupKeyboardAction::Previous
            } else {
                RadioGroupKeyboardAction::None
            }
        }
        RadioGroupOrientation::Horizontal => match primitive_horizontal_arrow_step(
            direction,
            arrow_left_pressed,
            arrow_right_pressed,
        ) {
            Some(step) if step > 0 => RadioGroupKeyboardAction::Next,
            Some(_) => RadioGroupKeyboardAction::Previous,
            None => RadioGroupKeyboardAction::None,
        },
    }
}

pub fn radio_group_keyboard_target_index<T: Copy + PartialEq>(
    items: &[RadioItem<T>],
    current: Option<usize>,
    action: RadioGroupKeyboardAction,
    loop_focus: bool,
) -> Option<usize> {
    match action {
        RadioGroupKeyboardAction::None => None,
        RadioGroupKeyboardAction::Activate => {
            current.filter(|index| items.get(*index).is_some_and(|item| item.enabled))
        }
        RadioGroupKeyboardAction::First => items.iter().position(|item| item.enabled),
        RadioGroupKeyboardAction::Last => items.iter().rposition(|item| item.enabled),
        RadioGroupKeyboardAction::Next => {
            radio_group_next_keyboard_index(items, current, 1, loop_focus)
        }
        RadioGroupKeyboardAction::Previous => {
            radio_group_next_keyboard_index(items, current, -1, loop_focus)
        }
    }
}

fn radio_group_next_keyboard_index<T: Copy + PartialEq>(
    items: &[RadioItem<T>],
    current: Option<usize>,
    direction: isize,
    loop_focus: bool,
) -> Option<usize> {
    if items.is_empty() || !items.iter().any(|item| item.enabled) {
        return None;
    }
    let len = items.len() as isize;
    let start = current
        .map(|index| index as isize)
        .unwrap_or(if direction >= 0 { -1 } else { len });
    for step in 1..=items.len() {
        let raw = start + direction * step as isize;
        if !loop_focus && (raw < 0 || raw >= len) {
            return None;
        }
        let index = raw.rem_euclid(len) as usize;
        if items[index].enabled {
            return Some(index);
        }
    }
    None
}

pub fn slider_apply_value(current: &mut f32, next: f32, options: PrimitiveSliderOptions) -> bool {
    let snapped = slider_snap_value(next, options.min, options.max, options.step);
    if (*current - snapped).abs() < f32::EPSILON {
        return false;
    }
    *current = snapped;
    true
}

pub fn slider_apply_thumb_value(
    values: &mut [f32],
    thumb_index: usize,
    next: f32,
    options: &SliderRootOptions,
) -> bool {
    if options.disabled || thumb_index >= values.len() {
        return false;
    }
    let step = slider_positive_step(options.step);
    let minimum_gap = options.min_steps_between_thumbs as f32 * step;
    let lower_bound = if thumb_index == 0 {
        options.min
    } else {
        values[thumb_index - 1] + minimum_gap
    };
    let upper_bound = if thumb_index + 1 >= values.len() {
        options.max
    } else {
        values[thumb_index + 1] - minimum_gap
    };
    let bounded = next.clamp(lower_bound.min(upper_bound), lower_bound.max(upper_bound));
    let snapped = slider_snap_value(bounded, options.min, options.max, options.step)
        .clamp(lower_bound.min(upper_bound), lower_bound.max(upper_bound));
    if (values[thumb_index] - snapped).abs() < f32::EPSILON {
        return false;
    }
    values[thumb_index] = snapped;
    true
}

fn slider_positive_step(step: f32) -> f32 {
    if step > 0.0 { step } else { 1.0 }
}

fn checkbox_box_rect(rect: Rect, size: f32) -> Rect {
    primitive_checkbox_root_rect(rect, size)
}

pub fn primitive_switch_thumb_center(rect: Rect, checked: bool, height: f32) -> Pos2 {
    let radius = height * 0.5 - 3.0;
    egui::pos2(
        if checked {
            rect.right() - radius - 3.0
        } else {
            rect.left() + radius + 3.0
        },
        rect.center().y,
    )
}

fn draw_indeterminate_mark(ui: &egui::Ui, rect: Rect, color: Color32) {
    let stroke = egui::Stroke::new(1.8, color);
    ui.painter().line_segment(
        [
            egui::pos2(rect.left() + rect.width() * 0.25, rect.center().y),
            egui::pos2(rect.right() - rect.width() * 0.25, rect.center().y),
        ],
        stroke,
    );
}

fn draw_checkbox_check_mark(ui: &egui::Ui, rect: Rect, color: Color32) {
    let a = egui::pos2(rect.left() + rect.width() * 0.27, rect.center().y);
    let b = egui::pos2(
        rect.left() + rect.width() * 0.43,
        rect.bottom() - rect.height() * 0.30,
    );
    let c = egui::pos2(
        rect.right() - rect.width() * 0.24,
        rect.top() + rect.height() * 0.28,
    );
    ui.painter().line_segment([a, b], Stroke::new(1.8, color));
    ui.painter().line_segment([b, c], Stroke::new(1.8, color));
}

fn is_dark_primitive_theme(theme: PrimitiveTheme) -> bool {
    let fill = theme.content_fill;
    u16::from(fill.r()) + u16::from(fill.g()) + u16::from(fill.b()) < 160
}

fn control_accent_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x35, 0x55, 0xb8)
    } else {
        theme.item_selected_fill
    }
}

fn control_accent_stroke(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x86, 0x9f, 0xff)
    } else {
        theme.text
    }
}

fn control_idle_stroke(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x5b, 0x67, 0x7a)
    } else {
        theme.content_stroke.color
    }
}

fn control_mark_color(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0xf8, 0xfb, 0xff)
    } else {
        theme.text
    }
}

fn draw_rect_outline(ui: &egui::Ui, rect: Rect, stroke: egui::Stroke) {
    ui.painter()
        .line_segment([rect.left_top(), rect.right_top()], stroke);
    ui.painter()
        .line_segment([rect.right_top(), rect.right_bottom()], stroke);
    ui.painter()
        .line_segment([rect.right_bottom(), rect.left_bottom()], stroke);
    ui.painter()
        .line_segment([rect.left_bottom(), rect.left_top()], stroke);
}

fn draw_control_focus_ring(ui: &egui::Ui, rect: Rect, theme: PrimitiveTheme) {
    ui.painter().rect_stroke(
        rect,
        theme.row_radius + 2.0,
        control_focus_ring_stroke(theme),
        egui::StrokeKind::Inside,
    );
}

fn control_focus_ring_stroke(theme: PrimitiveTheme) -> Stroke {
    let color = if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x8e, 0xc8, 0xff)
    } else {
        radix_colors::INDIGO_9
    };
    Stroke::new(1.5, color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slider_snap_value_clamps_and_steps() {
        assert_eq!(slider_snap_value(12.2, 0.0, 10.0, 0.5), 10.0);
        assert_eq!(slider_snap_value(2.26, 0.0, 10.0, 0.5), 2.5);
        assert_eq!(slider_snap_value(-4.0, 0.0, 10.0, 0.0), 0.0);
    }

    #[test]
    fn checkbox_apply_checked_respects_disabled_and_noop_state() {
        let mut checked = true;

        assert!(checkbox_apply_checked(
            &mut checked,
            false,
            CheckboxRootOptions::default()
        ));
        assert!(!checked);
        assert!(!checkbox_apply_checked(
            &mut checked,
            false,
            CheckboxRootOptions::default()
        ));
        assert!(!checkbox_apply_checked(
            &mut checked,
            true,
            CheckboxRootOptions::default().disabled(true)
        ));
        assert!(!checked);
    }

    #[test]
    fn switch_apply_checked_respects_disabled_and_noop_state() {
        let mut checked = true;

        assert!(switch_apply_checked(
            &mut checked,
            false,
            SwitchRootOptions::default()
        ));
        assert!(!checked);
        assert!(!switch_apply_checked(
            &mut checked,
            false,
            SwitchRootOptions::default()
        ));
        assert!(!switch_apply_checked(
            &mut checked,
            true,
            SwitchRootOptions::default().disabled(true)
        ));
        assert!(!checked);
    }

    #[test]
    fn form_controls_activate_from_focused_space_or_enter() {
        assert_eq!(
            primitive_form_keyboard_activation(true, true, true, false),
            PrimitiveKeyboardActivation::Activate
        );
        assert_eq!(
            primitive_form_keyboard_activation(true, true, false, true),
            PrimitiveKeyboardActivation::Activate
        );
        assert_eq!(
            primitive_form_keyboard_activation(false, true, true, false),
            PrimitiveKeyboardActivation::None
        );
        assert_eq!(
            primitive_form_keyboard_activation(true, false, true, false),
            PrimitiveKeyboardActivation::None
        );
    }

    #[test]
    fn radio_group_keyboard_action_tracks_orientation_and_activation_keys() {
        assert_eq!(
            radio_group_keyboard_action(
                RadioGroupOrientation::Vertical,
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
            RadioGroupKeyboardAction::Activate
        );
        assert_eq!(
            radio_group_keyboard_action(
                RadioGroupOrientation::Vertical,
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
            RadioGroupKeyboardAction::Next
        );
        assert_eq!(
            radio_group_keyboard_action(
                RadioGroupOrientation::Horizontal,
                None,
                false,
                false,
                false,
                false,
                true,
                false,
                false,
                false,
            ),
            RadioGroupKeyboardAction::Previous
        );
        assert_eq!(
            radio_group_keyboard_action(
                RadioGroupOrientation::Vertical,
                None,
                false,
                false,
                false,
                false,
                false,
                false,
                true,
                false,
            ),
            RadioGroupKeyboardAction::First
        );
    }

    #[test]
    fn radio_group_keyboard_action_reverses_horizontal_arrows_in_rtl() {
        assert_eq!(
            radio_group_keyboard_action(
                RadioGroupOrientation::Horizontal,
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
            RadioGroupKeyboardAction::Previous
        );
        assert_eq!(
            radio_group_keyboard_action(
                RadioGroupOrientation::Horizontal,
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
            RadioGroupKeyboardAction::Next
        );
    }

    #[test]
    fn radio_group_keyboard_target_skips_disabled_and_respects_loop_focus() {
        let items = [
            RadioItem {
                value: "alpha",
                label: "Alpha",
                enabled: true,
            },
            RadioItem {
                value: "beta",
                label: "Beta",
                enabled: false,
            },
            RadioItem {
                value: "gamma",
                label: "Gamma",
                enabled: true,
            },
        ];

        assert_eq!(
            radio_group_keyboard_target_index(
                &items,
                Some(0),
                RadioGroupKeyboardAction::Next,
                true,
            ),
            Some(2)
        );
        assert_eq!(
            radio_group_keyboard_target_index(
                &items,
                Some(2),
                RadioGroupKeyboardAction::Next,
                true,
            ),
            Some(0)
        );
        assert_eq!(
            radio_group_keyboard_target_index(
                &items,
                Some(2),
                RadioGroupKeyboardAction::Next,
                false,
            ),
            None
        );
        assert_eq!(
            radio_group_keyboard_target_index(
                &items,
                Some(2),
                RadioGroupKeyboardAction::Previous,
                false,
            ),
            Some(0)
        );
        assert_eq!(
            radio_group_keyboard_target_index(
                &items,
                Some(1),
                RadioGroupKeyboardAction::Activate,
                true,
            ),
            None
        );
    }

    #[test]
    fn form_control_focus_ring_uses_visible_accent_stroke() {
        let light = control_focus_ring_stroke(PrimitiveTheme::light());
        let dark = control_focus_ring_stroke(PrimitiveTheme::dark());

        assert_eq!(light.width, 1.5);
        assert_eq!(dark.width, 1.5);
        assert_ne!(light.color, PrimitiveTheme::light().content_fill);
        assert_ne!(dark.color, PrimitiveTheme::dark().content_fill);
    }

    #[test]
    fn slider_apply_value_uses_snap_clamp_contract() {
        let mut value = 2.0;

        assert!(slider_apply_value(
            &mut value,
            8.8,
            PrimitiveSliderOptions::new(0.0, 10.0).step(0.5)
        ));
        assert_eq!(value, 9.0);
        assert!(slider_apply_value(
            &mut value,
            12.0,
            PrimitiveSliderOptions::new(0.0, 10.0).step(0.5)
        ));
        assert_eq!(value, 10.0);
        assert!(!slider_apply_value(
            &mut value,
            10.0,
            PrimitiveSliderOptions::new(0.0, 10.0).step(0.5)
        ));
    }

    #[test]
    fn slider_keyboard_action_maps_page_home_end_and_directional_arrows() {
        assert_eq!(
            slider_keyboard_action(
                SliderOrientation::Horizontal,
                None,
                false,
                egui::Key::ArrowRight
            ),
            SliderKeyboardAction::SmallIncrease
        );
        assert_eq!(
            slider_keyboard_action(
                SliderOrientation::Horizontal,
                Some(PrimitiveDirection::Rtl),
                false,
                egui::Key::ArrowRight
            ),
            SliderKeyboardAction::SmallDecrease
        );
        assert_eq!(
            slider_keyboard_action(
                SliderOrientation::Horizontal,
                None,
                true,
                egui::Key::ArrowRight
            ),
            SliderKeyboardAction::SmallDecrease
        );
        assert_eq!(
            slider_keyboard_action(SliderOrientation::Vertical, None, false, egui::Key::ArrowUp),
            SliderKeyboardAction::SmallIncrease
        );
        assert_eq!(
            slider_keyboard_action(
                SliderOrientation::Vertical,
                None,
                false,
                egui::Key::ArrowDown
            ),
            SliderKeyboardAction::SmallDecrease
        );
        assert_eq!(
            slider_keyboard_action(
                SliderOrientation::Horizontal,
                None,
                false,
                egui::Key::PageUp
            ),
            SliderKeyboardAction::LargeIncrease
        );
        assert_eq!(
            slider_keyboard_action(SliderOrientation::Horizontal, None, false, egui::Key::Home),
            SliderKeyboardAction::Minimum
        );
        assert_eq!(
            slider_keyboard_action(SliderOrientation::Horizontal, None, false, egui::Key::End),
            SliderKeyboardAction::Maximum
        );
    }

    #[test]
    fn slider_keyboard_value_steps_pages_and_clamps() {
        let options = SliderRootOptions::new(vec![5.0], 0.0, 10.0).step(0.5);

        assert_eq!(
            slider_keyboard_value(5.0, &options, SliderKeyboardAction::SmallIncrease),
            Some(5.5)
        );
        assert_eq!(
            slider_keyboard_value(5.0, &options, SliderKeyboardAction::LargeIncrease),
            Some(10.0)
        );
        assert_eq!(
            slider_keyboard_value(5.0, &options, SliderKeyboardAction::LargeDecrease),
            Some(0.0)
        );
        assert_eq!(
            slider_keyboard_value(5.0, &options, SliderKeyboardAction::Minimum),
            Some(0.0)
        );
        assert_eq!(
            slider_keyboard_value(5.0, &options.disabled(true), SliderKeyboardAction::Maximum),
            None
        );
    }

    #[test]
    fn slider_pointer_value_tracks_vertical_rtl_and_inverted_geometry() {
        let bounds = Rect::from_min_size(egui::pos2(0.0, 0.0), Vec2::new(100.0, 100.0));
        let horizontal = primitive_slider_root_with_options(
            bounds,
            SliderRootOptions::new(vec![50.0], 0.0, 100.0),
        );
        let rtl = primitive_slider_root_with_options(
            bounds,
            SliderRootOptions::new(vec![50.0], 0.0, 100.0).direction(PrimitiveDirection::Rtl),
        );
        let vertical = primitive_slider_root_with_options(
            bounds,
            SliderRootOptions::new(vec![50.0], 0.0, 100.0).orientation(SliderOrientation::Vertical),
        );
        let vertical_inverted = primitive_slider_root_with_options(
            bounds,
            SliderRootOptions::new(vec![50.0], 0.0, 100.0)
                .orientation(SliderOrientation::Vertical)
                .inverted(true),
        );

        assert_eq!(
            slider_value_from_pointer(egui::pos2(horizontal.track.right(), 50.0), horizontal),
            100.0
        );
        assert_eq!(
            slider_value_from_pointer(egui::pos2(rtl.track.right(), 50.0), rtl),
            0.0
        );
        assert_eq!(
            slider_value_from_pointer(egui::pos2(50.0, vertical.track.top()), vertical),
            100.0
        );
        assert_eq!(
            slider_value_from_pointer(
                egui::pos2(50.0, vertical_inverted.track.top()),
                vertical_inverted
            ),
            0.0
        );
    }

    #[test]
    fn slider_apply_thumb_value_preserves_min_step_separation() {
        let options = SliderRootOptions::new(vec![20.0, 80.0], 0.0, 100.0)
            .step(5.0)
            .min_steps_between_thumbs(2);
        let mut values = vec![20.0, 80.0];

        assert!(slider_apply_thumb_value(&mut values, 0, 75.0, &options));
        assert_eq!(values, vec![70.0, 80.0]);
        values = vec![20.0, 80.0];
        assert!(slider_apply_thumb_value(&mut values, 1, 25.0, &options));
        assert_eq!(values, vec![20.0, 30.0]);
        assert!(!slider_apply_thumb_value(
            &mut values,
            0,
            60.0,
            &options.disabled(true)
        ));
    }

    #[test]
    fn slider_part_rects_follow_value_fraction() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(100.0, 22.0));
        let track = primitive_slider_track_rect(bounds);
        let fraction = slider_value_fraction(25.0, 0.0, 100.0);
        let range = primitive_slider_range_rect(track, fraction);
        let thumb = primitive_slider_thumb_center(track, fraction);

        assert_eq!(fraction, 0.25);
        assert!((range.width() - 25.0).abs() < 0.001);
        assert!((thumb.x - 35.0).abs() < 0.001);
    }

    #[test]
    fn slider_root_preserves_bounds_track_and_fraction() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(160.0, 22.0));
        let options = PrimitiveSliderOptions::new(0.0, 100.0).width(160.0);
        let root = primitive_slider_root(bounds, 25.0, options);

        assert_eq!(root.rect, bounds);
        assert_eq!(root.track.center(), bounds.center());
        assert_eq!(root.fraction, 0.25);
    }

    #[test]
    fn slider_root_options_preserve_radix_contract() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(160.0, 80.0));
        let options = SliderRootOptions::new(vec![25.0, 75.0], 0.0, 100.0)
            .default_value(vec![10.0, 90.0])
            .step(5.0)
            .disabled(true)
            .orientation(SliderOrientation::Vertical)
            .direction(PrimitiveDirection::Rtl)
            .inverted(true)
            .min_steps_between_thumbs(2)
            .name("range")
            .form("settings");
        let root = primitive_slider_root_with_options(bounds, options.clone());
        let track = primitive_slider_track_output(root);
        let range = primitive_slider_range_output(root);
        let thumb = primitive_slider_thumb_output(root, 1);

        assert_eq!(root.orientation, SliderOrientation::Vertical);
        assert_eq!(root.data_orientation, SliderOrientation::Vertical);
        assert_eq!(root.data_orientation.as_str(), "vertical");
        assert_eq!(root.direction, Some(PrimitiveDirection::Rtl));
        assert_eq!(root.value, 25.0);
        assert_eq!(root.values, 2);
        assert_eq!(root.default_values, 2);
        assert_eq!(root.min, 0.0);
        assert_eq!(root.max, 100.0);
        assert_eq!(root.step, 5.0);
        assert!(root.disabled);
        assert!(root.data_disabled);
        assert!(root.inverted);
        assert_eq!(root.min_steps_between_thumbs, 2);
        assert_eq!(root.thumb_count, 2);
        assert_eq!(options.step, 5.0);
        assert_eq!(options.min_steps_between_thumbs, 2);
        assert_eq!(root.name, Some("range"));
        assert_eq!(root.form, Some("settings"));
        assert_eq!(track.orientation, SliderOrientation::Vertical);
        assert_eq!(track.data_orientation, SliderOrientation::Vertical);
        assert!(track.data_disabled);
        assert_eq!(range.orientation, SliderOrientation::Vertical);
        assert_eq!(range.data_orientation, SliderOrientation::Vertical);
        assert!(range.data_disabled);
        assert_eq!(thumb.index, 1);
        assert!(thumb.disabled);
        assert_eq!(thumb.data_orientation, SliderOrientation::Vertical);
        assert!(thumb.data_disabled);
    }

    #[test]
    fn switch_thumb_moves_between_track_edges() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(34.0, 18.0));

        assert!(primitive_switch_thumb_center(rect, false, 18.0).x < rect.center().x);
        assert!(primitive_switch_thumb_center(rect, true, 18.0).x > rect.center().x);
    }

    #[test]
    fn switch_root_output_preserves_rect_and_thumb_center() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(34.0, 18.0));
        let options = PrimitiveSwitchOptions::default();
        let off = primitive_switch_root_output(rect, false, options);
        let on = primitive_switch_root_output(rect, true, options);

        assert_eq!(off.rect, rect);
        assert_eq!(on.rect, rect);
        assert!(!off.checked);
        assert_eq!(off.data_state, SwitchDataState::Unchecked);
        assert_eq!(off.data_state.as_str(), "unchecked");
        assert!(on.checked);
        assert_eq!(on.data_state, SwitchDataState::Checked);
        assert_eq!(on.data_state.as_str(), "checked");
        assert!(off.thumb_center.x < rect.center().x);
        assert!(on.thumb_center.x > rect.center().x);
    }

    #[test]
    fn switch_root_options_preserve_form_contract() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(34.0, 18.0));
        let root_options = SwitchRootOptions::default()
            .checked(true)
            .default_checked(false)
            .disabled(true)
            .required(true)
            .name("alerts")
            .value("enabled");
        let root = primitive_switch_root_output_with_options(
            rect,
            root_options.clone(),
            Default::default(),
        );
        let thumb = primitive_switch_thumb_output(root);

        assert!(root.checked);
        assert_eq!(root.default_checked, Some(false));
        assert!(root.disabled);
        assert!(root.required);
        assert_eq!(root.data_state, SwitchDataState::Checked);
        assert!(root.data_disabled);
        assert_eq!(root_options.name.as_deref(), Some("alerts"));
        assert_eq!(root_options.value, "enabled");
        assert_eq!(thumb.center, root.thumb_center);
        assert!(thumb.checked);
        assert!(thumb.disabled);
        assert_eq!(thumb.data_state, SwitchDataState::Checked);
        assert!(thumb.data_disabled);
    }

    #[test]
    fn checkbox_root_rect_uses_leading_square_slot() {
        let bounds = Rect::from_min_size(egui::pos2(20.0, 10.0), Vec2::new(120.0, 24.0));
        let rect = primitive_checkbox_root_rect(bounds, 16.0);

        assert_eq!(rect.width(), 16.0);
        assert_eq!(rect.height(), 16.0);
        assert!((rect.center().x - 28.0).abs() < 0.001);
        assert!((rect.center().y - bounds.center().y).abs() < 0.001);
    }

    #[test]
    fn checkbox_root_output_preserves_state_disabled_and_required_contract() {
        let bounds = Rect::from_min_size(egui::pos2(20.0, 10.0), Vec2::new(120.0, 24.0));
        let options = CheckboxRootOptions::default()
            .state(CheckboxState::Indeterminate)
            .default_state(CheckboxState::Checked)
            .disabled(true)
            .required(true)
            .name("terms")
            .value("accepted")
            .size(18.0);
        let output = primitive_checkbox_root_output(bounds, options);

        assert_eq!(output.rect.width(), 18.0);
        assert_eq!(output.state, CheckboxState::Indeterminate);
        assert_eq!(output.default_state, Some(CheckboxState::Checked));
        assert!(output.disabled);
        assert!(output.required);
        assert_eq!(output.name, Some("terms"));
        assert_eq!(output.value, "accepted");
        assert_eq!(output.data_state, CheckboxState::Indeterminate);
        assert!(output.data_disabled);
    }

    #[test]
    fn checkbox_state_from_bool_matches_checked_contract() {
        assert_eq!(CheckboxState::from_bool(true), CheckboxState::Checked);
        assert_eq!(CheckboxState::from_bool(false), CheckboxState::Unchecked);
        assert!(CheckboxState::Checked.is_checked());
        assert!(!CheckboxState::Indeterminate.is_checked());
        assert_eq!(CheckboxState::Indeterminate.as_str(), "indeterminate");
        assert_eq!(CheckboxState::Checked.as_str(), "checked");
        assert_eq!(CheckboxState::Unchecked.as_str(), "unchecked");
    }

    #[test]
    fn checkbox_indicator_output_mounts_checked_indeterminate_or_force_mounted() {
        let checked = primitive_checkbox_indicator_output(CheckboxIndicatorOptions::new(
            CheckboxState::Checked,
        ));
        let indeterminate = primitive_checkbox_indicator_output(CheckboxIndicatorOptions::new(
            CheckboxState::Indeterminate,
        ));
        let forced = primitive_checkbox_indicator_output(
            CheckboxIndicatorOptions::new(CheckboxState::Unchecked)
                .disabled(true)
                .force_mount(true),
        );
        let hidden = primitive_checkbox_indicator_output(CheckboxIndicatorOptions::new(
            CheckboxState::Unchecked,
        ));

        assert!(checked.mounted);
        assert_eq!(checked.data_state, CheckboxState::Checked);
        assert!(indeterminate.mounted);
        assert_eq!(indeterminate.data_state, CheckboxState::Indeterminate);
        assert!(forced.mounted);
        assert!(forced.disabled);
        assert!(forced.data_disabled);
        assert_eq!(forced.data_state, CheckboxState::Unchecked);
        assert!(!hidden.mounted);
    }

    #[test]
    fn radio_item_rect_uses_same_leading_slot_as_checkbox() {
        let bounds = Rect::from_min_size(egui::pos2(12.0, 8.0), Vec2::new(96.0, 24.0));
        let rect = primitive_radio_item_rect(bounds, 14.0);

        assert_eq!(rect.width(), 14.0);
        assert_eq!(rect.height(), 14.0);
        assert!((rect.center().x - 19.0).abs() < 0.001);
        assert!((rect.center().y - bounds.center().y).abs() < 0.001);
    }

    #[test]
    fn radio_group_root_output_preserves_item_count_contract() {
        let id = egui::Id::new("radio");
        let options = RadioGroupRootOptions::default()
            .value("comfortable")
            .default_value("default")
            .orientation(RadioGroupOrientation::Horizontal)
            .direction(PrimitiveDirection::Rtl)
            .required(true)
            .loop_focus(false)
            .name("density");
        let root = radio_group_root_output(id, 3, options);

        assert_eq!(root.id, id);
        assert_eq!(root.item_count, 3);
        assert_eq!(root.value.as_deref(), Some("comfortable"));
        assert_eq!(root.default_value.as_deref(), Some("default"));
        assert_eq!(root.orientation, RadioGroupOrientation::Horizontal);
        assert_eq!(root.direction, Some(PrimitiveDirection::Rtl));
        assert!(root.required);
        assert!(!root.loop_focus);
        assert_eq!(root.name.as_deref(), Some("density"));
    }

    #[test]
    fn radio_group_root_options_preserve_disabled_and_required_contract() {
        let options = RadioGroupRootOptions::default()
            .disabled(true)
            .required(true)
            .orientation(RadioGroupOrientation::Vertical);

        assert!(options.disabled);
        assert!(options.required);
        assert_eq!(options.orientation, RadioGroupOrientation::Vertical);
    }

    #[test]
    fn radio_group_item_output_preserves_data_state_and_required_contract() {
        let checked = primitive_radio_group_item_output(
            RadioGroupItemOptions::new("compact")
                .checked(true)
                .required(true),
        );
        let disabled = primitive_radio_group_item_output(
            RadioGroupItemOptions::new("disabled").disabled(true),
        );

        assert_eq!(checked.value, "compact");
        assert!(checked.checked);
        assert!(checked.required);
        assert_eq!(checked.data_state, RadioGroupDataState::Checked);
        assert_eq!(checked.data_state.as_str(), "checked");
        assert!(disabled.disabled);
        assert!(disabled.data_disabled);
        assert_eq!(disabled.data_state, RadioGroupDataState::Unchecked);
        assert_eq!(disabled.data_state.as_str(), "unchecked");
    }

    #[test]
    fn radio_group_apply_value_respects_root_item_and_noop_state() {
        let items = [
            RadioItem {
                value: "single",
                label: "Single",
                enabled: true,
            },
            RadioItem {
                value: "multi",
                label: "Multi",
                enabled: true,
            },
            RadioItem {
                value: "off",
                label: "Off",
                enabled: false,
            },
        ];
        let mut selected = "single";

        assert!(radio_group_apply_value(
            &mut selected,
            "multi",
            &items,
            &RadioGroupRootOptions::default()
        ));
        assert_eq!(selected, "multi");
        assert!(!radio_group_apply_value(
            &mut selected,
            "multi",
            &items,
            &RadioGroupRootOptions::default()
        ));
        assert!(!radio_group_apply_value(
            &mut selected,
            "off",
            &items,
            &RadioGroupRootOptions::default()
        ));
        assert_eq!(selected, "multi");
        assert!(!radio_group_apply_value(
            &mut selected,
            "single",
            &items,
            &RadioGroupRootOptions::default().disabled(true)
        ));
        assert_eq!(selected, "multi");
    }

    #[test]
    fn radio_group_indicator_output_mounts_when_checked_or_force_mounted() {
        let checked = primitive_radio_group_indicator_output(RadioGroupIndicatorOptions::new(true));
        let forced = primitive_radio_group_indicator_output(
            RadioGroupIndicatorOptions::new(false)
                .disabled(true)
                .force_mount(true),
        );
        let hidden = primitive_radio_group_indicator_output(RadioGroupIndicatorOptions::new(false));

        assert!(checked.mounted);
        assert_eq!(checked.data_state, RadioGroupDataState::Checked);
        assert!(forced.mounted);
        assert!(forced.disabled);
        assert!(forced.data_disabled);
        assert_eq!(forced.data_state, RadioGroupDataState::Unchecked);
        assert!(!hidden.mounted);
    }

    #[test]
    fn form_field_options_preserve_width_and_spacing() {
        let options = PrimitiveFormFieldOptions::new(240.0).spacing(8.0);

        assert_eq!(options.width, 240.0);
        assert_eq!(options.spacing, 8.0);
    }

    #[test]
    fn form_field_label_and_message_outputs_preserve_validity_contract() {
        let field = primitive_form_field_output(
            PrimitiveFormFieldPartOptions::new("email").server_invalid(true),
        );
        let label =
            primitive_form_label_output(PrimitiveFormLabelOptions::new("email").invalid(true));
        let message = primitive_form_message_output(
            PrimitiveFormMessageOptions::default()
                .name("email")
                .match_kind(PrimitiveFormMessageMatch::TypeMismatch)
                .field_invalid(field.invalid),
        );
        let forced = primitive_form_message_output(
            PrimitiveFormMessageOptions::default()
                .match_kind(PrimitiveFormMessageMatch::ServerInvalid)
                .force_match(true),
        );

        assert_eq!(field.name, "email");
        assert!(field.server_invalid);
        assert!(field.invalid);
        assert!(!field.valid);
        assert!(field.data_invalid);
        assert!(!field.data_valid);
        assert_eq!(label.name, "email");
        assert!(label.data_invalid);
        assert!(!label.data_valid);
        assert_eq!(
            message.match_kind,
            Some(PrimitiveFormMessageMatch::TypeMismatch)
        );
        assert_eq!(
            message.match_kind.map(PrimitiveFormMessageMatch::as_str),
            Some("typeMismatch")
        );
        assert_eq!(message.name.as_deref(), Some("email"));
        assert!(message.visible);
        assert!(forced.visible);
        assert_eq!(
            forced.match_kind.map(PrimitiveFormMessageMatch::as_str),
            Some("serverInvalid")
        );
    }

    #[test]
    fn form_association_output_links_label_control_description_and_error() {
        let association = primitive_form_association_output(
            PrimitiveFormAssociationOptions::new("Email Address")
                .description(true)
                .server_invalid(true)
                .error_match(PrimitiveFormMessageMatch::ServerInvalid),
        );

        assert_eq!(association.name, "Email Address");
        assert_eq!(association.field_id, "form-field-email-address");
        assert_eq!(association.control_id, "form-field-email-address-control");
        assert_eq!(association.label_id, "form-field-email-address-label");
        assert_eq!(association.label_for, association.control_id);
        assert_eq!(
            association.description_id.as_deref(),
            Some("form-field-email-address-description")
        );
        assert_eq!(
            association.error_id.as_deref(),
            Some("form-field-email-address-error")
        );
        assert_eq!(
            association.described_by,
            vec![
                "form-field-email-address-description".to_owned(),
                "form-field-email-address-error".to_owned()
            ]
        );
        assert_eq!(
            association
                .error_match
                .map(PrimitiveFormMessageMatch::as_str),
            Some("serverInvalid")
        );
        assert!(association.invalid);
        assert!(!association.valid);
    }

    #[test]
    fn form_association_output_omits_error_description_when_not_applicable() {
        let association = primitive_form_association_output(
            PrimitiveFormAssociationOptions::new("이메일").description(false),
        );

        assert_eq!(association.field_id, "form-field-field");
        assert_eq!(association.label_for, association.control_id);
        assert_eq!(association.description_id, None);
        assert_eq!(association.error_id, None);
        assert!(association.described_by.is_empty());
        assert_eq!(association.error_match, None);
        assert!(!association.invalid);
        assert!(association.valid);
    }

    #[test]
    fn form_control_options_preserve_control_contract() {
        let options = PrimitiveFormControlOptions::default()
            .width(220.0)
            .enabled(false)
            .invalid(true)
            .name("email");

        assert_eq!(options.width, Some(220.0));
        assert!(!options.enabled);
        assert!(options.invalid);
        assert_eq!(options.name, Some("email"));
    }
}
