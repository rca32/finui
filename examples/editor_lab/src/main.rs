use eframe::egui;
use finui_primitives::{PrimitiveTheme, ThemeMode};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([960.0, 600.0]),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Finui Editor Lab",
        options,
        Box::new(|_| Ok(Box::new(EditorLabApp))),
    )
}

struct EditorLabApp;

impl eframe::App for EditorLabApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mode = if ui.visuals().dark_mode {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        let theme = PrimitiveTheme::for_mode(mode);

        ui.heading("Finui Editor Lab");
        ui.label("wgpu application runtime");
        ui.colored_label(theme.text, "Backend boundary ready");
    }
}
