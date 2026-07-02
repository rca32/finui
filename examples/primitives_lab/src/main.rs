use eframe::egui;
use finui_primitives::{
    OtpFieldOrientation, OtpFieldRootOptions, PasswordToggleRootOptions, PrimitiveActionKind,
    PrimitiveCheckboxOptions, PrimitiveLabelOptions, PrimitiveSliderOptions,
    PrimitiveSwitchOptions, PrimitiveTheme, RovingFocusState, TabItem, TabsContentOptions,
    ThemeMode, ToggleButtonOptions, ToggleGroupItem, ToggleGroupMode, primitive_action_button,
    primitive_checkbox, primitive_label_root, primitive_otp_field, primitive_password_toggle_field,
    primitive_slider, primitive_switch, primitive_tabs_content, primitive_tabs_header,
    primitive_toggle, primitive_toggle_group,
};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Finui Primitives Lab",
        options,
        Box::new(|_cc| Ok(Box::new(PrimitivesLabApp::default()))),
    )
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
        states: &["open", "closed", "modal", "edge placement"],
    },
    PrimitiveCatalogueItem {
        name: "Popover",
        category: "Layer",
        states: &["open", "closed", "anchor", "edge placement"],
    },
    PrimitiveCatalogueItem {
        name: "Tooltip",
        category: "Layer",
        states: &["delayed", "instant", "hoverable", "disabled"],
    },
    PrimitiveCatalogueItem {
        name: "DropdownMenu",
        category: "Menu",
        states: &["open", "closed", "typeahead", "submenu"],
    },
    PrimitiveCatalogueItem {
        name: "ContextMenu",
        category: "Menu",
        states: &["pointer", "keyboard", "long press", "dismiss"],
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
        states: &["foreground", "background", "swipe", "hotkey"],
    },
    PrimitiveCatalogueItem {
        name: "Toolbar",
        category: "Action",
        states: &["horizontal", "vertical", "toggle", "disabled item"],
    },
    PrimitiveCatalogueItem {
        name: "Button",
        category: "Action",
        states: &["primary", "secondary", "danger"],
    },
    PrimitiveCatalogueItem {
        name: "Checkbox",
        category: "Form",
        states: &["checked", "unchecked", "disabled"],
    },
    PrimitiveCatalogueItem {
        name: "Switch",
        category: "Form",
        states: &["checked", "unchecked", "disabled"],
    },
    PrimitiveCatalogueItem {
        name: "Slider",
        category: "Form",
        states: &["value", "step", "drag"],
    },
    PrimitiveCatalogueItem {
        name: "Toggle",
        category: "Action",
        states: &["pressed", "unpressed", "group"],
    },
    PrimitiveCatalogueItem {
        name: "Tabs",
        category: "Navigation",
        states: &["active", "disabled item", "content"],
    },
    PrimitiveCatalogueItem {
        name: "OTP Field",
        category: "Form",
        states: &["numeric", "complete", "horizontal"],
    },
    PrimitiveCatalogueItem {
        name: "Password Toggle",
        category: "Form",
        states: &["hidden", "visible", "readonly"],
    },
    PrimitiveCatalogueItem {
        name: "Label",
        category: "Status",
        states: &["required", "muted", "disabled"],
    },
];

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
            show_open_layers: true,
            show_rtl: false,
            force_dark_preview: false,
            show_long_text: false,
            edge_placement: false,
        }
    }
}

impl eframe::App for PrimitivesLabApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme = PrimitiveTheme::for_mode(
            if self.force_dark_preview || ui.ctx().global_style().visuals.dark_mode {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        );

        ui.horizontal(|ui| {
            ui.heading("Finui primitives");
            ui.separator();
            ui.checkbox(&mut self.show_disabled, "Disabled states");
            ui.checkbox(&mut self.show_open_layers, "Open layers");
            ui.checkbox(&mut self.show_rtl, "RTL");
            ui.checkbox(&mut self.force_dark_preview, "Dark");
            ui.checkbox(&mut self.show_long_text, "Long labels");
            ui.checkbox(&mut self.edge_placement, "Edge");
        });

        ui.add_space(8.0);
        ui.columns(2, |columns| {
            self.catalogue_panel(&mut columns[0]);
            self.preview_panel(&mut columns[1], theme);
        });
    }
}

