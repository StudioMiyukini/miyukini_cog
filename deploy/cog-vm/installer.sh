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
# Paquet : copié d'abord dans /run/miyukini-paquet.XXXXXX (root 0700, sans suivre les liens), puis
# vérifié (aucun lien, liste exacte des fichiers = MANIFESTE, SHA-256) ; seuls les fichiers listés
# au MANIFESTE sont installés, depuis cette copie.
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
SOURCE=$(cd "$BUNDLE" && pwd)

# Copie privée (comme deploy/recette/installer.sh) : refus de tout lien symbolique, tube ou
# périphérique AVANT toute action, puis copie sans suivre les liens dans /run (root, 0700). Seule la
# copie est vérifiée, sourcée et installée ; le dossier déposé (/home/cog/cog-paquet) n'est plus relu.
PRIVE=""
trap 'if [[ -n $PRIVE ]]; then rm -rf "$PRIVE"; fi' EXIT
speciaux=$(cd "$SOURCE" && find . ! -type f ! -type d -printf '%y %p\n' | head -n 5)
[[ -z $speciaux ]] || die "paquet refusé : entrée(s) ni fichier régulier ni dossier (lien symbolique, tube, périphérique) : $(tr '\n' ' ' <<<"$speciaux")"
PRIVE=$(mktemp -d /run/miyukini-paquet.XXXXXX)
chmod 0700 "$PRIVE"
mkdir -m 0700 "$PRIVE/paquet"
cp -R -P --preserve=mode,timestamps -- "$SOURCE/." "$PRIVE/paquet/"
chown -R -h root:root "$PRIVE/paquet"
chmod -R go-w "$PRIVE/paquet"
BUNDLE=$PRIVE/paquet
log "paquet copié dans $BUNDLE (root, 0700) ; $SOURCE n'est plus relu"

# Avant de sourcer commun.sh (outils système seulement) : empreinte du MANIFESTE si elle est donnée,
# puis SHA-256 de commun.sh contre ce MANIFESTE.
[[ -f $BUNDLE/MANIFESTE && ! -L $BUNDLE/MANIFESTE ]] || die "MANIFESTE absent du paquet"
if [[ -n $MANIFESTE_ATTENDU ]]; then
	[[ $(sha256sum "$BUNDLE/MANIFESTE" | cut -d' ' -f1) == "$MANIFESTE_ATTENDU" ]] ||
		die "empreinte du MANIFESTE différente de celle attendue ($MANIFESTE_ATTENDU)"
fi
[[ -f $BUNDLE/commun.sh && ! -L $BUNDLE/commun.sh ]] || die "commun.sh absent du paquet"
(cd "$BUNDLE" && grep -v '^#' MANIFESTE | grep -E '^[0-9a-f]{64}  \./commun\.sh$' | sha256sum --check --strict --quiet -) ||
	die "commun.sh absent du MANIFESTE ou différent"

# Logique commune (copie privée vérifiée).
# shellcheck source=commun.sh
. "$BUNDLE/commun.sh"
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
