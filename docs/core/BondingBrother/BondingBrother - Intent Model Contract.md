# BondingBrother - Intent Model Contract

## 1. Contexte

Ce document définit le modèle contractuel des intentions dans Bonding Brother. Il spécifie comment les produits expriment leurs intentions, comment ces intentions sont structurées, et comment elles transitent dans le système jusqu'à leur évaluation par une autorité.

Ce document complète la Section 4 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur l'[Architecture et Composants](./BondingBrother%20-%20Architecture%20et%20Composants.md) pour définir les structures techniques précises.

Le modèle d'intention respecte les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md) : les intentions sont acceptées et traitées localement même en mode offline (**LOI-2**), et leur état local est souverain (**LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- La structure canonique d'une intention
- Les types d'intentions supportés
- Le cycle de vie d'une intention (création, transit, évaluation, résolution)
- Les règles de validation structurelle
- Les métadonnées et le contexte associés

Ce document **ne couvre pas** :
- Les règles de traduction (voir Translation Contract)
- Les règles de filtrage (voir Filtering & Projection Contract)
- La gestion des erreurs (voir Error & Rejection Model)
- Les détails d'implémentation

---

## 3. Principe fondamental

**Une intention est une déclaration de volonté, pas une instruction d'exécution.**

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
    payload: PayloadSpécifique;          // Données spécifiques au type
    
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
- Vérifié par Strong Father
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
    permissions_déclarées?: Permission[]; // Permissions revendiquées (non validées)
    
    // Environnement
    environnement: Environnement;       // dev, staging, prod
    région?: Région;                     // Zone géographique
    
    // Métadonnées
    metadata?: Map<string, any>;         // Informations complémentaires
    corrélation_id?: CorrélationId;      // ID pour traçabilité distribuée
}
```

### 5.2 Règles du contexte

**Règle CONT-01 : Complétude minimale**

Le contexte doit contenir au minimum :
- `produit_id` (obligatoire)
- `environnement` (obligatoire)
- `timestamp` (obligatoire, dans l'intention)

**Règle CONT-02 : Transmission intégrale**

Bonding Brother transmet le contexte intégralement aux autorités, sans filtrage ni modification.

**Règle CONT-03 : Non-validation**

Bonding Brother ne valide pas le contenu du contexte. La validation appartient aux autorités.

---

## 6. Types d'intentions

### 6.1 Taxonomie des types

Les types d'intentions sont organisés par domaine d'autorité :

| Domaine | Type d'intention | Autorité cible | Exemple |
|---------|------------------|---------------|---------|
| Données | `CREATE_CONTENT` | Kind Mother | Créer un contenu |
| Données | `UPDATE_CONTENT` | Kind Mother | Modifier un contenu |
| Données | `DELETE_CONTENT` | Kind Mother | Supprimer un contenu |
| Données | `READ_CONTENT` | Kind Mother | Lire un contenu |
| Données | `QUERY_CONTENT` | Kind Mother | Rechercher des contenus |
| Hiérarchie | `CREATE_NODE` | Kind Mother | Créer un nœud hiérarchique |
| Hiérarchie | `MOVE_NODE` | Kind Mother | Déplacer un nœud |
| Hiérarchie | `DELETE_NODE` | Kind Mother | Supprimer un nœud |
| Identité | `AUTHENTICATE` | Strong Father | Authentifier un utilisateur |
| Identité | `AUTHORIZE` | Strong Father | Vérifier une permission |
| Identité | `CREATE_SESSION` | Strong Father | Créer une session |
| Identité | `REVOKE_SESSION` | Strong Father | Révoquer une session |

### 6.2 Extension des types

**Règle TYPE-01 : Types canoniques**

Les types d'intentions sont définis de manière canonique. Un nouveau type nécessite :
- Une justification métier
- Une autorité cible identifiée
- Un schéma de payload défini
- Une version de schéma

**Règle TYPE-02 : Pas de types génériques**

Les types génériques (ex: `DO_ACTION`) sont interdits. Chaque intention doit avoir un type spécifique.

**Règle TYPE-03 : Un type = une autorité**

Un type d'intention cible une et une seule autorité. Les intentions multi-autorités sont interdites.

---

## 7. Cycle de vie d'une intention

### 7.1 États d'une intention

Une intention traverse les états suivants dans l'ordre strict :

```
CRÉÉE → VALIDÉE → TRADUITE → FILTRÉE → JOURNALISÉE → TRANSMISE → EN_ATTENTE → ÉVALUÉE → RÉSOLUE
```

#### CRÉÉE

**Déclencheur :** Le produit soumet une intention à Bonding Brother.

**Caractéristiques :**
- Structure brute reçue
- Pas encore validée
- Pas encore traçable dans le journal

**Durée :** Instantanée

#### VALIDÉE

**Déclencheur :** Validation structurelle réussie (format, champs obligatoires, types).

**Caractéristiques :**
- Structure conforme au schéma
- Prête pour traitement
- ID d'intention confirmé

**Durée :** Instantanée

**Transition :** Si validation échoue → État REJETÉE (voir Error & Rejection Model)

#### TRADUITE

**Déclencheur :** Traduction réussie de l'intention en demande pour l'autorité.

**Caractéristiques :**
- Format adapté au vocabulaire de l'autorité
- Sémantique préservée
- Prête pour transmission

**Durée :** Instantanée

**Transition :** Si traduction échoue → État REJETÉE

#### FILTRÉE

**Déclencheur :** Application des règles de filtrage d'entrée.

**Caractéristiques :**
- Demandes invalides rejetées
- Demandes valides prêtes pour transmission

**Durée :** Instantanée

**Transition :** Si filtrage rejette → État REJETÉE

#### JOURNALISÉE

**Déclencheur :** Enregistrement dans le journal d'audit.

**Caractéristiques :**
- Traçable dans le journal
- Horodatée
- Immuable

**Durée :** Instantanée

#### TRANSMISE

**Déclencheur :** Transmission réussie à l'autorité cible.

**Caractéristiques :**
- Reçue par l'autorité
- En attente d'évaluation

**Durée :** Variable (dépend de l'autorité)

**Transition :** Si transmission échoue → État EN_ERREUR (mode offline possible)

#### EN_ATTENTE

**Déclencheur :** Autorité a reçu la demande et l'évalue.

**Caractéristiques :**
- Évaluation en cours
- Pas de résultat disponible

**Durée :** Variable (dépend de l'autorité)

#### ÉVALUÉE

**Déclencheur :** Autorité a fourni une réponse (acceptée ou refusée).

**Caractéristiques :**
- Décision de l'autorité disponible
- Réponse reçue par Bonding Brother
- Prête pour traduction et filtrage

**Durée :** Instantanée

#### RÉSOLUE

**Déclencheur :** Résultat filtré transmis au produit.

**Caractéristiques :**
- Cycle complet terminé
- Produit informé
- Journal mis à jour

**Durée :** Instantanée

### 7.2 États d'erreur

Les états d'erreur sont détaillés dans le Error & Rejection Model. États possibles :
- REJETÉE (validation, traduction, ou filtrage échoué)
- EN_ERREUR (transmission échouée, autorité indisponible)
- ABANDONNÉE (timeout, erreur irrécupérable)

### 7.3 Mode offline

En mode offline, les intentions peuvent rester dans l'état TRANSMISE ou EN_ATTENTE jusqu'à la reconnexion. Voir Offline & Deferred Authority Contract.

---

## 8. Validation structurelle

### 8.1 Règles de validation

**Règle VAL-01 : Format JSON valide**

L'intention doit être un JSON valide, parsable sans erreur.

**Règle VAL-02 : Champs obligatoires**

Tous les champs marqués comme obligatoires doivent être présents :
- `id` (ou généré par BB)
- `produit_id`
- `type`
- `payload`
- `contexte`
- `timestamp`
- `version`

**Règle VAL-03 : Types de données**

Chaque champ doit respecter son type déclaré :
- `id` : string (UUID ou identifiant)
- `produit_id` : string
- `type` : string (type canonique)
- `payload` : object (structure dépend du type)
- `contexte` : object (structure définie Section 5)
- `timestamp` : string (ISO 8601) ou number (Unix timestamp)
- `version` : string (semver)

**Règle VAL-04 : Version supportée**

La version du schéma doit être supportée par Bonding Brother. Les versions obsolètes sont rejetées.

**Règle VAL-05 : Type reconnu**

Le type d'intention doit être un type canonique reconnu par Bonding Brother.

### 8.2 Validation non effectuée

Bonding Brother **ne valide pas** :
- Le contenu sémantique du payload (validation métier)
- Les permissions réelles de l'utilisateur (validation par Strong Father)
- La cohérence des données (validation par Kind Mother)
- La véracité du contexte (validation par les autorités)

---

## 9. Métadonnées et traçabilité

### 9.1 Métadonnées obligatoires

Chaque intention doit contenir :

| Métadonnée | Type | Source | Usage |
|------------|------|--------|-------|
| `id` | IntentionId | Produit ou BB | Traçabilité unique |
| `timestamp` | Timestamp | Produit | Ordre chronologique |
| `version` | VersionIntention | Produit | Compatibilité schéma |
| `produit_id` | ProduitId | Produit | Attribution |

### 9.2 Métadonnées optionnelles

| Métadonnée | Type | Source | Usage |
|------------|------|--------|-------|
| `corrélation_id` | CorrélationId | Produit | Traçabilité distribuée |
| `priorité` | Priorité | Produit | Ordre de traitement (si supporté) |
| `timeout` | Durée | Produit | Délai maximum d'attente |

### 9.3 Traçabilité

**Règle TRACE-01 : Journalisation systématique**

Toute intention est journalisée avec :
- L'intention complète (structure + payload)
- Le contexte complet
- Le timestamp de réception
- L'identité du produit

**Règle TRACE-02 : Immuabilité**

Une fois journalisée, une intention ne peut être modifiée. Les corrections se font par nouvelle intention.

**Règle TRACE-03 : Accessibilité**

Un produit peut consulter l'historique de ses propres intentions via l'API de traçabilité.

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
    "catégorie": "blog"
  },
  "contexte": {
    "produit_id": "miyukini-cms",
    "utilisateur_id": "user-123",
    "session_id": "session-456",
    "environnement": "production",
    "permissions_déclarées": ["content:write"],
    "metadata": {
      "user_agent": "MiyukiniCMS/1.0",
      "ip_address": "192.168.1.1"
    }
  },
  "timestamp": "2026-01-26T10:30:00Z",
  "version": "1.0.0"
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
  "timestamp": "2026-01-26T10:31:00Z",
  "version": "1.0.0"
}
```

---

## 11. Contraintes et limites

### 11.1 Taille maximale

**Règle LIM-01 : Taille du payload**

La taille maximale du payload est définie par configuration. Par défaut : 1 MB.

**Règle LIM-02 : Taille du contexte**

La taille maximale du contexte est définie par configuration. Par défaut : 100 KB.

### 11.2 Délais

**Règle LIM-03 : Timeout**

Si une intention n'est pas résolue dans le délai spécifié (ou délai par défaut), elle passe en état ABANDONNÉE.

**Règle LIM-04 : Expiration**

Les intentions non résolues expirent après une durée configurable (par défaut : 24 heures).

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit le modèle d'intention que tous les produits doivent respecter pour interagir avec Bonding Brother.

Toute intention soumise à Bonding Brother doit respecter ce contrat. Toute violation entraîne un rejet avec code d'erreur approprié.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 4)
- Architecture et Composants v1.0
- Glossaire et Terminologie v1.0
