param(
    [ValidateRange(10, 10000)]
    [int]$Frames = 30,
    [string]$OutputDirectory = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$outputRoot = if ($OutputDirectory) {
    $OutputDirectory
}
else {
    Join-Path $repoRoot ".tmp/editor-baselines"
}
New-Item -ItemType Directory -Force -Path $outputRoot | Out-Null

$baselines = @()
foreach ($theme in @("light", "dark")) {
    foreach ($dpi in @(100, 125, 150, 200)) {
        $stem = "$theme-$dpi"
        $receiptPath = Join-Path $outputRoot "$stem.json"
        $screenshotPath = Join-Path $outputRoot "$stem.png"
        & (Join-Path $PSScriptRoot "run-editor-smoke.ps1") `
            -Frames $Frames `
            -ExpectedPlatform "windows" `
            -Theme $theme `
            -DpiPercent $dpi `
            -ReceiptPath $receiptPath `
            -ScreenshotPath $screenshotPath | Out-Null
        $receipt = Get-Content -Raw -LiteralPath $receiptPath | ConvertFrom-Json
        $baselines += [ordered]@{
            theme = $theme
            dpi_percent = $dpi
            pixels_per_point = $receipt.workbench.pixels_per_point
            screenshot = (Split-Path -Leaf $screenshotPath)
            screenshot_sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $screenshotPath).Hash.ToLowerInvariant()
            logical_actions = @($receipt.workbench.keyboard_transport.logical_actions)
            timeline_accessibility_label = $receipt.workbench.timeline.accessibility_label
            media_accessibility_label = $receipt.workbench.media_surface.accessibility_label
        }
    }
}

$referenceActions = @($baselines[0].logical_actions)
foreach ($baseline in $baselines) {
    if (@(Compare-Object $referenceActions @($baseline.logical_actions)).Count -ne 0) {
        throw "keyboard logical actions vary between DPI/theme baselines"
    }
}

$manifest = [ordered]@{
    schema_version = 1
    baseline_count = $baselines.Count
    logical_actions_stable = $true
    baselines = $baselines
}
$manifestPath = Join-Path $outputRoot "manifest.json"
$manifest | ConvertTo-Json -Depth 6 | Set-Content -Encoding utf8 -LiteralPath $manifestPath
$manifest | ConvertTo-Json -Depth 6
