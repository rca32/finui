use std::hash::Hash;

use egui::{self, Color32, FontId, Rect, Response, Sense, Stroke, Vec2};

use super::{
    PrimitiveTheme, RadixIcon, paint_radix_icon, primitive_focus_ring_stroke,
    primitive_theme_is_dark,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveButtonVariant {
    Solid,
    Soft,
    Outline,
    Ghost,
    Destructive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveButtonInteractionState {
    Rest,
    Hover,
    Pressed,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveButtonOptions {
    pub size: Vec2,
    pub enabled: bool,
    pub variant: PrimitiveButtonVariant,
    pub leading_icon: Option<RadixIcon>,
    pub trailing_icon: Option<RadixIcon>,
    pub theme: PrimitiveTheme,
    pub focus_ring_radius: f32,
    pub focus_ring_offset: f32,
    pub request_focus: bool,
}

impl Default for PrimitiveButtonOptions {
    fn default() -> Self {
        Self {
            size: Vec2::new(96.0, 28.0),
            enabled: true,
            variant: PrimitiveButtonVariant::Solid,
            leading_icon: None,
            trailing_icon: None,
            theme: PrimitiveTheme::default(),
            focus_ring_radius: 6.0,
            focus_ring_offset: 2.0,
            request_focus: false,
        }
    }
}

impl PrimitiveButtonOptions {
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn variant(mut self, variant: PrimitiveButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn leading_icon(mut self, icon: RadixIcon) -> Self {
        self.leading_icon = Some(icon);
        self
    }

    pub fn trailing_icon(mut self, icon: RadixIcon) -> Self {
        self.trailing_icon = Some(icon);
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn focus_ring(mut self, radius: f32, offset: f32) -> Self {
        self.focus_ring_radius = radius;
        self.focus_ring_offset = offset.max(0.0);
        self
    }

    pub fn request_focus(mut self, request_focus: bool) -> Self {
        self.request_focus = request_focus;
        self
    }
}

pub struct PrimitiveButtonOutput {
    pub response: Response,
    pub activated: bool,
    pub focused: bool,
    pub focus_visible: bool,
    pub interaction_state: PrimitiveButtonInteractionState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveButtonVisualStyle {
    pub fill: Color32,
    pub stroke: Stroke,
    pub text: Color32,
    pub icon: Color32,
}

pub fn primitive_button(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    label: &str,
    options: PrimitiveButtonOptions,
) -> PrimitiveButtonOutput {
    let (rect, _) = ui.allocate_exact_size(options.size, Sense::hover());
    primitive_button_at(ui, rect, id_source, label, options)
}

pub fn primitive_button_at(
    ui: &mut egui::Ui,
    rect: Rect,
    id_source: impl Hash,
    label: &str,
    options: PrimitiveButtonOptions,
) -> PrimitiveButtonOutput {
    let output = primitive_button_interaction_at(ui, rect, id_source, label, options);
    paint_primitive_button_at(
        ui,
        rect,
        label,
        output.interaction_state,
        output.focus_visible,
        options,
    );
    output
}

pub fn primitive_button_interaction_at(
    ui: &mut egui::Ui,
    rect: Rect,
    id_source: impl Hash,
    label: &str,
    options: PrimitiveButtonOptions,
) -> PrimitiveButtonOutput {
    let response = ui.interact(
        rect,
        ui.id().with(id_source),
        if options.enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, options.enabled, label)
    });
    if options.request_focus && options.enabled {
        response.request_focus();
    }
    if options.enabled && response.clicked_by(egui::PointerButton::Primary) {
        response.request_focus();
    }
    if options.enabled && response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let focused = options.enabled && response.has_focus();
    let pointer_engaged = options.enabled
        && response.contains_pointer()
        && ui.input(|input| input.pointer.any_pressed() || input.pointer.any_released());
    let keyboard_engaged = focused
        && ui.input(|input| {
            input.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::Key {
                        pressed: true,
                        repeat: false,
                        ..
                    }
                )
            })
        });
    let focus_state_id = response.id.with("finui-button-focus-visible");
    let previous_focus_visible = ui
        .ctx()
        .data(|data| data.get_temp::<bool>(focus_state_id))
        .unwrap_or(false);
    let focus_visible = primitive_button_focus_visible_next(
        options.enabled,
        focused,
        response.gained_focus(),
        pointer_engaged,
        keyboard_engaged,
        previous_focus_visible,
    );
    ui.ctx()
        .data_mut(|data| data.insert_temp(focus_state_id, focus_visible));

    let interaction_state = if !options.enabled {
        PrimitiveButtonInteractionState::Disabled
    } else if response.is_pointer_button_down_on() {
        PrimitiveButtonInteractionState::Pressed
    } else if response.hovered() {
        PrimitiveButtonInteractionState::Hover
    } else {
        PrimitiveButtonInteractionState::Rest
    };

    PrimitiveButtonOutput {
        activated: options.enabled && response.clicked(),
        response,
        focused,
        focus_visible,
        interaction_state,
    }
}

pub fn primitive_button_focus_visible_next(
    enabled: bool,
    focused: bool,
    gained_focus: bool,
    pointer_engaged: bool,
    keyboard_engaged: bool,
    previous_focus_visible: bool,
) -> bool {
    if !enabled || !focused {
        return false;
    }
    if pointer_engaged {
        return false;
    }
    gained_focus || keyboard_engaged || previous_focus_visible
}

pub fn primitive_button_visual_style(
    variant: PrimitiveButtonVariant,
    state: PrimitiveButtonInteractionState,
    theme: PrimitiveTheme,
) -> PrimitiveButtonVisualStyle {
    let dark = primitive_theme_is_dark(theme);
    if state == PrimitiveButtonInteractionState::Disabled {
        return PrimitiveButtonVisualStyle {
            fill: theme.item_hover_fill,
            stroke: Stroke::new(1.0, theme.content_stroke.color.gamma_multiply(0.7)),
            text: theme.disabled_text,
            icon: theme.disabled_text,
        };
    }

    let (fill, stroke, text) = match variant {
        PrimitiveButtonVariant::Solid => {
            let fill = match (dark, state) {
                (true, PrimitiveButtonInteractionState::Hover) => {
                    Color32::from_rgb(0x4b, 0x6d, 0xe3)
                }
                (true, PrimitiveButtonInteractionState::Pressed) => {
                    Color32::from_rgb(0x34, 0x55, 0xc7)
                }
                (false, PrimitiveButtonInteractionState::Hover) => {
                    Color32::from_rgb(0x33, 0x55, 0xc7)
                }
                (false, PrimitiveButtonInteractionState::Pressed) => {
                    Color32::from_rgb(0x2f, 0x4f, 0xb8)
                }
                _ => super::radix_colors::INDIGO_9,
            };
            (fill, Stroke::NONE, Color32::WHITE)
        }
        PrimitiveButtonVariant::Soft => {
            let fill = if state == PrimitiveButtonInteractionState::Pressed {
                theme.item_selected_fill.gamma_multiply(1.18)
            } else if state == PrimitiveButtonInteractionState::Hover {
                theme.item_selected_fill.gamma_multiply(1.08)
            } else {
                theme.item_selected_fill
            };
            (fill, Stroke::NONE, theme.text)
        }
        PrimitiveButtonVariant::Outline => {
            let fill = if state == PrimitiveButtonInteractionState::Pressed {
                theme.item_selected_fill
            } else if state == PrimitiveButtonInteractionState::Hover {
                theme.item_hover_fill
            } else {
                theme.content_fill
            };
            (fill, theme.content_stroke, theme.text)
        }
        PrimitiveButtonVariant::Ghost => {
            let fill = if state == PrimitiveButtonInteractionState::Pressed {
                theme.item_selected_fill
            } else if state == PrimitiveButtonInteractionState::Hover {
                theme.item_hover_fill
            } else {
                Color32::TRANSPARENT
            };
            (fill, Stroke::NONE, theme.text)
        }
        PrimitiveButtonVariant::Destructive => {
            let fill = match state {
                PrimitiveButtonInteractionState::Hover => Color32::from_rgb(0xec, 0x61, 0x43),
                PrimitiveButtonInteractionState::Pressed => Color32::from_rgb(0xc8, 0x42, 0x28),
                _ => super::radix_colors::TOMATO_9,
            };
            (fill, Stroke::NONE, Color32::WHITE)
        }
    };
    PrimitiveButtonVisualStyle {
        fill,
        stroke,
        text,
        icon: text,
    }
}

pub fn paint_primitive_button_focus_ring(
    ui: &egui::Ui,
    rect: Rect,
    radius: f32,
    offset: f32,
    theme: PrimitiveTheme,
) {
    ui.painter().rect_stroke(
        rect.expand(offset),
        radius + offset,
        primitive_focus_ring_stroke(theme),
        egui::StrokeKind::Inside,
    );
}

pub fn paint_primitive_button_at(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    interaction_state: PrimitiveButtonInteractionState,
    focus_visible: bool,
    options: PrimitiveButtonOptions,
) {
    let interaction_state = if options.enabled {
        interaction_state
    } else {
        PrimitiveButtonInteractionState::Disabled
    };
    let style = primitive_button_visual_style(options.variant, interaction_state, options.theme);
    ui.painter().rect(
        rect,
        options.theme.row_radius,
        style.fill,
        style.stroke,
        egui::StrokeKind::Inside,
    );

    let icon_size = 14.0;
    let gap = 6.0;
    let mut label_center_x = rect.center().x;
    if options.leading_icon.is_some() && options.trailing_icon.is_none() {
        label_center_x += (icon_size + gap) * 0.5;
    } else if options.trailing_icon.is_some() && options.leading_icon.is_none() {
        label_center_x -= (icon_size + gap) * 0.5;
    }
    if let Some(icon) = options.leading_icon {
        paint_radix_icon(
            ui,
            icon,
            Rect::from_center_size(
                egui::pos2(rect.left() + 12.0, rect.center().y),
                Vec2::splat(icon_size),
            ),
            style.icon,
        );
    }
    if let Some(icon) = options.trailing_icon {
        paint_radix_icon(
            ui,
            icon,
            Rect::from_center_size(
                egui::pos2(rect.right() - 12.0, rect.center().y),
                Vec2::splat(icon_size),
            ),
            style.icon,
        );
    }
    ui.painter().text(
        egui::pos2(label_center_x, rect.center().y),
        egui::Align2::CENTER_CENTER,
        label,
        FontId::proportional(13.0),
        style.text,
    );
    if focus_visible && options.enabled {
        paint_primitive_button_focus_ring(
            ui,
            rect,
            options.focus_ring_radius,
            options.focus_ring_offset,
            options.theme,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_visible_tracks_keyboard_origin_without_sticking_after_pointer_focus() {
        assert!(primitive_button_focus_visible_next(
            true, true, true, false, false, false
        ));
        assert!(!primitive_button_focus_visible_next(
            true, true, true, true, false, false
        ));
        assert!(primitive_button_focus_visible_next(
            true, true, false, false, true, false
        ));
        assert!(!primitive_button_focus_visible_next(
            false, true, true, false, true, true
        ));
        assert!(!primitive_button_focus_visible_next(
            true, false, false, false, false, true
        ));
    }

    #[test]
    fn button_visuals_are_theme_aware_and_disabled_is_quiet() {
        let dark = primitive_button_visual_style(
            PrimitiveButtonVariant::Solid,
            PrimitiveButtonInteractionState::Rest,
            PrimitiveTheme::dark(),
        );
        let light = primitive_button_visual_style(
            PrimitiveButtonVariant::Outline,
            PrimitiveButtonInteractionState::Rest,
            PrimitiveTheme::light(),
        );
        let disabled = primitive_button_visual_style(
            PrimitiveButtonVariant::Solid,
            PrimitiveButtonInteractionState::Disabled,
            PrimitiveTheme::dark(),
        );

        assert_ne!(dark.fill, light.fill);
        assert_eq!(dark.text, Color32::WHITE);
        assert_eq!(light.stroke, PrimitiveTheme::light().content_stroke);
        assert_eq!(disabled.text, PrimitiveTheme::dark().disabled_text);
    }

    #[test]
    fn real_response_activates_from_keyboard_focus() {
        let context = egui::Context::default();
        let mut output = None;
        let draw = |context: &egui::Context,
                    events: Vec<egui::Event>,
                    request_focus: bool,
                    output: &mut Option<(bool, bool)>| {
            let raw_input = egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(180.0, 80.0),
                )),
                events,
                ..Default::default()
            };
            let _ = context.run_ui(raw_input, |ui| {
                let button = primitive_button(
                    ui,
                    "keyboard-button",
                    "Run",
                    PrimitiveButtonOptions::default().request_focus(request_focus),
                );
                *output = Some((button.activated, button.focus_visible));
            });
        };

        draw(&context, Vec::new(), true, &mut output);
        draw(&context, Vec::new(), false, &mut output);
        assert_eq!(output, Some((false, true)));
        draw(
            &context,
            vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: Some(egui::Key::Enter),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            false,
            &mut output,
        );
        assert_eq!(output, Some((true, true)));
    }

    #[test]
    fn button_at_uses_the_supplied_rect_with_full_button_behavior() {
        let context = egui::Context::default();
        let rect = Rect::from_min_size(egui::pos2(18.0, 14.0), Vec2::new(144.0, 32.0));
        let mut output_rect = Rect::NOTHING;
        let raw_input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(180.0, 80.0),
            )),
            ..Default::default()
        };

        let frame = context.run_ui(raw_input, |ui| {
            let output = primitive_button_at(
                ui,
                rect,
                "positioned-button",
                "Continue",
                PrimitiveButtonOptions::default()
                    .size(rect.size())
                    .variant(PrimitiveButtonVariant::Outline)
                    .leading_icon(RadixIcon::Play)
                    .trailing_icon(RadixIcon::ChevronRight),
            );
            output_rect = output.response.rect;
        });

        assert_eq!(output_rect, rect);
        assert!(!frame.shapes.is_empty());
    }

    #[test]
    fn positioned_button_paint_accepts_a_deterministic_visual_state() {
        let context = egui::Context::default();
        let rect = Rect::from_min_size(egui::pos2(18.0, 14.0), Vec2::new(144.0, 32.0));
        let raw_input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(180.0, 80.0),
            )),
            ..Default::default()
        };

        let frame = context.run_ui(raw_input, |ui| {
            paint_primitive_button_at(
                ui,
                rect,
                "Continue",
                PrimitiveButtonInteractionState::Pressed,
                true,
                PrimitiveButtonOptions::default()
                    .size(rect.size())
                    .variant(PrimitiveButtonVariant::Outline)
                    .leading_icon(RadixIcon::Play)
                    .trailing_icon(RadixIcon::ChevronRight),
            );
        });

        assert!(!frame.shapes.is_empty());
        assert_eq!(
            primitive_button_visual_style(
                PrimitiveButtonVariant::Outline,
                PrimitiveButtonInteractionState::Pressed,
                PrimitiveTheme::default(),
            )
            .fill,
            PrimitiveTheme::default().item_selected_fill
        );
    }
}
