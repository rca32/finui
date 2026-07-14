use egui::{self, Color32, Rect, Response, Stroke, Vec2};

use super::{PrimitiveTheme, RadixIcon, paint_radix_icon, primitive_theme_is_dark, radix_colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeTone {
    Neutral,
    Accent,
    Positive,
    Warning,
    Danger,
}

impl BadgeTone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Accent => "accent",
            Self::Positive => "positive",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    Soft,
    Surface,
    Outline,
    Solid,
}

impl BadgeVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Soft => "soft",
            Self::Surface => "surface",
            Self::Outline => "outline",
            Self::Solid => "solid",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BadgeRootOptions {
    pub tone: BadgeTone,
    pub variant: BadgeVariant,
    pub icon: Option<RadixIcon>,
    pub theme: PrimitiveTheme,
}

impl Default for BadgeRootOptions {
    fn default() -> Self {
        Self {
            tone: BadgeTone::Neutral,
            variant: BadgeVariant::Surface,
            icon: None,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl BadgeRootOptions {
    pub fn tone(mut self, tone: BadgeTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn icon(mut self, icon: RadixIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadgeVisual {
    pub fill: Color32,
    pub stroke: Color32,
    pub text: Color32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BadgeRootOutput {
    pub rect: Rect,
    pub label: String,
    pub tone: BadgeTone,
    pub variant: BadgeVariant,
    pub icon: Option<RadixIcon>,
    pub icon_rect: Option<Rect>,
    pub text_pos: egui::Pos2,
    pub visual: BadgeVisual,
    pub data_tone: &'static str,
    pub data_variant: &'static str,
    pub role: Option<&'static str>,
}

const BADGE_HEIGHT: f32 = 22.0;
const BADGE_HORIZONTAL_INSET: f32 = 8.0;
const BADGE_ICON_SIZE: f32 = 12.0;
const BADGE_ICON_GAP: f32 = 5.0;
const BADGE_FONT_SIZE: f32 = 11.0;
const BADGE_RADIUS: f32 = 4.0;

pub fn primitive_badge(ui: &mut egui::Ui, label: &str, options: BadgeRootOptions) -> Response {
    let font = crate::scaled_proportional_font(ui, BADGE_FONT_SIZE);
    let text_width = ui
        .painter()
        .layout_no_wrap(label.to_string(), font, options.theme.text)
        .size()
        .x;
    let size = primitive_badge_size(text_width, options.icon.is_some());
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());
    primitive_badge_at(ui, rect, label, options);
    response
}

pub fn primitive_badge_size(text_width: f32, has_icon: bool) -> Vec2 {
    let icon_width = if has_icon {
        BADGE_ICON_SIZE + BADGE_ICON_GAP
    } else {
        0.0
    };
    Vec2::new(
        (BADGE_HORIZONTAL_INSET * 2.0 + icon_width + text_width).max(BADGE_HEIGHT),
        BADGE_HEIGHT,
    )
}

pub fn primitive_badge_at(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    options: BadgeRootOptions,
) -> BadgeRootOutput {
    let output = primitive_badge_root_output(rect, label, options);
    primitive_badge_root(ui, &output);
    output
}

pub fn primitive_badge_root_output(
    rect: Rect,
    label: &str,
    options: BadgeRootOptions,
) -> BadgeRootOutput {
    let icon_rect = options.icon.map(|_| {
        Rect::from_center_size(
            egui::pos2(
                rect.left() + BADGE_HORIZONTAL_INSET + BADGE_ICON_SIZE * 0.5,
                rect.center().y,
            ),
            Vec2::splat(BADGE_ICON_SIZE),
        )
    });
    let text_x = rect.left()
        + BADGE_HORIZONTAL_INSET
        + if options.icon.is_some() {
            BADGE_ICON_SIZE + BADGE_ICON_GAP
        } else {
            0.0
        };
    BadgeRootOutput {
        rect,
        label: label.to_string(),
        tone: options.tone,
        variant: options.variant,
        icon: options.icon,
        icon_rect,
        text_pos: egui::pos2(text_x, rect.center().y),
        visual: badge_visual(options.tone, options.variant, options.theme),
        data_tone: options.tone.as_str(),
        data_variant: options.variant.as_str(),
        role: None,
    }
}

pub fn primitive_badge_root(ui: &egui::Ui, output: &BadgeRootOutput) {
    ui.painter()
        .rect_filled(output.rect, BADGE_RADIUS, output.visual.fill);
    if output.visual.stroke != Color32::TRANSPARENT {
        ui.painter().rect_stroke(
            output.rect,
            BADGE_RADIUS,
            Stroke::new(1.0, output.visual.stroke),
            egui::StrokeKind::Inside,
        );
    }
    if let (Some(icon), Some(icon_rect)) = (output.icon, output.icon_rect) {
        paint_radix_icon(ui, icon, icon_rect, output.visual.text);
    }
    ui.painter().text(
        output.text_pos,
        egui::Align2::LEFT_CENTER,
        output.label.as_str(),
        crate::scaled_proportional_font(ui, BADGE_FONT_SIZE),
        output.visual.text,
    );
}

pub fn badge_visual(tone: BadgeTone, variant: BadgeVariant, theme: PrimitiveTheme) -> BadgeVisual {
    let palette = badge_palette(tone, primitive_theme_is_dark(theme));
    match variant {
        BadgeVariant::Soft => BadgeVisual {
            fill: palette.subtle,
            stroke: Color32::TRANSPARENT,
            text: palette.text,
        },
        BadgeVariant::Surface => BadgeVisual {
            fill: palette.subtle,
            stroke: palette.stroke,
            text: palette.text,
        },
        BadgeVariant::Outline => BadgeVisual {
            fill: Color32::TRANSPARENT,
            stroke: palette.stroke,
            text: palette.text,
        },
        BadgeVariant::Solid => BadgeVisual {
            fill: palette.solid,
            stroke: palette.solid,
            text: palette.solid_text,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BadgePalette {
    subtle: Color32,
    stroke: Color32,
    text: Color32,
    solid: Color32,
    solid_text: Color32,
}

fn badge_palette(tone: BadgeTone, dark: bool) -> BadgePalette {
    let white = Color32::from_rgb(0xff, 0xff, 0xff);
    let dark_text = Color32::from_rgb(0x21, 0x1a, 0x00);
    match (dark, tone) {
        (false, BadgeTone::Neutral) => BadgePalette {
            subtle: radix_colors::SLATE_3,
            stroke: radix_colors::SLATE_7,
            text: radix_colors::SLATE_11,
            solid: radix_colors::SLATE_9,
            solid_text: white,
        },
        (false, BadgeTone::Accent) => BadgePalette {
            subtle: radix_colors::INDIGO_3,
            stroke: radix_colors::INDIGO_7,
            text: radix_colors::INDIGO_11,
            solid: radix_colors::INDIGO_9,
            solid_text: white,
        },
        (false, BadgeTone::Positive) => BadgePalette {
            subtle: Color32::from_rgb(0xe9, 0xf9, 0xee),
            stroke: Color32::from_rgb(0x8e, 0xce, 0xaa),
            text: Color32::from_rgb(0x21, 0x83, 0x58),
            solid: radix_colors::GREEN_9,
            solid_text: white,
        },
        (false, BadgeTone::Warning) => BadgePalette {
            subtle: Color32::from_rgb(0xff, 0xf7, 0xc2),
            stroke: Color32::from_rgb(0xfa, 0xcf, 0x57),
            text: Color32::from_rgb(0xab, 0x64, 0x00),
            solid: radix_colors::AMBER_9,
            solid_text: dark_text,
        },
        (false, BadgeTone::Danger) => BadgePalette {
            subtle: Color32::from_rgb(0xff, 0xf0, 0xee),
            stroke: Color32::from_rgb(0xf3, 0xae, 0xaf),
            text: Color32::from_rgb(0xce, 0x2c, 0x31),
            solid: radix_colors::TOMATO_9,
            solid_text: white,
        },
        (true, BadgeTone::Neutral) => BadgePalette {
            subtle: Color32::from_rgb(0x20, 0x21, 0x23),
            stroke: Color32::from_rgb(0x3e, 0x3e, 0x44),
            text: Color32::from_rgb(0xb4, 0xb4, 0xbb),
            solid: Color32::from_rgb(0x69, 0x6e, 0x77),
            solid_text: white,
        },
        (true, BadgeTone::Accent) => BadgePalette {
            subtle: Color32::from_rgb(0x18, 0x24, 0x49),
            stroke: Color32::from_rgb(0x34, 0x51, 0xb2),
            text: Color32::from_rgb(0x9e, 0xb1, 0xff),
            solid: Color32::from_rgb(0x3e, 0x63, 0xdd),
            solid_text: white,
        },
        (true, BadgeTone::Positive) => BadgePalette {
            subtle: Color32::from_rgb(0x17, 0x3b, 0x2d),
            stroke: Color32::from_rgb(0x28, 0x68, 0x4a),
            text: Color32::from_rgb(0x3d, 0xd6, 0x8c),
            solid: Color32::from_rgb(0x30, 0xa4, 0x6c),
            solid_text: white,
        },
        (true, BadgeTone::Warning) => BadgePalette {
            subtle: Color32::from_rgb(0x3b, 0x2e, 0x00),
            stroke: Color32::from_rgb(0x8f, 0x64, 0x24),
            text: Color32::from_rgb(0xf2, 0xb9, 0x0f),
            solid: Color32::from_rgb(0xff, 0xc5, 0x3d),
            solid_text: dark_text,
        },
        (true, BadgeTone::Danger) => BadgePalette {
            subtle: Color32::from_rgb(0x39, 0x17, 0x14),
            stroke: Color32::from_rgb(0x7f, 0x2c, 0x2c),
            text: Color32::from_rgb(0xff, 0x97, 0x7d),
            solid: Color32::from_rgb(0xe5, 0x4d, 0x2e),
            solid_text: white,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn badge_root_output_preserves_tone_variant_and_radix_icon_contract() {
        let rect = Rect::from_min_size(egui::pos2(10.0, 8.0), Vec2::new(84.0, 22.0));
        let output = primitive_badge_root_output(
            rect,
            "Validated",
            BadgeRootOptions::default()
                .tone(BadgeTone::Positive)
                .variant(BadgeVariant::Surface)
                .icon(RadixIcon::Check),
        );

        assert_eq!(output.rect, rect);
        assert_eq!(output.label, "Validated");
        assert_eq!(output.tone, BadgeTone::Positive);
        assert_eq!(output.data_tone, "positive");
        assert_eq!(output.variant, BadgeVariant::Surface);
        assert_eq!(output.data_variant, "surface");
        assert_eq!(output.icon, Some(RadixIcon::Check));
        assert!(output.icon_rect.is_some());
        assert_eq!(output.role, None);
    }

    #[test]
    fn badge_surface_visuals_support_light_and_dark_semantic_tones() {
        for tone in [
            BadgeTone::Neutral,
            BadgeTone::Accent,
            BadgeTone::Positive,
            BadgeTone::Warning,
            BadgeTone::Danger,
        ] {
            let light = badge_visual(tone, BadgeVariant::Surface, PrimitiveTheme::light());
            let dark = badge_visual(tone, BadgeVariant::Surface, PrimitiveTheme::dark());

            assert_ne!(light.fill, Color32::TRANSPARENT);
            assert_ne!(light.stroke, Color32::TRANSPARENT);
            assert_ne!(dark.fill, Color32::TRANSPARENT);
            assert_ne!(dark.stroke, Color32::TRANSPARENT);
            assert_ne!(light.fill, dark.fill);
            assert_ne!(light.text, dark.text);
        }
    }

    #[test]
    fn badge_size_reserves_icon_without_turning_status_into_a_control() {
        let text_only = primitive_badge_size(48.0, false);
        let with_icon = primitive_badge_size(48.0, true);

        assert_eq!(text_only.y, 22.0);
        assert_eq!(with_icon.y, 22.0);
        assert_eq!(with_icon.x - text_only.x, BADGE_ICON_SIZE + BADGE_ICON_GAP);
    }
}
