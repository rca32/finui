use eframe::egui::{self, Align2, Color32, Pos2, Rect, Sense, Stroke, Vec2};

use super::action::{GridAction, GridProvenancePolicy};
use super::agent::GridAgentBridge;
use super::cell::GridCellValue;
use super::column::{GridCellAlign, GridColumnDef};
use super::ids::{GridCellRef, GridRowId};
use super::provenance::GridCellProvenance;
use super::source::GridRowSource;
use super::state::GridState;
use super::viewport::GridColumnLayout;
use finui_primitives::LayerPlacement;
use finui_primitives::{
    ContextMenuItemOptions, ContextMenuOptions, HoverCardOptions, PrimitiveTheme, ThemeMode,
    primitive_context_menu_item, primitive_hover_card_content, radix_colors, show_context_menu,
    show_hover_card,
};

pub fn paint_row_cells(
    ui: &mut egui::Ui,
    painter: &egui::Painter,
    grid_id: &str,
    grid_rect: Rect,
    row_rect: Rect,
    row_index: usize,
    visible_slot: usize,
    row_id: &GridRowId,
    column_layouts: &[GridColumnLayout],
    columns: &[GridColumnDef],
    source: &dyn GridRowSource,
    state: &mut GridState,
    agent_bridge: Option<&GridAgentBridge>,
    provenance_policy: GridProvenancePolicy,
    row_selection_only: bool,
    theme_mode: ThemeMode,
    actions: &mut Vec<GridAction>,
    hovered_cell: &mut Option<GridCellRef>,
) {
    let paint_theme = grid_paint_theme(theme_mode);
    for layout in column_layouts {
        let Some(column) = columns.iter().find(|column| column.id == layout.column_id) else {
            continue;
        };
        if layout.x > grid_rect.right() {
            break;
        }
        let cell_rect = Rect::from_min_size(
            Pos2::new(layout.x, row_rect.top()),
            Vec2::new(layout.width, row_rect.height()),
        );
        let cell_ref = GridCellRef {
            row_id: row_id.clone(),
            column_id: column.id.clone(),
        };
        let response = ui
            .interact(
                cell_rect,
                ui.make_persistent_id((grid_id, "cell-slot", visible_slot, &column.id.0)),
                Sense::click_and_drag(),
            )
            .on_hover_cursor(egui::CursorIcon::PointingHand);
        let range_anchor_id = ui.make_persistent_id((grid_id, "range-drag-anchor"));
        if response.hovered() {
            *hovered_cell = Some(cell_ref.clone());
            if provenance_policy == GridProvenancePolicy::VisibleOnHover {
                if let Some(provenance) = source.cell_provenance(row_index, &column.id) {
                    let description = provenance_hover_text(&provenance);
                    let _ = show_hover_card(
                        ui.ctx(),
                        true,
                        HoverCardOptions::anchored(
                            (grid_id, "cell-provenance-hover", &row_id.0, &column.id.0),
                            cell_rect,
                            292.0,
                            LayerPlacement::RightStart {
                                offset: egui::vec2(6.0, 0.0),
                            },
                        ),
                        |ui| {
                            primitive_hover_card_content(
                                ui,
                                "Cell provenance",
                                &description,
                                PrimitiveTheme::for_mode(theme_mode),
                            );
                            None::<()>
                        },
                    );
                }
            }
        }
        if response.clicked() {
            response.request_focus();
            state.selection.selected_row = Some(row_id.clone());
            if row_selection_only {
                state.selection.selected_cell = None;
                state.selection.selected_range = None;
                state.focused_cell = None;
                actions.push(GridAction::RowSelected(row_id.clone()));
            } else {
                state.selection.selected_cell = Some(cell_ref.clone());
                state.focused_cell = Some(cell_ref.clone());
                actions.push(GridAction::CellSelected(cell_ref.clone()));
            }
        }
        if response.drag_started() {
            response.request_focus();
            state.selection.selected_row = Some(row_id.clone());
            if row_selection_only {
                state.selection.selected_cell = None;
                state.selection.selected_range = None;
                state.focused_cell = None;
            } else {
                ui.memory_mut(|memory| {
                    memory
                        .data
                        .insert_persisted(range_anchor_id, Some(cell_ref.clone()));
                });
                state.selection.selected_cell = Some(cell_ref.clone());
                state.focused_cell = Some(cell_ref.clone());
            }
        }
        if response.dragged() && !row_selection_only {
            ui.output_mut(|output| output.cursor_icon = egui::CursorIcon::Grabbing);
            let anchor = ui.memory_mut(|memory| {
                memory
                    .data
                    .get_persisted_mut_or_default::<Option<GridCellRef>>(range_anchor_id)
                    .clone()
            });
            if let Some(anchor) = anchor {
                state.select_range(anchor.clone(), cell_ref.clone());
                if let Some(range) = state.selection.selected_range.clone() {
                    actions.push(GridAction::RangeSelected(range));
                }
            }
        }
        if !row_selection_only {
            let active_anchor = ui.memory_mut(|memory| {
                memory
                    .data
                    .get_persisted::<Option<GridCellRef>>(range_anchor_id)
                    .flatten()
            });
            if let Some(anchor) = active_anchor {
                let pointer_pos = ui.input(|input| input.pointer.interact_pos());
                let primary_down = ui.input(|input| input.pointer.primary_down());
                if primary_down && pointer_pos.is_some_and(|pos| cell_rect.contains(pos)) {
                    ui.output_mut(|output| output.cursor_icon = egui::CursorIcon::Grabbing);
                    state.select_range(anchor, cell_ref.clone());
                    if let Some(range) = state.selection.selected_range.clone() {
                        actions.push(GridAction::RangeSelected(range));
                    }
                }
            }
        }
        if response.drag_stopped() {
            ui.memory_mut(|memory| {
                memory.data.remove::<Option<GridCellRef>>(range_anchor_id);
            });
        }
        show_cell_context_menu(
            ui, grid_id, row_index, row_id, column, cell_rect, &response, source, state, actions,
        );
        if is_annotated(agent_bridge, row_id, &cell_ref) {
            painter.rect_filled(cell_rect.shrink(1.0), 0.0, paint_theme.agent_fill);
        }
        if range_touches_cell(state, &cell_ref, columns) {
            painter.rect_filled(cell_rect.shrink(2.0), 0.0, paint_theme.range_fill);
        }
        if state.flash_cells.contains_key(&cell_ref) {
            painter.rect_filled(cell_rect.shrink(1.0), 0.0, paint_theme.flash_fill);
        }
        if !row_selection_only && state.selection.selected_cell.as_ref() == Some(&cell_ref) {
            painter.rect_stroke(
                cell_rect.shrink(1.5),
                0.0,
                Stroke::new(1.5, paint_theme.selected_cell_stroke),
                egui::StrokeKind::Inside,
            );
        }
        if !row_selection_only && state.focused_cell.as_ref() == Some(&cell_ref) {
            painter.rect_stroke(
                cell_rect.shrink(3.0),
                0.0,
                Stroke::new(1.5, paint_theme.focus_stroke),
                egui::StrokeKind::Inside,
            );
        }
        if let Some(draft) = state
            .edit_draft
            .as_ref()
            .filter(|draft| draft.cell == cell_ref)
        {
            let color = if draft.read_only {
                paint_theme.edit_read_only_stroke
            } else if draft.valid {
                paint_theme.edit_valid_stroke
            } else {
                paint_theme.edit_invalid_stroke
            };
            painter.rect_stroke(
                cell_rect.shrink(4.5),
                0.0,
                Stroke::new(2.0, color),
                egui::StrokeKind::Inside,
            );
        }
        if let Some(provenance) = source.cell_provenance(row_index, &column.id) {
            if provenance.stale_after_ms.is_some() {
                painter.rect_filled(
                    Rect::from_min_size(cell_rect.left_top(), Vec2::new(3.0, cell_rect.height())),
                    0.0,
                    paint_theme.stale_marker,
                );
            }
            if response.double_clicked() {
                if let Some(evidence_ref) = provenance.evidence_ref {
                    actions.push(GridAction::EvidenceOpened(evidence_ref));
                }
            }
        }
        let cell_value = source.cell_value(row_index, &column.id);
        let raw_value = cell_value.display(&column.formatter);
        let value = ellipsize_cell_text(&raw_value, cell_rect.width());
        match &cell_value {
            GridCellValue::Sparkline(values) => {
                paint_sparkline(
                    painter,
                    cell_rect.shrink2(Vec2::new(8.0, 5.0)),
                    values,
                    paint_theme,
                );
            }
            GridCellValue::DeltaBar(value) => {
                paint_delta_bar(
                    painter,
                    cell_rect.shrink2(Vec2::new(8.0, 7.0)),
                    *value,
                    paint_theme,
                );
            }
            _ => {}
        }
        if matches!(cell_value, GridCellValue::Status(_)) {
            let badge_color = match raw_value.as_str() {
                "live" => paint_theme.live_badge_fill,
                "stale" => paint_theme.stale_badge_fill,
                _ => paint_theme.badge_fill,
            };
            painter.rect_filled(cell_rect.shrink2(Vec2::new(6.0, 4.0)), 4.0, badge_color);
        }
        let (anchor, align) = match column.align {
            GridCellAlign::Left => (
                cell_rect.left_center() + Vec2::new(8.0, 0.0),
                Align2::LEFT_CENTER,
            ),
            GridCellAlign::Right => (
                cell_rect.right_center() - Vec2::new(8.0, 0.0),
                Align2::RIGHT_CENTER,
            ),
            GridCellAlign::Center => (cell_rect.center(), Align2::CENTER_CENTER),
        };
        if !matches!(
            cell_value,
            GridCellValue::Sparkline(_) | GridCellValue::DeltaBar(_)
        ) {
            let clipped_painter = painter.with_clip_rect(cell_rect.shrink2(Vec2::new(1.0, 0.0)));
            clipped_painter.text(
                anchor,
                align,
                value,
                finui_primitives::scaled_proportional_font(ui, 12.0),
                cell_text_color(&cell_value, paint_theme),
            );
        }
        painter.line_segment(
            [
                Pos2::new(cell_rect.right(), cell_rect.top()),
                Pos2::new(cell_rect.right(), cell_rect.bottom()),
            ],
            Stroke::new(1.0, paint_theme.grid_line),
        );
    }
}

