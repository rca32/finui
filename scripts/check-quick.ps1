Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    cargo fmt --all --check
    if ($LASTEXITCODE -ne 0) { throw "cargo fmt failed" }
    & "$PSScriptRoot/check-library-boundary.ps1"
    & "$PSScriptRoot/check-backend-selection.ps1"
    & "$PSScriptRoot/check-editor-texture-path.ps1"
    cargo check --workspace --all-targets
    if ($LASTEXITCODE -ne 0) { throw "workspace check failed" }
    cargo check -p finui-grid --no-default-features
    if ($LASTEXITCODE -ne 0) { throw "no-default-features check failed" }
    cargo test -p finui-primitives --lib
    if ($LASTEXITCODE -ne 0) { throw "finui-primitives tests failed" }
    cargo test -p finui-grid --lib
    if ($LASTEXITCODE -ne 0) { throw "finui-grid tests failed" }
}
finally {
    Pop-Location
}
