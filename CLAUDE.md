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
| **Jean** | Responsable Efficience IA | Prompt engineering, comptage tokens, recommandation modeles, detection fuites tokens |

## Protocole MIP v2 — Miyukini Implementation Protocol

MIP v2 est **universel** : il s'adapte a n'importe quel projet, stack, et environnement. La Phase SETUP configure l'environnement une seule fois. Le noyau (classification, phases, gates, agents) est invariant.

### Phase SETUP — Onboarding (UNE SEULE FOIS par environnement)

SETUP-1 a SETUP-6 : Detection systeme → Config environnement → Profil utilisateur → Detection outil IA → Dependencies → Agents.
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
| 4 | Inventaire prerequis + Evaluation infra + Modeles | Denis (lead) + Hugo (T4-T5) + Jean + Francois + Lise |
| 5 | Analyse de securite | Victor (T3+) |
| 6 | Specification technique + Context7 | Francois |
| 7 | Plan exhaustif + Guide d'implementation | Denis |
| 8 | Audit de faisabilite + Validation efficience | Arianne (agents, deps, outils, memoire) + Jean |
| 9 | Verification pipeline CI/CD | Hugo (si CI/CD en place) |
| 10 | Synthese & Brief | Maria |

**Gate P0** : Brief ecrit en fichier → resume en chat (TL;DR + approches + risques) → AskUserQuestion approbation → SI APPROUVE → AskUserQuestion mode autonomie. 2 questions separees, jamais 1. Choix eclaire obligatoire (invariant I-4).

### Execution (P3 → P6 — adapte au mode d'autonomie)

| Phase | Nom | Agents | Gate |
|-------|-----|--------|------|
| Git | Creation feature branch + push | Denis | Branch prete |
| P3 | Implementation TDD parallele | Francois + Lise (+ Victor spot-checks + Jean spot-checks) | Tests + clippy + push par tache |
| P4 | Integration, Audit & Securite & Efficience | Denis + George + Victor + Hugo (T4-T5) + Jean | 0 defaut bloquant + score securite conforme |
| P5 | Livraison, Test humain & Validation | Denis + George | Verdict utilisateur (ACCEPTE/REFUSE) |
| P6 | Rapport final, Archivage & Capitalisation | Arianne + Jean (refactorisation memoire) | Rapport + memoire a jour |

