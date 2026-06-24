use eframe::egui::{self, RichText, Stroke};

use super::{RadixIcon, paint_radix_icon, theme::PrimitiveTheme};

#[derive(Debug, Clone, Copy)]
pub struct PrimitiveSettingsRowOptions<'a> {
    pub id: egui::Id,
    pub label: &'a str,
    pub description: Option<&'a str>,
    pub enabled: bool,
    pub selected: bool,
    pub icon: Option<RadixIcon>,
    pub theme: PrimitiveTheme,
}

impl<'a> PrimitiveSettingsRowOptions<'a> {
    pub fn new(id: impl std::hash::Hash, label: &'a str, theme: PrimitiveTheme) -> Self {
        Self {
            id: egui::Id::new(id),
            label,
            description: None,
            enabled: true,
            selected: false,
            icon: None,
            theme,
        }
    }

    pub fn description(mut self, description: Option<&'a str>) -> Self {
        self.description = description;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn icon(mut self, icon: RadixIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveSettingsRowOutput {
    pub clicked: bool,
    pub changed: bool,
    pub hovered: bool,
    pub rect: egui::Rect,
}

pub fn primitive_settings_nav_row(
    ui: &mut egui::Ui,
    options: PrimitiveSettingsRowOptions<'_>,
) -> PrimitiveSettingsRowOutput {
    let height = 34.0;
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::click(),
    );
    let visuals = ui.style().interact_selectable(&response, options.selected);
    let fill = if options.selected {
        options.theme.item_selected_fill
    } else if response.hovered() {
        options.theme.item_hover_fill
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, 5.0, fill);
    let text_color = if options.selected {
        ui.visuals().selection.stroke.color
    } else {
        visuals.text_color()
    };
    let label_x = if let Some(icon) = options.icon {
        let icon_rect = egui::Rect::from_center_size(
            rect.left_center() + egui::vec2(17.0, 0.0),
            egui::vec2(18.0, 18.0),
        );
        paint_radix_icon(ui, icon, icon_rect, text_color);
        36.0
    } else {
        12.0
    };
    ui.painter().text(
        rect.left_center() + egui::vec2(label_x, 0.0),
        egui::Align2::LEFT_CENTER,
        options.label,
        egui::FontId::proportional(13.0),
        text_color,
    );
    PrimitiveSettingsRowOutput {
        clicked: response.clicked(),
        changed: false,
        hovered: response.hovered(),
        rect,
    }
}

pub fn primitive_settings_row<R>(
    ui: &mut egui::Ui,
    options: PrimitiveSettingsRowOptions<'_>,
    add_control: impl FnOnce(&mut egui::Ui) -> R,
) -> (PrimitiveSettingsRowOutput, R) {
    let frame = egui::Frame::default()
        .fill(if options.selected {
            options.theme.item_selected_fill
        } else {
            egui::Color32::TRANSPARENT
        })
        .inner_margin(egui::Margin::symmetric(10, 8))
        .corner_radius(6.0)
        .stroke(Stroke::new(
            1.0,
            if options.selected {
                options.theme.content_stroke.color
            } else {
                egui::Color32::TRANSPARENT
            },
        ));
    let frame_response = frame.show(ui, |ui| {
        ui.add_enabled_ui(options.enabled, |ui| {
            ui.horizontal(|ui| {
                ui.set_min_height(38.0);
                ui.vertical(|ui| {
                    ui.set_width((ui.available_width() - 230.0).max(180.0));
                    ui.label(
                        RichText::new(options.label)
                            .size(13.0)
                            .color(options.theme.text),
                    );
                    if let Some(description) = options.description {
                        ui.label(
                            RichText::new(description)
                                .size(11.0)
                                .color(options.theme.muted_text),
                        );
                    }
                });
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    add_control,
                )
                .inner
            })
            .inner
        })
        .inner
    });
    let row_rect = frame_response.response.rect;
    let inner = frame_response.inner;
    let row_click_rect = egui::Rect::from_min_max(
        row_rect.min,
        egui::pos2(
            (row_rect.right() - 230.0).max(row_rect.left()),
            row_rect.bottom(),
        ),
    );
    let row_response = ui.interact(
        row_click_rect,
        options.id.with("row_response"),
        egui::Sense::click(),
    );
    (
        PrimitiveSettingsRowOutput {
            clicked: row_response.clicked(),
            changed: false,
            hovered: row_response.hovered(),
            rect: row_rect,
        },
        inner,
    )
}

pub fn primitive_color_swatch(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    color: egui::Color32,
    selected: bool,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(30.0, 22.0), egui::Sense::click());
    let stroke = if selected {
        Stroke::new(2.0, ui.visuals().selection.stroke.color)
    } else {
        Stroke::new(1.0, ui.visuals().widgets.inactive.bg_stroke.color)
    };
    ui.painter().rect_filled(rect.shrink(3.0), 4.0, color);
    ui.painter()
        .rect_stroke(rect.shrink(3.0), 4.0, stroke, egui::StrokeKind::Inside);
    response
        .on_hover_text(tooltip.to_owned())
        .union(ui.interact(rect, egui::Id::new(id), egui::Sense::hover()))
}

