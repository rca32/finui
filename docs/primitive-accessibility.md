# Primitive Accessibility

Finui does not expose browser DOM nodes. Accessibility contracts are represented
as option/output structs, ids, labels, roles, focus targets, data-state fields,
and JSON snapshots that an egui bridge or agent can read.

| Primitive | Web ARIA expectation | Finui bridge contract | Verification |
| --- | --- | --- | --- |
| Dialog | `role=dialog`, labelled by title, described by optional description, modal focus management | `DialogAnnounceOutput`, `PrimitiveFocusManagerOutput`, `PrimitiveModalInertOutput` | `dialog_announce_output_requires_title_and_allows_description_omission`, `focus_manager_targets_dialog_initial_focus_and_restores_trigger_on_escape` |
| AlertDialog | `role=alertdialog`, forced modal, cancel/action semantics | `AlertDialogAnnounceOutput`, action focus priority and destructive flags | `alert_dialog_announce_output_forces_alertdialog_role`, `alert_dialog_action_output_separates_focus_priority_and_destructive_semantics` |
| Popover | non-modal or modal popup with focus hooks | modal policy output, focus hook output, restore-focus target | `popover_modal_policy_output_traps_focus_only_for_open_modal_root`, `popover_focus_hook_output_respects_prevent_default_for_auto_focus_and_dismiss` |
| Tooltip | trigger is described by tooltip content; content is not interactive unless policy allows hoverable content | trigger accessibility output and provider delay outputs | `tooltip_trigger_accessibility_output_links_open_trigger_to_content_label`, `tooltip_delay_output_respects_hoverable_content_grace_and_disable_flag` |
| HoverCard | trigger retains focus; card content is supplemental | focus stays on trigger; delay output opens on hover/focus | `hover_card_delay_output_opens_on_focus_and_closes_on_blur`, `focus_manager_keeps_tooltip_and_hover_card_focus_on_trigger` |
| Menu/ContextMenu/Menubar | `menu`, `menuitem`, checked/radio variants, roving focus | item output roles/states, roving focus output, keyboard open origins | `dropdown_menu_roving_focus_output_skips_disabled_items_and_wraps`, `context_menu_keyboard_open_output_uses_trigger_center_for_context_key_or_shift_f10`, `menubar_state_machine_integrates_top_level_and_open_content_navigation` |
| NavigationMenu | navigational list with active link and controlled panel | root/list/item/link outputs and viewport/indicator visibility | `navigation_menu_link_state_preserves_active_enabled_hovered_parts`, `navigation_menu_interaction_output_connects_hover_focus_open_viewport_indicator_and_motion` |
| Select | button/listbox-like popup with selected item, value text, disabled items | trigger/value/item/indicator outputs and keyboard output | `select_keyboard_output_focuses_selected_item_when_opening`, `select_item_options_preserve_item_part_state` |
| Tabs | tab list, tab triggers, tab panels, automatic/manual activation | selected index, roving focus state, content output | `tabs_apply_keyboard_action_separates_manual_focus_from_selected_value`, `tabs_content_options_preserve_part_contract` |
| Accordion/Collapsible | button-like trigger controls expandable content | trigger icon state, open/disabled/root output | `accordion_keyboard_action_maps_toggle_and_header_navigation_keys`, `collapsible_trigger_icon_tracks_open_state` |
| Checkbox | checkbox state, disabled/required, indicator | root output and indicator output expose checked/disabled/required | `checkbox_root_output_preserves_state_disabled_and_required_contract`, `checkbox_indicator_output_mounts_checked_indeterminate_or_force_mounted` |
| RadioGroup | grouped radio items, one selected value, keyboard movement | root/item/indicator outputs and radio keyboard helpers | `radio_group_keyboard_target_skips_disabled_and_respects_loop_focus`, `radio_group_item_output_preserves_data_state_and_required_contract` |
| Switch | switch checked state and form semantics | root/thumb outputs, required/name/value fields | `switch_root_options_preserve_form_contract`, `switch_apply_checked_respects_disabled_and_noop_state` |
| Slider | range control with value, min/max, orientation | root/thumb/range outputs and keyboard value helpers | `slider_keyboard_action_maps_page_home_end_and_directional_arrows`, `slider_root_options_preserve_radix_contract` |
| Form Field | label/control/message association and validation state | field/label/control/message outputs, form association, validation preview | `form_field_label_and_message_outputs_preserve_validity_contract`, `form_validation_preview_output_matches_client_server_and_async_states` |
| Toast | live region, action alt text, focusable viewport hotkey, close/action targets | announce queue, viewport focus output, focus targets, root accessibility output | `toast_announce_queue_prioritizes_foreground_and_preserves_accessible_actions`, `toast_focus_targets_output_exposes_action_and_close_focus_contract` |
| Toolbar | toolbar orientation and grouped controls | root output, item outputs, action output | `toolbar_root_output_preserves_radix_contract`, `toolbar_toggle_outputs_match_group_and_item_data_attributes` |
| OTP Field | grouped one-time-code inputs and hidden form value | root role group, cell output, hidden input output | `otp_root_output_preserves_radix_contract`, `otp_hidden_input_output_carries_form_value_contract` |
| PasswordToggle | password input plus toggle button with pressed state | input output, button `aria_pressed`, decorative icon output | `password_toggle_button_and_icon_outputs_track_state`, `password_toggle_input_output_maps_visibility_to_type_and_display` |
| Label/Progress/Separator/Avatar/Icon | label association, progress value, decorative separator, fallback image text, icon label/decorative mode | root outputs and accessibility tree helpers | `label_root_output_preserves_for_id_and_display_contract`, `progress_root_output_preserves_radix_state_contract`, `separator_root_output_preserves_semantic_accessibility_contract`, `avatar_fallback_output_respects_delay_and_loaded_image`, `accessible_icon_root_output_allows_decorative_icon` |

## Agent-Readable Snapshot Contract

Primitive accessibility receipts should be emitted through
`PrimitiveAccessibilityTreeOutput` when a test or example needs a machine-readable
view of state. The required JSON stability is covered by
`accessibility_tree_json_snapshot_is_stable_and_agent_readable`.
