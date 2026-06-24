use super::super::{
    analytics::{
        GridAggregation, GridAggregationSpec, GridPivotSpec, facet_counts, pivot_aggregate,
    },
    core::{build_row_model, build_row_model_cache_key},
    demo_data::{demo_columns, demo_rows},
    ids::{GridCellRef, GridColumnId, GridRowId},
    interaction::{GridKeyboardCommand, apply_grid_keyboard_command},
    row::page_row_model,
    source::{
        GridRowSource, InMemoryGridSource, StreamingGridRow, StreamingGridSource, VirtualGridSource,
    },
    state::{GridFilter, GridSort, GridSortDirection, GridState},
};

#[test]
fn split_core_tests_module_builds_row_model() {
    let columns = demo_columns();
    let mut state = GridState::default();
    state.normalize_columns(&columns);
    let source = InMemoryGridSource::new(demo_rows(4));

    let row_model = build_row_model(&source, &state, &columns);

    assert_eq!(row_model, vec![0, 1, 2, 3]);
}

#[test]
fn row_model_cache_key_tracks_source_revision_and_state() {
    let columns = demo_columns();
    let mut state = GridState::default();
    state.normalize_columns(&columns);
    state.sort.push(GridSort {
        column_id: GridColumnId::new("price"),
        direction: GridSortDirection::Desc,
    });
    state.filters.push(GridFilter {
        column_id: GridColumnId::new("status"),
        query: "live".to_owned(),
    });
    let mut source = StreamingGridSource::default();

    let initial = build_row_model_cache_key(&source, &state);
    source.upsert_row(StreamingGridRow::new(GridRowId::new("ACME")));
    let after_update = build_row_model_cache_key(&source, &state);
    state
        .column_visibility
        .insert(GridColumnId::new("volume"), false);
    let after_layout = build_row_model_cache_key(&source, &state);

    assert_ne!(initial.source_revision, after_update.source_revision);
    assert_ne!(initial.row_count, after_update.row_count);
    assert_ne!(after_update.visible_columns, after_layout.visible_columns);
    assert_eq!(after_layout.sort.len(), 1);
    assert_eq!(after_layout.filters.len(), 1);
}

#[test]
fn advanced_analytics_facets_pivots_and_pages_visible_models() {
    let columns = demo_columns();
    let mut state = GridState::default();
    state.normalize_columns(&columns);
    let source = InMemoryGridSource::new(demo_rows(12));
    let row_model = build_row_model(&source, &state, &columns);

    let facets = facet_counts(&source, &row_model, &GridColumnId::new("status"));
    let pivot = pivot_aggregate(
        &source,
        &row_model,
        &GridPivotSpec {
            row_column_id: GridColumnId::new("status"),
            column_column_id: GridColumnId::new("source"),
            value: GridAggregationSpec {
                column_id: GridColumnId::new("volume"),
                aggregation: GridAggregation::Sum,
            },
        },
    );
    let page = page_row_model(&row_model, 4, 3);

    assert!(facets.iter().any(|facet| facet.value == "stale"));
    assert!(pivot.iter().any(|row| row.cells.contains_key("api")));
    assert_eq!(page.len(), 3);
}

#[test]
fn paged_virtual_source_supports_lazy_advanced_windows() {
    let columns = demo_columns();
    let mut state = GridState::default();
    state.normalize_columns(&columns);
    let source = VirtualGridSource::new(
        250_000,
        |row_index| GridRowId::new(format!("PV{row_index:06}")),
        |row_index, column_id| match column_id.0.as_str() {
            "symbol" => super::super::cell::GridCellValue::Text(format!("PV{row_index:06}")),
            "price" => super::super::cell::GridCellValue::Decimal(100.0 + row_index as f64),
            "status" => super::super::cell::GridCellValue::Status("live".to_owned()),
            _ => super::super::cell::GridCellValue::Empty,
        },
    );

    let row_model = build_row_model(&source, &state, &columns);
    let page = page_row_model(&row_model, 2_400, 25);

    assert_eq!(row_model.len(), 250_000);
    assert_eq!(page.len(), 25);
    assert_eq!(source.row_id(page[0]), GridRowId::new("PV002400"));
    assert_eq!(source.row_id(page[24]), GridRowId::new("PV002424"));
}

#[test]
fn keyboard_navigation_supports_page_home_end_and_cell_columns() {
    let columns = demo_columns();
    let mut state = GridState::default();
    state.normalize_columns(&columns);
    let visible_columns = state.visible_column_ids();
    let source = InMemoryGridSource::new(demo_rows(20));
    let row_model = build_row_model(&source, &state, &columns);
    let mut actions = Vec::new();

    apply_grid_keyboard_command(
        &source,
        &mut state,
        &row_model,
        &visible_columns,
        5,
        GridKeyboardCommand::PageDown,
        &mut actions,
    );
    assert_eq!(state.selection.selected_row, Some(GridRowId::new("900005")));

    apply_grid_keyboard_command(
        &source,
        &mut state,
        &row_model,
        &visible_columns,
        5,
        GridKeyboardCommand::End,
        &mut actions,
    );
    assert_eq!(state.selection.selected_row, Some(GridRowId::new("900019")));

    apply_grid_keyboard_command(
        &source,
        &mut state,
        &row_model,
        &visible_columns,
        5,
        GridKeyboardCommand::Home,
        &mut actions,
    );
    state.focused_cell = Some(GridCellRef {
        row_id: GridRowId::new("ACME"),
        column_id: GridColumnId::new("symbol"),
    });
    apply_grid_keyboard_command(
        &source,
        &mut state,
        &row_model,
        &visible_columns,
        5,
        GridKeyboardCommand::Right,
        &mut actions,
    );

    assert_eq!(state.selection.selected_row, Some(GridRowId::new("ACME")));
    assert_eq!(
        state
            .focused_cell
            .as_ref()
            .map(|cell| cell.column_id.0.as_str()),
        Some("name")
    );
}