pub const PRIMITIVE_COLOR_PICKER_REFERENCE_PNG: &[u8] = include_bytes!("색상선택.png");

pub fn primitive_color_picker_reference_bytes() -> &'static [u8] {
    PRIMITIVE_COLOR_PICKER_REFERENCE_PNG
}

#[derive(Debug, Clone, Copy)]
pub struct PrimitiveColorPickerLabels {
    pub custom: &'static str,
    pub opacity: &'static str,
    pub current: &'static str,
}

impl Default for PrimitiveColorPickerLabels {
    fn default() -> Self {
        Self {
            custom: "Custom color",
            opacity: "Opacity",
            current: "Current color",
        }
    }
}

pub fn primitive_color_picker_field(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    current: egui::Color32,
    labels: PrimitiveColorPickerLabels,
) -> Option<egui::Color32> {
    primitive_color_picker_field_with_open(ui, id, current, labels, false)
}

pub fn primitive_color_picker_field_with_open(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    current: egui::Color32,
    labels: PrimitiveColorPickerLabels,
    force_open: bool,
) -> Option<egui::Color32> {
    let id = egui::Id::new(id);
    let mut selected = None;
    let trigger = primitive_color_picker_trigger(ui, id, current, labels.current);
    let open_command = if force_open {
        Some(egui::SetOpenCommand::Bool(true))
    } else if trigger.clicked() {
        Some(egui::SetOpenCommand::Toggle)
    } else {
        None
    };
    egui::Popup::from_toggle_button_response(&trigger)
        .id(id)
        .open_memory(open_command)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            selected = primitive_color_picker_panel(ui, id, current, labels);
        });
    selected
}

#[derive(Debug, Clone)]
pub struct PrimitiveCrosshairStylePickerOutput {
    pub color: Option<egui::Color32>,
    pub line_width: Option<u8>,
    pub line_style: Option<String>,
}

pub fn primitive_crosshair_style_picker_with_open(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    current_color: egui::Color32,
    current_line_width: u8,
    current_line_style: &str,
    line_style_options: &[PrimitiveSelectOption],
    labels: PrimitiveColorPickerLabels,
    force_open: bool,
) -> PrimitiveCrosshairStylePickerOutput {
    let id = egui::Id::new(id);
    let mut output = PrimitiveCrosshairStylePickerOutput {
        color: None,
        line_width: None,
        line_style: None,
    };
    let trigger = primitive_crosshair_style_trigger(
        ui,
        id,
        current_color,
        current_line_width,
        current_line_style,
        labels.current,
    );
    let open_command = if force_open {
        Some(egui::SetOpenCommand::Bool(true))
    } else if trigger.clicked() {
        Some(egui::SetOpenCommand::Toggle)
    } else {
        None
    };
    egui::Popup::from_toggle_button_response(&trigger)
        .id(id)
        .open_memory(open_command)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            output = primitive_crosshair_style_panel(
                ui,
                id,
                current_color,
                current_line_width,
                current_line_style,
                line_style_options,
                labels,
            );
        });
    output
}

