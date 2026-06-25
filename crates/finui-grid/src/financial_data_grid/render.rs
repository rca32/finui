use eframe::egui::{self, Align2, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use super::action::{GridAction, GridOutput};
use super::core::{build_row_model, build_row_model_cache_key};
use super::export::{GridExportOptions, export_selected_row};
use super::interaction::handle_grid_keyboard;
use super::menu::show_grid_status_bar;
use super::paint::paint_row_cells;
use super::state::{GridFilter, GridPinSide, GridSort, GridSortDirection};
use super::viewport::{build_column_layout, center_total_width, column_width};
use crate::FinancialDataGrid;
use finui_primitives::{
    ContextMenuItemOptions, ContextMenuOptions, RadixIcon, ThemeMode, paint_radix_icon,
    primitive_context_menu_item, primitive_scroll_thumb_rect, radix_colors, show_context_menu,
};

impl<'a> FinancialDataGrid<'a> {
    pub fn show(self, ui: &mut egui::Ui) -> GridOutput {
        self.state.normalize_columns(self.columns);
        let row_model_cache_key = build_row_model_cache_key(self.source, self.state);
        let row_model = build_row_model(self.source, self.state, self.columns);
        let surface_theme = grid_surface_theme_for_ui(ui);
        debug_assert_eq!(row_model_cache_key.row_count, self.source.row_count());
        let mut actions = Vec::new();
        if self.status_bar {
            show_grid_status_bar(ui, self.state);
        }
        let row_height = self.density.row_height();
        let header_height = row_height + 4.0;
        let available = ui.available_size();
        let viewport_rect = ui.input(|input| input.content_rect());
        let viewport_remaining = (viewport_rect.bottom() - ui.cursor().top()).max(0.0);
        let height = self
            .height
            .unwrap_or_else(|| (available.y.min(viewport_remaining) - 180.0).clamp(140.0, 300.0))
            .clamp(80.0, viewport_remaining.max(80.0));
        let viewport_width = (viewport_rect.right() - ui.cursor().left()).max(0.0);
        let width = available.x.min(viewport_width).max(240.0);
        let desired = Vec2::new(width, height);
        let (rect, response) = ui.allocate_exact_size(desired, Sense::click());
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 6.0, surface_theme.surface_fill);
        painter.rect_stroke(
            rect,
            6.0,
            Stroke::new(1.0, surface_theme.surface_stroke),
            StrokeKind::Inside,
        );

        let header_rect = Rect::from_min_size(rect.min, Vec2::new(rect.width(), header_height));
        painter.rect_filled(header_rect, 6.0, surface_theme.header_fill);
        let visible_columns = self.state.visible_column_ids();
        let (pinned_left, pinned_right) = self.state.pinned_visible_columns();
        let pinned_left_width: f32 = pinned_left
            .iter()
            .filter_map(|id| {
                self.columns
                    .iter()
                    .find(|column| column.id == *id)
                    .map(|column| {
                        self.state
                            .column_widths
                            .get(id)
                            .copied()
                            .unwrap_or_else(|| column.width.initial())
                    })
            })
            .sum();
        let pinned_right_width: f32 = pinned_right
            .iter()
            .filter_map(|id| {
                self.columns
                    .iter()
                    .find(|column| column.id == *id)
                    .map(|column| {
                        self.state
                            .column_widths
                            .get(id)
                            .copied()
                            .unwrap_or_else(|| column.width.initial())
                    })
            })
            .sum();
        let body_outer_rect = Rect::from_min_max(
            Pos2::new(rect.left(), header_rect.bottom()),
            Pos2::new(rect.right(), rect.bottom()),
        );
        let scrollbar_size = GRID_SCROLLBAR_SIZE;
        let preliminary_visible_count =
            ((body_outer_rect.height() / row_height).floor() as usize).max(1);
        let has_vertical_scroll = row_model.len() > preliminary_visible_count;
        let content_right = body_outer_rect.right()
            - if has_vertical_scroll {
                scrollbar_size
            } else {
                0.0
            };
        let preliminary_body_width = (content_right - body_outer_rect.left()).max(0.0);
        let max_scroll = (center_total_width(
            &visible_columns,
            &pinned_left,
            &pinned_right,
            self.columns,
            self.state,
        ) - (preliminary_body_width - pinned_left_width - pinned_right_width)
            .max(0.0))
        .max(0.0);
        let has_horizontal_scroll = max_scroll > 0.0;
        let body_rect = Rect::from_min_max(
            body_outer_rect.left_top(),
            Pos2::new(
                content_right,
                body_outer_rect.bottom()
                    - if has_horizontal_scroll {
                        scrollbar_size
                    } else {
                        0.0
                    },
            ),
        );
        let column_layouts = build_column_layout(
            body_rect.left(),
            body_rect.right(),
            &visible_columns,
            &pinned_left,
            &pinned_right,
            self.columns,
            self.state,
            self.state.horizontal_scroll,
        );
        let mut hovered_cell = None;
        for layout in &column_layouts {
            let Some(column) = self
                .columns
                .iter()
                .find(|column| column.id == layout.column_id)
            else {
                continue;
            };
            let width = layout.width;
            let actual_width = column_width(self.columns, self.state, &column.id);
            if layout.x > rect.right() {
                break;
            }
            let column_rect = Rect::from_min_size(
                Pos2::new(layout.x, rect.top()),
                Vec2::new(width, header_height),
            );
            let header_response = ui
                .interact(
                    column_rect,
                    ui.make_persistent_id((self.id, "header", &column.id.0)),
                    Sense::click_and_drag(),
                )
                .on_hover_cursor(egui::CursorIcon::Grab);
            let sort_mark = self
                .state
                .sort
                .iter()
                .find(|sort| sort.column_id == column.id)
                .map(|sort| sort.direction);
            let header_icon = header_icon_for_column(&column.id, &pinned_left, &pinned_right);
            if let Some(icon) = header_icon {
                paint_radix_icon(
                    ui,
                    icon,
                    Rect::from_min_size(
                        column_rect.left_center() + Vec2::new(7.0, -6.0),
                        Vec2::splat(12.0),
                    ),
                    surface_theme.header_text,
                );
            }
            let text_offset = if header_icon.is_some() { 24.0 } else { 8.0 };
            painter.text(
                column_rect.left_center() + Vec2::new(text_offset, 0.0),
                Align2::LEFT_CENTER,
                column.label.as_str(),
                finui_primitives::scaled_proportional_font(ui, 12.0),
                surface_theme.header_text,
            );
            let mut sort_response = None;
            if column.sortable {
                let sort_rect = Rect::from_min_size(
                    column_rect.right_center() + Vec2::new(-22.0, -6.0),
                    Vec2::splat(12.0),
                );
                let response = ui
                    .interact(
                        sort_rect.expand(4.0),
                        ui.make_persistent_id((self.id, "sort-icon", &column.id.0)),
                        Sense::click(),
                    )
                    .on_hover_cursor(egui::CursorIcon::PointingHand);
                let sort_icon = match sort_mark {
                    Some(GridSortDirection::Asc) => RadixIcon::CaretUp,
                    Some(GridSortDirection::Desc) => RadixIcon::CaretDown,
                    None => RadixIcon::CaretSort,
                };
                paint_radix_icon(
                    ui,
                    sort_icon,
                    sort_rect,
                    if sort_mark.is_some() {
                        surface_theme.accent_text
                    } else {
                        surface_theme.muted_text
                    },
                );
                sort_response = Some(response);
            }
            let resize_rect = Rect::from_min_max(
                Pos2::new(column_rect.right() - 6.0, column_rect.top()),
                Pos2::new(column_rect.right() + 6.0, column_rect.bottom()),
            );
            let resize_id = ui.make_persistent_id((self.id, "resize", &column.id.0));
            let resize_initial_width_id =
                ui.make_persistent_id((self.id, "resize-initial-width", &column.id.0));
            let header_reorder_done_id =
                ui.make_persistent_id((self.id, "header-reorder-done", &column.id.0));
            let resize_response = ui
                .interact(resize_rect, resize_id, Sense::click_and_drag())
                .on_hover_cursor(egui::CursorIcon::ResizeHorizontal);
            if header_response.drag_started() {
                ui.memory_mut(|memory| {
                    memory.data.remove::<bool>(header_reorder_done_id);
                });
            }
            if resize_response.drag_started() {
                ui.memory_mut(|memory| {
                    memory
                        .data
                        .insert_persisted(resize_initial_width_id, actual_width);
                });
            }
            if resize_response.dragged() {
                ui.output_mut(|output| output.cursor_icon = egui::CursorIcon::ResizeHorizontal);
                let initial_width = ui.memory_mut(|memory| {
                    memory
                        .data
                        .get_persisted::<f32>(resize_initial_width_id)
                        .unwrap_or(actual_width)
                });
                let drag_delta = resize_response
                    .total_drag_delta()
                    .map(|delta| delta.x)
                    .unwrap_or(0.0);
                let next = (initial_width + drag_delta).clamp(52.0, 320.0);
                self.state.column_widths.insert(column.id.clone(), next);
                actions.push(GridAction::ColumnResized {
                    column_id: column.id.clone(),
                    width: next.round() as u32,
                });
            }
            if resize_response.drag_stopped() {
                ui.memory_mut(|memory| {
                    memory.data.remove::<f32>(resize_initial_width_id);
                });
            }
            if header_response.dragged() && !resize_response.dragged() {
                ui.output_mut(|output| output.cursor_icon = egui::CursorIcon::Grabbing);
            }
            let should_try_reorder = !resize_response.dragged()
                && (header_response.dragged() || header_response.drag_stopped());
            if should_try_reorder {
                let already_reordered = ui.memory_mut(|memory| {
                    memory
                        .data
                        .get_persisted::<bool>(header_reorder_done_id)
                        .unwrap_or(false)
                });
                let drag_delta = header_response
                    .total_drag_delta()
                    .map(|delta| delta.x)
                    .unwrap_or(0.0);
                if !already_reordered
                    && drag_delta.abs() > (actual_width * 0.35).max(28.0)
                    && let Some(current_index) = self
                        .state
                        .column_order
                        .iter()
                        .position(|candidate| *candidate == column.id)
                {
                    let target_index = if drag_delta.is_sign_positive() {
                        current_index.saturating_add(1)
                    } else {
                        current_index.saturating_sub(1)
                    }
                    .min(self.state.column_order.len().saturating_sub(1));
                    if target_index != current_index
                        && self.state.reorder_column(&column.id, target_index)
                    {
                        ui.memory_mut(|memory| {
                            memory.data.insert_persisted(header_reorder_done_id, true);
                        });
                        actions.push(GridAction::ColumnReordered {
                            column_id: column.id.clone(),
                            target_index,
                        });
                    }
                }
            }
            if header_response.drag_stopped() {
                ui.memory_mut(|memory| {
                    memory.data.remove::<bool>(header_reorder_done_id);
                });
            }
            let sort_clicked = sort_response
                .as_ref()
                .is_some_and(|response| response.clicked());
            if (sort_clicked || header_response.clicked())
                && !resize_response.clicked()
                && !resize_response.drag_started()
                && !header_response.dragged()
                && column.sortable
            {
                let direction = self
                    .state
                    .sort
                    .iter()
                    .find(|sort| sort.column_id == column.id)
                    .map(|sort| sort.direction.next())
                    .unwrap_or(GridSortDirection::Asc);
                self.state.sort = vec![GridSort {
                    column_id: column.id.clone(),
                    direction,
                }];
                actions.push(GridAction::SortChanged(self.state.sort.clone()));
            }
            if header_response.secondary_clicked() {
                let menu_position = header_response
                    .interact_pointer_pos()
                    .unwrap_or_else(|| column_rect.left_bottom());
                let menu_state_id = ui.make_persistent_id((self.id, "header-context-menu-state"));
                ui.memory_mut(|memory| {
                    memory.data.insert_persisted(
                        menu_state_id,
                        Some((column.id.0.clone(), menu_position)),
                    );
                });
            }
            let menu_state_id = ui.make_persistent_id((self.id, "header-context-menu-state"));
            let header_menu_state = ui.memory_mut(|memory| {
                memory
                    .data
                    .get_persisted_mut_or_default::<Option<(String, Pos2)>>(menu_state_id)
                    .clone()
            });
            if let Some((_menu_column_id, menu_position)) =
                header_menu_state.filter(|state| state.0 == column.id.0)
            {
                let output = show_context_menu(
                    ui.ctx(),
                    ContextMenuOptions::at(
                        (self.id, "header-context-menu", &column.id.0),
                        menu_position,
                        190.0,
                    )
                    .max_height(208.0),
                    |ui| {
                        if primitive_context_menu_item(
                            ui,
                            "Pin left",
                            ContextMenuItemOptions::new(170.0),
                        )
                        .clicked()
                        {
                            return Some("pin-left");
                        }
                        if primitive_context_menu_item(
                            ui,
                            "Hide column",
                            ContextMenuItemOptions::new(170.0),
                        )
                        .clicked()
                        {
                            return Some("hide");
                        }
                        if primitive_context_menu_item(
                            ui,
                            "Filter sample",
                            ContextMenuItemOptions::new(170.0),
                        )
                        .clicked()
                        {
                            return Some("filter");
                        }
                        if primitive_context_menu_item(
                            ui,
                            "Clear filters",
                            ContextMenuItemOptions::new(170.0),
                        )
                        .clicked()
                        {
                            return Some("clear-filters");
                        }
                        None
                    },
                );
                match output.action {
                    Some("pin-left") => {
                        self.state.pinned_left.retain(|id| id != &column.id);
                        self.state.pinned_right.retain(|id| id != &column.id);
                        self.state.pinned_left.push(column.id.clone());
                        actions.push(GridAction::ColumnPinned {
                            column_id: column.id.clone(),
                            side: GridPinSide::Left,
                        });
                        ui.memory_mut(|memory| {
                            memory.data.remove::<Option<(String, Pos2)>>(menu_state_id)
                        });
                    }
                    Some("hide") => {
                        self.state
                            .column_visibility
                            .insert(column.id.clone(), false);
                        actions.push(GridAction::ColumnVisibilityChanged {
                            column_id: column.id.clone(),
                            visible: false,
                        });
                        ui.memory_mut(|memory| {
                            memory.data.remove::<Option<(String, Pos2)>>(menu_state_id)
                        });
                    }
                    Some("filter") => {
                        let query = sample_filter_query(&column.id.0);
                        self.state
                            .filters
                            .retain(|filter| filter.column_id != column.id);
                        self.state.filters.push(GridFilter {
                            column_id: column.id.clone(),
                            query,
                        });
                        actions.push(GridAction::FilterChanged(self.state.filters.clone()));
                        ui.memory_mut(|memory| {
                            memory.data.remove::<Option<(String, Pos2)>>(menu_state_id)
                        });
                    }
                    Some("clear-filters") => {
                        self.state.filters.clear();
                        actions.push(GridAction::FilterChanged(self.state.filters.clone()));
                        ui.memory_mut(|memory| {
                            memory.data.remove::<Option<(String, Pos2)>>(menu_state_id)
                        });
                    }
                    _ => {}
                }
                if output.should_close {
                    ui.memory_mut(|memory| {
                        memory.data.remove::<Option<(String, Pos2)>>(menu_state_id)
                    });
                }
            }
            let resize_stroke = if resize_response.hovered() || resize_response.dragged() {
                paint_radix_icon(
                    ui,
                    RadixIcon::DragHandleVertical,
                    Rect::from_min_size(
                        resize_rect.center() + Vec2::new(-6.0, -6.0),
                        Vec2::splat(12.0),
                    ),
                    surface_theme.accent_text,
                );
                Stroke::new(1.5, surface_theme.accent_text)
            } else {
                Stroke::new(1.0, surface_theme.surface_stroke)
            };
            painter.line_segment(
                [
                    Pos2::new(column_rect.right(), column_rect.top() + 4.0),
                    Pos2::new(column_rect.right(), column_rect.bottom() - 4.0),
                ],
                resize_stroke,
            );
        }

        painter.rect_filled(body_rect, 0.0, surface_theme.body_fill);
        let body_painter = painter.with_clip_rect(body_rect);
        let visible_count = ((body_rect.height() / row_height).floor() as usize).max(1);
        let max_first_row = row_model.len().saturating_sub(visible_count);
        self.state.row_scroll = self.state.row_scroll.min(max_first_row);
        let body_response = ui.interact(
            body_rect,
            ui.make_persistent_id((self.id, "body-scroll")),
            Sense::hover(),
        );
        let pointer_in_grid = ui.input(|input| {
            input.pointer.hover_pos().is_some_and(|position| {
                body_outer_rect.contains(position) || rect.contains(position)
            })
        });
        if pointer_in_grid || response.hovered() || body_response.hovered() {
            let scroll = ui.input(financial_grid_scroll_delta);
            if scroll.y.abs() > f32::EPSILON {
                let row_delta = (-scroll.y / row_height).round().clamp(-12.0, 12.0) as isize;
                if row_delta != 0 {
                    self.state.row_scroll =
                        scroll_row_index(self.state.row_scroll, row_delta, max_first_row);
                }
            }
            if scroll.x.abs() > f32::EPSILON && max_scroll > 0.0 {
                self.state.horizontal_scroll =
                    (self.state.horizontal_scroll + scroll.x).clamp(0.0, max_scroll);
            }
        }
        let visible_rows =
            self.state.row_scroll..row_model.len().min(self.state.row_scroll + visible_count);
        for (screen_index, model_index) in row_model
            .iter()
            .take(visible_rows.end)
            .skip(visible_rows.start)
            .enumerate()
        {
            let row_id = self.source.row_id(*model_index);
            let row_top = body_rect.top() + screen_index as f32 * row_height;
            let row_rect = Rect::from_min_size(
                Pos2::new(body_rect.left(), row_top),
                Vec2::new(body_rect.width(), row_height),
            );
            if self.state.selection.selected_row.as_ref() == Some(&row_id) {
                body_painter.rect_filled(row_rect, 0.0, surface_theme.selected_row_fill);
            } else if screen_index % 2 == 1 {
                body_painter.rect_filled(row_rect, 0.0, surface_theme.row_alt_fill);
            }
            let row_response = ui.interact(
                row_rect,
                ui.make_persistent_id((self.id, "row", *model_index, screen_index, &row_id.0)),
                Sense::click(),
            );
            if row_response.clicked() {
                row_response.request_focus();
                response.request_focus();
                self.state.selection.selected_row = Some(row_id.clone());
                actions.push(GridAction::RowSelected(row_id.clone()));
            }
            if row_response.secondary_clicked() {
                let menu_position = row_response
                    .interact_pointer_pos()
                    .unwrap_or_else(|| row_rect.left_top());
                let menu_state_id = ui.make_persistent_id((self.id, "row-context-menu-state"));
                ui.memory_mut(|memory| {
                    memory
                        .data
                        .insert_persisted(menu_state_id, Some((row_id.0.clone(), menu_position)));
                });
            }
            let menu_state_id = ui.make_persistent_id((self.id, "row-context-menu-state"));
            let row_menu_state = ui.memory_mut(|memory| {
                memory
                    .data
                    .get_persisted_mut_or_default::<Option<(String, Pos2)>>(menu_state_id)
                    .clone()
            });
            if let Some((_menu_row_id, menu_position)) =
                row_menu_state.filter(|state| state.0 == row_id.0)
            {
                let output = show_context_menu(
                    ui.ctx(),
                    ContextMenuOptions::at(
                        (self.id, "row-context-menu", &row_id.0),
                        menu_position,
                        176.0,
                    )
                    .max_height(160.0),
                    |ui| {
                        if primitive_context_menu_item(
                            ui,
                            "Open row detail",
                            ContextMenuItemOptions::new(156.0),
                        )
                        .clicked()
                        {
                            return Some("detail");
                        }
                        if primitive_context_menu_item(
                            ui,
                            "Copy row",
                            ContextMenuItemOptions::new(156.0),
                        )
                        .clicked()
                        {
                            return Some("copy");
                        }
                        None
                    },
                );
                match output.action {
                    Some("detail") => {
                        self.state.selection.selected_row = Some(row_id.clone());
                        actions.push(GridAction::RowDetailOpened(row_id.clone()));
                        ui.memory_mut(|memory| {
                            memory.data.remove::<Option<(String, Pos2)>>(menu_state_id)
                        });
                    }
                    Some("copy") => {
                        self.state.selection.selected_row = Some(row_id.clone());
                        if let Some(text) = export_selected_row(
                            self.source,
                            self.columns,
                            self.state,
                            &row_model,
                            &GridExportOptions::default(),
                        ) {
                            ui.copy_text(text);
                        }
                        actions.push(GridAction::CopiedSelection {
                            format: "row-tsv".to_owned(),
                        });
                        ui.memory_mut(|memory| {
                            memory.data.remove::<Option<(String, Pos2)>>(menu_state_id)
                        });
                    }
                    _ => {}
                }
                if output.should_close {
                    ui.memory_mut(|memory| {
                        memory.data.remove::<Option<(String, Pos2)>>(menu_state_id)
                    });
                }
            }
            paint_row_cells(
                ui,
                &body_painter,
                self.id,
                body_rect,
                row_rect,
                *model_index,
                &row_id,
                &column_layouts,
                self.columns,
                self.source,
                self.state,
                self.agent_bridge,
                self.provenance_policy,
                self.row_selection_only,
                theme_mode_for_ui(ui),
                &mut actions,
                &mut hovered_cell,
            );
        }
        if row_model.is_empty() {
            paint_empty_grid_state(ui, &body_painter, body_rect, self.state, surface_theme);
        }
        if response.has_focus() {
            handle_grid_keyboard(
                ui,
                self.source,
                self.state,
                &row_model,
                &visible_columns,
                visible_count,
                &mut actions,
            );
        }
        if ui.input(|input| input.modifiers.command && input.key_pressed(egui::Key::C))
            && (self.state.selection.selected_cell.is_some()
                || self.state.selection.selected_row.is_some())
        {
            if let Some(text) = export_selected_row(
                self.source,
                self.columns,
                self.state,
                &row_model,
                &GridExportOptions::default(),
            ) {
                ui.copy_text(text);
            }
            actions.push(GridAction::CopiedSelection {
                format: "tsv".to_owned(),
            });
        }
        paint_grid_scrollbars(
            ui,
            self.id,
            body_outer_rect,
            body_rect,
            row_model.len(),
            visible_count,
            max_first_row,
            self.state,
            max_scroll,
            has_vertical_scroll,
            has_horizontal_scroll,
            pointer_in_grid || response.hovered() || body_response.hovered(),
            surface_theme,
        );
        GridOutput {
            actions,
            hovered_cell,
            focused_cell: self.state.focused_cell.clone(),
            visible_rows,
            visible_columns: column_layouts
                .iter()
                .map(|layout| layout.column_id.clone())
                .collect(),
        }
    }
}

