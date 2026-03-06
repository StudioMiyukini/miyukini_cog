param(
    [string]$Profile = "debug"
)

$ErrorActionPreference = "Stop"

$workspaceRoot = Split-Path -Parent $PSScriptRoot
$gameRoot = Join-Path $workspaceRoot "games/sodomight"
$distRoot = Join-Path $workspaceRoot "dist/sodomight"
$binRoot = Join-Path $distRoot "bin"
$dataRoot = Join-Path $distRoot "data"

cargo build -p sodomight

New-Item -ItemType Directory -Force -Path $binRoot | Out-Null
New-Item -ItemType Directory -Force -Path $dataRoot | Out-Null

$binaryPath = Join-Path $workspaceRoot "target/$Profile/sodomight.exe"
Copy-Item $binaryPath (Join-Path $binRoot "sodomight.exe") -Force
Copy-Item (Join-Path $gameRoot "service.manifest.json") (Join-Path $distRoot "service.manifest.json") -Force
Copy-Item (Join-Path $gameRoot "data/bootstrap.ron") (Join-Path $dataRoot "bootstrap.ron") -Force

Write-Host "Package local genere dans $distRoot"
