mod dialog;
mod layer;
mod overlay;
mod primitives;

use eframe::egui::{self, FontId};

pub use dialog::{CommandDialogOptions, CommandDialogOutput, show_command_dialog};
pub use layer::{
    AnchoredLayerOptions, DismissPolicy, LayerOutput, LayerPlacement, clamp_layer_pos,
    show_anchored_layer,
};
pub use overlay::{OverlayKind, modal_backdrop};
pub use primitives::*;

pub fn scaled_proportional_font(_ui: &egui::Ui, size: f32) -> FontId {
    FontId::proportional(size)
}

pub fn scaled_monospace_font(_ui: &egui::Ui, size: f32) -> FontId {
    FontId::monospace(size)
}

mod config {
    use eframe::egui::Color32;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TvTheme {
        pub popup_background: Color32,
        pub toolbar_border: Color32,
    }

    pub const TV_LIGHT: TvTheme = TvTheme {
        popup_background: Color32::from_rgb(0xfc, 0xfc, 0xfd),
        toolbar_border: Color32::from_rgb(0xd9, 0xd9, 0xe0),
    };

    pub fn tv_theme_for_dark_mode(dark_mode: bool) -> TvTheme {
        if dark_mode {
            TvTheme {
                popup_background: Color32::from_rgb(0x14, 0x18, 0x1f),
                toolbar_border: Color32::from_rgb(0x36, 0x3d, 0x49),
            }
        } else {
            TV_LIGHT
        }
    }
}
