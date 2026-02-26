# build-services.ps1 — Compile et package les services standalone en .msp (Windows)
#
# Usage :
#   .\tools\build-service-packages\build-services.ps1 [service_id...]
#
# Sans arguments : compile et package TOUS les services.
# Produit : dist\services\{service_id}.msp (archive ZIP prête pour Central)

param(
    [string[]]$Services
)

$ErrorActionPreference = "Stop"
$WorkspaceRoot = (Resolve-Path "$PSScriptRoot\..\..")
$DistDir = "$WorkspaceRoot\dist\services"
$TargetDir = "$WorkspaceRoot\target\release"

# Correspondances
$ServiceMap = @{
    "jayxpose"           = @{ Bin = "jayxpose";           Pkg = "jayxpose-app";       Crate = "crates\jayxpose" }
    "jayfestival"        = @{ Bin = "jayfestival";        Pkg = "jayfestival-app";    Crate = "crates\jayfestival" }
    "jaykoa"             = @{ Bin = "jaykoa";             Pkg = "jaykoa-app";         Crate = "crates\jaykoa" }
    "jaykonta"           = @{ Bin = "jaykonta";           Pkg = "jaykonta-app";       Crate = "crates\jaykonta" }
    "miyukiniwatch"      = @{ Bin = "miyukiniwatch";      Pkg = "miyukiniwatch-app";  Crate = "crates\miyukiniwatch" }
    "jay1tribu"          = @{ Bin = "jay1tribu";          Pkg = "jay1tribu-app";      Crate = "crates\jay1tribu" }
    "jaymanga"           = @{ Bin = "jaymanga";           Pkg = "jaymanga-app";       Crate = "crates\jaymanga" }
    "miyuclicker"        = @{ Bin = "miyuclicker";        Pkg = "miyuclicker-app";    Crate = "crates\miyuclicker" }
    "lord_of_the_castle" = @{ Bin = "lord_of_the_castle"; Pkg = "lord_of_the_castle"; Crate = "crates\lord_of_the_castle" }
}

if ($Services.Count -eq 0) {
    $Services = $ServiceMap.Keys
}

Write-Host "=== Build Service Packages ===" -ForegroundColor Cyan
Write-Host "Workspace : $WorkspaceRoot"
Write-Host "Services  : $($Services -join ', ')"
Write-Host ""

New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

foreach ($svcId in $Services) {
    $info = $ServiceMap[$svcId]
    if (-not $info) {
        Write-Host "ERREUR: Service inconnu '$svcId'" -ForegroundColor Red
        continue
    }

    $binName = $info.Bin
    $pkgName = $info.Pkg
    $crateDir = $info.Crate
    $manifestFile = "$WorkspaceRoot\$crateDir\service.manifest.json"

    if (-not (Test-Path $manifestFile)) {
        Write-Host "ERREUR: $manifestFile manquant" -ForegroundColor Red
        continue
    }

    Write-Host "--- [$svcId] Compilation (release) ---" -ForegroundColor Yellow
    & cargo build --release --bin $binName -p $pkgName
    if ($LASTEXITCODE -ne 0) {
        Write-Host "ERREUR: Compilation échouée pour $svcId" -ForegroundColor Red
        continue
    }

    $binary = "$TargetDir\$binName.exe"
    if (-not (Test-Path $binary)) {
        Write-Host "ERREUR: Binaire non trouvé: $binary" -ForegroundColor Red
        continue
    }

    Write-Host "--- [$svcId] Packaging .msp ---" -ForegroundColor Yellow
    $mspFile = "$DistDir\$svcId.msp"
    $tmpDir = New-Item -ItemType Directory -Force -Path "$env:TEMP\msp_$svcId"

    try {
        # Copier le manifeste
        Copy-Item $manifestFile "$tmpDir\service.manifest.json"

        # Copier le binaire dans bin/
        New-Item -ItemType Directory -Force -Path "$tmpDir\bin" | Out-Null
        Copy-Item $binary "$tmpDir\bin\$binName.exe"

        # Créer data/ vide
        New-Item -ItemType Directory -Force -Path "$tmpDir\data" | Out-Null

        # Créer l'archive ZIP (.msp)
        if (Test-Path $mspFile) { Remove-Item $mspFile }
        Compress-Archive -Path "$tmpDir\*" -DestinationPath $mspFile -Force

        $size = (Get-Item $mspFile).Length / 1MB
        Write-Host "  -> $mspFile ($([math]::Round($size, 1)) MB)" -ForegroundColor Green
    }
    finally {
        Remove-Item -Recurse -Force $tmpDir -ErrorAction SilentlyContinue
    }
    Write-Host ""
}

Write-Host "=== Packaging terminé ===" -ForegroundColor Cyan
Get-ChildItem "$DistDir\*.msp" -ErrorAction SilentlyContinue | Format-Table Name, Length
