use egui::{
    Align, Align2, CursorIcon, FontId, Layout, Rect, Sense, StrokeKind, Ui, UiBuilder, pos2, vec2,
};

use crate::{
    CommandScopeOutput, FocusRoute, LayoutValidationError, PanelId, SplitAxis, WorkbenchAction,
    WorkbenchGeometry, WorkbenchState, calculate_workbench_geometry,
};

#[derive(Clone, Debug)]
pub struct WorkbenchOutput {
    pub actions: Vec<WorkbenchAction>,
    pub focus_route: Option<FocusRoute>,
    pub command_scope: CommandScopeOutput,
    pub geometry: WorkbenchGeometry,
}

pub fn show_workbench(
    ui: &mut Ui,
    state: &WorkbenchState,
    mut add_panel: impl FnMut(&mut Ui, &PanelId),
) -> Result<WorkbenchOutput, LayoutValidationError> {
    let root_rect = ui.available_rect_before_wrap();
    let geometry = calculate_workbench_geometry(state, root_rect, ui.ctx().pixels_per_point())?;
    ui.allocate_rect(root_rect, Sense::hover());

    let painter = ui.painter().clone();
    let visuals = ui.visuals().clone();
    let mut actions = Vec::new();

    for region in &geometry.tab_regions {
        painter.rect_filled(region.rect, 0.0, visuals.extreme_bg_color);
        painter.rect_filled(region.tab_bar_rect, 0.0, visuals.faint_bg_color);
        painter.rect_stroke(
            region.rect,
            0.0,
            visuals.widgets.noninteractive.bg_stroke,
            StrokeKind::Inside,
        );

        let tab_width = tab_width(region.tab_bar_rect, region.tabs.len());
        for (index, tab) in region.tabs.iter().enumerate() {
            let tab_rect = Rect::from_min_size(
                pos2(
                    region.tab_bar_rect.min.x + tab_width * index as f32,
                    region.tab_bar_rect.min.y,
                ),
                vec2(tab_width, region.tab_bar_rect.height()),
            );
            let active = tab.id == region.active_panel;
            if active {
                painter.rect_filled(tab_rect, 0.0, visuals.selection.bg_fill);
            }
            painter.text(
                tab_rect.center(),
                Align2::CENTER_CENTER,
                &tab.title,
                FontId::proportional(12.0),
                if active {
                    visuals.selection.stroke.color
                } else {
                    visuals.text_color()
                },
            );

            let response = ui.interact(
                tab_rect,
                ui.make_persistent_id((
                    "finui-workbench-tab",
                    region.region_id.as_str(),
                    tab.id.as_str(),
                )),
                Sense::click(),
            );
            if response.clicked() {
                if !active {
                    actions.push(WorkbenchAction::ActivateTab {
                        region_id: region.region_id.clone(),
                        panel_id: tab.id.clone(),
                    });
                }
                actions.push(WorkbenchAction::FocusPanel {
                    panel_id: tab.id.clone(),
                });
            }
        }

        let panel_rect = region.content_rect.shrink2(vec2(8.0, 6.0));
        if panel_rect.is_positive() {
            let mut panel_ui = ui.new_child(
                UiBuilder::new()
                    .id_salt((
                        "finui-workbench-panel",
                        region.region_id.as_str(),
                        region.active_panel.as_str(),
                    ))
                    .max_rect(panel_rect)
                    .layout(Layout::top_down(Align::Min))
                    .sense(Sense::click()),
            );
            panel_ui.set_clip_rect(region.content_rect);
            add_panel(&mut panel_ui, &region.active_panel);
            if panel_ui.response().clicked() {
                actions.push(WorkbenchAction::FocusPanel {
                    panel_id: region.active_panel.clone(),
                });
            }
        }
    }

    for divider in &geometry.dividers {
        let response = ui
            .interact(
                divider.hit_rect,
                ui.make_persistent_id(("finui-workbench-divider", divider.split_id.as_str())),
                Sense::drag(),
            )
            .on_hover_cursor(match divider.axis {
                SplitAxis::Horizontal => CursorIcon::ResizeHorizontal,
                SplitAxis::Vertical => CursorIcon::ResizeVertical,
            });
        let divider_color = if response.dragged() || response.hovered() {
            visuals.selection.bg_fill
        } else {
            visuals.widgets.noninteractive.bg_stroke.color
        };
        painter.rect_filled(divider.visual_rect, 0.0, divider_color);

        if response.dragged()
            && let Some(pointer) = response.interact_pointer_pos()
        {
            actions.push(WorkbenchAction::ResizeSplit {
                split_id: divider.split_id.clone(),
                fraction: divider.fraction_for_pointer(pointer),
            });
        }
    }

    // Project outputs through this frame's typed actions without mutating the
    // caller-owned state passed to the widget.
    let mut projected = state.clone();
    projected.apply_all(&actions);

    Ok(WorkbenchOutput {
        actions,
        focus_route: projected.focus_route(),
        command_scope: projected.command_scope(),
        geometry,
    })
}

fn tab_width(tab_bar_rect: Rect, tab_count: usize) -> f32 {
    if tab_count == 0 {
        return 0.0;
    }
    (tab_bar_rect.width() / tab_count as f32).min(144.0)
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::{
        CommandScopeOutput, PaneConstraints, PanelId, PanelTab, RegionId, SplitAxis, WorkbenchNode,
        WorkbenchState,
    };

    use super::show_workbench;

    #[test]
    fn workbench_renders_each_active_tab_and_reports_command_scope() {
        let state = WorkbenchState::new(WorkbenchNode::split(
            "root",
            SplitAxis::Horizontal,
            0.5,
            PaneConstraints::minimum(100.0),
            PaneConstraints::minimum(100.0),
            WorkbenchNode::tabs("left", vec![PanelTab::new("media", "Media")], "media"),
            WorkbenchNode::tabs(
                "right",
                vec![PanelTab::new("preview", "Preview")],
                "preview",
            ),
        ))
        .with_focused_panel("preview");
        let rendered = RefCell::new(Vec::new());

        egui::__run_test_ui(|ui| {
            ui.set_min_size(egui::vec2(800.0, 600.0));
            let output = show_workbench(ui, &state, |_, panel_id| {
                rendered.borrow_mut().push(panel_id.clone());
            })
            .unwrap();
            assert_eq!(
                output.command_scope,
                CommandScopeOutput::Panel {
                    region_id: RegionId::from("right"),
                    panel_id: PanelId::from("preview"),
                }
            );
        });

        assert_eq!(
            rendered.into_inner(),
            vec![PanelId::from("media"), PanelId::from("preview")]
        );
    }
}
