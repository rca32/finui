# Workbench Preview API

`finui-workbench` is a renderer-neutral Preview API for editor shells. It depends
on `egui`, `serde`, and `serde_json`; it does not depend on `eframe`, `glow`,
`wgpu`, or an application domain model.

## State And Output Contract

The caller owns `WorkbenchState`. `show_workbench` receives an immutable state,
paints the current layout, and returns typed actions:

- `ResizeSplit`
- `ActivateTab`
- `FocusPanel`

The widget projects those actions to produce the frame's `FocusRoute` and
`CommandScopeOutput`, but it never mutates the supplied state. The application
may accept, reject, log, undo, or apply each action with `WorkbenchState::apply`.

`CommandScopeOutput::Panel` identifies the focused panel and its tab region.
When no panel is focused it falls back to `Global`. `FocusRoute` also includes
the ordered split ancestors, so an application can route commands or keyboard
focus without inspecting paint geometry.

## Split Geometry And Constraints

A `WorkbenchNode` is either a split or a tab region. Horizontal splits place
children left-to-right; vertical splits place them top-to-bottom. Each split
stores a caller-owned fraction and min/max constraints for both child subtrees.

Geometry clamps the divider to all feasible constraints. If a viewport cannot
satisfy both children's limits, `DividerGeometry::constraints_satisfied` is
false and the divider remains inside a valid 0..1 range. Applications can use
that output to adapt or report undersized windows.

The visible divider is 1 logical point. Its interaction rectangle remains 8
logical points at every scale, which yields 8, 12, and 16 physical pixels at
100%, 150%, and 200% DPI. Unit tests cover all three scales.

## Persistence

With the opt-in `experimental` feature, `WorkbenchState` serializes a versioned
split/tab tree with active tabs, split fractions, constraints, and focused panel.
`to_json_pretty` and `from_json` provide the persistence boundary. Restore validates:

- schema version
- unique split, region, and panel IDs
- finite fractions and valid min/max constraints
- non-empty tab regions and valid active tabs
- a focused panel that exists in the tree

The schema version is currently `2`. Schema 1 migrates to schema 2; missing,
unsupported, and future versions return distinct errors. Round-trip equality and
v1 migration are acceptance tests.

## Keyboard transport

The default `preview` feature exposes renderer-neutral keyboard transport actions.
Space emits Play/Pause and Left/Right emit exact one-frame steps only when the
caller marks the transport surface focused. FinUI returns typed actions and never
mutates playback state.

## Editor Lab Layout

`editor_lab` uses four tab regions and three resizable splits:

```text
+---------+--------------------------+-------------+
| Media   | Preview                  | Inspector   |
|         | GPU-native texture       | Agent (tab) |
+---------+--------------------------+-------------+
| Timeline                                            |
+-----------------------------------------------------+
```

The five panels are generic mock application content. Palmier types and media
runtime types remain outside `finui-workbench`.
