#![allow(dead_code)]

use super::GridRowSource;
use crate::{GridCellProvenance, GridCellValue, GridColumnId, GridRowId};

type RowIdAccessor = dyn Fn(usize) -> GridRowId + Send + Sync;
type CellValueAccessor = dyn Fn(usize, &GridColumnId) -> GridCellValue + Send + Sync;
type CellProvenanceAccessor =
    dyn Fn(usize, &GridColumnId) -> Option<GridCellProvenance> + Send + Sync;

#[allow(dead_code)]
pub struct VirtualGridSource {
    row_count: usize,
    row_id: Box<RowIdAccessor>,
    cell_value: Box<CellValueAccessor>,
    cell_provenance: Box<CellProvenanceAccessor>,
}

impl VirtualGridSource {
    #[allow(dead_code)]
    pub fn new(
        row_count: usize,
        row_id: impl Fn(usize) -> GridRowId + Send + Sync + 'static,
        cell_value: impl Fn(usize, &GridColumnId) -> GridCellValue + Send + Sync + 'static,
    ) -> Self {
        Self {
            row_count,
            row_id: Box::new(row_id),
            cell_value: Box::new(cell_value),
            cell_provenance: Box::new(|_, _| None),
        }
    }

    #[allow(dead_code)]
    pub fn with_provenance(
        mut self,
        cell_provenance: impl Fn(usize, &GridColumnId) -> Option<GridCellProvenance>
        + Send
        + Sync
        + 'static,
    ) -> Self {
        self.cell_provenance = Box::new(cell_provenance);
        self
    }
}

impl GridRowSource for VirtualGridSource {
    fn row_count(&self) -> usize {
        self.row_count
    }

    fn row_id(&self, row_index: usize) -> GridRowId {
        (self.row_id)(row_index)
    }

    fn cell_value(&self, row_index: usize, column_id: &GridColumnId) -> GridCellValue {
        (self.cell_value)(row_index, column_id)
    }

    fn cell_provenance(
        &self,
        row_index: usize,
        column_id: &GridColumnId,
    ) -> Option<GridCellProvenance> {
        (self.cell_provenance)(row_index, column_id)
    }
}
