# Module MIP — P3 Implementation

> Ce module est charge au debut de P3 (toutes classes).

---

## Agents

**Francois** (back-end) + **Lise** (front-end) en PARALLELE. Denis coordonne les checkpoints.

**Execution par subagent frais** : Chaque tache est executee par un subagent frais pour eviter la pollution de contexte.

---

## Smoke test prioritaire (AVANT le TDD tache par tache)

Denis ecrit un **test d'integration end-to-end** du happy path principal. Ce test DOIT :
- **Compiler** (sinon le plan a un defaut structurel → corriger avant de continuer)
- **Echouer** (la fonctionnalite n'existe pas encore — c'est normal)

```rust
// Exemple smoke test
#[test]
fn smoke_feature_happy_path() {
    // Compile mais echoue (RED) → normal
    let result = FeatureModule::new(Config::default());
    assert!(result.is_ok());
}
```

---

## Pre-flight par tache (avant d'ecrire du code)

1. **Lire la tache** du plan exhaustif (fichier, code attendu, test)
2. **Context7 spot-check** si la tache touche une API externe ou un pattern framework :
   - Verifier le pattern via `query-docs`
   - Ex: composant Dioxus → verifier RSX syntax (`/dioxuslabs/dioxus`)
   - Ex: handler axum → verifier extractors (`/tokio-rs/axum`)
3. **Charger contexte anti-patterns** : MEMORY.md (section "Erreurs a ne pas repeter") + pieges RSX (Lise) ou patterns DB (Francois)

---

## Cycle TDD par tache (10 pas)

| # | Pas | Action |
|---|-------|--------|
| 1 | **START** | Annoter la tache dans le plan : `Demarre a HH:MM` |
| 2 | **RED** | Ecrire le test qui echoue |
| 3 | **GREEN** | Ecrire le code minimal pour que le test passe |
| 4 | **REFACTOR** | Nettoyer si necessaire |
| 5 | **VERIFY** | `cargo test -p {crate}` passe |
| 6 | **LINT** | `cargo clippy -p {crate} -- -D warnings` propre |
| 7 | **COMMIT** | Commit atomique avec message conventionnel |
| 8 | **PUSH** | `git push` sur la feature branch |
| 9 | **LOG** | TodoWrite : marquer la tache `completed` |
| 10 | **TRACK** | Annoter le plan : `Termine a HH:MM avec [model] pour ~N tokens` |

> Les commandes build/test/lint sont lues depuis `.mip/environment.md` (pas hardcodees).

---

## Checkpoint intermediaire (toutes les 5 taches)

Denis lance un mini-audit :

1. `cargo build -p {crate}` des crates modifies
2. `cargo clippy -p {crate} -- -D warnings`
3. Verifier que les taches precedentes ne sont pas cassees
4. **Victor spot-check securite** (si tache touche auth, crypto, validation, secrets) :
   - Grep patterns dangereux : `unwrap()`, URLs en dur, secrets hardcodes, `eval()`, SQL non-parametre
   - Verification algorithmes crypto (table approuves)
   - Verification validation entrees utilisateur
5. **Jean spot-check efficience** : tokens/ligne, fichiers charges inutilement, boucles correction >3 retries. Si anomalie → recommandation (ajustement prompt, changement modele). Autorite CONSULTATIVE.
6. Si regression → corriger avant de continuer
6. `git push` — pousser l'etat courant

---

## Annonce par etape

A chaque etape du plan completee :

```
[YYYY-MM-DD HH:MM] Etape X/<total> — <nom> terminee.
  Taches: X/Y completees | Tests: X passes | Commits: N
  Prochaine etape: <nom>
```

**Comportement selon mode** :
- **FULL** : Annonce informative, execution continue
- **BIG_STEPS** : Informative en P3, gate entre P3→P4
- **GUIDED** : Attendre validation utilisateur (VALIDER / MODIFIER / REVENIR / SAUTER)

---

## Parallelisme

Francois et Lise travaillent simultanement quand leurs taches sont independantes. Les taches avec dependances sont sequencees par Denis.

> Pour T4-T5 : voir `modules/mass.md` pour la parallelisation avancee par DAG et vagues.

---

## Auto-correction

Si un test echoue :

1. Lire le message d'erreur, identifier la cause (root cause analysis)
2. Verifier contre Context7 si probleme de pattern/API
3. Corriger et re-tester (**tentative 1**)
4. Si echec → corriger differemment (**tentative 2**)
5. Si echec → **frein d'urgence** avec diagnostic complet

---

## Gate P3

Chaque tache passe test + clippy.

### Gate BIG_STEPS (P3→P4)

Denis presente un resume :

```
[YYYY-MM-DD HH:MM] Resume P3 — Implementation terminee.
  Etapes: X/X | Taches: X/X | Tests: X passes, Y echoues | Commits: N
  Auto-corrections: N | Lignes ecrites: N
  Continuer vers P4 (Integration & Audit) ?
  [CONTINUER] / [CORRIGER: <instructions>] / [STOPPER]
```
