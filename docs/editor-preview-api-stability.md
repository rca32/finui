# Editor Preview API stability manifest

The editor-facing crates use Cargo features to make their compatibility boundary
machine-verifiable.

| Crate | Default feature | Opt-in feature |
| --- | --- | --- |
| `finui-workbench` | `preview` | `experimental` |
| `finui-timeline` | `preview` | `experimental` |
| `finui-media-surface` | `preview` | `experimental` |

`preview` is enabled by default. It provides caller-owned snapshots, typed actions,
geometry, widgets, semantic receipts, and keyboard transport actions. Before 1.0,
a Preview breaking change requires a changelog entry, a migration note, and a minor
version increment.

`experimental` implies `preview` and is never enabled transitively by a library.
It contains persistence, benchmark schemas, and application-owned renderer handles.
Experimental items may change without semver compatibility, but a change must still
update this manifest and the generated-documentation gate.

## Preview items

- Workbench: `WorkbenchNode`, `WorkbenchState`, `WorkbenchAction`, `FocusRoute`,
  `CommandScopeOutput`, `KeyboardTransportAction`, `KeyboardTransportOutput`,
  `KeyboardTransportReceipt`, `keyboard_transport_actions`,
  `keyboard_transport_actions_from_pressed`, `WorkbenchOutput`, and `show_workbench`.
- Timeline: `TimelineSnapshot`, `TimelineViewport`, `TimelineGeometryCache`,
  `TimelineAction`, `TimelineInteractionState`, `TimelineOutput`,
  `TimelineUxReceipt`, `timeline_receipt_json`, and `show_timeline`.
- Media surface: `MediaSurfaceSnapshot`, `MediaTransform`, `MediaSurfaceAction`,
  `MediaSurfaceInteractionState`, `MediaSurfaceOutput`, `MediaSurfaceUxReceipt`,
  `media_surface_receipt_json`, and `show_media_surface`.

## Experimental items

- Workbench product chrome: `WorkbenchOptions`,
  `calculate_workbench_geometry_with_options`, and
  `show_workbench_with_options`.
- Workbench persistence: `WorkbenchState::to_json_pretty`,
  `WorkbenchState::from_json`, `LayoutRestoreError`, and
  `WORKBENCH_LAYOUT_SCHEMA_VERSION`.
- Timeline benchmarking: `TimelinePerformanceReceipt`.
- Renderer-owned media state: `MediaTextureHandle`, `MediaFrameState`,
  `MediaFrameStateReceipt`, and `show_media_frame_state`.

The workbench persistence schema is version 2. Schema 1 has the same structural
fields and migrates by upgrading `schema_version` after JSON-envelope validation.
Missing versions, non-integer versions, versions older than the migration floor,
and future versions return distinct errors. FinUI never reads or writes a file;
the caller owns storage and passes JSON explicitly.

Run `scripts/check-editor-preview-api.ps1` to compile the feature matrix, generate
rustdoc with all features, and verify every manifest item appears in generated
documentation.

`scripts/run-editor-baselines.ps1` captures light/dark screenshots at
100/125/150/200% effective pixels-per-point. Every receipt must retain the same
logical play and frame-step actions, timeline focus order, media accessibility
labels, and zero-retained-texture shutdown result.
