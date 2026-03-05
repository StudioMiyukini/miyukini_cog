param(
    [Parameter(Mandatory = $true)]
    [string]$SequencePath,

    [ValidateSet("update", "create-only", "overwrite")]
    [string]$RegenerationMode = "update",

    [string[]]$TargetPhases = @("P3", "P4", "P5", "P6", "MASS"),

    [switch]$DryRun
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$GeneratorId = ".mip/scripts/generate-sequence-finetuned-agents.ps1"

function Resolve-ExistingPath {
    param([string]$PathValue)

    if (Test-Path -LiteralPath $PathValue) {
        return (Resolve-Path -LiteralPath $PathValue).Path
    }

    $candidate = Join-Path (Get-Location) $PathValue
    if (Test-Path -LiteralPath $candidate) {
        return (Resolve-Path -LiteralPath $candidate).Path
    }

    throw "Path introuvable: $PathValue"
}

function Ensure-Directory {
    param([string]$Path)
    if (-not (Test-Path -LiteralPath $Path)) {
        if ($DryRun) {
            Write-Host "[DRYRUN] MKDIR $Path"
        } else {
            New-Item -ItemType Directory -Path $Path | Out-Null
        }
    }
}

function Assert-InsideRoot {
    param(
        [string]$Root,
        [string]$PathValue
    )

    $rootFull = [System.IO.Path]::GetFullPath($Root)
    $pathFull = [System.IO.Path]::GetFullPath($PathValue)
    $rootNormalized = $rootFull.TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar
    $pathNormalized = $pathFull.TrimEnd('\', '/') + [System.IO.Path]::DirectorySeparatorChar

    if (-not $pathNormalized.StartsWith($rootNormalized, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Ecriture hors sequence interdite: $pathFull"
    }
}

function Read-FileSafe {
    param([string]$Path)

    if (Test-Path -LiteralPath $Path) {
        return Get-Content -LiteralPath $Path -Raw
    }
    return ""
}

function Get-FrontMatter {
    param([string]$Content)

    $map = @{}
    if (-not $Content.StartsWith("---")) {
        return $map
    }

    $lines = $Content -split "`r?`n"
    if ($lines.Count -lt 3) {
        return $map
    }

    $end = -1
    for ($i = 1; $i -lt $lines.Count; $i++) {
        if ($lines[$i] -eq "---") {
            $end = $i
            break
        }
    }

    if ($end -lt 0) {
        return $map
    }

    for ($i = 1; $i -lt $end; $i++) {
        $line = $lines[$i]
        if ($line -match "^([A-Za-z0-9_-]+):\s*(.+)$") {
            $map[$Matches[1].ToLowerInvariant()] = $Matches[2].Trim()
        }
    }

    return $map
}

function Extract-RoleBullets {
    param([string]$Content)

    $lines = $Content -split "`r?`n"
    $start = -1
    for ($i = 0; $i -lt $lines.Count; $i++) {
        if ($lines[$i] -match "^##\s+Ton role principal") {
            $start = $i + 1
            break
        }
    }

    $bullets = @()
    if ($start -lt 0) {
        return $bullets
    }

    for ($i = $start; $i -lt $lines.Count; $i++) {
        $line = $lines[$i]
        if ($line -match "^##\s+") {
            break
        }
        if ($line -match "^\s*-\s+(.+)$") {
            $txt = $Matches[1] -replace "\*\*", ""
            $bullets += $txt.Trim()
        }
    }

    return $bullets
}

function Extract-PhaseFocus {
    param([string]$PhaseContent)

    if ($PhaseContent -match "Responsabilites de l'agent pour:\s*([^\r\n\.]+)") {
        return $Matches[1].Trim()
    }

    if ($PhaseContent -match "## Mission de phase\s*([\s\S]+?)##") {
        return ($Matches[1] -replace "`r?`n", " " -replace "\s+", " ").Trim()
    }

    return "Execution ciblee sur la phase"
}

function Detect-Class {
    param(
        [string]$SpecContent,
        [string]$BriefContent
    )

    $combined = "$SpecContent`n$BriefContent"
    if ($combined -match "\bT([3-5])\b") {
        return "T$($Matches[1])"
    }
    return "T3"
}

function Build-BaselineAgents {
    param([string]$Class)

    $base = [System.Collections.Generic.List[string]]::new()
    foreach ($a in @("maria", "denis", "francois", "lise", "victor", "george", "jean", "arianne")) {
        $base.Add($a)
    }

    if ($Class -in @("T4", "T5")) {
        $base.Add("hugo")
    }
    if ($Class -eq "T5") {
        $base.Add("bob")
    }

    return $base
}

function Add-IfMissing {
    param(
        [System.Collections.Generic.HashSet[string]]$Set,
        [string]$Value
    )
    if ($Value) {
        [void]$Set.Add($Value)
    }
}

function Get-PhaseModule {
    param([string]$Phase)

    switch ($Phase) {
        "P3" { return ".mip/modules/p3-execution.md" }
        "P4" { return ".mip/modules/p4-p5-p6.md" }
        "P5" { return ".mip/modules/p4-p5-p6.md" }
        "P6" { return ".mip/modules/p4-p5-p6.md" }
        "MASS" { return ".mip/modules/mass.md" }
        default { return ".mip/modules/workflow.md" }
    }
}

function Get-DefaultOutputPath {
    param(
        [string]$Phase,
        [string]$Date,
        [string]$Slug
    )

    switch ($Phase) {
        "P3" { return "plans_p3/$Date-$Slug-plan.md" }
        "P4" { return "audits/$Date-$Slug.md" }
        "P5" { return "audits/$Date-$Slug-p5-validation.md" }
        "P6" { return "rapports_finaux/$Date-$Slug-report.md" }
        "MASS" { return "phases/dag.json" }
        default { return "phases/p0-trace.md" }
    }
}

function Is-ManagedFile {
    param([string]$Path)

    if (-not (Test-Path -LiteralPath $Path)) {
        return $false
    }

    $head = (Get-Content -LiteralPath $Path -TotalCount 8) -join "`n"
    return $head -match [Regex]::Escape($GeneratorId)
}

function Should-Write {
    param(
        [string]$Path,
        [string]$Mode
    )

    if (-not (Test-Path -LiteralPath $Path)) {
        return $true
    }

    switch ($Mode) {
        "create-only" { return $false }
        "overwrite" { return $true }
        "update" { return (Is-ManagedFile -Path $Path) }
        default { return $false }
    }
}

function Write-ManagedFile {
    param(
        [string]$Path,
        [string]$Content,
        [string]$Mode
    )

    $dir = Split-Path -Path $Path -Parent
    Ensure-Directory -Path $dir

    $write = Should-Write -Path $Path -Mode $Mode
    if (-not $write) {
        Write-Host "SKIP (mode=$Mode, unmanaged/existing): $Path"
        return $false
    }

    if ($DryRun) {
        Write-Host "[DRYRUN] WRITE $Path"
        return $true
    }

    Set-Content -LiteralPath $Path -Value $Content -Encoding UTF8
    Write-Host "WRITE $Path"
    return $true
}

$validPhases = @("P3", "P4", "P5", "P6", "MASS")
$targetPhasesNormalized = @()
foreach ($p in $TargetPhases) {
    $u = $p.ToUpperInvariant()
    if ($validPhases -notcontains $u) {
        throw "Phase cible invalide: $p"
    }
    $targetPhasesNormalized += $u
}
$targetPhasesNormalized = $targetPhasesNormalized | Select-Object -Unique

$sequenceRoot = Resolve-ExistingPath -PathValue $SequencePath
$leaf = Split-Path -Path $sequenceRoot -Leaf
if ($leaf -notmatch "^(?<date>\d{4}-\d{2}-\d{2})-(?<slug>[a-z0-9][a-z0-9\-]*)$") {
    throw "Le dossier de sequence doit suivre YYYY-MM-DD-<slug>. Recu: $leaf"
}
$date = $Matches["date"]
$slug = $Matches["slug"]

$mipRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
$sequencesRoot = (Resolve-Path -LiteralPath (Join-Path $mipRoot "sequences")).Path
$agentsLibraryRoot = Join-Path $mipRoot "agents"
$templatePath = Join-Path $agentsLibraryRoot "TEMPLATE_PHASE_AGENT.md"
if (-not (Test-Path -LiteralPath $templatePath)) {
    throw "Template introuvable: $templatePath"
}
Assert-InsideRoot -Root $sequencesRoot -PathValue $sequenceRoot

$sequenceAgentsRoot = Join-Path $sequenceRoot "agents"
Ensure-Directory -Path $sequenceAgentsRoot

Assert-InsideRoot -Root $sequenceRoot -PathValue $sequenceAgentsRoot

$inputFiles = @(
    (Join-Path $sequenceRoot "phases\p0\temps\temps-04-inventaire.md"),
    (Join-Path $sequenceRoot "phases\p0\temps\temps-05-securite.md"),
    (Join-Path $sequenceRoot "specs\$date-$slug-spec.md")
)

$combinedNeedsText = ""
foreach ($f in $inputFiles) {
    $combinedNeedsText += "`n" + (Read-FileSafe -Path $f)
}

$specText = Read-FileSafe -Path (Join-Path $sequenceRoot "specs\$date-$slug-spec.md")
$briefText = Read-FileSafe -Path (Join-Path $sequenceRoot "briefs\$date-$slug.md")
$class = Detect-Class -SpecContent $specText -BriefContent $briefText

$agentNames = @("maria", "fabrice", "denis", "lise", "francois", "victor", "george", "hugo", "jean", "arianne", "bob")
$neededAgents = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::OrdinalIgnoreCase)
$needSignals = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::OrdinalIgnoreCase)

foreach ($a in $agentNames) {
    if ($combinedNeedsText -match "(?i)\b$([Regex]::Escape($a))\b") {
        Add-IfMissing -Set $neededAgents -Value $a
        Add-IfMissing -Set $needSignals -Value "agent:$a"
    }
}

$keywordRules = @(
    @{ Pattern = "(?i)\b(ui|front|frontend|dioxus)\b"; Signal = "cap:ui"; Agents = @("lise") },
    @{ Pattern = "(?i)\b(api|backend|back-end|service|crud)\b"; Signal = "cap:backend"; Agents = @("francois", "denis") },
    @{ Pattern = "(?i)\b(securite|security|owasp|auth|crypto|vuln)\b"; Signal = "cap:security"; Agents = @("victor", "george") },
    @{ Pattern = "(?i)\b(audit|conformite|compliance|ras|pass-0|pass-01)\b"; Signal = "cap:audit"; Agents = @("george", "victor") },
    @{ Pattern = "(?i)\b(infra|ci/cd|cicd|deploy|docker|kubernetes|k8s)\b"; Signal = "cap:infra"; Agents = @("hugo", "denis") },
    @{ Pattern = "(?i)\b(metrics|metriques|tokens|efficience)\b"; Signal = "cap:metrics"; Agents = @("jean", "arianne") },
    @{ Pattern = "(?i)\b(parallel|parallele|mass|dag|swarm|worktree)\b"; Signal = "cap:mass"; Agents = @("maria", "denis", "bob", "francois", "lise", "victor") }
)

foreach ($rule in $keywordRules) {
    if ($combinedNeedsText -match $rule.Pattern) {
        Add-IfMissing -Set $needSignals -Value $rule.Signal
        foreach ($a in $rule.Agents) {
            Add-IfMissing -Set $neededAgents -Value $a
        }
    }
}

if ($neededAgents.Count -lt 2) {
    foreach ($a in (Build-BaselineAgents -Class $class)) {
        Add-IfMissing -Set $neededAgents -Value $a
    }
    Add-IfMissing -Set $needSignals -Value "fallback:baseline-$class"
}

$templateContent = Get-Content -LiteralPath $templatePath -Raw
if ([string]::IsNullOrWhiteSpace($templateContent)) {
    throw "Template vide: $templatePath"
}

$generatedFiles = [System.Collections.Generic.List[object]]::new()
$selectedAgents = [System.Collections.Generic.List[object]]::new()

foreach ($agent in ($neededAgents | Sort-Object)) {
    $agentDir = Join-Path $agentsLibraryRoot $agent
    if (-not (Test-Path -LiteralPath $agentDir)) {
        continue
    }

    $fullPath = Join-Path $agentDir ("FULL_{0}.md" -f $agent)
    if (-not (Test-Path -LiteralPath $fullPath)) {
        continue
    }

    $fullContent = Get-Content -LiteralPath $fullPath -Raw
    $fullFm = Get-FrontMatter -Content $fullContent
    $model = if ($fullFm.ContainsKey("model")) { $fullFm["model"] } else { "sonnet" }
    $tools = if ($fullFm.ContainsKey("tools")) { $fullFm["tools"] } else { "Read, Edit, Write, Grep" }

    $roleBullets = @(Extract-RoleBullets -Content $fullContent)

    $agentPhases = [System.Collections.Generic.List[string]]::new()

    foreach ($phase in $targetPhasesNormalized) {
        $phaseBasePath = Join-Path $agentDir ("{0}_{1}.md" -f $phase, $agent)
        $hasPhaseBase = Test-Path -LiteralPath $phaseBasePath
        $phaseContent = if ($hasPhaseBase) { Get-Content -LiteralPath $phaseBasePath -Raw } else { "" }
        $phaseFocus = if ($hasPhaseBase) { Extract-PhaseFocus -PhaseContent $phaseContent } else { "Derivation directe depuis FULL_$agent.md pour la phase $phase" }
        $sourcePhaseRel = if ($hasPhaseBase) { ".mip/agents/$agent/${phase}_$agent.md" } else { "none (FULL-derived)" }
        $phaseModule = Get-PhaseModule -Phase $phase
        $defaultOutput = Get-DefaultOutputPath -Phase $phase -Date $date -Slug $slug

        $mission = @()
        if ($roleBullets.Count -gt 0) {
            $mission += $roleBullets | Select-Object -First 2
        }
        $mission += "Focus phase ${phase}: $phaseFocus"
        if ($needSignals.Count -gt 0) {
            $mission += "Besoins emerges: $(([string[]]($needSignals | Sort-Object)) -join ', ')"
        }
        while ($mission.Count -lt 3) {
            $mission += "Execution ciblee sur la phase $phase"
        }

        $relativePath = "agents/{0}_{1}.md" -f $phase, $agent
        $outputPath = Join-Path $sequenceRoot ($relativePath -replace "/", "\\")
        Assert-InsideRoot -Root $sequenceRoot -PathValue $outputPath

        $generatedHeader = @(
            "<!-- Generated-By: $GeneratorId -->",
            "<!-- Generated-At: $((Get-Date).ToString('s')) -->",
            "<!-- Source-Template: .mip/agents/TEMPLATE_PHASE_AGENT.md -->",
            "<!-- Source-FULL: .mip/agents/$agent/FULL_$agent.md -->",
            "<!-- Source-PHASE: $sourcePhaseRel -->",
            "<!-- Need-Signals: $(([string[]]($needSignals | Sort-Object)) -join ', ') -->",
            ""
        ) -join "`r`n"

        $content = @"
$generatedHeader---
name: $agent-$($phase.ToLowerInvariant())-sequence
description: >
  Version fine-tuned de sequence pour $agent en phase $phase.
  Derivee du template canonique, du FULL agent, et des besoins emerges T4/T5/T6.
model: $model
tools: $tools
---

Tu es **$agent**, role borne a **$phase** pour la sequence **$leaf**.

## Mission de phase
- $($mission[0])
- $($mission[1])
- $($mission[2])

## Bloc variable injecte par l orchestrateur
- Task ID: `<task_id>`
- Task summary: `<task_summary>`
- Files allowed: `<file_list>`
- Inputs: `<inputs_list>`
- Acceptance criteria: `<acceptance_criteria>`
- Output path: `$defaultOutput`

## Inputs obligatoires
- `.mip/environment.md`
- `$phaseModule`
- `phases/p0/temps/temps-04-inventaire.md`
- `phases/p0/temps/temps-05-securite.md`
- `specs/$date-$slug-spec.md`
- Fichiers autorises de la tache uniquement

## Output obligatoire
- Livrable conforme aux `acceptance criteria`
- Compte rendu court:
```text
[PHASE:$phase] [AGENT:$agent] [TASK:<task_id>]
Actions:
- <action_1>
- <action_2>
Checks:
- <check_1>
- <check_2>
Status: DONE | BLOCKED
```

## Regles d execution (hard)
1. Ne modifier que `Files allowed`.
2. Respecter les invariants MIP et conventions MSCM.
3. Executer les checks requis de la phase avant `DONE`.
4. Si info manquante, retourner `BLOCKED` avec manque explicite.

## Hors scope (interdit)
- Changer architecture globale sans demande explicite.
- Lire des fichiers non autorises "au cas ou".
- Charger `FULL_$agent.md` par defaut.

## Escalade vers FULL_$agent.md (si et seulement si)
1. Ambiguite bloquante non resolvable localement.
2. Regle metier/certification absente de cette version phase.
3. Conflit inter-phase necessitant arbitrage global.

## Sequence d execution courte
1. Lire bloc variable + inputs obligatoires.
2. Executer strictement la tache dans le scope.
3. Lancer checks de phase.
4. Produire output + compte rendu `DONE` ou `BLOCKED`.
"@

        $written = Write-ManagedFile -Path $outputPath -Content $content -Mode $RegenerationMode
        if ($written) {
            $generatedFiles.Add([ordered]@{
                path = $relativePath
                agent = $agent
                phase = $phase
                source_full = ".mip/agents/$agent/FULL_$agent.md"
                source_phase = $sourcePhaseRel
            }) | Out-Null
        }

        $agentPhases.Add($phase)
    }

    if ($agentPhases.Count -gt 0) {
        $selectedAgents.Add([ordered]@{
            agent = $agent
            phases = ($agentPhases | Sort-Object -Unique)
        }) | Out-Null
    }
}

$indexPath = Join-Path $sequenceAgentsRoot "index.md"
Assert-InsideRoot -Root $sequenceRoot -PathValue $indexPath

$indexLines = [System.Collections.Generic.List[string]]::new()
$indexLines.Add("<!-- Generated-By: $GeneratorId -->")
$indexLines.Add("# Agents fine-tuned de sequence")
$indexLines.Add("")
$indexLines.Add("- Sequence: $leaf")
$indexLines.Add("- Date generation: $(Get-Date -Format s)")
$indexLines.Add("- Mode regeneration: $RegenerationMode")
$indexLines.Add("- Target phases: $($targetPhasesNormalized -join ', ')")
$indexLines.Add("- Besoins emerges: $(([string[]]($needSignals | Sort-Object)) -join ', ')")
$indexLines.Add("")
$indexLines.Add("## Fichiers generes")
if ($generatedFiles.Count -eq 0) {
    $indexLines.Add("- Aucun fichier ecrit (mode/regles de regeneration)")
} else {
    foreach ($f in $generatedFiles) {
        $indexLines.Add("- $($f.path) (source: $($f.source_full), $($f.source_phase))")
    }
}

$indexContent = ($indexLines -join "`r`n") + "`r`n"
$indexWriteMode = if ($RegenerationMode -eq "create-only") { "create-only" } else { "overwrite" }
Write-ManagedFile -Path $indexPath -Content $indexContent -Mode $indexWriteMode | Out-Null

$manifestPath = Join-Path $sequenceAgentsRoot "manifest.json"
Assert-InsideRoot -Root $sequenceRoot -PathValue $manifestPath

$manifest = [ordered]@{
    generated_by = $GeneratorId
    sequence = $leaf
    generated_at = (Get-Date).ToString("s")
    regeneration_mode = $RegenerationMode
    target_phases = $targetPhasesNormalized
    inputs = @(
        "phases/p0/temps/temps-04-inventaire.md",
        "phases/p0/temps/temps-05-securite.md",
        "specs/$date-$slug-spec.md",
        ".mip/agents/TEMPLATE_PHASE_AGENT.md"
    )
    need_signals = ($needSignals | Sort-Object)
    class_detected = $class
    agents = $selectedAgents
    files = $generatedFiles
}

$manifestJson = $manifest | ConvertTo-Json -Depth 8
$manifestWriteMode = if ($RegenerationMode -eq "create-only") { "create-only" } else { "overwrite" }
Write-ManagedFile -Path $manifestPath -Content $manifestJson -Mode $manifestWriteMode | Out-Null

Write-Host ""
Write-Host "Generation agents fine-tuned terminee pour: $leaf"
Write-Host "Dossier: $sequenceAgentsRoot"
Write-Host "Fichiers ecrits: $($generatedFiles.Count)"
Write-Host "Mode: $RegenerationMode"
if ($DryRun) {
    Write-Host "Execution en dry-run, aucun fichier modifie."
}
