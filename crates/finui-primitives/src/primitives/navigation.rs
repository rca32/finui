use std::hash::Hash;

use eframe::egui::{self, Color32, FontId, Rect, Response, Vec2};

use super::{PrimitiveTheme, RovingFocusState, radix_colors};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabItem {
    pub label: &'static str,
    pub enabled: bool,
}

pub struct TabsHeaderOutput {
    pub response: Option<Response>,
    pub changed: bool,
}

pub struct TabsContentOutput<T> {
    pub inner: T,
    pub rect: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabsTriggerState {
    pub active: bool,
    pub enabled: bool,
    pub hovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TabsContentOptions {
    pub min_height: f32,
    pub inner_margin: egui::Margin,
    pub theme: PrimitiveTheme,
}

impl Default for TabsContentOptions {
    fn default() -> Self {
        Self {
            min_height: 0.0,
            inner_margin: egui::Margin::symmetric(10, 8),
            theme: PrimitiveTheme::default(),
        }
    }
}

impl TabsContentOptions {
    pub fn min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height;
        self
    }

    pub fn inner_margin(mut self, inner_margin: egui::Margin) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

pub fn primitive_tabs_header(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    selected: &mut usize,
    focus: &mut RovingFocusState,
    items: &[TabItem],
    theme: PrimitiveTheme,
) -> TabsHeaderOutput {
    let enabled = items.iter().map(|item| item.enabled).collect::<Vec<_>>();
    let _ = ui.input(|input| focus.handle_keyboard(input, &enabled));
    if let Some(index) = focus.active_index.filter(|index| *index < items.len()) {
        if enabled[index] {
            *selected = index;
        }
    }

    let bounds = ui.available_rect_before_wrap();
    let rects = tab_rects(bounds, items);
    let mut changed = false;
    let mut last_response = None;
    for (index, (item, rect)) in items.iter().zip(rects.into_iter()).enumerate() {
        let mut response = ui.interact(
            rect,
            ui.id().with((&id_source, index)),
            if item.enabled {
                egui::Sense::click()
            } else {
                egui::Sense::hover()
            },
        );
        if item.enabled {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }
        if response.clicked() && item.enabled && *selected != index {
            *selected = index;
            focus.active_index = Some(index);
            changed = true;
        }
        primitive_tabs_trigger(
            ui,
            rect,
            item.label,
            TabsTriggerState {
                active: *selected == index,
                enabled: item.enabled,
                hovered: response.hovered(),
            },
            theme,
        );
        last_response = Some(response);
    }
    if let Some(last) = primitive_tabs_list_rect(bounds, items) {
        ui.allocate_rect(last, egui::Sense::hover());
    }
    TabsHeaderOutput {
        response: last_response,
        changed,
    }
}

pub fn primitive_tabs_content<T>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    options: TabsContentOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> T,
) -> TabsContentOutput<T> {
    let frame = egui::Frame::new()
        .fill(options.theme.content_fill)
        .stroke(options.theme.content_stroke)
        .corner_radius(options.theme.row_radius)
        .inner_margin(options.inner_margin);
    let output = frame.show(ui, |ui| {
        ui.set_min_height(options.min_height);
        add_contents(ui)
    });
    let response = ui.interact(
        output.response.rect,
        ui.id().with(id_source),
        egui::Sense::hover(),
    );
    TabsContentOutput {
        inner: output.inner,
        rect: response.rect,
    }
}

pub fn primitive_tabs_list_rect(bounds: Rect, items: &[TabItem]) -> Option<Rect> {
    let rects = tab_rects(bounds, items);
    let first = rects.first()?;
    let mut list = *first;
    for rect in rects.iter().skip(1) {
        list = list.union(*rect);
    }
    Some(list)
}

pub fn tabs_index_by_label(items: &[TabItem], query: &str) -> Option<usize> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(index) = trimmed.parse::<usize>() {
        return items.get(index).filter(|item| item.enabled).map(|_| index);
    }

    let query_key = tab_label_key(trimmed);
    items.iter().enumerate().find_map(|(index, item)| {
        if !item.enabled {
            return None;
        }
        let label_key = tab_label_key(item.label);
        let singular_label_key = label_key.strip_suffix('s').unwrap_or(&label_key);
        (label_key == query_key
            || singular_label_key == query_key
            || label_key.ends_with(&query_key))
        .then_some(index)
    })
}

