# Primitive API Reference

This is the public reference for stable primitive contracts. It summarizes
options, outputs, data-state mapping, defaults, and egui-specific constraints.
The exact field list remains source-of-truth in
`crates/finui-primitives/src/primitives`.

| Primitive | Options | Outputs | State/data mapping | Defaults | egui constraints |
| --- | --- | --- | --- | --- | --- |
| Dialog | `DialogRootOptions`, `DialogTriggerOptions`, `DialogPortalOptions`, `DialogContentOptions`, `DialogOverlayOptions`, `DialogCloseOptions`, `DialogAnnounceOptions` | root, portal, part state, announce, content, overlay | `open` maps to `DialogDataState::Open/Closed`; modal drives focus trap and inert policy | closed, modal true for dialog usage, restore focus true | portal is a route, not a DOM portal; title/description are egui labels |
| AlertDialog | `AlertDialogRootOptions`, trigger/portal/content/overlay/action options | root, portal, part state, action, announce | forced modal, forced `alertdialog`, action kind separates cancel/destructive | modal true even if option is false | action focus priority replaces browser focus order |
| Popover | `PopoverRootOptions`, anchor/trigger/portal/content/close options | root, anchor, portal, modal policy, focus hook, content | open state, side/align, `force_mount`, focus hook prevention | closed, non-modal, bottom/start placement | collision and arrow are geometry outputs for egui painting |
| Tooltip | provider/root/trigger/portal/content options | provider, shared delay, trigger accessibility, content | open/closed content state, hoverable content, delay/skip delay | delayed open, non-interactive closed content | accessible description is modeled as ids and labels |
| HoverCard | root/trigger/portal/content options | root, delay, portal, content | hover/focus delayed open, non-dismissable hover content | delayed open/close, force mount false | hover intent is computed from timestamps and pointer path |
| DropdownMenu | root/trigger/portal/content/item/sub options | root, portal, content, group, item, sub outputs | open/closed, side/align, item highlighted/disabled/checked, submenu open | closed, bottom/start placement, looping focus | typeahead and roving focus are pure helpers; painter draws rows |
| ContextMenu | root/trigger/portal/content/item/sub options | open origin, root, portal, content, item, sub outputs | origin is pointer/touch/keyboard; content shares dropdown menu states | closed until origin event | trigger center is used for keyboard-origin geometry |
| Menubar | root/menu/portal/content/item options | root, menu, trigger, portal, content, state machine | top-level active index, opened menu value, content focus, typeahead | horizontal, loop focus true | one state machine coordinates menubar and menu content |
| NavigationMenu | root/list/item/trigger/content/link/indicator/viewport options | root/list/item/content/link/indicator/viewport/interactions | active item, viewport open, indicator visible, motion direction | horizontal, loop focus true | viewport/indicator are geometry outputs, not floating DOM nodes |
| Select | root/trigger/portal/content/viewport/item/group/label/separator options | root, trigger, value text, icon, content, viewport, item indicator, group, scroll button | open/closed, selected value, active value, side/align, disabled | closed, item-aligned content, placeholder until value | selected item focus is computed from item array |
| ScrollArea | root/viewport/scrollbar options | native scroll, viewport rect, scrollbar rect, thumb, corner visibility | scrollbar visible after delay or force mount; corner visible when both axes show | native scrolling preserved, delay before scrollbar visible | uses egui scroll state; thumb geometry is contract output |
| Tabs | `TabsHeaderOptions`, `TabsContentOptions`, item array | header output, content rect, trigger render state | selected index and roving focus; automatic/manual activation | horizontal, automatic activation, loop focus true | List is represented by trigger rects |
| Accordion | root/item/content options | root, item, header, content | single/multiple value, collapsible, disabled item, keyboard target | single/multiple root options decide collapse behavior | content is rendered by closure when mounted |
| Collapsible | root/trigger/content options | root, trigger, content | open/default/disabled, force-mounted content | closed, enabled | content output controls whether closure should render |
| Form Field | field/label/control/message/association/validation preview options | field, label, control, message, association, validation preview | invalid/required/disabled, client/server/async message matching | valid, enabled, required false | form ids and associations are strings for egui bridge |
| Checkbox | root/indicator options plus render options | root, indicator, control output | checked/unchecked/indeterminate, disabled, required, `data-state` | unchecked, enabled | keyboard activation is helper-driven |
| RadioGroup | root/item/indicator options | root, item, indicator | group value, checked item, orientation, required, disabled | horizontal, enabled, loop focus true | typed values are caller-owned |
| Switch | root/render options | root, thumb, control output | checked/unchecked, disabled, required, form value | unchecked, enabled, value `on` | thumb center is geometry output |
| Slider | root/render options | root, track, range, thumb, control output | value/default/min/max/step/orientation/inverted, keyboard action | horizontal, min/max caller-supplied, step 1 for root options | multi-thumb semantics are expressed in root/thumb helpers |
| Toggle | root/render options | root, render state, control output | pressed maps to `data-state=on/off`, disabled | unpressed, enabled | text/icon content is caller supplied |
| ToggleGroup | root/item/render options | root, item, group output | single/multiple selected values, item disabled, orientation | horizontal, loop focus true | selected indices are caller-owned in render helper |
| Toolbar | root/item/button/link/separator/toggle options | root, action, button, link, separator, toggle outputs | orientation, pressed, disabled, item kind | horizontal, loop focus true | layout helpers assign rects before painting |
| Toast | provider/viewport/root/action/announce options | provider, viewport, focus targets, pause, swipe, root, announce | foreground/background priority, swipe state, hotkey focus, action alt text | duration and swipe policy from provider | store is in-memory; viewport focus is helper output |
| OTP Field | root options | root, per-input, hidden input | sanitized value, completed, orientation, input type, validation type | one-time-code, vertical, numeric, text input | cells are visual; hidden input carries form value |
| PasswordToggle | root options | root, input, button, icon | visible/hidden maps to input type, `aria-pressed`, icon name | hidden, autocomplete current-password | value masking is string based |
| Avatar | root/image/fallback options | root, image, fallback | loaded/error/loading controls fallback | fallback delayed until image not loaded | fallback text derives initials |
| Progress | root options | root, indicator | loading/complete/indeterminate state and normalized value | value optional, max caller-supplied | indicator width is computed geometry |
| Separator | root options | root, line geometry | decorative vs semantic, orientation | decorative true by default | line endpoints are painter data |
| Label | root/label options | root | `html_for`, nested control, required suffix, disabled/muted color | not required, enabled | double-click selection prevention is a contract flag |
| AspectRatio | root options | output rect/size | ratio controls bounded rect | caller ratio | layout only; no web intrinsic sizing |
| VisuallyHidden | text options | output text/rect | text remains available while rect is offscreen | hidden outside visible bounds | accessibility bridge consumes output |
| AccessibleIcon | root options | root | label absent for decorative icon, disabled flag | labeled and enabled | tooltip/hover label represents accessible name in egui |

## Stability Notes

`OTP Field` and `PasswordToggle` are stable primitive APIs. Their status is
enforced by `OTP_FIELD_API_STABILITY`, `PASSWORD_TOGGLE_API_STABILITY`,
`otp_field_is_a_stable_public_api_not_preview`, and
`password_toggle_is_a_stable_public_api_not_preview`.
