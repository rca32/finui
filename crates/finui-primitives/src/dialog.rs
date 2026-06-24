use std::hash::Hash;

use eframe::egui::{self, Color32, Stroke, Vec2};

use super::{AnchoredLayerOptions, DismissPolicy, LayerPlacement, modal_backdrop};
use crate::config::{TV_LIGHT, tv_theme_for_dark_mode};

pub struct CommandDialogOptions {
    pub id: egui::Id,
    pub backdrop_id: egui::Id,
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub top_margin: f32,
    pub inner_margin: egui::Margin,
    pub backdrop_tint: Color32,
    pub fill: Color32,
    pub stroke: egui::Stroke,
    pub radius: u8,
}

impl CommandDialogOptions {
    pub fn new(id: impl Hash) -> Self {
        Self {
            id: egui::Id::new(&id),
            backdrop_id: egui::Id::new((id, "backdrop")),
            min_width: 560.0,
            max_width: 1_050.0,
            min_height: 520.0,
            max_height: 770.0,
            top_margin: 24.0,
            inner_margin: egui::Margin::same(24),
            backdrop_tint: Color32::from_rgba_unmultiplied(0, 0, 0, 48),
            fill: TV_LIGHT.popup_background,
            stroke: Stroke::new(1.0, TV_LIGHT.toolbar_border),
            radius: 6,
        }
    }

    pub fn size(
        mut self,
        min_width: f32,
        max_width: f32,
        min_height: f32,
        max_height: f32,
    ) -> Self {
        self.min_width = min_width;
        self.max_width = max_width;
        self.min_height = min_height;
        self.max_height = max_height;
        self
    }

    pub fn top_margin(mut self, top_margin: f32) -> Self {
        self.top_margin = top_margin;
        self
    }

    pub fn inner_margin(mut self, inner_margin: egui::Margin) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    pub fn backdrop_tint(mut self, backdrop_tint: Color32) -> Self {
        self.backdrop_tint = backdrop_tint;
        self
    }

    pub fn panel_style(mut self, fill: Color32, stroke: Stroke, radius: u8) -> Self {
        self.fill = fill;
        self.stroke = stroke;
        self.radius = radius;
        self
    }
}

pub struct CommandDialogOutput<T> {
    pub action: Option<T>,
    pub should_close: bool,
}

pub fn show_command_dialog<T>(
    ctx: &egui::Context,
    mut options: CommandDialogOptions,
    add_contents: impl FnOnce(&mut egui::Ui, Vec2) -> Option<T>,
) -> CommandDialogOutput<T> {
    if ctx.global_style().visuals.dark_mode
        && options.fill == TV_LIGHT.popup_background
        && options.stroke.color == TV_LIGHT.toolbar_border
    {
        let tv = tv_theme_for_dark_mode(true);
        options.fill = tv.popup_background;
        options.stroke = Stroke::new(1.0, tv.toolbar_border);
    }
    let screen = ctx.content_rect();
    let panel_size = compute_command_dialog_panel_size(
        screen.size(),
        options.min_width,
        options.max_width,
        options.min_height,
        options.max_height,
        options.top_margin,
    );
    let content_size = Vec2::new(
        panel_size.x - f32::from(options.inner_margin.left + options.inner_margin.right),
        panel_size.y - f32::from(options.inner_margin.top + options.inner_margin.bottom),
    );
    let backdrop = modal_backdrop(ctx, options.backdrop_id, options.backdrop_tint);
    let output = super::show_anchored_layer(
        ctx,
        {
            let mut layer = AnchoredLayerOptions::new(options.id, content_size.x)
                .placement(LayerPlacement::Centered {
                    top_margin: options.top_margin,
                })
                .min_height(content_size.y)
                .max_height(content_size.y)
                .inner_margin(options.inner_margin)
                .dismiss_policy(DismissPolicy::EscapeOnly)
                .order(egui::Order::Tooltip);
            layer.fill = options.fill;
            layer.stroke = options.stroke;
            layer.radius = options.radius;
            layer
        },
        |ui| {
            ui.set_min_size(content_size);
            ui.set_max_size(content_size);
            add_contents(ui, content_size)
        },
    );
    let should_close = backdrop.clicked()
        || output.should_close
        || ctx.input(|input| input.key_pressed(egui::Key::Escape));

    CommandDialogOutput {
        action: output.action,
        should_close,
    }
}

fn compute_command_dialog_panel_size(
    screen_size: Vec2,
    min_width: f32,
    max_width: f32,
    min_height: f32,
    max_height: f32,
    top_margin: f32,
) -> Vec2 {
    Vec2::new(
        screen_size.x.min(max_width).max(min_width),
        (screen_size.y - top_margin * 2.0).clamp(min_height, max_height),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_dialog_size_uses_viewport_constraints() {
        let size = compute_command_dialog_panel_size(
            Vec2::new(900.0, 640.0),
            560.0,
            1050.0,
            520.0,
            770.0,
            24.0,
        );

        assert_eq!(size.x, 900.0);
        assert_eq!(size.y, 592.0);
    }

    #[test]
    fn command_dialog_size_respects_minimums_on_small_viewports() {
        let size = compute_command_dialog_panel_size(
            Vec2::new(420.0, 360.0),
            560.0,
            1050.0,
            520.0,
            770.0,
            24.0,
        );

        assert_eq!(size.x, 560.0);
        assert_eq!(size.y, 520.0);
    }

    #[test]
    fn command_dialog_panel_style_overrides_light_defaults() {
        let fill = Color32::from_rgb(20, 24, 31);
        let stroke = Stroke::new(1.0, Color32::from_rgb(62, 71, 86));
        let options = CommandDialogOptions::new("dialog-style-test").panel_style(fill, stroke, 7);

        assert_eq!(options.fill, fill);
        assert_eq!(options.stroke, stroke);
        assert_eq!(options.radius, 7);
    }
}
