# Skill : MIP v2 â€” Protocole d'implÃ©mentation Miyukini

## Quand utiliser ce skill

Utiliser ce skill pour **toute demande de dÃ©veloppement** impliquant du code, une nouvelle fonctionnalitÃ©, un fix, un refactor ou un nouveau paquet/service. Le protocole MIP v2 orchestre l'Ã©quipe et structure le travail.

MIP v2 est **universel** : il s'adapte Ã  tout projet, stack et environnement. Le noyau du protocole (classification, phases, gates, agents) est invariant. Seule la **configuration projet** change via la Phase SETUP.

---

## Nomenclature (OBLIGATOIRE â€” terminologie canonique)

La hiÃ©rarchie suivante est **stricte**. Chaque terme dÃ©signe un seul niveau. Ne pas utiliser un terme pour un autre niveau.

```
SÃ©quence MIP
  â””â”€ Phase (P0, Git, P3, P4, P5, P6)
       â””â”€ Temps (P0 uniquement : Temps 1 Ã  10, invariants)
       â”‚    â””â”€ TÃ¢che (1 tÃ¢che = 1 agent exÃ©cutant)
       â””â”€ Ã‰tape (P3 uniquement : groupes du plan Denis)
       â”‚    â””â”€ TÃ¢che (1 tÃ¢che = 1 agent, MASS ou sÃ©quentiel selon l'Ã©tape)
       â””â”€ Volet (P4, P5, P6 : blocs procÃ©duraux internes)
            â””â”€ TÃ¢che (1 tÃ¢che = 1 agent exÃ©cutant)
```

| Terme | Niveau | Scope | DÃ©finition |
|-------|--------|-------|------------|
| **SÃ©quence** | 0 | Cycle entier | Un cycle MIP complet (P0 â†’ P6). `mip_sequence_number` dans les mÃ©triques. |
| **Phase** | 1 | Grandes divisions | P0, Git, P3, P4, P5, P6. Invariantes (I-2). |
| **Temps** | 2 | P0 uniquement | Temps 1 Ã  10. Invariants, non rÃ©ordonnables. Carte de synchronisation R-P0-4. |
| **Ã‰tape** | 3 | P3 uniquement | Groupes de tÃ¢ches du plan Denis. Chaque Ã©tape a : prÃ©requis, agents, livrables, critÃ¨res de complÃ©tion. Mode GUIDED = gate par Ã©tape. |
| **Volet** | 3 | P4, P5, P6 | Blocs procÃ©duraux internes Ã  une phase. Ex : P5 Volet 1 Â« PrÃ©sentation livrable Â». |
| **TÃ¢che** | 4 | Partout | UnitÃ© atomique. 1 tÃ¢che = 1 agent exÃ©cutant. `[CODE-01]`, `[TEST-U-01]`. MASS ou sÃ©quentiel selon le contexte de l'Ã©tape/volet parent. |

**RÃ¨gles** :
- Â« Ã‰tape Â» est **RÃ‰SERVÃ‰** au plan P3. Ne jamais utiliser Â« Ã©tape Â» pour les subdivisions de P0 (= Temps), P5/P6 (= Volet) ou SETUP (= SETUP-1 Ã  SETUP-6).
- Â« Temps Â» est **RÃ‰SERVÃ‰** Ã  P0. Ne jamais utiliser Â« Temps Â» hors de P0.
- Â« SÃ©quence Â» dÃ©signe le cycle MIP entier. Ne pas utiliser Â« sÃ©quence Â» pour une sÃ©rie ordonnÃ©e d'actions internes (utiliser Â« dÃ©roulement Â» ou Â« processus Â»).
- Le cycle TDD (RED, GREEN, REFACTOR...) = **pas** du cycle TDD (niveau infra-tÃ¢che, pas un terme hiÃ©rarchique).

---

## Architecture modulaire â€” Chargement Ã  la demande

Ce skill est **modulaire**. Ce fichier contient le noyau du protocole. Les dÃ©tails de chaque phase sont dans des modules sÃ©parÃ©s chargÃ©s **Ã€ LA DEMANDE** pour Ã©conomiser la fenÃªtre de contexte.

