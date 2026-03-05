# BondingBrother - Product Interface Contract

## 1. Contexte

Ce document dÃ©finit le contrat d'interface stable que Bonding Brother expose aux produits de l'Ã©cosystÃ¨me. Il spÃ©cifie les interfaces, les formats, les protocoles, et les garanties que les produits peuvent s'attendre Ã  recevoir de Bonding Brother.

Ce document complÃ¨te la Section 7 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Bilateral Flow Contract](../flows/BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour dÃ©finir l'interface complÃ¨te.

L'interface respecte les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) : elle fonctionne mÃªme en mode offline (**LOI-2**), et les intentions sont acceptÃ©es et traitÃ©es localement mÃªme sans connexion aux autoritÃ©s.

## 2. PortÃ©e / Scope

Ce document couvre :
- L'interface de soumission d'intentions
- L'interface de consommation de rÃ©sultats
- L'interface d'abonnement aux notifications
- Les formats de donnÃ©es standardisÃ©s
- Les protocoles de communication
- Les garanties de stabilitÃ© et de compatibilitÃ©
- Les rÃ¨gles de versionnement

Ce document **ne couvre pas** :
- Les dÃ©tails du flux Produit â†’ Ã‰cosystÃ¨me (voir [Product-to-Ecosystem Flow](../flows/BondingBrother%20-%20Product-to-Ecosystem%20Flow.md))
- Les dÃ©tails du flux Ã‰cosystÃ¨me â†’ Produit (voir [Ecosystem-to-Product Flow](../flows/BondingBrother%20-%20Ecosystem-to-Product%20Flow.md))
- Les rÃ¨gles d'adaptation des produits (voir [Product Adaptation Rules](./BondingBrother%20-%20Product%20Adaptation%20Rules.md))
- Les mÃ©canismes d'extension (voir [Extension & Specialization Contract](./BondingBrother%20-%20Extension%20&%20Specialization%20Contract.md))

---

## 3. Principe fondamental

**Bonding Brother offre une interface stable aux produits. Cette stabilitÃ© est fondamentale : les produits s'adaptent Ã  Bonding Brother, jamais l'inverse.**

L'interface est versionnÃ©e, documentÃ©e, et Ã©volue selon des rÃ¨gles strictes de compatibilitÃ©. Les produits peuvent s'appuyer sur cette interface sans crainte de rupture.

---

## 4. Interface de soumission d'intentions

### 4.1 Point d'entrÃ©e

**Interface :** `IIntentSubmission`

**Endpoint :** `/api/v1/intentions`

**MÃ©thode :** `POST`

**Format :** JSON

### 4.2 Structure de l'intention

```typescript
interface IntentionSubmission {
    // Identifiants
    id?: IntentionId;                    // Optionnel (gÃ©nÃ©rÃ© si absent)
    produit_id: ProduitId;               // Obligatoire
    
    // Type et contenu
    type: TypeIntention;                  // Obligatoire (type canonique)
    payload: PayloadSpÃ©cifique;         // Obligatoire (structure dÃ©pend du type)
    
    // Contexte
    contexte: Contexte;                   // Obligatoire (voir Section 5)
    
    // MÃ©tadonnÃ©es
    timestamp?: Timestamp;                // Optionnel (gÃ©nÃ©rÃ© si absent)
    version?: VersionIntention;          // Optionnel (dÃ©faut: derniÃ¨re version)
    corrÃ©lation_id?: CorrÃ©lationId;      // Optionnel (pour traÃ§abilitÃ© distribuÃ©e)
}
```

### 4.3 RÃ¨gles de soumission

**RÃ¨gle SUB-01 : Format JSON valide**

L'intention doit Ãªtre un JSON valide, parsable sans erreur.

**RÃ¨gle SUB-02 : Champs obligatoires**

Tous les champs marquÃ©s comme obligatoires doivent Ãªtre prÃ©sents :
- `produit_id`
- `type`
- `payload`
- `contexte`

**RÃ¨gle SUB-03 : Types de donnÃ©es**

Chaque champ doit respecter son type dÃ©clarÃ©.

**RÃ¨gle SUB-04 : Version supportÃ©e**

La version du schÃ©ma doit Ãªtre supportÃ©e par Bonding Brother. Les versions obsolÃ¨tes sont rejetÃ©es.

**RÃ¨gle SUB-05 : Type reconnu**

Le type d'intention doit Ãªtre un type canonique reconnu par Bonding Brother.

### 4.4 RÃ©ponse Ã  la soumission

**Format de rÃ©ponse :**

```typescript
interface SubmissionResponse {
    // Identifiants
    intention_id: IntentionId;           // ID de l'intention (confirmÃ© ou gÃ©nÃ©rÃ©)
    
    // Statut
    statut: StatutSoumission;            // ACCEPTÃ‰E, REJETÃ‰E, EN_ERREUR
    
    // MÃ©tadonnÃ©es
    timestamp: Timestamp;                 // Timestamp de rÃ©ception
    version: VersionIntention;            // Version utilisÃ©e
    
    // Erreurs (si applicable)
    erreurs?: ErreurSoumission[];        // Erreurs de validation
}
```

**Statuts possibles :**
- `ACCEPTÃ‰E` : Intention acceptÃ©e et en cours de traitement
- `REJETÃ‰E` : Intention rejetÃ©e (validation Ã©chouÃ©e)
- `EN_ERREUR` : Erreur technique lors de la rÃ©ception

**RÃ¨gle SUB-06 : RÃ©ponse immÃ©diate**

La rÃ©ponse Ã  la soumission est immÃ©diate (synchronisÃ©e). Elle indique uniquement l'acceptation ou le rejet de la structure, pas le rÃ©sultat de l'intention.

**RÃ¨gle SUB-07 : Pas de rÃ©sultat dans la rÃ©ponse**

La rÃ©ponse de soumission ne contient pas le rÃ©sultat de l'intention. Le rÃ©sultat arrive via l'interface de consommation (asynchrone).

---

## 5. Interface de consommation de rÃ©sultats

### 5.1 Point de rÃ©ception

**Interface :** `IResultConsumption`

**MÃ©canisme :** Callback ou polling (selon configuration)

**Format :** JSON

### 5.2 Structure du rÃ©sultat

```typescript
interface RÃ©sultat {
    // Identifiants
    rÃ©sultat_id: RÃ©sultatId;             // ID unique du rÃ©sultat
    intention_id: IntentionId;           // ID de l'intention source
    demande_id?: DemandeId;              // ID de la demande (traÃ§abilitÃ©)
    
    // Statut
    statut: StatutRÃ©sultat;              // SUCCÃˆS, REFUSÃ‰, ERREUR
    dÃ©cision: DÃ©cisionAutoritÃ©;          // DÃ©cision de l'autoritÃ© (traduite)
    
    // DonnÃ©es (si applicable)
    donnÃ©es?: DonnÃ©esTraduites;          // DonnÃ©es traduites pour le produit
    
    // Erreurs (si applicable)
    erreurs?: ErreurTraduite[];          // Erreurs dans le vocabulaire produit
    
    // MÃ©tadonnÃ©es
    timestamp: Timestamp;                 // Timestamp de la rÃ©ponse
    autoritÃ©: AutoritÃ©Id;                // AutoritÃ© qui a rÃ©pondu (Kind Mother ou Strong Father)
    durÃ©e_traitement?: DurÃ©e;             // DurÃ©e de traitement (si disponible)
}
```

### 5.3 Statuts de rÃ©sultat

**Statuts possibles :**

| Statut | Signification | DonnÃ©es prÃ©sentes |
|--------|---------------|-------------------|
| `SUCCÃˆS` | Intention acceptÃ©e et exÃ©cutÃ©e | Oui (donnÃ©es de rÃ©sultat) |
| `REFUSÃ‰` | Intention refusÃ©e par l'autoritÃ© | Non (raison du refus) |
| `ERREUR` | Erreur technique ou mÃ©tier | Non (dÃ©tails de l'erreur) |
| `TIMEOUT` | DÃ©lai d'attente dÃ©passÃ© | Non |
| `ABANDONNÃ‰E` | Intention abandonnÃ©e | Non (raison) |

**RÃ¨gle RES-01 : Un rÃ©sultat par intention**

Chaque intention acceptÃ©e reÃ§oit exactement un rÃ©sultat (succÃ¨s, refus, ou erreur).

**RÃ¨gle RES-02 : RÃ©sultat asynchrone**

Le rÃ©sultat arrive de maniÃ¨re asynchrone, aprÃ¨s l'Ã©valuation par l'autoritÃ©.

**RÃ¨gle RES-03 : Ordre prÃ©servÃ©**

Pour une mÃªme intention, l'ordre des rÃ©sultats est prÃ©servÃ© (FIFO).

**RÃ¨gle RES-04 : RÃ©sultat complet**

Le rÃ©sultat contient toutes les informations nÃ©cessaires et autorisÃ©es pour le produit.

### 5.4 MÃ©canismes de rÃ©ception

**Option 1 : Callback (recommandÃ©)**

Le produit fournit une URL de callback lors de la soumission. Bonding Brother appelle cette URL avec le rÃ©sultat.

**Option 2 : Polling**

Le produit interroge rÃ©guliÃ¨rement Bonding Brother pour rÃ©cupÃ©rer les rÃ©sultats.

**Option 3 : Webhook**

Le produit s'abonne Ã  un webhook pour recevoir les rÃ©sultats.

**RÃ¨gle RES-05 : MÃ©canisme configurable**

Le mÃ©canisme de rÃ©ception est configurable par produit, avec callback comme option par dÃ©faut.

---

## 6. Interface d'abonnement aux notifications

### 6.1 Point d'abonnement

**Interface :** `INotificationSubscription`

**Endpoint :** `/api/v1/notifications/subscribe`

**MÃ©thode :** `POST`

**Format :** JSON

### 6.2 Structure de l'abonnement

```typescript
interface AbonnementNotification {
    // Identifiants
    produit_id: ProduitId;               // Obligatoire
    abonnement_id?: AbonnementId;         // Optionnel (gÃ©nÃ©rÃ© si absent)
    
    // Types de notifications
    types: TypeNotification[];           // Obligatoire (types souhaitÃ©s)
    
    // Filtres
    filtres?: FiltresNotification;        // Optionnel (filtres spÃ©cifiques)
    
    // Destination
    callback_url: URL;                   // Obligatoire (URL de rÃ©ception)
    
    // MÃ©tadonnÃ©es
    timestamp?: Timestamp;                // Optionnel
}
```

### 6.3 Types de notifications

**Types disponibles :**

| Type | Description | DÃ©clencheur |
|------|-------------|-------------|
| `CONTENT_CREATED` | Contenu crÃ©Ã© | Kind Mother crÃ©e un contenu |
| `CONTENT_UPDATED` | Contenu modifiÃ© | Kind Mother modifie un contenu |
| `CONTENT_DELETED` | Contenu supprimÃ© | Kind Mother supprime un contenu |
| `HIERARCHY_CHANGED` | HiÃ©rarchie modifiÃ©e | Kind Mother modifie la hiÃ©rarchie |
| `PERMISSION_CHANGED` | Permission modifiÃ©e | Strong Father modifie une permission |
| `SESSION_REVOKED` | Session rÃ©voquÃ©e | Strong Father rÃ©voque une session |
| `SYSTEM_EVENT` | Ã‰vÃ©nement systÃ¨me | Ã‰vÃ©nement systÃ¨me |

**RÃ¨gle NOTIF-01 : Types canoniques**

Les types de notifications sont dÃ©finis de maniÃ¨re canonique. Un nouveau type nÃ©cessite une version majeure.

**RÃ¨gle NOTIF-02 : Filtres optionnels**

Les filtres permettent de restreindre les notifications reÃ§ues (ex: uniquement pour certaines ressources).

### 6.4 Structure de la notification

```typescript
interface Notification {
    // Identifiants
    notification_id: NotificationId;      // ID unique de la notification
    abonnement_id: AbonnementId;         // ID de l'abonnement
    
    // Type et contenu
    type: TypeNotification;               // Type de notification
    donnÃ©es: DonnÃ©esNotification;         // DonnÃ©es de la notification
    
    // MÃ©tadonnÃ©es
    timestamp: Timestamp;                 // Timestamp de l'Ã©vÃ©nement
    autoritÃ©: AutoritÃ©Id;                 // AutoritÃ© source (Kind Mother ou Strong Father)
    corrÃ©lation_id?: CorrÃ©lationId;       // ID de corrÃ©lation (si applicable)
}
```

**RÃ¨gle NOTIF-03 : Notification asynchrone**

Les notifications sont envoyÃ©es de maniÃ¨re asynchrone, sans garantie d'ordre (sauf pour les notifications corrÃ©lÃ©es).

**RÃ¨gle NOTIF-04 : Notification unique**

Chaque Ã©vÃ©nement gÃ©nÃ¨re une notification unique, envoyÃ©e Ã  tous les produits abonnÃ©s concernÃ©s.

**RÃ¨gle NOTIF-05 : Retry en cas d'Ã©chec**

En cas d'Ã©chec de livraison, Bonding Brother retente selon une politique dÃ©finie.

---

## 7. Formats de donnÃ©es standardisÃ©s

### 7.1 Format JSON

**RÃ¨gle FORMAT-01 : JSON strict**

Tous les Ã©changes utilisent JSON strict, avec validation de schÃ©ma.

**RÃ¨gle FORMAT-02 : Encodage UTF-8**

Tous les textes sont encodÃ©s en UTF-8.

**RÃ¨gle FORMAT-03 : Dates ISO 8601**

Toutes les dates sont au format ISO 8601 : `YYYY-MM-DDTHH:mm:ssZ`

### 7.2 Types de donnÃ©es

**Types primitifs supportÃ©s :**
- `string` : ChaÃ®ne de caractÃ¨res UTF-8
- `number` : Nombre (entier ou dÃ©cimal)
- `boolean` : BoolÃ©en
- `null` : Valeur nulle

**Types complexes supportÃ©s :**
- `object` : Objet JSON
- `array` : Tableau JSON

**RÃ¨gle FORMAT-04 : Types stricts**

Les types de donnÃ©es sont stricts. Pas de conversion implicite.

### 7.3 Vocabulaire canonique

**RÃ¨gle FORMAT-05 : Vocabulaire standard**

Tous les termes utilisÃ©s dans l'interface suivent le vocabulaire canonique dÃ©fini dans le [Vocabulary & Glossary](../../reference/BondingBrother%20-%20Vocabulary%20&%20Glossary.md).

**RÃ¨gle FORMAT-06 : Pas de synonymes**

Les synonymes sont interdits. Un seul terme par concept.

---

## 8. Protocoles de communication

### 8.1 Protocole HTTP/HTTPS

**RÃ¨gle PROTO-01 : HTTPS obligatoire**

Toutes les communications utilisent HTTPS en production. HTTP est autorisÃ© uniquement en dÃ©veloppement.

**RÃ¨gle PROTO-02 : Version HTTP**

HTTP/1.1 ou HTTP/2 sont supportÃ©s. HTTP/3 est supportÃ© si disponible.

**RÃ¨gle PROTO-03 : MÃ©thodes REST**

Les mÃ©thodes REST standard sont utilisÃ©es :
- `POST` : CrÃ©ation (soumission d'intention, abonnement)
- `GET` : Lecture (polling de rÃ©sultats)
- `DELETE` : Suppression (dÃ©sabonnement)

### 8.2 Authentification

**RÃ¨gle PROTO-04 : Authentification requise**

Toutes les requÃªtes doivent Ãªtre authentifiÃ©es. Le mÃ©canisme d'authentification est dÃ©fini par Strong Father.

**RÃ¨gle PROTO-05 : Tokens**

Les tokens d'authentification sont fournis via les en-tÃªtes HTTP standard (`Authorization: Bearer <token>`).

### 8.3 Gestion des erreurs HTTP

**Codes de statut :**

| Code | Signification | Usage |
|------|--------------|-------|
| `200 OK` | SuccÃ¨s | RÃ©ponse normale |
| `201 Created` | CrÃ©Ã© | Intention acceptÃ©e |
| `400 Bad Request` | RequÃªte invalide | Validation Ã©chouÃ©e |
| `401 Unauthorized` | Non autorisÃ© | Authentification Ã©chouÃ©e |
| `403 Forbidden` | Interdit | Permission insuffisante |
| `404 Not Found` | Non trouvÃ© | Ressource inexistante |
| `429 Too Many Requests` | Trop de requÃªtes | Rate limiting |
| `500 Internal Server Error` | Erreur serveur | Erreur technique |
| `503 Service Unavailable` | Service indisponible | Service temporairement indisponible |

**RÃ¨gle PROTO-06 : Corps d'erreur**

Toutes les erreurs HTTP incluent un corps JSON avec :
- `code` : Code d'erreur canonique
- `message` : Message d'erreur lisible
- `dÃ©tails` : DÃ©tails optionnels

---

## 9. Garanties de stabilitÃ©

### 9.1 StabilitÃ© de l'interface

**RÃ¨gle STAB-01 : Interface versionnÃ©e**

L'interface est versionnÃ©e. Les versions majeures introduisent des changements incompatibles. Les versions mineures ajoutent des fonctionnalitÃ©s compatibles.

**RÃ¨gle STAB-02 : RÃ©trocompatibilitÃ©**

Les versions mineures sont rÃ©trocompatibles. Un produit utilisant la version N fonctionne avec la version N+1 (mineure).

**RÃ¨gle STAB-03 : DÃ©prÃ©ciation**

Les fonctionnalitÃ©s dÃ©prÃ©ciÃ©es sont annoncÃ©es Ã  l'avance et maintenues pendant au moins une version majeure.

**RÃ¨gle STAB-04 : Pas de changement imprÃ©visible**

L'interface ne change jamais de maniÃ¨re imprÃ©visible ou rÃ©tro-incompatible sans processus formel de versionnement.

### 9.2 Garantie de disponibilitÃ©

**RÃ¨gle STAB-05 : DisponibilitÃ©**

Bonding Brother garantit une disponibilitÃ© Ã©levÃ©e (SLA dÃ©fini sÃ©parÃ©ment).

**RÃ¨gle STAB-06 : Mode offline**

Bonding Brother fonctionne en mode offline, avec synchronisation Ã  la reconnexion.

### 9.3 Garantie de performance

**RÃ¨gle STAB-07 : Temps de rÃ©ponse**

Les temps de rÃ©ponse sont documentÃ©s et respectÃ©s (mÃ©triques dÃ©finies sÃ©parÃ©ment).

**RÃ¨gle STAB-08 : Throughput**

Le throughput est documentÃ© et respectÃ© (limites dÃ©finies sÃ©parÃ©ment).

---

## 10. Versionnement

### 10.1 SchÃ©ma de versionnement

**Format :** `v<MAJEURE>.<MINEURE>.<PATCH>`

**Exemple :** `v1.2.3`

**RÃ¨gle VERS-01 : Version majeure**

Les versions majeures introduisent des changements incompatibles. Migration requise.

**RÃ¨gle VERS-02 : Version mineure**

Les versions mineures ajoutent des fonctionnalitÃ©s compatibles. Pas de migration requise.

**RÃ¨gle VERS-03 : Version patch**

Les versions patch corrigent des bugs. Pas de changement d'interface.

### 10.2 Gestion des versions

**RÃ¨gle VERS-04 : Version dans l'URL**

La version est spÃ©cifiÃ©e dans l'URL : `/api/v1/...`

**RÃ¨gle VERS-05 : Version dans les donnÃ©es**

La version du schÃ©ma peut Ãªtre spÃ©cifiÃ©e dans les donnÃ©es (optionnel).

**RÃ¨gle VERS-06 : Support multi-versions**

Bonding Brother peut supporter plusieurs versions simultanÃ©ment, avec dÃ©prÃ©ciation progressive.

**RÃ¨gle VERS-07 : Migration**

Les migrations entre versions majeures sont documentÃ©es et supportÃ©es.

---

## 11. Exemples

### 11.1 Soumission d'intention

**RequÃªte :**
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

**RÃ©ponse :**
```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "intention_id": "int-123",
  "statut": "ACCEPTÃ‰E",
  "timestamp": "2026-01-28T10:00:00Z",
  "version": "1.0.0"
}
```

### 11.2 RÃ©ception de rÃ©sultat (callback)

**RequÃªte de Bonding Brother vers le produit :**
```http
POST https://produit.example.com/callback HTTP/1.1
Host: produit.example.com
Content-Type: application/json

{
  "rÃ©sultat_id": "res-111",
  "intention_id": "int-123",
  "statut": "SUCCÃˆS",
  "dÃ©cision": "ACCEPTÃ‰E",
  "donnÃ©es": {
    "id": "content-999",
    "titre": "Mon article"
  },
  "timestamp": "2026-01-28T10:05:00Z",
  "autoritÃ©": "kind_mother"
}
```

### 11.3 Abonnement aux notifications

**RequÃªte :**
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

**RÃ©ponse :**
```http
HTTP/1.1 201 Created
Content-Type: application/json

{
  "abonnement_id": "sub-456",
  "statut": "ACTIF",
  "timestamp": "2026-01-28T10:00:00Z"
}
```

### 11.4 Notification reÃ§ue

**RequÃªte de Bonding Brother vers le produit :**
```http
POST https://produit.example.com/notifications HTTP/1.1
Host: produit.example.com
Content-Type: application/json

{
  "notification_id": "notif-789",
  "abonnement_id": "sub-456",
  "type": "CONTENT_CREATED",
  "donnÃ©es": {
    "content_id": "content-999",
    "titre": "Mon article"
  },
  "timestamp": "2026-01-28T10:10:00Z",
  "autoritÃ©": "kind_mother"
}
```

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit l'interface stable que Bonding Brother expose aux produits et que tous les produits doivent respecter pour interagir avec l'Ã©cosystÃ¨me.

Toute implÃ©mentation de l'interface produit de Bonding Brother doit respecter ce contrat. Toute Ã©volution doit suivre les rÃ¨gles de versionnement dÃ©finies.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0 (Section 7)
- [Bilateral Flow Contract](../flows/BondingBrother%20-%20Bilateral%20Flow%20Contract.md) v2.0
- [Intent Model Contract](../intent/BondingBrother%20-%20Intent%20Model%20Contract.md) v2.0
- [Architecture & Flows](../../architecture/BondingBrother%20-%20Architecture%20&%20Flows.md) v2.0
- [Vocabulary & Glossary](../../reference/BondingBrother%20-%20Vocabulary%20&%20Glossary.md) v2.0

