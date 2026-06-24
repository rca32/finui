use std::hash::Hash;

use eframe::egui::{self, FontId, Rect, Response, Vec2};

use super::{PrimitiveTheme, RadixIcon, paint_radix_icon, radix_colors};

pub struct CollapsibleOutput {
    pub response: Response,
    pub changed: bool,
}

pub struct CollapsibleRootOutput<T> {
    pub inner: T,
    pub open: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollapsibleRootOptions {
    pub open: bool,
    pub disabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for CollapsibleRootOptions {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl CollapsibleRootOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollapsibleTriggerOptions {
    pub open: bool,
    pub disabled: bool,
    pub theme: PrimitiveTheme,
}

impl Default for CollapsibleTriggerOptions {
    fn default() -> Self {
        Self {
            open: false,
            disabled: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl CollapsibleTriggerOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollapsibleContentOptions {
    pub open: bool,
    pub force_mount: bool,
    pub theme: PrimitiveTheme,
}

impl Default for CollapsibleContentOptions {
    fn default() -> Self {
        Self {
            open: false,
            force_mount: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl CollapsibleContentOptions {
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

pub struct AccordionItemOutput<T> {
    pub response: Response,
    pub changed: bool,
    pub open: bool,
    pub content: Option<T>,
}

pub struct AccordionRootOutput<T> {
    pub inner: T,
    pub multiple: bool,
    pub collapsible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccordionRootOptions {
    pub multiple: bool,
    pub collapsible: bool,
    pub theme: PrimitiveTheme,
}

impl Default for AccordionRootOptions {
    fn default() -> Self {
        Self {
            multiple: false,
            collapsible: false,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl AccordionRootOptions {
    pub fn single() -> Self {
        Self::default()
    }

    pub fn multiple() -> Self {
        Self {
            multiple: true,
            collapsible: true,
            ..Default::default()
        }
    }

    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccordionItemState {
    pub open: bool,
    pub enabled: bool,
    pub hovered: bool,
}

pub struct AccordionHeaderOutput {
    pub rect: Rect,
    pub state: AccordionItemState,
}

pub fn primitive_collapsible_header(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    open: &mut bool,
    label: &str,
    theme: PrimitiveTheme,
) -> CollapsibleOutput {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), egui::Sense::hover());
    let response = ui.interact(rect, ui.id().with(id_source), egui::Sense::click());
    let changed = response.clicked();
    if changed {
        *open = !*open;
    }
    primitive_collapsible_trigger(ui, rect, *open, response.hovered(), label, theme);
    CollapsibleOutput { response, changed }
}

pub fn primitive_collapsible_root<T>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    open: &mut bool,
    options: CollapsibleRootOptions,
    add_parts: impl FnOnce(&mut egui::Ui, &mut bool, CollapsibleRootOptions) -> T,
) -> CollapsibleRootOutput<T> {
    let inner = ui
        .push_id(id_source, |ui| add_parts(ui, open, options))
        .inner;
    CollapsibleRootOutput {
        inner,
        open: *open,
        disabled: options.disabled,
    }
}

pub fn primitive_collapsible_trigger_with_options(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    label: &str,
    open: &mut bool,
    options: CollapsibleTriggerOptions,
) -> CollapsibleOutput {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), egui::Sense::hover());
    let response = ui.interact(
        rect,
        ui.id().with(id_source),
        if options.disabled {
            egui::Sense::hover()
        } else {
            egui::Sense::click()
        },
    );
    let changed = !options.disabled && response.clicked();
    if changed {
        *open = !*open;
    }
    primitive_collapsible_trigger(
        ui,
        rect,
        *open,
        response.hovered() && !options.disabled,
        label,
        options.theme,
    );
    CollapsibleOutput { response, changed }
}

pub fn primitive_collapsible_trigger(
    ui: &egui::Ui,
    rect: Rect,
    open: bool,
    hovered: bool,
    label: &str,
    theme: PrimitiveTheme,
) {
    let fill = if open {
        theme.item_selected_fill
    } else if hovered {
        theme.item_hover_fill
    } else {
        theme.content_fill
    };
    ui.painter()
        .rect_filled(rect.shrink(1.0), theme.row_radius, fill);
    let icon = if open {
        RadixIcon::ChevronDown
    } else {
        RadixIcon::ChevronRight
    };
    paint_radix_icon(
        ui,
        icon,
        Rect::from_center_size(
            egui::pos2(rect.left() + 13.0, rect.center().y),
            Vec2::splat(14.0),
        ),
        if open { theme.text } else { theme.muted_text },
    );
    ui.painter().text(
        egui::pos2(rect.left() + 28.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        theme.text,
    );
}

pub fn primitive_collapsible_content_with_options<T>(
    ui: &mut egui::Ui,
    options: CollapsibleContentOptions,
    add_content: impl FnOnce(&mut egui::Ui) -> T,
) -> Option<T> {
    if !options.open && !options.force_mount {
        return None;
    }
    primitive_collapsible_content(ui, true, options.theme, add_content)
}

pub fn primitive_collapsible_content<T>(
    ui: &mut egui::Ui,
    open: bool,
    theme: PrimitiveTheme,
    add_content: impl FnOnce(&mut egui::Ui) -> T,
) -> Option<T> {
    if !open {
        return None;
    }
    let result = ui
        .horizontal(|ui| {
            ui.add_space(28.0);
            ui.vertical(|ui| add_content(ui)).inner
        })
        .inner;
    ui.painter().hline(
        ui.min_rect().left()..=ui.min_rect().right(),
        ui.cursor().min.y,
        theme.content_stroke,
    );
    Some(result)
}

pub fn primitive_accordion_item<T>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    index: usize,
    open_items: &mut Vec<usize>,
    multiple: bool,
    label: &str,
    theme: PrimitiveTheme,
    add_content: impl FnOnce(&mut egui::Ui) -> T,
) -> AccordionItemOutput<T> {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), egui::Sense::hover());
    let response = ui.interact(rect, ui.id().with(id_source), egui::Sense::click());
    let changed = response.clicked();
    if changed {
        accordion_apply_toggle(open_items, index, multiple);
    }
    let open = open_items.contains(&index);
    primitive_accordion_trigger(
        ui,
        rect,
        label,
        AccordionItemState {
            open,
            enabled: true,
            hovered: response.hovered(),
        },
        theme,
    );
    let content = primitive_accordion_content(ui, open, theme, add_content);

    AccordionItemOutput {
        response,
        changed,
        open,
        content,
    }
}

pub fn primitive_accordion_root<T>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    options: AccordionRootOptions,
    add_items: impl FnOnce(&mut egui::Ui, AccordionRootOptions) -> T,
) -> AccordionRootOutput<T> {
    let inner = ui.push_id(id_source, |ui| add_items(ui, options)).inner;
    AccordionRootOutput {
        inner,
        multiple: options.multiple,
        collapsible: options.collapsible,
    }
}

pub fn primitive_accordion_item_with_options<T>(
    ui: &mut egui::Ui,
    id_source: impl Hash,
    index: usize,
    open_items: &mut Vec<usize>,
    label: &str,
    enabled: bool,
    options: AccordionRootOptions,
    add_content: impl FnOnce(&mut egui::Ui) -> T,
) -> AccordionItemOutput<T> {
    let (rect, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), egui::Sense::hover());
    let response = ui.interact(
        rect,
        ui.id().with(id_source),
        if enabled {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let changed = enabled && response.clicked();
    if changed {
        accordion_apply_toggle_with_options(open_items, index, options);
    }
    let open = open_items.contains(&index);
    let state = AccordionItemState {
        open,
        enabled,
        hovered: response.hovered(),
    };
    primitive_accordion_header(ui, rect, label, state, options.theme);
    let content = primitive_accordion_content(ui, open, options.theme, add_content);

    AccordionItemOutput {
        response,
        changed,
        open,
        content,
    }
}

pub fn primitive_accordion_header(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    state: AccordionItemState,
    theme: PrimitiveTheme,
) -> AccordionHeaderOutput {
    primitive_accordion_trigger(ui, rect, label, state, theme);
    AccordionHeaderOutput { rect, state }
}

pub fn primitive_accordion_trigger(
    ui: &egui::Ui,
    rect: Rect,
    label: &str,
    state: AccordionItemState,
    theme: PrimitiveTheme,
) {
    let fill = if state.open && state.enabled {
        theme.item_selected_fill
    } else if state.hovered && state.enabled {
        theme.item_hover_fill
    } else {
        theme.content_fill
    };
    ui.painter()
        .rect_filled(rect.shrink(1.0), theme.row_radius, fill);
    let icon = if state.open {
        RadixIcon::ChevronDown
    } else {
        RadixIcon::ChevronRight
    };
    paint_radix_icon(
        ui,
        icon,
        Rect::from_center_size(
            egui::pos2(rect.left() + 13.0, rect.center().y),
            Vec2::splat(14.0),
        ),
        if state.open && state.enabled {
            theme.text
        } else {
            theme.muted_text
        },
    );
    ui.painter().text(
        egui::pos2(rect.left() + 28.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        crate::scaled_proportional_font(ui, 13.0),
        if state.open && state.enabled {
            theme.text
        } else if state.enabled {
            theme.text
        } else {
            theme.disabled_text
        },
    );
}

pub fn primitive_accordion_content<T>(
    ui: &mut egui::Ui,
    open: bool,
    theme: PrimitiveTheme,
    add_content: impl FnOnce(&mut egui::Ui) -> T,
) -> Option<T> {
    primitive_collapsible_content(ui, open, theme, add_content)
}

pub fn accordion_apply_toggle(open_items: &mut Vec<usize>, index: usize, multiple: bool) {
    accordion_apply_toggle_with_options(
        open_items,
        index,
        AccordionRootOptions {
            multiple,
            collapsible: true,
            ..Default::default()
        },
    );
}

pub fn accordion_apply_toggle_with_options(
    open_items: &mut Vec<usize>,
    index: usize,
    options: AccordionRootOptions,
) {
    if options.multiple {
        if let Some(pos) = open_items.iter().position(|item| *item == index) {
            open_items.remove(pos);
        } else {
            open_items.push(index);
            open_items.sort_unstable();
        }
    } else if open_items.len() == 1 && open_items[0] == index && options.collapsible {
        open_items.clear();
    } else {
        open_items.clear();
        open_items.push(index);
    }
}

pub fn accordion_apply_item_open(
    open_items: &mut Vec<usize>,
    index: usize,
    open: bool,
    options: AccordionRootOptions,
) -> bool {
    let contains = open_items.contains(&index);
    if options.multiple {
        if open {
            if contains {
                return false;
            }
            open_items.push(index);
            open_items.sort_unstable();
            return true;
        }
        if let Some(pos) = open_items.iter().position(|item| *item == index) {
            open_items.remove(pos);
            return true;
        }
        return false;
    }

    if open {
        if open_items.len() == 1 && open_items[0] == index {
            return false;
        }
        open_items.clear();
        open_items.push(index);
        return true;
    }

    if !contains || !options.collapsible {
        return false;
    }
    open_items.clear();
    true
}

pub fn collapsible_trigger_icon(open: bool) -> &'static str {
    if open { "v" } else { ">" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accordion_single_keeps_one_open_item() {
        let mut open = vec![0];

        accordion_apply_toggle(&mut open, 2, false);
        assert_eq!(open, vec![2]);
        accordion_apply_toggle(&mut open, 2, false);
        assert!(open.is_empty());
    }

    #[test]
    fn accordion_multiple_toggles_membership() {
        let mut open = vec![0, 2];

        accordion_apply_toggle(&mut open, 1, true);
        assert_eq!(open, vec![0, 1, 2]);
        accordion_apply_toggle(&mut open, 0, true);
        assert_eq!(open, vec![1, 2]);
    }

    #[test]
    fn accordion_apply_item_open_is_idempotent_for_single_and_multiple_roots() {
        let single = AccordionRootOptions::single().collapsible(true);
        let mut open = vec![0];

        assert!(accordion_apply_item_open(&mut open, 1, true, single));
        assert_eq!(open, vec![1]);
        assert!(!accordion_apply_item_open(&mut open, 1, true, single));
        assert_eq!(open, vec![1]);
        assert!(accordion_apply_item_open(&mut open, 1, false, single));
        assert_eq!(open, Vec::<usize>::new());

        let multiple = AccordionRootOptions::multiple();
        let mut open = vec![0];

        assert!(accordion_apply_item_open(&mut open, 2, true, multiple));
        assert_eq!(open, vec![0, 2]);
        assert!(!accordion_apply_item_open(&mut open, 2, true, multiple));
        assert!(accordion_apply_item_open(&mut open, 0, false, multiple));
        assert_eq!(open, vec![2]);
    }

    #[test]
    fn accordion_root_options_preserve_radix_single_and_multiple_contracts() {
        let single = AccordionRootOptions::single();
        let multiple = AccordionRootOptions::multiple();

        assert!(!single.multiple);
        assert!(!single.collapsible);
        assert!(multiple.multiple);
        assert!(multiple.collapsible);
    }

    #[test]
    fn accordion_toggle_with_options_respects_non_collapsible_single_root() {
        let mut open = vec![1];

        accordion_apply_toggle_with_options(&mut open, 1, AccordionRootOptions::single());
        assert_eq!(open, vec![1]);

        accordion_apply_toggle_with_options(
            &mut open,
            1,
            AccordionRootOptions::single().collapsible(true),
        );
        assert!(open.is_empty());
    }

    #[test]
    fn collapsible_trigger_icon_tracks_open_state() {
        assert_eq!(collapsible_trigger_icon(true), "v");
        assert_eq!(collapsible_trigger_icon(false), ">");
    }

    #[test]
    fn collapsible_root_options_preserve_open_disabled_and_theme() {
        let theme = PrimitiveTheme {
            row_radius: 8.0,
            ..PrimitiveTheme::default()
        };
        let options = CollapsibleRootOptions::default()
            .open(true)
            .disabled(true)
            .theme(theme);

        assert!(options.open);
        assert!(options.disabled);
        assert_eq!(options.theme.row_radius, 8.0);
    }

    #[test]
    fn collapsible_trigger_options_preserve_root_state_contract() {
        let options = CollapsibleTriggerOptions::default()
            .open(true)
            .disabled(true);

        assert!(options.open);
        assert!(options.disabled);
    }

    #[test]
    fn collapsible_content_options_preserve_force_mount_contract() {
        let options = CollapsibleContentOptions::default()
            .open(false)
            .force_mount(true);

        assert!(!options.open);
        assert!(options.force_mount);
    }

    #[test]
    fn collapsible_root_output_preserves_inner_and_state() {
        let output = CollapsibleRootOutput {
            inner: "inner",
            open: true,
            disabled: false,
        };

        assert_eq!(output.inner, "inner");
        assert!(output.open);
        assert!(!output.disabled);
    }

    #[test]
    fn accordion_item_state_preserves_part_state() {
        let state = AccordionItemState {
            open: true,
            enabled: false,
            hovered: true,
        };

        assert!(state.open);
        assert!(!state.enabled);
        assert!(state.hovered);
    }

    #[test]
    fn accordion_header_output_preserves_rect_and_item_state() {
        let rect = Rect::from_min_size(egui::pos2(4.0, 8.0), Vec2::new(120.0, 30.0));
        let state = AccordionItemState {
            open: true,
            enabled: true,
            hovered: false,
        };
        let output = AccordionHeaderOutput { rect, state };

        assert_eq!(output.rect, rect);
        assert_eq!(output.state, state);
    }
}
