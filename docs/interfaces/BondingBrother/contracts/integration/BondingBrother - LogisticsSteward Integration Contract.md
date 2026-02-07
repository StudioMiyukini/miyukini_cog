# BondingBrother - LogisticsSteward Integration Contract

## 1. Contexte

Ce document définit le contrat d'intégration entre Bonding Brother et LogisticsSteward. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées au transport des décisions d'arbitrage de ressources émises par LogisticsSteward.

Ce document complète la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md) pour les règles de traduction, et la [LogisticsSteward - Documentation Fondatrice](../../../LogisticsSteward/foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md) pour les spécifications du core.

L'intégration respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : les décisions d'arbitrage sont transportées localement et buffées en mode offline (**LOI-2**, **LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre Bonding Brother et LogisticsSteward
- Le protocole de communication (décisions et notifications)
- Les types de décisions d'arbitrage transportées
- Les règles de transport spécifiques à LogisticsSteward
- La gestion des erreurs de transport
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de LogisticsSteward (voir documentation LogisticsSteward)
- Les règles de traduction générales (voir [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md))
- Le mode offline détaillé (voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md))
- La logique d'arbitrage (voir [LogisticsSteward - Resource Arbitration Contract](../../../LogisticsSteward/contracts/resources/LogisticsSteward%20-%20Resource%20Arbitration%20Contract.md))

---

## 3. Principe fondamental

**Bonding Brother transporte les décisions d'arbitrage de LogisticsSteward vers les entités concernées, sans jamais les interpréter, les modifier, ou les évaluer. LogisticsSteward décide, BondingBrother transmet fidèlement.**

La relation est de transport : Bonding Brother reçoit les décisions d'arbitrage validées par StrongFather, les traduit dans le format des destinataires, et les transmet avec traçabilité complète. Bonding Brother ne prend aucune décision d'arbitrage.

**Différence avec KindMother/StrongFather :**
- KindMother et StrongFather sont des **autorités** vers lesquelles Bonding Brother délègue des décisions
- LogisticsSteward est un **émetteur de décisions** dont Bonding Brother transporte les résultats
- Le flux est principalement sortant (LogisticsSteward → entités via BondingBrother)

---

## 4. Positionnement de LogisticsSteward

### 4.1 Gouverneur des ressources

**LogisticsSteward est le gouverneur de l'allocation, de la priorisation et de la limitation des ressources.**

Il répond à la question : "Qui a le droit d'utiliser quoi, quand, et à quel niveau de priorité ?"

**Règle LS-POS-01 : Arbitrage sans exécution**

LogisticsSteward décide de l'arbitrage, le Kernel l'exécute. Bonding Brother transporte les décisions vers les entités concernées.

**Règle LS-POS-02 : Décisions validées par StrongFather**

Toute décision d'arbitrage de LogisticsSteward est validée par StrongFather avant d'être transportée par Bonding Brother.

**Règle LS-POS-03 : Transport fidèle**

Bonding Brother transporte les décisions d'arbitrage sans interprétation, modification, ou évaluation. Il est un canal de transmission, pas un décideur.

### 4.2 Relation dans la pyramide

```
LogisticsSteward (Strate 3 - Gouvernance Ressources)
       │
       ▼ décisions d'arbitrage
[StrongFather] (validation)
       │
       ▼ décisions validées
[BondingBrother] (transport)
       │
       ▼ notifications
[Entités destinataires : Opérateurs, Services, MasterButler, MiyukiniAdmin]
```

---

## 5. Types de décisions transportées

### 5.1 Décisions de quota

**QUOTA_ASSIGNED**
- **Source :** LogisticsSteward
- **Transport :** Notification vers l'entité concernée
- **Payload :** Identifiant entité, type de quota, valeur attribuée, durée
- **Destinataires :** Opérateurs, Services, MasterButler

**QUOTA_MODIFIED**
- **Source :** LogisticsSteward
- **Transport :** Notification de modification
- **Payload :** Identifiant entité, ancien quota, nouveau quota, raison
- **Destinataires :** Opérateurs, Services concernés

