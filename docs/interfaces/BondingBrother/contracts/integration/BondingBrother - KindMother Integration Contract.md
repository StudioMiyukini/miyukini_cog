# BondingBrother - KindMother Integration Contract

## 1. Contexte

Ce document définit le contrat d'intégration entre Bonding Brother et Kind Mother. Il spécifie l'interface, le protocole, les règles de communication, et les garanties associées à l'intégration avec Kind Mother en tant qu'autorité des données.

Ce document complète la Section 2 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Authority Delegation Contract](../authority/BondingBrother%20-%20Authority%20Delegation%20Contract.md) pour les principes de délégation, le [Product-to-Ecosystem Flow](../flows/BondingBrother%20-%20Product-to-Ecosystem%20Flow.md) pour le flux détaillé, et la documentation de Kind Mother pour les spécifications de l'autorité.

L'intégration respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : en mode offline, les intentions sont buffées et synchronisées à la reconnexion (**LOI-2**, **LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- L'interface contractuelle entre Bonding Brother et Kind Mother
- Le protocole de communication (demandes et réponses)
- Les types d'intentions déléguées à Kind Mother
- Les règles de traduction spécifiques à Kind Mother
- La gestion des erreurs et des réponses
- Les garanties de l'intégration

Ce document **ne couvre pas** :
- Les détails internes de Kind Mother (voir documentation Kind Mother)
- Les règles de traduction générales (voir [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md))
- Les règles de filtrage (voir [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md))
- Le mode offline détaillé (voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md))

---

## 3. Principe fondamental

**Bonding Brother reconnaît Kind Mother comme l'autorité absolue des données. Il s'adapte à Kind Mother, jamais l'inverse. Toute décision concernant la persistance, la cohérence, ou l'intégrité des données appartient exclusivement à Kind Mother.**

La relation est asymétrique : Bonding Brother traduit les intentions des produits en demandes que Kind Mother comprend, et traduit les réponses de Kind Mother en résultats que les produits peuvent consommer.

---

## 4. Positionnement de Kind Mother

### 4.1 Autorité des données

**Kind Mother est l'autorité absolue pour :**
- La persistance des données
- La cohérence des données
- L'intégrité des données
- La gestion des hiérarchies
- La gestion des relations entre entités
- La synchronisation entre instances

**Règle KM-POS-01 : Autorité exclusive**

Toute décision concernant les données est déléguée à Kind Mother. Bonding Brother ne prend jamais de décision sur les données.

**Règle KM-POS-02 : Pas de contournement**

Bonding Brother ne permet jamais aux produits de contourner Kind Mother pour accéder directement aux données.

**Règle KM-POS-03 : Adaptation unidirectionnelle**

Bonding Brother s'adapte à Kind Mother, jamais l'inverse. Les formats, vocabulaires, et protocoles sont définis par Kind Mother.

---

## 5. Types d'intentions déléguées

### 5.1 Intentions de contenu

**CREATE_CONTENT**
- **Délégation :** Création d'un nouveau contenu
- **Traduction :** `CREATE_CONTENT` → `create_content` (Kind Mother)
- **Payload :** Données du contenu à créer (titre, corps, métadonnées, etc.)
- **Réponse :** Contenu créé avec identifiant unique

**UPDATE_CONTENT**
- **Délégation :** Modification d'un contenu existant
- **Traduction :** `UPDATE_CONTENT` → `update_content` (Kind Mother)
- **Payload :** Identifiant du contenu + modifications
- **Réponse :** Contenu modifié ou erreur si non trouvé/non autorisé

**DELETE_CONTENT**
- **Délégation :** Suppression d'un contenu
- **Traduction :** `DELETE_CONTENT` → `delete_content` (Kind Mother)
- **Payload :** Identifiant du contenu à supprimer
- **Réponse :** Confirmation de suppression ou erreur

**READ_CONTENT**
- **Délégation :** Lecture d'un contenu par identifiant
- **Traduction :** `READ_CONTENT` → `read_content` (Kind Mother)
- **Payload :** Identifiant du contenu à lire
- **Réponse :** Données du contenu ou erreur si non trouvé/non autorisé