| Phase | Module Ã  charger (Read) | DÃ©clencheur |
|-------|-------------------------|-------------|
| SETUP | `.mip/modules/setup.md` | `.mip/environment.md` absent ou `/mip_setup` |
| P0 | `.mip/modules/p0-details-index.md` puis drill-down p0-details.md | DÃ©but de P0 (T3+) |
| P3 | `.mip/modules/p3-execution.md` | DÃ©but de P3 |
| P4-P6 | `.mip/modules/p4-p5-p6.md` | DÃ©but de P4 |
| MÃ©triques | `.mip/modules/metrics.md` | Init mÃ©triques (dÃ©but de sÃ©quence) |
| MASS | `.mip/modules/mass.md` | T4-T5 avec parallÃ©lisation |
| Ref. outils IA | `.mip/modules/tools-reference.md` | Sur demande explicite |

**Instruction** : Lire le module avec l'outil Read AVANT de commencer la phase correspondante. Ne pas charger tous les modules au dÃ©but â€” charger uniquement le module de la phase en cours.

**Tables de rÃ©fÃ©rence partagÃ©es** : `.mip/protocol/conventions.md` (classification, Ã©quipe agents, artefacts MIP, workflow). Ne pas les dupliquer ici.

---

## Phase SETUP â€” Onboarding universel (UNE SEULE FOIS)

> DÃ©tail complet : `.mip/modules/setup.md`

La Phase SETUP s'exÃ©cute **une seule fois** lors de la premiÃ¨re utilisation de MIP dans un nouvel environnement. Elle produit `.mip/environment.md` (configuration maÃ®tre). Si ce fichier existe dÃ©jÃ , la phase est **sautÃ©e**.

**DÃ©clenchement** : `.mip/environment.md` n'existe pas, ou `/mip_setup`, ou demande de reconfiguration.

**SETUP-1 Ã  SETUP-6** : DÃ©tection systÃ¨me â†’ Config environnement (19 questions) â†’ Profil utilisateur (8 questions) â†’ DÃ©tection outil IA â†’ DÃ©pendances â†’ Agents.

**Commandes** : `/mip_setup` (relancer), `/mip_status` (afficher statut), `/autonomy_mode <mode>` (changer mode).

---

## Structure par sÃ©quence â€” CrÃ©ation par Maria au dÃ©marrage

**Au dÃ©but de chaque sÃ©quence** (dÃ¨s le premier prompt P0), Maria crÃ©e une **structure dÃ©diÃ©e** Ã  la sÃ©quence. Chaque sÃ©quence a son propre dossier, isolÃ© des autres.

### Dossier de sÃ©quence

Chemin : `.mip/sequences/YYYY-MM-DD-<slug>/`

> Convention : dans ce document, `<sequence>` = `.mip/sequences/YYYY-MM-DD-<slug>/`

| Dossier | Contenu |
|---------|---------|
| `briefs/` | Briefs de cadrage (P0) |
| `specs/` | SpÃ©cifications techniques (P0 T6) |
| `gpi/` | Gouvernance, pilotage, initiatives (dÃ©cisions, jalons, arbitrages) |
| `phases/` | Traces et livrables par phase ; DAG MASS : `phases/dag.json` |
| `plans_p3/` | Plans d'exÃ©cution et guides P3 |
| `audits/` | Rapports d'audit (P4) |
| `metrics/` | MÃ©triques de la sÃ©quence (tokens, durÃ©es, compteurs) |
| `rapports_finaux/` | Rapport P6, synthÃ¨se, capitalisation |
| `ressources/` | Index des ressources (voir `ressources/index.md`) â€” docs, certs, libs, IDs Context7 |

### Dossiers partagÃ©s (racine .mip/)

**Seuls** les trois dossiers suivants sont **partagÃ©s** Ã  la racine de `.mip/` â€” ils ne sont pas dupliquÃ©s par sÃ©quence :

| Dossier | Usage |
|---------|-------|
| `.mip/memory/` | MÃ©moire projet, patterns, profils utilisateur, anti-patterns |
| `.mip/skills/` | Skills MIP et skills mÃ©tier |
| `.mip/modules/` | Modules du protocole (setup, p0-details, p3-execution, etc.) |

