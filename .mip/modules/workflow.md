# Workflow : MIP v2 — Protocole d'implémentation Miyukini

## Quand utiliser ce workflow

Utiliser ce workflow pour **toute demande de développement** impliquant du code, une nouvelle fonctionnalité, un fix, un refactor ou un nouveau paquet/service. Le protocole MIP v2 orchestre l'équipe et structure le travail.

MIP v2 est **universel** : il s'adapte à tout projet, stack et environnement. Le noyau du protocole (classification, phases, gates, agents) est invariant. Seule la **configuration projet** change via la Phase SETUP.

---

## Nomenclature (OBLIGATOIRE — terminologie canonique)

La hiérarchie suivante est **stricte**. Chaque terme désigne un seul niveau. Ne pas utiliser un terme pour un autre niveau.

```
Séquence MIP
  └─ Phase (P0, Git, P3, P4, P5, P6)
       └─ Temps (P0 uniquement : Temps 1 à 11, invariants)
       │    └─ Tâche (1 tâche = 1 agent exécutant)
       └─ Étape (P3 uniquement : groupes du plan Denis)
       │    └─ Tâche (1 tâche = 1 agent, MASS ou séquentiel selon l'étape)
       └─ Volet (P4, P5, P6 : blocs procéduraux internes)
            └─ Tâche (1 tâche = 1 agent exécutant)
```

| Terme | Niveau | Scope | Définition |
|-------|--------|-------|------------|
| **Séquence** | 0 | Cycle entier | Un cycle MIP complet (P0 → P6). `mip_sequence_number` dans les métriques. |
| **Phase** | 1 | Grandes divisions | P0, Git, P3, P4, P5, P6. Invariantes (I-2). |
| **Temps** | 2 | P0 uniquement | Temps 1 à 11. Invariants, non réordonnables. Carte de synchronisation R-P0-4. |
| **Étape** | 3 | P3 uniquement | Groupes de tâches du plan Denis. Chaque étape a : prérequis, agents, livrables, critères de complétion. Mode GUIDED = gate par étape. |
| **Volet** | 3 | P4, P5, P6 | Blocs procéduraux internes à une phase. Ex : P5 Volet 1 « Présentation livrable ». |
| **Tâche** | 4 | Partout | Unité atomique. 1 tâche = 1 agent exécutant. `[CODE-01]`, `[TEST-U-01]`. MASS ou séquentiel selon le contexte de l'étape/volet parent. |

**Règles** :
- « Étape » est **RÉSERVÉ** au plan P3. Ne jamais utiliser « étape » pour les subdivisions de P0 (= Temps), P5/P6 (= Volet) ou SETUP (= SETUP-1 à SETUP-6).
- « Temps » est **RÉSERVÉ** à P0. Ne jamais utiliser « Temps » hors de P0.
- « Séquence » désigne le cycle MIP entier. Ne pas utiliser « séquence » pour une série ordonnée d'actions internes (utiliser « déroulement » ou « processus »).
- Le cycle TDD (RED, GREEN, REFACTOR...) = **pas** du cycle TDD (niveau infra-tâche, pas un terme hiérarchique).

---

## Architecture modulaire — Chargement à la demande

Ce workflow est **modulaire**. Ce fichier contient le noyau du protocole. Les détails de chaque phase sont dans des modules séparés chargés **À LA DEMANDE** pour économiser la fenêtre de contexte.

| Phase | Module à charger (Read) | Déclencheur |
|-------|-------------------------|-------------|
| SETUP | `.mip/modules/setup.md` | `.mip/environment.md` absent ou `/mip_setup` |
| P0 | `.mip/modules/p0-details-index.md` puis drill-down p0-details.md | Début de P0 (T3+) |
| P3 | `.mip/modules/p3-execution.md` | Début de P3 |
| P4-P6 | `.mip/modules/p4-p5-p6.md` | Début de P4 |
| Métriques | `.mip/modules/metrics.md` | Init métriques (début de séquence) |
| MASS | `.mip/modules/mass.md` | T4-T5 avec parallélisation |
| Ref. outils IA | `.mip/modules/tools-reference.md` | Sur demande explicite |