pub fn ellipsize_cell_text(value: &str, cell_width: f32) -> String {
    const HORIZONTAL_PADDING: f32 = 16.0;
    const APPROX_CHAR_WIDTH: f32 = 7.0;
    if value.is_empty() {
        return String::new();
    }
    let available = (cell_width - HORIZONTAL_PADDING).max(0.0);
    let max_chars = (available / APPROX_CHAR_WIDTH).floor() as usize;
    let char_count = value.chars().count();
    if max_chars == 0 {
        return String::new();
    }
    if char_count <= max_chars {
        return value.to_owned();
    }
    if max_chars <= 3 {
        return ".".repeat(max_chars);
    }
    let mut clipped = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    clipped.push_str("...");
    clipped
}

fn show_cell_context_menu(
    ui: &mut egui::Ui,
    grid_id: &str,
    row_index: usize,
    row_id: &GridRowId,
    column: &GridColumnDef,
    cell_rect: Rect,
    response: &egui::Response,
    source: &dyn GridRowSource,
    state: &mut GridState,
    actions: &mut Vec<GridAction>,
) {
    let cell_ref = GridCellRef {
        row_id: row_id.clone(),
        column_id: column.id.clone(),
    };
    let menu_state_id = ui.make_persistent_id((grid_id, "cell-context-menu-state"));
    if response.secondary_clicked() {
        let menu_position = response
            .interact_pointer_pos()
            .unwrap_or_else(|| cell_rect.left_top());
        ui.memory_mut(|memory| {
            memory
                .data
                .insert_persisted(menu_state_id, Some((cell_ref.clone(), menu_position)));
        });
    }
    let cell_menu_state = ui.memory_mut(|memory| {
        memory
            .data
            .get_persisted_mut_or_default::<Option<(GridCellRef, Pos2)>>(menu_state_id)
            .clone()
    });
    let Some((_menu_cell, menu_position)) = cell_menu_state.filter(|state| state.0 == cell_ref)
    else {
        return;
    };
    let has_evidence = source
        .cell_provenance(row_index, &column.id)
        .and_then(|provenance| provenance.evidence_ref)
        .is_some();
    let output = show_context_menu(
        ui.ctx(),
        ContextMenuOptions::at(
            (grid_id, "cell-context-menu", &row_id.0, &column.id.0),
            menu_position,
            184.0,
        )
        .max_height(180.0),
        |ui| {
            if primitive_context_menu_item(ui, "Copy cell", ContextMenuItemOptions::new(164.0))
                .clicked()
            {
                return Some("copy");
            }
            if primitive_context_menu_item(
                ui,
                "Open evidence",
                ContextMenuItemOptions::new(164.0).disabled(!has_evidence),
            )
            .clicked()
            {
                return Some("evidence");
            }
            None
        },
    );
    match output.action {
        Some("copy") => {
            state.selection.selected_cell = Some(cell_ref.clone());
            state.focused_cell = Some(cell_ref.clone());
            let text = source
                .cell_value(row_index, &column.id)
                .display(&column.formatter);
            ui.copy_text(text);
            actions.push(GridAction::CopiedSelection {
                format: "cell-display".to_owned(),
            });
            ui.memory_mut(|memory| {
                memory
                    .data
                    .remove::<Option<(GridCellRef, Pos2)>>(menu_state_id)
            });
        }
        Some("evidence") => {
            if let Some(evidence_ref) = source
                .cell_provenance(row_index, &column.id)
                .and_then(|provenance| provenance.evidence_ref)
            {
                actions.push(GridAction::EvidenceOpened(evidence_ref));
            }
            ui.memory_mut(|memory| {
                memory
                    .data
                    .remove::<Option<(GridCellRef, Pos2)>>(menu_state_id)
            });
        }
        _ => {}
    }
    if output.should_close {
        ui.memory_mut(|memory| {
            memory
                .data
                .remove::<Option<(GridCellRef, Pos2)>>(menu_state_id)
        });
    }
}

