# Caring Nanny - Observation Flow Contract

## 1. Contexte

Ce document dÃ©finit le **contrat normatif du flux d'observation** de Caring Nanny. Le flux d'observation est le mÃ©canisme fondamental par lequel Caring Nanny collecte, Ã©value, agrÃ¨ge et enregistre les Ã©tats du systÃ¨me Miyukini.

Le flux d'observation est **strictement passif** : il ne modifie jamais l'Ã©tat du systÃ¨me qu'il observe, conformÃ©ment Ã  l'invariant **INV-CN-1** (Observateur pur).

Ce contrat est **dÃ©rivÃ© de la Documentation Fondatrice de Caring Nanny** (Section 8 - Interactions avec l'Ã©cosystÃ¨me) et de l'**Architecture et Composants** (Section 5 - Flux de donnÃ©es internes).

**Documents sources :**
- [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [Caring Nanny - Architecture et Composants](../../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

## 2. PortÃ©e / Scope

- **Applicable Ã  :** Toutes les opÃ©rations d'observation d'Ã©tat dans Caring Nanny
- **Audience :** Architectes, dÃ©veloppeurs, intÃ©grateurs, autres cores de l'Ã©cosystÃ¨me
- **Statut :** Contrat normatif â€” Non nÃ©gociable
- **DÃ©pendances :** Documentation Fondatrice Caring Nanny, Architecture et Composants, Glossaire Miyukini, Lois d'Autonomie SystÃ¨me

Ce document dÃ©finit :
- Les quatre Ã©tapes du flux d'observation
- Les composants impliquÃ©s Ã  chaque Ã©tape
- Les rÃ¨gles et contraintes de chaque Ã©tape
- Les garanties du flux d'observation
- Les conditions d'entrÃ©e et de sortie

Ce document **ne couvre pas** :
- Le flux de propagation (voir Caring Nanny - Propagation Flow Contract)
- Le flux de consultation (voir Caring Nanny - Consultation Contract)
- Les contrats d'intÃ©gration avec les autres cores (voir contracts/integration/)

---

## 3. Vue d'ensemble du flux d'observation

Le flux d'observation est composÃ© de **quatre Ã©tapes sÃ©quentielles et obligatoires** :

```
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚                        FLUX D'OBSERVATION                                   â”‚
â”œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â”‚                                                                             â”‚
â”‚  Ã‰TAPE 1              Ã‰TAPE 2              Ã‰TAPE 3              Ã‰TAPE 4     â”‚
â”‚  DÃ‰TECTION    â”€â”€â–º    Ã‰VALUATION    â”€â”€â–º   AGRÃ‰GATION    â”€â”€â–º   TRANSITION    â”‚
â”‚                                                                             â”‚
â”‚  Composant           Condition            Ã‰tats partiels       Ã‰tat global  â”‚
â”‚  Ã©met une      â”€â”€â–º   traduite en    â”€â”€â–º   agrÃ©gÃ©s en     â”€â”€â–º   comparÃ© et   â”‚
â”‚  condition           Ã©tat partiel         Ã©tat systÃ¨me         enregistrÃ©   â”‚
â”‚                                                                             â”‚
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

**PropriÃ©tÃ©s fondamentales du flux :**

| PropriÃ©tÃ© | Description |
|-----------|-------------|
| **SÃ©quentiel** | Les Ã©tapes s'exÃ©cutent dans l'ordre, sans saut possible |
| **DÃ©terministe** | Une mÃªme condition produit toujours le mÃªme rÃ©sultat |
| **Non-bloquant** | Le flux n'interfÃ¨re jamais avec les opÃ©rations du systÃ¨me |
| **TraÃ§able** | Chaque Ã©tape produit des donnÃ©es auditables |
| **Passif** | Aucune modification de l'Ã©tat observÃ© |

---

## 4. Ã‰tape 1 : DÃ©tection de condition

### 4.1 DÃ©finition

La **dÃ©tection de condition** est le mÃ©canisme par lequel Caring Nanny collecte les faits observables depuis les composants du systÃ¨me. Une condition est un fait brut, avant toute interprÃ©tation en termes d'Ã©tat.

### 4.2 Composants impliquÃ©s

```
Composant source (KindMother, StrongFather, Module SPM, ...)
         â”‚
         â”‚ Condition brute
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ ComponentProbe  â”‚ â† Sonde passive spÃ©cifique au type de composant
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚ Condition dÃ©tectÃ©e
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ ConditionNormalizer â”‚ â† Normalisation dans un format unifiÃ©
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚ Condition normalisÃ©e
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ TimestampMarker â”‚ â† Horodatage local (conforme LOI-4)
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚ Condition horodatÃ©e
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ConditionCollectorâ”‚ â† Point de collecte centralisÃ©
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
```

### 4.3 Types de conditions dÃ©tectÃ©es

| Source | Type de condition | Exemples |
|--------|-------------------|----------|
| **KindMother** | SantÃ© de persistance | Disponible, dÃ©gradÃ©, indisponible |
| **KindMother** | Synchronisation | SynchronisÃ©, en cours, dÃ©synchronisÃ©, conflits |
| **KindMother** | Instances | DB MÃ¨re accessible, DB Filles connectÃ©es |
| **KindMother** | OpÃ©rations | Ã‰critures en attente, deltas non propagÃ©s |
| **StrongFather** | Politiques | Active, suspendue, en validation |
| **StrongFather** | Ã‰valuations | En cours, succÃ¨s, Ã©chec |
| **Modules SPM** | Ã‰tat module | PrÃªt, en chargement, erreur |
| **RÃ©seau** | ConnectivitÃ© | Disponible, indisponible, latente |
| **SystÃ¨me** | Ressources | MÃ©moire, CPU, stockage |

### 4.4 RÃ¨gles de dÃ©tection

| RÃ¨gle | Ã‰noncÃ© | RÃ©fÃ©rence |
|-------|--------|-----------|
| **RÃˆGLE-DET-1** | Toute dÃ©tection est **passive** et sans effet de bord | INV-CN-1 |
| **RÃˆGLE-DET-2** | Une condition est **factuelle** (fait observÃ©, pas interprÃ©tation) | Section 4, Doc Fondatrice |
| **RÃˆGLE-DET-3** | Chaque condition est **horodatÃ©e localement** | LOI-4 |
| **RÃˆGLE-DET-4** | La dÃ©tection est **non-bloquante** pour le composant observÃ© | INV-CN-6 |
| **RÃˆGLE-DET-5** | La sonde est **spÃ©cifique au type de composant** | Architecture 3.1 |
| **RÃˆGLE-DET-6** | Le format de condition est **normalisÃ©** avant collecte | Architecture 3.1 |

### 4.5 Format de condition normalisÃ©e

```
Condition {
    source_id        : Identifiant unique du composant source
    source_type      : Type du composant (kindmother, strongfather, spm_module, ...)
    condition_type   : Type de condition (health, sync, operation, ...)
    condition_value  : Valeur brute de la condition
    timestamp_local  : Horodatage local (pas de temps global, conforme LOI-4)
    context          : MÃ©tadonnÃ©es contextuelles
}
```

### 4.6 Conditions d'entrÃ©e et de sortie

**EntrÃ©e :** Un composant du systÃ¨me Ã©met un fait observable (changement de connexion, fin d'opÃ©ration, erreur, etc.)

**Sortie :** Une condition normalisÃ©e et horodatÃ©e est transmise Ã  l'Ã©tape d'Ã©valuation

**Ã‰chec possible :** Si la condition ne peut pas Ãªtre normalisÃ©e, elle est enregistrÃ©e comme anomalie et transmise telle quelle avec un marqueur d'erreur de normalisation.

---

## 5. Ã‰tape 2 : Ã‰valuation de l'Ã©tat

### 5.1 DÃ©finition

L'**Ã©valuation de l'Ã©tat** est le mÃ©canisme par lequel Caring Nanny traduit une condition brute en Ã©tat partiel classifiÃ©. Cette Ã©tape applique les rÃ¨gles de classification pour transformer un fait en catÃ©gorie d'Ã©tat.

### 5.2 Composants impliquÃ©s

```
Condition normalisÃ©e (depuis ConditionCollector)
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ StateEvaluator  â”‚ â† Ã‰valuation condition â†’ Ã©tat partiel
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚ Ã‰tat partiel
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚CategoryClassifierâ”‚ â† Classification selon les 5 catÃ©gories
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
     Ã‰tat classifiÃ©
```

### 5.3 CatÃ©gories d'Ã©tat

Caring Nanny classifie chaque Ã©tat dans l'une des **cinq catÃ©gories exclusives** dÃ©finies dans la Documentation Fondatrice :

| CatÃ©gorie | DÃ©finition | Comportement attendu |
|-----------|------------|---------------------|
| **healthy** | Tous les composants fonctionnent normalement | OpÃ©rations normales |
| **degraded** | Certains composants en mode dÃ©gradÃ©, systÃ¨me opÃ©rationnel | FonctionnalitÃ©s rÃ©duites |
| **offline** | Mode dÃ©connectÃ©, sans accÃ¨s aux autoritÃ©s centrales | Autonomie locale (LOI-2) |
| **syncing** | Synchronisation en cours, opÃ©rations potentiellement diffÃ©rÃ©es | Ã‰tat transitoire |
| **error** | Erreur critique dÃ©tectÃ©e, certaines opÃ©rations impossibles | Investigation requise |

**Important (LOI-2) :** L'Ã©tat `offline` est un Ã©tat **normal**, pas une erreur. Il reprÃ©sente l'isolement acceptÃ© du systÃ¨me, conformÃ©ment Ã  la Loi d'Autonomie LOI-2.

### 5.4 RÃ¨gles d'Ã©valuation

| RÃ¨gle | Ã‰noncÃ© | RÃ©fÃ©rence |
|-------|--------|-----------|
| **RÃˆGLE-EVAL-1** | L'Ã©valuation est **dÃ©terministe** : une condition donnÃ©e produit toujours le mÃªme Ã©tat | Architecture 3.2 |
| **RÃˆGLE-EVAL-2** | L'Ã©valuation est **reproductible** : le contexte est suffisant pour reproduire le rÃ©sultat | Architecture 3.2 |
| **RÃˆGLE-EVAL-3** | Chaque Ã©valuation produit **exactement une** catÃ©gorie d'Ã©tat | INV-CN-4 |
| **RÃˆGLE-EVAL-4** | L'Ã©tat `offline` n'est **jamais** Ã©valuÃ© comme `error` (isolation â‰  anomalie) | LOI-2 |
| **RÃˆGLE-EVAL-5** | L'Ã©valuation **n'interprÃ¨te pas**, elle applique des rÃ¨gles dÃ©finies | Architecture 3.2 |
| **RÃˆGLE-EVAL-6** | Les rÃ¨gles d'Ã©valuation sont **fournies par le produit ou l'Ã©cosystÃ¨me** | Section 6, Doc Fondatrice |

### 5.5 Matrice d'Ã©valuation par type de condition

| Type de condition | CritÃ¨re healthy | CritÃ¨re degraded | CritÃ¨re offline | CritÃ¨re syncing | CritÃ¨re error |
|-------------------|-----------------|------------------|-----------------|-----------------|---------------|
| **SantÃ© persistance** | Disponible | Latence Ã©levÃ©e | N/A | N/A | Indisponible |
| **Synchronisation** | SynchronisÃ© | Conflits mineurs | DÃ©connectÃ© | En cours | Ã‰chec rÃ©pÃ©tÃ© |
| **ConnectivitÃ©** | Disponible | Latente | Indisponible | Reconnexion | N/A |
| **Ressources** | Normales | Proches des limites | N/A | N/A | Ã‰puisÃ©es |
| **OpÃ©rations** | SuccÃ¨s | Retry nÃ©cessaire | DiffÃ©rÃ©es | En attente | Ã‰chec critique |

### 5.6 Format d'Ã©tat partiel

```
PartialState {
    source_id        : Identifiant du composant source
    source_type      : Type du composant
    category         : healthy | degraded | offline | syncing | error
    condition        : Condition source (rÃ©fÃ©rence)
    evaluation_rules : RÃ¨gles appliquÃ©es pour cette Ã©valuation
    timestamp        : Horodatage de l'Ã©valuation
    confidence       : Niveau de confiance de l'Ã©valuation (high, medium, low)
}
```

### 5.7 Conditions d'entrÃ©e et de sortie

**EntrÃ©e :** Une condition normalisÃ©e et horodatÃ©e

**Sortie :** Un Ã©tat partiel classifiÃ© dans l'une des cinq catÃ©gories

**Ã‰chec possible :** Si aucune rÃ¨gle d'Ã©valuation ne correspond, l'Ã©tat est classifiÃ© comme `error` avec une note explicative.

---

## 6. Ã‰tape 3 : AgrÃ©gation

### 6.1 DÃ©finition

L'**agrÃ©gation** est le mÃ©canisme par lequel Caring Nanny synthÃ©tise les Ã©tats partiels de tous les composants en un Ã©tat systÃ¨me global unique et cohÃ©rent.

### 6.2 Composants impliquÃ©s

```
Ã‰tats partiels (depuis CategoryClassifier)
         â”‚
         â”‚ Multiple Ã©tats partiels
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚ StateAggregator â”‚ â† AgrÃ©gation en Ã©tat systÃ¨me global
â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
         â–¼
     Ã‰tat systÃ¨me global
```

### 6.3 RÃ¨gles d'agrÃ©gation

L'agrÃ©gation suit des **rÃ¨gles de prioritÃ©** pour rÃ©soudre les situations oÃ¹ diffÃ©rents composants sont dans des Ã©tats diffÃ©rents.

**RÃ¨gle de prioritÃ© (du plus critique au moins critique) :**

```
error > syncing > offline > degraded > healthy
```

| RÃ¨gle | Ã‰noncÃ© | Justification |
|-------|--------|---------------|
| **RÃˆGLE-AGG-1** | Si **au moins un** composant est en `error`, l'Ã©tat systÃ¨me est `error` | La criticitÃ© prime |
| **RÃˆGLE-AGG-2** | Si aucun `error` mais **au moins un** `syncing`, l'Ã©tat systÃ¨me est `syncing` | Synchronisation active |
| **RÃˆGLE-AGG-3** | Si aucun `error`/`syncing` mais **au moins un** `offline`, l'Ã©tat systÃ¨me est `offline` | Isolation dÃ©tectÃ©e |
| **RÃˆGLE-AGG-4** | Si aucun `error`/`syncing`/`offline` mais **au moins un** `degraded`, l'Ã©tat systÃ¨me est `degraded` | DÃ©gradation partielle |
| **RÃˆGLE-AGG-5** | Si **tous** les composants sont `healthy`, l'Ã©tat systÃ¨me est `healthy` | Fonctionnement nominal |
| **RÃˆGLE-AGG-6** | L'agrÃ©gation est **dÃ©terministe** et **reproductible** | INV-CN-4 |

### 6.4 RÃ©solution des contradictions

Les contradictions apparentes sont rÃ©solues par les rÃ¨gles de prioritÃ©. Une contradiction est un cas oÃ¹ l'interprÃ©tation naturelle des Ã©tats est ambiguÃ«.

**Exemples de rÃ©solution :**

| Ã‰tats partiels observÃ©s | Ã‰tat systÃ¨me rÃ©sultant | Justification |
|------------------------|----------------------|---------------|
| healthy + healthy | healthy | Tous nominaux |
| healthy + degraded | degraded | Un composant dÃ©gradÃ© affecte le systÃ¨me |
| healthy + offline | offline | Isolation dÃ©tectÃ©e |
| degraded + syncing | syncing | Synchronisation prioritaire sur dÃ©gradation |
| error + healthy + healthy | error | Une erreur critique suffit |
| offline + offline | offline | SystÃ¨me isolÃ© (Ã©tat normal, LOI-2) |

### 6.5 Format d'Ã©tat systÃ¨me global

```
SystemState {
    category             : healthy | degraded | offline | syncing | error
    partial_states       : Liste des Ã©tats partiels contributifs
    contributing_sources : Liste des composants ayant contribuÃ©
    aggregation_rules    : RÃ¨gles appliquÃ©es pour l'agrÃ©gation
    timestamp            : Horodatage de l'agrÃ©gation
    previous_state       : RÃ©fÃ©rence Ã  l'Ã©tat systÃ¨me prÃ©cÃ©dent
}
```

### 6.6 Conditions d'entrÃ©e et de sortie

**EntrÃ©e :** Un ou plusieurs Ã©tats partiels classifiÃ©s

**Sortie :** Un Ã©tat systÃ¨me global unique et cohÃ©rent

**Ã‰chec possible :** Aucun Ã©chec possible â€” l'agrÃ©gation produit toujours un rÃ©sultat valide grÃ¢ce aux rÃ¨gles de prioritÃ©.

---

## 7. Ã‰tape 4 : DÃ©tection de transition

### 7.1 DÃ©finition

La **dÃ©tection de transition** est le mÃ©canisme par lequel Caring Nanny identifie et enregistre les changements d'Ã©tat systÃ¨me. Une transition est le passage d'un Ã©tat Ã  un autre.

### 7.2 Composants impliquÃ©s

```
Ã‰tat systÃ¨me global (depuis StateAggregator)
         â”‚
         â–¼
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
â”‚TransitionDetectorâ”‚ â† Comparaison avec l'Ã©tat prÃ©cÃ©dent
â””â”€â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â”‚
    â”Œâ”€â”€â”€â”€â”´â”€â”€â”€â”€â”
    â–¼         â–¼
Historique  Propagation
(HistoryStore)  (si transition dÃ©tectÃ©e)
```

### 7.3 CaractÃ©ristiques d'une transition

| PropriÃ©tÃ© | Description | RÃ©fÃ©rence |
|-----------|-------------|-----------|
| **DÃ©terministe** | Un Ã©tat donnÃ© ne peut conduire qu'Ã  un ensemble fini d'Ã©tats possibles | Section 4, Doc Fondatrice |
| **Observable** | La transition elle-mÃªme est un fait observable | Section 4, Doc Fondatrice |
| **TraÃ§able** | Chaque transition est enregistrÃ©e avec son contexte | INV-CN-5 |
| **Causale** | Une transition a toujours une cause identifiable | Section 4, Doc Fondatrice |

### 7.4 RÃ¨gles de dÃ©tection de transition

| RÃ¨gle | Ã‰noncÃ© | RÃ©fÃ©rence |
|-------|--------|-----------|
| **RÃˆGLE-TRANS-1** | Une transition est dÃ©tectÃ©e si et seulement si `Ã©tat_actuel â‰  Ã©tat_prÃ©cÃ©dent` | DÃ©finition de transition |
| **RÃˆGLE-TRANS-2** | Chaque transition est **enregistrÃ©e** avec l'Ã©tat prÃ©cÃ©dent, l'Ã©tat actuel, et la cause | INV-CN-5 |
| **RÃˆGLE-TRANS-3** | La cause est la **condition qui a dÃ©clenchÃ©** l'Ã©valuation menant Ã  la transition | TraÃ§abilitÃ© |
| **RÃˆGLE-TRANS-4** | Si aucune transition n'est dÃ©tectÃ©e, l'Ã©tat est tout de mÃªme mis Ã  jour dans l'historique (avec marqueur "unchanged") | AuditabilitÃ© |
| **RÃˆGLE-TRANS-5** | Une transition dÃ©clenche **optionnellement** une propagation (voir Propagation Flow Contract) | Architecture 5.2 |

### 7.5 Transitions valides

Le graphe suivant dÃ©finit les transitions valides entre Ã©tats systÃ¨me :

```
                    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”
                    â”‚                                      â”‚
                    â–¼                                      â”‚
        â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                 â”‚
        â”‚    healthy     â”‚â—„â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
        â””â”€â”€â”€â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”€â”€â”€â”€â”˜                                 â”‚
                â”‚                                          â”‚
    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¼â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                          â”‚
    â”‚           â”‚               â”‚                          â”‚
    â–¼           â–¼               â–¼                          â”‚
â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”  â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”    â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”                      â”‚
â”‚degradedâ”‚  â”‚offline â”‚    â”‚ syncing â”‚â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
â””â”€â”€â”€â”¬â”€â”€â”€â”€â”˜  â””â”€â”€â”€â”¬â”€â”€â”€â”€â”˜    â””â”€â”€â”€â”€â”¬â”€â”€â”€â”€â”˜                      â”‚
    â”‚           â”‚              â”‚                           â”‚
    â”‚           â”‚              â”‚                           â”‚
    â–¼           â–¼              â–¼                           â”‚
    â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”´â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”¤
                â”‚                                          â”‚
                â–¼                                          â”‚
         â”Œâ”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”                                     â”‚
         â”‚   error   â”‚â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
         â””â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”˜
           (peut revenir Ã  tout Ã©tat aprÃ¨s rÃ©solution)
```

**Transitions depuis healthy :**
- healthy â†’ degraded (dÃ©gradation partielle)
- healthy â†’ offline (perte de connexion)
- healthy â†’ syncing (dÃ©but de synchronisation)
- healthy â†’ error (erreur critique dÃ©tectÃ©e)

**Transitions depuis degraded :**
- degraded â†’ healthy (rÃ©cupÃ©ration)
- degraded â†’ offline (perte de connexion en mode dÃ©gradÃ©)
- degraded â†’ syncing (dÃ©but de synchronisation)
- degraded â†’ error (aggravation)

**Transitions depuis offline :**
- offline â†’ healthy (reconnexion rÃ©ussie)
- offline â†’ syncing (dÃ©but de synchronisation aprÃ¨s reconnexion)
- offline â†’ degraded (reconnexion partielle)
- offline â†’ error (erreur en mode isolÃ©)

**Transitions depuis syncing :**
- syncing â†’ healthy (synchronisation rÃ©ussie)
- syncing â†’ degraded (synchronisation partielle)
- syncing â†’ offline (perte de connexion pendant sync)
- syncing â†’ error (Ã©chec de synchronisation)

**Transitions depuis error :**
- error â†’ healthy (rÃ©solution complÃ¨te)
- error â†’ degraded (rÃ©solution partielle)
- error â†’ offline (isolation aprÃ¨s erreur)
- error â†’ syncing (tentative de rÃ©cupÃ©ration par sync)

### 7.6 Format de transition

```
Transition {
    id                 : Identifiant unique de la transition
    previous_state     : Ã‰tat systÃ¨me avant la transition
    current_state      : Ã‰tat systÃ¨me aprÃ¨s la transition
    trigger_condition  : Condition qui a dÃ©clenchÃ© la transition
    trigger_source     : Composant source de la condition dÃ©clencheuse
    timestamp          : Horodatage de la dÃ©tection de transition
    propagation_needed : Boolean (indique si propagation requise)
    metadata           : Contexte additionnel
}
```

### 7.7 Conditions d'entrÃ©e et de sortie

**EntrÃ©e :** Un Ã©tat systÃ¨me global et l'Ã©tat systÃ¨me prÃ©cÃ©dent

**Sortie :** 
- Si transition : un enregistrement de transition dans l'historique + notification de propagation
- Si pas de transition : un enregistrement "unchanged" dans l'historique

**Ã‰chec possible :** Aucun â€” la comparaison d'Ã©tats est toujours possible.

---

## 8. Garanties du flux d'observation

Le flux d'observation garantit les propriÃ©tÃ©s suivantes, dÃ©rivÃ©es des invariants de la Documentation Fondatrice :

### 8.1 Garantie de passivitÃ© (INV-CN-1)

> Le flux d'observation ne modifie **jamais** l'Ã©tat du systÃ¨me observÃ©.

**VÃ©rification :** Ã€ aucune Ã©tape du flux, une Ã©criture ou modification n'est effectuÃ©e sur les composants observÃ©s.

### 8.2 Garantie de cohÃ©rence (INV-CN-4)

> L'Ã©tat systÃ¨me rapportÃ© est **toujours cohÃ©rent** â€” aucune contradiction interne.

**VÃ©rification :** L'agrÃ©gation dÃ©terministe garantit qu'un seul Ã©tat est produit, sans ambiguÃ¯tÃ©.

### 8.3 Garantie de traÃ§abilitÃ© (INV-CN-5)

> Chaque observation, Ã©valuation, agrÃ©gation et transition est **entiÃ¨rement traÃ§able**.

**VÃ©rification :** Chaque Ã©tape produit des donnÃ©es structurÃ©es enregistrÃ©es dans l'historique.

### 8.4 Garantie de non-blocage (INV-CN-6)

> Le flux d'observation ne bloque **jamais** les opÃ©rations du systÃ¨me.

**VÃ©rification :** Toutes les opÃ©rations sont asynchrones et non-bloquantes.

### 8.5 Garantie d'autonomie (LOI-1 Ã  LOI-5)

> Le flux d'observation fonctionne **localement**, sans dÃ©pendance externe.

| Loi | ConformitÃ© | MÃ©canisme |
|-----|------------|-----------|
| **LOI-1** | âœ… | Observation locale, pas d'appel externe obligatoire |
| **LOI-2** | âœ… | L'Ã©tat `offline` est reconnu comme Ã©tat normal |
| **LOI-3** | âœ… | L'historique local est souverain |
| **LOI-4** | âœ… | Horodatage local, pas de temps global requis |
| **LOI-5** | âœ… | Flux lÃ©ger, consommation minimale de ressources |

---

## 9. Anomalies et cas limites

### 9.1 Condition non normalisable

**Situation :** Une condition brute ne peut pas Ãªtre normalisÃ©e par le ConditionNormalizer.

**Comportement :** La condition est marquÃ©e comme `anomaly:normalization_failure` et transmise avec cette annotation. L'Ã©valuation classifie l'Ã©tat comme `error` avec mention de l'anomalie.

### 9.2 RÃ¨gle d'Ã©valuation absente

**Situation :** Aucune rÃ¨gle d'Ã©valuation ne correspond Ã  la condition.

**Comportement :** L'Ã©tat est classifiÃ© comme `error` avec mention `evaluation_rule_missing`. Un signal d'alerte est Ã©mis pour configuration manquante.

### 9.3 Composant non observable

**Situation :** Un composant configurÃ© pour observation ne rÃ©pond pas Ã  la sonde.

**Comportement :** L'Ã©tat partiel du composant est classifiÃ© comme `error` avec mention `probe_timeout`. L'agrÃ©gation inclut cet Ã©tat dans le calcul.

### 9.4 Historique saturÃ©

**Situation :** Le HistoryStore atteint sa capacitÃ© maximale.

**Comportement :** Les observations les plus anciennes sont archivÃ©es selon la politique de rÃ©tention. Le flux continue sans interruption.

---

## 10. Invariants applicables au flux

Ce contrat est gouvernÃ© par les invariants suivants :

| Invariant | Ã‰noncÃ© | Application au flux |
|-----------|--------|---------------------|
| **INV-CN-1** | Observateur pur | Aucune modification du systÃ¨me observÃ© |
| **INV-CN-2** | Aucune capacitÃ© d'exÃ©cution | Le flux n'exÃ©cute aucune action corrective |
| **INV-CN-3** | Non-autoritaire | Le flux ne valide ni n'invalide rien |
| **INV-CN-4** | Ã‰tat cohÃ©rent | L'agrÃ©gation produit un Ã©tat unique et cohÃ©rent |
| **INV-CN-5** | TraÃ§abilitÃ© complÃ¨te | Chaque Ã©tape est enregistrÃ©e |
| **INV-CN-6** | Non-bloquant | Le flux ne bloque jamais |
| **INV-CN-7** | Propagation fidÃ¨le | Les transitions sont rapportÃ©es sans altÃ©ration |

---

## 11. ConformitÃ© aux Lois d'Autonomie

Ce contrat respecte les Lois d'Autonomie SystÃ¨me :

| Loi | ConformitÃ© | MÃ©canisme |
|-----|------------|-----------|
| **LOI-1** | âœ… Conforme | Observation locale, pas de dÃ©pendance externe |
| **LOI-2** | âœ… Conforme | Ã‰tat `offline` reconnu comme normal |
| **LOI-3** | âœ… Conforme | Historique local souverain |
| **LOI-4** | âœ… Conforme | Horodatage local, pas de temps global |
| **LOI-5** | âœ… Conforme | Flux lÃ©ger, ressources minimales |
| **LOI-6** | âœ… Conforme | Compatible avec fÃ©dÃ©ration sans modification |

**RÃ©fÃ©rence :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

## 12. RÃ©fÃ©rences croisÃ©es

- **Document source :** [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- **Architecture :** [Caring Nanny - Architecture et Composants](../../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)
- **Contrat complÃ©mentaire :** Caring Nanny - Propagation Flow Contract (flux de propagation)
- **Contrat complÃ©mentaire :** Caring Nanny - State Model Contract (modÃ¨le d'Ã©tat)
- **Invariants :** [Caring Nanny - Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)
- **Glossaire :** [Miyukini Conceptual References - Glossaire](..//..//..//..//miyukini-webway-system//reference//_index.md)
- **Lois d'Autonomie :** [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)
- **Connexion Inter-COG :** [Miyukini Conceptual References - Connexion Inter-COG](..//..//..//..//miyukini-webway-system//reference//_index.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif â€” Non nÃ©gociable  
**DÃ©rivÃ© de :** Caring Nanny - Documentation Fondatrice v1.6, Section 8  
**Type :** Contrat d'observabilitÃ©

