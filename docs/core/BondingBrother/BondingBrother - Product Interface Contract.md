# BondingBrother - Product Interface Contract

## 1. Contexte

Ce document définit le contrat d'interface stable que Bonding Brother expose aux produits de l'écosystème. Il spécifie les interfaces, les formats, les protocoles, et les garanties que les produits peuvent s'attendre à recevoir de Bonding Brother.

Ce document complète la Section 7 de la [Documentation Fondatrice](./BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour définir l'interface complète.

L'interface respecte les [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md) : elle fonctionne même en mode offline (**LOI-2**), et les intentions sont acceptées et traitées localement même sans connexion aux autorités.

## 2. Portée / Scope

Ce document couvre :
- L'interface de soumission d'intentions
- L'interface de consommation de résultats
- L'interface d'abonnement aux notifications
- Les formats de données standardisés
- Les protocoles de communication
- Les garanties de stabilité et de compatibilité
- Les règles de versionnement

Ce document **ne couvre pas** :
- Les détails du flux Produit → Écosystème (voir Product-to-Ecosystem Flow)
- Les détails du flux Écosystème → Produit (voir Ecosystem-to-Product Flow)
- Les règles d'adaptation des produits (voir Product Adaptation Rules)
- Les mécanismes d'extension (voir Extension & Specialization Contract)

---

## 3. Principe fondamental

**Bonding Brother offre une interface stable aux produits. Cette stabilité est fondamentale : les produits s'adaptent à Bonding Brother, jamais l'inverse.**

L'interface est versionnée, documentée, et évolue selon des règles strictes de compatibilité. Les produits peuvent s'appuyer sur cette interface sans crainte de rupture.

---

## 4. Interface de soumission d'intentions

### 4.1 Point d'entrée

**Interface :** `IIntentSubmission`

**Endpoint :** `/api/v1/intentions`

**Méthode :** `POST`

**Format :** JSON

### 4.2 Structure de l'intention

```typescript
interface IntentionSubmission {
    // Identifiants
    id?: IntentionId;                    // Optionnel (généré si absent)
    produit_id: ProduitId;               // Obligatoire
    
    // Type et contenu
    type: TypeIntention;                  // Obligatoire (type canonique)
    payload: PayloadSpécifique;         // Obligatoire (structure dépend du type)
    
    // Contexte
    contexte: Contexte;                   // Obligatoire (voir Section 5)
    
    // Métadonnées
    timestamp?: Timestamp;                // Optionnel (généré si absent)
    version?: VersionIntention;          // Optionnel (défaut: dernière version)
    corrélation_id?: CorrélationId;      // Optionnel (pour traçabilité distribuée)
}
```

### 4.3 Règles de soumission

**Règle SUB-01 : Format JSON valide**

L'intention doit être un JSON valide, parsable sans erreur.

**Règle SUB-02 : Champs obligatoires**

Tous les champs marqués comme obligatoires doivent être présents :
- `produit_id`
- `type`
- `payload`
- `contexte`

**Règle SUB-03 : Types de données**

Chaque champ doit respecter son type déclaré.

**Règle SUB-04 : Version supportée**

La version du schéma doit être supportée par Bonding Brother. Les versions obsolètes sont rejetées.

**Règle SUB-05 : Type reconnu**

Le type d'intention doit être un type canonique reconnu par Bonding Brother.

### 4.4 Réponse à la soumission

**Format de réponse :**

```typescript
interface SubmissionResponse {
    // Identifiants
    intention_id: IntentionId;           // ID de l'intention (confirmé ou généré)
    
    // Statut
    statut: StatutSoumission;            // ACCEPTÉE, REJETÉE, EN_ERREUR
    
    // Métadonnées
    timestamp: Timestamp;                 // Timestamp de réception
    version: VersionIntention;            // Version utilisée
    
    // Erreurs (si applicable)
    erreurs?: ErreurSoumission[];        // Erreurs de validation
}
```

**Statuts possibles :**
- `ACCEPTÉE` : Intention acceptée et en cours de traitement
- `REJETÉE` : Intention rejetée (validation échouée)
- `EN_ERREUR` : Erreur technique lors de la réception

**Règle SUB-06 : Réponse immédiate**

La réponse à la soumission est immédiate (synchronisée). Elle indique uniquement l'acceptation ou le rejet de la structure, pas le résultat de l'intention.

**Règle SUB-07 : Pas de résultat dans la réponse**

La réponse de soumission ne contient pas le résultat de l'intention. Le résultat arrive via l'interface de consommation (asynchrone).

---

## 5. Interface de consommation de résultats

### 5.1 Point de réception

**Interface :** `IResultConsumption`

**Mécanisme :** Callback ou polling (selon configuration)

**Format :** JSON

### 5.2 Structure du résultat

```typescript
interface Résultat {
    // Identifiants
    résultat_id: RésultatId;             // ID unique du résultat
    intention_id: IntentionId;           // ID de l'intention source
    demande_id?: DemandeId;              // ID de la demande (traçabilité)
    
    // Statut
    statut: StatutRésultat;              // SUCCÈS, REFUSÉ, ERREUR
    décision: DécisionAutorité;          // Décision de l'autorité (traduite)
    
    // Données (si applicable)
    données?: DonnéesTraduites;          // Données traduites pour le produit
    
    // Erreurs (si applicable)
    erreurs?: ErreurTraduite[];          // Erreurs dans le vocabulaire produit
    
    // Métadonnées
    timestamp: Timestamp;                 // Timestamp de la réponse
    autorité: AutoritéId;                // Autorité qui a répondu (Kind Mother ou Strong Father)
    durée_traitement?: Durée;             // Durée de traitement (si disponible)
}
```

### 5.3 Statuts de résultat

**Statuts possibles :**

| Statut | Signification | Données présentes |
|--------|---------------|-------------------|
| `SUCCÈS` | Intention acceptée et exécutée | Oui (données de résultat) |
| `REFUSÉ` | Intention refusée par l'autorité | Non (raison du refus) |
| `ERREUR` | Erreur technique ou métier | Non (détails de l'erreur) |
| `TIMEOUT` | Délai d'attente dépassé | Non |
| `ABANDONNÉE` | Intention abandonnée | Non (raison) |

**Règle RES-01 : Un résultat par intention**

Chaque intention acceptée reçoit exactement un résultat (succès, refus, ou erreur).

**Règle RES-02 : Résultat asynchrone**

Le résultat arrive de manière asynchrone, après l'évaluation par l'autorité.

**Règle RES-03 : Ordre préservé**

Pour une même intention, l'ordre des résultats est préservé (FIFO).

**Règle RES-04 : Résultat complet**

Le résultat contient toutes les informations nécessaires et autorisées pour le produit.

### 5.4 Mécanismes de réception

**Option 1 : Callback (recommandé)**

Le produit fournit une URL de callback lors de la soumission. Bonding Brother appelle cette URL avec le résultat.

**Option 2 : Polling**

Le produit interroge régulièrement Bonding Brother pour récupérer les résultats.

**Option 3 : Webhook**

Le produit s'abonne à un webhook pour recevoir les résultats.

**Règle RES-05 : Mécanisme configurable**

Le mécanisme de réception est configurable par produit, avec callback comme option par défaut.

---

## 6. Interface d'abonnement aux notifications

### 6.1 Point d'abonnement

**Interface :** `INotificationSubscription`

**Endpoint :** `/api/v1/notifications/subscribe`

**Méthode :** `POST`

**Format :** JSON

### 6.2 Structure de l'abonnement

```typescript
interface AbonnementNotification {
    // Identifiants
    produit_id: ProduitId;               // Obligatoire
    abonnement_id?: AbonnementId;         // Optionnel (généré si absent)
    
    // Types de notifications
    types: TypeNotification[];           // Obligatoire (types souhaités)
    
    // Filtres
    filtres?: FiltresNotification;        // Optionnel (filtres spécifiques)
    
    // Destination
    callback_url: URL;                   // Obligatoire (URL de réception)
    
    // Métadonnées
    timestamp?: Timestamp;                // Optionnel
}
```

### 6.3 Types de notifications

**Types disponibles :**

| Type | Description | Déclencheur |
|------|-------------|-------------|
| `CONTENT_CREATED` | Contenu créé | Kind Mother crée un contenu |
| `CONTENT_UPDATED` | Contenu modifié | Kind Mother modifie un contenu |
| `CONTENT_DELETED` | Contenu supprimé | Kind Mother supprime un contenu |
| `HIERARCHY_CHANGED` | Hiérarchie modifiée | Kind Mother modifie la hiérarchie |
| `PERMISSION_CHANGED` | Permission modifiée | Strong Father modifie une permission |
| `SESSION_REVOKED` | Session révoquée | Strong Father révoque une session |
| `SYSTEM_EVENT` | Événement système | Événement système |

**Règle NOTIF-01 : Types canoniques**

Les types de notifications sont définis de manière canonique. Un nouveau type nécessite une version majeure.

**Règle NOTIF-02 : Filtres optionnels**

Les filtres permettent de restreindre les notifications reçues (ex: uniquement pour certaines ressources).

### 6.4 Structure de la notification

```typescript
interface Notification {
    // Identifiants
    notification_id: NotificationId;      // ID unique de la notification
    abonnement_id: AbonnementId;         // ID de l'abonnement
    
    // Type et contenu
    type: TypeNotification;               // Type de notification
    données: DonnéesNotification;         // Données de la notification
    
    // Métadonnées
    timestamp: Timestamp;                 // Timestamp de l'événement
    autorité: AutoritéId;                 // Autorité source (Kind Mother ou Strong Father)
    corrélation_id?: CorrélationId;       // ID de corrélation (si applicable)
}
```

**Règle NOTIF-03 : Notification asynchrone**

Les notifications sont envoyées de manière asynchrone, sans garantie d'ordre (sauf pour les notifications corrélées).

**Règle NOTIF-04 : Notification unique**

Chaque événement génère une notification unique, envoyée à tous les produits abonnés concernés.

**Règle NOTIF-05 : Retry en cas d'échec**

En cas d'échec de livraison, Bonding Brother retente selon une politique définie.

---

## 7. Formats de données standardisés

### 7.1 Format JSON

**Règle FORMAT-01 : JSON strict**

Tous les échanges utilisent JSON strict, avec validation de schéma.

**Règle FORMAT-02 : Encodage UTF-8**

Tous les textes sont encodés en UTF-8.

**Règle FORMAT-03 : Dates ISO 8601**

Toutes les dates sont au format ISO 8601 : `YYYY-MM-DDTHH:mm:ssZ`

### 7.2 Types de données

**Types primitifs supportés :**
- `string` : Chaîne de caractères UTF-8
- `number` : Nombre (entier ou décimal)
- `boolean` : Booléen
- `null` : Valeur nulle

**Types complexes supportés :**
- `object` : Objet JSON
- `array` : Tableau JSON

**Règle FORMAT-04 : Types stricts**

Les types de données sont stricts. Pas de conversion implicite.

### 7.3 Vocabulaire canonique

**Règle FORMAT-05 : Vocabulaire standard**

Tous les termes utilisés dans l'interface suivent le vocabulaire canonique défini dans le Glossaire et Terminologie.

**Règle FORMAT-06 : Pas de synonymes**

Les synonymes sont interdits. Un seul terme par concept.

---

## 8. Protocoles de communication

### 8.1 Protocole HTTP/HTTPS

**Règle PROTO-01 : HTTPS obligatoire**

Toutes les communications utilisent HTTPS en production. HTTP est autorisé uniquement en développement.

**Règle PROTO-02 : Version HTTP**

HTTP/1.1 ou HTTP/2 sont supportés. HTTP/3 est supporté si disponible.

**Règle PROTO-03 : Méthodes REST**

Les méthodes REST standard sont utilisées :
- `POST` : Création (soumission d'intention, abonnement)
- `GET` : Lecture (polling de résultats)
- `DELETE` : Suppression (désabonnement)

### 8.2 Authentification

**Règle PROTO-04 : Authentification requise**

Toutes les requêtes doivent être authentifiées. Le mécanisme d'authentification est défini par Strong Father.

**Règle PROTO-05 : Tokens**

Les tokens d'authentification sont fournis via les en-têtes HTTP standard (`Authorization: Bearer <token>`).

### 8.3 Gestion des erreurs HTTP

**Codes de statut :**

| Code | Signification | Usage |
|------|--------------|-------|
| `200 OK` | Succès | Réponse normale |
| `201 Created` | Créé | Intention acceptée |
| `400 Bad Request` | Requête invalide | Validation échouée |
| `401 Unauthorized` | Non autorisé | Authentification échouée |
| `403 Forbidden` | Interdit | Permission insuffisante |
| `404 Not Found` | Non trouvé | Ressource inexistante |
| `429 Too Many Requests` | Trop de requêtes | Rate limiting |
| `500 Internal Server Error` | Erreur serveur | Erreur technique |
| `503 Service Unavailable` | Service indisponible | Service temporairement indisponible |

**Règle PROTO-06 : Corps d'erreur**

Toutes les erreurs HTTP incluent un corps JSON avec :
- `code` : Code d'erreur canonique
- `message` : Message d'erreur lisible
- `détails` : Détails optionnels

---

## 9. Garanties de stabilité

### 9.1 Stabilité de l'interface

**Règle STAB-01 : Interface versionnée**

L'interface est versionnée. Les versions majeures introduisent des changements incompatibles. Les versions mineures ajoutent des fonctionnalités compatibles.

**Règle STAB-02 : Rétrocompatibilité**

Les versions mineures sont rétrocompatibles. Un produit utilisant la version N fonctionne avec la version N+1 (mineure).

**Règle STAB-03 : Dépréciation**

Les fonctionnalités dépréciées sont annoncées à l'avance et maintenues pendant au moins une version majeure.

**Règle STAB-04 : Pas de changement imprévisible**

L'interface ne change jamais de manière imprévisible ou rétro-incompatible sans processus formel de versionnement.

### 9.2 Garantie de disponibilité

**Règle STAB-05 : Disponibilité**

Bonding Brother garantit une disponibilité élevée (SLA défini séparément).

**Règle STAB-06 : Mode offline**

Bonding Brother fonctionne en mode offline, avec synchronisation à la reconnexion.

### 9.3 Garantie de performance

**Règle STAB-07 : Temps de réponse**

Les temps de réponse sont documentés et respectés (métriques définies séparément).

**Règle STAB-08 : Throughput**

Le throughput est documenté et respecté (limites définies séparément).

---

## 10. Versionnement

### 10.1 Schéma de versionnement

**Format :** `v<MAJEURE>.<MINEURE>.<PATCH>`

**Exemple :** `v1.2.3`

**Règle VERS-01 : Version majeure**

Les versions majeures introduisent des changements incompatibles. Migration requise.

**Règle VERS-02 : Version mineure**

Les versions mineures ajoutent des fonctionnalités compatibles. Pas de migration requise.

**Règle VERS-03 : Version patch**

Les versions patch corrigent des bugs. Pas de changement d'interface.

### 10.2 Gestion des versions

**Règle VERS-04 : Version dans l'URL**

La version est spécifiée dans l'URL : `/api/v1/...`

**Règle VERS-05 : Version dans les données**

La version du schéma peut être spécifiée dans les données (optionnel).

**Règle VERS-06 : Support multi-versions**

Bonding Brother peut supporter plusieurs versions simultanément, avec dépréciation progressive.

**Règle VERS-07 : Migration**

Les migrations entre versions majeures sont documentées et supportées.

---

## 11. Exemples

### 11.1 Soumission d'intention

**Requête :**
```http
POST /api/v1/intentions HTTP/1.1
Host: bonding-brother.example.com
Authorization: Bearer <token>
Content-Type: application/json

{
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Mon article",
    "contenu": "Contenu de l'article..."
  },
  "contexte": {
    "produit_id": "miyukini-cms",
    "utilisateur_id": "user-123",
    "environnement": "production"
  }
}
```

**Réponse :**
```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "intention_id": "int-123",
  "statut": "ACCEPTÉE",
  "timestamp": "2026-01-26T10:00:00Z",
  "version": "1.0.0"
}
```

### 11.2 Réception de résultat (callback)

**Requête de Bonding Brother vers le produit :**
```http
POST https://produit.example.com/callback HTTP/1.1
Host: produit.example.com
Content-Type: application/json

{
  "résultat_id": "res-111",
  "intention_id": "int-123",
  "statut": "SUCCÈS",
  "décision": "ACCEPTÉE",
  "données": {
    "id": "content-999",
    "titre": "Mon article"
  },
  "timestamp": "2026-01-26T10:05:00Z",
  "autorité": "kind_mother"
}
```

### 11.3 Abonnement aux notifications

**Requête :**
```http
POST /api/v1/notifications/subscribe HTTP/1.1
Host: bonding-brother.example.com
Authorization: Bearer <token>
Content-Type: application/json

{
  "produit_id": "miyukini-cms",
  "types": ["CONTENT_CREATED", "CONTENT_UPDATED"],
  "callback_url": "https://produit.example.com/notifications"
}
```

**Réponse :**
```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "abonnement_id": "sub-456",
  "statut": "ACTIF",
  "timestamp": "2026-01-26T10:00:00Z"
}
```

### 11.4 Notification reçue

**Requête de Bonding Brother vers le produit :**
```http
POST https://produit.example.com/notifications HTTP/1.1
Host: produit.example.com
Content-Type: application/json

{
  "notification_id": "notif-789",
  "abonnement_id": "sub-456",
  "type": "CONTENT_CREATED",
  "données": {
    "content_id": "content-999",
    "titre": "Mon article"
  },
  "timestamp": "2026-01-26T10:10:00Z",
  "autorité": "kind_mother"
}
```

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit l'interface stable que Bonding Brother expose aux produits et que tous les produits doivent respecter pour interagir avec l'écosystème.

Toute implémentation de l'interface produit de Bonding Brother doit respecter ce contrat. Toute évolution doit suivre les règles de versionnement définies.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- Documentation Fondatrice v1.0 (Section 7)
- Bilateral Flow Contract v1.0
- Intent Model Contract v1.0
- Architecture et Composants v1.0
- Glossaire et Terminologie v1.0
