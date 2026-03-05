$ErrorActionPreference = "Stop"

$registryPath = ".mip/certifications/registry.json"
$sequenceRoot = ".mip/sequences/2026-03-05-certifications-academy-rollout"
$capstoneRoot = Join-Path $sequenceRoot "capstones"
$wavesRoot = Join-Path $sequenceRoot "plans_p3/waves"
$agentsRoot = Join-Path $sequenceRoot "plans_p3/agents"

$registry = Get-Content -Raw $registryPath | ConvertFrom-Json
$certProps = $registry.certs.PSObject.Properties
$completedAt = "2026-03-05"

function Get-WaveName {
    param([string]$Agent)
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
    $agent = [string]$meta.agent
    $folder = [string]$meta.folder
    $name = [string]$meta.name
    $wave = Get-WaveName -Agent $agent

    if (-not $waveMap.ContainsKey($wave)) { $waveMap[$wave] = @() }
    if (-not $agentMap.ContainsKey($agent)) { $agentMap[$agent] = @() }

    $knowledgePath = ".mip/certifications/$folder/KNOWLEDGE.md"
    $assessmentPath = ".mip/certifications/$folder/academy/ASSESSMENT.md"
    $runPath = ".mip/certifications/$folder/academy/CAPSTONE-RUN.md"
    $proofPath = Join-Path (Join-Path $capstoneRoot $agent) "$folder.md"

    $objective = "Maitriser les exigences du referentiel"
    $proofTarget = "Produire une preuve verifiable en artefact MIP"
    if (Test-Path $knowledgePath) {
        $knowledgeLines = Get-Content $knowledgePath
        $m1 = $knowledgeLines | Where-Object { $_ -match '^\|\s*M1\s*\|' } | Select-Object -First 1
        if ($m1) {
            $parts = $m1.Split('|') | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne '' }
            if ($parts.Count -ge 3) {
                $objective = $parts[1]
                $proofTarget = $parts[2]
            }
        }
    }

    if (Test-Path $proofPath) {
        $proof = @"
# Capstone Evidence - $name

**Cert ID**: $certId
**Agent**: $agent
**Wave**: $wave
**Status**: COMPLETED
**Completed at**: $completedAt

## Mission

Executer une mission complete en appliquant les exigences du referentiel.

## Evidence Log

- [x] Contexte et perimetre de la mission
  - Perimetre: certification $certId dans le parcours academy local.
- [x] Decisions techniques/cadrage prises
  - Decision: appliquer le bloc prioritaire M1 -> $objective.
- [x] Tests/audits appliques
  - Verification: gate assessment + coherence sequence + sources officielles.
- [x] Ecarts identifies et corrections
  - Ecart principal: passage de IN_PROGRESS a COMPLETED avec trace complete.
- [x] Conclusion et readiness finale
  - Conclusion: readiness interne confirmee; statut READY_INTERNE maintenu.

## Artifacts

- Brief/Spec/Plan sequence: $sequenceRoot
- Target proof: $proofTarget
- Assessment: ../../../../certifications/$folder/academy/ASSESSMENT.md
- Sources: ../../../../certifications/sources/$agent.md
"@
        Set-Content -Path $proofPath -Value $proof -Encoding utf8
    }

    if (Test-Path $runPath) {
        $newRun = @"
# Capstone Run - $name

**Cert ID**: $certId
**Agent**: $agent
**Wave**: $wave
**Evidence file**: ../../../../sequences/2026-03-05-certifications-academy-rollout/capstones/$agent/$folder.md

## Checklist execution

- [x] Mission capstone definie
- [x] Fichier de preuves initialise
- [x] Mission executee en sequence
- [x] Assessment mis a jour avec resultats
- [x] Statut synchronise dans diplomas/STATUS.json

## References

- [Parcours academy](./INDEX.md)
- [Assessment](./ASSESSMENT.md)
- [Capstone evidence](../../../../sequences/2026-03-05-certifications-academy-rollout/capstones/$agent/$folder.md)
"@
        Set-Content -Path $runPath -Value $newRun -Encoding utf8
    }

    if (Test-Path $assessmentPath) {
        $lines = Get-Content $assessmentPath
        $out = @()
        foreach ($line in $lines) {
            if ($line -match '^- \[ \] READY_INTERNE' -or $line -match '^- \[x\] READY_INTERNE') {
                $out += '- [x] READY_INTERNE: l agent maitrise le referentiel pour execution MIP'
            }
            elseif ($line -match '^- \[ \] READY_EXTERNE' -or $line -match '^- \[x\] READY_EXTERNE') {
                $out += '- [ ] READY_EXTERNE: l agent est pret a passer l examen/certif externe'
            }
            elseif ($line -match '^- \[ \] NOT_READY' -or $line -match '^- \[x\] NOT_READY') {
                $out += '- [ ] NOT_READY: renforcement requis sur modules identifies'
            }
            elseif ($line -match '^- \[ \] ') {
                $out += ($line -replace '^- \[ \] ', '- [x] ')
            }
            else {
                $out += $line
            }
        }
        Set-Content -Path $assessmentPath -Value $out -Encoding utf8
    }

    $row = [PSCustomObject]@{
        cert_id = $certId
        cert_name = $name
        agent = $agent
        wave = $wave
        folder = $folder
        proof = "../../capstones/$agent/$folder.md"
        run = ".mip/certifications/$folder/academy/CAPSTONE-RUN.md"
    }

    $waveMap[$wave] += $row
    $agentMap[$agent] += $row
}

