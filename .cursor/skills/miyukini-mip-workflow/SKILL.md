# Skill: MIP v2 — Miyukini Implementation Protocol

## Quand utiliser ce skill

Utiliser ce skill pour **toute demande de developpement** impliquant du code, une nouvelle fonctionnalite, un fix, un refactor, ou un nouveau crate/service. Le protocole MIP v2 orchestre l'equipe et structure le travail.

---

## Etape 1 — Classification (OBLIGATOIRE)

Avant toute action, classer la demande :

| Classe | Critere | Phases |
|--------|---------|--------|
| **T1** | Micro-fix, 1 fichier, <20 lignes | P3 → P5 |
| **T2** | Fix cible, 1-3 fichiers, bug connu | P2 → P3 → P5 |
| **T3** | Feature moderee, 3-10 fichiers | P0 → P3 → P4 → P5 → P6 |
| **T4** | Feature majeure, 10+ fichiers, multi-crate | P0 → P3 → P4 → P5 → P6 |
| **T5** | Chantier strategique, nouveau crate/app | P0 → P3 → P4 → P5 → P6 |

**Regle** : En cas de doute, classer UN CRAN AU-DESSUS.

**Qui classifie** : Maria (Chef de Projet). En son absence, l'utilisateur ou le coordinateur Claude.

---

## Etape 2 — Routing des phases

### P0 — Cadrage complet : Brainstorming, Analyse, Specification & Planification (T3+)

**Agents** : Maria (lead) + Lise (direction visuelle) + Fabrice (analyse PR, T4-T5) + Francois (spec technique) + Denis (plan exhaustif) + Arianne (audit de faisabilite)

P0 est **LA phase humaine** : elle determine la direction de tout le travail. Aucun code ne sera ecrit avant la fin de P0. Le brainstorming est **structure en 7 temps**. Apres approbation du brief P0, **tout est automatique** (P3 → P6).

#### Temps 1 — Exploration & Brainstorming structure (Maria)

Maria reformule la demande, creuse le contexte, et guide l'utilisateur a travers un **questionnaire de brainstorming structure** inspire de methodes reconnues (Design Thinking, Six Thinking Hats, SCAMPER, 5 Whys, How Might We).

**Etapes** :

1. **Reformuler** la demande utilisateur en termes precis
2. **Classifier** la demande (T1-T5)
3. **Explorer le code existant** : lire les fichiers concernes (Glob, Grep, Read) pour comprendre l'etat actuel
4. **Administrer le questionnaire de brainstorming** (voir ci-dessous) — poser les questions par section, adapter selon la classe T et le contexte
5. **Identifier les contraintes** : Lois d'Autonomie applicables, stack technique, compatibilite existante

**Hard gate** : NE PAS passer au temps 2 sans reponses de l'utilisateur.

---

#### Questionnaire de Brainstorming Standard — P0

> **Objectif** : Guider l'utilisateur pour extraire les informations et decisions necessaires au cadrage du projet. Chaque section s'inspire d'une methode de brainstorming reconnue. Maria **adapte les questions** au contexte : certaines sont universelles, d'autres reservees aux projets complexes (T4-T5). Les questions marquees `[OPT]` sont optionnelles pour T3.

##### Section 1 — COMPRENDRE : Le probleme et son contexte
*Inspire de : Design Thinking (Empathize/Define) + 5 Whys*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 1.1 | **Quel probleme ou besoin cette demande resout-elle ?** Decrivez la situation actuelle et ce qui ne va pas ou manque. | Design Thinking: Empathize | T3-T5 |
| 1.2 | **Pourquoi maintenant ?** Qu'est-ce qui declenche cette demande aujourd'hui ? (urgence, opportunite, prerequis pour un autre projet...) | 5 Whys (niveau 1) | T3-T5 |
| 1.3 | **Qui est l'utilisateur final ?** Qui va utiliser cette fonctionnalite au quotidien ? (vous-meme, un autre utilisateur, un service, une API...) | Design Thinking: Define | T3-T5 |
| 1.4 | **Quel est le parcours actuel ?** Si une solution partielle existe deja, comment l'utilisateur fait-il aujourd'hui ? Quels sont les points de friction ? | Design Thinking: Empathize | T3-T5 |
| 1.5 | `[OPT]` **Pourquoi cette approche plutot qu'une autre ?** Si vous avez deja une idee de solution, qu'est-ce qui vous y a mene ? (creuser les "pourquoi" sous-jacents) | 5 Whys (niveaux 2-3) | T4-T5 |

