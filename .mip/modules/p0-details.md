# Module MIP — P0 Cadrage (10 temps)

> Ce module est chargé au début de P0 pour les tâches T3+.

---

## Règles de présentation P0 (BORNANTES)

Ces règles s'appliquent à TOUS les temps de P0 pour garantir la stabilité de présentation :

### R-P0-1 : Outil de questionnement
- **AskUserQuestion** est l'outil OBLIGATOIRE pour toute question à l'utilisateur en P0
- Autant de questions que nécessaire par section (pas de limite fixe)
- Chaque section du questionnaire de brainstorming = **1 appel AskUserQuestion** (grouper les questions de la section)
- Texte libre dans le chat = uniquement pour les annonces, résumés et présentations (jamais pour poser des questions)

### R-P0-2 : Visibilité des résultats intermédiaires
Après chaque temps, l'orchestrateur **annonce dans le chat** un résumé de 3 à 5 lignes :
```
[YYYY-MM-DD HH:MM] P0 Temps X — <Nom> terminé.
  Agent(s) : <liste>
  Résultat : <résumé 2-3 lignes>
  Prochain temps : Temps X+1 — <Nom>
```
Les résultats détaillés des agents parallèles (Temps 2-9) sont visibles dans les artefacts `.mip/`, pas dans le chat.

### R-P0-3 : Présentation du brief (Gate P0)
Déroulement strict en 4 tâches — invariant I-4 :
1. **Écrire** le brief dans `<sequence>/briefs/` (fichier persistant)
2. **Présenter dans le chat** : TL;DR (5 lignes) + section Approches + section Risques + lien vers le fichier complet
3. **AskUserQuestion** : « Approuvez-vous ce brief ? » avec options APPROUVÉ / MODIFIÉ / REJETÉ
4. **SI APPROUVÉ -> AskUserQuestion** : « Mode d'autonomie ? » avec options FULL / BIG_STEPS / GUIDED (+ description de chaque mode)

NE JAMAIS :
- Demander approbation et mode d'autonomie dans la même question
- Déverser le brief complet dans le chat (utiliser fichier + résumé)
- Demander le mode d'autonomie avant l'approbation du brief

### R-P0-4 : Carte de synchronisation des temps
```
T1 (Maria, HUMAIN) -------------------------------- [gate : réponses utilisateur]
  +- T2 (Maria+Lise) --+
  +- T3 (Fabrice, T4+) -+
                         +- [sync : T2+T3 terminés]
T4 (Denis+Hugo+Jean) ---+
T5 (Victor) -------------+
                          +- [sync : T4+T5 terminés]
T6 (François) ------------+
                           +- [sync : T6 terminé]
T7 (Denis) ----------------+
                            +- [sync : T7 terminé]
T8 (Arianne+Jean) ----------+
T9 (Hugo) -------------------+
                              +- [sync : T8+T9 terminés]
T10 (Maria, brief) ----------+
                               +- [GATE P0 : brief + autonomie, HUMAIN]
```
Règles :
- T2+T3 peuvent être parallèles entre eux, mais APRÈS T1
- T4+T5 peuvent être parallèles, mais APRÈS T2+T3
- T6 attend T4+T5 (nécessite inventaire et checklist sécurité)
- T7 attend T6 (nécessite la spec)
- T8+T9 attendent T7 (nécessitent le plan)
- T10 attend T8+T9

### R-P0-5 : Allègement par classe
| Temps | T3 | T4 | T5 |
|-------|----|----|-----|
| T1 Brainstorming | 12 questions (non-OPT) | 20 questions | 21 questions + HMW |
| T2 Idéation | Maria seule (Lise si UI) | Maria + Lise | Maria + Lise |
| T3 Concurrentiel | **SAUTÉ** (annoncer « T3 sauté, non applicable ») | Fabrice | Fabrice |
| T4 Inventaire | Denis simplifié (sans étapes) | Denis + Hugo + Jean | Denis + Hugo + Jean |
| T5 Sécurité | Victor (checklist légère) | Victor (complet) | Victor (complet + modèle de menace) |
| T6 Spec | François (simplifiée, intégrée au brief) | François | François |
| T7 Plan | Denis (peut être intégré au brief) | Denis | Denis (plan séparé obligatoire) |
| T8 Audit | Arianne + Jean (léger) | Arianne + Jean | Arianne + Jean |
| T9 CI/CD | **SAUTÉ** sauf si CI/CD en place | Hugo (si CI/CD) | Hugo |
| T10 Brief | Maria | Maria | Maria |

