use std::hash::Hash;

use eframe::egui::{self, Align2, Color32, FontId, Response, Sense, Stroke, Vec2};

use crate::{CommandDialogOptions, show_command_dialog};

use super::PrimitiveTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogDataState {
    Open,
    Closed,
}

impl DialogDataState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Closed => "closed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogRootOptions {
    pub open: bool,
    pub default_open: Option<bool>,
    pub modal: bool,
}

impl Default for DialogRootOptions {
    fn default() -> Self {
        Self {
            open: false,
            default_open: None,
            modal: true,
        }
    }
}

impl DialogRootOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn default_open(mut self, default_open: bool) -> Self {
        self.default_open = Some(default_open);
        self
    }

    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogRootOutput {
    pub open: bool,
    pub default_open: Option<bool>,
    pub modal: bool,
    pub data_state: DialogDataState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogTriggerOptions {
    pub width: f32,
    pub height: f32,
    pub open: bool,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for DialogTriggerOptions {
    fn default() -> Self {
        Self {
            width: 112.0,
            height: 32.0,
            open: false,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl DialogTriggerOptions {
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.enabled = !disabled;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DialogCloseOptions {
    pub size: f32,
    pub enabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for DialogCloseOptions {
    fn default() -> Self {
        Self {
            size: 28.0,
            enabled: true,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl DialogCloseOptions {
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.enabled = !disabled;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogContentOptions {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
    pub inner_margin: egui::Margin,
    pub force_mount: bool,
    pub open: bool,
}

impl DialogContentOptions {
    pub fn new(min_width: f32, max_width: f32, min_height: f32, max_height: f32) -> Self {
        Self {
            min_width,
            max_width,
            min_height,
            max_height,
            inner_margin: egui::Margin::same(12),
            force_mount: false,
            open: true,
        }
    }

    pub fn inner_margin(mut self, inner_margin: egui::Margin) -> Self {
        self.inner_margin = inner_margin;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogOverlayOptions {
    pub tint: Color32,
    pub force_mount: bool,
    pub open: bool,
}

impl Default for DialogOverlayOptions {
    fn default() -> Self {
        Self {
            tint: Color32::from_rgba_unmultiplied(12, 18, 28, 72),
            force_mount: false,
            open: true,
        }
    }
}

impl DialogOverlayOptions {
    pub fn tint(mut self, tint: Color32) -> Self {
        self.tint = tint;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DialogPortalOptions {
    pub force_mount: bool,
    pub container: Option<String>,
}

impl Default for DialogPortalOptions {
    fn default() -> Self {
        Self {
            force_mount: false,
            container: None,
        }
    }
}

impl DialogPortalOptions {
    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn container(mut self, container: impl Into<String>) -> Self {
        self.container = Some(container.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DialogPortalOutput {
    pub force_mount: bool,
    pub container: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogPartStateOutput {
    pub data_state: DialogDataState,
    pub force_mount: bool,
    pub mounted: bool,
}

pub struct DialogOptions {
    inner: CommandDialogOptions,
}

impl DialogOptions {
    pub fn new(id: impl Hash) -> Self {
        Self {
            inner: CommandDialogOptions::new(id),
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

    pub fn inner_margin(mut self, inner_margin: egui::Margin) -> Self {
        self.inner = self.inner.inner_margin(inner_margin);
        self
    }

    pub fn backdrop_tint(mut self, tint: Color32) -> Self {
        self.inner = self.inner.backdrop_tint(tint);
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.inner = self
            .inner
            .panel_style(theme.content_fill, theme.content_stroke, theme.radius);
        self
    }

    pub fn content(mut self, content: DialogContentOptions) -> Self {
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

    pub fn overlay(mut self, overlay: DialogOverlayOptions) -> Self {
        self.inner = self.inner.backdrop_tint(overlay.tint);
        self
    }
}

pub struct DialogOutput {
    pub should_close: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogAnnounceRole {
    Dialog,
    AlertDialog,
}

impl DialogAnnounceRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dialog => "dialog",
            Self::AlertDialog => "alertdialog",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogTitleVisibility {
    Visible,
    VisuallyHidden,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogAnnounceOptions {
    pub content_id: String,
    pub role: DialogAnnounceRole,
    pub title: Option<String>,
    pub title_visibility: DialogTitleVisibility,
    pub description: Option<String>,
    pub title_required: bool,
}

impl DialogAnnounceOptions {
    pub fn new(content_id: impl Into<String>) -> Self {
        Self {
            content_id: content_id.into(),
            role: DialogAnnounceRole::Dialog,
            title: None,
            title_visibility: DialogTitleVisibility::Visible,
            description: None,
            title_required: true,
        }
    }

    pub fn role(mut self, role: DialogAnnounceRole) -> Self {
        self.role = role;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn visually_hidden_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self.title_visibility = DialogTitleVisibility::VisuallyHidden;
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn title_required(mut self, title_required: bool) -> Self {
        self.title_required = title_required;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DialogAnnounceOutput {
    pub role: DialogAnnounceRole,
    pub role_name: &'static str,
    pub content_id: String,
    pub title_id: Option<String>,
    pub description_id: Option<String>,
    pub labelled_by: Option<String>,
    pub described_by: Option<String>,
    pub accessible_name: Option<String>,
    pub accessible_description: Option<String>,
    pub title_required: bool,
    pub title_present: bool,
    pub title_visible: bool,
    pub title_visually_hidden: bool,
    pub description_present: bool,
    pub missing_required_title: bool,
}

pub fn primitive_dialog_root_output(options: DialogRootOptions) -> DialogRootOutput {
    DialogRootOutput {
        open: options.open,
        default_open: options.default_open,
        modal: options.modal,
        data_state: if options.open {
            DialogDataState::Open
        } else {
            DialogDataState::Closed
        },
    }
}

pub fn primitive_dialog_announce_output(options: DialogAnnounceOptions) -> DialogAnnounceOutput {
    let title_present = options.title.is_some();
    let description_present = options.description.is_some();
    let title_id = title_present.then(|| format!("{}-title", options.content_id));
    let description_id = description_present.then(|| format!("{}-description", options.content_id));
    let title_visible = title_present && options.title_visibility == DialogTitleVisibility::Visible;
    let title_visually_hidden =
        title_present && options.title_visibility == DialogTitleVisibility::VisuallyHidden;

    DialogAnnounceOutput {
        role: options.role,
        role_name: options.role.as_str(),
        content_id: options.content_id,
        title_id: title_id.clone(),
        description_id: description_id.clone(),
        labelled_by: title_id,
        described_by: description_id,
        accessible_name: options.title,
        accessible_description: options.description,
        title_required: options.title_required,
        title_present,
        title_visible,
        title_visually_hidden,
        description_present,
        missing_required_title: options.title_required && !title_present,
    }
}

pub fn dialog_apply_open(current: &mut bool, next: bool, options: &DialogRootOptions) -> bool {
    let output = primitive_dialog_root_output((*options).open(*current));
    if output.open == next {
        return false;
    }
    *current = next;
    true
}

pub fn primitive_dialog_portal_output(options: DialogPortalOptions) -> DialogPortalOutput {
    DialogPortalOutput {
        force_mount: options.force_mount,
        container: options.container,
    }
}

pub fn primitive_dialog_part_state(open: bool, force_mount: bool) -> DialogPartStateOutput {
    DialogPartStateOutput {
        data_state: if open {
            DialogDataState::Open
        } else {
            DialogDataState::Closed
        },
        force_mount,
        mounted: open || force_mount,
    }
}

pub fn show_dialog<T>(
    ctx: &egui::Context,
    options: DialogOptions,
    add_contents: impl FnOnce(&mut egui::Ui, Vec2) -> Option<T>,
) -> DialogOutput {
    let output = show_command_dialog(ctx, options.inner, add_contents);
    DialogOutput {
        should_close: output.should_close,
    }
}

pub fn primitive_dialog_trigger(
    ui: &mut egui::Ui,
    label: &str,
    options: DialogTriggerOptions,
) -> Response {
    let trigger_state = primitive_dialog_part_state(options.open, false);
    let sense = if options.enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(options.width, options.height), sense);
    let fill = if trigger_state.data_state == DialogDataState::Open
        || (response.hovered() && options.enabled)
    {
        options.theme.item_hover_fill
    } else {
        options.theme.content_fill
    };
    let text_color = if options.enabled {
        options.theme.text
    } else {
        options.theme.disabled_text
    };
    ui.painter().rect(
        rect,
        options.theme.row_radius,
        fill,
        options.theme.content_stroke,
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

pub fn primitive_dialog_content_options(
    min_width: f32,
    max_width: f32,
    min_height: f32,
    max_height: f32,
) -> DialogContentOptions {
    DialogContentOptions::new(min_width, max_width, min_height, max_height)
}

pub fn primitive_dialog_overlay_options(tint: Color32) -> DialogOverlayOptions {
    DialogOverlayOptions::default().tint(tint)
}

pub fn primitive_dialog_title(ui: &mut egui::Ui, text: &str, theme: PrimitiveTheme) {
    ui.label(
        egui::RichText::new(text)
            .font(crate::scaled_proportional_font(ui, 15.0))
            .color(theme.text),
    );
}

pub fn primitive_dialog_description(ui: &mut egui::Ui, text: &str, theme: PrimitiveTheme) {
    ui.label(
        egui::RichText::new(text)
            .font(crate::scaled_proportional_font(ui, 12.0))
            .color(theme.muted_text),
    );
}

pub fn primitive_dialog_close_button(
    ui: &mut egui::Ui,
    _id_source: impl Hash,
    options: DialogCloseOptions,
) -> Response {
    let size = options.size.max(18.0);
    let sense = if options.enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), sense);
    let response = response.on_hover_text("Close");
    let fill = if response.hovered() && options.enabled {
        options.theme.item_hover_fill
    } else {
        Color32::TRANSPARENT
    };
    let color = if options.enabled {
        options.theme.text
    } else {
        options.theme.disabled_text
    };
    let stroke = Stroke::new(1.0, color);

    ui.painter().rect(
        rect,
        options.theme.row_radius,
        fill,
        Stroke::NONE,
        egui::StrokeKind::Inside,
    );
    let center = rect.center();
    let half = size * 0.18;
    ui.painter().line_segment(
        [
            center + Vec2::new(-half, -half),
            center + Vec2::new(half, half),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            center + Vec2::new(half, -half),
            center + Vec2::new(-half, half),
        ],
        stroke,
    );
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialog_close_options_preserve_size_and_disabled_state() {
        let options = DialogCloseOptions::default().size(24.0).disabled(true);

        assert_eq!(options.size, 24.0);
        assert!(!options.enabled);
    }

    #[test]
    fn dialog_trigger_options_preserve_root_state() {
        let options = DialogTriggerOptions::default()
            .size(96.0, 28.0)
            .open(true)
            .disabled(true);

        assert_eq!(options.width, 96.0);
        assert_eq!(options.height, 28.0);
        assert!(options.open);
        assert!(!options.enabled);
    }

    #[test]
    fn dialog_root_output_preserves_open_default_and_modal_contract() {
        let output = primitive_dialog_root_output(
            DialogRootOptions::default()
                .open(true)
                .default_open(false)
                .modal(false),
        );

        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert!(!output.modal);
        assert_eq!(output.data_state, DialogDataState::Open);
        assert_eq!(output.data_state.as_str(), "open");
    }

    #[test]
    fn dialog_announce_output_requires_title_and_allows_description_omission() {
        let missing = primitive_dialog_announce_output(DialogAnnounceOptions::new("settings"));
        let described = primitive_dialog_announce_output(
            DialogAnnounceOptions::new("settings")
                .title("Settings")
                .description("Update workspace preferences"),
        );
        let title_only = primitive_dialog_announce_output(
            DialogAnnounceOptions::new("confirm").title("Confirm changes"),
        );

        assert_eq!(missing.role, DialogAnnounceRole::Dialog);
        assert_eq!(missing.role_name, "dialog");
        assert!(missing.title_required);
        assert!(!missing.title_present);
        assert!(missing.missing_required_title);
        assert_eq!(missing.labelled_by, None);
        assert_eq!(missing.described_by, None);

        assert_eq!(described.title_id.as_deref(), Some("settings-title"));
        assert_eq!(
            described.description_id.as_deref(),
            Some("settings-description")
        );
        assert_eq!(described.labelled_by.as_deref(), Some("settings-title"));
        assert_eq!(
            described.described_by.as_deref(),
            Some("settings-description")
        );
        assert_eq!(described.accessible_name.as_deref(), Some("Settings"));
        assert_eq!(
            described.accessible_description.as_deref(),
            Some("Update workspace preferences")
        );
        assert!(described.title_visible);
        assert!(!described.title_visually_hidden);
        assert!(!described.missing_required_title);

        assert_eq!(title_only.described_by, None);
        assert!(!title_only.description_present);
        assert!(!title_only.missing_required_title);
    }

    #[test]
    fn dialog_announce_output_supports_visually_hidden_title() {
        let output = primitive_dialog_announce_output(
            DialogAnnounceOptions::new("filter").visually_hidden_title("Filter rows"),
        );

        assert_eq!(output.labelled_by.as_deref(), Some("filter-title"));
        assert_eq!(output.accessible_name.as_deref(), Some("Filter rows"));
        assert!(output.title_present);
        assert!(!output.title_visible);
        assert!(output.title_visually_hidden);
        assert!(!output.missing_required_title);
    }

    #[test]
    fn dialog_apply_open_respects_noop_state_and_root_contract() {
        let options = DialogRootOptions::default()
            .open(false)
            .default_open(false)
            .modal(false);
        let mut open = false;

        assert!(!dialog_apply_open(&mut open, false, &options));
        assert!(!open);
        assert!(dialog_apply_open(&mut open, true, &options));
        assert!(open);

        let output = primitive_dialog_root_output(options.open(open));
        assert!(output.open);
        assert_eq!(output.default_open, Some(false));
        assert!(!output.modal);
        assert_eq!(output.data_state, DialogDataState::Open);
    }

    #[test]
    fn dialog_portal_output_preserves_force_mount_and_container_contract() {
        let output = primitive_dialog_portal_output(
            DialogPortalOptions::default()
                .force_mount(true)
                .container("primitive-demo-root"),
        );

        assert!(output.force_mount);
        assert_eq!(output.container.as_deref(), Some("primitive-demo-root"));
    }

    #[test]
    fn dialog_part_state_mounts_when_open_or_force_mounted() {
        let closed = primitive_dialog_part_state(false, false);
        let forced = primitive_dialog_part_state(false, true);

        assert_eq!(closed.data_state, DialogDataState::Closed);
        assert!(!closed.mounted);
        assert_eq!(forced.data_state, DialogDataState::Closed);
        assert!(forced.force_mount);
        assert!(forced.mounted);
    }

    #[test]
    fn dialog_content_options_preserve_content_bounds_and_margin() {
        let options = primitive_dialog_content_options(620.0, 820.0, 520.0, 720.0)
            .inner_margin(egui::Margin::same(18))
            .force_mount(true)
            .open(false);

        assert_eq!(options.min_width, 620.0);
        assert_eq!(options.max_width, 820.0);
        assert_eq!(options.min_height, 520.0);
        assert_eq!(options.max_height, 720.0);
        assert_eq!(options.inner_margin.left, 18);
        assert!(options.force_mount);
        assert!(!options.open);
    }

    #[test]
    fn dialog_overlay_options_preserve_tint() {
        let tint = Color32::from_rgba_unmultiplied(1, 2, 3, 4);
        let options = primitive_dialog_overlay_options(tint)
            .force_mount(true)
            .open(false);

        assert_eq!(options.tint, tint);
        assert!(options.force_mount);
        assert!(!options.open);
    }
}
