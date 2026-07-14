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

    pub fn zoom_around(&mut self, anchor_tick: i64, scale: f64) {
        let previous = self.ticks_per_point.max(f64::EPSILON);
        let next = (previous * scale).clamp(0.01, 1_000_000.0);
        let anchor_points = (anchor_tick.saturating_sub(self.start_tick)) as f64 / previous;
        self.start_tick = anchor_tick
            .saturating_sub((anchor_points * next).round() as i64)
            .max(0);
        self.ticks_per_point = next;
    }

    pub fn scroll_ticks(&mut self, delta_ticks: i64) {
        self.start_tick = self.start_tick.saturating_add(delta_ticks).max(0);
    }

    pub fn scroll_tracks(&mut self, delta_points: f32) {
        self.vertical_scroll_points = (self.vertical_scroll_points + delta_points).max(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::TimelineViewport;

    #[test]
    fn zoom_and_scroll_preserve_pointer_tick_and_clamp_navigation() {
        let mut viewport = TimelineViewport::new(100, 2.0);
        viewport.zoom_around(300, 0.5);
        assert_eq!(viewport.start_tick, 200);
        assert_eq!(viewport.ticks_per_point, 1.0);
        viewport.scroll_ticks(-500);
        assert_eq!(viewport.start_tick, 0);
        viewport.scroll_tracks(68.0);
        assert_eq!(viewport.vertical_scroll_points, 68.0);
        viewport.scroll_tracks(-100.0);
        assert_eq!(viewport.vertical_scroll_points, 0.0);
    }
}
