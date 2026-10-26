#!/usr/bin/env bash
# Installateur COG pour la MACHINE DE RECETTE (physique ou VM dédiée, Ubuntu 26.04, root).
#
#   sudo ./installer.sh --confirme-machine-de-recette --cle-publique-pem FICHIER.pem
#   sudo ./installer.sh --confirme-machine-de-recette --empreinte-cle-pem <sha256 du PEM publié>
#   … [--modele <artefact.mmia>]   mini modèle (paquet modèle séparé, A-026), installé par le canal
#        de maia-llm : signature contre l'ancre épinglée dans commun.sh (paquet signé), empreinte
#        SHA-256 du modèle comparée à celle consignée ; absent : modèle déjà installé conservé
#
# Exécuté depuis le paquet cog-<version>-<commit>.tar.zst extrait. Ordre strict :
#   1. refus si la machine ressemble à une production (heuristique ci-dessous) ;
#   2. COPIE PRIVÉE : le paquet entier est copié (sans suivre aucun lien ; un lien symbolique, tube
#      ou périphérique dans le paquet est refusé avant toute action) dans /run/miyukini-paquet.XXXXXX,
#      root, 0700. Le dossier d'origine n'est plus jamais relu : le modifier ensuite n'a aucun effet ;
#   3. AUTHENTIFICATION de la copie, par des outils SYSTÈME seulement (openssl, sha256sum) : la clé
#      publique PEM fournie par l'opérateur (ou celle de la copie SI son SHA-256 égale l'empreinte
#      donnée par l'opérateur) vérifie la signature détachée Ed25519 du MANIFESTE ; puis liste exacte
#      des fichiers (aucun lien, aucun fichier hors manifeste) et SHA-256 de chacun ;
#   4. seulement alors, depuis la copie : bin/outil-sceau (sceau), commun.sh et l'installation
#      (fichiers listés au MANIFESTE uniquement) ; la copie est effacée à la sortie ;
#   5. amorçage : instructions. Aucun secret affiché, aucun mot de passe écrit par ce script.
# Le corps du script est un bloc { … } : bash le lit en entier avant de l'exécuter (une
# réécriture de installer.sh pendant l'exécution est sans effet).
#
# Heuristique « machine de production » (refus si l'une est vraie) :
#   - unité active parmi : cloudflared*, docker, containerd, nginx, apache2, php*-fpm,
#     pm2-*, immich*, nextcloud*, miyucloud (unité de l'ancien déploiement hôte) ;
#   - répertoire présent : /etc/cloudflared, /opt/miyucloud, /var/www/nextcloud ;
#   - --confirme-machine-de-recette absent.
# Une installation COG précédente (/opt/miyukini) n'est pas un signe de production.
{
set -euo pipefail
umask 0022
LOG_TAG=recette

BUNDLE=""
PEM_OPTION=""
EMPREINTE_PEM=""
CONFIRME=0
MODELE_ARTEFACT=""
AUTH_DIR=""
PRIVE=""
trap 'if [[ -n $AUTH_DIR ]]; then rm -rf "$AUTH_DIR"; fi; if [[ -n $PRIVE ]]; then rm -rf "$PRIVE"; fi' EXIT

log() { printf '[recette] %s\n' "$*"; }
die() { printf '[recette] ERREUR: %s\n' "$*" >&2; exit 1; }

usage() {
	sed -n '2,12p' "$0" | sed 's/^# \{0,1\}//'
}

while (($#)); do
	case $1 in
		--bundle) BUNDLE=${2:?--bundle : valeur attendue}; shift ;;
		--cle-publique-pem) PEM_OPTION=${2:?--cle-publique-pem : valeur attendue}; shift ;;
		--empreinte-cle-pem) EMPREINTE_PEM=${2:?--empreinte-cle-pem : valeur attendue}; shift ;;
		--confirme-machine-de-recette) CONFIRME=1 ;;
		--modele) MODELE_ARTEFACT=${2:?--modele : fichier attendu}; shift ;;
		-h | --help) usage; exit 0 ;;
		*) usage >&2; die "option inconnue : $1" ;;
	esac
	shift
done

((EUID == 0)) || die "root requis (sudo)"
((CONFIRME)) || die "refus : cette commande s'exécute sur la machine de recette ; confirmez avec --confirme-machine-de-recette"

# ------------------------------------------------------------------ 1. heuristique de production

