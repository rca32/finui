#[path = "financial_data_grid/mod.rs"]
mod modules;

use eframe::egui;

pub use modules::action::*;
pub use modules::agent::*;
#[cfg(test)]
pub use modules::analytics::*;
pub use modules::cell::*;
pub use modules::column::*;
#[cfg(test)]
pub use modules::core::*;
pub use modules::demo::*;
#[cfg(test)]
pub use modules::demo_data::*;
#[cfg(test)]
pub use modules::export::*;
pub use modules::ids::*;
#[cfg(test)]
pub use modules::persistence::*;
pub use modules::primitive_integration::*;
pub use modules::provenance::*;
pub use modules::source::*;
pub use modules::state::*;
#[cfg(test)]
pub use modules::viewport::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridSurfaceTheme {
    pub surface_fill: egui::Color32,
    pub surface_stroke: egui::Color32,
    pub header_fill: egui::Color32,
    pub body_fill: egui::Color32,
    pub header_text: egui::Color32,
    pub muted_text: egui::Color32,
    pub accent_text: egui::Color32,
    pub selected_row_fill: egui::Color32,
    pub row_alt_fill: egui::Color32,
    pub scrollbar_track: egui::Color32,
    pub scrollbar_stroke: egui::Color32,
    pub scrollbar_thumb: egui::Color32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridTextTheme {
    pub text: egui::Color32,
    pub positive_text: egui::Color32,
    pub negative_text: egui::Color32,
    pub error_text: egui::Color32,
    pub agent_text: egui::Color32,
    pub grid_line: egui::Color32,
    pub cell_font_size: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridHeaderTheme {
    pub height: f32,
    pub font_size: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridRowTheme {
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridRowInteractionTheme {
    pub hover_fill: egui::Color32,
}

pub struct FinancialDataGrid<'a> {
    id: &'a str,
    columns: &'a [GridColumnDef],
    source: &'a dyn GridRowSource,
    state: &'a mut GridState,
    density: GridDensity,
    provenance_policy: GridProvenancePolicy,
    agent_bridge: Option<&'a GridAgentBridge>,
    status_bar: bool,
    row_selection_only: bool,
    height: Option<f32>,
    theme_mode: Option<finui_primitives::ThemeMode>,
    surface_theme: Option<GridSurfaceTheme>,
    text_theme: Option<GridTextTheme>,
    header_theme: Option<GridHeaderTheme>,
    row_theme: Option<GridRowTheme>,
    row_interaction_theme: Option<GridRowInteractionTheme>,
    header_icons: bool,
}

impl<'a> FinancialDataGrid<'a> {
    pub fn new(id: &'a str) -> FinancialDataGridBuilder<'a> {
        FinancialDataGridBuilder {
            id,
            columns: None,
            source: None,
            state: None,
            density: GridDensity::Compact,
            provenance_policy: GridProvenancePolicy::VisibleOnHover,
            agent_bridge: None,
            status_bar: true,
            row_selection_only: false,
            height: None,
            theme_mode: None,
            surface_theme: None,
            text_theme: None,
            header_theme: None,
            row_theme: None,
            row_interaction_theme: None,
            header_icons: true,
        }
    }
}

pub struct FinancialDataGridBuilder<'a> {
    id: &'a str,
    columns: Option<&'a [GridColumnDef]>,
    source: Option<&'a dyn GridRowSource>,
    state: Option<&'a mut GridState>,
    density: GridDensity,
    provenance_policy: GridProvenancePolicy,
    agent_bridge: Option<&'a GridAgentBridge>,
    status_bar: bool,
    row_selection_only: bool,
    height: Option<f32>,
    theme_mode: Option<finui_primitives::ThemeMode>,
    surface_theme: Option<GridSurfaceTheme>,
    text_theme: Option<GridTextTheme>,
    header_theme: Option<GridHeaderTheme>,
    row_theme: Option<GridRowTheme>,
    row_interaction_theme: Option<GridRowInteractionTheme>,
    header_icons: bool,
}

impl<'a> FinancialDataGridBuilder<'a> {
    pub fn columns(mut self, columns: &'a [GridColumnDef]) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn source(mut self, source: &'a dyn GridRowSource) -> Self {
        self.source = Some(source);
        self
    }

    pub fn state(mut self, state: &'a mut GridState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn density(mut self, density: GridDensity) -> Self {
        self.density = density;
        self
    }

    pub fn provenance_policy(mut self, policy: GridProvenancePolicy) -> Self {
        self.provenance_policy = policy;
        self
    }

    pub fn agent_bridge(mut self, bridge: &'a GridAgentBridge) -> Self {
        self.agent_bridge = Some(bridge);
        self
    }

    pub fn status_bar(mut self, status_bar: bool) -> Self {
        self.status_bar = status_bar;
        self
    }

    pub fn row_selection_only(mut self, row_selection_only: bool) -> Self {
        self.row_selection_only = row_selection_only;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn theme_mode(mut self, theme_mode: finui_primitives::ThemeMode) -> Self {
        self.theme_mode = Some(theme_mode);
        self
    }

    pub fn surface_theme(mut self, theme: GridSurfaceTheme) -> Self {
        self.surface_theme = Some(theme);
        self
    }

    pub fn text_theme(mut self, theme: GridTextTheme) -> Self {
        self.text_theme = Some(theme);
        self
    }

    pub fn header_theme(mut self, theme: GridHeaderTheme) -> Self {
        self.header_theme = Some(theme);
        self
    }

    pub fn row_theme(mut self, theme: GridRowTheme) -> Self {
        self.row_theme = Some(theme);
        self
    }

    pub fn row_interaction_theme(mut self, theme: GridRowInteractionTheme) -> Self {
        self.row_interaction_theme = Some(theme);
        self
    }

    pub fn header_icons(mut self, header_icons: bool) -> Self {
        self.header_icons = header_icons;
        self
    }

    pub fn show(self, ui: &mut egui::Ui) -> GridOutput {
        FinancialDataGrid {
            id: self.id,
            columns: self
                .columns
                .expect("FinancialDataGrid columns are required"),
            source: self.source.expect("FinancialDataGrid source is required"),
            state: self.state.expect("FinancialDataGrid state is required"),
            density: self.density,
            provenance_policy: self.provenance_policy,
            agent_bridge: self.agent_bridge,
            status_bar: self.status_bar,
            row_selection_only: self.row_selection_only,
            height: self.height,
            theme_mode: self.theme_mode,
            surface_theme: self.surface_theme,
            text_theme: self.text_theme,
            header_theme: self.header_theme,
            row_theme: self.row_theme,
            row_interaction_theme: self.row_interaction_theme,
            header_icons: self.header_icons,
        }
        .show(ui)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn financial_data_grid_builder_accepts_explicit_height() {
        let columns = demo_columns();
        let source = InMemoryGridSource::new(demo_rows(2));
        let mut state = GridState::default();

        let builder = FinancialDataGrid::new("height-contract")
            .columns(&columns)
            .source(&source)
            .state(&mut state)
            .height(224.0);
        let FinancialDataGridBuilder {
            id,
            columns,
            source,
            state,
            density,
            provenance_policy,
            agent_bridge,
            status_bar,
            row_selection_only,
            height,
            theme_mode,
            surface_theme,
            text_theme,
            header_theme,
            row_theme,
            row_interaction_theme,
            header_icons,
        } = builder;
        let grid = FinancialDataGrid {
            id,
            columns: columns.expect("columns"),
            source: source.expect("source"),
            state: state.expect("state"),
            density,
            provenance_policy,
            agent_bridge,
            status_bar,
            row_selection_only,
            height,
            theme_mode,
            surface_theme,
            text_theme,
            header_theme,
            row_theme,
            row_interaction_theme,
            header_icons,
        };

        assert_eq!(grid.height, Some(224.0));
        assert_eq!(grid.theme_mode, None);
        assert_eq!(grid.surface_theme, None);
        assert_eq!(grid.text_theme, None);
        assert_eq!(grid.header_theme, None);
        assert_eq!(grid.row_theme, None);
        assert_eq!(grid.row_interaction_theme, None);
        assert!(grid.header_icons);
    }

    #[test]
    fn financial_data_grid_builder_accepts_explicit_text_theme() {
        let columns = demo_columns();
        let source = InMemoryGridSource::new(demo_rows(2));
        let mut state = GridState::default();
        let theme = GridTextTheme {
            text: egui::Color32::from_rgb(0xaa, 0xaa, 0xaa),
            positive_text: egui::Color32::from_rgb(0x40, 0xc0, 0x80),
            negative_text: egui::Color32::from_rgb(0xe0, 0x60, 0x68),
            error_text: egui::Color32::from_rgb(0xf0, 0xb4, 0x48),
            agent_text: egui::Color32::from_rgb(0x90, 0xa0, 0xff),
            grid_line: egui::Color32::from_rgb(0x28, 0x28, 0x30),
            cell_font_size: 11.0,
        };

        let builder = FinancialDataGrid::new("text-theme-contract")
            .columns(&columns)
            .source(&source)
            .state(&mut state)
            .text_theme(theme);
        let FinancialDataGridBuilder {
            id,
            columns,
            source,
            state,
            density,
            provenance_policy,
            agent_bridge,
            status_bar,
            row_selection_only,
            height,
            theme_mode,
            surface_theme,
            text_theme,
            header_theme,
            row_theme,
            row_interaction_theme,
            header_icons,
        } = builder;
        let grid = FinancialDataGrid {
            id,
            columns: columns.expect("columns"),
            source: source.expect("source"),
            state: state.expect("state"),
            density,
            provenance_policy,
            agent_bridge,
            status_bar,
            row_selection_only,
            height,
            theme_mode,
            surface_theme,
            text_theme,
            header_theme,
            row_theme,
            row_interaction_theme,
            header_icons,
        };

        assert_eq!(grid.text_theme, Some(theme));
    }

    #[test]
    fn financial_data_grid_builder_accepts_explicit_theme_mode() {
        let columns = demo_columns();
        let source = InMemoryGridSource::new(demo_rows(2));
        let mut state = GridState::default();

        let builder = FinancialDataGrid::new("theme-mode-contract")
            .columns(&columns)
            .source(&source)
            .state(&mut state)
            .theme_mode(finui_primitives::ThemeMode::Light);
        let FinancialDataGridBuilder { theme_mode, .. } = builder;

        assert_eq!(theme_mode, Some(finui_primitives::ThemeMode::Light));
    }

    #[test]
    fn financial_data_grid_builder_accepts_explicit_header_theme() {
        let columns = demo_columns();
        let source = InMemoryGridSource::new(demo_rows(2));
        let mut state = GridState::default();
        let theme = GridHeaderTheme {
            height: 22.0,
            font_size: 10.0,
        };

        let builder = FinancialDataGrid::new("header-theme-contract")
            .columns(&columns)
            .source(&source)
            .state(&mut state)
            .header_theme(theme);
        let FinancialDataGridBuilder { header_theme, .. } = builder;

        assert_eq!(header_theme, Some(theme));
    }

    #[test]
    fn financial_data_grid_builder_accepts_explicit_row_theme() {
        let columns = demo_columns();
        let source = InMemoryGridSource::new(demo_rows(2));
        let mut state = GridState::default();
        let theme = GridRowTheme { height: 21.0 };

        let builder = FinancialDataGrid::new("row-theme-contract")
            .columns(&columns)
            .source(&source)
            .state(&mut state)
            .row_theme(theme);
        let FinancialDataGridBuilder { row_theme, .. } = builder;

        assert_eq!(row_theme, Some(theme));
    }

    #[test]
    fn financial_data_grid_builder_accepts_explicit_row_interaction_theme() {
        let columns = demo_columns();
        let source = InMemoryGridSource::new(demo_rows(2));
        let mut state = GridState::default();
        let theme = GridRowInteractionTheme {
            hover_fill: egui::Color32::from_rgba_unmultiplied(0x20, 0x20, 0x28, 40),
        };

        let builder = FinancialDataGrid::new("row-interaction-theme-contract")
            .columns(&columns)
            .source(&source)
            .state(&mut state)
            .row_interaction_theme(theme);
        let FinancialDataGridBuilder {
            row_interaction_theme,
            ..
        } = builder;

        assert_eq!(row_interaction_theme, Some(theme));
    }

    #[test]
    fn financial_data_grid_builder_can_hide_header_icons() {
        let columns = demo_columns();
        let source = InMemoryGridSource::new(demo_rows(2));
        let mut state = GridState::default();

        let builder = FinancialDataGrid::new("header-icon-contract")
            .columns(&columns)
            .source(&source)
            .state(&mut state)
            .header_icons(false);
        let FinancialDataGridBuilder { header_icons, .. } = builder;

        assert!(!header_icons);
    }

    #[test]
    fn financial_data_grid_sorts_numeric_values() {
        let columns = demo_columns();
        let mut state = GridState {
            sort: vec![GridSort {
                column_id: GridColumnId::new("change_pct"),
                direction: GridSortDirection::Desc,
            }],
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        let source = InMemoryGridSource::new(demo_rows(4));

        let model = build_row_model(&source, &state, &columns);

        assert_eq!(source.row_id(model[0]), GridRowId::new("NOVA"));
        assert_eq!(source.row_id(model[1]), GridRowId::new("ACME"));
    }

    #[test]
    fn financial_data_grid_normalizes_visibility_and_pins() {
        let columns = demo_columns();
        let mut state = GridState {
            column_order: vec![GridColumnId::new("unknown"), GridColumnId::new("price")],
            pinned_left: vec![GridColumnId::new("price"), GridColumnId::new("price")],
            pinned_right: vec![GridColumnId::new("price"), GridColumnId::new("source")],
            ..GridState::default()
        };

        state.normalize_columns(&columns);

        assert!(!state.column_order.contains(&GridColumnId::new("unknown")));
        assert_eq!(state.pinned_left, vec![GridColumnId::new("price")]);
        assert_eq!(state.pinned_right, vec![GridColumnId::new("source")]);
        assert!(
            state
                .visible_column_ids()
                .contains(&GridColumnId::new("symbol"))
        );
    }

    #[test]
    fn financial_data_grid_filters_rows_by_column_value() {
        let columns = demo_columns();
        let mut state = GridState {
            filters: vec![GridFilter {
                column_id: GridColumnId::new("name"),
                query: "Orbit Search".to_owned(),
            }],
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        let source = InMemoryGridSource::new(demo_rows(8));

        let model = build_row_model(&source, &state, &columns);

        assert_eq!(model.len(), 2);
        assert_eq!(source.row_id(model[0]), GridRowId::new("ORBT"));
        assert_eq!(source.row_id(model[1]), GridRowId::new("900006"));
    }

    #[test]
    fn financial_data_grid_filters_numeric_predicates_by_typed_value() {
        let columns = demo_columns();
        let mut state = GridState {
            filters: vec![GridFilter {
                column_id: GridColumnId::new("change_pct"),
                query: ">=1.0".to_owned(),
            }],
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        let source = InMemoryGridSource::new(demo_rows(4));

        let model = build_row_model(&source, &state, &columns);
        let row_ids: Vec<_> = model.iter().map(|index| source.row_id(*index)).collect();

        assert_eq!(
            row_ids,
            vec![GridRowId::new("ACME"), GridRowId::new("NOVA")]
        );
    }

    #[test]
    fn financial_data_grid_filters_numeric_ranges() {
        let columns = demo_columns();
        let mut state = GridState {
            filters: vec![GridFilter {
                column_id: GridColumnId::new("change_pct"),
                query: "-1.0..1.5".to_owned(),
            }],
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        let source = InMemoryGridSource::new(demo_rows(4));

        let model = build_row_model(&source, &state, &columns);
        let row_ids: Vec<_> = model.iter().map(|index| source.row_id(*index)).collect();

        assert_eq!(
            row_ids,
            vec![GridRowId::new("ACME"), GridRowId::new("ORBT")]
        );
    }

    #[test]
    fn financial_data_grid_applies_multi_column_sort_in_order() {
        let columns = vec![
            GridColumnDef::new("group", "Group").sortable(true),
            GridColumnDef::new("rank", "Rank")
                .kind(GridValueKind::Integer)
                .sortable(true),
        ];
        let rows = [
            ("a1", "A", 2),
            ("a0", "A", 1),
            ("b1", "B", 2),
            ("b0", "B", 1),
        ]
        .into_iter()
        .map(|(id, group, rank)| {
            let mut cells = BTreeMap::new();
            cells.insert(
                GridColumnId::new("group"),
                GridCellValue::Text(group.to_owned()),
            );
            cells.insert(GridColumnId::new("rank"), GridCellValue::Integer(rank));
            InMemoryGridRow {
                id: GridRowId::new(id),
                cells,
                provenance: BTreeMap::new(),
            }
        })
        .collect();
        let source = InMemoryGridSource::new(rows);
        let mut state = GridState {
            sort: vec![
                GridSort {
                    column_id: GridColumnId::new("group"),
                    direction: GridSortDirection::Asc,
                },
                GridSort {
                    column_id: GridColumnId::new("rank"),
                    direction: GridSortDirection::Desc,
                },
            ],
            ..GridState::default()
        };
        state.normalize_columns(&columns);

        let model = build_row_model(&source, &state, &columns);
        let row_ids: Vec<_> = model.iter().map(|index| source.row_id(*index)).collect();

        assert_eq!(
            row_ids,
            vec![
                GridRowId::new("a1"),
                GridRowId::new("a0"),
                GridRowId::new("b1"),
                GridRowId::new("b0")
            ]
        );
    }

    #[test]
    fn financial_data_grid_horizontal_scroll_keeps_left_pin_visible() {
        let columns = demo_columns();
        let mut state = GridState {
            pinned_left: vec![GridColumnId::new("symbol")],
            horizontal_scroll: 120.0,
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        let visible = state.visible_column_ids();

        let center = scrolled_center_columns(
            &visible,
            &state.pinned_left,
            &state.pinned_right,
            &columns,
            &state,
            state.horizontal_scroll,
        );

        assert_eq!(state.pinned_left, vec![GridColumnId::new("symbol")]);
        assert!(!center.contains(&GridColumnId::new("symbol")));
        assert!(center.contains(&GridColumnId::new("price")));
    }

    #[test]
    fn financial_data_grid_column_layout_keeps_right_pin_at_viewport_edge() {
        let columns = demo_columns();
        let mut state = GridState {
            pinned_left: vec![GridColumnId::new("symbol")],
            pinned_right: vec![GridColumnId::new("status")],
            horizontal_scroll: 180.0,
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        let visible = state.visible_column_ids();
        let (pinned_left, pinned_right) = state.pinned_visible_columns();

        let layout = build_column_layout(
            10.0,
            610.0,
            &visible,
            &pinned_left,
            &pinned_right,
            &columns,
            &state,
            state.horizontal_scroll,
        );
        let status = layout
            .iter()
            .find(|layout| layout.column_id == GridColumnId::new("status"))
            .unwrap();

        assert_eq!(status.region, GridColumnRegion::PinnedRight);
        assert_eq!(status.x + status.width, 610.0);
        assert!(
            !layout
                .iter()
                .filter(|layout| layout.region == GridColumnRegion::Center)
                .any(|layout| layout.x + layout.width > status.x)
        );
    }

    #[test]
    fn financial_data_grid_state_tracks_vertical_viewport() {
        let columns = demo_columns();
        let mut state = GridState {
            row_scroll: 9_999,
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        let source = InMemoryGridSource::new(demo_rows(50_000));
        let model = build_row_model(&source, &state, &columns);

        let visible_count = 14;
        let max_first = model.len().saturating_sub(visible_count);
        state.row_scroll = state.row_scroll.min(max_first);
        let visible_rows = state.row_scroll..model.len().min(state.row_scroll + visible_count);

        assert_eq!(visible_rows.start, 9_999);
        assert_eq!(visible_rows.end - visible_rows.start, visible_count);
    }

    #[test]
    fn financial_data_grid_agent_bridge_records_reversible_before_after() {
        let columns = demo_columns();
        let mut state = GridState::default();
        state.normalize_columns(&columns);
        let mut bridge = GridAgentBridge::default();

        let actions = bridge.apply_command(
            &mut state,
            GridAgentCommand::PinColumn {
                column_id: GridColumnId::new("price"),
                side: GridPinSide::Left,
                reason_ref: "test".to_owned(),
            },
        );

        assert_eq!(actions.len(), 1);
        assert_eq!(state.pinned_left, vec![GridColumnId::new("price")]);
        assert_eq!(bridge.action_log.len(), 1);
        assert!(bridge.action_log[0].reversible);
        assert!(bridge.action_log[0].before.pinned_left.is_empty());
        assert_eq!(
            bridge.action_log[0].after.pinned_left,
            vec![GridColumnId::new("price")]
        );
        assert!(bridge.undo_last_reversible(&mut state));
        assert!(state.pinned_left.is_empty());
    }

    #[test]
    fn financial_data_grid_agent_snapshot_exposes_rows_columns_and_highlights() {
        let mut demo = DemoFinancialGrid::default();
        demo.apply_agent_demo_action(FinancialGridDemoAction::HighlightVolatile);
        demo.apply_agent_demo_action(FinancialGridDemoAction::SortChangeDesc);

        let snapshot = demo.snapshot();

        assert_eq!(snapshot.grid_id, "financial-data-grid-demo");
        assert_eq!(snapshot.row_count, 1_000);
        assert!(
            snapshot
                .visible_columns
                .contains(&GridColumnId::new("symbol"))
        );
        assert_eq!(snapshot.sort[0].column_id, GridColumnId::new("change_pct"));
        assert_eq!(snapshot.highlights.len(), 1);
        assert_eq!(
            snapshot.highlights[0].evidence_ref.as_deref(),
            Some("evidence:grid-demo:volatile")
        );
    }

    #[test]
    fn financial_data_grid_large_fixture_keeps_row_model_index_based() {
        let columns = demo_columns();
        let mut state = GridState::default();
        state.normalize_columns(&columns);
        let source = InMemoryGridSource::new(demo_rows(50_000));

        let model = build_row_model(&source, &state, &columns);

        assert_eq!(source.row_count(), 50_000);
        assert_eq!(model[0], 0);
        assert_eq!(model[49_999], 49_999);
    }

    #[test]
    fn financial_data_grid_virtual_source_uses_lazy_accessors() {
        let columns = vec![
            GridColumnDef::new("symbol", "Symbol").sortable(true),
            GridColumnDef::new("rank", "Rank")
                .kind(GridValueKind::Integer)
                .sortable(true),
        ];
        let source = VirtualGridSource::new(
            50_000,
            |row| GridRowId::new(format!("row-{row}")),
            |row, column_id| match column_id.0.as_str() {
                "symbol" => GridCellValue::Text(format!("9{row:05}")),
                "rank" => GridCellValue::Integer((50_000 - row) as i64),
                _ => GridCellValue::Empty,
            },
        );
        let mut state = GridState {
            filters: vec![GridFilter {
                column_id: GridColumnId::new("rank"),
                query: ">49998".to_owned(),
            }],
            ..GridState::default()
        };
        state.normalize_columns(&columns);

        let model = build_row_model(&source, &state, &columns);

        assert_eq!(model, vec![0, 1]);
        assert_eq!(source.row_id(model[0]), GridRowId::new("row-0"));
    }

    #[test]
    fn financial_data_grid_streaming_source_updates_and_deletes_by_row_id() {
        let mut source = StreamingGridSource::default();
        source.update_cell(
            GridRowId::new("ACME"),
            GridColumnId::new("price"),
            GridCellValue::Decimal(74200.0),
            None,
        );
        source.update_cell(
            GridRowId::new("NOVA"),
            GridColumnId::new("price"),
            GridCellValue::Decimal(231500.0),
            None,
        );

        assert_eq!(source.row_count(), 2);
        assert_eq!(source.revision(), 2);
        assert_eq!(
            source.cell_value(0, &GridColumnId::new("price")),
            GridCellValue::Decimal(74200.0)
        );

        source.update_cell(
            GridRowId::new("ACME"),
            GridColumnId::new("price"),
            GridCellValue::Decimal(74300.0),
            None,
        );
        assert_eq!(source.revision(), 3);
        assert_eq!(
            source.cell_value(0, &GridColumnId::new("price")),
            GridCellValue::Decimal(74300.0)
        );

        assert!(source.delete_row(&GridRowId::new("NOVA")));
        assert_eq!(source.row_count(), 1);
        assert_eq!(source.row_id(0), GridRowId::new("ACME"));
    }

    #[test]
    fn financial_data_grid_exports_visible_sorted_rows_as_tsv() {
        let columns = demo_columns();
        let mut state = GridState {
            sort: vec![GridSort {
                column_id: GridColumnId::new("change_pct"),
                direction: GridSortDirection::Desc,
            }],
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        state
            .column_visibility
            .insert(GridColumnId::new("volume"), false);
        let source = InMemoryGridSource::new(demo_rows(4));
        let row_model = build_row_model(&source, &state, &columns);

        let exported = export_rows(
            &source,
            &columns,
            &state,
            &row_model[0..2],
            &GridExportOptions::default(),
        );

        assert!(exported.starts_with("Symbol\tName\tPrice\tChange\tSource\tStatus"));
        assert!(!exported.contains("Volume"));
        assert!(exported.contains("NOVA"));
        assert!(exported.contains("ACME"));
    }

    #[test]
    fn financial_data_grid_persisted_state_excludes_transient_selection() {
        let columns = demo_columns();
        let mut state = GridState {
            pinned_left: vec![GridColumnId::new("symbol")],
            selection: GridSelection {
                selected_row: Some(GridRowId::new("ACME")),
                selected_cell: None,
                selected_range: None,
            },
            focused_cell: Some(GridCellRef {
                row_id: GridRowId::new("ACME"),
                column_id: GridColumnId::new("price"),
            }),
            ..GridState::default()
        };
        state.normalize_columns(&columns);
        let persisted = GridPersistedState::from_state(&state, GridDensity::Compact);
        let mut restored = GridState::default();

        persisted.apply_to_state(&mut restored);
        restored.normalize_columns(&columns);

        assert_eq!(persisted.version, GRID_PERSISTENCE_VERSION);
        assert_eq!(restored.pinned_left, vec![GridColumnId::new("symbol")]);
        assert!(restored.selection.selected_row.is_none());
        assert!(restored.focused_cell.is_none());
    }

    #[test]
    fn financial_data_grid_tracks_range_selection_column_reorder_and_edit_draft() {
        let columns = demo_columns();
        let mut state = GridState::default();
        state.normalize_columns(&columns);
        let anchor = GridCellRef {
            row_id: GridRowId::new("ACME"),
            column_id: GridColumnId::new("price"),
        };
        let focus = GridCellRef {
            row_id: GridRowId::new("NOVA"),
            column_id: GridColumnId::new("change_pct"),
        };

        state.select_range(anchor.clone(), focus.clone());
        assert_eq!(
            state.selection.selected_range,
            Some(GridCellRange::new(anchor, focus.clone()))
        );
        assert_eq!(state.focused_cell, Some(focus.clone()));

        assert!(state.reorder_column(&GridColumnId::new("change_pct"), 1));
        assert_eq!(state.column_order[1], GridColumnId::new("change_pct"));

        state.begin_edit(focus.clone(), GridCellValue::Decimal(std::f64::consts::PI));
        assert_eq!(
            state.edit_draft.as_ref().map(|draft| &draft.cell),
            Some(&focus)
        );
        state.cancel_edit();
        assert!(state.edit_draft.is_none());
    }

    #[test]
    fn financial_data_grid_flash_cells_are_transient() {
        let mut state = GridState::default();
        let cell = GridCellRef {
            row_id: GridRowId::new("ACME"),
            column_id: GridColumnId::new("price"),
        };

        state.mark_flash(cell.clone());
        assert!(state.flash_cells.contains_key(&cell));
        state.decay_flash_cells();
        assert!(state.flash_cells.contains_key(&cell));
        state.decay_flash_cells();
        state.decay_flash_cells();
        assert!(!state.flash_cells.contains_key(&cell));
    }

    #[test]
    fn financial_data_grid_groups_and_aggregates_numeric_values() {
        let columns = vec![
            GridColumnDef::new("sector", "Sector"),
            GridColumnDef::new("value", "Value")
                .kind(GridValueKind::Decimal)
                .sortable(true),
        ];
        let rows = [("r1", "IT", 10.0), ("r2", "IT", 30.0), ("r3", "Bio", 5.0)]
            .into_iter()
            .map(|(id, sector, value)| {
                let mut cells = BTreeMap::new();
                cells.insert(
                    GridColumnId::new("sector"),
                    GridCellValue::Text(sector.to_owned()),
                );
                cells.insert(GridColumnId::new("value"), GridCellValue::Decimal(value));
                InMemoryGridRow {
                    id: GridRowId::new(id),
                    cells,
                    provenance: BTreeMap::new(),
                }
            })
            .collect();
        let source = InMemoryGridSource::new(rows);
        let state = GridState::default();
        let row_model = build_row_model(&source, &state, &columns);

        let grouped = group_and_aggregate(
            &source,
            &row_model,
            &GridGroupSpec {
                column_id: GridColumnId::new("sector"),
            },
            &[GridAggregationSpec {
                column_id: GridColumnId::new("value"),
                aggregation: GridAggregation::Average,
            }],
        );

        let it = grouped.iter().find(|row| row.key == "IT").unwrap();
        assert_eq!(it.row_count, 2);
        assert_eq!(
            it.values.get(&GridColumnId::new("value")),
            Some(&GridCellValue::Decimal(20.0))
        );
    }

    #[test]
    fn financial_data_grid_provenance_stale_contract_uses_received_time() {
        let provenance = GridCellProvenance {
            source_kind: "api".to_owned(),
            endpoint: None,
            tr_code: None,
            request_id: None,
            source_timestamp: None,
            received_at: None,
            materialized_table: None,
            entity_resolution_ref: None,
            stale_after_ms: Some(100),
            evidence_ref: Some("evidence:test".to_owned()),
        };

        assert!(provenance.is_stale(250, Some(100)));
        assert!(!provenance.is_stale(150, Some(100)));
    }
}
