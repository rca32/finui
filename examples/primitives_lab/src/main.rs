use std::time::{Duration, Instant};

use eframe::egui;
use finui_primitives::{
    AccordionRootOptions, ContextMenuItemOptions, ContextMenuOptions, ContextMenuTriggerOptions,
    DialogCloseOptions, DialogContentOptions, DialogOptions, DialogTriggerOptions,
    DropdownMenuOptions, DropdownMenuTriggerOptions, LayerPlacement, MenuItemOptions,
    OtpFieldOrientation, OtpFieldRootOptions, PasswordToggleRootOptions, PopoverOptions,
    PopoverTriggerOptions, PrimitiveActionKind, PrimitiveCheckboxOptions, PrimitiveDirection,
    PrimitiveLabelOptions, PrimitiveSliderOptions, PrimitiveSwitchOptions, PrimitiveTheme,
    RovingFocusState, SelectItemOptions, SelectOptions, SelectTriggerOptions,
    SelectViewportOptions, TabItem, TabsContentOptions, TabsHeaderOptions, ThemeMode, ToastAction,
    ToastKind, ToastProviderOptions, ToastStore, ToastViewportOptions, ToggleButtonOptions,
    ToggleGroupItem, ToggleGroupMode, ToolbarButtonSpec, TooltipOptions, TooltipTriggerOptions,
    primitive_accordion_item_with_options, primitive_accordion_root, primitive_action_button,
    primitive_checkbox, primitive_context_menu_checkbox_item, primitive_context_menu_item,
    primitive_context_menu_label, primitive_context_menu_radio_item,
    primitive_context_menu_separator, primitive_context_menu_trigger,
    primitive_dialog_close_button, primitive_dialog_description, primitive_dialog_title,
    primitive_dialog_trigger, primitive_direction_provider, primitive_dropdown_menu_checkbox_item,
    primitive_dropdown_menu_item, primitive_dropdown_menu_label,
    primitive_dropdown_menu_radio_item, primitive_dropdown_menu_separator,
    primitive_dropdown_menu_trigger, primitive_label_root, primitive_otp_field,
    primitive_password_toggle_field, primitive_popover_content, primitive_popover_trigger,
    primitive_select_item, primitive_select_label, primitive_select_separator,
    primitive_select_trigger, primitive_select_viewport, primitive_slider, primitive_switch,
    primitive_tabs_content, primitive_tabs_header_with_options, primitive_toast_provider,
    primitive_toggle, primitive_toggle_group, primitive_toolbar, primitive_tooltip_trigger,
    show_context_menu, show_dialog, show_dropdown_menu, show_popover, show_select, show_tooltip,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 900.0])
            .with_min_inner_size([960.0, 640.0]),
        ..eframe::NativeOptions::default()
    };
    eframe::run_native(
        "Finui Primitives Lab",
        options,
        Box::new(|cc| {
            configure_lab_style(&cc.egui_ctx);
            Ok(Box::new(PrimitivesLabApp::default()))
        }),
    )
}

fn configure_lab_style(ctx: &egui::Context) {
    #[cfg(debug_assertions)]
    ctx.all_styles_mut(|style| {
        style.debug.debug_on_hover = false;
        style.debug.debug_on_hover_with_all_modifiers = false;
        style.debug.show_interactive_widgets = false;
        style.debug.show_widget_hits = false;
        style.debug.warn_if_rect_changes_id = false;
        style.debug.show_focused_widget = false;
        style.debug.show_unaligned = false;
    });
}

struct PrimitiveCatalogueItem {
    name: &'static str,
    category: &'static str,
    states: &'static [&'static str],
}

