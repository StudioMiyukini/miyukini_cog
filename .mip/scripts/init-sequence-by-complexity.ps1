# Script 2 : Cree les artefacts de sequence en fonction de la complexite C1-C5.
# A executer a la fin de P0 T2, une fois la complexite estimee.
#
# Complexite :
#   C1 - Mineure     : petit fix (1 fichier)
#   C2 - Faible      : grand fix < 3 fichiers
#   C3 - Moyenne     : fonctionnalite
#   C4 - Elevee      : petit service COG
#   C5 - Strategique : App / services complexes / architecture
#
# Usage :
#   powershell -ExecutionPolicy Bypass -File init-sequence-by-complexity.ps1 `
#     -SequencePath <chemin> -Complexity <C1|C2|C3|C4|C5>
#
# Application : C3 applique les deltas c3, c2, c1 (ordre descendant).
# Le niveau le plus eleve est applique en premier pour que ses templates
# priment sur les versions de meme nom des niveaux inferieurs.
# Les niveaux inferieurs completent les fichiers manquants (Ensure-File SKIP).

param(
    [Parameter(Mandatory = $true)]
    [string]$SequencePath,

    [Parameter(Mandatory = $true)]
    [ValidateSet("C1", "C2", "C3", "C4", "C5")]
    [string]$Complexity
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Resolve-SequenceRoot {
    param([string]$InputPath)
    if (Test-Path -LiteralPath $InputPath) {
        return (Resolve-Path -LiteralPath $InputPath).Path
    }
    $candidate = Join-Path (Get-Location) $InputPath
    if (Test-Path -LiteralPath $candidate) {
        return (Resolve-Path -LiteralPath $candidate).Path
    }
    throw "SequencePath introuvable: $InputPath"
}

function Ensure-Directory {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        New-Item -ItemType Directory -Path $Path | Out-Null
    }
}

function Ensure-File {
    param([string]$Path, [string]$Content)
    if (Test-Path -LiteralPath $Path) {
        Write-Host "SKIP (exists): $Path"
        return
    }
    $parent = Split-Path -Path $Path -Parent
    Ensure-Directory -Path $parent
    Set-Content -LiteralPath $Path -Value $Content -Encoding UTF8
    Write-Host "CREATE $Path"
}

function Copy-AndSubstitute {
    param([string]$TemplatePath, [string]$DestPath, [string]$Leaf, [string]$Date, [string]$Timestamp)
    $content = Get-Content -LiteralPath $TemplatePath -Raw -Encoding UTF8
    $content = $content -replace [regex]::Escape("DATE-SLUG"), $Leaf
    $content = $content -replace [regex]::Escape("THE-DATE"),  $Date
    $content = $content -replace [regex]::Escape("__TIMESTAMP__"), $Timestamp
    Ensure-File -Path $DestPath -Content $content
}

function Apply-ComplexityDelta {
    param([string]$DeltaDir, [string]$SequenceRoot, [string]$Leaf, [string]$Date, [string]$Timestamp)

    if (-not (Test-Path -LiteralPath $DeltaDir)) {
        Write-Host "  (aucun delta pour ce niveau)"
        return
    }

    $files = @(Get-ChildItem -LiteralPath $DeltaDir -Recurse -File)
    if ($files.Count -eq 0) {
        Write-Host "  (delta vide)"
        return
    }

    $files | ForEach-Object {
        $relPath  = $_.FullName.Substring($DeltaDir.Length).TrimStart('\', '/')
        $destRel  = $relPath -replace [regex]::Escape("DATE-SLUG"), $Leaf
        $destPath = Join-Path $SequenceRoot $destRel
        Copy-AndSubstitute -TemplatePath $_.FullName -DestPath $destPath -Leaf $Leaf -Date $Date -Timestamp $Timestamp
    }
}

# --- Resolution sequence ---

$sequenceRoot = Resolve-SequenceRoot -InputPath $SequencePath
$leaf = Split-Path -Path $sequenceRoot -Leaf

if ($leaf -notmatch "^(?<date>\d{4}-\d{2}-\d{2})-(?<slug>[a-z0-9][a-z0-9\-]*)$") {
    throw "Le dossier de sequence doit suivre YYYY-MM-DD-<slug>. Recu: $leaf"
}

$date           = $Matches["date"]
$timestamp      = (Get-Date).ToString("s") + "Z"
$complexityNum  = [int]$Complexity.Substring(1)

$scriptDir    = Split-Path -Parent $MyInvocation.MyCommand.Path
$templateRoot = (Resolve-Path (Join-Path $scriptDir "..\sequences\_template")).Path

$complexityLabels = @{
    1 = "C1 - Mineure (petit fix)"
    2 = "C2 - Faible (grand fix < 3 fichiers)"
    3 = "C3 - Moyenne (fonctionnalite)"
    4 = "C4 - Elevee (petit service COG)"
    5 = "C5 - Strategique (App / services complexes / architecture)"
}

Write-Host ""
Write-Host "=== Initialisation par complexite : $($complexityLabels[$complexityNum]) ==="
Write-Host "    Sequence : $leaf"
Write-Host ""

# --- Application des deltas cN..c1 (niveau le plus specifique en premier) ---

for ($level = $complexityNum; $level -ge 1; $level--) {
    $label    = $complexityLabels[$level]
    $deltaDir = Join-Path $templateRoot "c$level"

    Write-Host "--- Delta C$level : $label ---"
    Apply-ComplexityDelta -DeltaDir $deltaDir -SequenceRoot $sequenceRoot -Leaf $leaf -Date $date -Timestamp $timestamp
    Write-Host ""
}

# --- Resume ---

Write-Host "Artefacts crees pour complexite $Complexity : $leaf"

if ($complexityNum -ge 3) {
    Write-Host ""
    Write-Host "Mini-site disponible : $sequenceRoot\ui\index.html"
}

# --- Mise a jour index sequences ---

$rebuildIndexScript = Join-Path $scriptDir "rebuild-sequences-index.ps1"
if (Test-Path -LiteralPath $rebuildIndexScript) {
    try {
        & powershell -NoProfile -ExecutionPolicy Bypass -File $rebuildIndexScript | Out-Null
        Write-Host "Index sequences mis a jour."
    } catch {
        Write-Warning "Impossible de regenerer sequences/index.json: $($_.Exception.Message)"
    }
}
