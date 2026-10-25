#!/usr/bin/env bash
# @id     script_verifier_regle_rusqlite
# @do     verifier_rusqlite_reserve_a_kindmother_service
# @role   securite
# @layer  outillage
# @human  Règle de dépôt (lot C, K-13, C-32) : `rusqlite` n'est permis que dans
#         kindmother-service, seul écrivain des données, plus une liste transitoire
#         explicite et commentée (constat, lot de retrait).
#
# Usage : scripts/verifier-regle-rusqlite.sh [racine]   (défaut : racine du dépôt)
# Sortie : 0 si conforme ; 1 si un Cargo.toml hors liste déclare rusqlite, ou si une
#          entrée de la liste n'en déclare plus (la liste doit rétrécir avec les retraits).
#
# Toute déclaration compte : [dependencies], [dev-dependencies], [build-dependencies],
# dépendances de cible, dépendances optionnelles (features `legacy-sqlite`), forme
# `rusqlite = …`, `rusqlite.workspace = true` ou `[dependencies.rusqlite]`.
# Les dépendances *indirectes* (ex. dev-dependency sur kindmother-service pour les tests
# d'intégration) ne sont pas visées : seul kindmother-service ouvre les fichiers.

set -euo pipefail

racine="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
cd "$racine"

# Seul écrivain autorisé.
AUTORISE="crates/kindmother-service"

# Liste transitoire : chemin|constat|lot de retrait|motif.
# Chaque entrée DOIT disparaître au lot indiqué ; le script échoue si une entrée ne
# déclare plus rusqlite (la retirer alors de cette liste).
TRANSITOIRE=(
  # Services COG passés à KindMother par défaut ; accès SQLite direct gardé derrière la
  # feature non défaut `legacy-sqlite` le temps d'importer les données existantes.
  "crates/jaykonta|K-13|lot C (suite)|feature legacy-sqlite non défaut ; import : kindmother-import jaykonta"
  "crates/jay1tribu|K-13|lot C (suite)|feature legacy-sqlite non défaut ; jayclub l'active encore"
  "crates/miyukiniwatch|K-13|lot C (suite)|feature legacy-sqlite non défaut ; import : kindmother-import miyukiniwatch"
  "crates/miyukini-central|K-13|lot C (suite)|feature legacy-sqlite ; apps/central et central-remote l'activent encore"
  # Outil d'import des bases héritées : lecture seule (SQLITE_OPEN_READ_ONLY), fichier jamais
  # modifié ; les services l'emploient par leur feature non défaut `import-heritage`.
  "crates/kindmother-heritage|K-13|lot C (suite)|import des bases héritées (kindmother-import) ; retiré une fois les bases importées"
  # Services sans chemin KindMother complet (SQLite direct par défaut).
  "crates/jaymanga|K-13|lot C (suite)|chemin kindmother-only à écrire (client = placeholder)"
  "crates/jaybureau-suite/jayclub|K-13|lot C (suite)|refonte de Jay1Tribu, persistance directe"
  "crates/miyucloud|K-13|lot 5|MiyuCloud migré après le premier service H2 (plan, lot 5)"
  "crates/miyucloud-dav|K-13|lot 5|suit MiyuCloud"
  # Hors zone de données COG ou outillage : à revoir (stockage propre ou KindMother).
  "apps/origin|K-13|lot 6|serveur Origin (hors COG) ; stockage propre à isoler avec la fédération"
  "apps/team|K-13|lot 8|outil d'équipe hors zone COG"
  "apps/mipower|K-13|lot 8|éditeur MIP (outillage local)"
  "apps/alicia-mvp|K-13|lot 7|banc d'essai Alicia"
)

declare -A attendu=()
for entree in "${TRANSITOIRE[@]}"; do
  attendu["${entree%%|*}"]="$entree"
done

# Manifestes du dépôt : suivis ou nouveaux non ignorés (git), sinon recherche hors
# dossiers cachés (.git, .claude/worktrees…), target/ et node_modules/.
lister_manifestes() {
  if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    git ls-files -z --cached --others --exclude-standard -- 'Cargo.toml' '*/Cargo.toml' \
      | while IFS= read -r -d '' f; do [[ -f "$f" ]] && printf './%s\0' "$f"; done
  else
    find . \( -path './.*' -o -name target -o -name node_modules \) -prune -o \
      -name Cargo.toml -print0
  fi
}

# Cargo.toml déclarant rusqlite.
mapfile -t declarants < <(
  lister_manifestes \
    | xargs -0 grep -lE '^[[:space:]]*rusqlite[[:space:]]*(=|\.)|^\[([a-z0-9_.-]+\.)?dependencies\.rusqlite\]' \
    | sed -e 's|^\./||' -e 's|/\?Cargo\.toml$||' -e 's|^$|.|' \
    | sort
)

echec=0
declare -A vus=()
for chemin in "${declarants[@]}"; do
  vus["$chemin"]=1
  if [[ "$chemin" == "$AUTORISE" ]]; then
    continue
  fi
  if [[ -n "${attendu[$chemin]:-}" ]]; then
    IFS='|' read -r _ constat lot motif <<<"${attendu[$chemin]}"
    echo "transitoire : $chemin ($constat, retrait $lot) — $motif"
    continue
  fi
  echo "ERREUR : $chemin/Cargo.toml déclare rusqlite. Seul $AUTORISE ouvre les bases (K-13, C-32) :" >&2
  echo "         passer par kindmother-client (intentions typées), ou ajouter une entrée" >&2
  echo "         transitoire justifiée (constat, lot de retrait) dans $0." >&2
  echec=1
done

for chemin in "${!attendu[@]}"; do
  if [[ -z "${vus[$chemin]:-}" ]]; then
    echo "ERREUR : $chemin figure dans la liste transitoire mais ne déclare plus rusqlite : retirer l'entrée." >&2
    echec=1
  fi
done

if [[ -z "${vus[$AUTORISE]:-}" ]]; then
  echo "ERREUR : $AUTORISE ne déclare plus rusqlite (règle à revoir)." >&2
  echec=1
fi

if [[ "$echec" -ne 0 ]]; then
  exit 1
fi
echo "règle rusqlite : conforme (${#declarants[@]} déclarants, dont ${#TRANSITOIRE[@]} transitoires)."
