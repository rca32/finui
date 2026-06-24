#![allow(dead_code)]

use std::collections::BTreeMap;

use super::GridRowSource;
use crate::{GridCellProvenance, GridCellValue, GridColumnId, GridRowId};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct StreamingGridRow {
    pub id: GridRowId,
    pub cells: BTreeMap<GridColumnId, GridCellValue>,
    pub provenance: BTreeMap<GridColumnId, GridCellProvenance>,
    pub update_sequence: u64,
}

impl StreamingGridRow {
    #[allow(dead_code)]
    pub fn new(id: GridRowId) -> Self {
        Self {
            id,
            cells: BTreeMap::new(),
            provenance: BTreeMap::new(),
            update_sequence: 0,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
#[allow(dead_code)]
pub struct StreamingGridSource {
    rows: BTreeMap<GridRowId, StreamingGridRow>,
    order: Vec<GridRowId>,
    revision: u64,
}

impl StreamingGridSource {
    #[allow(dead_code)]
    pub fn revision(&self) -> u64 {
        self.revision
    }

    #[allow(dead_code)]
    pub fn upsert_row(&mut self, mut row: StreamingGridRow) {
        self.revision = self.revision.saturating_add(1);
        row.update_sequence = self.revision;
        if !self.rows.contains_key(&row.id) {
            self.order.push(row.id.clone());
        }
        self.rows.insert(row.id.clone(), row);
    }

    #[allow(dead_code)]
    pub fn delete_row(&mut self, row_id: &GridRowId) -> bool {
        let removed = self.rows.remove(row_id).is_some();
        if removed {
            self.revision = self.revision.saturating_add(1);
            self.order.retain(|id| id != row_id);
        }
        removed
    }

    #[allow(dead_code)]
    pub fn update_cell(
        &mut self,
        row_id: GridRowId,
        column_id: GridColumnId,
        value: GridCellValue,
        provenance: Option<GridCellProvenance>,
    ) {
        self.revision = self.revision.saturating_add(1);
        if !self.rows.contains_key(&row_id) {
            self.order.push(row_id.clone());
        }
        let row = self
            .rows
            .entry(row_id.clone())
            .or_insert_with(|| StreamingGridRow::new(row_id));
        row.update_sequence = self.revision;
        row.cells.insert(column_id.clone(), value);
        if let Some(provenance) = provenance {
            row.provenance.insert(column_id, provenance);
        }
    }
}

impl GridRowSource for StreamingGridSource {
    fn row_count(&self) -> usize {
        self.order.len()
    }

    fn revision(&self) -> u64 {
        self.revision
    }

    fn row_id(&self, row_index: usize) -> GridRowId {
        self.order
            .get(row_index)
            .cloned()
            .unwrap_or_else(|| GridRowId::new(format!("missing-stream-row-{row_index}")))
    }

    fn cell_value(&self, row_index: usize, column_id: &GridColumnId) -> GridCellValue {
        self.order
            .get(row_index)
            .and_then(|row_id| self.rows.get(row_id))
            .and_then(|row| row.cells.get(column_id))
            .cloned()
            .unwrap_or(GridCellValue::Empty)
    }

    fn cell_provenance(
        &self,
        row_index: usize,
        column_id: &GridColumnId,
    ) -> Option<GridCellProvenance> {
        self.order
            .get(row_index)
            .and_then(|row_id| self.rows.get(row_id))
            .and_then(|row| row.provenance.get(column_id))
            .cloned()
    }
}
