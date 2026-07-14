use std::fmt;

use serde::{Deserialize, Serialize};

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }
    };
}

string_id!(ClipId);
string_id!(TrackId);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineClip {
    pub id: ClipId,
    pub label: String,
    pub start_tick: i64,
    pub duration_ticks: i64,
    pub selected: bool,
}

impl TimelineClip {
    pub fn new(
        id: impl Into<ClipId>,
        label: impl Into<String>,
        start_tick: i64,
        duration_ticks: i64,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            start_tick,
            duration_ticks: duration_ticks.max(1),
            selected: false,
        }
    }

    pub fn end_tick(&self) -> i64 {
        self.start_tick.saturating_add(self.duration_ticks.max(1))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineTrack {
    pub id: TrackId,
    pub label: String,
    pub clips: Vec<TimelineClip>,
}

impl TimelineTrack {
    pub fn new(id: impl Into<TrackId>, label: impl Into<String>, clips: Vec<TimelineClip>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            clips,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimelineSnapshot {
    pub revision: u64,
    pub tracks: Vec<TimelineTrack>,
}

impl TimelineSnapshot {
    pub fn total_clip_count(&self) -> usize {
        self.tracks.iter().map(|track| track.clips.len()).sum()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimelineViewport {
    pub start_tick: i64,
    pub ticks_per_point: f64,
    pub vertical_scroll_points: f32,
    pub track_height_points: f32,
}

impl TimelineViewport {
    pub fn new(start_tick: i64, ticks_per_point: f64) -> Self {
        Self {
            start_tick,
            ticks_per_point: ticks_per_point.max(f64::EPSILON),
            vertical_scroll_points: 0.0,
            track_height_points: 34.0,
        }
    }
}
