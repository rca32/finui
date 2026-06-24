use eframe::egui;
use finui_grid::{DemoFinancialGrid, FinancialGridDemoAction};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Finui Grid Lab",
        options,
        Box::new(|_cc| Ok(Box::new(GridLabApp::default()))),
    )
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
        });
        self.grid.show(ui);
    }
}
