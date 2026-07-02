# Finui

Finui is a Rust workspace for dense, testable financial user interfaces built on
`egui`.

The first release candidates are:

- `finui-primitives`: Radix-style primitive controls for immediate-mode egui apps.
- `finui-grid`: an agent-testable financial data grid with typed cells, row sources,
  provenance metadata, sorting, filtering, virtual sources, export helpers, and demo
  fixtures.

## Status

This repository is pre-release. APIs are being narrowed before crates.io publication.
Use git dependencies while the surface is stabilizing.

## Workspace

```text
crates/
  finui-primitives/
  finui-grid/
examples/
  grid_lab/
  primitives_lab/
docs/
```

## Quick Check

Use the quick path during ordinary development. It keeps format, compile, feature
boundary, and crate-level lib tests fast.

```powershell
powershell -ExecutionPolicy Bypass -File scripts/check-quick.ps1
```

Use the full path before publishing or opening a broad pull request. It includes
the quick gates plus workspace clippy and workspace tests.

```powershell
powershell -ExecutionPolicy Bypass -File scripts/check-full.ps1
```

## Examples

```powershell
cargo run -p grid_lab
cargo run -p primitives_lab
```

## Screenshots And Visual Baselines

The checked-in images show the current target surfaces for dense financial
workflows and Radix-style primitive visual states. They live in `docs/images/` so
README previews and visual baselines stay close to the code.

### Grid Lab

![Finui grid lab sample](docs/images/grid-lab.png)

`grid_lab` exercises the financial grid surface: typed cells, dense row layout,
sorting/filtering affordances, row-source metadata, and demo fixtures that mimic
table-like endpoint payloads.

### Primitive Visual Baseline: Light States

![Finui primitive lab light state sample](docs/images/primitives-lab-light.png)

This is a stable visual-regression baseline for light-mode primitive states,
including open, disabled, and long-text coverage. It is not a live window capture;
it is the image used to anchor regression thresholds in
`docs/visual-regression-baseline.md`.

### Primitive Visual Baseline: Dark And RTL States

![Finui primitive lab dark and RTL state sample](docs/images/primitives-lab-dark.png)

The dark baseline keeps contrast, disabled treatment, and right-to-left state
coverage visible from the top-level project documentation.

### Primitive Visual Baseline: Layer States

![Finui primitive lab layer state sample](docs/images/primitives-lab-layers.png)

The layer baseline focuses on open-layer and edge-placement coverage. Use it when
checking popup, menu, dialog, and placement behavior against the Radix-style UX
target.

## Feature Boundaries

`finui-grid` keeps generic row-source contracts in core:

- `GridRowSource`
- `InMemoryGridSource`
- `StreamingGridSource`
- `VirtualGridSource`

Endpoint and DuckDB-style table payload fixtures are behind the `fixtures` feature.
The default feature keeps the demo convenient, while `--no-default-features` proves
the core grid is not coupled to those fixtures.

API stability levels, preview/internal boundaries, and breaking-change rules are
defined in `docs/api-stability-policy.md`.

## License

Apache-2.0. Radix icon assets under `crates/finui-primitives/assets/radix-icons`
retain their upstream MIT license notice.
