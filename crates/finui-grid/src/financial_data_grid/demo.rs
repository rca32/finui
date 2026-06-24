use std::time::{Duration, Instant};

use eframe::egui;

use super::action::{GridDensity, GridOutput, GridProvenancePolicy};
use super::agent::{GridAgentBridge, GridAgentCommand, GridEvidenceTarget, GridSnapshot};
use super::analytics::{
    GridAggregation, GridAggregationSpec, GridGroupSpec, facet_counts, group_and_aggregate,
    pivot_aggregate,
};
use super::cell::GridCellValue;
use super::column::GridColumnDef;
use super::core::build_row_model;
use super::demo_data::{demo_columns, demo_rows};
use super::export::{
    GridExportFormat, GridExportOptions, GridExportValueMode, export_selected_row,
};
use super::ids::{GridCellRef, GridColumnId, GridRowId};
use super::persistence::GridPersistedState;
use super::provenance::GridCellProvenance;
use super::source::{GridRowSource, InMemoryGridRow, InMemoryGridSource, VirtualGridSource};
use super::state::{GridFilter, GridPinSide, GridSort, GridSortDirection, GridState};
use crate::FinancialDataGrid;

enum DemoGridSource {
    Memory(InMemoryGridSource),
    PagedVirtual(VirtualGridSource),
}

impl std::fmt::Debug for DemoGridSource {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("DemoGridSource")
            .field("kind", &self.kind())
            .field("row_count", &self.row_count())
            .finish()
    }
}

impl DemoGridSource {
    fn memory(rows: Vec<InMemoryGridRow>) -> Self {
        Self::Memory(InMemoryGridSource::new(rows))
    }

    fn paged_virtual(row_count: usize) -> Self {
        Self::PagedVirtual(
            VirtualGridSource::new(
                row_count,
                |row_index| GridRowId::new(format!("PV{row_index:06}")),
                |row_index, column_id| paged_virtual_cell_value(row_index, column_id),
            )
            .with_provenance(|row_index, column_id| {
                Some(GridCellProvenance {
                    source_kind: "paged-virtual".to_owned(),
                    endpoint: Some("/api/grid/page".to_owned()),
                    tr_code: None,
                    request_id: Some(format!("page-{}", row_index / 100)),
                    source_timestamp: Some("2026-06-04T09:30:00+09:00".to_owned()),
                    received_at: Some("2026-06-04T09:30:01+09:00".to_owned()),
                    materialized_table: None,
                    entity_resolution_ref: Some(format!("virtual:{}", column_id.0)),
                    stale_after_ms: Some(5_000),
                    evidence_ref: Some("evidence:grid-demo:paged-virtual".to_owned()),
                })
            }),
        )
    }

    fn kind(&self) -> &'static str {
        match self {
            Self::Memory(_) => "memory",
            Self::PagedVirtual(_) => "paged-virtual",
        }
    }

    fn memory_rows_mut(&mut self) -> Option<&mut Vec<InMemoryGridRow>> {
        match self {
            Self::Memory(source) => Some(&mut source.rows),
            Self::PagedVirtual(_) => None,
        }
    }
}

impl GridRowSource for DemoGridSource {
    fn row_count(&self) -> usize {
        match self {
            Self::Memory(source) => source.row_count(),
            Self::PagedVirtual(source) => source.row_count(),
        }
    }

    fn revision(&self) -> u64 {
        match self {
            Self::Memory(source) => source.revision(),
            Self::PagedVirtual(source) => source.revision(),
        }
    }

    fn row_id(&self, row_index: usize) -> GridRowId {
        match self {
            Self::Memory(source) => source.row_id(row_index),
            Self::PagedVirtual(source) => source.row_id(row_index),
        }
    }

    fn cell_value(&self, row_index: usize, column_id: &GridColumnId) -> GridCellValue {
        match self {
            Self::Memory(source) => source.cell_value(row_index, column_id),
            Self::PagedVirtual(source) => source.cell_value(row_index, column_id),
        }
    }

    fn cell_provenance(
        &self,
        row_index: usize,
        column_id: &GridColumnId,
    ) -> Option<GridCellProvenance> {
        match self {
            Self::Memory(source) => source.cell_provenance(row_index, column_id),
            Self::PagedVirtual(source) => source.cell_provenance(row_index, column_id),
        }
    }
}

