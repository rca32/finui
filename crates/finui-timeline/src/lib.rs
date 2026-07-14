//! Preview virtualized timeline with caller-owned state and typed actions.

mod geometry;
mod interaction;
mod model;
mod widget;

pub use geometry::{
    RULER_HEIGHT_POINTS, TimelineClipGeometry, TimelineGeometry, TimelineGeometryCache,
    TimelineHitTarget, TimelineHitZone, TimelineRulerMark,
};
pub use interaction::{
    TimelineAction, TimelineDragKind, TimelineDragSession, TimelineInteractionState,
};
pub use model::{ClipId, TimelineClip, TimelineSnapshot, TimelineTrack, TimelineViewport, TrackId};
pub use widget::{TimelineOutput, TimelineUxReceipt, show_timeline, timeline_receipt_json};

pub const TIMELINE_API_STABILITY: &str = "preview";
