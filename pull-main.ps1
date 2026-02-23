# Pull main depuis https://github.com/StudioMiyukini/miyukini-cog
# Lit le token dans .git_token.md

$tokenFile = Join-Path $PSScriptRoot ".git_token.md"
if (-not (Test-Path $tokenFile)) {
    Write-Error "Fichier .git_token.md introuvable"
    exit 1
}

$lines = Get-Content $tokenFile -Encoding UTF8
$token = ""; $repo = ""
foreach ($line in $lines) {
    if ($line -match "Repo\s*:\s*(.+)") { $repo = $Matches[1].Trim() }
    if ($line -match ":\s*(ghp_[a-zA-Z0-9]+)\s*$") { $token = $Matches[1] }
}

if (-not $token -or -not $repo) {
    Write-Error "Format attendu dans .git_token.md : Repo : <url> et Clé : <token>"
    exit 1
}

$authUrl = "https://${token}@github.com/$($repo -replace 'https://github.com/','')"

Set-Location $PSScriptRoot

if (Test-Path ".git") {
    git remote set-url origin $authUrl
    git fetch origin main
    git checkout main -f 2>$null; if (-not $?) { git checkout main }
    git pull origin main
} else {
    git init
    git remote add origin $authUrl
    git fetch origin main
    git checkout -b main origin/main
}

Write-Host "Pull main termine."