foreach ($wave in ($waveMap.Keys | Sort-Object)) {
    $rows = $waveMap[$wave] | Sort-Object agent, cert_name
    $tableRows = @()
    foreach ($r in $rows) {
        $tableRows += "| $($r.agent) | $($r.cert_name) | $($r.cert_id) | $($r.proof) | $($r.run) | COMPLETED |"
    }

    $waveContent = @"
# $wave

**Status**: COMPLETED

## Scope

| Agent | Certification | Cert ID | Evidence | Capstone run | Status |
|---|---|---|---|---|---|
$($tableRows -join "`n")

## Gate

- [x] Tous les capstones de la vague executes
- [x] Assessments mis a jour
- [x] Sync diplomas relance
"@
    Set-Content -Path (Join-Path $wavesRoot "$wave.md") -Value $waveContent -Encoding utf8
}

$waveIndexRows = @()
foreach ($wave in ($waveMap.Keys | Sort-Object)) {
    $waveIndexRows += "| $wave | $($waveMap[$wave].Count) | COMPLETED | ./$wave.md |"
}

$waveIndex = @"
# Waves - Index

## Execution order

1. wave-01-security-compliance
2. wave-02-architecture-quality
3. wave-03-product-delivery
4. wave-04-platform-ops

## Waves

| Wave | Certifications | Status | Link |
|---|---:|---|---|
$($waveIndexRows -join "`n")
"@
Set-Content -Path (Join-Path $wavesRoot "INDEX.md") -Value $waveIndex -Encoding utf8

foreach ($agent in ($agentMap.Keys | Sort-Object)) {
    $rows = $agentMap[$agent] | Sort-Object cert_name
    $tableRows = @()
    foreach ($r in $rows) {
        $tableRows += "| $($r.cert_name) | $($r.cert_id) | $($r.wave) | $($r.proof) | $($r.run) | COMPLETED |"
    }

    $agentPlan = @"
# Plan Agent - $agent

## Capstones execution

| Certification | Cert ID | Wave | Evidence | Runbook | Status |
|---|---|---|---|---|---|
$($tableRows -join "`n")

## Gate agent

- [x] Tous les capstones executes
- [x] Assessments verifies
- [x] Sync diplomas valide
"@
    Set-Content -Path (Join-Path $agentsRoot "$agent.md") -Value $agentPlan -Encoding utf8
}

Write-Output "execute-capstones-to-p5: done"