Quand un temps est SAUTÉ, Maria annonce dans le chat : `[HH:MM] P0 Temps X — SAUTÉ (classe T3, non applicable).`

---

## Temps 1 — Exploration et brainstorming structuré (Maria)

Maria reformule la demande, explore le contexte et guide l'utilisateur à travers un questionnaire de brainstorming structuré.

**Tâches** :
1. **Reformuler** la demande utilisateur en termes précis
2. **Classifier** la demande (T1-T5)
3. **Explorer le code existant** (Glob, Grep, Read) pour comprendre l'état actuel
4. **Administrer le questionnaire de brainstorming** via AskUserQuestion, **section par section** (R-P0-1)
5. **Identifier les contraintes** : Lois d'Autonomie, stack, compatibilité

**Gate stricte** : NE PAS passer au temps 2 sans réponses utilisateur. Utiliser AskUserQuestion, pas de texte libre.

### Questionnaire de brainstorming standard

> Maria pose les questions **section par section** via AskUserQuestion (1 appel = 1 section).
> Questions `[OPT]` sautées en T3 (R-P0-5). Maria annonce les questions sautées.
> **T3** : 12 questions non-OPT | **T4** : 20 questions | **T5** : 21 questions + HMW
> **Boucle MIP** : Reposer les sections 1 et 4 orientées sur les écarts constatés.

#### Section 1 — COMPRENDRE (Design Thinking + 5 Whys)

| # | Question | Classes |
|---|----------|---------|
| 1.1 | Quel problème ou besoin cette demande résout-elle ? | T3-T5 |
| 1.2 | Pourquoi maintenant ? Qu'est-ce qui déclenche cette demande ? | T3-T5 |
| 1.3 | Qui est l'utilisateur final ? | T3-T5 |
| 1.4 | Quel est le flux de travail actuel ? Points de friction ? | T3-T5 |
| 1.5 | `[OPT]` Pourquoi cette approche plutôt qu'une autre ? | T4-T5 |

#### Section 2 — CADRER (Six Thinking Hats : Blanc/Bleu)

| # | Question | Classes |
|---|----------|---------|
| 2.1 | Contraintes techniques connues ? | T3-T5 |
| 2.2 | Périmètre souhaité ? INCLUS et EXCLUS. | T3-T5 |
| 2.3 | Priorité ? (a) minimal viable, (b) souhaité, (c) nice-to-have. | T3-T5 |
| 2.4 | `[OPT]` Échéance ou jalon externe ? | T4-T5 |
| 2.5 | `[OPT]` Données ou références existantes ? | T4-T5 |

#### Section 3 — IMAGINER (Chapeau vert + SCAMPER)

| # | Question | Classes |
|---|----------|---------|
| 3.1 | Idées ou préférences d'approche technique ? | T3-T5 |
| 3.2 | Quelque chose de similaire dans le projet qu'on pourrait adapter ? | T3-T5 |
| 3.3 | `[OPT]` Combiner avec une fonctionnalité existante ? | T4-T5 |
| 3.4 | `[OPT]` Que peut-on éliminer pour simplifier ? | T4-T5 |
| 3.5 | `[OPT]` Produits/services concurrents similaires ? | T4-T5 |
| 3.6 | `[OPT]` « Comment pourrions-nous... » — reformuler en opportunité ? | T5 |

#### Section 4 — ÉVALUER (Chapeaux Jaune/Noir/Rouge)

| # | Question | Classes |
|---|----------|---------|
| 4.1 | Bénéfice principal attendu ? LA chose qui DOIT fonctionner ? | T3-T5 |
| 4.2 | Risques ou difficultés anticipés ? | T3-T5 |
| 4.3 | Intuition sur la complexité ? (simple/modérée/complexe/très complexe) | T3-T5 |
| 4.4 | `[OPT]` Importance stratégique ? (1-5) | T4-T5 |
| 4.5 | `[OPT]` Que se passe-t-il si on NE fait PAS ce projet ? | T4-T5 |

#### Section 5 — DÉCIDER (Lightning Decision Jam)

| # | Question | Classes |
|---|----------|---------|
| 5.1 | Fonctionnalité MINIMALE viable ? | T3-T5 |
| 5.2 | Arbitrages ? Prioriser : (a) rapidité, (b) complétude, (c) qualité ? | T3-T5 |
| 5.3 | `[OPT]` Qu'est-ce qui peut être reporté au prochain sprint ? | T4-T5 |
| 5.4 | `[OPT]` Décisions déjà figées ? | T4-T5 |

---

