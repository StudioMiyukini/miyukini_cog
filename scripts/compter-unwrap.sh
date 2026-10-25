#!/usr/bin/env bash
# compter-unwrap.sh — contrainte C-10 (docs/implementation/CONTRAINTES-IMPLEMENTATION.md)
#
# Compte, par crate, les occurrences hors tests de :
#   .unwrap()  .unwrap_err()  .expect(  .expect_err(  panic!  todo!  unimplemented!  unreachable!
# Exclus : modules et items #[cfg(test)] / #[test], modules fichier `#[cfg(test)] mod x;`,
#          dossiers tests/ benches/ examples/ fuzz/ target/, build.rs, commentaires et doc-tests,
#          chaînes littérales, appels `self.expect(...)` (méthodes propres, ex. analyseurs).
# L'analyse lexicale est faite par scripts/compter_unwrap.py (python3, sans dépendance).
#
# Usage :
#   scripts/compter-unwrap.sh                 # tableau trié de toutes les crates non nulles
#   scripts/compter-unwrap.sh --detail        # + chaque occurrence (fichier:ligne)
#   scripts/compter-unwrap.sh --crate miyauth # restreint à une ou plusieurs crates
#
# Code de sortie : 0 si toutes les crates « zéro exigé » sont à zéro,
#                  1 si l'une d'elles contient au moins une occurrence,
#                  2 si une crate de la liste est introuvable.
#
# Le garde-fou de compilation reste l'attribut posé dans chaque crate à zéro :
#   #![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::panic,
#                               clippy::todo, clippy::unimplemented))]

set -euo pipefail

# --- Crates où zéro occurrence est exigée (noms de paquet Cargo) -----------------------
ZERO_EXIGE=(
  # Kernel et Cores
  miyukini-kernel
  strongfather
  kindmother
  kindmother-db-key
  kindmother-service
  kindmother-client
  kindmother-heritage
  kindmother-import
  borderguard
  caringnanny
  masterbutler
  bondingbrother
  everbuddy
  worrysentinel
  tamr
  logisticssteward
  # Toolkits sécurité
  miyauth
  miyusceau
  miyuantispam
  miyuvalidate
  miyuregistre
  miyucanal
  miyuportail
  miyucdr
  miyudouane
  miyuvigie
  miyufrontiere
  # Code exposé au réseau
  miyukini-central
  central-remote
  central-remote-protocol
  miyumarket
  miyukini-admin
  miyualicia-api
  miyuwebway_participant
  miyuwebway_tracker
)
# ----------------------------------------------------------------------------------------

RACINE="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LISTE="$(IFS=,; echo "${ZERO_EXIGE[*]}")"

exec python3 "$RACINE/scripts/compter_unwrap.py" "$RACINE" --zero "$LISTE" "$@"