fn range_touches_cell(
    state: &GridState,
    cell_ref: &GridCellRef,
    columns: &[GridColumnDef],
) -> bool {
    let Some(range) = state.selection.selected_range.as_ref() else {
        return false;
    };
    let Some(anchor_column) = columns
        .iter()
        .position(|column| column.id == range.anchor.column_id)
    else {
        return range.anchor == *cell_ref || range.focus == *cell_ref;
    };
    let Some(focus_column) = columns
        .iter()
        .position(|column| column.id == range.focus.column_id)
    else {
        return range.anchor == *cell_ref || range.focus == *cell_ref;
    };
    let Some(cell_column) = columns
        .iter()
        .position(|column| column.id == cell_ref.column_id)
    else {
        return false;
    };
    let min_column = anchor_column.min(focus_column);
    let max_column = anchor_column.max(focus_column);
    if !(min_column..=max_column).contains(&cell_column) {
        return false;
    }
    string_between(
        &cell_ref.row_id.0,
        &range.anchor.row_id.0,
        &range.focus.row_id.0,
    )
}

fn string_between(value: &str, a: &str, b: &str) -> bool {
    if a <= b {
        value >= a && value <= b
    } else {
        value >= b && value <= a
    }
}

fn cell_text_color(value: &GridCellValue, theme: GridPaintTheme) -> Color32 {
    match value {
        GridCellValue::Decimal(value) | GridCellValue::DeltaBar(value) if *value > 0.0 => {
            theme.positive_text
        }
        GridCellValue::Decimal(value) | GridCellValue::DeltaBar(value) if *value < 0.0 => {
            theme.negative_text
        }
        GridCellValue::Error(_) => theme.error_text,
        GridCellValue::AgentAnnotation(_) => theme.agent_text,
        _ => theme.text,
    }
}

