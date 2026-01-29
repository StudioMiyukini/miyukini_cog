# BondingBrother - StrongFather Integration Contract

## 1. Contexte

Ce document définit le contrat d'intégration entre Bonding Brother et StrongFather. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec StrongFather en tant qu'autorité des décisions stratégiques et politiques.

Ce document complète la Section 2 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Authority Delegation Contract](../authority/BondingBrother%20-%20Authority%20Delegation%20Contract.md) pour les principes de délégation, le [Product-to-Ecosystem Flow](../flows/BondingBrother%20-%20Product-to-Ecosystem%20Flow.md) pour le flux détaillé, et la documentation de Strong Father pour les spécifications de l'autorité.

L'intégration respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : en mode offline, les intentions sont buffées et synchronisées à la reconnexion (**LOI-2**, **LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre Bonding Brother et Strong Father
- Le protocole de communication (demandes et réponses)
- Les types d'intentions déléguées à Strong Father
- Les règles de traduction spécifiques à Strong Father
- La gestion des erreurs et des réponses
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de Strong Father (voir documentation Strong Father)
- Les règles de traduction générales (voir [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md))
- Les règles de filtrage (voir [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md))
- Le mode offline détaillé (voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md))

---

## 3. Principe fondamental

**Bonding Brother reconnaît StrongFather comme l'autorité absolue des décisions stratégiques et politiques. Il délègue toute évaluation d'intention à StrongFather, et transmet fidèlement le résultat sans interprétation ni modification.**

La relation est de délégation : Bonding Brother transmet les intentions des produits à Strong Father pour évaluation, et transmet les décisions de Strong Father aux produits.

---

## 4. Positionnement de Strong Father

### 4.1 Autorité des décisions stratégiques et politiques

**StrongFather est l'autorité absolue pour :**
- L'évaluation des intentions selon des politiques
- Les décisions politiques et stratégiques
- L'autorisation des actions
- Les règles de sécurité conceptuelles
- La détection des ambiguïtés
- L'établissement des priorités

**Règle SF-POS-01 : Autorité exclusive**

Toute décision concernant les identités, les permissions, ou les politiques est déléguée à Strong Father. Bonding Brother ne prend jamais de décision stratégique ou politique.

**Règle SF-POS-02 : Pas de contournement**

Bonding Brother ne permet jamais aux produits de contourner Strong Father pour prendre des décisions d'autorisation ou d'authentification.

**Règle SF-POS-03 : Adaptation unidirectionnelle**

Bonding Brother s'adapte à Strong Father, jamais l'inverse. Les formats, vocabulaires, et protocoles sont définis par Strong Father.

---

## 5. Types d'intentions déléguées

### 5.1 Intentions d'authentification

**AUTHENTICATE**
- **Délégation :** Authentification d'un utilisateur
- **Traduction :** `AUTHENTICATE` → `authenticate` (Strong Father)
- **Payload :** Identifiants de l'utilisateur (credentials, tokens, etc.)
- **Réponse :** Session créée avec token d'authentification ou erreur

**Règle SF-AUTH-01 : Délégation totale**

Toute décision d'authentification est déléguée à Strong Father. Bonding Brother ne valide jamais les credentials lui-même.

### 5.2 Intentions d'autorisation

**AUTHORIZE**
- **Délégation :** Vérification de l'autorisation d'une action
- **Traduction :** `AUTHORIZE` → `authorize` (Strong Father)
- **Payload :** Action à autoriser, ressource concernée, utilisateur
- **Réponse :** Autorisé ou refusé avec justification

**CHECK_PERMISSION**
- **Délégation :** Vérification d'une permission spécifique
- **Traduction :** `CHECK_PERMISSION` → `check_permission` (Strong Father)
- **Payload :** Permission à vérifier, ressource, utilisateur
- **Réponse :** Permission accordée ou refusée

**Règle SF-AUTHZ-01 : Délégation totale**

Toute décision d'autorisation est déléguée à Strong Father. Bonding Brother ne prend jamais de décision d'autorisation.

**Règle SF-AUTHZ-02 : Pas de cache d'autorisation**

