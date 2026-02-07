# Miyukini Core System - Installation des dépendances
# Usage : exécuter depuis la racine du projet (après clone)
# .\scripts\install-dependencies.ps1

$ErrorActionPreference = "Stop"
$ProjectRoot = if ($PSScriptRoot) { Split-Path $PSScriptRoot -Parent } else { Get-Location }

Write-Host "=== Miyukini Core System - Installation des dépendances ===" -ForegroundColor Cyan
Write-Host "Racine projet : $ProjectRoot`n" -ForegroundColor Gray

$installed = $false

# --- Node.js / npm / pnpm / yarn ---
function Install-NodeDeps {
    param([string]$dir = $ProjectRoot)
    $pkg = Join-Path $dir "package.json"
    if (-not (Test-Path $pkg)) { return $false }
    Push-Location $dir
    try {
        if (Get-Command pnpm -ErrorAction SilentlyContinue) {
            Write-Host "[Node] pnpm install dans $dir" -ForegroundColor Yellow
            pnpm install
        } elseif (Get-Command yarn -ErrorAction SilentlyContinue) {
            Write-Host "[Node] yarn install dans $dir" -ForegroundColor Yellow
            yarn install
        } elseif (Get-Command npm -ErrorAction SilentlyContinue) {
            Write-Host "[Node] npm install dans $dir" -ForegroundColor Yellow
            npm install
        } else {
            Write-Host "Aucun gestionnaire Node (npm/pnpm/yarn) trouvé. Installez Node.js." -ForegroundColor Red
            return $false
        }
        return $true
    } finally { Pop-Location }
}

# --- Python pip ---
function Install-PythonDeps {
    param([string]$dir = $ProjectRoot)
    $req = Join-Path $dir "requirements.txt"
    $pyproj = Join-Path $dir "pyproject.toml"
    $done = $false
    if (Test-Path $req) {
        if (Get-Command pip -ErrorAction SilentlyContinue) {
            Write-Host "[Python] pip install -r requirements.txt dans $dir" -ForegroundColor Yellow
            Push-Location $dir; pip install -r requirements.txt; Pop-Location
            $done = $true
        } elseif (Get-Command pip3 -ErrorAction SilentlyContinue) {
            Write-Host "[Python] pip3 install -r requirements.txt dans $dir" -ForegroundColor Yellow
            Push-Location $dir; pip3 install -r requirements.txt; Pop-Location
            $done = $true
        }
    }
    if (Test-Path $pyproj) {
        $pipCmd = if (Get-Command pip -ErrorAction SilentlyContinue) { "pip" } elseif (Get-Command pip3 -ErrorAction SilentlyContinue) { "pip3" } else { $null }
        if ($pipCmd) {
            Write-Host "[Python] $pipCmd install -e . (pyproject.toml) dans $dir" -ForegroundColor Yellow
            Push-Location $dir; & $pipCmd install -e .; Pop-Location
            $done = $true
        }
    }
    if (-not $done -and ((Test-Path $req) -or (Test-Path $pyproj))) {
        Write-Host "Fichier Python trouvé mais pip/pip3 non trouvé. Installez Python." -ForegroundColor Red
    }
    return $done
}

# Racine
if (Install-NodeDeps) { $installed = $true }
if (Install-PythonDeps) { $installed = $true }

# Sous-dossiers courants (monorepo)
$subdirs = @("apps", "packages", "frontend", "backend", "api", "web", "server", "client")
foreach ($sub in $subdirs) {
    $path = Join-Path $ProjectRoot $sub
    if (-not (Test-Path $path -PathType Container)) { continue }
    Get-ChildItem $path -Directory | ForEach-Object {
        if (Install-NodeDeps $_.FullName) { $installed = $true }
        if (Install-PythonDeps $_.FullName) { $installed = $true }
    }
}

if (-not $installed) {
    Write-Host "`nAucun package.json, requirements.txt ou pyproject.toml trouvé à la racine." -ForegroundColor Yellow
    Write-Host "Assurez-vous d'avoir cloné le dépôt :" -ForegroundColor Gray
    Write-Host "  git clone https://github.com/StudioMiyukini/miyukini-core-system.git ." -ForegroundColor Gray
    exit 1
}

Write-Host "`n=== Installation terminée ===" -ForegroundColor Green
