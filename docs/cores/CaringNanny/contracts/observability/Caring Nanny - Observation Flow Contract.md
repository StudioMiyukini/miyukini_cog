# Caring Nanny - Observation Flow Contract

## 1. Contexte

Ce document définit le **contrat normatif du flux d'observation** de Caring Nanny. Le flux d'observation est le mécanisme fondamental par lequel Caring Nanny collecte, évalue, agrège et enregistre les états du système Miyukini.

Le flux d'observation est **strictement passif** : il ne modifie jamais l'état du système qu'il observe, conformément à l'invariant **INV-CN-1** (Observateur pur).

Ce contrat est **dérivé de la Documentation Fondatrice de Caring Nanny** (Section 8 - Interactions avec l'écosystème) et de l'**Architecture et Composants** (Section 5 - Flux de données internes).

**Documents sources :**
- [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- [Caring Nanny - Architecture et Composants](../../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)

---

## 2. Portée / Scope

- **Applicable à :** Toutes les opérations d'observation d'état dans Caring Nanny
- **Audience :** Architectes, développeurs, intégrateurs, autres cores de l'écosystème
- **Statut :** Contrat normatif — Non négociable
- **Dépendances :** Documentation Fondatrice Caring Nanny, Architecture et Composants, Glossaire Miyukini, Lois d'Autonomie Système

Ce document définit :
- Les quatre étapes du flux d'observation
- Les composants impliqués à chaque étape
- Les règles et contraintes de chaque étape
- Les garanties du flux d'observation
- Les conditions d'entrée et de sortie

Ce document **ne couvre pas** :
- Le flux de propagation (voir Caring Nanny - Propagation Flow Contract)
- Le flux de consultation (voir Caring Nanny - Consultation Contract)
- Les contrats d'intégration avec les autres cores (voir contracts/integration/)

---

## 3. Vue d'ensemble du flux d'observation

Le flux d'observation est composé de **quatre étapes séquentielles et obligatoires** :

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        FLUX D'OBSERVATION                                   │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ÉTAPE 1              ÉTAPE 2              ÉTAPE 3              ÉTAPE 4     │
│  DÉTECTION    ──►    ÉVALUATION    ──►   AGRÉGATION    ──►   TRANSITION    │
│                                                                             │
│  Composant           Condition            États partiels       État global  │
│  émet une      ──►   traduite en    ──►   agrégés en     ──►   comparé et   │
│  condition           état partiel         état système         enregistré   │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**Propriétés fondamentales du flux :**

| Propriété | Description |
|-----------|-------------|
| **Séquentiel** | Les étapes s'exécutent dans l'ordre, sans saut possible |
| **Déterministe** | Une même condition produit toujours le même résultat |
| **Non-bloquant** | Le flux n'interfère jamais avec les opérations du système |
| **Traçable** | Chaque étape produit des données auditables |
| **Passif** | Aucune modification de l'état observé |

---

## 4. Étape 1 : Détection de condition

### 4.1 Définition

La **détection de condition** est le mécanisme par lequel Caring Nanny collecte les faits observables depuis les composants du système. Une condition est un fait brut, avant toute interprétation en termes d'état.

### 4.2 Composants impliqués

```
Composant source (KindMother, StrongFather, Module SPM, ...)
         │
         │ Condition brute
         ▼
┌─────────────────┐
│ ComponentProbe  │ ← Sonde passive spécifique au type de composant
└────────┬────────┘
         │ Condition détectée
         ▼
┌─────────────────────┐
│ ConditionNormalizer │ ← Normalisation dans un format unifié
└────────┬────────────┘
         │ Condition normalisée
         ▼
┌─────────────────┐
│ TimestampMarker │ ← Horodatage local (conforme LOI-4)
└────────┬────────┘
         │ Condition horodatée
         ▼
┌──────────────────┐
│ConditionCollector│ ← Point de collecte centralisé
└──────────────────┘
```

### 4.3 Types de conditions détectées

| Source | Type de condition | Exemples |
|--------|-------------------|----------|
| **KindMother** | Santé de persistance | Disponible, dégradé, indisponible |
| **KindMother** | Synchronisation | Synchronisé, en cours, désynchronisé, conflits |
| **KindMother** | Instances | DB Mère accessible, DB Filles connectées |
| **KindMother** | Opérations | Écritures en attente, deltas non propagés |
| **StrongFather** | Politiques | Active, suspendue, en validation |
| **StrongFather** | Évaluations | En cours, succès, échec |
| **Modules SPM** | État module | Prêt, en chargement, erreur |
| **Réseau** | Connectivité | Disponible, indisponible, latente |
| **Système** | Ressources | Mémoire, CPU, stockage |

### 4.4 Règles de détection

| Règle | Énoncé | Référence |
|-------|--------|-----------|
| **RÈGLE-DET-1** | Toute détection est **passive** et sans effet de bord | INV-CN-1 |
| **RÈGLE-DET-2** | Une condition est **factuelle** (fait observé, pas interprétation) | Section 4, Doc Fondatrice |
| **RÈGLE-DET-3** | Chaque condition est **horodatée localement** | LOI-4 |
| **RÈGLE-DET-4** | La détection est **non-bloquante** pour le composant observé | INV-CN-6 |
| **RÈGLE-DET-5** | La sonde est **spécifique au type de composant** | Architecture 3.1 |
| **RÈGLE-DET-6** | Le format de condition est **normalisé** avant collecte | Architecture 3.1 |

### 4.5 Format de condition normalisée

```
Condition {
    source_id        : Identifiant unique du composant source
    source_type      : Type du composant (kindmother, strongfather, spm_module, ...)
    condition_type   : Type de condition (health, sync, operation, ...)
    condition_value  : Valeur brute de la condition
    timestamp_local  : Horodatage local (pas de temps global, conforme LOI-4)
    context          : Métadonnées contextuelles
}
```

### 4.6 Conditions d'entrée et de sortie

**Entrée :** Un composant du système émet un fait observable (changement de connexion, fin d'opération, erreur, etc.)

**Sortie :** Une condition normalisée et horodatée est transmise à l'étape d'évaluation

**Échec possible :** Si la condition ne peut pas être normalisée, elle est enregistrée comme anomalie et transmise telle quelle avec un marqueur d'erreur de normalisation.

---

## 5. Étape 2 : Évaluation de l'état

### 5.1 Définition

L'**évaluation de l'état** est le mécanisme par lequel Caring Nanny traduit une condition brute en état partiel classifié. Cette étape applique les règles de classification pour transformer un fait en catégorie d'état.

### 5.2 Composants impliqués

```
Condition normalisée (depuis ConditionCollector)
         │
         ▼
┌─────────────────┐
│ StateEvaluator  │ ← Évaluation condition → état partiel
└────────┬────────┘
         │ État partiel
         ▼
┌──────────────────┐
│CategoryClassifier│ ← Classification selon les 5 catégories
└──────────────────┘
         │
         ▼
     État classifié
```

### 5.3 Catégories d'état

Caring Nanny classifie chaque état dans l'une des **cinq catégories exclusives** définies dans la Documentation Fondatrice :

| Catégorie | Définition | Comportement attendu |
|-----------|------------|---------------------|
| **healthy** | Tous les composants fonctionnent normalement | Opérations normales |
| **degraded** | Certains composants en mode dégradé, système opérationnel | Fonctionnalités réduites |
| **offline** | Mode déconnecté, sans accès aux autorités centrales | Autonomie locale (LOI-2) |
| **syncing** | Synchronisation en cours, opérations potentiellement différées | État transitoire |
| **error** | Erreur critique détectée, certaines opérations impossibles | Investigation requise |

**Important (LOI-2) :** L'état `offline` est un état **normal**, pas une erreur. Il représente l'isolement accepté du système, conformément à la Loi d'Autonomie LOI-2.

### 5.4 Règles d'évaluation

| Règle | Énoncé | Référence |
|-------|--------|-----------|
| **RÈGLE-EVAL-1** | L'évaluation est **déterministe** : une condition donnée produit toujours le même état | Architecture 3.2 |
| **RÈGLE-EVAL-2** | L'évaluation est **reproductible** : le contexte est suffisant pour reproduire le résultat | Architecture 3.2 |
| **RÈGLE-EVAL-3** | Chaque évaluation produit **exactement une** catégorie d'état | INV-CN-4 |
| **RÈGLE-EVAL-4** | L'état `offline` n'est **jamais** évalué comme `error` (isolation ≠ anomalie) | LOI-2 |
| **RÈGLE-EVAL-5** | L'évaluation **n'interprète pas**, elle applique des règles définies | Architecture 3.2 |
| **RÈGLE-EVAL-6** | Les règles d'évaluation sont **fournies par le produit ou l'écosystème** | Section 6, Doc Fondatrice |

### 5.5 Matrice d'évaluation par type de condition

| Type de condition | Critère healthy | Critère degraded | Critère offline | Critère syncing | Critère error |
|-------------------|-----------------|------------------|-----------------|-----------------|---------------|
| **Santé persistance** | Disponible | Latence élevée | N/A | N/A | Indisponible |
| **Synchronisation** | Synchronisé | Conflits mineurs | Déconnecté | En cours | Échec répété |
| **Connectivité** | Disponible | Latente | Indisponible | Reconnexion | N/A |
| **Ressources** | Normales | Proches des limites | N/A | N/A | Épuisées |
| **Opérations** | Succès | Retry nécessaire | Différées | En attente | Échec critique |

### 5.6 Format d'état partiel

```
PartialState {
    source_id        : Identifiant du composant source
    source_type      : Type du composant
    category         : healthy | degraded | offline | syncing | error
    condition        : Condition source (référence)
    evaluation_rules : Règles appliquées pour cette évaluation
    timestamp        : Horodatage de l'évaluation
    confidence       : Niveau de confiance de l'évaluation (high, medium, low)
}
```

### 5.7 Conditions d'entrée et de sortie

**Entrée :** Une condition normalisée et horodatée

**Sortie :** Un état partiel classifié dans l'une des cinq catégories

**Échec possible :** Si aucune règle d'évaluation ne correspond, l'état est classifié comme `error` avec une note explicative.

---

## 6. Étape 3 : Agrégation

### 6.1 Définition

L'**agrégation** est le mécanisme par lequel Caring Nanny synthétise les états partiels de tous les composants en un état système global unique et cohérent.

### 6.2 Composants impliqués

```
États partiels (depuis CategoryClassifier)
         │
         │ Multiple états partiels
         ▼
┌─────────────────┐
│ StateAggregator │ ← Agrégation en état système global
└─────────────────┘
         │
         ▼
     État système global
```

### 6.3 Règles d'agrégation

L'agrégation suit des **règles de priorité** pour résoudre les situations où différents composants sont dans des états différents.

**Règle de priorité (du plus critique au moins critique) :**

```
error > syncing > offline > degraded > healthy
```

| Règle | Énoncé | Justification |
|-------|--------|---------------|
| **RÈGLE-AGG-1** | Si **au moins un** composant est en `error`, l'état système est `error` | La criticité prime |
| **RÈGLE-AGG-2** | Si aucun `error` mais **au moins un** `syncing`, l'état système est `syncing` | Synchronisation active |
| **RÈGLE-AGG-3** | Si aucun `error`/`syncing` mais **au moins un** `offline`, l'état système est `offline` | Isolation détectée |
| **RÈGLE-AGG-4** | Si aucun `error`/`syncing`/`offline` mais **au moins un** `degraded`, l'état système est `degraded` | Dégradation partielle |
| **RÈGLE-AGG-5** | Si **tous** les composants sont `healthy`, l'état système est `healthy` | Fonctionnement nominal |
| **RÈGLE-AGG-6** | L'agrégation est **déterministe** et **reproductible** | INV-CN-4 |

### 6.4 Résolution des contradictions

Les contradictions apparentes sont résolues par les règles de priorité. Une contradiction est un cas où l'interprétation naturelle des états est ambiguë.

**Exemples de résolution :**

| États partiels observés | État système résultant | Justification |
|------------------------|----------------------|---------------|
| healthy + healthy | healthy | Tous nominaux |
| healthy + degraded | degraded | Un composant dégradé affecte le système |
| healthy + offline | offline | Isolation détectée |
| degraded + syncing | syncing | Synchronisation prioritaire sur dégradation |
| error + healthy + healthy | error | Une erreur critique suffit |
| offline + offline | offline | Système isolé (état normal, LOI-2) |

### 6.5 Format d'état système global

```
SystemState {
    category             : healthy | degraded | offline | syncing | error
    partial_states       : Liste des états partiels contributifs
    contributing_sources : Liste des composants ayant contribué
    aggregation_rules    : Règles appliquées pour l'agrégation
    timestamp            : Horodatage de l'agrégation
    previous_state       : Référence à l'état système précédent
}
```

### 6.6 Conditions d'entrée et de sortie

**Entrée :** Un ou plusieurs états partiels classifiés

**Sortie :** Un état système global unique et cohérent

**Échec possible :** Aucun échec possible — l'agrégation produit toujours un résultat valide grâce aux règles de priorité.

---

## 7. Étape 4 : Détection de transition

### 7.1 Définition

La **détection de transition** est le mécanisme par lequel Caring Nanny identifie et enregistre les changements d'état système. Une transition est le passage d'un état à un autre.

### 7.2 Composants impliqués

```
État système global (depuis StateAggregator)
         │
         ▼
┌──────────────────┐
│TransitionDetector│ ← Comparaison avec l'état précédent
└────────┬─────────┘
         │
    ┌────┴────┐
    ▼         ▼
Historique  Propagation
(HistoryStore)  (si transition détectée)
```

### 7.3 Caractéristiques d'une transition

| Propriété | Description | Référence |
|-----------|-------------|-----------|
| **Déterministe** | Un état donné ne peut conduire qu'à un ensemble fini d'états possibles | Section 4, Doc Fondatrice |
| **Observable** | La transition elle-même est un fait observable | Section 4, Doc Fondatrice |
| **Traçable** | Chaque transition est enregistrée avec son contexte | INV-CN-5 |
| **Causale** | Une transition a toujours une cause identifiable | Section 4, Doc Fondatrice |

### 7.4 Règles de détection de transition

| Règle | Énoncé | Référence |
|-------|--------|-----------|
| **RÈGLE-TRANS-1** | Une transition est détectée si et seulement si `état_actuel ≠ état_précédent` | Définition de transition |
| **RÈGLE-TRANS-2** | Chaque transition est **enregistrée** avec l'état précédent, l'état actuel, et la cause | INV-CN-5 |
| **RÈGLE-TRANS-3** | La cause est la **condition qui a déclenché** l'évaluation menant à la transition | Traçabilité |
| **RÈGLE-TRANS-4** | Si aucune transition n'est détectée, l'état est tout de même mis à jour dans l'historique (avec marqueur "unchanged") | Auditabilité |
| **RÈGLE-TRANS-5** | Une transition déclenche **optionnellement** une propagation (voir Propagation Flow Contract) | Architecture 5.2 |

### 7.5 Transitions valides

Le graphe suivant définit les transitions valides entre états système :

```
                    ┌──────────────────────────────────────┐
                    │                                      │
                    ▼                                      │
        ┌────────────────┐                                 │
        │    healthy     │◄────────────────────────────────┤
        └───────┬────────┘                                 │
                │                                          │
    ┌───────────┼───────────────┐                          │
    │           │               │                          │
    ▼           ▼               ▼                          │
┌────────┐  ┌────────┐    ┌─────────┐                      │
│degraded│  │offline │    │ syncing │──────────────────────┤
└───┬────┘  └───┬────┘    └────┬────┘                      │
    │           │              │                           │
    │           │              │                           │
    ▼           ▼              ▼                           │
    └───────────┴──────────────┴───────────────────────────┤
                │                                          │
                ▼                                          │
         ┌───────────┐                                     │
         │   error   │─────────────────────────────────────┘
         └───────────┘
           (peut revenir à tout état après résolution)
```

**Transitions depuis healthy :**
- healthy → degraded (dégradation partielle)
- healthy → offline (perte de connexion)
- healthy → syncing (début de synchronisation)
- healthy → error (erreur critique détectée)

**Transitions depuis degraded :**
- degraded → healthy (récupération)
- degraded → offline (perte de connexion en mode dégradé)
- degraded → syncing (début de synchronisation)
- degraded → error (aggravation)

**Transitions depuis offline :**
- offline → healthy (reconnexion réussie)
- offline → syncing (début de synchronisation après reconnexion)
- offline → degraded (reconnexion partielle)
- offline → error (erreur en mode isolé)

**Transitions depuis syncing :**
- syncing → healthy (synchronisation réussie)
- syncing → degraded (synchronisation partielle)
- syncing → offline (perte de connexion pendant sync)
- syncing → error (échec de synchronisation)

**Transitions depuis error :**
- error → healthy (résolution complète)
- error → degraded (résolution partielle)
- error → offline (isolation après erreur)
- error → syncing (tentative de récupération par sync)

### 7.6 Format de transition

```
Transition {
    id                 : Identifiant unique de la transition
    previous_state     : État système avant la transition
    current_state      : État système après la transition
    trigger_condition  : Condition qui a déclenché la transition
    trigger_source     : Composant source de la condition déclencheuse
    timestamp          : Horodatage de la détection de transition
    propagation_needed : Boolean (indique si propagation requise)
    metadata           : Contexte additionnel
}
```

### 7.7 Conditions d'entrée et de sortie

**Entrée :** Un état système global et l'état système précédent

**Sortie :** 
- Si transition : un enregistrement de transition dans l'historique + notification de propagation
- Si pas de transition : un enregistrement "unchanged" dans l'historique

**Échec possible :** Aucun — la comparaison d'états est toujours possible.

---

## 8. Garanties du flux d'observation

Le flux d'observation garantit les propriétés suivantes, dérivées des invariants de la Documentation Fondatrice :

### 8.1 Garantie de passivité (INV-CN-1)

> Le flux d'observation ne modifie **jamais** l'état du système observé.

**Vérification :** À aucune étape du flux, une écriture ou modification n'est effectuée sur les composants observés.

### 8.2 Garantie de cohérence (INV-CN-4)

> L'état système rapporté est **toujours cohérent** — aucune contradiction interne.

**Vérification :** L'agrégation déterministe garantit qu'un seul état est produit, sans ambiguïté.

### 8.3 Garantie de traçabilité (INV-CN-5)

> Chaque observation, évaluation, agrégation et transition est **entièrement traçable**.

**Vérification :** Chaque étape produit des données structurées enregistrées dans l'historique.

### 8.4 Garantie de non-blocage (INV-CN-6)

> Le flux d'observation ne bloque **jamais** les opérations du système.

**Vérification :** Toutes les opérations sont asynchrones et non-bloquantes.

### 8.5 Garantie d'autonomie (LOI-1 à LOI-5)

> Le flux d'observation fonctionne **localement**, sans dépendance externe.

| Loi | Conformité | Mécanisme |
|-----|------------|-----------|
| **LOI-1** | ✅ | Observation locale, pas d'appel externe obligatoire |
| **LOI-2** | ✅ | L'état `offline` est reconnu comme état normal |
| **LOI-3** | ✅ | L'historique local est souverain |
| **LOI-4** | ✅ | Horodatage local, pas de temps global requis |
| **LOI-5** | ✅ | Flux léger, consommation minimale de ressources |

---

## 9. Anomalies et cas limites

### 9.1 Condition non normalisable

**Situation :** Une condition brute ne peut pas être normalisée par le ConditionNormalizer.

**Comportement :** La condition est marquée comme `anomaly:normalization_failure` et transmise avec cette annotation. L'évaluation classifie l'état comme `error` avec mention de l'anomalie.

### 9.2 Règle d'évaluation absente

**Situation :** Aucune règle d'évaluation ne correspond à la condition.

**Comportement :** L'état est classifié comme `error` avec mention `evaluation_rule_missing`. Un signal d'alerte est émis pour configuration manquante.

### 9.3 Composant non observable

**Situation :** Un composant configuré pour observation ne répond pas à la sonde.

**Comportement :** L'état partiel du composant est classifié comme `error` avec mention `probe_timeout`. L'agrégation inclut cet état dans le calcul.

### 9.4 Historique saturé

**Situation :** Le HistoryStore atteint sa capacité maximale.

**Comportement :** Les observations les plus anciennes sont archivées selon la politique de rétention. Le flux continue sans interruption.

---

## 10. Invariants applicables au flux

Ce contrat est gouverné par les invariants suivants :

| Invariant | Énoncé | Application au flux |
|-----------|--------|---------------------|
| **INV-CN-1** | Observateur pur | Aucune modification du système observé |
| **INV-CN-2** | Aucune capacité d'exécution | Le flux n'exécute aucune action corrective |
| **INV-CN-3** | Non-autoritaire | Le flux ne valide ni n'invalide rien |
| **INV-CN-4** | État cohérent | L'agrégation produit un état unique et cohérent |
| **INV-CN-5** | Traçabilité complète | Chaque étape est enregistrée |
| **INV-CN-6** | Non-bloquant | Le flux ne bloque jamais |
| **INV-CN-7** | Propagation fidèle | Les transitions sont rapportées sans altération |

---

## 11. Conformité aux Lois d'Autonomie

Ce contrat respecte les Lois d'Autonomie Système :

| Loi | Conformité | Mécanisme |
|-----|------------|-----------|
| **LOI-1** | ✅ Conforme | Observation locale, pas de dépendance externe |
| **LOI-2** | ✅ Conforme | État `offline` reconnu comme normal |
| **LOI-3** | ✅ Conforme | Historique local souverain |
| **LOI-4** | ✅ Conforme | Horodatage local, pas de temps global |
| **LOI-5** | ✅ Conforme | Flux léger, ressources minimales |
| **LOI-6** | ✅ Conforme | Compatible avec fédération sans modification |

**Référence :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

---

## 12. Références croisées

- **Document source :** [Caring Nanny - Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md)
- **Architecture :** [Caring Nanny - Architecture et Composants](../../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md)
- **Contrat complémentaire :** Caring Nanny - Propagation Flow Contract (flux de propagation)
- **Contrat complémentaire :** Caring Nanny - State Model Contract (modèle d'état)
- **Invariants :** [Caring Nanny - Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md)
- **Glossaire :** [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- **Lois d'Autonomie :** [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
- **Connexion Inter-COG :** [Miyukini Conceptual References - Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md)

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** Contrat normatif — Non négociable  
**Dérivé de :** Caring Nanny - Documentation Fondatrice v1.6, Section 8  
**Type :** Contrat d'observabilité
