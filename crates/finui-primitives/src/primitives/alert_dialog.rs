use std::hash::Hash;

use eframe::egui::{self, Align2, Color32, FontId, Response, Sense, Stroke, Vec2};

use crate::{CommandDialogOptions, show_command_dialog};

use super::{
    DialogAnnounceOptions, DialogAnnounceOutput, DialogAnnounceRole, DialogContentOptions,
    DialogDataState, DialogOverlayOptions, DialogPartStateOutput, DialogPortalOptions,
    DialogPortalOutput, DialogRootOptions, DialogRootOutput, DialogTriggerOptions, PrimitiveTheme,
    dialog_apply_open, primitive_dialog_announce_output, primitive_dialog_content_options,
    primitive_dialog_description, primitive_dialog_overlay_options, primitive_dialog_part_state,
    primitive_dialog_portal_output, primitive_dialog_root_output, primitive_dialog_title,
    primitive_dialog_trigger,
};

pub struct AlertDialogOptions {
    inner: CommandDialogOptions,
}

impl AlertDialogOptions {
    pub fn new(id: impl Hash) -> Self {
        Self {
            inner: CommandDialogOptions::new(id)
                .size(360.0, 460.0, 160.0, 240.0)
                .inner_margin(egui::Margin::same(18))
                .backdrop_tint(Color32::from_rgba_unmultiplied(12, 18, 28, 96)),
        }
    }

    pub fn size(
        mut self,
        min_width: f32,
        max_width: f32,
        min_height: f32,
        max_height: f32,
    ) -> Self {
        self.inner = self
            .inner
            .size(min_width, max_width, min_height, max_height);
        self
    }

    pub fn top_margin(mut self, top_margin: f32) -> Self {
        self.inner = self.inner.top_margin(top_margin);
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.inner = self
            .inner
            .panel_style(theme.content_fill, theme.content_stroke, theme.radius);
        self
    }

    pub fn content(mut self, content: AlertDialogContentOptions) -> Self {
        self.inner = self
            .inner
            .size(
                content.min_width,
                content.max_width,
                content.min_height,
                content.max_height,
            )
            .inner_margin(content.inner_margin);
        self
    }

    pub fn overlay(mut self, overlay: AlertDialogOverlayOptions) -> Self {
        self.inner = self.inner.backdrop_tint(overlay.tint);
        self
    }
}

pub struct AlertDialogOutput<T> {
    pub action: Option<T>,
    pub should_close: bool,
}

