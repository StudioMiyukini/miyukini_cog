# Master Butler - StrongFather Integration Contract

## 1. Contexte

Ce document définit le **contrat d'intégration entre Master Butler et StrongFather**. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec StrongFather en tant qu'autorité des décisions stratégiques et politiques.

Ce document complète la Section 3 de la [Documentation Fondatrice](../../foundation/Master%20Butler%20-%20Documentation%20Fondatrice.md) et s'appuie sur :
- [Master Butler - Capability API Contract](../api/Master%20Butler%20-%20Capability%20API%20Contract.md) pour l'API des capacités
- [Master Butler - Permission API Contract](../api/Master%20Butler%20-%20Permission%20API%20Contract.md) pour l'API des permissions
- [StrongFather - Documentation Fondatrice](../../../StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md) pour la nature de StrongFather
- [StrongFather - Integration Readiness Contract](../../../StrongFather/architecture/StrongFather%20-%20Integration%20Readiness%20Contract.md) pour les règles d'intégration

L'intégration respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : toutes les interrogations sont locales et ne requièrent aucune dépendance externe (**LOI-1**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre Master Butler et StrongFather
- Le protocole de communication (interrogations et réponses)
- Les types d'interrogations acceptées par Master Butler
- Les règles d'intégration spécifiques à StrongFather
- La gestion des erreurs et des réponses
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de StrongFather (voir documentation StrongFather)
- Les détails internes des registres (voir Capability Registry Contract, Permission Registry Contract)
- Les API de déclaration (voir Discovery API Contract)
- L'intégration avec BondingBrother (voir BondingBrother Integration Contract)

---

## 3. Principe fondamental

**StrongFather interroge Master Butler pour obtenir les informations nécessaires à ses décisions. Master Butler fournit ces informations de manière exhaustive, exacte, et non interprétée, sans jamais participer à la décision elle-même.**

La relation est de consultation : StrongFather interroge Master Butler sur les capacités et permissions, Master Butler répond avec les informations demandées. Cette relation est unidirectionnelle en termes de flux informationnel : Master Butler informe, StrongFather décide.

---

## 4. Nature de la relation Master Butler — StrongFather

### 4.1 Relation de consultation

**Master Butler est consulté par StrongFather :**
- Pour connaître l'existence des capacités
- Pour obtenir les permissions associées aux capacités
- Pour calculer le contexte de capacité d'un demandeur
- Pour découvrir les métadonnées des capacités et permissions

**Règle MB-SF-01 : Consultation sans décision**

Master Butler ne participe jamais aux décisions de StrongFather. Il fournit des informations factuelles sur les capacités et permissions, sans recommandation, sans interprétation, sans jugement.

**Règle MB-SF-02 : Exhaustivité des réponses**

Les réponses de Master Butler à StrongFather sont exhaustives. Aucune information pertinente n'est omise ou filtrée.

**Règle MB-SF-03 : Accès privilégié**

StrongFather dispose d'un accès privilégié à Master Butler. Aucune restriction d'accès ne s'applique aux interrogations de StrongFather.

### 4.2 Séparation des responsabilités

| Responsabilité | Master Butler | StrongFather |
|----------------|---------------|--------------|
| **Connaître les capacités** | ✅ Exclusif | ❌ Interroge |
| **Connaître les permissions** | ✅ Exclusif | ❌ Interroge |
| **Décider si autorisé** | ❌ Jamais | ✅ Exclusif |
| **Appliquer des politiques** | ❌ Jamais | ✅ Exclusif |
| **Évaluer des intentions** | ❌ Jamais | ✅ Exclusif |
| **Fournir des informations** | ✅ Exclusif | ❌ Consomme |

**Règle MB-SF-04 : Aucun chevauchement**

Aucun chevauchement de responsabilités n'est autorisé. Master Butler ne prend jamais de décision, StrongFather ne maintient jamais de registre de capacités ou permissions.

---

## 5. Types d'interrogations

### 5.1 Interrogation d'existence de capacité