**Instruction** : Lire le module avec l'outil Read AVANT de commencer la phase correspondante. Ne pas charger tous les modules au début — charger uniquement le module de la phase en cours.

**Tables de référence partagées** : `.mip/protocol/conventions.md` (classification, équipe agents, artefacts MIP, workflow). Ne pas les dupliquer ici.

---

## Phase SETUP — Onboarding universel (UNE SEULE FOIS)

> Détail complet : `.mip/modules/setup.md`

La Phase SETUP s'exécute **une seule fois** lors de la première utilisation de MIP dans un nouvel environnement. Elle produit `.mip/environment.md` (configuration maître). Si ce fichier existe déjà, la phase est **sautée**.

**Déclenchement** : `.mip/environment.md` n'existe pas, ou `/mip_setup`, ou demande de reconfiguration.

**SETUP-1 à SETUP-6** : Détection système → Config environnement (19 questions) → Profil utilisateur (8 questions) → Détection outil IA → Dépendances → Agents.

**Commandes** : `/mip_setup` (relancer), `/mip_status` (afficher statut), `/autonomy_mode <mode>` (changer mode).

---

## Structure par séquence — Création par Maria au démarrage

**Au début de chaque séquence** (dès le premier prompt P0), Maria crée une **structure dédiée** à la séquence. Chaque séquence a son propre dossier, isolé des autres.

### Dossier de séquence

Chemin : `.mip/sequences/YYYY-MM-DD-<slug>/`

> Convention : dans ce document, `<sequence>` = `.mip/sequences/YYYY-MM-DD-<slug>/`

| Dossier | Contenu |
|---------|---------|
| `briefs/` | Briefs de cadrage (P0) |
| `specs/` | Spécifications techniques (P0 T6) |
| `gpi/` | Gouvernance, pilotage, initiatives (décisions, jalons, arbitrages) |
| `phases/` | Traces et livrables par phase ; DAG MASS : `phases/dag.json` |
| `plans_p3/` | Plans d'exécution et guides P3 |
| `audits/` | Rapports d'audit (P4) |
| `metrics/` | Métriques de la séquence (tokens, durées, compteurs) |
| `rapports_finaux/` | Rapport P6, synthèse, capitalisation |
| `ressources/` | Index des ressources (voir `ressources/index.md`) — docs, certs, libs, IDs Context7 |
| `agents/` | Prompts agents fine-tuned de séquence (P0 T7) |
| `ui/` | Mini-site JSX de la séquence (`ui/index.html` + `ui/manifest.json`) |

### Dossiers partagés (racine .mip/)

**Seuls** les trois dossiers suivants sont **partagés** à la racine de `.mip/` — ils ne sont pas dupliqués par séquence :

| Dossier | Usage |
|---------|-------|
| `.mip/memory/` | Mémoire projet, patterns, profils utilisateur, anti-patterns |
| `.mip/skills/` | Skills MIP et skills metier |
| `.mip/modules/` | Modules du protocole (setup, p0-details, p3-execution, etc.) |

Les artefacts spécifiques à une séquence (briefs, specs, plans, audits, rapports) vont **dans le dossier de la séquence**, pas à la racine.

### T1/T2 — Protocole allégé

T1 et T2 sont **inclus** dans la structure par séquence, avec un protocole allégé :
- **Slug** : dérivé de la demande (ex. `fix-typo-auth`, `fix-crash-login`)
- **Structure** : Maria crée `.mip/sequences/YYYY-MM-DD-<slug>/` avec dossiers minimaux (briefs/, plans_p3/, audits/, metrics/)
- **Pas de P0** : pas de brainstorm, spec intégrée au mini-plan de Denis

### Checklist création séquence (Maria)

1. Classifier (T1–T5)
2. Déterminer slug (provisoire si T3+ : `in-progress` ou dérivé de la demande)
3. Créer `.mip/sequences/YYYY-MM-DD-<slug>/`
4. **Script 1 — init base** (avant P0, obligatoire) :
   `powershell -ExecutionPolicy Bypass -File .mip/scripts/init-sequence-base.ps1 -SequencePath <sequence>`
   → Crée : brief, métriques, P0 T1 et T2.