pub type AlertDialogTriggerOptions = DialogTriggerOptions;
pub type AlertDialogContentOptions = DialogContentOptions;
pub type AlertDialogOverlayOptions = DialogOverlayOptions;
pub type AlertDialogRootOptions = DialogRootOptions;
pub type AlertDialogRootOutput = DialogRootOutput;
pub type AlertDialogPortalOptions = DialogPortalOptions;
pub type AlertDialogPortalOutput = DialogPortalOutput;
pub type AlertDialogPartStateOutput = DialogPartStateOutput;
pub type AlertDialogAnnounceOptions = DialogAnnounceOptions;
pub type AlertDialogAnnounceOutput = DialogAnnounceOutput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertDialogActionKind {
    Action,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertDialogActionFocusPriority {
    Initial,
    Normal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlertDialogActionOptions {
    pub width: f32,
    pub height: f32,
    pub kind: AlertDialogActionKind,
    pub destructive: bool,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AlertDialogActionOutput {
    pub kind: AlertDialogActionKind,
    pub destructive: bool,
    pub focus_priority: AlertDialogActionFocusPriority,
    pub enabled: bool,
}

impl AlertDialogActionOptions {
    pub fn new(kind: AlertDialogActionKind) -> Self {
        Self {
            width: 96.0,
            height: 32.0,
            kind,
            destructive: false,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.enabled = !disabled;
        self
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

pub fn primitive_alert_dialog_root_output(
    options: AlertDialogRootOptions,
) -> AlertDialogRootOutput {
    primitive_dialog_root_output(options.modal(true))
}

pub fn alert_dialog_apply_open(
    current: &mut bool,
    next: bool,
    options: &AlertDialogRootOptions,
) -> bool {
    let modal_options = (*options).modal(true);
    dialog_apply_open(current, next, &modal_options)
}

pub fn primitive_alert_dialog_portal_output(
    options: AlertDialogPortalOptions,
) -> AlertDialogPortalOutput {
    primitive_dialog_portal_output(options)
}

pub fn primitive_alert_dialog_part_state(
    open: bool,
    force_mount: bool,
) -> AlertDialogPartStateOutput {
    primitive_dialog_part_state(open, force_mount)
}

pub fn show_alert_dialog<T>(
    ctx: &egui::Context,
    options: AlertDialogOptions,
    add_contents: impl FnOnce(&mut egui::Ui, Vec2) -> Option<T>,
) -> AlertDialogOutput<T> {
    let output = show_command_dialog(ctx, options.inner, add_contents);
    AlertDialogOutput {
        action: output.action,
        should_close: output.should_close,
    }
}

pub fn primitive_alert_dialog_trigger(
    ui: &mut egui::Ui,
    label: &str,
    options: AlertDialogTriggerOptions,
) -> Response {
    primitive_dialog_trigger(ui, label, options)
}

pub fn primitive_alert_dialog_content_options(
    min_width: f32,
    max_width: f32,
    min_height: f32,
    max_height: f32,
) -> AlertDialogContentOptions {
    primitive_dialog_content_options(min_width, max_width, min_height, max_height)
}

pub fn primitive_alert_dialog_overlay_options(tint: Color32) -> AlertDialogOverlayOptions {
    primitive_dialog_overlay_options(tint)
}

pub fn primitive_alert_dialog_title(ui: &mut egui::Ui, text: &str, theme: PrimitiveTheme) {
    primitive_dialog_title(ui, text, theme);
}

pub fn primitive_alert_dialog_description(ui: &mut egui::Ui, text: &str, theme: PrimitiveTheme) {
    primitive_dialog_description(ui, text, theme);
}

pub fn primitive_alert_dialog_announce_output(
    options: AlertDialogAnnounceOptions,
) -> AlertDialogAnnounceOutput {
    primitive_dialog_announce_output(options.role(DialogAnnounceRole::AlertDialog))
}

pub fn primitive_alert_dialog_action_output(
    options: AlertDialogActionOptions,
) -> AlertDialogActionOutput {
    AlertDialogActionOutput {
        kind: options.kind,
        destructive: options.kind == AlertDialogActionKind::Action && options.destructive,
        focus_priority: match options.kind {
            AlertDialogActionKind::Action => AlertDialogActionFocusPriority::Normal,
            AlertDialogActionKind::Cancel => AlertDialogActionFocusPriority::Initial,
        },
        enabled: options.enabled,
    }
}

pub fn primitive_alert_dialog_action(
    ui: &mut egui::Ui,
    label: &str,
    options: AlertDialogActionOptions,
) -> Response {
    primitive_alert_dialog_action_button(
        ui,
        label,
        AlertDialogActionOptions {
            kind: AlertDialogActionKind::Action,
            ..options
        },
    )
}

pub fn primitive_alert_dialog_cancel(
    ui: &mut egui::Ui,
    label: &str,
    options: AlertDialogActionOptions,
) -> Response {
    primitive_alert_dialog_action_button(
        ui,
        label,
        AlertDialogActionOptions {
            kind: AlertDialogActionKind::Cancel,
            ..options
        },
    )
}

fn primitive_alert_dialog_action_button(
    ui: &mut egui::Ui,
    label: &str,
    options: AlertDialogActionOptions,
) -> Response {
    let output = primitive_alert_dialog_action_output(options);
    let sense = if options.enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(options.width, options.height), sense);
    let response = if options.enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    };
    let fill = match (output.kind, response.hovered() && output.enabled) {
        (AlertDialogActionKind::Action, true) => options.theme.item_selected_fill,
        (AlertDialogActionKind::Action, false) => options.theme.item_hover_fill,
        (AlertDialogActionKind::Cancel, true) => options.theme.item_hover_fill,
        (AlertDialogActionKind::Cancel, false) => options.theme.content_fill,
    };
    let stroke = Stroke::new(1.0, options.theme.content_stroke.color);
    let text_color = if options.enabled {
        options.theme.text
    } else {
        options.theme.disabled_text
    };
    ui.painter().rect(
        rect,
        options.theme.row_radius,
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        text_color,
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_dialog_action_options_preserve_part_state() {
        let options = AlertDialogActionOptions::new(AlertDialogActionKind::Action)
            .size(112.0, 28.0)
            .disabled(true);

        assert_eq!(options.width, 112.0);
        assert_eq!(options.height, 28.0);
        assert_eq!(options.kind, AlertDialogActionKind::Action);
        assert!(!options.destructive);
        assert!(!options.enabled);
    }

    #[test]
    fn alert_dialog_action_options_preserve_custom_theme() {
        let theme = PrimitiveTheme {
            menu_row_height: 24.0,
            ..PrimitiveTheme::default()
        };
        let options = AlertDialogActionOptions::new(AlertDialogActionKind::Cancel).theme(theme);

        assert_eq!(options.kind, AlertDialogActionKind::Cancel);
        assert_eq!(options.theme.menu_row_height, 24.0);
    }

    #[test]
    fn alert_dialog_action_output_separates_focus_priority_and_destructive_semantics() {
        let action = primitive_alert_dialog_action_output(
            AlertDialogActionOptions::new(AlertDialogActionKind::Action).destructive(true),
        );
        let cancel = primitive_alert_dialog_action_output(AlertDialogActionOptions::new(
            AlertDialogActionKind::Cancel,
        ));
        let cancel_with_destructive_flag = primitive_alert_dialog_action_output(
            AlertDialogActionOptions::new(AlertDialogActionKind::Cancel).destructive(true),
        );

        assert_eq!(action.kind, AlertDialogActionKind::Action);
        assert!(action.destructive);
        assert_eq!(
            action.focus_priority,
            AlertDialogActionFocusPriority::Normal
        );
        assert_eq!(cancel.kind, AlertDialogActionKind::Cancel);
        assert!(!cancel.destructive);
        assert_eq!(
            cancel.focus_priority,
            AlertDialogActionFocusPriority::Initial
        );
        assert!(!cancel_with_destructive_flag.destructive);
    }

    #[test]
    fn alert_dialog_announce_output_forces_alertdialog_role() {
        let output = primitive_alert_dialog_announce_output(
            AlertDialogAnnounceOptions::new("delete-position")
                .title("Delete position")
                .description("This cannot be undone"),
        );

        assert_eq!(output.role, DialogAnnounceRole::AlertDialog);
        assert_eq!(output.role_name, "alertdialog");
        assert_eq!(output.labelled_by.as_deref(), Some("delete-position-title"));
        assert_eq!(
            output.described_by.as_deref(),
            Some("delete-position-description")
        );
        assert!(!output.missing_required_title);
    }

    #[test]
    fn alert_dialog_trigger_options_use_dialog_trigger_contract() {
        let options = AlertDialogTriggerOptions::default()
            .open(true)
            .size(120.0, 30.0);

        assert!(options.open);
        assert_eq!(options.width, 120.0);
        assert_eq!(options.height, 30.0);
    }

    #[test]
    fn alert_dialog_root_output_forces_modal_contract() {
        let output = primitive_alert_dialog_root_output(
            AlertDialogRootOptions::default()
                .open(true)
                .default_open(false)
                .modal(false),
        );

        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert!(output.modal);
        assert_eq!(output.data_state, DialogDataState::Open);
    }

    #[test]
    fn alert_dialog_apply_open_respects_noop_state_and_forced_modal_contract() {
        let options = AlertDialogRootOptions::default().open(false).modal(false);
        let mut open = false;

        assert!(!alert_dialog_apply_open(&mut open, false, &options));
        assert!(!open);
        assert!(alert_dialog_apply_open(&mut open, true, &options));
        assert!(open);

        let output = primitive_alert_dialog_root_output(options.open(open));
        assert!(output.open);
        assert!(output.modal);
    }

    #[test]
    fn alert_dialog_portal_output_preserves_force_mount_and_container() {
        let output = primitive_alert_dialog_portal_output(
            AlertDialogPortalOptions::default()
                .force_mount(true)
                .container("alert-dialog-root"),
        );

        assert!(output.force_mount);
        assert_eq!(output.container.as_deref(), Some("alert-dialog-root"));
    }

    #[test]
    fn alert_dialog_part_state_uses_dialog_data_state_contract() {
        let output = primitive_alert_dialog_part_state(false, true);

        assert_eq!(output.data_state, DialogDataState::Closed);
        assert!(output.force_mount);
        assert!(output.mounted);
    }

    #[test]
    fn alert_dialog_title_and_description_share_dialog_typography_contract() {
        let theme = PrimitiveTheme::default();

        primitive_alert_dialog_title as fn(&mut egui::Ui, &str, PrimitiveTheme);
        primitive_alert_dialog_description as fn(&mut egui::Ui, &str, PrimitiveTheme);
        assert_eq!(theme.text, PrimitiveTheme::default().text);
    }

    #[test]
    fn alert_dialog_content_options_share_dialog_content_contract() {
        let options = primitive_alert_dialog_content_options(360.0, 460.0, 160.0, 240.0)
            .inner_margin(egui::Margin::same(18));

        assert_eq!(options.min_width, 360.0);
        assert_eq!(options.max_width, 460.0);
        assert_eq!(options.min_height, 160.0);
        assert_eq!(options.max_height, 240.0);
        assert_eq!(options.inner_margin.left, 18);
    }

    #[test]
    fn alert_dialog_overlay_options_share_dialog_overlay_contract() {
        let tint = Color32::from_rgba_unmultiplied(12, 18, 28, 96);
        let options = primitive_alert_dialog_overlay_options(tint);

        assert_eq!(options.tint, tint);
    }
}
