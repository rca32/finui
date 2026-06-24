use eframe::egui::{self, Color32, FontId, Rect, Response, Vec2};

use super::PrimitiveTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarImageStatus {
    Loading,
    Loaded,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AvatarImageOptions {
    pub status: AvatarImageStatus,
    pub fill: Color32,
    pub accent: Color32,
    pub theme: PrimitiveTheme,
}

impl Default for AvatarImageOptions {
    fn default() -> Self {
        let theme = PrimitiveTheme::default();
        Self {
            status: AvatarImageStatus::Loaded,
            fill: Color32::from_rgb(226, 242, 255),
            accent: Color32::from_rgb(41, 98, 255),
            theme,
        }
    }
}

impl AvatarImageOptions {
    pub fn status(mut self, status: AvatarImageStatus) -> Self {
        self.status = status;
        self
    }

    pub fn colors(mut self, fill: Color32, accent: Color32) -> Self {
        self.fill = fill;
        self.accent = accent;
        self
    }
}

pub struct AvatarOutput {
    pub response: Response,
    pub rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AvatarRootOptions {
    pub size: f32,
    pub theme: PrimitiveTheme,
}

impl Default for AvatarRootOptions {
    fn default() -> Self {
        Self {
            size: 32.0,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl AvatarRootOptions {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvatarRootOutput {
    pub rect: Rect,
    pub size: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvatarImageOutput {
    pub status: AvatarImageStatus,
    pub loaded: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvatarFallbackOptions {
    pub text: String,
    pub delay_ms: Option<u64>,
    pub elapsed_ms: u64,
    pub image_status: AvatarImageStatus,
    pub theme: PrimitiveTheme,
}

impl AvatarFallbackOptions {
    pub fn new(text: impl Into<String>, image_status: AvatarImageStatus) -> Self {
        Self {
            text: text.into(),
            delay_ms: None,
            elapsed_ms: 0,
            image_status,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = Some(delay_ms);
        self
    }

    pub fn elapsed_ms(mut self, elapsed_ms: u64) -> Self {
        self.elapsed_ms = elapsed_ms;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvatarFallbackOutput {
    pub text: String,
    pub initials: String,
    pub visible: bool,
    pub delayed: bool,
}

pub fn primitive_avatar(
    ui: &mut egui::Ui,
    fallback: &str,
    size: f32,
    theme: PrimitiveTheme,
) -> AvatarOutput {
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    let root =
        primitive_avatar_root_output(rect, AvatarRootOptions::default().size(size).theme(theme));
    primitive_avatar_root(ui, root.rect, theme);
    primitive_avatar_fallback(ui, rect, fallback, size, theme);
    AvatarOutput { response, rect }
}

pub fn primitive_avatar_root_output(rect: Rect, options: AvatarRootOptions) -> AvatarRootOutput {
    AvatarRootOutput {
        rect,
        size: options.size.max(1.0),
    }
}

pub fn primitive_avatar_image_output(options: AvatarImageOptions) -> AvatarImageOutput {
    AvatarImageOutput {
        status: options.status,
        loaded: options.status == AvatarImageStatus::Loaded,
    }
}

pub fn primitive_avatar_fallback_output(options: AvatarFallbackOptions) -> AvatarFallbackOutput {
    let image_loaded = options.image_status == AvatarImageStatus::Loaded;
    let delayed = options
        .delay_ms
        .is_some_and(|delay_ms| options.elapsed_ms < delay_ms);
    AvatarFallbackOutput {
        initials: avatar_fallback_text(&options.text),
        text: options.text,
        visible: !image_loaded && !delayed,
        delayed,
    }
}

pub fn primitive_avatar_root(ui: &egui::Ui, rect: Rect, theme: PrimitiveTheme) {
    let radius = avatar_radius(rect);
    ui.painter()
        .circle_filled(rect.center(), radius, theme.content_fill);
    ui.painter()
        .circle_stroke(rect.center(), radius, theme.content_stroke);
}

pub fn primitive_avatar_image(ui: &egui::Ui, rect: Rect, options: AvatarImageOptions) -> bool {
    let output = primitive_avatar_image_output(options);
    if !output.loaded {
        return false;
    }
    let radius = avatar_radius(rect);
    ui.painter()
        .circle_filled(rect.center(), radius, options.fill);
    let highlight = Rect::from_center_size(
        rect.center() + Vec2::new(radius * 0.18, -radius * 0.16),
        Vec2::splat(radius * 1.2),
    );
    ui.painter().circle_filled(
        highlight.center(),
        highlight.width() * 0.5,
        Color32::from_rgba_unmultiplied(
            options.accent.r(),
            options.accent.g(),
            options.accent.b(),
            42,
        ),
    );
    ui.painter()
        .circle_stroke(rect.center(), radius, options.theme.content_stroke);
    true
}

pub fn primitive_avatar_fallback(
    ui: &egui::Ui,
    rect: Rect,
    fallback: &str,
    size: f32,
    theme: PrimitiveTheme,
) {
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        avatar_fallback_text(fallback),
        crate::scaled_proportional_font(ui, (size * 0.38).max(10.0)),
        theme.text,
    );
}

pub fn avatar_fallback_text(text: &str) -> String {
    text.split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

pub fn avatar_radius(rect: Rect) -> f32 {
    rect.width().min(rect.height()) * 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar_fallback_uses_two_initials() {
        assert_eq!(avatar_fallback_text("Acme Holdings"), "AH");
        assert_eq!(avatar_fallback_text("EXG"), "E");
    }

    #[test]
    fn avatar_radius_uses_smallest_side() {
        let rect = Rect::from_min_size(egui::pos2(0.0, 0.0), Vec2::new(40.0, 24.0));

        assert_eq!(avatar_radius(rect), 12.0);
    }

    #[test]
    fn avatar_image_options_preserve_loading_status_and_colors() {
        let options = AvatarImageOptions::default()
            .status(AvatarImageStatus::Loading)
            .colors(Color32::BLACK, Color32::WHITE);

        assert_eq!(options.status, AvatarImageStatus::Loading);
        assert_eq!(options.fill, Color32::BLACK);
        assert_eq!(options.accent, Color32::WHITE);
    }

    #[test]
    fn avatar_image_status_fallback_states_are_not_loaded() {
        assert_ne!(AvatarImageStatus::Loading, AvatarImageStatus::Loaded);
        assert_ne!(AvatarImageStatus::Error, AvatarImageStatus::Loaded);
    }

    #[test]
    fn avatar_image_output_tracks_loaded_status() {
        let loaded = primitive_avatar_image_output(AvatarImageOptions::default());
        let error = primitive_avatar_image_output(
            AvatarImageOptions::default().status(AvatarImageStatus::Error),
        );

        assert!(loaded.loaded);
        assert_eq!(loaded.status, AvatarImageStatus::Loaded);
        assert!(!error.loaded);
        assert_eq!(error.status, AvatarImageStatus::Error);
    }

    #[test]
    fn avatar_fallback_output_respects_delay_and_loaded_image() {
        let delayed = primitive_avatar_fallback_output(
            AvatarFallbackOptions::new("Acme Holdings", AvatarImageStatus::Loading)
                .delay_ms(600)
                .elapsed_ms(120),
        );
        let visible = primitive_avatar_fallback_output(
            AvatarFallbackOptions::new("Acme Holdings", AvatarImageStatus::Error)
                .delay_ms(600)
                .elapsed_ms(600),
        );
        let hidden = primitive_avatar_fallback_output(AvatarFallbackOptions::new(
            "Loaded User",
            AvatarImageStatus::Loaded,
        ));

        assert_eq!(delayed.initials, "AH");
        assert!(delayed.delayed);
        assert!(!delayed.visible);
        assert!(!visible.delayed);
        assert!(visible.visible);
        assert!(!hidden.visible);
    }

    #[test]
    fn avatar_root_output_preserves_root_size_contract() {
        let rect = Rect::from_min_size(egui::pos2(0.0, 0.0), Vec2::new(40.0, 24.0));
        let output = primitive_avatar_root_output(rect, AvatarRootOptions::default().size(40.0));

        assert_eq!(output.rect, rect);
        assert_eq!(output.size, 40.0);
    }
}
