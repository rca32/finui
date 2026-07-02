# Dialog API Contract

This document defines the Finui primitive contract for Dialog anatomy. It maps
Radix-style parts to the current egui implementation and records which parts are
required for a complete user-facing dialog.

## Parts

| Part | Current API | Required | Contract |
| --- | --- | --- | --- |
| Root | `DialogRootOptions`, `DialogRootOutput` | Yes | Owns `open`, `default_open`, `modal`, and `data_state`. |
| Trigger | `DialogTriggerOptions`, `primitive_dialog_trigger` | Recommended | Opens the dialog and provides the return-focus target for runtime focus management. |
| Portal | `DialogPortalOptions`, `DialogPortalOutput` | Optional | Carries `force_mount` and optional `container`; required when content is mounted outside the caller layout. |
| Overlay | `DialogOverlayOptions`, `primitive_dialog_overlay_options` | Required for modal dialogs | Carries backdrop tint, `force_mount`, and open state. Non-modal dialogs may omit it. |
| Content | `DialogContentOptions`, `primitive_dialog_content_options` | Yes | Defines content bounds, margin, open state, and force-mounted behavior. |
| Title | `primitive_dialog_title` | Required for accessible dialogs | Gives the dialog its accessible name. If the title is visually hidden, the same text must still be available to the accessibility bridge. |
| Description | `primitive_dialog_description` | Recommended | Provides supporting context. It may be omitted for simple dialogs with a self-explanatory title. |
| Close | `DialogCloseOptions`, `primitive_dialog_close_button` | Recommended | Provides explicit close affordance. Escape and outside-dismiss policy are handled by the layer/runtime contract. |

## Required Runtime Semantics

- `Root` must expose `data_state = open` or `closed` through
  `primitive_dialog_root_output`.
- `Content` and `Overlay` must use `primitive_dialog_part_state(open,
  force_mount)` semantics: mounted only when open or force-mounted.
- `Portal` must preserve `container` instead of interpreting it as a visual
  label.
- `Trigger`, `Close`, and layer dismiss events must converge on the same
  caller-owned open state through `dialog_apply_open`.
- Modal dialogs need focus trapping and inert background behavior at runtime.
  The current contract records `modal`; full inert/focus behavior is tracked
  separately in the Radix UX checklist.

## AlertDialog Relationship

AlertDialog reuses Dialog part contracts through type aliases for Root,
Trigger, Portal, Overlay, Content, and part state. It differs in two places:

- `primitive_alert_dialog_root_output` forces `modal = true`.
- Actions are split into `AlertDialogActionKind::Cancel` and
  `AlertDialogActionKind::Action`; destructive semantics and cancel-first focus
  priority are tracked separately.

## Current Limits

- Title/Description are render helpers, not yet a structured accessibility tree.
- Trigger return focus and modal inert behavior are not completed by this
  document.
- The `container` field is preserved in output, but full container routing is a
  separate layer/portal task.
