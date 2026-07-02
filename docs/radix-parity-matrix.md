# Radix Parity Matrix

Status meanings:

- Complete: contract, focused tests, and at least one public render/helper path exist.
- Partial: core contract exists, but runtime, visual, or example coverage remains.
- Missing: no meaningful contract yet.

| Component | Supported parts | Missing parts or gaps | Keyboard | Focus | Accessibility | Visual tests | Examples | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Dialog | Root, Trigger, Portal, Overlay, Content, Title, Description, Close | runtime tab sequence harness | focus manager tests | modal inert and restore focus | announce output | missing snapshots | lab pending | Complete contract, partial runtime |
| AlertDialog | Dialog parts, Cancel, Action | runtime destructive workflow example | focus manager tests | forced modal | alertdialog announce | missing snapshots | lab pending | Complete contract, partial runtime |
| Popover | Root, Anchor, Trigger, Portal, Content, Arrow, Close | full edge placement lab | helper tests | focus hook output | modal policy output | missing snapshots | lab pending | Complete contract, partial visual |
| Tooltip | Provider, Root, Trigger, Portal, Content, Arrow | runtime hover sequence harness | delay helper tests | trigger retained | trigger accessibility output | missing snapshots | lab pending | Complete contract, partial runtime |
| HoverCard | Root, Trigger, Portal, Content, Arrow | runtime hover grace harness | delay helper tests | trigger retained | supplemental content output | missing snapshots | lab pending | Complete contract, partial runtime |
| DropdownMenu | Root, Trigger, Portal, Content, Group, Item, CheckboxItem, RadioItem, Label, Separator, Sub, Arrow, Shortcut | runtime egui input sequence | roving/typeahead/submenu tests | roving focus output | item state outputs | missing snapshots | lab pending | Complete contract, partial runtime |
| ContextMenu | Root, Trigger, Portal, Content, menu item variants, Sub | runtime dismiss sequence | keyboard origin and shared menu tests | roving focus output | origin output | missing snapshots | grid integration pending | Complete contract, partial runtime |
| Menubar | Root, Menu, Trigger, Portal, Content, menu item variants | runtime sequence | state machine tests | top/content focus state | trigger output | missing snapshots | lab pending | Complete contract, partial runtime |
| NavigationMenu | Root, List, Item, Trigger, Content, Link, Indicator, Viewport | full responsive visual coverage | typeahead/navigation tests | hover/focus/open output | link state output | missing snapshots | lab pending | Complete contract, partial visual |
| Select | Root, Trigger, Value, Icon, Portal, Content, Viewport, Group, Label, Item, ItemIndicator, ScrollButton, Separator | runtime listbox sequence | keyboard/typeahead tests | selected item focus output | value/item outputs | missing snapshots | lab pending | Complete contract, partial runtime |
| ScrollArea | Root, Viewport, Scrollbar, Thumb, Corner | visual diff for scrollbar states | not primary | native scroll preserved | structural outputs | missing snapshots | lab pending | Complete contract, partial visual |
| Tabs | List/Trigger/Content through header/content helpers | runtime focus sequence | keyboard tests | roving focus state | selected content output | missing snapshots | primitives_lab basic | Complete contract, partial runtime |
| Accordion | Root, Item, Header, Trigger, Content | runtime sequence | keyboard tests | enabled header focus | open/disabled outputs | missing snapshots | lab pending | Complete contract, partial runtime |
| Collapsible | Root, Trigger, Content | visual snapshot | keyboard tests | trigger state | open/disabled output | missing snapshots | lab pending | Complete contract, partial visual |
| Checkbox | Root, Indicator | runtime keyboard sequence | activation helper tests | focus through egui response | checked/required output | missing snapshots | primitives_lab basic | Complete contract, partial runtime |
| RadioGroup | Root, Item, Indicator | runtime keyboard sequence | keyboard tests | target index output | group/item outputs | missing snapshots | lab pending | Complete contract, partial runtime |
| Switch | Root, Thumb | runtime keyboard sequence | activation helper tests | egui response focus | form output | missing snapshots | primitives_lab basic | Complete contract, partial runtime |
| Slider | Root, Track, Range, Thumb | runtime drag sequence | keyboard tests | thumb output | value output | missing snapshots | primitives_lab basic | Complete contract, partial runtime |
| Form | Field, Label, Control, Message | full form demo | activation helper tests | associated ids | validation preview | missing snapshots | lab pending | Complete contract, partial examples |
| Toast | Provider, Viewport, Root, Title, Description, Action, Close | runtime swipe gesture capture | hotkey helper tests | focus target output | announce queue | missing snapshots | lab pending | Complete contract, partial runtime |
| Toolbar | Root, Button, Link, Separator, ToggleGroup, ToggleItem | runtime focus sequence | action helper tests | item output | orientation output | missing snapshots | lab pending | Complete contract, partial runtime |
| Toggle/ToggleGroup | Root, Item | runtime keyboard sequence | state helper tests | egui response focus | pressed output | missing snapshots | primitives_lab basic | Complete contract, partial runtime |
| OTP Field | Root, Input, HiddenInput | editable cell runtime | value helper tests | focused index output | group/form output | missing snapshots | primitives_lab basic | Stable contract, partial runtime |
| PasswordToggle | Root, Input, Button, Icon | runtime button sequence | visibility helper tests | button output | pressed/input outputs | missing snapshots | primitives_lab basic | Stable contract, partial runtime |
| Avatar | Root, Image, Fallback | image load runtime | not primary | not primary | fallback output | missing snapshots | lab pending | Complete contract, partial visual |
| Progress | Root, Indicator | visual snapshot | not primary | not primary | value output | missing snapshots | lab pending | Complete contract, partial visual |
| Separator | Root | visual snapshot | not primary | not primary | decorative/semantic output | missing snapshots | lab pending | Complete contract, partial visual |
| Label | Root | long text policy | not primary | label association | association output | missing snapshots | primitives_lab basic | Complete contract, partial text policy |
| Layout/Utility | AspectRatio, VisuallyHidden, AccessibleIcon, DirectionProvider, Slot | broader example coverage | direction helpers | hidden output | accessibility tree output | missing snapshots | lab pending | Complete contract, partial examples |
