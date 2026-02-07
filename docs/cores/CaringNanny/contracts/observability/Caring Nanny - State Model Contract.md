# Caring Nanny - State Model Contract

## 1. Contexte

Ce document définit le **modèle formel des états** observés et rapportés par Caring Nanny. Il établit les catégories d'états canoniques, leurs caractéristiques, leurs conditions d'entrée/sortie, et les règles qui gouvernent leur usage dans le Miyukini Core System.

Ce contrat étend la Section 4 de la [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) en formalisant le modèle d'état comme spécification normative.

## 2. Portée / Scope

Ce document couvre :
- Les catégories d'état système (healthy, degraded, offline, syncing, error)
- Les catégories d'état applicatif (états partiels des composants)
- Les propriétés formelles de chaque état
- Les conditions d'entrée et de sortie de chaque état
- Les règles d'agrégation des états partiels en état global
- Les transitions valides entre états
- Les états d'isolement conformes à LOI-2

Ce document **ne couvre pas** :
- Les flux d'observation (voir Observation Flow Contract)
- Les flux de propagation (voir Propagation Flow Contract)
- Les détails d'implémentation
- Les mécanismes de détection (voir Architecture et Composants)

---

## 3. Définitions

### 3.1 État

Un **état** est une condition observable et classifiable d'un composant ou du système à un instant donné. Un état est :
- **Catégorisé** : appartient à une catégorie canonique définie
- **Horodaté** : associé à un instant de temps local (via le kernel Clock)
- **Contextualisé** : accompagné d'informations de contexte
- **Non-interprétable** : Caring Nanny ne donne pas d'opinion sur l'état, elle le rapporte

### 3.2 État système

L'**état système** est la condition globale du Miyukini Core System à un instant donné. C'est une synthèse agrégée de tous les états partiels des composants.

**Propriétés :**
- Unique : un seul état système à un instant T
- Cohérent : sans contradiction interne
- Observable : accessible par interrogation
- Instantané : valide à un moment précis

### 3.3 État applicatif

L'**état applicatif** est la condition d'un module ou composant spécifique au sein du système. C'est un état partiel qui contribue à l'état système global.

**Propriétés :**
- Partiel : concerne un composant spécifique
- Contributif : participe à l'agrégation de l'état système
- Autonome : peut être observé indépendamment
- Spécialisé : sémantique propre au composant

### 3.4 Transition

Une **transition** est le passage d'un état à un autre. Elle est :
- **Causale** : provoquée par une ou plusieurs conditions
- **Instantanée** : se produit à un moment précis
- **Traçable** : enregistrée avec son contexte
- **Validable** : conforme aux règles de transition

---

## 4. Catégories d'état système

Caring Nanny reconnaît exactement **cinq catégories d'état système**. Ces catégories sont mutuellement exclusives : à tout instant, le système est dans exactement une de ces catégories.

### 4.1 État : healthy

**Définition :** Tous les composants fonctionnent normalement, aucune anomalie n'est détectée, toutes les fonctionnalités sont disponibles.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| Code | `healthy` |
| Sévérité | 0 (normale) |
| Opérations permises | Toutes |
| Notifications | Aucune (état nominal) |
| Durée typique | Indéfinie (état cible) |

**Conditions d'entrée :**
- Tous les composants critiques rapportent un état nominal
- Aucune anomalie active
- Aucune synchronisation en cours
- Connexion disponible (si mode connecté)

**Conditions de sortie :**
- Détection d'une anomalie → `degraded` ou `error`
- Perte de connexion → `offline`
- Démarrage de synchronisation → `syncing`

**Conformité LOI-1 :** L'état `healthy` est atteignable sans dépendance externe. Un système isolé peut être `healthy` s'il fonctionne correctement en mode autonome.

---

### 4.2 État : degraded

