Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $repoRoot "Cargo.toml"

function Get-NormalDependencyTree([string]$packageName) {
    $tree = & cargo tree `
        --locked `
        --manifest-path $manifestPath `
        --package $packageName `
        --edges normal `
        --prefix none `
        --format "{p}"
    if ($LASTEXITCODE -ne 0) {
        throw "cargo tree failed for $packageName"
    }
    return @($tree)
}

foreach ($packageName in @("grid_lab", "primitives_lab")) {
    $tree = Get-NormalDependencyTree $packageName
    if (-not ($tree -match "^egui_glow v")) {
        throw "$packageName does not select the egui glow renderer"
    }
    if ($tree -match "^egui-wgpu v") {
        throw "$packageName unexpectedly selects the egui wgpu renderer"
    }
}

$editorTree = Get-NormalDependencyTree "editor_lab"
if (-not ($editorTree -match "^egui-wgpu v")) {
    throw "editor_lab does not select the egui wgpu renderer"
}
if ($editorTree -match "^egui_glow v") {
    throw "editor_lab unexpectedly selects the egui glow renderer"
}

[pscustomobject]@{
    status = "PASS"
    glow = @("grid_lab", "primitives_lab")
    wgpu = @("editor_lab")
} | ConvertTo-Json -Depth 4