const CATALOGUE: &[PrimitiveCatalogueItem] = &[
    PrimitiveCatalogueItem {
        name: "Dialog",
        category: "Layer",
        states: &["open", "closed", "modal", "dismiss", "focus"],
    },
    PrimitiveCatalogueItem {
        name: "Popover",
        category: "Layer",
        states: &["open", "closed", "anchor", "edge placement"],
    },
    PrimitiveCatalogueItem {
        name: "Tooltip",
        category: "Layer",
        states: &["hover", "instant", "delayed", "disabled"],
    },
    PrimitiveCatalogueItem {
        name: "DropdownMenu",
        category: "Menu",
        states: &[
            "open",
            "closed",
            "checked",
            "radio",
            "typeahead",
            "shortcut",
        ],
    },
    PrimitiveCatalogueItem {
        name: "ContextMenu",
        category: "Menu",
        states: &["pointer", "keyboard", "checked", "radio"],
    },
    PrimitiveCatalogueItem {
        name: "Select",
        category: "Menu",
        states: &["open", "closed", "value", "disabled item"],
    },
    PrimitiveCatalogueItem {
        name: "Accordion",
        category: "Disclosure",
        states: &["single", "multiple", "open", "disabled item"],
    },
    PrimitiveCatalogueItem {
        name: "Toast",
        category: "Feedback",
        states: &["foreground", "background", "swipe", "hotkey", "action"],
    },
    PrimitiveCatalogueItem {
        name: "Toolbar",
        category: "Action",
        states: &["horizontal", "toggle", "separator", "disabled item"],
    },
    PrimitiveCatalogueItem {
        name: "Checkbox",
        category: "Form",
        states: &["checked", "unchecked", "required", "disabled"],
    },
    PrimitiveCatalogueItem {
        name: "Switch",
        category: "Form",
        states: &["checked", "unchecked", "disabled", "focus"],
    },
    PrimitiveCatalogueItem {
        name: "Slider",
        category: "Form",
        states: &["value", "step", "drag", "disabled"],
    },
    PrimitiveCatalogueItem {
        name: "Toggle",
        category: "Action",
        states: &["pressed", "unpressed", "group"],
    },
    PrimitiveCatalogueItem {
        name: "Tabs",
        category: "Navigation",
        states: &["active", "disabled item", "content", "roving focus"],
    },
    PrimitiveCatalogueItem {
        name: "OTP Field",
        category: "Form",
        states: &["numeric", "complete", "paste", "focused index"],
    },
    PrimitiveCatalogueItem {
        name: "Password Toggle",
        category: "Form",
        states: &["hidden", "visible", "readonly", "icon"],
    },
];

const MARKET_OPTIONS: &[(&str, &str)] = &[
    ("KR3Y", "Korea Treasury 3Y"),
    ("KR10Y", "Korea Treasury 10Y"),
    ("USDJPY", "USD/JPY Spot"),
    ("SOFR", "SOFR Futures"),
];

const CATALOGUE_FAMILY_COUNT: usize = CATALOGUE.len();

struct PrimitivesLabApp {
    checkbox_checked: bool,
    switch_checked: bool,
    slider_value: f32,
    toggle_pressed: bool,
    toggle_group_selected: Vec<usize>,
    selected_tab: usize,
    tab_focus: RovingFocusState,
    otp_value: String,
    password_visible: bool,
    show_disabled: bool,
    show_open_layers: bool,
    show_rtl: bool,
    force_dark_preview: bool,
    show_long_text: bool,
    edge_placement: bool,
    dialog_open: bool,
    popover_open: bool,
    dropdown_open: bool,
    dropdown_checked: bool,
    dropdown_radio: &'static str,
    context_menu_position: Option<egui::Pos2>,
    context_checked: bool,
    context_radio: &'static str,
    select_open: bool,
    selected_market: Option<&'static str>,
    accordion_open: Vec<usize>,
    toolbar_bold: bool,
    toolbar_filter: bool,
    toast_store: ToastStore,
    toast_seeded: bool,
    last_action: String,
}

impl Default for PrimitivesLabApp {
    fn default() -> Self {
        Self {
            checkbox_checked: true,
            switch_checked: true,
            slider_value: 64.0,
            toggle_pressed: false,
            toggle_group_selected: vec![0],
            selected_tab: 0,
            tab_focus: RovingFocusState::new(),
            otp_value: "1234".to_owned(),
            password_visible: false,
            show_disabled: false,
            show_open_layers: false,
            show_rtl: false,
            force_dark_preview: false,
            show_long_text: false,
            edge_placement: false,
            dialog_open: false,
            popover_open: false,
            dropdown_open: false,
            dropdown_checked: true,
            dropdown_radio: "bid",
            context_menu_position: None,
            context_checked: true,
            context_radio: "desk",
            select_open: false,
            selected_market: Some("KR3Y"),
            accordion_open: vec![0],
            toolbar_bold: true,
            toolbar_filter: false,
            toast_store: ToastStore::default(),
            toast_seeded: false,
            last_action: "Ready".to_owned(),
        }
    }
}

