#!/usr/bin/env bash
# Construction du paquet de recette COG (MACHINE DE CONSTRUCTION uniquement, jamais la recette).
#
#   deploy/recette/construire-paquet.sh --generer-cle
#   deploy/recette/construire-paquet.sh [--cle FICHIER] [--sortie DIR] [--suffixe TEXTE]
#         [--source DIR] [--sans-compilation] [--autoriser-arbre-modifie] [--verifier-reproductible]
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
# --source DIR : arbre git à empaqueter (défaut : le dépôt qui contient ce script). Ce script n'est
# qu'un outil de construction (il n'est pas dans le paquet). Usage type : extraire un arbre propre
# (git worktree add DIR <commit>) et le construire sans toucher au dépôt de travail.
#
# Export : le contenu du paquet (binaires, outil-sceau, installateur, tests, exploitation,
# configuration) est compilé et copié depuis un EXPORT du commit (git archive), toujours au même
# chemin absolu, /var/tmp/cog-construction/src (verrou : une construction à la fois). Arbre
# modifié autorisé : l'export reprend les fichiers suivis de l'arbre de travail. Le chemin fixe est
# nécessaire : outil-sceau est un workspace autonome dont la dépendance miyusceau est hors de sa
# racine, et cargo mêle alors le chemin ABSOLU de miyusceau au hachage des symboles (-C metadata),
# ce que --remap-path-prefix ne couvre pas.
#
# Reproductibilité : à commit égal, deux constructions donnent la même archive (même SHA-256),
# quels que soient le chemin de --source et CARGO_TARGET_DIR : export au chemin fixe ; compilation
# avec chemins remappés (export, target, CARGO_HOME, sysroot), SOURCE_DATE_EPOCH = date
# du commit, CARGO_INCREMENTAL=0, codegen-units=1 (mêmes réglages que scripts/build-reproductible.sh) ;
# dates du MANIFESTE et du sceau = date du commit (jamais l'horloge) ; tar trié, propriétaire 0,
# mtime du commit. Les signatures Ed25519 sont déterministes. Cargo.lock de outil-sceau : s'il n'est
# pas suivi par git, son SHA-256 est consigné dans VERSION (dépendances hors commit).
#
# --verifier-reproductible lance scripts/build-reproductible.sh sur les binaires serveur qu'il
# couvre (kindmother-server, portail-sas, miyudouane) : deux compilations du commun HEAD, SHA-256
# identiques exigés. Long (compilation depuis zéro) ; son résultat est écrit dans VERSION.
#
# Variables : CARGO_TARGET_DIR (défaut <source>/target ; outil-sceau dans <target>/outil-sceau).
# Sortie par défaut : <source>/target/paquets.
# Outils requis : cargo, git, sha256sum, tar, zstd, openssl, cc (horloge fixe du scellement).
set -euo pipefail
umask 0022
export LC_ALL=C TZ=UTC
export PATH="$HOME/.cargo/bin:$PATH"

ICI=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
RACINE=$(cd "$ICI/../.." && pwd)
CLE_DEFAUT=$HOME/.miyukini-beta/cle-editeur-beta.hex

CLE=$CLE_DEFAUT
SORTIE=""
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
		--source) RACINE=$(cd "${2:?--source : valeur attendue}" && pwd) || die "--source : dossier introuvable"; shift ;;
		--sans-compilation) SANS_COMPILATION=1 ;;
		--autoriser-arbre-modifie) AUTORISER_MODIFIE=1 ;;
		--verifier-reproductible) VERIFIER_REPRO=1 ;;
		--generer-cle) GENERER_CLE=1 ;;
		-h | --help) sed -n '2,59p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
		*) die "option inconnue : $1" ;;
	esac
	shift
done

for outil in cargo rustc git sha256sum tar zstd openssl cc; do
	command -v "$outil" >/dev/null || die "outil requis absent : $outil"
done

