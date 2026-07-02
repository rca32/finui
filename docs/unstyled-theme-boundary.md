# Unstyled Primitive And Theme Boundary

Finui primitive APIs are split into:

- unstyled logic outputs: option/output structs and pure helpers that can be
  consumed without `PrimitiveTheme`
- default theme renderers: egui painter/widget helpers that apply
  `PrimitiveTheme`

Use `primitive_style_boundary_output` when a test or downstream integration
needs to prove that a part has an unstyled contract and a themed renderer.

| Primitive | Part | Unstyled contract | Default theme renderer |
| --- | --- | --- | --- |
| Dialog | content | `DialogContentOptions`, `DialogPartStateOutput` | `primitive_dialog_content_options`, `show_dialog` |
| AlertDialog | action | `AlertDialogActionOutput` | `primitive_alert_dialog_action` |
| Popover | content | `PopoverContentOutput` | `primitive_popover_content_with_options` |
| Tooltip | content | `TooltipContentOutput` | `primitive_tooltip_content_with_options` |
| DropdownMenu | item | `DropdownMenuItemOptions`, `MenuItemOptions` | `primitive_dropdown_menu_item` |
| ContextMenu | item | `ContextMenuItemOptions` | `primitive_context_menu_item` |
| Menubar | trigger | `MenubarTriggerOutput` | `primitive_menubar_trigger` |
| NavigationMenu | viewport | `NavigationMenuViewportOutput` | `primitive_navigation_viewport` |
| Select | item | `SelectItemOptions`, `SelectItemIndicatorOutput` | `primitive_select_item` |
| ScrollArea | thumb | thumb geometry helpers | `primitive_scroll_thumb` |
| Tabs | trigger | `TabsTriggerState` | `primitive_tabs_trigger` |
| Accordion | item/content | `AccordionItemOutput`, `CollapsibleContentOptions` | `primitive_accordion_item`, `primitive_accordion_content` |
| Form | label/control/message | `PrimitiveFormLabelOutput`, `PrimitiveFormControlOutput`, `PrimitiveFormMessageOutput` | `primitive_form_label`, `primitive_form_control`, `primitive_form_message` |
| Checkbox | root/indicator | `CheckboxRootOutput`, `CheckboxIndicatorOutput` | `primitive_checkbox_root`, `primitive_checkbox_indicator` |
| RadioGroup | item/indicator | `RadioGroupItemOutput`, `RadioGroupIndicatorOutput` | `primitive_radio_item_part`, `primitive_radio_indicator` |
| Switch | root/thumb | `SwitchRootOutput`, `SwitchThumbOutput` | `primitive_switch_root`, `primitive_switch_thumb` |
| Slider | track/range/thumb | `SliderTrackOutput`, `SliderRangeOutput`, `SliderThumbOutput` | `primitive_slider_track`, `primitive_slider_range`, `primitive_slider_thumb` |
| Toast | root/action/close | `ToastRootOutput`, `ToastFocusTargetsOutput` | `primitive_toast_root`, `primitive_toast_action`, `primitive_toast_close` |
| Toolbar | item/action | `ToolbarActionOutput`, item outputs | `primitive_toolbar_button`, `primitive_toolbar_toggle_item` |
| OTP Field | root/input/hidden input | `OtpFieldRootOutput`, `OtpFieldInputOutput`, `OtpFieldHiddenInputOutput` | `primitive_otp_field`, `primitive_otp_field_input` |
| PasswordToggle | root/input/button/icon | `PasswordToggleRootOutput`, input/button/icon outputs | `primitive_password_toggle_field` |

Logic-only outputs may exist without a themed renderer. Accessibility trees,
data attributes, controllable state receipts, and UX receipts are intentionally
unstyled.

## Acceptance Tests

- `style_boundary_output_separates_unstyled_contract_from_default_theme_renderer`
- `style_boundary_output_allows_logic_only_parts_without_theme_renderer`
