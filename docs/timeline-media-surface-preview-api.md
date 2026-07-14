# Timeline And Media Surface Preview APIs

`finui-timeline` and `finui-media-surface` are renderer-neutral Preview APIs for
editor applications. They use direct egui types and keep project mutation,
undo/redo, media decoding, texture creation, and GPU lifecycle in the caller.

## Timeline Snapshot And Cache

`TimelineSnapshot` is immutable input with integer ticks, tracks, clips, and a
revision. The application builds `TimelineGeometryCache` when that revision
changes. Building the cache sorts each track by clip start and stores a
monotonic prefix maximum of clip end ticks.

A viewport query uses binary partition points to find intervals that can overlap
the visible tick range. Its expected cost is:

```text
O(visible tracks * log(clips per track) + candidate clips)
```

The 100-track × 100-clip acceptance fixture contains 10,000 clips. Tests verify
that a small viewport examines fewer than 120 candidates and that indexed output
matches a naive overlap calculation in the same stable track/start order.

`TimelineGeometry::visible_clips` is the only clip collection used by both
painting and pointer hit-testing. Clips outside the horizontal tick range or
vertical track range are therefore excluded from both paths. The UX receipt
records the two ID lists independently so runtime checks can require equality.

## Timeline Interaction

The timeline paints an integer-tick ruler and playhead and emits caller-owned
actions for selection, playhead changes, clip moves, and start/end trims.
Dragging is a transaction:

```text
BeginDrag -> UpdateDrag* -> CommitDrag | CancelDrag
```

`TimelineInteractionState` retains only the active gesture. Applying an action
to it never changes clip start, duration, selection, or the project snapshot.
The editor core decides how to validate, commit, undo, or reject model changes.

## External Media Surface

`MediaSurfaceSnapshot` contains an opaque `egui::TextureId`, source dimensions,
selection, and caller-owned transform. The component computes a centered
aspect-fit rectangle, then applies position and scale for overlay geometry.

Selected surfaces expose four scale handles and a move target. Transform output
uses the same begin/update/commit/cancel transaction shape as timeline drags.
During a drag, projected geometry comes from interaction state while the input
snapshot remains unchanged. The caller applies a committed transform if desired.

The component performs no pixel conversion or texture upload. `editor_lab`
passes the native texture ID registered by its wgpu bridge directly into the
surface.

## UX Receipts

`TimelineUxReceipt` includes:

- total and indexed candidate clip counts
- visible tick and track ranges
- paint and hit-test clip ID lists
- visible selected clip IDs
- drag phase

`MediaSurfaceUxReceipt` includes:

- opaque texture ID and source dimensions
- transformed surface rectangle
- selection and four-handle overlay count
- transform phase

Both receipts have stable JSON helpers for tests and automated editor inspection.