**CAPABILITY_EXISTS**
- **Objectif :** Vérifier si une capacité existe dans le registre
- **Payload :** Identifiant de la capacité
- **Réponse :** Existence (booléen) + métadonnées si existante

**Règle MB-SF-QUERY-01 : Réponse binaire enrichie**

L'existence est une réponse binaire (existe/n'existe pas), mais si la capacité existe, les métadonnées sont fournies.

### 5.2 Interrogation des permissions requises

**REQUIRED_PERMISSIONS**
- **Objectif :** Obtenir les permissions requises pour accéder à une capacité
- **Payload :** Identifiant de la capacité
- **Réponse :** Liste des permissions associées avec leurs métadonnées

**Règle MB-SF-QUERY-02 : Liste exhaustive**

La liste des permissions est exhaustive. Toutes les permissions associées à la capacité sont retournées.

### 5.3 Interrogation du contexte de capacité

**CAPABILITY_CONTEXT**
- **Objectif :** Calculer le contexte de capacité pour un demandeur donné
- **Payload :** Identité du demandeur, rôles, module cible
- **Réponse :** Capacités accessibles, permissions détenues, associations

**Règle MB-SF-QUERY-03 : Calcul de projection**

Le contexte de capacité est une projection des capacités et permissions disponibles pour le demandeur dans le contexte donné. Ce calcul ne modifie pas le registre.

### 5.4 Interrogation de permission

**PERMISSION_EXISTS**
- **Objectif :** Vérifier si une permission existe dans le registre
- **Payload :** Identifiant de la permission
- **Réponse :** Existence (booléen) + métadonnées si existante

**PERMISSION_DETAILS**
- **Objectif :** Obtenir les détails d'une permission
- **Payload :** Identifiant de la permission
- **Réponse :** Métadonnées complètes, capacités associées, niveaux

### 5.5 Interrogation d'association rôle-permission

**ROLE_PERMISSIONS**
- **Objectif :** Obtenir les permissions associées à un rôle
- **Payload :** Identifiant du rôle
- **Réponse :** Liste des permissions avec leurs métadonnées

**Règle MB-SF-QUERY-04 : Rôles connus uniquement**

Master Butler connaît les associations rôles-permissions, mais ne gère pas les attributions de rôles aux utilisateurs (hors-scope).

### 5.6 Interrogation de Tool/Toolkit

**TOOL_EXISTS**
- **Objectif :** Vérifier si un Tool existe dans le catalogue
- **Payload :** Identifiant du Tool
- **Réponse :** Existence + métadonnées si existant

**TOOLKIT_COMPOSITION**
- **Objectif :** Obtenir la composition d'un Toolkit
- **Payload :** Identifiant du Toolkit
- **Réponse :** Liste des Tools composant le Toolkit avec leurs métadonnées

### 5.7 Règles générales d'interrogation

**Règle MB-SF-QUERY-05 : Toute interrogation est sans état**

Les interrogations de StrongFather ne modifient jamais l'état de Master Butler. Ce sont des lectures pures.

**Règle MB-SF-QUERY-06 : Pas d'effet de bord**

Aucune interrogation ne produit d'effet de bord sur le registre, les associations, ou les métadonnées.

**Règle MB-SF-QUERY-07 : Réponse immédiate**

Les réponses sont fournies immédiatement. Aucune interrogation n'est mise en attente ou différée.

---

## 6. Protocole de communication

### 6.1 Format des interrogations

Les interrogations de StrongFather suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `interrogation_id` | Identifiant unique de l'interrogation | ✅ Oui |
| `intention_id` | Référence à l'intention en cours d'évaluation | ✅ Oui |
| `type` | Type d'interrogation | ✅ Oui |
| `payload` | Données spécifiques à l'interrogation | ✅ Oui |
| `contexte_appelant` | Contexte de StrongFather | ✅ Oui |
| `timestamp` | Horodatage de l'interrogation | ✅ Oui |

**Règle MB-SF-PROT-01 : Format standardisé**

Toutes les interrogations respectent le format standardisé. Aucune interrogation ad-hoc n'est acceptée.

**Règle MB-SF-PROT-02 : Traçabilité par intention**

