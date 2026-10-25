#!/usr/bin/env bash
# Construction du paquet de recette COG (MACHINE DE CONSTRUCTION uniquement, jamais la recette).
#
#   deploy/recette/construire-paquet.sh --generer-cle
#   deploy/recette/construire-paquet.sh [--cle FICHIER] [--sortie DIR] [--suffixe TEXTE]
#         [--sans-compilation] [--autoriser-arbre-modifie] [--verifier-reproductible]
#
# Produit cog-<version>-<commit12>[-<suffixe>].tar.zst (+ .sha256) :
#   bin/        binaires serveur (kindmother-server, maia, miyucloud-server, cog-web-portal,
#               portail-sas, miyudouane, miyurelais, miyuregistre-publier), exemple arretes_essai,
#               outil-sceau. miyuracine N'EST PAS inclus (cérémonie hors ligne).
#   config/     bridge.toml et strongfather/*.toml (défauts fail-closed du dépôt)
#   static/     statiques MiyuCloud (version de production et source)
#   apparmor/   profils miyukini-* (mode enforce à l'installation)
#   systemd/    unités générées par commun.sh (couvertes par le MANIFESTE)
#   exploitation/ sauvegarder, restaurer, mettre-a-jour, etat, desinstaller
#   tests/      tester.sh et scenarios.sh (cog-vm), zone.sh, installateur.sh et mise-a-jour.sh (recette)
#   commun.sh installer.sh cle-publique-editeur.{hex,pem} VERSION
#   MANIFESTE   SHA-256 de chaque fichier (sauf MANIFESTE, MANIFESTE.sceau, MANIFESTE.sig), lié au commit
#   MANIFESTE.sig   signature détachée Ed25519 brute du MANIFESTE (openssl pkeyutl -rawin) : AUTHENTIFICATION
#   MANIFESTE.sceau sceau Ed25519 de l'éditeur (Sceau v1.1, type ManifestePaquet), second contrôle
#
# Clé d'éditeur : --cle (défaut ~/.miyukini-beta/cle-editeur-beta.hex, dossier 0700, fichier 0600),
# hors dépôt. --generer-cle la crée une seule fois (refuse d'écraser). La clé publique est écrite
# dans le paquet et doit figurer dans les notes de version.
#
# Arbre modifié : refus par défaut. --autoriser-arbre-modifie construit quand même, et marque le
# paquet « non reproductible » (nom, en-tête du MANIFESTE, VERSION).
#
# --verifier-reproductible lance scripts/build-reproductible.sh sur les binaires serveur qu'il
# couvre (kindmother-server, portail-sas, miyudouane) : deux compilations du commun HEAD, SHA-256
# identiques exigés. Long (compilation depuis zéro) ; son résultat est écrit dans VERSION.
#
# Variables : CARGO_TARGET_DIR (défaut <dépôt>/target).
set -euo pipefail
umask 0022
export LC_ALL=C TZ=UTC
export PATH="$HOME/.cargo/bin:$PATH"

ICI=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
RACINE=$(cd "$ICI/../.." && pwd)
REL=${CARGO_TARGET_DIR:-$RACINE/target}/release
OUTIL_DIR=$ICI/outil-sceau
OUTIL=$OUTIL_DIR/target/release/outil-sceau
CLE_DEFAUT=$HOME/.miyukini-beta/cle-editeur-beta.hex

CLE=$CLE_DEFAUT
SORTIE=$RACINE/target/paquets
SUFFIXE=""
SANS_COMPILATION=0
AUTORISER_MODIFIE=0
VERIFIER_REPRO=0
GENERER_CLE=0

log() { printf '[construire] %s\n' "$*"; }
die() { printf '[construire] ERREUR: %s\n' "$*" >&2; exit 1; }

