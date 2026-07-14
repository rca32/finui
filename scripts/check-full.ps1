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
    cargo clippy --workspace --all-targets
    if ($LASTEXITCODE -ne 0) { throw "workspace clippy failed" }
    cargo test --workspace
    if ($LASTEXITCODE -ne 0) { throw "workspace tests failed" }
}
finally {
    Pop-Location
}
