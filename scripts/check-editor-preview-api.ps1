Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    $packages = @('finui-workbench', 'finui-timeline', 'finui-media-surface')
    foreach ($package in $packages) {
        & cargo check --locked -p $package --no-default-features
        if ($LASTEXITCODE -ne 0) { throw "$package no-default-features check failed" }
        & cargo check --locked -p $package --all-features
        if ($LASTEXITCODE -ne 0) { throw "$package all-features check failed" }
    }

    & cargo doc --locked --no-deps --all-features `
        -p finui-workbench -p finui-timeline -p finui-media-surface
    if ($LASTEXITCODE -ne 0) { throw 'editor Preview API rustdoc generation failed' }

    $checks = @(
        @{
            Path = 'target\doc\finui_workbench\all.html'
            Items = @('WorkbenchNode', 'WorkbenchState', 'WorkbenchAction', 'FocusRoute',
                'CommandScopeOutput', 'KeyboardTransportAction', 'KeyboardTransportOutput',
                'KeyboardTransportReceipt', 'keyboard_transport_actions',
                'keyboard_transport_actions_from_pressed', 'WorkbenchOutput', 'show_workbench',
                'WorkbenchOptions', 'calculate_workbench_geometry_with_options',
                'show_workbench_with_options', 'LayoutRestoreError',
                'WORKBENCH_LAYOUT_SCHEMA_VERSION')
        },
        @{
            Path = 'target\doc\finui_workbench\struct.WorkbenchState.html'
            Items = @('to_json_pretty', 'from_json')
        },
        @{
            Path = 'target\doc\finui_timeline\all.html'
            Items = @('TimelineSnapshot', 'TimelineViewport', 'TimelineGeometryCache',
                'TimelineAction', 'TimelineInteractionState', 'TimelineOutput',
                'TimelineUxReceipt', 'timeline_receipt_json', 'show_timeline', 'TimelinePerformanceReceipt')
        },
        @{
            Path = 'target\doc\finui_media_surface\all.html'
            Items = @('MediaSurfaceSnapshot', 'MediaTransform', 'MediaSurfaceAction',
                'MediaSurfaceInteractionState', 'MediaSurfaceOutput', 'MediaSurfaceUxReceipt',
                'media_surface_receipt_json', 'show_media_surface', 'MediaTextureHandle', 'MediaFrameState',
                'MediaFrameStateReceipt', 'show_media_frame_state')
        }
    )
    foreach ($check in $checks) {
        $path = Join-Path $repoRoot $check.Path
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "generated rustdoc file is missing: $path"
        }
        $html = Get-Content -Raw -LiteralPath $path
        foreach ($item in $check.Items) {
            if (-not $html.Contains($item)) {
                throw "generated rustdoc $($check.Path) is missing API item: $item"
            }
        }
    }

    [pscustomobject]@{
        status = 'PASS'
        packages = $packages
        default_feature = 'preview'
        opt_in_feature = 'experimental'
        generated_documentation_checked = $true
    } | ConvertTo-Json -Depth 4
}
finally {
    Pop-Location
}
