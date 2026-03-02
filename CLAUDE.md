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
9. Anti-serial-collapse : si >3 taches independantes, parallelisation obligatoire

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
| **Denis** | Chef Dev Senior | Doc technique, coordination, tests finaux |
| **Francois** | Dev Back-End | Implementation Rust, API, DB, tests |
| **Lise** | Dev Front-End | UI/UX Dioxus, atomic design, assets |
| **Arianne** | Team Manager | Qualite, memoire, anti-hallucination, archivage |
| **George** | Audit Expert | Conformite, UX audit, tests globaux |
| **Victor** | Expert Cybersecurite | Threat modeling, surfaces d'attaque, audit securite, OWASP |
| **Hugo** | DevOps & Infrastructure | CI/CD, conteneurisation, deploiement, monitoring |

## Protocole MIP v2 — Miyukini Implementation Protocol

MIP v2 est **universel** : il s'adapte a n'importe quel projet, stack, et environnement. La Phase SETUP configure l'environnement une seule fois. Le noyau (classification, phases, gates, agents) est invariant.

### Phase SETUP — Onboarding (UNE SEULE FOIS par environnement)

6 etapes : Detection systeme → Config environnement → Profil utilisateur → Detection outil IA → Dependencies → Agents.
Produit `.mip/environment.md` (configuration maitre). Commande : `/mip_setup` pour reconfigurer.
25+ outils IA supportes : Claude Code, Cursor, Codex CLI, Aider, Copilot, Continue.dev, Cline, JetBrains, Zed, Windsurf, LM Studio, Ollama, etc.
MIP adapte automatiquement ses capacites (parallelisme, TodoWrite, Context7, terminal) a l'outil detecte.

### Classification des taches (AVANT toute action)

| Classe | Critere | Phases | Exemple |
|--------|---------|--------|---------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 → P5 | Fix typo, ajout comment |
| **T2** | Fix cible, 1-3 fichiers | P2 → P3 → P5 | Corriger unwrap(), fix RSX |
| **T3** | Feature moderee, 3-10 fichiers | P0 → P3 → P4 → P5 → P6 | Nouveau composant, endpoint |
| **T4** | Feature majeure, 10+ fichiers | P0 → P3 → P4 → P5 → P6 | Nouveau service, refactor |
| **T5** | Chantier strategique | P0 → P3 → P4 → P5 → P6 | Nouveau crate/app, engine |

En cas de doute, classer **UN CRAN AU-DESSUS**. Maria classifie.

### P0 — Cadrage complet en 10 temps (SEULE phase humaine)

| Temps | Nom | Agent(s) |
|-------|-----|----------|
| 1 | Exploration + Questionnaire brainstorming | Maria |
| 2 | Ideation | Maria + Lise (parallele) |
| 3 | Analyse concurrentielle | Fabrice (T4-T5, parallele Temps 2) |
| 4 | Inventaire prerequis + Evaluation infra | Denis (lead) + Hugo (T4-T5) + Francois + Lise |
| 5 | Analyse de securite | Victor (T3+) |
| 6 | Specification technique + Context7 | Francois |
| 7 | Plan exhaustif + Guide d'implementation | Denis |
| 8 | Audit de faisabilite | Arianne (agents, deps, outils, memoire) |
| 9 | Verification pipeline CI/CD | Hugo (si CI/CD en place) |
| 10 | Synthese & Brief | Maria |

**Gate P0** : Brief approuve + mode d'autonomie choisi (FULL/BIG_STEPS/GUIDED).

### Execution (P3 → P6 — adapte au mode d'autonomie)

| Phase | Nom | Agents | Gate |
|-------|-----|--------|------|
| Git | Creation feature branch + push | Denis | Branch prete |
| P3 | Implementation TDD parallele | Francois + Lise (+ Victor spot-checks) | Tests + clippy + push par tache |
| P4 | Integration, Audit & Securite | Denis + George + Victor + Hugo (T4-T5) | 0 defaut bloquant + score securite conforme |
| P5 | Livraison, Test humain & Validation | Denis + George | Verdict utilisateur (ACCEPTE/REFUSE) |
| P6 | Rapport final, Archivage & Capitalisation | Arianne | Rapport + memoire a jour |