fn header_icon_for_column(
    column_id: &super::ids::GridColumnId,
    pinned_left: &[super::ids::GridColumnId],
    pinned_right: &[super::ids::GridColumnId],
) -> Option<RadixIcon> {
    if pinned_left.contains(column_id) {
        return Some(RadixIcon::PinLeft);
    }
    if pinned_right.contains(column_id) {
        return Some(RadixIcon::PinRight);
    }
    match column_id.0.as_str() {
        "source" => Some(RadixIcon::Database),
        "status" => Some(RadixIcon::ActivityLog),
        _ => None,
    }
}

fn sample_filter_query(column_id: &str) -> String {
    match column_id {
        "price" => ">100000".to_owned(),
        "change_pct" => "-1..2".to_owned(),
        "status" => "live".to_owned(),
        "name" => "Acme".to_owned(),
        "source" => "api".to_owned(),
        _ => String::new(),
    }
}

fn financial_grid_scroll_delta(input: &egui::InputState) -> Vec2 {
    if input.smooth_scroll_delta.length_sq() > f32::EPSILON {
        return input.smooth_scroll_delta;
    }
    const LINE_SCROLL_POINTS: f32 = 50.0;
    input
        .events
        .iter()
        .filter_map(|event| match event {
            egui::Event::MouseWheel { unit, delta, .. } => Some(match unit {
                egui::MouseWheelUnit::Point => *delta,
                egui::MouseWheelUnit::Line => *delta * LINE_SCROLL_POINTS,
                egui::MouseWheelUnit::Page => Vec2::new(
                    delta.x * input.viewport_rect().height(),
                    delta.y * input.viewport_rect().height(),
                ),
            }),
            _ => None,
        })
        .fold(Vec2::ZERO, |total, delta| total + delta)
}