impl eframe::App for PrimitivesLabApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if !self.toast_seeded {
            self.toast_store.push_accessible(
                ToastKind::Info,
                "Primitive lab is live",
                Some("Use the toolbar and layer controls to exercise runtime states."),
                Some("Review"),
                Some("Review primitive runtime state"),
                Duration::from_secs(90),
                Instant::now(),
            );
            self.toast_seeded = true;
        }

        let theme = PrimitiveTheme::for_mode(
            if self.force_dark_preview || ui.ctx().global_style().visuals.dark_mode {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        );

        self.top_bar(ui);
        ui.add_space(8.0);
        egui::ScrollArea::vertical()
            .id_salt("primitives_lab_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                self.summary_strip(ui, theme);
                ui.add_space(10.0);
                ui.columns(2, |columns| {
                    columns[0].set_min_width(430.0);
                    columns[1].set_min_width(430.0);
                    self.layer_and_menu_surface(&mut columns[0], theme);
                    self.form_and_navigation_surface(&mut columns[1], theme);
                });
                ui.add_space(10.0);
                self.catalogue_matrix(ui);
            });
    }
}

impl PrimitivesLabApp {
    fn top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.heading("Finui primitives");
            ui.separator();
            ui.checkbox(&mut self.show_disabled, "Disabled states");
            ui.checkbox(&mut self.show_open_layers, "Open layers");
            ui.checkbox(&mut self.show_rtl, "RTL");
            ui.checkbox(&mut self.force_dark_preview, "Dark preview");
            ui.checkbox(&mut self.show_long_text, "Long labels");
            ui.checkbox(&mut self.edge_placement, "Edge placement");
            ui.separator();
            ui.label(egui::RichText::new(&self.last_action).monospace());
        });
    }

    fn summary_strip(&self, ui: &mut egui::Ui, theme: PrimitiveTheme) {
        ui.horizontal_wrapped(|ui| {
            let coverage = format!("{CATALOGUE_FAMILY_COUNT} families");
            state_chip(ui, "coverage", &coverage, theme);
            state_chip(
                ui,
                "layers",
                if self.show_open_layers {
                    "open"
                } else {
                    "closed"
                },
                theme,
            );
            state_chip(
                ui,
                "direction",
                if self.show_rtl { "rtl" } else { "ltr" },
                theme,
            );
            state_chip(
                ui,
                "placement",
                if self.edge_placement {
                    "edge"
                } else {
                    "center"
                },
                theme,
            );
            state_chip(
                ui,
                "forms",
                if self.show_disabled {
                    "disabled"
                } else {
                    "enabled"
                },
                theme,
            );
            state_chip(
                ui,
                "selected",
                self.selected_market.unwrap_or("placeholder"),
                theme,
            );
        });
    }

    fn layer_and_menu_surface(&mut self, ui: &mut egui::Ui, theme: PrimitiveTheme) {
        section(ui, "Layer primitives", |ui| {
            let mut popover_anchor = None;
            ui.horizontal_wrapped(|ui| {
                let trigger = primitive_dialog_trigger(
                    ui,
                    "Dialog",
                    DialogTriggerOptions {
                        theme,
                        ..DialogTriggerOptions::default().open(self.dialog_open)
                    },
                );
                if trigger.clicked() {
                    self.dialog_open = true;
                    self.last_action = "dialog opened".to_owned();
                }

                let popover_trigger = primitive_popover_trigger(
                    ui,
                    "Popover",
                    PopoverTriggerOptions::default()
                        .open(self.popover_open)
                        .theme(theme),
                );
                if popover_trigger.clicked() {
                    self.popover_open = !self.popover_open;
                    self.last_action = format!("popover {}", open_label(self.popover_open));
                }
                popover_anchor = Some(popover_trigger.rect);

                let tooltip_trigger = primitive_tooltip_trigger(
                    ui,
                    "Tooltip",
                    TooltipTriggerOptions::default()
                        .open(self.show_open_layers)
                        .theme(theme),
                );
                let tooltip_open = tooltip_trigger.hovered() || self.show_open_layers;
                if tooltip_open {
                    let _ = show_tooltip(
                        ui.ctx(),
                        true,
                        TooltipOptions::new("lab-tooltip", tooltip_trigger.rect)
                            .width(250.0)
                            .placement(self.layer_placement())
                            .theme(theme),
                        self.tooltip_copy(),
                    );
                }
            });

            if self.dialog_open {
                let output = show_dialog(
                    ui.ctx(),
                    DialogOptions::new("lab-dialog")
                        .content(DialogContentOptions::new(320.0, 440.0, 160.0, 240.0))
                        .theme(theme),
                    |ui, _available| {
                        let mut action = None;
                        ui.horizontal(|ui| {
                            primitive_dialog_title(ui, "Trade ticket", theme);
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if primitive_dialog_close_button(
                                        ui,
                                        "dialog-close",
                                        DialogCloseOptions {
                                            theme,
                                            ..DialogCloseOptions::default()
                                        },
                                    )
                                    .clicked()
                                    {
                                        action = Some("close");
                                    }
                                },
                            );
                        });
                        if action.is_some() {
                            return action;
                        }
                        primitive_dialog_description(ui, self.dialog_copy(), theme);
                        ui.add_space(8.0);
                        if primitive_action_button(ui, "Confirm", PrimitiveActionKind::Primary)
                            .clicked()
                        {
                            return Some("confirm");
                        }
                        None
                    },
                );
                if output.should_close {
                    self.dialog_open = false;
                }
            }

            if self.popover_open || self.show_open_layers {
                if let Some(anchor) = popover_anchor {
                    let output = show_popover(
                        ui.ctx(),
                        PopoverOptions::anchored(
                            "lab-popover",
                            anchor,
                            280.0,
                            self.layer_placement(),
                        )
                        .theme(theme),
                        |ui| {
                            primitive_popover_content(
                                ui,
                                "Execution note",
                                self.popover_copy(),
                                theme,
                            );
                            if primitive_action_button(ui, "Close", PrimitiveActionKind::Secondary)
                                .clicked()
                            {
                                return Some("close");
                            }
                            None
                        },
                    );
                    if output.action == Some("close") || output.should_close {
                        self.popover_open = false;
                    }
                }
            }
        });

        section(ui, "Menus and selection", |ui| {
            let mut dropdown_anchor = None;
            let mut select_anchor = None;
            ui.horizontal_wrapped(|ui| {
                let dropdown_trigger = primitive_dropdown_menu_trigger(
                    ui,
                    "Actions",
                    DropdownMenuTriggerOptions::default()
                        .open(self.dropdown_open)
                        .theme(theme),
                );
                if dropdown_trigger.clicked() {
                    self.dropdown_open = !self.dropdown_open;
                }
                dropdown_anchor = Some(dropdown_trigger.rect);

                let context_trigger = primitive_context_menu_trigger(
                    ui,
                    self.context_menu_label(),
                    ContextMenuTriggerOptions::default().open(self.context_menu_position.is_some()),
                );
                if context_trigger.secondary_clicked() || context_trigger.clicked() {
                    self.context_menu_position = Some(
                        context_trigger
                            .interact_pointer_pos()
                            .unwrap_or_else(|| context_trigger.rect.left_bottom()),
                    );
                }

                let value_label = selected_market_label(self.selected_market);
                let select_trigger = primitive_select_trigger(
                    ui,
                    "market-select",
                    value_label,
                    SelectTriggerOptions::default()
                        .size(190.0, 32.0)
                        .open(self.select_open)
                        .disabled(self.show_disabled)
                        .placeholder(self.selected_market.is_none())
                        .theme(theme),
                );
                if select_trigger.response.clicked() {
                    self.select_open = !self.select_open;
                }
                select_anchor = Some(select_trigger.response.rect);
            });

            if self.dropdown_open || self.show_open_layers {
                if let Some(anchor) = dropdown_anchor {
                    let output = show_dropdown_menu(
                        ui.ctx(),
                        DropdownMenuOptions::anchored(
                            "lab-dropdown",
                            anchor,
                            220.0,
                            self.layer_placement(),
                        )
                        .theme(theme)
                        .max_height(220.0),
                        |ui| {
                            primitive_dropdown_menu_label(ui, "Order actions", 204.0);
                            if primitive_dropdown_menu_item(
                                ui,
                                self.dropdown_primary_label(),
                                MenuItemOptions::new(204.0).highlighted(true).theme(theme),
                            )
                            .clicked()
                            {
                                return Some("ticket");
                            }
                            primitive_dropdown_menu_checkbox_item(
                                ui,
                                "Include stale quotes",
                                &mut self.dropdown_checked,
                                MenuItemOptions::new(204.0).theme(theme),
                            );
                            primitive_dropdown_menu_separator(ui, 204.0);
                            primitive_dropdown_menu_radio_item(
                                ui,
                                "Bid side",
                                "bid",
                                &mut self.dropdown_radio,
                                MenuItemOptions::new(204.0).theme(theme),
                            );
                            primitive_dropdown_menu_radio_item(
                                ui,
                                "Ask side",
                                "ask",
                                &mut self.dropdown_radio,
                                MenuItemOptions::new(204.0)
                                    .disabled(self.show_disabled)
                                    .theme(theme),
                            );
                            None
                        },
                    );
                    if let Some(action) = output.action {
                        self.last_action = format!("dropdown {action}");
                        self.dropdown_open = false;
                    }
                    if output.should_close {
                        self.dropdown_open = false;
                    }
                }
            }

            if let Some(position) = self.context_menu_position {
                let output = show_context_menu(
                    ui.ctx(),
                    ContextMenuOptions::at("lab-context", position, 220.0).theme(theme),
                    |ui| {
                        primitive_context_menu_label(ui, "Context menu", 204.0);
                        if primitive_context_menu_item(
                            ui,
                            "Copy row",
                            ContextMenuItemOptions::new(204.0).theme(theme),
                        )
                        .clicked()
                        {
                            return Some("copy");
                        }
                        primitive_context_menu_checkbox_item(
                            ui,
                            "Pinned",
                            &mut self.context_checked,
                            ContextMenuItemOptions::new(204.0).theme(theme),
                        );
                        primitive_context_menu_separator(ui, 204.0);
                        primitive_context_menu_radio_item(
                            ui,
                            "Desk",
                            "desk",
                            &mut self.context_radio,
                            ContextMenuItemOptions::new(204.0).theme(theme),
                        );
                        primitive_context_menu_radio_item(
                            ui,
                            "Agent",
                            "agent",
                            &mut self.context_radio,
                            ContextMenuItemOptions::new(204.0).theme(theme),
                        );
                        None
                    },
                );
                if let Some(action) = output.action {
                    self.last_action = format!("context {action}");
                    self.context_menu_position = None;
                }
                if output.should_close {
                    self.context_menu_position = None;
                }
            }

            if self.select_open || self.show_open_layers {
                if let Some(anchor) = select_anchor {
                    let output = show_select(
                        ui.ctx(),
                        SelectOptions::anchored(
                            "lab-select",
                            anchor,
                            240.0,
                            self.layer_placement(),
                        )
                        .theme(theme)
                        .max_height(190.0),
                        |ui| {
                            primitive_select_viewport(
                                ui,
                                SelectViewportOptions::new(224.0),
                                |ui, _| {
                                    primitive_select_label(
                                        ui,
                                        "Markets",
                                        finui_primitives::SelectLabelOptions::new(224.0),
                                    );
                                    for (value, label) in MARKET_OPTIONS {
                                        let selected = self.selected_market == Some(*value);
                                        if primitive_select_item(
                                            ui,
                                            label,
                                            SelectItemOptions::new(224.0)
                                                .selected(selected)
                                                .checked(selected)
                                                .disabled(self.show_disabled && *value == "SOFR")
                                                .theme(theme),
                                        )
                                        .clicked()
                                        {
                                            return Some(*value);
                                        }
                                    }
                                    primitive_select_separator(
                                        ui,
                                        finui_primitives::SelectSeparatorOptions::new(224.0),
                                    );
                                    None
                                },
                            )
                        },
                    );
                    if let Some(value) = output.action {
                        self.selected_market = Some(value);
                        self.select_open = false;
                        self.last_action = format!("select {value}");
                    }
                    if output.should_close {
                        self.select_open = false;
                    }
                }
            }
        });

        section(ui, "Disclosure, toolbar, toast", |ui| {
            primitive_accordion_root(
                ui,
                "lab-accordion-root",
                AccordionRootOptions::multiple().theme(theme),
                |ui, options| {
                    primitive_accordion_item_with_options(
                        ui,
                        "acc-layer",
                        0,
                        &mut self.accordion_open,
                        "Layer contract",
                        true,
                        options,
                        |ui| {
                            ui.label("Open, close, force-mount, outside click, and Escape dismissal are exposed.");
                        },
                    );
                    primitive_accordion_item_with_options(
                        ui,
                        "acc-keyboard",
                        1,
                        &mut self.accordion_open,
                        "Keyboard contract",
                        !self.show_disabled,
                        options,
                        |ui| {
                            ui.label("Arrow keys, Enter, Space, Home, End, typeahead, and roving focus are tracked by helper outputs.");
                        },
                    );
                },
            );

            ui.add_space(6.0);
            let mut disabled_button = ToolbarButtonSpec::button("Off", 48.0);
            disabled_button.enabled = !self.show_disabled;
            let toolbar_items = [
                ToolbarButtonSpec::toggle("Bold", self.toolbar_bold, 54.0),
                ToolbarButtonSpec::toggle("Filter", self.toolbar_filter, 62.0),
                ToolbarButtonSpec::separator(),
                ToolbarButtonSpec::button("Export", 64.0),
                disabled_button,
            ];
            let toolbar = primitive_toolbar(ui, "lab-toolbar", &toolbar_items, 34.0, 6.0, theme);
            if let Some(index) = toolbar.clicked {
                match index {
                    0 => self.toolbar_bold = !self.toolbar_bold,
                    1 => self.toolbar_filter = !self.toolbar_filter,
                    3 => self.push_toast(ToastKind::Success, "Export queued"),
                    _ => {}
                }
                self.last_action = format!("toolbar index {index}");
            }

            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                if primitive_action_button(ui, "Info toast", PrimitiveActionKind::Secondary)
                    .clicked()
                {
                    self.push_toast(ToastKind::Info, "Quote refreshed");
                }
                if primitive_action_button(ui, "Warning toast", PrimitiveActionKind::Secondary)
                    .clicked()
                {
                    self.push_toast(ToastKind::Warning, "Stale quote detected");
                }
            });
            let outputs = primitive_toast_provider(
                ui,
                &mut self.toast_store,
                Instant::now(),
                ToastProviderOptions::default()
                    .duration(Duration::from_secs(90))
                    .viewport(
                        ToastViewportOptions::default()
                            .max_width(360.0)
                            .theme(theme),
                    ),
            );
            for output in outputs {
                match output.action {
                    Some(ToastAction::Action(id)) => {
                        self.toast_store.complete_action(id);
                        self.last_action = format!("toast action #{id}");
                    }
                    Some(ToastAction::Close(id)) => {
                        self.toast_store.dismiss(id);
                        self.last_action = format!("toast close #{id}");
                    }
                    None => {}
                }
            }
        });
    }

    fn form_and_navigation_surface(&mut self, ui: &mut egui::Ui, theme: PrimitiveTheme) {
        section(ui, "Forms and controls", |ui| {
            row_layout(ui, self.primitive_direction(), |ui| {
                primitive_label_root(
                    ui,
                    "Market alert",
                    PrimitiveLabelOptions {
                        theme,
                        ..PrimitiveLabelOptions::default().required(true)
                    },
                );
                primitive_checkbox(
                    ui,
                    "lab-checkbox",
                    &mut self.checkbox_checked,
                    "Enabled",
                    PrimitiveCheckboxOptions {
                        enabled: !self.show_disabled,
                        theme,
                        ..PrimitiveCheckboxOptions::default()
                    },
                );
                ui.label(if self.checkbox_checked {
                    "checked"
                } else {
                    "unchecked"
                });
            });

            row_layout(ui, self.primitive_direction(), |ui| {
                ui.label("Auto refresh");
                primitive_switch(
                    ui,
                    "lab-switch",
                    &mut self.switch_checked,
                    PrimitiveSwitchOptions {
                        enabled: !self.show_disabled,
                        theme,
                        ..PrimitiveSwitchOptions::default()
                    },
                );
                ui.label(if self.switch_checked { "on" } else { "off" });
            });

            row_layout(ui, self.primitive_direction(), |ui| {
                ui.label(format!("Risk limit {:.0}", self.slider_value));
                primitive_slider(
                    ui,
                    &mut self.slider_value,
                    PrimitiveSliderOptions::new(0.0, 100.0)
                        .step(5.0)
                        .width(220.0)
                        .theme(theme),
                );
            });
        });

        section(ui, "Actions and navigation", |ui| {
            ui.horizontal_wrapped(|ui| {
                primitive_toggle(
                    ui,
                    "lab-toggle",
                    &mut self.toggle_pressed,
                    "Bold",
                    ToggleButtonOptions {
                        enabled: !self.show_disabled,
                        theme,
                        ..ToggleButtonOptions::default()
                    },
                );
                primitive_toggle_group(
                    ui,
                    "lab-toggle-group",
                    &mut self.toggle_group_selected,
                    ToggleGroupMode::Single,
                    &[
                        ToggleGroupItem {
                            label: "Day",
                            enabled: true,
                        },
                        ToggleGroupItem {
                            label: "Week",
                            enabled: !self.show_disabled,
                        },
                        ToggleGroupItem {
                            label: "Month",
                            enabled: true,
                        },
                    ],
                    ToggleButtonOptions {
                        theme,
                        ..ToggleButtonOptions::default()
                    },
                );
            });

            ui.horizontal_wrapped(|ui| {
                let _ = primitive_action_button(ui, "Apply", PrimitiveActionKind::Primary);
                let _ = primitive_action_button(ui, "Reset", PrimitiveActionKind::Secondary);
                let _ = primitive_action_button(ui, "Delete", PrimitiveActionKind::Destructive);
            });

            let tabs = [
                TabItem {
                    label: "Overview",
                    enabled: true,
                },
                TabItem {
                    label: "Orders",
                    enabled: !self.show_disabled,
                },
                TabItem {
                    label: "Audit",
                    enabled: true,
                },
            ];
            let direction = self.primitive_direction();
            primitive_tabs_header_with_options(
                ui,
                "lab-tabs",
                &mut self.selected_tab,
                &mut self.tab_focus,
                &tabs,
                TabsHeaderOptions::default()
                    .theme(theme)
                    .direction(direction),
            );
            primitive_tabs_content(
                ui,
                "lab-tabs-content",
                TabsContentOptions::default().min_height(56.0).theme(theme),
                |ui| {
                    ui.label(format!(
                        "{} panel is selected. Use arrow keys to verify roving focus.",
                        tabs.get(self.selected_tab)
                            .map(|tab| tab.label)
                            .unwrap_or("Unknown")
                    ));
                },
            );
        });

        section(ui, "Text and security", |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui.button("OTP 1234").clicked() {
                    self.otp_value = "1234".to_owned();
                }
                if ui.button("OTP full").clicked() {
                    self.otp_value = "123456".to_owned();
                }
                if ui.button("Clear").clicked() {
                    self.otp_value.clear();
                }
                primitive_otp_field(
                    ui,
                    &OtpFieldRootOptions::default()
                        .value(&self.otp_value)
                        .orientation(OtpFieldOrientation::Horizontal),
                    6,
                    Some(self.otp_value.chars().count().min(5)),
                    theme,
                );
            });

            ui.horizontal_wrapped(|ui| {
                if ui.button("Toggle password").clicked() {
                    self.password_visible = !self.password_visible;
                }
                primitive_password_toggle_field(
                    ui,
                    &PasswordToggleRootOptions::default()
                        .value("s3cret")
                        .visible(self.password_visible)
                        .read_only(self.show_disabled),
                    240.0,
                    32.0,
                    theme,
                );
            });
        });
    }

    fn catalogue_matrix(&self, ui: &mut egui::Ui) {
        section(ui, "Coverage matrix", |ui| {
            egui::Grid::new("primitive_catalogue")
                .striped(true)
                .min_col_width(108.0)
                .show(ui, |ui| {
                    ui.strong("Primitive");
                    ui.strong("Category");
                    ui.strong("Visible states");
                    ui.end_row();
                    for item in CATALOGUE {
                        ui.label(item.name);
                        ui.label(item.category);
                        ui.label(item.states.join(", "));
                        ui.end_row();
                    }
                });
        });
    }

    fn layer_placement(&self) -> LayerPlacement {
        if self.edge_placement {
            LayerPlacement::RightStart {
                offset: egui::vec2(8.0, 0.0),
            }
        } else {
            LayerPlacement::BelowStart {
                offset: egui::vec2(0.0, 6.0),
            }
        }
    }

    fn primitive_direction(&self) -> PrimitiveDirection {
        if self.show_rtl {
            PrimitiveDirection::Rtl
        } else {
            PrimitiveDirection::Ltr
        }
    }

    fn dialog_copy(&self) -> &'static str {
        if self.show_long_text {
            "Modal content demonstrates a deliberately long localized sentence, close action, outside click dismissal, Escape dismissal, and focus-visible styling without clipping the content area."
        } else {
            "Modal content, title, description, close action, and outside/Escape dismissal are rendered."
        }
    }

    fn popover_copy(&self) -> &'static str {
        if self.show_long_text {
            "Popover content is anchored to the trigger, uses constrained width, and keeps long operational copy readable without overflowing adjacent controls."
        } else {
            "Popover content is mounted in a layer and closes on outside click or Escape."
        }
    }

    fn tooltip_copy(&self) -> &'static str {
        if self.show_long_text {
            "Tooltip content follows the hovered trigger and verifies a longer sentence can wrap inside the configured layer width."
        } else {
            "Tooltip content follows the trigger, supports delay policy, and stays dismissable."
        }
    }

    fn dropdown_primary_label(&self) -> &'static str {
        if self.show_long_text {
            "Open ticket with extended execution context"
        } else {
            "Open ticket"
        }
    }

    fn context_menu_label(&self) -> &'static str {
        if self.show_long_text {
            "row context menu"
        } else {
            "row menu"
        }
    }

    fn push_toast(&mut self, kind: ToastKind, title: &'static str) {
        self.toast_store.push_accessible(
            kind,
            title,
            Some("Action output is focusable and closeable from the toast viewport."),
            Some("Open"),
            Some("Open notification action"),
            Duration::from_secs(90),
            Instant::now(),
        );
        self.last_action = format!("toast {title}");
    }
}

