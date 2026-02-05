# Range les icônes Lucide dans des dossiers par catégorie (d'après les JSON du dépôt Lucide).
# Usage: depuis la racine du repo, avec temp_lucide contenant le clone Lucide :
#   .\ui\lucide_icons\organize_by_category.ps1

$ErrorActionPreference = "Stop"
$root = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
$iconsRoot = Join-Path $root "ui\lucide_icons"
$tempLucideIcons = Join-Path $root "temp_lucide\icons"

if (-not (Test-Path $tempLucideIcons)) {
    Write-Error "temp_lucide\icons introuvable. Clonez d'abord: git clone --depth 1 https://github.com/lucide-icons/lucide.git temp_lucide"
}

$moved = 0
$skipped = 0
$errors = 0

Get-ChildItem -Path $tempLucideIcons -Filter "*.json" | ForEach-Object {
    $name = [System.IO.Path]::GetFileNameWithoutExtension($_.Name)
    $svgPath = Join-Path $iconsRoot "$name.svg"
    if (-not (Test-Path $svgPath)) {
        $script:skipped++
        return
    }
    try {
        $json = Get-Content -Path $_.FullName -Raw | ConvertFrom-Json
        $categories = $json.categories
        if (-not $categories -or $categories.Count -eq 0) {
            $cat = "other"
        } else {
            $cat = $categories[0]
        }
        $destDir = Join-Path $iconsRoot $cat
        if (-not (Test-Path $destDir)) {
            New-Item -ItemType Directory -Path $destDir -Force | Out-Null
        }
        $destPath = Join-Path $destDir "$name.svg"
        Move-Item -Path $svgPath -Destination $destPath -Force
        $script:moved++
    } catch {
        Write-Warning "Erreur pour $name : $_"
        $script:errors++
    }
}

Write-Host "Terminé: $moved déplacés, $skipped SVG absents, $errors erreurs."