fn paint_sparkline(painter: &egui::Painter, rect: Rect, values: &[f64], theme: GridPaintTheme) {
    if values.len() < 2 || rect.width() <= 1.0 || rect.height() <= 1.0 {
        return;
    }
    let min = values.iter().copied().fold(f64::INFINITY, f64::min);
    let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let span = (max - min).max(f64::EPSILON);
    let step = rect.width() / (values.len().saturating_sub(1) as f32);
    let mut previous = None;
    for (index, value) in values.iter().enumerate() {
        let x = rect.left() + index as f32 * step;
        let normalized = ((*value - min) / span) as f32;
        let y = rect.bottom() - normalized * rect.height();
        let point = Pos2::new(x, y);
        if let Some(previous) = previous {
            painter.line_segment([previous, point], Stroke::new(1.2, theme.sparkline_stroke));
        }
        previous = Some(point);
    }
}

fn paint_delta_bar(painter: &egui::Painter, rect: Rect, value: f64, theme: GridPaintTheme) {
    if rect.width() <= 1.0 || rect.height() <= 1.0 {
        return;
    }
    let center_x = rect.center().x;
    painter.line_segment(
        [
            Pos2::new(center_x, rect.top()),
            Pos2::new(center_x, rect.bottom()),
        ],
        Stroke::new(1.0, theme.grid_line),
    );
    let width = ((value.abs() / 100.0).min(1.0) as f32) * rect.width() * 0.5;
    let color = if value >= 0.0 {
        theme.delta_positive_fill
    } else {
        theme.delta_negative_fill
    };
    let bar_rect = if value >= 0.0 {
        Rect::from_min_max(
            Pos2::new(center_x, rect.top()),
            Pos2::new(center_x + width, rect.bottom()),
        )
    } else {
        Rect::from_min_max(
            Pos2::new(center_x - width, rect.top()),
            Pos2::new(center_x, rect.bottom()),
        )
    };
    painter.rect_filled(bar_rect, 3.0, color);
}

