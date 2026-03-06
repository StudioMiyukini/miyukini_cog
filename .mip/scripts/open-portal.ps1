Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$mipRoot = Resolve-Path (Join-Path $scriptDir "..")

# --- Mise a jour automatique de l'index sequences ---
$rebuildScript = Join-Path $scriptDir "rebuild-sequences-index.ps1"
if (Test-Path -LiteralPath $rebuildScript) {
    Write-Host "Mise a jour de sequences/index.json..."
    try {
        & powershell -NoProfile -ExecutionPolicy Bypass -File $rebuildScript
    } catch {
        Write-Warning "Impossible de regenerer l'index: $($_.Exception.Message)"
    }
} else {
    Write-Warning "Script rebuild-sequences-index.ps1 introuvable, index non mis a jour."
}

# --- Ouverture du portail ---
$portalPath = Join-Path $mipRoot "index.html"
if (-not (Test-Path -LiteralPath $portalPath)) {
    throw "Portail introuvable: $portalPath"
}

Write-Host "Ouverture du portail MIP: $portalPath"
Start-Process $portalPath
