# Controlled And Uncontrolled Primitive Examples

Finui primitives expose caller-owned state first. Helper functions describe when
the caller should emit a change and whether an uncontrolled local value should be
updated.

## Caller-Owned State

```rust
use finui_primitives::{
    PrimitiveSwitchOptions, primitive_switch, switch_apply_checked, SwitchRootOptions,
};

fn show_switch(ui: &mut eframe::egui::Ui, enabled: &mut bool) {
    let output = primitive_switch(
        ui,
        "notifications-switch",
        enabled,
        PrimitiveSwitchOptions::default(),
    );

    if output.changed {
        // Persist `enabled` in the owning application state.
    }

    let mut next = *enabled;
    if switch_apply_checked(&mut next, true, SwitchRootOptions::default()) {
        *enabled = next;
    }
}
```

## Uncontrolled Local State

```rust
use finui_primitives::{
    primitive_controllable_state_output, PrimitiveControllableScope,
};

let receipt = primitive_controllable_state_output(
    PrimitiveControllableScope::SelectValue,
    None::<&str>,
    Some("1D"),
    "1D",
    "1W",
);

assert!(receipt.should_emit_change);
assert!(receipt.should_update_internal);
```

## Controlled External State

```rust
use finui_primitives::{
    primitive_controllable_state_output, PrimitiveControllableScope,
};

let receipt = primitive_controllable_state_output(
    PrimitiveControllableScope::DialogOpen,
    Some(false),
    Some(false),
    true,
    true,
);

assert!(receipt.should_emit_change);
assert!(!receipt.should_update_internal);
```

## Agent-Controlled State Receipt

```rust
use finui_primitives::{
    primitive_accessibility_node_output, primitive_accessibility_tree_json_snapshot,
    primitive_accessibility_tree_output, PrimitiveAccessibilityNodeOptions,
    PrimitiveAccessibilityRole,
};

let tree = primitive_accessibility_tree_output([
    primitive_accessibility_node_output(
        PrimitiveAccessibilityNodeOptions::new("select-period", PrimitiveAccessibilityRole::Button)
            .name("Period")
            .state("open", "true")
            .state("value", "1W"),
    ),
]);

let snapshot = primitive_accessibility_tree_json_snapshot(&tree);
assert!(snapshot.contains("\"select-period\""));
```

## Acceptance Tests

- `controllable_open_output_keeps_dialog_popover_and_menu_controlled_by_owner`
- `controllable_open_output_updates_uncontrolled_local_state`
- `controllable_value_output_unifies_select_menu_and_form_value_ownership`
- `accessibility_tree_json_snapshot_is_stable_and_agent_readable`
