use eframe::egui;
use finui_primitives::{
    PrimitiveActionKind, PrimitiveCheckboxOptions, PrimitiveTheme, primitive_action_button,
    primitive_checkbox,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Finui Primitives Lab",
        options,
        Box::new(|_cc| Ok(Box::new(PrimitivesLabApp::default()))),
    )
}

#[derive(Default)]
struct PrimitivesLabApp {
    checked: bool,
}

impl eframe::App for PrimitivesLabApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme = PrimitiveTheme::for_mode(if ui.ctx().global_style().visuals.dark_mode {
            finui_primitives::ThemeMode::Dark
        } else {
            finui_primitives::ThemeMode::Light
        });
        ui.heading("Finui primitives");
        ui.horizontal(|ui| {
            primitive_checkbox(
                ui,
                "alerts",
                &mut self.checked,
                "Enable alerts",
                PrimitiveCheckboxOptions {
                    theme,
                    ..PrimitiveCheckboxOptions::default()
                },
            );
            let _ = primitive_action_button(ui, "Apply", PrimitiveActionKind::Primary);
        });
    }
}