**QUOTA_EXCEEDED**
- **Source :** LogisticsSteward
- **Transport :** Alerte vers l'entité et WorrySentinel
- **Payload :** Identifiant entité, quota dépassé, niveau de dépassement
- **Destinataires :** Entité concernée, WorrySentinel

### 5.2 Décisions de priorité

**PRIORITY_ASSIGNED**
- **Source :** LogisticsSteward
- **Transport :** Notification vers l'entité concernée
- **Payload :** Identifiant entité, niveau de priorité, contexte
- **Destinataires :** Opérateurs, Services

**PRIORITY_CHANGED**
- **Source :** LogisticsSteward
- **Transport :** Notification de changement
- **Payload :** Identifiant entité, ancienne priorité, nouvelle priorité, raison
- **Destinataires :** Entité concernée

**PREEMPTION_NOTIFIED**
- **Source :** LogisticsSteward
- **Transport :** Notification de préemption
- **Payload :** Identifiant entité préemptée, identifiant entité prioritaire, ressource concernée
- **Destinataires :** Entité préemptée, WorrySentinel

### 5.3 Décisions de dégradation

**DEGRADATION_INITIATED**
- **Source :** LogisticsSteward
- **Transport :** Notification vers toutes les entités concernées
- **Payload :** Niveau de dégradation (D0 à D4), entités affectées, restrictions actives
- **Destinataires :** Toutes les entités du périmètre

**DEGRADATION_ESCALATED**
- **Source :** LogisticsSteward
- **Transport :** Notification d'escalade
- **Payload :** Ancien niveau, nouveau niveau, raison, nouvelles restrictions
- **Destinataires :** Toutes les entités du périmètre, WorrySentinel

**DEGRADATION_LIFTED**
- **Source :** LogisticsSteward
- **Transport :** Notification de retour à la normale
- **Payload :** Niveau actuel, restrictions levées, capacités restaurées
- **Destinataires :** Toutes les entités du périmètre

### 5.4 Décisions de restriction

**RESOURCE_RESTRICTED**
- **Source :** LogisticsSteward
- **Transport :** Notification de restriction
- **Payload :** Type de ressource, entités affectées, niveau de restriction, durée
- **Destinataires :** Entités concernées

**RESOURCE_UNRESTRICTED**
- **Source :** LogisticsSteward
- **Transport :** Notification de levée de restriction
- **Payload :** Type de ressource, entités libérées, nouvelles limites
- **Destinataires :** Entités concernées

### 5.5 Règles de transport

**Règle LS-TRANS-01 : Toutes les décisions d'arbitrage**

Toute décision d'arbitrage de LogisticsSteward validée par StrongFather est transportée par Bonding Brother.

**Règle LS-TRANS-02 : Destinataires explicites**

Chaque décision spécifie ses destinataires. Bonding Brother ne déduit jamais les destinataires.

**Règle LS-TRANS-03 : Transport sans interprétation**

Bonding Brother transporte les décisions sans les interpréter. Il ne modifie pas les quotas, les priorités, ou les restrictions.

---

## 6. Protocole de communication

### 6.1 Format des décisions entrantes

Les décisions reçues de LogisticsSteward suivent ce format :

**Structure de base :**
```typescript
interface DecisionArbitrage {
    decision_id: DecisionId;
    type: TypeDecisionLS;             // quota_assigned, priority_changed, etc.
    source: "logistics_steward";
    validation: ValidationStrongFather; // Preuve de validation par StrongFather
    
    // Contenu de la décision
    payload: PayloadDecision;         // Données spécifiques au type
    
    // Destinataires
    destinataires: Destinataire[];    // Liste des entités à notifier
    
    // Traçabilité
    contexte: ContexteArbitrage;      // Contexte de la décision
    timestamp: LogicalClock;          // Horloge logique (LOI-4)
}
```

**Règle LS-PROT-01 : Validation obligatoire**

Toute décision doit inclure la preuve de validation par StrongFather. Bonding Brother refuse de transporter une décision non validée.

**Règle LS-PROT-02 : Destinataires explicites**

La liste des destinataires est fournie par LogisticsSteward. Bonding Brother ne calcule jamais les destinataires.

**Règle LS-PROT-03 : Horloge logique**

Les décisions utilisent une horloge logique (LOI-4), pas de timestamp absolu.

