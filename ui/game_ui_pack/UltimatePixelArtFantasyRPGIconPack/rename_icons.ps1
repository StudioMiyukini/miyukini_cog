# Renomme N.png en icon_NNNN.png (4 chiffres) dans le dossier cible.
# Usage: .\rename_icons.ps1 [-Folder "Dark Outline\32X32"]
param([string]$Folder = "Dark Outline\32X32")

$base = $PSScriptRoot
$target = Join-Path $base $Folder
if (-not (Test-Path $target)) { Write-Error "Folder not found: $target"; exit 1 }

$count = 0
Get-ChildItem -Path $target -Filter "*.png" | ForEach-Object {
    if ($_.Name -match '^(\d+)\.png$') {
        $n = [int]$Matches[1]
        $newName = "icon_{0:D4}.png" -f $n
        if ($_.Name -ne $newName) {
            $newPath = Join-Path $target $newName
            if (-not (Test-Path $newPath)) {
                Rename-Item -Path $_.FullName -NewName $newName
                $count++
            }
        }
    }
}
Write-Host "Renamed $count files in $Folder"
