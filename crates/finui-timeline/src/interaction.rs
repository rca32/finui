use crate::{ClipId, TimelineHitZone};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelineDragKind {
    Move,
    TrimStart,
    TrimEnd,
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
    pub delta_ticks: i64,
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
        delta_ticks: i64,
    },
    CommitDrag {
        clip_id: ClipId,
        kind: TimelineDragKind,
        delta_ticks: i64,
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
                    delta_ticks: 0,
                });
                true
            }
            TimelineAction::UpdateDrag {
                clip_id,
                kind,
                delta_ticks,
            } => {
                let Some(drag) = self.drag.as_mut() else {
                    return false;
                };
                if drag.clip_id != *clip_id || drag.kind != *kind {
                    return false;
                }
                drag.delta_ticks = *delta_ticks;
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
                delta_ticks: 25,
            }));
            assert_eq!(state.drag.as_ref().unwrap().delta_ticks, 25);
            assert!(state.apply(&TimelineAction::CommitDrag {
                clip_id: ClipId::from("clip-a"),
                kind,
                delta_ticks: 25,
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
