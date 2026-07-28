use egui::{Pos2, Rect, pos2, vec2};

use crate::{
    LayoutValidationError, PaneConstraints, PanelId, PanelTab, RegionId, SplitAxis, SplitId,
    WorkbenchNode, WorkbenchState,
};

pub const DIVIDER_VISUAL_POINTS: f32 = 1.0;
pub const DIVIDER_HIT_POINTS: f32 = 8.0;
const TAB_BAR_POINTS: f32 = 30.0;

#[derive(Clone, Debug, PartialEq)]
pub struct DividerGeometry {
    pub split_id: SplitId,
    pub axis: SplitAxis,
    pub parent_rect: Rect,
    pub visual_rect: Rect,
    pub hit_rect: Rect,
    pub min_first_extent: f32,
    pub max_first_extent: f32,
    pub constraints_satisfied: bool,
}

impl DividerGeometry {
    pub fn fraction_for_pointer(&self, pointer: Pos2) -> f32 {
        let usable = (axis_extent(self.axis, self.parent_rect) - DIVIDER_VISUAL_POINTS).max(0.0);
        if usable <= f32::EPSILON {
            return 0.5;
        }
        let raw = axis_position(self.axis, pointer)
            - axis_min(self.axis, self.parent_rect)
            - DIVIDER_VISUAL_POINTS * 0.5;
        raw.clamp(self.min_first_extent, self.max_first_extent) / usable
    }

