Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
$manifestPath = Join-Path $repoRoot "Cargo.toml"
$metadataOutput = & cargo metadata `
    --format-version 1 `
    --no-deps `
    --manifest-path $manifestPath
if ($LASTEXITCODE -ne 0) {
    throw "cargo metadata failed"
}

$metadata = ($metadataOutput -join [Environment]::NewLine) | ConvertFrom-Json
$packages = @($metadata.packages | Where-Object { $_.name -like "finui-*" })
if ($packages.Count -eq 0) {
    throw "No finui-* library packages found"
}

$forbiddenPattern = "^(eframe|egui-wgpu|egui_glow|glow|wgpu) v"
$violations = [System.Collections.Generic.List[string]]::new()

foreach ($package in $packages) {
    $tree = & cargo tree `
        --locked `
        --manifest-path $manifestPath `
        --package $package.name `
        --edges normal `
        --prefix none `
        --format "{p}"
    if ($LASTEXITCODE -ne 0) {
        throw "cargo tree failed for $($package.name)"
    }

    foreach ($dependency in $tree) {
        if ($dependency -match $forbiddenPattern) {
            $violations.Add("$($package.name) reaches renderer dependency: $dependency")
        }
    }
}

if ($violations.Count -gt 0) {
    $violations | ForEach-Object { Write-Error $_ }
    throw "Finui library backend boundary check failed"
}

[pscustomobject]@{
    status = "PASS"
    packages = @($packages.name | Sort-Object)
    rule = "finui-* dependency trees exclude eframe, glow, wgpu, and egui renderer crates"
} | ConvertTo-Json -Depth 4
