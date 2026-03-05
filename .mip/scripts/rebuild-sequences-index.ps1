Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Get-PropValue {
    param(
        [Parameter(Mandatory = $false)]$Object,
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $false)]$Default = $null
    )

    $current = $Object
    foreach ($segment in $Path.Split(".")) {
        if ($null -eq $current) { return $Default }

        if ($current -is [System.Collections.IDictionary]) {
            if ($current.Contains($segment)) {
                $current = $current[$segment]
                continue
            }
            return $Default
        }

        $prop = $current.PSObject.Properties[$segment]
        if ($null -eq $prop) { return $Default }
        $current = $prop.Value
    }

    if ($null -eq $current) { return $Default }
    return $current
}

function As-Number {
    param(
        [Parameter(Mandatory = $false)]$Value,
        [Parameter(Mandatory = $false)][double]$Default = 0
    )

    try {
        return [double]$Value
    } catch {
        return $Default
    }
}

$mipRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$sequencesRoot = Join-Path $mipRoot "sequences"
$outPath = Join-Path $sequencesRoot "index.json"

if (-not (Test-Path -LiteralPath $sequencesRoot)) {
    throw "Dossier sequences introuvable: $sequencesRoot"
}

$rows = @()
Get-ChildItem -LiteralPath $sequencesRoot -Directory |
    Where-Object { $_.Name -ne "_template" } |
    Sort-Object Name |
    ForEach-Object {
        $name = $_.Name
        $date = ""
        $slug = $name
        if ($name -match "^(?<d>\d{4}-\d{2}-\d{2})-(?<s>.+)$") {
            $date = $Matches["d"]
            $slug = $Matches["s"]
        }

        $metricsPathFull = $null
        $metricsPath = ""
        $metrics = $null
        $metricsDir = Join-Path $_.FullName "metrics"
        if (Test-Path -LiteralPath $metricsDir) {
            $metricsFile = Get-ChildItem -LiteralPath $metricsDir -File -Filter "*.json" | Sort-Object Name | Select-Object -First 1
            if ($null -ne $metricsFile) {
                $metricsPathFull = $metricsFile.FullName
                $metricsPath = "./sequences/$name/metrics/$($metricsFile.Name)"
                try {
                    $metrics = Get-Content -LiteralPath $metricsPathFull -Raw | ConvertFrom-Json -Depth 32
                } catch {
                    $metrics = $null
                }
            }
        }

        $briefPath = ""
        $briefRaw = ""
        $briefsDir = Join-Path $_.FullName "briefs"
        if (Test-Path -LiteralPath $briefsDir) {
            $briefFiles = Get-ChildItem -LiteralPath $briefsDir -File -Filter "*.md" | Sort-Object Name
            $mainBrief = $briefFiles | Where-Object { $_.Name -notmatch "-P0-travail" } | Select-Object -First 1
            if ($null -eq $mainBrief) {
                $mainBrief = $briefFiles | Select-Object -First 1
            }
            if ($null -ne $mainBrief) {
                $briefPath = "./sequences/$name/briefs/$($mainBrief.Name)"
                try {
                    $briefRaw = Get-Content -LiteralPath $mainBrief.FullName -Raw
                } catch {
                    $briefRaw = ""
                }
            }
        }

        $type = (Get-PropValue -Object $metrics -Path "project.class" -Default "UNKNOWN").ToString().ToUpperInvariant()
        if ($type -notmatch "^T[1-5]$") {
            $type = "UNKNOWN"
        }
        if ($type -eq "UNKNOWN" -and $briefRaw) {
            $m = [regex]::Match($briefRaw, "(?im)(?:Classe|Classification MIP)\s*[:\-]\s*(T[1-5])")
            if ($m.Success) {
                $type = $m.Groups[1].Value.ToUpperInvariant()
            }
        }

        $mscmBlocks = As-Number (Get-PropValue -Object $metrics -Path "mscm.blocks" -Default 0)
        if ($mscmBlocks -eq 0) {
            $mscmBlocks = As-Number (Get-PropValue -Object $metrics -Path "counters.lines_written" -Default 0)
        }

        $mscmFiles = As-Number (Get-PropValue -Object $metrics -Path "mscm.files" -Default 0)
        if ($mscmFiles -eq 0) {
            $created = As-Number (Get-PropValue -Object $metrics -Path "counters.files_created" -Default 0)
            $modified = As-Number (Get-PropValue -Object $metrics -Path "counters.files_modified" -Default 0)
            $mscmFiles = $created + $modified
        }

        $securityScore = As-Number (Get-PropValue -Object $metrics -Path "counters.security_score" -Default 0)
        $sequenceStart = (Get-PropValue -Object $metrics -Path "timestamps.sequence_start" -Default $date).ToString()
        $updated = (Get-PropValue -Object $metrics -Path "timestamps.total_end" -Default "").ToString()
        if ([string]::IsNullOrWhiteSpace($updated)) {
            $updated = (Get-PropValue -Object $metrics -Path "timestamps.p6_end" -Default "").ToString()
        }
        if ([string]::IsNullOrWhiteSpace($updated)) {
            $updated = (Get-PropValue -Object $metrics -Path "timestamps.p5_end" -Default "").ToString()
        }
        if ([string]::IsNullOrWhiteSpace($updated)) {
            $updated = $sequenceStart
        }

        $rows += [ordered]@{
            name = $name
            date = $date
            slug = $slug
            type = $type
            sequence_start = $sequenceStart
            updated = $updated
            mscm_blocks = [math]::Round($mscmBlocks, 0)
            mscm_files = [math]::Round($mscmFiles, 0)
            security_score = [math]::Round($securityScore, 0)
            ui_path = "./sequences/$name/ui/index.html"
            manifest_path = "./sequences/$name/ui/manifest.json"
            metrics_path = $metricsPath
            brief_path = $briefPath
        }
    }

$obj = [ordered]@{
    generated_at = (Get-Date).ToString("s")
    schema = "mip-sequences-index-v2"
    count = $rows.Count
    sequences = $rows
}

$obj | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath $outPath -Encoding UTF8
Write-Host "Sequences index generated: $outPath ($($rows.Count) sequences)"
