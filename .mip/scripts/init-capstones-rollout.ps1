$ErrorActionPreference = "Stop"

$registryPath = ".mip/certifications/registry.json"
$sequenceRoot = ".mip/sequences/2026-03-05-certifications-academy-rollout"

if (-not (Test-Path $registryPath)) {
    throw "Missing registry.json at $registryPath"
}

$registry = Get-Content -Raw $registryPath | ConvertFrom-Json
$certProps = $registry.certs.PSObject.Properties
$certCount = ($certProps | Measure-Object).Count

$capstoneRoot = Join-Path $sequenceRoot "capstones"
$wavesRoot = Join-Path $sequenceRoot "plans_p3/waves"
$agentsRoot = Join-Path $sequenceRoot "plans_p3/agents"

$dirs = @($capstoneRoot, $wavesRoot, $agentsRoot)
foreach ($dir in $dirs) {
    New-Item -ItemType Directory -Path $dir -Force | Out-Null
}

function Get-WaveName {
    param(
        [string]$Agent
    )

    switch ($Agent) {
        "victor" { return "wave-01-security-compliance" }
        "george" { return "wave-01-security-compliance" }
        "denis" { return "wave-02-architecture-quality" }
        "arianne" { return "wave-02-architecture-quality" }
        "maria" { return "wave-03-product-delivery" }
        "fabrice" { return "wave-03-product-delivery" }
        "francois" { return "wave-03-product-delivery" }
        "lise" { return "wave-03-product-delivery" }
        "hugo" { return "wave-04-platform-ops" }
        "jean" { return "wave-04-platform-ops" }
        default { return "wave-05-backlog" }
    }
}

$waveMap = @{}
$agentMap = @{}

foreach ($prop in $certProps) {
    $certId = $prop.Name
    $meta = $prop.Value
    $agent = $meta.agent
    $folder = $meta.folder
    $name = $meta.name
    $wave = Get-WaveName -Agent $agent

    if (-not $waveMap.ContainsKey($wave)) { $waveMap[$wave] = @() }
    if (-not $agentMap.ContainsKey($agent)) { $agentMap[$agent] = @() }

    $academyDir = ".mip/certifications/$folder/academy"
    New-Item -ItemType Directory -Path $academyDir -Force | Out-Null

    $proofDir = Join-Path $capstoneRoot $agent
    New-Item -ItemType Directory -Path $proofDir -Force | Out-Null
    $proofFile = Join-Path $proofDir "$folder.md"

    if (-not (Test-Path $proofFile)) {
        $proofContent = @"
# Capstone Evidence - $name

**Cert ID**: $certId
**Agent**: $agent
**Wave**: $wave
**Status**: IN_PROGRESS

## Mission

Executer une mission complete en appliquant les exigences du referentiel.

## Evidence Log

- [ ] Contexte et perimetre de la mission
- [ ] Decisions techniques/cadrage prises
- [ ] Tests/audits appliques
- [ ] Ecarts identifies et corrections
- [ ] Conclusion et readiness finale

## Artifacts

- Brief/spec/plan lie a la mission
- References academy utilisees
- Lien assessment mis a jour
"@
        Set-Content -Path $proofFile -Value $proofContent -Encoding utf8
    }

    $runFile = Join-Path $academyDir "CAPSTONE-RUN.md"
    $runContent = @"
# Capstone Run - $name

**Cert ID**: $certId
**Agent**: $agent
**Wave**: $wave
**Evidence file**: ../../../../sequences/2026-03-05-certifications-academy-rollout/capstones/$agent/$folder.md

## Checklist execution

- [x] Mission capstone definie
- [x] Fichier de preuves initialise
- [ ] Mission executee en sequence
- [ ] Assessment mis a jour avec resultats
- [ ] Statut synchronise dans diplomas/STATUS.json

## References

- [Parcours academy](./INDEX.md)
- [Assessment](./ASSESSMENT.md)
- [Capstone evidence](../../../../sequences/2026-03-05-certifications-academy-rollout/capstones/$agent/$folder.md)
"@
    Set-Content -Path $runFile -Value $runContent -Encoding utf8

    $row = [PSCustomObject]@{
        cert_id = $certId
        cert_name = $name
        folder = $folder
        agent = $agent
        wave = $wave
        proof = "capstones/$agent/$folder.md"
        run = ".mip/certifications/$folder/academy/CAPSTONE-RUN.md"
    }

    $waveMap[$wave] += $row
    $agentMap[$agent] += $row
}

