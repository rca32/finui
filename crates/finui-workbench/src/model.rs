use std::{collections::HashSet, error::Error, fmt};

use serde::{Deserialize, Serialize};

pub const WORKBENCH_LAYOUT_SCHEMA_VERSION: u32 = 1;

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
    };
}

string_id!(PanelId);
string_id!(RegionId);
string_id!(SplitId);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelTab {
    pub id: PanelId,
    pub title: String,
}

impl PanelTab {
    pub fn new(id: impl Into<PanelId>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaneConstraints {
    pub min_points: f32,
    pub max_points: Option<f32>,
}

impl PaneConstraints {
    pub const fn new(min_points: f32, max_points: Option<f32>) -> Self {
        Self {
            min_points,
            max_points,
        }
    }

    pub const fn minimum(min_points: f32) -> Self {
        Self::new(min_points, None)
    }

    pub const fn bounded(min_points: f32, max_points: f32) -> Self {
        Self::new(min_points, Some(max_points))
    }
}

impl Default for PaneConstraints {
    fn default() -> Self {
        Self::minimum(0.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitAxis {
    /// Places child panes left-to-right with a vertical divider.
    Horizontal,
    /// Places child panes top-to-bottom with a horizontal divider.
    Vertical,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WorkbenchNode {
    Split {
        id: SplitId,
        axis: SplitAxis,
        fraction: f32,
        first_constraints: PaneConstraints,
        second_constraints: PaneConstraints,
        first: Box<WorkbenchNode>,
        second: Box<WorkbenchNode>,
    },
    Tabs {
        id: RegionId,
        tabs: Vec<PanelTab>,
        active: PanelId,
    },
}

impl WorkbenchNode {
    pub fn split(
        id: impl Into<SplitId>,
        axis: SplitAxis,
        fraction: f32,
        first_constraints: PaneConstraints,
        second_constraints: PaneConstraints,
        first: WorkbenchNode,
        second: WorkbenchNode,
    ) -> Self {
        Self::Split {
            id: id.into(),
            axis,
            fraction,
            first_constraints,
            second_constraints,
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    pub fn tabs(id: impl Into<RegionId>, tabs: Vec<PanelTab>, active: impl Into<PanelId>) -> Self {
        Self::Tabs {
            id: id.into(),
            tabs,
            active: active.into(),
        }
    }

    fn contains_panel(&self, panel_id: &PanelId) -> bool {
        match self {
            Self::Split { first, second, .. } => {
                first.contains_panel(panel_id) || second.contains_panel(panel_id)
            }
            Self::Tabs { tabs, .. } => tabs.iter().any(|tab| &tab.id == panel_id),
        }
    }

    fn resize_split(&mut self, split_id: &SplitId, fraction: f32) -> bool {
        if !fraction.is_finite() {
            return false;
        }
        match self {
            Self::Split {
                id,
                fraction: current,
                first,
                second,
                ..
            } => {
                if id == split_id {
                    let next = fraction.clamp(0.0, 1.0);
                    if *current != next {
                        *current = next;
                        return true;
                    }
                    return false;
                }
                first.resize_split(split_id, fraction) || second.resize_split(split_id, fraction)
            }
            Self::Tabs { .. } => false,
        }
    }

    fn activate_tab(&mut self, region_id: &RegionId, panel_id: &PanelId) -> bool {
        match self {
            Self::Split { first, second, .. } => {
                first.activate_tab(region_id, panel_id) || second.activate_tab(region_id, panel_id)
            }
            Self::Tabs { id, tabs, active } => {
                if id != region_id || !tabs.iter().any(|tab| &tab.id == panel_id) {
                    return false;
                }
                if active == panel_id {
                    return false;
                }
                *active = panel_id.clone();
                true
            }
        }
    }

    fn focus_route(&self, panel_id: &PanelId, split_path: &mut Vec<SplitId>) -> Option<FocusRoute> {
        match self {
            Self::Split {
                id, first, second, ..
            } => {
                split_path.push(id.clone());
                let route = first
                    .focus_route(panel_id, split_path)
                    .or_else(|| second.focus_route(panel_id, split_path));
                split_path.pop();
                route
            }
            Self::Tabs { id, tabs, .. } => {
                tabs.iter()
                    .any(|tab| &tab.id == panel_id)
                    .then(|| FocusRoute {
                        split_path: split_path.clone(),
                        region_id: id.clone(),
                        panel_id: panel_id.clone(),
                    })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkbenchState {
    pub schema_version: u32,
    pub root: WorkbenchNode,
    pub focused_panel: Option<PanelId>,
}

impl WorkbenchState {
    pub fn new(root: WorkbenchNode) -> Self {
        Self {
            schema_version: WORKBENCH_LAYOUT_SCHEMA_VERSION,
            root,
            focused_panel: None,
        }
    }

    pub fn with_focused_panel(mut self, panel_id: impl Into<PanelId>) -> Self {
        self.focused_panel = Some(panel_id.into());
        self
    }

    pub fn apply(&mut self, action: &WorkbenchAction) -> bool {
        match action {
            WorkbenchAction::ResizeSplit { split_id, fraction } => {
                self.root.resize_split(split_id, *fraction)
            }
            WorkbenchAction::ActivateTab {
                region_id,
                panel_id,
            } => self.root.activate_tab(region_id, panel_id),
            WorkbenchAction::FocusPanel { panel_id } => {
                if !self.root.contains_panel(panel_id)
                    || self.focused_panel.as_ref() == Some(panel_id)
                {
                    return false;
                }
                self.focused_panel = Some(panel_id.clone());
                true
            }
        }
    }

    pub fn apply_all<'a>(&mut self, actions: impl IntoIterator<Item = &'a WorkbenchAction>) {
        for action in actions {
            self.apply(action);
        }
    }

    pub fn focus_route(&self) -> Option<FocusRoute> {
        self.focused_panel
            .as_ref()
            .and_then(|panel_id| self.root.focus_route(panel_id, &mut Vec::new()))
    }

    pub fn command_scope(&self) -> CommandScopeOutput {
        self.focus_route()
            .map_or(CommandScopeOutput::Global, |route| {
                CommandScopeOutput::Panel {
                    region_id: route.region_id,
                    panel_id: route.panel_id,
                }
            })
    }

    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, LayoutRestoreError> {
        let state: Self = serde_json::from_str(json).map_err(LayoutRestoreError::Json)?;
        state.validate().map_err(LayoutRestoreError::Invalid)?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<(), LayoutValidationError> {
        if self.schema_version != WORKBENCH_LAYOUT_SCHEMA_VERSION {
            return Err(LayoutValidationError::UnsupportedSchemaVersion(
                self.schema_version,
            ));
        }

        let mut node_ids = HashSet::new();
        let mut panel_ids = HashSet::new();
        validate_node(&self.root, &mut node_ids, &mut panel_ids)?;
        if let Some(focused) = self.focused_panel.as_ref()
            && !panel_ids.contains(focused.as_str())
        {
            return Err(LayoutValidationError::FocusedPanelMissing(focused.clone()));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WorkbenchAction {
    ResizeSplit {
        split_id: SplitId,
        fraction: f32,
    },
    ActivateTab {
        region_id: RegionId,
        panel_id: PanelId,
    },
    FocusPanel {
        panel_id: PanelId,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FocusRoute {
    pub split_path: Vec<SplitId>,
    pub region_id: RegionId,
    pub panel_id: PanelId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandScopeOutput {
    Global,
    Panel {
        region_id: RegionId,
        panel_id: PanelId,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum LayoutValidationError {
    UnsupportedSchemaVersion(u32),
    DuplicateNodeId(String),
    DuplicatePanelId(PanelId),
    EmptyTabs(RegionId),
    ActivePanelMissing {
        region_id: RegionId,
        panel_id: PanelId,
    },
    InvalidFraction(SplitId),
    InvalidConstraints(SplitId),
    FocusedPanelMissing(PanelId),
}

impl fmt::Display for LayoutValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion(version) => {
                write!(formatter, "unsupported workbench schema version {version}")
            }
            Self::DuplicateNodeId(id) => write!(formatter, "duplicate workbench node id {id}"),
            Self::DuplicatePanelId(id) => write!(formatter, "duplicate panel id {id}"),
            Self::EmptyTabs(id) => write!(formatter, "tab region {id} has no panels"),
            Self::ActivePanelMissing {
                region_id,
                panel_id,
            } => write!(
                formatter,
                "active panel {panel_id} is missing from region {region_id}"
            ),
            Self::InvalidFraction(id) => write!(formatter, "split {id} has an invalid fraction"),
            Self::InvalidConstraints(id) => {
                write!(formatter, "split {id} has invalid pane constraints")
            }
            Self::FocusedPanelMissing(id) => {
                write!(formatter, "focused panel {id} is not in the layout")
            }
        }
    }
}

impl Error for LayoutValidationError {}

#[derive(Debug)]
pub enum LayoutRestoreError {
    Json(serde_json::Error),
    Invalid(LayoutValidationError),
}

impl fmt::Display for LayoutRestoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(formatter, "invalid workbench JSON: {error}"),
            Self::Invalid(error) => error.fmt(formatter),
        }
    }
}

impl Error for LayoutRestoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Invalid(error) => Some(error),
        }
    }
}

fn validate_node<'a>(
    node: &'a WorkbenchNode,
    node_ids: &mut HashSet<&'a str>,
    panel_ids: &mut HashSet<&'a str>,
) -> Result<(), LayoutValidationError> {
    match node {
        WorkbenchNode::Split {
            id,
            fraction,
            first_constraints,
            second_constraints,
            first,
            second,
            ..
        } => {
            if !node_ids.insert(id.as_str()) {
                return Err(LayoutValidationError::DuplicateNodeId(id.to_string()));
            }
            if !fraction.is_finite() || !(0.0..=1.0).contains(fraction) {
                return Err(LayoutValidationError::InvalidFraction(id.clone()));
            }
            if !constraints_are_valid(*first_constraints)
                || !constraints_are_valid(*second_constraints)
            {
                return Err(LayoutValidationError::InvalidConstraints(id.clone()));
            }
            validate_node(first, node_ids, panel_ids)?;
            validate_node(second, node_ids, panel_ids)
        }
        WorkbenchNode::Tabs { id, tabs, active } => {
            if !node_ids.insert(id.as_str()) {
                return Err(LayoutValidationError::DuplicateNodeId(id.to_string()));
            }
            if tabs.is_empty() {
                return Err(LayoutValidationError::EmptyTabs(id.clone()));
            }
            for tab in tabs {
                if !panel_ids.insert(tab.id.as_str()) {
                    return Err(LayoutValidationError::DuplicatePanelId(tab.id.clone()));
                }
            }
            if !tabs.iter().any(|tab| tab.id == *active) {
                return Err(LayoutValidationError::ActivePanelMissing {
                    region_id: id.clone(),
                    panel_id: active.clone(),
                });
            }
            Ok(())
        }
    }
}

fn constraints_are_valid(constraints: PaneConstraints) -> bool {
    constraints.min_points.is_finite()
        && constraints.min_points >= 0.0
        && constraints
            .max_points
            .is_none_or(|max| max.is_finite() && max >= constraints.min_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_state() -> WorkbenchState {
        WorkbenchState::new(WorkbenchNode::split(
            "root",
            SplitAxis::Horizontal,
            0.7,
            PaneConstraints::minimum(200.0),
            PaneConstraints::bounded(180.0, 360.0),
            WorkbenchNode::tabs(
                "main",
                vec![
                    PanelTab::new("preview", "Preview"),
                    PanelTab::new("media", "Media"),
                ],
                "preview",
            ),
            WorkbenchNode::tabs(
                "right",
                vec![PanelTab::new("inspector", "Inspector")],
                "inspector",
            ),
        ))
        .with_focused_panel("preview")
    }

    #[test]
    fn layout_state_json_round_trips() {
        let state = sample_state();
        let json = state.to_json_pretty().unwrap();
        let restored = WorkbenchState::from_json(&json).unwrap();
        assert_eq!(restored, state);
        assert!(json.contains("\"schema_version\": 1"));
    }

    #[test]
    fn typed_actions_preserve_caller_owned_state_and_scope() {
        let state = sample_state();
        let mut projected = state.clone();
        let actions = [
            WorkbenchAction::ActivateTab {
                region_id: RegionId::from("main"),
                panel_id: PanelId::from("media"),
            },
            WorkbenchAction::FocusPanel {
                panel_id: PanelId::from("media"),
            },
        ];
        projected.apply_all(&actions);

        assert_eq!(
            state.command_scope(),
            CommandScopeOutput::Panel {
                region_id: RegionId::from("main"),
                panel_id: PanelId::from("preview"),
            }
        );
        assert_eq!(
            projected.command_scope(),
            CommandScopeOutput::Panel {
                region_id: RegionId::from("main"),
                panel_id: PanelId::from("media"),
            }
        );
    }

    #[test]
    fn invalid_active_panel_is_rejected_on_restore() {
        let mut state = sample_state();
        let WorkbenchNode::Split { first, .. } = &mut state.root else {
            unreachable!();
        };
        let WorkbenchNode::Tabs { active, .. } = first.as_mut() else {
            unreachable!();
        };
        *active = PanelId::from("missing");
        let json = state.to_json_pretty().unwrap();
        assert!(WorkbenchState::from_json(&json).is_err());
    }
}
