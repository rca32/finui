use crate::{ClipId, TimelineHitZone};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelineDragKind {
    Move,
    TrimStart,
    TrimEnd,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TimelineSnapKind {
    #[default]
    Frame,
    Grid,
    ClipEdge,
    Bypassed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TimelineModifiers {
    pub bypass_snap: bool,
    pub grid_only: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimelineSnapPolicy {
    pub grid_ticks: i64,
    pub threshold_points: f64,
}

impl Default for TimelineSnapPolicy {
    fn default() -> Self {
        Self {
            grid_ticks: 10,
            threshold_points: 4.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimelineSnapResult {
    pub delta_ticks: i64,
    pub kind: TimelineSnapKind,
}

impl From<TimelineHitZone> for TimelineDragKind {
    fn from(value: TimelineHitZone) -> Self {
        match value {
            TimelineHitZone::Body => Self::Move,
            TimelineHitZone::TrimStart => Self::TrimStart,
            TimelineHitZone::TrimEnd => Self::TrimEnd,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimelineDragSession {
    pub clip_id: ClipId,
    pub kind: TimelineDragKind,
    pub origin_pointer_tick: i64,
    pub origin_start_tick: i64,
    pub origin_duration_ticks: i64,
    pub raw_delta_ticks: i64,
    pub delta_ticks: i64,
    pub snap_kind: TimelineSnapKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimelineAction {
    SelectClip {
        clip_id: ClipId,
        additive: bool,
    },
    SetPlayhead {
        tick: i64,
    },
    BeginDrag {
        clip_id: ClipId,
        kind: TimelineDragKind,
        pointer_tick: i64,
        origin_start_tick: i64,
        origin_duration_ticks: i64,
    },
    UpdateDrag {
        clip_id: ClipId,
        kind: TimelineDragKind,
        raw_delta_ticks: i64,
        delta_ticks: i64,
        snap_kind: TimelineSnapKind,
    },
    CommitDrag {
        clip_id: ClipId,
        kind: TimelineDragKind,
        raw_delta_ticks: i64,
        delta_ticks: i64,
        snap_kind: TimelineSnapKind,
    },
    CancelDrag {
        clip_id: ClipId,
        kind: TimelineDragKind,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TimelineInteractionState {
    pub drag: Option<TimelineDragSession>,
}

impl TimelineInteractionState {
    /// Applies only interaction state. The caller remains responsible for clip mutation.
    pub fn apply(&mut self, action: &TimelineAction) -> bool {
        match action {
            TimelineAction::BeginDrag {
                clip_id,
                kind,
                pointer_tick,
                origin_start_tick,
                origin_duration_ticks,
            } => {
                self.drag = Some(TimelineDragSession {
                    clip_id: clip_id.clone(),
                    kind: *kind,
                    origin_pointer_tick: *pointer_tick,
                    origin_start_tick: *origin_start_tick,
                    origin_duration_ticks: *origin_duration_ticks,
                    raw_delta_ticks: 0,
                    delta_ticks: 0,
                    snap_kind: TimelineSnapKind::Frame,
                });
                true
            }
            TimelineAction::UpdateDrag {
                clip_id,
                kind,
                raw_delta_ticks,
                delta_ticks,
                snap_kind,
            } => {
                let Some(drag) = self.drag.as_mut() else {
                    return false;
                };
                if drag.clip_id != *clip_id || drag.kind != *kind {
                    return false;
                }
                drag.raw_delta_ticks = *raw_delta_ticks;
                drag.delta_ticks = *delta_ticks;
                drag.snap_kind = *snap_kind;
                true
            }
            TimelineAction::CommitDrag { clip_id, kind, .. }
            | TimelineAction::CancelDrag { clip_id, kind } => {
                if self
                    .drag
                    .as_ref()
                    .is_some_and(|drag| drag.clip_id == *clip_id && drag.kind == *kind)
                {
                    self.drag = None;
                    true
                } else {
                    false
                }
            }
            TimelineAction::SelectClip { .. } | TimelineAction::SetPlayhead { .. } => false,
        }
    }

    pub fn apply_all<'a>(&mut self, actions: impl IntoIterator<Item = &'a TimelineAction>) {
        for action in actions {
            self.apply(action);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn begin(kind: TimelineDragKind) -> TimelineAction {
        TimelineAction::BeginDrag {
            clip_id: ClipId::from("clip-a"),
            kind,
            pointer_tick: 100,
            origin_start_tick: 90,
            origin_duration_ticks: 50,
        }
    }

    #[test]
    fn drag_begin_update_commit_is_transactional() {
        for kind in [
            TimelineDragKind::Move,
            TimelineDragKind::TrimStart,
            TimelineDragKind::TrimEnd,
        ] {
            let mut state = TimelineInteractionState::default();
            assert!(state.apply(&begin(kind)));
            assert!(state.apply(&TimelineAction::UpdateDrag {
                clip_id: ClipId::from("clip-a"),
                kind,
                raw_delta_ticks: 27,
                delta_ticks: 25,
                snap_kind: TimelineSnapKind::Grid,
            }));
            assert_eq!(state.drag.as_ref().unwrap().raw_delta_ticks, 27);
            assert_eq!(state.drag.as_ref().unwrap().delta_ticks, 25);
            assert_eq!(
                state.drag.as_ref().unwrap().snap_kind,
                TimelineSnapKind::Grid
            );
            assert!(state.apply(&TimelineAction::CommitDrag {
                clip_id: ClipId::from("clip-a"),
                kind,
                raw_delta_ticks: 27,
                delta_ticks: 25,
                snap_kind: TimelineSnapKind::Grid,
            }));
            assert!(state.drag.is_none());
        }
    }

    #[test]
    fn drag_cancel_clears_session_without_model_mutation() {
        let mut state = TimelineInteractionState::default();
        state.apply(&begin(TimelineDragKind::Move));
        assert!(state.apply(&TimelineAction::CancelDrag {
            clip_id: ClipId::from("clip-a"),
            kind: TimelineDragKind::Move,
        }));
        assert!(state.drag.is_none());
    }
}