**Définition :** Certains composants fonctionnent en mode dégradé, le système reste opérationnel mais avec des fonctionnalités réduites ou des performances diminuées.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| Code | `degraded` |
| Sévérité | 1 (avertissement) |
| Opérations permises | Opérations essentielles |
| Notifications | Changement d'état |
| Durée typique | Variable |

**Conditions d'entrée :**
- Un ou plusieurs composants non-critiques dysfonctionnent
- Performance dégradée (latence, débit)
- Ressources limitées (mémoire, CPU)
- Certaines fonctionnalités indisponibles

**Conditions de sortie :**
- Résolution de toutes les dégradations → `healthy`
- Aggravation critique → `error`
- Perte de connexion (si connecté) → `offline`

**Sous-catégories informatives (non-canoniques) :**
- `degraded:performance` : dégradation de performance
- `degraded:feature` : fonctionnalité indisponible
- `degraded:resource` : ressources limitées

**Conformité LOI-2 :** L'état `degraded` est un état normal, pas une erreur. Le système fonctionne avec ce qu'il a disponible, conformément à LOI-2 (le système accepte l'isolement comme état normal).

---

### 4.3 État : offline

**Définition :** Le système fonctionne en mode déconnecté, sans accès aux autorités centrales (DB Mère, nœuds fédérés). C'est un **état normal**, pas une erreur.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| Code | `offline` |
| Sévérité | 0 (normale) |
| Opérations permises | Opérations locales |
| Notifications | Transition d'état uniquement |
| Durée typique | Variable (état normal) |

**Conditions d'entrée :**
- Perte de connexion réseau
- Indisponibilité de la DB Mère
- Décision explicite de fonctionnement isolé
- Démarrage sans connexion disponible

**Conditions de sortie :**
- Rétablissement de la connexion → `syncing` (puis `healthy`)
- Détection d'anomalie locale → `degraded` ou `error`

**Distinctions critiques :**

| Aspect | offline (normal) | error (problème) |
|--------|------------------|------------------|
| Nature | État souhaité ou accepté | Condition anormale |
| Fonctionnement | Complet en mode local | Limité ou bloqué |
| Réaction | Aucune correction requise | Diagnostic/correction |
| Durée | Indéfinie acceptable | À résoudre |

**Conformité LOI-2 :** L'état `offline` implémente directement LOI-2 (le système accepte l'isolement comme état normal). L'isolement n'est pas une erreur, c'est un mode de fonctionnement valide et explicitement reconnu.

**Conformité LOI-3 :** En état `offline`, l'état local est souverain. Les décisions prises sont valables localement, les données locales constituent la vérité locale.

---

### 4.4 État : syncing

**Définition :** Une synchronisation est en cours entre la source locale et une source distante (DB Mère, nœud fédéré). Certaines opérations peuvent être différées ou contraintes.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| Code | `syncing` |
| Sévérité | 0 (normale) |
| Opérations permises | Lectures, écritures locales |
| Notifications | Progression, conflits éventuels |
| Durée typique | Transitoire |

**Conditions d'entrée :**
- Reconnexion après mode offline
- Réconciliation programmée
- Réception de deltas à traiter
- Demande explicite de synchronisation

**Conditions de sortie :**
- Synchronisation terminée avec succès → `healthy`
- Synchronisation terminée avec résidus → `degraded`
- Perte de connexion pendant sync → `offline`
- Erreur critique de synchronisation → `error`

**Sous-états informatifs (non-canoniques) :**
- `syncing:receiving` : réception de deltas
- `syncing:applying` : application des changements
- `syncing:reconciling` : résolution de conflits
- `syncing:sending` : envoi de deltas locaux

**Conformité LOI-4 :** La synchronisation ne dépend pas d'un temps global. Les comparaisons utilisent des horloges logiques ou des points de synchronisation, conformément à LOI-4 (pas de temps global requis).

---

### 4.5 État : error

**Définition :** Une erreur critique a été détectée. Certaines opérations ne sont pas possibles. Le système nécessite une attention ou une intervention.

**Caractéristiques :**

