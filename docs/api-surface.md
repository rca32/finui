# API Surface

Finui keeps the public API intentionally narrow.

Stability levels and breaking-change rules are defined in
`docs/api-stability-policy.md`.

## `finui-primitives`

Public primitives expose:

- option structs for caller-owned state
- output structs for accessibility, geometry, and state transitions
- pure helpers for keyboard, geometry, and color decisions
- egui render helpers for primitive parts

Primitive-specific reference documents:

- `docs/primitive-anatomy.md`
- `docs/primitive-api-reference.md`
- `docs/primitive-accessibility.md`
- `docs/primitive-keyboard-interactions.md`
- `docs/controlled-uncontrolled-examples.md`
- `docs/radix-parity-matrix.md`
- `docs/text-overflow-policy.md`
- `docs/agent-ux-receipts.md`
- `docs/unstyled-theme-boundary.md`
- `docs/accessibility-snapshots.md`
- `docs/visual-regression-baseline.md`

Application-specific shell, market data, terminal, api, or chart runtime code
does not belong in this crate.

## `finui-grid`

Public grid API is centered on:

- `FinancialDataGrid`
- `FinancialDataGridBuilder`
- column, cell, row id, state, action, provenance, and source types
- in-memory, streaming, and virtual row sources
- demo and fixture helpers while the crate is pre-release

Internal row-model, viewport, persistence, and export helpers may become narrower
before the first published version. Prefer building through `FinancialDataGrid`
unless a helper is explicitly documented.

## `finui-workbench` Preview API

The workbench crate currently exposes a Preview API centered on:

- `WorkbenchState` and serializable `WorkbenchNode` split/tab trees
- `PaneConstraints` and pure `calculate_workbench_geometry`
- caller-applied `WorkbenchAction` outputs
- `FocusRoute` and `CommandScopeOutput`
- `show_workbench` for renderer-neutral egui application shells

The `experimental` feature additionally exposes `WorkbenchOptions` plus
`calculate_workbench_geometry_with_options` and `show_workbench_with_options`.
Callers can opt into hiding a redundant tab strip when a tab region owns exactly
one panel; the default Preview API keeps the existing visible tab strip.

The state schema and behavior are exercised by `editor_lab`, but can change
before promotion to the stable surface. See `docs/workbench-preview-api.md`.

## `finui-timeline` Preview API

The timeline crate exposes integer-tick snapshots, a revision-keyed geometry
cache, viewport geometry, hit targets, transactional drag actions, and an
agent-readable UX receipt. Paint and hit-test share the same culled clip list.

## `finui-media-surface` Preview API

The media surface crate accepts an external `egui::TextureId`, computes
aspect-fit and transform-overlay geometry, and emits begin/update/commit/cancel
actions without mutating the caller's transform. See
`docs/timeline-media-surface-preview-api.md` for both Preview contracts.

## Fixtures

`fixtures` keeps demo payload adapters out of the core grid contract. Disable it
with:

```powershell
cargo check -p finui-grid --no-default-features
```