### 6.2 Format des notifications sortantes

Les notifications transmises aux entités suivent ce format :

**Structure de base :**
```typescript
interface NotificationArbitrage {
    notification_id: NotificationId;
    decision_id: DecisionId;          // Référence à la décision source
    type: TypeNotificationLS;         // Type traduit pour le destinataire
    
    // Contenu traduit
    payload: PayloadTraduit;          // Payload adapté au destinataire
    
    // Traçabilité
    source: "logistics_steward";
    transporteur: "bonding_brother";
    timestamp: LogicalClock;
}
```

**Règle LS-PROT-04 : Traduction sans modification**

Le payload est traduit dans le vocabulaire du destinataire, mais le sens est préservé intégralement.

**Règle LS-PROT-05 : Traçabilité complète**

Chaque notification référence la décision source et inclut l'identité du transporteur.

---

## 7. Traduction spécifique à LogisticsSteward

### 7.1 Traduction décision → notification

**Règle LS-TRAD-01 : Mapping de type**

Le type de décision est mappé vers le type de notification selon le destinataire :

**Exemples de mapping :**
- `QUOTA_ASSIGNED` → `quota_notification` (Opérateurs)
- `PRIORITY_CHANGED` → `priority_update` (Services)
- `DEGRADATION_INITIATED` → `degradation_alert` (Tous)
- `RESOURCE_RESTRICTED` → `restriction_notice` (Entités concernées)

**Règle LS-TRAD-02 : Adaptation du payload**

Le payload est adapté au vocabulaire du destinataire :
- Opérateurs : vocabulaire métier simplifié
- MasterButler : vocabulaire technique de capacités
- WorrySentinel : vocabulaire de surveillance
- MiyukiniAdmin : vocabulaire d'administration

**Règle LS-TRAD-03 : Préservation du sens**

La sémantique de la décision est préservée intégralement. Un quota de 100 reste un quota de 100, une priorité de niveau 3 reste de niveau 3.

**Règle LS-TRAD-04 : Métadonnées de transport**

Des métadonnées de transport sont ajoutées (notification_id, timestamp_transport), mais pas de métadonnées qui modifient le sens.

### 7.2 Règles de filtrage

**Règle LS-TRAD-05 : Pas de filtrage du contenu**

Bonding Brother ne filtre pas le contenu des décisions d'arbitrage. Toute la décision est transmise.

**Règle LS-TRAD-06 : Filtrage des destinataires uniquement**

Le seul filtrage autorisé concerne les destinataires : chaque entité reçoit uniquement les notifications qui la concernent.

---

## 8. Gestion des erreurs

### 8.1 Types d'erreurs

**Erreurs de réception :**
- Décision invalide (format incorrect)
- Validation StrongFather absente ou invalide
- Destinataires non spécifiés

**Erreurs de transport :**
- Destinataire indisponible
- Timeout de transmission
- Erreur réseau

**Erreurs de traduction :**
- Type de décision inconnu
- Payload non traduisible

### 8.2 Traitement des erreurs

**Règle LS-ERR-01 : Décision invalide**

Une décision invalide est rejetée avec notification à LogisticsSteward et journalisation complète.

**Règle LS-ERR-02 : Validation absente**

Une décision sans validation StrongFather est rejetée. Bonding Brother ne transporte jamais une décision non validée.

**Règle LS-ERR-03 : Destinataire indisponible**

Si un destinataire est indisponible, la notification est mise en buffer et retentée à la reconnexion (mode offline).

**Règle LS-ERR-04 : Journalisation**

Toutes les erreurs sont journalisées pour audit et analyse.

**Règle LS-ERR-05 : Pas de retry pour rejet**

Les erreurs de validation (décision invalide, validation absente) ne sont pas retentées. Seules les erreurs de transport sont retentées.

---

## 9. Garanties de l'intégration

### 9.1 Garantie de transport fidèle

**Engagement :** Bonding Brother transporte les décisions d'arbitrage sans interprétation, modification, ou évaluation. Le sens de la décision est préservé intégralement.

### 9.2 Garantie de non-décision

**Engagement :** Bonding Brother ne prend jamais de décision d'arbitrage. Il ne modifie jamais un quota, une priorité, ou une restriction.

