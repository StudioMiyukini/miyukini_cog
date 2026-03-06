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

function New-PlaceholderMarkdown {
    param(
        [string]$Title,
        [string]$Phase,
        [string]$Owner
    )
@"
# $Title

## Statut

- Etat : A completer
- Phase : $Phase
- Responsable principal : $Owner

## TL;DR

Document pre-cree pour la sequence. Les agents completeront ce fichier au fil de l'execution.

## Contenu

- [A completer]
"@
}

$sequenceRoot = Resolve-SequenceRoot -InputPath $SequencePath
$leaf = Split-Path -Path $sequenceRoot -Leaf

if ($leaf -notmatch "^(?<date>\d{4}-\d{2}-\d{2})-(?<slug>[a-z0-9][a-z0-9\-]*)$") {
    throw "Le dossier de sequence doit suivre YYYY-MM-DD-<slug>. Recu: $leaf"
}

$date = $Matches["date"]
$slug = $Matches["slug"]

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$templateRoot = Join-Path $scriptDir "..\sequences\_template"

$dirs = @(
    "briefs",
    "specs",
    "gpi",
    "phases",
    "phases\p0",
    "phases\p0\temps",
    "plans_p3",
    "plans_p3\etapes",
    "audits",
    "metrics",
    "rapports_finaux",
    "ressources",
    "agents",
    "ui"
)

foreach ($dir in $dirs) {
    Ensure-Directory -Path (Join-Path $sequenceRoot $dir)
}

$files = @{}

$files["briefs/$date-$slug.md"] = New-PlaceholderMarkdown -Title "Brief sequence $slug" -Phase "P0" -Owner "Maria"
$files["briefs/$date-$slug-P0-travail.md"] = New-PlaceholderMarkdown -Title "Travail P0 $slug" -Phase "P0" -Owner "Maria"
$files["specs/$date-$slug-spec.md"] = New-PlaceholderMarkdown -Title "Specification $slug" -Phase "P0 Temps 6" -Owner "Francois"
$files["gpi/$date-$slug-gpi.md"] = New-PlaceholderMarkdown -Title "GPI $slug" -Phase "P3" -Owner "Denis/Victor"

$files["plans_p3/$date-$slug-plan.md"] = New-PlaceholderMarkdown -Title "Plan P3 $slug" -Phase "P3" -Owner "Denis"
$files["plans_p3/etapes/index.md"] = @"
# Index des etapes P3

- etape-01.md
- etape-02.md
- etape-03.md
- etape-04.md
- etape-05.md

Chaque etape est pre-creee pour permettre un suivi incremental.
"@

1..5 | ForEach-Object {
    $n = $_.ToString("00")
    $files["plans_p3/etapes/etape-$n.md"] = New-PlaceholderMarkdown -Title "Etape P3 $n" -Phase "P3" -Owner "Denis + agents"
}

$files["phases/p0-trace.md"] = New-PlaceholderMarkdown -Title "Trace P0" -Phase "P0" -Owner "Maria"
$files["phases/p3-trace.md"] = New-PlaceholderMarkdown -Title "Trace P3" -Phase "P3" -Owner "Denis"
$files["phases/p4-trace.md"] = New-PlaceholderMarkdown -Title "Trace P4" -Phase "P4" -Owner "George/Victor"
$files["phases/p5-trace.md"] = New-PlaceholderMarkdown -Title "Trace P5" -Phase "P5" -Owner "Denis"
$files["phases/p6-trace.md"] = New-PlaceholderMarkdown -Title "Trace P6" -Phase "P6" -Owner "Arianne"
$files["phases/dag.json"] = "{`n  `"nodes`": [],`n  `"edges`": []`n}`n"