impl PrimitivesLabApp {
    fn catalogue_panel(&self, ui: &mut egui::Ui) {
        ui.heading("Catalogue");
        ui.add_space(4.0);
        egui::Grid::new("primitive_catalogue")
            .striped(true)
            .min_col_width(84.0)
            .show(ui, |ui| {
                ui.strong("Primitive");
                ui.strong("Category");
                ui.strong("States");
                ui.end_row();
                for item in CATALOGUE {
                    ui.label(item.name);
                    ui.label(item.category);
                    ui.label(item.states.join(", "));
                    ui.end_row();
                }
            });
    }

    fn preview_panel(&mut self, ui: &mut egui::Ui, theme: PrimitiveTheme) {
        ui.heading("Interactive states");
        ui.add_space(4.0);
        self.layer_controls(ui);
        ui.separator();
        self.form_controls(ui, theme);
        ui.separator();
        self.action_controls(ui, theme);
        ui.separator();
        self.navigation_controls(ui, theme);
        ui.separator();
        self.text_and_security_controls(ui, theme);
    }

    fn layer_controls(&mut self, ui: &mut egui::Ui) {
        let direction = if self.show_rtl { "rtl" } else { "ltr" };
        let placement = if self.edge_placement {
            "edge placement"
        } else {
            "center placement"
        };
        let layer_state = if self.show_open_layers {
            "open"
        } else {
            "closed"
        };
        let label = if self.show_long_text {
            "KR103502GG38 한국국채 3Y dropdown menu context menu select value"
        } else {
            "Layer state"
        };

        egui::Grid::new("layer_state_controls")
            .striped(true)
            .min_col_width(92.0)
            .show(ui, |ui| {
                for primitive in [
                    "Dialog",
                    "Popover",
                    "Tooltip",
                    "DropdownMenu",
                    "ContextMenu",
                    "Select",
                    "Accordion",
                    "Toast",
                    "Toolbar",
                ] {
                    ui.label(primitive);
                    ui.label(layer_state);
                    ui.label(direction);
                    ui.label(placement);
                    ui.label(label);
                    ui.end_row();
                }
            });
    }

    fn form_controls(&mut self, ui: &mut egui::Ui, theme: PrimitiveTheme) {
        ui.horizontal(|ui| {
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
        });

        ui.horizontal(|ui| {
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
        });

        ui.horizontal(|ui| {
            ui.label(format!("Risk limit {:.0}", self.slider_value));
            primitive_slider(
                ui,
                &mut self.slider_value,
                PrimitiveSliderOptions::new(0.0, 100.0)
                    .step(5.0)
                    .width(180.0)
                    .theme(theme),
            );
        });
    }

    fn action_controls(&mut self, ui: &mut egui::Ui, theme: PrimitiveTheme) {
        ui.horizontal(|ui| {
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

        ui.horizontal(|ui| {
            let _ = primitive_action_button(ui, "Apply", PrimitiveActionKind::Primary);
            let _ = primitive_action_button(ui, "Reset", PrimitiveActionKind::Secondary);
            let _ = primitive_action_button(ui, "Delete", PrimitiveActionKind::Destructive);
        });
    }

    fn navigation_controls(&mut self, ui: &mut egui::Ui, theme: PrimitiveTheme) {
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
        primitive_tabs_header(
            ui,
            "lab-tabs",
            &mut self.selected_tab,
            &mut self.tab_focus,
            &tabs,
            theme,
        );
        primitive_tabs_content(
            ui,
            "lab-tabs-content",
            TabsContentOptions::default().min_height(48.0).theme(theme),
            |ui| {
                ui.label(format!(
                    "{} panel is selected",
                    tabs.get(self.selected_tab)
                        .map(|tab| tab.label)
                        .unwrap_or("Unknown")
                ));
            },
        );
    }

    fn text_and_security_controls(&mut self, ui: &mut egui::Ui, theme: PrimitiveTheme) {
        ui.horizontal(|ui| {
            if ui.button("OTP 1234").clicked() {
                self.otp_value = "1234".to_owned();
            }
            if ui.button("OTP full").clicked() {
                self.otp_value = "123456".to_owned();
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

        ui.horizontal(|ui| {
            if ui.button("Toggle password").clicked() {
                self.password_visible = !self.password_visible;
            }
            primitive_password_toggle_field(
                ui,
                &PasswordToggleRootOptions::default()
                    .value("s3cret")
                    .visible(self.password_visible)
                    .read_only(self.show_disabled),
                220.0,
                32.0,
                theme,
            );
        });
    }
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
        ] {
            assert!(states.contains(required), "missing state {required}");
        }
    }
}
