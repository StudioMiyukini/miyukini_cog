#!/usr/bin/env bash
# Compilation reproductible des binaires officiels (exigence EX-04).
#
# Principe : le commit HEAD est exporte deux fois (git archive) dans deux
# repertoires source distincts, chacun avec son propre repertoire target.
# Les deux builds sont faits en --release --locked avec des chemins remappes
# (source, target, CARGO_HOME, sysroot) et SOURCE_DATE_EPOCH fixe au commit.
# Les SHA-256 des binaires sont compares ; toute difference fait echouer.
#
# Binaires officiels (paquet:binaire). Les Cores sans binaire (kernel,
# strongfather, borderguard, caringnanny, ...) sont des bibliotheques
# compilees transitivement par ces binaires.
#   miyudouane:miyudouane                     Core Douane
#   miyuportail:portail-sas                   Core Portail, sas seccomp
#   miyuracine:miyuracine                     Racine COG hors ligne
#   kindmother-service:kindmother-server      Core KindMother, serveur libSQL
#   kindmother-import:kindmother-import       import transitoire (K-13)
#   miyukini-central-native:miyukini-central  Central (Dioxus, libs GTK requises)
#
# Par defaut Central est exclu (build Linux bloque sur les libs GTK) :
# le passer explicitement avec --sous-ensemble miyukini-central.
#
# Usage :
#   scripts/build-reproductible.sh [--sous-ensemble b1,b2] [--garder]
#                                  [--manifeste FICHIER] [--liste]
# Variables : MCOG_REPRO_BASE (defaut /tmp), MCOG_REPRO_CODEGEN_UNITS (defaut 1).
# Codes de sortie : 0 identique, 1 difference detectee, 2 erreur d'usage ou de build.

set -euo pipefail
export LC_ALL=C TZ=UTC
export PATH="$HOME/.cargo/bin:$PATH"

RACINE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# paquet:binaire, un par ligne
BINAIRES_OFFICIELS=(
  "miyudouane:miyudouane"
  "miyuportail:portail-sas"
  "miyuracine:miyuracine"
  "kindmother-service:kindmother-server"
  "kindmother-import:kindmother-import"
  "miyukini-central-native:miyukini-central"
)
# Exclus du defaut (libs GTK), accessibles via --sous-ensemble
BINAIRES_HORS_DEFAUT=("miyukini-central")

SOUS_ENSEMBLE=""
GARDER=0
MANIFESTE=""
BASE_RACINE="${MCOG_REPRO_BASE:-/tmp}"
CODEGEN_UNITS="${MCOG_REPRO_CODEGEN_UNITS:-1}"

usage() {
  sed -n '2,25p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

est_hors_defaut() {
  local h
  for h in "${BINAIRES_HORS_DEFAUT[@]}"; do
    if [[ "$h" == "$1" ]]; then return 0; fi
  done
  return 1
}

afficher_liste() {
  printf '%-22s %-24s %s\n' BINAIRE PAQUET DEFAUT
  for entree in "${BINAIRES_OFFICIELS[@]}"; do
    local paquet="${entree%%:*}" bin="${entree##*:}" defaut=oui
    if est_hors_defaut "$bin"; then defaut=non; fi
    printf '%-22s %-24s %s\n' "$bin" "$paquet" "$defaut"
  done
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --sous-ensemble) SOUS_ENSEMBLE="${2:-}"; shift 2 ;;
    --garder) GARDER=1; shift ;;
    --manifeste) MANIFESTE="${2:-}"; shift 2 ;;
    --liste) afficher_liste; exit 0 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Option inconnue : $1" >&2; usage >&2; exit 2 ;;
  esac
done

# Selection des paires paquet:binaire
PAIRES=()
if [[ -n "$SOUS_ENSEMBLE" ]]; then
  IFS=',' read -r -a demandes <<< "$SOUS_ENSEMBLE"
  for demande in "${demandes[@]}"; do
    trouve=0
    for entree in "${BINAIRES_OFFICIELS[@]}"; do
      if [[ "${entree##*:}" == "$demande" ]]; then PAIRES+=("$entree"); trouve=1; fi
    done
    if [[ $trouve -eq 0 ]]; then
      echo "Binaire inconnu dans la liste officielle : $demande (voir --liste)" >&2
      exit 2
    fi
  done
