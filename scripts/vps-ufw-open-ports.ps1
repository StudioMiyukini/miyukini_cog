# Ouvre les ports MWS sur le VPS Origin (46.202.129.65) via SSH
# Exécuter depuis la racine du repo : .\scripts\vps-ufw-open-ports.ps1
# Si la clé n'est pas au défaut : $env:KEY = "C:\chemin\vers\ssh-key-2026-02-12.key"; .\scripts\vps-ufw-open-ports.ps1

$ErrorActionPreference = "Stop"
$repoRoot = (Get-Item $PSScriptRoot).Parent.FullName
$keyPath = Join-Path $repoRoot "ssh-key-2026-02-12.key"
$scriptPath = Join-Path $PSScriptRoot "vps-ufw-open-ports.sh"

$sshArgs = @("-o", "ConnectTimeout=15", "root@46.202.129.65", "bash -s")
if ($env:KEY) { $sshArgs = @("-i", $env:KEY) + $sshArgs }
elseif (Test-Path $keyPath) { $sshArgs = @("-i", $keyPath) + $sshArgs }

(Get-Content $scriptPath -Raw).Replace("`r`n", "`n") | & ssh @sshArgs
