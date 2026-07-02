Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot
try {
    cargo fmt --all --check
    cargo check --workspace --all-targets
    cargo check -p finui-grid --no-default-features
    cargo test -p finui-primitives --lib
    cargo test -p finui-grid --lib
}
finally {
    Pop-Location
}
