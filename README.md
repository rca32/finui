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

```powershell
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets
cargo test --workspace
cargo check -p finui-grid --no-default-features
```

## Examples

```powershell
cargo run -p grid_lab
cargo run -p primitives_lab
```

## Feature Boundaries

`finui-grid` keeps generic row-source contracts in core:

- `GridRowSource`
- `InMemoryGridSource`
- `StreamingGridSource`
- `VirtualGridSource`

Endpoint and DuckDB-style table payload fixtures are behind the `fixtures` feature.
The default feature keeps the demo convenient, while `--no-default-features` proves
the core grid is not coupled to those fixtures.

## License

Apache-2.0. Radix icon assets under `crates/finui-primitives/assets/radix-icons`
retain their upstream MIT license notice.
