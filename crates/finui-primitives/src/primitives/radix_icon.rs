use eframe::egui::{self, Color32, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadixIcon {
    Accessibility,
    ActivityLog,
    BarChart,
    BorderSplit,
    CaretDown,
    CaretSort,
    CaretUp,
    Check,
    ChevronDown,
    ChevronRight,
    ChevronUp,
    Circle,
    Columns,
    Copy,
    Cross2,
    Dashboard,
    Database,
    DotsHorizontal,
    DotFilled,
    Download,
    DragHandleDots2,
    DragHandleVertical,
    DrawingPin,
    DrawingPinFilled,
    EnterFullScreen,
    ExitFullScreen,
    EyeClosed,
    EyeOpen,
    ExclamationTriangle,
    FileText,
    Filter,
    InfoCircled,
    Link2,
    LinkBreak2,
    MixerHorizontal,
    Pause,
    PinLeft,
    PinRight,
    Play,
    Plus,
    QuestionMarkCircled,
    Reader,
    Reload,
    Reset,
    Resume,
    Rows,
    Server,
    Stack,
    Table,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadixIconAsset {
    pub filename: &'static str,
    pub svg: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadixIconVisual<'a> {
    pub icon: Option<RadixIcon>,
    pub fallback_text: Option<&'a str>,
}

pub fn radix_icon_asset(icon: RadixIcon) -> RadixIconAsset {
    match icon {
        RadixIcon::Accessibility => RadixIconAsset {
            filename: "accessibility.svg",
            svg: include_str!("../../assets/radix-icons/accessibility.svg"),
        },
        RadixIcon::ActivityLog => RadixIconAsset {
            filename: "activity-log.svg",
            svg: include_str!("../../assets/radix-icons/activity-log.svg"),
        },
        RadixIcon::BarChart => RadixIconAsset {
            filename: "bar-chart.svg",
            svg: include_str!("../../assets/radix-icons/bar-chart.svg"),
        },
        RadixIcon::BorderSplit => RadixIconAsset {
            filename: "border-split.svg",
            svg: include_str!("../../assets/radix-icons/border-split.svg"),
        },
        RadixIcon::CaretDown => RadixIconAsset {
            filename: "caret-down.svg",
            svg: include_str!("../../assets/radix-icons/caret-down.svg"),
        },
        RadixIcon::CaretSort => RadixIconAsset {
            filename: "caret-sort.svg",
            svg: include_str!("../../assets/radix-icons/caret-sort.svg"),
        },
        RadixIcon::CaretUp => RadixIconAsset {
            filename: "caret-up.svg",
            svg: include_str!("../../assets/radix-icons/caret-up.svg"),
        },
        RadixIcon::Check => RadixIconAsset {
            filename: "check.svg",
            svg: include_str!("../../assets/radix-icons/check.svg"),
        },
        RadixIcon::ChevronDown => RadixIconAsset {
            filename: "chevron-down.svg",
            svg: include_str!("../../assets/radix-icons/chevron-down.svg"),
        },
        RadixIcon::ChevronRight => RadixIconAsset {
            filename: "chevron-right.svg",
            svg: include_str!("../../assets/radix-icons/chevron-right.svg"),
        },
        RadixIcon::ChevronUp => RadixIconAsset {
            filename: "chevron-up.svg",
            svg: include_str!("../../assets/radix-icons/chevron-up.svg"),
        },
        RadixIcon::Circle => RadixIconAsset {
            filename: "circle.svg",
            svg: include_str!("../../assets/radix-icons/circle.svg"),
        },
        RadixIcon::Columns => RadixIconAsset {
            filename: "columns.svg",
            svg: include_str!("../../assets/radix-icons/columns.svg"),
        },
        RadixIcon::Copy => RadixIconAsset {
            filename: "copy.svg",
            svg: include_str!("../../assets/radix-icons/copy.svg"),
        },
        RadixIcon::Cross2 => RadixIconAsset {
            filename: "cross-2.svg",
            svg: include_str!("../../assets/radix-icons/cross-2.svg"),
        },
        RadixIcon::Dashboard => RadixIconAsset {
            filename: "dashboard.svg",
            svg: include_str!("../../assets/radix-icons/dashboard.svg"),
        },
        RadixIcon::Database => RadixIconAsset {
            filename: "database.svg",
            svg: include_str!("../../assets/radix-icons/database.svg"),
        },
        RadixIcon::DotsHorizontal => RadixIconAsset {
            filename: "dots-horizontal.svg",
            svg: include_str!("../../assets/radix-icons/dots-horizontal.svg"),
        },
        RadixIcon::DotFilled => RadixIconAsset {
            filename: "dot-filled.svg",
            svg: include_str!("../../assets/radix-icons/dot-filled.svg"),
        },
        RadixIcon::Download => RadixIconAsset {
            filename: "download.svg",
            svg: include_str!("../../assets/radix-icons/download.svg"),
        },
        RadixIcon::DragHandleDots2 => RadixIconAsset {
            filename: "drag-handle-dots-2.svg",
            svg: include_str!("../../assets/radix-icons/drag-handle-dots-2.svg"),
        },
        RadixIcon::DragHandleVertical => RadixIconAsset {
            filename: "drag-handle-vertical.svg",
            svg: include_str!("../../assets/radix-icons/drag-handle-vertical.svg"),
        },
        RadixIcon::DrawingPin => RadixIconAsset {
            filename: "drawing-pin.svg",
            svg: include_str!("../../assets/radix-icons/drawing-pin.svg"),
        },
        RadixIcon::DrawingPinFilled => RadixIconAsset {
            filename: "drawing-pin-filled.svg",
            svg: include_str!("../../assets/radix-icons/drawing-pin-filled.svg"),
        },
        RadixIcon::EnterFullScreen => RadixIconAsset {
            filename: "enter-full-screen.svg",
            svg: include_str!("../../assets/radix-icons/enter-full-screen.svg"),
        },
        RadixIcon::ExitFullScreen => RadixIconAsset {
            filename: "exit-full-screen.svg",
            svg: include_str!("../../assets/radix-icons/exit-full-screen.svg"),
        },
        RadixIcon::EyeClosed => RadixIconAsset {
            filename: "eye-closed.svg",
            svg: include_str!("../../assets/radix-icons/eye-closed.svg"),
        },
        RadixIcon::EyeOpen => RadixIconAsset {
            filename: "eye-open.svg",
            svg: include_str!("../../assets/radix-icons/eye-open.svg"),
        },
        RadixIcon::ExclamationTriangle => RadixIconAsset {
            filename: "exclamation-triangle.svg",
            svg: include_str!("../../assets/radix-icons/exclamation-triangle.svg"),
        },
        RadixIcon::FileText => RadixIconAsset {
            filename: "file-text.svg",
            svg: include_str!("../../assets/radix-icons/file-text.svg"),
        },
        RadixIcon::Filter => RadixIconAsset {
            filename: "filter.svg",
            svg: include_str!("../../assets/radix-icons/filter.svg"),
        },
        RadixIcon::InfoCircled => RadixIconAsset {
            filename: "info-circled.svg",
            svg: include_str!("../../assets/radix-icons/info-circled.svg"),
        },
        RadixIcon::Link2 => RadixIconAsset {
            filename: "link-2.svg",
            svg: include_str!("../../assets/radix-icons/link-2.svg"),
        },
        RadixIcon::LinkBreak2 => RadixIconAsset {
            filename: "link-break-2.svg",
            svg: include_str!("../../assets/radix-icons/link-break-2.svg"),
        },
        RadixIcon::MixerHorizontal => RadixIconAsset {
            filename: "mixer-horizontal.svg",
            svg: include_str!("../../assets/radix-icons/mixer-horizontal.svg"),
        },
        RadixIcon::Pause => RadixIconAsset {
            filename: "pause.svg",
            svg: include_str!("../../assets/radix-icons/pause.svg"),
        },
        RadixIcon::PinLeft => RadixIconAsset {
            filename: "pin-left.svg",
            svg: include_str!("../../assets/radix-icons/pin-left.svg"),
        },
        RadixIcon::PinRight => RadixIconAsset {
            filename: "pin-right.svg",
            svg: include_str!("../../assets/radix-icons/pin-right.svg"),
        },
        RadixIcon::Play => RadixIconAsset {
            filename: "play.svg",
            svg: include_str!("../../assets/radix-icons/play.svg"),
        },
        RadixIcon::Plus => RadixIconAsset {
            filename: "plus.svg",
            svg: include_str!("../../assets/radix-icons/plus.svg"),
        },
        RadixIcon::QuestionMarkCircled => RadixIconAsset {
            filename: "question-mark-circled.svg",
            svg: include_str!("../../assets/radix-icons/question-mark-circled.svg"),
        },
        RadixIcon::Reader => RadixIconAsset {
            filename: "reader.svg",
            svg: include_str!("../../assets/radix-icons/reader.svg"),
        },
        RadixIcon::Reload => RadixIconAsset {
            filename: "reload.svg",
            svg: include_str!("../../assets/radix-icons/reload.svg"),
        },
        RadixIcon::Reset => RadixIconAsset {
            filename: "reset.svg",
            svg: include_str!("../../assets/radix-icons/reset.svg"),
        },
        RadixIcon::Resume => RadixIconAsset {
            filename: "resume.svg",
            svg: include_str!("../../assets/radix-icons/resume.svg"),
        },
        RadixIcon::Rows => RadixIconAsset {
            filename: "rows.svg",
            svg: include_str!("../../assets/radix-icons/rows.svg"),
        },
        RadixIcon::Server => RadixIconAsset {
            filename: "server.svg",
            svg: include_str!("../../assets/radix-icons/server.svg"),
        },
        RadixIcon::Stack => RadixIconAsset {
            filename: "stack.svg",
            svg: include_str!("../../assets/radix-icons/stack.svg"),
        },
        RadixIcon::Table => RadixIconAsset {
            filename: "table.svg",
            svg: include_str!("../../assets/radix-icons/table.svg"),
        },
    }
}

pub fn radix_icon_from_visual(visual: &str) -> Option<RadixIcon> {
    match visual {
        "accessibility" => Some(RadixIcon::Accessibility),
        "activity-log" => Some(RadixIcon::ActivityLog),
        "bar-chart" | "chart" => Some(RadixIcon::BarChart),
        "border-split" | "split" => Some(RadixIcon::BorderSplit),
        "caret-down" => Some(RadixIcon::CaretDown),
        "caret-sort" => Some(RadixIcon::CaretSort),
        "caret-up" => Some(RadixIcon::CaretUp),
        "check" | "✓" => Some(RadixIcon::Check),
        "chevron-down" => Some(RadixIcon::ChevronDown),
        "chevron-right" => Some(RadixIcon::ChevronRight),
        "chevron-up" => Some(RadixIcon::ChevronUp),
        "circle" | "independent" => Some(RadixIcon::Circle),
        "columns" => Some(RadixIcon::Columns),
        "copy" => Some(RadixIcon::Copy),
        "cross-2" | "close" | "x" => Some(RadixIcon::Cross2),
        "dashboard" | "analysis" => Some(RadixIcon::Dashboard),
        "database" => Some(RadixIcon::Database),
        "dots-horizontal" | "menu" | "..." => Some(RadixIcon::DotsHorizontal),
        "dot-filled" | "live-dot" => Some(RadixIcon::DotFilled),
        "download" => Some(RadixIcon::Download),
        "drag-handle-dots-2" | "drag" => Some(RadixIcon::DragHandleDots2),
        "drag-handle-vertical" => Some(RadixIcon::DragHandleVertical),
        "drawing-pin" | "pin" => Some(RadixIcon::DrawingPin),
        "drawing-pin-filled" | "pinned" => Some(RadixIcon::DrawingPinFilled),
        "enter-full-screen" | "maximize" => Some(RadixIcon::EnterFullScreen),
        "exit-full-screen" | "restore" => Some(RadixIcon::ExitFullScreen),
        "eye-closed" => Some(RadixIcon::EyeClosed),
        "eye-open" => Some(RadixIcon::EyeOpen),
        "exclamation-triangle" | "warning" => Some(RadixIcon::ExclamationTriangle),
        "file-text" | "news" => Some(RadixIcon::FileText),
        "filter" => Some(RadixIcon::Filter),
        "info" | "i" => Some(RadixIcon::InfoCircled),
        "link-2" | "linked" => Some(RadixIcon::Link2),
        "link-break-2" | "unresolved" | "broken-link" => Some(RadixIcon::LinkBreak2),
        "mixer-horizontal" => Some(RadixIcon::MixerHorizontal),
        "pause" => Some(RadixIcon::Pause),
        "pin-left" => Some(RadixIcon::PinLeft),
        "pin-right" => Some(RadixIcon::PinRight),
        "play" => Some(RadixIcon::Play),
        "plus" | "add" | "+" => Some(RadixIcon::Plus),
        "question" | "?" => Some(RadixIcon::QuestionMarkCircled),
        "reader" | "evidence" => Some(RadixIcon::Reader),
        "reload" | "refresh" => Some(RadixIcon::Reload),
        "reset" => Some(RadixIcon::Reset),
        "resume" => Some(RadixIcon::Resume),
        "rows" => Some(RadixIcon::Rows),
        "server" | "endpoint" => Some(RadixIcon::Server),
        "stack" => Some(RadixIcon::Stack),
        "table" => Some(RadixIcon::Table),
        _ => None,
    }
}

pub fn radix_icon_visual(visual: &str) -> RadixIconVisual<'_> {
    match radix_icon_from_visual(visual) {
        Some(icon) => RadixIconVisual {
            icon: Some(icon),
            fallback_text: None,
        },
        None => RadixIconVisual {
            icon: None,
            fallback_text: Some(visual),
        },
    }
}

pub fn radix_icon_tintable_svg(asset: RadixIconAsset) -> String {
    asset.svg.replace("currentColor", "#FFFFFF")
}

pub fn paint_radix_icon(ui: &egui::Ui, icon: RadixIcon, rect: Rect, color: Color32) {
    egui_extras::install_image_loaders(ui.ctx());
    let asset = radix_icon_asset(icon);
    let svg = radix_icon_tintable_svg(asset);
    let image = egui::Image::from_bytes(
        format!("bytes://radix-icons/tintable/{}", asset.filename),
        svg.into_bytes(),
    )
    .tint(color)
    .fit_to_exact_size(rect.size());
    image.paint_at(ui, rect);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workstation_reference_icons_are_asset_backed() {
        for (name, expected) in [
            ("link-2", RadixIcon::Link2),
            ("link-break-2", RadixIcon::LinkBreak2),
            ("database", RadixIcon::Database),
            ("table", RadixIcon::Table),
            ("bar-chart", RadixIcon::BarChart),
            ("dashboard", RadixIcon::Dashboard),
            ("file-text", RadixIcon::FileText),
            ("server", RadixIcon::Server),
            ("exclamation-triangle", RadixIcon::ExclamationTriangle),
            ("filter", RadixIcon::Filter),
            ("columns", RadixIcon::Columns),
            ("download", RadixIcon::Download),
            ("reset", RadixIcon::Reset),
            ("border-split", RadixIcon::BorderSplit),
            ("stack", RadixIcon::Stack),
        ] {
            let icon = radix_icon_from_visual(name).expect(name);
            assert_eq!(icon, expected);
            assert!(radix_icon_asset(icon).svg.starts_with("<svg"));
        }
    }

    #[test]
    fn icon_visual_uses_text_fallback_for_unknown_visuals() {
        let known = radix_icon_visual("refresh");
        assert_eq!(known.icon, Some(RadixIcon::Reload));
        assert_eq!(known.fallback_text, None);

        let unknown = radix_icon_visual("custom-korean-label");
        assert_eq!(unknown.icon, None);
        assert_eq!(unknown.fallback_text, Some("custom-korean-label"));
    }

    #[test]
    fn icon_assets_are_tintable_before_egui_tint_is_applied() {
        let asset = radix_icon_asset(RadixIcon::Check);
        let svg = radix_icon_tintable_svg(asset);

        assert!(asset.svg.contains("currentColor"));
        assert!(!svg.contains("currentColor"));
        assert!(svg.contains("#FFFFFF"));
    }
}
