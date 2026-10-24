# MIPOWER — index

<!-- @id tools.mipower.index -->
<!-- @role app-reference -->
<!-- @layer S -->
<!-- @human Éditeur et relecteur de bureau des séquences MIP (.mip/sequences) : tableau de bord, rapports, prompt de démarrage -->
<!-- @do reference_mipower -->

- **Crate** : `apps/mipower` (binaire `mipower`, v0.2.0) · **Strate** : S (outil interne ; cité en zone H0 par la [vue globale](../../architecture/COG-VUE-GLOBALE.md) §4, hors catalogue Central — à vérifier)
- **Nom** : « Miyukini Implementation Protocol Oriented Workflow Editor & Reviewer » (`Cargo.toml`)
- **Pile** : Dioxus 0.7 desktop, `pulldown-cmark` + `ammonia`, `arboard` (presse-papier) ; CSS propre (`assets/app.css`), pas de `miyuki-ui-*`
- **Lien TEAM** : TEAM lit aussi `.mip/sequences/` (voir [TEAM](../TEAM/_index.md)) ; aucun appel réseau entre les deux

## 1. Rôle

<!-- @id tools.mipower.role -->
<!-- @role functional-summary -->
<!-- @do summarize_mipower -->

| Vue | Fichier | Fonction |
|---|---|---|
| Dashboard | `views/dashboard.rs` | liste les séquences par classe T1–T5 et statut `active` / `done` / `archived` |
| Report | `views/report.rs` | parcourt les artefacts `.md` par phase (brief, p0–p6…), rendu HTML, édition, clôture |
| Starter | `views/starter.rs` | formulaire → prompt de démarrage (copie presse-papier) ; crée le dossier de séquence depuis `_template` |
| Paramètres | `views/settings.rs` | choix du workspace MIP, régénération de l'index |

- Source des séquences : `.mip/sequences/index.json` (schéma `mip-sequences-index-v2`, `state.rs`, `models.rs`)
- Statut dérivé de `phases/p6-trace.md` (`SUCCES` → done, `REFUSE` / `abandonne` → archived)
- Clôture : ajoute `CLOTUREE -- SUCCES` à `p6-trace.md` puis `rebuild_index` (`views/report.rs`, `init.rs`)
- Initialisation : copie `_template/base` puis deltas `c1`…`cN` selon complexité (`init.rs`)

## 2. Points d'entrée

<!-- @id tools.mipower.api -->
<!-- @role entry-points -->
<!-- @do list_mipower_entry_points -->

- Lancement : `mipower` (fenêtre 1280×820, `main.rs`)
- Racine MIP : `MIP_ROOT` > `%APPDATA%/mipower/mip_root.txt` > dossier courant (`load_persisted_root`)
- Fonctions internes : `md_to_html` (`render.rs`), `init_base`, `init_by_complexity`, `rebuild_index` (`init.rs`), `load_sequences` (`state.rs`)
- Agents autorisés dans Starter : liste blanche de 10 noms (`VALID_AGENTS`, `views/starter.rs`)

## 3. Statut

<!-- @id tools.mipower.statut -->
<!-- @role status-measured -->
<!-- @do report_mipower_status -->

- Membre du workspace (`Cargo.toml`, section « MIPOWER — Workflow Editor ») ; version, édition et lints **propres** (pas `workspace = true`)
- 9 tests unitaires : render 4, init 3, db 2 ; aucun test des vues
- ~1 900 lignes Rust (`apps/mipower/src`)

## 4. Sécurité

<!-- @id tools.mipower.securite -->
<!-- @role security-notes -->
<!-- @do assess_mipower_security -->

- Markdown rendu via `ammonia` avant `dangerous_inner_html` (`render.rs`, `views/report.rs`) ; test `test_script_stripped`
- Aucune interface réseau
- Écriture disque libre sous la racine MIP (édition d'artefacts, création de séquences) ; chemins non canonicalisés (`full_path`, `views/report.rs`)

## 5. Écarts connus

<!-- @id tools.mipower.ecarts -->
<!-- @role known-gaps -->
<!-- @do list_mipower_gaps -->

- Module `db.rs` (SQLite : `sequences`, `artefacts`, `metrics_snapshot`) **jamais appelé** hors tests
- `ProgressInfo`, `PhaseProgress` marqués `#[allow(dead_code)]` (`models.rs`)
- Chemins dépendants de Windows (`APPDATA`) ; comportement Linux non défini (à vérifier)
- `static/app.css` présent mais non référencé (seul `assets/app.css` est inclus)
- Commentaire `SequenceMeta` « Format retourné par /api/sequences » : aucune API HTTP dans MIPOWER
