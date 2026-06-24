use eframe::egui;

use super::state::GridState;
use finui_primitives::{ThemeMode, radix_colors};

pub fn show_grid_status_bar(ui: &mut egui::Ui, state: &GridState) {
    let theme = grid_status_theme(ui);
    ui.vertical(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} visible columns",
                    state.visible_column_ids().len()
                ))
                .color(theme.text),
            );
            if let Some(row) = state.selection.selected_row.as_ref() {
                status_pill(ui, &format!("selected {}", row.0), theme.accent, theme);
            }
            for sort in &state.sort {
                let direction = match sort.direction {
                    super::state::GridSortDirection::Asc => "asc",
                    super::state::GridSortDirection::Desc => "desc",
                };
                status_pill(
                    ui,
                    &format!("sort {} {direction}", sort.column_id.0),
                    theme.accent,
                    theme,
                );
            }
            if state.filters.is_empty() {
                status_pill(ui, "filters clear", theme.muted_fill, theme);
            } else {
                for filter in &state.filters {
                    let invalid = is_invalid_filter_query(&filter.query);
                    status_pill(
                        ui,
                        &format!("filter {} {}", filter.column_id.0, filter.query),
                        if invalid { theme.warning } else { theme.accent },
                        theme,
                    );
                }
            }
            if let Some(range) = state.selection.selected_range.as_ref() {
                status_pill(
                    ui,
                    &format!(
                        "range {}:{} -> {}:{}",
                        range.anchor.row_id.0,
                        range.anchor.column_id.0,
                        range.focus.row_id.0,
                        range.focus.column_id.0
                    ),
                    theme.accent,
                    theme,
                );
            }
            if let Some(edit) = state.edit_draft.as_ref() {
                let label = if edit.read_only {
                    "edit read-only"
                } else if edit.valid {
                    "edit draft"
                } else {
                    "edit invalid"
                };
                status_pill(
                    ui,
                    label,
                    if edit.valid {
                        theme.accent
                    } else {
                        theme.warning
                    },
                    theme,
                );
            }
        });
    });
}

#[derive(Clone, Copy)]
struct GridStatusTheme {
    text: egui::Color32,
    muted_text: egui::Color32,
    muted_fill: egui::Color32,
    accent: egui::Color32,
    warning: egui::Color32,
    stroke: egui::Stroke,
}

fn grid_status_theme(ui: &egui::Ui) -> GridStatusTheme {
    if ui.visuals().dark_mode {
        GridStatusTheme {
            text: egui::Color32::from_rgb(0xed, 0xf1, 0xf7),
            muted_text: egui::Color32::from_rgb(0xac, 0xb4, 0xc2),
            muted_fill: egui::Color32::from_rgb(0x22, 0x28, 0x32),
            accent: egui::Color32::from_rgb(0x25, 0x36, 0x68),
            warning: egui::Color32::from_rgb(0x5a, 0x3d, 0x14),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(0x36, 0x3d, 0x49)),
        }
    } else {
        let _mode = ThemeMode::Light;
        GridStatusTheme {
            text: radix_colors::SLATE_12,
            muted_text: radix_colors::SLATE_11,
            muted_fill: radix_colors::SLATE_3,
            accent: radix_colors::INDIGO_3,
            warning: egui::Color32::from_rgb(0xff, 0xf7, 0xd6),
            stroke: egui::Stroke::new(1.0, radix_colors::SLATE_6),
        }
    }
}

fn status_pill(ui: &mut egui::Ui, label: &str, fill: egui::Color32, theme: GridStatusTheme) {
    let text = egui::RichText::new(label)
        .color(theme.muted_text)
        .size(12.0);
    egui::Frame::new()
        .fill(fill)
        .stroke(theme.stroke)
        .corner_radius(egui::CornerRadius::same(5))
        .inner_margin(egui::Margin::symmetric(7, 3))
        .show(ui, |ui| {
            ui.label(text);
        });
}

fn is_invalid_filter_query(query: &str) -> bool {
    let query = query.trim();
    query.eq_ignore_ascii_case("not-a-number") || query.eq_ignore_ascii_case("nan")
}