while (($#)); do
	case $1 in
		--cle) CLE=${2:?--cle : valeur attendue}; shift ;;
		--sortie) SORTIE=${2:?--sortie : valeur attendue}; shift ;;
		--suffixe) SUFFIXE="-${2:?--suffixe : valeur attendue}"; shift ;;
		--sans-compilation) SANS_COMPILATION=1 ;;
		--autoriser-arbre-modifie) AUTORISER_MODIFIE=1 ;;
		--verifier-reproductible) VERIFIER_REPRO=1 ;;
		--generer-cle) GENERER_CLE=1 ;;
		-h | --help) sed -n '2,36p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
		*) die "option inconnue : $1" ;;
	esac
	shift
done

for outil in cargo git sha256sum tar zstd; do
	command -v "$outil" >/dev/null || die "outil requis absent : $outil"
done

# ------------------------------------------------------------------ outil de signature

construire_outil() {
	log "outil de signature (crate autonome deploy/recette/outil-sceau)"
	(cd "$OUTIL_DIR" && cargo build --release --locked) >/dev/null || die "compilation de outil-sceau"
	[[ -x $OUTIL ]] || die "outil-sceau absent après compilation"
}

# ------------------------------------------------------------------ clé d'éditeur

verifier_cle() {
	[[ -f $CLE ]] || die "clé d'éditeur absente : $CLE (--generer-cle pour la créer une fois)"
	local dir mode_f mode_d
	dir=$(dirname "$CLE")
	mode_d=$(stat -c %a "$dir")
	mode_f=$(stat -c %a "$CLE")
	[[ $mode_d == 700 ]] || die "dossier de la clé $dir : mode $mode_d (0700 attendu)"
	[[ $mode_f == 600 ]] || die "clé $CLE : mode $mode_f (0600 attendu)"
	case "$(realpath "$CLE")" in
		"$RACINE"/*) die "la clé d'éditeur ne doit pas se trouver dans le dépôt" ;;
	esac
}

if ((GENERER_CLE)); then
	construire_outil
	install -d -m 0700 "$(dirname "$CLE")"
	[[ ! -e $CLE ]] || die "clé déjà présente, jamais écrasée : $CLE"
	"$OUTIL" generer-cle --sortie "$CLE" | sed 's/^/[construire] /'
	cle_pub=$("$OUTIL" info-cle --cle "$CLE" | awk '/^cle_publique/{print $2}')
	printf '%s\n' "$cle_pub" >"$(dirname "$CLE")/cle-publique-editeur-beta.hex"
	log "clé publique (à publier dans les notes de version) : $cle_pub"
	exit 0
fi

# ------------------------------------------------------------------ état de l'arbre

COMMIT=$(git -C "$RACINE" rev-parse --verify HEAD)
COMMIT12=${COMMIT:0:12}
EPOCH=$(git -C "$RACINE" log -1 --format=%ct "$COMMIT")
VERSION=$(tr -d '\r' <"$RACINE/Cargo.toml" | sed -n '/^\[workspace.package\]/,/^\[/ s/^version = "\(.*\)"$/\1/p')
[[ -n $VERSION ]] || die "version du workspace introuvable dans Cargo.toml"
MODIFS=$(git -C "$RACINE" status --porcelain --untracked-files=no | wc -l)
if ((MODIFS > 0)); then
	if ((!AUTORISER_MODIFIE)); then
		die "arbre de travail modifié ($MODIFS fichier(s) suivis non commités) : refus. Commitez, ou passez --autoriser-arbre-modifie (paquet marqué non reproductible)."
	fi
	ARBRE="modifié ($MODIFS fichier(s) suivis non commités : binaires NON reproductibles depuis le commit)"
	MARQUE="-non-reproductible"
	log "AVERTISSEMENT : arbre modifié, paquet marqué non reproductible"
else
	ARBRE="propre"
	MARQUE=""
fi
NOM="cog-$VERSION-$COMMIT12$MARQUE$SUFFIXE"
log "version $VERSION ; commit $COMMIT ; arbre $ARBRE"

verifier_cle

# ------------------------------------------------------------------ compilation

if ((SANS_COMPILATION == 0)); then
	construire_outil
	log "compilation release (--locked) des binaires serveur et outils"
	(cd "$RACINE" && cargo build --release --locked \
		-p kindmother-service -p maia -p miyucloud-server -p cog-web-portal -p miyuportail \
		-p miyudouane -p miyurelais -p miyuregistre -p maia-memoire --features miyuregistre/publier \
		--bin kindmother-server --bin maia --bin miyucloud-server --bin cog-web-portal \
		--bin portail-sas --bin miyudouane --bin miyurelais --bin miyuregistre-publier \
		--example arretes_essai) || die "compilation release"
fi

for b in kindmother-server maia miyucloud-server cog-web-portal portail-sas miyudouane miyurelais miyuregistre-publier; do
	[[ -x $REL/$b ]] || die "binaire absent : $REL/$b"
done
[[ -x $REL/examples/arretes_essai ]] || die "exemple absent : $REL/examples/arretes_essai"
[[ -x $OUTIL ]] || construire_outil

# ------------------------------------------------------------------ reproductibilité (optionnelle)

RESULTAT_REPRO="non verifie (--verifier-reproductible non demande)"
if ((VERIFIER_REPRO)); then
	log "scripts/build-reproductible.sh (kindmother-server, portail-sas, miyudouane) : longue compilation"
	if (cd "$RACINE" && scripts/build-reproductible.sh --sous-ensemble kindmother-server,portail-sas,miyudouane); then
		RESULTAT_REPRO="verifie (SHA-256 identiques sur deux compilations du commit HEAD)"
	else
		die "reproductibilité : SHA-256 différents ou erreur de compilation"
	fi
fi

# ------------------------------------------------------------------ assemblage

mkdir -p "$SORTIE"
STAGE_PARENT=$(mktemp -d "$SORTIE/.construction-XXXXXX")
trap 'rm -rf "$STAGE_PARENT"' EXIT
P="$STAGE_PARENT/$NOM"
mkdir -p "$P/bin" "$P/config" "$P/static" "$P/apparmor" "$P/systemd" "$P/exploitation" "$P/tests"

for b in kindmother-server maia miyucloud-server cog-web-portal portail-sas miyudouane miyurelais miyuregistre-publier; do
	cp "$REL/$b" "$P/bin/$b"
done
cp "$REL/examples/arretes_essai" "$P/bin/arretes_essai"
cp "$OUTIL" "$P/bin/outil-sceau"
[[ ! -e $P/bin/miyuracine ]] || die "miyuracine ne doit jamais figurer dans un paquet de recette"

cp "$RACINE/bridge.toml" "$P/config/bridge.toml"
cp -r "$RACINE/config/strongfather" "$P/config/strongfather"
cp -r "$RACINE/apps/miyucloud/static" "$P/static/static"
cp -r "$RACINE/apps/miyucloud/static-dist" "$P/static/static-dist"
cp "$RACINE"/deploy/apparmor/miyukini-* "$P/apparmor/"

# Unités : générées par la logique commune, donc identiques à celles de l'installateur.
cp "$ICI/../cog-vm/commun.sh" "$P/commun.sh"
(
	LOG_TAG=construire
	# shellcheck source=../cog-vm/commun.sh
	. "$P/commun.sh"
	generer_unites "$P/systemd"
)
cp "$ICI/installer.sh" "$P/installer.sh"
cp "$ICI/exploitation/"*.sh "$P/exploitation/"
cp "$ICI/../cog-vm/tester.sh" "$ICI/../cog-vm/scenarios.sh" "$ICI"/tests/*.sh "$P/tests/"

chmod 0755 "$P/installer.sh" "$P"/exploitation/*.sh "$P/tests/"*.sh "$P/bin/"*
chmod 0644 "$P/commun.sh"

"$OUTIL" info-cle --cle "$CLE" | awk '/^cle_publique/{print $2}' >"$P/cle-publique-editeur.hex"
"$OUTIL" cle-publique-pem --cle "$CLE" --sortie "$P/cle-publique-editeur.pem"
CLE_PUB=$(cat "$P/cle-publique-editeur.hex")
EMPREINTE_CLE=$("$OUTIL" info-cle --cle "$CLE" | awk '/^empreinte/{print $2}')

cat >"$P/VERSION" <<FIN
paquet cog
version $VERSION
commit $COMMIT
arbre $ARBRE
commun_date $(date -u -d "@$EPOCH" +%Y-%m-%dT%H:%M:%SZ)
reproductible $RESULTAT_REPRO
FIN

{
	printf '# paquet %s (recette beta, non destiné à la production)\n' "$NOM"
	printf '# version %s\n' "$VERSION"
	printf '# commit %s\n' "$COMMIT"
	printf '# arbre %s\n' "$ARBRE"
	printf '# reproductible %s\n' "$RESULTAT_REPRO"
	printf '# prepare_le %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
	(cd "$P" && find . -type f ! -path ./MANIFESTE ! -path ./MANIFESTE.sceau ! -path ./MANIFESTE.sig -print0 | LC_ALL=C sort -z | xargs -0 sha256sum)
} >"$P/MANIFESTE"

"$OUTIL" sceller --cle "$CLE" --manifeste "$P/MANIFESTE" --paquet cog --version "$VERSION" \
	--commit "$COMMIT" --arbre "$ARBRE" --sortie "$P/MANIFESTE.sceau" >/dev/null ||
	die "scellement du MANIFESTE"

# Signature détachée Ed25519 brute du MANIFESTE (authentification par openssl, avant toute exécution).
"$OUTIL" signer-brut --cle "$CLE" --fichier "$P/MANIFESTE" --sortie "$P/MANIFESTE.sig"
openssl pkeyutl -verify -pubin -inkey "$P/cle-publique-editeur.pem" -rawin -in "$P/MANIFESTE" \
	-sigfile "$P/MANIFESTE.sig" >/dev/null 2>&1 || die "signature détachée du MANIFESTE non vérifiée par openssl"

# Vérification immédiate du paquet assemblé (sceau puis SHA-256), avec la clé publique publiée.
"$OUTIL" verifier --cle-publique "$CLE_PUB" --manifeste "$P/MANIFESTE" --sceau "$P/MANIFESTE.sceau" >/dev/null ||
	die "le paquet assemblé ne se vérifie pas avec sa propre clé publique"
(cd "$P" && grep -v '^#' MANIFESTE | sha256sum --check --strict --quiet -) || die "SHA-256 incohérents après assemblage"

# Archive déterministe : propriétaire 0, dates du commit.
ARCHIVE="$SORTIE/$NOM.tar.zst"
tar -C "$STAGE_PARENT" --sort=name --owner=0 --group=0 --numeric-owner --mtime="@$EPOCH" -cf - "$NOM" |
	zstd -q -19 -T0 -f -o "$ARCHIVE"
(cd "$SORTIE" && sha256sum "$NOM.tar.zst" >"$NOM.tar.zst.sha256")

log "paquet : $ARCHIVE ($(du -h "$ARCHIVE" | cut -f1))"
log "sha256 archive : $(cut -d' ' -f1 "$SORTIE/$NOM.tar.zst.sha256")"
log "sha256 MANIFESTE : $(sha256sum "$P/MANIFESTE" | cut -d' ' -f1)"
log "sha256 outil-sceau : $(sha256sum "$P/bin/outil-sceau" | cut -d' ' -f1)"
log "clé publique d'éditeur : $CLE_PUB"
log "empreinte (kid) de la clé : $EMPREINTE_CLE"
log "fichiers au manifeste : $(grep -vc '^#' "$P/MANIFESTE")"