else
  for entree in "${BINAIRES_OFFICIELS[@]}"; do
    if ! est_hors_defaut "${entree##*:}"; then
      PAIRES+=("$entree")
    fi
  done
fi
[[ ${#PAIRES[@]} -gt 0 ]] || { echo "Aucun binaire selectionne" >&2; exit 2; }

for outil in git cargo rustc sha256sum tar; do
  command -v "$outil" >/dev/null || { echo "Outil requis absent : $outil" >&2; exit 2; }
done

COMMIT="$(git -C "$RACINE" rev-parse --verify HEAD)"
EPOCH="$(git -C "$RACINE" log -1 --format=%ct "$COMMIT")"
if [[ -n "$(git -C "$RACINE" status --porcelain --untracked-files=no)" ]]; then
  echo "AVERTISSEMENT : arbre de travail modifie, seul le commit $COMMIT est compile." >&2
fi

BASE="$(mktemp -d "$BASE_RACINE/mcog-repro-XXXXXX")"
nettoyer() {
  if [[ $GARDER -eq 1 ]]; then
    echo "Repertoires conserves : $BASE"
  else
    rm -rf "$BASE"
  fi
}
trap nettoyer EXIT

PAQUETS_ARGS=()
BINS_ARGS=()
for entree in "${PAIRES[@]}"; do
  PAQUETS_ARGS+=(-p "${entree%%:*}")
  BINS_ARGS+=(--bin "${entree##*:}")
done

construire() {
  local etiquette="$1"
  local dossier="$BASE/$etiquette"
  local src="$dossier/src" tgt="$dossier/target" sortie="$dossier/sortie"
  local cargo_home="${CARGO_HOME:-$HOME/.cargo}"
  local sysroot
  sysroot="$(rustc --print sysroot)"
  mkdir -p "$src" "$tgt" "$sortie"
  git -C "$RACINE" archive --format=tar "$COMMIT" | tar -x -C "$src"

  local rf="--remap-path-prefix=$src=/build/src"
  rf+=" --remap-path-prefix=$tgt=/build/target"
  rf+=" --remap-path-prefix=$cargo_home=/build/cargo"
  rf+=" --remap-path-prefix=$sysroot=/build/rustc"

  echo "== Build $etiquette : $src (target $tgt)"
  (
    cd "$src"
    env -i PATH="$PATH" HOME="$HOME" \
      CARGO_HOME="$cargo_home" RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}" \
      LC_ALL=C TZ=UTC SOURCE_DATE_EPOCH="$EPOCH" \
      CARGO_INCREMENTAL=0 CARGO_TERM_COLOR=never \
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS="$CODEGEN_UNITS" \
      CARGO_PROFILE_RELEASE_DEBUG=0 \
      RUSTFLAGS="$rf" \
      CARGO_TARGET_DIR="$tgt" \
      cargo build --release --locked "${PAQUETS_ARGS[@]}" "${BINS_ARGS[@]}"
  )
  for entree in "${PAIRES[@]}"; do
    local bin="${entree##*:}"
    cp "$tgt/release/$bin" "$sortie/$bin"
  done
}

construire a
construire b

MANIFESTE="${MANIFESTE:-$RACINE/target/reproductible/manifeste.txt}"
mkdir -p "$(dirname "$MANIFESTE")"
: > "$MANIFESTE"
differents=()
for entree in "${PAIRES[@]}"; do
  bin="${entree##*:}"
  ha="$(sha256sum "$BASE/a/sortie/$bin" | cut -d' ' -f1)"
  hb="$(sha256sum "$BASE/b/sortie/$bin" | cut -d' ' -f1)"
  printf '%s  %s\n' "$bin" "$ha" >> "$MANIFESTE"
  if [[ "$ha" == "$hb" ]]; then
    echo "IDENTIQUE  $bin  $ha"
  else
    echo "DIFFERENT  $bin  a=$ha b=$hb"
    differents+=("$bin")
  fi
done

echo "Commit $COMMIT (SOURCE_DATE_EPOCH=$EPOCH)"
echo "Manifeste : $MANIFESTE"
if [[ ${#differents[@]} -gt 0 ]]; then
  echo "NON REPRODUCTIBLE : ${differents[*]}" >&2
  exit 1
fi
echo "Reproductible : ${#PAIRES[@]} binaire(s) identiques."
