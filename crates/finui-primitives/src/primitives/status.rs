use eframe::egui::{self, Color32, FontId, Rect, Response, RichText, Vec2};

use super::{PrimitiveTheme, radix_colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

impl SeparatorOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeparatorRootOptions {
    pub orientation: SeparatorOrientation,
    pub decorative: bool,
    pub available_width: f32,
    pub available_height: f32,
    pub theme: PrimitiveTheme,
}

impl SeparatorRootOptions {
    pub fn new(orientation: SeparatorOrientation) -> Self {
        Self {
            orientation,
            decorative: false,
            available_width: 0.0,
            available_height: 24.0,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    pub fn available(mut self, width: f32, height: f32) -> Self {
        self.available_width = width;
        self.available_height = height;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeparatorRootOutput {
    pub rect: Rect,
    pub line: [egui::Pos2; 2],
    pub orientation: SeparatorOrientation,
    pub decorative: bool,
    pub data_orientation: SeparatorOrientation,
    pub role: Option<&'static str>,
    pub aria_orientation: Option<SeparatorOrientation>,
    pub aria_hidden: bool,
}

pub fn primitive_separator(ui: &mut egui::Ui, orientation: SeparatorOrientation) -> Response {
    let options = SeparatorRootOptions::new(orientation)
        .available(ui.available_width(), ui.available_height())
        .decorative(true);
    let size = primitive_separator_size(
        options.orientation,
        options.available_width,
        options.available_height,
    );
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    let root = primitive_separator_root_output(rect, options);
    primitive_separator_root(ui, root.rect, root.orientation, options.theme);
    response
}

pub fn primitive_separator_size(
    orientation: SeparatorOrientation,
    available_width: f32,
    available_height: f32,
) -> Vec2 {
    match orientation {
        SeparatorOrientation::Horizontal => Vec2::new(available_width, 1.0),
        SeparatorOrientation::Vertical => Vec2::new(1.0, available_height.max(24.0)),
    }
}

pub fn primitive_separator_line(rect: Rect, orientation: SeparatorOrientation) -> [egui::Pos2; 2] {
    match orientation {
        SeparatorOrientation::Horizontal => [rect.left_center(), rect.right_center()],
        SeparatorOrientation::Vertical => [rect.center_top(), rect.center_bottom()],
    }
}

pub fn primitive_separator_root_output(
    rect: Rect,
    options: SeparatorRootOptions,
) -> SeparatorRootOutput {
    SeparatorRootOutput {
        rect,
        line: primitive_separator_line(rect, options.orientation),
        orientation: options.orientation,
        decorative: options.decorative,
        data_orientation: options.orientation,
        role: if options.decorative {
            None
        } else {
            Some("separator")
        },
        aria_orientation: if options.decorative {
            None
        } else {
            Some(options.orientation)
        },
        aria_hidden: options.decorative,
    }
}

pub fn primitive_separator_root(
    ui: &egui::Ui,
    rect: Rect,
    orientation: SeparatorOrientation,
    theme: PrimitiveTheme,
) {
    ui.painter().line_segment(
        primitive_separator_line(rect, orientation),
        egui::Stroke::new(1.0, theme.content_stroke.color),
    );
}

pub fn primitive_label(ui: &mut egui::Ui, text: &str) -> Response {
    primitive_label_root(ui, text, PrimitiveLabelOptions::default().muted(true))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveLabelOptions {
    pub required: bool,
    pub disabled: bool,
    pub muted: bool,
    pub theme: PrimitiveTheme,
}

impl Default for PrimitiveLabelOptions {
    fn default() -> Self {
        Self {
            required: false,
            disabled: false,
            muted: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl PrimitiveLabelOptions {
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelRootOptions {
    pub text: String,
    pub for_id: Option<String>,
    pub nested_control: bool,
    pub required: bool,
    pub disabled: bool,
    pub muted: bool,
    pub theme: PrimitiveTheme,
}

impl LabelRootOptions {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            for_id: None,
            nested_control: false,
            required: false,
            disabled: false,
            muted: false,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn for_id(mut self, for_id: impl Into<String>) -> Self {
        self.for_id = Some(for_id.into());
        self
    }

    pub fn nested_control(mut self, nested_control: bool) -> Self {
        self.nested_control = nested_control;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabelRootOutput {
    pub text: String,
    pub for_id: Option<String>,
    pub html_for: Option<String>,
    pub display_text: String,
    pub color: Color32,
    pub required: bool,
    pub disabled: bool,
    pub nested_control: bool,
    pub associated: bool,
    pub prevents_text_selection_on_double_click: bool,
}

pub fn primitive_label_root(
    ui: &mut egui::Ui,
    text: &str,
    options: PrimitiveLabelOptions,
) -> Response {
    let root = primitive_label_root_output(
        LabelRootOptions::new(text)
            .required(options.required)
            .disabled(options.disabled)
            .muted(options.muted)
            .theme(options.theme),
    );
    ui.label(
        RichText::new(root.display_text)
            .font(crate::scaled_proportional_font(ui, 12.0))
            .color(root.color),
    )
}

pub fn primitive_label_root_output(options: LabelRootOptions) -> LabelRootOutput {
    let label_options = PrimitiveLabelOptions {
        required: options.required,
        disabled: options.disabled,
        muted: options.muted,
        theme: options.theme,
    };
    let associated = options.for_id.is_some() || options.nested_control;
    LabelRootOutput {
        text: options.text.clone(),
        for_id: options.for_id.clone(),
        html_for: options.for_id,
        display_text: primitive_label_text(&options.text, label_options),
        color: primitive_label_color(label_options),
        required: options.required,
        disabled: options.disabled,
        nested_control: options.nested_control,
        associated,
        prevents_text_selection_on_double_click: true,
    }
}

pub fn primitive_label_rich_text(text: &str, options: PrimitiveLabelOptions) -> RichText {
    RichText::new(primitive_label_text(text, options)).color(primitive_label_color(options))
}

pub fn primitive_label_text(text: &str, options: PrimitiveLabelOptions) -> String {
    let suffix = if options.required { " *" } else { "" };
    format!("{text}{suffix}")
}

pub fn primitive_label_color(options: PrimitiveLabelOptions) -> Color32 {
    if options.disabled {
        options.theme.disabled_text
    } else if options.muted {
        options.theme.muted_text
    } else {
        options.theme.text
    }
}

pub fn primitive_progress(
    ui: &mut egui::Ui,
    value: Option<f32>,
    width: f32,
    theme: PrimitiveTheme,
) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 6.0), egui::Sense::hover());
    let root = primitive_progress_root_output(
        rect,
        ProgressRootOptions::new(value.map(|value| value * 100.0), 100.0),
    );
    primitive_progress_root(ui, root.rect, theme);
    if let Some(indicator) = primitive_progress_indicator_output(root) {
        primitive_progress_indicator(ui, indicator.rect, theme);
    }
    response
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressState {
    Loading,
    Complete,
    Indeterminate,
}

impl ProgressState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Complete => "complete",
            Self::Indeterminate => "indeterminate",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressRootOptions {
    pub value: Option<f32>,
    pub max: f32,
    pub value_label: Option<String>,
    pub theme: PrimitiveTheme,
}

impl ProgressRootOptions {
    pub fn new(value: Option<f32>, max: f32) -> Self {
        Self {
            value,
            max,
            value_label: None,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn value_label(mut self, value_label: impl Into<String>) -> Self {
        self.value_label = Some(value_label.into());
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressRootOutput {
    pub rect: Rect,
    pub value: Option<f32>,
    pub max: f32,
    pub normalized: Option<f32>,
    pub state: ProgressState,
    pub value_label: Option<String>,
    pub data_state: ProgressState,
    pub data_value: Option<f32>,
    pub data_max: f32,
    pub aria_value_now: Option<f32>,
    pub aria_value_min: f32,
    pub aria_value_max: f32,
    pub aria_value_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressIndicatorOutput {
    pub rect: Rect,
    pub state: ProgressState,
    pub value: Option<f32>,
    pub max: f32,
    pub data_state: ProgressState,
    pub data_value: Option<f32>,
    pub data_max: f32,
}

pub fn primitive_progress_root_output(
    rect: Rect,
    options: ProgressRootOptions,
) -> ProgressRootOutput {
    let max = options.max.max(f32::EPSILON);
    let normalized = options.value.map(|value| normalized_progress(value / max));
    let state = match normalized {
        None => ProgressState::Indeterminate,
        Some(value) if value >= 1.0 => ProgressState::Complete,
        Some(_) => ProgressState::Loading,
    };
    ProgressRootOutput {
        rect,
        value: options.value,
        max,
        normalized,
        state,
        value_label: options.value_label.clone(),
        data_state: state,
        data_value: options.value,
        data_max: max,
        aria_value_now: options.value,
        aria_value_min: 0.0,
        aria_value_max: max,
        aria_value_text: options.value_label,
    }
}

pub fn primitive_progress_indicator_output(
    root: ProgressRootOutput,
) -> Option<ProgressIndicatorOutput> {
    root.normalized.map(|value| ProgressIndicatorOutput {
        rect: progress_fill_rect(root.rect, value),
        state: root.state,
        value: root.value,
        max: root.max,
        data_state: root.data_state,
        data_value: root.data_value,
        data_max: root.data_max,
    })
}

pub fn primitive_progress_root(ui: &egui::Ui, rect: Rect, _theme: PrimitiveTheme) {
    ui.painter()
        .rect_filled(rect, rect.height() * 0.5, radix_colors::SLATE_4);
    ui.painter().rect_stroke(
        rect,
        rect.height() * 0.5,
        egui::Stroke::new(1.0, radix_colors::SLATE_5),
        egui::StrokeKind::Inside,
    );
}

pub fn primitive_progress_indicator(ui: &egui::Ui, rect: Rect, _theme: PrimitiveTheme) {
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    ui.painter()
        .rect_filled(rect, rect.height() * 0.5, radix_colors::INDIGO_9);
}

pub fn progress_fill_rect(rect: Rect, value: f32) -> Rect {
    Rect::from_min_max(
        rect.min,
        egui::pos2(
            rect.left() + rect.width() * normalized_progress(value),
            rect.bottom(),
        ),
    )
}

pub fn normalized_progress(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalized_progress_clamps_to_unit_range() {
        assert_eq!(normalized_progress(-0.25), 0.0);
        assert_eq!(normalized_progress(0.5), 0.5);
        assert_eq!(normalized_progress(2.0), 1.0);
    }

    #[test]
    fn progress_fill_rect_uses_normalized_width() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(100.0, 6.0));
        let fill = progress_fill_rect(rect, 0.25);

        assert_eq!(fill.left(), 10.0);
        assert_eq!(fill.right(), 35.0);
    }

    #[test]
    fn progress_root_output_preserves_radix_state_contract() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(100.0, 6.0));
        let loading = primitive_progress_root_output(
            rect,
            ProgressRootOptions::new(Some(25.0), 100.0).value_label("25%"),
        );
        let complete =
            primitive_progress_root_output(rect, ProgressRootOptions::new(Some(1.0), 1.0));
        let indeterminate =
            primitive_progress_root_output(rect, ProgressRootOptions::new(None, 100.0));

        assert_eq!(loading.state, ProgressState::Loading);
        assert_eq!(loading.data_state, ProgressState::Loading);
        assert_eq!(loading.data_state.as_str(), "loading");
        assert_eq!(loading.normalized, Some(0.25));
        assert_eq!(loading.value_label.as_deref(), Some("25%"));
        assert_eq!(loading.data_value, Some(25.0));
        assert_eq!(loading.data_max, 100.0);
        assert_eq!(loading.aria_value_now, Some(25.0));
        assert_eq!(loading.aria_value_min, 0.0);
        assert_eq!(loading.aria_value_max, 100.0);
        assert_eq!(loading.aria_value_text.as_deref(), Some("25%"));
        assert_eq!(complete.state, ProgressState::Complete);
        assert_eq!(complete.data_state.as_str(), "complete");
        assert_eq!(indeterminate.state, ProgressState::Indeterminate);
        assert_eq!(indeterminate.data_state.as_str(), "indeterminate");
        assert_eq!(indeterminate.data_value, None);
        assert_eq!(indeterminate.aria_value_now, None);
        assert!(primitive_progress_indicator_output(indeterminate).is_none());
    }

    #[test]
    fn progress_indicator_output_uses_normalized_root_value() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(100.0, 6.0));
        let root =
            primitive_progress_root_output(rect, ProgressRootOptions::new(Some(50.0), 100.0));
        let indicator = primitive_progress_indicator_output(root).expect("loading indicator");

        assert_eq!(indicator.rect.right(), 60.0);
        assert_eq!(indicator.state, ProgressState::Loading);
        assert_eq!(indicator.value, Some(50.0));
        assert_eq!(indicator.max, 100.0);
        assert_eq!(indicator.data_state, ProgressState::Loading);
        assert_eq!(indicator.data_state.as_str(), "loading");
        assert_eq!(indicator.data_value, Some(50.0));
        assert_eq!(indicator.data_max, 100.0);
    }

    #[test]
    fn separator_line_tracks_orientation() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(80.0, 24.0));

        assert_eq!(
            primitive_separator_line(rect, SeparatorOrientation::Horizontal),
            [rect.left_center(), rect.right_center()]
        );
        assert_eq!(
            primitive_separator_line(rect, SeparatorOrientation::Vertical),
            [rect.center_top(), rect.center_bottom()]
        );
        assert_eq!(
            primitive_separator_size(SeparatorOrientation::Vertical, 80.0, 4.0).y,
            24.0
        );
    }

    #[test]
    fn separator_root_options_preserve_orientation_and_decorative_contract() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(80.0, 24.0));
        let options = SeparatorRootOptions::new(SeparatorOrientation::Vertical)
            .decorative(true)
            .available(80.0, 24.0);
        let root = primitive_separator_root_output(rect, options);

        assert_eq!(root.orientation, SeparatorOrientation::Vertical);
        assert_eq!(root.data_orientation, SeparatorOrientation::Vertical);
        assert_eq!(root.data_orientation.as_str(), "vertical");
        assert!(root.decorative);
        assert_eq!(root.role, None);
        assert_eq!(root.aria_orientation, None);
        assert!(root.aria_hidden);
        assert_eq!(root.line, [rect.center_top(), rect.center_bottom()]);
    }

    #[test]
    fn separator_root_output_preserves_semantic_accessibility_contract() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(80.0, 1.0));
        let root = primitive_separator_root_output(
            rect,
            SeparatorRootOptions::new(SeparatorOrientation::Horizontal)
                .decorative(false)
                .available(80.0, 1.0),
        );

        assert_eq!(root.data_orientation.as_str(), "horizontal");
        assert_eq!(root.role, Some("separator"));
        assert_eq!(
            root.aria_orientation,
            Some(SeparatorOrientation::Horizontal)
        );
        assert!(!root.aria_hidden);
    }

    #[test]
    fn separator_root_options_compute_default_sizes() {
        let horizontal =
            SeparatorRootOptions::new(SeparatorOrientation::Horizontal).available(120.0, 30.0);
        let vertical =
            SeparatorRootOptions::new(SeparatorOrientation::Vertical).available(120.0, 4.0);

        assert_eq!(
            primitive_separator_size(
                horizontal.orientation,
                horizontal.available_width,
                horizontal.available_height
            ),
            Vec2::new(120.0, 1.0)
        );
        assert_eq!(
            primitive_separator_size(
                vertical.orientation,
                vertical.available_width,
                vertical.available_height
            ),
            Vec2::new(1.0, 24.0)
        );
    }

    #[test]
    fn label_options_drive_required_text_and_disabled_color() {
        let theme = PrimitiveTheme::default();
        let required = PrimitiveLabelOptions::default().required(true);
        let disabled_muted = PrimitiveLabelOptions::default().disabled(true).muted(true);

        assert_eq!(primitive_label_text("가격", required), "가격 *");
        assert_eq!(primitive_label_color(required), theme.text);
        assert_eq!(primitive_label_color(disabled_muted), theme.disabled_text);
    }

    #[test]
    fn label_root_output_preserves_for_id_and_display_contract() {
        let root = primitive_label_root_output(
            LabelRootOptions::new("가격")
                .for_id("price-input")
                .required(true),
        );

        assert_eq!(root.text, "가격");
        assert_eq!(root.for_id.as_deref(), Some("price-input"));
        assert_eq!(root.html_for.as_deref(), Some("price-input"));
        assert_eq!(root.display_text, "가격 *");
        assert!(root.required);
        assert!(!root.disabled);
        assert!(root.associated);
        assert!(!root.nested_control);
        assert!(root.prevents_text_selection_on_double_click);
    }

    #[test]
    fn label_root_output_uses_disabled_color() {
        let theme = PrimitiveTheme::default();
        let root = primitive_label_root_output(LabelRootOptions::new("비활성").disabled(true));

        assert_eq!(root.color, theme.disabled_text);
        assert!(root.disabled);
        assert!(!root.associated);
    }

    #[test]
    fn label_root_output_supports_nested_control_association() {
        let root =
            primitive_label_root_output(LabelRootOptions::new("Nested").nested_control(true));

        assert_eq!(root.html_for, None);
        assert!(root.nested_control);
        assert!(root.associated);
    }
}
