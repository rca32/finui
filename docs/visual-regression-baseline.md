# Visual Regression Baseline

Primitive visual regression starts from three PNG baselines under
`docs/images/`. The manifest is exposed through
`primitive_visual_snapshot_baselines`.

| PNG | State | Diff threshold |
| --- | --- | --- |
| `docs/images/primitives-lab-light.png` | light, open, disabled, long text | `0.01` |
| `docs/images/primitives-lab-dark.png` | dark, RTL, disabled | `0.01` |
| `docs/images/primitives-lab-layers.png` | layer open and edge placement | `0.015` |

The thresholds are intentionally tight because these files are stable baselines,
not live captures. Runtime screenshot capture can compare against this manifest
without changing the checklist.

## Acceptance Tests

- `visual_snapshot_baselines_cover_representative_primitive_states`
- `visual_snapshot_baseline_png_files_exist_and_have_png_signature`
