$ErrorActionPreference = "Stop"

$errors = New-Object System.Collections.Generic.List[string]
$warnings = New-Object System.Collections.Generic.List[string]

function Assert-PathExists {
    param(
        [string]$Path,
        [string]$Message
    )
    if (-not (Test-Path $Path)) {
        $errors.Add($Message)
    }
}

function Assert-Contains {
    param(
        [string]$Path,
        [string]$Pattern,
        [string]$Message
    )
    if (-not (Test-Path $Path)) {
        $errors.Add("Fichier introuvable: $Path")
        return
    }
    if (-not (Select-String -Path $Path -Pattern $Pattern -Quiet)) {
        $errors.Add($Message)
    }
}

function Assert-ContainsLiteralLine {
    param(
        [string]$Path,
        [string]$LiteralLine,
        [string]$Message
    )
    if (-not (Test-Path $Path)) {
        $errors.Add("Fichier introuvable: $Path")
        return
    }
    $lines = Get-Content $Path
    if (-not ($lines -contains $LiteralLine)) {
        $errors.Add($Message)
    }
}

$protocolFiles = @(
    ".mip/README.md",
    ".mip/protocol/conventions.md",
    ".mip/modules/workflow.md",
    ".mip/modules/setup.md",
    ".mip/memory/INDEX.md",
    ".mip/memory/SCHEMA.md",
    ".mip/secrets/README.md"
)

foreach ($file in $protocolFiles) {
    Assert-PathExists -Path $file -Message "Fichier protocole manquant: $file"
}

$requiredMemoryFiles = @(
    ".mip/memory/project-file-map.md",
    ".mip/memory/stack-patterns.md",
    ".mip/memory/stack-cheatsheet.md",
    ".mip/memory/api-contracts.md",
    ".mip/memory/test-templates.md",
    ".mip/memory/code-annotations-templates.md",
    ".mip/memory/security-patterns.md",
    ".mip/memory/mip-decisions.md",
    ".mip/memory/mip-performance-history.md",
    ".mip/memory/team-skills-audit.md",
    ".mip/memory/mip-lessons.md",
    ".mip/memory/agent-tuning.md"
)

foreach ($file in $requiredMemoryFiles) {
    Assert-PathExists -Path $file -Message "Fichier baseline memoire manquant: $file"
}

Assert-ContainsLiteralLine -Path ".gitignore" -LiteralLine ".mip/secrets/*" -Message "Regle .gitignore manquante: .mip/secrets/*"
Assert-ContainsLiteralLine -Path ".gitignore" -LiteralLine "!.mip/secrets/README.md" -Message "Regle .gitignore manquante: !.mip/secrets/README.md"

Assert-Contains -Path ".mip/modules/setup.md" -Pattern "ping -n 1 github.com" -Message "SETUP-1: variante Windows du test reseau manquante."
Assert-Contains -Path ".mip/modules/setup.md" -Pattern "Get-CimInstance Win32_Processor" -Message "SETUP-1: detection CPU Windows manquante."
Assert-Contains -Path ".mip/modules/setup.md" -Pattern "Politique minimale secrets" -Message "SETUP: politique minimale secrets manquante."

$p2Hits = Select-String -Path ".mip/README.md", ".mip/protocol/conventions.md", ".mip/modules/workflow.md" -Pattern "\bP2\b" -ErrorAction SilentlyContinue
foreach ($hit in $p2Hits) {
    if ($hit.Line -match "ancien P2|phase P2 distincte") {
        continue
    }
    $errors.Add("Reference P2 non autorisee: $($hit.Path):$($hit.LineNumber)")
}

if (Select-String -Path ".mip/modules/workflow.md" -Pattern "Artefacts de sequence modulaires, 400 lignes max" -Quiet) {
    # ok
} else {
    $warnings.Add("Workflow: clarification I-14 (artefacts de sequence uniquement) non detectee.")
}

Write-Output "=== Lint MIP coherence ==="
if ($warnings.Count -gt 0) {
    Write-Output "Warnings:"
    foreach ($w in $warnings) { Write-Output " - $w" }
}

if ($errors.Count -gt 0) {
    Write-Output "Errors:"
    foreach ($e in $errors) { Write-Output " - $e" }
    exit 1
}

Write-Output "OK: aucune erreur bloquante detectee."
exit 0
