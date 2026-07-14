//! Feature-gated virtualized timeline with caller-owned state and typed actions.

#[cfg(feature = "preview")]
mod geometry;
#[cfg(feature = "preview")]
mod interaction;
#[cfg(feature = "preview")]
mod model;
#[cfg(feature = "preview")]
mod widget;

#[cfg(feature = "preview")]
pub use geometry::{
    RULER_HEIGHT_POINTS, TimelineClipGeometry, TimelineGeometry, TimelineGeometryCache,
    TimelineHitTarget, TimelineHitZone, TimelineRulerMark,
};
#[cfg(feature = "preview")]
pub use interaction::{
    TimelineAction, TimelineDragKind, TimelineDragSession, TimelineInteractionState,
    TimelineModifiers, TimelineSnapKind, TimelineSnapPolicy, TimelineSnapResult,
};
#[cfg(feature = "preview")]
pub use model::{ClipId, TimelineClip, TimelineSnapshot, TimelineTrack, TimelineViewport, TrackId};
#[cfg(feature = "preview")]
pub use widget::{
    TimelineOutput, TimelineUxReceipt, show_timeline, show_timeline_with_policy,
    timeline_receipt_json,
};

pub const TIMELINE_API_STABILITY: &str = "preview";

pub const TIMELINE_PREVIEW_API: &[&str] = &[
    "TimelineSnapshot",
    "TimelineViewport",
    "TimelineGeometryCache",
    "TimelineAction",
    "TimelineInteractionState",
    "TimelineOutput",
    "TimelineUxReceipt",
    "timeline_receipt_json",
    "show_timeline",
];

pub const TIMELINE_EXPERIMENTAL_API: &[&str] = &["TimelinePerformanceReceipt"];

/// Benchmark schema that may change while product-window measurements stabilize.
#[cfg(feature = "experimental")]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TimelinePerformanceReceipt {
    pub sample_count: usize,
    pub geometry_p95_micros: f64,
    pub paint_p95_micros: f64,
}