fn scroll_row_index(current: usize, delta: isize, max_first_row: usize) -> usize {
    if delta.is_negative() {
        current
            .saturating_sub(delta.unsigned_abs())
            .min(max_first_row)
    } else {
        current.saturating_add(delta as usize).min(max_first_row)
    }
}

fn paint_grid_scrollbars(
    ui: &mut egui::Ui,
    grid_id: &str,
    _body_outer_rect: Rect,
    body_rect: Rect,
    row_count: usize,
    visible_count: usize,
    max_first_row: usize,
    state: &mut super::state::GridState,
    max_horizontal_scroll: f32,
    has_vertical_scroll: bool,
    has_horizontal_scroll: bool,
    hovered: bool,
    theme: GridSurfaceTheme,
) {
    let viewport_rect = ui.input(|input| input.content_rect());
    let visible_body_rect = body_rect.intersect(viewport_rect);
    if has_vertical_scroll {
        let track = Rect::from_min_max(
            Pos2::new(
                visible_body_rect.right() - GRID_SCROLLBAR_SIZE,
                visible_body_rect.top(),
            ),
            visible_body_rect.right_bottom(),
        );
        let fraction = if max_first_row == 0 {
            0.0
        } else {
            state.row_scroll as f32 / max_first_row as f32
        };
        let thumb = primitive_scroll_thumb_rect(
            track,
            row_count as f32,
            visible_count.max(1) as f32,
            fraction,
        );
        let response = ui
            .interact(
                track,
                ui.make_persistent_id((grid_id, "financial_grid_vertical_scrollbar")),
                Sense::click_and_drag(),
            )
            .on_hover_cursor(egui::CursorIcon::Grab);
        if response.dragged() {
            ui.output_mut(|output| output.cursor_icon = egui::CursorIcon::Grabbing);
        }
        let visible = hovered || response.hovered() || response.dragged();
        if visible {
            paint_grid_scrollbar_track(ui, track, theme);
        }
        if (response.clicked() || response.dragged())
            && let Some(pointer) = response.interact_pointer_pos()
        {
            let travel = (track.height() - thumb.height()).max(1.0);
            let next_fraction =
                ((pointer.y - track.top() - thumb.height() * 0.5) / travel).clamp(0.0, 1.0);
            state.row_scroll = (next_fraction * max_first_row as f32).round() as usize;
        }
        if visible {
            paint_grid_scrollbar_thumb(ui, thumb, theme);
        }
    }

    if has_horizontal_scroll {
        let track = Rect::from_min_max(
            Pos2::new(
                visible_body_rect.left(),
                visible_body_rect.bottom() - GRID_SCROLLBAR_SIZE,
            ),
            visible_body_rect.right_bottom(),
        );
        let fraction = if max_horizontal_scroll <= 0.0 {
            0.0
        } else {
            state.horizontal_scroll / max_horizontal_scroll
        };
        let thumb_width =
            horizontal_scroll_thumb_width(body_rect.width(), max_horizontal_scroll, track.width());
        let travel = (track.width() - thumb_width).max(0.0);
        let thumb = Rect::from_min_size(
            Pos2::new(
                track.left() + travel * fraction.clamp(0.0, 1.0),
                track.top(),
            ),
            Vec2::new(thumb_width, track.height()),
        );
        let response = ui
            .interact(
                track,
                ui.make_persistent_id((grid_id, "financial_grid_horizontal_scrollbar")),
                Sense::click_and_drag(),
            )
            .on_hover_cursor(egui::CursorIcon::Grab);
        if response.dragged() {
            ui.output_mut(|output| output.cursor_icon = egui::CursorIcon::Grabbing);
        }
        let visible = hovered || response.hovered() || response.dragged();
        if visible {
            paint_grid_scrollbar_track(ui, track, theme);
        }
        if (response.clicked() || response.dragged())
            && let Some(pointer) = response.interact_pointer_pos()
        {
            let travel = (track.width() - thumb.width()).max(1.0);
            let next_fraction =
                ((pointer.x - track.left() - thumb.width() * 0.5) / travel).clamp(0.0, 1.0);
            state.horizontal_scroll = next_fraction * max_horizontal_scroll;
        }
        if visible {
            paint_grid_scrollbar_thumb(ui, thumb, theme);
        }
    }
}