Bonding Brother ne met jamais en cache les décisions d'autorisation. Chaque vérification est déléguée à Strong Father.

### 5.3 Intentions de session

**CREATE_SESSION**
- **Délégation :** Création d'une nouvelle session utilisateur
- **Traduction :** `CREATE_SESSION` → `create_session` (Strong Father)
- **Payload :** Identité de l'utilisateur, métadonnées de session
- **Réponse :** Session créée avec identifiant et token

**REVOKE_SESSION**
- **Délégation :** Révocation d'une session existante
- **Traduction :** `REVOKE_SESSION` → `revoke_session` (Strong Father)
- **Payload :** Identifiant de la session à révoquer
- **Réponse :** Confirmation de révocation ou erreur

**VALIDATE_SESSION**
- **Délégation :** Validation d'une session existante
- **Traduction :** `VALIDATE_SESSION` → `validate_session` (Strong Father)
- **Payload :** Token de session à valider
- **Réponse :** Session valide avec informations utilisateur ou erreur

**Règle SF-SESS-01 : Délégation totale**

Toute décision concernant les sessions est déléguée à Strong Father. Bonding Brother ne gère jamais les sessions lui-même.

### 5.4 Intentions d'évaluation politique

**EVALUATE_INTENTION**
- **Délégation :** Évaluation d'une intention selon des politiques
- **Traduction :** `EVALUATE_INTENTION` → `evaluate_intention` (Strong Father)
- **Payload :** Intention à évaluer, contexte, politiques applicables
- **Réponse :** Décision (acceptée, refusée, ambiguë, différée) avec justification

**Règle SF-POL-01 : Délégation totale**

Toute évaluation politique ou stratégique est déléguée à Strong Father. Bonding Brother ne prend jamais de décision politique.

**Règle SF-POL-02 : Pas d'interprétation**

Bonding Brother ne modifie jamais la décision de Strong Father, même si elle semble contradictoire ou inattendue.

### 5.5 Règles de délégation

**Règle SF-DELEG-01 : Toutes les intentions d'identité**

Toute intention liée à l'authentification, l'autorisation, les sessions, ou les politiques est déléguée à Strong Father.

**Règle SF-DELEG-02 : Pas d'intentions mixtes**

Une intention ne peut pas mélanger des opérations sur identités/permissions et des opérations sur données. Ces dernières sont déléguées à Kind Mother.

**Règle SF-DELEG-03 : Routage déterministe**

Le routage vers Strong Father est déterministe basé sur le type d'intention, pas sur le contenu.

---

## 6. Protocole de communication

### 6.1 Format des demandes

Les demandes transmises à Strong Father suivent le format défini par Strong Father dans son interface contractuelle.

**Structure de base :**
```typescript
interface DemandeStrongFather {
    demande_id: DemandeId;
    intention_id: IntentionId;
    type: TypeDemandeSF;              // authenticate, authorize, create_session, etc.
    données: DonnéesSpécifiques;      // Données traduites
    contexte: ContexteComplet;        // Contexte préservé intégralement
    timestamp: Timestamp;
}
```

**Règle SF-PROT-01 : Format Strong Father**

La demande est dans le format et le vocabulaire que Strong Father comprend, pas dans le format du produit.

**Règle SF-PROT-02 : Contexte complet**

Le contexte est transmis intégralement à Strong Father, sans modification ni filtrage. Strong Father a besoin du contexte complet pour évaluer les politiques.

**Règle SF-PROT-03 : Pas d'enrichissement métier**

Bonding Brother n'ajoute aucune information métier non présente dans l'intention originale.

---

### 6.2 Format des réponses

Les réponses reçues de Strong Father suivent le format défini par Strong Father.

**Structure de base :**
```typescript
interface RéponseStrongFather {
    réponse_id: RéponseId;
    demande_id: DemandeId;
    statut: StatutSF;                  // authorized, denied, ambiguous, deferred
    décision: DécisionSF;              // Décision détaillée
    justification?: JustificationSF;    // Justification de la décision
    données?: DonnéesRetournées;        // Données si applicable (session, permissions, etc.)
    erreurs?: ErreurSF[];               // Erreurs si applicable
    timestamp: Timestamp;
}
```

