use eframe::egui::{self, Event, Key, Modifiers, PointerButton, Pos2, RawInput, Rect, Vec2};

use crate::{DismissPolicy, dismiss_event_for_interaction};

use super::{
    PrimitiveControllableScope, PrimitiveFocusCloseReason, PrimitiveFocusScope,
    PrimitiveFocusTarget, RovingFocusAction, RovingFocusKey, RovingFocusOptions,
    primitive_controllable_state_output, primitive_focus_manager_output,
    primitive_roving_focus_output,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimitiveRuntimeScenarioOutput {
    pub pointer_down_outside: bool,
    pub keyboard_moved_focus: bool,
    pub focus_restored_to_trigger: bool,
    pub dismiss_requested: bool,
    pub controlled_emits_without_internal_update: bool,
    pub uncontrolled_updates_internal_state: bool,
}

impl PrimitiveRuntimeScenarioOutput {
    pub fn passed(self) -> bool {
        self.pointer_down_outside
            && self.keyboard_moved_focus
            && self.focus_restored_to_trigger
            && self.dismiss_requested
            && self.controlled_emits_without_internal_update
            && self.uncontrolled_updates_internal_state
    }
}

pub fn primitive_runtime_scenario_output() -> PrimitiveRuntimeScenarioOutput {
    let pointer_pos = egui_runtime_pointer_down_pos();
    let dismiss = pointer_pos.and_then(|pos| {
        dismiss_event_for_interaction(
            DismissPolicy::OutsideClickAndEscape,
            Rect::from_min_size(Pos2::new(20.0, 20.0), Vec2::new(120.0, 80.0)),
            Some(Rect::from_min_size(
                Pos2::new(20.0, 0.0),
                Vec2::new(120.0, 18.0),
            )),
            false,
            Some((PointerButton::Primary, pos)),
        )
    });
    let keyboard_output = if egui_runtime_key_pressed(Key::ArrowDown) {
        primitive_roving_focus_output(
            &[true, false, true],
            Some(0),
            Some(RovingFocusKey::ArrowDown),
            RovingFocusOptions::default(),
        )
    } else {
        primitive_roving_focus_output(
            &[true, false, true],
            Some(0),
            None,
            RovingFocusOptions::default(),
        )
    };
    let focus = primitive_focus_manager_output(
        super::PrimitiveFocusManagerOptions::new(PrimitiveFocusScope::Dialog, true)
            .modal(true)
            .has_initial_focus(true),
        Some(PrimitiveFocusCloseReason::Escape),
    );
    let controlled = primitive_controllable_state_output(
        PrimitiveControllableScope::DialogOpen,
        Some(false),
        Some(false),
        false,
        true,
    );
    let uncontrolled = primitive_controllable_state_output(
        PrimitiveControllableScope::SelectValue,
        None,
        Some("1D"),
        "1D",
        "1W",
    );

    PrimitiveRuntimeScenarioOutput {
        pointer_down_outside: dismiss.is_some_and(|event| event.should_close()),
        keyboard_moved_focus: keyboard_output.action == RovingFocusAction::Moved
            && keyboard_output.active_index == Some(2),
        focus_restored_to_trigger: focus.restore_focus_target
            == Some(PrimitiveFocusTarget::Trigger),
        dismiss_requested: focus.close_requested,
        controlled_emits_without_internal_update: controlled.should_emit_change
            && !controlled.should_update_internal,
        uncontrolled_updates_internal_state: uncontrolled.should_emit_change
            && uncontrolled.should_update_internal,
    }
}

fn egui_runtime_key_pressed(key: Key) -> bool {
    let ctx = egui::Context::default();
    let mut pressed = false;
    let _ = ctx.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0))),
            events: vec![Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Modifiers::default(),
            }],
            ..Default::default()
        },
        |ui| {
            pressed = ui.input(|input| input.key_pressed(key));
        },
    );
    pressed
}

fn egui_runtime_pointer_down_pos() -> Option<Pos2> {
    let ctx = egui::Context::default();
    let pos = Pos2::new(220.0, 160.0);
    let mut pointer_pos = None;
    let _ = ctx.run_ui(
        RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0))),
            events: vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: Modifiers::default(),
                },
            ],
            ..Default::default()
        },
        |ui| {
            pointer_pos = ui.input(|input| input.pointer.interact_pos());
        },
    );
    pointer_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_runtime_scenario_verifies_pointer_keyboard_focus_dismiss_and_state() {
        let output = primitive_runtime_scenario_output();

        assert!(output.pointer_down_outside);
        assert!(output.keyboard_moved_focus);
        assert!(output.focus_restored_to_trigger);
        assert!(output.dismiss_requested);
        assert!(output.controlled_emits_without_internal_update);
        assert!(output.uncontrolled_updates_internal_state);
        assert!(output.passed());
    }
}