5. Exécuter P0 T1 (Exploration) puis P0 T2 (Idéation).
6. Estimer la complexité C1-C5 (confirmée en T8 par Denis).
7. **Script 2 — init par complexité** (fin de T2, obligatoire) :
   `powershell -ExecutionPolicy Bypass -File .mip/scripts/init-sequence-by-complexity.ps1 -SequencePath <sequence> -Complexity <C1|C2|C3|C4|C5>`
   → Crée les artefacts cumulatifs correspondant au niveau de complexité.

### Artefacts créés par complexité

| Artefact | C1 | C2 | C3 | C4 | C5 |
|----------|:--:|:--:|:--:|:--:|:--:|
| brief + métriques + T1 + T2 | ✓ | ✓ | ✓ | ✓ | ✓ |
| plan P3 + etape-00 + p3-trace | ✓ | ✓ | ✓ | ✓ | ✓ |
| etape-buf + p4/p5-trace + audits efficience/p5 | — | ✓ | ✓ | ✓ | ✓ |
| P0-travail + spec + gpi + T3-T11 + audits sécu + rapport + ui | — | — | ✓ | ✓ | ✓ |
| agents/ (index + manifest) | — | — | — | ✓ | ✓ |
- Agents : `agents/index.md`, `agents/manifest.json`, `agents/<PHASE>_<agent>.md`
- Suivi : `metrics/YYYY-MM-DD-<slug>.json`, `phases/dag.json`
- UI : `ui/index.html`, `ui/manifest.json` (onglets standards : P0, P3, P4, P5, Rapport final)

---

## Invariants (NOYAU IMMUABLE)

Les éléments suivants sont **invariants** — ils s'appliquent quel que soit le projet, la stack, l'environnement ou l'outil IA :

| # | Invariant | Portée |
|---|-----------|--------|
| I-1 | **Classification T1-T5 avant toute action** | Maria classifie, doute = un cran au-dessus |
| I-2 | **Séquence des phases** : P0 → Git → P3 → P4 → P5 → P6 (T3+) | Pas de saut, pas de réordonnancement |
| I-3 | **P0 = seule phase humaine** | Pas de code avant approbation du brief |
| I-4 | **Brief lu par l'utilisateur AVANT choix d'autonomie** | L'utilisateur ne peut choisir FULL/BIG_STEPS/GUIDED sans avoir lu le brief — choix éclairé obligatoire |
| I-5 | **Gates strictes entre phases** | Chaque gate a des critères explicites, pas de passage sans validation |
| I-6 | **TDD obligatoire en P3** | Cycle RED → GREEN → REFACTOR → VERIFY → LINT → COMMIT |
| I-7 | **Métriques mesurées, jamais estimées** | Sources : notifications de tâches, horodatages fichiers, comptages. Aucune approximation dans le rapport P6 |
| I-8 | **P5 = test humain obligatoire** | L'utilisateur teste le livrable et rend un verdict (ACCEPTÉ/REFUSÉ) |
| I-9 | **9 Lois d'Autonomie** | Non négociables, applicables à tout le code produit |
| I-10 | **Rôles agents fixes** | Chaque agent a un rôle, des compétences et un périmètre définis |
| I-11 | **Workflow feature branch** | `feat/<slug>`, merge --no-ff vers main après P5 |
| I-12 | **Artefacts structurés par séquence** | `<sequence>/` contient briefs/, specs/, gpi/, phases/, plans_p3/, audits/, metrics/, rapports_finaux/, ressources/, agents/, ui/ |
| I-13 | **Frein d'urgence** | Arrêt auto si bug bloquant après 2 tentatives ou delta majeur |
| I-14 | **Artefacts de sequence modulaires, 400 lignes max** | Limite appliquee aux artefacts de sequence (brief, spec, plan, audit, rapport), decoupage index + annexes |
| I-15 | **Boucle MIP bornée** | Comptage `mip_loops` ; après 10 itérations, suggérer de réduire le scope |
| I-16 | **Chargement agents borne par phase** | Charger d'abord `.mip/agents/<agent>/<PHASE>_<agent>.md`, puis escalader vers `FULL_<agent>.md` uniquement si justifie (regles: `.mip/agents/INDEX.md`) |