**Règle SF-PROT-04 : Réception fidèle**

La réponse de Strong Father est reçue intégralement, sans modification ni interprétation.

**Règle SF-PROT-05 : Préservation de la décision**

La décision de Strong Father (autorisé, refusé, ambigu, différé) est préservée intégralement. Aucune modification n'est autorisée.

**Règle SF-PROT-06 : Pas de validation**

Bonding Brother ne valide pas la réponse de Strong Father. Il la transmet telle quelle (après traduction).

**Règle SF-PROT-07 : Préservation de la justification**

La justification de la décision est préservée intégralement et transmise au produit.

---

## 7. Traduction spécifique à Strong Father

### 7.1 Traduction intention → demande

**Règle SF-TRAD-01 : Mapping de type**

Le type d'intention est mappé vers le type de demande Strong Father selon le registre de mappings.

**Exemples de mapping :**
- `AUTHENTICATE` → `authenticate`
- `AUTHORIZE` → `authorize`
- `CHECK_PERMISSION` → `check_permission`
- `CREATE_SESSION` → `create_session`
- `REVOKE_SESSION` → `revoke_session`
- `VALIDATE_SESSION` → `validate_session`
- `EVALUATE_INTENTION` → `evaluate_intention`

**Règle SF-TRAD-02 : Traduction du payload**

Le payload de l'intention est traduit champ par champ selon les règles de mapping définies pour Strong Father.

**Règle SF-TRAD-03 : Préservation du contexte**

Le contexte est transmis intégralement, sans modification. Strong Father a besoin du contexte complet pour évaluer les politiques.

**Règle SF-TRAD-04 : Ajout de métadonnées techniques**

Des métadonnées techniques peuvent être ajoutées (intention_id, timestamp_demande), mais pas de métadonnées métier.

---

### 7.2 Traduction réponse → résultat

**Règle SF-TRAD-05 : Préservation de la décision**

La décision de Strong Father (autorisé, refusé, ambigu, différé) est préservée intégralement. Aucune modification n'est autorisée.

**Règle SF-TRAD-06 : Traduction du statut**

Le statut de la réponse est traduit dans le vocabulaire du produit :
- `authorized` → `AUTORISÉ`
- `denied` → `REFUSÉ`
- `ambiguous` → `AMBIGU`
- `deferred` → `DIFFÉRÉ`
- `error` → `ERREUR`

**Règle SF-TRAD-07 : Traduction des données**

Les données de la réponse (session, permissions, etc.) sont traduites champ par champ selon les règles de mapping définies.

**Règle SF-TRAD-08 : Traduction des erreurs**

Les erreurs de Strong Father sont traduites dans le vocabulaire du produit, avec préservation du code d'erreur technique.

**Règle SF-TRAD-09 : Préservation de la justification**

La justification de la décision est préservée intégralement et transmise au produit.

---

## 8. Gestion des erreurs

### 8.1 Types d'erreurs

**Erreurs de transmission :**
- Autorité indisponible (offline)
- Timeout de connexion
- Erreur réseau

**Erreurs de Strong Father :**
- Demande invalide
- Credentials invalides
- Permission insuffisante
- Session invalide ou expirée
- Politique non applicable
- Ambiguïté non résolue
- Erreur interne

### 8.2 Traitement des erreurs

**Règle SF-ERR-01 : Erreurs de transmission**

Les erreurs de transmission sont gérées en mode offline : l'intention est mise en buffer et retentée lors de la reconnexion.

**Règle SF-ERR-02 : Erreurs de Strong Father**

Les erreurs de Strong Father sont traduites et transmises fidèlement au produit, sans modification ni interprétation.

**Règle SF-ERR-03 : Journalisation**

Toutes les erreurs sont journalisées pour audit et analyse.

**Règle SF-ERR-04 : Pas de retry automatique**

Les erreurs de Strong Father (refus, credentials invalides) ne sont pas retentées automatiquement. Seules les erreurs de transmission sont retentées.