verifier_machine_de_recette() {
	local u trouve="" rep
	for u in cloudflared cloudflared@ docker containerd nginx apache2 pm2-miyukini immich-server nextcloud miyucloud; do
		if systemctl is-active --quiet "$u" 2>/dev/null; then trouve+="unité $u active ; "; fi
	done
	while read -r u _; do
		case $u in
			php*-fpm.service | pm2-*.service | immich*.service | nextcloud*.service | cloudflared*.service)
				trouve+="unité $u active ; " ;;
		esac
	done < <(systemctl list-units --type=service --state=active --no-legend --plain 2>/dev/null || true)
	for rep in /etc/cloudflared /opt/miyucloud /var/www/nextcloud; do
		if [[ -e $rep ]]; then trouve+="$rep présent ; "; fi
	done
	if [[ -n $trouve ]]; then
		die "la machine ressemble à une production ($trouve) : refus. L'installateur de recette ne s'exécute que sur une machine dédiée."
	fi
	log "machine : aucun service de production reconnu"
}

verifier_machine_de_recette

# ------------------------------------------------------------------ 2. copie privée (avant tout)

# copier_paquet_prive <dossier source> : refuse toute entrée qui n'est ni fichier régulier ni
# dossier, puis copie le paquet SANS SUIVRE LES LIENS (cp -P) dans PRIVE=/run/miyukini-paquet.XXXXXX
# (root, 0700) ; les fichiers copiés appartiennent à root et ne sont modifiables par aucun autre
# compte. La copie est recontrôlée (types, liste exacte, SHA-256) par authentifier_paquet : un
# changement de la source pendant la copie est donc soit absent de la copie, soit refusé.
copier_paquet_prive() {
	local src=$1 speciaux
	speciaux=$(cd "$src" && find . ! -type f ! -type d -printf '%y %p\n' | head -n 5) || return 1
	if [[ -n $speciaux ]]; then
		echo "[recette] ERREUR: paquet refusé : entrée(s) ni fichier régulier ni dossier (lien symbolique, tube, périphérique) : $(tr '\n' ' ' <<<"$speciaux")" >&2
		return 1
	fi
	PRIVE=$(mktemp -d /run/miyukini-paquet.XXXXXX) || return 1
	chmod 0700 "$PRIVE"
	mkdir -m 0700 "$PRIVE/paquet"
	cp -R -P --preserve=mode,timestamps -- "$src/." "$PRIVE/paquet/" || return 1
	chown -R -h root:root "$PRIVE/paquet"
	chmod -R go-w "$PRIVE/paquet"
}

# authentifier_paquet <copie privée du paquet> <clé publique PEM de confiance>
# Outils système seulement (openssl, sha256sum, find) ; rien du paquet n'est exécuté ni sourcé.
#   a. la copie appartient à root et n'est modifiable par aucun autre compte ;
#   b. aucune entrée qui ne soit ni fichier régulier ni dossier (lien symbolique compris) ;
#   c. MANIFESTE, MANIFESTE.sig et la clé copiés dans AUTH_DIR (root 0700), signature détachée
#      Ed25519 vérifiée par openssl ;
#   d. LISTE EXACTE triée des fichiers présents (hors ./MANIFESTE, ./MANIFESTE.sig, ./MANIFESTE.sceau
#      à la racine) = celle du MANIFESTE signé, lignes « <sha256>  ./<chemin> » sans doublon ;
#   e. SHA-256 de chaque fichier contre le MANIFESTE signé.
# Code 1 au moindre écart. (Fonction exportée vers /opt/miyukini/exploitation/authentification.sh
# par la même installation : mettre-a-jour.sh la réutilise.)
authentifier_paquet() {
	local bundle=$1 pem=$2 lignes presents speciaux mal
	if [[ ! -d $bundle || -L $bundle ]] || [[ -n $(find "$bundle" \( ! -user root -o -perm /022 \) -print -quit) ]]; then
		echo "[recette] ERREUR: paquet refusé : $bundle n'est pas une copie privée (root, non modifiable par un autre compte)" >&2
		return 1
	fi
	speciaux=$(cd "$bundle" && find . ! -type f ! -type d -printf '%y %p\n' | head -n 5)
	if [[ -n $speciaux ]]; then
		echo "[recette] ERREUR: paquet refusé : entrée(s) ni fichier régulier ni dossier (lien symbolique, tube, périphérique) : $(tr '\n' ' ' <<<"$speciaux")" >&2
		return 1
	fi
	AUTH_DIR=$(mktemp -d /run/miyukini-auth.XXXXXX) || return 1
	chmod 0700 "$AUTH_DIR"
	cp "$bundle/MANIFESTE" "$bundle/MANIFESTE.sig" "$AUTH_DIR/" || return 1
	cp "$pem" "$AUTH_DIR/cle.pem" || return 1
	if ! openssl pkeyutl -verify -pubin -inkey "$AUTH_DIR/cle.pem" -rawin \
		-in "$AUTH_DIR/MANIFESTE" -sigfile "$AUTH_DIR/MANIFESTE.sig" >/dev/null 2>&1; then
		echo "[recette] ERREUR: signature du MANIFESTE refusée (clé d'éditeur attendue non reconnue, ou paquet altéré)" >&2
		return 1
	fi
	printf '[recette] signature Ed25519 du MANIFESTE vérifiée (openssl)\n'
	mal=$(grep -v '^#' "$AUTH_DIR/MANIFESTE" | grep -cvE '^[0-9a-f]{64}  \./[^[:cntrl:]]+$' || true)
	if ((mal > 0)); then
		echo "[recette] ERREUR: paquet refusé : $mal ligne(s) mal formée(s) dans le MANIFESTE signé" >&2
		return 1
	fi
	lignes=$(grep -v '^#' "$AUTH_DIR/MANIFESTE" | sed -n 's/^[0-9a-f]\{64\}  //p' | LC_ALL=C sort)
	if [[ -z $lignes || $lignes != "$(LC_ALL=C sort -u <<<"$lignes")" ]]; then
		echo "[recette] ERREUR: paquet refusé : MANIFESTE signé vide ou chemin en double" >&2
		return 1
	fi
	presents=$(cd "$bundle" && find . -type f ! -path ./MANIFESTE ! -path ./MANIFESTE.sig ! -path ./MANIFESTE.sceau | LC_ALL=C sort)
	if [[ $lignes != "$presents" ]]; then
		echo "[recette] ERREUR: paquet refusé : liste des fichiers différente du MANIFESTE signé ($(grep -c . <<<"$presents") présents, $(grep -c . <<<"$lignes") listés) : $(diff <(printf '%s\n' "$lignes") <(printf '%s\n' "$presents") | sed -n 's/^< /absent /p; s/^> /hors manifeste /p' | head -n 5 | tr '\n' ' ')" >&2
		return 1
	fi
	(cd "$bundle" && grep -v '^#' "$AUTH_DIR/MANIFESTE" | sha256sum --check --strict --quiet -) || {
		printf '[recette] ERREUR: SHA-256 des fichiers différents du MANIFESTE signé\n' >&2
		return 1
	}
	printf '[recette] %s fichiers conformes au manifeste signé (liste exacte, aucun lien)\n' "$(grep -c . <<<"$lignes")"
}