#[derive(Debug, Clone, Copy)]
struct GridPaintTheme {
    text: Color32,
    positive_text: Color32,
    negative_text: Color32,
    error_text: Color32,
    agent_text: Color32,
    agent_fill: Color32,
    range_fill: Color32,
    flash_fill: Color32,
    selected_cell_stroke: Color32,
    focus_stroke: Color32,
    edit_valid_stroke: Color32,
    edit_invalid_stroke: Color32,
    edit_read_only_stroke: Color32,
    stale_marker: Color32,
    live_badge_fill: Color32,
    stale_badge_fill: Color32,
    badge_fill: Color32,
    grid_line: Color32,
    sparkline_stroke: Color32,
    delta_positive_fill: Color32,
    delta_negative_fill: Color32,
}

fn grid_paint_theme(mode: ThemeMode) -> GridPaintTheme {
    match mode {
        ThemeMode::Light => GridPaintTheme {
            text: radix_colors::SLATE_12,
            positive_text: radix_colors::INDIGO_11,
            negative_text: radix_colors::INDIGO_11,
            error_text: radix_colors::AMBER_9,
            agent_text: radix_colors::INDIGO_11,
            agent_fill: radix_colors::INDIGO_3,
            range_fill: radix_colors::INDIGO_4,
            flash_fill: radix_colors::INDIGO_3,
            selected_cell_stroke: radix_colors::INDIGO_8,
            focus_stroke: radix_colors::INDIGO_11,
            edit_valid_stroke: radix_colors::INDIGO_9,
            edit_invalid_stroke: radix_colors::AMBER_9,
            edit_read_only_stroke: radix_colors::SLATE_9,
            stale_marker: radix_colors::AMBER_9,
            live_badge_fill: radix_colors::GREEN_9,
            stale_badge_fill: radix_colors::AMBER_9,
            badge_fill: radix_colors::SLATE_3,
            grid_line: radix_colors::SLATE_6,
            sparkline_stroke: radix_colors::INDIGO_11,
            delta_positive_fill: radix_colors::INDIGO_8,
            delta_negative_fill: radix_colors::INDIGO_8,
        },
        ThemeMode::Dark => GridPaintTheme {
            text: Color32::from_rgb(0xed, 0xf1, 0xf7),
            positive_text: Color32::from_rgb(0x8f, 0xa8, 0xff),
            negative_text: Color32::from_rgb(0xff, 0xa3, 0x9a),
            error_text: Color32::from_rgb(0xff, 0xca, 0x58),
            agent_text: Color32::from_rgb(0xa8, 0xb8, 0xff),
            agent_fill: Color32::from_rgb(0x1e, 0x2b, 0x50),
            range_fill: Color32::from_rgb(0x25, 0x36, 0x68),
            flash_fill: Color32::from_rgb(0x25, 0x36, 0x68),
            selected_cell_stroke: Color32::from_rgb(0x7a, 0x99, 0xff),
            focus_stroke: Color32::from_rgb(0xa8, 0xb8, 0xff),
            edit_valid_stroke: Color32::from_rgb(0x7a, 0x99, 0xff),
            edit_invalid_stroke: Color32::from_rgb(0xff, 0xca, 0x58),
            edit_read_only_stroke: Color32::from_rgb(0x8f, 0x98, 0xa6),
            stale_marker: Color32::from_rgb(0xff, 0xca, 0x58),
            live_badge_fill: Color32::from_rgb(0x30, 0xa4, 0x6c),
            stale_badge_fill: Color32::from_rgb(0xc7, 0x8d, 0x2c),
            badge_fill: Color32::from_rgb(0x2b, 0x31, 0x3a),
            grid_line: Color32::from_rgb(0x2b, 0x31, 0x3a),
            sparkline_stroke: Color32::from_rgb(0x8f, 0xa8, 0xff),
            delta_positive_fill: Color32::from_rgb(0x7a, 0x99, 0xff),
            delta_negative_fill: Color32::from_rgb(0xff, 0xa3, 0x9a),
        },
    }
}

fn provenance_hover_text(provenance: &GridCellProvenance) -> String {
    format!(
        "source={} endpoint={} tr={} evidence={}",
        provenance.source_kind,
        provenance.endpoint.as_deref().unwrap_or("-"),
        provenance.tr_code.as_deref().unwrap_or("-"),
        provenance.evidence_ref.as_deref().unwrap_or("-")
    )
}

fn is_annotated(
    agent_bridge: Option<&GridAgentBridge>,
    row_id: &GridRowId,
    cell_ref: &GridCellRef,
) -> bool {
    agent_bridge
        .map(|bridge| {
            bridge.annotations.iter().any(|annotation| {
                annotation.rows.contains(row_id) || annotation.cells.contains(cell_ref)
            })
        })
        .unwrap_or(false)
}
