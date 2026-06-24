use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::cell::GridCellValue;
use super::column::GridColumnDef;
use super::ids::{GridCellRange, GridCellRef, GridColumnId, GridRowId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridSortDirection {
    Asc,
    Desc,
}

impl GridSortDirection {
    pub fn next(self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridSort {
    pub column_id: GridColumnId,
    pub direction: GridSortDirection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridFilter {
    pub column_id: GridColumnId,
    pub query: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridPinSide {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridSelection {
    pub selected_row: Option<GridRowId>,
    pub selected_cell: Option<GridCellRef>,
    pub selected_range: Option<GridCellRange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridEditDraft {
    pub cell: GridCellRef,
    pub value: GridCellValue,
    pub valid: bool,
    pub read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridState {
    pub sort: Vec<GridSort>,
    pub filters: Vec<GridFilter>,
    pub column_order: Vec<GridColumnId>,
    pub column_visibility: BTreeMap<GridColumnId, bool>,
    pub pinned_left: Vec<GridColumnId>,
    pub pinned_right: Vec<GridColumnId>,
    pub column_widths: BTreeMap<GridColumnId, f32>,
    pub horizontal_scroll: f32,
    pub row_scroll: usize,
    pub selection: GridSelection,
    pub focused_cell: Option<GridCellRef>,
    pub edit_draft: Option<GridEditDraft>,
    pub flash_cells: BTreeMap<GridCellRef, u8>,
}

impl Default for GridState {
    fn default() -> Self {
        Self {
            sort: Vec::new(),
            filters: Vec::new(),
            column_order: Vec::new(),
            column_visibility: BTreeMap::new(),
            pinned_left: Vec::new(),
            pinned_right: Vec::new(),
            column_widths: BTreeMap::new(),
            horizontal_scroll: 0.0,
            row_scroll: 0,
            selection: GridSelection {
                selected_row: None,
                selected_cell: None,
                selected_range: None,
            },
            focused_cell: None,
            edit_draft: None,
            flash_cells: BTreeMap::new(),
        }
    }
}

impl GridState {
    pub fn normalize_columns(&mut self, columns: &[GridColumnDef]) {
        let known: BTreeSet<GridColumnId> =
            columns.iter().map(|column| column.id.clone()).collect();
        self.column_order.retain(|id| known.contains(id));
        for column in columns {
            if !self.column_order.contains(&column.id) {
                self.column_order.push(column.id.clone());
            }
            self.column_visibility
                .entry(column.id.clone())
                .or_insert(true);
            self.column_widths
                .entry(column.id.clone())
                .or_insert(column.width.initial());
        }
        self.column_visibility.retain(|id, _| known.contains(id));
        self.column_widths.retain(|id, _| known.contains(id));
        self.pinned_left = normalize_pin_side(&self.pinned_left, &known, &BTreeSet::new());
        let left: BTreeSet<GridColumnId> = self.pinned_left.iter().cloned().collect();
        self.pinned_right = normalize_pin_side(&self.pinned_right, &known, &left);
        self.sort.retain(|sort| known.contains(&sort.column_id));
        self.filters
            .retain(|filter| known.contains(&filter.column_id));
        if self
            .edit_draft
            .as_ref()
            .is_some_and(|draft| !known.contains(&draft.cell.column_id))
        {
            self.edit_draft = None;
        }
        self.flash_cells
            .retain(|cell, _| known.contains(&cell.column_id));
    }

    pub fn visible_column_ids(&self) -> Vec<GridColumnId> {
        self.column_order
            .iter()
            .filter(|id| self.column_visibility.get(id).copied().unwrap_or(true))
            .cloned()
            .collect()
    }

    pub fn pinned_visible_columns(&self) -> (Vec<GridColumnId>, Vec<GridColumnId>) {
        let visible: BTreeSet<GridColumnId> = self.visible_column_ids().into_iter().collect();
        (
            self.pinned_left
                .iter()
                .filter(|id| visible.contains(*id))
                .cloned()
                .collect(),
            self.pinned_right
                .iter()
                .filter(|id| visible.contains(*id))
                .cloned()
                .collect(),
        )
    }

    #[allow(dead_code)]
    pub fn reorder_column(&mut self, column_id: &GridColumnId, target_index: usize) -> bool {
        let Some(current_index) = self
            .column_order
            .iter()
            .position(|candidate| candidate == column_id)
        else {
            return false;
        };
        let column_id = self.column_order.remove(current_index);
        let target_index = target_index.min(self.column_order.len());
        self.column_order.insert(target_index, column_id);
        true
    }

    #[allow(dead_code)]
    pub fn select_range(&mut self, anchor: GridCellRef, focus: GridCellRef) {
        self.selection.selected_row = Some(focus.row_id.clone());
        self.selection.selected_cell = Some(focus.clone());
        self.selection.selected_range = Some(GridCellRange::new(anchor, focus.clone()));
        self.focused_cell = Some(focus);
    }

    #[allow(dead_code)]
    pub fn begin_edit(&mut self, cell: GridCellRef, value: GridCellValue) {
        self.focused_cell = Some(cell.clone());
        self.edit_draft = Some(GridEditDraft {
            cell,
            value,
            valid: true,
            read_only: false,
        });
    }

    #[allow(dead_code)]
    pub fn begin_invalid_edit(&mut self, cell: GridCellRef, value: GridCellValue) {
        self.focused_cell = Some(cell.clone());
        self.edit_draft = Some(GridEditDraft {
            cell,
            value,
            valid: false,
            read_only: false,
        });
    }

    #[allow(dead_code)]
    pub fn begin_read_only_edit(&mut self, cell: GridCellRef, value: GridCellValue) {
        self.focused_cell = Some(cell.clone());
        self.edit_draft = Some(GridEditDraft {
            cell,
            value,
            valid: true,
            read_only: true,
        });
    }

    #[allow(dead_code)]
    pub fn commit_edit(&mut self) -> Option<GridEditDraft> {
        let draft = self.edit_draft.take()?;
        if draft.valid && !draft.read_only {
            Some(draft)
        } else {
            self.edit_draft = Some(draft);
            None
        }
    }

    #[allow(dead_code)]
    pub fn cancel_edit(&mut self) {
        self.edit_draft = None;
    }

    pub fn mark_flash(&mut self, cell: GridCellRef) {
        self.flash_cells.insert(cell, 3);
    }

    pub fn decay_flash_cells(&mut self) {
        self.flash_cells.retain(|_, remaining| {
            *remaining = remaining.saturating_sub(1);
            *remaining > 0
        });
    }
}

fn normalize_pin_side(
    pinned: &[GridColumnId],
    known: &BTreeSet<GridColumnId>,
    excluded: &BTreeSet<GridColumnId>,
) -> Vec<GridColumnId> {
    let mut seen = BTreeSet::new();
    pinned
        .iter()
        .filter(|id| known.contains(*id) && !excluded.contains(*id))
        .filter(|id| seen.insert((*id).clone()))
        .cloned()
        .collect()
}
