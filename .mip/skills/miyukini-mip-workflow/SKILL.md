# Skill: MIP v2 — Miyukini Implementation Protocol

## Quand utiliser ce skill

Utiliser ce skill pour **toute demande de developpement** impliquant du code, une nouvelle fonctionnalite, un fix, un refactor, ou un nouveau crate/service. Le protocole MIP v2 orchestre l'equipe et structure le travail.

MIP v2 est **universel** : il s'adapte a n'importe quel projet, stack, et environnement. Le noyau du protocole (classification, phases, gates, agents) est invariant. Seule la **configuration projet** change via la Phase SETUP.

---

## Nomenclature (OBLIGATOIRE — terminologie canonique)

La hierarchie suivante est **stricte**. Chaque terme designe un seul niveau. Ne PAS utiliser un terme pour un autre niveau.

```
Sequence MIP
  └─ Phase (P0, Git, P3, P4, P5, P6)
       └─ Temps (P0 uniquement : Temps 1 a 10, invariants)
       │    └─ Tache (1 tache = 1 agent traitant)
       └─ Etape (P3 uniquement : groupes du plan Denis)
       │    └─ Tache (1 tache = 1 agent, MASS ou sequentiel selon l'etape)
       └─ Volet (P4, P5, P6 : blocs proceduraux internes)
            └─ Tache (1 tache = 1 agent traitant)
```

| Terme | Niveau | Scope | Definition |
|-------|--------|-------|-----------|
| **Sequence** | 0 | Cycle entier | Un cycle MIP complet (P0 → P6). `mip_sequence_number` dans les metriques. |
| **Phase** | 1 | Grandes divisions | P0, Git, P3, P4, P5, P6. Invariantes (I-2). |
| **Temps** | 2 | P0 uniquement | Temps 1 a 10. Invariants, non reordonnables. Carte de synchronisation R-P0-4. |
| **Etape** | 3 | P3 uniquement | Groupes de taches du plan Denis. Chaque etape a : prerequis, agents, livrables, critere de completion. Mode GUIDED = gate par etape. |
| **Volet** | 3 | P4, P5, P6 | Blocs proceduraux internes a une phase. Ex : P5 Volet 1 "Presentation livrable". |
| **Tache** | 4 | Partout | Unite atomique. 1 tache = 1 agent traitant. `[CODE-01]`, `[TEST-U-01]`. MASS ou sequentiel selon le contexte de l'etape/volet parent. |

**Regles** :
- "Etape" est **RESERVE** au plan P3. Ne JAMAIS utiliser "etape" pour les subdivisions de P0 (= Temps), P5/P6 (= Volet), ou SETUP (= SETUP-1 a SETUP-6).
- "Temps" est **RESERVE** a P0. Ne JAMAIS utiliser "Temps" hors de P0.
- "Sequence" designe le cycle MIP entier. Ne PAS utiliser "sequence" pour designer une serie ordonnee d'actions internes (utiliser "deroulement" ou "processus" a la place).
- Le cycle TDD (RED, GREEN, REFACTOR...) = **pas** du cycle TDD (niveau infra-tache, pas un terme hierarchique).

---

## Architecture modulaire — Chargement a la demande

Ce skill est **modulaire**. Ce fichier contient le noyau du protocole. Les details de chaque phase sont dans des modules separes charges **A LA DEMANDE** pour economiser la fenetre de contexte.

| Phase | Module a charger (Read) | Declencheur |
|-------|------------------------|-------------|
| SETUP | `.mip/modules/setup.md` | `.mip/environment.md` absent ou `/mip_setup` |
| P0 | `.mip/modules/p0-details.md` | Debut de P0 (T3+) |
| P3 | `.mip/modules/p3-execution.md` | Debut de P3 |
| P4-P6 | `.mip/modules/p4-p5-p6.md` | Debut de P4 |
| Metriques | `.mip/modules/metrics.md` | Init metriques (debut sequence) |
| MASS | `.mip/modules/mass.md` | T4-T5 avec parallelisation |
| Ref. outils IA | `.mip/modules/tools-reference.md` | Sur demande explicite |

**Instruction** : Lire le module avec l'outil Read AVANT de commencer la phase correspondante. Ne PAS charger tous les modules au debut — charger uniquement le module de la phase en cours. Chemin canonique : `.mip/modules/<fichier>`.

