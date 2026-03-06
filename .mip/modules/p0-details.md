# Module MIP â€” P0 Cadrage (11 temps)

> Ce module est chargÃ© au dÃ©but de P0 pour les tÃ¢ches T3+.

---

## RÃ¨gles de prÃ©sentation P0 (BORNANTES)

Ces rÃ¨gles s'appliquent Ã  TOUS les temps de P0 pour garantir la stabilitÃ© de prÃ©sentation :

### R-P0-1 : Outil de questionnement
- **AskUserQuestion** est l'outil OBLIGATOIRE pour toute question Ã  l'utilisateur en P0
- Autant de questions que nÃ©cessaire par section (pas de limite fixe)
- Chaque section du questionnaire de brainstorming = **1 appel AskUserQuestion** (grouper les questions de la section)
- Texte libre dans le chat = uniquement pour les annonces, rÃ©sumÃ©s et prÃ©sentations (jamais pour poser des questions)

### R-P0-2 : VisibilitÃ© des rÃ©sultats intermÃ©diaires
AprÃ¨s chaque temps, l'orchestrateur **annonce dans le chat** un rÃ©sumÃ© de 3 Ã  5 lignes :
```
[YYYY-MM-DD HH:MM] P0 Temps X â€” <Nom> terminÃ©.
  Agent(s) : <liste>
  RÃ©sultat : <rÃ©sumÃ© 2-3 lignes>
  Prochain temps : Temps X+1 â€” <Nom>
```
Les rÃ©sultats dÃ©taillÃ©s des agents parallÃ¨les (Temps 2-10) sont visibles dans les artefacts `.mip/`, pas dans le chat.

### R-P0-3 : PrÃ©sentation du brief (Gate P0)
DÃ©roulement strict en 4 tÃ¢ches â€” invariant I-4 :
1. **Ã‰crire** le brief dans `<sequence>/briefs/` (fichier persistant)
2. **PrÃ©senter dans le chat** : TL;DR (5 lignes) + section Approches + section Risques + lien vers le fichier complet
3. **AskUserQuestion** : Â« Approuvez-vous ce brief ? Â» avec options APPROUVÃ‰ / MODIFIÃ‰ / REJETÃ‰
4. **SI APPROUVÃ‰ -> AskUserQuestion** : Â« Mode d'autonomie ? Â» avec options FULL / BIG_STEPS / GUIDED (+ description de chaque mode)

NE JAMAIS :
- Demander approbation et mode d'autonomie dans la mÃªme question
- DÃ©verser le brief complet dans le chat (utiliser fichier + rÃ©sumÃ©)
- Demander le mode d'autonomie avant l'approbation du brief

### R-P0-4 : Carte de synchronisation des temps
```
T1 (Maria, HUMAIN) -------------------------------- [gate : reponses utilisateur]
  +- T2 (Maria+Lise) --+
  +- T3 (Fabrice, T4+) -+
                         +- [sync : T2+T3 termines]
T4 (Denis+Hugo+Jean) ---+
T5 (Victor) -------------+
                          +- [sync : T4+T5 termines]
T6 (Francois) ------------+
                           +- [sync : T6 termine]
T7 (Maria, agents fine-tuned) -+
                                +- [sync : T7 termine]
T8 (Denis) ---------------------+
                                 +- [sync : T8 termine]
T9 (Arianne+Jean) --------------+
T10 (Hugo) ----------------------+
                                  +- [sync : T9+T10 termines]
T11 (Maria, brief) --------------+
                                   +- [GATE P0 : brief + autonomie, HUMAIN]
```
Regles :
- T2+T3 peuvent etre paralleles entre eux, mais APRES T1
- T4+T5 peuvent etre paralleles, mais APRES T2+T3
- T6 attend T4+T5 (necessite inventaire et checklist securite)
- T7 attend T6 (generation prompts fine-tuned)
- T8 attend T7 (plan base sur spec + prompts)
- T9+T10 attendent T8 (faisabilite et CI/CD sur plan fige)
- T11 attend T9+T10

