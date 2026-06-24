use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::ids::{GridCellRange, GridCellRef, GridColumnId, GridRowId};
use super::state::{GridFilter, GridPinSide, GridSort};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridAction {
    SortChanged(Vec<GridSort>),
    FilterChanged(Vec<GridFilter>),
    RowSelected(GridRowId),
    CellSelected(GridCellRef),
    ColumnResized {
        column_id: GridColumnId,
        width: u32,
    },
    ColumnVisibilityChanged {
        column_id: GridColumnId,
        visible: bool,
    },
    ColumnPinned {
        column_id: GridColumnId,
        side: GridPinSide,
    },
    ColumnReordered {
        column_id: GridColumnId,
        target_index: usize,
    },
    RangeSelected(GridCellRange),
    RowDetailOpened(GridRowId),
    EvidenceOpened(String),
    CopiedSelection {
        format: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridOutput {
    pub actions: Vec<GridAction>,
    pub hovered_cell: Option<GridCellRef>,
    pub focused_cell: Option<GridCellRef>,
    pub visible_rows: Range<usize>,
    pub visible_columns: Vec<GridColumnId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum GridDensity {
    Compact,
    Comfortable,
}

impl GridDensity {
    pub fn row_height(self) -> f32 {
        match self {
            Self::Compact => 24.0,
            Self::Comfortable => 31.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum GridProvenancePolicy {
    Hidden,
    VisibleOnHover,
}