fn paged_virtual_cell_value(row_index: usize, column_id: &GridColumnId) -> GridCellValue {
    match column_id.0.as_str() {
        "symbol" => GridCellValue::Text(format!("PV{row_index:06}")),
        "name" => GridCellValue::Text(format!("Paged Virtual Instrument {row_index:06}")),
        "price" => GridCellValue::Decimal(10_000.0 + (row_index % 7_000) as f64),
        "change_pct" => GridCellValue::Decimal((row_index as f64 % 21.0) / 10.0 - 1.0),
        "volume" => GridCellValue::Integer(1_000_000 + row_index as i64 * 10),
        "trend" => GridCellValue::Sparkline(vec![
            row_index as f64 % 5.0,
            row_index as f64 % 7.0,
            row_index as f64 % 11.0,
        ]),
        "delta" => GridCellValue::DeltaBar((row_index as f64 % 200.0) - 100.0),
        "source" => GridCellValue::Source("paged-virtual".to_owned()),
        "status" => GridCellValue::Status(if row_index % 3 == 0 {
            "live".to_owned()
        } else {
            "snapshot".to_owned()
        }),
        _ => GridCellValue::Empty,
    }
}

#[derive(Debug)]
pub struct DemoFinancialGrid {
    pub columns: Vec<GridColumnDef>,
    source: DemoGridSource,
    pub state: GridState,
    pub bridge: GridAgentBridge,
    pub last_capture_manifest: Option<String>,
    pub last_export: Option<String>,
    pub last_persisted_state: Option<GridPersistedState>,
    pub last_analytics_row_count: usize,
    live_started: Instant,
}

impl Default for DemoFinancialGrid {
    fn default() -> Self {
        let columns = demo_columns();
        let mut state = GridState::default();
        state.pinned_left = vec![GridColumnId::new("symbol")];
        state.normalize_columns(&columns);
        Self {
            columns,
            source: DemoGridSource::memory(demo_rows(1_000)),
            state,
            bridge: GridAgentBridge::default(),
            last_capture_manifest: None,
            last_export: None,
            last_persisted_state: None,
            last_analytics_row_count: 0,
            live_started: Instant::now(),
        }
    }
}

