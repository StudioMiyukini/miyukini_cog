#!/bin/bash
# Déploiement de miyukini-origin sur le VPS Hostinger.
# À exécuter SUR le VPS (root@46.202.129.65), pas depuis la machine locale.
#
# Usage :
#   curl -fsSL https://raw.githubusercontent.com/StudioMiyukini/miyukini_cog/main/scripts/deploy-origin.sh | bash
# OU :
#   cd /root/miyukini-cog && git pull && bash scripts/deploy-origin.sh
set -euo pipefail

REPO_DIR="${REPO_DIR:-/root/miyukini-cog}"
BIN_DST="${BIN_DST:-/usr/local/bin/miyukini-origin}"
SERVICE_NAME="${SERVICE_NAME:-miyukini-origin}"

echo "=== Deploy miyukini-origin ==="
echo "Repo  : $REPO_DIR"
echo "Bin   : $BIN_DST"
echo "Service : $SERVICE_NAME"
echo

# 1. Mettre à jour les sources
if [ ! -d "$REPO_DIR/.git" ]; then
    echo "ERREUR : $REPO_DIR n'est pas un dépôt git. Cloner d'abord :"
    echo "    git clone https://github.com/StudioMiyukini/miyukini_cog $REPO_DIR"
    exit 1
fi

echo "[1/4] git pull..."
cd "$REPO_DIR"
git fetch origin main
git reset --hard origin/main

# 2. Build release
echo "[2/4] cargo build --release -p miyukini-origin..."
# shellcheck disable=SC1091
source "$HOME/.cargo/env" 2>/dev/null || true
export PATH="$HOME/.cargo/bin:$PATH"
cargo build --release -p miyukini-origin

# 3. Installer le binaire
echo "[3/4] Copie du binaire..."
install -m 0755 "$REPO_DIR/target/release/miyukini-origin" "$BIN_DST"

# 4. Redémarrer le service
echo "[4/4] systemctl restart $SERVICE_NAME..."
systemctl daemon-reload
systemctl restart "$SERVICE_NAME"
sleep 2
systemctl status "$SERVICE_NAME" --no-pager | head -10

echo
echo "=== Déploiement terminé ==="
echo "Vérifier : curl -k https://localhost:8080/pricing | head -5"