fn section(ui: &mut egui::Ui, title: &str, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.add_space(6.0);
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label(egui::RichText::new(title).strong().size(15.0));
        ui.add_space(5.0);
        add_contents(ui);
    });
}

fn row_layout(
    ui: &mut egui::Ui,
    direction: PrimitiveDirection,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    let _ = primitive_direction_provider(ui, direction, add_contents);
}

fn state_chip(ui: &mut egui::Ui, label: &str, value: &str, theme: PrimitiveTheme) {
    let text = format!("{label}: {value}");
    let width = (text.chars().count() as f32 * 7.4 + 18.0).clamp(74.0, 180.0);
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, 24.0), egui::Sense::hover());
    ui.painter()
        .rect_filled(rect, theme.row_radius, theme.content_fill);
    ui.painter().rect_stroke(
        rect,
        theme.row_radius,
        theme.content_stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        finui_primitives::scaled_monospace_font(ui, 11.0),
        theme.muted_text,
    );
}

fn selected_market_label(value: Option<&'static str>) -> &'static str {
    value
        .and_then(|value| {
            MARKET_OPTIONS
                .iter()
                .find_map(|(candidate, label)| (*candidate == value).then_some(*label))
        })
        .unwrap_or("Select market")
}

fn open_label(open: bool) -> &'static str {
    if open { "open" } else { "closed" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives_lab_catalogue_covers_required_radix_families() {
        let names = CATALOGUE
            .iter()
            .map(|item| item.name)
            .collect::<std::collections::BTreeSet<_>>();

        for required in [
            "Dialog",
            "Popover",
            "Tooltip",
            "DropdownMenu",
            "ContextMenu",
            "Select",
            "Tabs",
            "Accordion",
            "Slider",
            "Toast",
            "Toolbar",
        ] {
            assert!(names.contains(required), "missing {required}");
        }
    }

    #[test]
    fn primitives_lab_catalogue_exposes_required_interaction_states() {
        let states = CATALOGUE
            .iter()
            .flat_map(|item| item.states.iter().copied())
            .collect::<std::collections::BTreeSet<_>>();

        for required in [
            "open",
            "closed",
            "disabled",
            "edge placement",
            "keyboard",
            "typeahead",
            "swipe",
            "hotkey",
            "focus",
        ] {
            assert!(states.contains(required), "missing state {required}");
        }
    }

    #[test]
    fn selected_market_label_uses_placeholder_without_value() {
        assert_eq!(selected_market_label(None), "Select market");
        assert_eq!(selected_market_label(Some("KR3Y")), "Korea Treasury 3Y");
    }

    #[test]
    fn primitives_lab_summary_count_matches_catalogue() {
        assert_eq!(CATALOGUE_FAMILY_COUNT, CATALOGUE.len());
        assert_eq!(CATALOGUE_FAMILY_COUNT, 16);
    }

    #[test]
    fn primitives_lab_defaults_start_with_readable_closed_layers() {
        let app = PrimitivesLabApp::default();
        assert!(!app.show_open_layers);
        assert!(!app.popover_open);
        assert!(!app.dropdown_open);
        assert!(!app.select_open);
    }

    #[test]
    fn primitives_lab_direction_helper_tracks_rtl_toggle() {
        let mut app = PrimitivesLabApp::default();
        assert_eq!(app.primitive_direction(), PrimitiveDirection::Ltr);
        app.show_rtl = true;
        assert_eq!(app.primitive_direction(), PrimitiveDirection::Rtl);
    }
}
