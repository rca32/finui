#[cfg(feature = "fixtures")]
pub mod duckdb_payload;
pub mod memory;
pub mod streaming;
pub mod virtual_source;

#[allow(unused_imports)]
#[cfg(feature = "fixtures")]
pub use duckdb_payload::{
    GridPayloadAdapterResult, GridPayloadColumn, GridPayloadRow, GridPayloadSourceKind,
    GridTablePayload, adapt_table_payload, duckdb_fixture_payload, endpoint_fixture_payload,
};
pub use memory::{InMemoryGridRow, InMemoryGridSource};
#[allow(unused_imports)]
pub use streaming::{StreamingGridRow, StreamingGridSource};
#[allow(unused_imports)]
pub use virtual_source::VirtualGridSource;

use super::cell::GridCellValue;
use super::ids::{GridColumnId, GridRowId};
use super::provenance::GridCellProvenance;

pub trait GridRowSource {
    fn row_count(&self) -> usize;
    fn revision(&self) -> u64 {
        0
    }
    fn row_id(&self, row_index: usize) -> GridRowId;
    fn cell_value(&self, row_index: usize, column_id: &GridColumnId) -> GridCellValue;
    fn cell_provenance(
        &self,
        row_index: usize,
        column_id: &GridColumnId,
    ) -> Option<GridCellProvenance>;
}