## Temps 2 — Idéation (Maria + Lise en parallèle)

**Maria** — Cadrage fonctionnel :
1. Lister les **objectifs** (principal + secondaires)
2. Définir le **périmètre** : IN / OUT explicites
3. Identifier les **risques** et mitigations
4. Proposer **2-3 approches techniques** avec pour/contre

**Lise** (T3+ si aspect front/UI) — Direction visuelle :
1. Analyser l'**UI existante** (thème, composants, patterns)
2. Proposer la **direction artistique** : style, ton, inspirations
3. Décrire le **parcours utilisateur** (flux écran par écran)
4. Identifier les **composants** à créer/réutiliser (atomic design)
5. Référencer les **inspirations visuelles**

---

## Temps 3 — Analyse concurrentielle (Fabrice, T4-T5 uniquement)

> Lancé en parallèle avec le temps 2.

1. Identifier les **produits/services concurrents**
2. Analyser les **forces et faiblesses** de chaque concurrent
3. Identifier l'**utilisateur cible** et ses attentes
4. Lister les **fonctionnalités différenciatrices**
5. Détecter les **points de friction** des concurrents

---

## Temps 4 — Inventaire des prérequis + évaluation infra + modèles (Denis + Hugo + Jean + équipe)

Denis coordonne un inventaire complet. Hugo (T4-T5) évalue l'infrastructure. Jean recommande les modèles.

**1. Compétences requises** (par agent) :
- François : compétences back-end requises
- Lise : compétences UI/front-end requises
- Denis : compétences architecture requises

**2. Connaissances nécessaires** :
- Domaine métier, patterns existants (depuis `.mip/memory/mip-decisions.md`), anti-patterns (depuis `.mip/memory/patterns-and-lessons.md`), documentation

**3. Outils et ressources** :
- Paquets externes (versions, maintenance, compatibilité)
- Paquets/modules internes à utiliser/modifier
- Outils de dev (compilateur, IDs de vérification docs, outils CLI)
- Assets, infrastructure, docs et références

**4. Étapes générales** : Denis décompose en étapes macro (avant le plan atomique en temps 7) :
```markdown
### Étape N — <nom>
- Objectif : <ce que cette étape accomplit>
- Agents : <qui travaille>
- Prérequis : <ce qui doit être fait avant>
- Livrables : <ce qui est produit>
- Critères de complétion : <comment savoir que c'est fait>
- Risques identifiés : <ce qui pourrait bloquer>
```

**5. Matrice de disponibilité** : Statut de chaque prérequis (disponible / à créer / manquant).

**6. Évaluation infrastructure** (Hugo, T4-T5) : Serveurs, réseau (ports, TLS, DNS), PaaS/Stockage (volumes, sauvegarde), conteneurisation, CI/CD, scalabilité.

**7. Recommandation modèles** (Jean) : Analyser la classe (T1-T5), recommander le modèle par agent (opus/sonnet/haiku), estimer le budget tokens total. Autorité CONSULTATIVE — Denis et Maria valident.

---

## Temps 5 — Analyse de sécurité (Victor, T3+)

Victor intervient après l'inventaire (temps 4) et avant la spec (temps 6).

**5 domaines** :

1. **Modèle de menace** : Actifs à protéger, acteurs (attaquants), surfaces d'attaque, scénarios d'attaque, impact (CIA)

2. **Niveau de sécurité** (depuis `.mip/environment.md` S2.8-S2.11) :
   - Standard : bases OWASP
   - Durci : Crypto obligatoire, audit régulier, RGPD
   - Critique : Zero-trust, audit formel, conformité sectorielle

3. **Audit des dépendances** : CVE connus, dernier commit (>6 mois = risque), nombre de mainteneurs (<2 = risque), licence compatible

4. **Checklist sécurité pour la spec** (transmise à François) :
   - [ ] Authentification : quel mécanisme ?
   - [ ] Autorisation : quel modèle ?
   - [ ] Validation des entrées : quels points ?
   - [ ] Chiffrement : quelles données ? quel algorithme ?
   - [ ] Gestion des secrets : où stockés ?
   - [ ] Logging sécurité : quels événements ?
   - [ ] Limitation de débit : quels endpoints ?
   - [ ] CORS : quelle politique ?

5. **Recommandations de durcissement** proportionnelles au niveau

---

## Temps 6 — Spécification technique + vérification docs (François)

François analyse le contexte technique, vérifie les docs, intègre la checklist sécurité de Victor.