##### Section 2 — CADRER : Faits, contraintes et priorites
*Inspire de : Six Thinking Hats (White Hat: faits, Blue Hat: processus)*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 2.1 | **Quelles sont les contraintes techniques connues ?** (stack, versions, dependances, performance, compatibilite, plateforme...) | White Hat: Faits | T3-T5 |
| 2.2 | **Quel est le perimetre souhaite ?** Listez ce qui doit etre INCLUS et ce qui est EXCLU explicitement. | Blue Hat: Processus | T3-T5 |
| 2.3 | **Quelle est la priorite ?** Classez par importance : (a) fonctionnalite minimale viable, (b) ameliorations souhaitees, (c) bonus / nice-to-have. | Blue Hat: Processus | T3-T5 |
| 2.4 | `[OPT]` **Y a-t-il une deadline ou un jalon externe ?** (release, demo, dependance d'un autre projet...) | White Hat: Faits | T4-T5 |
| 2.5 | `[OPT]` **Quelles donnees ou references avez-vous ?** (maquettes, specs existantes, exemples, liens, captures d'ecran...) | White Hat: Faits | T4-T5 |

##### Section 3 — IMAGINER : Idees, alternatives et inspiration
*Inspire de : Six Thinking Hats (Green Hat: creativite) + SCAMPER*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 3.1 | **Avez-vous deja des idees ou preferences d'approche technique ?** Decrivez meme partiellement — toute piste est utile. | Green Hat: Creativite | T3-T5 |
| 3.2 | **Existe-t-il dans le projet quelque chose de similaire qu'on pourrait adapter ?** (un service, composant, pattern, crate existant...) | SCAMPER: Adapter | T3-T5 |
| 3.3 | `[OPT]` **Peut-on combiner avec une fonctionnalite existante ou prevue ?** (fusionner deux besoins en un seul dev) | SCAMPER: Combiner | T4-T5 |
| 3.4 | `[OPT]` **Que peut-on eliminer pour simplifier ?** Y a-t-il des aspects non-essentiels qu'on pourrait retirer pour un MVP plus rapide ? | SCAMPER: Eliminer | T4-T5 |
| 3.5 | `[OPT]` **Connaissez-vous des produits/services qui font quelque chose de similaire ?** (inspiration concurrence, references visuelles ou fonctionnelles) | SCAMPER: Adapter | T4-T5 |
| 3.6 | `[OPT]` **"How Might We..."** — Comment pourrait-on reformuler le probleme en opportunite ? (ex: "Comment pourrait-on rendre le partage de fichiers aussi simple qu'un glisser-deposer ?") | How Might We | T5 |

##### Section 4 — EVALUER : Risques, benefices et intuition
*Inspire de : Six Thinking Hats (Yellow Hat: valeur, Black Hat: risques, Red Hat: intuition)*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 4.1 | **Quel est le benefice principal attendu ?** Une fois livre, quelle est LA chose qui doit fonctionner ? | Yellow Hat: Valeur | T3-T5 |
| 4.2 | **Quels risques ou difficultes anticipez-vous ?** (techniques, UX, compatibilite, performance, securite...) | Black Hat: Risques | T3-T5 |
| 4.3 | **Quelle est votre intuition sur la complexite ?** (simple / modere / complexe / tres complexe) | Red Hat: Intuition | T3-T5 |
| 4.4 | `[OPT]` **Quelle importance strategique ?** (1 = utilitaire, 5 = critique pour l'ecosysteme Miyukini) | Red Hat: Intuition | T4-T5 |
| 4.5 | `[OPT]` **Que se passe-t-il si on ne fait PAS ce projet ?** (impact de l'inaction) | Reverse Brainstorming | T4-T5 |

##### Section 5 — DECIDER : Arbitrages et priorites
*Inspire de : Lightning Decision Jam (LDJ)*

| # | Question | Methode | Classes |
|---|----------|---------|---------|
| 5.1 | **Quelle est la fonctionnalite MINIMALE viable ?** Si on ne pouvait livrer qu'une seule chose, ce serait quoi ? | LDJ: Prioriser | T3-T5 |
| 5.2 | **Preference de compromis ?** En cas de tension, que privilegier : (a) rapidite de livraison, (b) exhaustivite fonctionnelle, (c) robustesse/qualite ? | LDJ: Arbitrer | T3-T5 |
| 5.3 | `[OPT]` **Qu'est-ce qui peut etre reporte a un prochain sprint ?** (fonctionnalites phase 2, optimisations, polish...) | LDJ: Reporter | T4-T5 |
| 5.4 | `[OPT]` **Y a-t-il des decisions deja verrouillees ?** (choix techniques, patterns, conventions qui ne sont pas negociables pour ce projet) | LDJ: Contraindre | T4-T5 |

---

**Utilisation par Maria** :

- **T3** : Poser les questions non-`[OPT]` (12 questions). Adapter selon le contexte — si la reponse est evidente, ne pas insister.
- **T4** : Poser toutes les questions (20 questions). Regrouper en 2-3 messages pour ne pas submerger l'utilisateur.
- **T5** : Poser toutes les questions (21 questions) + question HMW (3.6). Accepter des reponses longues et encourager la reflexion.
- **Boucle MIP** (retour P5 → P0) : Ne re-poser que les sections 1 et 4 en les orientant sur les **ecarts constates** et les **corrections souhaitees**.

**Hard gate inchangee** : NE PAS passer au temps 2 sans reponses suffisantes de l'utilisateur.

#### Temps 2 — Ideation (Maria + Lise en parallele)

Deux explorations paralleles :

**Maria** — Cadrage fonctionnel :
1. Lister les **objectifs** (principal + secondaires)
2. Definir le **perimetre** : IN / OUT explicite
3. Identifier les **risques** et leurs mitigations
4. Proposer **2-3 approches** techniques differentes avec pros/cons

**Lise** (T3+ si la tache a un aspect front/UI) — Vision graphique :
1. Analyser l'**UI existante** (theme, composants, patterns visuels en place)
2. Proposer la **direction artistique** : style, ton, inspirations visuelles
3. Decrire le **parcours utilisateur** (flux ecran par ecran, interactions)
4. Identifier les **composants** a creer/reutiliser (atomic design : atomes, molecules, organismes)
5. Si pertinent, referencer des **inspirations visuelles** (apps concurrentes, design systems)

#### Temps 3 — Analyse concurrentielle (Fabrice, T4-T5 seulement)

**Fabrice** (lance en parallele du temps 2 pour T4-T5) :
1. Identifier les **produits/services concurrents**
2. Analyser **forces et faiblesses** de chaque concurrent
3. Identifier la **cible utilisateur** et ses attentes
4. Lister les **fonctionnalites differenciantes** a envisager
5. Detecter les **points de friction** des concurrents

#### Temps 4 — Specification technique + Verification Context7 (Francois)

**Francois** analyse le contexte technique, **verifie les docs actuelles**, et produit la spec :

1. **Explorer le code existant** en profondeur (Glob, Grep, Read)
2. **Verification Context7 obligatoire** — Pour chaque librairie/framework implique :
   - Appeler `resolve-library-id` pour identifier la lib
   - Appeler `query-docs` pour verifier les patterns/API actuels
   - **Libs a toujours verifier** : Dioxus (`/dioxuslabs/dioxus`), axum (`/tokio-rs/axum`), serde (`/serde-rs/serde`), tokio, et toute lib ajoutee
   - Documenter les **breaking changes** ou **deprecations** detectees
   - Comparer avec les patterns existants dans le code — signaler les ecarts
3. **Charger les anti-patterns connus** : Lire `memory/mip-antipatterns.md` et `memory/MEMORY.md` (section "Erreurs a ne pas repeter") — verifier qu'aucun pattern interdit n'est planifie
4. **Identifier les fichiers** a modifier/creer avec numeros de ligne
5. **Definir les types, traits, API** (signatures completes — validees contre les docs Context7)
6. **Evaluer les dependances** entre modules et crates
7. **Verifier la conformite architecturale** :
   - [ ] Lois d'Autonomie respectees (LOI-1 a LOI-8)
   - [ ] `unsafe_code = "forbid"` dans tout nouveau Cargo.toml
   - [ ] Strate correcte dans la pyramide COG
   - [ ] Annotations MSCM planifiees (@id, @do, @role, @layer)
   - [ ] Versions des dependances a jour (pas de crates deprecated)
8. **Documenter** les risques techniques identifies

**Output supplementaire** : Section "Verification documentaire" dans la spec avec :
- Libs verifiees + versions
- Breaking changes detectes
- Anti-patterns evites
- Ecarts code existant vs docs actuelles

Artefact : `.mip/specs/YYYY-MM-DD-<slug>.md`

#### Temps 5 — Plan general de developpement exhaustif (Denis)

**Denis** compile la spec de Francois et produit le **plan exhaustif** couvrant TOUTE la chaine de production :

1. **Decomposer en taches atomiques** (2-5 minutes chacune)
2. **Couvrir exhaustivement** les categories suivantes :

| Categorie | Contenu |
|-----------|---------|
| **Code** | Implementation back-end (Francois) + front-end (Lise) |
| **Tests unitaires** | Un test minimum par fonction/methode ajoutee |
| **Tests d'integration** | Tests de flux complets (API, UI flows) |
| **Tests generaux** | `cargo test --workspace`, `cargo clippy --workspace -- -D warnings` |
| **Audit** | Checklist George (MSCM, securite, UX, conformite) |
| **Corrections** | Taches de correction pre-planifiees (buffer 20% des taches) |

3. **Chaque tache DOIT contenir** :
   - Numero sequentiel et categorie (`[CODE-01]`, `[TEST-U-01]`, `[TEST-I-01]`, `[AUDIT-01]`, etc.)
   - Agent assigne (Francois, Lise, Denis, George)
   - Fichier(s) exact(s) a modifier (chemin complet)
   - Code complet a ecrire (pas de "ajouter de la validation")
   - Commande de test : `cargo test -p {crate} -- {pattern}`
   - Output attendu : `"test xxx ... ok"`
   - Message de commit : `"type(scope): description"`
   - Dependances : liste des taches prerequises (ex: `depends: [CODE-01, CODE-02]`)

4. **Principe** : Presumer que l'executant n'a AUCUN contexte projet.

5. **Ordonnancement** : Les taches sont ordonnees par dependance. Les taches independantes sont marquees comme parallelisables.

Artefact : `.mip/plans/YYYY-MM-DD-<slug>.md`

#### Temps 6 — Audit de faisabilite & Conformite (Arianne)

**Arianne** verifie que le projet est **realisable tel que planifie**, que les agents, dependances et outils sont conformes, et qu'il n'y a ni trou ni ambiguite.

**Verification des agents** :
1. **Agents necessaires** : Verifier que chaque tache du plan a un agent assigne et que cet agent possede les competences requises (consulter `memory/team-skills-audit.md`)
2. **Capacite du modele** : Evaluer si le modele LLM utilise est capable de la complexite des taches planifiees. Si risque de deviation → recommander un modele different ou un decoupage plus fin
3. **Coherence inter-agents** : Verifier que les outputs attendus de chaque agent correspondent aux inputs attendus par les agents suivants (pas de gap)

**Verification des dependances** :
4. **Crates externes** : Verifier que toutes les dependances listees dans la spec existent, sont maintenues, et sont compatibles entre elles (versions)
5. **Crates internes** : Verifier que les crates du workspace utilises existent et que les types/traits references sont bien definis
6. **Outils** : Verifier que tous les outils necessaires au dev sont disponibles (compilateur, Context7 IDs, outils CLI, assets)

**Verification contre la memoire** :
7. **Anti-patterns** : Relire `memory/mip-antipatterns.md` — verifier qu'aucune tache ne reproduit une erreur connue
8. **Patterns confirmes** : Relire `memory/mip-decisions.md` — verifier que les patterns confirmes sont bien utilises
9. **Historique** : Consulter `memory/mip-performance-history.md` — si un projet similaire a deja ete fait, en tirer des lecons

**Verification Context7** (complement de Francois) :
10. **Spot-check** : Verifier via Context7 que 2-3 patterns critiques du plan sont bien valides (ex: RSX signal patterns, axum middleware, serde derives)
11. **Breaking changes recents** : Verifier si les libs ont ete mises a jour depuis la derniere sequence MIP

**Diagnostic** :

| Resultat | Action |
|----------|--------|
| **Conforme** | Feu vert → Maria compile le brief (Temps 7) |
| **Trous mineurs** | Lister les manques, suggerer les complements, corriger le plan |
| **Ambiguite** | Identifier les points flous, poser des questions a l'utilisateur ou a l'agent concerne |
| **Manque critique** (outil, crate, skill agent) | Suggerer la **creation des manquants** comme projet precurseur |
| **Projet irrealisable tel quel** | Suggerer une **reorientation** : decomposer en un projet precurseur (prereqs) + projet final |

**Suggestion de projet precurseur** : Si Arianne detecte qu'il manque un crate, un outil, ou une competence pour realiser le projet, elle propose un **mini-projet precurseur** (T2-T3) a realiser d'abord, qui debloquera le projet principal. Le brief est alors modifie pour inclure cette dependance.

Artefact : Section "Audit de faisabilite" integree au brief (pas d'artefact separe en P0)

#### Temps 7 — Synthese & Brief (Maria)

Maria compile tout dans le brief final :

1. **Fusionner** les contributions de tous les agents (Maria + Lise + Fabrice + Francois + Denis + Arianne)
2. **Integrer l'audit d'Arianne** : section conformite, alertes, prerequis eventuels
3. **Rediger le brief structure** avec toutes les sections
4. **Presenter les approches** avec la recommandation de l'equipe
5. **Si projet precurseur detecte** : presenter les deux projets (precurseur + final) et demander l'ordre de priorite
6. **Inclure le plan exhaustif** de Denis en annexe du brief
7. Artefact : `.mip/briefs/YYYY-MM-DD-<slug>.md`

**Template du brief** :

```markdown
# Brief: <titre>

## Metadata
- Classe: T3/T4/T5
- Date: YYYY-MM-DD
- Demandeur: utilisateur

## Contexte
[Pourquoi cette demande, quel probleme elle resout]

## Objectifs
- Objectif principal: ...
- Objectifs secondaires: ...
- Criteres de succes mesurables: ...

## Perimetre
### Inclus
- [Fonctionnalites IN]
### Exclus
- [Fonctionnalites OUT — explicitement rejetees]

## Approches proposees
### Approche A — [nom] (RECOMMANDEE)
- Description: ...
- Pros: ...
- Cons: ...
- Effort: ...

### Approche B — [nom]
- Description: ...
- Pros: ...
- Cons: ...

## Direction visuelle (par Lise)
- Style/ton: ...
- Composants identifies: [atomes, molecules, organismes]
- Parcours utilisateur: [flux ecran par ecran]
- Inspirations: ...

## Analyse concurrentielle (par Fabrice, T4-T5)
- Concurrents: ...
- Differenciateurs: ...
- Cible utilisateur: ...

## Specification technique (par Francois)
- Fichiers modifies/crees: [liste avec numeros de ligne]
- Types et API definis: [signatures]
- Conformite: [checklist LOI, MSCM, unsafe]
- Risques techniques: [liste]

## Plan de developpement exhaustif (par Denis)
[Voir annexe .mip/plans/YYYY-MM-DD-<slug>.md]
- Nombre total de taches: X
  - Code: X taches (Y Francois, Z Lise)
  - Tests unitaires: X taches
  - Tests integration: X taches
  - Tests generaux: X taches
  - Audit: X taches
  - Buffer corrections: X taches

## Audit de faisabilite (par Arianne)
### Conformite agents
- Agents necessaires: [liste avec competences verifiees]
- Capacite modele LLM: [OK / risque identifie]
- Coherence inter-agents: [OK / gaps identifies]

### Conformite dependances
- Crates externes: [toutes verifiees / manquants]
- Crates internes: [tous presents / manquants]
- Outils: [tous disponibles / manquants]

### Verification memoire
- Anti-patterns evites: [liste]
- Patterns confirmes appliques: [liste]
- Lecons historiques: [si applicable]

### Verdict faisabilite
- **CONFORME** / **TROUS MINEURS** (corriges) / **PREREQUIS NECESSAIRE**
- Si prerequis: [description du projet precurseur]

## Contraintes
- Lois d'Autonomie: LOI-x applicables
- Stack: ...
- Compatibilite: ...

## Risques
| Risque | Probabilite | Impact | Mitigation |
|--------|-------------|--------|------------|
| ... | ... | ... | ... |

## Decision
APPROUVE / REJETE / MODIFIE / PREREQUIS D'ABORD
```

**Quality Gate P0** : Utilisateur approuve le brief ET choisit l'approche.

**Hard gate** : AUCUN passage en execution sans brief approuve. C'est la **DERNIERE intervention humaine** avant la livraison (sauf bug/delta majeur).

---

## Metriques & Horodatage — Collecte continue

> Tout au long de la sequence MIP, les agents collectent des metriques pour mesurer la performance de l'equipe et alimenter le rapport final.

### Initialisation (debut de sequence)

A l'ouverture de chaque sequence MIP, Maria cree le fichier `.mip/metrics/YYYY-MM-DD-<slug>.json` avec la structure suivante :

```json
{
  "project": {
    "title": "<titre du brief>",
    "description": "<description courte>",
    "class": "T3|T4|T5",
    "slug": "<slug>",
    "mip_sequence_number": 1
  },
  "timestamps": {
    "p0_start": "ISO8601",
    "p0_end": null,
    "autopilot_start": null,
    "p3_start": null, "p3_end": null,
    "p4_start": null, "p4_end": null,
    "p5_start": null, "p5_end": null,
    "p5_test_start": null, "p5_test_end": null,
    "p6_start": null, "p6_end": null,
    "total_end": null
  },
  "counters": {
    "lines_written": 0,
    "lines_deleted": 0,
    "crates_touched": [],
    "crates_created": [],
    "files_created": 0,
    "files_modified": 0,
    "commits": 0,
    "agents_engaged": [],
    "mip_loops": 1,
    "unit_tests_total": 0,
    "unit_tests_failed": 0,
    "integration_tests_total": 0,
    "integration_tests_failed": 0,
    "global_tests_total": 0,
    "global_tests_failed": 0,
    "auto_corrections": 0,
    "audits": 0,
    "audit_defects": [],
    "emergency_brakes": 0
  },
  "human_interventions": [],
  "agent_questions": [],
  "satisfaction": null,
  "notes": null
}
```

### Collecte par phase

| Phase | Qui collecte | Quoi |
|-------|-------------|------|
| **P0** | Maria | `p0_start`, `p0_end`, `agents_engaged`, questions posees a l'humain |
| **Git** | Denis | `autopilot_start` |
| **P3** | Francois/Lise | `p3_start`, `p3_end`, `lines_written/deleted`, `commits`, `unit_tests_*`, `auto_corrections`, `crates_touched` |
| **P4** | Denis/George | `p4_start`, `p4_end`, `audits`, `audit_defects[]`, `global_tests_*`, `integration_tests_*` |
| **P5** | Denis | `p5_start`, `p5_end`, `p5_test_start`, `p5_test_end`, `satisfaction`, `human_interventions[]` |
| **P6** | Arianne | `p6_start`, `p6_end`, `total_end`, compilation du rapport final |

### Enregistrement des interventions humaines

Chaque intervention humaine est loggee avec :
```json
{
  "timestamp": "ISO8601",
  "type": "precision|arret|pause|changement_direction|constat_erreur|delta|autre",
  "phase": "P0|P3|P4|P5",
  "description": "<description de l'intervention>",
  "impact": "aucun|mineur|majeur|critique"
}
```

### Enregistrement des questions agents → humain

Chaque question posee a l'utilisateur est loggee avec :
```json
{
  "timestamp": "ISO8601",
  "agent": "Maria|Denis|Francois|Lise|George|Arianne|Fabrice",
  "phase": "P0|P3|P4|P5",
  "nature": "clarification|validation|choix_technique|choix_design|blocage|autre",
  "question": "<texte de la question>",
  "response_summary": "<resume de la reponse>"
}
```

---

## MODE AUTOPILOT — P3 a P6 (apres approbation P0)

> **PRINCIPE FONDAMENTAL** : Apres l'approbation du brief P0, l'execution est **entierement automatique**. L'utilisateur n'intervient plus sauf en cas de **bug bloquant** ou de **delta majeur** par rapport au plan.

### Git Branch Setup (premiere action de l'AUTOPILOT)

Avant toute implementation, creer une branche de feature et la pousser sur le remote :

```bash
# Creer la branche depuis main
git checkout -b feat/<slug>    # slug = nom court de la feature (ex: feat/miyuvoice)

# Pousser la branche pour suivi distant
git push -u origin feat/<slug>
```

**Convention de nommage des branches** :
- `feat/<slug>` — Nouvelle fonctionnalite (T3-T5)
- `fix/<slug>` — Correction de bug (T1-T2)
- `refactor/<slug>` — Refactoring (T3+)

Le `<slug>` est derive du titre du brief (ex: brief "Ajouter MiyuVoice" → `feat/miyuvoice`).

### Logging obligatoire

**Chaque tache** du plan exhaustif est tracee via **TodoWrite** pour que l'utilisateur puisse suivre l'avancement en temps reel :

- Chaque tache commence par un `TodoWrite` qui la passe en `in_progress`
- Chaque tache terminee est immediatement marquee `completed`
- Les erreurs/blocages sont signales dans le statut de la tache
- L'utilisateur voit la progression sans avoir a intervenir

### Frein d'urgence

L'autopilot s'arrete UNIQUEMENT si :
1. **Bug bloquant** : un test echoue apres 2 tentatives de correction automatique
2. **Delta majeur** : une tache revele un probleme qui remet en question le plan (ex: API incompatible, dependance cassee)
3. **Echec audit** : George identifie un defaut CRITIQUE que Denis ne peut pas corriger automatiquement

Dans ces cas, l'agent qui detecte le probleme **arrete l'autopilot** et **presente le probleme a l'utilisateur** avec une proposition de resolution.

---

### P3 — Implementation automatique (toutes classes)

**Agents** : Francois (back-end) + Lise (front-end) en PARALLELE

**Execution par subagent frais** : Chaque tache est executee par un subagent frais pour eviter la pollution de contexte.

**Pre-flight par tache** (avant d'ecrire du code) :
1. **Lire la tache** du plan exhaustif (fichier, code attendu, test)
2. **Context7 spot-check** (si la tache touche une API externe ou un pattern framework) :
   - Verifier le pattern contre la doc actuelle via `query-docs`
   - Ex: avant d'ecrire un composant Dioxus → verifier RSX syntax (`/dioxuslabs/dioxus`)
   - Ex: avant d'ecrire un handler axum → verifier extractors (`/tokio-rs/axum`)
3. **Charger le contexte anti-patterns** : relire les pieges RSX (Lise) ou patterns DB (Francois) depuis MEMORY.md

**Cycle TDD par tache** :
1. **RED** — Ecrire le test qui echoue
2. **GREEN** — Ecrire le code minimal pour que le test passe
3. **REFACTOR** — Nettoyer si necessaire
4. **VERIFY** — `cargo test -p {crate}` passe
5. **LINT** — `cargo clippy -p {crate} -- -D warnings` propre
6. **COMMIT** — Commit atomique avec message conventionnel
7. **PUSH** — `git push` sur la feature branch (sauvegarde distante)
8. **LOG** — `TodoWrite` : marquer la tache `completed`

**Checkpoint intermediaire** : Toutes les **5 taches completees**, Denis lance un mini-audit :
- `cargo build -p {crate}` des crates modifies
- `cargo clippy -p {crate} -- -D warnings`
- Verifier que les taches precedentes ne sont pas cassees par les nouvelles
- Si regression detectee → corriger avant de continuer
- `git push` — pousser l'etat courant sur la feature branch

**Parallelisme** : Francois et Lise travaillent simultanement quand leurs taches sont independantes. Les taches avec dependances sont sequencees par Denis.

**Auto-correction** : Si un test echoue, l'agent :
1. Lit le message d'erreur et identifie la cause (root cause analysis)
2. Verifie contre Context7 si c'est un probleme de pattern/API
3. Corrige et re-teste (tentative 1)
4. Si echec → corrige differemment (tentative 2)
5. Si echec → **frein d'urgence** avec diagnostic complet

**Quality Gate P3** : Chaque tache passe test + clippy.

---

### P4 — Integration & Audit automatique (T3+)

**Agents** : Denis + George

**Denis** — Integration :
1. `cargo build --workspace`
2. `cargo test --workspace`
3. `cargo clippy --workspace -- -D warnings`
4. Verifier l'integration back + front
5. Si echec : corriger automatiquement, re-tester
6. Si echec apres 2 tentatives → **frein d'urgence**
7. **LOG** : `TodoWrite` pour chaque verification

**George** — Audit de conformite :
- [ ] Build workspace OK
- [ ] Tests workspace OK
- [ ] Clippy propre
- [ ] Pas de `unwrap()` en production (hors `#[cfg(test)]`)
- [ ] Pas d'URL hardcodees
- [ ] Pas de donnees sensibles en clair
- [ ] Annotations MSCM presentes sur les nouveaux fichiers
- [ ] Lois d'Autonomie respectees
- [ ] Parcours utilisateur coherent (si UI)

Artefact : `.mip/audits/YYYY-MM-DD-<slug>.md`

**Auto-correction** : Defauts NON-BLOQUANTS sont corriges automatiquement par Denis. Defauts CRITIQUES → **frein d'urgence**.

**Quality Gate P4** : George valide — 0 defaut BLOQUANT.

---

### P5 — Livraison, Test humain & Validation (toutes classes)

**Agent** : Denis (livraison) + George (assistance test)

#### Etape 1 — Presentation du livrable

1. **Commit final** si necessaire (message conventionnel)
2. **Push final** — `git push` sur la feature branch
3. **Horodater** : `p5_start` dans le fichier metriques
4. **Presenter le resume a l'utilisateur** :
   - Ce qui a ete fait (fonctionnalites implementees)
   - Nombre de fichiers crees/modifies, lignes ecrites
   - Tests passes (unitaires, integration, globaux)
   - Anomalies detectees et corrigees
   - Instructions pour tester le livrable (commandes, parcours utilisateur)

#### Etape 2 — Test humain

5. **Horodater** : `p5_test_start`
6. **L'utilisateur teste le livrable** dans son environnement
7. George fournit une **checklist de test** adaptee au projet :
   - [ ] Build OK (`cargo build --workspace`)
   - [ ] Lancement de l'application OK
   - [ ] Parcours utilisateur principal fonctionne
   - [ ] Cas limites testes (si applicable)
   - [ ] Performance acceptable
   - [ ] UI conforme a la direction visuelle (si applicable)

#### Etape 3 — Questionnaire de satisfaction

8. **Horodater** : `p5_test_end`
9. Denis presente le **questionnaire de satisfaction** :

```
## Questionnaire de satisfaction — <titre du projet>

### Conformite fonctionnelle
1. Le livrable correspond-il a votre demande initiale ? (OUI / PARTIELLEMENT / NON)
2. Si non/partiellement, quels ecarts constatez-vous ?

### Qualite percue
3. Le code est-il propre et comprehensible ? (1-5)
4. L'UI est-elle satisfaisante ? (1-5, si applicable)
5. La performance est-elle acceptable ? (1-5)

### Satisfaction globale
6. Note globale de satisfaction (1-5) :
   1 = Inacceptable, 2 = Insuffisant, 3 = Acceptable, 4 = Bon, 5 = Excellent
7. Commentaires libres :

### Verdict
- [ ] ACCEPTE — Merger vers main
- [ ] ACCEPTE AVEC RESERVES — Merger, mais corrections mineures a planifier
- [ ] REFUSE — Retour en correction (boucle MIP)
```

#### Etape 4 — Decision

**Si ACCEPTE ou ACCEPTE AVEC RESERVES** :

10. **Horodater** : `p5_end`
11. **Merger les reserves** dans une liste de taches futures si applicable
12. **Merge vers main** — processus standard Git :
    ```bash
    git checkout main
    git pull origin main
    git merge feat/<slug> --no-ff
    git push origin main
    ```
13. **Tag si release** : `git tag -a vX.Y.Z -m "description"` + `git push origin vX.Y.Z`
14. **Nettoyage** : supprimer la branche de feature
    ```bash
    git branch -d feat/<slug>
    git push origin --delete feat/<slug>
    ```
15. **LOG** : `TodoWrite` marquer livraison `completed`
16. **Enregistrer** la satisfaction dans le fichier metriques

**Si REFUSE — Boucle MIP** :

10. **Logger l'intervention humaine** : type `constat_erreur` ou `delta`, impact `majeur`
11. **Incrementer** `mip_loops` dans le fichier metriques
12. **NE PAS merger** — la feature branch reste en l'etat
13. **Retour en P0** avec le contexte suivant :
    - Problemes constates par l'utilisateur (verbatim)
    - Ecarts entre l'attendu et le livre
    - Metriques de la boucle precedente
    - Maria reprend en **Temps 1** avec les problemes comme input
    - Le brief precedent sert de reference (pas de repartir de zero)
14. **Nouvelle sequence AUTOPILOT** sur la meme feature branch (pas de nouvelle branche)

**Alternative PR** : Remplacer le merge direct par `gh pr create`. L'utilisateur merge manuellement.

**Quality Gate P5** : Verdict utilisateur = ACCEPTE ou ACCEPTE AVEC RESERVES.

---

### P6 — Rapport final, Archivage & Capitalisation (T3+)

**Agent** : Arianne

#### Etape 1 — Rapport final de developpement

Arianne compile toutes les metriques collectees et produit le **rapport final independant du livrable**.

Artefact : `.mip/reports/YYYY-MM-DD-<slug>-report.md`

**Template du rapport final** :

```markdown
# Rapport MIP — <titre du projet>

## 1. Identite du projet
- **Titre** : <titre>
- **Description** : <description courte>
- **Type** : T3/T4/T5 — <description du type>
- **Complexite** : <evaluation qualitative : simple / moderee / complexe / tres complexe>
- **Branche** : feat/<slug>

## 2. Chrono & Duree
- **Debut** : <YYYY-MM-DD HH:MM> (debut P0)
- **Fin** : <YYYY-MM-DD HH:MM> (fin P6)
- **Duree totale IRL** : <Xh Ymin>
- **Decomposition** :
  | Phase | Debut | Fin | Duree |
  |-------|-------|-----|-------|
  | P0 Cadrage | ... | ... | ... |
  | P3 Implementation | ... | ... | ... |
  | P4 Integration & Audit | ... | ... | ... |
  | P5 Livraison & Test | ... | ... | ... |
  | P5 Test humain | ... | ... | ... |
  | P6 Rapport & Archivage | ... | ... | ... |

## 3. Ressources
- **Modele LLM** : <nom du modele> (ex: Claude Opus 4.6)
- **Tokens utilises** : ~<estimation> (entree: X, sortie: Y)
- **Nombre de boucles MIP** : <N> (1 = pas de retour)

## 4. Production
- **Lignes ecrites** : <N>
- **Lignes supprimees** : <N>
- **Fichiers crees** : <N>
- **Fichiers modifies** : <N>
- **Crates touches** : <N> (<liste>)
- **Crates crees** : <N> (<liste>)
- **Commits** : <N>

## 5. Equipe
- **Agents engages** : <N> (<liste avec roles>)
  | Agent | Role | Phases | Taches |
  |-------|------|--------|--------|
  | Maria | Chef de Projet | P0 | ... |
  | ... | ... | ... | ... |

## 6. Interactions humaines
- **Interventions humaines** : <N>
  | # | Timestamp | Type | Phase | Description | Impact |
  |---|-----------|------|-------|-------------|--------|
  | 1 | ... | precision | P0 | ... | mineur |
  | ... | ... | ... | ... | ... | ... |

- **Questions agents → humain** : <N>
  | # | Timestamp | Agent | Phase | Nature | Question |
  |---|-----------|-------|-------|--------|----------|
  | 1 | ... | Maria | P0 | clarification | ... |
  | ... | ... | ... | ... | ... | ... |

## 7. Tests
### Tests unitaires
- **Total** : <N>
- **Erreurs** : <N> (<N> corrigees, <N> restantes)

### Tests d'integration
- **Total** : <N>
- **Erreurs** : <N>

### Tests globaux
- **Total** : <N>
- **Erreurs** : <N>

### Auto-corrections
- **Nombre d'erreurs auto-corrigees** : <N>
- **Freins d'urgence declenches** : <N>

## 8. Audits
- **Nombre d'audits** : <N>
  | # | Type | Defauts | Gravite | Nature | Resolution |
  |---|------|---------|---------|--------|------------|
  | 1 | conformite | ... | bloquant/non-bloquant | ... | corrige/accepte |
  | ... | ... | ... | ... | ... | ... |

## 9. Satisfaction utilisateur
- **Verdict** : ACCEPTE / ACCEPTE AVEC RESERVES / REFUSE (boucle N)
- **Note satisfaction** : <1-5>
- **Commentaires** : <verbatim>

## 10. Notation globale

| Critere | Note /20 | Commentaire |
|---------|----------|-------------|
| **Note globale** | /20 | Moyenne ponderee des notes ci-dessous |
| Vitesse de dev (vs historique MIP) | /20 | Comparaison avec les sequences precedentes |
| Qualite des interventions agents | /20 | Pertinence, precision, autonomie |
| Qualite du code | /20 | Lisibilite, patterns, clippy, tests |
| Qualite de gestion des erreurs | /20 | Detection, correction, prevention |
| Qualite des interactions utilisateur | /20 | Clarte, pertinence des questions, ecoute |
| Respect du protocole MIP | /20 | Gates, artefacts, logging, TDD |
| Qualite de l'indexation MSCM | /20 | Couverture, precision des annotations |

**Bareme** :
- 18-20 : Excellent — reference pour les futures sequences
- 14-17 : Bon — quelques axes d'amelioration
- 10-13 : Acceptable — ameliorations significatives necessaires
- 6-9 : Insuffisant — problemes majeurs a resoudre
- 0-5 : Inacceptable — remise en question du processus

**Methode de notation** : Arianne evalue sur base des metriques objectives (tests, erreurs, timings) et du feedback utilisateur. La note est comparee a l'historique stocke dans `memory/mip-performance-history.md`.

## 11. Resume du developpement
<Resume narratif : ce qui a ete fait, les difficultes rencontrees, les decisions prises, les points forts et faibles de la sequence>

## 12. Profil utilisateur — Apprentissages
- **Competences techniques observees** : <ce que l'utilisateur connait/maitrise>
- **Connaissances domaine** : <expertise metier observee>
- **Preferences de travail** : <style de communication, niveau de detail souhaite, degre d'autonomie attendu>
- **Points d'attention** : <sujets sensibles, exigences recurrentes>

## 13. Capitalisation agents
- **Patterns confirmes** : <nouveaux patterns a ajouter a mip-decisions.md>
- **Anti-patterns decouverts** : <erreurs a ajouter a mip-antipatterns.md>
- **Configurations agents** : <ajustements recommandes pour les agents>
- **Ameliorations protocole** : <suggestions d'evolution du MIP>
```

#### Etape 2 — Archivage des artefacts

1. Archiver les artefacts MIP (brief, spec, plan, audit, rapport) dans `.mip/`
2. Verifier que tous les artefacts sont complets et coherents

#### Etape 3 — Capitalisation

3. Extraire les apprentissages :
   - Patterns confirmes → `memory/mip-decisions.md`
   - Erreurs a eviter → `memory/mip-antipatterns.md`
   - Lecons par chantier → `memory/mip-lessons.md`
   - Competences par agent → `memory/team-skills-audit.md`
4. **Enregistrer les notes** dans `memory/mip-performance-history.md` pour comparaison future
5. **Enregistrer le profil utilisateur** dans `memory/user-profile.md` (cumulatif)
6. **Enregistrer les configurations agents** dans `memory/agent-tuning.md`
7. Mettre a jour `memory/MEMORY.md` (index, max 200 lignes)
8. **Horodater** : `p6_end`, `total_end`
9. **LOG** : `TodoWrite` marquer archivage `completed`

---

## Regles NON NEGOCIABLES

1. **Classification avant action** — Aucun code sans classification T1-T5
2. **Spec avant code** (T3+) — Pas d'implementation sans spec Francois (Temps 4)
3. **Plan exhaustif avant execution** (T3+) — Pas d'implementation sans plan Denis (Temps 5)
4. **Verification Context7 obligatoire** (T3+) — Verifier les docs des libs impliquees avant de coder
5. **Anti-patterns charges** — Lire `memory/mip-antipatterns.md` et MEMORY.md avant chaque sprint
6. **TDD obligatoire** — RED-GREEN-REFACTOR, pas d'exception
7. **Subagent frais par tache** — Eviter la pollution de contexte
8. **Checkpoint toutes les 5 taches** — Mini-audit intermediaire en P3
9. **Gates non-bypassables** — Chaque gate doit etre explicitement validee
10. **Artefacts obligatoires** — Chaque phase produit son artefact dans `.mip/`
11. **Clippy propre** — `cargo clippy -p {crate} -- -D warnings` apres chaque tache
12. **Pas de unwrap() en prod** — Uniquement dans `#[cfg(test)]`
13. **Archivage systematique** (T3+) — Arianne capitalise apres chaque livraison
14. **Logging obligatoire** — Chaque tache tracee via TodoWrite
15. **Autopilot apres P0** — Aucune intervention humaine sauf frein d'urgence
16. **Feature branch obligatoire** (T2+) — Tout travail sur branche, merge vers main apres validation
17. **Push regulier** — Chaque commit est pousse sur le remote pour sauvegarde
18. **Metriques obligatoires** — Horodatage et compteurs collectes tout au long de la sequence
19. **Test humain en P5** — L'utilisateur teste le livrable avant merge
20. **Questionnaire satisfaction** — Feedback structure avant decision de merge
21. **Boucle MIP si refus** — Retour en P0 avec les problemes constates, pas de merge
22. **Rapport final en P6** — Rapport complet independant du livrable, notes /20, capitalisation
23. **Audit faisabilite en P0** (T3+) — Arianne verifie agents, dependances, outils et memoire avant synthese
24. **Questionnaire brainstorming en P0** (T3+) — Maria administre le questionnaire standard (5 sections) en Temps 1 pour cadrer le projet

---

## Registre Context7 — Libs a verifier

Identifiants Context7 pre-resolus pour les libs du projet. Utiliser `query-docs` directement avec ces IDs.

| Lib | Context7 ID | Quand verifier |
|-----|-------------|----------------|
| **Dioxus 0.6** | `/dioxuslabs/dioxus/v0.6.3` | Tout composant UI, RSX, signals, hooks |
| **Dioxus docs** | `/llmstxt/dioxuslabs_learn_0_6_llms-full_txt` | Patterns avances, migration, pitfalls |
| **axum** | `/tokio-rs/axum/axum_v0_7_9` | Tout endpoint REST, middleware, extractors |
| **serde** | `/serde-rs/serde` | Serialization custom, derive macros, attributes |
| **Dioxus Components** | `/dioxuslabs/components` | Composants primitifs ARIA |

**Quand verifier** :
- **Toujours** en P0 Temps 4 (spec) pour chaque lib impliquee
- **Spot-check** en P3 si la tache touche un pattern specifique
- **En cas d'erreur** : verifier si le pattern utilise est encore valide

**Queries recommandees** :
- Dioxus : `RSX syntax`, `use_signal hooks`, `component props`, `event handlers`, `async spawn`
- axum : `Router handlers`, `extractors State Json`, `error handling`, `middleware layers`
- serde : `derive attributes`, `custom serialization`, `default values`, `rename`

---

## Integration SuperClaude

Ce protocole s'appuie sur les skills SuperClaude quand ils sont disponibles :

| Phase MIP | Skill SuperClaude | Usage |
|-----------|-------------------|-------|
| P0 (Temps 1-2) | `brainstorming` | Maria structure le brief (7 temps : exploration → ideation → analyse → spec → plan → audit faisabilite → synthese) |
| P0 (Temps 6) | `verification-before-completion` | Arianne verifie conformite agents, deps, outils, memoire |
| P0 (Temps 5) | `writing-plans` | Denis cree les taches atomiques exhaustives |
| P3 | `subagent-driven-development` | Execution par subagent frais |
| P3 | `test-driven-development` | Cycle RED-GREEN-REFACTOR |
| P3 | `systematic-debugging` | Root cause avant tout fix + auto-correction |
| P4 | `verification-before-completion` | George verifie |
| P5 | `finishing-a-development-branch` | Denis finalise + test humain + questionnaire |
| P6 | — | Arianne : rapport final + capitalisation + profil utilisateur |

---

## Raccourcis pour taches simples

**T1 (micro-fix)** : Pas besoin de brief ni spec. Corriger directement, tester, committer.
**T2 (fix cible)** : Denis ecrit un mini-plan (1-3 taches), execution directe.

Le protocole est **proportionnel** : les petites taches ne sont pas alourdies.

---

## Flux concret — Exemple T4

```
Utilisateur : "Je veux ajouter MiyuVoice"
  |
  +-- Maria (P0 Temps 1) : Classifie T4, explore code, pose questions
  |   [GATE] Attendre reponses utilisateur
  |
  +-- PARALLELE (Temps 2 + 3) :
  |   +-- Maria : Cadrage fonctionnel, 2-3 approches
  |   +-- Lise : Direction visuelle, parcours UX, composants
  |   +-- Fabrice : Analyse concurrence (Alexa, Siri, etc.)
  |
  +-- Francois (Temps 4) : Spec technique + VERIFICATION CONTEXT7
  |   +-- Context7 : Dioxus 0.6 RSX, axum handlers, serde patterns
  |   +-- Anti-patterns : charge MEMORY.md + mip-antipatterns.md
  |   +-- Output : spec + section "Verification documentaire"
  |
  +-- Denis (Temps 5) : Plan exhaustif (42 taches : 18 CODE, 12 TEST-U, 4 TEST-I, 3 TEST-G, 3 AUDIT, 2 CORRECT)
  |
  +-- Arianne (Temps 6) : Audit de faisabilite
  |   +-- Verification : agents, deps, outils, memoire, Context7 spot-check
  |   +-- Diagnostic : CONFORME / TROUS MINEURS / PREREQUIS
  |   +-- Si prerequis → suggere mini-projet precurseur
  |
  +-- Maria (Temps 7) : Synthese → Brief complet (inclut audit Arianne)
  |   [GATE] Utilisateur approuve le brief
  |
  +=== AUTOPILOT START (metriques initialisees) ============
  |
  +-- Git : git checkout -b feat/miyuvoice + git push -u origin
  |
  +-- P3 PARALLELE (automatique) :
  |   +-- Pre-flight : Context7 spot-check + anti-patterns par tache
  |   +-- Francois : Taches CODE back-end (TDD) → commit → push → metriques → TodoWrite
  |   +-- Lise : Taches CODE front-end (TDD) → commit → push → metriques → TodoWrite
  |   +-- [Checkpoint toutes les 5 taches : mini-audit Denis + push]
  |   +-- [Auto-correction intelligente : root cause + Context7 + 2 tentatives]
  |
  +-- P4 (automatique) :
  |   +-- Denis : Integration workspace (build/test/clippy)
  |   +-- George : Audit conformite → .mip/audits/ + metriques audit
  |   [Auto-correction defauts non-bloquants, frein d'urgence si critique]
  |
  +-- P5 (automatique) :
  |   +-- Denis : Push final + resume a l'utilisateur + instructions test
  |   +-- [Utilisateur teste le livrable]
  |   +-- Denis : Questionnaire satisfaction
  |   +-- [GATE] Verdict utilisateur :
  |       +-- ACCEPTE → merge main + push + tag + nettoyage branche
  |       +-- RESERVES → merge main + ajout taches futures
  |       +-- REFUSE → log intervention + increment boucle → retour P0
  |
  +-- P6 (automatique) : Arianne
  |   +-- Rapport final (notes /20, metriques, profil utilisateur)
  |   +-- → .mip/reports/ + memory/mip-performance-history.md
  |   +-- Capitalisation : anti-patterns, decisions, agent-tuning
  |   +-- Profil utilisateur → memory/user-profile.md
  |
  +=== AUTOPILOT END =====================================
```
