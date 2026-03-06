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
    param(
        [string]$Path,
        [string]$Content
    )

    if (Test-Path -LiteralPath $Path) {
        Write-Host "SKIP file exists: $Path"
        return
    }

    $parent = Split-Path -Path $Path -Parent
    Ensure-Directory -Path $parent
    Set-Content -LiteralPath $Path -Value $Content -Encoding UTF8
    Write-Host "CREATE $Path"
}

function Copy-AndSubstitute {
    param(
        [string]$TemplatePath,
        [string]$DestPath,
        [string]$Leaf,
        [string]$Date,
        [string]$Timestamp
    )

    $content = Get-Content -LiteralPath $TemplatePath -Raw -Encoding UTF8
    $content = $content -replace [regex]::Escape("DATE-SLUG"), $Leaf
    $content = $content -replace [regex]::Escape("THE-DATE"), $Date
    $content = $content -replace [regex]::Escape("__TIMESTAMP__"), $Timestamp

    Ensure-File -Path $DestPath -Content $content
}

# --- Resolve sequence root and extract date/slug ---

$sequenceRoot = Resolve-SequenceRoot -InputPath $SequencePath
$leaf = Split-Path -Path $sequenceRoot -Leaf

if ($leaf -notmatch "^(?<date>\d{4}-\d{2}-\d{2})-(?<slug>[a-z0-9][a-z0-9\-]*)$") {
    throw "Le dossier de sequence doit suivre YYYY-MM-DD-<slug>. Recu: $leaf"
}

$date = $Matches["date"]
$timestamp = (Get-Date).ToString("s") + "Z"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$templateRoot = (Resolve-Path (Join-Path $scriptDir "..\sequences\_template")).Path

# --- Walk all template files and copy with DATE-SLUG substitution ---

Get-ChildItem -LiteralPath $templateRoot -Recurse -File | ForEach-Object {
    $templateFile = $_.FullName
    $relPath = $templateFile.Substring($templateRoot.Length).TrimStart('\', '/')

    # Substitute DATE-SLUG in the destination path as well
    $destRel = $relPath -replace [regex]::Escape("DATE-SLUG"), $leaf
    $destPath = Join-Path $sequenceRoot $destRel

    Copy-AndSubstitute `
        -TemplatePath $templateFile `
        -DestPath     $destPath `
        -Leaf         $leaf `
        -Date         $date `
        -Timestamp    $timestamp
}

Write-Host ""
Write-Host "Scaffold standard initialise pour la sequence: $leaf"
Write-Host "Mini-site: $sequenceRoot\ui\index.html"

$rebuildIndexScript = Join-Path $scriptDir "rebuild-sequences-index.ps1"
if (Test-Path -LiteralPath $rebuildIndexScript) {
    try {
        & powershell -NoProfile -ExecutionPolicy Bypass -File $rebuildIndexScript | Out-Null
        Write-Host "Index sequences mis a jour: $(Join-Path (Resolve-Path (Join-Path $scriptDir "..")) "sequences\index.json")"
    } catch {
        Write-Warning "Impossible de regenerer sequences/index.json automatiquement: $($_.Exception.Message)"
    }
}
