//! Feature-gated API for caller-owned editor workbench layouts.
//!
//! The crate depends on `egui`, but not on `eframe` or a renderer backend.

#[cfg(feature = "preview")]
mod geometry;
#[cfg(feature = "preview")]
mod model;
#[cfg(feature = "preview")]
mod transport;
#[cfg(feature = "preview")]
mod widget;

#[cfg(feature = "preview")]
pub use geometry::{
    DIVIDER_HIT_POINTS, DIVIDER_VISUAL_POINTS, DividerGeometry, TabRegionGeometry,
    WorkbenchGeometry, calculate_workbench_geometry,
};
#[cfg(feature = "experimental")]
pub use geometry::{WorkbenchOptions, calculate_workbench_geometry_with_options};
#[cfg(feature = "preview")]
pub use model::{
    CommandScopeOutput, FocusRoute, LayoutValidationError, PaneConstraints, PanelId, PanelTab,
    RegionId, SplitAxis, SplitId, WorkbenchAction, WorkbenchNode, WorkbenchState,
};
#[cfg(feature = "experimental")]
pub use model::{LayoutRestoreError, WORKBENCH_LAYOUT_SCHEMA_VERSION};
#[cfg(feature = "preview")]
pub use transport::{
    KeyboardTransportAction, KeyboardTransportOutput, KeyboardTransportReceipt,
    keyboard_transport_actions, keyboard_transport_actions_from_pressed,
};
#[cfg(feature = "experimental")]
pub use widget::show_workbench_with_options;
#[cfg(feature = "preview")]
pub use widget::{WorkbenchOutput, show_workbench};

/// Stability label for the feature-gated workbench API.
pub const WORKBENCH_API_STABILITY: &str = "preview";

/// Public items covered by the Preview compatibility policy.
pub const WORKBENCH_PREVIEW_API: &[&str] = &[
    "WorkbenchNode",
    "WorkbenchState",
    "WorkbenchAction",
    "FocusRoute",
    "CommandScopeOutput",
    "KeyboardTransportAction",
    "KeyboardTransportOutput",
    "KeyboardTransportReceipt",
    "keyboard_transport_actions",
    "keyboard_transport_actions_from_pressed",
    "WorkbenchOutput",
    "show_workbench",
];

/// Public items that require the `experimental` Cargo feature.
pub const WORKBENCH_EXPERIMENTAL_API: &[&str] = &[
    "WorkbenchOptions",
    "calculate_workbench_geometry_with_options",
    "show_workbench_with_options",
    "WorkbenchState::to_json_pretty",
    "WorkbenchState::from_json",
    "LayoutRestoreError",
    "WORKBENCH_LAYOUT_SCHEMA_VERSION",
];
