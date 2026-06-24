use std::collections::BTreeMap;

use super::GridRowSource;
use crate::{GridCellProvenance, GridCellValue, GridColumnId, GridRowId};

#[derive(Debug, Clone)]
pub struct InMemoryGridSource {
    pub rows: Vec<InMemoryGridRow>,
}

#[derive(Debug, Clone)]
pub struct InMemoryGridRow {
    pub id: GridRowId,
    pub cells: BTreeMap<GridColumnId, GridCellValue>,
    pub provenance: BTreeMap<GridColumnId, GridCellProvenance>,
}

impl InMemoryGridSource {
    pub fn new(rows: Vec<InMemoryGridRow>) -> Self {
        Self { rows }
    }
}

impl GridRowSource for InMemoryGridSource {
    fn row_count(&self) -> usize {
        self.rows.len()
    }

    fn row_id(&self, row_index: usize) -> GridRowId {
        self.rows
            .get(row_index)
            .map(|row| row.id.clone())
            .unwrap_or_else(|| GridRowId::new(format!("missing-{row_index}")))
    }

    fn cell_value(&self, row_index: usize, column_id: &GridColumnId) -> GridCellValue {
        self.rows
            .get(row_index)
            .and_then(|row| row.cells.get(column_id).cloned())
            .unwrap_or(GridCellValue::Empty)
    }

    fn cell_provenance(
        &self,
        row_index: usize,
        column_id: &GridColumnId,
    ) -> Option<GridCellProvenance> {
        self.rows
            .get(row_index)
            .and_then(|row| row.provenance.get(column_id).cloned())
    }
}
