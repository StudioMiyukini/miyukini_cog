#!/usr/bin/env bash
# couverture-securite.sh — contraintes C-62 / C-26 (docs/implementation/CONTRAINTES-IMPLEMENTATION.md)
#
# Mesure la couverture des lignes HORS TESTS des crates de sécurité (cargo-llvm-cov),
# affiche un tableau par crate et échoue si l'une d'elles est sous le seuil.
# Lignes exclues : items #[cfg(test)] / #[test], modules de test, tests/, examples/, fuzz/
# (analyse partagée avec compter-unwrap.sh, voir scripts/couverture_securite.py).
#
# Prérequis : cargo install cargo-llvm-cov --locked ; rustup component add llvm-tools-preview
#
# Usage :
#   scripts/couverture-securite.sh                    # toutes les crates, seuil 80 %
#   scripts/couverture-securite.sh --seuil 85         # seuil (ou variable SEUIL_COUVERTURE)
#   scripts/couverture-securite.sh --crate miyauth    # restreint (option répétable)
#   scripts/couverture-securite.sh --detail           # + couverture par fichier
#   scripts/couverture-securite.sh --garder           # conserve le target dédié et le LCOV
#   scripts/couverture-securite.sh --lcov F           # recalcule le tableau depuis un LCOV existant
#                                                     # (+ régions si F.json existe à côté : <F sans .lcov>.json)
#
# Deux tableaux : (1) LIGNES hors tests (indicateur du seuil) ; (2) RÉGIONS llvm par crate,
# informatif, mesure brute de `cargo llvm-cov report --json` sur les fichiers de la crate
# hors tests/, examples/, fuzz/ (les modules #[cfg(test)] internes y restent comptés).
#
# Target dédié : $COUV_TARGET_DIR (défaut /tmp/mcog-cov-target), supprimé à la fin sauf --garder.
# Le target principal du workspace n'est jamais touché.
#
# Tests exclus de la mesure (SAUTES ci-dessous) : ils lancent le sas `portail-sas` réel, confiné
# par seccomp et RLIMIT_FSIZE = 0 ; le binaire instrumenté ne peut pas écrire son profil et est tué
# (FinAnormale) : c'est le confinement qui fonctionne. Ils restent exécutés par `cargo test` (cog-ci).
#
# Code de sortie : 0 tout au seuil ; 1 une crate sous le seuil ; 2 crate introuvable ;
#                  3 échec de compilation ou de tests ; 4 outil manquant.

set -euo pipefail

# --- Crates de sécurité (noms de paquet Cargo) ------------------------------------------
CRATES_SECURITE=(
  miyauth
  miyusceau
  miyuracine
  miyuportail
  miyucdr
  miyudouane
  miyuvigie
  miyufrontiere
  miyuregistre
  miyucanal
  miyufederation
  miyuhybride
  maia-federee
  maia-memoire
  maia-modeles
  miyurelais
  strongfather
  borderguard
  bondingbrother
  worrysentinel
  tamr
  kindmother
  kindmother-service
)
# ----------------------------------------------------------------------------------------

# Tests incompatibles avec l'instrumentation (voir l'en-tête).
SAUTES=(
  chemin_nominal_par_le_sas_reel
  sas_refuse_entree_malformee_ou_surdimensionnee
  deuxieme_validation_divergente_refusee
  descripteur_sans_cloexec_refuse_le_lancement
)

RACINE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SEUIL="${SEUIL_COUVERTURE:-80}"
CIBLE="${COUV_TARGET_DIR:-/tmp/mcog-cov-target}"
GARDER=0
DETAIL=()
LCOV=""
CHOIX=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --seuil) SEUIL="$2"; shift 2 ;;
    --crate) CHOIX+=("$2"); shift 2 ;;
    --garder) GARDER=1; shift ;;
    --detail) DETAIL=(--detail); shift ;;
    --lcov) LCOV="$2"; shift 2 ;;
    -h|--help) sed -n '2,29p' "$0"; exit 0 ;;
    *) echo "option inconnue : $1" >&2; exit 2 ;;
  esac