[[ -n $BUNDLE ]] || BUNDLE=$(cd "$(dirname "$0")" && pwd)
SOURCE=$(cd "$BUNDLE" && pwd)
[[ -f $SOURCE/MANIFESTE ]] || die "élément du paquet absent : MANIFESTE"
copier_paquet_prive "$SOURCE" || die "copie privée du paquet refusée : rien n'a été exécuté ni installé"
BUNDLE=$PRIVE/paquet
log "paquet copié dans $BUNDLE (root, 0700) ; $SOURCE n'est plus relu"
if [[ -n $(find "$SOURCE" -perm /022 -print -quit) ]]; then
	log "avertissement : $SOURCE est modifiable par un autre compte (groupe ou autres) ; seule la copie privée est utilisée"
fi
for f in MANIFESTE MANIFESTE.sig MANIFESTE.sceau cle-publique-editeur.pem cle-publique-editeur.hex commun.sh bin/outil-sceau; do
	[[ -f $BUNDLE/$f && ! -L $BUNDLE/$f ]] || die "élément du paquet absent : $f"
done

# ------------------------------------------------------------------ 3. authentification

# Clé de confiance : fournie par l'opérateur (recommandé), ou celle de la copie privée si son
# empreinte (sha256sum, outil système) égale celle donnée par l'opérateur.
if [[ -n $PEM_OPTION ]]; then
	[[ -f $PEM_OPTION ]] || die "clé PEM introuvable : $PEM_OPTION"
	PEM_CONFIANCE=$PEM_OPTION
elif [[ -n $EMPREINTE_PEM ]]; then
	[[ $EMPREINTE_PEM =~ ^[0-9a-f]{64}$ ]] || die "--empreinte-cle-pem : 64 caractères hexadécimaux attendus"
	fp=$(sha256sum "$BUNDLE/cle-publique-editeur.pem" | cut -d' ' -f1)
	[[ $fp == "$EMPREINTE_PEM" ]] || die "empreinte de la clé du paquet ($fp) différente de celle fournie : refus ; rien n'a été exécuté ni installé"
	PEM_CONFIANCE=$BUNDLE/cle-publique-editeur.pem
else
	die "clé de confiance requise : --cle-publique-pem FICHIER ou --empreinte-cle-pem SHA256 (publiés dans les notes de version)"
fi

authentifier_paquet "$BUNDLE" "$PEM_CONFIANCE" || die "authentification du paquet refusée : rien n'a été exécuté ni installé"

