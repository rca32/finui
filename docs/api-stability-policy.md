# API Stability Policy

Finui is pre-release, but public surface changes still need explicit intent. This
policy defines which APIs downstream applications can depend on and how breaking
changes are handled before the first crates.io release.

## Stability Levels

| Level | Meaning | Allowed locations |
| --- | --- | --- |
| Stable | Intended for downstream application code. Changes require compatibility or an explicit breaking-change note. | crate root exports, documented builders, options, outputs, row sources, primitive part helpers |
| Preview | Usable, but the shape may change while Radix parity or grid workflows are still being proven. | newly added primitives, lab-facing helpers, agent-testable receipts, demo-oriented APIs |
| Experimental | Exploratory APIs that can be renamed, moved, or removed without migration support before release. | prototypes, incomplete Radix ports, feature-gated experiments |
| Internal | Not a downstream contract even if Rust visibility is currently public for tests or examples. | implementation modules, layout internals, fixture adapters, rendering details |

## Stable Surface

Stable APIs are the types and functions documented in `docs/api-surface.md` and
exported for normal application use:

- `finui-primitives` option structs, output structs, state helpers, geometry
  helpers, and primitive part render helpers used to compose Radix-style controls.
- `finui-grid` grid builder, cell/row/column/state/action/provenance types, row
  source traits and implementations, export options, and agent bridge contracts.

Adding fields to an output struct is allowed when existing callers can ignore the
new field. Removing fields, changing enum semantics, renaming constructors, or
changing state transition behavior is treated as breaking.

## Preview And Experimental Surface

Preview APIs must be called out in docs or examples before downstream apps adopt
them. They should include focused tests before promotion to stable.

Experimental APIs should use at least one of these signals:

- a doc comment or docs entry that says `experimental`
- a feature flag
- a module path or type name that makes the preview status explicit
- an examples-only location

`OTP` and `PasswordToggle` stay preview until their form semantics, accessibility
outputs, and acceptance tests are promoted alongside the rest of the primitive
catalogue.

## Breaking Changes

Before the first published release, breaking changes are allowed when they reduce
ambiguity or bring the API closer to the documented Radix parity target. A change
is breaking when downstream code must update names, constructors, fields, feature
flags, state semantics, or expected output values.

Breaking changes require:

- a commit message that names the affected crate or primitive
- an update to `docs/api-surface.md` or this policy when the stable/preview
  boundary changes
- tests for the replacement behavior when behavior changes
- a migration note in the pull request or release notes when publishing outside
  this repository

After the first crates.io release, breaking changes should wait for the next
minor version while the crates are below `1.0`, and for the next major version
after `1.0`.

## Feature Policy

Default features should keep examples convenient without coupling core contracts
to fixtures or demo payloads. Core behavior must keep compiling with:

```powershell
cargo check -p finui-grid --no-default-features
```

New features must document whether they expose stable, preview, experimental, or
internal API. Feature-gated fixtures and demos should not become required by core
grid or primitive contracts.