1. Explorer le code existant en profondeur
2. **Vérification docs obligatoire** pour chaque lib impliquée (Context7, recherche web ou fallback si indisponible) :
   - Documenter les breaking changes / dépréciations
   - Comparer avec les patterns existants
3. Charger les **anti-patterns connus** (MEMORY.md + patterns-and-lessons.md)
4. Identifier les **fichiers** à modifier/créer avec numéros de ligne
5. Définir les **types, traits, API** (signatures complètes validées contre les docs)
6. Évaluer les **dépendances** entre modules
7. **Conformité architecturale** : Lois d'Autonomie, règles de sûreté du code, couche architecture, annotations, versions dépendances
8. **Intégrer la checklist sécurité de Victor** : auth, validation, chiffrement, secrets, limitation débit
9. Documenter les **risques techniques**

**Production** : `<sequence>/specs/YYYY-MM-DD-<slug>.md` — commence par TL;DR 5 lignes max. **400 lignes max** (règle I-14). Si dépassé, découper : `spec.md` (index) + `spec-module-X.md`.

---

## Temps 7 — Plan exhaustif et guide d'implémentation (Denis)

Denis compile l'inventaire (T4) + sécurité (T5) + spec (T6) et produit le plan exhaustif.

1. **Décomposer en tâches atomiques** (2-5 minutes chacune)
2. **Couvrir** : Code (François+Lise), tests unitaires, tests d'intégration, tests sécurité (Victor), tests globaux, audit (George+Victor), infra (Hugo), buffer corrections (20 %)
3. **Chaque tâche contient** :
   - Numéro séquentiel + catégorie (`[CODE-01]`, `[TEST-U-01]`, `[TEST-I-01]`, `[TEST-S-01]`, `[AUDIT-01]`, `[INFRA-01]`)
   - Agent assigné
   - Fichier(s) exact(s) (chemin complet)
   - Code complet à écrire
   - Commande test + sortie attendue
   - Message de commit
   - Dépendances (`depends: [CODE-01, CODE-02]`)
4. **Principe** : Supposer que l'exécuteur a ZÉRO contexte projet
5. **Ordonnancement** : Par dépendance. Tâches indépendantes marquées parallélisables.
6. **Guide d'implémentation** par étape macro :
```markdown
## Guide — Étape X : <nom>
### Prérequis : compétences, outils, paquets, docs
### Tâches : [CODE-01] -> [CODE-02] -> [TEST-U-01] -> ...
### Critères de complétion :
- [ ] Tests de l'étape passent
- [ ] Lint propre
- [ ] Revue de code (checkpoint Denis si >=5 tâches)
```

**Production** : `<sequence>/plans_p3/YYYY-MM-DD-<slug>.md` — commence par TL;DR 5 lignes max. **400 lignes max** (règle I-14). Si dépassé, découper : `plan.md` (index + navigation) + `plan-etape-X.md` par étape macro.

---

## Temps 8 — Audit de faisabilité, conformité et validation efficience (Arianne + Jean)

Arianne vérifie que le projet est faisable tel que planifié. Jean valide l'efficience du plan.

**Vérification agents** : Agents requis avec compétences, capacité du modèle LLM, cohérence inter-agents (sorties -> entrées).

**Vérification dépendances** : Paquets externes (existent, maintenus, compatibles), paquets internes (types/traits définis), outils disponibles.

**Vérification mémoire** : Anti-patterns (patterns-and-lessons.md), patterns confirmés (mip-decisions.md), historique (mip-performance-history.md).

**Vérification docs spot-check** : Spot-check 2-3 patterns critiques, breaking changes récents.

**Diagnostic** :
| Résultat | Action |
|----------|--------|
| Conforme | Feu vert -> Maria compile le brief |
| Manques mineurs | Lister les manques, corriger le plan |
| Ambiguïté | Poser des questions à l'utilisateur/agent |
| Manque critique | Suggérer un **projet précurseur** (T2-T3) |
| Infaisable | Suggérer une réorientation |

**Validation efficience** (Jean) : Lister les fichiers chargés par chaque agent, identifier les redondances, recommander le chargement sélectif (modules, index+drill-down), valider que les modules SKILL.md requis sont identifiés.

---

## Temps 9 — Vérification pipeline CI/CD (Hugo, si CI/CD en place)

Hugo vérifie la compatibilité du pipeline existant avec le nouveau code.

1. Relire la config CI/CD (`.github/workflows/`, `.gitlab-ci.yml`)
2. Compatibilité des étapes existantes avec les nouveaux paquets/fichiers
3. Adaptations requises (jobs, variables, secrets CI)
4. Temps de build supplémentaire estimé
5. Si pas de CI/CD : proposer une configuration initiale

