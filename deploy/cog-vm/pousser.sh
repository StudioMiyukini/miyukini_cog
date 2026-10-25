#!/usr/bin/env bash
# Exécuté SUR L'HÔTE de développement : prépare un paquet à partir des binaires déjà
# construits (target/release), le copie dans la VM jetable cog-essai, puis lance
# l'installateur DANS la VM. Ne installe rien sur l'hôte.
#
#   deploy/cog-vm/pousser.sh [--sans-installation]
#
#   --sans-installation  copie seulement le paquet (et le banc) dans la VM
#
# Variables : COG_VM (défaut ~/vm/cog-essai), CARGO_TARGET_DIR (défaut <dépôt>/target).
set -euo pipefail

ICI=$(cd "$(dirname "$0")" && pwd)
RACINE=$(cd "$ICI/../.." && pwd)
VM=${COG_VM:-$HOME/vm/cog-essai}
REL=${CARGO_TARGET_DIR:-$RACINE/target}/release
DEST=/home/cog/cog-paquet
SANS_INSTALLATION=0

log() { printf '[pousser] %s\n' "$*"; }
die() { printf '[pousser] ERREUR: %s\n' "$*" >&2; exit 1; }

while (($#)); do
	case $1 in
		--sans-installation) SANS_INSTALLATION=1 ;;
		-h | --help) sed -n '2,11p' "$0"; exit 0 ;;
		*) die "option inconnue : $1" ;;
	esac
	shift
done

[[ -x $VM/ssh-vm.sh ]] || die "accès VM introuvable : $VM/ssh-vm.sh"
SSH_OPTS=(-q -o BatchMode=yes -o StrictHostKeyChecking=no -o "UserKnownHostsFile=$VM/known_hosts" -i "$VM/cle_vm" -P 2222)
ssh_vm() { "$VM/ssh-vm.sh" "$@"; }

# ------------------------------------------------------------------ vérifications locales

BINAIRES=(kindmother-server maia miyucloud-server cog-web-portal miyudouane portail-sas)
for b in "${BINAIRES[@]}"; do
	[[ -x $REL/$b ]] || die "binaire manquant : $REL/$b (cargo build --release -p <paquet>)"
done
[[ -x $REL/examples/arretes_essai ]] || die "exemple manquant : $REL/examples/arretes_essai"

# ------------------------------------------------------------------ paquet (hors dépôt)

STAGE=$(mktemp -d "${TMPDIR:-/tmp}/cog-paquet.XXXXXX")
trap 'rm -rf "$STAGE"' EXIT
mkdir -p "$STAGE/bin" "$STAGE/config" "$STAGE/static" "$STAGE/banc/tests" "$STAGE/banc/cible/examples"

for b in "${BINAIRES[@]}"; do
	cp "$REL/$b" "$STAGE/bin/$b"
done
cp "$RACINE/bridge.toml" "$STAGE/config/bridge.toml"
cp -r "$RACINE/config/strongfather" "$STAGE/config/strongfather"
cp -r "$RACINE/apps/miyucloud/static" "$STAGE/static/static"
cp -r "$RACINE/apps/miyucloud/static-dist" "$STAGE/static/static-dist"
cp "$ICI/installer.sh" "$ICI/tester.sh" "$STAGE/"
cp -r "$RACINE/tests/intrusion-lot3" "$STAGE/banc/tests/intrusion-lot3"
cp "$REL/miyudouane" "$STAGE/banc/cible/miyudouane"
cp "$REL/examples/arretes_essai" "$STAGE/banc/cible/examples/arretes_essai"
chmod 0755 "$STAGE/installer.sh" "$STAGE/tester.sh"
find "$STAGE/banc" -name '*.sh' -exec chmod 0755 {} +

log "paquet préparé : $(du -sh "$STAGE" | cut -f1)"

# ------------------------------------------------------------------ envoi

log "envoi vers la VM : $DEST"
ssh_vm "rm -rf '$DEST' && mkdir -p '$DEST'"
scp -r "${SSH_OPTS[@]}" "$STAGE/." "cog@127.0.0.1:$DEST/"

if ((SANS_INSTALLATION)); then
	log "paquet déposé ; installation ignorée (--sans-installation)"
	exit 0
fi

# ------------------------------------------------------------------ installation dans la VM

log "installation dans la VM"
ssh_vm "sudo bash '$DEST/installer.sh' --bundle '$DEST' --avec-banc"
log "terminé"