pub fn primitive_tabs_trigger(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    state: TabsTriggerState,
    theme: PrimitiveTheme,
) {
    let chip_rect = rect.shrink2(Vec2::new(2.0, 3.0));
    let fill = if state.active {
        theme.content_fill
    } else if state.hovered && state.enabled {
        theme.item_hover_fill
    } else {
        Color32::TRANSPARENT
    };
    let stroke = if state.active {
        theme.content_stroke
    } else {
        egui::Stroke::NONE
    };
    ui.painter().rect(
        chip_rect,
        theme.row_radius,
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    if state.active {
        let underline = Rect::from_min_max(
            chip_rect.left_bottom() + Vec2::new(8.0, -2.0),
            chip_rect.right_bottom() - Vec2::new(8.0, 0.0),
        );
        ui.painter()
            .rect_filled(underline, 2.0, radix_colors::INDIGO_9);
    }
    ui.painter().text(
        chip_rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        if state.active {
            theme.text
        } else if state.enabled {
            theme.text
        } else {
            theme.disabled_text
        },
    );
}

pub fn tab_rects(bounds: Rect, items: &[TabItem]) -> Vec<Rect> {
    let mut x = bounds.left();
    let mut y = bounds.top();
    let right = bounds.right().max(bounds.left() + 1.0);
    items
        .iter()
        .map(|item| {
            let width = tab_width(item.label);
            if x > bounds.left() && x + width > right {
                x = bounds.left();
                y += 30.0;
            }
            let rect = Rect::from_min_size(egui::pos2(x, y), Vec2::new(width, 30.0));
            x += width;
            rect
        })
        .collect()
}

fn tab_width(label: &str) -> f32 {
    (label.chars().count() as f32 * 8.0 + 24.0).max(54.0)
}

fn tab_label_key(label: &str) -> String {
    label
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_lowercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_rects_keep_order_and_minimum_width() {
        let rects = tab_rects(
            Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(400.0, 30.0)),
            &[
                TabItem {
                    label: "A",
                    enabled: true,
                },
                TabItem {
                    label: "Long Tab",
                    enabled: true,
                },
            ],
        );

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].left(), 10.0);
        assert!(rects[0].width() >= 54.0);
        assert_eq!(rects[1].left(), rects[0].right());
    }

    #[test]
    fn tabs_list_rect_covers_all_trigger_widths() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 4.0), Vec2::new(400.0, 30.0));
        let items = [
            TabItem {
                label: "A",
                enabled: true,
            },
            TabItem {
                label: "Long Tab",
                enabled: true,
            },
        ];
        let rects = tab_rects(bounds, &items);
        let list = primitive_tabs_list_rect(bounds, &items).expect("non-empty tab list");

        assert_eq!(list.left(), bounds.left());
        assert_eq!(list.right(), rects.last().unwrap().right());
        assert_eq!(list.height(), 30.0);
    }

    #[test]
    fn tabs_index_by_label_matches_normalized_labels_and_numeric_indices() {
        let items = [
            TabItem {
                label: "Controls",
                enabled: true,
            },
            TabItem {
                label: "Overlays",
                enabled: true,
            },
            TabItem {
                label: "New Parts",
                enabled: true,
            },
        ];

        assert_eq!(tabs_index_by_label(&items, "control"), Some(0));
        assert_eq!(tabs_index_by_label(&items, "overlay"), Some(1));
        assert_eq!(tabs_index_by_label(&items, "new_parts"), Some(2));
        assert_eq!(tabs_index_by_label(&items, "newparts"), Some(2));
        assert_eq!(tabs_index_by_label(&items, "parts"), Some(2));
        assert_eq!(tabs_index_by_label(&items, "2"), Some(2));
    }

    #[test]
    fn tabs_index_by_label_skips_disabled_or_unknown_tabs() {
        let items = [
            TabItem {
                label: "Controls",
                enabled: true,
            },
            TabItem {
                label: "Overlays",
                enabled: false,
            },
        ];

        assert_eq!(tabs_index_by_label(&items, "overlays"), None);
        assert_eq!(tabs_index_by_label(&items, "1"), None);
        assert_eq!(tabs_index_by_label(&items, "missing"), None);
    }

    #[test]
    fn tabs_content_options_preserve_part_contract() {
        let options = TabsContentOptions::default()
            .min_height(120.0)
            .inner_margin(egui::Margin::symmetric(6, 7));

        assert_eq!(options.min_height, 120.0);
        assert_eq!(options.inner_margin.left, 6);
        assert_eq!(options.inner_margin.top, 7);
    }
}
