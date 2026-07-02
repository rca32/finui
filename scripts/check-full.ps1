Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    cargo fmt --all --check
    cargo check --workspace --all-targets
    cargo check -p finui-grid --no-default-features
    cargo clippy --workspace --all-targets
    cargo test --workspace
}
finally {
    Pop-Location
}
