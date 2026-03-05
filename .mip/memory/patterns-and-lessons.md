<!-- @id mem.patterns.lessons
     @do provide_patterns_errors_antipatterns
     @role patterns
     @layer memory
     @human Patterns, erreurs et anti-patterns — charger avant P3 -->

# Patterns, Leçons & Anti-Patterns

> Fichier de référence unifié. Charger par les agents avant chaque sprint (P3).

## Patterns confirmés

- **spawn_blocking pour SQLite dans async** : `MarketStore::open` utilise `blocking_lock()`, qui panic dans un runtime tokio. Toujours wrapper dans `tokio::task::spawn_blocking`.
- **`#[serde(default)]` pour retrocompatibilite** : Tout nouveau champ dans une struct serialisee doit porter `#[serde(default)]` ou `#[serde(default = "fn")]` pour ne pas casser la deserialisation de donnees existantes.
- **Extraction variables avant RSX** : Styles conditionnels, labels, couleurs doivent etre dans des `let` avant le bloc `rsx!{}`.
- **Service embarque vs externe** : MiyuVoice est embarque dans Central (necessite acces natif permanent). Tous les autres services sont des binaires independants lances par Central.
- **Themes const pour UI tokens** : Les themes sont des `const UiTheme` resolus a la compilation. Zero overhead runtime, zero allocation. Le feature flag `serde` est optionnel pour serialisation.
- **Path relatif pour cross-workspace** : Pour partager un crate entre workspaces separes (COG/MGE), utiliser `path = "../../crates/nom"` dans Cargo.toml.
- **Dual server pattern (API + web)** : Un binaire lance deux serveurs axum (API localhost HTTP + web expose HTTPS) partageant un `Arc<AppState>`. Pattern reproductible pour services exposes au reseau.
- **`serde(alias)` pour reconciliation front/back** : Quand les noms de champs divergent entre crate et UI, `#[serde(alias = "...")]` evite de casser l'API.
- **Canary pattern pour verification passphrase** : Chiffrer une valeur connue avec la master key derivee, puis verifier au redemarrage. Evite de stocker la passphrase.
- **Isolation fichier dans les vagues MASS** : Dans une vague parallele, deux agents ne PEUVENT PAS toucher le meme fichier. Si chevauchement, Denis reordonnance. Parallelisme maximal quand fichiers disjoints.
- **MSCM prefixe `mas.*` pour sections swarm** : Section principale `mas`, sous-sections `mas.architecture`, `mas.dag-format`, etc. Agents : `mas.agent.<nom>`. Couverture 100% MSCM sur le chantier MASS.

## Erreurs a ne pas repeter

- **AP-10 : URL externe en dur dans le code source** : Utiliser configs ou variables d'environnement. L'ancien tunnel Cloudflare dans `llm_client.rs` etait une violation de LOI-1.
- **AP-11 : `unwrap()` en production** : 7 occurrences identifiees dans `apps/central/src/` (audio.rs, miou/engine.rs, jaymanga/, jayfestival/) a corriger.
- **AP-12 : Commits WIP sur main** : Utiliser des branches de feature pour les gros refactors.
- **AP-13 : Passphrase par defaut hardcodee** : Refuser le demarrage ou generer un secret aleatoire au premier lancement.
- **AP-14 : Comparaison de secrets avec `Iterator::all()`** : Court-circuite. Utiliser `subtle::ConstantTimeEq` ou accumulateur XOR.
- **AP-15 : HTML non echappe dans les templates web** : Les noms de fichiers sont du contenu utilisateur non fiable. XSS possible sans echappement.

## Anti-Patterns (ex mip-antipatterns.md)

### Worktrees & Branches

- **AP-01 : Worktree cree depuis la mauvaise base** (Mar 2026, MGE Render Reforge S4)
  Worktree cree depuis `main` au lieu de la feature branch. Diff de -29000 lignes.
  **Fix** : Toujours `git log --oneline -3` dans le worktree AVANT merge. Si base mauvaise, extraire fichier par fichier.

- **AP-02 : Semantique stash pop inversee** (Mar 2026)
  `--ours` = HEAD, `--theirs` = stash. Inverse de l'intuition.
  **Fix** : En conflit stash pop, utiliser `git show HEAD:<file>` pour la version correcte.

### Cross-Sprint Compatibilite

- **AP-03 : Variants enum non-exhaustifs apres merge** (Mar 2026)
  Nouveaux variants `GuiAction` non matches apres merge multi-sprints.
  **Fix** : Apres tout merge ou stash pop, toujours `cargo check` immediatement.

- **AP-04 : Agent reecrit au lieu d'integrer** (Mar 2026, S4)
  Agent a cree un fichier neuf au lieu d'integrer dans l'existant (perte du systeme FSM).
  **Fix** : Prompt doit mentionner "integrer dans le fichier existant" et lister les elements a preserver.

### Clippy

- **AP-05 : match_same_arms** (recurrent)
  Match arms identiques doivent etre combines avec `|`. Clippy pedantic le detecte.

### Code & Architecture

- **AP-06** : Fusionne avec AP-11 (voir section "Erreurs a ne pas repeter" ci-dessus)

- **AP-07 : Feature flags soustractifs** (confirmé MGE Render Reforge)
  Les feature flags doivent etre additifs. `legacy-batcher` active l'ancien en plus du nouveau. `index-u32` active u32 en plus du u16.
  **Fix** : Jamais de feature flag qui desactive un comportement par defaut.

### Swarm & Parallelisme

- **AP-08 : Taches paralleles sur le meme fichier** (Mar 2026, MASS)
  4 taches paralleles sur le meme fichier = parallelisme effectif de 1.
  **Fix** : Regrouper en 1 tache monolithique OU placer dans la meme vague avec 1 seul agent.

- **AP-09 : References LOI-N non propagees apres ajout de loi** (Mar 2026, MASS)
  6 occurrences de "LOI-1 a LOI-8" restees non mises a jour apres ajout de Loi 9.
  **Fix** : Apres ajout/suppression d'une Loi, grep `LOI-.*LOI-[0-9]` dans tout le workspace.
