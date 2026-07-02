# Text Overflow Policy

Finui primitives use one overflow contract for long English labels, dense
financial identifiers, Korean labels, and mixed numeric labels.

## Contract

Use `primitive_text_overflow_output` before painting text when a primitive has a
fixed-width text area.

| Field | Meaning |
| --- | --- |
| `display_text` | Text safe to paint in the constrained region. |
| `clipped` | True when the original text exceeded the configured character budget. |
| `tooltip_text` | Full text to expose on hover when clipping happened and tooltips are enabled. |
| `accessibility_text` | Full original text for snapshots and accessibility bridge output. |
| `label_kind_name` | `plain`, `dense_financial`, `korean`, or `mixed_numeric`. |
| `mode_name` | `clip` or `ellipsis`. |

## Defaults

- Default mode is `ellipsis`.
- Default label kind is `plain`.
- Tooltip text is emitted only when the label was clipped.
- Truncation is character-based, so Korean labels are not cut on byte
  boundaries.
- Very small ellipsis budgets use repeated `.` characters rather than a Unicode
  ellipsis so the helper remains ASCII-compatible.

## Label Kinds

| Kind | Use for | Recommended mode |
| --- | --- | --- |
| `plain` | Normal action labels, menu labels, tab labels. | `ellipsis` |
| `dense_financial` | ISINs, symbols, tenor labels, compact instrument names. | `ellipsis` with tooltip |
| `korean` | Korean UI labels or mixed Hangul labels. | `clip` for tiny cells, `ellipsis` elsewhere |
| `mixed_numeric` | Labels combining text, decimals, percentages, and units. | `ellipsis` with accessibility full text |

## Acceptance Tests

- `text_overflow_output_ellipsizes_long_dense_financial_labels`
- `text_overflow_output_clips_korean_labels_on_char_boundaries`
- `text_overflow_output_preserves_mixed_numeric_label_when_it_fits`