**Git workflow** : Feature branch (`feat/<slug>`) creee au debut, push apres chaque commit, merge vers main apres validation.
**Metriques** : Horodatage + compteurs collectes tout au long dans `.mip/metrics/`. Maria initialise, tous les agents alimentent.
**Frein d'urgence** : L'autopilot s'arrete si bug bloquant apres 2 auto-corrections, ou delta majeur.
**Logging** : Chaque tache tracee via TodoWrite pour suivi utilisateur temps reel.
**Context7** : Verification docs libs (Dioxus, axum, serde) en P0-T4 + spot-checks en P3.
**Brainstorming** : Questionnaire standard en 5 sections (Comprendre/Cadrer/Imaginer/Evaluer/Decider) inspire Design Thinking, Six Thinking Hats, SCAMPER, 5 Whys, HMW, LDJ. Maria administre en Temps 1.
**Inventaire** : Denis + Hugo inventorient competences, connaissances, outils, infra, etapes generales en Temps 4.
**Securite** : Victor analyse surfaces d'attaque et transmet checklist securite a Francois en Temps 5. Audit securite /100 en P4.
**Infrastructure** : Hugo evalue l'infra en Temps 4, verifie CI/CD en Temps 9, et verifie le deploiement en P4 (T4-T5).
**Annonces** : Chaque Temps P0 et etape macro P3 annonces dans le chat avec date/heure. TodoWrite suit la progression.
**Mode autonomie** : FULL (autopilot complet), BIG_STEPS (gates P3→P4, P4→P5), GUIDED (gate par etape macro). Persistance dans `memory/user-profile.md`. Changeable via `/autonomy_mode`.
**Smoke test** : Test e2e happy path compile-mais-echoue AVANT le TDD tache par tache (valide la structure du plan).
**Token efficiency** : Fichiers memoire pre-indexes par agent. TL;DR 5 lignes sur chaque artefact.
**Checkpoints** : Mini-audit Denis toutes les 5 taches en P3.
**Boucle MIP** : Si refus P5, retour en P0 avec feedback utilisateur (increment `mip_loops`).
**Rapport final** : Notes /20 sur 8 criteres, resume dev, profil utilisateur, capitalisation agents.
**Phase SETUP** : Onboarding universel en 6 etapes. Scan systeme + config environnement + profil utilisateur + detection outil IA + dependencies + agents. Produit `.mip/environment.md`. Run once.
**Universalite** : Noyau MIP invariant (classification, phases, gates, TDD, metriques). Config projet dans `.mip/environment.md`. Commandes adaptees a la stack detectee.
**MASS (Agent Swarm)** : Parallelisation par DAG de dependances et vagues d'execution. 3 modes : subagent burst (T2-T3), worktree swarm (T4), team swarm (T5). Merge coordination par Denis. DAG dans `.mip/dags/`. Metriques swarm dans `.mip/metrics/`. Loi 9 : >3 taches independantes -> parallelisation obligatoire.

### Workflow standard

```
Utilisateur → Maria (P0 : 10 temps + init metriques) + Lise + Fabrice + Denis+Hugo (inventaire+infra) + Victor (securite) + Francois (Context7) + Denis (plan) + Arianne (audit) + Hugo (CI/CD)
→ [GATE] Brief approuve + mode autonomie (FULL/BIG_STEPS/GUIDED)
→ === EXECUTION (mode choisi) ===
→ Git : checkout -b feat/<slug> + push -u origin
→ Denis : Smoke test e2e (compile mais echoue → valide structure)
→ Francois (P3 back) + Lise (P3 front) en PARALLELE [TDD + commit + push + metriques + TodoWrite]
→ Denis (P3 checkpoint /5 taches + Victor spot-check securite + push)
→ Denis (P4 integration) + George (P4 audit) + Victor (P4 audit securite /100) + Hugo (P4 verif deploiement T4-T5)
→ Denis (P5 push final + resume + instructions test) → [Utilisateur teste]
→ Denis (P5 questionnaire satisfaction) → [GATE] Verdict utilisateur
→   Si ACCEPTE : Denis (P5 merge main + push + tag + nettoyage branche)
→   Si REFUSE : Maria (retour P0 avec feedback, boucle MIP)
→ Arianne (P6 rapport final /20 + score securite + archivage + capitalisation)
```

### Artefacts MIP

- `.mip/environment.md` — Configuration maitre de l'environnement (Phase SETUP, run once)
- `.mip/briefs/` — Briefs de cadrage (P0 Temps 8)
- `.mip/specs/` — Specs techniques (P0 Temps 5)
- `.mip/plans/` — Plans exhaustifs + guides d'implementation (P0 Temps 6)
- `.mip/audits/` — Rapports d'audit (P4)
- `.mip/metrics/` — Metriques et horodatage (collecte continue)
- `.mip/reports/` — Rapports finaux de developpement (P6)
- `.mip/dags/` — DAG de dependances pour parallelisation swarm (P3)

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
