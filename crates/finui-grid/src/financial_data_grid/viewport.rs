use super::column::GridColumnDef;
use super::ids::GridColumnId;
use super::state::GridState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridColumnRegion {
    PinnedLeft,
    Center,
    PinnedRight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GridColumnLayout {
    pub column_id: GridColumnId,
    pub x: f32,
    pub width: f32,
    pub region: GridColumnRegion,
}

pub fn scrolled_center_columns(
    visible_columns: &[GridColumnId],
    pinned_left: &[GridColumnId],
    pinned_right: &[GridColumnId],
    columns: &[GridColumnDef],
    state: &GridState,
    horizontal_scroll: f32,
) -> Vec<GridColumnId> {
    let mut skipped_width = 0.0;
    visible_columns
        .iter()
        .filter(|id| !pinned_left.contains(*id))
        .filter(|id| !pinned_right.contains(*id))
        .filter(|id| {
            if skipped_width >= horizontal_scroll {
                return true;
            }
            let width = column_width(columns, state, id);
            skipped_width += width;
            skipped_width >= horizontal_scroll
        })
        .cloned()
        .collect()
}

pub fn center_total_width(
    visible_columns: &[GridColumnId],
    pinned_left: &[GridColumnId],
    pinned_right: &[GridColumnId],
    columns: &[GridColumnDef],
    state: &GridState,
) -> f32 {
    visible_columns
        .iter()
        .filter(|id| !pinned_left.contains(*id))
        .filter(|id| !pinned_right.contains(*id))
        .map(|id| column_width(columns, state, id))
        .sum()
}

pub fn build_column_layout(
    rect_left: f32,
    rect_right: f32,
    visible_columns: &[GridColumnId],
    pinned_left: &[GridColumnId],
    pinned_right: &[GridColumnId],
    columns: &[GridColumnDef],
    state: &GridState,
    horizontal_scroll: f32,
) -> Vec<GridColumnLayout> {
    let right_width: f32 = pinned_right
        .iter()
        .map(|id| column_width(columns, state, id))
        .sum();
    let right_start = (rect_right - right_width).max(rect_left);
    let mut layouts = Vec::new();

    let mut x = rect_left;
    for column_id in pinned_left {
        let width = column_width(columns, state, column_id);
        if x + width <= right_start {
            layouts.push(GridColumnLayout {
                column_id: column_id.clone(),
                x,
                width,
                region: GridColumnRegion::PinnedLeft,
            });
        }
        x += width;
    }

    for column_id in scrolled_center_columns(
        visible_columns,
        pinned_left,
        pinned_right,
        columns,
        state,
        horizontal_scroll,
    ) {
        let width = column_width(columns, state, &column_id);
        if x >= right_start {
            break;
        }
        let clipped_width = width.min(right_start - x);
        if clipped_width > 0.0 {
            layouts.push(GridColumnLayout {
                column_id,
                x,
                width: clipped_width,
                region: GridColumnRegion::Center,
            });
        }
        x += width;
    }

    let mut right_x = right_start;
    for column_id in pinned_right {
        let width = column_width(columns, state, column_id);
        layouts.push(GridColumnLayout {
            column_id: column_id.clone(),
            x: right_x,
            width,
            region: GridColumnRegion::PinnedRight,
        });
        right_x += width;
    }

    layouts
}

pub fn column_width(columns: &[GridColumnDef], state: &GridState, column_id: &GridColumnId) -> f32 {
    columns
        .iter()
        .find(|column| column.id == *column_id)
        .map(|column| {
            state
                .column_widths
                .get(column_id)
                .copied()
                .unwrap_or_else(|| column.width.initial())
        })
        .unwrap_or(96.0)
}
