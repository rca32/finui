//! Feature-gated external-texture surface with caller-owned transform state.

#![cfg_attr(not(feature = "preview"), allow(dead_code, unused_imports))]

#[cfg(feature = "preview")]
mod preview {

    use egui::{
        Align2, Color32, FontId, Pos2, Rect, Sense, Stroke, StrokeKind, TextureId, Ui, Vec2, pos2,
        vec2,
    };
    use serde::{Deserialize, Serialize};

    pub const MEDIA_SURFACE_API_STABILITY: &str = "preview";
    const HANDLE_POINTS: f32 = 10.0;

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct MediaTransform {
        pub offset_points: Vec2,
        pub scale: f32,
    }

    impl Default for MediaTransform {
        fn default() -> Self {
            Self {
                offset_points: Vec2::ZERO,
                scale: 1.0,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct MediaSurfaceSnapshot {
        pub texture_id: TextureId,
        pub source_size: [u32; 2],
        pub transform: MediaTransform,
        pub selected: bool,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MediaTransformKind {
        Move,
        Scale,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct MediaTransformSession {
        pub kind: MediaTransformKind,
        pub origin_pointer: Pos2,
        pub origin_transform: MediaTransform,
        pub delta_points: Vec2,
        pub scale_multiplier: f32,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum MediaSurfaceAction {
        Select,
        BeginTransform {
            kind: MediaTransformKind,
            pointer: Pos2,
            origin: MediaTransform,
        },
        UpdateTransform {
            kind: MediaTransformKind,
            delta_points: Vec2,
            scale_multiplier: f32,
        },
        CommitTransform {
            kind: MediaTransformKind,
            delta_points: Vec2,
            scale_multiplier: f32,
        },
        CancelTransform {
            kind: MediaTransformKind,
        },
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct MediaSurfaceInteractionState {
        pub transform: Option<MediaTransformSession>,
    }

    impl MediaSurfaceInteractionState {
        /// Updates interaction state only; the caller owns the media transform.
        pub fn apply(&mut self, action: &MediaSurfaceAction) -> bool {
            match action {
                MediaSurfaceAction::BeginTransform {
                    kind,
                    pointer,
                    origin,
                } => {
                    self.transform = Some(MediaTransformSession {
                        kind: *kind,
                        origin_pointer: *pointer,
                        origin_transform: *origin,
                        delta_points: Vec2::ZERO,
                        scale_multiplier: 1.0,
                    });
                    true
                }
                MediaSurfaceAction::UpdateTransform {
                    kind,
                    delta_points,
                    scale_multiplier,
                } => {
                    let Some(session) = self.transform.as_mut() else {
                        return false;
                    };
                    if session.kind != *kind {
                        return false;
                    }
                    session.delta_points = *delta_points;
                    session.scale_multiplier = *scale_multiplier;
                    true
                }
                MediaSurfaceAction::CommitTransform { kind, .. }
                | MediaSurfaceAction::CancelTransform { kind } => {
                    if self
                        .transform
                        .as_ref()
                        .is_some_and(|session| session.kind == *kind)
                    {
                        self.transform = None;
                        true
                    } else {
                        false
                    }
                }
                MediaSurfaceAction::Select => false,
            }
        }

        pub fn apply_all<'a>(&mut self, actions: impl IntoIterator<Item = &'a MediaSurfaceAction>) {
            for action in actions {
                self.apply(action);
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct MediaSurfaceGeometry {
        pub bounds: Rect,
        pub fitted_rect: Rect,
        pub transformed_rect: Rect,
        pub scale_handles: [Rect; 4],
    }

    pub fn media_surface_geometry(
        bounds: Rect,
        source_size: [u32; 2],
        transform: MediaTransform,
    ) -> MediaSurfaceGeometry {
        let fitted_rect = aspect_fit_rect(bounds, source_size);
        let scale = transform.scale.max(0.01);
        let transformed_size = fitted_rect.size() * scale;
        let transformed_rect = Rect::from_center_size(
            fitted_rect.center() + transform.offset_points,
            transformed_size,
        );
        let handle =
            |center: Pos2| Rect::from_center_size(center, vec2(HANDLE_POINTS, HANDLE_POINTS));
        MediaSurfaceGeometry {
            bounds,
            fitted_rect,
            transformed_rect,
            scale_handles: [
                handle(transformed_rect.left_top()),
                handle(transformed_rect.right_top()),
                handle(transformed_rect.right_bottom()),
                handle(transformed_rect.left_bottom()),
            ],
        }
    }

    pub fn aspect_fit_rect(bounds: Rect, source_size: [u32; 2]) -> Rect {
        if source_size[0] == 0 || source_size[1] == 0 || !bounds.is_positive() {
            return Rect::from_center_size(bounds.center(), Vec2::ZERO);
        }
        let source_aspect = source_size[0] as f32 / source_size[1] as f32;
        let bounds_aspect = bounds.width() / bounds.height().max(f32::EPSILON);
        let size = if source_aspect > bounds_aspect {
            vec2(bounds.width(), bounds.width() / source_aspect)
        } else {
            vec2(bounds.height() * source_aspect, bounds.height())
        };
        Rect::from_center_size(bounds.center(), size)
    }

    #[derive(Clone, Debug)]
    pub struct MediaSurfaceOutput {
        pub actions: Vec<MediaSurfaceAction>,
        pub geometry: MediaSurfaceGeometry,
        pub receipt: MediaSurfaceUxReceipt,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct MediaSurfaceUxReceipt {
        pub primitive: String,
        pub texture_id: String,
        pub source_size: [u32; 2],
        pub surface_rect: [f32; 4],
        pub selected: bool,
        pub overlay_handle_count: usize,
        pub transform_phase: String,
        pub accessibility_label: String,
        pub keyboard_selection_enabled: bool,
    }

    pub fn media_surface_receipt_json(receipt: &MediaSurfaceUxReceipt) -> String {
        serde_json::to_string_pretty(receipt).expect("media surface receipt is serializable")
    }

    pub fn show_media_surface(
        ui: &mut Ui,
        snapshot: &MediaSurfaceSnapshot,
        interaction: &MediaSurfaceInteractionState,
    ) -> MediaSurfaceOutput {
        let bounds = ui.available_rect_before_wrap();
        let root_response = ui.allocate_rect(bounds, Sense::hover());
        root_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Other, true, "Media preview")
        });
        let projected_transform = projected_transform(snapshot.transform, interaction);
        let geometry = media_surface_geometry(bounds, snapshot.source_size, projected_transform);
        let painter = ui.painter().clone();
        let visuals = ui.visuals().clone();

        painter.rect_filled(bounds, 0.0, Color32::BLACK);
        painter.image(
            snapshot.texture_id,
            geometry.transformed_rect,
            Rect::from_min_max(Pos2::ZERO, pos2(1.0, 1.0)),
            Color32::WHITE,
        );

        let mut actions = Vec::new();
        if snapshot.selected {
            painter.rect_stroke(
                geometry.transformed_rect,
                0.0,
                Stroke::new(1.0, visuals.selection.bg_fill),
                StrokeKind::Inside,
            );
            for (index, handle) in geometry.scale_handles.iter().enumerate() {
                painter.rect_filled(*handle, 1.0, visuals.selection.bg_fill);
                if interaction.transform.is_none() {
                    let response = ui.interact(
                        *handle,
                        ui.make_persistent_id(("finui-media-scale", index)),
                        Sense::drag(),
                    );
                    if response.drag_started()
                        && let Some(pointer) = response.interact_pointer_pos()
                    {
                        actions.push(MediaSurfaceAction::BeginTransform {
                            kind: MediaTransformKind::Scale,
                            pointer,
                            origin: snapshot.transform,
                        });
                    }
                }
            }
        }

        if interaction.transform.is_none() {
            let response = ui.interact(
                geometry.transformed_rect,
                ui.make_persistent_id("finui-media-surface-body"),
                Sense::click_and_drag(),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Media canvas")
            });
            if response.clicked() && !snapshot.selected {
                response.request_focus();
                actions.push(MediaSurfaceAction::Select);
            }
            if response.has_focus()
                && !snapshot.selected
                && ui.input(|input| {
                    input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                })
            {
                actions.push(MediaSurfaceAction::Select);
            }
            if response.drag_started()
                && let Some(pointer) = response.interact_pointer_pos()
            {
                actions.push(MediaSurfaceAction::BeginTransform {
                    kind: MediaTransformKind::Move,
                    pointer,
                    origin: snapshot.transform,
                });
            }
        }

        if let Some(session) = interaction.transform.as_ref() {
            if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
                actions.push(MediaSurfaceAction::CancelTransform { kind: session.kind });
            } else if ui.input(|input| input.pointer.any_released()) {
                actions.push(MediaSurfaceAction::CommitTransform {
                    kind: session.kind,
                    delta_points: session.delta_points,
                    scale_multiplier: session.scale_multiplier,
                });
            } else if let Some(pointer) = ui.input(|input| input.pointer.interact_pos()) {
                let delta = pointer - session.origin_pointer;
                let scale_multiplier = match session.kind {
                    MediaTransformKind::Move => 1.0,
                    MediaTransformKind::Scale => (1.0 + (delta.x + delta.y) / 300.0).max(0.05),
                };
                actions.push(MediaSurfaceAction::UpdateTransform {
                    kind: session.kind,
                    delta_points: delta,
                    scale_multiplier,
                });
            }
        }

        painter.text(
            bounds.left_top() + vec2(8.0, 8.0),
            Align2::LEFT_TOP,
            format!("{}x{}", snapshot.source_size[0], snapshot.source_size[1]),
            FontId::monospace(10.0),
            Color32::LIGHT_GRAY,
        );

        let receipt = MediaSurfaceUxReceipt {
            primitive: "media_surface".to_owned(),
            texture_id: format!("{:?}", snapshot.texture_id),
            source_size: snapshot.source_size,
            surface_rect: [
                geometry.transformed_rect.min.x,
                geometry.transformed_rect.min.y,
                geometry.transformed_rect.max.x,
                geometry.transformed_rect.max.y,
            ],
            selected: snapshot.selected,
            overlay_handle_count: usize::from(snapshot.selected) * geometry.scale_handles.len(),
            transform_phase: if interaction.transform.is_some() {
                "active".to_owned()
            } else {
                "idle".to_owned()
            },
            accessibility_label: "Media preview".to_owned(),
            keyboard_selection_enabled: true,
        };
        MediaSurfaceOutput {
            actions,
            geometry,
            receipt,
        }
    }

    fn projected_transform(
        transform: MediaTransform,
        interaction: &MediaSurfaceInteractionState,
    ) -> MediaTransform {
        let Some(session) = interaction.transform.as_ref() else {
            return transform;
        };
        match session.kind {
            MediaTransformKind::Move => MediaTransform {
                offset_points: session.origin_transform.offset_points + session.delta_points,
                ..session.origin_transform
            },
            MediaTransformKind::Scale => MediaTransform {
                scale: (session.origin_transform.scale * session.scale_multiplier).max(0.01),
                ..session.origin_transform
            },
        }
    }
}

#[cfg(feature = "preview")]
pub use preview::*;

pub const MEDIA_SURFACE_PREVIEW_API: &[&str] = &[
    "MediaSurfaceSnapshot",
    "MediaTransform",
    "MediaSurfaceAction",
    "MediaSurfaceInteractionState",
    "MediaSurfaceOutput",
    "MediaSurfaceUxReceipt",
    "media_surface_receipt_json",
    "show_media_surface",
];

pub const MEDIA_SURFACE_EXPERIMENTAL_API: &[&str] = &[
    "MediaTextureHandle",
    "MediaFrameState",
    "MediaFrameStateReceipt",
    "show_media_frame_state",
];

/// Application-owned renderer handle. FinUI never dereferences or frees it.
#[cfg(feature = "experimental")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MediaTextureHandle {
    pub texture_id: egui::TextureId,
    pub generation: u64,
    pub pixel_size: [u32; 2],
    pub sample_aspect_ratio: [u32; 2],
}

/// Experimental product-window state surface for empty/loading/offline behavior.
#[cfg(feature = "experimental")]
#[derive(Clone, Debug, PartialEq)]
pub enum MediaFrameState {
    Empty,
    Loading,
    Ready(MediaTextureHandle),
    Offline { reason: String },
    Failed { message: String },
}

#[cfg(feature = "experimental")]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MediaFrameStateReceipt {
    pub state: String,
    pub accessibility_label: String,
    pub retry_available: bool,
}

/// Renders non-ready product states without dereferencing a renderer handle.
#[cfg(feature = "experimental")]
pub fn show_media_frame_state(
    ui: &mut egui::Ui,
    state: &MediaFrameState,
) -> MediaFrameStateReceipt {
    let (name, label, retry) = match state {
        MediaFrameState::Empty => ("empty", "No media selected", false),
        MediaFrameState::Loading => ("loading", "Loading media preview", false),
        MediaFrameState::Ready(handle) => (
            "ready",
            if handle.pixel_size.contains(&0) {
                "Media preview has invalid dimensions"
            } else {
                "Media preview ready"
            },
            false,
        ),
        MediaFrameState::Offline { .. } => ("offline", "Media is offline", true),
        MediaFrameState::Failed { .. } => ("failed", "Media preview failed", true),
    };
    let response = ui.label(label);
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, label));
    MediaFrameStateReceipt {
        state: name.to_owned(),
        accessibility_label: label.to_owned(),
        retry_available: retry,
    }
}

#[cfg(test)]
mod tests {
    use egui::{Pos2, Rect, TextureId, pos2, vec2};