Chaque interrogation référence l'intention en cours d'évaluation pour assurer la traçabilité bout-en-bout.

### 6.2 Format des réponses

Les réponses de Master Butler suivent un format standardisé.

**Structure de base :**

| Élément | Description | Obligatoire |
|---------|-------------|-------------|
| `reponse_id` | Identifiant unique de la réponse | ✅ Oui |
| `interrogation_id` | Référence à l'interrogation | ✅ Oui |
| `statut` | Statut de la réponse (SUCCESS, NOT_FOUND, ERROR) | ✅ Oui |
| `donnees` | Données de la réponse | Si SUCCESS |
| `erreur` | Détails de l'erreur | Si ERROR |
| `timestamp` | Horodatage de la réponse | ✅ Oui |

**Règle MB-SF-PROT-03 : Réponse toujours structurée**

Master Butler retourne toujours une réponse structurée, même en cas d'erreur ou de non-existence.

**Règle MB-SF-PROT-04 : Pas d'interprétation**

Les réponses sont des informations brutes. Master Butler n'interprète pas les données pour StrongFather.

### 6.3 Statuts de réponse

| Statut | Signification |
|--------|---------------|
| `SUCCESS` | L'interrogation a abouti, les données sont fournies |
| `NOT_FOUND` | L'élément recherché n'existe pas dans le registre |
| `INVALID_QUERY` | L'interrogation est mal formée ou incomplète |
| `ERROR` | Une erreur interne s'est produite |

**Règle MB-SF-PROT-05 : NOT_FOUND n'est pas une erreur**

Le statut `NOT_FOUND` est une réponse valide, pas une erreur. Il indique que l'élément recherché n'existe pas dans le registre.

---

## 7. Flux d'interrogation typique

### 7.1 Flux complet d'évaluation d'intention

**Acteurs :** BondingBrother, StrongFather, Master Butler

**Séquence :**

1. BondingBrother soumet une intention à StrongFather pour évaluation
2. StrongFather identifie les capacités impliquées dans l'intention
3. StrongFather interroge Master Butler : `CAPABILITY_EXISTS`
4. Master Butler répond avec l'existence et les métadonnées
5. StrongFather interroge Master Butler : `REQUIRED_PERMISSIONS`
6. Master Butler répond avec les permissions requises
7. StrongFather interroge Master Butler : `ROLE_PERMISSIONS` (pour le demandeur)
8. Master Butler répond avec les permissions du demandeur
9. StrongFather évalue l'intention selon les politiques avec les informations obtenues
10. StrongFather produit une décision (acceptée, refusée, ambiguë, différée)

**Règle MB-SF-FLOW-01 : Interrogations multiples possibles**

StrongFather peut effectuer plusieurs interrogations pour une même évaluation d'intention. Master Butler répond à chacune indépendamment.

### 7.2 Flux de calcul de contexte de capacité

**Acteurs :** BondingBrother, StrongFather, Master Butler

**Séquence :**

1. BondingBrother demande le contexte de capacité pour traduire une intention
2. BondingBrother interroge Master Butler : `CAPABILITY_CONTEXT`
3. Master Butler calcule le contexte de capacité
4. Master Butler retourne les capacités accessibles et permissions
5. BondingBrother utilise le contexte pour la traduction

**Note :** Ce flux peut aussi être initié par StrongFather selon l'architecture choisie.

### 7.3 Diagramme de séquence

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  BondingBrother │    │   StrongFather  │    │  Master Butler  │
└────────┬────────┘    └────────┬────────┘    └────────┬────────┘
         │                      │                      │
         ├── Intention ────────►│                      │
         │                      │                      │
         │                      ├── CAPABILITY_EXISTS ►│
         │                      │                      │
         │                      │◄── Existence + Meta ─┤
         │                      │                      │
         │                      ├── REQUIRED_PERMS ───►│
         │                      │                      │
         │                      │◄── Permissions ──────┤
         │                      │                      │
         │                      ├── ROLE_PERMISSIONS ─►│
         │                      │                      │
         │                      │◄── Permissions rôle ─┤
         │                      │                      │
         │                      ├── Évaluation ────────┤
         │                      │   (interne)          │
         │                      │                      │
         │◄── Décision ─────────┤                      │
         │                      │                      │
