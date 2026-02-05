# Lance la démo egui officielle depuis deps/egui (clone du repo emilk/egui).
# Usage : .\tools\run-egui-demo.ps1
# Option --glow pour utiliser le backend glow au lieu de wgpu.

param(
    [switch]$glow
)

$eguiDir = Join-Path $PSScriptRoot ".." "deps" "egui"
if (-not (Test-Path $eguiDir)) {
    Write-Error "deps/egui introuvable. Exécutez d'abord : git clone --depth 1 https://github.com/emilk/egui.git deps/egui"
    exit 1
}

Push-Location $eguiDir
try {
    if ($glow) {
        cargo run --release -p egui_demo_app --no-default-features --features glow,persistence
    } else {
        cargo run --release -p egui_demo_app
    }
} finally {
    Pop-Location
}
