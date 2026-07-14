param(
    [ValidateRange(6, 10000)]
    [int]$Frames = 60,
    [string]$ExpectedPlatform = "",
    [ValidateSet("light", "dark")]
    [string]$Theme = "dark",
    [ValidateSet(100, 125, 150, 200)]
    [int]$DpiPercent = 100,
    [string]$ReceiptPath = "",
    [string]$ScreenshotPath = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$receiptPath = if ($ReceiptPath) { $ReceiptPath } else { Join-Path $repoRoot ".tmp/editor-lab-smoke.json" }
$previousFrames = $env:FINUI_EDITOR_LAB_SMOKE_FRAMES
$previousReceipt = $env:FINUI_EDITOR_LAB_SMOKE_RECEIPT
$previousScreenshot = $env:FINUI_EDITOR_LAB_SMOKE_SCREENSHOT
$previousTheme = $env:FINUI_EDITOR_LAB_SMOKE_THEME
$previousDpi = $env:FINUI_EDITOR_LAB_SMOKE_DPI_PERCENT

Push-Location $repoRoot
try {
    if (Test-Path -LiteralPath $receiptPath) {
        Remove-Item -LiteralPath $receiptPath -Force
    }

    $env:FINUI_EDITOR_LAB_SMOKE_FRAMES = $Frames.ToString()
    $env:FINUI_EDITOR_LAB_SMOKE_RECEIPT = $receiptPath
    $env:FINUI_EDITOR_LAB_SMOKE_THEME = $Theme
    $env:FINUI_EDITOR_LAB_SMOKE_DPI_PERCENT = $DpiPercent.ToString()
    if ($ScreenshotPath) {
        $env:FINUI_EDITOR_LAB_SMOKE_SCREENSHOT = $ScreenshotPath
        if (Test-Path -LiteralPath $ScreenshotPath) {
            Remove-Item -LiteralPath $ScreenshotPath -Force
        }
    }
    else {
        Remove-Item Env:FINUI_EDITOR_LAB_SMOKE_SCREENSHOT -ErrorAction SilentlyContinue
    }
    cargo run -p editor_lab
    if ($LASTEXITCODE -ne 0) {
        throw "editor_lab smoke process failed with exit code $LASTEXITCODE"
    }
    if (-not (Test-Path -LiteralPath $receiptPath)) {
        throw "editor_lab did not write the smoke receipt"
    }

    $receipt = Get-Content -Raw -LiteralPath $receiptPath | ConvertFrom-Json
    if ($receipt.render_passes -lt $Frames) {
        throw "editor_lab rendered $($receipt.render_passes) frames; expected at least $Frames"
    }
    if ($receipt.texture_registrations -lt 2 -or $receipt.texture_releases -lt 1) {
        throw "editor_lab did not prove texture replacement and deferred release"
    }
    if ($receipt.cpu_pixel_readbacks -ne 0 -or $receipt.cpu_pixel_upload_bytes -ne 0) {
        throw "editor_lab reported a CPU pixel transfer"
    }
    if ($receipt.texture_registrations -ne $receipt.texture_releases -or
        $receipt.retained_textures -ne 0) {
        throw "editor_lab shutdown retained one or more registered textures"
    }
    if ($receipt.workbench.schema_version -ne 2) {
        throw "editor_lab reported an unsupported workbench schema"
    }
    if ($receipt.workbench.divider_count -lt 3 -or $receipt.workbench.region_count -lt 4) {
        throw "editor_lab did not render the complete split workbench"
    }
    if ($receipt.workbench.panel_tab_count -lt 5 -or $receipt.workbench.active_panels_rendered -lt 4) {
        throw "editor_lab did not render the required editor panel set"
    }
    if ($receipt.workbench.timeline.total_clip_count -ne 10000) {
        throw "editor_lab did not load the 10,000 clip timeline fixture"
    }
    if ($receipt.workbench.timeline.candidate_clip_count -ge $receipt.workbench.timeline.total_clip_count) {
        throw "editor_lab timeline did not cull offscreen clips"
    }
    if ($receipt.workbench.timeline.painted_clip_ids.Count -eq 0 -or
        $receipt.workbench.timeline.painted_clip_ids.Count -ne $receipt.workbench.timeline.hit_test_clip_ids.Count) {
        throw "editor_lab timeline paint and hit-test sets do not match"
    }
    if ($receipt.workbench.media_surface.overlay_handle_count -ne 4 -or
        $receipt.workbench.media_surface.source_size[0] -le 0 -or
        $receipt.workbench.media_surface.source_size[1] -le 0) {
        throw "editor_lab media surface did not expose the external texture overlay"
    }
    if ($receipt.workbench.timeline.accessibility_label -ne "Timeline editor" -or
        -not $receipt.workbench.timeline.keyboard_selection_enabled -or
        $receipt.workbench.timeline.focus_order_clip_ids.Count -eq 0) {
        throw "editor_lab timeline accessibility/focus receipt is incomplete"
    }
    if ($receipt.workbench.media_surface.accessibility_label -ne "Media preview" -or
        -not $receipt.workbench.media_surface.keyboard_selection_enabled) {
        throw "editor_lab media surface accessibility receipt is incomplete"
    }
    $expectedActions = @("play", "step_frames:-1", "step_frames:1")
    if (@(Compare-Object $expectedActions $receipt.workbench.keyboard_transport.logical_actions).Count -ne 0) {
        throw "editor_lab keyboard logical actions changed at $DpiPercent% $Theme"
    }
    if ($receipt.workbench.theme -ne $Theme -or $receipt.workbench.dpi_percent -ne $DpiPercent) {
        throw "editor_lab theme/DPI receipt does not match the requested baseline"
    }
    $expectedPixelsPerPoint = $DpiPercent / 100.0
    if ([Math]::Abs([double]$receipt.workbench.pixels_per_point - $expectedPixelsPerPoint) -gt 0.01) {
        throw "editor_lab effective pixels-per-point $($receipt.workbench.pixels_per_point) did not reach $expectedPixelsPerPoint"
    }
    if ($ScreenshotPath) {
        if (-not (Test-Path -LiteralPath $ScreenshotPath)) {
            throw "editor_lab did not write screenshot $ScreenshotPath"
        }
        if ((Get-Item -LiteralPath $ScreenshotPath).Length -le 0 -or
            $receipt.workbench.screenshot_size[0] -le 0 -or
            $receipt.workbench.screenshot_size[1] -le 0) {
            throw "editor_lab screenshot receipt is empty"
        }
    }
    if ($ExpectedPlatform -and $receipt.platform -ne $ExpectedPlatform) {
        throw "editor_lab ran on $($receipt.platform); expected $ExpectedPlatform"
    }

    $receipt | ConvertTo-Json -Depth 5
}
finally {
    $env:FINUI_EDITOR_LAB_SMOKE_FRAMES = $previousFrames
    $env:FINUI_EDITOR_LAB_SMOKE_RECEIPT = $previousReceipt
    $env:FINUI_EDITOR_LAB_SMOKE_SCREENSHOT = $previousScreenshot
    $env:FINUI_EDITOR_LAB_SMOKE_THEME = $previousTheme
    $env:FINUI_EDITOR_LAB_SMOKE_DPI_PERCENT = $previousDpi
    Pop-Location
}
