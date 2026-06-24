use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RovingFocusAction {
    None,
    Moved,
    Activate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RovingFocusState {
    pub active_index: Option<usize>,
}

impl RovingFocusState {
    pub fn new() -> Self {
        Self { active_index: None }
    }

    pub fn handle_keyboard(
        &mut self,
        input: &egui::InputState,
        enabled: &[bool],
    ) -> RovingFocusAction {
        if enabled.is_empty() || !enabled.iter().any(|enabled| *enabled) {
            self.active_index = None;
            return RovingFocusAction::None;
        }
        if input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space) {
            return RovingFocusAction::Activate;
        }
        if input.key_pressed(egui::Key::Home) {
            self.active_index = first_enabled(enabled);
            return RovingFocusAction::Moved;
        }
        if input.key_pressed(egui::Key::End) {
            self.active_index = last_enabled(enabled);
            return RovingFocusAction::Moved;
        }
        if input.key_pressed(egui::Key::ArrowDown) || input.key_pressed(egui::Key::ArrowRight) {
            self.active_index = next_enabled(enabled, self.active_index, 1);
            return RovingFocusAction::Moved;
        }
        if input.key_pressed(egui::Key::ArrowUp) || input.key_pressed(egui::Key::ArrowLeft) {
            self.active_index = next_enabled(enabled, self.active_index, -1);
            return RovingFocusAction::Moved;
        }
        RovingFocusAction::None
    }
}

fn first_enabled(enabled: &[bool]) -> Option<usize> {
    enabled.iter().position(|enabled| *enabled)
}

fn last_enabled(enabled: &[bool]) -> Option<usize> {
    enabled.iter().rposition(|enabled| *enabled)
}

fn next_enabled(enabled: &[bool], current: Option<usize>, direction: isize) -> Option<usize> {
    let len = enabled.len() as isize;
    let start = current
        .map(|index| index as isize)
        .unwrap_or(if direction >= 0 { -1 } else { len });
    for step in 1..=enabled.len() {
        let next = (start + direction * step as isize).rem_euclid(len) as usize;
        if enabled[next] {
            return Some(next);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_enabled_skips_disabled_items_and_wraps() {
        let enabled = [true, false, true];

        assert_eq!(next_enabled(&enabled, Some(0), 1), Some(2));
        assert_eq!(next_enabled(&enabled, Some(2), 1), Some(0));
        assert_eq!(next_enabled(&enabled, Some(0), -1), Some(2));
    }
}
