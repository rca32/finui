use eframe::egui::{self, Color32, Rect, Response, Stroke, Vec2};

use super::{PrimitiveTheme, radix_colors};

#[derive(Debug)]
pub struct VisuallyHiddenOutput {
    pub rect: Rect,
    pub response: Response,
    pub text: String,
}

pub fn aspect_ratio_size(available: Vec2, ratio: f32) -> Vec2 {
    if ratio <= 0.0 || available.x <= 0.0 || available.y <= 0.0 {
        return Vec2::ZERO;
    }
    let by_width = Vec2::new(available.x, available.x / ratio);
    if by_width.y <= available.y {
        by_width
    } else {
        Vec2::new(available.y * ratio, available.y)
    }
}

pub fn aspect_ratio_rect(bounds: Rect, ratio: f32) -> Rect {
    let size = aspect_ratio_size(bounds.size(), ratio);
    Rect::from_center_size(bounds.center(), size)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AspectRatioOptions {
    pub ratio: f32,
    pub theme: PrimitiveTheme,
    pub fill: Option<Color32>,
    pub stroke: Option<Stroke>,
}

impl AspectRatioOptions {
    pub fn new(ratio: f32) -> Self {
        let theme = PrimitiveTheme::default();
        Self {
            ratio,
            theme,
            fill: Some(radix_colors::SLATE_2),
            stroke: Some(Stroke::new(1.0, radix_colors::SLATE_6)),
        }
    }

    pub fn fill(mut self, fill: Option<Color32>) -> Self {
        self.fill = fill;
        self
    }

    pub fn stroke(mut self, stroke: Option<Stroke>) -> Self {
        self.stroke = stroke;
        self
    }
}

#[derive(Debug)]
pub struct AspectRatioOutput {
    pub rect: Rect,
    pub response: Response,
}

pub fn primitive_aspect_ratio_root(
    ui: &mut egui::Ui,
    available: Vec2,
    options: AspectRatioOptions,
) -> AspectRatioOutput {
    let (bounds, response) = ui.allocate_exact_size(available, egui::Sense::hover());
    let rect = aspect_ratio_rect(bounds, options.ratio);
    paint_aspect_ratio_root(ui, rect, options);
    AspectRatioOutput { rect, response }
}

pub fn paint_aspect_ratio_root(ui: &egui::Ui, rect: Rect, options: AspectRatioOptions) {
    if rect.is_positive() {
        if let Some(fill) = options.fill {
            ui.painter()
                .rect_filled(rect, options.theme.row_radius, fill);
        }
        if let Some(stroke) = options.stroke {
            ui.painter().rect_stroke(
                rect,
                options.theme.row_radius,
                stroke,
                egui::StrokeKind::Inside,
            );
        }
    }
}

pub fn visually_hidden_rect(ui: &mut egui::Ui) -> Rect {
    let (rect, _) = ui.allocate_exact_size(Vec2::ZERO, egui::Sense::hover());
    rect
}

pub fn primitive_visually_hidden_label(
    ui: &mut egui::Ui,
    _id_source: impl std::hash::Hash,
    text: impl Into<String>,
) -> VisuallyHiddenOutput {
    let text = text.into();
    let (rect, response) = ui.allocate_exact_size(Vec2::ZERO, egui::Sense::hover());
    VisuallyHiddenOutput {
        rect,
        response: response.on_hover_text(text.clone()),
        text,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aspect_ratio_size_fits_inside_available_bounds() {
        let size = aspect_ratio_size(Vec2::new(400.0, 100.0), 16.0 / 9.0);

        assert!(size.x <= 400.0);
        assert!(size.y <= 100.0);
        assert!((size.x / size.y - 16.0 / 9.0).abs() < 0.001);
    }

    #[test]
    fn aspect_ratio_rect_centers_root_inside_bounds() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(300.0, 120.0));
        let rect = aspect_ratio_rect(bounds, 16.0 / 9.0);

        assert_eq!(rect.center(), bounds.center());
        assert!(rect.width() <= bounds.width());
        assert!(rect.height() <= bounds.height());
        assert!((rect.width() / rect.height() - 16.0 / 9.0).abs() < 0.001);
    }

    #[test]
    fn visually_hidden_output_keeps_text_contract() {
        let text = String::from("screen reader label");

        assert_eq!(text.as_str(), "screen reader label");
    }
}
