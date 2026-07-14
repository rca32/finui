use egui::{self, Color32, Stroke};

use super::{
    DataListItemOutput, DataListLabelOutput, DataListRootOutput, DataListValueOutput,
    PrimitiveTheme,
};
use crate::paint_radix_icon;

pub fn primitive_data_list_root(ui: &egui::Ui, output: &DataListRootOutput, theme: PrimitiveTheme) {
    if output.visual.fill != Color32::TRANSPARENT {
        ui.painter()
            .rect_filled(output.rect, theme.radius, output.visual.fill);
    }
    if output.visual.stroke != Color32::TRANSPARENT {
        ui.painter().rect_stroke(
            output.rect,
            theme.radius,
            Stroke::new(1.0, output.visual.stroke),
            egui::StrokeKind::Inside,
        );
    }
}

pub fn primitive_data_list_item(ui: &egui::Ui, output: &DataListItemOutput, color: Color32) {
    if let Some(divider) = output.divider_rect {
        ui.painter().rect_filled(divider, 0.0, color);
    }
}

pub fn primitive_data_list_label(ui: &egui::Ui, output: &DataListLabelOutput) {
    let text = ellipsize_to_width(
        ui,
        output.text.as_str(),
        output.font_size,
        output.rect.width(),
    );
    ui.painter().with_clip_rect(output.rect).text(
        output.text_pos,
        egui::Align2::LEFT_CENTER,
        text,
        crate::scaled_proportional_font(ui, output.font_size),
        output.color,
    );
}

pub fn primitive_data_list_value(ui: &egui::Ui, output: &DataListValueOutput) {
    if let Some(icon) = output.icon {
        paint_radix_icon(ui, icon.icon, icon.rect, icon.color);
    }
    let text = ellipsize_to_width(
        ui,
        output.text.as_str(),
        output.font_size,
        output.text_rect.width(),
    );
    ui.painter().with_clip_rect(output.text_rect).text(
        output.text_pos,
        egui::Align2::LEFT_CENTER,
        text,
        crate::scaled_proportional_font(ui, output.font_size),
        output.color,
    );
}

fn ellipsize_to_width(ui: &egui::Ui, value: &str, font_size: f32, max_width: f32) -> String {
    if value.is_empty() || max_width <= 0.0 {
        return String::new();
    }
    let font = crate::scaled_proportional_font(ui, font_size);
    let text_width = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_string(), font.clone(), Color32::WHITE)
            .size()
            .x
    };
    if text_width(value) <= max_width {
        return value.to_string();
    }
    let suffix = "...";
    if text_width(suffix) > max_width {
        return String::new();
    }
    let chars = value.chars().collect::<Vec<_>>();
    let mut low = 0;
    let mut high = chars.len();
    while low < high {
        let mid = (low + high + 1) / 2;
        let candidate = chars[..mid].iter().collect::<String>() + suffix;
        if text_width(candidate.as_str()) <= max_width {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    chars[..low].iter().collect::<String>() + suffix
}
