# Caring Nanny - KindMother Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre Caring Nanny et KindMother**. Il spécifie l'interface d'observation, le protocole, les types de données observées, et les garanties associées à l'observation de KindMother en tant qu'autorité des données.

Ce document complète la Section 3 de la [Documentation Fondatrice](../../foundation/Caring%20Nanny%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [Caring Nanny - Architecture et Composants](../../architecture/Caring%20Nanny%20-%20Architecture%20et%20Composants.md) pour l'architecture d'observation
- [Caring Nanny - Invariants et Garanties](../governance/Caring%20Nanny%20-%20Invariants%20et%20Garanties.md) pour les invariants d'observation
- [KindMother - Documentation Fondatrice](../../../KindMother/foundation/KindMother%20-%20Documentation%20Fondatrice.md) pour la nature de KindMother
- [Connexion Inter-COG](../../../../reference/Miyukini%20Conceptual%20References%20-%20Connexion%20Inter-COG.md) pour le contexte inter-composants

L'intégration respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : l'observation fonctionne localement sans dépendance externe (**LOI-1**), et l'état offline est reconnu comme normal (**LOI-2**).

## 2. Portée / Scope

Ce document couvre :
- L'interface d'observation entre Caring Nanny et KindMother
- Le protocole d'observation (unidirectionnel, passif)
- Les types de données observées depuis KindMother
- Les états dérivés de l'observation de KindMother
- Les règles d'intégration spécifiques
- La gestion des états et transitions
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de KindMother (voir documentation KindMother)
- Les flux de propagation vers BondingBrother (voir BondingBrother Integration Contract)
- Les décisions basées sur l'état observé (voir StrongFather Integration Contract)
- Le modèle d'état global (voir State Model Contract)

---

## 3. Principe fondamental

**Caring Nanny observe KindMother sans jamais interagir avec elle. La relation est strictement unidirectionnelle : KindMother produit des faits sur les données, Caring Nanny observe l'état de ces données. Caring Nanny ne modifie jamais, ne déclenche jamais, ne valide jamais.**

La relation est asymétrique : KindMother gère les données et leur persistance, Caring Nanny observe passivement les signaux d'état émis par KindMother sans jamais influencer son fonctionnement.

---

## 4. Nature de la relation Caring Nanny — KindMother

### 4.1 Relation d'observation pure

**Caring Nanny est un observateur passif de KindMother :**
- Elle observe les signaux d'état émis par KindMother
- Elle détecte les transitions d'état de la persistance
- Elle agrège les informations en état cohérent
- Elle ne sollicite jamais KindMother pour des opérations

**Règle CN-KM-01 : Observation sans interaction**

Caring Nanny ne produit jamais de demande vers KindMother. Elle observe les signaux émis, elle ne provoque pas d'émission.

**Règle CN-KM-02 : Aucune capacité d'écriture**

Caring Nanny ne peut jamais modifier les données gérées par KindMother. Aucun WriteIntent, aucune modification, aucun delta ne peut être émis par Caring Nanny.

**Règle CN-KM-03 : Aucune influence sur la synchronisation**

Caring Nanny ne peut jamais déclencher, suspendre, ou modifier une synchronisation entre DB Mère et DB Filles. Elle observe l'état de synchronisation, elle n'agit jamais sur lui.

### 4.2 Séparation des responsabilités

| Responsabilité | Caring Nanny | KindMother |
|----------------|--------------|------------|
| **Gérer les données** | ❌ Jamais | ✅ Exclusif |
| **Observer l'état des données** | ✅ Exclusif | ❌ |
| **Déclencher la synchronisation** | ❌ Jamais | ✅ Exclusif |
| **Observer l'état de synchronisation** | ✅ Exclusif | ❌ |
| **Valider les WriteIntent** | ❌ Jamais | ✅ Exclusif |
| **Détecter les anomalies de persistance** | ✅ Exclusif | ❌ |
| **Propager l'état observé** | ✅ Exclusif | ❌ |

**Règle CN-KM-04 : Aucun chevauchement d'autorité**

Caring Nanny n'a aucune autorité sur les données. KindMother n'a aucune responsabilité de propagation d'état. Les domaines sont strictement séparés.

---

## 5. Données observées depuis KindMother

### 5.1 État de santé de la persistance

**PERSISTENCE_HEALTH**
- **Objet d'observation :** Disponibilité et fonctionnement de la couche de persistance
- **Valeurs possibles :** `available`, `degraded`, `unavailable`
- **Signaux observés :** Temps de réponse, erreurs de lecture/écriture, intégrité des fichiers

**Règle CN-KM-OBS-01 : Observation non intrusive**

L'observation de la santé de la persistance n'interfère pas avec les opérations de KindMother. Caring Nanny observe les métriques exposées, elle ne provoque pas de requête de diagnostic.

### 5.2 État de synchronisation

**SYNC_STATUS**
- **Objet d'observation :** État de la synchronisation entre DB Mère et DB Filles
- **Valeurs possibles :** `synchronized`, `syncing`, `desynchronized`, `conflict`
- **Signaux observés :** Deltas en attente, état de connexion, conflits détectés

**Règle CN-KM-OBS-02 : État de synchronisation, pas action de synchronisation**

Caring Nanny observe si la synchronisation est en cours, réussie, ou en échec. Elle ne peut jamais initier, annuler, ou modifier une synchronisation.

### 5.3 État des instances

**INSTANCE_STATUS**
- **Objet d'observation :** Disponibilité et connectivité des instances DB
- **Données observées :**
  - DB Mère : accessible, inaccessible
  - DB Filles : connectées, déconnectées, nombre de filles actives
  - Latence de communication entre instances

**Règle CN-KM-OBS-03 : Observation globale des instances**

Caring Nanny observe l'état de toutes les instances connues. Elle agrège cette information en vue d'ensemble cohérente.

### 5.4 État des opérations en cours

**OPERATION_STATUS**
- **Objet d'observation :** WriteIntent en attente, deltas non propagés
- **Données observées :**
  - Nombre de WriteIntent en attente de validation
  - Nombre de deltas non propagés
  - Âge des opérations en attente
  - Opérations en échec ou en retry

**Règle CN-KM-OBS-04 : Observation quantitative, pas qualitative**

Caring Nanny observe le volume et l'état des opérations en cours. Elle ne connaît pas le contenu des WriteIntent ni des deltas.

### 5.5 Tableau récapitulatif des observations

| Catégorie | Données observées | Fréquence | Impact sur état système |
|-----------|-------------------|-----------|------------------------|
| **PERSISTENCE_HEALTH** | Disponibilité, temps de réponse, erreurs | Continue | `healthy` → `degraded` → `error` |
| **SYNC_STATUS** | État sync, deltas, conflits | Continue | `syncing`, `conflict` |
| **INSTANCE_STATUS** | DB Mère, DB Filles, latence | Continue | `offline` si DB Mère inaccessible |
| **OPERATION_STATUS** | WriteIntent, deltas, âge | Continue | `degraded` si accumulation |

---

## 6. États dérivés de l'observation de KindMother

### 6.1 Contribution aux catégories d'état système

L'observation de KindMother contribue directement aux catégories d'état système définies dans la Documentation Fondatrice :

**healthy**
- Persistance disponible et fonctionnelle
- Synchronisation à jour (si applicable)
- Toutes les instances connectées
- Aucune opération en échec

**degraded**
- Temps de réponse de la persistance élevé
- Deltas en attente depuis longtemps
- Certaines DB Filles déconnectées
- WriteIntent en retry

**offline**
- DB Mère inaccessible
- Mode offline actif sur la DB Fille locale
- Synchronisation impossible

**syncing**
- Synchronisation en cours entre instances
- Deltas en transfert
- État temporaire pendant la réconciliation

**error**
- Persistance indisponible
- Conflits de synchronisation non résolus
- Erreurs critiques sur la couche de stockage

### 6.2 Règles de dérivation d'état

**Règle CN-KM-STATE-01 : Priorité des états**

En cas de conditions multiples, l'état le plus critique prime :
`error` > `offline` > `syncing` > `degraded` > `healthy`

**Règle CN-KM-STATE-02 : État offline reconnu comme normal**

Conformément à LOI-2, l'état `offline` est un état normal, pas une erreur. La DB Fille fonctionne de manière autonome.

**Règle CN-KM-STATE-03 : Transition traçable**

Chaque transition d'état dérivée de l'observation de KindMother est traçable : cause, timestamp, état précédent, état nouveau.

---

## 7. Protocole d'observation

### 7.1 Modèle d'observation

L'observation suit un modèle **push passif** : KindMother émet des signaux d'état, Caring Nanny les reçoit et les traite.

**Caractéristiques :**
- Unidirectionnel : KindMother → Caring Nanny
- Passif : Caring Nanny reçoit, elle ne demande pas
- Non bloquant : L'observation n'interfère pas avec KindMother
- Continue : Observation permanente, pas ponctuelle

### 7.2 Format des signaux observés

**Structure conceptuelle d'un signal :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `signal_id` | Identifiant unique du signal | ✅ Oui |
| `source` | Origine du signal (kindmother) | ✅ Oui |
| `type` | Type de signal (health, sync, instance, operation) | ✅ Oui |
| `données` | Données spécifiques au type | ✅ Oui |
| `timestamp` | Horodatage du signal | ✅ Oui |
| `instance_id` | Instance concernée (si applicable) | ❌ Optionnel |

**Règle CN-KM-PROT-01 : Réception fidèle**

Le signal est reçu intégralement, sans modification ni interprétation initiale.

**Règle CN-KM-PROT-02 : Pas de filtrage à la source**

Caring Nanny reçoit tous les signaux de KindMother. Le filtrage éventuel est fait côté Caring Nanny, jamais côté KindMother.

### 7.3 Traitement des signaux

**Séquence de traitement :**

1. **Réception** — Le signal est reçu depuis KindMother
2. **Validation** — Le signal est validé (format, cohérence)
3. **Classification** — Le signal est classé par type
4. **Évaluation** — La condition observée est évaluée
5. **Agrégation** — L'état partiel est agrégé à l'état global
6. **Transition** — Si l'état global change, une transition est enregistrée
7. **Propagation** — Le changement d'état est propagé (via BondingBrother)

**Règle CN-KM-PROT-03 : Traitement séquentiel**

Les signaux sont traités dans l'ordre de réception. Aucun signal n'est sauté ou traité hors séquence.

**Règle CN-KM-PROT-04 : Pas d'effet de bord**

Le traitement d'un signal ne produit jamais d'effet de bord sur KindMother.

---

## 8. Flux d'observation

### 8.1 Flux d'observation de santé de persistance

**Déclencheur :** Signal de santé émis par KindMother

**Séquence :**
1. KindMother détecte un changement de santé (disponibilité, latence)
2. KindMother émet un signal `PERSISTENCE_HEALTH`
3. Caring Nanny reçoit le signal
4. Caring Nanny évalue la condition (healthy, degraded, unavailable)
5. Caring Nanny met à jour l'état partiel de la persistance
6. Si l'état global change, Caring Nanny enregistre la transition
7. Caring Nanny propage le changement d'état

### 8.2 Flux d'observation de synchronisation

**Déclencheur :** Changement d'état de synchronisation

**Séquence :**
1. KindMother démarre, progresse, ou termine une synchronisation
2. KindMother émet un signal `SYNC_STATUS`
3. Caring Nanny reçoit le signal
4. Caring Nanny évalue l'état (synchronized, syncing, desynchronized, conflict)
5. Caring Nanny met à jour l'état partiel de la synchronisation
6. Si l'état global change, Caring Nanny enregistre la transition
7. Caring Nanny propage le changement d'état

### 8.3 Flux d'observation d'instances

**Déclencheur :** Changement de connectivité d'une instance

**Séquence :**
1. Une instance DB (Mère ou Fille) change d'état de connexion
2. KindMother émet un signal `INSTANCE_STATUS`
3. Caring Nanny reçoit le signal
4. Caring Nanny met à jour la cartographie des instances
5. Caring Nanny évalue l'impact sur l'état global (notamment offline)
6. Si l'état global change, Caring Nanny enregistre la transition
7. Caring Nanny propage le changement d'état

### 8.4 Diagramme de séquence

```
┌─────────────────┐                      ┌─────────────────┐
│   KindMother    │                      │  Caring Nanny   │
└────────┬────────┘                      └────────┬────────┘
         │                                        │
         │                                        │
         ├── Signal (health/sync/instance) ─────►│
         │                                        │
         │                                        ├── Réception
         │                                        │
         │                                        ├── Validation
         │                                        │
         │                                        ├── Classification
         │                                        │
         │                                        ├── Évaluation
         │                                        │
         │                                        ├── Agrégation
         │                                        │
         │                                        ├── Transition?
         │                                        │
         │                                        ├── Propagation
         │                                        │   (vers BondingBrother)
         │                                        │
```

---

## 9. Règles d'intégration

### 9.1 Règles de communication

**Règle CN-KM-INT-01 : KindMother émet, Caring Nanny reçoit**

La direction de communication est toujours KindMother → Caring Nanny. Caring Nanny ne sollicite jamais KindMother.

**Règle CN-KM-INT-02 : Pas de callback vers KindMother**

Caring Nanny ne fournit jamais de callback ou de point d'entrée pour que KindMother l'interroge.

**Règle CN-KM-INT-03 : Observation continue**

L'observation est continue et permanente. Il n'y a pas de mode "observation désactivée".

### 9.2 Règles de données

**Règle CN-KM-INT-04 : Données d'état uniquement**

Caring Nanny observe uniquement les données d'état de KindMother, jamais les données métier (contenu, hiérarchie, etc.).

**Règle CN-KM-INT-05 : Pas d'accès à SQLite**

Caring Nanny n'a jamais accès à la couche SQLite interne de KindMother. L'abstraction de KindMother est respectée.

**Règle CN-KM-INT-06 : Pas de connaissance des WriteIntent**

Caring Nanny connaît le nombre et l'âge des WriteIntent en attente, mais jamais leur contenu.

### 9.3 Règles de traçabilité

**Règle CN-KM-INT-07 : Traçabilité des signaux**

Tous les signaux reçus de KindMother sont enregistrés dans l'historique de Caring Nanny.

**Règle CN-KM-INT-08 : Corrélation signal-transition**

Chaque transition d'état est corrélée au(x) signal(aux) qui l'a(ont) provoquée.

---

## 10. Gestion des états spéciaux

### 10.1 État offline

**Comportement :**
- Caring Nanny détecte l'inaccessibilité de la DB Mère
- L'état global passe à `offline`
- L'observation continue sur les signaux locaux (DB Fille)
- La transition est enregistrée avec la cause

**Règle CN-KM-OFFLINE-01 : Offline est un état normal**

Conformément à LOI-2, l'état `offline` n'est pas une erreur. C'est un état normal de fonctionnement autonome.

**Règle CN-KM-OFFLINE-02 : Observation locale maintenue**

En mode offline, Caring Nanny continue d'observer les signaux de la DB Fille locale.

### 10.2 État de conflit

**Comportement :**
- KindMother détecte un conflit de synchronisation
- Caring Nanny reçoit un signal `SYNC_STATUS` avec `conflict`
- L'état global inclut `conflict` dans son évaluation
- Caring Nanny propage l'information, mais ne résout pas le conflit

**Règle CN-KM-CONFLICT-01 : Observation du conflit, pas résolution**

Caring Nanny observe l'existence d'un conflit. La résolution appartient à KindMother ou au produit.

### 10.3 État d'erreur de persistance

**Comportement :**
- KindMother détecte une erreur critique de persistance
- Caring Nanny reçoit un signal `PERSISTENCE_HEALTH` avec `unavailable`
- L'état global passe à `error`
- Caring Nanny propage l'information immédiatement

**Règle CN-KM-ERROR-01 : Propagation immédiate des erreurs critiques**

Les erreurs critiques de persistance sont propagées immédiatement, sans délai d'agrégation.

---

## 11. Garanties de l'intégration

### 11.1 Garantie de passivité

**Engagement :** Caring Nanny n'a jamais d'effet sur KindMother. L'observation est strictement passive et unidirectionnelle.

### 11.2 Garantie de fidélité

**Engagement :** Les signaux observés sont traités fidèlement. Caring Nanny ne modifie pas, n'interprète pas subjectivement, ne filtre pas arbitrairement les signaux.

### 11.3 Garantie de complétude

**Engagement :** Tous les signaux émis par KindMother sont observés. Aucun signal n'est ignoré ou perdu.

### 11.4 Garantie de traçabilité

**Engagement :** Toute observation de KindMother est traçable de bout en bout. L'historique permet de reconstituer l'évolution de l'état observé.

### 11.5 Garantie de cohérence

**Engagement :** L'état dérivé de l'observation de KindMother est cohérent avec les autres sources d'observation. Pas de contradiction dans l'état global.

### 11.6 Garantie de non-blocage

**Engagement :** L'observation de KindMother ne bloque jamais le fonctionnement de KindMother ou du système. Conformité à INV-CN-6.

---

## 12. Invariants de l'intégration

### 12.1 Invariants de relation

**INV-CN-KM-1 : Observation unidirectionnelle**

KindMother émet des signaux. Caring Nanny reçoit et observe. La direction est toujours KindMother → Caring Nanny.

**INV-CN-KM-2 : Aucune capacité de modification**

Caring Nanny ne peut jamais modifier l'état ou les données de KindMother. Aucune exception.

**INV-CN-KM-3 : Respect de l'abstraction KindMother**

Caring Nanny n'accède jamais aux détails internes de KindMother (SQLite, schémas, etc.). Elle observe uniquement les signaux d'état exposés.

### 12.2 Invariants de données

**INV-CN-KM-4 : Observation d'état, pas de contenu**

Caring Nanny observe l'état des données (santé, synchronisation, disponibilité), jamais le contenu des données.

**INV-CN-KM-5 : Signaux complets**

Tous les signaux de KindMother sont reçus et traités. Aucun signal n'est filtré à la source.

### 12.3 Invariants de protocole

**INV-CN-KM-6 : Traitement séquentiel**

Les signaux sont traités dans l'ordre de réception. La séquence est préservée.

**INV-CN-KM-7 : Traçabilité complète**

Chaque signal reçu est enregistré dans l'historique avec son contexte complet.

---

## 13. Exemples

### 13.1 Observation de santé normale

**Signal KindMother :**
```
{
  "signal_id": "sig-km-001",
  "source": "kindmother",
  "type": "PERSISTENCE_HEALTH",
  "données": {
    "status": "available",
    "latency_ms": 5,
    "error_count": 0
  },
  "timestamp": "2026-01-27T14:00:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- Réception du signal
- Évaluation : persistance saine (latency < 50ms, error_count = 0)
- État partiel : `healthy`
- Pas de transition (état stable)
- Enregistrement dans l'historique

### 13.2 Détection de dégradation

**Signal KindMother :**
```
{
  "signal_id": "sig-km-002",
  "source": "kindmother",
  "type": "PERSISTENCE_HEALTH",
  "données": {
    "status": "available",
    "latency_ms": 250,
    "error_count": 3
  },
  "timestamp": "2026-01-27T14:05:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- Réception du signal
- Évaluation : latence élevée (> 100ms), erreurs présentes
- État partiel : `degraded`
- Transition : `healthy` → `degraded`
- Cause : "latence élevée (250ms), erreurs (3)"
- Propagation du changement d'état

### 13.3 Passage en mode offline

**Signal KindMother :**
```
{
  "signal_id": "sig-km-003",
  "source": "kindmother",
  "type": "INSTANCE_STATUS",
  "données": {
    "db_mere": {
      "status": "unreachable",
      "last_seen": "2026-01-27T13:55:00Z"
    },
    "db_fille_local": {
      "status": "active",
      "mode": "offline"
    }
  },
  "timestamp": "2026-01-27T14:10:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- Réception du signal
- Évaluation : DB Mère inaccessible, DB Fille en mode offline
- État partiel : `offline`
- Transition : `degraded` → `offline`
- Cause : "DB Mère inaccessible depuis 15 minutes"
- État `offline` reconnu comme normal (LOI-2)
- Propagation du changement d'état

### 13.4 Synchronisation en cours

**Signal KindMother :**
```
{
  "signal_id": "sig-km-004",
  "source": "kindmother",
  "type": "SYNC_STATUS",
  "données": {
    "status": "syncing",
    "deltas_pending": 42,
    "progress_percent": 65,
    "estimated_completion": "2026-01-27T14:20:00Z"
  },
  "timestamp": "2026-01-27T14:15:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- Réception du signal
- Évaluation : synchronisation en cours
- État partiel : `syncing`
- Transition : `offline` → `syncing`
- Cause : "Reconnexion DB Mère, synchronisation démarrée"
- Propagation du changement d'état

### 13.5 Détection de conflit

**Signal KindMother :**
```
{
  "signal_id": "sig-km-005",
  "source": "kindmother",
  "type": "SYNC_STATUS",
  "données": {
    "status": "conflict",
    "conflict_count": 3,
    "conflict_types": ["write_intent_collision", "version_mismatch"],
    "requires_resolution": true
  },
  "timestamp": "2026-01-27T14:18:00Z",
  "instance_id": "db-fille-001"
}
```

**Traitement Caring Nanny :**
- Réception du signal
- Évaluation : conflits de synchronisation détectés
- État partiel : `syncing` avec `conflict`
- Information propagée : conflits à résoudre
- Caring Nanny n'intervient pas dans la résolution
- Propagation de l'état incluant les informations de conflit

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que Caring Nanny doit respecter pour observer KindMother.

Toute implémentation de l'intégration avec KindMother doit respecter ce contrat. Toute violation (tentative de modification, d'interaction bidirectionnelle, d'accès aux données) constitue une rupture de contrat grave.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- Caring Nanny - Documentation Fondatrice v1.6 (Section 3)
- Caring Nanny - Architecture et Composants v1.0
- Caring Nanny - Invariants et Garanties v1.0
- KindMother - Documentation Fondatrice v1.2

---

## 15. Mini log de génération

### Décision éditoriale E1 : Nature de la relation

**Décision prise :** La relation est strictement unidirectionnelle d'observation : KindMother émet des signaux d'état, Caring Nanny les observe passivement. Cette approche diffère des contrats d'intégration BondingBrother (délégation bidirectionnelle) ou Master Butler (consultation).

**Application :** Tout le document est structuré autour de l'observation passive sans interaction.

### Décision éditoriale E2 : Types de données observées

**Décision prise :** Les données observées sont catégorisées en 4 types : santé de la persistance, état de synchronisation, état des instances, état des opérations. Ces catégories correspondent aux informations mentionnées dans la Documentation Fondatrice.

**Application :** Section 5 définit exhaustivement chaque type d'observation.

### Warning W1 : Risque de confusion observation/action

**Warning rencontré :** Risque que l'observation soit interprétée comme permettant une action corrective.

**Décision prise :** Renforcement explicite dans toutes les sections que Caring Nanny ne peut jamais agir sur KindMother. Règles CN-KM-01, CN-KM-02, CN-KM-03 établissent l'impossibilité d'action.

**Correction effectuée :** Ajout d'invariants INV-CN-KM-1, INV-CN-KM-2, INV-CN-KM-3 pour formaliser cette impossibilité.

### Warning W2 : État offline

**Warning rencontré :** Risque que l'état offline soit traité comme une erreur.

**Décision prise :** Conformément à LOI-2, l'état offline est explicitement reconnu comme un état normal. Règles CN-KM-OFFLINE-01 et CN-KM-STATE-02 clarifient ce point.

**Correction effectuée :** Section 10.1 dédiée à l'état offline.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Caring Nanny - Documentation Fondatrice : Confirmée (Section 3, relation avec KindMother)
- ✅ Cohérence avec KindMother - Documentation Fondatrice : Confirmée (relation d'observation mentionnée Section 7)
- ✅ Conformité LOI-1 : Confirmée (observation locale, aucune dépendance externe)
- ✅ Conformité LOI-2 : Confirmée (offline reconnu comme état normal)
- ✅ Conformité INV-CN-1 à INV-CN-7 : Confirmée (observateur pur, aucune modification, non-bloquant)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent avec l'écosystème documentaire existant.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
