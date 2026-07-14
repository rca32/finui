use egui::{self, Color32, Rect, Stroke, Vec2};

use super::{BadgeTone, BadgeVariant, PrimitiveTheme, RadixIcon, badge_visual, paint_radix_icon};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalloutTone {
    Neutral,
    Accent,
    Positive,
    Warning,
    Danger,
}

impl CalloutTone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Accent => "accent",
            Self::Positive => "positive",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    fn badge_tone(self) -> BadgeTone {
        match self {
            Self::Neutral => BadgeTone::Neutral,
            Self::Accent => BadgeTone::Accent,
            Self::Positive => BadgeTone::Positive,
            Self::Warning => BadgeTone::Warning,
            Self::Danger => BadgeTone::Danger,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalloutVariant {
    Soft,
    Surface,
    Outline,
}

impl CalloutVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Soft => "soft",
            Self::Surface => "surface",
            Self::Outline => "outline",
        }
    }

    fn badge_variant(self) -> BadgeVariant {
        match self {
            Self::Soft => BadgeVariant::Soft,
            Self::Surface => BadgeVariant::Surface,
            Self::Outline => BadgeVariant::Outline,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalloutRootOptions {
    pub tone: CalloutTone,
    pub variant: CalloutVariant,
    pub icon: Option<RadixIcon>,
    pub accent_bar: bool,
    pub theme: PrimitiveTheme,
}

impl Default for CalloutRootOptions {
    fn default() -> Self {
        Self {
            tone: CalloutTone::Neutral,
            variant: CalloutVariant::Surface,
            icon: None,
            accent_bar: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl CalloutRootOptions {
    pub fn tone(mut self, tone: CalloutTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn variant(mut self, variant: CalloutVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn icon(mut self, icon: RadixIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn accent_bar(mut self, accent_bar: bool) -> Self {
        self.accent_bar = accent_bar;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalloutVisual {
    pub fill: Color32,
    pub stroke: Color32,
    pub accent: Color32,
    pub title: Color32,
    pub body: Color32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalloutIconOutput {
    pub rect: Rect,
    pub icon: RadixIcon,
    pub color: Color32,
    pub aria_hidden: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalloutTextOutput {
    pub title: String,
    pub body: String,
    pub title_pos: egui::Pos2,
    pub body_pos: egui::Pos2,
    pub max_width: f32,
    pub title_color: Color32,
    pub body_color: Color32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalloutRootOutput {
    pub rect: Rect,
    pub tone: CalloutTone,
    pub variant: CalloutVariant,
    pub visual: CalloutVisual,
    pub accent_rect: Option<Rect>,
    pub icon: Option<CalloutIconOutput>,
    pub text: CalloutTextOutput,
    pub data_tone: &'static str,
    pub data_variant: &'static str,
    pub role: &'static str,
}

const CALLOUT_RADIUS: f32 = 6.0;
const CALLOUT_ICON_LEFT: f32 = 12.0;
const CALLOUT_ICON_TOP: f32 = 15.0;
const CALLOUT_ICON_SIZE: f32 = 14.0;
const CALLOUT_TEXT_LEFT_WITH_ICON: f32 = 26.0;
const CALLOUT_TEXT_LEFT: f32 = 12.0;
const CALLOUT_TITLE_TOP: f32 = 6.0;
const CALLOUT_BODY_TOP: f32 = 23.0;
const CALLOUT_RIGHT_INSET: f32 = 10.0;
const CALLOUT_ACCENT_TOP: f32 = 7.0;
const CALLOUT_ACCENT_BOTTOM: f32 = 7.0;
const CALLOUT_TITLE_FONT_SIZE: f32 = 12.5;
const CALLOUT_BODY_FONT_SIZE: f32 = 10.5;

pub fn primitive_callout_at(
    ui: &egui::Ui,
    rect: Rect,
    title: &str,
    body: &str,
    options: CalloutRootOptions,
) -> CalloutRootOutput {
    let output = primitive_callout_root_output(rect, title, body, options);
    primitive_callout_root(ui, &output);
    if let Some(icon) = output.icon {
        primitive_callout_icon(ui, icon);
    }
    primitive_callout_text(ui, &output.text);
    output
}

pub fn primitive_callout_root_output(
    rect: Rect,
    title: &str,
    body: &str,
    options: CalloutRootOptions,
) -> CalloutRootOutput {
    let visual = callout_visual(options.tone, options.variant, options.theme);
    let text_left = if options.icon.is_some() {
        CALLOUT_TEXT_LEFT_WITH_ICON
    } else {
        CALLOUT_TEXT_LEFT
    };
    let icon = options.icon.map(|icon| CalloutIconOutput {
        rect: Rect::from_center_size(
            rect.left_top() + egui::vec2(CALLOUT_ICON_LEFT, CALLOUT_ICON_TOP),
            Vec2::splat(CALLOUT_ICON_SIZE),
        ),
        icon,
        color: visual.accent,
        aria_hidden: true,
    });
    let accent_rect = options.accent_bar.then(|| {
        Rect::from_min_max(
            rect.left_top() + egui::vec2(0.0, CALLOUT_ACCENT_TOP),
            egui::pos2(
                rect.left() + 2.0,
                (rect.bottom() - CALLOUT_ACCENT_BOTTOM).max(rect.top() + CALLOUT_ACCENT_TOP),
            ),
        )
    });
    let text = CalloutTextOutput {
        title: title.to_string(),
        body: body.to_string(),
        title_pos: rect.left_top() + egui::vec2(text_left, CALLOUT_TITLE_TOP),
        body_pos: rect.left_top() + egui::vec2(text_left, CALLOUT_BODY_TOP),
        max_width: (rect.width() - text_left - CALLOUT_RIGHT_INSET).max(0.0),
        title_color: visual.title,
        body_color: visual.body,
    };
    CalloutRootOutput {
        rect,
        tone: options.tone,
        variant: options.variant,
        visual,
        accent_rect,
        icon,
        text,
        data_tone: options.tone.as_str(),
        data_variant: options.variant.as_str(),
        role: "note",
    }
}

pub fn primitive_callout_root(ui: &egui::Ui, output: &CalloutRootOutput) {
    ui.painter()
        .rect_filled(output.rect, CALLOUT_RADIUS, output.visual.fill);
    if output.visual.stroke != Color32::TRANSPARENT {
        ui.painter().rect_stroke(
            output.rect,
            CALLOUT_RADIUS,
            Stroke::new(1.0, output.visual.stroke),
            egui::StrokeKind::Inside,
        );
    }
    if let Some(accent_rect) = output.accent_rect {
        ui.painter()
            .rect_filled(accent_rect, 1.0, output.visual.accent);
    }
}

pub fn primitive_callout_icon(ui: &egui::Ui, output: CalloutIconOutput) {
    paint_radix_icon(ui, output.icon, output.rect, output.color);
}

pub fn primitive_callout_text(ui: &egui::Ui, output: &CalloutTextOutput) {
    let clip_rect = Rect::from_min_max(
        output.title_pos,
        egui::pos2(
            output.title_pos.x + output.max_width,
            ui.clip_rect().bottom(),
        ),
    );
    let painter = ui.painter().with_clip_rect(clip_rect);
    painter.text(
        output.title_pos,
        egui::Align2::LEFT_TOP,
        output.title.as_str(),
        crate::scaled_proportional_font(ui, CALLOUT_TITLE_FONT_SIZE),
        output.title_color,
    );
    painter.text(
        output.body_pos,
        egui::Align2::LEFT_TOP,
        output.body.as_str(),
        crate::scaled_proportional_font(ui, CALLOUT_BODY_FONT_SIZE),
        output.body_color,
    );
}

pub fn callout_visual(
    tone: CalloutTone,
    variant: CalloutVariant,
    theme: PrimitiveTheme,
) -> CalloutVisual {
    let badge = badge_visual(tone.badge_tone(), variant.badge_variant(), theme);
    CalloutVisual {
        fill: badge.fill,
        stroke: badge.stroke,
        accent: badge.text,
        title: theme.text,
        body: theme.muted_text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callout_root_output_exposes_radix_part_geometry_and_state() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(320.0, 44.0));
        let output = primitive_callout_root_output(
            rect,
            "Review blocked",
            "Resolve required evidence",
            CalloutRootOptions::default()
                .tone(CalloutTone::Warning)
                .variant(CalloutVariant::Surface)
                .icon(RadixIcon::ExclamationTriangle),
        );

        assert_eq!(output.rect, rect);
        assert_eq!(output.data_tone, "warning");
        assert_eq!(output.data_variant, "surface");
        assert_eq!(output.role, "note");
        assert!(output.accent_rect.is_some());
        assert_eq!(
            output.icon.map(|icon| icon.icon),
            Some(RadixIcon::ExclamationTriangle)
        );
        assert_eq!(output.text.title, "Review blocked");
        assert_eq!(output.text.body, "Resolve required evidence");
        assert!(output.text.max_width > 0.0);
    }

    #[test]
    fn callout_parts_support_light_and_dark_semantic_tones() {
        for tone in [
            CalloutTone::Neutral,
            CalloutTone::Accent,
            CalloutTone::Positive,
            CalloutTone::Warning,
            CalloutTone::Danger,
        ] {
            let light = callout_visual(tone, CalloutVariant::Surface, PrimitiveTheme::light());
            let dark = callout_visual(tone, CalloutVariant::Surface, PrimitiveTheme::dark());

            assert_ne!(light.fill, dark.fill);
            assert_ne!(light.stroke, dark.stroke);
            assert_ne!(light.accent, dark.accent);
            assert_eq!(light.title, PrimitiveTheme::light().text);
            assert_eq!(dark.body, PrimitiveTheme::dark().muted_text);
        }
    }

    #[test]
    fn callout_without_icon_reclaims_text_width() {
        let rect = Rect::from_min_size(egui::Pos2::ZERO, Vec2::new(240.0, 44.0));
        let with_icon = primitive_callout_root_output(
            rect,
            "Title",
            "Body",
            CalloutRootOptions::default().icon(RadixIcon::InfoCircled),
        );
        let without_icon =
            primitive_callout_root_output(rect, "Title", "Body", CalloutRootOptions::default());

        assert!(without_icon.text.title_pos.x < with_icon.text.title_pos.x);
        assert!(without_icon.text.max_width > with_icon.text.max_width);
    }
}