### R-P0-5 : Allegement par classe
| Temps | T3 | T4 | T5 |
|-------|----|----|-----|
| T1 Brainstorming | 12 questions (non-OPT) | 20 questions | 21 questions + HMW |
| T2 Ideation | Maria seule (Lise si UI) | Maria + Lise | Maria + Lise |
| T3 Concurrentiel | **SAUTE** (annoncer "T3 saute, non applicable") | Fabrice | Fabrice |
| T4 Inventaire | Denis simplifie (sans etapes) | Denis + Hugo + Jean | Denis + Hugo + Jean |
| T5 Securite | Victor (checklist legere) | Victor (complet) | Victor (complet + modele de menace) |
| T6 Spec | Francois (simplifiee, integree au brief) | Francois | Francois |
| T7 Agents fine-tuned | Maria (selection minimale) | Maria (selection par besoins) | Maria (selection + MASS) |
| T8 Plan | Denis (peut etre integre au brief) | Denis | Denis (plan separe obligatoire) |
| T9 Audit faisabilite | Arianne + Jean (leger) | Arianne + Jean | Arianne + Jean |
| T10 CI/CD | **SAUTE** sauf si CI/CD en place | Hugo (si CI/CD) | Hugo |
| T11 Brief | Maria | Maria | Maria |

Quand un temps est SAUTE, Maria annonce dans le chat : `[HH:MM] P0 Temps X - SAUTE (classe T3, non applicable).`

---

## Temps 1 â€” Exploration et brainstorming structurÃ© (Maria)

Maria reformule la demande, explore le contexte et guide l'utilisateur Ã  travers un questionnaire de brainstorming structurÃ©.

**TÃ¢ches** :
1. **Reformuler** la demande utilisateur en termes prÃ©cis
2. **Classifier** la demande (T1-T5)
3. **Explorer le code existant** (Glob, Grep, Read) pour comprendre l'Ã©tat actuel
4. **Administrer le questionnaire de brainstorming** via AskUserQuestion, **section par section** (R-P0-1)
5. **Identifier les contraintes** : Lois d'Autonomie, stack, compatibilitÃ©

**Gate stricte** : NE PAS passer au temps 2 sans rÃ©ponses utilisateur. Utiliser AskUserQuestion, pas de texte libre.

### Questionnaire de brainstorming standard

> Maria pose les questions **section par section** via AskUserQuestion (1 appel = 1 section).
> Questions `[OPT]` sautÃ©es en T3 (R-P0-5). Maria annonce les questions sautÃ©es.
> **T3** : 12 questions non-OPT | **T4** : 20 questions | **T5** : 21 questions + HMW
> **Boucle MIP** : Reposer les sections 1 et 4 orientÃ©es sur les Ã©carts constatÃ©s.

#### Section 1 â€” COMPRENDRE (Design Thinking + 5 Whys)

| # | Question | Classes |
|---|----------|---------|
| 1.1 | Quel problÃ¨me ou besoin cette demande rÃ©sout-elle ? | T3-T5 |
| 1.2 | Pourquoi maintenant ? Qu'est-ce qui dÃ©clenche cette demande ? | T3-T5 |
| 1.3 | Qui est l'utilisateur final ? | T3-T5 |
| 1.4 | Quel est le flux de travail actuel ? Points de friction ? | T3-T5 |
| 1.5 | `[OPT]` Pourquoi cette approche plutÃ´t qu'une autre ? | T4-T5 |

#### Section 2 â€” CADRER (Six Thinking Hats : Blanc/Bleu)

| # | Question | Classes |
|---|----------|---------|
| 2.1 | Contraintes techniques connues ? | T3-T5 |
| 2.2 | PÃ©rimÃ¨tre souhaitÃ© ? INCLUS et EXCLUS. | T3-T5 |
| 2.3 | PrioritÃ© ? (a) minimal viable, (b) souhaitÃ©, (c) nice-to-have. | T3-T5 |
| 2.4 | `[OPT]` Ã‰chÃ©ance ou jalon externe ? | T4-T5 |
| 2.5 | `[OPT]` DonnÃ©es ou rÃ©fÃ©rences existantes ? | T4-T5 |

#### Section 3 â€” IMAGINER (Chapeau vert + SCAMPER)

