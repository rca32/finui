# Editor Texture Bridge

`editor_lab` demonstrates the GPU presentation boundary intended for a media
compositor. It renders an animated pattern into an off-screen wgpu texture and
registers that texture directly with egui's wgpu renderer.

```text
wgpu fragment pass
  -> Rgba8Unorm texture (RENDER_ATTACHMENT | TEXTURE_BINDING)
  -> egui_wgpu::Renderer::register_native_texture
  -> egui::Image
  -> eframe wgpu paint
```

The shader output remains GPU-resident across the whole path. The CPU updates a
16-byte frame uniform containing time, dimensions, and texture generation. It
does not upload or read back pixel data.

## Ownership And Lifecycle

`GpuPreview` borrows no application globals. It owns the off-screen texture,
view, pipeline, uniform resources, and the native egui texture registration. It
clones eframe's `RenderState`, so the preview and egui renderer use the same
wgpu `Device` and `Queue`.

On initial allocation or a quantized size change, the bridge creates and
registers a new texture before retiring the old one. The old texture and egui
registration stay alive for two additional application frames. The deferred
release prevents a resource that may still be referenced by an already encoded
egui frame from being destroyed during resize. Dropping the bridge unregisters
the active and any retired texture IDs.

The texture dimensions are clamped to 16..2048 pixels and quantized to 16-pixel
steps. This avoids allocating a new texture for every sub-pixel window-size
change while keeping the preview responsive to DPI and viewport resizing.

## Device-Loss Boundary

A registered texture ID belongs to one specific eframe `RenderState`: its
`Device`, `Queue`, and egui `Renderer`. Those resources must never cross a real
device replacement. After eframe creates a replacement render state, the
application must drop the old `GpuPreview` and construct a new one, which
rebuilds the shader, pipeline, uniform resources, texture, and registration.

The lab button and smoke test exercise texture replacement and deferred release
on the current device. The `device-recovery-simulation` receipt label defines
the rebuild boundary; it is not evidence that the operating system delivered
and recovered from an actual GPU device-loss event.

## No-Readback Gate

`scripts/check-editor-texture-path.ps1` is part of both quick and full checks. It
requires the render-attachment, texture-binding, native-registration, GPU render
pass, and egui image path. It rejects pixel-copy/readback APIs such as
`COPY_SRC`, `MAP_READ`, `copy_texture_to_buffer`, `map_async`, `write_texture`,
`ColorImage`, and `ImageData` in the bridge.

Run the deterministic runtime smoke test with:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/run-editor-smoke.ps1
```

Smoke mode keeps the native viewport hidden, forces one texture replacement,
runs enough frames to release the retired registration, writes adapter and
transfer counters to the receipt, and closes the application.

Run `cargo run -p editor_lab` without smoke variables for the visible animated
preview. Each target platform must run both the visible check and the smoke
script; a successful build on one platform is not runtime evidence for another.