fn paint_grid_scrollbar_track(ui: &egui::Ui, rect: Rect, theme: GridSurfaceTheme) {
    ui.painter().rect_filled(rect, 2.0, theme.scrollbar_track);
    ui.painter().rect_stroke(
        rect,
        2.0,
        Stroke::new(1.0, theme.scrollbar_stroke),
        StrokeKind::Inside,
    );
}

fn paint_grid_scrollbar_thumb(ui: &egui::Ui, rect: Rect, theme: GridSurfaceTheme) {
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    ui.painter()
        .rect_filled(rect.shrink(1.5), 3.0, theme.scrollbar_thumb);
}

const GRID_SCROLLBAR_SIZE: f32 = 8.0;

#[derive(Debug, Clone, Copy)]
struct GridSurfaceTheme {
    surface_fill: Color32,
    surface_stroke: Color32,
    header_fill: Color32,
    body_fill: Color32,
    header_text: Color32,
    muted_text: Color32,
    accent_text: Color32,
    selected_row_fill: Color32,
    row_alt_fill: Color32,
    scrollbar_track: Color32,
    scrollbar_stroke: Color32,
    scrollbar_thumb: Color32,
}

fn horizontal_scroll_thumb_width(
    body_width: f32,
    max_horizontal_scroll: f32,
    track_width: f32,
) -> f32 {
    let track_width = track_width.max(0.0);
    if track_width <= 0.0 {
        return 0.0;
    }
    let denominator = (body_width + max_horizontal_scroll).max(1.0);
    let raw = body_width.max(0.0) / denominator * track_width;
    raw.clamp(24.0_f32.min(track_width), track_width)
}

