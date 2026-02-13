# Prepare Public Distribution - Miyukini COG
# Genere public-dist/ : binaires + doc complete + mode d'emploi, SANS code source.

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot
$DistRoot = Join-Path $RepoRoot "public-dist"

Write-Host "Miyukini COG - Preparation distribution publique" -ForegroundColor Cyan
Write-Host "  Racine depot : $RepoRoot"
Write-Host "  Sortie      : $DistRoot"
Write-Host ""

# 1. Build release (Start-Process evite que stderr cargo declenche erreur PowerShell)
Push-Location $RepoRoot
try {
    Write-Host "[1/5] Compilation release miyukini-central..." -ForegroundColor Yellow
    $p1 = Start-Process -FilePath "cargo" -ArgumentList "build","--release","-p","miyukini-central-native" -WorkingDirectory $RepoRoot -Wait -NoNewWindow -PassThru
    if ($p1.ExitCode -ne 0) { throw "Build miyukini-central failed (exit $($p1.ExitCode))" }

    Write-Host "[2/5] Compilation release kindmother-server..." -ForegroundColor Yellow
    $p2 = Start-Process -FilePath "cargo" -ArgumentList "build","--release","-p","kindmother-service" -WorkingDirectory $RepoRoot -Wait -NoNewWindow -PassThru
    if ($p2.ExitCode -ne 0) { throw "Build kindmother-service failed (exit $($p2.ExitCode))" }
}
finally {
    Pop-Location
}

$TargetRelease = Join-Path $RepoRoot "target" "release"
$BinDir = Join-Path $DistRoot "bin"
$DocsDir = Join-Path $DistRoot "docs"

# 2. Creer structure
Write-Host "[3/5] Creation arborescence public-dist/..." -ForegroundColor Yellow
if (Test-Path $DistRoot) { Remove-Item $DistRoot -Recurse -Force }
New-Item -ItemType Directory -Path $BinDir -Force | Out-Null
New-Item -ItemType Directory -Path $DocsDir -Force | Out-Null

# 3. Copier binaires
$CentralExe = Join-Path $TargetRelease "miyukini-central.exe"
$KindMotherExe = Join-Path $TargetRelease "kindmother-server.exe"
if (Test-Path $CentralExe) { Copy-Item $CentralExe $BinDir -Force; Write-Host "    -> bin/miyukini-central.exe" }
else { Write-Host "    ATTENTION: miyukini-central.exe non trouve" -ForegroundColor Red }
if (Test-Path $KindMotherExe) { Copy-Item $KindMotherExe $BinDir -Force; Write-Host "    -> bin/kindmother-server.exe" }
else { Write-Host "    ATTENTION: kindmother-server.exe non trouve" -ForegroundColor Red }

# 4. Copier documentation
Write-Host "[4/5] Copie documentation..." -ForegroundColor Yellow
$SrcDocs = Join-Path $RepoRoot "docs"
Copy-Item -Path (Join-Path $SrcDocs "*") -Destination $DocsDir -Recurse -Force

# 5. README et MODE_EMPLOI a la racine
$DistDocsDir = Join-Path $RepoRoot "docs" "distribution"
$ReadmeDist = Join-Path $DistDocsDir "README-DISTRIBUTION.md"
$ModeEmploi = Join-Path $DistDocsDir "MODE_EMPLOI.md"

if (Test-Path $ReadmeDist) {
    Copy-Item $ReadmeDist (Join-Path $DistRoot "README.md") -Force
    Write-Host "    -> README.md"
}
else {
    $fallback = "# Miyukini COG - Distribution publique" + [Environment]::NewLine + [Environment]::NewLine + "Binaires dans bin/, doc dans docs/. Voir MODE_EMPLOI.md."
    Set-Content (Join-Path $DistRoot "README.md") -Value $fallback -Encoding UTF8
    Write-Host "    -> README.md (minimal)"
}

if (Test-Path $ModeEmploi) {
    Copy-Item $ModeEmploi (Join-Path $DistRoot "MODE_EMPLOI.md") -Force
    Write-Host "    -> MODE_EMPLOI.md"
}
else {
    Write-Host "    MODE_EMPLOI.md absent" -ForegroundColor Yellow
}

# LICENSE
$Lic = Join-Path $RepoRoot "LICENSE"
if (Test-Path $Lic) {
    Copy-Item $Lic (Join-Path $DistRoot "LICENSE") -Force
    Write-Host "    -> LICENSE"
}

Write-Host "[5/5] Termine." -ForegroundColor Green
Write-Host ""
Write-Host "Distribution prete : $DistRoot"
Write-Host "Puis : copier public-dist vers clone du depot public, git add, commit, push."
Write-Host ""
