//! Generic timeline audio controls with caller-owned state and typed actions.

use egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct AudioTrackSnapshot {
    pub id: String,
    pub muted: bool,
    pub soloed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioControlsSnapshot {
    pub master_volume: f64,
    pub tracks: Vec<AudioTrackSnapshot>,
    pub device_state: String,
    pub retry_enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioControlsAction {
    SetMasterVolume(f64),
    SetTrackMuted { track_id: String, muted: bool },
    SetTrackSoloed { track_id: String, soloed: bool },
    RetryDevice,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioControlsReceipt {
    pub primitive: String,
    pub track_count: usize,
    pub device_state: String,
    pub retry_enabled: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AudioControlsOutput {
    pub actions: Vec<AudioControlsAction>,
    pub receipt: AudioControlsReceipt,
}

pub fn show_audio_controls(ui: &mut Ui, snapshot: &AudioControlsSnapshot) -> AudioControlsOutput {
    let mut actions = Vec::new();
    ui.group(|ui| {
        ui.label("Audio");
        let mut master_volume = snapshot.master_volume;
        let response = ui.add(egui::Slider::new(&mut master_volume, 0.0..=1.0).text("Volume"));
        if response.drag_stopped() || (response.changed() && !response.dragged()) {
            actions.push(AudioControlsAction::SetMasterVolume(master_volume));
        }
        for track in &snapshot.tracks {
            ui.horizontal(|ui| {
                ui.label(&track.id);
                let mut muted = track.muted;
                if ui.checkbox(&mut muted, "Mute").changed() {
                    actions.push(AudioControlsAction::SetTrackMuted {
                        track_id: track.id.clone(),
                        muted,
                    });
                }
                let mut soloed = track.soloed;
                if ui.checkbox(&mut soloed, "Solo").changed() {
                    actions.push(AudioControlsAction::SetTrackSoloed {
                        track_id: track.id.clone(),
                        soloed,
                    });
                }
            });
        }
        ui.label(format!("Device: {}", snapshot.device_state));
        if ui
            .add_enabled(
                snapshot.retry_enabled,
                egui::Button::new("Retry Audio Device"),
            )
            .clicked()
        {
            actions.push(AudioControlsAction::RetryDevice);
        }
    });
    AudioControlsOutput {
        actions,
        receipt: AudioControlsReceipt {
            primitive: "finui.audio-controls".to_owned(),
            track_count: snapshot.tracks.len(),
            device_state: snapshot.device_state.clone(),
            retry_enabled: snapshot.retry_enabled,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_is_domain_neutral_and_stable() {
        let context = egui::Context::default();
        let mut output = None;
        let _ = context.run_ui(Default::default(), |ui| {
            output = Some(show_audio_controls(
                ui,
                &AudioControlsSnapshot {
                    master_volume: 0.8,
                    tracks: vec![AudioTrackSnapshot {
                        id: "dialogue".to_owned(),
                        muted: false,
                        soloed: true,
                    }],
                    device_state: "ready".to_owned(),
                    retry_enabled: false,
                },
            ));
        });
        let output = output.expect("audio control output");
        assert_eq!(output.receipt.primitive, "finui.audio-controls");
        assert_eq!(output.receipt.track_count, 1);
    }
}