fn grid_surface_theme_for_ui(ui: &egui::Ui) -> GridSurfaceTheme {
    match theme_mode_for_ui(ui) {
        ThemeMode::Light => GridSurfaceTheme {
            surface_fill: radix_colors::SLATE_2,
            surface_stroke: radix_colors::SLATE_7,
            header_fill: radix_colors::SLATE_3,
            body_fill: radix_colors::SLATE_1,
            header_text: radix_colors::SLATE_11,
            muted_text: radix_colors::SLATE_10,
            accent_text: radix_colors::INDIGO_11,
            selected_row_fill: radix_colors::INDIGO_3,
            row_alt_fill: radix_colors::SLATE_1,
            scrollbar_track: alpha_color(radix_colors::SLATE_4, 120),
            scrollbar_stroke: alpha_color(radix_colors::SLATE_8, 150),
            scrollbar_thumb: alpha_color(radix_colors::INDIGO_9, 185),
        },
        ThemeMode::Dark => GridSurfaceTheme {
            surface_fill: Color32::from_rgb(0x14, 0x18, 0x20),
            surface_stroke: Color32::from_rgb(0x37, 0x40, 0x4d),
            header_fill: Color32::from_rgb(0x1b, 0x21, 0x2b),
            body_fill: Color32::from_rgb(0x10, 0x14, 0x1a),
            header_text: Color32::from_rgb(0xc5, 0xcd, 0xd8),
            muted_text: Color32::from_rgb(0x8f, 0x98, 0xa6),
            accent_text: Color32::from_rgb(0x8f, 0xa8, 0xff),
            selected_row_fill: Color32::from_rgb(0x1f, 0x2d, 0x56),
            row_alt_fill: Color32::from_rgb(0x13, 0x18, 0x20),
            scrollbar_track: Color32::from_rgba_unmultiplied(0x36, 0x3d, 0x49, 120),
            scrollbar_stroke: Color32::from_rgba_unmultiplied(0x72, 0x7b, 0x8c, 150),
            scrollbar_thumb: Color32::from_rgba_unmultiplied(0x8f, 0xa8, 0xff, 185),
        },
    }
}