impl DemoFinancialGrid {
    pub fn apply_agent_demo_action(&mut self, action: FinancialGridDemoAction) {
        match action {
            FinancialGridDemoAction::Open => {
                self.record_manifest("open");
            }
            FinancialGridDemoAction::SortChangeDesc => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::SetSort {
                        column_id: GridColumnId::new("change_pct"),
                        direction: GridSortDirection::Desc,
                        reason_ref: "agent:grid-demo:sort-change".to_owned(),
                    },
                );
                self.record_manifest("sort-change-desc");
            }
            FinancialGridDemoAction::MultiSortDemo => {
                self.state.sort = vec![
                    GridSort {
                        column_id: GridColumnId::new("change_pct"),
                        direction: GridSortDirection::Desc,
                    },
                    GridSort {
                        column_id: GridColumnId::new("volume"),
                        direction: GridSortDirection::Desc,
                    },
                ];
                self.record_manifest("multi-sort");
            }
            FinancialGridDemoAction::SelectAcme => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::SelectRow {
                        row_id: GridRowId::new("ACME"),
                        reason_ref: "agent:grid-demo:select".to_owned(),
                    },
                );
                self.record_manifest("select-samsung");
            }
            FinancialGridDemoAction::SelectCell => {
                let cell = GridCellRef {
                    row_id: GridRowId::new("ACME"),
                    column_id: GridColumnId::new("price"),
                };
                self.state.selection.selected_row = Some(cell.row_id.clone());
                self.state.selection.selected_cell = Some(cell.clone());
                self.state.focused_cell = Some(cell);
                self.record_manifest("select-cell");
            }
            FinancialGridDemoAction::RangeSelection => {
                self.state.select_range(
                    GridCellRef {
                        row_id: GridRowId::new("ACME"),
                        column_id: GridColumnId::new("price"),
                    },
                    GridCellRef {
                        row_id: GridRowId::new("ORBT"),
                        column_id: GridColumnId::new("delta"),
                    },
                );
                self.record_manifest("range-selection");
            }
            FinancialGridDemoAction::HighlightVolatile => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::HighlightRows {
                        row_ids: vec![GridRowId::new("ACME"), GridRowId::new("NOVA")],
                        reason_ref: "agent:grid-demo:volatility".to_owned(),
                        evidence_ref: Some("evidence:grid-demo:volatile".to_owned()),
                    },
                );
                self.record_manifest("highlight-volatile");
            }
            FinancialGridDemoAction::AgentHighlightCells => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::HighlightCells {
                        cell_refs: vec![GridCellRef {
                            row_id: GridRowId::new("NOVA"),
                            column_id: GridColumnId::new("change_pct"),
                        }],
                        reason_ref: "agent:grid-demo:cell-highlight".to_owned(),
                        evidence_ref: Some("evidence:grid-demo:cell-highlight".to_owned()),
                    },
                );
                self.record_manifest("highlight-cells");
            }
            FinancialGridDemoAction::OpenEvidence => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::OpenRowDetail {
                        row_id: GridRowId::new("ACME"),
                        reason_ref: "agent:grid-demo:evidence".to_owned(),
                    },
                );
                self.record_manifest("open-evidence");
            }
            FinancialGridDemoAction::PinPrice => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::PinColumn {
                        column_id: GridColumnId::new("price"),
                        side: GridPinSide::Left,
                        reason_ref: "agent:grid-demo:pin-price".to_owned(),
                    },
                );
                self.record_manifest("pin-price");
            }
            FinancialGridDemoAction::PinSymbol => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::PinColumn {
                        column_id: GridColumnId::new("symbol"),
                        side: GridPinSide::Left,
                        reason_ref: "agent:grid-demo:pin-symbol".to_owned(),
                    },
                );
                self.record_manifest("pin-symbol");
            }
            FinancialGridDemoAction::ResizePrice => {
                self.state
                    .column_widths
                    .insert(GridColumnId::new("price"), 132.0);
                self.record_manifest("resize-price");
            }
            FinancialGridDemoAction::ColumnReorder => {
                let _ = self.state.reorder_column(&GridColumnId::new("trend"), 1);
                self.record_manifest("column-reorder");
            }
            FinancialGridDemoAction::FilterNameOrbit => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::SetFilter {
                        column_id: GridColumnId::new("name"),
                        query: "Orbit Search".to_owned(),
                        reason_ref: "agent:grid-demo:filter-name".to_owned(),
                    },
                );
                self.record_manifest("filter-name");
            }
            FinancialGridDemoAction::FilterPriceGreaterThan => {
                self.set_filter("price", ">100000");
            }
            FinancialGridDemoAction::FilterPercentRange => {
                self.set_filter("change_pct", "-1..2");
            }
            FinancialGridDemoAction::FacetedFilter => {
                let row_model = build_row_model(&self.source, &self.state, &self.columns);
                self.last_analytics_row_count =
                    facet_counts(&self.source, &row_model, &GridColumnId::new("status")).len();
                self.state
                    .filters
                    .retain(|filter| filter.column_id != GridColumnId::new("status"));
                self.state.filters.push(GridFilter {
                    column_id: GridColumnId::new("status"),
                    query: "live".to_owned(),
                });
                self.record_manifest("faceted-filter");
            }
            FinancialGridDemoAction::QuickFilter => {
                self.set_filter("name", "Acme");
            }
            FinancialGridDemoAction::InvalidFilter => {
                self.set_filter("price", "not-a-number");
            }
            FinancialGridDemoAction::ClearFilters => {
                self.state.filters.clear();
                self.record_manifest("clear-filters");
            }
            FinancialGridDemoAction::HideVolume => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::HideColumn {
                        column_id: GridColumnId::new("volume"),
                        reason_ref: "agent:grid-demo:hide-volume".to_owned(),
                    },
                );
                self.record_manifest("hide-volume");
            }
            FinancialGridDemoAction::ShowVolume => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::ShowColumn {
                        column_id: GridColumnId::new("volume"),
                        reason_ref: "agent:grid-demo:show-volume".to_owned(),
                    },
                );
                self.record_manifest("show-volume");
            }
            FinancialGridDemoAction::PinRightStatus => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::PinColumn {
                        column_id: GridColumnId::new("status"),
                        side: GridPinSide::Right,
                        reason_ref: "agent:grid-demo:pin-status-right".to_owned(),
                    },
                );
                self.record_manifest("pin-right-status");
            }
            FinancialGridDemoAction::ScrollVertical => {
                self.state.row_scroll = 48.min(self.source.row_count().saturating_sub(1));
                self.record_manifest("scroll-vertical");
            }
            FinancialGridDemoAction::ScrollHorizontal => {
                self.state.horizontal_scroll = 220.0;
                self.record_manifest("scroll-horizontal");
            }
            FinancialGridDemoAction::HomeEnd => {
                self.state.row_scroll = self.source.row_count().saturating_sub(20);
                self.record_manifest("home-end");
            }
            FinancialGridDemoAction::SmallViewport => {
                self.state.row_scroll = 4;
                self.state.horizontal_scroll = 90.0;
                self.record_manifest("small-viewport");
            }
            FinancialGridDemoAction::FormatFixture => {
                self.apply_format_fixture();
                self.record_manifest("format-fixture");
            }
            FinancialGridDemoAction::SparklineDeltaRenderer => {
                if let Some(row) = self
                    .source
                    .memory_rows_mut()
                    .and_then(|rows| rows.get_mut(0))
                {
                    row.cells.insert(
                        GridColumnId::new("trend"),
                        GridCellValue::Sparkline(vec![1.0, 1.4, 1.1, 1.8, 2.1, 1.9, 2.6]),
                    );
                    row.cells
                        .insert(GridColumnId::new("delta"), GridCellValue::DeltaBar(-42.0));
                }
                self.record_manifest("sparkline-delta-renderer");
            }
            FinancialGridDemoAction::LongText => {
                if let Some(row) = self
                    .source
                    .memory_rows_mut()
                    .and_then(|rows| rows.get_mut(0))
                {
                    row.cells.insert(
                        GridColumnId::new("name"),
                        GridCellValue::Text(
                            "Acme Holdings ultra long analyst watchlist label".to_owned(),
                        ),
                    );
                }
                self.record_manifest("long-text");
            }
            FinancialGridDemoAction::StaleAfterTimeout => {
                if let Some(row) = self
                    .source
                    .memory_rows_mut()
                    .and_then(|rows| rows.get_mut(0))
                {
                    row.cells.insert(
                        GridColumnId::new("status"),
                        GridCellValue::Status("stale".to_owned()),
                    );
                }
                self.record_manifest("stale-after-timeout");
            }
            FinancialGridDemoAction::AgentDerivedCell => {
                if let Some(row) = self
                    .source
                    .memory_rows_mut()
                    .and_then(|rows| rows.get_mut(0))
                {
                    row.cells.insert(
                        GridColumnId::new("source"),
                        GridCellValue::AgentAnnotation("agent-derived".to_owned()),
                    );
                }
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::HighlightCells {
                        cell_refs: vec![GridCellRef {
                            row_id: GridRowId::new("ACME"),
                            column_id: GridColumnId::new("source"),
                        }],
                        reason_ref: "agent:grid-demo:derived-cell".to_owned(),
                        evidence_ref: Some("evidence:grid-demo:derived-cell".to_owned()),
                    },
                );
                self.record_manifest("agent-derived-cell");
            }
            FinancialGridDemoAction::LiveTick => {
                let price_cell = super::ids::GridCellRef {
                    row_id: GridRowId::new("ACME"),
                    column_id: GridColumnId::new("price"),
                };
                if let Some(row) = self
                    .source
                    .memory_rows_mut()
                    .and_then(|rows| rows.get_mut(0))
                {
                    row.cells
                        .insert(GridColumnId::new("price"), GridCellValue::Decimal(74_300.0));
                    row.cells.insert(
                        GridColumnId::new("status"),
                        GridCellValue::Status("live".to_owned()),
                    );
                }
                self.state.mark_flash(price_cell);
                self.record_manifest("live-tick");
            }
            FinancialGridDemoAction::LiveDeleteRow => {
                if let Some(rows) = self.source.memory_rows_mut() {
                    rows.retain(|row| row.id.0 != "NOVA");
                }
                self.state.selection.selected_row = Some(GridRowId::new("ACME"));
                self.record_manifest("live-delete-row");
            }
            FinancialGridDemoAction::LivePause => {
                if let Some(row) = self
                    .source
                    .memory_rows_mut()
                    .and_then(|rows| rows.get_mut(0))
                {
                    row.cells.insert(
                        GridColumnId::new("status"),
                        GridCellValue::Status("paused".to_owned()),
                    );
                }
                self.record_manifest("live-pause");
            }
            FinancialGridDemoAction::LargeFixture => {
                self.source = DemoGridSource::memory(demo_rows(50_000));
                self.state.row_scroll = 0;
                self.record_manifest("large-fixture");
            }
            FinancialGridDemoAction::PagedVirtualSource => {
                self.source = DemoGridSource::paged_virtual(250_000);
                self.state.row_scroll = 2_400;
                self.state.horizontal_scroll = 180.0;
                self.record_manifest("paged-virtual-source");
            }
            FinancialGridDemoAction::SourceStreaming => {
                self.apply_agent_demo_action(FinancialGridDemoAction::LiveTick);
                self.record_manifest("source-streaming");
            }
            FinancialGridDemoAction::SourceEndpointPayload => {
                if let Some(row) = self
                    .source
                    .memory_rows_mut()
                    .and_then(|rows| rows.get_mut(0))
                {
                    row.cells.insert(
                        GridColumnId::new("source"),
                        GridCellValue::Source("endpoint:/api/market/watch".to_owned()),
                    );
                }
                self.record_manifest("source-endpoint-payload");
            }
            FinancialGridDemoAction::SourceDuckDbPayload => {
                if let Some(row) = self
                    .source
                    .memory_rows_mut()
                    .and_then(|rows| rows.get_mut(0))
                {
                    row.cells.insert(
                        GridColumnId::new("source"),
                        GridCellValue::Source("duckdb:market_watch_materialized".to_owned()),
                    );
                }
                self.record_manifest("source-duckdb-payload");
            }
            FinancialGridDemoAction::AgentAttachEvidence => {
                self.bridge.apply_command(
                    &mut self.state,
                    GridAgentCommand::AttachEvidence {
                        target: GridEvidenceTarget::Cell(GridCellRef {
                            row_id: GridRowId::new("ACME"),
                            column_id: GridColumnId::new("price"),
                        }),
                        evidence_ref: "evidence:grid-demo:attached".to_owned(),
                        reason_ref: "agent:grid-demo:attach-evidence".to_owned(),
                    },
                );
                self.record_manifest("attach-evidence");
            }
            FinancialGridDemoAction::CopyCell => {
                self.apply_agent_demo_action(FinancialGridDemoAction::SelectCell);
                self.last_export = self
                    .columns
                    .iter()
                    .find(|column| column.id == GridColumnId::new("price"))
                    .map(|column| {
                        self.source
                            .cell_value(0, &column.id)
                            .display(&column.formatter)
                    });
                self.record_manifest("copy-cell");
            }
            FinancialGridDemoAction::CopyRow => {
                self.apply_agent_demo_action(FinancialGridDemoAction::SelectAcme);
                let row_model = build_row_model(&self.source, &self.state, &self.columns);
                self.last_export = export_selected_row(
                    &self.source,
                    &self.columns,
                    &self.state,
                    &row_model,
                    &GridExportOptions::default(),
                );
                self.record_manifest("copy-row");
            }
            FinancialGridDemoAction::ExportJson => {
                self.apply_agent_demo_action(FinancialGridDemoAction::SelectAcme);
                let row_model = build_row_model(&self.source, &self.state, &self.columns);
                self.last_export = export_selected_row(
                    &self.source,
                    &self.columns,
                    &self.state,
                    &row_model,
                    &GridExportOptions {
                        format: GridExportFormat::Json,
                        value_mode: GridExportValueMode::Display,
                        include_hidden_columns: false,
                        include_header: false,
                    },
                );
                self.record_manifest("export-json");
            }
            FinancialGridDemoAction::OpenMenuFixture => {
                self.state.focused_cell = Some(GridCellRef {
                    row_id: GridRowId::new("ACME"),
                    column_id: GridColumnId::new("price"),
                });
                self.record_manifest("menu-fixture");
            }
            FinancialGridDemoAction::PersistCustomLayout => {
                self.state
                    .column_widths
                    .insert(GridColumnId::new("price"), 132.0);
                self.state
                    .column_visibility
                    .insert(GridColumnId::new("volume"), false);
                self.state.pinned_right = vec![GridColumnId::new("status")];
                self.last_persisted_state = Some(GridPersistedState::from_state(
                    &self.state,
                    GridDensity::Compact,
                ));
                self.record_manifest("persist-custom-layout");
            }
            FinancialGridDemoAction::RestoreCustomLayout => {
                if self.last_persisted_state.is_none() {
                    self.apply_agent_demo_action(FinancialGridDemoAction::PersistCustomLayout);
                }
                if let Some(persisted) = &self.last_persisted_state {
                    persisted.apply_to_state(&mut self.state);
                    self.state.normalize_columns(&self.columns);
                }
                self.record_manifest("restore-custom-layout");
            }
            FinancialGridDemoAction::PanelContextPropagation => {
                self.apply_agent_demo_action(FinancialGridDemoAction::SelectAcme);
                self.apply_agent_demo_action(FinancialGridDemoAction::OpenEvidence);
                self.record_manifest("panel-context-propagation");
            }
            FinancialGridDemoAction::Grouping => {
                let row_model = build_row_model(&self.source, &self.state, &self.columns);
                let rows = group_and_aggregate(
                    &self.source,
                    &row_model,
                    &GridGroupSpec {
                        column_id: GridColumnId::new("status"),
                    },
                    &[
                        GridAggregationSpec {
                            column_id: GridColumnId::new("price"),
                            aggregation: GridAggregation::Average,
                        },
                        GridAggregationSpec {
                            column_id: GridColumnId::new("volume"),
                            aggregation: GridAggregation::Sum,
                        },
                    ],
                );
                self.last_analytics_row_count = rows.len();
                self.record_manifest("grouping-aggregation");
            }
            FinancialGridDemoAction::Aggregation => {
                let row_model = build_row_model(&self.source, &self.state, &self.columns);
                let rows = group_and_aggregate(
                    &self.source,
                    &row_model,
                    &GridGroupSpec {
                        column_id: GridColumnId::new("source"),
                    },
                    &[
                        GridAggregationSpec {
                            column_id: GridColumnId::new("price"),
                            aggregation: GridAggregation::Min,
                        },
                        GridAggregationSpec {
                            column_id: GridColumnId::new("price"),
                            aggregation: GridAggregation::Max,
                        },
                        GridAggregationSpec {
                            column_id: GridColumnId::new("volume"),
                            aggregation: GridAggregation::Count,
                        },
                    ],
                );
                self.last_analytics_row_count = rows.len();
                self.record_manifest("aggregation");
            }
            FinancialGridDemoAction::PivotAnalytics => {
                let row_model = build_row_model(&self.source, &self.state, &self.columns);
                let rows = pivot_aggregate(
                    &self.source,
                    &row_model,
                    &super::analytics::GridPivotSpec {
                        row_column_id: GridColumnId::new("status"),
                        column_column_id: GridColumnId::new("source"),
                        value: GridAggregationSpec {
                            column_id: GridColumnId::new("volume"),
                            aggregation: GridAggregation::Sum,
                        },
                    },
                );
                self.last_analytics_row_count = rows.len();
                self.record_manifest("pivot-analytics-native-view");
            }
            FinancialGridDemoAction::InlineEdit => {
                self.state.begin_edit(
                    GridCellRef {
                        row_id: GridRowId::new("ACME"),
                        column_id: GridColumnId::new("price"),
                    },
                    GridCellValue::Decimal(74_500.0),
                );
                self.record_manifest("inline-edit");
            }
            FinancialGridDemoAction::InlineEditCommit => {
                self.apply_agent_demo_action(FinancialGridDemoAction::InlineEdit);
                let _ = self.state.commit_edit();
                self.record_manifest("inline-edit-commit");
            }
            FinancialGridDemoAction::InlineEditCancel => {
                self.apply_agent_demo_action(FinancialGridDemoAction::InlineEdit);
                self.state.cancel_edit();
                self.record_manifest("inline-edit-cancel");
            }
            FinancialGridDemoAction::InlineEditInvalid => {
                self.state.begin_invalid_edit(
                    GridCellRef {
                        row_id: GridRowId::new("ACME"),
                        column_id: GridColumnId::new("price"),
                    },
                    GridCellValue::Error("invalid price".to_owned()),
                );
                self.record_manifest("inline-edit-invalid");
            }
            FinancialGridDemoAction::InlineEditReadOnly => {
                self.state.begin_read_only_edit(
                    GridCellRef {
                        row_id: GridRowId::new("ACME"),
                        column_id: GridColumnId::new("source"),
                    },
                    GridCellValue::Source("api".to_owned()),
                );
                self.record_manifest("inline-edit-read-only");
            }
            FinancialGridDemoAction::Undo => {
                let _ = self.bridge.undo_last_reversible(&mut self.state);
                self.record_manifest("undo");
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> GridOutput {
        self.state.decay_flash_cells();
        if self.live_started.elapsed() > Duration::from_millis(800) {
            if let Some(row) = self
                .source
                .memory_rows_mut()
                .and_then(|rows| rows.get_mut(0))
            {
                row.cells.insert(
                    GridColumnId::new("status"),
                    GridCellValue::Status("live".to_owned()),
                );
            }
        }
        FinancialDataGrid::new("financial-data-grid-demo")
            .columns(&self.columns)
            .source(&self.source)
            .state(&mut self.state)
            .density(GridDensity::Compact)
            .provenance_policy(GridProvenancePolicy::VisibleOnHover)
            .agent_bridge(&self.bridge)
            .show(ui)
    }

    pub fn snapshot(&self) -> GridSnapshot {
        self.bridge
            .snapshot("financial-data-grid-demo", &self.state, &self.source, 0..20)
    }

    pub fn row_scroll(&self) -> usize {
        self.state.row_scroll
    }

    pub fn feature_summary(&self) -> Vec<(&'static str, String, bool)> {
        let mut summary = Vec::new();
        summary.push(("Source", self.source.kind().to_owned(), true));
        summary.push((
            "Filters",
            if self.state.filters.is_empty() {
                "clear".to_owned()
            } else {
                self.state
                    .filters
                    .iter()
                    .map(|filter| format!("{} {}", filter.column_id.0, filter.query))
                    .collect::<Vec<_>>()
                    .join(", ")
            },
            !self.state.filters.is_empty(),
        ));
        summary.push((
            "Layout",
            format!(
                "left={} right={} hidden={} widths={}",
                self.state.pinned_left.len(),
                self.state.pinned_right.len(),
                self.state
                    .column_visibility
                    .values()
                    .filter(|visible| !**visible)
                    .count(),
                self.state.column_widths.len()
            ),
            self.state.pinned_left.len() + self.state.pinned_right.len() > 0,
        ));
        summary.push((
            "Analytics",
            if self.last_analytics_row_count == 0 {
                "not derived".to_owned()
            } else {
                format!("{} derived rows", self.last_analytics_row_count)
            },
            self.last_analytics_row_count > 0,
        ));
        summary.push((
            "Edit",
            self.state
                .edit_draft
                .as_ref()
                .map(|draft| {
                    if draft.read_only {
                        "read-only".to_owned()
                    } else if draft.valid {
                        "valid".to_owned()
                    } else {
                        "invalid".to_owned()
                    }
                })
                .unwrap_or_else(|| "none".to_owned()),
            self.state.edit_draft.is_some(),
        ));
        summary.push((
            "Export",
            self.last_export
                .as_ref()
                .map(|value| format!("{} bytes", value.len()))
                .unwrap_or_else(|| "none".to_owned()),
            self.last_export.is_some(),
        ));
        summary
    }

    fn set_filter(&mut self, column_id: &str, query: &str) {
        self.state
            .filters
            .retain(|filter| filter.column_id.0 != column_id);
        self.state.filters.push(GridFilter {
            column_id: GridColumnId::new(column_id),
            query: query.to_owned(),
        });
        self.record_manifest("set-filter");
    }

    fn apply_format_fixture(&mut self) {
        if let Some(row) = self
            .source
            .memory_rows_mut()
            .and_then(|rows| rows.get_mut(0))
        {
            row.cells.insert(
                GridColumnId::new("change_pct"),
                GridCellValue::Decimal(-3.25),
            );
            row.cells.insert(
                GridColumnId::new("status"),
                GridCellValue::Status("delayed".to_owned()),
            );
            row.cells.insert(
                GridColumnId::new("source"),
                GridCellValue::Timestamp("2026-06-04T09:30:00+09:00".to_owned()),
            );
        }
    }

    fn record_manifest(&mut self, action: &'static str) {
        let row_model = build_row_model(&self.source, &self.state, &self.columns);
        let visible_columns = self.state.visible_column_ids();
        let visible_row_count = row_model.len().min(20);
        let requested_cell_count = visible_columns.len().saturating_mul(visible_row_count);
        let selected_range = self
            .state
            .selection
            .selected_range
            .as_ref()
            .map(|range| {
                format!(
                    "{}:{}..{}:{}",
                    range.anchor.row_id.0,
                    range.anchor.column_id.0,
                    range.focus.row_id.0,
                    range.focus.column_id.0
                )
            })
            .unwrap_or_else(|| "none".to_owned());
        let edit_state = self
            .state
            .edit_draft
            .as_ref()
            .map(|draft| {
                format!(
                    "{}:{}:valid={}:read_only={}",
                    draft.cell.row_id.0, draft.cell.column_id.0, draft.valid, draft.read_only
                )
            })
            .unwrap_or_else(|| "none".to_owned());
        self.last_capture_manifest = Some(format!(
            "action={action};source_kind={};row_count={};filtered_row_count={};visible_columns={};requested_cell_count={requested_cell_count};source_revision={};filters={};sort={};selected_row={};selected_range={selected_range};edit_state={edit_state};analytics_rows={};export_bytes={}",
            self.source.kind(),
            self.source.row_count(),
            row_model.len(),
            visible_columns.len(),
            self.source.revision(),
            self.state.filters.len(),
            self.state.sort.len(),
            self.state
                .selection
                .selected_row
                .as_ref()
                .map(|row| row.0.as_str())
                .unwrap_or("none"),
            self.last_analytics_row_count,
            self.last_export
                .as_ref()
                .map(|value| value.len())
                .unwrap_or(0)
        ));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinancialGridDemoAction {
    Open,
    SortChangeDesc,
    MultiSortDemo,
    SelectAcme,
    SelectCell,
    RangeSelection,
    HighlightVolatile,
    AgentHighlightCells,
    OpenEvidence,
    PinPrice,
    PinSymbol,
    ResizePrice,
    ColumnReorder,
    FilterNameOrbit,
    FilterPriceGreaterThan,
    FilterPercentRange,
    FacetedFilter,
    QuickFilter,
    InvalidFilter,
    ClearFilters,
    HideVolume,
    ShowVolume,
    PinRightStatus,
    ScrollVertical,
    ScrollHorizontal,
    HomeEnd,
    SmallViewport,
    FormatFixture,
    SparklineDeltaRenderer,
    LongText,
    StaleAfterTimeout,
    AgentDerivedCell,
    LiveTick,
    LiveDeleteRow,
    LivePause,
    LargeFixture,
    SourceStreaming,
    SourceEndpointPayload,
    SourceDuckDbPayload,
    AgentAttachEvidence,
    CopyCell,
    CopyRow,
    ExportJson,
    OpenMenuFixture,
    PersistCustomLayout,
    RestoreCustomLayout,
    PanelContextPropagation,
    Grouping,
    Aggregation,
    PivotAnalytics,
    PagedVirtualSource,
    InlineEdit,
    InlineEditCommit,
    InlineEditCancel,
    InlineEditInvalid,
    InlineEditReadOnly,
    Undo,
}