fn primitive_crosshair_style_trigger(
    ui: &mut egui::Ui,
    id: egui::Id,
    current_color: egui::Color32,
    current_line_width: u8,
    current_line_style: &str,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(78.0, 30.0), egui::Sense::click());
    let visuals = ui.style().interact(&response);
    ui.painter().rect_filled(rect, 4.0, visuals.bg_fill);
    ui.painter()
        .rect_stroke(rect, 4.0, visuals.bg_stroke, egui::StrokeKind::Inside);
    let swatch_rect = egui::Rect::from_min_size(
        rect.left_center() + egui::vec2(7.0, -10.0),
        egui::vec2(22.0, 20.0),
    );
    ui.painter().rect_filled(swatch_rect, 3.0, current_color);
    paint_line_preview(
        ui,
        egui::Rect::from_min_max(
            egui::pos2(swatch_rect.right() + 9.0, rect.top() + 6.0),
            egui::pos2(rect.right() - 8.0, rect.bottom() - 6.0),
        ),
        current_line_style,
        f32::from(current_line_width.clamp(1, 4)),
        visuals.text_color(),
    );
    response
        .on_hover_text(tooltip.to_owned())
        .union(ui.interact(rect, id.with("hover"), egui::Sense::hover()))
}

fn primitive_crosshair_style_panel(
    ui: &mut egui::Ui,
    id: egui::Id,
    current_color: egui::Color32,
    current_line_width: u8,
    current_line_style: &str,
    line_style_options: &[PrimitiveSelectOption],
    labels: PrimitiveColorPickerLabels,
) -> PrimitiveCrosshairStylePickerOutput {
    let mut output = PrimitiveCrosshairStylePickerOutput {
        color: None,
        line_width: None,
        line_style: None,
    };
    egui::Frame::popup(ui.style())
        .inner_margin(egui::Margin::symmetric(14, 12))
        .show(ui, |ui| {
            ui.set_min_width(270.0);
            let colors = trading_view_color_picker_palette();
            for row in colors.chunks(10) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(7.0, 6.0);
                    for color in row {
                        let response =
                            primitive_color_chip(ui, id, *color, *color == current_color);
                        if response.clicked() {
                            output.color = Some(*color);
                        }
                    }
                });
            }
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(24.0, 24.0), egui::Sense::click());
                let stroke = Stroke::new(1.0, ui.visuals().widgets.inactive.fg_stroke.color);
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.center().x, rect.top() + 5.0),
                        egui::pos2(rect.center().x, rect.bottom() - 5.0),
                    ],
                    stroke,
                );
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.left() + 5.0, rect.center().y),
                        egui::pos2(rect.right() - 5.0, rect.center().y),
                    ],
                    stroke,
                );
                response.on_hover_text(labels.custom);
            });
            ui.add_space(8.0);
            ui.label(
                RichText::new(labels.opacity)
                    .size(12.0)
                    .color(ui.visuals().weak_text_color()),
            );
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let opacity = primitive_opacity_slider(ui, current_color);
                ui.label(format!("{}%", (opacity * 100.0).round() as i32));
                if (opacity - (current_color.a() as f32 / 255.0)).abs() > f32::EPSILON {
                    output.color = Some(egui::Color32::from_rgba_premultiplied(
                        current_color.r(),
                        current_color.g(),
                        current_color.b(),
                        (opacity * 255.0).round().clamp(0.0, 255.0) as u8,
                    ));
                }
            });
            ui.add_space(8.0);
            ui.label(
                RichText::new("두께")
                    .size(12.0)
                    .color(ui.visuals().weak_text_color()),
            );
            ui.add_space(4.0);
            if let Some(width) = primitive_line_width_picker(ui, current_line_width) {
                output.line_width = Some(width);
            }
            ui.add_space(8.0);
            ui.label(
                RichText::new("라인 스타일")
                    .size(12.0)
                    .color(ui.visuals().weak_text_color()),
            );
            ui.add_space(4.0);
            if let Some(style) =
                primitive_line_style_picker(ui, current_line_style, line_style_options)
            {
                output.line_style = Some(style);
            }
        });
    output
}