**Git workflow** : Feature branch (`feat/<slug>`) creee au debut, push apres chaque commit, merge vers main apres validation.
**Metriques** : Horodatage + compteurs collectes tout au long dans `.mip/metrics/`. Maria initialise, tous les agents alimentent. **Valeurs mesurees uniquement** (task-notifications + timestamps filesystem). Aucune estimation dans le rapport P6.
**Frein d'urgence** : L'autopilot s'arrete si bug bloquant apres 2 auto-corrections, ou delta majeur.
**Logging** : Chaque tache tracee via TodoWrite pour suivi utilisateur temps reel.
**Context7** : Verification docs libs (Dioxus, axum, serde) en P0-T4 + spot-checks en P3.
**Brainstorming** : Questionnaire standard en 5 sections (Comprendre/Cadrer/Imaginer/Evaluer/Decider). Maria administre en Temps 1 via AskUserQuestion, **section par section** (1 appel = 1 section, max 4 questions). Jamais de texte libre pour poser des questions.
**Inventaire** : Denis + Hugo inventorient competences, connaissances, outils, infra, etapes generales en Temps 4.
**Securite** : Victor analyse surfaces d'attaque et transmet checklist securite a Francois en Temps 5. Audit securite /100 en P4.
**Infrastructure** : Hugo evalue l'infra en Temps 4, verifie CI/CD en Temps 9, et verifie le deploiement en P4 (T4-T5).
**Annonces** : Chaque Temps P0 et etape P3 annonces dans le chat avec date/heure. TodoWrite suit la progression.
**Mode autonomie** : FULL (autopilot complet), BIG_STEPS (gates P3→P4, P4→P5), GUIDED (gate par etape). Choix toujours APRES lecture du brief (invariant I-4). Persistance dans `memory/user-profile.md`. Changeable via `/autonomy_mode`.
**Smoke test** : Test e2e happy path compile-mais-echoue AVANT le TDD tache par tache (valide la structure du plan).
**Token efficiency** : Fichiers memoire pre-indexes par agent. TL;DR 5 lignes sur chaque artefact.
**Checkpoints** : Mini-audit Denis toutes les 5 taches en P3.
**Boucle MIP** : Si refus P5, retour en P0 avec feedback utilisateur (increment `mip_loops`).
**Plan = document de suivi** : Chaque agent annote sa tache dans le plan a la completion : `Demarre a HH:MM:SS. Termine a HH:MM:SS avec [model] pour N tokens (mesures).` ZERO estimation — valeurs extraites des task-notifications. Le plan annote est la source de verite pour le rapport P6.
**Rapport final** : Notes /20 sur 8 criteres, trace d'execution extraite du plan annote, metriques tokens/duree, resume dev, profil utilisateur, capitalisation agents. Un rapport sans trace est INCOMPLET.
**Metriques consommation** : Tokens agreges depuis les task-notifications (valeurs reelles), duree totale (premier prompt → P6, timestamps filesystem), duree effective (hors attentes), indicateurs efficacite (tokens/ligne, lignes/heure, taches/heure). T0 horodate par Maria des P0. **ZERO estimation dans P6** — `~`, `environ`, `approximation` interdits dans les sections metriques.
**Phase SETUP** : Onboarding universel en SETUP-1 a SETUP-6. Scan systeme + config environnement + profil utilisateur + detection outil IA + dependencies + agents. Produit `.mip/environment.md`. Run once.
**Universalite** : 14 invariants MIP (I-1 a I-14) definissent le noyau immuable : classification, phases, gates, TDD, metriques mesurees, brief lu avant choix autonomie, documents 400l max. Config projet dans `.mip/environment.md`. Commandes adaptees a la stack detectee. Detail : SKILL.md section "Invariants MIP".
**MASS (Agent Swarm)** : Parallelisation par DAG de dependances et vagues d'execution. 3 modes : subagent burst (T2-T3), worktree swarm (T4), team swarm (T5). Merge coordination par Denis. DAG dans `.mip/dags/`. Metriques swarm dans `.mip/metrics/`. Loi 9 : >3 taches independantes -> parallelisation obligatoire.
**Documents modulaires** : Tout artefact MIP limite a 400 lignes max. Au-dela, decouper en index + modules annexes. Refuser les documents monolithiques.

### Workflow standard

```
Utilisateur → Maria (P0 : 10 temps + init metriques) + Lise + Fabrice + Denis+Hugo (inventaire+infra) + Jean (modeles) + Victor (securite) + Francois (Context7) + Denis (plan) + Arianne+Jean (audit+efficience) + Hugo (CI/CD)
→ [GATE] Brief approuve + mode autonomie (FULL/BIG_STEPS/GUIDED)
→ === EXECUTION (mode choisi) ===
→ Git : checkout -b feat/<slug> + push -u origin
→ Denis : Smoke test e2e (compile mais echoue → valide structure)
→ Francois (P3 back) + Lise (P3 front) en PARALLELE [TDD + commit + push + metriques + TodoWrite]
→ Denis (P3 checkpoint /5 taches + Victor spot-check securite + Jean spot-check efficience + push)
→ Denis (P4 integration) + George (P4 audit) + Victor (P4 audit securite /100) + Hugo (P4 verif deploiement T4-T5) + Jean (P4 audit efficience tokens)
→ Denis (P5 push final + resume + instructions test) → [Utilisateur teste]
→ Denis (P5 questionnaire satisfaction) → [GATE] Verdict utilisateur
→   Si ACCEPTE : Denis (P5 merge main + push + tag + nettoyage branche)
→   Si REFUSE : Maria (retour P0 avec feedback, boucle MIP)
→ Arianne + Jean (P6 rapport final /20 + score securite + archivage + capitalisation + refactorisation memoire)
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
