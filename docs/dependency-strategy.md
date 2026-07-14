# Dependency Strategy

Finui is currently pre-release. Downstream applications should prefer a git
dependency pinned to a commit until the crates are published.

Recommended pre-release dependency shape:

```toml
finui-primitives = { git = "https://github.com/rca32/finui", rev = "<commit>" }
finui-grid = { git = "https://github.com/rca32/finui", rev = "<commit>" }
```

After the public API stabilizes, downstream applications can move to crates.io
versions for `finui-primitives` and `finui-grid`.

## UI Toolkit And Runtime Boundary

Finui library crates depend directly on `egui`. They do not depend on `eframe`,
`glow`, `wgpu`, or an egui renderer crate. This keeps reusable component types
independent from application window and GPU lifecycle choices.

Applications select their renderer explicitly:

```toml
# OpenGL runtime used by the existing labs.
eframe = { workspace = true, features = ["glow"] }

# wgpu runtime used by editor_lab and Palmier.
eframe = { workspace = true, features = ["wgpu"] }
```

`scripts/check-library-boundary.ps1` walks each `finui-*` normal dependency tree
and rejects `eframe`, `glow`, `wgpu`, `egui-wgpu`, or `egui_glow`. Both the quick
and full check paths run this gate. `scripts/check-backend-selection.ps1` also
proves that the existing labs resolve `egui_glow` without `egui-wgpu`, while
`editor_lab` resolves `egui-wgpu` without `egui_glow`.