| # | Question | Classes |
|---|----------|---------|
| 3.1 | IdÃ©es ou prÃ©fÃ©rences d'approche technique ? | T3-T5 |
| 3.2 | Quelque chose de similaire dans le projet qu'on pourrait adapter ? | T3-T5 |
| 3.3 | `[OPT]` Combiner avec une fonctionnalitÃ© existante ? | T4-T5 |
| 3.4 | `[OPT]` Que peut-on Ã©liminer pour simplifier ? | T4-T5 |
| 3.5 | `[OPT]` Produits/services concurrents similaires ? | T4-T5 |
| 3.6 | `[OPT]` Â« Comment pourrions-nous... Â» â€” reformuler en opportunitÃ© ? | T5 |

#### Section 4 â€” Ã‰VALUER (Chapeaux Jaune/Noir/Rouge)

| # | Question | Classes |
|---|----------|---------|
| 4.1 | BÃ©nÃ©fice principal attendu ? LA chose qui DOIT fonctionner ? | T3-T5 |
| 4.2 | Risques ou difficultÃ©s anticipÃ©s ? | T3-T5 |
| 4.3 | Complexité de séquence ? (C1 mineur / C2 faible / C3 moyenne / C4 élevée / C5 stratégique) | T3-T5 |
| 4.4 | `[OPT]` Importance stratÃ©gique ? (1-5) | T4-T5 |
| 4.5 | `[OPT]` Que se passe-t-il si on NE fait PAS ce projet ? | T4-T5 |

#### Section 5 â€” DÃ‰CIDER (Lightning Decision Jam)

| # | Question | Classes |
|---|----------|---------|
| 5.1 | FonctionnalitÃ© MINIMALE viable ? | T3-T5 |
| 5.2 | Arbitrages ? Prioriser : (a) rapiditÃ©, (b) complÃ©tude, (c) qualitÃ© ? | T3-T5 |
| 5.3 | `[OPT]` Qu'est-ce qui peut Ãªtre reportÃ© au prochain sprint ? | T4-T5 |
| 5.4 | `[OPT]` DÃ©cisions dÃ©jÃ  figÃ©es ? | T4-T5 |

---

## Temps 2 â€” IdÃ©ation (Maria + Lise en parallÃ¨le)

**Maria** â€” Cadrage fonctionnel :
1. Lister les **objectifs** (principal + secondaires)
2. DÃ©finir le **pÃ©rimÃ¨tre** : IN / OUT explicites
3. Identifier les **risques** et mitigations
4. Proposer **2-3 approches techniques** avec pour/contre

**Lise** (T3+ si aspect front/UI) â€” Direction visuelle :
1. Analyser l'**UI existante** (thÃ¨me, composants, patterns)
2. Proposer la **direction artistique** : style, ton, inspirations
3. DÃ©crire le **parcours utilisateur** (flux Ã©cran par Ã©cran)
4. Identifier les **composants** Ã  crÃ©er/rÃ©utiliser (atomic design)
5. RÃ©fÃ©rencer les **inspirations visuelles**

---

## Temps 3 â€” Analyse concurrentielle (Fabrice, T4-T5 uniquement)

> LancÃ© en parallÃ¨le avec le temps 2.

1. Identifier les **produits/services concurrents**
2. Analyser les **forces et faiblesses** de chaque concurrent
3. Identifier l'**utilisateur cible** et ses attentes
4. Lister les **fonctionnalitÃ©s diffÃ©renciatrices**
5. DÃ©tecter les **points de friction** des concurrents

---

## Temps 4 â€” Inventaire des prÃ©requis + Ã©valuation infra + modÃ¨les (Denis + Hugo + Jean + Ã©quipe)

Denis coordonne un inventaire complet. Hugo (T4-T5) Ã©value l'infrastructure. Jean recommande les modÃ¨les.

**1. CompÃ©tences requises** (par agent) :
- FranÃ§ois : compÃ©tences back-end requises
- Lise : compÃ©tences UI/front-end requises
- Denis : compÃ©tences architecture requises

**2. Connaissances nÃ©cessaires** :
- Domaine mÃ©tier, patterns existants (depuis `.mip/memory/mip-decisions.md`), anti-patterns (depuis `.mip/memory/patterns-and-lessons.md`), documentation

**3. Outils et ressources** :
- Paquets externes (versions, maintenance, compatibilitÃ©)
- Paquets/modules internes Ã  utiliser/modifier
- Outils de dev (compilateur, IDs de vÃ©rification docs, outils CLI)
- Assets, infrastructure, docs et rÃ©fÃ©rences

