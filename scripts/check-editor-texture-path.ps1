Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$previewPath = Join-Path $repoRoot "examples/editor_lab/src/gpu_preview.rs"
$mainPath = Join-Path $repoRoot "examples/editor_lab/src/main.rs"
$previewSource = Get-Content -Raw $previewPath
$mainSource = Get-Content -Raw $mainPath

$requiredPreviewTokens = @(
    "TextureUsages::RENDER_ATTACHMENT",
    "TextureUsages::TEXTURE_BINDING",
    "register_native_texture",
    "begin_render_pass",
    "queue.submit"
)

foreach ($token in $requiredPreviewTokens) {
    if (-not $previewSource.Contains($token)) {
        throw "editor texture path is missing required GPU token: $token"
    }
}

if (-not $mainSource.Contains("egui::Image::new")) {
    throw "editor_lab does not paint the registered native texture"
}

$forbiddenPixelTransferTokens = @(
    "TextureUsages::COPY_SRC",
    "BufferUsages::MAP_READ",
    "copy_texture_to_buffer",
    "map_async",
    "write_texture",
    "ColorImage",
    "ImageData"
)

foreach ($token in $forbiddenPixelTransferTokens) {
    if ($previewSource.Contains($token) -or $mainSource.Contains($token)) {
        throw "editor texture path contains a CPU pixel transfer token: $token"
    }
}

[pscustomobject]@{
    status = "PASS"
    bridge = "wgpu render attachment -> egui native texture"
    cpu_pixel_readback = $false
    cpu_pixel_upload = $false
} | ConvertTo-Json -Depth 3