git -C "$RACINE" rev-parse --is-inside-work-tree >/dev/null 2>&1 || die "--source : $RACINE n'est pas un arbre git"
[[ $(git -C "$RACINE" rev-parse --show-toplevel) == "$RACINE" ]] || die "--source : $RACINE n'est pas la racine de l'arbre git"

# ------------------------------------------------------------------ état de l'arbre

COMMIT=$(git -C "$RACINE" rev-parse --verify HEAD)
COMMIT12=${COMMIT:0:12}
EPOCH=$(git -C "$RACINE" log -1 --format=%ct "$COMMIT")
VERSION=$(git -C "$RACINE" show "$COMMIT:Cargo.toml" | tr -d '\r' | sed -n '/^\[workspace.package\]/,/^\[/ s/^version = "\(.*\)"$/\1/p')
[[ -n $VERSION ]] || die "version du workspace introuvable dans Cargo.toml"
MODIFS=$(git -C "$RACINE" status --porcelain --untracked-files=no | wc -l)
if ((MODIFS > 0)); then
	if ((!AUTORISER_MODIFIE && !GENERER_CLE)); then
		die "arbre de travail modifié ($MODIFS fichier(s) suivis non commités) : refus. Commitez, ou passez --autoriser-arbre-modifie (paquet marqué non reproductible)."
	fi
	ARBRE="modifié ($MODIFS fichier(s) suivis non commités : binaires NON reproductibles depuis le commit)"
	MARQUE="-non-reproductible"
	log "AVERTISSEMENT : arbre modifié, paquet marqué non reproductible"
else
	ARBRE="propre"
	MARQUE=""
fi

# ------------------------------------------------------------------ export au chemin fixe

# Chemin fixe (la reproductibilité en dépend) sous /var/tmp, partagé et inscriptible par tous :
# le dossier doit appartenir à l'utilisateur courant, en 0700, sans lien symbolique, sinon refus
# (un autre compte qui le créerait d'avance pourrait empoisonner la construction).
CONSTRUCTION=/var/tmp/cog-construction
SRC=$CONSTRUCTION/src
if [[ ! -e $CONSTRUCTION && ! -L $CONSTRUCTION ]]; then
	mkdir -m 0700 "$CONSTRUCTION" || die "création de $CONSTRUCTION impossible"
fi
[[ -d $CONSTRUCTION && ! -L $CONSTRUCTION ]] || die "$CONSTRUCTION n'est pas un dossier (lien symbolique ou fichier) : refus"
[[ $(stat -c %u "$CONSTRUCTION") == "$(id -u)" ]] || die "$CONSTRUCTION n'appartient pas à l'utilisateur courant : refus"
chmod 0700 "$CONSTRUCTION"
[[ ! -L $CONSTRUCTION/verrou ]] || die "$CONSTRUCTION/verrou est un lien symbolique : refus"
exec 9>>"$CONSTRUCTION/verrou"
flock -n 9 || die "une autre construction utilise $CONSTRUCTION (verrou)"
rm -rf "$SRC"
install -d -m 0700 "$SRC"
if ((MODIFS > 0)); then
	# Fichiers suivis de l'arbre de travail (modifications comprises, fichiers supprimés ignorés).
	(cd "$RACINE" && git ls-files -z | while IFS= read -r -d '' f; do [[ -e $f || -L $f ]] && printf '%s\0' "$f"; done |
		tar --null -T - -cf -) | tar -x -C "$SRC" || die "export de l'arbre de travail"
else
	git -C "$RACINE" archive --format=tar "$COMMIT" | tar -x -C "$SRC" || die "export du commit"
fi
# Cargo.lock de outil-sceau : copié de --source s'il n'est pas suivi par git.
LOCK_OUTIL_HORS_GIT=""
if [[ ! -f $SRC/deploy/recette/outil-sceau/Cargo.lock ]]; then
	[[ -f $RACINE/deploy/recette/outil-sceau/Cargo.lock ]] ||
		die "deploy/recette/outil-sceau/Cargo.lock absent (ni suivi par git ni présent dans $RACINE)"
	cp "$RACINE/deploy/recette/outil-sceau/Cargo.lock" "$SRC/deploy/recette/outil-sceau/Cargo.lock"
	LOCK_OUTIL_HORS_GIT=$(sha256sum "$SRC/deploy/recette/outil-sceau/Cargo.lock" | cut -d' ' -f1)