**4. Ã‰tapes gÃ©nÃ©rales** : Denis dÃ©compose en Ã©tapes macro (avant le plan atomique en temps 7) :
```markdown
### Ã‰tape N â€” <nom>
- Objectif : <ce que cette Ã©tape accomplit>
- Agents : <qui travaille>
- PrÃ©requis : <ce qui doit Ãªtre fait avant>
- Livrables : <ce qui est produit>
- CritÃ¨res de complÃ©tion : <comment savoir que c'est fait>
- Risques identifiÃ©s : <ce qui pourrait bloquer>
```

**5. Matrice de disponibilitÃ©** : Statut de chaque prÃ©requis (disponible / Ã  crÃ©er / manquant).

**6. Ã‰valuation infrastructure** (Hugo, T4-T5) : Serveurs, rÃ©seau (ports, TLS, DNS), PaaS/Stockage (volumes, sauvegarde), conteneurisation, CI/CD, scalabilitÃ©.

**7. Recommandation modÃ¨les** (Jean) : Analyser la classe (T1-T5), recommander le modÃ¨le par agent (opus/sonnet/haiku), estimer le budget tokens total. AutoritÃ© CONSULTATIVE â€” Denis et Maria valident.

---

## Temps 5 â€” Analyse de sÃ©curitÃ© (Victor, T3+)

Victor intervient aprÃ¨s l'inventaire (temps 4) et avant la spec (temps 6).

**5 domaines** :

1. **ModÃ¨le de menace** : Actifs Ã  protÃ©ger, acteurs (attaquants), surfaces d'attaque, scÃ©narios d'attaque, impact (CIA)

2. **Niveau de sÃ©curitÃ©** (depuis `.mip/environment.md` S2.8-S2.11) :
   - Standard : bases OWASP
   - Durci : Crypto obligatoire, audit rÃ©gulier, RGPD
   - Critique : Zero-trust, audit formel, conformitÃ© sectorielle

3. **Audit des dÃ©pendances** : CVE connus, dernier commit (>6 mois = risque), nombre de mainteneurs (<2 = risque), licence compatible

4. **Checklist sÃ©curitÃ© pour la spec** (transmise Ã  FranÃ§ois) :
   - [ ] Authentification : quel mÃ©canisme ?
   - [ ] Autorisation : quel modÃ¨le ?
   - [ ] Validation des entrÃ©es : quels points ?
   - [ ] Chiffrement : quelles donnÃ©es ? quel algorithme ?
   - [ ] Gestion des secrets : oÃ¹ stockÃ©s ?
   - [ ] Logging sÃ©curitÃ© : quels Ã©vÃ©nements ?
   - [ ] Limitation de dÃ©bit : quels endpoints ?
   - [ ] CORS : quelle politique ?

5. **Recommandations de durcissement** proportionnelles au niveau

### RPS - Rapport preliminaire de securite (obligatoire)

Le Temps 5 produit un **RPS** qui sert d'entree directe pour la spec, le plan et le futur audit P4.

Contenu minimal obligatoire du RPS :
1. Surfaces d'attaque et risques majeurs
2. Ressources securite necessaires (agents, controles, outillage)
3. Normes/certifications applicables (ISO 27001, HDS, NF525, etc.)
4. Niveau de securite requis par zone du perimetre
5. Conclusion avec le niveau de securite maximal requis

Sorties obligatoires du Temps 5 :
- Integrer le RPS dans le brief P0 (`<sequence>/briefs/`)
- Ajouter dans `<sequence>/ressources/index.md` les competences/certifications/procedures a charger
- Alimenter `<sequence>/gpi/` avec le volet securite :
  - selection et planification des implementations securite dans la sequence
  - ordre, dependances, criteres de completion
  - recherche/adaptation d'un prompt d'implementation securite reutilisable

---

## Temps 6 â€” SpÃ©cification technique + vÃ©rification docs (FranÃ§ois)

FranÃ§ois analyse le contexte technique, vÃ©rifie les docs, intÃ¨gre la checklist sÃ©curitÃ© de Victor.

1. Explorer le code existant en profondeur
2. **VÃ©rification docs obligatoire** pour chaque lib impliquÃ©e (Context7, recherche web ou fallback si indisponible) :
   - Documenter les breaking changes / dÃ©prÃ©ciations
   - Comparer avec les patterns existants