fn primitive_color_picker_trigger(
    ui: &mut egui::Ui,
    id: egui::Id,
    current: egui::Color32,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(44.0, 28.0), egui::Sense::click());
    let visuals = ui.style().interact(&response);
    ui.painter().rect_filled(rect, 4.0, visuals.bg_fill);
    ui.painter()
        .rect_stroke(rect, 4.0, visuals.bg_stroke, egui::StrokeKind::Inside);
    let swatch_rect = egui::Rect::from_min_size(
        rect.left_center() + egui::vec2(6.0, -9.0),
        egui::vec2(20.0, 18.0),
    );
    ui.painter().rect_filled(swatch_rect, 3.0, current);
    ui.painter().rect_stroke(
        swatch_rect,
        3.0,
        Stroke::new(1.0, ui.visuals().widgets.inactive.bg_stroke.color),
        egui::StrokeKind::Inside,
    );
    let chevron_x = rect.right() - 11.0;
    let center_y = rect.center().y;
    ui.painter().line_segment(
        [
            egui::pos2(chevron_x - 4.0, center_y - 2.0),
            egui::pos2(chevron_x, center_y + 2.0),
        ],
        Stroke::new(1.2, visuals.fg_stroke.color),
    );
    ui.painter().line_segment(
        [
            egui::pos2(chevron_x, center_y + 2.0),
            egui::pos2(chevron_x + 4.0, center_y - 2.0),
        ],
        Stroke::new(1.2, visuals.fg_stroke.color),
    );
    response
        .on_hover_text(tooltip.to_owned())
        .union(ui.interact(rect, id.with("hover"), egui::Sense::hover()))
}

fn primitive_color_picker_panel(
    ui: &mut egui::Ui,
    id: egui::Id,
    current: egui::Color32,
    labels: PrimitiveColorPickerLabels,
) -> Option<egui::Color32> {
    let _reference_asset = primitive_color_picker_reference_bytes();
    let mut selected = None;
    egui::Frame::popup(ui.style())
        .inner_margin(egui::Margin::symmetric(14, 12))
        .show(ui, |ui| {
            ui.set_min_width(244.0);
            let colors = trading_view_color_picker_palette();
            for row in colors.chunks(10) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(7.0, 6.0);
                    for color in row {
                        let response = primitive_color_chip(ui, id, *color, *color == current);
                        if response.clicked() {
                            selected = Some(*color);
                            egui::Popup::close_id(ui.ctx(), id);
                        }
                    }
                });
            }
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(24.0, 24.0), egui::Sense::click());
                let stroke = Stroke::new(1.0, ui.visuals().widgets.inactive.fg_stroke.color);
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.center().x, rect.top() + 5.0),
                        egui::pos2(rect.center().x, rect.bottom() - 5.0),
                    ],
                    stroke,
                );
                ui.painter().line_segment(
                    [
                        egui::pos2(rect.left() + 5.0, rect.center().y),
                        egui::pos2(rect.right() - 5.0, rect.center().y),
                    ],
                    stroke,
                );
                response.on_hover_text(labels.custom);
            });
            ui.add_space(8.0);
            ui.label(
                RichText::new(labels.opacity)
                    .size(12.0)
                    .color(ui.visuals().weak_text_color()),
            );
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                let opacity = primitive_opacity_slider(ui, current);
                ui.label(format!("{}%", (opacity * 100.0).round() as i32));
                if (opacity - (current.a() as f32 / 255.0)).abs() > f32::EPSILON {
                    selected = Some(egui::Color32::from_rgba_premultiplied(
                        current.r(),
                        current.g(),
                        current.b(),
                        (opacity * 255.0).round().clamp(0.0, 255.0) as u8,
                    ));
                }
            });
        });
    selected
}

