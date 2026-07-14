param(
    [ValidateRange(6, 10000)]
    [int]$Frames = 60,
    [string]$ExpectedPlatform = ""
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$receiptPath = Join-Path $repoRoot ".tmp/editor-lab-smoke.json"
$previousFrames = $env:FINUI_EDITOR_LAB_SMOKE_FRAMES
$previousReceipt = $env:FINUI_EDITOR_LAB_SMOKE_RECEIPT

Push-Location $repoRoot
try {
    if (Test-Path -LiteralPath $receiptPath) {
        Remove-Item -LiteralPath $receiptPath -Force
    }

    $env:FINUI_EDITOR_LAB_SMOKE_FRAMES = $Frames.ToString()
    $env:FINUI_EDITOR_LAB_SMOKE_RECEIPT = $receiptPath
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
    if ($receipt.workbench.schema_version -ne 1) {
        throw "editor_lab reported an unsupported workbench schema"
    }
    if ($receipt.workbench.divider_count -lt 3 -or $receipt.workbench.region_count -lt 4) {
        throw "editor_lab did not render the complete split workbench"
    }
    if ($receipt.workbench.panel_tab_count -lt 5 -or $receipt.workbench.active_panels_rendered -lt 4) {
        throw "editor_lab did not render the required editor panel set"
    }
    if ($ExpectedPlatform -and $receipt.platform -ne $ExpectedPlatform) {
        throw "editor_lab ran on $($receipt.platform); expected $ExpectedPlatform"
    }

    $receipt | ConvertTo-Json -Depth 5
}
finally {
    $env:FINUI_EDITOR_LAB_SMOKE_FRAMES = $previousFrames
    $env:FINUI_EDITOR_LAB_SMOKE_RECEIPT = $previousReceipt
    Pop-Location
}