3. Charger les **anti-patterns connus** (MEMORY.md + patterns-and-lessons.md)
4. Identifier les **fichiers** Ã  modifier/crÃ©er avec numÃ©ros de ligne
5. DÃ©finir les **types, traits, API** (signatures complÃ¨tes validÃ©es contre les docs)
6. Ã‰valuer les **dÃ©pendances** entre modules
7. **ConformitÃ© architecturale** : Lois d'Autonomie, rÃ¨gles de sÃ»retÃ© du code, couche architecture, annotations, versions dÃ©pendances
8. **IntÃ©grer la checklist sÃ©curitÃ© de Victor** : auth, validation, chiffrement, secrets, limitation dÃ©bit
9. Documenter les **risques techniques**

**Production** : `<sequence>/specs/YYYY-MM-DD-<slug>.md` â€” commence par TL;DR 5 lignes max. **400 lignes max** (rÃ¨gle I-14). Si dÃ©passÃ©, dÃ©couper : `spec.md` (index) + `spec-module-X.md`.

---

## Temps 7 - Generation des agents fine-tuned de sequence (Maria)

Maria genere les prompts agents fine-tuned necessaires a l execution aval (P3-P6 + MASS).

**Entrees obligatoires** :
1. `<sequence>/phases/p0/temps/temps-04-inventaire.md`
2. `<sequence>/phases/p0/temps/temps-05-securite.md`
3. `<sequence>/specs/YYYY-MM-DD-<slug>-spec.md`
4. Template canonique : `.mip/agents/TEMPLATE_PHASE_AGENT.md`
5. Bases agents : `.mip/agents/<agent>/FULL_<agent>.md` (obligatoire) ; `.mip/agents/<agent>/<PHASE>_<agent>.md` (optionnel, si present)

**Sorties obligatoires** (dans la sequence) :
- `<sequence>/agents/index.md`
- `<sequence>/agents/manifest.json`
- `<sequence>/agents/<PHASE>_<agent>.md`

**Generation** :
```powershell
powershell -ExecutionPolicy Bypass -File .mip/scripts/generate-sequence-finetuned-agents.ps1 -SequencePath <sequence> -RegenerationMode update -TargetPhases P3,P4,P5,P6,MASS
```

**Regles de selection** :
- Base sur les besoins emergents T4/T5/T6 (agents + capacites detectes)
- Fallback deterministe par classe si extraction insuffisante
- Generation cible `P3,P4,P5,P6,MASS` pour les agents retenus
- Source prioritaire : `FULL_<agent>.md`, puis enrichissement par `<PHASE>_<agent>.md` si disponible

**Gate stricte** : le Temps 8 (plan Denis) ne demarre qu apres generation terminee et manifeste agents valide.

---

## Temps 8 - Plan exhaustif et guide d'implementation (Denis)

Denis compile l'inventaire (T4) + sÃ©curitÃ© (T5) + spec (T6) et produit le plan exhaustif.

1. **DÃ©composer en tÃ¢ches atomiques** (2-5 minutes chacune)
2. **Couvrir** : Code (FranÃ§ois+Lise), tests unitaires, tests d'intÃ©gration, tests sÃ©curitÃ© (Victor), tests globaux, audit (George+Victor), infra (Hugo), buffer corrections (20 %)
3. **Chaque tÃ¢che contient** :
   - NumÃ©ro sÃ©quentiel + catÃ©gorie (`[CODE-01]`, `[TEST-U-01]`, `[TEST-I-01]`, `[TEST-S-01]`, `[AUDIT-01]`, `[INFRA-01]`)
   - Agent assignÃ©
   - Fichier(s) exact(s) (chemin complet)
   - Code complet Ã  Ã©crire
   - Commande test + sortie attendue
   - Message de commit
   - DÃ©pendances (`depends: [CODE-01, CODE-02]`)
4. **Principe** : Supposer que l'exÃ©cuteur a ZÃ‰RO contexte projet
5. **Ordonnancement** : Par dÃ©pendance. TÃ¢ches indÃ©pendantes marquÃ©es parallÃ©lisables.
6. **Guide d'implÃ©mentation** par Ã©tape macro :
```markdown
## Guide â€” Ã‰tape X : <nom>
### PrÃ©requis : compÃ©tences, outils, paquets, docs
### TÃ¢ches : [CODE-01] -> [CODE-02] -> [TEST-U-01] -> ...
### CritÃ¨res de complÃ©tion :
- [ ] Tests de l'Ã©tape passent
- [ ] Lint propre
- [ ] Revue de code (checkpoint Denis si >=5 tÃ¢ches)
```