fn primitive_opacity_slider(ui: &mut egui::Ui, current: egui::Color32) -> f32 {
    let mut opacity = current.a() as f32 / 255.0;
    let (rect, response) = ui.allocate_exact_size(egui::vec2(184.0, 16.0), egui::Sense::drag());
    if (response.dragged() || response.clicked())
        && let Some(pointer) = response.interact_pointer_pos()
    {
        opacity = ((pointer.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
    }

    let checker = 6.0;
    let mut y = rect.top();
    let mut row = 0;
    while y < rect.bottom() {
        let mut x = rect.left();
        let mut column = 0;
        while x < rect.right() {
            let tile = egui::Rect::from_min_max(
                egui::pos2(x, y),
                egui::pos2(
                    (x + checker).min(rect.right()),
                    (y + checker).min(rect.bottom()),
                ),
            );
            let fill = if (row + column) % 2 == 0 {
                egui::Color32::from_rgb(238, 238, 238)
            } else {
                egui::Color32::from_rgb(196, 216, 216)
            };
            ui.painter().rect_filled(tile, 0.0, fill);
            x += checker;
            column += 1;
        }
        y += checker;
        row += 1;
    }

    let steps = 24;
    for step in 0..steps {
        let t0 = step as f32 / steps as f32;
        let t1 = (step + 1) as f32 / steps as f32;
        let segment = egui::Rect::from_min_max(
            egui::pos2(rect.left() + rect.width() * t0, rect.top()),
            egui::pos2(rect.left() + rect.width() * t1 + 1.0, rect.bottom()),
        );
        ui.painter().rect_filled(
            segment,
            0.0,
            egui::Color32::from_rgba_unmultiplied(
                current.r(),
                current.g(),
                current.b(),
                (t1 * 255.0).round() as u8,
            ),
        );
    }
    ui.painter().rect_stroke(
        rect,
        4.0,
        Stroke::new(1.0, ui.visuals().widgets.inactive.bg_stroke.color),
        egui::StrokeKind::Inside,
    );
    let knob_center = egui::pos2(rect.left() + rect.width() * opacity, rect.center().y);
    ui.painter()
        .circle_filled(knob_center, 6.0, egui::Color32::WHITE);
    ui.painter().circle_stroke(
        knob_center,
        6.0,
        Stroke::new(1.5, egui::Color32::from_rgb(20, 20, 20)),
    );
    opacity
}

fn primitive_color_chip(
    ui: &mut egui::Ui,
    id: egui::Id,
    color: egui::Color32,
    selected: bool,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(18.0, 18.0), egui::Sense::click());
    ui.painter().rect_filled(rect, 2.0, color);
    if selected {
        ui.painter().rect_stroke(
            rect.expand(2.0),
            3.0,
            Stroke::new(2.0, ui.visuals().selection.stroke.color),
            egui::StrokeKind::Inside,
        );
    }
    response.union(ui.interact(rect, id.with(color.to_array()), egui::Sense::hover()))
}

fn trading_view_color_picker_palette() -> [egui::Color32; 70] {
    use egui::Color32;
    [
        Color32::from_rgb(255, 255, 255),
        Color32::from_rgb(229, 229, 229),
        Color32::from_rgb(190, 190, 190),
        Color32::from_rgb(158, 158, 158),
        Color32::from_rgb(117, 117, 117),
        Color32::from_rgb(97, 97, 97),
        Color32::from_rgb(66, 66, 66),
        Color32::from_rgb(33, 33, 33),
        Color32::from_rgb(15, 15, 15),
        Color32::from_rgb(0, 0, 0),
        Color32::from_rgb(244, 67, 54),
        Color32::from_rgb(255, 152, 0),
        Color32::from_rgb(255, 235, 59),
        Color32::from_rgb(76, 175, 80),
        Color32::from_rgb(0, 150, 136),
        Color32::from_rgb(0, 188, 212),
        Color32::from_rgb(33, 150, 243),
        Color32::from_rgb(63, 81, 181),
        Color32::from_rgb(156, 39, 176),
        Color32::from_rgb(233, 30, 99),
        Color32::from_rgb(255, 205, 210),
        Color32::from_rgb(255, 224, 178),
        Color32::from_rgb(255, 249, 196),
        Color32::from_rgb(200, 230, 201),
        Color32::from_rgb(178, 223, 219),
        Color32::from_rgb(178, 235, 242),
        Color32::from_rgb(187, 222, 251),
        Color32::from_rgb(197, 202, 233),
        Color32::from_rgb(225, 190, 231),
        Color32::from_rgb(248, 187, 208),
        Color32::from_rgb(239, 154, 154),
        Color32::from_rgb(255, 204, 128),
        Color32::from_rgb(255, 245, 157),
        Color32::from_rgb(165, 214, 167),
        Color32::from_rgb(128, 203, 196),
        Color32::from_rgb(128, 222, 234),
        Color32::from_rgb(144, 202, 249),
        Color32::from_rgb(159, 168, 218),
        Color32::from_rgb(206, 147, 216),
        Color32::from_rgb(244, 143, 177),
        Color32::from_rgb(229, 115, 115),
        Color32::from_rgb(255, 183, 77),
        Color32::from_rgb(255, 238, 88),
        Color32::from_rgb(129, 199, 132),
        Color32::from_rgb(77, 182, 172),
        Color32::from_rgb(77, 208, 225),
        Color32::from_rgb(100, 181, 246),
        Color32::from_rgb(121, 134, 203),
        Color32::from_rgb(186, 104, 200),
        Color32::from_rgb(240, 98, 146),
        Color32::from_rgb(198, 40, 40),
        Color32::from_rgb(239, 108, 0),
        Color32::from_rgb(251, 192, 45),
        Color32::from_rgb(46, 125, 50),
        Color32::from_rgb(0, 105, 92),
        Color32::from_rgb(0, 131, 143),
        Color32::from_rgb(21, 101, 192),
        Color32::from_rgb(40, 53, 147),
        Color32::from_rgb(106, 27, 154),
        Color32::from_rgb(173, 20, 87),
        Color32::from_rgb(183, 28, 28),
        Color32::from_rgb(230, 81, 0),
        Color32::from_rgb(245, 124, 0),
        Color32::from_rgb(27, 94, 32),
        Color32::from_rgb(0, 77, 64),
        Color32::from_rgb(0, 96, 100),
        Color32::from_rgb(13, 71, 161),
        Color32::from_rgb(26, 35, 126),
        Color32::from_rgb(74, 20, 140),
        Color32::from_rgb(136, 14, 79),
    ]
}

pub fn primitive_line_style_picker(
    ui: &mut egui::Ui,
    current: &str,
    options: &[PrimitiveSelectOption],
) -> Option<String> {
    let mut selected = None;
    ui.horizontal(|ui| {
        for option in options {
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(38.0, 24.0), egui::Sense::click());
            let active = option.id == current;
            let visuals = ui.style().interact_selectable(&response, active);
            ui.painter().rect_filled(
                rect,
                4.0,
                if active {
                    ui.visuals().selection.bg_fill
                } else {
                    visuals.bg_fill
                },
            );
            let y = rect.center().y;
            let x0 = rect.left() + 8.0;
            let x1 = rect.right() - 8.0;
            let stroke = Stroke::new(1.5, visuals.text_color());
            match option.id {
                "dotted" => {
                    let mut x = x0;
                    while x <= x1 {
                        ui.painter()
                            .circle_filled(egui::pos2(x, y), 1.4, stroke.color);
                        x += 6.0;
                    }
                }
                "dashed" => {
                    let mut x = x0;
                    while x < x1 {
                        ui.painter().line_segment(
                            [egui::pos2(x, y), egui::pos2((x + 7.0).min(x1), y)],
                            stroke,
                        );
                        x += 12.0;
                    }
                }
                _ => {
                    ui.painter()
                        .line_segment([egui::pos2(x0, y), egui::pos2(x1, y)], stroke);
                }
            }
            if response.clicked() {
                selected = Some(option.id.to_owned());
            }
            response.on_hover_text(option.label);
        }
    });
    selected
}