```

---

## 8. Règles d'intégration

### 8.1 Règles de communication

**Règle MB-SF-INT-01 : StrongFather initie toujours**

StrongFather initie toujours les interrogations. Master Butler ne contacte jamais StrongFather spontanément.

**Règle MB-SF-INT-02 : Pas de notification proactive**

Master Butler ne notifie jamais StrongFather de changements dans les registres. Si StrongFather a besoin d'informations à jour, il interroge à nouveau.

**Règle MB-SF-INT-03 : Synchronisme des réponses**

Les réponses de Master Butler sont synchrones. StrongFather attend la réponse avant de poursuivre l'évaluation.

### 8.2 Règles de données

**Règle MB-SF-INT-04 : Données fraîches**

Les données retournées par Master Butler reflètent l'état actuel du registre au moment de l'interrogation.

**Règle MB-SF-INT-05 : Pas de cache côté StrongFather**

StrongFather ne met jamais en cache les réponses de Master Butler. Chaque évaluation nécessite de nouvelles interrogations.

**Règle MB-SF-INT-06 : Cohérence garantie**

Master Butler garantit la cohérence des données retournées. Les informations sur une capacité et ses permissions sont cohérentes entre elles.

### 8.3 Règles de traçabilité

**Règle MB-SF-INT-07 : Traçabilité des interrogations**

Toutes les interrogations de StrongFather sont tracées par Master Butler avec le contexte complet.

**Règle MB-SF-INT-08 : Corrélation intention-interrogation**

Chaque interrogation est corrélée à l'intention en cours d'évaluation pour permettre l'audit bout-en-bout.

---

## 9. Gestion des erreurs

### 9.1 Types d'erreurs

**Erreurs de format :**
- Interrogation mal formée
- Champ obligatoire manquant
- Type d'interrogation inconnu

**Erreurs de données :**
- Capacité inexistante (NOT_FOUND, pas une erreur)
- Permission inexistante (NOT_FOUND, pas une erreur)
- Rôle inconnu

**Erreurs internes :**
- Erreur de registre
- Erreur de calcul de contexte

### 9.2 Traitement des erreurs

**Règle MB-SF-ERR-01 : Réponse structurée toujours**

Master Butler retourne toujours une réponse structurée, même en cas d'erreur. StrongFather peut toujours interpréter la réponse.

**Règle MB-SF-ERR-02 : NOT_FOUND est informatif**

Le statut `NOT_FOUND` est une information, pas une erreur. StrongFather peut utiliser cette information dans son évaluation (capacité inexistante = intention invalide).

**Règle MB-SF-ERR-03 : Journalisation des erreurs**

Toutes les erreurs sont journalisées par Master Butler pour audit et diagnostic.

**Règle MB-SF-ERR-04 : Pas de retry automatique**

En cas d'erreur, StrongFather décide de la stratégie (retry, échec de l'évaluation). Master Butler ne retry jamais automatiquement.

---

## 10. Garanties de l'intégration

### 10.1 Garantie d'exhaustivité

**Engagement :** Les réponses de Master Butler sont exhaustives. Toutes les informations pertinentes sont fournies sans omission.

### 10.2 Garantie d'exactitude

**Engagement :** Les informations fournies par Master Butler sont exactes et reflètent l'état actuel du registre.

### 10.3 Garantie de neutralité

**Engagement :** Master Butler fournit des informations sans interprétation, sans recommandation, sans jugement. La décision appartient exclusivement à StrongFather.

### 10.4 Garantie de traçabilité

**Engagement :** Toute interaction entre StrongFather et Master Butler est traçable de bout en bout. L'audit complet des interrogations et réponses est possible.

### 10.5 Garantie de disponibilité

**Engagement :** Master Butler est disponible pour répondre aux interrogations de StrongFather sans dépendance externe (conformité LOI-1).

### 10.6 Garantie de cohérence

**Engagement :** Les informations retournées sont cohérentes entre elles. Si une capacité et ses permissions sont interrogées, les données sont mutuellement cohérentes.

---

## 11. Invariants de l'intégration

### 11.1 Invariants de relation

**INV-MB-SF-1 : Consultation unidirectionnelle**

StrongFather interroge Master Butler. Master Butler ne sollicite jamais StrongFather.

**INV-MB-SF-2 : Information sans décision**

Master Butler fournit des informations. Il ne participe jamais aux décisions de StrongFather.

**INV-MB-SF-3 : Accès sans restriction**

StrongFather a un accès sans restriction aux informations de Master Butler.

### 11.2 Invariants de données

**INV-MB-SF-4 : Lecture pure**

Les interrogations sont des lectures pures. Aucune modification du registre n'est causée par une interrogation.

**INV-MB-SF-5 : Données factuelles**

Les données retournées sont factuelles (existe/n'existe pas, liste de permissions, métadonnées). Aucune donnée interprétée n'est retournée.

### 11.3 Invariants de protocole

**INV-MB-SF-6 : Format respecté**

Toutes les interrogations et réponses respectent le format standardisé.

**INV-MB-SF-7 : Traçabilité complète**

Toute interaction est traçable avec son contexte complet.

---

## 12. Exemples

### 12.1 Interrogation d'existence de capacité

**Interrogation StrongFather :**
```
{
  "interrogation_id": "int-sf-001",
  "intention_id": "intention-500",
  "type": "CAPABILITY_EXISTS",
  "payload": {
    "capability_id": "content.create"
  },
  "contexte_appelant": {
    "source": "strongfather",
    "evaluation_id": "eval-200"
  },
  "timestamp": "2026-01-27T14:00:00Z"
}
```

**Réponse Master Butler :**
```
{
  "reponse_id": "resp-mb-001",
  "interrogation_id": "int-sf-001",
  "statut": "SUCCESS",
  "donnees": {
    "exists": true,
    "capability": {
      "id": "content.create",
      "name": "Create Content",
      "module": "miyukini-spm-cms-content",
      "description": "Ability to create new content items",
      "created_at": "2026-01-15T10:00:00Z"
    }
  },
  "timestamp": "2026-01-27T14:00:01Z"
}
```

### 12.2 Interrogation des permissions requises

**Interrogation StrongFather :**
```
{
  "interrogation_id": "int-sf-002",
  "intention_id": "intention-500",
  "type": "REQUIRED_PERMISSIONS",
  "payload": {
    "capability_id": "content.create"
  },
  "contexte_appelant": {
    "source": "strongfather",
    "evaluation_id": "eval-200"
  },
  "timestamp": "2026-01-27T14:00:02Z"
}
```

**Réponse Master Butler :**
```
{
  "reponse_id": "resp-mb-002",
  "interrogation_id": "int-sf-002",
  "statut": "SUCCESS",
  "donnees": {
    "capability_id": "content.create",
    "required_permissions": [
      {
        "id": "content.create.any",
        "name": "Create Any Content",
        "level": "standard"
      },
      {
        "id": "content.create.own",
        "name": "Create Own Content",
        "level": "basic"
      }
    ]
  },
  "timestamp": "2026-01-27T14:00:03Z"
}
```

### 12.3 Capacité inexistante

**Interrogation StrongFather :**
```
{
  "interrogation_id": "int-sf-003",
  "intention_id": "intention-501",
  "type": "CAPABILITY_EXISTS",
  "payload": {
    "capability_id": "nonexistent.capability"
  },
  "contexte_appelant": {
    "source": "strongfather",
    "evaluation_id": "eval-201"
  },
  "timestamp": "2026-01-27T14:05:00Z"
}
```

**Réponse Master Butler :**
```
{
  "reponse_id": "resp-mb-003",
  "interrogation_id": "int-sf-003",
  "statut": "NOT_FOUND",
  "donnees": {
    "exists": false,
    "capability_id": "nonexistent.capability"
  },
  "timestamp": "2026-01-27T14:05:01Z"
}
```

**Note :** StrongFather peut utiliser cette information pour refuser l'intention (capacité inexistante = intention invalide).

### 12.4 Interrogation du contexte de capacité

**Interrogation StrongFather :**
```
{
  "interrogation_id": "int-sf-004",
  "intention_id": "intention-502",
  "type": "CAPABILITY_CONTEXT",
  "payload": {
    "requester_id": "user-123",
    "roles": ["editor", "reviewer"],
    "target_module": "miyukini-spm-cms-content"
  },
  "contexte_appelant": {
    "source": "strongfather",
    "evaluation_id": "eval-202"
  },
  "timestamp": "2026-01-27T14:10:00Z"
}
```

**Réponse Master Butler :**
```
{
  "reponse_id": "resp-mb-004",
  "interrogation_id": "int-sf-004",
  "statut": "SUCCESS",
  "donnees": {
    "requester_id": "user-123",
    "target_module": "miyukini-spm-cms-content",
    "accessible_capabilities": [
      "content.create",
      "content.read",
      "content.update.own"
    ],
    "held_permissions": [
      "content.create.own",
      "content.read.all",
      "content.update.own"
    ],
    "missing_for_full_access": [
      "content.delete",
      "content.publish"
    ]
  },
  "timestamp": "2026-01-27T14:10:02Z"
}
```

---

## 13. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que Master Butler doit respecter pour s'intégrer avec StrongFather.

Toute implémentation de l'intégration avec StrongFather doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 1.0  
**Date :** 2026-01-27  
**Statut :** CONTRAT — Normatif  
**Dépendances :**
- Master Butler - Documentation Fondatrice v1.4 (Section 3)
- Master Butler - Capability API Contract v1.0
- Master Butler - Permission API Contract v1.0
- StrongFather - Documentation Fondatrice v1.5
- StrongFather - Integration Readiness Contract v1.0

---

## 14. Mini log de génération

### Décision éditoriale E1 : Direction de la relation

**Décision prise :** La relation est de consultation : StrongFather interroge, Master Butler répond. Cette direction est l'inverse de la relation BondingBrother → StrongFather.

**Application :** Tout le document est structuré autour de cette direction unidirectionnelle.

### Décision éditoriale E2 : Types d'interrogations

**Décision prise :** Les types d'interrogations sont définis exhaustivement : existence de capacité, permissions requises, contexte de capacité, détails de permission, permissions de rôle, Tools et Toolkits.

**Application :** Section 5 définit chaque type avec objectif, payload, et réponse.

### Warning W1 : NOT_FOUND vs ERROR

**Warning rencontré :** Risque de confusion entre "élément non trouvé" (information valide) et "erreur".

**Décision prise :** Le statut `NOT_FOUND` est explicitement défini comme une réponse valide, pas une erreur. StrongFather peut utiliser cette information dans son évaluation.

**Correction effectuée :** Section 6.3 et règle MB-SF-ERR-02 clarifient cette distinction.

### Warning W2 : Cache côté StrongFather

**Warning rencontré :** Risque que StrongFather mette en cache les réponses, conduisant à des décisions basées sur des données obsolètes.

**Décision prise :** Règle MB-SF-INT-05 interdit explicitement le cache côté StrongFather.

**Correction effectuée :** Règle explicite ajoutée dans la section 8.2.

### Vérification de cohérence

**Vérification effectuée :**
- ✅ Cohérence avec Master Butler - Documentation Fondatrice : Confirmée (flux d'interrogation, séparation des responsabilités)
- ✅ Cohérence avec StrongFather - Documentation Fondatrice : Confirmée (StrongFather interroge, ne maintient pas de registre)
- ✅ Cohérence avec StrongFather - Integration Readiness Contract : Confirmée (interfaces conformes)
- ✅ Conformité LOI-1 : Confirmée (aucune dépendance externe pour les interrogations)

**Conclusion :** Aucune contradiction détectée. Le document est cohérent et non ambigu.

---

*Aucune autre erreur, warning, ou ambiguïté rencontrée lors de la rédaction de ce document.*