fn theme_mode_for_ui(ui: &egui::Ui) -> ThemeMode {
    if ui.visuals().dark_mode {
        ThemeMode::Dark
    } else {
        ThemeMode::Light
    }
}

fn paint_empty_grid_state(
    ui: &egui::Ui,
    painter: &egui::Painter,
    rect: Rect,
    state: &super::state::GridState,
    theme: GridSurfaceTheme,
) {
    let title = if state.filters.is_empty() {
        "No rows"
    } else {
        "No rows match active filters"
    };
    let detail = if state.filters.is_empty() {
        "The current source returned an empty page."
    } else {
        "Clear filters or revise the numeric/text predicate."
    };
    let center = rect.center();
    painter.text(
        center - Vec2::new(0.0, 10.0),
        Align2::CENTER_CENTER,
        title,
        finui_primitives::scaled_proportional_font(ui, 13.0),
        theme.header_text,
    );
    painter.text(
        center + Vec2::new(0.0, 12.0),
        Align2::CENTER_CENTER,
        detail,
        finui_primitives::scaled_proportional_font(ui, 12.0),
        theme.muted_text,
    );
    if !state.filters.is_empty() {
        ui.output_mut(|output| output.cursor_icon = egui::CursorIcon::Default);
    }
}

fn alpha_color(color: Color32, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_scroll_thumb_width_handles_tiny_tracks() {
        let width = horizontal_scroll_thumb_width(420.0, 1800.0, 8.75);
        assert_eq!(width, 8.75);
    }

    #[test]
    fn horizontal_scroll_thumb_width_keeps_minimum_on_normal_tracks() {
        let width = horizontal_scroll_thumb_width(420.0, 1800.0, 180.0);
        assert!(width >= 24.0);
        assert!(width <= 180.0);
    }
}