**Règle SF-ERR-05 : Gestion des ambiguïtés**

Les décisions ambiguës de Strong Father sont transmises au produit avec la justification. Le produit doit clarifier l'intention.

---

## 9. Notifications et événements

### 9.1 Réception depuis Strong Father

Strong Father peut émettre des notifications et événements vers Bonding Brother pour informer les produits de changements dans les permissions, les sessions, ou les politiques.

**Types de notifications :**
- Notification de création de session
- Notification de révocation de session
- Notification de changement de permission
- Notification de modification de politique
- Notification d'expiration de session

**Règle SF-NOTIF-01 : Réception fidèle**

Les notifications de Strong Father sont reçues intégralement, sans modification.

**Règle SF-NOTIF-02 : Traduction et distribution**

Les notifications sont traduites et distribuées aux produits concernés selon les règles du flux Écosystème → Produit.

---

## 10. Garanties de l'intégration

### 10.1 Garantie de délégation

**Engagement :** Toute décision concernant les identités, les permissions, ou les politiques est déléguée à Strong Father. Bonding Brother ne prend jamais de décision stratégique ou politique.

### 10.2 Garantie de fidélité

**Engagement :** La sémantique de l'intention est préservée lors de la traduction vers Strong Father, et la décision de Strong Father est transmise fidèlement au produit.

### 10.3 Garantie de non-modification

**Engagement :** Bonding Brother ne modifie jamais la demande avant transmission ni la réponse après réception. Il traduit le format, pas le sens. La décision et la justification sont préservées intégralement.

### 10.4 Garantie de traçabilité

**Engagement :** Toute interaction avec Strong Father est traçable de bout en bout. Le journal contient toutes les informations nécessaires pour reconstruire l'interaction complète, y compris les décisions et justifications.

---

## 11. Mode offline

### 11.1 Comportement en mode offline

En mode offline, Strong Father peut être indisponible. Bonding Brother :
1. Met les intentions en buffer
2. Retente la transmission lors de la reconnexion
3. Transmet les résultats différés aux produits

**Règle SF-OFFLINE-01 : Buffer systématique**

Toute intention destinée à Strong Father est mise en buffer si l'autorité est indisponible.

**Règle SF-OFFLINE-02 : Retry à la reconnexion**

Lors de la reconnexion, toutes les intentions en buffer sont retentées dans l'ordre chronologique.

**Règle SF-OFFLINE-03 : Transmission différée**

Les résultats différés sont transmis aux produits lors de la réception.

**Règle SF-OFFLINE-04 : Pas de décision locale**

Bonding Brother ne prend jamais de décision d'autorisation ou d'authentification à la place de Strong Father, même en mode offline.

Voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md) pour les détails.

---

## 12. Performance et limites

### 12.1 Délais

**Délai de transmission :** Variable selon la disponibilité de Strong Father
**Délai d'évaluation :** Variable selon la complexité de l'évaluation politique
**Timeout par défaut :** 30 secondes (configurable)

### 12.2 Limites

**Taille maximale de demande :** Définie par Strong Father (généralement 100 KB)
**Taille maximale de réponse :** Définie par Strong Father (généralement 1 MB)
**Nombre de demandes simultanées :** Illimité (sous réserve de ressources)

---

## 13. Exemples

### 13.1 Authentification

**Intention produit :**
```json
{
  "id": "int-200",
  "produit_id": "miyukini-cms",
  "type": "AUTHENTICATE",
  "payload": {
    "username": "user@example.com",
    "password": "hashed_password"
  },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:00:00Z",
  "version": "1.0.0"
}
```

**Demande traduite (Strong Father) :**
```json
{
  "demande_id": "dem-800",
  "intention_id": "int-200",
  "type": "authenticate",
  "données": {
    "username": "user@example.com",
    "password": "hashed_password"
  },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:00:00Z"
}
```