### 9.3 Garantie de traçabilité

**Engagement :** Tout transport de décision est traçable de bout en bout. Le journal contient toutes les informations nécessaires pour reconstruire le transport complet.

### 9.4 Garantie de livraison

**Engagement :** Toute décision validée sera livrée à ses destinataires (at-least-once). En cas d'indisponibilité, la notification est buffée et retentée.

### 9.5 Garantie de validation

**Engagement :** Bonding Brother ne transporte que des décisions validées par StrongFather. Aucune décision non validée ne sera transmise.

---

## 10. Mode offline

### 10.1 Comportement en mode offline

En mode offline, certains destinataires peuvent être indisponibles. Bonding Brother :
1. Met les notifications en buffer
2. Retente la transmission lors de la reconnexion
3. Journalise l'état du transport

**Règle LS-OFFLINE-01 : Buffer systématique**

Toute notification destinée à un destinataire indisponible est mise en buffer.

**Règle LS-OFFLINE-02 : Retry à la reconnexion**

Lors de la reconnexion, toutes les notifications en buffer sont retentées dans l'ordre chronologique.

**Règle LS-OFFLINE-03 : Journalisation du buffer**

Les notifications en buffer sont journalisées avec leur état (en attente, transmis, échoué).

**Règle LS-OFFLINE-04 : Pas de décision locale**

Bonding Brother ne prend jamais de décision d'arbitrage à la place de LogisticsSteward, même en mode offline.

Voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md) pour les détails.

---

## 11. Performance et limites

### 11.1 Délais

**Délai de transport :** Variable selon la disponibilité des destinataires
**Délai de traduction :** Négligeable (traduction locale)
**Timeout par défaut :** 10 secondes par destinataire (configurable)

### 11.2 Limites

**Taille maximale de décision :** 100 KB
**Taille maximale de notification :** 50 KB
**Nombre de destinataires par décision :** Illimité
**Nombre de notifications simultanées :** Illimité (sous réserve de ressources)

---

## 12. Exemples

### 12.1 Attribution de quota

**Décision LogisticsSteward :**
```json
{
  "decision_id": "dec-ls-001",
  "type": "QUOTA_ASSIGNED",
  "source": "logistics_steward",
  "validation": {
    "validateur": "strong_father",
    "timestamp": "2026-01-28T10:00:00Z",
    "signature": "sf-sig-123"
  },
  "payload": {
    "entite_id": "operateur-cms",
    "type_quota": "api_calls",
    "valeur": 10000,
    "periode": "daily",
    "debut": "2026-01-28T00:00:00Z"
  },
  "destinataires": [
    { "id": "operateur-cms", "type": "operateur" }
  ],
  "contexte": {
    "raison": "attribution_initiale",
    "demandeur": "system"
  },
  "timestamp": "2026-01-28T10:00:00Z"
}
```

**Notification traduite (Opérateur) :**
```json
{
  "notification_id": "notif-bb-001",
  "decision_id": "dec-ls-001",
  "type": "quota_notification",
  "payload": {
    "quota_type": "api_calls",
    "quota_value": 10000,
    "period": "daily",
    "starts_at": "2026-01-28T00:00:00Z",
    "message": "Nouveau quota attribué : 10000 appels API par jour"
  },
  "source": "logistics_steward",
  "transporteur": "bonding_brother",
  "timestamp": "2026-01-28T10:00:01Z"
}
```

### 12.2 Notification de dégradation

**Décision LogisticsSteward :**
```json
{
  "decision_id": "dec-ls-002",
  "type": "DEGRADATION_INITIATED",
  "source": "logistics_steward",
  "validation": {
    "validateur": "strong_father",
    "timestamp": "2026-01-28T11:00:00Z",
    "signature": "sf-sig-456"
  },
  "payload": {
    "niveau": "D2",
    "niveau_precedent": "D0",
    "raison": "charge_elevee",
    "restrictions": [
      { "type": "batch_operations", "action": "disabled" },
      { "type": "background_sync", "action": "reduced" }
    ]
  },
  "destinataires": [
    { "id": "operateur-cms", "type": "operateur" },
    { "id": "operateur-auth", "type": "operateur" },
    { "id": "master-butler", "type": "core" },
    { "id": "worry-sentinel", "type": "core" }
  ],
  "contexte": {
    "raison": "protection_systeme",
    "demandeur": "auto"
  },
  "timestamp": "2026-01-28T11:00:00Z"
}
```

