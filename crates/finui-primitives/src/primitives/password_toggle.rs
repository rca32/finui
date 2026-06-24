use eframe::egui::{self, Align2, Color32, FontId, Rect, Response, Sense, Stroke, Vec2, pos2};

use super::{PrimitiveTheme, radix_colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordToggleVisibility {
    Hidden,
    Visible,
}

impl PasswordToggleVisibility {
    pub fn is_visible(self) -> bool {
        matches!(self, Self::Visible)
    }

    pub fn data_state(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::Visible => "visible",
        }
    }

    pub fn input_type(self) -> &'static str {
        match self {
            Self::Hidden => "password",
            Self::Visible => "text",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordToggleRootOptions {
    pub value: String,
    pub default_value: Option<String>,
    pub visible: bool,
    pub default_visible: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub name: Option<&'static str>,
    pub form: Option<&'static str>,
    pub auto_complete: &'static str,
}

impl Default for PasswordToggleRootOptions {
    fn default() -> Self {
        Self {
            value: String::new(),
            default_value: None,
            visible: false,
            default_visible: false,
            disabled: false,
            read_only: false,
            required: false,
            name: None,
            form: None,
            auto_complete: "current-password",
        }
    }
}

impl PasswordToggleRootOptions {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn default_value(mut self, default_value: Option<impl Into<String>>) -> Self {
        self.default_value = default_value.map(Into::into);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn default_visible(mut self, default_visible: bool) -> Self {
        self.default_visible = default_visible;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn name(mut self, name: Option<&'static str>) -> Self {
        self.name = name;
        self
    }

    pub fn form(mut self, form: Option<&'static str>) -> Self {
        self.form = form;
        self
    }

    pub fn auto_complete(mut self, auto_complete: &'static str) -> Self {
        self.auto_complete = auto_complete;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordToggleRootOutput {
    pub value: String,
    pub default_value: Option<String>,
    pub visibility: PasswordToggleVisibility,
    pub default_visibility: PasswordToggleVisibility,
    pub disabled: bool,
    pub data_disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub name: Option<&'static str>,
    pub form: Option<&'static str>,
    pub auto_complete: &'static str,
    pub data_state: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordToggleInputOutput {
    pub input_type: &'static str,
    pub value: String,
    pub display_value: String,
    pub name: Option<&'static str>,
    pub form: Option<&'static str>,
    pub disabled: bool,
    pub read_only: bool,
    pub required: bool,
    pub auto_complete: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordToggleButtonOutput {
    pub pressed: bool,
    pub disabled: bool,
    pub aria_pressed: bool,
    pub aria_label: &'static str,
    pub data_state: &'static str,
    pub data_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordToggleIconOutput {
    pub visible: bool,
    pub icon_name: &'static str,
    pub decorative: bool,
}

pub fn primitive_password_toggle_root_output(
    options: &PasswordToggleRootOptions,
) -> PasswordToggleRootOutput {
    let visibility = if options.visible {
        PasswordToggleVisibility::Visible
    } else {
        PasswordToggleVisibility::Hidden
    };
    let default_visibility = if options.default_visible {
        PasswordToggleVisibility::Visible
    } else {
        PasswordToggleVisibility::Hidden
    };
    PasswordToggleRootOutput {
        value: options.value.clone(),
        default_value: options.default_value.clone(),
        visibility,
        default_visibility,
        disabled: options.disabled,
        data_disabled: options.disabled,
        read_only: options.read_only,
        required: options.required,
        name: options.name,
        form: options.form,
        auto_complete: options.auto_complete,
        data_state: visibility.data_state(),
    }
}

pub fn primitive_password_toggle_input_output(
    root: &PasswordToggleRootOutput,
) -> PasswordToggleInputOutput {
    PasswordToggleInputOutput {
        input_type: root.visibility.input_type(),
        value: root.value.clone(),
        display_value: password_toggle_display_value(&root.value, root.visibility),
        name: root.name,
        form: root.form,
        disabled: root.disabled,
        read_only: root.read_only,
        required: root.required,
        auto_complete: root.auto_complete,
    }
}

pub fn primitive_password_toggle_button_output(
    root: &PasswordToggleRootOutput,
) -> PasswordToggleButtonOutput {
    PasswordToggleButtonOutput {
        pressed: root.visibility.is_visible(),
        disabled: root.disabled,
        aria_pressed: root.visibility.is_visible(),
        aria_label: if root.visibility.is_visible() {
            "Hide password"
        } else {
            "Show password"
        },
        data_state: root.data_state,
        data_disabled: root.disabled,
    }
}

pub fn primitive_password_toggle_icon_output(
    root: &PasswordToggleRootOutput,
) -> PasswordToggleIconOutput {
    PasswordToggleIconOutput {
        visible: root.visibility.is_visible(),
        icon_name: if root.visibility.is_visible() {
            "eye-off"
        } else {
            "eye"
        },
        decorative: true,
    }
}

pub fn password_toggle_display_value(value: &str, visibility: PasswordToggleVisibility) -> String {
    if visibility.is_visible() {
        value.to_owned()
    } else {
        "*".repeat(value.chars().count())
    }
}

pub fn password_toggle_apply_visible(
    current: &mut bool,
    next: bool,
    options: &PasswordToggleRootOptions,
) -> bool {
    if options.disabled || options.read_only || *current == next {
        return false;
    }
    *current = next;
    true
}

pub fn password_toggle_part_rects(bounds: Rect, toggle_width: f32) -> (Rect, Rect) {
    let toggle_width = toggle_width.max(0.0).min(bounds.width());
    let input_rect = Rect::from_min_max(
        bounds.min,
        pos2(
            (bounds.right() - toggle_width).max(bounds.left()),
            bounds.bottom(),
        ),
    );
    let toggle_rect = Rect::from_min_max(
        pos2(input_rect.right(), bounds.top()),
        pos2(bounds.right(), bounds.bottom()),
    );
    (input_rect, toggle_rect)
}

pub fn primitive_password_toggle_field(
    ui: &mut egui::Ui,
    options: &PasswordToggleRootOptions,
    width: f32,
    height: f32,
    theme: PrimitiveTheme,
) -> (PasswordToggleRootOutput, Response) {
    let root = primitive_password_toggle_root_output(options);
    let (bounds, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    let response = if root.disabled || root.read_only {
        response
    } else {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    };
    let (input_rect, toggle_rect) = password_toggle_part_rects(bounds, 38.0);
    let input = primitive_password_toggle_input_output(&root);
    let button = primitive_password_toggle_button_output(&root);
    let fill = if root.disabled {
        password_disabled_fill(theme)
    } else {
        theme.content_fill
    };
    ui.painter().rect(
        bounds,
        theme.row_radius,
        fill,
        theme.content_stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().line_segment(
        [toggle_rect.left_top(), toggle_rect.left_bottom()],
        Stroke::new(1.0, password_divider_color(theme)),
    );
    primitive_password_toggle_input(ui, input_rect, &input, theme);
    primitive_password_toggle_button(ui, toggle_rect, &button, theme);
    (root, response)
}

pub fn primitive_password_toggle_input(
    ui: &egui::Ui,
    rect: Rect,
    input: &PasswordToggleInputOutput,
    theme: PrimitiveTheme,
) {
    ui.painter().text(
        rect.left_center() + Vec2::new(8.0, 0.0),
        Align2::LEFT_CENTER,
        &input.display_value,
        crate::scaled_proportional_font(ui, 13.0),
        if input.disabled {
            theme.disabled_text
        } else {
            theme.text
        },
    );
}

pub fn primitive_password_toggle_button(
    ui: &egui::Ui,
    rect: Rect,
    button: &PasswordToggleButtonOutput,
    theme: PrimitiveTheme,
) {
    let color = if button.disabled {
        theme.disabled_text
    } else if button.pressed {
        password_icon_active_color(theme)
    } else {
        password_icon_color(theme)
    };
    paint_password_eye_icon(ui, rect.shrink(9.0), button.pressed, color);
}

fn paint_password_eye_icon(ui: &egui::Ui, rect: Rect, closed: bool, color: Color32) {
    let stroke = Stroke::new(1.45, color);
    let center = rect.center();
    let left = egui::pos2(rect.left(), center.y);
    let top = egui::pos2(center.x, rect.top() + rect.height() * 0.18);
    let right = egui::pos2(rect.right(), center.y);
    let bottom = egui::pos2(center.x, rect.bottom() - rect.height() * 0.18);
    ui.painter().line_segment([left, top], stroke);
    ui.painter().line_segment([top, right], stroke);
    ui.painter().line_segment([right, bottom], stroke);
    ui.painter().line_segment([bottom, left], stroke);
    ui.painter()
        .circle_stroke(center, rect.height() * 0.18, stroke);
    if closed {
        ui.painter().line_segment(
            [
                rect.left_top() + Vec2::new(1.0, rect.height() * 0.12),
                rect.right_bottom() - Vec2::new(1.0, rect.height() * 0.12),
            ],
            Stroke::new(1.7, color),
        );
    }
}

fn is_dark_primitive_theme(theme: PrimitiveTheme) -> bool {
    let fill = theme.content_fill;
    u16::from(fill.r()) + u16::from(fill.g()) + u16::from(fill.b()) < 160
}

fn password_disabled_fill(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x10, 0x14, 0x1b)
    } else {
        radix_colors::SLATE_2
    }
}

fn password_divider_color(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0x55, 0x61, 0x76)
    } else {
        radix_colors::SLATE_5
    }
}

fn password_icon_color(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0xd6, 0xde, 0xec)
    } else {
        theme.muted_text
    }
}

fn password_icon_active_color(theme: PrimitiveTheme) -> Color32 {
    if is_dark_primitive_theme(theme) {
        Color32::from_rgb(0xee, 0xf3, 0xff)
    } else {
        radix_colors::INDIGO_11
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_toggle_root_output_preserves_radix_contract() {
        let options = PasswordToggleRootOptions::default()
            .value("secret")
            .default_value(Some("default"))
            .visible(true)
            .default_visible(false)
            .disabled(true)
            .read_only(true)
            .required(true)
            .name(Some("password"))
            .form(Some("login"))
            .auto_complete("new-password");

        let output = primitive_password_toggle_root_output(&options);

        assert_eq!(output.value, "secret");
        assert_eq!(output.default_value.as_deref(), Some("default"));
        assert_eq!(output.visibility, PasswordToggleVisibility::Visible);
        assert_eq!(output.default_visibility, PasswordToggleVisibility::Hidden);
        assert!(output.disabled);
        assert!(output.data_disabled);
        assert!(output.read_only);
        assert!(output.required);
        assert_eq!(output.name, Some("password"));
        assert_eq!(output.form, Some("login"));
        assert_eq!(output.auto_complete, "new-password");
        assert_eq!(output.data_state, "visible");
    }

    #[test]
    fn password_toggle_input_output_maps_visibility_to_type_and_display() {
        let hidden = primitive_password_toggle_root_output(
            &PasswordToggleRootOptions::default().value("secret"),
        );
        let visible = primitive_password_toggle_root_output(
            &PasswordToggleRootOptions::default()
                .value("secret")
                .visible(true),
        );

        let hidden_input = primitive_password_toggle_input_output(&hidden);
        let visible_input = primitive_password_toggle_input_output(&visible);

        assert_eq!(hidden_input.input_type, "password");
        assert_eq!(hidden_input.display_value, "******");
        assert_eq!(visible_input.input_type, "text");
        assert_eq!(visible_input.display_value, "secret");
    }

    #[test]
    fn password_toggle_button_and_icon_outputs_track_state() {
        let root = primitive_password_toggle_root_output(
            &PasswordToggleRootOptions::default()
                .value("secret")
                .visible(true),
        );

        let button = primitive_password_toggle_button_output(&root);
        let icon = primitive_password_toggle_icon_output(&root);

        assert!(button.pressed);
        assert!(button.aria_pressed);
        assert_eq!(button.aria_label, "Hide password");
        assert_eq!(button.data_state, "visible");
        assert!(icon.visible);
        assert_eq!(icon.icon_name, "eye-off");
        assert!(icon.decorative);
    }

    #[test]
    fn password_toggle_apply_visible_respects_disabled_readonly_and_noop_state() {
        let mut visible = false;

        assert!(password_toggle_apply_visible(
            &mut visible,
            true,
            &PasswordToggleRootOptions::default()
        ));
        assert!(visible);
        assert!(!password_toggle_apply_visible(
            &mut visible,
            true,
            &PasswordToggleRootOptions::default().visible(true)
        ));
        assert!(!password_toggle_apply_visible(
            &mut visible,
            false,
            &PasswordToggleRootOptions::default().disabled(true)
        ));
        assert!(visible);
        assert!(!password_toggle_apply_visible(
            &mut visible,
            false,
            &PasswordToggleRootOptions::default().read_only(true)
        ));
        assert!(visible);
    }

    #[test]
    fn password_toggle_part_rects_reserve_toggle_width() {
        let (input, toggle) = password_toggle_part_rects(
            Rect::from_min_size(pos2(10.0, 20.0), Vec2::new(180.0, 32.0)),
            40.0,
        );

        assert_eq!(input.left(), 10.0);
        assert_eq!(input.width(), 140.0);
        assert_eq!(toggle.left(), input.right());
        assert_eq!(toggle.width(), 40.0);
    }
}
