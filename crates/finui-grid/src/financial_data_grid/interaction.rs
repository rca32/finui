use eframe::egui;

use super::action::GridAction;
use super::ids::GridColumnId;
use super::source::GridRowSource;
use super::state::GridState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridKeyboardCommand {
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Left,
    Right,
}

pub fn handle_grid_keyboard(
    ui: &mut egui::Ui,
    source: &dyn GridRowSource,
    state: &mut GridState,
    row_model: &[usize],
    visible_columns: &[GridColumnId],
    visible_row_count: usize,
    actions: &mut Vec<GridAction>,
) {
    for command in ui.input(active_keyboard_commands) {
        apply_grid_keyboard_command(
            source,
            state,
            row_model,
            visible_columns,
            visible_row_count,
            command,
            actions,
        );
    }
}

pub fn apply_grid_keyboard_command(
    source: &dyn GridRowSource,
    state: &mut GridState,
    row_model: &[usize],
    visible_columns: &[GridColumnId],
    visible_row_count: usize,
    command: GridKeyboardCommand,
    actions: &mut Vec<GridAction>,
) {
    if row_model.is_empty() {
        return;
    }
    match command {
        GridKeyboardCommand::Left | GridKeyboardCommand::Right => {
            move_focused_column(state, visible_columns, command, actions);
        }
        GridKeyboardCommand::Up
        | GridKeyboardCommand::Down
        | GridKeyboardCommand::PageUp
        | GridKeyboardCommand::PageDown
        | GridKeyboardCommand::Home
        | GridKeyboardCommand::End => {
            move_selected_row(
                source,
                state,
                row_model,
                visible_row_count.max(1),
                command,
                actions,
            );
        }
    }
}

fn active_keyboard_commands(input: &egui::InputState) -> Vec<GridKeyboardCommand> {
    [
        (egui::Key::ArrowDown, GridKeyboardCommand::Down),
        (egui::Key::ArrowUp, GridKeyboardCommand::Up),
        (egui::Key::PageDown, GridKeyboardCommand::PageDown),
        (egui::Key::PageUp, GridKeyboardCommand::PageUp),
        (egui::Key::Home, GridKeyboardCommand::Home),
        (egui::Key::End, GridKeyboardCommand::End),
        (egui::Key::ArrowLeft, GridKeyboardCommand::Left),
        (egui::Key::ArrowRight, GridKeyboardCommand::Right),
    ]
    .into_iter()
    .filter_map(|(key, command)| input.key_pressed(key).then_some(command))
    .collect()
}

fn move_selected_row(
    source: &dyn GridRowSource,
    state: &mut GridState,
    row_model: &[usize],
    visible_row_count: usize,
    command: GridKeyboardCommand,
    actions: &mut Vec<GridAction>,
) {
    let current = state
        .selection
        .selected_row
        .as_ref()
        .and_then(|row_id| {
            row_model
                .iter()
                .position(|index| source.row_id(*index) == *row_id)
        })
        .unwrap_or(0);
    let last = row_model.len().saturating_sub(1);
    let next = match command {
        GridKeyboardCommand::Up => current.saturating_sub(1),
        GridKeyboardCommand::Down => (current + 1).min(last),
        GridKeyboardCommand::PageUp => current.saturating_sub(visible_row_count),
        GridKeyboardCommand::PageDown => (current + visible_row_count).min(last),
        GridKeyboardCommand::Home => 0,
        GridKeyboardCommand::End => last,
        GridKeyboardCommand::Left | GridKeyboardCommand::Right => current,
    };
    let row_id = source.row_id(row_model[next]);
    state.selection.selected_row = Some(row_id.clone());
    if let Some(focused) = &mut state.focused_cell {
        focused.row_id = row_id.clone();
    }
    if next < state.row_scroll {
        state.row_scroll = next;
    } else if next >= state.row_scroll.saturating_add(visible_row_count) {
        state.row_scroll = next.saturating_sub(visible_row_count.saturating_sub(1));
    }
    actions.push(GridAction::RowSelected(row_id));
}

fn move_focused_column(
    state: &mut GridState,
    visible_columns: &[GridColumnId],
    command: GridKeyboardCommand,
    actions: &mut Vec<GridAction>,
) {
    if visible_columns.is_empty() {
        return;
    }
    let row_id = state
        .focused_cell
        .as_ref()
        .map(|cell| cell.row_id.clone())
        .or_else(|| state.selection.selected_row.clone());
    let Some(row_id) = row_id else {
        return;
    };
    let current = state
        .focused_cell
        .as_ref()
        .and_then(|cell| {
            visible_columns
                .iter()
                .position(|column_id| *column_id == cell.column_id)
        })
        .unwrap_or(0);
    let next = match command {
        GridKeyboardCommand::Left => current.saturating_sub(1),
        GridKeyboardCommand::Right => (current + 1).min(visible_columns.len().saturating_sub(1)),
        _ => current,
    };
    let cell = super::ids::GridCellRef {
        row_id,
        column_id: visible_columns[next].clone(),
    };
    state.selection.selected_cell = Some(cell.clone());
    state.focused_cell = Some(cell.clone());
    actions.push(GridAction::CellSelected(cell));
}