> **Ce qui varie** : la stack technique, les commandes build/test/lint, les librairies, la config CI/CD, le nombre d'agents mobilisés par classe. Tout cela est configuré dans `.mip/environment.md` (Phase SETUP).

---

## Classification (OBLIGATOIRE — AVANT toute action)

> Table de référence : `.mip/protocol/conventions.md`, section « Classification des tâches ».

| Classe | Critère | Phases |
|--------|---------|--------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 → P5 |
| **T2** | Fix cible, 1-3 fichiers | P3 -> P5 |
| **T3** | Feature modérée, 3-10 fichiers | P0 → P3 → P4 → P5 → P6 |
| **T4** | Feature majeure, 10+ fichiers | P0 → P3 → P4 → P5 → P6 |
| **T5** | Chantier stratégique | P0 → P3 → P4 → P5 → P6 |

**Qui classifie** : Maria. En cas de doute, classer **UN CRAN AU-DESSUS**.

Note T2 : mini-plan Denis au demarrage de P3 (pas de phase P2 distincte).

### Complexité de séquence C1-C5 (définie en P0 Temps 8 par Denis)

| Complexité | Description |
|-----------|-------------|
| **C1** | Complexité mineure — petit fix |
| **C2** | Complexité faible — grand fix, < 3 fichiers |
| **C3** | Complexité moyenne — fonctionnalité |
| **C4** | Complexité élevée — petit service COG |
| **C5** | Complexité stratégique — App, services complexes/sensibles, architecture |

**Qui définit** : Denis (P0 Temps 8). Inscrit dans le brief et dans `metrics/<date>-<slug>.json`.

---

## P0 — Cadrage complet en 11 temps (T3+, SEULE phase humaine)

> Index : `.mip/modules/p0-details-index.md`. Détail par temps : `.mip/modules/p0-details.md` (drill-down offset/limit).

**Agents** : Maria (lead) + Lise + Fabrice (T4-T5) + Denis + Hugo (T4-T5) + Jean + Victor (T3+) + François + Arianne

P0 est **LA phase humaine** : elle détermine la direction. Pas de code avant la fin de P0. Après approbation du brief, **tout est automatique** (P3 → P6).

**Règles de bornage P0** (détail dans `.mip/modules/p0-details.md` section « Règles de présentation P0 ») :
- **R-P0-1** : AskUserQuestion obligatoire pour toute question à l'utilisateur (pas de texte libre). Autant de questions que nécessaire par section (pas de limite 4).
- **R-P0-2** : Annonce visible dans le chat après chaque Temps (3-5 lignes).
- **R-P0-3** : Brief présenté dans le chat (TL;DR + approches + risques + lien fichier) PUIS 2 appels AskUserQuestion distincts (approbation, puis autonomie).
- **R-P0-4** : Carte de synchronisation des Temps — dépendances explicites, aucun temps lancé avant ses prérequis.
- **R-P0-5** : Allègement par classe (T3 vs T4 vs T5) — Temps sautés annoncés explicitement.

| Temps | Nom | Agent(s) | Module détail |
|-------|-----|----------|---------------|
| 1 | Exploration + Brainstorming structuré | Maria | p0-details.md |
| 2 | Idéation | Maria + Lise (parallèle) | p0-details.md |
| 3 | Analyse concurrentielle | Fabrice (T4-T5, parallèle avec T2) | p0-details.md |
| 4 | Inventaire prérequis + Infra + Modèles | Denis (lead) + Hugo (T4-T5) + Jean + François + Lise | p0-details.md |
| 5 | Analyse de sécurité | Victor (T3+) | p0-details.md |
| 6 | Spécification technique + vérification docs | François | p0-details.md |
| 7 | Génération agents fine-tuned de séquence | Maria (validation rapide Denis + Jean) | p0-details.md |
| 8 | Plan exhaustif + Guide d'implémentation | Denis | p0-details.md |
| 9 | Audit de faisabilité + Efficience | Arianne + Jean | p0-details.md |
| 10 | Vérification pipeline CI/CD | Hugo (si CI/CD en place) | p0-details.md |
| 11 | Synthèse et Brief | Maria | p0-details.md |