**Tables de reference partagees** : `.mip/protocol/conventions.md` (classification T1-T5, equipe, artefacts, workflow). Ne pas les dupliquer ici.

---

## Phase SETUP — Onboarding Universel (UNE SEULE FOIS)

> Detail complet : `.mip/modules/setup.md`

La Phase SETUP s'execute **une seule fois** lors de la premiere utilisation de MIP dans un nouvel environnement. Elle produit `.mip/environment.md` (configuration maitre). Si ce fichier existe deja, la phase est **sautee**.

**Declenchement** : `.mip/environment.md` n'existe pas, ou `/mip_setup`, ou demande de reconfiguration.

**SETUP-1 a SETUP-6** : Detection systeme → Config environnement (19q) → Profil utilisateur (8q) → Detection outil IA → Dependencies → Agents.

**Commandes** : `/mip_setup` (relancer), `/mip_status` (afficher statut), `/autonomy_mode <mode>` (changer mode).

---

## Invariants MIP (NOYAU IMMUABLE)

Les elements suivants sont **invariants** — ils s'appliquent quel que soit le projet, la stack, l'environnement, ou l'outil IA :

| # | Invariant | Portee |
|---|-----------|--------|
| I-1 | **Classification T1-T5 avant toute action** | Maria classifie, doute = un cran au-dessus |
| I-2 | **Sequence de phases** : P0 → Git → P3 → P4 → P5 → P6 (T3+) | Pas de saut de phase, pas de reordonnancement |
| I-3 | **P0 = seule phase humaine** | Aucun code avant brief approuve |
| I-4 | **Brief lu par l'utilisateur AVANT choix d'autonomie** | L'utilisateur ne peut pas choisir FULL/BIG_STEPS/GUIDED sans avoir lu le brief — choix eclaire obligatoire |
| I-5 | **Hard gates entre phases** | Chaque gate a des criteres explicites, pas de passage sans validation |
| I-6 | **TDD obligatoire en P3** | Cycle RED → GREEN → REFACTOR → VERIFY → LINT → COMMIT |
| I-7 | **Metriques mesurees, jamais estimees** | Sources : task-notifications, filesystem timestamps, comptages. Aucune approximation dans le rapport P6 |
| I-8 | **P5 = test humain obligatoire** | L'utilisateur teste le livrable et rend un verdict (ACCEPTE/REFUSE) |
| I-9 | **9 Lois d'Autonomie** | Non negociables, applicables a tout le code produit |
| I-10 | **Roles agents fixes** | Chaque agent a un role, des competences et un perimetre definis |
| I-11 | **Feature branch workflow** | `feat/<slug>`, merge --no-ff vers main apres P5 |
| I-12 | **Artefacts structures par sequence** | `<sequence>/` contient briefs/, specs/, gpi/, phases/, plans_p3/, audits/, metrics/, rapports_finaux/, ressources/ |
| I-13 | **Frein d'urgence** | Arret automatique si bug bloquant apres 2 tentatives ou delta majeur |
| I-14 | **Documents modulaires, 400 lignes max** | Tout artefact decoupe si depassement ; volet optimisation P4/P6 si depassement |
| I-15 | **Boucle MIP bornee** | Comptage `mip_loops` ; apres 10 iterations, suggerer de reduire le scope |

> **Ce qui varie** : la stack technique, les commandes build/test/lint, les libs, la config CI/CD, le nombre d'agents mobilises par classe. Tout cela est configure dans `.mip/environment.md` (Phase SETUP).

---

## Classification (OBLIGATOIRE — AVANT toute action)

> Table de reference : `.mip/protocol/conventions.md`, section "Classification des taches".

| Classe | Critere | Phases |
|--------|---------|--------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 → P5 |
| **T2** | Fix cible, 1-3 fichiers | P2 → P3 → P5 |
| **T3** | Feature moderee, 3-10 fichiers | P0 → P3 → P4 → P5 → P6 |
| **T4** | Feature majeure, 10+ fichiers | P0 → P3 → P4 → P5 → P6 |
| **T5** | Chantier strategique | P0 → P3 → P4 → P5 → P6 |

**Qui classifie** : Maria. En cas de doute, classer **UN CRAN AU-DESSUS**.

---

## P0 — Cadrage complet en 10 temps (T3+, SEULE phase humaine)

> Detail de chaque Temps : `.mip/modules/p0-details.md`

