use eframe::egui::{Rect, pos2, vec2};
use finui_primitives::{
    ContextMenuItem, HoverCardRootOptions, RovingFocusAction, RovingFocusKey,
    ScrollAreaRootOptions, context_menu_keyboard_open_output, context_menu_roving_focus_output,
    context_menu_sub_dismiss_output, primitive_hover_card_root_output,
    primitive_scroll_area_native_scroll_output,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridPrimitiveIntegrationScenarioOutput {
    pub header_context_menu_opens_from_keyboard: bool,
    pub header_context_menu_focuses_next_enabled_item: bool,
    pub cell_context_menu_dismiss_closes_root_outside_layers: bool,
    pub hover_card_uses_primitive_open_state: bool,
    pub scroll_area_preserves_native_scroll: bool,
}

impl GridPrimitiveIntegrationScenarioOutput {
    pub fn passed(&self) -> bool {
        self.header_context_menu_opens_from_keyboard
            && self.header_context_menu_focuses_next_enabled_item
            && self.cell_context_menu_dismiss_closes_root_outside_layers
            && self.hover_card_uses_primitive_open_state
            && self.scroll_area_preserves_native_scroll
    }
}

pub fn grid_primitive_integration_scenario_output() -> GridPrimitiveIntegrationScenarioOutput {
    let trigger_rect = Rect::from_min_size(pos2(12.0, 8.0), vec2(120.0, 24.0));
    let keyboard_open = context_menu_keyboard_open_output(trigger_rect, true, false, false);

    let header_items = [
        ContextMenuItem {
            value: "pin-left",
            label: "Pin left",
            enabled: true,
        },
        ContextMenuItem {
            value: "hide",
            label: "Hide column",
            enabled: false,
        },
        ContextMenuItem {
            value: "filter",
            label: "Filter sample",
            enabled: true,
        },
    ];
    let header_focus = context_menu_roving_focus_output(
        &header_items,
        Some(0),
        Some(RovingFocusKey::ArrowDown),
        true,
    );

    let dismiss = context_menu_sub_dismiss_output(
        Rect::from_min_size(pos2(0.0, 0.0), vec2(180.0, 120.0)),
        Rect::from_min_size(pos2(184.0, 0.0), vec2(140.0, 96.0)),
        pos2(420.0, 160.0),
    );
    let hover_card = primitive_hover_card_root_output(HoverCardRootOptions::default().open(true));
    let scroll = primitive_scroll_area_native_scroll_output(
        ScrollAreaRootOptions::default().max_height(208.0),
    );

    GridPrimitiveIntegrationScenarioOutput {
        header_context_menu_opens_from_keyboard: keyboard_open.open
            && keyboard_open.position == Some(trigger_rect.center())
            && keyboard_open.data_origin == Some("keyboard"),
        header_context_menu_focuses_next_enabled_item: header_focus.action
            == RovingFocusAction::Moved
            && header_focus.active_index == Some(2),
        cell_context_menu_dismiss_closes_root_outside_layers: dismiss.should_close_submenu
            && dismiss.should_close_root,
        hover_card_uses_primitive_open_state: hover_card.open,
        scroll_area_preserves_native_scroll: scroll.uses_native_scroll
            && (scroll.max_height - 208.0).abs() < f32::EPSILON,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_primitive_integration_scenario_matches_context_menu_hover_and_scroll_contracts() {
        let output = grid_primitive_integration_scenario_output();

        assert!(output.header_context_menu_opens_from_keyboard);
        assert!(output.header_context_menu_focuses_next_enabled_item);
        assert!(output.cell_context_menu_dismiss_closes_root_outside_layers);
        assert!(output.hover_card_uses_primitive_open_state);
        assert!(output.scroll_area_preserves_native_scroll);
        assert!(output.passed());
    }
}
