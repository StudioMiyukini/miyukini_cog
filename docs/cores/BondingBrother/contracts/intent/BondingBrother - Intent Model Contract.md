# BondingBrother â€” Intent Model Contract

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif

---

## 1. Contexte

Ce document dÃ©finit le modÃ¨le contractuel des intentions dans Bonding Brother. Il spÃ©cifie comment les produits expriment leurs intentions, comment ces intentions sont structurÃ©es, et comment elles transitent dans le systÃ¨me jusqu'Ã  leur Ã©valuation par une autoritÃ©.

**DÃ©pendances :**
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) â€” Section 6 (Principe d'intention)
- [Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md)
- [Miyukini Conceptual References - Lois Autonomie Systeme](..//..//..//..//miyukini-webway-system//reference//_index.md)

Le modÃ¨le d'intention respecte les Lois d'Autonomie SystÃ¨me : les intentions sont acceptÃ©es et traitÃ©es localement mÃªme en mode offline (**LOI-2**), et leur Ã©tat local est souverain (**LOI-3**).

## 2. PortÃ©e / Scope

Ce document couvre :
- La structure canonique d'une intention
- Les types d'intentions supportÃ©s
- Le cycle de vie d'une intention (crÃ©ation, transit, Ã©valuation, rÃ©solution)
- Les rÃ¨gles de validation structurelle
- Les mÃ©tadonnÃ©es et le contexte associÃ©s

Ce document **ne couvre pas** :
- Les rÃ¨gles de traduction (voir [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md))
- Les rÃ¨gles de filtrage (voir [Filtering & Projection Contract](./BondingBrother%20-%20Filtering%20&%20Projection%20Contract.md))
- La gestion des erreurs (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20&%20Rejection%20Model.md))

---

## 3. Principe fondamental

> **Une intention est une dÃ©claration de volontÃ©, pas une instruction d'exÃ©cution.**

Les produits expriment ce qu'ils souhaitent faire, pas ce qu'ils ordonnent. L'Ã©valuation et la dÃ©cision appartiennent exclusivement aux autoritÃ©s.

---

## 4. Structure canonique d'une intention

### 4.1 SchÃ©ma de base

Toute intention suit cette structure minimale :

```typescript
interface Intention {
    // Identifiants
    id: IntentionId;                    // Identifiant unique de l'intention
    produit_id: ProduitId;              // IdentitÃ© du produit Ã©metteur
    
    // Type et contenu
    type: TypeIntention;                 // Type canonique de l'intention
    payload: PayloadSpecifique;          // DonnÃ©es spÃ©cifiques au type
    
    // Contexte
    contexte: Contexte;                  // Contexte complet (voir Section 5)
    
    // MÃ©tadonnÃ©es
    timestamp: Timestamp;                // Moment de crÃ©ation
    version: VersionIntention;           // Version du schÃ©ma d'intention
}
```

### 4.2 Identifiants

#### IntentionId

**Type :** UUID v4 ou identifiant unique dÃ©terministe

**CaractÃ©ristiques :**
- Unique globalement
- Immuable aprÃ¨s crÃ©ation
- TraÃ§able dans tous les logs
- Non rÃ©utilisable (mÃªme aprÃ¨s rÃ©solution)

**GÃ©nÃ©ration :**
- Par le produit Ã©metteur
- Ou par Bonding Brother si le produit ne fournit pas d'ID

#### ProduitId

**Type :** Identifiant canonique du produit

**CaractÃ©ristiques :**
- Identifie de maniÃ¨re unique le produit dans l'Ã©cosystÃ¨me
- VÃ©rifiÃ© par StrongFather
- Transmis intÃ©gralement aux autoritÃ©s

---

## 5. Contexte d'une intention

### 5.1 Structure du contexte

Le contexte est l'ensemble des informations nÃ©cessaires Ã  l'Ã©valuation par une autoritÃ©.

```typescript
interface Contexte {
    // IdentitÃ©s
    produit_id: ProduitId;              // Redondant mais requis pour traÃ§abilitÃ©
    utilisateur_id?: UtilisateurId;      // IdentitÃ© de l'utilisateur (si applicable)
    session_id?: SessionId;              // Identifiant de session
    
    // Permissions dÃ©clarÃ©es
    permissions_declarees?: Permission[]; // Permissions revendiquÃ©es (non validÃ©es)
    
    // Environnement
    environnement: Environnement;       // dev, staging, prod
    region?: Region;                     // Zone gÃ©ographique
    
    // MÃ©tadonnÃ©es
    metadata?: Map<string, any>;         // Informations complÃ©mentaires
    correlation_id?: CorrelationId;      // ID pour traÃ§abilitÃ© distribuÃ©e
}
```

### 5.2 RÃ¨gles du contexte

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **CONT-01** | ComplÃ©tude minimale | `produit_id`, `environnement`, `timestamp` obligatoires |
| **CONT-02** | Transmission intÃ©grale | Contexte transmis sans filtrage aux autoritÃ©s |
| **CONT-03** | Non-validation | BB ne valide pas le contenu, les autoritÃ©s valident |

---

## 6. Types d'intentions

### 6.1 Taxonomie des types

Les types d'intentions sont organisÃ©s par domaine d'autoritÃ© :

| Domaine | Type d'intention | AutoritÃ© cible | Exemple |
|---------|------------------|---------------|---------|
| DonnÃ©es | `CREATE_CONTENT` | KindMother | CrÃ©er un contenu |
| DonnÃ©es | `UPDATE_CONTENT` | KindMother | Modifier un contenu |
| DonnÃ©es | `DELETE_CONTENT` | KindMother | Supprimer un contenu |
| DonnÃ©es | `READ_CONTENT` | KindMother | Lire un contenu |
| DonnÃ©es | `QUERY_CONTENT` | KindMother | Rechercher des contenus |
| HiÃ©rarchie | `CREATE_NODE` | KindMother | CrÃ©er un nÅ“ud hiÃ©rarchique |
| HiÃ©rarchie | `MOVE_NODE` | KindMother | DÃ©placer un nÅ“ud |
| HiÃ©rarchie | `DELETE_NODE` | KindMother | Supprimer un nÅ“ud |
| IdentitÃ© | `AUTHENTICATE` | StrongFather | Authentifier un utilisateur |
| IdentitÃ© | `AUTHORIZE` | StrongFather | VÃ©rifier une permission |
| IdentitÃ© | `CREATE_SESSION` | StrongFather | CrÃ©er une session |
| IdentitÃ© | `REVOKE_SESSION` | StrongFather | RÃ©voquer une session |

### 6.2 RÃ¨gles des types

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **TYPE-01** | Types canoniques | Nouveau type = justification + autoritÃ© + schÃ©ma + version |
| **TYPE-02** | Pas de types gÃ©nÃ©riques | `DO_ACTION` interdit â€” types spÃ©cifiques obligatoires |
| **TYPE-03** | Un type = une autoritÃ© | Intentions multi-autoritÃ©s interdites |

---

## 7. Cycle de vie d'une intention

### 7.1 Ã‰tats d'une intention

Une intention traverse les Ã©tats suivants dans l'ordre strict :

```
CRÃ‰Ã‰E â†’ VALIDÃ‰E â†’ TRADUITE â†’ FILTRÃ‰E â†’ JOURNALISÃ‰E â†’ TRANSMISE â†’ EN_ATTENTE â†’ Ã‰VALUÃ‰E â†’ RÃ‰SOLUE
```

| Ã‰tat | DÃ©clencheur | CaractÃ©ristiques |
|------|-------------|------------------|
| **CRÃ‰Ã‰E** | Produit soumet une intention | Structure brute, pas encore validÃ©e |
| **VALIDÃ‰E** | Validation structurelle rÃ©ussie | Structure conforme, ID confirmÃ© |
| **TRADUITE** | Traduction rÃ©ussie | Format adaptÃ© Ã  l'autoritÃ© |
| **FILTRÃ‰E** | Filtrage d'entrÃ©e appliquÃ© | Demandes invalides rejetÃ©es |
| **JOURNALISÃ‰E** | EnregistrÃ©e dans le journal | TraÃ§able, horodatÃ©e, immuable |
| **TRANSMISE** | Transmise Ã  l'autoritÃ© | ReÃ§ue par l'autoritÃ© |
| **EN_ATTENTE** | AutoritÃ© Ã©value | Ã‰valuation en cours |
| **Ã‰VALUÃ‰E** | RÃ©ponse de l'autoritÃ© | DÃ©cision disponible |
| **RÃ‰SOLUE** | RÃ©sultat transmis au produit | Cycle terminÃ© |

### 7.2 Ã‰tats d'erreur

| Ã‰tat | Cause | Action |
|------|-------|--------|
| **REJETÃ‰E** | Validation/traduction/filtrage Ã©chouÃ© | Notification au produit |
| **EN_ERREUR** | Transmission Ã©chouÃ©e | Mode offline possible |
| **ABANDONNÃ‰E** | Timeout/erreur irrÃ©cupÃ©rable | Journalisation + notification |

### 7.3 Mode offline

En mode offline, les intentions restent en Ã©tat TRANSMISE ou EN_ATTENTE jusqu'Ã  la reconnexion. Voir [Offline & Deferred Authority Contract](../offline/BondingBrother%20-%20Offline%20&%20Deferred%20Authority%20Contract.md).

---

## 8. Validation structurelle

### 8.1 RÃ¨gles de validation

| Code | RÃ¨gle | Action si Ã©chec |
|------|-------|-----------------|
| **VAL-01** | Format JSON valide | Rejet immÃ©diat |
| **VAL-02** | Champs obligatoires prÃ©sents | Rejet immÃ©diat |
| **VAL-03** | Types de donnÃ©es corrects | Rejet immÃ©diat |
| **VAL-04** | Version supportÃ©e | Rejet immÃ©diat |
| **VAL-05** | Type reconnu | Rejet immÃ©diat |

### 8.2 Champs obligatoires

| Champ | Type | Description |
|-------|------|-------------|
| `id` | string (UUID) | GÃ©nÃ©rÃ© par BB si absent |
| `produit_id` | string | IdentitÃ© du produit |
| `type` | string | Type canonique |
| `payload` | object | DonnÃ©es du type |
| `contexte` | object | Contexte complet |
| `timestamp` | string/number | ISO 8601 ou Unix |
| `version` | string | Semver |

### 8.3 Validation NON effectuÃ©e par BB

- Contenu sÃ©mantique du payload (validation mÃ©tier)
- Permissions rÃ©elles de l'utilisateur (validation par StrongFather)
- CohÃ©rence des donnÃ©es (validation par KindMother)
- VÃ©racitÃ© du contexte (validation par les autoritÃ©s)

---

## 9. MÃ©tadonnÃ©es et traÃ§abilitÃ©

### 9.1 MÃ©tadonnÃ©es obligatoires

| MÃ©tadonnÃ©e | Type | Source | Usage |
|------------|------|--------|-------|
| `id` | IntentionId | Produit ou BB | TraÃ§abilitÃ© unique |
| `timestamp` | Timestamp | Produit | Ordre chronologique |
| `version` | VersionIntention | Produit | CompatibilitÃ© schÃ©ma |
| `produit_id` | ProduitId | Produit | Attribution |

### 9.2 MÃ©tadonnÃ©es optionnelles

| MÃ©tadonnÃ©e | Type | Usage |
|------------|------|-------|
| `correlation_id` | CorrelationId | TraÃ§abilitÃ© distribuÃ©e |
| `priorite` | Priorite | Ordre de traitement |
| `timeout` | Duree | DÃ©lai maximum |

### 9.3 RÃ¨gles de traÃ§abilitÃ©

| Code | RÃ¨gle | Description |
|------|-------|-------------|
| **TRACE-01** | Journalisation systÃ©matique | Intention + contexte + timestamp + produit |
| **TRACE-02** | ImmuabilitÃ© | Pas de modification aprÃ¨s journalisation |
| **TRACE-03** | AccessibilitÃ© | Produit peut consulter son historique |

---

## 10. Exemples

### 10.1 Intention de crÃ©ation de contenu

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

| Ã‰lÃ©ment | Limite par dÃ©faut |
|---------|-------------------|
| Payload | 1 MB |
| Contexte | 100 KB |

### 11.2 DÃ©lais

| Ã‰lÃ©ment | Valeur par dÃ©faut |
|---------|-------------------|
| Timeout rÃ©solution | Configurable |
| Expiration non rÃ©solue | 24 heures |

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit le modÃ¨le d'intention que tous les produits doivent respecter pour interagir avec Bonding Brother.

Toute intention soumise Ã  Bonding Brother doit respecter ce contrat. Toute violation entraÃ®ne un rejet avec code d'erreur appropriÃ©.

---

## Navigation

- [Index BondingBrother](../../_index.md)
- [Translation Contract](./BondingBrother%20-%20Translation%20Contract.md)
- [Filtering & Projection Contract](./BondingBrother%20-%20Filtering%20&%20Projection%20Contract.md)

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** Documentation Fondatrice v2.0, Architecture & Flows v2.0