**Notification traduite (Opérateur CMS) :**
```json
{
  "notification_id": "notif-bb-002",
  "decision_id": "dec-ls-002",
  "type": "degradation_alert",
  "payload": {
    "level": "D2",
    "previous_level": "D0",
    "reason": "Charge système élevée",
    "restrictions": [
      "Opérations batch désactivées",
      "Synchronisation en arrière-plan réduite"
    ],
    "message": "Mode dégradé D2 activé. Certaines fonctionnalités sont restreintes."
  },
  "source": "logistics_steward",
  "transporteur": "bonding_brother",
  "timestamp": "2026-01-28T11:00:01Z"
}
```

### 12.3 Notification de préemption

**Décision LogisticsSteward :**
```json
{
  "decision_id": "dec-ls-003",
  "type": "PREEMPTION_NOTIFIED",
  "source": "logistics_steward",
  "validation": {
    "validateur": "strong_father",
    "timestamp": "2026-01-28T12:00:00Z",
    "signature": "sf-sig-789"
  },
  "payload": {
    "entite_preemptee": "operateur-analytics",
    "entite_prioritaire": "miyukini-admin",
    "ressource": "compute_slots",
    "duree_estimee": "PT5M"
  },
  "destinataires": [
    { "id": "operateur-analytics", "type": "operateur" },
    { "id": "worry-sentinel", "type": "core" }
  ],
  "contexte": {
    "raison": "priorite_admin",
    "demandeur": "miyukini-admin"
  },
  "timestamp": "2026-01-28T12:00:00Z"
}
```

**Notification traduite (Opérateur Analytics) :**
```json
{
  "notification_id": "notif-bb-003",
  "decision_id": "dec-ls-003",
  "type": "preemption_notice",
  "payload": {
    "resource": "compute_slots",
    "preempted_by": "système d'administration",
    "estimated_duration": "5 minutes",
    "message": "Vos créneaux de calcul sont temporairement préemptés pour une opération prioritaire."
  },
  "source": "logistics_steward",
  "transporteur": "bonding_brother",
  "timestamp": "2026-01-28T12:00:01Z"
}
```

---

## 13. Invariants

### 13.1 Invariants de transport

| Code | Invariant | Description |
|------|-----------|-------------|
| **BB-LS-INV-1** | Transport sans interprétation | BB ne modifie jamais le sens d'une décision |
| **BB-LS-INV-2** | Validation obligatoire | BB refuse toute décision non validée par SF |
| **BB-LS-INV-3** | Traçabilité complète | Tout transport est journalisé |
| **BB-LS-INV-4** | Destinataires explicites | BB ne calcule jamais les destinataires |
| **BB-LS-INV-5** | Livraison garantie | Toute décision validée sera livrée |

### 13.2 Interdictions

| Code | Interdiction | Raison |
|------|--------------|--------|
| **BB-LS-INT-1** | Modification de quota | Préserver l'intégrité de l'arbitrage |
| **BB-LS-INT-2** | Modification de priorité | Préserver l'intégrité de l'arbitrage |
| **BB-LS-INT-3** | Ajout de destinataires | Préserver la gouvernance de LS |
| **BB-LS-INT-4** | Suppression de destinataires | Préserver la gouvernance de LS |
| **BB-LS-INT-5** | Transport sans validation | Préserver la chaîne de validation SF |

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que Bonding Brother doit respecter pour transporter les décisions d'arbitrage de LogisticsSteward.

Toute implémentation de l'intégration avec LogisticsSteward doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice v2.0](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md)
- [Translation Contract v2.0](../intent/BondingBrother%20-%20Translation%20Contract.md)
- [Offline & Deferred Authority Contract v2.0](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md)
- [LogisticsSteward - Documentation Fondatrice v1.0.0](../../../LogisticsSteward/foundation/LogisticsSteward%20-%20Documentation%20Fondatrice.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)
