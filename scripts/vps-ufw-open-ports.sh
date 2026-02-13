#!/bin/bash
# Ouvre les ports MWS sur le VPS Origin (Debian / ufw)
# À exécuter sur le VPS : bash vps-ufw-open-ports.sh
# Ou depuis Windows (PowerShell) :
#   Get-Content scripts\vps-ufw-open-ports.sh -Raw | ssh root@46.202.129.65 "bash -s"

set -e

# Installer ufw si absent (Debian)
if ! command -v ufw &>/dev/null; then
    apt-get update -qq && apt-get install -y ufw
fi

ufw allow 22/tcp   comment 'SSH'
ufw allow 80/tcp   comment 'HTTP'
ufw allow 443/tcp  comment 'HTTPS'
ufw allow 7000/tcp comment 'Origin Relay'
ufw allow 21000/tcp comment 'Origin Tracker'
ufw --force enable
ufw status numbered
printf 'Ports ouverts : 22, 80, 443, 7000, 21000\n'