**Production** : `<sequence>/plans_p3/YYYY-MM-DD-<slug>.md` â€” commence par TL;DR 5 lignes max. **400 lignes max** (rÃ¨gle I-14). Si dÃ©passÃ©, dÃ©couper : `plan.md` (index + navigation) + `plan-etape-X.md` par Ã©tape macro.

---

## Temps 9 - Audit de faisabilite, conformite et validation efficience (Arianne + Jean)

Arianne vÃ©rifie que le projet est faisable tel que planifiÃ©. Jean valide l'efficience du plan.

**VÃ©rification agents** : Agents requis avec compÃ©tences, capacitÃ© du modÃ¨le LLM, cohÃ©rence inter-agents (sorties -> entrÃ©es).

**VÃ©rification dÃ©pendances** : Paquets externes (existent, maintenus, compatibles), paquets internes (types/traits dÃ©finis), outils disponibles.

**VÃ©rification mÃ©moire** : Anti-patterns (patterns-and-lessons.md), patterns confirmÃ©s (mip-decisions.md), historique (mip-performance-history.md).

**VÃ©rification docs spot-check** : Spot-check 2-3 patterns critiques, breaking changes rÃ©cents.

**Diagnostic** :
| RÃ©sultat | Action |
|----------|--------|
| Conforme | Feu vert -> Maria compile le brief |
| Manques mineurs | Lister les manques, corriger le plan |
| AmbiguÃ¯tÃ© | Poser des questions Ã  l'utilisateur/agent |
| Manque critique | SuggÃ©rer un **projet prÃ©curseur** (T2-T3) |
| Infaisable | SuggÃ©rer une rÃ©orientation |

**Validation efficience** (Jean) : Lister les fichiers chargÃ©s par chaque agent, identifier les redondances, recommander le chargement sÃ©lectif (modules, index+drill-down), valider que les modules SKILL.md requis sont identifiÃ©s.

---

## Temps 10 - Verification pipeline CI/CD (Hugo, si CI/CD en place)

Hugo vÃ©rifie la compatibilitÃ© du pipeline existant avec le nouveau code.

1. Relire la config CI/CD (`.github/workflows/`, `.gitlab-ci.yml`)
2. CompatibilitÃ© des Ã©tapes existantes avec les nouveaux paquets/fichiers
3. Adaptations requises (jobs, variables, secrets CI)
4. Temps de build supplÃ©mentaire estimÃ©
5. Si pas de CI/CD : proposer une configuration initiale

---

## Temps 11 - Synthese et brief (Maria)

Maria compile tout et prÃ©sente le brief en suivant la sÃ©quence R-P0-3 :

1. Fusionner les contributions de tous les agents
2. IntÃ©grer l'audit d'Arianne
3. RÃ©diger le brief structurÃ© (modÃ¨le ci-dessous)
4. **Ã‰crire le brief** dans `<sequence>/briefs/YYYY-MM-DD-<slug>.md` â€” **400 lignes max** (rÃ¨gle I-14). Si dÃ©passÃ©, dÃ©couper : `brief.md` (index + TL;DR + dÃ©cisions) + `brief-annexe-X.md` (plan Denis, analyse Fabrice, etc.)
5. **PrÃ©senter dans le chat** (R-P0-3) :
   - TL;DR (5 lignes, copiÃ©es du brief)
   - Section Approches proposÃ©es (avec recommandation)
   - Section Risques (tableau)
   - Lien vers le fichier complet : Â« Brief complet : `<sequence>/briefs/YYYY-MM-DD-<slug>.md` Â»
6. **AskUserQuestion** â€” approbation : APPROUVÃ‰ / MODIFIÃ‰ / REJETÃ‰
7. **SI APPROUVÃ‰ -> AskUserQuestion** â€” mode autonomie : FULL / BIG_STEPS / GUIDED (invariant I-4, aprÃ¨s lecture du brief)
8. **SI APPROUVÃ‰ + mode choisi** : initialiser le scaffold standard de sÃ©quence (artefacts futurs + mini-site JSX) :
   `powershell -ExecutionPolicy Bypass -File .mip/scripts/init-sequence-standard-artifacts.ps1 -SequencePath <sequence>`

