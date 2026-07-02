# Agent-Testable UX Receipts

Use `PrimitiveUxReceiptOutput` when an automated agent or test needs to inspect
primitive state without reading pixels.

## Snapshot Fields

| Field | Meaning |
| --- | --- |
| `primitive` | Stable primitive name, such as `dialog`, `select`, or `toolbar`. |
| `part` | Anatomy part being inspected, such as `root`, `trigger`, or `content`. |
| `states` | Ordered key/value state facts like `open=true`, `disabled=false`, `highlighted=1W`. |
| `focused_id` | Focused item/control id when focus is inside the primitive. |
| `selected_item` | Human-readable selected item label, when applicable. |
| `selected_value` | Stable selected value, when applicable. |
| `open_layers` | Ordered open layer stack, from outer route to inner content. |

## Example

```rust
use finui_primitives::{
    primitive_ux_receipt_json_snapshot, primitive_ux_receipt_output,
    PrimitiveUxReceiptOptions,
};

let receipt = primitive_ux_receipt_output(
    PrimitiveUxReceiptOptions::new("select", "content")
        .state("open", "true")
        .state("highlighted", "1W")
        .focused_id("select-period-item-1w")
        .selected_item("1W")
        .selected_value("1W")
        .open_layer("select.content")
        .open_layer("select.viewport"),
);

let json = primitive_ux_receipt_json_snapshot(&receipt);
assert!(json.contains("\"open_layers\""));
```

## Acceptance Tests

- `ux_receipt_output_collects_state_focus_selection_and_open_layers`
- `ux_receipt_json_snapshot_is_stable_and_agent_readable`
