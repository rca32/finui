#![allow(dead_code)]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::action::GridDensity;
use super::ids::GridColumnId;
use super::state::{GridFilter, GridSort, GridState};

pub const GRID_PERSISTENCE_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridPersistedState {
    pub version: u16,
    pub column_order: Vec<GridColumnId>,
    pub column_widths: BTreeMap<GridColumnId, f32>,
    pub column_visibility: BTreeMap<GridColumnId, bool>,
    pub pinned_left: Vec<GridColumnId>,
    pub pinned_right: Vec<GridColumnId>,
    pub sort: Vec<GridSort>,
    pub filters: Vec<GridFilter>,
    pub density: GridDensity,
}

impl GridPersistedState {
    pub fn from_state(state: &GridState, density: GridDensity) -> Self {
        Self {
            version: GRID_PERSISTENCE_VERSION,
            column_order: state.column_order.clone(),
            column_widths: state.column_widths.clone(),
            column_visibility: state.column_visibility.clone(),
            pinned_left: state.pinned_left.clone(),
            pinned_right: state.pinned_right.clone(),
            sort: state.sort.clone(),
            filters: state.filters.clone(),
            density,
        }
    }

    pub fn apply_to_state(&self, state: &mut GridState) {
        state.column_order = self.column_order.clone();
        state.column_widths = self.column_widths.clone();
        state.column_visibility = self.column_visibility.clone();
        state.pinned_left = self.pinned_left.clone();
        state.pinned_right = self.pinned_right.clone();
        state.sort = self.sort.clone();
        state.filters = self.filters.clone();
        state.selection.selected_cell = None;
        state.selection.selected_row = None;
        state.selection.selected_range = None;
        state.focused_cell = None;
        state.edit_draft = None;
        state.flash_cells.clear();
    }
}