Les artefacts spÃ©cifiques Ã  une sÃ©quence (briefs, specs, plans, audits, rapports) vont **dans le dossier de la sÃ©quence**, pas Ã  la racine.

### T1/T2 â€” Protocole allÃ©gÃ©

T1 et T2 sont **inclus** dans la structure par sÃ©quence, avec un protocole allÃ©gÃ© :
- **Slug** : dÃ©rivÃ© de la demande (ex. `fix-typo-auth`, `fix-crash-login`)
- **Structure** : Maria crÃ©e `.mip/sequences/YYYY-MM-DD-<slug>/` avec dossiers minimaux (briefs/, plans_p3/, audits/, metrics/)
- **Pas de P0** : pas de brainstorm, spec intÃ©grÃ©e au mini-plan de Denis

### Checklist crÃ©ation sÃ©quence (Maria)

1. Classifier (T1â€“T5)
2. DÃ©terminer slug (provisoire si T3+ : `in-progress` ou dÃ©rivÃ© de la demande)
3. CrÃ©er `.mip/sequences/YYYY-MM-DD-<slug>/` + sous-dossiers (briefs/, specs/, gpi/, phases/, plans_p3/, audits/, metrics/, rapports_finaux/, ressources/)
4. Initialiser `<sequence>/metrics/YYYY-MM-DD-<slug>.json`
5. Remplir ou copier `ressources/index.md` (squelette depuis `_template` si disponible)

---

## Invariants (NOYAU IMMUABLE)

Les Ã©lÃ©ments suivants sont **invariants** â€” ils s'appliquent quel que soit le projet, la stack, l'environnement ou l'outil IA :

| # | Invariant | PortÃ©e |
|---|-----------|--------|
| I-1 | **Classification T1-T5 avant toute action** | Maria classifie, doute = un cran au-dessus |
| I-2 | **SÃ©quence des phases** : P0 â†’ Git â†’ P3 â†’ P4 â†’ P5 â†’ P6 (T3+) | Pas de saut, pas de rÃ©ordonnancement |
| I-3 | **P0 = seule phase humaine** | Pas de code avant approbation du brief |
| I-4 | **Brief lu par l'utilisateur AVANT choix d'autonomie** | L'utilisateur ne peut choisir FULL/BIG_STEPS/GUIDED sans avoir lu le brief â€” choix Ã©clairÃ© obligatoire |
| I-5 | **Gates strictes entre phases** | Chaque gate a des critÃ¨res explicites, pas de passage sans validation |
| I-6 | **TDD obligatoire en P3** | Cycle RED â†’ GREEN â†’ REFACTOR â†’ VERIFY â†’ LINT â†’ COMMIT |
| I-7 | **MÃ©triques mesurÃ©es, jamais estimÃ©es** | Sources : notifications de tÃ¢ches, horodatages fichiers, comptages. Aucune approximation dans le rapport P6 |
| I-8 | **P5 = test humain obligatoire** | L'utilisateur teste le livrable et rend un verdict (ACCEPTÃ‰/REFUSÃ‰) |
| I-9 | **9 Lois d'Autonomie** | Non nÃ©gociables, applicables Ã  tout le code produit |
| I-10 | **RÃ´les agents fixes** | Chaque agent a un rÃ´le, des compÃ©tences et un pÃ©rimÃ¨tre dÃ©finis |
| I-11 | **Workflow feature branch** | `feat/<slug>`, merge --no-ff vers main aprÃ¨s P5 |
| I-12 | **Artefacts structurÃ©s par sÃ©quence** | `<sequence>/` contient briefs/, specs/, gpi/, phases/, plans_p3/, audits/, metrics/, rapports_finaux/, ressources/ |
| I-13 | **Frein d'urgence** | ArrÃªt auto si bug bloquant aprÃ¨s 2 tentatives ou delta majeur |
| I-14 | **Artefacts de sequence modulaires, 400 lignes max** | Limite appliquee aux artefacts de sequence (brief, spec, plan, audit, rapport), decoupage index + annexes |
| I-15 | **Boucle MIP bornÃ©e** | Comptage `mip_loops` ; aprÃ¨s 10 itÃ©rations, suggÃ©rer de rÃ©duire le scope |

