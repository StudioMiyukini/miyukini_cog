#!/usr/bin/env bash
set -euo pipefail

SERVICE_NAME="${SERVICE_NAME:-miyucloud}"
INSTALL_ROOT="${INSTALL_ROOT:-/opt/miyucloud}"
RELEASES_DIR="${RELEASES_DIR:-$INSTALL_ROOT/releases}"
CURRENT_LINK="${CURRENT_LINK:-$INSTALL_ROOT/current}"
BIN_NAME="${BIN_NAME:-miyucloud-server}"
HEALTHCHECK_SCRIPT="${HEALTHCHECK_SCRIPT:-$INSTALL_ROOT/scripts/healthcheck.sh}"

timestamp="$(date +%Y%m%d-%H%M%S)"
release_dir="$RELEASES_DIR/$timestamp"

echo "[deploy] build release binary"
cargo build -p miyucloud-server --release

echo "[deploy] prepare release directory: $release_dir"
mkdir -p "$release_dir"
cp "target/release/$BIN_NAME" "$release_dir/$BIN_NAME"
chmod 0755 "$release_dir/$BIN_NAME"

previous_target=""
if [ -L "$CURRENT_LINK" ]; then
  previous_target="$(readlink -f "$CURRENT_LINK" || true)"
fi

echo "[deploy] switch current symlink"
ln -sfn "$release_dir" "$CURRENT_LINK"

echo "[deploy] restart systemd service: $SERVICE_NAME"
sudo systemctl daemon-reload
sudo systemctl restart "$SERVICE_NAME"

echo "[deploy] healthcheck"
if ! "$HEALTHCHECK_SCRIPT"; then
  echo "[deploy] healthcheck failed, rollback in progress"
  if [ -n "$previous_target" ] && [ -d "$previous_target" ]; then
    ln -sfn "$previous_target" "$CURRENT_LINK"
    sudo systemctl restart "$SERVICE_NAME"
  fi
  exit 1
fi

echo "[deploy] success"