fi

REC=$SRC/deploy/recette
VMD=$SRC/deploy/cog-vm
CIBLE=${CARGO_TARGET_DIR:-$RACINE/target}
mkdir -p "$CIBLE"
CIBLE=$(cd "$CIBLE" && pwd)
REL=$CIBLE/release
OUTIL_DIR=$REC/outil-sceau
OUTIL_CIBLE=$CIBLE/outil-sceau
OUTIL=$OUTIL_CIBLE/release/outil-sceau
SORTIE=${SORTIE:-$RACINE/target/paquets}

# Environnement de compilation reproductible (mêmes réglages que scripts/build-reproductible.sh).
CARGO_HOME_EFF=${CARGO_HOME:-$HOME/.cargo}
SYSROOT=$(rustc --print sysroot)
RF="--remap-path-prefix=$SRC=/build/src --remap-path-prefix=$CIBLE=/build/target"
RF+=" --remap-path-prefix=$CARGO_HOME_EFF=/build/cargo --remap-path-prefix=$SYSROOT=/build/rustc"
cargo_repro() {
	env SOURCE_DATE_EPOCH="$EPOCH" CARGO_INCREMENTAL=0 CARGO_TERM_COLOR=never \
		CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 CARGO_PROFILE_RELEASE_DEBUG=0 RUSTFLAGS="$RF" \
		cargo "$@"
}

# ------------------------------------------------------------------ outil de signature

