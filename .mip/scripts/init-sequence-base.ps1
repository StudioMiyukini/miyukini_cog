# Script 1 : Initialise les artefacts de base d'une sequence MIP.
# Cree le brief, les metriques, et les artefacts P0 T1-T2.
# A executer au debut de chaque sequence, avant P0.
#
# Usage :
#   powershell -ExecutionPolicy Bypass -File init-sequence-base.ps1 -SequencePath <chemin>
#
# Apres P0 T2, executer :
#   init-sequence-by-complexity.ps1 -SequencePath <chemin> -Complexity <C1|C2|C3|C4|C5>

param(
    [Parameter(Mandatory = $true)]
    [string]$SequencePath
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

# --- Resolution sequence ---

$sequenceRoot = Resolve-SequenceRoot -InputPath $SequencePath
$leaf = Split-Path -Path $sequenceRoot -Leaf

if ($leaf -notmatch "^(?<date>\d{4}-\d{2}-\d{2})-(?<slug>[a-z0-9][a-z0-9\-]*)$") {
    throw "Le dossier de sequence doit suivre YYYY-MM-DD-<slug>. Recu: $leaf"
}

$date      = $Matches["date"]
$timestamp = (Get-Date).ToString("s") + "Z"

$scriptDir   = Split-Path -Parent $MyInvocation.MyCommand.Path
$templateRoot = (Resolve-Path (Join-Path $scriptDir "..\sequences\_template")).Path
$baseRoot     = Join-Path $templateRoot "base"

# --- Repertoires minimaux ---

@("briefs", "metrics", "phases\p0\temps") | ForEach-Object {
    Ensure-Directory -Path (Join-Path $sequenceRoot $_)
}

# --- Copie des artefacts base (brief, metriques, T1, T2) ---

Write-Host ""
Write-Host "=== Initialisation base sequence: $leaf ==="

Get-ChildItem -LiteralPath $baseRoot -Recurse -File | ForEach-Object {
    $relPath  = $_.FullName.Substring($baseRoot.Length).TrimStart('\', '/')
    $destRel  = $relPath -replace [regex]::Escape("DATE-SLUG"), $leaf
    $destPath = Join-Path $sequenceRoot $destRel
    Copy-AndSubstitute -TemplatePath $_.FullName -DestPath $destPath -Leaf $leaf -Date $date -Timestamp $timestamp
}

Write-Host ""
Write-Host "Base initialisee pour: $leaf"
Write-Host ""
Write-Host "Prochaines etapes :"
Write-Host "  1. Executer P0 T1 (Exploration)"
Write-Host "  2. Executer P0 T2 (Ideation)"
Write-Host "  3. Estimer la complexite (C1 a C5)"
Write-Host "  4. Lancer le script de complexite :"
Write-Host "     powershell -ExecutionPolicy Bypass -File init-sequence-by-complexity.ps1 -SequencePath $leaf -Complexity <C1|C2|C3|C4|C5>"
