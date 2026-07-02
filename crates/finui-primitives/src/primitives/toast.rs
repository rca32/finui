use std::time::{Duration, Instant};

use eframe::egui::{self, Align2, Color32, FontId, Response, Sense, Stroke, Vec2};

use super::{PrimitiveTheme, RadixIcon, paint_radix_icon, radix_colors};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToastKind {
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Foreground,
    Background,
}

impl ToastType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Foreground => "foreground",
            Self::Background => "background",
        }
    }

    pub fn aria_live(self) -> &'static str {
        match self {
            Self::Foreground => "assertive",
            Self::Background => "polite",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastSwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ToastSwipeDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastSwipeState {
    Start,
    Move,
    Cancel,
    End,
}

impl ToastSwipeState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Move => "move",
            Self::Cancel => "cancel",
            Self::End => "end",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToastMessage {
    pub id: u64,
    pub kind: ToastKind,
    pub title: String,
    pub description: Option<String>,
    pub action_label: Option<String>,
    pub action_alt_text: Option<String>,
    pub created_at: Instant,
    pub ttl: Duration,
}

impl ToastMessage {
    pub fn is_expired(&self, now: Instant) -> bool {
        now.duration_since(self.created_at) >= self.ttl
    }
}

#[derive(Debug, Default)]
pub struct ToastStore {
    messages: Vec<ToastMessage>,
    next_id: u64,
}

impl ToastStore {
    pub fn push(
        &mut self,
        kind: ToastKind,
        title: impl Into<String>,
        ttl: Duration,
        now: Instant,
    ) -> u64 {
        self.push_detailed(kind, title, None::<String>, None::<String>, ttl, now)
    }

    pub fn push_detailed(
        &mut self,
        kind: ToastKind,
        title: impl Into<String>,
        description: Option<impl Into<String>>,
        action_label: Option<impl Into<String>>,
        ttl: Duration,
        now: Instant,
    ) -> u64 {
        self.push_accessible(
            kind,
            title,
            description,
            action_label,
            None::<String>,
            ttl,
            now,
        )
    }

    pub fn push_accessible(
        &mut self,
        kind: ToastKind,
        title: impl Into<String>,
        description: Option<impl Into<String>>,
        action_label: Option<impl Into<String>>,
        action_alt_text: Option<impl Into<String>>,
        ttl: Duration,
        now: Instant,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.messages.push(ToastMessage {
            id,
            kind,
            title: title.into(),
            description: description.map(Into::into),
            action_label: action_label.map(Into::into),
            action_alt_text: action_alt_text.map(Into::into),
            created_at: now,
            ttl,
        });
        id
    }

    pub fn push_front_accessible(
        &mut self,
        kind: ToastKind,
        title: impl Into<String>,
        description: Option<impl Into<String>>,
        action_label: Option<impl Into<String>>,
        action_alt_text: Option<impl Into<String>>,
        ttl: Duration,
        now: Instant,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.messages.insert(
            0,
            ToastMessage {
                id,
                kind,
                title: title.into(),
                description: description.map(Into::into),
                action_label: action_label.map(Into::into),
                action_alt_text: action_alt_text.map(Into::into),
                created_at: now,
                ttl,
            },
        );
        id
    }

    pub fn dismiss(&mut self, id: u64) -> bool {
        let before = self.messages.len();
        self.messages.retain(|message| message.id != id);
        before != self.messages.len()
    }

    pub fn complete_action(&mut self, id: u64) -> bool {
        self.dismiss(id)
    }

    pub fn retain_active(&mut self, now: Instant) {
        self.messages.retain(|message| !message.is_expired(now));
    }

    pub fn messages(&self) -> &[ToastMessage] {
        &self.messages
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastAction {
    Action(u64),
    Close(u64),
}

#[derive(Debug)]
pub struct ToastOutput {
    pub response: Response,
    pub action: Option<ToastAction>,
    pub root: ToastRootOutput,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToastRootOptions {
    pub width: f32,
    pub compact_height: f32,
    pub expanded_height: f32,
    pub toast_type: ToastType,
    pub duration: Option<Duration>,
    pub open: bool,
    pub default_open: bool,
    pub force_mount: bool,
    pub swipe_direction: ToastSwipeDirection,
    pub swipe_state: Option<ToastSwipeState>,
    pub theme: PrimitiveTheme,
}

impl Default for ToastRootOptions {
    fn default() -> Self {
        Self {
            width: 300.0,
            compact_height: 42.0,
            expanded_height: 86.0,
            toast_type: ToastType::Foreground,
            duration: None,
            open: true,
            default_open: true,
            force_mount: false,
            swipe_direction: ToastSwipeDirection::Right,
            swipe_state: None,
            theme: PrimitiveTheme::default(),
        }
    }
}

impl ToastRootOptions {
    pub fn toast_type(mut self, toast_type: ToastType) -> Self {
        self.toast_type = toast_type;
        self
    }

    pub fn duration(mut self, duration: Option<Duration>) -> Self {
        self.duration = duration;
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn force_mount(mut self, force_mount: bool) -> Self {
        self.force_mount = force_mount;
        self
    }

    pub fn swipe_direction(mut self, swipe_direction: ToastSwipeDirection) -> Self {
        self.swipe_direction = swipe_direction;
        self
    }

    pub fn swipe_state(mut self, swipe_state: Option<ToastSwipeState>) -> Self {
        self.swipe_state = swipe_state;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToastRootOutput {
    pub open: bool,
    pub default_open: bool,
    pub force_mount: bool,
    pub toast_type: &'static str,
    pub duration: Option<Duration>,
    pub data_state: &'static str,
    pub data_swipe: Option<&'static str>,
    pub data_swipe_direction: &'static str,
    pub role: &'static str,
    pub aria_live: &'static str,
    pub action_alt_text: Option<String>,
    pub close_aria_label: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct ToastAnnounceOptions<'a> {
    pub message: &'a ToastMessage,
    pub toast_type: ToastType,
}

impl<'a> ToastAnnounceOptions<'a> {
    pub fn new(message: &'a ToastMessage, toast_type: ToastType) -> Self {
        Self {
            message,
            toast_type,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToastAnnounceOutput {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub role: &'static str,
    pub aria_live: &'static str,
    pub priority: u8,
    pub action_alt_text: Option<String>,
    pub close_aria_label: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToastViewportOptions {
    pub max_width: f32,
    pub gap: f32,
    pub hotkey: &'static [&'static str],
    pub label: &'static str,
    pub theme: PrimitiveTheme,
}

const DEFAULT_TOAST_HOTKEY: &[&str] = &["F8"];

impl Default for ToastViewportOptions {
    fn default() -> Self {
        Self {
            max_width: 300.0,
            gap: 6.0,
            hotkey: DEFAULT_TOAST_HOTKEY,
            label: "Notifications ({hotkey})",
            theme: PrimitiveTheme::default(),
        }
    }
}

impl ToastViewportOptions {
    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn hotkey(mut self, hotkey: &'static [&'static str]) -> Self {
        self.hotkey = hotkey;
        self
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    pub fn theme(mut self, theme: PrimitiveTheme) -> Self {
        self.theme = theme;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToastViewportOutput {
    pub hotkey: Vec<&'static str>,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToastViewportFocusOutput {
    pub hotkey: Vec<&'static str>,
    pub label: String,
    pub focus_requested: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToastProviderOptions {
    pub viewport: ToastViewportOptions,
    pub duration: Duration,
    pub label: &'static str,
    pub swipe_direction: ToastSwipeDirection,
    pub swipe_threshold: f32,
    pub pause_on_hover: bool,
}

impl Default for ToastProviderOptions {
    fn default() -> Self {
        Self {
            viewport: ToastViewportOptions::default(),
            duration: Duration::from_secs(5),
            label: "Notification",
            swipe_direction: ToastSwipeDirection::Right,
            swipe_threshold: 50.0,
            pause_on_hover: true,
        }
    }
}

impl ToastProviderOptions {
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn label(mut self, label: &'static str) -> Self {
        self.label = label;
        self
    }

    pub fn swipe_direction(mut self, swipe_direction: ToastSwipeDirection) -> Self {
        self.swipe_direction = swipe_direction;
        self
    }

    pub fn swipe_threshold(mut self, swipe_threshold: f32) -> Self {
        self.swipe_threshold = swipe_threshold;
        self
    }

    pub fn viewport(mut self, viewport: ToastViewportOptions) -> Self {
        self.viewport = viewport;
        self
    }

    pub fn pause_on_hover(mut self, pause_on_hover: bool) -> Self {
        self.pause_on_hover = pause_on_hover;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastProviderOutput {
    pub duration: Duration,
    pub label: &'static str,
    pub swipe_direction: &'static str,
    pub swipe_threshold: f32,
    pub pause_on_hover: bool,
    pub viewport: ToastViewportOutput,
}

pub fn primitive_toast_provider_output(options: ToastProviderOptions) -> ToastProviderOutput {
    ToastProviderOutput {
        duration: options.duration,
        label: options.label,
        swipe_direction: options.swipe_direction.as_str(),
        swipe_threshold: options.swipe_threshold,
        pause_on_hover: options.pause_on_hover,
        viewport: primitive_toast_viewport_output(options.viewport),
    }
}

pub fn primitive_toast_provider(
    ui: &mut egui::Ui,
    store: &mut ToastStore,
    now: Instant,
    options: ToastProviderOptions,
) -> Vec<ToastOutput> {
    store.retain_active(now);
    let outputs = primitive_toast_viewport(ui, store, options.viewport);
    if options.pause_on_hover && outputs.iter().any(|output| output.response.hovered()) {
        ui.ctx().request_repaint_after(Duration::from_millis(250));
    }
    outputs
}

pub fn primitive_toast_list(ui: &mut egui::Ui, store: &ToastStore) -> Vec<ToastOutput> {
    primitive_toast_viewport(ui, store, ToastViewportOptions::default())
}

pub fn primitive_toast_viewport(
    ui: &mut egui::Ui,
    store: &ToastStore,
    options: ToastViewportOptions,
) -> Vec<ToastOutput> {
    ui.set_max_width(options.max_width);
    let mut outputs = Vec::with_capacity(store.messages().len());
    for (index, message) in store.messages().iter().enumerate() {
        if index > 0 {
            ui.add_space(options.gap);
        }
        outputs.push(primitive_toast(ui, message, options.theme));
    }
    outputs
}

pub fn primitive_toast_viewport_output(options: ToastViewportOptions) -> ToastViewportOutput {
    let hotkey = options.hotkey.to_vec();
    let label = options.label.replace("{hotkey}", &hotkey.join("+"));
    ToastViewportOutput { hotkey, label }
}

pub fn primitive_toast_viewport_focus_output(
    options: ToastViewportOptions,
    pressed_keys: &[&str],
) -> ToastViewportFocusOutput {
    let viewport = primitive_toast_viewport_output(options);
    let focus_requested = viewport.hotkey.as_slice() == pressed_keys;
    ToastViewportFocusOutput {
        hotkey: viewport.hotkey,
        label: viewport.label,
        focus_requested,
    }
}

pub fn primitive_toast_announce_output(options: ToastAnnounceOptions<'_>) -> ToastAnnounceOutput {
    ToastAnnounceOutput {
        id: options.message.id,
        title: options.message.title.clone(),
        description: options.message.description.clone(),
        role: "status",
        aria_live: options.toast_type.aria_live(),
        priority: toast_announce_priority(options.toast_type),
        action_alt_text: options.message.action_alt_text.clone(),
        close_aria_label: "Close",
    }
}

pub fn primitive_toast_announce_queue<'a>(
    options: impl IntoIterator<Item = ToastAnnounceOptions<'a>>,
) -> Vec<ToastAnnounceOutput> {
    let mut items = options
        .into_iter()
        .enumerate()
        .map(|(index, options)| {
            (
                toast_announce_priority(options.toast_type),
                index,
                primitive_toast_announce_output(options),
            )
        })
        .collect::<Vec<_>>();
    items.sort_by_key(|(priority, index, _)| (*priority, *index));
    items.into_iter().map(|(_, _, output)| output).collect()
}

fn toast_announce_priority(toast_type: ToastType) -> u8 {
    match toast_type {
        ToastType::Foreground => 0,
        ToastType::Background => 1,
    }
}

pub fn primitive_toast(
    ui: &mut egui::Ui,
    message: &ToastMessage,
    theme: PrimitiveTheme,
) -> ToastOutput {
    let options = ToastRootOptions::default().theme(theme);
    let height = toast_root_height(message, options);
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(options.width, height), egui::Sense::hover());
    primitive_toast_root(ui, rect, options);
    let marker = toast_kind_color(&message.kind);
    let accent_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left(), rect.top() + 6.0),
        egui::pos2(rect.left() + 3.0, rect.bottom() - 6.0),
    );
    ui.painter()
        .rect_filled(accent_rect, options.theme.row_radius, marker);
    ui.painter().circle_filled(
        egui::pos2(rect.left() + 17.0, rect.top() + 21.0),
        3.5,
        marker,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 28.0, rect.top() + 15.0),
        Align2::LEFT_CENTER,
        toast_kind_label(&message.kind),
        crate::scaled_monospace_font(ui, 9.5),
        options.theme.muted_text,
    );
    primitive_toast_title(
        ui,
        egui::pos2(rect.left() + 28.0, rect.top() + 31.0),
        &message.title,
        theme,
    );
    if let Some(description) = &message.description {
        primitive_toast_description(
            ui,
            egui::pos2(rect.left() + 28.0, rect.top() + 52.0),
            description,
            theme,
        );
    } else {
        ui.painter().text(
            egui::pos2(rect.right() - 38.0, rect.center().y),
            Align2::RIGHT_CENTER,
            format!("#{}", message.id),
            crate::scaled_monospace_font(ui, 10.0),
            theme.muted_text,
        );
    }

    let close_rect = egui::Rect::from_min_size(
        egui::pos2(rect.right() - 30.0, rect.top() + 8.0),
        Vec2::splat(22.0),
    );
    let close_response = primitive_toast_close(ui, close_rect, theme);
    let mut action = close_response
        .clicked()
        .then_some(ToastAction::Close(message.id));

    if let Some(action_label) = &message.action_label {
        let action_width = (action_label.chars().count() as f32 * 7.0 + 28.0).clamp(58.0, 92.0);
        let action_rect = egui::Rect::from_min_size(
            egui::pos2(rect.right() - action_width - 36.0, rect.bottom() - 31.0),
            Vec2::new(action_width, 22.0),
        );
        let action_response = primitive_toast_action(ui, action_rect, action_label, theme);
        if action_response.clicked() {
            action = Some(ToastAction::Action(message.id));
        }
    }

    ToastOutput {
        response,
        action,
        root: primitive_toast_root_output(message, options),
    }
}

pub fn primitive_toast_root(ui: &egui::Ui, rect: egui::Rect, options: ToastRootOptions) {
    ui.painter().rect(
        rect,
        options.theme.row_radius,
        if options.toast_type == ToastType::Foreground {
            options.theme.content_fill
        } else {
            options.theme.item_hover_fill
        },
        options.theme.content_stroke,
        egui::StrokeKind::Inside,
    );
}

pub fn toast_root_height(message: &ToastMessage, options: ToastRootOptions) -> f32 {
    if message.description.is_some() || message.action_label.is_some() {
        options.expanded_height
    } else {
        options.compact_height
    }
}

pub fn primitive_toast_root_output(
    message: &ToastMessage,
    options: ToastRootOptions,
) -> ToastRootOutput {
    let open = options.open || options.force_mount;
    ToastRootOutput {
        open,
        default_open: options.default_open,
        force_mount: options.force_mount,
        toast_type: options.toast_type.as_str(),
        duration: options.duration.or(Some(message.ttl)),
        data_state: if open { "open" } else { "closed" },
        data_swipe: options.swipe_state.map(ToastSwipeState::as_str),
        data_swipe_direction: options.swipe_direction.as_str(),
        role: "status",
        aria_live: options.toast_type.aria_live(),
        action_alt_text: message.action_alt_text.clone(),
        close_aria_label: "Close",
    }
}

pub fn primitive_toast_title(ui: &egui::Ui, pos: egui::Pos2, text: &str, theme: PrimitiveTheme) {
    ui.painter().text(
        pos,
        Align2::LEFT_CENTER,
        text,
        crate::scaled_proportional_font(ui, 13.0),
        theme.text,
    );
}

pub fn primitive_toast_description(
    ui: &egui::Ui,
    pos: egui::Pos2,
    text: &str,
    theme: PrimitiveTheme,
) {
    ui.painter().text(
        pos,
        Align2::LEFT_CENTER,
        text,
        crate::scaled_proportional_font(ui, 12.0),
        theme.muted_text,
    );
}

pub fn primitive_toast_action(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    label: &str,
    theme: PrimitiveTheme,
) -> Response {
    let response = ui.interact(rect, toast_part_id(ui, "action", rect), Sense::click());
    let fill = if response.hovered() {
        theme.item_selected_fill
    } else {
        theme.item_selected_fill
    };
    ui.painter().rect(
        rect,
        theme.row_radius,
        fill,
        theme.content_stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        crate::scaled_proportional_font(ui, 12.0),
        theme.text,
    );
    response
}

pub fn primitive_toast_close(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    theme: PrimitiveTheme,
) -> Response {
    let response = ui
        .interact(rect, toast_part_id(ui, "close", rect), Sense::click())
        .on_hover_text("Close");
    let fill = if response.hovered() {
        theme.item_hover_fill
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect(
        rect,
        theme.row_radius,
        fill,
        Stroke::NONE,
        egui::StrokeKind::Inside,
    );
    paint_radix_icon(ui, RadixIcon::Cross2, rect.shrink(4.0), theme.muted_text);
    response
}

fn toast_part_id(ui: &egui::Ui, part: &'static str, rect: egui::Rect) -> egui::Id {
    ui.id().with((
        "toast",
        part,
        rect.min.x.to_bits(),
        rect.min.y.to_bits(),
        rect.max.x.to_bits(),
        rect.max.y.to_bits(),
    ))
}

pub fn toast_kind_color(kind: &ToastKind) -> Color32 {
    match kind {
        ToastKind::Info => radix_colors::INDIGO_9,
        ToastKind::Success => radix_colors::GREEN_9,
        ToastKind::Warning => radix_colors::AMBER_9,
        ToastKind::Error => radix_colors::TOMATO_9,
    }
}

fn toast_kind_label(kind: &ToastKind) -> &'static str {
    match kind {
        ToastKind::Info => "INFO",
        ToastKind::Success => "SUCCESS",
        ToastKind::Warning => "WARNING",
        ToastKind::Error => "ERROR",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_store_expires_old_messages() {
        let start = Instant::now();
        let mut store = ToastStore::default();

        store.push(ToastKind::Info, "Saved", Duration::from_secs(2), start);
        store.retain_active(start + Duration::from_secs(3));

        assert!(store.messages().is_empty());
    }

    #[test]
    fn toast_kind_colors_are_distinct() {
        assert_ne!(
            toast_kind_color(&ToastKind::Info),
            toast_kind_color(&ToastKind::Error)
        );
    }

    #[test]
    fn toast_store_push_detailed_preserves_parts() {
        let start = Instant::now();
        let mut store = ToastStore::default();

        let id = store.push_detailed(
            ToastKind::Success,
            "Saved",
            Some("2 changes"),
            Some("Undo"),
            Duration::from_secs(5),
            start,
        );

        assert_eq!(id, 0);
        assert_eq!(
            store.messages()[0].description.as_deref(),
            Some("2 changes")
        );
        assert_eq!(store.messages()[0].action_label.as_deref(), Some("Undo"));
    }

    #[test]
    fn toast_store_push_accessible_preserves_action_alt_text() {
        let start = Instant::now();
        let mut store = ToastStore::default();

        store.push_accessible(
            ToastKind::Success,
            "Saved",
            Some("2 changes"),
            Some("Undo"),
            Some("Undo the last save"),
            Duration::from_secs(5),
            start,
        );

        assert_eq!(
            store.messages()[0].action_alt_text.as_deref(),
            Some("Undo the last save")
        );
    }

    #[test]
    fn toast_store_push_front_accessible_prioritizes_latest_message() {
        let start = Instant::now();
        let mut store = ToastStore::default();

        store.push_accessible(
            ToastKind::Info,
            "Ready",
            None::<String>,
            None::<String>,
            None::<String>,
            Duration::from_secs(5),
            start,
        );
        let id = store.push_front_accessible(
            ToastKind::Warning,
            "Manual demo toast",
            Some("Action and close buttons are part renderers"),
            Some("Undo"),
            Some("Undo the manual demo toast action"),
            Duration::from_secs(5),
            start,
        );

        assert_eq!(id, 1);
        assert_eq!(store.messages()[0].title, "Manual demo toast");
        assert_eq!(store.messages()[1].title, "Ready");
    }

    #[test]
    fn toast_store_dismiss_removes_matching_message() {
        let start = Instant::now();
        let mut store = ToastStore::default();

        let id = store.push(ToastKind::Info, "Saved", Duration::from_secs(5), start);

        assert!(store.dismiss(id));
        assert!(store.messages().is_empty());
    }

    #[test]
    fn toast_store_complete_action_removes_matching_message() {
        let start = Instant::now();
        let mut store = ToastStore::default();

        let id = store.push_accessible(
            ToastKind::Warning,
            "Manual demo toast",
            Some("Action and close buttons are part renderers"),
            Some("Undo"),
            Some("Undo the manual demo toast action"),
            Duration::from_secs(5),
            start,
        );

        assert!(store.complete_action(id));
        assert!(!store.messages().iter().any(|message| message.id == id));
    }

    #[test]
    fn toast_root_height_tracks_content_parts() {
        let start = Instant::now();
        let mut store = ToastStore::default();
        store.push(ToastKind::Info, "Saved", Duration::from_secs(5), start);
        store.push_detailed(
            ToastKind::Success,
            "Saved",
            Some("2 changes"),
            Some("Undo"),
            Duration::from_secs(5),
            start,
        );

        let options = ToastRootOptions::default();
        assert_eq!(toast_root_height(&store.messages()[0], options), 42.0);
        assert_eq!(toast_root_height(&store.messages()[1], options), 86.0);
    }

    #[test]
    fn toast_viewport_options_preserve_layout_contract() {
        let options = ToastViewportOptions::default().max_width(320.0).gap(8.0);

        assert_eq!(options.max_width, 320.0);
        assert_eq!(options.gap, 8.0);
    }

    #[test]
    fn toast_provider_options_preserve_viewport_and_pause_policy() {
        let viewport = ToastViewportOptions::default().max_width(280.0).gap(4.0);
        let options = ToastProviderOptions::default()
            .viewport(viewport)
            .pause_on_hover(false);

        assert_eq!(options.viewport, viewport);
        assert!(!options.pause_on_hover);
    }

    #[test]
    fn toast_provider_output_exposes_radix_contract() {
        let options = ToastProviderOptions::default()
            .label("Chart notice")
            .duration(Duration::from_secs(3))
            .swipe_direction(ToastSwipeDirection::Left)
            .swipe_threshold(64.0)
            .viewport(
                ToastViewportOptions::default()
                    .hotkey(&["altKey", "KeyT"])
                    .label("Alerts ({hotkey})"),
            );

        let output = primitive_toast_provider_output(options);

        assert_eq!(output.duration, Duration::from_secs(3));
        assert_eq!(output.label, "Chart notice");
        assert_eq!(output.swipe_direction, "left");
        assert_eq!(output.swipe_threshold, 64.0);
        assert_eq!(output.viewport.hotkey, vec!["altKey", "KeyT"]);
        assert_eq!(output.viewport.label, "Alerts (altKey+KeyT)");
    }

    #[test]
    fn toast_announce_queue_prioritizes_foreground_and_preserves_accessible_actions() {
        let start = Instant::now();
        let mut store = ToastStore::default();
        store.push_accessible(
            ToastKind::Info,
            "Background sync",
            Some("Portfolio prices refreshed"),
            None::<String>,
            None::<String>,
            Duration::from_secs(5),
            start,
        );
        store.push_accessible(
            ToastKind::Error,
            "Order rejected",
            Some("Limit price is stale"),
            Some("Review"),
            Some("Review rejected order"),
            Duration::from_secs(5),
            start,
        );

        let queue = primitive_toast_announce_queue([
            ToastAnnounceOptions::new(&store.messages()[0], ToastType::Background),
            ToastAnnounceOptions::new(&store.messages()[1], ToastType::Foreground),
        ]);

        assert_eq!(queue.len(), 2);
        assert_eq!(queue[0].title, "Order rejected");
        assert_eq!(queue[0].aria_live, "assertive");
        assert_eq!(queue[0].priority, 0);
        assert_eq!(
            queue[0].action_alt_text.as_deref(),
            Some("Review rejected order")
        );
        assert_eq!(queue[0].close_aria_label, "Close");
        assert_eq!(queue[1].title, "Background sync");
        assert_eq!(queue[1].aria_live, "polite");
        assert_eq!(queue[1].priority, 1);
    }

    #[test]
    fn toast_viewport_hotkey_output_requests_focus_only_for_matching_chord() {
        let options = ToastViewportOptions::default()
            .hotkey(&["altKey", "KeyT"])
            .label("Alerts ({hotkey})");

        let focused = primitive_toast_viewport_focus_output(options, &["altKey", "KeyT"]);
        let ignored = primitive_toast_viewport_focus_output(options, &["KeyT"]);

        assert_eq!(focused.hotkey, vec!["altKey", "KeyT"]);
        assert_eq!(focused.label, "Alerts (altKey+KeyT)");
        assert!(focused.focus_requested);
        assert!(!ignored.focus_requested);
    }

    #[test]
    fn toast_root_output_exposes_state_swipe_and_accessibility_contract() {
        let start = Instant::now();
        let mut store = ToastStore::default();
        store.push_accessible(
            ToastKind::Info,
            "Saved",
            Some("2 changes"),
            Some("Undo"),
            Some("Undo last change"),
            Duration::from_secs(5),
            start,
        );

        let output = primitive_toast_root_output(
            &store.messages()[0],
            ToastRootOptions::default()
                .toast_type(ToastType::Background)
                .duration(Some(Duration::from_secs(2)))
                .open(false)
                .force_mount(true)
                .swipe_direction(ToastSwipeDirection::Down)
                .swipe_state(Some(ToastSwipeState::Move)),
        );

        assert!(output.open);
        assert_eq!(output.toast_type, "background");
        assert_eq!(output.duration, Some(Duration::from_secs(2)));
        assert_eq!(output.data_state, "open");
        assert_eq!(output.data_swipe, Some("move"));
        assert_eq!(output.data_swipe_direction, "down");
        assert_eq!(output.aria_live, "polite");
        assert_eq!(output.action_alt_text.as_deref(), Some("Undo last change"));
        assert_eq!(output.close_aria_label, "Close");
    }
}