**Agents** : Maria (lead) + Lise + Fabrice (T4-T5) + Denis + Hugo (T4-T5) + Jean + Victor (T3+) + Francois + Arianne

P0 est **LA phase humaine** : elle determine la direction. Aucun code avant la fin de P0. Apres approbation du brief, **tout est automatique** (P3 → P6).

**Regles de bornage P0** (detail dans `.mip/modules/p0-details.md` section "Regles de presentation P0") :
- **R-P0-1** : AskUserQuestion obligatoire pour toute question a l'utilisateur (pas de texte libre). Questionnaire section par section.
- **R-P0-2** : Annonce visible dans le chat apres chaque Temps (3-5 lignes).
- **R-P0-3** : Brief presente en chat (TL;DR + approches + risques + lien fichier) PUIS 2 AskUserQuestion separes (approbation, puis autonomie).
- **R-P0-4** : Carte de synchronisation des Temps — dependances explicites, pas de Temps lance avant ses prerequis.
- **R-P0-5** : Allegements par classe (T3 vs T4 vs T5) — Temps sautes annonces explicitement.

| Temps | Nom | Agent(s) | Module detail |
|-------|-----|----------|---------------|
| 1 | Exploration + Brainstorming structure | Maria | p0-details.md |
| 2 | Ideation | Maria + Lise (parallele) | p0-details.md |
| 3 | Analyse concurrentielle | Fabrice (T4-T5, parallele T2) | p0-details.md |
| 4 | Inventaire prerequis + Infra + Modeles | Denis (lead) + Hugo (T4-T5) + Jean + Francois + Lise | p0-details.md |
| 5 | Analyse de securite | Victor (T3+) | p0-details.md |
| 6 | Specification technique + Context7 | Francois | p0-details.md |
| 7 | Plan exhaustif + Guide implementation | Denis | p0-details.md |
| 8 | Audit de faisabilite + Efficience | Arianne + Jean | p0-details.md |
| 9 | Verification pipeline CI/CD | Hugo (si CI/CD en place) | p0-details.md |
| 10 | Synthese & Brief | Maria | p0-details.md |

**Gate P0** (deroulement strict) :
1. Maria presente le brief complet a l'utilisateur
2. L'utilisateur **lit le brief** (TL;DR + contenu)
3. L'utilisateur **approuve ou rejette** le brief (approche, perimetre, plan)
4. **SI approuve** : l'utilisateur choisit le mode d'autonomie (FULL/BIG_STEPS/GUIDED) — **choix eclaire** apres lecture du brief (invariant I-4)
5. Execution demarre

**Hard gate** : AUCUN passage en execution sans brief approuve. AUCUN choix d'autonomie sans lecture prealable du brief.

### Suivi P0 — Annonces temps reel

