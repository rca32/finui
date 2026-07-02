# Primitive Accessibility Snapshots

`primitive_accessibility_snapshot_catalogue_output` provides a stable baseline
tree for the major primitive families. It is not a replacement for runtime
screen-reader integration, but it gives tests and agents a consistent semantic
snapshot to compare.

| Snapshot id | Primitive | Role |
| --- | --- | --- |
| `dialog.content` | Dialog | `dialog` |
| `alert_dialog.content` | AlertDialog | `alertdialog` |
| `tooltip.content` | Tooltip | `tooltip` |
| `menu.content` | DropdownMenu | `menu` |
| `menu.item` | DropdownMenu item | `menuitem` |
| `select.trigger` | Select | `button` |
| `tabs.root` | Tabs | `group` |
| `accordion.trigger` | Accordion | `button` |
| `checkbox.root` | Checkbox | `checkbox` |
| `radio_group.root` | RadioGroup | `group` |
| `switch.root` | Switch | `button` |
| `slider.root` | Slider | `slider` |
| `toast.viewport` | Toast | `status` |
| `toolbar.root` | Toolbar | `toolbar` |
| `otp.root` | OTP Field | `group` |
| `password_toggle.input` | PasswordToggle | `textbox` |

## Acceptance Tests

- `accessibility_snapshot_catalogue_covers_major_primitives`
- `accessibility_snapshot_catalogue_json_is_stable`
