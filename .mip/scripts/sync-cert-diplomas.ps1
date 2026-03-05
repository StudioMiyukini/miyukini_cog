$ErrorActionPreference = "Stop"

$registry = Get-Content -Raw ".mip/certifications/registry.json" | ConvertFrom-Json
$certProps = $registry.certs.PSObject.Properties
$diplomaDir = ".mip/certifications/diplomas"
New-Item -ItemType Directory -Path $diplomaDir -Force | Out-Null

$byAgent = @{}
$statusSnapshot = @{}

foreach ($prop in $certProps) {
    $certId = $prop.Name
    $meta = $prop.Value
    $agent = $meta.agent
    $folder = $meta.folder
    $name = $meta.name

    if (-not $byAgent.ContainsKey($agent)) {
        $byAgent[$agent] = @()
    }

    $assessmentPath = ".mip/certifications/$folder/academy/ASSESSMENT.md"
    $status = "FORMATION_IMPORTED"

    if (Test-Path $assessmentPath) {
        $assessment = Get-Content -Raw $assessmentPath
        if ($assessment -match "(?mi)^- \[x\] READY_EXTERNE") {
            $status = "READY_EXTERNE"
        }
        elseif ($assessment -match "(?mi)^- \[x\] READY_INTERNE") {
            $status = "READY_INTERNE"
        }
        elseif ($assessment -match "(?mi)^- \[x\] NOT_READY") {
            $status = "NOT_READY"
        }
    }
    else {
        $status = "NO_ASSESSMENT"
    }

    $entry = [PSCustomObject]@{
        cert_id   = $certId
        cert_name = $name
        folder    = $folder
        status    = $status
        parcours  = "../$folder/academy/INDEX.md"
        gate      = "../$folder/academy/ASSESSMENT.md"
    }

    $byAgent[$agent] += $entry
    $statusSnapshot[$certId] = $status
}

$indexLinks = @()
$summaryRows = @()

foreach ($agent in ($byAgent.Keys | Sort-Object)) {
    $rows = $byAgent[$agent] | Sort-Object cert_name

    $readyInterne = ($rows | Where-Object { $_.status -eq "READY_INTERNE" }).Count
    $readyExterne = ($rows | Where-Object { $_.status -eq "READY_EXTERNE" }).Count
    $imported = ($rows | Where-Object { $_.status -eq "FORMATION_IMPORTED" }).Count
    $notReady = ($rows | Where-Object { $_.status -eq "NOT_READY" }).Count
    $total = $rows.Count

    $indexLinks += "- [$agent](./$agent.md)"
    $summaryRows += "| $agent | $total | $readyInterne | $readyExterne | $imported | $notReady |"

    $tableRows = @()
    foreach ($row in $rows) {
        $tableRows += "| $($row.cert_name) | $($row.cert_id) | $($row.status) | $($row.parcours) | $($row.gate) |"
    }

    $passport = @"
# Passeport diplome - $agent

> Statut: diplome simulation interne. Le statut est synchronise depuis les fichiers `academy/ASSESSMENT.md`.

| Certification | Cert ID | Statut | Parcours | Gate |
|---|---|---|---|---|
$($tableRows -join "`n")

## Resume

- Total: $total
- READY_INTERNE: $readyInterne
- READY_EXTERNE: $readyExterne
- FORMATION_IMPORTED: $imported
- NOT_READY: $notReady

## Regle d usage

- Ce passeport vaut competence operationnelle interne MIP.
- Il ne remplace pas une certification externe delivree par un organisme.
"@

    Set-Content -Path "$diplomaDir/$agent.md" -Value $passport -Encoding utf8
}

$index = @"
# Diplomas - Index

> Passeports de formation locale par agent (mode diplome simulation). Synchronisation: assessment -> passeport.

## Agents

$($indexLinks -join "`n")

## Tableau global

| Agent | Total certs | READY_INTERNE | READY_EXTERNE | FORMATION_IMPORTED | NOT_READY |
|---|---:|---:|---:|---:|---:|
$($summaryRows -join "`n")
"@

Set-Content -Path "$diplomaDir/INDEX.md" -Value $index -Encoding utf8

$snapshot = [ordered]@{
    generated_at = "2026-03-05"
    mode = "assessment_sync"
    agents = [ordered]@{}
    cert_status = $statusSnapshot
}

foreach ($agent in ($byAgent.Keys | Sort-Object)) {
    $rows = $byAgent[$agent]
    $snapshot.agents[$agent] = [ordered]@{
        total = $rows.Count
        ready_interne = ($rows | Where-Object { $_.status -eq "READY_INTERNE" }).Count
        ready_externe = ($rows | Where-Object { $_.status -eq "READY_EXTERNE" }).Count
        formation_imported = ($rows | Where-Object { $_.status -eq "FORMATION_IMPORTED" }).Count
        not_ready = ($rows | Where-Object { $_.status -eq "NOT_READY" }).Count
    }
}

$snapshot | ConvertTo-Json -Depth 6 | Set-Content -Path "$diplomaDir/STATUS.json" -Encoding utf8

Write-Output "sync-cert-diplomas: done"
