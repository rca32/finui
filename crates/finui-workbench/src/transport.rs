use egui::{Context, Key};
use serde::{Deserialize, Serialize};

/// Renderer-neutral logical transport commands emitted from keyboard input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardTransportAction {
    Play,
    Pause,
    StepFrames(i32),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyboardTransportReceipt {
    pub primitive: String,
    pub focused: bool,
    pub logical_actions: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardTransportOutput {
    pub actions: Vec<KeyboardTransportAction>,
    pub receipt: KeyboardTransportReceipt,
}

/// Converts keys pressed during this egui frame into caller-owned transport actions.
///
/// Space maps to play/pause. Left and right arrow map to one-frame exact steps.
/// No action is emitted unless the caller's transport surface owns keyboard focus.
pub fn keyboard_transport_actions(
    context: &Context,
    focused: bool,
    is_playing: bool,
) -> KeyboardTransportOutput {
    let pressed = context.input(|input| {
        [Key::Space, Key::ArrowLeft, Key::ArrowRight]
            .into_iter()
            .filter(|key| input.key_pressed(*key))
            .collect::<Vec<_>>()
    });
    keyboard_transport_actions_from_pressed(&pressed, focused, is_playing)
}

/// Pure event adapter used by native receipts and non-egui input harnesses.
pub fn keyboard_transport_actions_from_pressed(
    pressed: &[Key],
    focused: bool,
    is_playing: bool,
) -> KeyboardTransportOutput {
    let mut actions = Vec::new();
    if focused {
        if pressed.contains(&Key::Space) {
            actions.push(if is_playing {
                KeyboardTransportAction::Pause
            } else {
                KeyboardTransportAction::Play
            });
        }
        if pressed.contains(&Key::ArrowLeft) {
            actions.push(KeyboardTransportAction::StepFrames(-1));
        }
        if pressed.contains(&Key::ArrowRight) {
            actions.push(KeyboardTransportAction::StepFrames(1));
        }
    }
    let logical_actions = actions
        .iter()
        .map(|action| match action {
            KeyboardTransportAction::Play => "play".to_owned(),
            KeyboardTransportAction::Pause => "pause".to_owned(),
            KeyboardTransportAction::StepFrames(delta) => format!("step_frames:{delta}"),
        })
        .collect();
    KeyboardTransportOutput {
        actions,
        receipt: KeyboardTransportReceipt {
            primitive: "keyboard_transport".to_owned(),
            focused,
            logical_actions,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focused_keyboard_transport_emits_play_pause_and_exact_frame_steps() {
        let paused = keyboard_transport_actions_from_pressed(
            &[Key::Space, Key::ArrowLeft, Key::ArrowRight],
            true,
            false,
        );
        assert_eq!(
            paused.actions,
            [
                KeyboardTransportAction::Play,
                KeyboardTransportAction::StepFrames(-1),
                KeyboardTransportAction::StepFrames(1),
            ]
        );
        assert_eq!(
            paused.receipt.logical_actions,
            ["play", "step_frames:-1", "step_frames:1"]
        );

        let playing = keyboard_transport_actions_from_pressed(&[Key::Space], true, true);
        assert_eq!(playing.actions, [KeyboardTransportAction::Pause]);
    }

    #[test]
    fn unfocused_transport_ignores_keyboard_input() {
        let output =
            keyboard_transport_actions_from_pressed(&[Key::Space, Key::ArrowRight], false, false);
        assert!(output.actions.is_empty());
        assert!(!output.receipt.focused);
    }
}
