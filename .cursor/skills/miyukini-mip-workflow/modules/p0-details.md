# Module MIP — P0 Cadrage (10 Temps)

> Ce module est charge au debut de P0 pour les taches T3+.

---

## Regles de presentation P0 (BORNAGE)

Ces regles s'appliquent a TOUS les Temps de P0 pour garantir la stabilite de la presentation :

### R-P0-1 : Outil de questionnement
- **AskUserQuestion** est l'outil OBLIGATOIRE pour toute question a l'utilisateur en P0
- Max **4 questions par appel** AskUserQuestion (limite technique de l'outil)
- Chaque section du questionnaire brainstorming = **1 appel AskUserQuestion** (regrouper les questions de la section)
- Texte libre en chat = uniquement pour les annonces, resumes, et presentations (jamais pour poser des questions)

### R-P0-2 : Visibilite des resultats intermediaires
Apres chaque Temps, l'orchestrateur **annonce dans le chat** un resume de 3-5 lignes :
```
[YYYY-MM-DD HH:MM] P0 Temps X — <Nom> termine.
  Agent(s): <liste>
  Resultat: <resume 2-3 lignes>
  Prochain Temps: Temps X+1 — <Nom>
```
Les resultats detailles des agents paralleles (T2-T9) sont visibles dans les artefacts `.mip/`, pas dans le chat.

### R-P0-3 : Presentation du brief (Gate P0)
Deroulement strict en 4 taches — invariant I-4 :
1. **Ecrire** le brief dans `.mip/briefs/` (fichier persistant)
2. **Presenter dans le chat** : TL;DR (5 lignes) + section Approches + section Risques + lien vers le fichier complet
3. **AskUserQuestion** : "Approuvez-vous ce brief ?" avec options APPROUVE / MODIFIE / REJETE
4. **SI APPROUVE → AskUserQuestion** : "Mode d'autonomie ?" avec options FULL / BIG_STEPS / GUIDED (+ description de chaque mode)

NE JAMAIS :
- Demander l'approbation et le mode d'autonomie dans la meme question
- Dumper le brief entier dans le chat (utiliser le fichier + resume)
- Demander le mode d'autonomie avant l'approbation du brief

### R-P0-4 : Carte de synchronisation des Temps
```
T1 (Maria, HUMAIN) ──────────────────────────── [gate: reponses utilisateur]
  ├─ T2 (Maria+Lise) ──┐
  └─ T3 (Fabrice, T4+) ─┤
                         ├─ [sync: T2+T3 termines]
T4 (Denis+Hugo+Jean) ───┤
T5 (Victor) ─────────────┤
                          ├─ [sync: T4+T5 termines]
T6 (Francois) ────────────┤
                           ├─ [sync: T6 termine]
T7 (Denis) ────────────────┤
                            ├─ [sync: T7 termine]
T8 (Arianne+Jean) ──────────┤
T9 (Hugo) ───────────────────┤
                              ├─ [sync: T8+T9 termines]
T10 (Maria, brief) ──────────┤
                               └─ [GATE P0: brief + autonomie, HUMAIN]
```
Regles :
- T2+T3 peuvent etre paralleles entre eux, mais APRES T1
- T4+T5 peuvent etre paralleles, mais APRES T2+T3
- T6 attend T4+T5 (besoin de l'inventaire et de la checklist securite)
- T7 attend T6 (besoin de la spec)
- T8+T9 attendent T7 (besoin du plan)
- T10 attend T8+T9

### R-P0-5 : Allegements par classe
| Temps | T3 | T4 | T5 |
|-------|----|----|-----|
| T1 Brainstorming | 12 questions (non-OPT) | 20 questions | 21 questions + HMW |
| T2 Ideation | Maria seule (Lise si UI) | Maria + Lise | Maria + Lise |
| T3 Concurrence | **SAUTE** (annonce "T3 saute, T3 non applicable") | Fabrice | Fabrice |
| T4 Inventaire | Denis simplifie (pas d'etapes) | Denis + Hugo + Jean | Denis + Hugo + Jean |
| T5 Securite | Victor (checklist legere) | Victor (complet) | Victor (complet + threat model) |
| T6 Spec | Francois (simplifie, integre au brief) | Francois | Francois |
| T7 Plan | Denis (peut etre integre au brief) | Denis | Denis (plan obligatoirement separe) |
| T8 Audit | Arianne + Jean (leger) | Arianne + Jean | Arianne + Jean |
| T9 CI/CD | **SAUTE** sauf si CI/CD en place | Hugo (si CI/CD) | Hugo |
| T10 Brief | Maria | Maria | Maria |

Quand un Temps est SAUTE, Maria annonce dans le chat : `[HH:MM] P0 Temps X — SAUTE (classe T3, non applicable).`

---

## Temps 1 — Exploration & Brainstorming structure (Maria)

Maria reformule la demande, creuse le contexte, et guide l'utilisateur a travers un questionnaire de brainstorming structure.

**Taches** :
1. **Reformuler** la demande utilisateur en termes precis
2. **Classifier** la demande (T1-T5)
3. **Explorer le code existant** (Glob, Grep, Read) pour comprendre l'etat actuel
4. **Administrer le questionnaire de brainstorming** via AskUserQuestion, **section par section** (R-P0-1)
5. **Identifier les contraintes** : Lois d'Autonomie, stack, compatibilite

**Hard gate** : NE PAS passer au Temps 2 sans reponses de l'utilisateur. Utiliser AskUserQuestion, pas du texte libre.

### Questionnaire de Brainstorming Standard

> Maria pose les questions **section par section** via AskUserQuestion (1 appel = 1 section, max 4 questions).
> Questions `[OPT]` sautees en T3 (R-P0-5). Maria annonce les questions sautees.
> **T3** : 12 questions non-OPT | **T4** : 20 questions | **T5** : 21 questions + HMW
> **Boucle MIP** : Re-poser sections 1 et 4 orientees sur les ecarts constates.

#### Section 1 — COMPRENDRE (Design Thinking + 5 Whys)

| # | Question | Classes |
|---|----------|---------|
| 1.1 | Quel probleme ou besoin cette demande resout-elle ? | T3-T5 |
| 1.2 | Pourquoi maintenant ? Qu'est-ce qui declenche cette demande ? | T3-T5 |
| 1.3 | Qui est l'utilisateur final ? | T3-T5 |
| 1.4 | Quel est le parcours actuel ? Points de friction ? | T3-T5 |
| 1.5 | `[OPT]` Pourquoi cette approche plutot qu'une autre ? | T4-T5 |

#### Section 2 — CADRER (Six Thinking Hats: White/Blue)

| # | Question | Classes |
|---|----------|---------|
| 2.1 | Contraintes techniques connues ? | T3-T5 |
| 2.2 | Perimetre souhaite ? INCLUS et EXCLU. | T3-T5 |
| 2.3 | Priorite ? (a) minimale viable, (b) souhaitees, (c) nice-to-have. | T3-T5 |
| 2.4 | `[OPT]` Deadline ou jalon externe ? | T4-T5 |
| 2.5 | `[OPT]` Donnees ou references existantes ? | T4-T5 |

#### Section 3 — IMAGINER (Green Hat + SCAMPER)

| # | Question | Classes |
|---|----------|---------|
| 3.1 | Idees ou preferences d'approche technique ? | T3-T5 |
| 3.2 | Quelque chose de similaire dans le projet qu'on pourrait adapter ? | T3-T5 |
| 3.3 | `[OPT]` Combiner avec une fonctionnalite existante ? | T4-T5 |
| 3.4 | `[OPT]` Que peut-on eliminer pour simplifier ? | T4-T5 |
| 3.5 | `[OPT]` Produits/services concurrents similaires ? | T4-T5 |
| 3.6 | `[OPT]` "How Might We..." — reformuler en opportunite ? | T5 |

#### Section 4 — EVALUER (Yellow/Black/Red Hats)

| # | Question | Classes |
|---|----------|---------|
| 4.1 | Benefice principal attendu ? LA chose qui doit fonctionner ? | T3-T5 |
| 4.2 | Risques ou difficultes anticipes ? | T3-T5 |
| 4.3 | Intuition sur la complexite ? (simple/modere/complexe/tres complexe) | T3-T5 |
| 4.4 | `[OPT]` Importance strategique ? (1-5) | T4-T5 |
| 4.5 | `[OPT]` Que se passe-t-il si on ne fait PAS ce projet ? | T4-T5 |

#### Section 5 — DECIDER (Lightning Decision Jam)

| # | Question | Classes |
|---|----------|---------|
| 5.1 | Fonctionnalite MINIMALE viable ? | T3-T5 |
| 5.2 | Compromis ? Privilegier : (a) rapidite, (b) exhaustivite, (c) qualite ? | T3-T5 |
| 5.3 | `[OPT]` Qu'est-ce qui peut etre reporte a un prochain sprint ? | T4-T5 |
| 5.4 | `[OPT]` Decisions deja verrouillees ? | T4-T5 |

---

## Temps 2 — Ideation (Maria + Lise en parallele)

**Maria** — Cadrage fonctionnel :
1. Lister les **objectifs** (principal + secondaires)
2. Definir le **perimetre** : IN / OUT explicite
3. Identifier les **risques** et mitigations
4. Proposer **2-3 approches** techniques avec pros/cons

**Lise** (T3+ si aspect front/UI) — Vision graphique :
1. Analyser l'**UI existante** (theme, composants, patterns)
2. Proposer la **direction artistique** : style, ton, inspirations
3. Decrire le **parcours utilisateur** (flux ecran par ecran)
4. Identifier les **composants** a creer/reutiliser (atomic design)
5. Referencer des **inspirations visuelles**

---

## Temps 3 — Analyse concurrentielle (Fabrice, T4-T5 seulement)

> Lance en parallele du Temps 2.

1. Identifier les **produits/services concurrents**
2. Analyser **forces et faiblesses** de chaque concurrent
3. Identifier la **cible utilisateur** et ses attentes
4. Lister les **fonctionnalites differenciantes**
5. Detecter les **points de friction** des concurrents

---

## Temps 4 — Inventaire des prerequis + Evaluation infra + Modeles (Denis + Hugo + Jean + equipe)

Denis coordonne un inventaire complet. Hugo (T4-T5) evalue l'infrastructure. Jean recommande les modeles.

**1. Competences requises** (par agent) :
- Francois : competences Rust/back-end necessaires
- Lise : competences UI/front-end necessaires
- Denis : competences architecture necessaires

**2. Connaissances necessaires** :
- Domaine metier, patterns existants (depuis `memory/mip-decisions.md`), anti-patterns (depuis `memory/patterns-and-lessons.md`), documentation

**3. Outils et ressources** :
- Crates/packages externes (versions, maintenance, compatibilite)
- Crates/modules internes a utiliser/modifier
- Outils dev (compilateur, Context7 IDs, CLI tools)
- Assets, infrastructure, docs & refs

**4. Etapes generales** : Denis decompose en etapes macro (avant le plan atomique du Temps 7) :
```markdown
### Etape N — <nom>
- Objectif: <ce que cette etape accomplit>
- Agents: <qui travaille>
- Prerequis: <ce qui doit etre fait avant>
- Livrables: <ce qui est produit>
- Critere de completion: <comment savoir que c'est fini>
- Risques identifies: <ce qui pourrait bloquer>
```

**5. Matrice de disponibilite** : Statut de chaque prerequis (disponible / a creer / manquant).

**6. Evaluation infrastructure** (Hugo, T4-T5) : Serveurs, reseau (ports, TLS, DNS), persistance (volumes, backup), conteneurisation, CI/CD, scalabilite.

**7. Recommandation modeles** (Jean) : Analyser la classe (T1-T5), recommander le modele par agent (opus/sonnet/haiku), estimer le budget tokens total. Autorite CONSULTATIVE — Denis et Maria valident.

---

## Temps 5 — Analyse de securite (Victor, T3+)

Victor intervient apres l'inventaire (Temps 4) et avant la spec (Temps 6).

**5 volets** :

1. **Threat Model** : Assets a proteger, acteurs (attaquants), surfaces d'attaque, scenarios d'attaque, impact (CIA)

2. **Niveau de securite** (depuis `.mip/environment.md` S2.8-S2.11) :
   - Standard : OWASP basics
   - Renforce : Crypto obligatoire, audit regulier, RGPD
   - Critique : Zero-trust, audit formel, conformite sectorielle

3. **Audit des dependances** : CVE connues, dernier commit (>6 mois = risque), nombre de mainteneurs (<2 = risque), licence compatible

4. **Checklist securite pour la spec** (transmise a Francois) :
   - [ ] Authentification : quel mecanisme ?
   - [ ] Autorisation : quel modele ?
   - [ ] Validation des entrees : quels points ?
   - [ ] Chiffrement : quelles donnees ? quel algo ?
   - [ ] Gestion des secrets : ou stockes ?
   - [ ] Logging securite : quels evenements ?
   - [ ] Rate limiting : quels endpoints ?
   - [ ] CORS : quelle politique ?

5. **Recommandations de durcissement** proportionnees au niveau

---

## Temps 6 — Specification technique + Context7 (Francois)

Francois analyse le contexte technique, verifie les docs, integre la checklist securite de Victor.

1. Explorer le code existant en profondeur
2. **Verification Context7 obligatoire** pour chaque lib impliquee :
   - `resolve-library-id` → `query-docs`
   - Documenter breaking changes / deprecations
   - Comparer avec patterns existants
3. Charger les **anti-patterns connus** (MEMORY.md + patterns-and-lessons.md)
4. Identifier les **fichiers** a modifier/creer avec numeros de ligne
5. Definir les **types, traits, API** (signatures completes validees Context7)
6. Evaluer les **dependances** entre modules
7. **Conformite architecturale** : Lois d'Autonomie, `unsafe_code = "forbid"`, strate COG, MSCM, versions deps
8. **Integrer checklist securite Victor** : auth, validation, chiffrement, secrets, rate limiting
9. Documenter les **risques techniques**

**Output** : `.mip/specs/YYYY-MM-DD-<slug>.md` — commence par TL;DR 5 lignes max. **400 lignes max** (regle 17). Si depassement, decouper : `spec.md` (index) + `spec-module-X.md`.

---

## Temps 7 — Plan exhaustif & Guide d'implementation (Denis)

Denis compile inventaire (T4) + securite (T5) + spec (T6) et produit le plan exhaustif.

1. **Decomposer en taches atomiques** (2-5 minutes chacune)
2. **Couvrir** : Code (Francois+Lise), Tests unitaires, Tests integration, Tests securite (Victor), Tests generaux, Audit (George+Victor), Infra (Hugo), Buffer corrections (20%)
3. **Chaque tache contient** :
   - Numero sequentiel + categorie (`[CODE-01]`, `[TEST-U-01]`, `[TEST-I-01]`, `[TEST-S-01]`, `[AUDIT-01]`, `[INFRA-01]`)
   - Agent assigne
   - Fichier(s) exact(s) (chemin complet)
   - Code complet a ecrire
   - Commande de test + output attendu
   - Message de commit
   - Dependances (`depends: [CODE-01, CODE-02]`)
4. **Principe** : Presumer que l'executant n'a AUCUN contexte projet
5. **Ordonnancement** : Par dependance. Taches independantes marquees parallelisables.
6. **Guide d'implementation** par etape macro :
```markdown
## Guide — Etape X : <nom>
### Prerequis : competences, outils, crates, docs Context7
### Taches : [CODE-01] → [CODE-02] → [TEST-U-01] → ...
### Critere de completion :
- [ ] Tests de l'etape passent
- [ ] Clippy propre
- [ ] Code review (checkpoint Denis si >=5 taches)
```

**Output** : `.mip/plans/YYYY-MM-DD-<slug>.md` — commence par TL;DR 5 lignes max. **400 lignes max** (regle 17). Si depassement, decouper : `plan.md` (index + navigation) + `plan-etape-X.md` par etape macro.

---

## Temps 8 — Audit de faisabilite, Conformite & Validation efficience (Arianne + Jean)

Arianne verifie que le projet est realisable tel que planifie. Jean valide l'efficience du plan.

**Verification agents** : Agents necessaires avec competences, capacite modele LLM, coherence inter-agents (outputs → inputs).

**Verification dependances** : Crates externes (existent, maintenus, compatibles), crates internes (types/traits definis), outils disponibles.

**Verification memoire** : Anti-patterns (patterns-and-lessons.md), patterns confirmes (mip-decisions.md), historique (mip-performance-history.md).

**Verification Context7** : Spot-check 2-3 patterns critiques, breaking changes recents.

**Diagnostic** :
| Resultat | Action |
|----------|--------|
| Conforme | Feu vert → Maria compile le brief |
| Trous mineurs | Lister manques, corriger le plan |
| Ambiguite | Poser questions a l'utilisateur/agent |
| Manque critique | Suggerer **projet precurseur** (T2-T3) |
| Irrealisable | Suggerer reorientation |

**Validation efficience** (Jean) : Lister les fichiers charges par chaque agent, identifier les redondances, recommander le chargement selectif (modules, index+drill-down), valider que les modules SKILL.md necessaires sont identifies.

---

## Temps 9 — Verification pipeline CI/CD (Hugo, si CI/CD en place)

Hugo verifie la compatibilite de la pipeline existante avec le nouveau code.

1. Relire config CI/CD (`.github/workflows/`, `.gitlab-ci.yml`)
2. Compatibilite des etapes existantes avec nouveaux crates/fichiers
3. Adaptations necessaires (jobs, variables, secrets CI)
4. Estimation temps de build additionnel
5. Si pas de CI/CD : proposer config initiale

---

## Temps 10 — Synthese & Brief (Maria)

Maria compile tout et presente le brief selon la sequence R-P0-3 :

1. Fusionner contributions de tous les agents
2. Integrer audit Arianne
3. Rediger le brief structure (template ci-dessous)
4. **Ecrire le brief** dans `.mip/briefs/YYYY-MM-DD-<slug>.md` — **400 lignes max** (regle 17). Si depassement, decouper : `brief.md` (index + TL;DR + decisions) + `brief-annexe-X.md` (plan Denis, analyse Fabrice, etc.)
5. **Presenter dans le chat** (R-P0-3) :
   - TL;DR (5 lignes, copie depuis le brief)
   - Section Approches proposees (avec recommandation)
   - Section Risques (table)
   - Lien vers le fichier complet : "Brief complet : `.mip/briefs/YYYY-MM-DD-<slug>.md`"
6. **AskUserQuestion** — approbation : APPROUVE / MODIFIE / REJETE
7. **SI APPROUVE → AskUserQuestion** — mode d'autonomie : FULL / BIG_STEPS / GUIDED (invariant I-4, apres lecture du brief)

### Template du brief

```markdown
# Brief: <titre>

## TL;DR (5 lignes max)
<Resume ultra-concis : projet, approche, effort, risque, etape critique>

## Metadata
- Classe: T3/T4/T5
- Date: YYYY-MM-DD

## Contexte
[Pourquoi cette demande, quel probleme]

## Objectifs
- Principal: ...
- Secondaires: ...
- Criteres de succes mesurables: ...

## Perimetre
### Inclus
- [Fonctionnalites IN]
### Exclus
- [Fonctionnalites OUT]

## Approches proposees
### Approche A — [nom] (RECOMMANDEE)
- Description, pros, cons, effort

### Approche B — [nom]
- Description, pros, cons

## Direction visuelle (Lise)
- Style/ton, composants, parcours utilisateur, inspirations

## Analyse concurrentielle (Fabrice, T4-T5)
- Concurrents, differenciateurs, cible

## Analyse de securite (Victor, T3+)
### Threat Model
| Surface | Scenario | Impact | Mitigation |
### Niveau + Conformite + Dependances auditees
### Checklist securite + Recommandations

## Pipeline CI/CD (Hugo, si applicable)
- Compatibilite, adaptations, impact build

## Inventaire des prerequis (Denis + Hugo + equipe)
### Competences, connaissances, outils, etapes generales

## Specification technique (Francois)
- Fichiers, types/API, conformite, risques

## Plan de developpement (Denis)
[Voir annexe .mip/plans/]
- Nombre total taches (code, tests, audit, infra, buffer)

## Audit de faisabilite (Arianne)
- Conformite agents, dependances, memoire
- Verdict: CONFORME / TROUS MINEURS / PREREQUIS

## Risques
| Risque | Probabilite | Impact | Mitigation |

## Decision
APPROUVE / REJETE / MODIFIE / PREREQUIS D'ABORD

## Mode d'autonomie (choisi APRES lecture du brief — invariant I-4)
> L'utilisateur choisit en connaissance de cause apres avoir lu tout le brief ci-dessus.
- [ ] FULL | [ ] BIG_STEPS | [ ] GUIDED
- Garder pour les futures sequences ? OUI / NON / JE SAIS PAS
```

**Quality Gate P0** (sequence stricte — invariant I-4, R-P0-3) :

La Gate P0 se fait en **2 AskUserQuestion separes**, jamais en 1 :

**AskUserQuestion 1 — Approbation** (apres presentation TL;DR + approches + risques dans le chat) :
- Question : "Approuvez-vous ce brief ?"
- Options : APPROUVE / MODIFIE (preciser les changements) / REJETE (preciser la raison)

**AskUserQuestion 2 — Mode d'autonomie** (uniquement si APPROUVE) :
- Question : "Quel mode d'autonomie pour l'execution ?"
- Options :
  - FULL — Autopilot complet, prochaine intervention = test P5
  - BIG_STEPS — Gates entre chaque phase (P3→P4, P4→P5)
  - GUIDED — Validation a chaque etape

Hard gates :
- AUCUN passage en execution sans brief approuve
- AUCUN choix d'autonomie sans lecture prealable du brief
- AUCUN melange approbation + autonomie dans la meme question