---

## Temps 10 — Synthèse et brief (Maria)

Maria compile tout et présente le brief en suivant la séquence R-P0-3 :

1. Fusionner les contributions de tous les agents
2. Intégrer l'audit d'Arianne
3. Rédiger le brief structuré (modèle ci-dessous)
4. **Écrire le brief** dans `<sequence>/briefs/YYYY-MM-DD-<slug>.md` — **400 lignes max** (règle I-14). Si dépassé, découper : `brief.md` (index + TL;DR + décisions) + `brief-annexe-X.md` (plan Denis, analyse Fabrice, etc.)
5. **Présenter dans le chat** (R-P0-3) :
   - TL;DR (5 lignes, copiées du brief)
   - Section Approches proposées (avec recommandation)
   - Section Risques (tableau)
   - Lien vers le fichier complet : « Brief complet : `<sequence>/briefs/YYYY-MM-DD-<slug>.md` »
6. **AskUserQuestion** — approbation : APPROUVÉ / MODIFIÉ / REJETÉ
7. **SI APPROUVÉ -> AskUserQuestion** — mode autonomie : FULL / BIG_STEPS / GUIDED (invariant I-4, après lecture du brief)

### Modèle de brief

```markdown
# Brief : <titre>

## TL;DR (5 lignes max)
<Résumé ultra-concis : projet, approche, effort, risque, étape critique>

## Métadonnées
- Classe : T3/T4/T5
- Date : YYYY-MM-DD

## Contexte
[Pourquoi cette demande, quel problème]

## Objectifs
- Principal : ...
- Secondaires : ...
- Critères de succès mesurables : ...

## Périmètre
### Inclus
- [Fonctionnalités IN]
### Exclus
- [Fonctionnalités OUT]

## Approches proposées
### Approche A — [nom] (RECOMMANDÉE)
- Description, pour, contre, effort

### Approche B — [nom]
- Description, pour, contre

## Direction visuelle (Lise)
- Style/ton, composants, parcours utilisateur, inspirations

## Analyse concurrentielle (Fabrice, T4-T5)
- Concurrents, différenciateurs, cible

## Analyse de sécurité (Victor, T3+)
### Modèle de menace
| Surface | Scénario | Impact | Mitigation |
### Niveau + Conformité + Dépendances auditées
### Checklist sécurité + Recommandations

## Pipeline CI/CD (Hugo, si applicable)
- Compatibilité, adaptations, impact build

## Inventaire des prérequis (Denis + Hugo + équipe)
### Compétences, connaissances, outils, étapes générales

## Spécification technique (François)
- Fichiers, types/API, conformité, risques

## Plan de développement (Denis)
[Voir annexe plans_p3/]
- Nombre total de tâches (code, tests, audit, infra, buffer)

## Audit de faisabilité (Arianne)
- Conformité agents, dépendances, mémoire
- Verdict : CONFORME / MANQUES MINEURS / PRÉREQUIS

## Risques
| Risque | Probabilité | Impact | Mitigation |

## Décision
APPROUVÉ / REJETÉ / MODIFIÉ / PRÉREQUIS D'ABORD

## Mode d'autonomie (choisi APRÈS lecture du brief — invariant I-4)
> L'utilisateur choisit en pleine connaissance après avoir lu l'intégralité du brief ci-dessus.
- [ ] FULL | [ ] BIG_STEPS | [ ] GUIDED
- Conserver pour les futures séquences ? OUI / NON / PAS SÛR
```

**Gate qualité P0** (séquence stricte — invariant I-4, R-P0-3) :

La Gate P0 se fait en **2 appels AskUserQuestion distincts**, jamais en 1 :

**AskUserQuestion 1 — Approbation** (après présentation TL;DR + approches + risques dans le chat) :
- Question : « Approuvez-vous ce brief ? »
- Options : APPROUVÉ / MODIFIÉ (préciser les changements) / REJETÉ (préciser la raison)

**AskUserQuestion 2 — Mode autonomie** (seulement si APPROUVÉ) :
- Question : « Quel mode d'autonomie pour l'exécution ? »
- Options :
  - FULL — Autopilot complet, prochaine interaction = test P5
  - BIG_STEPS — Gates entre chaque phase (P3->P4, P4->P5)
  - GUIDED — Validation à chaque étape

Gates strictes :
- PAS de passage en exécution sans brief approuvé
- PAS de choix d'autonomie sans lecture préalable du brief
- PAS de mélange approbation + autonomie dans la même question
