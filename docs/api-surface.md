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

## Fixtures

`fixtures` keeps demo payload adapters out of the core grid contract. Disable it
with:

```powershell
cargo check -p finui-grid --no-default-features
```
