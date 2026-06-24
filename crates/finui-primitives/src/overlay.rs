use eframe::egui::{self, Color32};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OverlayKind {
    SymbolSearch,
    SymbolResults,
    InstrumentMenu,
    ChartStyleMenu,
    CompareMenu,
    LayoutMenu,
    TimeframeMenu,
    IndicatorDialog,
    ScriptEditor,
    SettingsMenu,
    PrimitiveDemo,
    FinancialGridPanel,
    DrawingToolFlyout,
    DrawingColorMenu,
    DrawingStylePanel,
    DrawingStrokeWidthMenu,
    DrawingLineStyleMenu,
    DrawingMoreMenu,
    DrawingObjectManager,
}

pub fn modal_backdrop(
    ctx: &egui::Context,
    id: impl std::hash::Hash,
    tint: Color32,
) -> egui::Response {
    let screen = ctx.content_rect();
    egui::Area::new(egui::Id::new(id))
        .order(egui::Order::Foreground)
        .fixed_pos(screen.left_top())
        .show(ctx, |ui| {
            let (rect, response) = ui.allocate_exact_size(screen.size(), egui::Sense::click());
            ui.painter().rect_filled(rect, 0.0, tint);
            response
        })
        .inner
}