| Propriété | Valeur |
|-----------|--------|
| Code | `error` |
| Sévérité | 2 (critique) |
| Opérations permises | Limitées (diagnostic, lecture) |
| Notifications | Alerte, détails d'erreur |
| Durée typique | À résoudre |

**Conditions d'entrée :**
- Échec d'un composant critique
- Corruption de données détectée
- Incohérence non résolvable
- Erreur système critique

**Conditions de sortie :**
- Résolution de l'erreur → `healthy` ou `degraded`
- Redémarrage → état initial selon contexte

**Sous-catégories informatives (non-canoniques) :**
- `error:critical` : composant critique défaillant
- `error:data` : problème de données
- `error:system` : erreur système
- `error:unrecoverable` : erreur non récupérable

**Distinction avec offline :**

L'état `error` représente un **problème** à résoudre, tandis que `offline` représente un **mode de fonctionnement** valide. Cette distinction est fondamentale pour la conformité LOI-2.

---

## 5. Catégories d'état applicatif

Les états applicatifs sont les états partiels des composants individuels. Ils contribuent à l'état système global par agrégation.

### 5.1 États KindMother

| État | Description | Contribution à l'état système |
|------|-------------|------------------------------|
| `km:available` | Persistance disponible | → healthy |
| `km:degraded` | Performance réduite | → degraded |
| `km:syncing` | Synchronisation en cours | → syncing |
| `km:offline` | Mode local uniquement | → offline |
| `km:error` | Erreur de persistance | → error |

### 5.2 États StrongFather

| État | Description | Contribution à l'état système |
|------|-------------|------------------------------|
| `sf:ready` | Moteur de décision prêt | → healthy |
| `sf:degraded` | Certaines politiques non disponibles | → degraded |
| `sf:error` | Erreur du moteur de décision | → error |

### 5.3 États BondingBrother

| État | Description | Contribution à l'état système |
|------|-------------|------------------------------|
| `bb:available` | Médiation disponible | → healthy |
| `bb:degraded` | Canaux partiellement disponibles | → degraded |
| `bb:offline` | Médiation locale uniquement | → offline |
| `bb:error` | Erreur de médiation | → error |

### 5.4 États Module SPM

| État | Description | Contribution à l'état système |
|------|-------------|------------------------------|
| `mod:ready` | Module opérationnel | → healthy |
| `mod:loading` | Module en chargement | → syncing |
| `mod:degraded` | Fonctionnalités réduites | → degraded |
| `mod:unavailable` | Module non disponible | → degraded |
| `mod:error` | Erreur de module | → error |

---

## 6. Règles d'agrégation

L'état système global est déterminé par l'agrégation des états applicatifs selon des règles de priorité définies.

### 6.1 Règle de priorité

Caring Nanny applique la règle de **priorité par sévérité maximale** :

```
État système = max(sévérité(états applicatifs))
```

**Ordre de priorité (du plus prioritaire au moins prioritaire) :**
1. `error` (sévérité 2) : si un composant critique est en erreur
2. `syncing` (sévérité 0, mais transitoire prioritaire)
3. `degraded` (sévérité 1)
4. `offline` (sévérité 0, mode)
5. `healthy` (sévérité 0, nominal)

### 6.2 Règles d'agrégation spécifiques

| Condition | État système résultant |
|-----------|------------------------|
| Au moins un composant critique en `error` | `error` |
| Synchronisation en cours | `syncing` |
| Au moins un composant en `degraded`, aucun `error` | `degraded` |
| Tous les composants `offline` ou mode déconnecté | `offline` |
| Tous les composants `healthy` | `healthy` |

### 6.3 Composants critiques vs non-critiques

La distinction entre composants **critiques** et **non-critiques** influence l'agrégation :

**Composants critiques :**
- KindMother (persistance)
- StrongFather (décisions)

**Composants non-critiques :**
- Modules SPM individuels
- Canaux de médiation optionnels