### P0 Temps 5 — Analyse sécurité Victor (RPS)

Le Temps 5 produit un **Rapport Préliminaire de Sécurité (RPS)**, dérivé des artefacts P0 disponibles au moment de l'analyse.

**Entrées minimales** :
- brief initial
- inventaire prérequis (Temps 4)
- contraintes légales connues
- contexte infra disponible

**Contenu obligatoire du RPS** :
1. surfaces d'attaque et risques majeurs
2. ressources sécurité requises (agents, outillage, contrôles)
3. normes/certifications applicables (ISO 27001, HDS, NF525, etc.)
4. niveau de sécurité requis par zone du scope
5. conclusion avec **niveau de sécurité maximal requis**

**Sorties obligatoires** :
- ajout du RPS dans le brief P0
- ajout dans `ressources/index.md` des compétences/certifications/procédures à charger en exécution

### P0 GPI — Planification sécurité pilotée par Victor

Après le Temps 5, Victor alimente la gouvernance de séquence (`<sequence>/gpi/`) avec un volet sécurité:

1. sélection des implémentations sécurité à réaliser pendant la séquence
2. planification (ordre, dépendances, critères de complétion)
3. recherche d'un prompt/protocole existant réutilisable
4. adaptation ou construction du prompt d'implémentation sécurité

**Traçabilité attendue** :
- décision GPI horodatée
- lien vers ressources certifications chargées
- lien vers tâches P3/P4 concernées

**Gate P0** (déroulement strict) :
1. Maria présente le brief complet à l'utilisateur
2. L'utilisateur **lit le brief** (TL;DR + contenu)
3. L'utilisateur **approuve ou rejette** le brief (approche, périmètre, plan)
4. **SI approuvé** : l'utilisateur choisit le mode d'autonomie (FULL/BIG_STEPS/GUIDED) — **choix éclairé** après lecture du brief (invariant I-4)
5. **SI approuvé + mode choisi** : Maria exécute l'initialisation des artefacts standard + mini-site JSX :
   `powershell -ExecutionPolicy Bypass -File .mip/scripts/init-sequence-standard-artifacts.ps1 -SequencePath <sequence>`
6. L'exécution démarre

**Gate stricte** : Pas de passage en exécution sans brief approuvé. Pas de choix d'autonomie sans lecture préalable du brief.

### Suivi P0 — Annonces temps réel