    pub fn hit_extent_points(&self) -> f32 {
        axis_extent(self.axis, self.hit_rect)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabRegionGeometry {
    pub region_id: RegionId,
    pub rect: Rect,
    pub tab_bar_rect: Rect,
    pub content_rect: Rect,
    pub tabs: Vec<PanelTab>,
    pub active_panel: PanelId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkbenchGeometry {
    pub root_rect: Rect,
    pub pixels_per_point: f32,
    pub dividers: Vec<DividerGeometry>,
    pub tab_regions: Vec<TabRegionGeometry>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorkbenchOptions {
    pub hide_single_tab_bar: bool,
}

pub fn calculate_workbench_geometry(
    state: &WorkbenchState,
    root_rect: Rect,
    pixels_per_point: f32,
) -> Result<WorkbenchGeometry, LayoutValidationError> {
    calculate_workbench_geometry_with_options(
        state,
        root_rect,
        pixels_per_point,
        WorkbenchOptions::default(),
    )
}

pub fn calculate_workbench_geometry_with_options(
    state: &WorkbenchState,
    root_rect: Rect,
    pixels_per_point: f32,
    options: WorkbenchOptions,
) -> Result<WorkbenchGeometry, LayoutValidationError> {
    state.validate()?;
    let pixels_per_point = if pixels_per_point.is_finite() && pixels_per_point > 0.0 {
        pixels_per_point
    } else {
        1.0
    };
    let mut geometry = WorkbenchGeometry {
        root_rect,
        pixels_per_point,
        dividers: Vec::new(),
        tab_regions: Vec::new(),
    };
    layout_node(&state.root, root_rect, options, &mut geometry);
    Ok(geometry)
}

fn layout_node(
    node: &WorkbenchNode,
    rect: Rect,
    options: WorkbenchOptions,
    geometry: &mut WorkbenchGeometry,
) {
    match node {
        WorkbenchNode::Split {
            id,
            axis,
            fraction,
            first_constraints,
            second_constraints,
            first,
            second,
        } => {
            let usable = (axis_extent(*axis, rect) - DIVIDER_VISUAL_POINTS).max(0.0);
            let (minimum, maximum, constraints_satisfied) =
                constraint_bounds(usable, *first_constraints, *second_constraints);
            let first_extent = (usable * fraction).clamp(minimum, maximum);
            let divider_center = axis_min(*axis, rect) + first_extent + DIVIDER_VISUAL_POINTS * 0.5;
            let (first_rect, second_rect, visual_rect, hit_rect) =
                split_rects(*axis, rect, divider_center);

            geometry.dividers.push(DividerGeometry {
                split_id: id.clone(),
                axis: *axis,
                parent_rect: rect,
                visual_rect,
                hit_rect,
                min_first_extent: minimum,
                max_first_extent: maximum,
                constraints_satisfied,
            });
            layout_node(first, first_rect, options, geometry);
            layout_node(second, second_rect, options, geometry);
        }
        WorkbenchNode::Tabs { id, tabs, active } => {
            let tab_bar_height = if options.hide_single_tab_bar && tabs.len() == 1 {
                0.0
            } else {
                TAB_BAR_POINTS.min(rect.height().max(0.0))
            };
            let tab_bar_rect =
                Rect::from_min_max(rect.min, pos2(rect.max.x, rect.min.y + tab_bar_height));
            let content_rect = Rect::from_min_max(pos2(rect.min.x, tab_bar_rect.max.y), rect.max);
            geometry.tab_regions.push(TabRegionGeometry {
                region_id: id.clone(),
                rect,
                tab_bar_rect,
                content_rect,
                tabs: tabs.clone(),
                active_panel: active.clone(),
            });
        }
    }
}

fn constraint_bounds(
    usable: f32,
    first: PaneConstraints,
    second: PaneConstraints,
) -> (f32, f32, bool) {
    let first_max = first.max_points.unwrap_or(usable).min(usable);
    let second_max = second.max_points.unwrap_or(usable).min(usable);
    let minimum = first.min_points.max(usable - second_max).clamp(0.0, usable);
    let maximum = first_max.min(usable - second.min_points).clamp(0.0, usable);

    if minimum <= maximum {
        (minimum, maximum, true)
    } else {
        // The viewport is too small (or too large for both maxima) to satisfy
        // every constraint. Keep the divider valid and expose the conflict.
        (0.0, usable, false)
    }
}

fn split_rects(axis: SplitAxis, rect: Rect, center: f32) -> (Rect, Rect, Rect, Rect) {
    let half_visual = DIVIDER_VISUAL_POINTS * 0.5;
    match axis {
        SplitAxis::Horizontal => {
            let first = Rect::from_min_max(rect.min, pos2(center - half_visual, rect.max.y));
            let second = Rect::from_min_max(pos2(center + half_visual, rect.min.y), rect.max);
            let visual = Rect::from_center_size(
                pos2(center, rect.center().y),
                vec2(DIVIDER_VISUAL_POINTS, rect.height()),
            );
            let hit = Rect::from_center_size(
                pos2(center, rect.center().y),
                vec2(DIVIDER_HIT_POINTS, rect.height()),
            );
            (first, second, visual, hit)
        }
        SplitAxis::Vertical => {
            let first = Rect::from_min_max(rect.min, pos2(rect.max.x, center - half_visual));
            let second = Rect::from_min_max(pos2(rect.min.x, center + half_visual), rect.max);
            let visual = Rect::from_center_size(
                pos2(rect.center().x, center),
                vec2(rect.width(), DIVIDER_VISUAL_POINTS),
            );
            let hit = Rect::from_center_size(
                pos2(rect.center().x, center),
                vec2(rect.width(), DIVIDER_HIT_POINTS),
            );
            (first, second, visual, hit)
        }
    }
}

fn axis_extent(axis: SplitAxis, rect: Rect) -> f32 {
    match axis {
        SplitAxis::Horizontal => rect.width(),
        SplitAxis::Vertical => rect.height(),
    }
}

fn axis_min(axis: SplitAxis, rect: Rect) -> f32 {
    match axis {
        SplitAxis::Horizontal => rect.min.x,
        SplitAxis::Vertical => rect.min.y,
    }
}

fn axis_position(axis: SplitAxis, position: Pos2) -> f32 {
    match axis {
        SplitAxis::Horizontal => position.x,
        SplitAxis::Vertical => position.y,
    }
}

#[cfg(test)]
mod tests {
    use egui::{Rect, pos2, vec2};

    use crate::{PaneConstraints, PanelTab, SplitAxis, WorkbenchNode, WorkbenchState};

    use super::{
        DIVIDER_HIT_POINTS, WorkbenchOptions, calculate_workbench_geometry,
        calculate_workbench_geometry_with_options,
    };

    fn split_state(fraction: f32) -> WorkbenchState {
        WorkbenchState::new(WorkbenchNode::split(
            "root",
            SplitAxis::Horizontal,
            fraction,
            PaneConstraints::minimum(200.0),
            PaneConstraints::bounded(300.0, 500.0),
            WorkbenchNode::tabs("left", vec![PanelTab::new("media", "Media")], "media"),
            WorkbenchNode::tabs(
                "right",
                vec![PanelTab::new("preview", "Preview")],
                "preview",
            ),
        ))
    }

    #[test]
    fn split_extent_respects_minimum_and_maximum_constraints() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1000.0, 600.0));
        let too_small = calculate_workbench_geometry(&split_state(0.05), rect, 1.0).unwrap();
        // The second pane has a 500 point maximum, so the first pane must
        // consume the rest even though its own minimum is only 200.
        assert_eq!(too_small.tab_regions[0].rect.width(), 499.0);

        let too_large = calculate_workbench_geometry(&split_state(0.95), rect, 1.0).unwrap();
        // The right pane has a 300 point minimum, so the first pane stops there.
        assert_eq!(too_large.tab_regions[0].rect.width(), 699.0);
    }

    #[test]
    fn divider_hit_target_is_maintained_at_common_dpi_scales() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1000.0, 600.0));
        for pixels_per_point in [1.0, 1.5, 2.0] {
            let geometry =
                calculate_workbench_geometry(&split_state(0.5), rect, pixels_per_point).unwrap();
            let divider = &geometry.dividers[0];
            assert_eq!(divider.hit_extent_points(), DIVIDER_HIT_POINTS);
            assert!(
                divider.hit_extent_points() * geometry.pixels_per_point
                    >= DIVIDER_HIT_POINTS * pixels_per_point
            );
        }
    }

    #[test]
    fn divider_pointer_fraction_is_clamped_by_constraints() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1000.0, 600.0));
        let geometry = calculate_workbench_geometry(&split_state(0.5), rect, 1.0).unwrap();
        let divider = &geometry.dividers[0];
        let fraction = divider.fraction_for_pointer(pos2(5.0, 200.0));
        assert!((fraction - 499.0 / 999.0).abs() < 0.0001);
    }

    #[test]
    fn default_single_tab_region_keeps_its_tab_strip() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1000.0, 600.0));
        let geometry = calculate_workbench_geometry(&split_state(0.5), rect, 1.0).unwrap();

        for region in &geometry.tab_regions {
            assert_eq!(region.tabs.len(), 1);
            assert_eq!(region.tab_bar_rect.height(), 30.0);
            assert_eq!(region.content_rect.min.y, region.rect.min.y + 30.0);
        }
    }

    #[test]
    fn opt_in_single_tab_policy_gives_the_full_rect_to_the_panel() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1000.0, 600.0));
        let geometry = calculate_workbench_geometry_with_options(
            &split_state(0.5),
            rect,
            1.0,
            WorkbenchOptions {
                hide_single_tab_bar: true,
            },
        )
        .unwrap();

        for region in &geometry.tab_regions {
            assert_eq!(region.tabs.len(), 1);
            assert_eq!(region.tab_bar_rect.height(), 0.0);
            assert_eq!(region.content_rect, region.rect);
        }
    }
}
