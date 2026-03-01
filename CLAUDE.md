# Miyukini COG — Conventions Projet

## Definition

**C**ore-**O**rchestrated **G**overnance Environment — environnement de gouvernance orchestre par des Cores.

## Stack technique

- **Langage** : Rust (workspace Cargo)
- **UI** : Dioxus 0.6 desktop (`apps/central/`)
- **DB** : KindMother (SQLite gouverne) — feature flags `legacy-sqlite` / `kindmother-only`
- **API** : REST avec axum, serde JSON
- **LLM** : miou-llm-bridge (proxy LM Studio, 17 agents, skills, contextes)
- **Architecture** : Pyramide COG — Strates 0-9, 8 Cores (Strate 4), Outils (Strate 6), Operateurs (Strate 7)

## Lois d'Autonomie (NON NEGOCIABLES)

1. Aucune dependance externe critique a l'execution
2. Isolement = etat normal
3. Etat local souverain
4. Pas de temps global requis
5. Cout proportionnel au hardware
6. Autonomie n'empeche pas la federation
7. Strate Cores immuable
8. Migration = diplomatie entre environnements

## Regles de code

- `unsafe_code = "forbid"` dans TOUS les Cargo.toml
- Clippy pedantic active (`all = "warn"`, `pedantic = "warn"`)
- Pas de `unwrap()` en production (uniquement dans les tests)
- Types d'erreur explicites par module
- UUIDs v4 pour les IDs primaires
- ISO 8601 pour les timestamps
- Annotations MSCM obligatoires : `@id`, `@do`, `@role`, `@layer`, `@human`

## Pieges RSX Dioxus 0.6

- **Pas de nested braces** dans les format strings RSX — toujours extraire en variable avant `rsx!`
- **Pas de named format args** dans les text nodes RSX
- **Pas de read+set** sur le meme signal dans une expression — lire dans un `let` d'abord

## Commandes

```bash
cargo build --workspace                    # Build complet
cargo test --workspace                     # Tests complets
cargo clippy --workspace -- -D warnings    # Lint complet
cargo test -p {crate} -- --nocapture       # Tests verbose d'un crate
```

## Equipe dev (agents Claude Code)

| Agent | Role | Responsabilite |
|-------|------|----------------|
| **Maria** | Chef de Projet | Analyse requetes, plan projet, suivi |
| **Fabrice** | Analyste PR | Audit concurrence, qualites/defauts, cibles, fonctionnalites, points de friction |
| **Denis** | Chef Dev Senior | Doc technique, coordination, tests finaux, securite |
| **Francois** | Dev Back-End | Implementation Rust, API, DB, tests |
| **Lise** | Dev Front-End | UI/UX Dioxus, atomic design, assets |
| **Arianne** | Team Manager | Qualite, memoire, anti-hallucination, archivage |
| **George** | Audit Expert | Conformite, UX audit, tests globaux |

## Protocole MIP v2 — Miyukini Implementation Protocol

### Classification des taches (AVANT toute action)

| Classe | Critere | Phases | Exemple |
|--------|---------|--------|---------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 → P5 | Fix typo, ajout comment |
| **T2** | Fix cible, 1-3 fichiers | P2 → P3 → P5 | Corriger unwrap(), fix RSX |
| **T3** | Feature moderee, 3-10 fichiers | P1 → P2 → P3 → P5 → P6 | Nouveau composant, endpoint |
| **T4** | Feature majeure, 10+ fichiers | Toutes phases | Nouveau service, refactor |
| **T5** | Chantier strategique | Toutes phases | Nouveau crate/app, engine |

En cas de doute, classer **UN CRAN AU-DESSUS**. Maria classifie.

### Phases et agents

| Phase | Nom | Agents | Gate |
|-------|-----|--------|------|
| P0 | Cadrage & Analyse | Maria + Fabrice | Brief approuve |
| P1 | Specification technique | Denis | Spec validee |
| P2 | Plan d'execution atomique | Denis | Plan valide |
| P3 | Implementation parallele | Francois + Lise | Tests + clippy |
| P4 | Integration & Audit | Denis + George | 0 defaut bloquant |
| P5 | Livraison | Denis | Utilisateur confirme |
| P6 | Archivage & Capitalisation | Arianne | Memoire a jour |

### Workflow standard

```
Utilisateur → Maria (P0 classification + brief) → Fabrice (P0 analyse PR, T4-T5)
→ Denis (P1 spec + P2 plan) → Francois (P3 back) + Lise (P3 front) en PARALLELE
→ Denis (P4 integration) + George (P4 audit) → Denis (P5 livraison)
→ Arianne (P6 archivage) → Utilisateur
```

### Artefacts MIP

- `.mip/briefs/` — Briefs de cadrage (P0)
- `.mip/specs/` — Specs techniques (P1)
- `.mip/plans/` — Plans atomiques (P2)
- `.mip/audits/` — Rapports d'audit (P4)

Skill complet : `.cursor/skills/miyukini-mip-workflow/SKILL.md`

## Structure des crates

### Toolkit (Strate 6)
```
crates/miyu{nom}/src/ → lib.rs, admin_cell.rs, context.rs, errors.rs, {metier}.rs
```

### Service (Strate 7)
```
crates/{service}/src/ → lib.rs, data/ (mod.rs, types.rs, kindmother_db.rs), auth/, services/, export/
```

## Documentation

- Nomenclature : `<PREFIX> - <SUJET> <DETAIL>.<ext>`
- Arborescence : `docs/` → core/, tools/, services/, reference/, miyukini-webway-system/
- Tout document : titre H1, section Contexte, section Portee/Scope

## Skills disponibles (.cursor/skills/)

| Skill | Usage |
|-------|-------|
| miyukini-architecture | Decisions architecturales, strates, Cores |
| miyukini-rust-patterns | Structure crates, admin_cell, context, errors |
| miyukini-mscm-mip | Balisage semantique MSCM, index MSCM (ne pas confondre avec MIP v2) |
| miyukini-services | Pattern services (data/, auth/, adapters) |
| miyukini-dioxus-ui | UI Dioxus 0.6, theme, composants, pieges RSX |
| miyukini-testing | Tests unitaires, cycle MiyukiniSQLtest |
| miyukini-docs | Nomenclature documentation |
| miyukini-kindmother-db | Standards DB KindMother |
| miyukini-mip-workflow | Protocole MIP v2 : classification T1-T5, phases, gates, routing |