construire_outil() {
	log "outil de signature (crate autonome deploy/recette/outil-sceau)"
	(cd "$OUTIL_DIR" && CARGO_TARGET_DIR="$OUTIL_CIBLE" cargo_repro build --release --locked) >/dev/null ||
		die "compilation de outil-sceau"
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
		"$RACINE"/* | "$CONSTRUCTION"/*) die "la clé d'éditeur ne doit pas se trouver dans le dépôt" ;;
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

NOM="cog-$VERSION-$COMMIT12$MARQUE$SUFFIXE"
log "version $VERSION ; commit $COMMIT ; arbre $ARBRE ; export $SRC"

verifier_cle

# ------------------------------------------------------------------ compilation

if ((SANS_COMPILATION == 0)); then
	construire_outil
	log "compilation release (--locked) des binaires serveur et outils"
	# static-dist est produit DANS l'export par build.rs de miyucloud. L'export porte les dates du
	# commit : sans ce touch, un target déjà chaud jugerait build.rs à jour et ne le relancerait pas.
	touch "$SRC/apps/miyucloud/build.rs"
	(cd "$SRC" && CARGO_TARGET_DIR="$CIBLE" cargo_repro build --release --locked \
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

cp "$SRC/bridge.toml" "$P/config/bridge.toml"
cp -r "$SRC/config/strongfather" "$P/config/strongfather"
cp -r "$SRC/apps/miyucloud/static" "$P/static/static"
[[ -d $SRC/apps/miyucloud/static-dist ]] ||
	die "static-dist absent de l'export (produit par build.rs de miyucloud à la compilation : retirez --sans-compilation)"
cp -r "$SRC/apps/miyucloud/static-dist" "$P/static/static-dist"
cp "$SRC"/deploy/apparmor/miyukini-* "$P/apparmor/"

# Unités : générées par la logique commune, donc identiques à celles de l'installateur.
cp "$VMD/commun.sh" "$P/commun.sh"
(
	LOG_TAG=construire
	# shellcheck source=../cog-vm/commun.sh
	. "$P/commun.sh"
	generer_unites "$P/systemd"
)
cp "$REC/installer.sh" "$P/installer.sh"
cp "$REC/exploitation/"*.sh "$P/exploitation/"
cp "$VMD/tester.sh" "$VMD/scenarios.sh" "$REC"/tests/*.sh "$P/tests/"

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
if [[ -n $LOCK_OUTIL_HORS_GIT ]]; then
	printf 'outil_sceau_cargo_lock %s (non suivi par git : dependances hors commit)\n' "$LOCK_OUTIL_HORS_GIT" >>"$P/VERSION"
	log "AVERTISSEMENT : deploy/recette/outil-sceau/Cargo.lock non suivi par git (SHA-256 $LOCK_OUTIL_HORS_GIT consigné dans VERSION)"
fi

{
	printf '# paquet %s (recette beta, non destiné à la production)\n' "$NOM"
	printf '# version %s\n' "$VERSION"
	printf '# commit %s\n' "$COMMIT"
	printf '# arbre %s\n' "$ARBRE"
	printf '# reproductible %s\n' "$RESULTAT_REPRO"
	printf '# date_source %s\n' "$(date -u -d "@$EPOCH" +%Y-%m-%dT%H:%M:%SZ)"
	(cd "$P" && find . -type f ! -path ./MANIFESTE ! -path ./MANIFESTE.sceau ! -path ./MANIFESTE.sig -print0 | LC_ALL=C sort -z | xargs -0 sha256sum)
} >"$P/MANIFESTE"

# outil-sceau date le sceau avec l'horloge (CLOCK_REALTIME) : cet appel seul voit l'horloge fixée
# à SOURCE_DATE_EPOCH (date du commit) par une bibliothèque LD_PRELOAD compilée ici, hors paquet.
# Le sceau ne porte pas de fenêtre de validité : la date n'est qu'informative.
HORLOGE=$STAGE_PARENT/horloge-fixe.so
cc -shared -fPIC -O2 -o "$HORLOGE" -x c - -ldl <<'C' || die "compilation de l'horloge fixe"
#define _GNU_SOURCE
#include <dlfcn.h>
#include <stdlib.h>
#include <time.h>
int clock_gettime(clockid_t id, struct timespec *ts) {
	static int (*vrai)(clockid_t, struct timespec *);
	if (!vrai) vrai = (int (*)(clockid_t, struct timespec *))dlsym(RTLD_NEXT, "clock_gettime");
	int r = vrai(id, ts);
	const char *e = getenv("SOURCE_DATE_EPOCH");
	if (r == 0 && e && (id == CLOCK_REALTIME || id == CLOCK_REALTIME_COARSE)) {
		ts->tv_sec = (time_t)strtoll(e, NULL, 10);
		ts->tv_nsec = 0;
	}
	return r;
}
C
SOURCE_DATE_EPOCH=$EPOCH LD_PRELOAD=$HORLOGE "$OUTIL" sceller --cle "$CLE" --manifeste "$P/MANIFESTE" \
	--paquet cog --version "$VERSION" --commit "$COMMIT" --arbre "$ARBRE" --sortie "$P/MANIFESTE.sceau" >/dev/null ||
	die "scellement du MANIFESTE"
rm -f "$HORLOGE"

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
tar -C "$STAGE_PARENT" --format=gnu --sort=name --owner=0 --group=0 --numeric-owner --mtime="@$EPOCH" \
	--no-acls --no-xattrs --no-selinux -cf - "$NOM" |
	zstd -q -19 -T0 -f -o "$ARCHIVE"
(cd "$SORTIE" && sha256sum "$NOM.tar.zst" >"$NOM.tar.zst.sha256")

log "paquet : $ARCHIVE ($(du -h "$ARCHIVE" | cut -f1))"
log "sha256 archive : $(cut -d' ' -f1 "$SORTIE/$NOM.tar.zst.sha256")"
log "sha256 MANIFESTE : $(sha256sum "$P/MANIFESTE" | cut -d' ' -f1)"
log "sha256 outil-sceau : $(sha256sum "$P/bin/outil-sceau" | cut -d' ' -f1)"
log "clé publique d'éditeur : $CLE_PUB"
log "empreinte (kid) de la clé : $EMPREINTE_CLE"
log "fichiers au manifeste : $(grep -vc '^#' "$P/MANIFESTE")"
