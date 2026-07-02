use eframe::egui;
use finui_grid::{
    DemoFinancialGrid, FinancialGridDemoAction, grid_primitive_integration_scenario_output,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([760.0, 420.0]),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Finui Grid Lab",
        options,
        Box::new(|cc| {
            configure_lab_style(&cc.egui_ctx);
            Ok(Box::new(GridLabApp::default()))
        }),
    )
}

fn configure_lab_style(ctx: &egui::Context) {
    #[cfg(debug_assertions)]
    ctx.all_styles_mut(|style| {
        style.debug.debug_on_hover = false;
        style.debug.debug_on_hover_with_all_modifiers = false;
        style.debug.show_interactive_widgets = false;
        style.debug.show_widget_hits = false;
        style.debug.warn_if_rect_changes_id = false;
        style.debug.show_focused_widget = false;
        style.debug.show_unaligned = false;
    });
}

#[derive(Default)]
struct GridLabApp {
    grid: DemoFinancialGrid,
}

impl eframe::App for GridLabApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            if ui.button("Sort").clicked() {
                self.grid
                    .apply_agent_demo_action(FinancialGridDemoAction::SortChangeDesc);
            }
            if ui.button("Filter").clicked() {
                self.grid
                    .apply_agent_demo_action(FinancialGridDemoAction::FilterNameOrbit);
            }
            if ui.button("Reset").clicked() {
                self.grid
                    .apply_agent_demo_action(FinancialGridDemoAction::ClearFilters);
            }
            let integration = grid_primitive_integration_scenario_output();
            ui.label(if integration.passed() {
                "Primitive integration: ready"
            } else {
                "Primitive integration: check"
            });
        });
        self.grid.show(ui);
    }
}