**QUERY_CONTENT**
- **Délégation :** Recherche de contenus selon des critères
- **Traduction :** `QUERY_CONTENT` → `query_content` (Kind Mother)
- **Payload :** Critères de recherche (filtres, tri, pagination)
- **Réponse :** Liste de contenus correspondants

### 5.2 Intentions de hiérarchie

**CREATE_NODE**
- **Délégation :** Création d'un nœud dans la hiérarchie
- **Traduction :** `CREATE_NODE` → `create_node` (Kind Mother)
- **Payload :** Données du nœud + position dans la hiérarchie
- **Réponse :** Nœud créé avec identifiant unique

**MOVE_NODE**
- **Délégation :** Déplacement d'un nœud dans la hiérarchie
- **Traduction :** `MOVE_NODE` → `move_node` (Kind Mother)
- **Payload :** Identifiant du nœud + nouvelle position
- **Réponse :** Confirmation de déplacement ou erreur

**DELETE_NODE**
- **Délégation :** Suppression d'un nœud de la hiérarchie
- **Traduction :** `DELETE_NODE` → `delete_node` (Kind Mother)
- **Payload :** Identifiant du nœud à supprimer
- **Réponse :** Confirmation de suppression ou erreur

### 5.3 Règles de délégation

**Règle KM-DELEG-01 : Toutes les intentions de données**

Toute intention liée à la persistance, la modification, ou la consultation de données est déléguée à Kind Mother.

**Règle KM-DELEG-02 : Pas d'intentions mixtes**

Une intention ne peut pas mélanger des opérations sur données et des opérations sur identités/permissions. Ces dernières sont déléguées à Strong Father.

**Règle KM-DELEG-03 : Routage déterministe**

Le routage vers Kind Mother est déterministe basé sur le type d'intention, pas sur le contenu.

---

## 6. Protocole de communication

### 6.1 Format des demandes

Les demandes transmises à Kind Mother suivent le format défini par Kind Mother dans son interface contractuelle.

**Structure de base :**
```typescript
interface DemandeKindMother {
    demande_id: DemandeId;
    intention_id: IntentionId;
    type: TypeDemandeKM;              // create_content, update_content, etc.
    données: DonnéesSpécifiques;      // Données traduites
    contexte: ContexteComplet;        // Contexte préservé intégralement
    timestamp: Timestamp;
}
```

**Règle KM-PROT-01 : Format Kind Mother**

La demande est dans le format et le vocabulaire que Kind Mother comprend, pas dans le format du produit.

**Règle KM-PROT-02 : Contexte complet**

Le contexte est transmis intégralement à Kind Mother, sans modification ni filtrage.

**Règle KM-PROT-03 : Pas d'enrichissement métier**

Bonding Brother n'ajoute aucune information métier non présente dans l'intention originale.

---

### 6.2 Format des réponses

Les réponses reçues de Kind Mother suivent le format défini par Kind Mother.

**Structure de base :**
```typescript
interface RéponseKindMother {
    réponse_id: RéponseId;
    demande_id: DemandeId;
    statut: StatutKM;                  // accepted, denied, error
    données?: DonnéesRetournées;        // Données si applicable
    erreurs?: ErreurKM[];               // Erreurs si applicable
    timestamp: Timestamp;
}
```

**Règle KM-PROT-04 : Réception fidèle**

La réponse de Kind Mother est reçue intégralement, sans modification ni interprétation.

**Règle KM-PROT-05 : Préservation de la décision**

La décision de Kind Mother (acceptée, refusée, erreur) est préservée intégralement.

**Règle KM-PROT-06 : Pas de validation**

Bonding Brother ne valide pas la réponse de Kind Mother. Il la transmet telle quelle (après traduction).

---

## 7. Traduction spécifique à Kind Mother

### 7.1 Traduction intention → demande

**Règle KM-TRAD-01 : Mapping de type**

Le type d'intention est mappé vers le type de demande Kind Mother selon le registre de mappings.

**Exemples de mapping :**
- `CREATE_CONTENT` → `create_content`
- `UPDATE_CONTENT` → `update_content`
- `DELETE_CONTENT` → `delete_content`
- `READ_CONTENT` → `read_content`
- `QUERY_CONTENT` → `query_content`
- `CREATE_NODE` → `create_node`
- `MOVE_NODE` → `move_node`
- `DELETE_NODE` → `delete_node`