> **Ce qui varie** : la stack technique, les commandes build/test/lint, les librairies, la config CI/CD, le nombre d'agents mobilisÃ©s par classe. Tout cela est configurÃ© dans `.mip/environment.md` (Phase SETUP).

---

## Classification (OBLIGATOIRE â€” AVANT toute action)

> Table de rÃ©fÃ©rence : `.mip/protocol/conventions.md`, section Â« Classification des tÃ¢ches Â».

| Classe | CritÃ¨re | Phases |
|--------|---------|--------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 â†’ P5 |
| **T2** | Fix cible, 1-3 fichiers | P3 -> P5 |
| **T3** | Feature modÃ©rÃ©e, 3-10 fichiers | P0 â†’ P3 â†’ P4 â†’ P5 â†’ P6 |
| **T4** | Feature majeure, 10+ fichiers | P0 â†’ P3 â†’ P4 â†’ P5 â†’ P6 |
| **T5** | Chantier stratÃ©gique | P0 â†’ P3 â†’ P4 â†’ P5 â†’ P6 |

**Qui classifie** : Maria. En cas de doute, classer **UN CRAN AU-DESSUS**.

Note T2 : mini-plan Denis au demarrage de P3 (pas de phase P2 distincte).

---

## P0 â€” Cadrage complet en 10 temps (T3+, SEULE phase humaine)

> Index : `.mip/modules/p0-details-index.md`. DÃ©tail par temps : `.mip/modules/p0-details.md` (drill-down offset/limit).

**Agents** : Maria (lead) + Lise + Fabrice (T4-T5) + Denis + Hugo (T4-T5) + Jean + Victor (T3+) + FranÃ§ois + Arianne

P0 est **LA phase humaine** : elle dÃ©termine la direction. Pas de code avant la fin de P0. AprÃ¨s approbation du brief, **tout est automatique** (P3 â†’ P6).

**RÃ¨gles de bornage P0** (dÃ©tail dans `.mip/modules/p0-details.md` section Â« RÃ¨gles de prÃ©sentation P0 Â») :
- **R-P0-1** : AskUserQuestion obligatoire pour toute question Ã  l'utilisateur (pas de texte libre). Autant de questions que nÃ©cessaire par section (pas de limite 4).
- **R-P0-2** : Annonce visible dans le chat aprÃ¨s chaque Temps (3-5 lignes).
- **R-P0-3** : Brief prÃ©sentÃ© dans le chat (TL;DR + approches + risques + lien fichier) PUIS 2 appels AskUserQuestion distincts (approbation, puis autonomie).
- **R-P0-4** : Carte de synchronisation des Temps â€” dÃ©pendances explicites, aucun temps lancÃ© avant ses prÃ©requis.
- **R-P0-5** : AllÃ¨gement par classe (T3 vs T4 vs T5) â€” Temps sautÃ©s annoncÃ©s explicitement.

| Temps | Nom | Agent(s) | Module dÃ©tail |
|-------|-----|----------|---------------|
| 1 | Exploration + Brainstorming structurÃ© | Maria | p0-details.md |
| 2 | IdÃ©ation | Maria + Lise (parallÃ¨le) | p0-details.md |
| 3 | Analyse concurrentielle | Fabrice (T4-T5, parallÃ¨le avec T2) | p0-details.md |
| 4 | Inventaire prÃ©requis + Infra + ModÃ¨les | Denis (lead) + Hugo (T4-T5) + Jean + FranÃ§ois + Lise | p0-details.md |
| 5 | Analyse de sÃ©curitÃ© | Victor (T3+) | p0-details.md |
| 6 | SpÃ©cification technique + vÃ©rification docs | FranÃ§ois | p0-details.md |
| 7 | Plan exhaustif + Guide d'implÃ©mentation | Denis | p0-details.md |
| 8 | Audit de faisabilitÃ© + Efficience | Arianne + Jean | p0-details.md |
| 9 | VÃ©rification pipeline CI/CD | Hugo (si CI/CD en place) | p0-details.md |
| 10 | SynthÃ¨se et Brief | Maria | p0-details.md |

