# Primitive State Tokens

This document is the shared contract for Radix-style primitive visual states in
`finui-primitives`. It describes the state vocabulary and the current Finui
default theme mapping. New primitives should use these tokens before adding
local paint branches.

## Source Of Truth

`PrimitiveTheme` is the common theme input for primitive renderers:

- `content_fill`: resting surface/background.
- `content_stroke`: resting surface border, currently `1.0`.
- `item_hover_fill`: hover fill for row-sized and control surfaces.
- `item_selected_fill`: selected/checked fill when no stronger accent helper is
  needed.
- `text`: primary foreground.
- `muted_text`: secondary foreground and closed force-mounted detail text.
- `disabled_text`: disabled foreground and disabled outline.
- `radius`: panel/content radius, currently `6`.
- `row_radius`: row and small control radius, currently `5.0`.
- `menu_row_height`: row target height, currently `32.0`.

Mode-specific colors come from `PrimitiveTheme::light()` and
`PrimitiveTheme::dark()`. Renderers may use local semantic helpers only when the
state needs mode-aware emphasis that does not yet exist as a first-class theme
field.

## State Mapping

| State | Output token | Default visual mapping |
| --- | --- | --- |
| Rest | none or default enum value | `content_fill`, `content_stroke`, `text`, `row_radius` for rows/controls and `radius` for panels. |
| Hover | `response.hovered()` plus enabled state | `item_hover_fill`; if the primitive uses an outline, keep `content_stroke` or a local hover stroke. |
| Active / pressed | explicit pressed flag or active highlighted row | Use the selected/highlighted state color family with a stronger local helper only for mode contrast. |
| Selected | `selected` option or selected output value | `item_selected_fill`; menu rows use mode-aware selected fill/stroke/text helpers. |
| Highlighted | `highlighted` option or roving-focus target | Highlighted rows use `item_selected_fill` family with local highlighted fill/stroke helpers. |
| Checked | `data_state = checked` or `indeterminate` | Checkbox/switch use control accent fill/stroke/mark helpers; radio can use `item_selected_fill` plus indicator. |
| Disabled | `data_disabled = true` or disabled option | Disable interaction, use `content_fill` for surface and `disabled_text` for text or outline. |
| Invalid | `data_invalid = true` or error message kind | Form label/control outputs expose invalid data. Error text uses the current error red until an invalid theme token is added. |
| Open | `data_state = open` | Trigger/content stays visually enabled; mounted content uses `text` and `muted_text`. Triggers may use hover/selected affordance while open. |
| Closed | `data_state = closed` | Closed force-mounted content is visually muted with `muted_text` and `disabled_text`. |

Orientation, side, align, and placement are layout tokens, not color tokens.
They must remain explicit on part outputs when the primitive has a Radix-style
`data-orientation`, `data-side`, or `data-align` equivalent.

## Stroke And Radius

- Content and row borders use `content_stroke`, currently width `1.0`.
- Checkbox and switch outline states use a control outline around `1.2`.
- Keyboard focus rings use width `1.5`; light mode uses `INDIGO_9`, dark mode
  uses `#8ec8ff`.
- Rows and compact controls use `row_radius`.
- Popups, panels, and layered content use `radius`.

## Implementation Rules

- State must be present in the logic output first: `data_state`,
  `data_disabled`, `data_invalid`, `data_orientation`, or the matching enum.
- Renderers map output state to theme tokens. Avoid one-off colors unless the
  local helper is documented by state and mode.
- Adding a new primitive state requires a focused unit test for the output token
  and, when the state paints differently, a helper-level or snapshot-friendly
  test for the visual mapping.
- Do not encode business meaning in colors. Use state names such as selected,
  highlighted, invalid, checked, open, and disabled.
