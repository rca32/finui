use egui::{self, Color32, Rect, Vec2};

use super::{BadgeTone, BadgeVariant, PrimitiveTheme, RadixIcon, badge_visual};

mod paint;

pub use paint::{
    primitive_data_list_item, primitive_data_list_label, primitive_data_list_root,
    primitive_data_list_value,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataListOrientation {
    Horizontal,
    Vertical,
}

impl DataListOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataListSize {
    One,
    Two,
    Three,
}

impl DataListSize {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
        }
    }

    fn horizontal_item_height(self) -> f32 {
        match self {
            Self::One => 22.0,
            Self::Two => 28.0,
            Self::Three => 34.0,
        }
    }

    fn vertical_item_height(self) -> f32 {
        match self {
            Self::One => 36.0,
            Self::Two => 44.0,
            Self::Three => 52.0,
        }
    }

    fn label_font_size(self) -> f32 {
        match self {
            Self::One => 10.0,
            Self::Two => 11.0,
            Self::Three => 12.0,
        }
    }

    fn value_font_size(self) -> f32 {
        match self {
            Self::One => 10.5,
            Self::Two => 12.0,
            Self::Three => 13.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataListVariant {
    Plain,
    Surface,
}

impl DataListVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain",
            Self::Surface => "surface",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataListValueTone {
    Muted,
    Neutral,
    Accent,
    Positive,
    Warning,
    Danger,
}

impl DataListValueTone {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Muted => "muted",
            Self::Neutral => "neutral",
            Self::Accent => "accent",
            Self::Positive => "positive",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    fn badge_tone(self) -> Option<BadgeTone> {
        match self {
            Self::Muted | Self::Neutral => None,
            Self::Accent => Some(BadgeTone::Accent),
            Self::Positive => Some(BadgeTone::Positive),
            Self::Warning => Some(BadgeTone::Warning),
            Self::Danger => Some(BadgeTone::Danger),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataListRootOptions {
    pub orientation: DataListOrientation,
    pub size: DataListSize,
    pub variant: DataListVariant,
    pub label_width: f32,
    pub show_dividers: bool,
    pub theme: PrimitiveTheme,
}

impl Default for DataListRootOptions {
    fn default() -> Self {
        Self {
            orientation: DataListOrientation::Horizontal,
            size: DataListSize::Two,
            variant: DataListVariant::Plain,
            label_width: 88.0,
            show_dividers: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl DataListRootOptions {
    pub fn orientation(mut self, orientation: DataListOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn size(mut self, size: DataListSize) -> Self {
        self.size = size;
        self
    }

    pub fn variant(mut self, variant: DataListVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn label_width(mut self, label_width: f32) -> Self {
        self.label_width = label_width.max(0.0);
        self
    }

    pub fn show_dividers(mut self, show_dividers: bool) -> Self {
        self.show_dividers = show_dividers;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataListItem<'a> {
    pub label: &'a str,
    pub value: &'a str,
    pub tone: DataListValueTone,
    pub icon: Option<RadixIcon>,
}

impl<'a> DataListItem<'a> {
    pub fn new(label: &'a str, value: &'a str) -> Self {
        Self {
            label,
            value,
            tone: DataListValueTone::Neutral,
            icon: None,
        }
    }

    pub fn tone(mut self, tone: DataListValueTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn icon(mut self, icon: RadixIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataListVisual {
    pub fill: Color32,
    pub stroke: Color32,
    pub divider: Color32,
    pub label: Color32,
    pub value: Color32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataListValueIconOutput {
    pub rect: Rect,
    pub icon: RadixIcon,
    pub color: Color32,
    pub aria_hidden: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataListLabelOutput {
    pub rect: Rect,
    pub text_pos: egui::Pos2,
    pub text: String,
    pub color: Color32,
    pub font_size: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataListValueOutput {
    pub rect: Rect,
    pub text_rect: Rect,
    pub text_pos: egui::Pos2,
    pub text: String,
    pub color: Color32,
    pub font_size: f32,
    pub icon: Option<DataListValueIconOutput>,
    pub data_tone: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataListItemOutput {
    pub rect: Rect,
    pub index: usize,
    pub divider_rect: Option<Rect>,
    pub label: DataListLabelOutput,
    pub value: DataListValueOutput,
    pub role: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataListRootOutput {
    pub rect: Rect,
    pub orientation: DataListOrientation,
    pub size: DataListSize,
    pub variant: DataListVariant,
    pub visual: DataListVisual,
    pub items: Vec<DataListItemOutput>,
    pub data_orientation: &'static str,
    pub data_size: &'static str,
    pub data_variant: &'static str,
    pub role: &'static str,
}

const DATA_LIST_SURFACE_INSET: f32 = 10.0;
const DATA_LIST_ICON_SIZE: f32 = 10.0;
const DATA_LIST_ICON_GAP: f32 = 6.0;

pub fn primitive_data_list_at(
    ui: &egui::Ui,
    rect: Rect,
    items: &[DataListItem<'_>],
    options: DataListRootOptions,
) -> DataListRootOutput {
    let output = primitive_data_list_root_output(rect, items, options);
    primitive_data_list_root(ui, &output, options.theme);
    for item in &output.items {
        primitive_data_list_item(ui, item, output.visual.divider);
        primitive_data_list_label(ui, &item.label);
        primitive_data_list_value(ui, &item.value);
    }
    output
}

pub fn primitive_data_list_height(
    item_count: usize,
    orientation: DataListOrientation,
    size: DataListSize,
) -> f32 {
    let item_height = match orientation {
        DataListOrientation::Horizontal => size.horizontal_item_height(),
        DataListOrientation::Vertical => size.vertical_item_height(),
    };
    item_height * item_count as f32
}

pub fn primitive_data_list_root_output(
    rect: Rect,
    items: &[DataListItem<'_>],
    options: DataListRootOptions,
) -> DataListRootOutput {
    let visual = data_list_visual(options.variant, options.theme);
    let item_height = match options.orientation {
        DataListOrientation::Horizontal => options.size.horizontal_item_height(),
        DataListOrientation::Vertical => options.size.vertical_item_height(),
    };
    let horizontal_inset = match options.variant {
        DataListVariant::Plain => 0.0,
        DataListVariant::Surface => DATA_LIST_SURFACE_INSET,
    };
    let inner = rect.shrink2(egui::vec2(horizontal_inset, 0.0));
    let item_outputs = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let item_rect = Rect::from_min_size(
                inner.left_top() + egui::vec2(0.0, index as f32 * item_height),
                Vec2::new(inner.width(), item_height),
            );
            data_list_item_output(index, item_rect, *item, options, visual)
        })
        .collect();

    DataListRootOutput {
        rect,
        orientation: options.orientation,
        size: options.size,
        variant: options.variant,
        visual,
        items: item_outputs,
        data_orientation: options.orientation.as_str(),
        data_size: options.size.as_str(),
        data_variant: options.variant.as_str(),
        role: "list",
    }
}

fn data_list_item_output(
    index: usize,
    rect: Rect,
    item: DataListItem<'_>,
    options: DataListRootOptions,
    visual: DataListVisual,
) -> DataListItemOutput {
    let (label_rect, value_rect, label_pos, value_center_y) = match options.orientation {
        DataListOrientation::Horizontal => {
            let label_width = options.label_width.min(rect.width()).max(0.0);
            let label_rect =
                Rect::from_min_size(rect.left_top(), Vec2::new(label_width, rect.height()));
            let value_rect = Rect::from_min_max(
                egui::pos2(label_rect.right(), rect.top()),
                rect.right_bottom(),
            );
            (
                label_rect,
                value_rect,
                egui::pos2(label_rect.left(), label_rect.center().y),
                value_rect.center().y,
            )
        }
        DataListOrientation::Vertical => {
            let label_height = rect.height() * 0.45;
            let label_rect = Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.right(), rect.top() + label_height),
            );
            let value_rect = Rect::from_min_max(
                egui::pos2(rect.left(), label_rect.bottom()),
                rect.right_bottom(),
            );
            (
                label_rect,
                value_rect,
                egui::pos2(label_rect.left(), label_rect.center().y),
                value_rect.center().y,
            )
        }
    };
    let value_color = data_list_value_color(item.tone, options.theme);
    let icon = item.icon.map(|icon| DataListValueIconOutput {
        rect: Rect::from_center_size(
            egui::pos2(
                value_rect.left() + DATA_LIST_ICON_SIZE * 0.5,
                value_center_y,
            ),
            Vec2::splat(DATA_LIST_ICON_SIZE),
        ),
        icon,
        color: value_color,
        aria_hidden: true,
    });
    let value_text_x = value_rect.left()
        + if icon.is_some() {
            DATA_LIST_ICON_SIZE + DATA_LIST_ICON_GAP
        } else {
            0.0
        };
    let divider_rect = (options.show_dividers && index > 0)
        .then(|| Rect::from_min_size(rect.left_top(), Vec2::new(rect.width(), 1.0)));

    DataListItemOutput {
        rect,
        index,
        divider_rect,
        label: DataListLabelOutput {
            rect: label_rect,
            text_pos: label_pos,
            text: item.label.to_string(),
            color: visual.label,
            font_size: options.size.label_font_size(),
        },
        value: DataListValueOutput {
            rect: value_rect,
            text_rect: Rect::from_min_max(
                egui::pos2(value_text_x, value_rect.top()),
                value_rect.right_bottom(),
            ),
            text_pos: egui::pos2(value_text_x, value_center_y),
            text: item.value.to_string(),
            color: value_color,
            font_size: options.size.value_font_size(),
            icon,
            data_tone: item.tone.as_str(),
        },
        role: "listitem",
    }
}

pub fn data_list_visual(variant: DataListVariant, theme: PrimitiveTheme) -> DataListVisual {
    let (fill, stroke) = match variant {
        DataListVariant::Plain => (Color32::TRANSPARENT, Color32::TRANSPARENT),
        DataListVariant::Surface => (theme.content_fill, theme.content_stroke.color),
    };
    DataListVisual {
        fill,
        stroke,
        divider: faded_color(theme.content_stroke.color, 0.55),
        label: theme.muted_text,
        value: theme.text,
    }
}

fn data_list_value_color(tone: DataListValueTone, theme: PrimitiveTheme) -> Color32 {
    match tone {
        DataListValueTone::Muted => theme.muted_text,
        DataListValueTone::Neutral => theme.text,
        _ => {
            badge_visual(
                tone.badge_tone().expect("semantic tone has badge palette"),
                BadgeVariant::Outline,
                theme,
            )
            .text
        }
    }
}

fn faded_color(color: Color32, opacity: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        color.r(),
        color.g(),
        color.b(),
        (f32::from(color.a()) * opacity.clamp(0.0, 1.0)).round() as u8,
    )
}

#[cfg(test)]
mod tests;
