# Icon Contract

Finui primitives use the vendored Radix icon set for primitive affordances before
falling back to ad-hoc painter glyphs or text.

## Asset Scope

Radix SVG assets live under `crates/finui-primitives/assets/radix-icons`.
`RadixIcon` variants must map to checked-in SVG files through
`radix_icon_asset`. New primitive icons should be added to this enum and covered
by an asset-backed test before they are used by controls.

The upstream source and license stay in the asset directory:

- `README.md`
- `README.upstream.md`
- `LICENSE`

## Visual Names And Fallback

Callers that receive string visual names should resolve them with
`radix_icon_visual`.

- Known names map to a `RadixIcon`.
- Unknown names keep `fallback_text`, so the control can still render an explicit
  text marker instead of silently dropping the icon.
- Alias names such as `refresh`, `close`, `warning`, and `evidence` are part of
  the Finui contract and should be tested when added.

## Tinting

Radix SVGs use `currentColor`. `paint_radix_icon` converts the asset with
`radix_icon_tintable_svg` and applies the final egui tint color at paint time.
Controls should pass semantic theme colors such as text, muted text, disabled
text, or state accent colors instead of editing SVG assets for each state.

## Accessible And Decorative Icons

Use `AccessibleIconRootOptions` and `primitive_accessible_icon_root_output` when
an icon is exposed as its own interactive or semantic part.

- Non-decorative icons must carry a label.
- Decorative icons set `decorative=true`; their output label is `None`.
- Disabled icons use disabled text color and hover-only input behavior.

Icons embedded inside labeled controls, such as menu checks or close buttons,
may remain decorative when the surrounding control already supplies the label or
action semantics.
