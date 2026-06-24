use std::cmp::Ordering;

use super::column::GridColumnDef;
use super::ids::GridColumnId;
use super::source::GridRowSource;
use super::state::{GridFilter, GridPinSide, GridSort, GridSortDirection, GridState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridRowModelCacheKey {
    pub source_revision: u64,
    pub row_count: usize,
    pub sort: Vec<GridSort>,
    pub filters: Vec<GridFilter>,
    pub column_order: Vec<GridColumnId>,
    pub visible_columns: Vec<GridColumnId>,
    pub pinned_columns: Vec<(GridColumnId, GridPinSide)>,
}

pub fn build_row_model_cache_key(
    source: &dyn GridRowSource,
    state: &GridState,
) -> GridRowModelCacheKey {
    let mut pinned_columns = state
        .pinned_left
        .iter()
        .cloned()
        .map(|column_id| (column_id, GridPinSide::Left))
        .collect::<Vec<_>>();
    pinned_columns.extend(
        state
            .pinned_right
            .iter()
            .cloned()
            .map(|column_id| (column_id, GridPinSide::Right)),
    );
    GridRowModelCacheKey {
        source_revision: source.revision(),
        row_count: source.row_count(),
        sort: state.sort.clone(),
        filters: state.filters.clone(),
        column_order: state.column_order.clone(),
        visible_columns: state.visible_column_ids(),
        pinned_columns,
    }
}

pub fn build_row_model(
    source: &dyn GridRowSource,
    state: &GridState,
    columns: &[GridColumnDef],
) -> Vec<usize> {
    let mut rows: Vec<usize> = (0..source.row_count()).collect();
    rows.retain(|row| {
        state.filters.iter().all(|filter| {
            source
                .cell_value(*row, &filter.column_id)
                .contains(filter.query.as_str())
        })
    });
    let sortable_sorts: Vec<_> = state
        .sort
        .iter()
        .filter(|sort| {
            columns
                .iter()
                .any(|column| column.id == sort.column_id && column.sortable)
        })
        .collect();
    if !sortable_sorts.is_empty() {
        rows.sort_by(|left, right| {
            for sort in &sortable_sorts {
                let ordering = source
                    .cell_value(*left, &sort.column_id)
                    .sort_key()
                    .partial_cmp(&source.cell_value(*right, &sort.column_id).sort_key())
                    .unwrap_or(Ordering::Equal);
                let ordering = match sort.direction {
                    GridSortDirection::Asc => ordering,
                    GridSortDirection::Desc => ordering.reverse(),
                };
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            left.cmp(right)
        });
    }
    rows
}