done

if [[ ${#CHOIX[@]} -gt 0 ]]; then CRATES=("${CHOIX[@]}"); else CRATES=("${CRATES_SECURITE[@]}"); fi
LISTE="$(IFS=,; echo "${CRATES[*]}")"

# Régions llvm par crate (informatif) depuis le résumé JSON de cargo-llvm-cov.
regions() {
  [[ -f "$1" ]] || return 0
  echo
  python3 - "$RACINE" "$1" "$LISTE" <<'PY'
import json, sys
from pathlib import Path
racine = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(racine / "scripts"))
import compter_unwrap as cu  # mêmes fichiers de crate que le tableau des lignes
crates = cu.trouver_crates(racine)
dossiers = {p.resolve() for p in crates.values()}
regions = {}
for d in json.load(open(sys.argv[2], encoding="utf-8"))["data"]:
    for f in d["files"]:
        regions[Path(f["filename"]).resolve()] = f["summary"]["regions"]
print(f"{'crate':<22} {'régions':>8} {'couvertes':>9} {'%':>7}  (llvm, brut)")
print("-" * 64)
gt = gc = 0
for nom in [c for c in sys.argv[3].split(",") if c and c in crates]:
    t = c = 0
    for f in cu.fichiers_crate(crates[nom], dossiers - {crates[nom].resolve()}):
        r = regions.get(f.resolve())
        if r:
            t += r["count"]
            c += r["covered"]
    gt += t
    gc += c
    print(f"{nom:<22} {t:>8} {c:>9} {(100.0 * c / t if t else 0.0):>6.1f}%")
print("-" * 64)
if gt:
    print(f"{'TOTAL':<22} {gt:>8} {gc:>9} {100.0 * gc / gt:>6.1f}%")
PY
}

if [[ -z "$LCOV" ]]; then
  if ! cargo llvm-cov --version >/dev/null 2>&1; then
    echo "cargo-llvm-cov absent : cargo install cargo-llvm-cov --locked" >&2
    exit 4
  fi
  nettoyer() { [[ $GARDER -eq 1 ]] || rm -rf "$CIBLE"; }
  trap nettoyer EXIT
  export CARGO_TARGET_DIR="$CIBLE"
  mkdir -p "$CIBLE"
  LCOV="$CIBLE/couverture-securite.lcov"
  PAQUETS=()
  for c in "${CRATES[@]}"; do PAQUETS+=(-p "$c"); done
  FILTRE=()
  for t in "${SAUTES[@]}"; do FILTRE+=(--skip "$t"); done
  cd "$RACINE"
  echo "Mesure de ${#CRATES[@]} crate(s), target $CIBLE" >&2
  statut=0
  cargo llvm-cov --no-report --locked --no-fail-fast "${PAQUETS[@]}" -- "${FILTRE[@]}" >&2 || statut=$?
  cargo llvm-cov report --lcov --output-path "$LCOV" >&2
  JSON="${LCOV%.lcov}.json"
  cargo llvm-cov report --json --summary-only --output-path "$JSON" >&2
  if [[ $GARDER -eq 1 ]]; then echo "LCOV conservé : $LCOV (résumé JSON : $JSON)" >&2; fi
  if [[ $statut -ne 0 ]]; then
    echo "ÉCHEC : compilation ou tests en échec (code $statut) ; couverture indicative" >&2
  fi
  code=0
  python3 "$RACINE/scripts/couverture_securite.py" "$RACINE" "$LCOV" \
    --crates "$LISTE" --seuil "$SEUIL" "${DETAIL[@]}" || code=$?
  regions "$JSON"
  [[ $statut -ne 0 ]] && exit 3
  exit "$code"
fi

code=0
python3 "$RACINE/scripts/couverture_securite.py" "$RACINE" "$LCOV" \
  --crates "$LISTE" --seuil "$SEUIL" "${DETAIL[@]}" || code=$?
regions "${LCOV%.lcov}.json"
exit "$code"
