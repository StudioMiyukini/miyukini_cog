# LogisticsSteward - MasterButler Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre LogisticsSteward et MasterButler**. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec MasterButler en tant que registre des capacités et permissions.

Ce document complète la Section 8.3 de la [Documentation Fondatrice](../../foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [LogisticsSteward - Architecture & Flows](../../architecture/LogisticsSteward%20-%20Architecture%20&%20Flows.md) pour les flux d'arbitrage
- [LogisticsSteward - Quota Definition Contract](../resources/LogisticsSteward%20-%20Quota%20Definition%20Contract.md) pour la définition des quotas
- [Master Butler - Documentation Fondatrice](../../../MasterButler/foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) pour la nature de MasterButler
- [Master Butler - Capability API Contract](../../../MasterButler/contracts/api/Master%20Butler%20-%20Capability%20API%20Contract.md) pour l'API des capacités

L'intégration respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : toutes les interactions sont locales et ne requièrent aucune dépendance externe (**LOI-1**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre LogisticsSteward et MasterButler
- Le protocole de communication (interrogations et limitations)
- Les types d'interactions LogisticsSteward → MasterButler
- Les règles d'intégration spécifiques
- La gestion des erreurs et des réponses
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de MasterButler (voir documentation MasterButler)
- Les détails des registres de capacités (voir Capability Registry Contract)
- Les stratégies de dégradation (voir Degradation Strategy Contract)
- L'intégration avec StrongFather (voir StrongFather Integration Contract)

---

## 3. Principe fondamental

**MasterButler expose les capacités existantes. LogisticsSteward limite leur usage sans jamais modifier leur existence. MasterButler dit ce qui est possible, LogisticsSteward dit ce qui est autorisé en termes de ressources.**

La relation est de **consultation et de limitation** :
- LogisticsSteward interroge MasterButler pour connaître les capacités disponibles
- LogisticsSteward applique des limitations d'usage sur ces capacités
- MasterButler reflète ces limitations sans les interpréter

La séparation est absolue : **l'existence d'une capacité est du ressort de MasterButler, la limitation de son usage est du ressort de LogisticsSteward**.

---

## 4. Nature de la relation LogisticsSteward — MasterButler

### 4.1 Relation de consultation et limitation

**LogisticsSteward consulte MasterButler pour :**
- Connaître les capacités existantes pour appliquer des quotas
- Identifier les entités consommatrices de ressources
- Obtenir les métadonnées des capacités pour le calcul d'arbitrage
- Découvrir les Tools et Toolkits pour la limitation d'usage

**LogisticsSteward limite les capacités exposées par MasterButler :**
- Applique des quotas d'utilisation
- Définit des priorités d'accès aux capacités
- Impose des plafonds de consommation
- Active des restrictions temporaires

**Règle LS-MB-01 : Limitation sans modification d'existence**

LogisticsSteward ne peut jamais modifier l'existence d'une capacité dans MasterButler. Il peut uniquement limiter son usage. Une capacité déclarée dans MasterButler reste déclarée, même si son usage est entièrement restreint.

**Règle LS-MB-02 : Exhaustivité de la connaissance**

LogisticsSteward a accès à l'intégralité des capacités déclarées dans MasterButler. Aucune capacité n'est masquée ou filtrée lors des interrogations.

**Règle LS-MB-03 : Indépendance des registres**

Le registre des capacités (MasterButler) et le registre des limitations (LogisticsSteward) sont strictement séparés. Aucun chevauchement n'est autorisé.

### 4.2 Séparation des responsabilités

| Responsabilité | LogisticsSteward | MasterButler |
|----------------|------------------|--------------|
| **Déclarer les capacités** | ❌ Jamais | ✅ Exclusif |
| **Connaître les capacités** | ❌ Interroge | ✅ Exclusif |
| **Limiter l'usage** | ✅ Exclusif | ❌ Jamais |
| **Définir les quotas** | ✅ Exclusif | ❌ Jamais |
| **Attribuer les priorités** | ✅ Exclusif | ❌ Jamais |
| **Appliquer les restrictions** | ✅ Exclusif | ❌ Jamais |
| **Supprimer des capacités** | ❌ Jamais | ✅ Exclusif |
| **Exposer les permissions** | ❌ Jamais | ✅ Exclusif |

**Règle LS-MB-04 : Aucun chevauchement**

Aucun chevauchement de responsabilités n'est autorisé. LogisticsSteward ne déclare jamais de capacités, MasterButler ne limite jamais l'usage.

---

## 5. Types d'interactions

### 5.1 Interrogation des capacités existantes

**CAPABILITY_CATALOG**
- **Objectif :** Obtenir la liste des capacités pour appliquer des limitations
- **Payload :** Filtres optionnels (module, type, niveau)
- **Réponse :** Liste des capacités avec leurs métadonnées

**Règle LS-MB-QUERY-01 : Catalogue complet**

LogisticsSteward peut interroger le catalogue complet des capacités pour établir ses règles de gouvernance.

### 5.2 Interrogation des entités consommatrices

**CONSUMER_ENTITIES**
- **Objectif :** Identifier les entités qui utilisent une capacité
- **Payload :** Identifiant de la capacité
- **Réponse :** Liste des entités consommatrices (Opérateurs, Équipes, Services)

**Règle LS-MB-QUERY-02 : Traçabilité des consommateurs**

La liste des consommateurs permet à LogisticsSteward de calculer les quotas et priorités par entité.

### 5.3 Interrogation des Tools et Toolkits

**TOOL_METADATA**
- **Objectif :** Obtenir les métadonnées d'un Tool pour le calcul de quota
- **Payload :** Identifiant du Tool
- **Réponse :** Métadonnées incluant coût estimé, fréquence d'appel, ressources requises

**TOOLKIT_COMPOSITION**
- **Objectif :** Obtenir la composition d'un Toolkit
- **Payload :** Identifiant du Toolkit
- **Réponse :** Liste des Tools avec leurs caractéristiques de consommation

**Règle LS-MB-QUERY-03 : Métadonnées de consommation**

MasterButler expose les métadonnées de consommation des Tools (coût, fréquence, ressources) pour permettre à LogisticsSteward de calculer les limitations.

### 5.4 Notification des limitations

**USAGE_LIMITATION**
- **Objectif :** Informer MasterButler d'une limitation d'usage
- **Payload :** Capacité concernée, type de limitation, paramètres
- **Réponse :** Acquittement

**Règle LS-MB-NOTIF-01 : Notification informative**

Les notifications de limitation sont informatives. MasterButler les enregistre mais ne les applique pas lui-même. L'application est du ressort du Kernel via les décisions de LogisticsSteward.

### 5.5 Notification de restauration

**USAGE_RESTORATION**
- **Objectif :** Informer MasterButler de la levée d'une limitation
- **Payload :** Capacité concernée, limitation levée
- **Réponse :** Acquittement

**Règle LS-MB-NOTIF-02 : Restauration explicite**

Toute levée de limitation fait l'objet d'une notification explicite pour maintenir la cohérence des états.

### 5.6 Règles générales d'interaction

**Règle LS-MB-QUERY-04 : Interrogation sans effet de bord**

Les interrogations de LogisticsSteward ne modifient jamais l'état de MasterButler. Ce sont des lectures pures.

**Règle LS-MB-QUERY-05 : Notification avec acquittement**

Les notifications de limitation ou restauration attendent un acquittement de MasterButler pour garantir la prise en compte.

**Règle LS-MB-QUERY-06 : Réponse immédiate**

Les réponses sont fournies immédiatement. Aucune interrogation n'est mise en attente ou différée.

---

## 6. Protocole de communication

### 6.1 Format des interrogations

Les interrogations de LogisticsSteward suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `interrogation_id` | Identifiant unique de l'interrogation | ✅ Oui |
| `arbitrage_id` | Référence à l'arbitrage en cours | ✅ Oui |
| `type` | Type d'interrogation | ✅ Oui |
| `payload` | Données spécifiques à l'interrogation | ✅ Oui |
| `contexte_appelant` | Contexte de LogisticsSteward | ✅ Oui |
| `timestamp` | Horodatage de l'interrogation | ✅ Oui |

**Règle LS-MB-PROT-01 : Format standardisé**

Toutes les interrogations respectent le format standardisé. Aucune interrogation ad-hoc n'est acceptée.

**Règle LS-MB-PROT-02 : Traçabilité par arbitrage**

Chaque interrogation référence l'arbitrage en cours pour assurer la traçabilité bout-en-bout.

### 6.2 Format des notifications

Les notifications de LogisticsSteward suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `notification_id` | Identifiant unique de la notification | ✅ Oui |
| `decision_id` | Référence à la décision d'arbitrage | ✅ Oui |
| `type` | Type de notification (LIMITATION, RESTORATION) | ✅ Oui |
| `payload` | Données spécifiques à la notification | ✅ Oui |
| `contexte_appelant` | Contexte de LogisticsSteward | ✅ Oui |
| `timestamp` | Horodatage de la notification | ✅ Oui |

**Règle LS-MB-PROT-03 : Notification structurée**

Toutes les notifications respectent le format structuré. MasterButler peut les enregistrer pour audit.

### 6.3 Format des réponses

Les réponses de MasterButler suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `reponse_id` | Identifiant unique de la réponse | ✅ Oui |
| `interrogation_id` | Référence à l'interrogation | ✅ Oui |
| `statut` | Statut de la réponse (SUCCESS, NOT_FOUND, ERROR) | ✅ Oui |
| `donnees` | Données de la réponse | Si SUCCESS |
| `erreur` | Détails de l'erreur | Si ERROR |
| `timestamp` | Horodatage de la réponse | ✅ Oui |

**Règle LS-MB-PROT-04 : Réponse toujours structurée**

MasterButler retourne toujours une réponse structurée, même en cas d'erreur ou de non-existence.

### 6.4 Statuts de réponse

| Statut | Signification |
|--------|---------------|
| `SUCCESS` | L'interrogation/notification a abouti |
| `NOT_FOUND` | L'élément recherché n'existe pas dans le registre |
| `INVALID_QUERY` | L'interrogation est mal formée ou incomplète |
| `ACKNOWLEDGED` | La notification a été prise en compte |
| `ERROR` | Une erreur interne s'est produite |

**Règle LS-MB-PROT-05 : NOT_FOUND est informatif**

Le statut `NOT_FOUND` indique qu'une capacité n'existe pas. LogisticsSteward ne peut pas limiter une capacité inexistante.

---

## 7. Flux d'interaction typique

### 7.1 Flux de calcul de quota

**Acteurs :** LogisticsSteward, MasterButler, Kernel

**Séquence :**

1. LogisticsSteward reçoit une demande de ressource (via Kernel)
2. LogisticsSteward interroge MasterButler : `CAPABILITY_CATALOG` (si nécessaire)
3. MasterButler répond avec les capacités concernées
4. LogisticsSteward interroge MasterButler : `TOOL_METADATA`
5. MasterButler répond avec les métadonnées de consommation
6. LogisticsSteward calcule les quotas applicables
7. LogisticsSteward soumet la décision à StrongFather
8. Si validée, LogisticsSteward notifie MasterButler : `USAGE_LIMITATION` (si limitation)
9. MasterButler acquitte la notification

**Règle LS-MB-FLOW-01 : Interrogation avant limitation**

LogisticsSteward interroge toujours MasterButler avant d'appliquer une limitation pour s'assurer que la capacité existe.

### 7.2 Flux de dégradation

**Acteurs :** LogisticsSteward, MasterButler, WorrySentinel

**Séquence :**

1. WorrySentinel signale une situation de stress (charge élevée)
2. LogisticsSteward évalue le niveau de dégradation requis
3. LogisticsSteward interroge MasterButler : `CAPABILITY_CATALOG` (capacités non critiques)
4. MasterButler répond avec les capacités et leurs niveaux de criticité
5. LogisticsSteward calcule les limitations de dégradation
6. LogisticsSteward notifie MasterButler : `USAGE_LIMITATION` (par capacité)
7. MasterButler acquitte les notifications

**Règle LS-MB-FLOW-02 : Dégradation par criticité**

Les capacités sont limitées par ordre de criticité inverse : les moins critiques d'abord.

### 7.3 Flux de restauration

**Acteurs :** LogisticsSteward, MasterButler, Kernel

**Séquence :**

1. Kernel signale un retour à la normale (charge réduite)
2. LogisticsSteward évalue les limitations à lever
3. LogisticsSteward notifie MasterButler : `USAGE_RESTORATION` (par capacité)
4. MasterButler acquitte les notifications
5. Les capacités reprennent leur usage normal

**Règle LS-MB-FLOW-03 : Restauration progressive**

La restauration est progressive, par paliers, en fonction de l'état système.

### 7.4 Diagramme de séquence

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│     Kernel      │    │ LogisticsSteward│    │  Master Butler  │
└────────┬────────┘    └────────┬────────┘    └────────┬────────┘
         │                      │                      │
         ├── Demande ressource ►│                      │
         │                      │                      │
         │                      ├── CAPABILITY_CATALOG►│
         │                      │                      │
         │                      │◄── Capacités ────────┤
         │                      │                      │
         │                      ├── TOOL_METADATA ────►│
         │                      │                      │
         │                      │◄── Métadonnées ──────┤
         │                      │                      │
         │                      ├── Calcul quota ──────┤
         │                      │   (interne)          │
         │                      │                      │
         │                      ├── USAGE_LIMITATION ─►│
         │                      │                      │
         │                      │◄── ACKNOWLEDGED ─────┤
         │                      │                      │
         │◄── Décision ─────────┤                      │
         │                      │                      │
```

---

## 8. Règles d'intégration

### 8.1 Règles de communication

**Règle LS-MB-INT-01 : LogisticsSteward initie**

LogisticsSteward initie les interrogations et notifications. MasterButler ne sollicite jamais LogisticsSteward spontanément pour des questions de limitation.

**Règle LS-MB-INT-02 : Notification après validation**

Les notifications de limitation sont émises uniquement après validation par StrongFather. Aucune limitation n'est notifiée avant validation.

**Règle LS-MB-INT-03 : Synchronisme des interrogations**

Les interrogations sont synchrones. LogisticsSteward attend la réponse avant de poursuivre l'arbitrage.

### 8.2 Règles de données

**Règle LS-MB-INT-04 : Données fraîches**

Les données retournées par MasterButler reflètent l'état actuel du registre au moment de l'interrogation.

**Règle LS-MB-INT-05 : Cache autorisé pour métadonnées statiques**

LogisticsSteward peut mettre en cache les métadonnées statiques des capacités (coût, description) mais pas les données dynamiques (consommateurs actuels).

**Règle LS-MB-INT-06 : Cohérence des limitations**

Les limitations notifiées par LogisticsSteward sont cohérentes avec les capacités déclarées dans MasterButler.

### 8.3 Règles de traçabilité

**Règle LS-MB-INT-07 : Traçabilité des interrogations**

Toutes les interrogations de LogisticsSteward sont tracées par les deux parties.

**Règle LS-MB-INT-08 : Corrélation arbitrage-interrogation**

Chaque interrogation est corrélée à l'arbitrage en cours pour permettre l'audit bout-en-bout.

**Règle LS-MB-INT-09 : Historique des limitations**

MasterButler maintient un historique des notifications de limitation reçues pour audit.

---

## 9. Gestion des erreurs

### 9.1 Types d'erreurs

**Erreurs de format :**
- Interrogation mal formée
- Champ obligatoire manquant
- Type d'interrogation inconnu

**Erreurs de données :**
- Capacité inexistante (NOT_FOUND)
- Tool inexistant (NOT_FOUND)
- Limitation sur capacité inexistante

**Erreurs internes :**
- Erreur de registre MasterButler
- Erreur de calcul de limitation

### 9.2 Traitement des erreurs

**Règle LS-MB-ERR-01 : Réponse structurée toujours**

MasterButler retourne toujours une réponse structurée, même en cas d'erreur. LogisticsSteward peut toujours interpréter la réponse.

**Règle LS-MB-ERR-02 : NOT_FOUND bloque la limitation**

Si une capacité n'existe pas (NOT_FOUND), LogisticsSteward ne peut pas la limiter. L'arbitrage échoue pour cette capacité.

**Règle LS-MB-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisées par les deux parties pour audit et diagnostic.

**Règle LS-MB-ERR-04 : Pas de retry automatique**

En cas d'erreur, LogisticsSteward décide de la stratégie (retry, échec de l'arbitrage). Aucun retry automatique.

### 9.3 Cas de capacité supprimée

**Règle LS-MB-ERR-05 : Limitation orpheline**

Si une capacité est supprimée de MasterButler alors qu'une limitation existe, LogisticsSteward doit être notifié pour nettoyer sa limitation orpheline.

**Règle LS-MB-ERR-06 : Notification de suppression**

MasterButler notifie LogisticsSteward lors de la suppression d'une capacité pour permettre le nettoyage des limitations associées.

---

## 10. Garanties de l'intégration

### 10.1 Garantie de visibilité

**Engagement :** LogisticsSteward a une visibilité complète sur les capacités déclarées dans MasterButler. Aucune capacité n'est masquée.

### 10.2 Garantie de non-interférence

**Engagement :** LogisticsSteward n'interfère jamais avec l'existence des capacités. Les limitations concernent uniquement l'usage.

### 10.3 Garantie de cohérence

**Engagement :** Les limitations notifiées sont cohérentes avec les capacités existantes. Aucune limitation orpheline n'est créée intentionnellement.

### 10.4 Garantie de traçabilité

**Engagement :** Toute interaction entre LogisticsSteward et MasterButler est traçable de bout en bout. L'audit complet est possible.

### 10.5 Garantie de disponibilité

**Engagement :** MasterButler est disponible pour répondre aux interrogations de LogisticsSteward sans dépendance externe (conformité LOI-1).

### 10.6 Garantie de restauration

**Engagement :** Toute limitation peut être levée. Le système peut toujours revenir à un état sans limitation.

---

## 11. Invariants de l'intégration

### 11.1 Invariants de relation

**INV-LS-MB-1 : Séparation existence/usage**

L'existence des capacités (MasterButler) et la limitation de leur usage (LogisticsSteward) sont strictement séparées.

**INV-LS-MB-2 : Limitation sur capacité existante**

LogisticsSteward ne peut limiter que des capacités existantes dans MasterButler.

**INV-LS-MB-3 : Non-modification d'existence**

LogisticsSteward ne peut jamais créer, modifier, ou supprimer une capacité dans MasterButler.

### 11.2 Invariants de données

**INV-LS-MB-4 : Lecture pure**

Les interrogations sont des lectures pures. Aucune modification du registre n'est causée par une interrogation.

**INV-LS-MB-5 : Notification avec acquittement**

Toute notification de limitation ou restauration attend un acquittement avant d'être considérée comme appliquée.

### 11.3 Invariants de protocole

**INV-LS-MB-6 : Format respecté**

Toutes les interrogations, notifications, et réponses respectent le format standardisé.

**INV-LS-MB-7 : Traçabilité complète**

Toute interaction est traçable avec son contexte complet.

**INV-LS-MB-8 : Validation préalable**

Toute limitation notifiée a été préalablement validée par StrongFather.

---

## 12. Exemples

### 12.1 Interrogation du catalogue de capacités

**Interrogation LogisticsSteward :**
```
{
  "interrogation_id": "int-ls-001",
  "arbitrage_id": "arb-100",
  "type": "CAPABILITY_CATALOG",
  "payload": {
    "module_filter": "miyukini-spm-cms",
    "include_metadata": true
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "reason": "quota_calculation"
  },
  "timestamp": "2026-01-28T10:00:00Z"
}
```

**Réponse MasterButler :**
```
{
  "reponse_id": "resp-mb-001",
  "interrogation_id": "int-ls-001",
  "statut": "SUCCESS",
  "donnees": {
    "capabilities": [
      {
        "id": "content.create",
        "name": "Create Content",
        "module": "miyukini-spm-cms-content",
        "criticality": "standard",
        "estimated_cost": "medium"
      },
      {
        "id": "content.publish",
        "name": "Publish Content",
        "module": "miyukini-spm-cms-content",
        "criticality": "high",
        "estimated_cost": "high"
      }
    ],
    "total_count": 2
  },
  "timestamp": "2026-01-28T10:00:01Z"
}
```

### 12.2 Notification de limitation d'usage

**Notification LogisticsSteward :**
```
{
  "notification_id": "notif-ls-001",
  "decision_id": "dec-arb-100",
  "type": "USAGE_LIMITATION",
  "payload": {
    "capability_id": "content.publish",
    "limitation_type": "quota",
    "parameters": {
      "max_calls_per_hour": 10,
      "affected_entities": ["operator-cms-001"],
      "reason": "high_system_load"
    },
    "expiration": "2026-01-28T12:00:00Z"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "strongfather_validation": "val-sf-050"
  },
  "timestamp": "2026-01-28T10:05:00Z"
}
```

**Réponse MasterButler :**
```
{
  "reponse_id": "resp-mb-002",
  "notification_id": "notif-ls-001",
  "statut": "ACKNOWLEDGED",
  "donnees": {
    "limitation_registered": true,
    "affected_capability": "content.publish",
    "registration_id": "lim-mb-001"
  },
  "timestamp": "2026-01-28T10:05:01Z"
}
```

### 12.3 Notification de restauration

**Notification LogisticsSteward :**
```
{
  "notification_id": "notif-ls-002",
  "decision_id": "dec-arb-101",
  "type": "USAGE_RESTORATION",
  "payload": {
    "capability_id": "content.publish",
    "limitation_id": "lim-mb-001",
    "reason": "system_load_normalized"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "strongfather_validation": "val-sf-051"
  },
  "timestamp": "2026-01-28T12:30:00Z"
}
```

**Réponse MasterButler :**
```
{
  "reponse_id": "resp-mb-003",
  "notification_id": "notif-ls-002",
  "statut": "ACKNOWLEDGED",
  "donnees": {
    "limitation_removed": true,
    "capability_id": "content.publish",
    "full_access_restored": true
  },
  "timestamp": "2026-01-28T12:30:01Z"
}
```

### 12.4 Capacité inexistante

**Interrogation LogisticsSteward :**
```
{
  "interrogation_id": "int-ls-002",
  "arbitrage_id": "arb-102",
  "type": "TOOL_METADATA",
  "payload": {
    "tool_id": "nonexistent.tool"
  },
  "contexte_appelant": {
    "source": "logisticssteward",
    "reason": "quota_calculation"
  },
  "timestamp": "2026-01-28T11:00:00Z"
}
```

**Réponse MasterButler :**
```
{
  "reponse_id": "resp-mb-004",
  "interrogation_id": "int-ls-002",
  "statut": "NOT_FOUND",
  "donnees": {
    "exists": false,
    "tool_id": "nonexistent.tool"
  },
  "timestamp": "2026-01-28T11:00:01Z"
}
```

**Note :** LogisticsSteward ne peut pas créer de limitation sur ce Tool inexistant.

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que LogisticsSteward doit respecter pour s'intégrer avec MasterButler.

Toute implémentation de l'intégration avec MasterButler doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- LogisticsSteward - Documentation Fondatrice v1.0.0 (Section 8.3)
- LogisticsSteward - Quota Definition Contract
- Master Butler - Documentation Fondatrice
- Master Butler - Capability API Contract

---

## 14. Mini log de génération

### Décision éditoriale E1 : Direction de la relation

**Décision prise :** La relation est bidirectionnelle asymétrique : LogisticsSteward interroge et notifie, MasterButler répond et acquitte. LogisticsSteward est l'initiateur, MasterButler est le répondant.

**Application :** Le document est structuré autour de cette direction d'interaction.

### Décision éditoriale E2 : Types d'interactions

**Décision prise :** Les interactions sont divisées en interrogations (lecture) et notifications (écriture informative). Les interrogations portent sur les capacités, les notifications portent sur les limitations.

**Application :** Section 5 définit chaque type avec objectif, payload, et réponse.

### Warning W1 : Limitation sur capacité inexistante

**Warning rencontré :** Risque de créer des limitations orphelines sur des capacités qui n'existent pas ou plus.

**Décision prise :** LogisticsSteward doit interroger l'existence avant de limiter. Les limitations orphelines sont nettoyées via notification de suppression.

**Correction effectuée :** Règles LS-MB-ERR-02, LS-MB-ERR-05, LS-MB-ERR-06 ajoutées.

### Warning W2 : Séparation existence/usage

**Warning rencontré :** Risque de confusion entre "capacité inexistante" et "capacité limitée à zéro".

**Décision prise :** La distinction est explicite : une capacité peut être totalement limitée mais existe toujours. LogisticsSteward ne peut pas supprimer une capacité.

**Correction effectuée :** INV-LS-MB-1 et INV-LS-MB-3 clarifient cette séparation.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec LogisticsSteward - Documentation Fondatrice : Confirmée (Section 8.3 respectée)
- ✅ Cohérence avec Master Butler - Documentation Fondatrice : Confirmée (registre des capacités respecté)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe pour les interactions)
- ✅ Conformité INV-LS-7 : Confirmée (séparation avec le Kernel maintenue)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