Chaque Temps est **suivi via TodoWrite** (ou liste todo tenue par l'agent orchestrateur si TodoWrite absent). À la complétion, l'agent annonce :

```
[YYYY-MM-DD HH:MM] P0 Temps X — <Nom> terminé.
  Agent(s) : <liste>
  Résultat : <résumé 1-2 lignes>
  Prochain Temps : Temps X+1 — <Nom>
```

---

## Modes d'autonomie

Le mode détermine **combien de gates humaines** existent entre P0 et P5.

| Mode | Comportement | Gates | Idéal pour |
|------|--------------|-------|------------|
| **FULL** | Autopilot complet après P0 | P0 → P5 (test humain) | T3, projets bien cadrés |
| **BIG_STEPS** | Automatique par phase, gates entre phases | P0 → P3→P4 → P4→P5 → P5 | T4, premiers projets |
| **GUIDED** | Validation à chaque étape | P0 → chaque étape → P4 → P5 | T5, supervision rapprochée |

**Persistance** : `.mip/memory/user-profile.md` section « Préférences de travail ». Modifiable via `/autonomy_mode <mode>`.

**En mode FULL** : dernière interaction humaine = approbation brief P0 + choix d'autonomie. Sauf frein d'urgence.

**Déroulement Gate P0** : brief présenté → utilisateur lit → utilisateur approuve → utilisateur choisit le mode d'autonomie. Le choix d'autonomie est la **dernière question** de la Gate P0, jamais la première.

---

## P3 — Implémentation (toutes classes)

> Détail complet : `.mip/modules/p3-execution.md`

**Agents** : François (back) + Lise (front) en PARALLÈLE. Denis coordonne.

**Concepts clés** :
- **Test fumée prioritaire** : Test e2e happy path qui COMPILE mais ÉCHOUE — valide la structure du plan avant TDD
- **Cycle TDD 10 pas** : START → RED → GREEN → REFACTOR → VERIFY → LINT → COMMIT → PUSH → LOG → TRACK
- **Subagent frais par tâche** : Éviter la pollution de contexte
- **Checkpoint toutes les 5 tâches** : Mini-audit Denis + spot-check sécurité Victor + spot-check efficience Jean + push
- **Spot-check docs** : Vérifier les patterns framework avant d'écrire du code
- **Auto-correction** : Cause racine → vérif docs → tentative 1 → tentative 2 → frein d'urgence

**Gate P3** : Chaque tâche passe test + lint.

**Gate BIG_STEPS (P3→P4)** : Denis présente le résumé (tâches, tests, auto-corrections). Utilisateur : CONTINUER / CORRIGER / ARRÊTER.

---

## P4 — Intégration, audit et sécurité (T3+)

> Détail complet : `.mip/modules/p4-p5-p6.md`

**Agents** : Denis (intégration) + George (conformité) + Victor (sécurité /100) + Hugo (déploiement, T4-T5) + Jean (efficience tokens)

- **Denis** : Build/test/lint workspace complet. Auto-correction, frein d'urgence si 2 échecs.
- **George** : Audit conformité (build, tests, lint, annotations, Lois d'Autonomie, UX).
- **Victor** : Score sécurité /100 (5 critères x /20 : auth, crypto, validation, deps, logging). Défaut critique = BLOQUANT.
- **Hugo** (T4-T5) : Build prod, Docker, CI/CD, health checks, config prod vs dev.
- **Jean** : Audit efficience tokens — consommation vs budget, anomalies, rapport dans `<sequence>/audits/`.

### P4 Audit sécurité — Orchestration Victor (PASS -> RAS)

Le flux d'audit sécurité P4 est normé comme suit :

1. Victor planifie l'audit en s'appuyant sur le RPS et l'implémentation réalisée.
2. Victor produit le plan d'audit sécurité **PASS-0** (liste des tâches, horodatage début/fin attendu).
3. Exécution des tâches d'audit en MASS ou séquentiel :
   1 tâche = 1 agent auditeur = 1 rapport `PASS-XX`.
4. Chaque agent auditeur charge les ressources adaptées à sa tâche depuis `ressources/index.md`.
5. Victor compile les `PASS-XX` dans un **Rapport d'Audit Sécurité (RAS)** unique.
6. Le RAS inclut : propositions de correction/amélioration, conclusion, note sécurité /100.

**Règle de rebouclage** :
- si brèche critique détectée **ou** note < 60/100, retour en cycle MIP avec RAS comme entrée de reprise (P0 Temps 1).

**Gate P4** : 0 défaut BLOQUANT + score sécurité conforme.

**Gate BIG_STEPS (P4→P5)** : George + Victor présentent le résumé d'audit. Utilisateur : CONTINUER / CORRIGER / ARRÊTER.

---

## P5 — Livraison, test humain et validation

> Détail complet : `.mip/modules/p4-p5-p6.md`

**Agent** : Denis (livraison) + George (assistance test)

4 volets : Présentation livrable → Test humain → Questionnaire satisfaction → Décision.

**Verdict** : ACCEPTÉ (merge main + tag) / ACCEPTÉ AVEC RÉSERVES (merge + tâches futures) / REFUSÉ (boucle MIP, retour P0).

**Merge** : `git merge --no-ff feat/<slug>` + push + tag si release + nettoyage branche.

**Boucle MIP** : Si REFUSÉ → Maria reprend en P0 Temps 1 avec le feedback. Le brief précédent sert de référence.

---

## P6 — Rapport final, archivage et capitalisation (T3+)

> Détail complet : `.mip/modules/p4-p5-p6.md`

**Agents** : Arianne + Jean (refactorisation mémoire)

3 volets : Rapport final (`<sequence>/rapports_finaux/`) → Archivage artefacts → Capitalisation (`.mip/memory/`) + Jean refactorise la mémoire avec Arianne.

**Rapport** : Notes /20 sur 8 critères, trace d'exécution extraite du plan annoté, métriques tokens/durée, résumé dev, profil utilisateur, capitalisation agents. **Un rapport sans trace d'exécution est INCOMPLET.**

**Capitalisation** : Patterns → `patterns-and-lessons.md`, anti-patterns → `patterns-and-lessons.md`, notes → `mip-performance-history.md`, profil → `user-profile.md`.

---

## Règles non négociables (essentielles)

> Les règles ci-dessous ne sont pas dérivables du protocole lui-même. Le protocole (gates P0, TDD, checkpoints, etc.) est lui-même non négociable.

1. **Vérification docs obligatoire** (T3+) — Vérifier la doc des libs avant de coder (Context7, recherche web ou fallback si indisponible)
2. **Anti-patterns chargés** — Lire `.mip/memory/patterns-and-lessons.md` et `.mip/memory/MEMORY.md` avant chaque sprint
3. **Subagent frais par tâche** — Éviter la pollution de contexte entre tâches
4. **Pas de code dangereux en prod** — Pas de `unwrap()` (Rust), `any` (TS), `bare except` (Python)
5. **Archivage systématique** (T3+) — Arianne capitalise après chaque livraison
6. **TL;DR obligatoire** — Chaque artefact MIP commence par un résumé de 5 lignes max
7. **Phase SETUP obligatoire** — `.mip/environment.md` doit exister avant le premier P0
8. **environment.md comme référence** — Commandes build/test/lint lues depuis `.mip/environment.md`, pas en dur
9. **Profils MIP** — Basculer d'outil/LLM via `/mip_profile <slug>`. MIP adapte ses capacités (parallélisme, TodoWrite, MCP, terminal) selon le profil actif (`.mip/profiles/active`)
10. **Analyse sécurité obligatoire** (T3+) — Victor analyse les surfaces d'attaque AVANT la spec (Temps 5)
11. **Audit sécurité avant livraison** (T3+) — Victor score /100 en P4. Défaut critique = BLOQUANT
12. **Vérification infra** (T4-T5) — Hugo évalue l'infra en P0 Temps 4 et vérifie en P4
13. **Annotation du plan obligatoire** — Chaque tâche annotée : `Démarré à HH:MM:SS. Terminé à HH:MM:SS avec [model] pour N tokens (mesurés).` — ZÉRO tilde, ZÉRO estimation
14. **Métriques consommation obligatoires** — Le rapport P6 DOIT inclure tokens agrégés, durée, indicateurs efficience. **Valeurs mesurées uniquement** (notifications de tâches + horodatages fichiers). Aucune estimation.
15. **Horodatage T0** — Maria horodate le premier prompt au début de P0
16. **ZÉRO estimation dans le rapport P6** — Les mots `~`, `environ`, `estimation`, `approximatif` sont INTERDITS dans les sections métriques. Si donnée manquante, écrire `null` + raison. Ne jamais fabriquer de valeur. Sources autorisées : notifications de tâches (tokens, durée), fichiers (horodatages), comptages (lignes, fichiers). Détail : `.mip/modules/metrics.md`
17. **Artefacts de sequence modulaires, 400 lignes max** — Limite stricte pour brief, spec, plan, audit, rapport. Les modules/profils/references de protocole ne sont pas concernes par cette limite.

---

## Efficience tokens — Connaissances pré-indexées

Chaque agent charge **uniquement ses fichiers pertinents** au début de chaque tâche :

| Agent (rôle) | Fichiers à charger |
|--------------|-------------------|
| **Dev Back-End** (François) | `stack-patterns.md`, `api-contracts.md`, `test-templates.md`, `code-annotations-templates.md` |
| **Dev Front-End** (Lise) | `stack-cheatsheet.md`, `api-contracts.md`, `project-file-map.md`, `code-annotations-templates.md` |
| **Chef Dev** (Denis) | `project-file-map.md`, `stack-patterns.md`, `mip-decisions.md`, `patterns-and-lessons.md` |
| **Expert Audit** (George) | `project-file-map.md`, `code-annotations-templates.md`, `patterns-and-lessons.md` |
| **Expert Cybersécurité** (Victor) | `security-patterns.md`, `patterns-and-lessons.md`, `stack-patterns.md`, `project-file-map.md` |
| **DevOps & Infra** (Hugo) | `project-file-map.md`, `.mip/environment.md` (section Infrastructure), `mip-decisions.md` |
| **Responsable Efficience IA** (Jean) | `mip-performance-history.md`, `MEMORY.md`, `<sequence>/metrics/` |
| **Team Manager** (Arianne) | `mip-decisions.md`, `patterns-and-lessons.md`, `mip-performance-history.md`, `team-skills-audit.md` |

> Tous ces fichiers sont dans `.mip/memory/` (sauf environment, metrics).

---

## Documentation des librairies — Registre de vérification

> Registre projet-spécifique. IDs pré-résolus lors du SETUP ou du premier P0.

### Protocole universel

- **Toujours** pendant P0 Temps 6 (spec) pour chaque lib impliquée
- **Spot-check** en P3 si la tâche touche un pattern spécifique
- **En cas d'erreur** : vérifier si le pattern utilisé est encore valide
- **Premier P0** : résoudre les IDs de toutes les libs via `resolve-library-id`

---

## MASS — Agent Swarm System

> Détail complet : `.mip/modules/mass.md`. Actif pour T4-T5.

**TL;DR** : MASS exécute les tâches MIP en parallèle via un DAG de dépendances décomposé en vagues. 3 couches : Orchestrateur (Maria/DAG) → Pool Workers (agents) → Synchronisation (Denis/merge). **Loi 9** : si >3 tâches indépendantes dans une vague, parallélisation OBLIGATOIRE.

3 modes de dispatch :
- **Subagent burst** : T2-T3 ou vague <=3 tâches
- **Worktree swarm** : T4 ou vague >3 tâches avec fichiers disjoints
- **Team swarm** : T5, flag Agent Teams expérimental

DAG dans `<sequence>/phases/dag.json`. Métriques swarm dans `<sequence>/metrics/`.

---

## Raccourcis pour tâches simples

**T1 (micro-fix)** : Pas de brief ni spec. Corriger directement, tester, committer.
**T2 (fix cible)** : Denis écrit un mini-plan (1-3 tâches), exécution directe. Feature branch si >1 fichier.

Le protocole est **proportionnel** : les petites tâches ne sont pas alourdies.

---

## Workflow Git

**Convention de branche** : `feat/<slug>` (T3-T5) | `fix/<slug>` (T1-T2) | `refactor/<slug>` (T3+). Slug dérivé du titre du brief.

**Première action AUTOPILOT** :
```bash
git checkout -b feat/<slug>
git push -u origin feat/<slug>
```

**Push** : Après chaque commit (TDD pas 8) + après chaque checkpoint Denis.

**Merge** : `git merge --no-ff feat/<slug>` → main, après verdict ACCEPTÉ en P5.

---

## Le plan comme document de suivi (OBLIGATOIRE)

Le plan exhaustif (`<sequence>/plans_p3/`) sert **à la fois** de plan d'exécution ET de journal de suivi. Chaque agent annote directement le plan :

```
> Démarré à HH:MM:SS. Terminé à HH:MM:SS avec [model] pour N tokens (mesurés).
```

**ZÉRO estimation** : Le `N` est la valeur réelle extraite de la notification de tâche de l'agent, pas une approximation. Si les tokens ne sont pas encore disponibles, écrire `tokens: à compléter` et l'orchestrateur remplit après réception de la notification. Voir `.mip/modules/metrics.md` pour les méthodes de collecte.

Le plan annoté est la **source unique de vérité** pour le rapport P6. Arianne lit le plan annoté — pas besoin de reconstituer la trace.

---

## Frein d'urgence

L'autopilot s'arrête UNIQUEMENT si :
1. **Bug bloquant** : le test échoue après 2 tentatives de correction automatique
2. **Delta majeur** : problème qui remet en question le plan
3. **Échec audit** : défaut CRITIQUE non corrigeable automatiquement

L'agent qui détecte le problème **arrête l'autopilot** et **présente le problème à l'utilisateur** avec une proposition de résolution.
