use eframe::egui::{self, Align2, FontId, Rect, Sense, Stroke, Vec2, pos2};

use super::{PrimitiveDirection, PrimitiveTheme, radix_colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpFieldOrientation {
    Horizontal,
    Vertical,
}

impl OtpFieldOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpFieldInputType {
    Text,
    Password,
}

impl OtpFieldInputType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpFieldValidationType {
    Numeric,
    Alphanumeric,
    Any,
}

impl OtpFieldValidationType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Numeric => "numeric",
            Self::Alphanumeric => "alphanumeric",
            Self::Any => "any",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpFieldRootOptions {
    pub auto_complete: &'static str,
    pub auto_focus: bool,
    pub value: String,
    pub default_value: Option<String>,
    pub auto_submit: bool,
    pub disabled: bool,
    pub direction: PrimitiveDirection,
    pub orientation: OtpFieldOrientation,
    pub form: Option<&'static str>,
    pub name: Option<&'static str>,
    pub placeholder: Option<char>,
    pub read_only: bool,
    pub input_type: OtpFieldInputType,
    pub validation_type: OtpFieldValidationType,
}

impl Default for OtpFieldRootOptions {
    fn default() -> Self {
        Self {
            auto_complete: "one-time-code",
            auto_focus: false,
            value: String::new(),
            default_value: None,
            auto_submit: false,
            disabled: false,
            direction: PrimitiveDirection::Ltr,
            orientation: OtpFieldOrientation::Vertical,
            form: None,
            name: None,
            placeholder: None,
            read_only: false,
            input_type: OtpFieldInputType::Text,
            validation_type: OtpFieldValidationType::Numeric,
        }
    }
}

impl OtpFieldRootOptions {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn default_value(mut self, default_value: Option<impl Into<String>>) -> Self {
        self.default_value = default_value.map(Into::into);
        self
    }

    pub fn auto_submit(mut self, auto_submit: bool) -> Self {
        self.auto_submit = auto_submit;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn direction(mut self, direction: PrimitiveDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn orientation(mut self, orientation: OtpFieldOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn form(mut self, form: Option<&'static str>) -> Self {
        self.form = form;
        self
    }

    pub fn name(mut self, name: Option<&'static str>) -> Self {
        self.name = name;
        self
    }

    pub fn placeholder(mut self, placeholder: Option<char>) -> Self {
        self.placeholder = placeholder;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn input_type(mut self, input_type: OtpFieldInputType) -> Self {
        self.input_type = input_type;
        self
    }

    pub fn validation_type(mut self, validation_type: OtpFieldValidationType) -> Self {
        self.validation_type = validation_type;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpFieldRootOutput {
    pub role: &'static str,
    pub auto_complete: &'static str,
    pub auto_focus: bool,
    pub value: String,
    pub default_value: Option<String>,
    pub auto_submit: bool,
    pub completed: bool,
    pub disabled: bool,
    pub data_disabled: bool,
    pub direction: PrimitiveDirection,
    pub orientation: OtpFieldOrientation,
    pub data_orientation: &'static str,
    pub form: Option<&'static str>,
    pub name: Option<&'static str>,
    pub placeholder: Option<char>,
    pub read_only: bool,
    pub input_type: &'static str,
    pub validation_type: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpFieldInputOutput {
    pub index: usize,
    pub data_index: usize,
    pub value: Option<char>,
    pub display_char: char,
    pub placeholder: Option<char>,
    pub focused: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub data_orientation: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpFieldHiddenInputOutput {
    pub name: Option<&'static str>,
    pub form: Option<&'static str>,
    pub value: String,
    pub disabled: bool,
    pub read_only: bool,
}

pub fn primitive_otp_field_root_output(
    options: &OtpFieldRootOptions,
    length: usize,
) -> OtpFieldRootOutput {
    let value = sanitize_otp_value(&options.value, options.validation_type, length);
    OtpFieldRootOutput {
        role: "group",
        auto_complete: options.auto_complete,
        auto_focus: options.auto_focus,
        completed: value.chars().count() == length,
        value,
        default_value: options.default_value.clone(),
        auto_submit: options.auto_submit,
        disabled: options.disabled,
        data_disabled: options.disabled,
        direction: options.direction,
        orientation: options.orientation,
        data_orientation: options.orientation.as_str(),
        form: options.form,
        name: options.name,
        placeholder: options.placeholder,
        read_only: options.read_only,
        input_type: options.input_type.as_str(),
        validation_type: options.validation_type.as_str(),
    }
}

pub fn primitive_otp_field_input_output(
    root: &OtpFieldRootOutput,
    index: usize,
    focused_index: Option<usize>,
) -> OtpFieldInputOutput {
    let value = root.value.chars().nth(index);
    let display_char = match (root.input_type, value, root.placeholder) {
        ("password", Some(_), _) => '*',
        (_, Some(value), _) => value,
        (_, None, Some(placeholder)) => placeholder,
        _ => ' ',
    };
    OtpFieldInputOutput {
        index,
        data_index: index,
        value,
        display_char,
        placeholder: root.placeholder,
        focused: focused_index == Some(index),
        disabled: root.disabled,
        read_only: root.read_only,
        data_orientation: root.data_orientation,
    }
}

pub fn primitive_otp_field_hidden_input_output(
    root: &OtpFieldRootOutput,
) -> OtpFieldHiddenInputOutput {
    OtpFieldHiddenInputOutput {
        name: root.name,
        form: root.form,
        value: root.value.clone(),
        disabled: root.disabled,
        read_only: root.read_only,
    }
}

pub fn sanitize_otp_value(
    value: &str,
    validation_type: OtpFieldValidationType,
    length: usize,
) -> String {
    value
        .chars()
        .filter(|ch| match validation_type {
            OtpFieldValidationType::Numeric => ch.is_ascii_digit(),
            OtpFieldValidationType::Alphanumeric => ch.is_ascii_alphanumeric(),
            OtpFieldValidationType::Any => !ch.is_control(),
        })
        .take(length)
        .collect()
}

pub fn otp_apply_value(
    current: &mut String,
    next: &str,
    validation_type: OtpFieldValidationType,
    length: usize,
    disabled: bool,
    read_only: bool,
) -> bool {
    if disabled || read_only {
        return false;
    }
    let sanitized = sanitize_otp_value(next, validation_type, length);
    if *current == sanitized {
        return false;
    }
    *current = sanitized;
    true
}

pub fn otp_field_input_rects(
    bounds: Rect,
    length: usize,
    orientation: OtpFieldOrientation,
    gap: f32,
) -> Vec<Rect> {
    if length == 0 {
        return Vec::new();
    }
    let gap = gap.max(0.0);
    match orientation {
        OtpFieldOrientation::Horizontal => {
            let size = ((bounds.width() - gap * length.saturating_sub(1) as f32) / length as f32)
                .min(bounds.height())
                .max(0.0);
            (0..length)
                .map(|index| {
                    Rect::from_min_size(
                        pos2(bounds.left() + index as f32 * (size + gap), bounds.top()),
                        Vec2::splat(size),
                    )
                })
                .collect()
        }
        OtpFieldOrientation::Vertical => {
            let size = ((bounds.height() - gap * length.saturating_sub(1) as f32) / length as f32)
                .min(bounds.width())
                .max(0.0);
            (0..length)
                .map(|index| {
                    Rect::from_min_size(
                        pos2(bounds.left(), bounds.top() + index as f32 * (size + gap)),
                        Vec2::splat(size),
                    )
                })
                .collect()
        }
    }
}

pub fn primitive_otp_field(
    ui: &mut egui::Ui,
    options: &OtpFieldRootOptions,
    length: usize,
    focused_index: Option<usize>,
    theme: PrimitiveTheme,
) -> OtpFieldRootOutput {
    let root = primitive_otp_field_root_output(options, length);
    let input_size = Vec2::splat(32.0);
    let gap = 6.0;
    let desired_size = match root.orientation {
        OtpFieldOrientation::Horizontal => Vec2::new(
            input_size.x * length as f32 + gap * length.saturating_sub(1) as f32,
            input_size.y,
        ),
        OtpFieldOrientation::Vertical => Vec2::new(
            input_size.x,
            input_size.y * length as f32 + gap * length.saturating_sub(1) as f32,
        ),
    };
    let (bounds, _) = ui.allocate_exact_size(desired_size, Sense::hover());
    let rects = otp_field_input_rects(bounds, length, root.orientation, gap);
    for (index, rect) in rects.iter().enumerate() {
        let input = primitive_otp_field_input_output(&root, index, focused_index);
        primitive_otp_field_input(ui, *rect, &input, theme);
    }
    root
}

pub fn primitive_otp_field_input(
    ui: &egui::Ui,
    rect: Rect,
    input: &OtpFieldInputOutput,
    theme: PrimitiveTheme,
) {
    let has_value = input.value.is_some();
    let stroke = if input.focused {
        Stroke::new(1.5, theme.text)
    } else if has_value {
        theme.content_stroke
    } else {
        theme.content_stroke
    };
    let fill = if input.disabled {
        theme.content_fill
    } else if input.focused {
        theme.item_selected_fill
    } else if has_value {
        theme.content_fill
    } else {
        theme.content_fill
    };
    ui.painter().rect(
        rect,
        theme.row_radius,
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        input.display_char,
        crate::scaled_proportional_font(ui, 15.0),
        if input.disabled {
            theme.disabled_text
        } else if has_value {
            theme.text
        } else {
            theme.muted_text
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn otp_root_output_preserves_radix_contract() {
        let options = OtpFieldRootOptions::default()
            .value("12a34")
            .default_value(Some("000000"))
            .auto_submit(true)
            .direction(PrimitiveDirection::Rtl)
            .orientation(OtpFieldOrientation::Horizontal)
            .form(Some("verify-form"))
            .name(Some("otp"))
            .placeholder(Some('_'))
            .read_only(true)
            .input_type(OtpFieldInputType::Password)
            .validation_type(OtpFieldValidationType::Alphanumeric);

        let output = primitive_otp_field_root_output(&options, 6);

        assert_eq!(output.role, "group");
        assert_eq!(output.auto_complete, "one-time-code");
        assert_eq!(output.value, "12a34");
        assert_eq!(output.default_value.as_deref(), Some("000000"));
        assert!(output.auto_submit);
        assert_eq!(output.direction, PrimitiveDirection::Rtl);
        assert_eq!(output.data_orientation, "horizontal");
        assert_eq!(output.form, Some("verify-form"));
        assert_eq!(output.name, Some("otp"));
        assert_eq!(output.placeholder, Some('_'));
        assert!(output.read_only);
        assert_eq!(output.input_type, "password");
        assert_eq!(output.validation_type, "alphanumeric");
    }

    #[test]
    fn otp_input_output_exposes_index_value_and_password_display() {
        let options = OtpFieldRootOptions::default()
            .value("123")
            .input_type(OtpFieldInputType::Password);
        let root = primitive_otp_field_root_output(&options, 6);

        let input = primitive_otp_field_input_output(&root, 1, Some(1));

        assert_eq!(input.index, 1);
        assert_eq!(input.data_index, 1);
        assert_eq!(input.value, Some('2'));
        assert_eq!(input.display_char, '*');
        assert!(input.focused);
        assert_eq!(input.data_orientation, "vertical");
    }

    #[test]
    fn otp_hidden_input_output_carries_form_value_contract() {
        let options = OtpFieldRootOptions::default()
            .value("123456")
            .form(Some("verify-form"))
            .name(Some("otp"));
        let root = primitive_otp_field_root_output(&options, 6);

        let hidden = primitive_otp_field_hidden_input_output(&root);

        assert_eq!(hidden.name, Some("otp"));
        assert_eq!(hidden.form, Some("verify-form"));
        assert_eq!(hidden.value, "123456");
    }

    #[test]
    fn sanitize_otp_value_applies_validation_and_length() {
        assert_eq!(
            sanitize_otp_value("12ab-34", OtpFieldValidationType::Numeric, 4),
            "1234"
        );
        assert_eq!(
            sanitize_otp_value("12ab-34", OtpFieldValidationType::Alphanumeric, 5),
            "12ab3"
        );
    }

    #[test]
    fn otp_apply_value_sanitizes_and_respects_disabled_readonly() {
        let mut value = "123456".to_owned();

        assert!(otp_apply_value(
            &mut value,
            "98a-76",
            OtpFieldValidationType::Numeric,
            4,
            false,
            false
        ));
        assert_eq!(value, "9876");
        assert!(!otp_apply_value(
            &mut value,
            "9876",
            OtpFieldValidationType::Numeric,
            4,
            false,
            false
        ));
        assert!(!otp_apply_value(
            &mut value,
            "1111",
            OtpFieldValidationType::Numeric,
            4,
            true,
            false
        ));
        assert_eq!(value, "9876");
        assert!(!otp_apply_value(
            &mut value,
            "2222",
            OtpFieldValidationType::Numeric,
            4,
            false,
            true
        ));
        assert_eq!(value, "9876");
    }

    #[test]
    fn otp_input_rects_follow_orientation() {
        let horizontal = otp_field_input_rects(
            Rect::from_min_size(pos2(0.0, 0.0), Vec2::new(110.0, 32.0)),
            3,
            OtpFieldOrientation::Horizontal,
            4.0,
        );
        let vertical = otp_field_input_rects(
            Rect::from_min_size(pos2(0.0, 0.0), Vec2::new(32.0, 110.0)),
            3,
            OtpFieldOrientation::Vertical,
            4.0,
        );

        assert_eq!(horizontal.len(), 3);
        assert!(horizontal[1].left() > horizontal[0].left());
        assert_eq!(vertical.len(), 3);
        assert!(vertical[1].top() > vertical[0].top());
    }
}
