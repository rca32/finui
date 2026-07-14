use egui::{Pos2, Rect, pos2};

use crate::{
    ClipId, TimelineClip, TimelineDragKind, TimelineDragSession, TimelineModifiers,
    TimelineSnapKind, TimelineSnapPolicy, TimelineSnapResult, TimelineSnapshot, TimelineViewport,
    TrackId,
};

pub const RULER_HEIGHT_POINTS: f32 = 24.0;
const TRIM_HIT_POINTS: f32 = 6.0;

#[derive(Clone, Debug)]
struct IndexedTrack {
    id: TrackId,
    label: String,
    clips: Vec<TimelineClip>,
    prefix_max_end: Vec<i64>,
}

#[derive(Clone, Debug)]
struct SnapEdge {
    tick: i64,
    clip_id: ClipId,
}

#[derive(Clone, Debug)]
pub struct TimelineGeometryCache {
    revision: u64,
    tracks: Vec<IndexedTrack>,
    snap_edges: Vec<SnapEdge>,
    total_clip_count: usize,
}

impl TimelineGeometryCache {
    /// Builds a stable per-track interval index in O(n log n) total time.
    pub fn build(snapshot: &TimelineSnapshot) -> Self {
        let tracks = snapshot
            .tracks
            .iter()
            .map(|track| {
                let mut clips = track.clips.clone();
                clips.sort_by_key(|clip| clip.start_tick);
                let mut maximum = i64::MIN;
                let prefix_max_end = clips
                    .iter()
                    .map(|clip| {
                        maximum = maximum.max(clip.end_tick());
                        maximum
                    })
                    .collect();
                IndexedTrack {
                    id: track.id.clone(),
                    label: track.label.clone(),
                    clips,
                    prefix_max_end,
                }
            })
            .collect();
        let mut snap_edges = snapshot
            .tracks
            .iter()
            .flat_map(|track| &track.clips)
            .flat_map(|clip| {
                [
                    SnapEdge {
                        tick: clip.start_tick,
                        clip_id: clip.id.clone(),
                    },
                    SnapEdge {
                        tick: clip.end_tick(),
                        clip_id: clip.id.clone(),
                    },
                ]
            })
            .collect::<Vec<_>>();
        snap_edges.sort_by_key(|edge| edge.tick);
        Self {
            revision: snapshot.revision,
            tracks,
            snap_edges,
            total_clip_count: snapshot.total_clip_count(),
        }
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn total_clip_count(&self) -> usize {
        self.total_clip_count
    }

    pub fn snap_drag_delta(
        &self,
        drag: &TimelineDragSession,
        raw_delta_ticks: i64,
        viewport: TimelineViewport,
        policy: TimelineSnapPolicy,
        modifiers: TimelineModifiers,
    ) -> TimelineSnapResult {
        if modifiers.bypass_snap {
            return TimelineSnapResult {
                delta_ticks: raw_delta_ticks,
                kind: TimelineSnapKind::Bypassed,
            };
        }
        let threshold_ticks = (policy.threshold_points.max(0.0)
            * viewport.ticks_per_point.max(f64::EPSILON))
        .ceil() as i64;
        let targets = match drag.kind {
            TimelineDragKind::Move => vec![
                drag.origin_start_tick.saturating_add(raw_delta_ticks),
                drag.origin_start_tick
                    .saturating_add(drag.origin_duration_ticks)
                    .saturating_add(raw_delta_ticks),
            ],
            TimelineDragKind::TrimStart => {
                vec![drag.origin_start_tick.saturating_add(raw_delta_ticks)]
            }
            TimelineDragKind::TrimEnd => vec![
                drag.origin_start_tick
                    .saturating_add(drag.origin_duration_ticks)
                    .saturating_add(raw_delta_ticks),
            ],
        };
        let mut best: Option<(i64, TimelineSnapKind)> = None;
        let grid_ticks = policy.grid_ticks.max(1);
        for target in &targets {
            let lower = target.div_euclid(grid_ticks).saturating_mul(grid_ticks);
            for grid in [lower, lower.saturating_add(grid_ticks)] {
                consider_snap(
                    &mut best,
                    grid.saturating_sub(*target),
                    threshold_ticks,
                    TimelineSnapKind::Grid,
                );
            }
        }
        if !modifiers.grid_only {
            for target in targets {
                let first = self
                    .snap_edges
                    .partition_point(|edge| edge.tick < target.saturating_sub(threshold_ticks));
                for edge in &self.snap_edges[first..] {
                    if edge.tick > target.saturating_add(threshold_ticks) {
                        break;
                    }
                    if edge.clip_id == drag.clip_id {
                        continue;
                    }
                    consider_snap(
                        &mut best,
                        edge.tick.saturating_sub(target),
                        threshold_ticks,
                        TimelineSnapKind::ClipEdge,
                    );
                }
            }
        }
        best.map_or(
            TimelineSnapResult {
                delta_ticks: raw_delta_ticks,
                kind: TimelineSnapKind::Frame,
            },
            |(adjustment, kind)| TimelineSnapResult {
                delta_ticks: raw_delta_ticks.saturating_add(adjustment),
                kind,
            },
        )
    }

    /// Queries O(visible_tracks * log(clips_per_track) + candidate_clips).
    pub fn geometry(
        &self,
        viewport: TimelineViewport,
        rect: Rect,
        playhead_tick: i64,
    ) -> TimelineGeometry {
        let ticks_per_point = viewport.ticks_per_point.max(f64::EPSILON);
        let track_height = viewport.track_height_points.max(12.0);
        let content_rect = Rect::from_min_max(
            pos2(
                rect.min.x,
                (rect.min.y + RULER_HEIGHT_POINTS).min(rect.max.y),
            ),
            rect.max,
        );
        let visible_tick_end = viewport
            .start_tick
            .saturating_add((f64::from(content_rect.width()) * ticks_per_point).ceil() as i64);
        let first_track = ((viewport.vertical_scroll_points.max(0.0) / track_height).floor()
            as usize)
            .min(self.tracks.len());
        let visible_track_count = (content_rect.height() / track_height).ceil() as usize + 1;
        let last_track = (first_track + visible_track_count).min(self.tracks.len());

        let mut visible_clips = Vec::new();
        let mut candidate_clip_count = 0;
        for track_index in first_track..last_track {
            let track = &self.tracks[track_index];
            let first_candidate = track
                .prefix_max_end
                .partition_point(|end_tick| *end_tick <= viewport.start_tick);
            let last_candidate = track
                .clips
                .partition_point(|clip| clip.start_tick < visible_tick_end);
            if first_candidate >= last_candidate {
                continue;
            }
            candidate_clip_count += last_candidate - first_candidate;
            let y = content_rect.min.y + track_index as f32 * track_height
                - viewport.vertical_scroll_points;
            for clip in &track.clips[first_candidate..last_candidate] {
                if clip.end_tick() <= viewport.start_tick || clip.start_tick >= visible_tick_end {
                    continue;
                }
                let left = rect.min.x
                    + ((clip.start_tick - viewport.start_tick) as f64 / ticks_per_point) as f32;
                let right = rect.min.x
                    + ((clip.end_tick() - viewport.start_tick) as f64 / ticks_per_point) as f32;
                let unclipped = Rect::from_min_max(
                    pos2(left, y + 2.0),
                    pos2(right.max(left + 1.0), y + track_height - 2.0),
                );
                let clipped = unclipped.intersect(content_rect);
                if clipped.is_positive() {
                    visible_clips.push(TimelineClipGeometry {
                        track_id: track.id.clone(),
                        track_label: track.label.clone(),
                        clip_id: clip.id.clone(),
                        label: clip.label.clone(),
                        start_tick: clip.start_tick,
                        duration_ticks: clip.duration_ticks.max(1),
                        selected: clip.selected,
                        rect: clipped,
                    });
                }
            }
        }

        let playhead_x =
            rect.min.x + ((playhead_tick - viewport.start_tick) as f64 / ticks_per_point) as f32;
        TimelineGeometry {
            rect,
            content_rect,
            visible_tick_range: (viewport.start_tick, visible_tick_end),
            visible_track_range: (first_track, last_track),
            visible_clips,
            ruler_marks: ruler_marks(viewport.start_tick, visible_tick_end, ticks_per_point),
            playhead_x: rect
                .contains(pos2(playhead_x, rect.center().y))
                .then_some(playhead_x),
            candidate_clip_count,
            total_clip_count: self.total_clip_count,
        }
    }
}

fn consider_snap(
    best: &mut Option<(i64, TimelineSnapKind)>,
    adjustment: i64,
    threshold_ticks: i64,
    kind: TimelineSnapKind,
) {
    if adjustment.unsigned_abs() > threshold_ticks.max(0) as u64 {
        return;
    }
    let replace = best.as_ref().is_none_or(|(current, current_kind)| {
        adjustment.unsigned_abs() < current.unsigned_abs()
            || (adjustment.unsigned_abs() == current.unsigned_abs()
                && kind == TimelineSnapKind::ClipEdge
                && *current_kind != TimelineSnapKind::ClipEdge)
    });
    if replace {
        *best = Some((adjustment, kind));
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineClipGeometry {
    pub track_id: TrackId,
    pub track_label: String,
    pub clip_id: ClipId,
    pub label: String,
    pub start_tick: i64,
    pub duration_ticks: i64,
    pub selected: bool,
    pub rect: Rect,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimelineHitZone {
    Body,
    TrimStart,
    TrimEnd,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineHitTarget {
    pub clip_id: ClipId,
    pub zone: TimelineHitZone,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimelineRulerMark {
    pub tick: i64,
    pub x: f32,
}

#[derive(Clone, Debug)]
pub struct TimelineGeometry {
    pub rect: Rect,
    pub content_rect: Rect,
    pub visible_tick_range: (i64, i64),
    pub visible_track_range: (usize, usize),
    pub visible_clips: Vec<TimelineClipGeometry>,
    pub ruler_marks: Vec<TimelineRulerMark>,
    pub playhead_x: Option<f32>,
    pub candidate_clip_count: usize,
    pub total_clip_count: usize,
}

impl TimelineGeometry {
    /// Hit-tests only the already culled visible clip geometry.
    pub fn hit_test(&self, pointer: Pos2) -> Option<TimelineHitTarget> {
        self.visible_clips.iter().rev().find_map(|clip| {
            if !clip.rect.contains(pointer) {
                return None;
            }
            let zone = if pointer.x <= clip.rect.min.x + TRIM_HIT_POINTS {
                TimelineHitZone::TrimStart
            } else if pointer.x >= clip.rect.max.x - TRIM_HIT_POINTS {
                TimelineHitZone::TrimEnd
            } else {
                TimelineHitZone::Body
            };
            Some(TimelineHitTarget {
                clip_id: clip.clip_id.clone(),
                zone,
            })
        })
    }
}

fn ruler_marks(start_tick: i64, end_tick: i64, ticks_per_point: f64) -> Vec<TimelineRulerMark> {
    let step = nice_step((ticks_per_point * 80.0).ceil() as i64);
    let first =
        start_tick.div_euclid(step) * step + i64::from(start_tick.rem_euclid(step) != 0) * step;
    let mut marks = Vec::new();
    let mut tick = first;
    while tick < end_tick {
        marks.push(TimelineRulerMark {
            tick,
            x: ((tick - start_tick) as f64 / ticks_per_point) as f32,
        });
        tick = tick.saturating_add(step);
    }
    marks
}

fn nice_step(minimum: i64) -> i64 {
    let minimum = minimum.max(1);
    let power = 10_i64.pow(minimum.ilog10());
    for multiplier in [1, 2, 5, 10] {
        let candidate = power.saturating_mul(multiplier);
        if candidate >= minimum {
            return candidate;
        }
    }
    power.saturating_mul(10)
}

#[cfg(test)]
mod tests {
    use egui::{Rect, pos2, vec2};

    use crate::{
        TimelineClip, TimelineDragKind, TimelineDragSession, TimelineModifiers, TimelineSnapKind,
        TimelineSnapPolicy, TimelineSnapshot, TimelineTrack, TimelineViewport,
    };

    use super::{TimelineGeometryCache, TimelineHitZone};

    fn fixture(track_count: usize, clips_per_track: usize) -> TimelineSnapshot {
        TimelineSnapshot {
            revision: 7,
            tracks: (0..track_count)
                .map(|track| {
                    TimelineTrack::new(
                        format!("track-{track}"),
                        format!("Track {track}"),
                        (0..clips_per_track)
                            .map(|clip| {
                                TimelineClip::new(
                                    format!("clip-{track}-{clip}"),
                                    format!("Clip {clip}"),
                                    clip as i64 * 120,
                                    100,
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    #[test]
    fn ten_thousand_clips_query_only_visible_candidates() {
        let snapshot = fixture(100, 100);
        let cache = TimelineGeometryCache::build(&snapshot);
        let viewport = TimelineViewport {
            start_tick: 4_800,
            ticks_per_point: 2.0,
            vertical_scroll_points: 34.0 * 40.0,
            track_height_points: 34.0,
        };
        let geometry = cache.geometry(
            viewport,
            Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 240.0)),
            5_000,
        );

        assert_eq!(geometry.total_clip_count, 10_000);
        assert!(geometry.candidate_clip_count < 120);
        assert!(geometry.visible_clips.len() < 120);
        assert!(
            geometry
                .visible_clips
                .iter()
                .all(|clip| clip.clip_id.as_str().starts_with("clip-4"))
        );
    }

    #[test]
    fn indexed_query_matches_naive_overlap_order() {
        let snapshot = fixture(3, 30);
        let original = snapshot.clone();
        let cache = TimelineGeometryCache::build(&snapshot);
        let viewport = TimelineViewport::new(600, 1.5);
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(500.0, 300.0));
        let geometry = cache.geometry(viewport, rect, 700);
        let expected: Vec<_> = snapshot
            .tracks
            .iter()
            .flat_map(|track| {
                track.clips.iter().filter(|clip| {
                    clip.start_tick < geometry.visible_tick_range.1
                        && clip.end_tick() > geometry.visible_tick_range.0
                })
            })
            .map(|clip| clip.id.clone())
            .collect();
        let actual: Vec<_> = geometry
            .visible_clips
            .iter()
            .map(|clip| clip.clip_id.clone())
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(snapshot, original);
    }

    #[test]
    fn offscreen_clips_are_not_hit_tested() {
        let cache = TimelineGeometryCache::build(&fixture(1, 100));
        let geometry = cache.geometry(
            TimelineViewport::new(4_800, 2.0),
            Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 120.0)),
            4_800,
        );
        let first = &geometry.visible_clips[0];
        let hit = geometry.hit_test(first.rect.center()).unwrap();
        assert_eq!(hit.clip_id, first.clip_id);
        assert_eq!(hit.zone, TimelineHitZone::Body);
        assert_eq!(
            geometry.hit_test(first.rect.left_center()).unwrap().zone,
            TimelineHitZone::TrimStart
        );
        assert_eq!(
            geometry.hit_test(first.rect.right_center()).unwrap().zone,
            TimelineHitZone::TrimEnd
        );
        assert!(
            geometry
                .visible_clips
                .iter()
                .all(|clip| clip.clip_id.as_str() != "clip-0-0")
        );
    }

    #[test]
    fn drag_delta_snaps_to_grid_and_other_clip_edges_with_modifier_policy() {
        let snapshot = TimelineSnapshot {
            revision: 1,
            tracks: vec![TimelineTrack::new(
                "v1",
                "V1",
                vec![
                    TimelineClip::new("moving", "Moving", 10, 20),
                    TimelineClip::new("neighbor", "Neighbor", 50, 20),
                ],
            )],
        };
        let cache = TimelineGeometryCache::build(&snapshot);
        let drag = TimelineDragSession {
            clip_id: "moving".into(),
            kind: TimelineDragKind::Move,
            origin_pointer_tick: 10,
            origin_start_tick: 10,
            origin_duration_ticks: 20,
            raw_delta_ticks: 0,
            delta_ticks: 0,
            snap_kind: TimelineSnapKind::Frame,
        };
        let policy = TimelineSnapPolicy {
            grid_ticks: 10,
            threshold_points: 4.0,
        };
        let viewport = TimelineViewport::new(0, 1.0);

        let edge = cache.snap_drag_delta(&drag, 18, viewport, policy, TimelineModifiers::default());
        assert_eq!(edge.delta_ticks, 20);
        assert_eq!(edge.kind, TimelineSnapKind::ClipEdge);

        let grid_only = cache.snap_drag_delta(
            &drag,
            18,
            viewport,
            policy,
            TimelineModifiers {
                bypass_snap: false,
                grid_only: true,
            },
        );
        assert_eq!(grid_only.delta_ticks, 20);
        assert_eq!(grid_only.kind, TimelineSnapKind::Grid);

        let bypassed = cache.snap_drag_delta(
            &drag,
            18,
            viewport,
            policy,
            TimelineModifiers {
                bypass_snap: true,
                grid_only: false,
            },
        );
        assert_eq!(bypassed.delta_ticks, 18);
        assert_eq!(bypassed.kind, TimelineSnapKind::Bypassed);
    }

    #[test]
    fn ten_thousand_clip_pointer_frame_p95_is_below_budget() {
        let cache = TimelineGeometryCache::build(&fixture(100, 100));
        let viewport = TimelineViewport {
            start_tick: 4_800,
            ticks_per_point: 2.0,
            vertical_scroll_points: 34.0 * 40.0,
            track_height_points: 34.0,
        };
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(1_400.0, 900.0));
        let mut samples = Vec::with_capacity(240);
        for sample in 0..240 {
            let started = std::time::Instant::now();
            let geometry = cache.geometry(viewport, rect, 4_800 + sample);
            let _ = geometry.hit_test(pos2((sample % 1_400) as f32, 120.0));
            samples.push(started.elapsed());
        }
        samples.sort_unstable();
        let p95 = samples[samples.len() * 95 / 100];
        assert!(
            p95 < std::time::Duration::from_micros(16_700),
            "10k pointer frame p95 {p95:?} exceeded 16.7ms"
        );
    }
}