**Réponse (Strong Father) :**
```json
{
  "réponse_id": "resp-500",
  "demande_id": "dem-800",
  "statut": "authorized",
  "décision": "AUTHENTICATED",
  "données": {
    "session_id": "session-123",
    "token": "jwt_token_here",
    "user_id": "user-456",
    "expires_at": "2026-01-26T18:00:00Z"
  },
  "timestamp": "2026-01-26T10:01:00Z"
}
```

**Résultat traduit (produit) :**
```json
{
  "résultat_id": "res-200",
  "intention_id": "int-200",
  "demande_id": "dem-800",
  "statut": "AUTORISÉ",
  "décision": "AUTHENTICATED",
  "données": {
    "session_id": "session-123",
    "token": "jwt_token_here",
    "utilisateur_id": "user-456",
    "expire_le": "2026-01-26T18:00:00Z"
  },
  "timestamp": "2026-01-26T10:01:00Z",
  "autorité": "strong_father"
}
```

### 13.2 Autorisation refusée

**Intention produit :**
```json
{
  "id": "int-201",
  "produit_id": "miyukini-cms",
  "type": "AUTHORIZE",
  "payload": {
    "action": "content:delete",
    "ressource_id": "content-789",
    "utilisateur_id": "user-123"
  },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:02:00Z",
  "version": "1.0.0"
}
```

**Réponse (Strong Father) :**
```json
{
  "réponse_id": "resp-501",
  "demande_id": "dem-801",
  "statut": "denied",
  "décision": "REFUSÉ",
  "justification": {
    "raison": "PERMISSION_INSUFFISANTE",
    "message": "L'utilisateur n'a pas la permission 'content:delete' sur cette ressource",
    "politique_appliquée": "content_policy_v1.2"
  },
  "timestamp": "2026-01-26T10:02:30Z"
}
```

**Résultat traduit (produit) :**
```json
{
  "résultat_id": "res-201",
  "intention_id": "int-201",
  "demande_id": "dem-801",
  "statut": "REFUSÉ",
  "décision": "REFUSÉ",
  "justification": {
    "raison": "PERMISSION_INSUFFISANTE",
    "message": "L'utilisateur n'a pas la permission 'content:delete' sur cette ressource",
    "politique_appliquée": "content_policy_v1.2"
  },
  "timestamp": "2026-01-26T10:02:30Z",
  "autorité": "strong_father"
}
```

### 13.3 Décision ambiguë

**Réponse (Strong Father) :**
```json
{
  "réponse_id": "resp-502",
  "demande_id": "dem-802",
  "statut": "ambiguous",
  "décision": "AMBIGU",
  "justification": {
    "raison": "INTENTION_INCOMPLÈTE",
    "message": "L'intention nécessite des clarifications : ressource_id manquant",
    "clarifications_nécessaires": [
      {
        "champ": "ressource_id",
        "raison": "Obligatoire pour évaluer la permission"
      }
    ]
  },
  "timestamp": "2026-01-26T10:03:00Z"
}
```

**Résultat traduit (produit) :**
```json
{
  "résultat_id": "res-202",
  "intention_id": "int-202",
  "demande_id": "dem-802",
  "statut": "AMBIGU",
  "décision": "AMBIGU",
  "justification": {
    "raison": "INTENTION_INCOMPLÈTE",
    "message": "L'intention nécessite des clarifications : ressource_id manquant",
    "clarifications_nécessaires": [
      {
        "champ": "ressource_id",
        "raison": "Obligatoire pour évaluer la permission"
      }
    ]
  },
  "timestamp": "2026-01-26T10:03:00Z",
  "autorité": "strong_father"
}
```

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que Bonding Brother doit respecter pour s'intégrer avec Strong Father.

Toute implémentation de l'intégration avec Strong Father doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice v2.0](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) (Section 2)
- [Authority Delegation Contract v2.0](../authority/BondingBrother%20-%20Authority%20Delegation%20Contract.md)
- [Product-to-Ecosystem Flow v2.0](../flows/BondingBrother%20-%20Product-to-Ecosystem%20Flow.md)
- [Translation Contract v2.0](../intent/BondingBrother%20-%20Translation%20Contract.md)
- StrongFather - Documentation Fondatrice v1.0