**Gate P0** (dÃ©roulement strict) :
1. Maria prÃ©sente le brief complet Ã  l'utilisateur
2. L'utilisateur **lit le brief** (TL;DR + contenu)
3. L'utilisateur **approuve ou rejette** le brief (approche, pÃ©rimÃ¨tre, plan)
4. **SI approuvÃ©** : l'utilisateur choisit le mode d'autonomie (FULL/BIG_STEPS/GUIDED) â€” **choix Ã©clairÃ©** aprÃ¨s lecture du brief (invariant I-4)
5. L'exÃ©cution dÃ©marre

**Gate stricte** : Pas de passage en exÃ©cution sans brief approuvÃ©. Pas de choix d'autonomie sans lecture prÃ©alable du brief.

### Suivi P0 â€” Annonces temps rÃ©el

Chaque Temps est **suivi via TodoWrite** (ou liste todo tenue par l'agent orchestrateur si TodoWrite absent). Ã€ la complÃ©tion, l'agent annonce :

```
[YYYY-MM-DD HH:MM] P0 Temps X â€” <Nom> terminÃ©.
  Agent(s) : <liste>
  RÃ©sultat : <rÃ©sumÃ© 1-2 lignes>
  Prochain Temps : Temps X+1 â€” <Nom>
```

---

## Modes d'autonomie

Le mode dÃ©termine **combien de gates humaines** existent entre P0 et P5.

| Mode | Comportement | Gates | IdÃ©al pour |
|------|--------------|-------|------------|
| **FULL** | Autopilot complet aprÃ¨s P0 | P0 â†’ P5 (test humain) | T3, projets bien cadrÃ©s |
| **BIG_STEPS** | Automatique par phase, gates entre phases | P0 â†’ P3â†’P4 â†’ P4â†’P5 â†’ P5 | T4, premiers projets |
| **GUIDED** | Validation Ã  chaque Ã©tape | P0 â†’ chaque Ã©tape â†’ P4 â†’ P5 | T5, supervision rapprochÃ©e |

**Persistance** : `.mip/memory/user-profile.md` section Â« PrÃ©fÃ©rences de travail Â». Modifiable via `/autonomy_mode <mode>`.

**En mode FULL** : derniÃ¨re interaction humaine = approbation brief P0 + choix d'autonomie. Sauf frein d'urgence.

**DÃ©roulement Gate P0** : brief prÃ©sentÃ© â†’ utilisateur lit â†’ utilisateur approuve â†’ utilisateur choisit le mode d'autonomie. Le choix d'autonomie est la **derniÃ¨re question** de la Gate P0, jamais la premiÃ¨re.

---

## P3 â€” ImplÃ©mentation (toutes classes)

> DÃ©tail complet : `.mip/modules/p3-execution.md`

**Agents** : FranÃ§ois (back) + Lise (front) en PARALLÃˆLE. Denis coordonne.

**Concepts clÃ©s** :
- **Test fumÃ©e prioritaire** : Test e2e happy path qui COMPILE mais Ã‰CHOUE â€” valide la structure du plan avant TDD
- **Cycle TDD 10 pas** : START â†’ RED â†’ GREEN â†’ REFACTOR â†’ VERIFY â†’ LINT â†’ COMMIT â†’ PUSH â†’ LOG â†’ TRACK
- **Subagent frais par tÃ¢che** : Ã‰viter la pollution de contexte
- **Checkpoint toutes les 5 tÃ¢ches** : Mini-audit Denis + spot-check sÃ©curitÃ© Victor + spot-check efficience Jean + push
- **Spot-check docs** : VÃ©rifier les patterns framework avant d'Ã©crire du code
- **Auto-correction** : Cause racine â†’ vÃ©rif docs â†’ tentative 1 â†’ tentative 2 â†’ frein d'urgence

**Gate P3** : Chaque tÃ¢che passe test + lint.

**Gate BIG_STEPS (P3â†’P4)** : Denis prÃ©sente le rÃ©sumÃ© (tÃ¢ches, tests, auto-corrections). Utilisateur : CONTINUER / CORRIGER / ARRÃŠTER.

---

## P4 â€” IntÃ©gration, audit et sÃ©curitÃ© (T3+)

> DÃ©tail complet : `.mip/modules/p4-p5-p6.md`

**Agents** : Denis (intÃ©gration) + George (conformitÃ©) + Victor (sÃ©curitÃ© /100) + Hugo (dÃ©ploiement, T4-T5) + Jean (efficience tokens)

- **Denis** : Build/test/lint workspace complet. Auto-correction, frein d'urgence si 2 Ã©checs.
- **George** : Audit conformitÃ© (build, tests, lint, annotations, Lois d'Autonomie, UX).
- **Victor** : Score sÃ©curitÃ© /100 (5 critÃ¨res x /20 : auth, crypto, validation, deps, logging). DÃ©faut critique = BLOQUANT.
- **Hugo** (T4-T5) : Build prod, Docker, CI/CD, health checks, config prod vs dev.
- **Jean** : Audit efficience tokens â€” consommation vs budget, anomalies, rapport dans `<sequence>/audits/`.

**Gate P4** : 0 dÃ©faut BLOQUANT + score sÃ©curitÃ© conforme.

**Gate BIG_STEPS (P4â†’P5)** : George + Victor prÃ©sentent le rÃ©sumÃ© d'audit. Utilisateur : CONTINUER / CORRIGER / ARRÃŠTER.

---

## P5 â€” Livraison, test humain et validation

> DÃ©tail complet : `.mip/modules/p4-p5-p6.md`

**Agent** : Denis (livraison) + George (assistance test)

4 volets : PrÃ©sentation livrable â†’ Test humain â†’ Questionnaire satisfaction â†’ DÃ©cision.

**Verdict** : ACCEPTÃ‰ (merge main + tag) / ACCEPTÃ‰ AVEC RÃ‰SERVES (merge + tÃ¢ches futures) / REFUSÃ‰ (boucle MIP, retour P0).

**Merge** : `git merge --no-ff feat/<slug>` + push + tag si release + nettoyage branche.

**Boucle MIP** : Si REFUSÃ‰ â†’ Maria reprend en P0 Temps 1 avec le feedback. Le brief prÃ©cÃ©dent sert de rÃ©fÃ©rence.

---

## P6 â€” Rapport final, archivage et capitalisation (T3+)

> DÃ©tail complet : `.mip/modules/p4-p5-p6.md`

**Agents** : Arianne + Jean (refactorisation mÃ©moire)

3 volets : Rapport final (`<sequence>/rapports_finaux/`) â†’ Archivage artefacts â†’ Capitalisation (`.mip/memory/`) + Jean refactorise la mÃ©moire avec Arianne.

**Rapport** : Notes /20 sur 8 critÃ¨res, trace d'exÃ©cution extraite du plan annotÃ©, mÃ©triques tokens/durÃ©e, rÃ©sumÃ© dev, profil utilisateur, capitalisation agents. **Un rapport sans trace d'exÃ©cution est INCOMPLET.**

**Capitalisation** : Patterns â†’ `patterns-and-lessons.md`, anti-patterns â†’ `patterns-and-lessons.md`, notes â†’ `mip-performance-history.md`, profil â†’ `user-profile.md`.

---

## RÃ¨gles non nÃ©gociables (essentielles)

> Les rÃ¨gles ci-dessous ne sont pas dÃ©rivables du protocole lui-mÃªme. Le protocole (gates P0, TDD, checkpoints, etc.) est lui-mÃªme non nÃ©gociable.

1. **VÃ©rification docs obligatoire** (T3+) â€” VÃ©rifier la doc des libs avant de coder (Context7, recherche web ou fallback si indisponible)
2. **Anti-patterns chargÃ©s** â€” Lire `.mip/memory/patterns-and-lessons.md` et `.mip/memory/MEMORY.md` avant chaque sprint
3. **Subagent frais par tÃ¢che** â€” Ã‰viter la pollution de contexte entre tÃ¢ches
4. **Pas de code dangereux en prod** â€” Pas de `unwrap()` (Rust), `any` (TS), `bare except` (Python)
5. **Archivage systÃ©matique** (T3+) â€” Arianne capitalise aprÃ¨s chaque livraison
6. **TL;DR obligatoire** â€” Chaque artefact MIP commence par un rÃ©sumÃ© de 5 lignes max
7. **Phase SETUP obligatoire** â€” `.mip/environment.md` doit exister avant le premier P0
8. **environment.md comme rÃ©fÃ©rence** â€” Commandes build/test/lint lues depuis `.mip/environment.md`, pas en dur
9. **Profils MIP** â€” Basculer d'outil/LLM via `/mip_profile <slug>`. MIP adapte ses capacitÃ©s (parallÃ©lisme, TodoWrite, MCP, terminal) selon le profil actif (`.mip/profiles/active`)
10. **Analyse sÃ©curitÃ© obligatoire** (T3+) â€” Victor analyse les surfaces d'attaque AVANT la spec (Temps 5)
11. **Audit sÃ©curitÃ© avant livraison** (T3+) â€” Victor score /100 en P4. DÃ©faut critique = BLOQUANT
12. **VÃ©rification infra** (T4-T5) â€” Hugo Ã©value l'infra en P0 Temps 4 et vÃ©rifie en P4
13. **Annotation du plan obligatoire** â€” Chaque tÃ¢che annotÃ©e : `DÃ©marrÃ© Ã  HH:MM:SS. TerminÃ© Ã  HH:MM:SS avec [model] pour N tokens (mesurÃ©s).` â€” ZÃ‰RO tilde, ZÃ‰RO estimation
14. **MÃ©triques consommation obligatoires** â€” Le rapport P6 DOIT inclure tokens agrÃ©gÃ©s, durÃ©e, indicateurs efficience. **Valeurs mesurÃ©es uniquement** (notifications de tÃ¢ches + horodatages fichiers). Aucune estimation.
15. **Horodatage T0** â€” Maria horodate le premier prompt au dÃ©but de P0
16. **ZÃ‰RO estimation dans le rapport P6** â€” Les mots `~`, `environ`, `estimation`, `approximatif` sont INTERDITS dans les sections mÃ©triques. Si donnÃ©e manquante, Ã©crire `null` + raison. Ne jamais fabriquer de valeur. Sources autorisÃ©es : notifications de tÃ¢ches (tokens, durÃ©e), fichiers (horodatages), comptages (lignes, fichiers). DÃ©tail : `.mip/modules/metrics.md`
17. **Artefacts de sequence modulaires, 400 lignes max** — Limite stricte pour brief, spec, plan, audit, rapport. Les modules/profils/references de protocole ne sont pas concernes par cette limite.

---

## Efficience tokens â€” Connaissances prÃ©-indexÃ©es

Chaque agent charge **uniquement ses fichiers pertinents** au dÃ©but de chaque tÃ¢che :

| Agent (rÃ´le) | Fichiers Ã  charger |
|--------------|-------------------|
| **Dev Back-End** (FranÃ§ois) | `stack-patterns.md`, `api-contracts.md`, `test-templates.md`, `code-annotations-templates.md` |
| **Dev Front-End** (Lise) | `stack-cheatsheet.md`, `api-contracts.md`, `project-file-map.md`, `code-annotations-templates.md` |
| **Chef Dev** (Denis) | `project-file-map.md`, `stack-patterns.md`, `mip-decisions.md`, `patterns-and-lessons.md` |
| **Expert Audit** (George) | `project-file-map.md`, `code-annotations-templates.md`, `patterns-and-lessons.md` |
| **Expert CybersÃ©curitÃ©** (Victor) | `security-patterns.md`, `patterns-and-lessons.md`, `stack-patterns.md`, `project-file-map.md` |
| **DevOps & Infra** (Hugo) | `project-file-map.md`, `.mip/environment.md` (section Infrastructure), `mip-decisions.md` |
| **Responsable Efficience IA** (Jean) | `mip-performance-history.md`, `MEMORY.md`, `<sequence>/metrics/` |
| **Team Manager** (Arianne) | `mip-decisions.md`, `patterns-and-lessons.md`, `mip-performance-history.md`, `team-skills-audit.md` |

> Tous ces fichiers sont dans `.mip/memory/` (sauf environment, metrics).

---

## Documentation des librairies â€” Registre de vÃ©rification

> Registre projet-spÃ©cifique. IDs prÃ©-rÃ©solus lors du SETUP ou du premier P0.

### Protocole universel

- **Toujours** pendant P0 Temps 6 (spec) pour chaque lib impliquÃ©e
- **Spot-check** en P3 si la tÃ¢che touche un pattern spÃ©cifique
- **En cas d'erreur** : vÃ©rifier si le pattern utilisÃ© est encore valide
- **Premier P0** : rÃ©soudre les IDs de toutes les libs via `resolve-library-id`

---

## MASS â€” Agent Swarm System

> DÃ©tail complet : `.mip/modules/mass.md`. Actif pour T4-T5.

**TL;DR** : MASS exÃ©cute les tÃ¢ches MIP en parallÃ¨le via un DAG de dÃ©pendances dÃ©composÃ© en vagues. 3 couches : Orchestrateur (Maria/DAG) â†’ Pool Workers (agents) â†’ Synchronisation (Denis/merge). **Loi 9** : si >3 tÃ¢ches indÃ©pendantes dans une vague, parallÃ©lisation OBLIGATOIRE.

3 modes de dispatch :
- **Subagent burst** : T2-T3 ou vague <=3 tÃ¢ches
- **Worktree swarm** : T4 ou vague >3 tÃ¢ches avec fichiers disjoints
- **Team swarm** : T5, flag Agent Teams expÃ©rimental

DAG dans `<sequence>/phases/dag.json`. MÃ©triques swarm dans `<sequence>/metrics/`.

---

## Raccourcis pour tÃ¢ches simples

**T1 (micro-fix)** : Pas de brief ni spec. Corriger directement, tester, committer.
**T2 (fix cible)** : Denis Ã©crit un mini-plan (1-3 tÃ¢ches), exÃ©cution directe. Feature branch si >1 fichier.

Le protocole est **proportionnel** : les petites tÃ¢ches ne sont pas alourdies.

---

## Workflow Git

**Convention de branche** : `feat/<slug>` (T3-T5) | `fix/<slug>` (T1-T2) | `refactor/<slug>` (T3+). Slug dÃ©rivÃ© du titre du brief.

**PremiÃ¨re action AUTOPILOT** :
```bash
git checkout -b feat/<slug>
git push -u origin feat/<slug>
```

**Push** : AprÃ¨s chaque commit (TDD pas 8) + aprÃ¨s chaque checkpoint Denis.

**Merge** : `git merge --no-ff feat/<slug>` â†’ main, aprÃ¨s verdict ACCEPTÃ‰ en P5.

---

## Le plan comme document de suivi (OBLIGATOIRE)

Le plan exhaustif (`<sequence>/plans_p3/`) sert **Ã  la fois** de plan d'exÃ©cution ET de journal de suivi. Chaque agent annote directement le plan :

```
> DÃ©marrÃ© Ã  HH:MM:SS. TerminÃ© Ã  HH:MM:SS avec [model] pour N tokens (mesurÃ©s).
```

**ZÃ‰RO estimation** : Le `N` est la valeur rÃ©elle extraite de la notification de tÃ¢che de l'agent, pas une approximation. Si les tokens ne sont pas encore disponibles, Ã©crire `tokens: Ã  complÃ©ter` et l'orchestrateur remplit aprÃ¨s rÃ©ception de la notification. Voir `.mip/modules/metrics.md` pour les mÃ©thodes de collecte.

Le plan annotÃ© est la **source unique de vÃ©ritÃ©** pour le rapport P6. Arianne lit le plan annotÃ© â€” pas besoin de reconstituer la trace.

---

## Frein d'urgence

L'autopilot s'arrÃªte UNIQUEMENT si :
1. **Bug bloquant** : le test Ã©choue aprÃ¨s 2 tentatives de correction automatique
2. **Delta majeur** : problÃ¨me qui remet en question le plan
3. **Ã‰chec audit** : dÃ©faut CRITIQUE non corrigeable automatiquement

L'agent qui dÃ©tecte le problÃ¨me **arrÃªte l'autopilot** et **prÃ©sente le problÃ¨me Ã  l'utilisateur** avec une proposition de rÃ©solution.