# ------------------------------------------------------------------ 4. contenu authentifié

# La copie privée est authentifiée (signature openssl, liste exacte, SHA-256) : son outil de sceau,
# désormais couvert par le MANIFESTE signé, et sa logique commune peuvent s'exécuter.
"$BUNDLE/bin/outil-sceau" verifier --cle-publique "$(tr -d '\n' <"$BUNDLE/cle-publique-editeur.hex")" \
	--manifeste "$AUTH_DIR/MANIFESTE" --sceau "$BUNDLE/MANIFESTE.sceau" >/dev/null ||
	die "sceau MANIFESTE.sceau refusé"
log "sceau de l'éditeur vérifié (outil-sceau)"

# shellcheck source=commun.sh
. "$BUNDLE/commun.sh"
export BUNDLE AVEC_BANC=0 MODELE_ARTEFACT
verifier_paquet

for b in "${BINAIRES[@]}"; do
	[[ -x $BUNDLE/bin/$b ]] || die "binaire absent du paquet : bin/$b"
done
[[ -f $BUNDLE/config/bridge.toml ]] || die "config/bridge.toml absent du paquet"
[[ -f $BUNDLE/config/maia/maia-llm.toml ]] || die "config/maia/maia-llm.toml absent du paquet"
[[ -d $BUNDLE/config/strongfather ]] || die "config/strongfather absent du paquet"
[[ -d $BUNDLE/static/static-dist ]] || die "static/static-dist absent du paquet"
[[ ! -e $BUNDLE/bin/miyuracine ]] || die "paquet interdit : bin/miyuracine présent (racine hors ligne)"
log "$(sed -n 's/^version //p' "$BUNDLE/VERSION") ; $(sed -n 's/^arbre //p' "$BUNDLE/VERSION")"

# ------------------------------------------------------------------ 5. installation

log "installation dans $PREFIXE (copie privée : $BUNDLE)"
paquets
utilisateurs
repertoires
binaires
verifier_binaires_installes
politiques_et_statiques
environnements
cle_maia
configurer_maia_llm
modele_ia
cle_sauvegarde_locale
postgres
apparmor_profils
douane_arretes
douane_cog
douane_externe
unites
installer_exploitation
# Exporte la fonction d'authentification pour mettre-a-jour.sh (source unique : ce script).
declare -f authentifier_paquet >"$EXPLOITATION/authentification.sh"
chmod 0644 "$EXPLOITATION/authentification.sh"
demarrage

if ! controle_sante; then
	die "installation terminée mais le contrôle de santé échoue (voir ci-dessus) ; utilisez $EXPLOITATION/etat.sh"
fi

# ------------------------------------------------------------------ 5. amorçage

cat <<FIN

[recette] Installation de COG $(sed -n 's/^version //p' "$BUNDLE/VERSION") terminée.

Amorçage (une fois, depuis votre poste, par tunnel SSH ; rien n'est écrit sur disque) :

  1. Premier administrateur MiyuCloud (refusé une fois le système initialisé) :
       ssh -N -L 8086:127.0.0.1:8086 <utilisateur>@<machine-de-recette>
       # autre terminal, mot de passe saisi au terminal (pas dans l'historique) :
       read -rs MDP && curl -fsS -X POST http://127.0.0.1:8086/api/setup \
         -H 'Content-Type: application/json' \
         -d "{\"username\":\"admin\",\"email\":\"admin@recette.invalide\",\"password\":\"\$MDP\"}"
       # 201 = administrateur créé. Vérifier : curl -fsS http://127.0.0.1:8086/api/auth/status

  2. Clés MAIA (routes mémoire) : désactivées (HTTP 503) tant qu'elles ne sont pas configurées.
     Procédure (docs/recette/NOTES-DE-VERSION-0.9.0-beta.1.md) :
       - déclarer l'utilisateur de mémoire dans /var/lib/miyukini/kindmother/maia-utilisateurs
         (une ligne, mode 600, propriétaire miyukini-kindmother), puis redémarrer miyukini-kindmother ;
       - copier le justificatif opérateur et server.pub vers /var/lib/miyukini/maia/kindmother-credentials ;
       - drop-in /etc/systemd/system/miyukini-maia.service.d/memoire.conf avec
         MAIA_MEMOIRE_CLES_UTILISATEUR (clé publique utilisateur) et MAIA_TAMR_CLES_APPROBATION
         (clé d'approbation, DISTINCTE de la précédente), puis systemctl daemon-reload et restart.
       Les graines des clés sont générées sur place (0600), jamais dans le paquet.

  3. Contrôle : $EXPLOITATION/etat.sh   ·   Sauvegarde : $EXPLOITATION/sauvegarder.sh --help
FIN
exit 0
}
