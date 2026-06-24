use eframe::egui::{Color32, Stroke};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub const fn options() -> &'static [Self] {
        &[Self::Light, Self::Dark]
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
        }
    }

    pub fn from_id(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "light" | "day" => Some(Self::Light),
            "dark" | "night" => Some(Self::Dark),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveTheme {
    pub content_fill: Color32,
    pub content_stroke: Stroke,
    pub item_hover_fill: Color32,
    pub item_selected_fill: Color32,
    pub text: Color32,
    pub muted_text: Color32,
    pub disabled_text: Color32,
    pub radius: u8,
    pub row_radius: f32,
    pub menu_row_height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveContentTextColors {
    pub title: Color32,
    pub detail: Color32,
}

pub fn primitive_mounted_content_text_colors(
    open: bool,
    theme: PrimitiveTheme,
) -> PrimitiveContentTextColors {
    if open {
        PrimitiveContentTextColors {
            title: theme.text,
            detail: theme.muted_text,
        }
    } else {
        PrimitiveContentTextColors {
            title: theme.muted_text,
            detail: theme.disabled_text,
        }
    }
}

pub mod radix_colors {
    use eframe::egui::Color32;

    pub const SLATE_1: Color32 = Color32::from_rgb(0xfc, 0xfc, 0xfd);
    pub const SLATE_2: Color32 = Color32::from_rgb(0xf9, 0xf9, 0xfb);
    pub const SLATE_3: Color32 = Color32::from_rgb(0xf0, 0xf0, 0xf3);
    pub const SLATE_4: Color32 = Color32::from_rgb(0xe8, 0xe8, 0xec);
    pub const SLATE_5: Color32 = Color32::from_rgb(0xe0, 0xe1, 0xe6);
    pub const SLATE_6: Color32 = Color32::from_rgb(0xd9, 0xd9, 0xe0);
    pub const SLATE_7: Color32 = Color32::from_rgb(0xcd, 0xce, 0xd6);
    pub const SLATE_8: Color32 = Color32::from_rgb(0xb9, 0xbb, 0xc6);
    pub const SLATE_9: Color32 = Color32::from_rgb(0x8b, 0x8d, 0x98);
    pub const SLATE_10: Color32 = Color32::from_rgb(0x80, 0x83, 0x8d);
    pub const SLATE_11: Color32 = Color32::from_rgb(0x60, 0x64, 0x6c);
    pub const SLATE_12: Color32 = Color32::from_rgb(0x1c, 0x20, 0x24);

    pub const INDIGO_2: Color32 = Color32::from_rgb(0xf7, 0xf9, 0xff);
    pub const INDIGO_3: Color32 = Color32::from_rgb(0xed, 0xf2, 0xfe);
    pub const INDIGO_4: Color32 = Color32::from_rgb(0xe1, 0xe9, 0xff);
    pub const INDIGO_7: Color32 = Color32::from_rgb(0xab, 0xbd, 0xf9);
    pub const INDIGO_8: Color32 = Color32::from_rgb(0x8d, 0xa4, 0xef);
    pub const INDIGO_9: Color32 = Color32::from_rgb(0x3e, 0x63, 0xdd);
    pub const INDIGO_11: Color32 = Color32::from_rgb(0x3a, 0x5b, 0xc7);

    pub const GREEN_9: Color32 = Color32::from_rgb(0x30, 0xa4, 0x6c);
    pub const AMBER_9: Color32 = Color32::from_rgb(0xff, 0xc5, 0x3d);
    pub const TOMATO_9: Color32 = Color32::from_rgb(0xe5, 0x4d, 0x2e);
}

impl Default for PrimitiveTheme {
    fn default() -> Self {
        Self::light()
    }
}

impl PrimitiveTheme {
    pub fn light() -> Self {
        Self {
            content_fill: radix_colors::SLATE_1,
            content_stroke: Stroke::new(1.0, radix_colors::SLATE_6),
            item_hover_fill: radix_colors::SLATE_3,
            item_selected_fill: radix_colors::INDIGO_3,
            text: radix_colors::SLATE_12,
            muted_text: radix_colors::SLATE_11,
            disabled_text: radix_colors::SLATE_8,
            radius: 6,
            row_radius: 5.0,
            menu_row_height: 32.0,
        }
    }

    pub fn dark() -> Self {
        Self {
            content_fill: Color32::from_rgb(0x14, 0x18, 0x1f),
            content_stroke: Stroke::new(1.0, Color32::from_rgb(0x36, 0x3d, 0x49)),
            item_hover_fill: Color32::from_rgb(0x1f, 0x25, 0x2e),
            item_selected_fill: Color32::from_rgb(0x20, 0x2d, 0x56),
            text: Color32::from_rgb(0xed, 0xf1, 0xf7),
            muted_text: Color32::from_rgb(0xac, 0xb4, 0xc2),
            disabled_text: Color32::from_rgb(0x6d, 0x75, 0x84),
            radius: 6,
            row_radius: 5.0,
            menu_row_height: 32.0,
        }
    }

    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mounted_content_text_colors_mute_closed_force_mounted_content() {
        let theme = PrimitiveTheme::light();

        assert_eq!(
            primitive_mounted_content_text_colors(true, theme),
            PrimitiveContentTextColors {
                title: theme.text,
                detail: theme.muted_text,
            }
        );
        assert_eq!(
            primitive_mounted_content_text_colors(false, theme),
            PrimitiveContentTextColors {
                title: theme.muted_text,
                detail: theme.disabled_text,
            }
        );
    }

    #[test]
    fn default_theme_remains_light_and_dark_theme_has_contrast() {
        let light = PrimitiveTheme::default();
        let dark = PrimitiveTheme::dark();

        assert_eq!(light, PrimitiveTheme::light());
        assert_ne!(light.content_fill, dark.content_fill);
        assert!(dark.text.r() > dark.content_fill.r());
        assert_ne!(dark.content_stroke.color, dark.content_fill);
        assert_eq!(PrimitiveTheme::for_mode(ThemeMode::Dark), dark);
    }
}
