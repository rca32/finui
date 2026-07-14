//! Preview API for caller-owned editor workbench layouts.
//!
//! The crate depends on `egui`, but not on `eframe` or a renderer backend.

mod geometry;
mod model;
mod widget;

pub use geometry::{
    DIVIDER_HIT_POINTS, DIVIDER_VISUAL_POINTS, DividerGeometry, TabRegionGeometry,
    WorkbenchGeometry, calculate_workbench_geometry,
};
pub use model::{
    CommandScopeOutput, FocusRoute, LayoutRestoreError, LayoutValidationError, PaneConstraints,
    PanelId, PanelTab, RegionId, SplitAxis, SplitId, WORKBENCH_LAYOUT_SCHEMA_VERSION,
    WorkbenchAction, WorkbenchNode, WorkbenchState,
};
pub use widget::{WorkbenchOutput, show_workbench};

/// This API is intentionally preview while editor workflows are exercised in labs.
pub const WORKBENCH_API_STABILITY: &str = "preview";