    use super::*;

    #[test]
    fn aspect_fit_handles_landscape_and_portrait_sources() {
        let bounds = Rect::from_min_size(Pos2::ZERO, vec2(400.0, 400.0));
        assert_eq!(
            aspect_fit_rect(bounds, [1920, 1080]).size(),
            vec2(400.0, 225.0)
        );
        assert_eq!(
            aspect_fit_rect(bounds, [1080, 1920]).size(),
            vec2(225.0, 400.0)
        );
    }

    #[test]
    fn selected_surface_has_four_scale_handles() {
        let geometry = media_surface_geometry(
            Rect::from_min_size(Pos2::ZERO, vec2(800.0, 600.0)),
            [1920, 1080],
            MediaTransform::default(),
        );
        assert_eq!(geometry.scale_handles.len(), 4);
        assert!(geometry.scale_handles.iter().all(Rect::is_positive));
    }

    #[test]
    fn transform_begin_update_commit_and_cancel_are_transactional() {
        let begin = MediaSurfaceAction::BeginTransform {
            kind: MediaTransformKind::Move,
            pointer: pos2(20.0, 20.0),
            origin: MediaTransform::default(),
        };
        let mut state = MediaSurfaceInteractionState::default();
        assert!(state.apply(&begin));
        assert!(state.apply(&MediaSurfaceAction::UpdateTransform {
            kind: MediaTransformKind::Move,
            delta_points: vec2(10.0, 5.0),
            scale_multiplier: 1.0,
        }));
        assert_eq!(
            state.transform.as_ref().unwrap().delta_points,
            vec2(10.0, 5.0)
        );
        assert!(state.apply(&MediaSurfaceAction::CommitTransform {
            kind: MediaTransformKind::Move,
            delta_points: vec2(10.0, 5.0),
            scale_multiplier: 1.0,
        }));
        assert!(state.transform.is_none());

        state.apply(&begin);
        assert!(state.apply(&MediaSurfaceAction::CancelTransform {
            kind: MediaTransformKind::Move,
        }));
        assert!(state.transform.is_none());
    }