$p0Times = @(
    @{ Num = 1; Slug = "exploration"; Title = "Exploration et brainstorming"; Owner = "Maria" },
    @{ Num = 2; Slug = "ideation"; Title = "Ideation"; Owner = "Maria/Lise" },
    @{ Num = 3; Slug = "concurrence"; Title = "Analyse concurrentielle"; Owner = "Fabrice" },
    @{ Num = 4; Slug = "inventaire"; Title = "Inventaire prerequis"; Owner = "Denis/Hugo/Jean" },
    @{ Num = 5; Slug = "securite"; Title = "Analyse securite"; Owner = "Victor" },
    @{ Num = 6; Slug = "specification"; Title = "Specification technique"; Owner = "Francois" },
    @{ Num = 7; Slug = "agents-finetunes"; Title = "Generation agents fine-tuned"; Owner = "Maria" },
    @{ Num = 8; Slug = "plan-execution"; Title = "Plan execution"; Owner = "Denis" },
    @{ Num = 9; Slug = "faisabilite"; Title = "Audit faisabilite"; Owner = "Arianne/Jean" },
    @{ Num = 10; Slug = "cicd"; Title = "Verification CI/CD"; Owner = "Hugo" },
    @{ Num = 11; Slug = "synthese-brief"; Title = "Synthese et brief"; Owner = "Maria" }
)

foreach ($t in $p0Times) {
    $n = [int]$t.Num
    $path = "phases/p0/temps/temps-$($n.ToString("00"))-$($t.Slug).md"
    $files[$path] = New-PlaceholderMarkdown -Title "P0 Temps $n - $($t.Title)" -Phase "P0 Temps $n" -Owner $t.Owner
}

$files["agents/index.md"] = @"
# Agents fine-tuned de sequence

## Statut
- Etat : A completer
- Phase : P0 Temps 7
- Responsable principal : Maria

## TL;DR
Inventaire des prompts fine-tuned de sequence derives de T4/T5/T6.

## Fichiers
- manifest.json
- <PHASE>_<agent>.md
"@

$files["agents/manifest.json"] = @"
{
  "sequence": "$leaf",
  "generated_at": "$((Get-Date).ToString("s"))",
  "regeneration_mode": "update",
  "target_phases": ["P3", "P4", "P5", "P6", "MASS"],
  "inputs": [
    "phases/p0/temps/temps-04-inventaire.md",
    "phases/p0/temps/temps-05-securite.md",
    "specs/$date-$slug-spec.md"
  ],
  "agents": [],
  "files": []
}
"@

$files["audits/$date-$slug.md"] = New-PlaceholderMarkdown -Title "Audit global $slug" -Phase "P4" -Owner "George"
$files["audits/$date-$slug-efficiency.md"] = New-PlaceholderMarkdown -Title "Audit efficience $slug" -Phase "P4" -Owner "Jean"
$files["audits/$date-$slug-pass-0.md"] = New-PlaceholderMarkdown -Title "PASS-0 securite $slug" -Phase "P4" -Owner "Victor"
$files["audits/$date-$slug-pass-01.md"] = New-PlaceholderMarkdown -Title "PASS-01 securite $slug" -Phase "P4" -Owner "Victor"
$files["audits/$date-$slug-ras.md"] = New-PlaceholderMarkdown -Title "RAS securite $slug" -Phase "P4" -Owner "Victor"
$files["audits/$date-$slug-p5-validation.md"] = New-PlaceholderMarkdown -Title "Validation P5 $slug" -Phase "P5" -Owner "George"

$files["rapports_finaux/$date-$slug-report.md"] = New-PlaceholderMarkdown -Title "Rapport final $slug" -Phase "P6" -Owner "Arianne"
$files["metrics/$date-$slug.json"] = @"
{
  "sequence": "$leaf",
  "created_at": "$((Get-Date).ToString("s"))",
  "p0": {},
  "p3": {},
  "p4": {},
  "p5": {},
  "p6": {}
}
"@

$resTemplate = Join-Path $templateRoot "ressources\index.md"
if (Test-Path -LiteralPath $resTemplate) {
    Ensure-File -Path (Join-Path $sequenceRoot "ressources\index.md") -Content (Get-Content -LiteralPath $resTemplate -Raw)
} else {
    Ensure-File -Path (Join-Path $sequenceRoot "ressources\index.md") -Content (New-PlaceholderMarkdown -Title "Ressources $slug" -Phase "P0" -Owner "Maria")
}

foreach ($rel in $files.Keys) {
    $full = Join-Path $sequenceRoot ($rel -replace "/", "\\")
    Ensure-File -Path $full -Content $files[$rel]
}

$uiTemplate = Join-Path $templateRoot "ui\index.html"
if (Test-Path -LiteralPath $uiTemplate) {
    Ensure-File -Path (Join-Path $sequenceRoot "ui\index.html") -Content (Get-Content -LiteralPath $uiTemplate -Raw)
}

