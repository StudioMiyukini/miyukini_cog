#!/usr/bin/env bash
# build-services.sh — Compile et package les services standalone en .msp
#
# Usage :
#   ./tools/build-service-packages/build-services.sh [service_id...]
#
# Sans arguments : compile et package TOUS les services.
# Avec arguments  : compile et package uniquement les services spécifiés.
#
# Produit : dist/services/{service_id}.msp (archive ZIP prête pour Central)

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
DIST_DIR="$WORKSPACE_ROOT/dist/services"
TARGET_DIR="$WORKSPACE_ROOT/target/release"

# Correspondance service_id → binary_name (nom du binaire cargo)
declare -A SERVICE_BINS=(
    [jayxpose]="jayxpose"
    [jayfestival]="jayfestival"
    [jaykoa]="jaykoa"
    [jaykonta]="jaykonta"
    [miyukiniwatch]="miyukiniwatch"
    [jay1tribu]="jay1tribu"
    [jaymanga]="jaymanga"
    [miyucloud]="miyucloud-server"
)

# Correspondance service_id → dossier du manifeste (crate source)
declare -A SERVICE_CRATE_DIRS=(
    [jayxpose]="crates/jayxpose"
    [jayfestival]="crates/jayfestival"
    [jaykoa]="crates/jaykoa"
    [jaykonta]="crates/jaykonta"
    [miyukiniwatch]="crates/miyukiniwatch"
    [jay1tribu]="crates/jay1tribu"
    [jaymanga]="crates/jaymanga"
    [miyucloud]="crates/miyucloud"
)

# Correspondance service_id → package cargo (pour --bin)
declare -A SERVICE_PACKAGES=(
    [jayxpose]="jayxpose-app"
    [jayfestival]="jayfestival-app"
    [jaykoa]="jaykoa-app"
    [jaykonta]="jaykonta-app"
    [miyukiniwatch]="miyukiniwatch-app"
    [jay1tribu]="jay1tribu-app"
    [jaymanga]="jaymanga-app"
    [miyucloud]="miyucloud-server"
)

# Services à traiter
if [ $# -gt 0 ]; then
    SERVICES=("$@")
else
    SERVICES=("${!SERVICE_BINS[@]}")
fi

echo "=== Build Service Packages ==="
echo "Workspace : $WORKSPACE_ROOT"
echo "Services  : ${SERVICES[*]}"
echo ""

mkdir -p "$DIST_DIR"

for svc_id in "${SERVICES[@]}"; do
    bin_name="${SERVICE_BINS[$svc_id]:-}"
    crate_dir="${SERVICE_CRATE_DIRS[$svc_id]:-}"
    pkg_name="${SERVICE_PACKAGES[$svc_id]:-}"

    if [ -z "$bin_name" ] || [ -z "$crate_dir" ] || [ -z "$pkg_name" ]; then
        echo "ERREUR: Service inconnu '$svc_id'"
        continue
    fi

    manifest_file="$WORKSPACE_ROOT/$crate_dir/service.manifest.json"
    if [ ! -f "$manifest_file" ]; then
        echo "ERREUR: $manifest_file manquant pour '$svc_id'"
        continue
    fi

    echo "--- [$svc_id] Compilation (release) ---"
    cargo build --release --bin "$bin_name" -p "$pkg_name"

    # Extension binaire
    if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "win32" ]]; then
        bin_ext=".exe"
    else
        bin_ext=""
    fi

    binary="$TARGET_DIR/${bin_name}${bin_ext}"
    if [ ! -f "$binary" ]; then
        echo "ERREUR: Binaire non trouvé: $binary"
        continue
    fi

    echo "--- [$svc_id] Packaging .msp ---"
    version="$(sed -n 's/.*"version"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$manifest_file" | head -n1)"
    msp_file="$DIST_DIR/${svc_id}-${version}.msp"

    # Créer un dossier temporaire pour le package
    tmp_dir=$(mktemp -d)
    trap "rm -rf $tmp_dir" EXIT

    # Copier le manifeste
    cp "$manifest_file" "$tmp_dir/service.manifest.json"

    # Copier le binaire dans bin/
    mkdir -p "$tmp_dir/bin"
    cp "$binary" "$tmp_dir/bin/${bin_name}${bin_ext}"

    # Créer l'archive ZIP (.msp)
    (cd "$tmp_dir" && zip -r "$msp_file" . -x ".*")

    size=$(du -h "$msp_file" | cut -f1)
    echo "  -> $msp_file ($size)"
    echo ""
done

echo "=== Packaging terminé ==="
ls -lh "$DIST_DIR"/*.msp 2>/dev/null || echo "(aucun package produit)"