    #[test]
    fn receipt_is_agent_readable_without_pixel_inspection() {
        egui::__run_test_ui(|ui| {
            ui.set_min_size(vec2(640.0, 360.0));
            let snapshot = MediaSurfaceSnapshot {
                texture_id: TextureId::User(42),
                source_size: [1920, 1080],
                transform: MediaTransform::default(),
                selected: true,
            };
            let output =
                show_media_surface(ui, &snapshot, &MediaSurfaceInteractionState::default());
            assert_eq!(output.receipt.overlay_handle_count, 4);
            assert!(media_surface_receipt_json(&output.receipt).contains("media_surface"));
            assert_eq!(snapshot.transform, MediaTransform::default());
        });
    }

    #[test]
    fn product_window_states_have_stable_accessibility_receipts() {
        let context = egui::Context::default();
        let states = [
            MediaFrameState::Empty,
            MediaFrameState::Loading,
            MediaFrameState::Offline {
                reason: "fixture missing".to_owned(),
            },
            MediaFrameState::Failed {
                message: "decode failed".to_owned(),
            },
        ];
        let mut receipts = Vec::new();
        let _ = context.run_ui(Default::default(), |ui| {
            receipts.extend(states.iter().map(|state| show_media_frame_state(ui, state)));
        });

        assert_eq!(
            receipts
                .iter()
                .map(|receipt| receipt.state.as_str())
                .collect::<Vec<_>>(),
            ["empty", "loading", "offline", "failed"]
        );
        assert!(
            receipts
                .iter()
                .all(|receipt| !receipt.accessibility_label.is_empty())
        );
        assert!(!receipts[0].retry_available);
        assert!(receipts[2].retry_available);
        assert!(receipts[3].retry_available);
    }
}