# Wave files
$waveIndexRows = @()
foreach ($wave in ($waveMap.Keys | Sort-Object)) {
    $rows = $waveMap[$wave] | Sort-Object agent, cert_name
    $tableRows = @()
    foreach ($r in $rows) {
        $tableRows += "| $($r.agent) | $($r.cert_name) | $($r.cert_id) | ../../$($r.proof) | $($r.run) |"
    }

    $waveFile = Join-Path $wavesRoot "$wave.md"
    $waveContent = @"
# $wave

## Scope

| Agent | Certification | Cert ID | Evidence | Capstone run |
|---|---|---|---|---|
$($tableRows -join "`n")

## Gate

- [ ] Tous les capstones de la vague executes
- [ ] Assessments mis a jour
- [ ] Sync diplomas relance
"@
    Set-Content -Path $waveFile -Value $waveContent -Encoding utf8
    $waveIndexRows += "| $wave | $($rows.Count) | ./$wave.md |"
}

$waveIndex = @"
# Waves - Index

## Execution order

1. wave-01-security-compliance
2. wave-02-architecture-quality
3. wave-03-product-delivery
4. wave-04-platform-ops

## Waves

| Wave | Certifications | Link |
|---|---:|---|
$($waveIndexRows -join "`n")
"@
Set-Content -Path (Join-Path $wavesRoot "INDEX.md") -Value $waveIndex -Encoding utf8

# Agent execution files (override to include progress columns)
foreach ($agent in ($agentMap.Keys | Sort-Object)) {
    $rows = $agentMap[$agent] | Sort-Object cert_name
    $tableRows = @()
    foreach ($r in $rows) {
        $tableRows += "| $($r.cert_name) | $($r.cert_id) | $($r.wave) | ../../$($r.proof) | $($r.run) | IN_PROGRESS |"
    }

    $agentPlan = @"
# Plan Agent - $agent

## Capstones execution

| Certification | Cert ID | Wave | Evidence | Runbook | Status |
|---|---|---|---|---|---|
$($tableRows -join "`n")

## Gate agent

- [ ] Tous les capstones executes
- [ ] Assessments verifies
- [ ] Sync diplomas valide
"@
    Set-Content -Path (Join-Path $agentsRoot "$agent.md") -Value $agentPlan -Encoding utf8
}

# Update sequence traces and metrics
$p3TracePath = Join-Path $sequenceRoot "phases/p3-trace.md"
$traceContent = @"
# P3 Trace

- 2026-03-05: Capstones rollout initialise.
- 4 vagues d execution creees (security/compliance -> platform/ops).
- 37 dossiers de preuves capstone initialises.
- Runbooks `CAPSTONE-RUN.md` crees pour chaque certification.
"@
Set-Content -Path $p3TracePath -Value $traceContent -Encoding utf8

$metricsPath = Join-Path $sequenceRoot "metrics/2026-03-05-certifications-academy-rollout.json"
$metrics = [ordered]@{
    project = [ordered]@{
        title = "Certifications Academy Rollout"
        description = "Execution capstones des certifications pour tous les agents"
        class = "T3"
        slug = "certifications-academy-rollout"
        mip_sequence_number = 1
        autonomy_mode = "FULL"
    }
    timestamps = [ordered]@{
        sequence_start = "2026-03-05T00:00:00Z"
        p0_start = "2026-03-05T00:00:00Z"
        p0_end = "2026-03-05T00:00:00Z"
        p3_start = "2026-03-05T00:00:00Z"
        p3_end = $null
        p6_start = $null
        p6_end = $null
    }
    counters = [ordered]@{
        certifications_total = $certCount
        waves_total = $waveMap.Keys.Count
        capstones_initialized = $certCount
        evidence_files = (Get-ChildItem -Recurse -File $capstoneRoot | Measure-Object).Count
        runbooks = (Get-ChildItem -Recurse -File ".mip/certifications" -Filter "CAPSTONE-RUN.md" | Measure-Object).Count
    }
}
$metrics | ConvertTo-Json -Depth 6 | Set-Content -Path $metricsPath -Encoding utf8

Write-Output "init-capstones-rollout: done"
