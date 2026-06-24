use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GridColumnId(pub String);

impl GridColumnId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GridRowId(pub String);

impl GridRowId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GridCellRef {
    pub row_id: GridRowId,
    pub column_id: GridColumnId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridCellRange {
    pub anchor: GridCellRef,
    pub focus: GridCellRef,
}

impl GridCellRange {
    #[allow(dead_code)]
    pub fn new(anchor: GridCellRef, focus: GridCellRef) -> Self {
        Self { anchor, focus }
    }
}
