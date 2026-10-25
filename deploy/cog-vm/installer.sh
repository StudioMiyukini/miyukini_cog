#!/usr/bin/env bash
# Installateur des services COG dans la VM jetable de test (idempotent).
#
# A exécuter DANS la VM, en root (sudo), depuis le paquet déposé par pousser.sh :
#   sudo ./installer.sh --vm-jetable --bundle <rép. paquet> [--avec-banc] [--manifeste-sha256 <hex>]
#
# INTERDIT sur un hôte réel (production, serveur personnel, poste de travail) : VM jetable
# cog-essai uniquement (voir README.md). Refus sans --vm-jetable et sur un hôte non virtualisé
# (systemd-detect-virt --vm), comme le banc lot 3.
#
# La logique commune (paquet, comptes, binaires, secrets, PostgreSQL, AppArmor, zone COG,
# unités, démarrage, contrôle de santé) est dans commun.sh, partagée avec
# deploy/recette/installer.sh. Ce script ne fait que les garde-fous propres à la VM jetable.
# Il ne touche ni à /etc/miyukini/douane (banc d'intrusion), ni au réseau hors table COG.
#
# Ré-exécuter ce script met à jour binaires, unités et configuration non secrète, et conserve
# les secrets existants.

set -euo pipefail
umask 0022
LOG_TAG=installer

BUNDLE=""
AVEC_BANC=0
MANIFESTE_ATTENDU=""
VM_JETABLE=0

usage() {
	cat <<'FIN'
Usage : sudo installer.sh --vm-jetable --bundle <répertoire> [--avec-banc] [--manifeste-sha256 <hex>]

  --vm-jetable               obligatoire : confirme la VM jetable cog-essai (jamais un hôte réel)
  --bundle <rép>             paquet déposé par pousser.sh (bin/, config/, static/, banc/, MANIFESTE)
  --avec-banc                installe aussi les outils du banc d'intrusion lot 3
  --manifeste-sha256 <hex>   empreinte du MANIFESTE affichée par pousser.sh (vérifiée)

VM jetable uniquement : refus sans --vm-jetable et sur un hôte non virtualisé.
FIN
}

log() { printf '[installer] %s\n' "$*"; }
die() { printf '[installer] ERREUR: %s\n' "$*" >&2; exit 1; }

while (($#)); do
	case $1 in
		--bundle) BUNDLE=${2:?--bundle : valeur attendue}; shift ;;
		--avec-banc) AVEC_BANC=1 ;;
		--vm-jetable) VM_JETABLE=1 ;;
		--manifeste-sha256) MANIFESTE_ATTENDU=${2:?--manifeste-sha256 : valeur attendue}; shift ;;
		-h | --help) usage; exit 0 ;;
		*) usage >&2; die "option inconnue : $1" ;;
	esac
	shift
done

((EUID == 0)) || die "root requis (sudo)"
# Garde-fou : jamais sur un hôte réel (voir README.md).
((VM_JETABLE)) || die "refus : VM jetable cog-essai uniquement ; confirmez avec --vm-jetable (jamais sur un hôte réel)"
if ! command -v systemd-detect-virt >/dev/null 2>&1 || ! systemd-detect-virt --vm -q; then
	die "refus : hôte non virtualisé (systemd-detect-virt --vm) ; cet installateur ne s'exécute que dans la VM jetable"
fi
[[ -n $BUNDLE ]] || BUNDLE=$(cd "$(dirname "$0")" && pwd)
BUNDLE=$(cd "$BUNDLE" && pwd)

# Logique commune (répertoire de ce script = paquet).
# shellcheck source=commun.sh
. "$(dirname "$0")/commun.sh"
export BUNDLE AVEC_BANC MANIFESTE_ATTENDU

verifier_paquet

for b in "${BINAIRES[@]}"; do
	[[ -x $BUNDLE/bin/$b ]] || die "binaire absent du paquet : bin/$b"
done
[[ -f $BUNDLE/config/bridge.toml ]] || die "config/bridge.toml absent du paquet"
[[ -d $BUNDLE/config/strongfather ]] || die "config/strongfather absent du paquet"
[[ -d $BUNDLE/static/static-dist ]] || die "static/static-dist absent du paquet"

log "bundle : $BUNDLE"
paquets
utilisateurs
repertoires
binaires
verifier_binaires_installes
politiques_et_statiques
environnements
cle_sauvegarde_locale
postgres
apparmor_profils
douane_arretes
douane_cog
unites
demarrage
controle_sante || log "ATTENTION : contrôle de santé en échec (voir ci-dessus)"

log "installation terminée (Douane non installée comme service : voir README.md)"
