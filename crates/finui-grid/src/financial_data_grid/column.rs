use serde::{Deserialize, Serialize};

use super::cell::GridValueKind;
use super::ids::GridColumnId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridCellAlign {
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GridColumnWidth {
    Fixed(f32),
    Flexible { min: f32, ideal: f32 },
}

impl GridColumnWidth {
    pub fn initial(self) -> f32 {
        match self {
            Self::Fixed(width) => width,
            Self::Flexible { ideal, .. } => ideal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GridFormatter {
    Plain,
    Decimal { decimals: usize },
    ThousandsDecimal { decimals: usize },
    Percent { decimals: usize },
    CompactQuantity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridColumnDef {
    pub id: GridColumnId,
    pub label: String,
    pub kind: GridValueKind,
    pub align: GridCellAlign,
    pub width: GridColumnWidth,
    pub sortable: bool,
    pub filterable: bool,
    pub pinnable: bool,
    pub formatter: GridFormatter,
}

impl GridColumnDef {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: GridColumnId::new(id),
            label: label.into(),
            kind: GridValueKind::Text,
            align: GridCellAlign::Left,
            width: GridColumnWidth::Fixed(96.0),
            sortable: false,
            filterable: false,
            pinnable: false,
            formatter: GridFormatter::Plain,
        }
    }

    pub fn kind(mut self, kind: GridValueKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn align(mut self, align: GridCellAlign) -> Self {
        self.align = align;
        self
    }

    pub fn width(mut self, width: GridColumnWidth) -> Self {
        self.width = width;
        self
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    pub fn pinnable(mut self, pinnable: bool) -> Self {
        self.pinnable = pinnable;
        self
    }

    pub fn formatter(mut self, formatter: GridFormatter) -> Self {
        self.formatter = formatter;
        self
    }
}
