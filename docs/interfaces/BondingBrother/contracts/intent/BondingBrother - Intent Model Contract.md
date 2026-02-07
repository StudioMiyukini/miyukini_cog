# BondingBrother — Intent Model Contract

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif

---

## 1. Contexte

Ce document définit le modèle contractuel des intentions dans Bonding Brother. Il spécifie comment les produits expriment leurs intentions, comment ces intentions sont structurées, et comment elles transitent dans le système jusqu'à leur évaluation par une autorité.

**Dépendances :**
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) — Section 6 (Principe d'intention)
- [Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md)

Le modèle d'intention respecte les Lois d'Autonomie Système : les intentions sont acceptées et traitées localement même en mode offline (**LOI-2**), et leur état local est souverain (**LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- La structure canonique d'une intention
- Les types d'intentions supportés
- Le cycle de vie d'une intention (création, transit, évaluation, résolution)
- Les règles de validation structurelle
- Les métadonnées et le contexte associés

Ce document **ne couvre pas** :
- Les règles de traduction (voir [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md))
- Les règles de filtrage (voir [Filtering & Projection Contract](./BondingBrother%20-%20Filtering%20&%20Projection%20Contract.md))
- La gestion des erreurs (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md))

---

## 3. Principe fondamental

> **Une intention est une déclaration de volonté, pas une instruction d'exécution.**

Les produits expriment ce qu'ils souhaitent faire, pas ce qu'ils ordonnent. L'évaluation et la décision appartiennent exclusivement aux autorités.

---

## 4. Structure canonique d'une intention

### 4.1 Schéma de base

Toute intention suit cette structure minimale :

```typescript
interface Intention {
    // Identifiants
    id: IntentionId;                    // Identifiant unique de l'intention
    produit_id: ProduitId;              // Identité du produit émetteur
    
    // Type et contenu
    type: TypeIntention;                 // Type canonique de l'intention
    payload: PayloadSpecifique;          // Données spécifiques au type
    
    // Contexte
    contexte: Contexte;                  // Contexte complet (voir Section 5)
    
    // Métadonnées
    timestamp: Timestamp;                // Moment de création
    version: VersionIntention;           // Version du schéma d'intention
}
```

### 4.2 Identifiants

#### IntentionId

**Type :** UUID v4 ou identifiant unique déterministe

**Caractéristiques :**
- Unique globalement
- Immuable après création
- Traçable dans tous les logs
- Non réutilisable (même après résolution)

**Génération :**
- Par le produit émetteur
- Ou par Bonding Brother si le produit ne fournit pas d'ID

#### ProduitId

**Type :** Identifiant canonique du produit

**Caractéristiques :**
- Identifie de manière unique le produit dans l'écosystème
- Vérifié par StrongFather
- Transmis intégralement aux autorités

---

## 5. Contexte d'une intention

### 5.1 Structure du contexte

Le contexte est l'ensemble des informations nécessaires à l'évaluation par une autorité.

```typescript
interface Contexte {
    // Identités
    produit_id: ProduitId;              // Redondant mais requis pour traçabilité
    utilisateur_id?: UtilisateurId;      // Identité de l'utilisateur (si applicable)
    session_id?: SessionId;              // Identifiant de session
    
    // Permissions déclarées
    permissions_declarees?: Permission[]; // Permissions revendiquées (non validées)
    
    // Environnement
    environnement: Environnement;       // dev, staging, prod
    region?: Region;                     // Zone géographique
    
    // Métadonnées
    metadata?: Map<string, any>;         // Informations complémentaires
    correlation_id?: CorrelationId;      // ID pour traçabilité distribuée
}
```

### 5.2 Règles du contexte

| Code | Règle | Description |
|------|-------|-------------|
| **CONT-01** | Complétude minimale | `produit_id`, `environnement`, `timestamp` obligatoires |
| **CONT-02** | Transmission intégrale | Contexte transmis sans filtrage aux autorités |
| **CONT-03** | Non-validation | BB ne valide pas le contenu, les autorités valident |

---

## 6. Types d'intentions

### 6.1 Taxonomie des types

Les types d'intentions sont organisés par domaine d'autorité :

| Domaine | Type d'intention | Autorité cible | Exemple |
|---------|------------------|---------------|---------|
| Données | `CREATE_CONTENT` | KindMother | Créer un contenu |
| Données | `UPDATE_CONTENT` | KindMother | Modifier un contenu |
| Données | `DELETE_CONTENT` | KindMother | Supprimer un contenu |
| Données | `READ_CONTENT` | KindMother | Lire un contenu |
| Données | `QUERY_CONTENT` | KindMother | Rechercher des contenus |
| Hiérarchie | `CREATE_NODE` | KindMother | Créer un nœud hiérarchique |
| Hiérarchie | `MOVE_NODE` | KindMother | Déplacer un nœud |
| Hiérarchie | `DELETE_NODE` | KindMother | Supprimer un nœud |
| Identité | `AUTHENTICATE` | StrongFather | Authentifier un utilisateur |
| Identité | `AUTHORIZE` | StrongFather | Vérifier une permission |
| Identité | `CREATE_SESSION` | StrongFather | Créer une session |
| Identité | `REVOKE_SESSION` | StrongFather | Révoquer une session |

### 6.2 Règles des types

| Code | Règle | Description |
|------|-------|-------------|
| **TYPE-01** | Types canoniques | Nouveau type = justification + autorité + schéma + version |
| **TYPE-02** | Pas de types génériques | `DO_ACTION` interdit — types spécifiques obligatoires |
| **TYPE-03** | Un type = une autorité | Intentions multi-autorités interdites |

---

## 7. Cycle de vie d'une intention

### 7.1 États d'une intention

Une intention traverse les états suivants dans l'ordre strict :

```
CRÉÉE → VALIDÉE → TRADUITE → FILTRÉE → JOURNALISÉE → TRANSMISE → EN_ATTENTE → ÉVALUÉE → RÉSOLUE
```

| État | Déclencheur | Caractéristiques |
|------|-------------|------------------|
| **CRÉÉE** | Produit soumet une intention | Structure brute, pas encore validée |
| **VALIDÉE** | Validation structurelle réussie | Structure conforme, ID confirmé |
| **TRADUITE** | Traduction réussie | Format adapté à l'autorité |
| **FILTRÉE** | Filtrage d'entrée appliqué | Demandes invalides rejetées |
| **JOURNALISÉE** | Enregistrée dans le journal | Traçable, horodatée, immuable |
| **TRANSMISE** | Transmise à l'autorité | Reçue par l'autorité |
| **EN_ATTENTE** | Autorité évalue | Évaluation en cours |
| **ÉVALUÉE** | Réponse de l'autorité | Décision disponible |
| **RÉSOLUE** | Résultat transmis au produit | Cycle terminé |

### 7.2 États d'erreur

| État | Cause | Action |
|------|-------|--------|
| **REJETÉE** | Validation/traduction/filtrage échoué | Notification au produit |
| **EN_ERREUR** | Transmission échouée | Mode offline possible |
| **ABANDONNÉE** | Timeout/erreur irrécupérable | Journalisation + notification |

### 7.3 Mode offline

En mode offline, les intentions restent en état TRANSMISE ou EN_ATTENTE jusqu'à la reconnexion. Voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md).

---

## 8. Validation structurelle

### 8.1 Règles de validation

| Code | Règle | Action si échec |
|------|-------|-----------------|
| **VAL-01** | Format JSON valide | Rejet immédiat |
| **VAL-02** | Champs obligatoires présents | Rejet immédiat |
| **VAL-03** | Types de données corrects | Rejet immédiat |
| **VAL-04** | Version supportée | Rejet immédiat |
| **VAL-05** | Type reconnu | Rejet immédiat |

### 8.2 Champs obligatoires

| Champ | Type | Description |
|-------|------|-------------|
| `id` | string (UUID) | Généré par BB si absent |
| `produit_id` | string | Identité du produit |
| `type` | string | Type canonique |
| `payload` | object | Données du type |
| `contexte` | object | Contexte complet |
| `timestamp` | string/number | ISO 8601 ou Unix |
| `version` | string | Semver |

### 8.3 Validation NON effectuée par BB

- Contenu sémantique du payload (validation métier)
- Permissions réelles de l'utilisateur (validation par StrongFather)
- Cohérence des données (validation par KindMother)
- Véracité du contexte (validation par les autorités)

---

## 9. Métadonnées et traçabilité

### 9.1 Métadonnées obligatoires

| Métadonnée | Type | Source | Usage |
|------------|------|--------|-------|
| `id` | IntentionId | Produit ou BB | Traçabilité unique |
| `timestamp` | Timestamp | Produit | Ordre chronologique |
| `version` | VersionIntention | Produit | Compatibilité schéma |
| `produit_id` | ProduitId | Produit | Attribution |

### 9.2 Métadonnées optionnelles

| Métadonnée | Type | Usage |
|------------|------|-------|
| `correlation_id` | CorrelationId | Traçabilité distribuée |
| `priorite` | Priorite | Ordre de traitement |
| `timeout` | Duree | Délai maximum |

### 9.3 Règles de traçabilité

| Code | Règle | Description |
|------|-------|-------------|
| **TRACE-01** | Journalisation systématique | Intention + contexte + timestamp + produit |
| **TRACE-02** | Immuabilité | Pas de modification après journalisation |
| **TRACE-03** | Accessibilité | Produit peut consulter son historique |

---

## 10. Exemples

### 10.1 Intention de création de contenu

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Article de blog",
    "contenu": "Contenu de l'article...",
    "auteur_id": "user-123",
    "categorie": "blog"
  },
  "contexte": {
    "produit_id": "miyukini-cms",
    "utilisateur_id": "user-123",
    "session_id": "session-456",
    "environnement": "production",
    "permissions_declarees": ["content:write"]
  },
  "timestamp": "2026-01-28T10:30:00Z",
  "version": "2.0.0"
}
```

### 10.2 Intention d'autorisation

```json
{
  "id": "660e8400-e29b-41d4-a716-446655440001",
  "produit_id": "miyukini-cms",
  "type": "AUTHORIZE",
  "payload": {
    "action": "content:delete",
    "ressource_id": "content-789",
    "utilisateur_id": "user-123"
  },
  "contexte": {
    "produit_id": "miyukini-cms",
    "utilisateur_id": "user-123",
    "session_id": "session-456",
    "environnement": "production"
  },
  "timestamp": "2026-01-28T10:31:00Z",
  "version": "2.0.0"
}
```

---

## 11. Contraintes et limites

### 11.1 Taille maximale

| Élément | Limite par défaut |
|---------|-------------------|
| Payload | 1 MB |
| Contexte | 100 KB |

### 11.2 Délais

| Élément | Valeur par défaut |
|---------|-------------------|
| Timeout résolution | Configurable |
| Expiration non résolue | 24 heures |

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit le modèle d'intention que tous les produits doivent respecter pour interagir avec Bonding Brother.

Toute intention soumise à Bonding Brother doit respecter ce contrat. Toute violation entraîne un rejet avec code d'erreur approprié.

---

## Navigation

- [Index BondingBrother](../../_index.md)
- [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md)
- [Filtering & Projection Contract](./BondingBrother%20-%20Filtering%20&%20Projection%20Contract.md)

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** Documentation Fondatrice v2.0, Architecture & Flows v2.0
