use eframe::egui::{self, Align2, FontId, Rect, Response, Sense, Vec2};

use super::{PrimitiveTheme, paint_radix_icon, radix_icon_from_visual};

#[derive(Debug, Clone, Copy)]
pub struct AccessibleIconOptions {
    pub size: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for AccessibleIconOptions {
    fn default() -> Self {
        Self {
            size: 32.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl AccessibleIconOptions {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.enabled = !disabled;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessibleIconRootOptions {
    pub label: String,
    pub decorative: bool,
    pub size: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl AccessibleIconRootOptions {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            decorative: false,
            size: 32.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn decorative(mut self, decorative: bool) -> Self {
        self.decorative = decorative;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.enabled = !disabled;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessibleIconRootOutput {
    pub label: Option<String>,
    pub decorative: bool,
    pub size: f32,
    pub enabled: bool,
}

pub fn primitive_accessible_icon_root_output(
    options: AccessibleIconRootOptions,
) -> AccessibleIconRootOutput {
    AccessibleIconRootOutput {
        label: if options.decorative {
            None
        } else {
            Some(options.label)
        },
        decorative: options.decorative,
        size: options.size.max(16.0),
        enabled: options.enabled,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveDirection {
    Ltr,
    Rtl,
}

impl PrimitiveDirection {
    pub fn is_rtl(self) -> bool {
        matches!(self, Self::Rtl)
    }
}

pub struct PrimitiveDirectionProviderOutput<T> {
    pub inner: T,
    pub direction: PrimitiveDirection,
}

pub fn accessible_icon_label(label: &str) -> egui::WidgetText {
    egui::WidgetText::from(label.to_owned())
}

pub fn primitive_accessible_icon(
    ui: &mut egui::Ui,
    _id_source: impl std::hash::Hash,
    visual: &str,
    label: &str,
    options: AccessibleIconOptions,
) -> Response {
    let root = primitive_accessible_icon_root_output(
        AccessibleIconRootOptions::new(label)
            .size(options.size)
            .disabled(!options.enabled)
            .theme(options.theme),
    );
    let sense = if root.enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(root.size), sense);
    let response = if let Some(label) = root.label.as_deref() {
        response.on_hover_text(accessible_icon_label(label))
    } else {
        response
    };
    let fill = if response.hovered() && root.enabled {
        options.theme.item_hover_fill
    } else {
        options.theme.content_fill
    };
    let text_color = if root.enabled {
        options.theme.text
    } else {
        options.theme.disabled_text
    };

    ui.painter().rect(
        rect,
        options.theme.radius,
        fill,
        options.theme.content_stroke,
        egui::StrokeKind::Inside,
    );
    if let Some(icon) = radix_icon_from_visual(visual) {
        paint_radix_icon(ui, icon, rect.shrink(root.size * 0.25), text_color);
    } else {
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            visual,
            crate::scaled_proportional_font(ui, root.size * 0.5),
            text_color,
        );
    }
    response
}

pub fn primitive_direction_provider<T>(
    ui: &mut egui::Ui,
    direction: PrimitiveDirection,
    add_contents: impl FnOnce(&mut egui::Ui) -> T,
) -> PrimitiveDirectionProviderOutput<T> {
    let layout = if direction.is_rtl() {
        egui::Layout::right_to_left(egui::Align::Center)
    } else {
        egui::Layout::left_to_right(egui::Align::Center)
    };
    let inner = ui.with_layout(layout, add_contents).inner;
    PrimitiveDirectionProviderOutput { inner, direction }
}

pub fn primitive_slot(
    ui: &mut egui::Ui,
    parent: egui::Id,
    part: &'static str,
    rect: Rect,
    sense: Sense,
) -> Response {
    ui.interact(rect, slot_id(parent, part), sense)
}

pub fn slot_id(parent: egui::Id, part: &'static str) -> egui::Id {
    parent.with(part)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_direction_reports_rtl() {
        assert!(!PrimitiveDirection::Ltr.is_rtl());
        assert!(PrimitiveDirection::Rtl.is_rtl());
    }

    #[test]
    fn accessible_icon_options_preserve_disabled_state() {
        let options = AccessibleIconOptions::default().size(28.0).disabled(true);

        assert_eq!(options.size, 28.0);
        assert!(!options.enabled);
    }

    #[test]
    fn accessible_icon_root_output_preserves_label_contract() {
        let output = primitive_accessible_icon_root_output(
            AccessibleIconRootOptions::new("도움말").size(30.0),
        );

        assert_eq!(output.label.as_deref(), Some("도움말"));
        assert!(!output.decorative);
        assert_eq!(output.size, 30.0);
        assert!(output.enabled);
    }

    #[test]
    fn accessible_icon_root_output_allows_decorative_icon() {
        let output = primitive_accessible_icon_root_output(
            AccessibleIconRootOptions::new("ignored")
                .decorative(true)
                .disabled(true),
        );

        assert_eq!(output.label, None);
        assert!(output.decorative);
        assert!(!output.enabled);
    }

    #[test]
    fn direction_provider_output_preserves_direction() {
        let output = PrimitiveDirectionProviderOutput {
            inner: 7,
            direction: PrimitiveDirection::Rtl,
        };

        assert_eq!(output.inner, 7);
        assert_eq!(output.direction, PrimitiveDirection::Rtl);
    }

    #[test]
    fn slot_id_is_stable_for_parent_and_part() {
        let parent = egui::Id::new("parent");

        assert_eq!(slot_id(parent, "trigger"), slot_id(parent, "trigger"));
        assert_ne!(slot_id(parent, "trigger"), slot_id(parent, "content"));
    }
}