**Règle KM-TRAD-02 : Traduction du payload**

Le payload de l'intention est traduit champ par champ selon les règles de mapping définies pour Kind Mother.

**Règle KM-TRAD-03 : Préservation du contexte**

Le contexte est transmis intégralement, sans modification.

**Règle KM-TRAD-04 : Ajout de métadonnées techniques**

Des métadonnées techniques peuvent être ajoutées (intention_id, timestamp_demande), mais pas de métadonnées métier.

---

### 7.2 Traduction réponse → résultat

**Règle KM-TRAD-05 : Préservation de la décision**

La décision de Kind Mother (acceptée, refusée, erreur) est préservée intégralement.

**Règle KM-TRAD-06 : Traduction du statut**

Le statut de la réponse est traduit dans le vocabulaire du produit :
- `accepted` → `SUCCÈS`
- `denied` → `REFUSÉ`
- `error` → `ERREUR`

**Règle KM-TRAD-07 : Traduction des données**

Les données de la réponse sont traduites champ par champ selon les règles de mapping définies.

**Règle KM-TRAD-08 : Traduction des erreurs**

Les erreurs de Kind Mother sont traduites dans le vocabulaire du produit, avec préservation du code d'erreur technique.

---

## 8. Gestion des erreurs

### 8.1 Types d'erreurs

**Erreurs de transmission :**
- Autorité indisponible (offline)
- Timeout de connexion
- Erreur réseau

**Erreurs de Kind Mother :**
- Demande invalide
- Permission insuffisante
- Contrainte violée
- Ressource non trouvée
- Erreur interne

### 8.2 Traitement des erreurs

**Règle KM-ERR-01 : Erreurs de transmission**

Les erreurs de transmission sont gérées en mode offline : l'intention est mise en buffer et retentée lors de la reconnexion.

**Règle KM-ERR-02 : Erreurs de Kind Mother**

Les erreurs de Kind Mother sont traduites et transmises fidèlement au produit, sans modification ni interprétation.

**Règle KM-ERR-03 : Journalisation**

Toutes les erreurs sont journalisées pour audit et analyse.

**Règle KM-ERR-04 : Pas de retry automatique**

Les erreurs de Kind Mother (refus, contrainte violée) ne sont pas retentées automatiquement. Seules les erreurs de transmission sont retentées.

---

## 9. Notifications et événements

### 9.1 Réception depuis Kind Mother

Kind Mother peut émettre des notifications et événements vers Bonding Brother pour informer les produits de changements dans les données.

**Types de notifications :**
- Notification de création de contenu
- Notification de modification de contenu
- Notification de suppression de contenu
- Notification de changement de hiérarchie
- Notification de synchronisation disponible

**Règle KM-NOTIF-01 : Réception fidèle**

Les notifications de Kind Mother sont reçues intégralement, sans modification.

**Règle KM-NOTIF-02 : Traduction et distribution**

Les notifications sont traduites et distribuées aux produits concernés selon les règles du flux Écosystème → Produit.

---

## 10. Garanties de l'intégration

### 10.1 Garantie de délégation

**Engagement :** Toute décision concernant les données est déléguée à Kind Mother. Bonding Brother ne prend jamais de décision sur les données.

### 10.2 Garantie de fidélité

**Engagement :** La sémantique de l'intention est préservée lors de la traduction vers Kind Mother, et la décision de Kind Mother est transmise fidèlement au produit.

### 10.3 Garantie de non-modification

**Engagement :** Bonding Brother ne modifie jamais la demande avant transmission ni la réponse après réception. Il traduit le format, pas le sens.

### 10.4 Garantie de traçabilité

**Engagement :** Toute interaction avec Kind Mother est traçable de bout en bout. Le journal contient toutes les informations nécessaires pour reconstruire l'interaction complète.

---

## 11. Mode offline

### 11.1 Comportement en mode offline

En mode offline, Kind Mother peut être indisponible. Bonding Brother :
1. Met les intentions en buffer
2. Retente la transmission lors de la reconnexion
3. Transmet les résultats différés aux produits