### ModÃ¨le de brief

```markdown
# Brief : <titre>

## TL;DR (5 lignes max)
<RÃ©sumÃ© ultra-concis : projet, approche, effort, risque, Ã©tape critique>

## MÃ©tadonnÃ©es
- Classe : T3/T4/T5
- Date : YYYY-MM-DD

## Contexte
[Pourquoi cette demande, quel problÃ¨me]

## Objectifs
- Principal : ...
- Secondaires : ...
- CritÃ¨res de succÃ¨s mesurables : ...

## PÃ©rimÃ¨tre
### Inclus
- [FonctionnalitÃ©s IN]
### Exclus
- [FonctionnalitÃ©s OUT]

## Approches proposÃ©es
### Approche A â€” [nom] (RECOMMANDÃ‰E)
- Description, pour, contre, effort

### Approche B â€” [nom]
- Description, pour, contre

## Direction visuelle (Lise)
- Style/ton, composants, parcours utilisateur, inspirations

## Analyse concurrentielle (Fabrice, T4-T5)
- Concurrents, diffÃ©renciateurs, cible

## Analyse de sÃ©curitÃ© (Victor, T3+)
### ModÃ¨le de menace
| Surface | ScÃ©nario | Impact | Mitigation |
### Niveau + ConformitÃ© + DÃ©pendances auditÃ©es
### Checklist sÃ©curitÃ© + Recommandations

## Pipeline CI/CD (Hugo, si applicable)
- CompatibilitÃ©, adaptations, impact build

## Inventaire des prÃ©requis (Denis + Hugo + Ã©quipe)
### CompÃ©tences, connaissances, outils, Ã©tapes gÃ©nÃ©rales

## SpÃ©cification technique (FranÃ§ois)
- Fichiers, types/API, conformitÃ©, risques

## Plan de dÃ©veloppement (Denis)
[Voir annexe plans_p3/]
- Nombre total de tÃ¢ches (code, tests, audit, infra, buffer)

## Audit de faisabilitÃ© (Arianne)
- ConformitÃ© agents, dÃ©pendances, mÃ©moire
- Verdict : CONFORME / MANQUES MINEURS / PRÃ‰REQUIS

## Risques
| Risque | ProbabilitÃ© | Impact | Mitigation |

## DÃ©cision
APPROUVÃ‰ / REJETÃ‰ / MODIFIÃ‰ / PRÃ‰REQUIS D'ABORD

## Mode d'autonomie (choisi APRÃˆS lecture du brief â€” invariant I-4)
> L'utilisateur choisit en pleine connaissance aprÃ¨s avoir lu l'intÃ©gralitÃ© du brief ci-dessus.
- [ ] FULL | [ ] BIG_STEPS | [ ] GUIDED
- Conserver pour les futures sÃ©quences ? OUI / NON / PAS SÃ›R
```

**Gate qualitÃ© P0** (sÃ©quence stricte â€” invariant I-4, R-P0-3) :

La Gate P0 se fait en **2 appels AskUserQuestion distincts**, jamais en 1 :

**AskUserQuestion 1 â€” Approbation** (aprÃ¨s prÃ©sentation TL;DR + approches + risques dans le chat) :
- Question : Â« Approuvez-vous ce brief ? Â»
- Options : APPROUVÃ‰ / MODIFIÃ‰ (prÃ©ciser les changements) / REJETÃ‰ (prÃ©ciser la raison)

**AskUserQuestion 2 â€” Mode autonomie** (seulement si APPROUVÃ‰) :
- Question : Â« Quel mode d'autonomie pour l'exÃ©cution ? Â»
- Options :
  - FULL â€” Autopilot complet, prochaine interaction = test P5
  - BIG_STEPS â€” Gates entre chaque phase (P3->P4, P4->P5)
  - GUIDED â€” Validation Ã  chaque Ã©tape

Gates strictes :
- PAS de passage en exÃ©cution sans brief approuvÃ©
- PAS de choix d'autonomie sans lecture prÃ©alable du brief
- PAS de mÃ©lange approbation + autonomie dans la mÃªme question


