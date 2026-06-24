use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::action::GridAction;
use super::ids::{GridCellRef, GridColumnId, GridRowId};
use super::source::GridRowSource;
use super::state::{GridFilter, GridPinSide, GridSort, GridSortDirection, GridState};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridAnnotation {
    pub cells: Vec<GridCellRef>,
    pub rows: Vec<GridRowId>,
    pub reason_ref: String,
    pub evidence_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridSnapshot {
    pub grid_id: String,
    pub row_count: usize,
    pub visible_rows: Vec<GridRowId>,
    pub visible_columns: Vec<GridColumnId>,
    pub sort: Vec<GridSort>,
    pub filters: Vec<GridFilter>,
    pub selected_row: Option<GridRowId>,
    pub focused_cell: Option<GridCellRef>,
    pub highlights: Vec<GridAnnotation>,
    pub open_evidence_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridAgentCommand {
    SetSort {
        column_id: GridColumnId,
        direction: GridSortDirection,
        reason_ref: String,
    },
    SetFilter {
        column_id: GridColumnId,
        query: String,
        reason_ref: String,
    },
    SelectRow {
        row_id: GridRowId,
        reason_ref: String,
    },
    HighlightRows {
        row_ids: Vec<GridRowId>,
        reason_ref: String,
        evidence_ref: Option<String>,
    },
    HighlightCells {
        cell_refs: Vec<GridCellRef>,
        reason_ref: String,
        evidence_ref: Option<String>,
    },
    OpenRowDetail {
        row_id: GridRowId,
        reason_ref: String,
    },
    PinColumn {
        column_id: GridColumnId,
        side: GridPinSide,
        reason_ref: String,
    },
    HideColumn {
        column_id: GridColumnId,
        reason_ref: String,
    },
    ShowColumn {
        column_id: GridColumnId,
        reason_ref: String,
    },
    ExportSelection {
        format: String,
        reason_ref: String,
    },
    AttachEvidence {
        target: GridEvidenceTarget,
        evidence_ref: String,
        reason_ref: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridEvidenceTarget {
    Row(GridRowId),
    Cell(GridCellRef),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridAgentActionLogEntry {
    pub command: GridAgentCommand,
    pub reversible: bool,
    pub before: GridState,
    pub after: GridState,
    pub evidence_ref: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridAgentBridge {
    pub annotations: Vec<GridAnnotation>,
    pub action_log: Vec<GridAgentActionLogEntry>,
    pub open_evidence_ref: Option<String>,
}

impl GridAgentBridge {
    pub fn apply_command(
        &mut self,
        state: &mut GridState,
        command: GridAgentCommand,
    ) -> Vec<GridAction> {
        let before = state.clone();
        let mut actions = Vec::new();
        let mut reversible = false;
        let mut evidence_ref = None;
        match &command {
            GridAgentCommand::SetSort {
                column_id,
                direction,
                ..
            } => {
                state.sort = vec![GridSort {
                    column_id: column_id.clone(),
                    direction: *direction,
                }];
                reversible = true;
                actions.push(GridAction::SortChanged(state.sort.clone()));
            }
            GridAgentCommand::SetFilter {
                column_id, query, ..
            } => {
                state
                    .filters
                    .retain(|filter| filter.column_id != *column_id);
                if !query.is_empty() {
                    state.filters.push(GridFilter {
                        column_id: column_id.clone(),
                        query: query.clone(),
                    });
                }
                reversible = true;
                actions.push(GridAction::FilterChanged(state.filters.clone()));
            }
            GridAgentCommand::SelectRow { row_id, .. } => {
                state.selection.selected_row = Some(row_id.clone());
                state.focused_cell = None;
                actions.push(GridAction::RowSelected(row_id.clone()));
            }
            GridAgentCommand::HighlightRows {
                row_ids,
                reason_ref,
                evidence_ref: command_evidence_ref,
            } => {
                evidence_ref = command_evidence_ref.clone();
                self.annotations.push(GridAnnotation {
                    rows: row_ids.clone(),
                    cells: Vec::new(),
                    reason_ref: reason_ref.clone(),
                    evidence_ref: command_evidence_ref.clone(),
                });
            }
            GridAgentCommand::HighlightCells {
                cell_refs,
                reason_ref,
                evidence_ref: command_evidence_ref,
            } => {
                evidence_ref = command_evidence_ref.clone();
                self.annotations.push(GridAnnotation {
                    rows: Vec::new(),
                    cells: cell_refs.clone(),
                    reason_ref: reason_ref.clone(),
                    evidence_ref: command_evidence_ref.clone(),
                });
            }
            GridAgentCommand::OpenRowDetail { row_id, .. } => {
                state.selection.selected_row = Some(row_id.clone());
                self.open_evidence_ref = Some(format!("row-detail:{}", row_id.0));
                actions.push(GridAction::RowDetailOpened(row_id.clone()));
                actions.push(GridAction::EvidenceOpened(format!(
                    "row-detail:{}",
                    row_id.0
                )));
            }
            GridAgentCommand::PinColumn {
                column_id, side, ..
            } => {
                state.pinned_left.retain(|id| id != column_id);
                state.pinned_right.retain(|id| id != column_id);
                match side {
                    GridPinSide::Left => state.pinned_left.push(column_id.clone()),
                    GridPinSide::Right => state.pinned_right.push(column_id.clone()),
                }
                reversible = true;
                actions.push(GridAction::ColumnPinned {
                    column_id: column_id.clone(),
                    side: *side,
                });
            }
            GridAgentCommand::HideColumn { column_id, .. } => {
                state.column_visibility.insert(column_id.clone(), false);
                reversible = true;
                actions.push(GridAction::ColumnVisibilityChanged {
                    column_id: column_id.clone(),
                    visible: false,
                });
            }
            GridAgentCommand::ShowColumn { column_id, .. } => {
                state.column_visibility.insert(column_id.clone(), true);
                reversible = true;
                actions.push(GridAction::ColumnVisibilityChanged {
                    column_id: column_id.clone(),
                    visible: true,
                });
            }
            GridAgentCommand::ExportSelection { format, .. } => {
                actions.push(GridAction::CopiedSelection {
                    format: format.clone(),
                });
            }
            GridAgentCommand::AttachEvidence {
                target,
                evidence_ref: command_evidence_ref,
                reason_ref,
            } => {
                evidence_ref = Some(command_evidence_ref.clone());
                match target {
                    GridEvidenceTarget::Row(row_id) => self.annotations.push(GridAnnotation {
                        rows: vec![row_id.clone()],
                        cells: Vec::new(),
                        reason_ref: reason_ref.clone(),
                        evidence_ref: Some(command_evidence_ref.clone()),
                    }),
                    GridEvidenceTarget::Cell(cell_ref) => self.annotations.push(GridAnnotation {
                        rows: Vec::new(),
                        cells: vec![cell_ref.clone()],
                        reason_ref: reason_ref.clone(),
                        evidence_ref: Some(command_evidence_ref.clone()),
                    }),
                }
            }
        }
        self.action_log.push(GridAgentActionLogEntry {
            command,
            reversible,
            before,
            after: state.clone(),
            evidence_ref,
        });
        actions
    }

    pub fn undo_last_reversible(&mut self, state: &mut GridState) -> bool {
        if let Some(index) = self.action_log.iter().rposition(|entry| entry.reversible) {
            let entry = self.action_log.remove(index);
            *state = entry.before;
            return true;
        }
        false
    }

    pub fn snapshot(
        &self,
        grid_id: impl Into<String>,
        state: &GridState,
        source: &dyn GridRowSource,
        visible_rows: Range<usize>,
    ) -> GridSnapshot {
        GridSnapshot {
            grid_id: grid_id.into(),
            row_count: source.row_count(),
            visible_rows: visible_rows.map(|row| source.row_id(row)).collect(),
            visible_columns: state.visible_column_ids(),
            sort: state.sort.clone(),
            filters: state.filters.clone(),
            selected_row: state.selection.selected_row.clone(),
            focused_cell: state.focused_cell.clone(),
            highlights: self.annotations.clone(),
            open_evidence_ref: self.open_evidence_ref.clone(),
        }
    }
}