**Règle KM-OFFLINE-01 : Buffer systématique**

Toute intention destinée à Kind Mother est mise en buffer si l'autorité est indisponible.

**Règle KM-OFFLINE-02 : Retry à la reconnexion**

Lors de la reconnexion, toutes les intentions en buffer sont retentées dans l'ordre chronologique.

**Règle KM-OFFLINE-03 : Transmission différée**

Les résultats différés sont transmis aux produits lors de la réception.

Voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20%26%20Deferred%20Authority%20Contract.md) pour les détails.

---

## 12. Performance et limites

### 12.1 Délais

**Délai de transmission :** Variable selon la disponibilité de Kind Mother
**Délai d'évaluation :** Variable selon la complexité de l'opération
**Timeout par défaut :** 30 secondes (configurable)

### 12.2 Limites

**Taille maximale de demande :** Définie par Kind Mother (généralement 1 MB)
**Taille maximale de réponse :** Définie par Kind Mother (généralement 10 MB)
**Nombre de demandes simultanées :** Illimité (sous réserve de ressources)

---

## 13. Exemples

### 13.1 Création de contenu

**Intention produit :**
```json
{
  "id": "int-123",
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Mon article",
    "contenu": "Contenu de l'article...",
    "auteur": "user-456"
  },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:00:00Z",
  "version": "1.0.0"
}
```

**Demande traduite (Kind Mother) :**
```json
{
  "demande_id": "dem-789",
  "intention_id": "int-123",
  "type": "create_content",
  "données": {
    "title": "Mon article",
    "body": "Contenu de l'article...",
    "author_id": "user-456"
  },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:00:00Z"
}
```

**Réponse (Kind Mother) :**
```json
{
  "réponse_id": "resp-456",
  "demande_id": "dem-789",
  "statut": "accepted",
  "données": {
    "content_id": "content-999",
    "title": "Mon article",
    "created_at": "2026-01-26T10:05:00Z"
  },
  "timestamp": "2026-01-26T10:05:00Z"
}
```

**Résultat traduit (produit) :**
```json
{
  "résultat_id": "res-111",
  "intention_id": "int-123",
  "demande_id": "dem-789",
  "statut": "SUCCÈS",
  "décision": "ACCEPTÉE",
  "données": {
    "id": "content-999",
    "titre": "Mon article",
    "créé_le": "2026-01-26T10:05:00Z"
  },
  "timestamp": "2026-01-26T10:05:00Z",
  "autorité": "kind_mother"
}
```

### 13.2 Refus par Kind Mother

**Réponse (Kind Mother) :**
```json
{
  "réponse_id": "resp-457",
  "demande_id": "dem-790",
  "statut": "denied",
  "erreurs": [
    {
      "code": "PERMISSION_INSUFFISANTE",
      "message": "L'utilisateur n'a pas la permission d'écrire ce contenu"
    }
  ],
  "timestamp": "2026-01-26T10:06:00Z"
}
```

**Résultat traduit (produit) :**
```json
{
  "résultat_id": "res-112",
  "intention_id": "int-124",
  "demande_id": "dem-790",
  "statut": "REFUSÉ",
  "décision": "REFUSÉE",
  "erreurs": [
    {
      "code": "PERMISSION_INSUFFISANTE",
      "message": "L'utilisateur n'a pas la permission d'écrire ce contenu"
    }
  ],
  "timestamp": "2026-01-26T10:06:00Z",
  "autorité": "kind_mother"
}
```

---

## 14. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface et le protocole que Bonding Brother doit respecter pour s'intégrer avec Kind Mother.

Toute implémentation de l'intégration avec Kind Mother doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice v2.0](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) (Section 2)
- [Authority Delegation Contract v2.0](../authority/BondingBrother%20-%20Authority%20Delegation%20Contract.md)
- [Product-to-Ecosystem Flow v2.0](../flows/BondingBrother%20-%20Product-to-Ecosystem%20Flow.md)
- [Translation Contract v2.0](../intent/BondingBrother%20-%20Translation%20Contract.md)
- KindMother - Documentation Fondatrice v1.0
