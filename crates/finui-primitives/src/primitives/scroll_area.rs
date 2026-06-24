use eframe::egui::{self, Rect, Response, Stroke, Vec2};

use super::{PrimitiveTheme, radix_colors};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAreaRootType {
    Auto,
    Always,
    Hover,
    Scroll,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollAreaRootOptions {
    pub area_type: ScrollAreaRootType,
    pub max_height: f32,
    pub id_salt: Option<&'static str>,
    pub vertical_scroll_offset: Option<f32>,
    pub scroll_hide_delay_ms: u64,
    pub theme: PrimitiveTheme,
}

impl Default for ScrollAreaRootOptions {
    fn default() -> Self {
        Self {
            area_type: ScrollAreaRootType::Hover,
            max_height: 160.0,
            id_salt: None,
            vertical_scroll_offset: None,
            scroll_hide_delay_ms: 600,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl ScrollAreaRootOptions {
    pub fn area_type(mut self, area_type: ScrollAreaRootType) -> Self {
        self.area_type = area_type;
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn id_salt(mut self, id_salt: &'static str) -> Self {
        self.id_salt = Some(id_salt);
        self
    }

    pub fn vertical_scroll_offset(mut self, vertical_scroll_offset: f32) -> Self {
        self.vertical_scroll_offset = Some(vertical_scroll_offset.max(0.0));
        self
    }

    pub fn scroll_hide_delay_ms(mut self, scroll_hide_delay_ms: u64) -> Self {
        self.scroll_hide_delay_ms = scroll_hide_delay_ms;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollAreaViewportOptions {
    pub bounds: Rect,
    pub scrollbar_width: f32,
}

impl ScrollAreaViewportOptions {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            scrollbar_width: 12.0,
        }
    }

    pub fn scrollbar_width(mut self, scrollbar_width: f32) -> Self {
        self.scrollbar_width = scrollbar_width;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAreaScrollbarOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollAreaScrollbarOptions {
    pub bounds: Rect,
    pub size: f32,
    pub orientation: ScrollAreaScrollbarOrientation,
    pub force_mount: bool,
    pub theme: PrimitiveTheme,
}

impl ScrollAreaScrollbarOptions {
    pub fn vertical(bounds: Rect) -> Self {
        Self {
            bounds,
            size: 12.0,
            orientation: ScrollAreaScrollbarOrientation::Vertical,
            force_mount: false,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn horizontal(bounds: Rect) -> Self {
        Self {
            bounds,
            size: 12.0,
            orientation: ScrollAreaScrollbarOrientation::Horizontal,
            force_mount: false,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

pub fn primitive_scroll_area<R>(
    ui: &mut egui::Ui,
    max_height: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::scroll_area::ScrollAreaOutput<R> {
    egui::ScrollArea::vertical()
        .max_height(max_height)
        .auto_shrink([false, false])
        .show(ui, add_contents)
}

pub fn primitive_scroll_area_root<R>(
    ui: &mut egui::Ui,
    options: ScrollAreaRootOptions,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::scroll_area::ScrollAreaOutput<R> {
    ui.scope(|ui| {
        let mut scroll_style = egui::style::ScrollStyle::solid();
        scroll_style.bar_width = 8.0;
        scroll_style.handle_min_length = 24.0;
        scroll_style.bar_inner_margin = 3.0;
        scroll_style.bar_outer_margin = 1.0;
        ui.style_mut().spacing.scroll = scroll_style;

        let mut scroll_area = egui::ScrollArea::vertical()
            .max_height(options.max_height)
            .auto_shrink([false, false]);
        if let Some(id_salt) = options.id_salt {
            scroll_area = scroll_area.id_salt(id_salt);
        }
        if let Some(vertical_scroll_offset) = options.vertical_scroll_offset {
            scroll_area = scroll_area.vertical_scroll_offset(vertical_scroll_offset);
        }
        scroll_area.show(ui, add_contents)
    })
    .inner
}

pub fn scroll_thumb_size(content_height: f32, viewport_height: f32, track_height: f32) -> f32 {
    if content_height <= 0.0 || viewport_height <= 0.0 || track_height <= 0.0 {
        return 0.0;
    }
    if content_height <= viewport_height {
        return track_height;
    }
    if track_height <= 24.0 {
        return track_height;
    }
    (viewport_height / content_height * track_height).clamp(24.0, track_height)
}

pub fn primitive_scroll_viewport_rect(bounds: Rect, scrollbar_width: f32) -> Rect {
    let right = (bounds.right() - scrollbar_width.max(0.0)).max(bounds.left());
    Rect::from_min_max(bounds.left_top(), egui::pos2(right, bounds.bottom()))
}

pub fn primitive_scroll_viewport_rect_with_options(options: ScrollAreaViewportOptions) -> Rect {
    primitive_scroll_viewport_rect(options.bounds, options.scrollbar_width)
}

pub fn primitive_scrollbar_rect(bounds: Rect, scrollbar_width: f32) -> Rect {
    let width = scrollbar_width.max(0.0).min(bounds.width());
    Rect::from_min_max(
        egui::pos2(bounds.right() - width, bounds.top()),
        bounds.right_bottom(),
    )
}

pub fn primitive_scrollbar_rect_with_options(options: ScrollAreaScrollbarOptions) -> Rect {
    match options.orientation {
        ScrollAreaScrollbarOrientation::Vertical => {
            primitive_scrollbar_rect(options.bounds, options.size)
        }
        ScrollAreaScrollbarOrientation::Horizontal => {
            let height = options.size.max(0.0).min(options.bounds.height());
            Rect::from_min_max(
                egui::pos2(options.bounds.left(), options.bounds.bottom() - height),
                options.bounds.right_bottom(),
            )
        }
    }
}

pub fn primitive_scroll_thumb_rect(
    scrollbar: Rect,
    content_height: f32,
    viewport_height: f32,
    offset_fraction: f32,
) -> Rect {
    let thumb_height = scroll_thumb_size(content_height, viewport_height, scrollbar.height());
    if thumb_height <= 0.0 {
        return Rect::from_min_size(scrollbar.left_top(), Vec2::ZERO);
    }
    let travel = (scrollbar.height() - thumb_height).max(0.0);
    let top = scrollbar.top() + travel * offset_fraction.clamp(0.0, 1.0);
    Rect::from_min_size(
        egui::pos2(scrollbar.left(), top),
        Vec2::new(scrollbar.width(), thumb_height),
    )
}

pub fn primitive_scrollbar(ui: &egui::Ui, rect: Rect, theme: PrimitiveTheme) {
    ui.painter()
        .rect_filled(rect, theme.row_radius, theme.content_fill);
    ui.painter().rect_stroke(
        rect,
        theme.row_radius,
        theme.content_stroke,
        egui::StrokeKind::Inside,
    );
}

pub fn primitive_scroll_thumb(ui: &egui::Ui, rect: Rect, theme: PrimitiveTheme) {
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    ui.painter()
        .rect_filled(rect.shrink(2.0), theme.row_radius, theme.disabled_text);
}

pub fn primitive_scroll_corner(ui: &egui::Ui, rect: Rect, theme: PrimitiveTheme) {
    ui.painter()
        .rect_filled(rect, theme.row_radius, theme.content_fill);
    ui.painter().rect_stroke(
        rect,
        theme.row_radius,
        theme.content_stroke,
        egui::StrokeKind::Inside,
    );
}

pub fn scroll_corner_placeholder(ui: &mut egui::Ui) -> Response {
    ui.allocate_response(Vec2::splat(12.0), egui::Sense::hover())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scroll_thumb_size_is_bounded() {
        assert_eq!(scroll_thumb_size(100.0, 100.0, 80.0), 80.0);
        assert_eq!(scroll_thumb_size(1000.0, 100.0, 80.0), 24.0);
        assert_eq!(scroll_thumb_size(1000.0, 100.0, 2.0), 2.0);
    }

    #[test]
    fn scroll_part_rects_reserve_vertical_scrollbar() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(120.0, 80.0));
        let viewport = primitive_scroll_viewport_rect(bounds, 12.0);
        let scrollbar = primitive_scrollbar_rect(bounds, 12.0);
        let thumb = primitive_scroll_thumb_rect(scrollbar, 240.0, 80.0, 0.5);

        assert_eq!(viewport.width(), 108.0);
        assert_eq!(scrollbar.width(), 12.0);
        assert!(thumb.top() > scrollbar.top());
        assert!(thumb.bottom() < scrollbar.bottom());
    }

    #[test]
    fn scroll_area_root_options_preserve_type_delay_height_and_theme() {
        let theme = PrimitiveTheme {
            row_radius: 8.0,
            ..PrimitiveTheme::default()
        };
        let options = ScrollAreaRootOptions::default()
            .area_type(ScrollAreaRootType::Always)
            .max_height(240.0)
            .id_salt("test_scroll_area")
            .vertical_scroll_offset(80.0)
            .scroll_hide_delay_ms(300)
            .theme(theme);

        assert_eq!(options.area_type, ScrollAreaRootType::Always);
        assert_eq!(options.max_height, 240.0);
        assert_eq!(options.id_salt, Some("test_scroll_area"));
        assert_eq!(options.vertical_scroll_offset, Some(80.0));
        assert_eq!(options.scroll_hide_delay_ms, 300);
        assert_eq!(options.theme.row_radius, 8.0);
    }

    #[test]
    fn scroll_area_viewport_options_drive_viewport_rect() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(120.0, 80.0));
        let viewport = primitive_scroll_viewport_rect_with_options(
            ScrollAreaViewportOptions::new(bounds).scrollbar_width(16.0),
        );

        assert_eq!(viewport.width(), 104.0);
        assert_eq!(viewport.height(), 80.0);
    }

    #[test]
    fn scroll_area_scrollbar_options_support_vertical_and_horizontal() {
        let bounds = Rect::from_min_size(egui::pos2(10.0, 20.0), Vec2::new(120.0, 80.0));
        let vertical = primitive_scrollbar_rect_with_options(
            ScrollAreaScrollbarOptions::vertical(bounds).size(14.0),
        );
        let horizontal = primitive_scrollbar_rect_with_options(
            ScrollAreaScrollbarOptions::horizontal(bounds)
                .size(10.0)
                .force_mount(true),
        );

        assert_eq!(vertical.width(), 14.0);
        assert_eq!(vertical.height(), 80.0);
        assert_eq!(horizontal.width(), 120.0);
        assert_eq!(horizontal.height(), 10.0);
    }
}