Chaque Temps est trace via TodoWrite (ou liste todo tenue par l'orchestrateur si TodoWrite absent). A la completion, l'agent annonce :

```
[YYYY-MM-DD HH:MM] P0 Temps X — <Nom> termine.
  Agent(s): <liste>
  Resultat: <resume 1-2 lignes>
  Prochain Temps: Temps X+1 — <Nom>
```

---

## Modes d'autonomie

Le mode determine **combien de gates humaines** existent entre P0 et P5.

| Mode | Comportement | Gates | Ideal pour |
|------|-------------|-------|-----------|
| **FULL** | Autopilot complet apres P0 | P0 → P5 (test humain) | T3, projets bien cadres |
| **BIG_STEPS** | Automatique par phase, gates entre phases | P0 → P3→P4 → P4→P5 → P5 | T4, premiers projets |
| **GUIDED** | Validation a chaque etape | P0 → chaque etape → P4 → P5 | T5, supervision etroite |

**Persistance** : `.mip/memory/user-profile.md` section "Preferences de travail". Changeable via `/autonomy_mode <mode>`.

**En mode FULL** : derniere intervention humaine = approbation brief P0 + choix d'autonomie. Sauf frein d'urgence.

**Deroulement Gate P0** : brief presente → utilisateur lit → utilisateur approuve → utilisateur choisit le mode d'autonomie. Le choix d'autonomie est la **derniere question** de la Gate P0, jamais la premiere.

---

## P3 — Implementation (toutes classes)

> Detail complet : `.mip/modules/p3-execution.md`

**Agents** : Francois (back) + Lise (front) en PARALLELE. Denis coordonne.

**Concepts cles** :
- **Smoke test prioritaire** : Test e2e happy path qui COMPILE mais ECHOUE — valide la structure du plan avant TDD
- **Cycle TDD 10 pas** : START → RED → GREEN → REFACTOR → VERIFY → LINT → COMMIT → PUSH → LOG → TRACK
- **Subagent frais par tache** : Eviter la pollution de contexte
- **Checkpoint /5 taches** : Mini-audit Denis + spot-check securite Victor + spot-check efficience Jean + push
- **Context7 spot-check** : Verifier patterns framework avant d'ecrire du code
- **Auto-correction** : Root cause → Context7 → tentative 1 → tentative 2 → frein d'urgence

**Gate P3** : Chaque tache passe test + clippy.

**Gate BIG_STEPS (P3→P4)** : Denis presente resume (taches, tests, auto-corrections). Utilisateur : CONTINUER / CORRIGER / STOPPER.

---

## P4 — Integration, Audit & Securite (T3+)

> Detail complet : `.mip/modules/p4-p5-p6.md`

**Agents** : Denis (integration) + George (conformite) + Victor (securite /100) + Hugo (deploiement, T4-T5) + Jean (efficience tokens)

- **Denis** : `cargo build/test/clippy --workspace`. Auto-correction, frein d'urgence si 2 echecs.
- **George** : Audit conformite (build, tests, clippy, MSCM, Lois d'Autonomie, UX).
- **Victor** : Score securite /100 (5 criteres x /20 : auth, crypto, validation, deps, logging). Defaut critique = BLOQUANT.
- **Hugo** (T4-T5) : Build prod, Docker, CI/CD, health checks, config prod vs dev.
- **Jean** : Audit efficience tokens — consommation vs budget, anomalies, rapport dans `<sequence>/audits/`.

**Gate P4** : 0 defaut BLOQUANT + score securite conforme.

**Gate BIG_STEPS (P4→P5)** : George + Victor presentent resume audit. Utilisateur : CONTINUER / CORRIGER / STOPPER.

---

## P5 — Livraison, Test humain & Validation

> Detail complet : `.mip/modules/p4-p5-p6.md`

**Agent** : Denis (livraison) + George (assistance test)

4 volets : Presentation livrable → Test humain → Questionnaire satisfaction → Decision.

**Verdict** : ACCEPTE (merge main + tag) / ACCEPTE AVEC RESERVES (merge + taches futures) / REFUSE (boucle MIP, retour P0).

**Merge** : `git merge --no-ff feat/<slug>` + push + tag si release + nettoyage branche.

**Boucle MIP** : Si REFUSE → Maria reprend en P0 Temps 1 avec feedback. Brief precedent sert de reference.

---

## P6 — Rapport final, Archivage & Capitalisation (T3+)

> Detail complet : `.mip/modules/p4-p5-p6.md`

**Agents** : Arianne + Jean (refactorisation memoire)

3 volets : Rapport final (`<sequence>/rapports_finaux/`) → Archivage artefacts → Capitalisation (.mip/memory/) + Jean refactorise memoire avec Arianne.

**Rapport** : Notes /20 sur 8 criteres, trace d'execution extraite du plan annote, metriques tokens/duree, resume dev, profil utilisateur, capitalisation agents. **Un rapport sans trace d'execution est INCOMPLET.**

**Capitalisation** : Patterns → `patterns-and-lessons.md`, anti-patterns → `patterns-and-lessons.md`, notes → `mip-performance-history.md`, profil → `user-profile.md`.

---

## Regles NON NEGOCIABLES (essentielles)

> Les regles ci-dessous sont celles qui ne sont PAS derivables du protocole lui-meme. Le protocole (P0 gates, TDD, checkpoints, etc.) est lui-meme non-negociable.

1. **Verification Context7 obligatoire** (T3+) — Verifier les docs des libs impliquees avant de coder
2. **Anti-patterns charges** — Lire `.mip/memory/patterns-and-lessons.md` et `.mip/memory/MEMORY.md` avant chaque sprint
3. **Subagent frais par tache** — Eviter la pollution de contexte entre taches
4. **Pas de code dangereux en prod** — Pas de `unwrap()` (Rust), `any` (TS), `bare except` (Python)
5. **Archivage systematique** (T3+) — Arianne capitalise apres chaque livraison
6. **TL;DR obligatoire** — Chaque artefact MIP commence par un resume de 5 lignes max
7. **Phase SETUP obligatoire** — `.mip/environment.md` doit exister avant le premier P0
8. **Environment.md referentiel** — Commandes build/test/lint lues depuis `.mip/environment.md`, pas hardcodees
9. **Profils MIP** — Basculer d'outil/LLM via `/mip_profile <slug>`. MIP adapte ses capacites (parallelisme, TodoWrite, MCP) selon le profil actif (`.mip/profiles/active`)
10. **Analyse securite obligatoire** (T3+) — Victor analyse surfaces d'attaque AVANT la spec (Temps 5)
11. **Audit securite avant livraison** (T3+) — Victor score /100 en P4. Defaut critique = BLOQUANT
12. **Verification infra** (T4-T5) — Hugo evalue infra en P0 Temps 4 et verifie en P4
13. **Annotation du plan obligatoire** — Chaque tache annotee : `Demarre a HH:MM:SS. Termine a HH:MM:SS avec [model] pour N tokens (mesures).` — ZERO tilde, ZERO estimation
14. **Metriques consommation obligatoires** — Rapport P6 DOIT inclure tokens agreges, duree, indicateurs efficacite. **Valeurs mesurees uniquement** (task-notifications + timestamps filesystem). Aucune estimation.
15. **Horodatage T0** — Maria horodate le premier prompt des le debut de P0
16. **ZERO estimation dans le rapport P6** — Les mots `~`, `environ`, `estimation`, `approximation` sont INTERDITS dans les sections metriques. Si une donnee manque, ecrire `null` + raison. Ne jamais fabriquer de valeur. Sources autorisees : task-notifications (tokens, duree), filesystem (timestamps), comptages (lignes, fichiers). Detail : `.mip/modules/metrics.md`
17. **Documents modulaires, 400 lignes max** — Tout artefact MIP (brief, spec, plan, audit, rapport) est limite a **400 lignes**. Au-dela, decouper en modules avec un document index qui reference les parties. Refuser les documents monolithiques. Approche : un fichier maitre court (index + TL;DR + navigation) + fichiers annexes par section. Exemple : `plan.md` (index) + `plan-etape-1.md`, `plan-etape-2.md`, etc.

---

## Efficience tokens — Connaissances pré-indexées

Chaque agent charge **uniquement ses fichiers pertinents** en debut de tache :

| Agent (role) | Fichiers a charger (chemin `.mip/memory/` sauf indication) |
|-------|-------------------|
| **Dev Back-End** (Francois) | `stack-patterns.md`, `api-contracts.md`, `test-templates.md`, `code-annotations-templates.md` |
| **Dev Front-End** (Lise) | `stack-cheatsheet.md`, `api-contracts.md`, `project-file-map.md`, `code-annotations-templates.md` |
| **Chef Dev** (Denis) | `project-file-map.md`, `stack-patterns.md`, `mip-decisions.md`, `patterns-and-lessons.md` |
| **Audit Expert** (George) | `project-file-map.md`, `code-annotations-templates.md`, `patterns-and-lessons.md` |
| **Expert Cybersecurite** (Victor) | `security-patterns.md`, `patterns-and-lessons.md`, `stack-patterns.md`, `project-file-map.md` |
| **DevOps & Infra** (Hugo) | `project-file-map.md`, `.mip/environment.md` (section Infrastructure), `mip-decisions.md` |
| **Responsable Efficience IA** (Jean) | `mip-performance-history.md`, `MEMORY.md`, `<sequence>/metrics/`, `.mip/agents/*.md` |
| **Team Manager** (Arianne) | `mip-decisions.md`, `patterns-and-lessons.md`, `mip-performance-history.md`, `team-skills-audit.md` |

> Note : Tous les fichiers ci-dessus sont dans `.mip/memory/` (ou `.mip/` pour environment, metrics, agents). Miyukini COG : `rust-patterns.md`, `dioxus-cheatsheet.md`, `mscm-templates.md`.

---

## Registre Context7 — Libs a verifier

> Registre projet-specifique. IDs pre-resolus en SETUP ou premier P0.

### Miyukini COG — IDs pre-resolus

| Lib | Context7 ID | Quand verifier |
|-----|-------------|----------------|
| **Dioxus 0.6** | `/dioxuslabs/dioxus/v0.6.3` | Tout composant UI, RSX, signals, hooks |
| **Dioxus docs** | `/llmstxt/dioxuslabs_learn_0_6_llms-full_txt` | Patterns avances, migration, pitfalls |
| **axum** | `/tokio-rs/axum/axum_v0_7_9` | Tout endpoint REST, middleware, extractors |
| **serde** | `/serde-rs/serde` | Serialization custom, derive macros, attributes |
| **Dioxus Components** | `/dioxuslabs/components` | Composants primitifs ARIA |

### Protocole universel Context7

- **Toujours** en P0 Temps 6 (spec) pour chaque lib impliquee
- **Spot-check** en P3 si la tache touche un pattern specifique
- **En cas d'erreur** : verifier si le pattern utilise est encore valide
- **Premier P0** : resoudre les IDs de toutes les libs via `resolve-library-id`

---

## MASS — Miyukini Agent Swarm System

> Detail complet : `.mip/modules/mass.md`. Actif pour T4-T5.

**TL;DR** : MASS execute les taches MIP en parallele via un DAG de dependances decompose en vagues. 3 couches : Orchestrateur (Maria/DAG) → Pool Workers (agents) → Synchronisation (Denis/merge). **Loi 9** : si >3 taches independantes dans une vague, parallelisation OBLIGATOIRE.

3 modes de dispatch :
- **Subagent burst** : T2-T3 ou vague <=3 taches
- **Worktree swarm** : T4 ou vague >3 taches avec fichiers disjoints
- **Team swarm** : T5, flag Agent Teams experimental

DAG dans `<sequence>/phases/dag.json`. Metriques swarm dans `<sequence>/metrics/`.

---

## Intégration SuperClaude

| Phase MIP | Skill SuperClaude | Usage |
|-----------|-------------------|-------|
| P0 (T1-2) | `brainstorming` | Maria structure le brief (10 temps) |
| P0 (T8) | `verification-before-completion` | Arianne verifie conformite |
| P0 (T7) | `writing-plans` | Denis cree taches atomiques + guide |
| P3 | `subagent-driven-development` | Execution par subagent frais |
| P3 | `test-driven-development` | Cycle RED-GREEN-REFACTOR |
| P3 | `systematic-debugging` | Root cause + auto-correction |
| P4 | `verification-before-completion` | George verifie |
| P5 | `finishing-a-development-branch` | Denis finalise + test humain |
| P6 | — | Arianne : rapport + capitalisation |

---

## Raccourcis pour taches simples

**T1 (micro-fix)** : Pas de brief ni spec. Corriger directement, tester, committer.
**T2 (fix cible)** : Denis ecrit un mini-plan (1-3 taches), execution directe. Feature branch si >1 fichier.

Le protocole est **proportionnel** : les petites taches ne sont pas alourdies.

---

## Workflow Git

**Convention branches** : `feat/<slug>` (T3-T5) | `fix/<slug>` (T1-T2) | `refactor/<slug>` (T3+). Slug derive du titre du brief.

**Premiere action AUTOPILOT** :
```bash
git checkout -b feat/<slug>
git push -u origin feat/<slug>
```

**Push** : Apres chaque commit (TDD pas 8) + apres chaque checkpoint Denis.

**Merge** : `git merge --no-ff feat/<slug>` → main, apres verdict ACCEPTE en P5.

---

## Le plan comme document de suivi (OBLIGATOIRE)

Le plan exhaustif (`<sequence>/plans_p3/`) sert **a la fois** de plan d'execution ET de journal de suivi. Chaque agent annote directement le plan :

```
> Demarre a HH:MM:SS. Termine a HH:MM:SS avec [model] pour N tokens (mesures).
```

**ZERO estimation** : Le `N` est la valeur reelle extraite de la task-notification de l'agent, pas une approximation. Si les tokens ne sont pas encore disponibles, ecrire `tokens: a completer` et l'orchestrateur remplit apres reception de la task-notification. Voir `.mip/modules/metrics.md` pour les methodes de collecte.

Le plan annote est la **source unique de verite** pour le rapport P6. Arianne lit le plan annote — pas besoin de reconstituer la trace.

---

## Frein d'urgence

L'autopilot s'arrete UNIQUEMENT si :
1. **Bug bloquant** : test echoue apres 2 tentatives de correction automatique
2. **Delta majeur** : probleme qui remet en question le plan
3. **Echec audit** : defaut CRITIQUE non corrigeable automatiquement

L'agent qui detecte le probleme **arrete l'autopilot** et **presente le probleme a l'utilisateur** avec une proposition de resolution.
