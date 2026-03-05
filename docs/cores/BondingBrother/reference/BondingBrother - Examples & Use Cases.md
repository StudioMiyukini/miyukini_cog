# BondingBrother - Examples & Use Cases

## 1. Contexte

Ce document fournit des exemples concrets et des cas d'usage illustrant l'utilisation de Bonding Brother dans diffÃ©rents scÃ©narios. Il complÃ¨te la documentation contractuelle en montrant comment les concepts thÃ©oriques se traduisent en pratique.

Ce document s'appuie sur le [Bilateral Flow Contract](../contracts/flows/BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour les flux et le [Product Interface Contract](../contracts/product/BondingBrother%20-%20Product%20Interface%20Contract.md) pour les interfaces.

Les exemples illustrent le respect des [Lois d'Autonomie SystÃ¨me](..//..//..//miyukini-webway-system//reference//_index.md), notamment le fonctionnement en mode offline (**LOI-2**) et la souverainetÃ© de l'Ã©tat local (**LOI-3**).

**Navigation :** [Index BondingBrother](../_index.md)

## 2. PortÃ©e / Scope

Ce document couvre :
- Des exemples complets de flux Produit â†’ Ã‰cosystÃ¨me
- Des exemples complets de flux Ã‰cosystÃ¨me â†’ Produit
- Des cas d'usage mÃ©tier typiques
- Des scÃ©narios d'erreur et de rÃ©cupÃ©ration
- Des exemples de mode offline
- Des patterns d'utilisation courants

Ce document **ne couvre pas** :
- Les dÃ©tails contractuels (voir les contrats spÃ©cifiques)
- Les rÃ¨gles de validation (voir Intent Model Contract)
- Les rÃ¨gles de traduction (voir Translation Contract)
- Les dÃ©tails d'implÃ©mentation

---

## 3. Principe fondamental

**Les exemples illustrent les concepts, mais ne remplacent pas les contrats.**

Ces exemples sont informatifs et non normatifs. En cas de contradiction avec un contrat, le contrat prime.

---

## 4. Exemples de flux Produit â†’ Ã‰cosystÃ¨me

### 4.1 Cas d'usage : CrÃ©ation de contenu

**Contexte :** Un produit CMS souhaite crÃ©er un nouvel article de blog.

**Ã‰tape 1 : Produit soumet l'intention**

```json
POST /api/v1/intentions
{
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Introduction Ã  Bonding Brother",
    "contenu": "Bonding Brother est l'interface fraternelle...",
    "auteur_id": "user-123",
    "catÃ©gorie": "documentation",
    "tags": ["bonding-brother", "architecture"]
  },
  "contexte": {
    "produit_id": "miyukini-cms",
    "utilisateur_id": "user-123",
    "session_id": "session-456",
    "environnement": "production",
    "permissions_dÃ©clarÃ©es": ["content:write"]
  },
  "timestamp": "2026-01-26T10:00:00Z",
  "version": "1.0.0"
}
```

**Ã‰tape 2 : RÃ©ponse de soumission**

```json
HTTP/1.1 201 Created
{
  "intention_id": "int-550e8400",
  "statut": "ACCEPTÃ‰E",
  "timestamp": "2026-01-26T10:00:01Z",
  "version": "1.0.0"
}
```

**Ã‰tape 3 : Traduction et transmission Ã  Kind Mother**

Bonding Brother traduit l'intention en demande pour Kind Mother :

```json
{
  "demande_id": "dem-789abc",
  "intention_id": "int-550e8400",
  "type": "create_content",
  "donnÃ©es": {
    "title": "Introduction Ã  Bonding Brother",
    "body": "Bonding Brother est l'interface fraternelle...",
    "author_id": "user-123",
    "category": "documentation",
    "tags": ["bonding-brother", "architecture"]
  },
  "contexte": {
    "produit_id": "miyukini-cms",
    "utilisateur_id": "user-123",
    "session_id": "session-456",
    "environnement": "production",
    "permissions_dÃ©clarÃ©es": ["content:write"]
  },
  "timestamp": "2026-01-26T10:00:01Z",
  "autoritÃ©_cible": "kind_mother"
}
```

**Ã‰tape 4 : RÃ©ponse de Kind Mother**

```json
{
  "response_id": "resp-456def",
  "request_id": "dem-789abc",
  "status": "accepted",
  "data": {
    "content_id": "content-999",
    "title": "Introduction Ã  Bonding Brother",
    "created_at": "2026-01-26T10:00:05Z",
    "version": 1
  },
  "timestamp": "2026-01-26T10:00:05Z"
}
```

**Ã‰tape 5 : RÃ©sultat transmis au produit (callback)**

```json
POST https://miyukini-cms.example.com/callback
{
  "rÃ©sultat_id": "res-111222",
  "intention_id": "int-550e8400",
  "demande_id": "dem-789abc",
  "statut": "SUCCÃˆS",
  "dÃ©cision": "ACCEPTÃ‰E",
  "donnÃ©es": {
    "id": "content-999",
    "titre": "Introduction Ã  Bonding Brother",
    "crÃ©Ã©_le": "2026-01-26T10:00:05Z",
    "version": 1
  },
  "timestamp": "2026-01-26T10:00:05Z",
  "autoritÃ©": "kind_mother"
}
```

**RÃ©sumÃ© du flux :**
- DurÃ©e totale : ~5 secondes
- Ã‰tats traversÃ©s : CRÃ‰Ã‰E â†’ VALIDÃ‰E â†’ TRADUITE â†’ FILTRÃ‰E â†’ JOURNALISÃ‰E â†’ TRANSMISE â†’ EN_ATTENTE â†’ Ã‰VALUÃ‰E â†’ RÃ‰SOLUE
- AutoritÃ© : Kind Mother
- RÃ©sultat : SuccÃ¨s

---

### 4.2 Cas d'usage : VÃ©rification d'autorisation

**Contexte :** Un produit souhaite vÃ©rifier si un utilisateur peut supprimer un contenu.

**Ã‰tape 1 : Produit soumet l'intention**

```json
POST /api/v1/intentions
{
  "produit_id": "miyukini-cms",
  "type": "AUTHORIZE",
  "payload": {
    "action": "content:delete",
    "ressource_id": "content-999",
    "utilisateur_id": "user-123"
  },
  "contexte": {
    "produit_id": "miyukini-cms",
    "utilisateur_id": "user-123",
    "session_id": "session-456",
    "environnement": "production"
  },
  "timestamp": "2026-01-26T10:15:00Z",
  "version": "1.0.0"
}
```

**Ã‰tape 2 : Traduction et transmission Ã  Strong Father**

```json
{
  "demande_id": "dem-auth-001",
  "intention_id": "int-auth-123",
  "type": "check_permission",
  "donnÃ©es": {
    "action": "content:delete",
    "resource_id": "content-999",
    "user_id": "user-123"
  },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:15:00Z",
  "autoritÃ©_cible": "strong_father"
}
```

**Ã‰tape 3 : RÃ©ponse de Strong Father**

```json
{
  "response_id": "resp-auth-001",
  "request_id": "dem-auth-001",
  "status": "denied",
  "reason": "Insufficient permissions: user-123 does not have 'content:delete' permission for resource content-999",
  "timestamp": "2026-01-26T10:15:01Z"
}
```

**Ã‰tape 4 : RÃ©sultat transmis au produit**

```json
{
  "rÃ©sultat_id": "res-auth-001",
  "intention_id": "int-auth-123",
  "statut": "REFUSÃ‰",
  "dÃ©cision": "REFUSÃ‰E",
  "erreurs": [{
    "code": "AUTH-001",
    "message": "Permissions insuffisantes : l'utilisateur user-123 n'a pas la permission 'content:delete' pour la ressource content-999",
    "catÃ©gorie": "AUTORITÃ‰"
  }],
  "timestamp": "2026-01-26T10:15:01Z",
  "autoritÃ©": "strong_father"
}
```

**RÃ©sumÃ© du flux :**
- DurÃ©e totale : ~1 seconde
- AutoritÃ© : Strong Father
- RÃ©sultat : RefusÃ© (permissions insuffisantes)

---

### 4.3 Cas d'usage : Erreur de validation

**Contexte :** Un produit soumet une intention avec un champ obligatoire manquant.

**Ã‰tape 1 : Produit soumet l'intention (invalide)**

```json
POST /api/v1/intentions
{
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT"
  // payload manquant
}
```

**Ã‰tape 2 : RÃ©ponse de rejet immÃ©diat**

```json
HTTP/1.1 400 Bad Request
{
  "intention_id": "int-err-001",
  "statut": "REJETÃ‰E",
  "erreurs": [{
    "code": "VAL-002",
    "message": "Le champ 'payload' est obligatoire mais absent",
    "catÃ©gorie": "VALIDATION"
  }],
  "timestamp": "2026-01-26T10:20:00Z"
}
```

**RÃ©sumÃ© du flux :**
- DurÃ©e totale : <1 seconde
- Ã‰tat final : REJETÃ‰E (validation Ã©chouÃ©e)
- Pas de transmission Ã  l'autoritÃ©

---

## 5. Exemples de flux Ã‰cosystÃ¨me â†’ Produit

### 5.1 Cas d'usage : Notification de crÃ©ation de contenu

**Contexte :** Kind Mother a crÃ©Ã© un contenu et notifie les produits abonnÃ©s.

**Ã‰tape 1 : Kind Mother Ã©met la notification**

```json
{
  "event_type": "content_created",
  "content_id": "content-999",
  "data": {
    "title": "Introduction Ã  Bonding Brother",
    "author_id": "user-123",
    "created_at": "2026-01-26T10:00:05Z"
  },
  "timestamp": "2026-01-26T10:00:05Z"
}
```

**Ã‰tape 2 : Bonding Brother identifie les produits cibles**

Produits abonnÃ©s au type `CONTENT_CREATED` :
- `miyukini-cms` (propriÃ©taire)
- `miyukini-publisher` (abonnÃ©)

**Ã‰tape 3 : Traduction et filtrage pour chaque produit**

Pour `miyukini-cms` :
```json
{
  "notification_id": "notif-001",
  "abonnement_id": "sub-cms-001",
  "type": "CONTENT_CREATED",
  "donnÃ©es": {
    "content_id": "content-999",
    "titre": "Introduction Ã  Bonding Brother",
    "auteur_id": "user-123",
    "crÃ©Ã©_le": "2026-01-26T10:00:05Z"
  },
  "timestamp": "2026-01-26T10:00:05Z",
  "autoritÃ©": "kind_mother"
}
```

Pour `miyukini-publisher` (filtrage diffÃ©rent selon permissions) :
```json
{
  "notification_id": "notif-002",
  "abonnement_id": "sub-pub-001",
  "type": "CONTENT_CREATED",
  "donnÃ©es": {
    "content_id": "content-999",
    "titre": "Introduction Ã  Bonding Brother",
    "crÃ©Ã©_le": "2026-01-26T10:00:05Z"
    // auteur_id filtrÃ© (non autorisÃ©)
  },
  "timestamp": "2026-01-26T10:00:05Z",
  "autoritÃ©": "kind_mother"
}
```

**Ã‰tape 4 : Transmission aux produits**

Les deux produits reÃ§oivent la notification via leur callback URL respective.

**RÃ©sumÃ© du flux :**
- DurÃ©e totale : <1 seconde
- Produits notifiÃ©s : 2
- Filtrage : AdaptÃ© selon permissions

---

### 5.2 Cas d'usage : RÃ©sultat diffÃ©rÃ© (mode offline)

**Contexte :** Un produit a soumis une intention en mode offline. AprÃ¨s reconnexion, le rÃ©sultat est disponible.

**Ã‰tape 1 : Intention soumise en mode offline**

Le produit soumet une intention alors que Bonding Brother est dÃ©connectÃ© de Kind Mother.

**Ã‰tape 2 : Mise en buffer**

L'intention est journalisÃ©e et mise en buffer avec l'Ã©tat `TRANSMISE` (en attente).

**Ã‰tape 3 : Reconnexion**

Bonding Brother se reconnecte Ã  Kind Mother et transmet toutes les intentions en buffer.

**Ã‰tape 4 : RÃ©ception du rÃ©sultat**

Kind Mother rÃ©pond aprÃ¨s traitement. Bonding Brother reÃ§oit la rÃ©ponse.

**Ã‰tape 5 : Transmission du rÃ©sultat au produit**

```json
POST https://miyukini-cms.example.com/callback
{
  "rÃ©sultat_id": "res-offline-001",
  "intention_id": "int-offline-123",
  "statut": "SUCCÃˆS",
  "dÃ©cision": "ACCEPTÃ‰E",
  "donnÃ©es": {
    "id": "content-1000",
    "titre": "Article crÃ©Ã© en offline"
  },
  "timestamp": "2026-01-26T11:00:00Z",
  "autoritÃ©": "kind_mother",
  "diffÃ©rÃ©": true,
  "soumis_le": "2026-01-26T10:30:00Z"
}
```

**RÃ©sumÃ© du flux :**
- DurÃ©e totale : 30 minutes (dÃ©lai de reconnexion)
- Mode : Offline â†’ Online
- RÃ©sultat : SuccÃ¨s diffÃ©rÃ©

---

## 6. Cas d'usage mÃ©tier complets

### 6.1 ScÃ©nario : Workflow de publication

**Contexte :** Un Ã©diteur souhaite publier un article via le CMS.

**Flux complet :**

1. **CrÃ©ation du contenu**
   - Produit : `miyukini-cms`
   - Intention : `CREATE_CONTENT`
   - AutoritÃ© : Kind Mother
   - RÃ©sultat : Contenu crÃ©Ã© avec ID `content-123`

2. **VÃ©rification des permissions**
   - Produit : `miyukini-cms`
   - Intention : `AUTHORIZE` (action: `content:publish`)
   - AutoritÃ© : Strong Father
   - RÃ©sultat : Autorisation accordÃ©e

3. **Publication du contenu**
   - Produit : `miyukini-cms`
   - Intention : `UPDATE_CONTENT` (champ `status: published`)
   - AutoritÃ© : Kind Mother
   - RÃ©sultat : Contenu publiÃ©

4. **Notification aux abonnÃ©s**
   - AutoritÃ© : Kind Mother Ã©met Ã©vÃ©nement `content_published`
   - Produits notifiÃ©s : `miyukini-publisher`, `miyukini-analytics`
   - RÃ©sultat : Tous les produits sont informÃ©s de la publication

**DurÃ©e totale :** ~10 secondes
**Intentions :** 3
**Notifications :** 1 Ã©vÃ©nement â†’ 2 produits

---

### 6.2 ScÃ©nario : Synchronisation aprÃ¨s dÃ©connexion

**Contexte :** Un produit mobile se reconnecte aprÃ¨s une pÃ©riode offline.

**Flux complet :**

1. **PÃ©riode offline**
   - Le produit soumet 5 intentions pendant la dÃ©connexion
   - Toutes sont journalisÃ©es et mises en buffer
   - Ã‰tats : `JOURNALISÃ‰E` â†’ `TRANSMISE` (en attente)

2. **Reconnexion**
   - Bonding Brother dÃ©tecte la reconnexion
   - Transmet les 5 intentions Ã  Kind Mother
   - Ã‰tats : `TRANSMISE` â†’ `EN_ATTENTE`

3. **RÃ©ception des rÃ©sultats**
   - Kind Mother traite les 5 intentions
   - 4 succÃ¨s, 1 refus (permission expirÃ©e)
   - Ã‰tats : `EN_ATTENTE` â†’ `Ã‰VALUÃ‰E`

4. **Transmission des rÃ©sultats**
   - Bonding Brother transmet les 5 rÃ©sultats au produit
   - Ã‰tats : `Ã‰VALUÃ‰E` â†’ `RÃ‰SOLUE`

5. **Synchronisation des notifications**
   - Bonding Brother transmet les notifications manquÃ©es
   - 3 notifications de mise Ã  jour de contenu
   - Produit synchronisÃ© avec l'Ã©tat actuel

**DurÃ©e totale :** ~2 minutes (reconnexion + traitement)
**Intentions :** 5
**RÃ©sultats :** 4 succÃ¨s, 1 refus
**Notifications :** 3

---

### 6.3 ScÃ©nario : Gestion d'erreurs en cascade

**Contexte :** Une intention Ã©choue Ã  plusieurs Ã©tapes.

**Flux d'erreur :**

1. **Soumission d'intention**
   - Produit soumet `UPDATE_CONTENT` avec payload invalide
   - Ã‰tat : `CRÃ‰Ã‰E`

2. **Validation structurelle**
   - Validation Ã©choue : champ `content_id` manquant
   - Ã‰tat : `REJETÃ‰E`
   - Erreur : `VAL-002` (champ obligatoire manquant)

3. **Correction et nouvelle soumission**
   - Produit corrige et soumet Ã  nouveau
   - Ã‰tat : `CRÃ‰Ã‰E` â†’ `VALIDÃ‰E` â†’ `TRADUITE`

4. **Filtrage d'entrÃ©e**
   - Filtrage rejette : produit non autorisÃ© pour ce type
   - Ã‰tat : `REJETÃ‰E`
   - Erreur : `FILT-002` (produit non autorisÃ©)

5. **Correction de configuration**
   - Produit demande autorisation Ã  Strong Father
   - Nouvelle soumission aprÃ¨s autorisation
   - Ã‰tat : `CRÃ‰Ã‰E` â†’ ... â†’ `TRANSMISE`

6. **Erreur d'autoritÃ©**
   - Kind Mother refuse : contenu verrouillÃ©
   - Ã‰tat : `Ã‰VALUÃ‰E` â†’ `RÃ‰SOLUE`
   - Erreur : `AUTH-001` (refusÃ© par l'autoritÃ©)

**RÃ©sumÃ© :**
- Intentions soumises : 3
- Intentions rejetÃ©es : 2 (validation, filtrage)
- Intentions refusÃ©es : 1 (autoritÃ©)
- DurÃ©e totale : ~15 secondes

---

## 7. Patterns d'utilisation

### 7.1 Pattern : Polling de rÃ©sultats

**Contexte :** Un produit qui ne peut pas recevoir de callbacks.

**Approche :**
1. Produit soumet l'intention
2. Produit reÃ§oit `intention_id`
3. Produit interroge rÃ©guliÃ¨rement : `GET /api/v1/intentions/{intention_id}/result`
4. Bonding Brother retourne le rÃ©sultat quand disponible

**Avantages :**
- Pas besoin de callback URL
- ContrÃ´le du timing par le produit

**InconvÃ©nients :**
- Latence plus Ã©levÃ©e
- Consommation de ressources (polling)

---

### 7.2 Pattern : Abonnement aux notifications

**Contexte :** Un produit souhaite Ãªtre notifiÃ© de tous les changements de contenu.

**Approche :**
1. Produit s'abonne : `POST /api/v1/notifications/subscribe`
   ```json
   {
     "produit_id": "miyukini-publisher",
     "types": ["CONTENT_CREATED", "CONTENT_UPDATED", "CONTENT_DELETED"],
     "callback_url": "https://publisher.example.com/notifications"
   }
   ```

2. Bonding Brother notifie le produit Ã  chaque Ã©vÃ©nement correspondant

3. Produit peut se dÃ©sabonner : `DELETE /api/v1/notifications/subscribe/{abonnement_id}`

**Avantages :**
- Notifications en temps rÃ©el
- Pas de polling nÃ©cessaire

**InconvÃ©nients :**
- NÃ©cessite un endpoint de callback
- Gestion de la disponibilitÃ© du callback

---

### 7.3 Pattern : Batch d'intentions

**Contexte :** Un produit souhaite soumettre plusieurs intentions liÃ©es.

**Approche :**
1. Produit soumet plusieurs intentions sÃ©quentiellement
2. Chaque intention a un `corrÃ©lation_id` commun
3. Produit peut suivre le traitement du batch via le `corrÃ©lation_id`

**Exemple :**
```json
// Intention 1
{
  "intention_id": "int-batch-001",
  "corrÃ©lation_id": "batch-import-2026-01-26",
  "type": "CREATE_CONTENT",
  ...
}

// Intention 2
{
  "intention_id": "int-batch-002",
  "corrÃ©lation_id": "batch-import-2026-01-26",
  "type": "CREATE_CONTENT",
  ...
}
```

**Avantages :**
- TraÃ§abilitÃ© groupÃ©e
- Gestion d'erreurs simplifiÃ©e

**InconvÃ©nients :**
- Pas de garantie d'ordre de traitement
- Pas de transaction atomique

---

## 8. ScÃ©narios de performance

### 8.1 ScÃ©nario : Charge Ã©levÃ©e

**Contexte :** 1000 intentions soumises simultanÃ©ment.

**Comportement attendu :**
- Bonding Brother accepte toutes les intentions
- Traitement asynchrone en parallÃ¨le
- Chaque intention suit son propre flux
- RÃ©sultats retournÃ©s au fur et Ã  mesure

**MÃ©triques typiques :**
- Temps de validation : <10ms par intention
- Temps de traduction : <5ms par intention
- Temps de transmission : variable (dÃ©pend de l'autoritÃ©)
- Throughput : 100-500 intentions/seconde (selon configuration)

---

### 8.2 ScÃ©nario : Latence minimale

**Contexte :** Une intention nÃ©cessite une rÃ©ponse rapide.

**Optimisations :**
- Utilisation de callback (pas de polling)
- Timeout court configurÃ©
- PrioritÃ© Ã©levÃ©e (si supportÃ©)

**MÃ©triques typiques :**
- Temps total : 100-500ms (si autoritÃ© rapide)
- Temps de traitement BB : <50ms
- Temps d'attente autoritÃ© : variable

---

## 9. Exemples de codes d'erreur

### 9.1 Erreur de validation

```json
{
  "statut": "REJETÃ‰E",
  "erreur": {
    "code": "VAL-002",
    "message": "Le champ 'payload.titre' est obligatoire mais absent",
    "catÃ©gorie": "VALIDATION"
  }
}
```

### 9.2 Erreur de traduction

```json
{
  "statut": "REJETÃ‰E",
  "erreur": {
    "code": "TRAD-001",
    "message": "Aucun mapping n'existe pour le type d'intention 'CUSTOM_ACTION' vers l'autoritÃ© 'kind_mother'",
    "catÃ©gorie": "TRADUCTION"
  }
}
```

### 9.3 Erreur d'autoritÃ©

```json
{
  "statut": "ERREUR",
  "erreur": {
    "code": "AUTH-001",
    "message": "L'autoritÃ© a refusÃ© votre demande : permissions insuffisantes",
    "catÃ©gorie": "AUTORITÃ‰"
  }
}
```

### 9.4 Timeout

```json
{
  "statut": "ERREUR",
  "erreur": {
    "code": "TIMEOUT-002",
    "message": "L'autoritÃ© n'a pas rÃ©pondu dans le dÃ©lai imparti (30 secondes)",
    "catÃ©gorie": "TIMEOUT"
  }
}
```

---

## 10. Bonnes pratiques

### 10.1 Gestion des erreurs

**Recommandation :** Toujours gÃ©rer tous les statuts de rÃ©sultat.

```typescript
switch (rÃ©sultat.statut) {
  case "SUCCÃˆS":
    // Traiter le succÃ¨s
    break;
  case "REFUSÃ‰":
    // GÃ©rer le refus (afficher message, logger)
    break;
  case "ERREUR":
    // GÃ©rer l'erreur (afficher message, logger, retry si transitoire)
    break;
  case "TIMEOUT":
    // GÃ©rer le timeout (retry ou notification utilisateur)
    break;
}
```

---

### 10.2 TraÃ§abilitÃ©

**Recommandation :** Toujours conserver les `intention_id` et `corrÃ©lation_id` pour le support.

```typescript
const intentionId = rÃ©sultat.intention_id;
const corrÃ©lationId = intention.contexte.corrÃ©lation_id;

// Logger pour support
logger.info("Intention rÃ©solue", {
  intention_id: intentionId,
  corrÃ©lation_id: corrÃ©lationId,
  statut: rÃ©sultat.statut
});
```

---

### 10.3 Gestion du mode offline

**Recommandation :** Toujours gÃ©rer les rÃ©sultats diffÃ©rÃ©s.

```typescript
if (rÃ©sultat.diffÃ©rÃ©) {
  // RÃ©sultat d'une intention soumise en offline
  logger.info("RÃ©sultat diffÃ©rÃ© reÃ§u", {
    intention_id: rÃ©sultat.intention_id,
    soumis_le: rÃ©sultat.soumis_le,
    reÃ§u_le: rÃ©sultat.timestamp
  });
}
```

---

## 11. Statut contractuel

Ce document est **informatif, non normatif, et de statut EXEMPLES**. Il illustre l'utilisation de Bonding Brother mais ne remplace pas les contrats.

En cas de contradiction avec un contrat, le contrat prime toujours.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** EXEMPLES â€” Informatif  
**DÃ©pendances :** 
- Bilateral Flow Contract v2.0
- Product Interface Contract v2.0
- Product-to-Ecosystem Flow v2.0
- Ecosystem-to-Product Flow v2.0
- Intent Model Contract v2.0
- Translation Contract v2.0

