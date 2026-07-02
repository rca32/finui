use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RovingFocusAction {
    None,
    Moved,
    Activate,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RovingFocusOrientation {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RovingFocusKey {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    Enter,
    Space,
    Escape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RovingFocusOptions {
    pub orientation: RovingFocusOrientation,
    pub loop_focus: bool,
    pub rtl: bool,
}

impl Default for RovingFocusOptions {
    fn default() -> Self {
        Self {
            orientation: RovingFocusOrientation::Vertical,
            loop_focus: true,
            rtl: false,
        }
    }
}

impl RovingFocusOptions {
    pub fn orientation(mut self, orientation: RovingFocusOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.loop_focus = loop_focus;
        self
    }

    pub fn rtl(mut self, rtl: bool) -> Self {
        self.rtl = rtl;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RovingFocusOutput {
    pub action: RovingFocusAction,
    pub active_index: Option<usize>,
    pub previous_index: Option<usize>,
    pub item_tab_indices: Vec<i8>,
    pub item_highlighted: Vec<bool>,
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

pub fn primitive_roving_focus_output(
    enabled: &[bool],
    current: Option<usize>,
    key: Option<RovingFocusKey>,
    options: RovingFocusOptions,
) -> RovingFocusOutput {
    let active_index = roving_focus_target_index(enabled, current, key, options);
    let action = match key {
        Some(RovingFocusKey::Enter | RovingFocusKey::Space)
            if current.is_some_and(|index| enabled.get(index).copied().unwrap_or(false)) =>
        {
            RovingFocusAction::Activate
        }
        Some(RovingFocusKey::Enter | RovingFocusKey::Space) => RovingFocusAction::None,
        Some(RovingFocusKey::Escape) => RovingFocusAction::Close,
        Some(_) if active_index != current => RovingFocusAction::Moved,
        _ => RovingFocusAction::None,
    };
    RovingFocusOutput {
        action,
        active_index,
        previous_index: current,
        item_tab_indices: enabled
            .iter()
            .enumerate()
            .map(|(index, item_enabled)| {
                if *item_enabled && Some(index) == active_index {
                    0
                } else {
                    -1
                }
            })
            .collect(),
        item_highlighted: enabled
            .iter()
            .enumerate()
            .map(|(index, item_enabled)| *item_enabled && Some(index) == active_index)
            .collect(),
    }
}

pub fn roving_focus_target_index(
    enabled: &[bool],
    current: Option<usize>,
    key: Option<RovingFocusKey>,
    options: RovingFocusOptions,
) -> Option<usize> {
    if enabled.is_empty() || !enabled.iter().any(|enabled| *enabled) {
        return None;
    }
    let current = current.filter(|index| enabled.get(*index).copied().unwrap_or(false));
    match key {
        Some(RovingFocusKey::Home) => first_enabled(enabled),
        Some(RovingFocusKey::End) => last_enabled(enabled),
        Some(RovingFocusKey::ArrowDown)
            if matches!(
                options.orientation,
                RovingFocusOrientation::Vertical | RovingFocusOrientation::Both
            ) =>
        {
            next_enabled_with_loop(enabled, current, 1, options.loop_focus)
        }
        Some(RovingFocusKey::ArrowUp)
            if matches!(
                options.orientation,
                RovingFocusOrientation::Vertical | RovingFocusOrientation::Both
            ) =>
        {
            next_enabled_with_loop(enabled, current, -1, options.loop_focus)
        }
        Some(RovingFocusKey::ArrowRight)
            if matches!(
                options.orientation,
                RovingFocusOrientation::Horizontal | RovingFocusOrientation::Both
            ) =>
        {
            let direction = if options.rtl { -1 } else { 1 };
            next_enabled_with_loop(enabled, current, direction, options.loop_focus)
        }
        Some(RovingFocusKey::ArrowLeft)
            if matches!(
                options.orientation,
                RovingFocusOrientation::Horizontal | RovingFocusOrientation::Both
            ) =>
        {
            let direction = if options.rtl { 1 } else { -1 };
            next_enabled_with_loop(enabled, current, direction, options.loop_focus)
        }
        Some(RovingFocusKey::Enter | RovingFocusKey::Space | RovingFocusKey::Escape) => current,
        Some(_) | None => current.or_else(|| first_enabled(enabled)),
    }
}

fn first_enabled(enabled: &[bool]) -> Option<usize> {
    enabled.iter().position(|enabled| *enabled)
}

fn last_enabled(enabled: &[bool]) -> Option<usize> {
    enabled.iter().rposition(|enabled| *enabled)
}

fn next_enabled(enabled: &[bool], current: Option<usize>, direction: isize) -> Option<usize> {
    next_enabled_with_loop(enabled, current, direction, true)
}

fn next_enabled_with_loop(
    enabled: &[bool],
    current: Option<usize>,
    direction: isize,
    loop_focus: bool,
) -> Option<usize> {
    let len = enabled.len() as isize;
    let start = current
        .map(|index| index as isize)
        .unwrap_or(if direction >= 0 { -1 } else { len });
    for step in 1..=enabled.len() {
        let raw_next = start + direction * step as isize;
        if !loop_focus && !(0..len).contains(&raw_next) {
            return current.or_else(|| first_enabled(enabled));
        }
        let next = raw_next.rem_euclid(len) as usize;
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

    #[test]
    fn roving_focus_output_tracks_tab_indices_and_highlight() {
        let enabled = [true, false, true];
        let output = primitive_roving_focus_output(
            &enabled,
            Some(0),
            Some(RovingFocusKey::ArrowDown),
            RovingFocusOptions::default(),
        );

        assert_eq!(output.action, RovingFocusAction::Moved);
        assert_eq!(output.active_index, Some(2));
        assert_eq!(output.previous_index, Some(0));
        assert_eq!(output.item_tab_indices, vec![-1, -1, 0]);
        assert_eq!(output.item_highlighted, vec![false, false, true]);
    }

    #[test]
    fn roving_focus_respects_loop_focus_false() {
        let enabled = [true, false, true];
        let output = primitive_roving_focus_output(
            &enabled,
            Some(2),
            Some(RovingFocusKey::ArrowDown),
            RovingFocusOptions::default().loop_focus(false),
        );

        assert_eq!(output.action, RovingFocusAction::None);
        assert_eq!(output.active_index, Some(2));
        assert_eq!(output.item_tab_indices, vec![-1, -1, 0]);
    }

    #[test]
    fn horizontal_roving_focus_uses_rtl_arrow_direction() {
        let enabled = [true, true, true];
        let output = primitive_roving_focus_output(
            &enabled,
            Some(1),
            Some(RovingFocusKey::ArrowRight),
            RovingFocusOptions::default()
                .orientation(RovingFocusOrientation::Horizontal)
                .rtl(true),
        );

        assert_eq!(output.active_index, Some(0));
    }

    #[test]
    fn roving_focus_activation_requires_enabled_current_item() {
        let enabled = [true, false, true];

        let disabled = primitive_roving_focus_output(
            &enabled,
            Some(1),
            Some(RovingFocusKey::Enter),
            RovingFocusOptions::default(),
        );
        let enabled = primitive_roving_focus_output(
            &enabled,
            Some(2),
            Some(RovingFocusKey::Space),
            RovingFocusOptions::default(),
        );

        assert_eq!(disabled.action, RovingFocusAction::None);
        assert_eq!(enabled.action, RovingFocusAction::Activate);
    }

    #[test]
    fn roving_focus_escape_requests_close_without_moving_focus() {
        let enabled = [true, false, true];
        let output = primitive_roving_focus_output(
            &enabled,
            Some(2),
            Some(RovingFocusKey::Escape),
            RovingFocusOptions::default(),
        );

        assert_eq!(output.action, RovingFocusAction::Close);
        assert_eq!(output.active_index, Some(2));
    }
}