pub fn primitive_line_width_picker(ui: &mut egui::Ui, current: u8) -> Option<u8> {
    let mut selected = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
        for width in 1_u8..=4 {
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(70.0, 32.0), egui::Sense::click());
            let active = width == current;
            let visuals = ui.style().interact_selectable(&response, active);
            ui.painter().rect_filled(
                rect,
                2.0,
                if active {
                    ui.visuals().selection.bg_fill
                } else {
                    visuals.bg_fill
                },
            );
            ui.painter().rect_stroke(
                rect,
                2.0,
                Stroke::new(1.0, ui.visuals().widgets.inactive.bg_stroke.color),
                egui::StrokeKind::Inside,
            );
            paint_line_preview(
                ui,
                rect.shrink2(egui::vec2(18.0, 8.0)),
                "solid",
                f32::from(width),
                visuals.text_color(),
            );
            if response.clicked() {
                selected = Some(width);
            }
            response.on_hover_text(format!("{} px", width));
        }
    });
    selected
}

fn paint_line_preview(
    ui: &egui::Ui,
    rect: egui::Rect,
    style: &str,
    width: f32,
    color: egui::Color32,
) {
    let y = rect.center().y;
    let x0 = rect.left();
    let x1 = rect.right();
    let stroke = Stroke::new(width, color);
    match style {
        "dotted" => {
            let mut x = x0;
            while x <= x1 {
                ui.painter()
                    .circle_filled(egui::pos2(x, y), width.max(1.4), color);
                x += 7.0;
            }
        }
        "dashed" => {
            let mut x = x0;
            while x < x1 {
                ui.painter()
                    .line_segment([egui::pos2(x, y), egui::pos2((x + 9.0).min(x1), y)], stroke);
                x += 14.0;
            }
        }
        _ => {
            ui.painter()
                .line_segment([egui::pos2(x0, y), egui::pos2(x1, y)], stroke);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveSelectOption {
    pub id: &'static str,
    pub label: &'static str,
}

impl PrimitiveSelectOption {
    pub const fn new(id: &'static str, label: &'static str) -> Self {
        Self { id, label }
    }
}

pub fn primitive_bool_field(ui: &mut egui::Ui, value: &mut bool) -> bool {
    let (rect, response) = ui.allocate_exact_size(egui::vec2(22.0, 22.0), egui::Sense::click());
    if response.clicked() {
        *value = !*value;
    }
    let visuals = ui.style().interact_selectable(&response, *value);
    let fill = if *value {
        egui::Color32::from_rgb(55, 58, 62)
    } else if response.hovered() {
        egui::Color32::from_rgb(248, 249, 250)
    } else {
        egui::Color32::WHITE
    };
    let stroke = if *value {
        Stroke::new(1.0, fill)
    } else {
        Stroke::new(1.0, egui::Color32::from_rgb(142, 146, 153))
    };
    ui.painter().rect(
        rect.shrink(1.0),
        3.0,
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    if *value {
        paint_radix_icon(ui, RadixIcon::Check, rect.shrink(4.0), egui::Color32::WHITE);
    } else if response.hovered() {
        ui.painter().rect_stroke(
            rect.shrink(1.0),
            3.0,
            Stroke::new(1.0, visuals.text_color()),
            egui::StrokeKind::Inside,
        );
    }
    response.clicked()
}

pub fn primitive_select_field(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    current: &str,
    options: &[PrimitiveSelectOption],
) -> Option<String> {
    let selected = options
        .iter()
        .find(|option| option.id == current)
        .map(|option| option.label)
        .unwrap_or(current);
    let mut next = None;
    egui::ComboBox::from_id_salt(id)
        .selected_text(selected)
        .width(132.0)
        .show_ui(ui, |ui| {
            for option in options {
                if ui
                    .selectable_label(option.id == current, option.label)
                    .clicked()
                {
                    next = Some(option.id.to_owned());
                }
            }
        });
    next
}

pub fn primitive_text_field(ui: &mut egui::Ui, value: &mut String) -> bool {
    ui.add_sized([160.0, 24.0], egui::TextEdit::singleline(value))
        .changed()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveActionKind {
    Primary,
    Secondary,
    Destructive,
}

pub fn primitive_action_button(
    ui: &mut egui::Ui,
    label: &str,
    kind: PrimitiveActionKind,
) -> egui::Response {
    let button = match kind {
        PrimitiveActionKind::Primary => {
            egui::Button::new(RichText::new(label).strong().color(egui::Color32::WHITE))
                .fill(egui::Color32::from_rgb(18, 18, 18))
        }
        PrimitiveActionKind::Secondary => egui::Button::new(label),
        PrimitiveActionKind::Destructive => {
            egui::Button::new(RichText::new(label).color(egui::Color32::from_rgb(180, 54, 42)))
        }
    };
    ui.add_sized([96.0, 28.0], button)
}

pub fn primitive_numeric_input_i64(
    ui: &mut egui::Ui,
    value: &mut i64,
    range: std::ops::RangeInclusive<i64>,
    suffix: &str,
) -> bool {
    let before = *value;
    ui.add(egui::DragValue::new(value).range(range).suffix(suffix));
    *value != before
}

pub fn primitive_numeric_input_f64(
    ui: &mut egui::Ui,
    value: &mut f64,
    range: std::ops::RangeInclusive<f64>,
    suffix: &str,
) -> bool {
    let before = *value;
    ui.add(egui::DragValue::new(value).range(range).suffix(suffix));
    (*value - before).abs() > f64::EPSILON
}

pub fn primitive_help_icon(ui: &mut egui::Ui, text: &str) {
    ui.small_button("?").on_hover_text(text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_row_options_preserve_contract_state() {
        let options = PrimitiveSettingsRowOptions::new("row", "Grid", PrimitiveTheme::default())
            .description(Some("desc"))
            .enabled(false)
            .selected(true);

        assert_eq!(options.id, egui::Id::new("row"));
        assert_eq!(options.label, "Grid");
        assert_eq!(options.description, Some("desc"));
        assert!(!options.enabled);
        assert!(options.selected);
    }

    #[test]
    fn select_option_keeps_internal_id_and_display_label_separate() {
        let option = PrimitiveSelectOption::new("visible-range", "Visible range");

        assert_eq!(option.id, "visible-range");
        assert_eq!(option.label, "Visible range");
    }

    #[test]
    fn color_picker_reference_asset_is_connected() {
        assert!(!primitive_color_picker_reference_bytes().is_empty());
    }

    #[test]
    fn color_picker_palette_matches_reference_grid_shape() {
        assert_eq!(trading_view_color_picker_palette().len(), 70);
    }

    #[test]
    fn crosshair_style_picker_output_can_hold_three_independent_changes() {
        let output = PrimitiveCrosshairStylePickerOutput {
            color: Some(egui::Color32::from_rgb(1, 2, 3)),
            line_width: Some(3),
            line_style: Some("dashed".to_owned()),
        };

        assert_eq!(output.color.unwrap(), egui::Color32::from_rgb(1, 2, 3));
        assert_eq!(output.line_width, Some(3));
        assert_eq!(output.line_style.as_deref(), Some("dashed"));
    }
}