$p0TimeEntries = @()
foreach ($t in $p0Times) {
    $n = [int]$t.Num
    $p0TimeEntries += [ordered]@{
        label = "Temps $($n.ToString("00")) - $($t.Title)"
        path = "phases/p0/temps/temps-$($n.ToString("00"))-$($t.Slug).md"
    }
}

$p3StepEntries = @()
1..5 | ForEach-Object {
    $n = $_.ToString("00")
    $p3StepEntries += [ordered]@{
        label = "Etape $n"
        path = "plans_p3/etapes/etape-$n.md"
    }
}

$manifest = [ordered]@{
    identity = [ordered]@{
        sequence = $leaf
        date = $date
        slug = $slug
        generated_at = (Get-Date).ToString("s")
    }
    tabs = @(
        [ordered]@{
            id = "p0"
            label = "P0"
            groups = @(
                [ordered]@{
                    label = "Documents P0"
                    files = @(
                        [ordered]@{ label = "Brief principal"; path = "briefs/$date-$slug.md" },
                        [ordered]@{ label = "Travail P0"; path = "briefs/$date-$slug-P0-travail.md" },
                        [ordered]@{ label = "Specification"; path = "specs/$date-$slug-spec.md" },
                        [ordered]@{ label = "Trace P0"; path = "phases/p0-trace.md" }
                    )
                },
                [ordered]@{
                    label = "Agents fine-tuned"
                    files = @(
                        [ordered]@{ label = "Index agents sequence"; path = "agents/index.md" },
                        [ordered]@{ label = "Manifest agents"; path = "agents/manifest.json" }
                    )
                },
                [ordered]@{
                    label = "Temps P0"
                    files = $p0TimeEntries
                }
            )
        },
        [ordered]@{
            id = "p3"
            label = "P3"
            groups = @(
                [ordered]@{
                    label = "GPI"
                    files = @(
                        [ordered]@{ label = "GPI sequence"; path = "gpi/$date-$slug-gpi.md" }
                    )
                },
                [ordered]@{
                    label = "Taches par etapes"
                    files = @(
                        [ordered]@{ label = "Plan P3"; path = "plans_p3/$date-$slug-plan.md" },
                        [ordered]@{ label = "Trace P3"; path = "phases/p3-trace.md" },
                        [ordered]@{ label = "Index etapes"; path = "plans_p3/etapes/index.md" }
                    ) + $p3StepEntries
                }
            )
        },
        [ordered]@{
            id = "p4"
            label = "P4"
            groups = @(
                [ordered]@{
                    label = "Audits et securite"
                    files = @(
                        [ordered]@{ label = "Audit global"; path = "audits/$date-$slug.md" },
                        [ordered]@{ label = "PASS-0"; path = "audits/$date-$slug-pass-0.md" },
                        [ordered]@{ label = "PASS-01"; path = "audits/$date-$slug-pass-01.md" },
                        [ordered]@{ label = "RAS"; path = "audits/$date-$slug-ras.md" },
                        [ordered]@{ label = "Efficiency"; path = "audits/$date-$slug-efficiency.md" },
                        [ordered]@{ label = "Trace P4"; path = "phases/p4-trace.md" }
                    )
                }
            )
        },
        [ordered]@{
            id = "p5"
            label = "P5"
            groups = @(
                [ordered]@{
                    label = "Validation utilisateur"
                    files = @(
                        [ordered]@{ label = "Validation P5"; path = "audits/$date-$slug-p5-validation.md" },
                        [ordered]@{ label = "Trace P5"; path = "phases/p5-trace.md" }
                    )
                }
            )
        },
        [ordered]@{
            id = "rapport-final"
            label = "Rapport final"
            groups = @(
                [ordered]@{
                    label = "Cloture"
                    files = @(
                        [ordered]@{ label = "Rapport final"; path = "rapports_finaux/$date-$slug-report.md" },
                        [ordered]@{ label = "Trace P6"; path = "phases/p6-trace.md" },
                        [ordered]@{ label = "Metriques"; path = "metrics/$date-$slug.json" }
                    )
                }
            )
        }
    )
}

$manifestJson = $manifest | ConvertTo-Json -Depth 8
Ensure-File -Path (Join-Path $sequenceRoot "ui\manifest.json") -Content $manifestJson

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