**Règle :** Une erreur sur un composant critique entraîne `error` système. Une erreur sur un composant non-critique entraîne `degraded` (sauf si bloquante).

### 6.4 Résolution des contradictions

En cas de contradiction apparente, Caring Nanny applique :

1. **Cohérence temporelle** : l'observation la plus récente prévaut
2. **Cohérence de sévérité** : la sévérité maximale prévaut
3. **Cohérence de source** : les composants critiques prévalent

**Exemple :**
- KindMother rapporte `km:available`
- Module A rapporte `mod:error`
- Résultat : `degraded` (module non-critique en erreur)

---

## 7. Matrice de transitions valides

### 7.1 Transitions entre états système

| État source | États cibles valides | Transitions directes interdites |
|-------------|---------------------|--------------------------------|
| `healthy` | `degraded`, `offline`, `syncing`, `error` | — |
| `degraded` | `healthy`, `offline`, `error` | `syncing` sans passer par `healthy` |
| `offline` | `syncing`, `degraded`, `error` | `healthy` sans passer par `syncing` |
| `syncing` | `healthy`, `degraded`, `offline`, `error` | — |
| `error` | `healthy`, `degraded`, `offline` | `syncing` (correction requise d'abord) |

### 7.2 Diagramme de transitions

```
                    ┌─────────────────────────────────────────┐
                    │                                         │
                    ▼                                         │
    ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐   │
    │ offline │◄──│ healthy │──►│degraded │──►│  error  │───┘
    └────┬────┘   └────┬────┘   └────┬────┘   └────┬────┘
         │             │             │             │
         │             │             │             │
         │        ┌────▼────┐        │             │
         └───────►│ syncing │◄───────┘             │
                  └────┬────┘                      │
                       │                           │
                       └───────────────────────────┘
```

### 7.3 Conditions de transition

| Transition | Condition de déclenchement |
|------------|---------------------------|
| `healthy` → `degraded` | Détection de dégradation non-critique |
| `healthy` → `offline` | Perte de connexion |
| `healthy` → `syncing` | Démarrage de synchronisation |
| `healthy` → `error` | Erreur critique détectée |
| `degraded` → `healthy` | Résolution de toutes les dégradations |
| `degraded` → `error` | Aggravation critique |
| `degraded` → `offline` | Perte de connexion |
| `offline` → `syncing` | Rétablissement de connexion |
| `offline` → `degraded` | Anomalie locale détectée |
| `offline` → `error` | Erreur critique locale |
| `syncing` → `healthy` | Synchronisation réussie |
| `syncing` → `degraded` | Synchronisation avec résidus |
| `syncing` → `offline` | Perte de connexion pendant sync |
| `syncing` → `error` | Erreur de synchronisation |
| `error` → `healthy` | Résolution complète |
| `error` → `degraded` | Résolution partielle |
| `error` → `offline` | Passage en mode isolé après erreur |

---

## 8. Propriétés formelles des états

### 8.1 Propriété d'exclusivité mutuelle

**PF-SM-01 :** À tout instant T, le système est dans exactement un état :

```
∀T : |{s ∈ {healthy, degraded, offline, syncing, error} : état(T) = s}| = 1
```

### 8.2 Propriété de complétude

**PF-SM-02 :** Toute condition observable peut être classifiée dans une catégorie d'état :

```
∀c ∈ Conditions : ∃s ∈ États : classifie(c) = s
```

### 8.3 Propriété de déterminisme

**PF-SM-03 :** L'agrégation des états partiels produit toujours le même état global :

```
∀(ep₁, ep₂, ..., epₙ) : agrège(ep₁, ep₂, ..., epₙ) = état_unique
```

### 8.4 Propriété de transition valide

**PF-SM-04 :** Toute transition respecte la matrice de transitions valides :

```
∀(s₁, s₂) : transition(s₁, s₂) ⟹ (s₁, s₂) ∈ TransitionsValides
```

### 8.5 Propriété de traçabilité

**PF-SM-05 :** Toute transition est associée à une cause identifiable :

```
∀transition(s₁, s₂) : ∃cause : provoquée_par(transition, cause)
```

---

## 9. États d'isolement (conformité LOI-2)

Ce contrat implémente explicitement la conformité à **LOI-2** (le système accepte l'isolement comme état normal).

### 9.1 Reconnaissance des états d'isolement

Caring Nanny reconnaît les états d'isolement suivants comme **états normaux** :

| État d'isolement | Code Caring Nanny | Nature |
|------------------|-------------------|--------|
| Connecté | `healthy` ou autre selon état | État nominal |
| Isolé | `offline` | **État normal** |
| Partiellement synchronisé | `syncing` | État transitoire |
| Dégradé | `degraded` | État normal |
| Fédéré | `healthy` avec flag fédération | État nominal |

### 9.2 Distinction isolé vs erreur

**Règle fondamentale (conformité LOI-2) :**

> L'isolement (`offline`) n'est **jamais** classifié comme erreur (`error`).

Cette distinction est non-négociable :

| Situation | État correct | État INTERDIT |
|-----------|--------------|---------------|
| Pas de connexion réseau | `offline` | `error` |
| DB Mère injoignable | `offline` | `error` |
| Démarrage sans réseau | `offline` | `error` |
| Fonctionnement volontaire isolé | `offline` | `error` |

### 9.3 Critères de distinction

Pour classifier une situation :

| Critère | → offline | → error |
|---------|-----------|---------|
| Fonctionnement local possible | ✓ | — |
| Fonctionnement local impossible | — | ✓ |
| Absence de connexion | ✓ | — |
| Composant critique défaillant | — | ✓ |
| Mode choisi explicitement | ✓ | — |
| Condition anormale | — | ✓ |

---

## 10. Conformité aux Lois d'Autonomie

Ce contrat garantit la conformité aux Lois d'Autonomie définies dans [Miyukini Conceptual References - Lois Autonomie Systeme.md](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md).

### LOI-1 : Aucune dépendance externe critique

**Conformité :** ✅ Le modèle d'état fonctionne localement. La classification des états ne nécessite aucun appel externe.

### LOI-2 : Le système accepte l'isolement comme état normal

**Conformité :** ✅ L'état `offline` est explicitement défini comme un état normal (sévérité 0), distinct de l'état `error`.

### LOI-3 : L'état local est souverain

**Conformité :** ✅ Les états sont déterminés à partir de conditions locales. L'état local est la source de vérité pour Caring Nanny.

### LOI-4 : Pas de temps global requis

**Conformité :** ✅ Les horodatages sont locaux (kernel Clock). Aucune comparaison inter-nœuds basée sur un temps global.

### LOI-5 : Le coût doit être proportionnel au hardware

**Conformité :** ✅ Le modèle d'état est léger (5 catégories, règles simples). Pas de structure complexe en mémoire.

### LOI-6 : L'autonomie n'empêche pas la fédération

**Conformité :** ✅ Les états `syncing` et la transition `offline` → `syncing` supportent la fédération optionnelle.

---

## 11. Correspondance avec la Documentation Fondatrice

| Section Fondatrice | Couverture dans ce contrat |
|-------------------|---------------------------|
| §4 État système | Section 4 (Catégories d'état système) |
| §4 État applicatif | Section 5 (Catégories d'état applicatif) |
| §4 Transition d'état | Section 7 (Matrice de transitions) |
| §4 Condition | Section 3.1 (Définitions) |
| §10 Conformité LOI-2 | Section 9 (États d'isolement) |

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit le modèle formel des états qui doit être respecté par toute implémentation de Caring Nanny.

Les catégories d'état, les règles d'agrégation, et les transitions valides sont **non-négociables**. Toute modification nécessite une nouvelle version majeure du contrat.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT — Modèle d'état normatif  
**Dépendances :**  
- [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) v1.6 (Section 4)
- [Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) v1.0
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) v1.1
