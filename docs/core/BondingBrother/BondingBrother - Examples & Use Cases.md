# BondingBrother - Examples & Use Cases

## 1. Contexte

Ce document fournit des exemples concrets et des cas d'usage illustrant l'utilisation de Bonding Brother dans différents scénarios. Il complète la documentation contractuelle en montrant comment les concepts théoriques se traduisent en pratique.

Ce document s'appuie sur le [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour les flux et le [Product Interface Contract](./BondingBrother%20-%20Product%20Interface%20Contract.md) pour les interfaces.

Les exemples illustrent le respect des [Lois d'Autonomie Système](../reference/Miyukini%20Framework%20-%20Lois%20Autonomie%20Systeme.md), notamment le fonctionnement en mode offline (**LOI-2**) et la souveraineté de l'état local (**LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- Des exemples complets de flux Produit → Écosystème
- Des exemples complets de flux Écosystème → Produit
- Des cas d'usage métier typiques
- Des scénarios d'erreur et de récupération
- Des exemples de mode offline
- Des patterns d'utilisation courants

Ce document **ne couvre pas** :
- Les détails contractuels (voir les contrats spécifiques)
- Les règles de validation (voir Intent Model Contract)
- Les règles de traduction (voir Translation Contract)
- Les détails d'implémentation

---

## 3. Principe fondamental

**Les exemples illustrent les concepts, mais ne remplacent pas les contrats.**

Ces exemples sont informatifs et non normatifs. En cas de contradiction avec un contrat, le contrat prime.

---

## 4. Exemples de flux Produit → Écosystème

### 4.1 Cas d'usage : Création de contenu

**Contexte :** Un produit CMS souhaite créer un nouvel article de blog.

**Étape 1 : Produit soumet l'intention**

```json
POST /api/v1/intentions
{
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT",
  "payload": {
    "titre": "Introduction à Bonding Brother",
    "contenu": "Bonding Brother est l'interface fraternelle...",
    "auteur_id": "user-123",
    "catégorie": "documentation",
    "tags": ["bonding-brother", "architecture"]
  },
  "contexte": {
    "produit_id": "miyukini-cms",
    "utilisateur_id": "user-123",
    "session_id": "session-456",
    "environnement": "production",
    "permissions_déclarées": ["content:write"]
  },
  "timestamp": "2026-01-26T10:00:00Z",
  "version": "1.0.0"
}
```

**Étape 2 : Réponse de soumission**

```json
HTTP/1.1 201 Created
{
  "intention_id": "int-550e8400",
  "statut": "ACCEPTÉE",
  "timestamp": "2026-01-26T10:00:01Z",
  "version": "1.0.0"
}
```

**Étape 3 : Traduction et transmission à Kind Mother**

Bonding Brother traduit l'intention en demande pour Kind Mother :

```json
{
  "demande_id": "dem-789abc",
  "intention_id": "int-550e8400",
  "type": "create_content",
  "données": {
    "title": "Introduction à Bonding Brother",
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
    "permissions_déclarées": ["content:write"]
  },
  "timestamp": "2026-01-26T10:00:01Z",
  "autorité_cible": "kind_mother"
}
```

**Étape 4 : Réponse de Kind Mother**

```json
{
  "response_id": "resp-456def",
  "request_id": "dem-789abc",
  "status": "accepted",
  "data": {
    "content_id": "content-999",
    "title": "Introduction à Bonding Brother",
    "created_at": "2026-01-26T10:00:05Z",
    "version": 1
  },
  "timestamp": "2026-01-26T10:00:05Z"
}
```

**Étape 5 : Résultat transmis au produit (callback)**

```json
POST https://miyukini-cms.example.com/callback
{
  "résultat_id": "res-111222",
  "intention_id": "int-550e8400",
  "demande_id": "dem-789abc",
  "statut": "SUCCÈS",
  "décision": "ACCEPTÉE",
  "données": {
    "id": "content-999",
    "titre": "Introduction à Bonding Brother",
    "créé_le": "2026-01-26T10:00:05Z",
    "version": 1
  },
  "timestamp": "2026-01-26T10:00:05Z",
  "autorité": "kind_mother"
}
```

**Résumé du flux :**
- Durée totale : ~5 secondes
- États traversés : CRÉÉE → VALIDÉE → TRADUITE → FILTRÉE → JOURNALISÉE → TRANSMISE → EN_ATTENTE → ÉVALUÉE → RÉSOLUE
- Autorité : Kind Mother
- Résultat : Succès

---

### 4.2 Cas d'usage : Vérification d'autorisation

**Contexte :** Un produit souhaite vérifier si un utilisateur peut supprimer un contenu.

**Étape 1 : Produit soumet l'intention**

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

**Étape 2 : Traduction et transmission à Strong Father**

```json
{
  "demande_id": "dem-auth-001",
  "intention_id": "int-auth-123",
  "type": "check_permission",
  "données": {
    "action": "content:delete",
    "resource_id": "content-999",
    "user_id": "user-123"
  },
  "contexte": { ... },
  "timestamp": "2026-01-26T10:15:00Z",
  "autorité_cible": "strong_father"
}
```

**Étape 3 : Réponse de Strong Father**

```json
{
  "response_id": "resp-auth-001",
  "request_id": "dem-auth-001",
  "status": "denied",
  "reason": "Insufficient permissions: user-123 does not have 'content:delete' permission for resource content-999",
  "timestamp": "2026-01-26T10:15:01Z"
}
```

**Étape 4 : Résultat transmis au produit**

```json
{
  "résultat_id": "res-auth-001",
  "intention_id": "int-auth-123",
  "statut": "REFUSÉ",
  "décision": "REFUSÉE",
  "erreurs": [{
    "code": "AUTH-001",
    "message": "Permissions insuffisantes : l'utilisateur user-123 n'a pas la permission 'content:delete' pour la ressource content-999",
    "catégorie": "AUTORITÉ"
  }],
  "timestamp": "2026-01-26T10:15:01Z",
  "autorité": "strong_father"
}
```

**Résumé du flux :**
- Durée totale : ~1 seconde
- Autorité : Strong Father
- Résultat : Refusé (permissions insuffisantes)

---

### 4.3 Cas d'usage : Erreur de validation

**Contexte :** Un produit soumet une intention avec un champ obligatoire manquant.

**Étape 1 : Produit soumet l'intention (invalide)**

```json
POST /api/v1/intentions
{
  "produit_id": "miyukini-cms",
  "type": "CREATE_CONTENT"
  // payload manquant
}
```

**Étape 2 : Réponse de rejet immédiat**

```json
HTTP/1.1 400 Bad Request
{
  "intention_id": "int-err-001",
  "statut": "REJETÉE",
  "erreurs": [{
    "code": "VAL-002",
    "message": "Le champ 'payload' est obligatoire mais absent",
    "catégorie": "VALIDATION"
  }],
  "timestamp": "2026-01-26T10:20:00Z"
}
```

**Résumé du flux :**
- Durée totale : <1 seconde
- État final : REJETÉE (validation échouée)
- Pas de transmission à l'autorité

---

## 5. Exemples de flux Écosystème → Produit

### 5.1 Cas d'usage : Notification de création de contenu

**Contexte :** Kind Mother a créé un contenu et notifie les produits abonnés.

**Étape 1 : Kind Mother émet la notification**

```json
{
  "event_type": "content_created",
  "content_id": "content-999",
  "data": {
    "title": "Introduction à Bonding Brother",
    "author_id": "user-123",
    "created_at": "2026-01-26T10:00:05Z"
  },
  "timestamp": "2026-01-26T10:00:05Z"
}
```

**Étape 2 : Bonding Brother identifie les produits cibles**

Produits abonnés au type `CONTENT_CREATED` :
- `miyukini-cms` (propriétaire)
- `miyukini-publisher` (abonné)

**Étape 3 : Traduction et filtrage pour chaque produit**

Pour `miyukini-cms` :
```json
{
  "notification_id": "notif-001",
  "abonnement_id": "sub-cms-001",
  "type": "CONTENT_CREATED",
  "données": {
    "content_id": "content-999",
    "titre": "Introduction à Bonding Brother",
    "auteur_id": "user-123",
    "créé_le": "2026-01-26T10:00:05Z"
  },
  "timestamp": "2026-01-26T10:00:05Z",
  "autorité": "kind_mother"
}
```

Pour `miyukini-publisher` (filtrage différent selon permissions) :
```json
{
  "notification_id": "notif-002",
  "abonnement_id": "sub-pub-001",
  "type": "CONTENT_CREATED",
  "données": {
    "content_id": "content-999",
    "titre": "Introduction à Bonding Brother",
    "créé_le": "2026-01-26T10:00:05Z"
    // auteur_id filtré (non autorisé)
  },
  "timestamp": "2026-01-26T10:00:05Z",
  "autorité": "kind_mother"
}
```

**Étape 4 : Transmission aux produits**

Les deux produits reçoivent la notification via leur callback URL respective.

**Résumé du flux :**
- Durée totale : <1 seconde
- Produits notifiés : 2
- Filtrage : Adapté selon permissions

---

### 5.2 Cas d'usage : Résultat différé (mode offline)

**Contexte :** Un produit a soumis une intention en mode offline. Après reconnexion, le résultat est disponible.

**Étape 1 : Intention soumise en mode offline**

Le produit soumet une intention alors que Bonding Brother est déconnecté de Kind Mother.

**Étape 2 : Mise en buffer**

L'intention est journalisée et mise en buffer avec l'état `TRANSMISE` (en attente).

**Étape 3 : Reconnexion**

Bonding Brother se reconnecte à Kind Mother et transmet toutes les intentions en buffer.

**Étape 4 : Réception du résultat**

Kind Mother répond après traitement. Bonding Brother reçoit la réponse.

**Étape 5 : Transmission du résultat au produit**

```json
POST https://miyukini-cms.example.com/callback
{
  "résultat_id": "res-offline-001",
  "intention_id": "int-offline-123",
  "statut": "SUCCÈS",
  "décision": "ACCEPTÉE",
  "données": {
    "id": "content-1000",
    "titre": "Article créé en offline"
  },
  "timestamp": "2026-01-26T11:00:00Z",
  "autorité": "kind_mother",
  "différé": true,
  "soumis_le": "2026-01-26T10:30:00Z"
}
```

**Résumé du flux :**
- Durée totale : 30 minutes (délai de reconnexion)
- Mode : Offline → Online
- Résultat : Succès différé

---

## 6. Cas d'usage métier complets

### 6.1 Scénario : Workflow de publication

**Contexte :** Un éditeur souhaite publier un article via le CMS.

**Flux complet :**

1. **Création du contenu**
   - Produit : `miyukini-cms`
   - Intention : `CREATE_CONTENT`
   - Autorité : Kind Mother
   - Résultat : Contenu créé avec ID `content-123`

2. **Vérification des permissions**
   - Produit : `miyukini-cms`
   - Intention : `AUTHORIZE` (action: `content:publish`)
   - Autorité : Strong Father
   - Résultat : Autorisation accordée

3. **Publication du contenu**
   - Produit : `miyukini-cms`
   - Intention : `UPDATE_CONTENT` (champ `status: published`)
   - Autorité : Kind Mother
   - Résultat : Contenu publié

4. **Notification aux abonnés**
   - Autorité : Kind Mother émet événement `content_published`
   - Produits notifiés : `miyukini-publisher`, `miyukini-analytics`
   - Résultat : Tous les produits sont informés de la publication

**Durée totale :** ~10 secondes
**Intentions :** 3
**Notifications :** 1 événement → 2 produits

---

### 6.2 Scénario : Synchronisation après déconnexion

**Contexte :** Un produit mobile se reconnecte après une période offline.

**Flux complet :**

1. **Période offline**
   - Le produit soumet 5 intentions pendant la déconnexion
   - Toutes sont journalisées et mises en buffer
   - États : `JOURNALISÉE` → `TRANSMISE` (en attente)

2. **Reconnexion**
   - Bonding Brother détecte la reconnexion
   - Transmet les 5 intentions à Kind Mother
   - États : `TRANSMISE` → `EN_ATTENTE`

3. **Réception des résultats**
   - Kind Mother traite les 5 intentions
   - 4 succès, 1 refus (permission expirée)
   - États : `EN_ATTENTE` → `ÉVALUÉE`

4. **Transmission des résultats**
   - Bonding Brother transmet les 5 résultats au produit
   - États : `ÉVALUÉE` → `RÉSOLUE`

5. **Synchronisation des notifications**
   - Bonding Brother transmet les notifications manquées
   - 3 notifications de mise à jour de contenu
   - Produit synchronisé avec l'état actuel

**Durée totale :** ~2 minutes (reconnexion + traitement)
**Intentions :** 5
**Résultats :** 4 succès, 1 refus
**Notifications :** 3

---

### 6.3 Scénario : Gestion d'erreurs en cascade

**Contexte :** Une intention échoue à plusieurs étapes.

**Flux d'erreur :**

1. **Soumission d'intention**
   - Produit soumet `UPDATE_CONTENT` avec payload invalide
   - État : `CRÉÉE`

2. **Validation structurelle**
   - Validation échoue : champ `content_id` manquant
   - État : `REJETÉE`
   - Erreur : `VAL-002` (champ obligatoire manquant)

3. **Correction et nouvelle soumission**
   - Produit corrige et soumet à nouveau
   - État : `CRÉÉE` → `VALIDÉE` → `TRADUITE`

4. **Filtrage d'entrée**
   - Filtrage rejette : produit non autorisé pour ce type
   - État : `REJETÉE`
   - Erreur : `FILT-002` (produit non autorisé)

5. **Correction de configuration**
   - Produit demande autorisation à Strong Father
   - Nouvelle soumission après autorisation
   - État : `CRÉÉE` → ... → `TRANSMISE`

6. **Erreur d'autorité**
   - Kind Mother refuse : contenu verrouillé
   - État : `ÉVALUÉE` → `RÉSOLUE`
   - Erreur : `AUTH-001` (refusé par l'autorité)

**Résumé :**
- Intentions soumises : 3
- Intentions rejetées : 2 (validation, filtrage)
- Intentions refusées : 1 (autorité)
- Durée totale : ~15 secondes

---

## 7. Patterns d'utilisation

### 7.1 Pattern : Polling de résultats

**Contexte :** Un produit qui ne peut pas recevoir de callbacks.

**Approche :**
1. Produit soumet l'intention
2. Produit reçoit `intention_id`
3. Produit interroge régulièrement : `GET /api/v1/intentions/{intention_id}/result`
4. Bonding Brother retourne le résultat quand disponible

**Avantages :**
- Pas besoin de callback URL
- Contrôle du timing par le produit

**Inconvénients :**
- Latence plus élevée
- Consommation de ressources (polling)

---

### 7.2 Pattern : Abonnement aux notifications

**Contexte :** Un produit souhaite être notifié de tous les changements de contenu.

**Approche :**
1. Produit s'abonne : `POST /api/v1/notifications/subscribe`
   ```json
   {
     "produit_id": "miyukini-publisher",
     "types": ["CONTENT_CREATED", "CONTENT_UPDATED", "CONTENT_DELETED"],
     "callback_url": "https://publisher.example.com/notifications"
   }
   ```

2. Bonding Brother notifie le produit à chaque événement correspondant

3. Produit peut se désabonner : `DELETE /api/v1/notifications/subscribe/{abonnement_id}`

**Avantages :**
- Notifications en temps réel
- Pas de polling nécessaire

**Inconvénients :**
- Nécessite un endpoint de callback
- Gestion de la disponibilité du callback

---

### 7.3 Pattern : Batch d'intentions

**Contexte :** Un produit souhaite soumettre plusieurs intentions liées.

**Approche :**
1. Produit soumet plusieurs intentions séquentiellement
2. Chaque intention a un `corrélation_id` commun
3. Produit peut suivre le traitement du batch via le `corrélation_id`

**Exemple :**
```json
// Intention 1
{
  "intention_id": "int-batch-001",
  "corrélation_id": "batch-import-2026-01-26",
  "type": "CREATE_CONTENT",
  ...
}

// Intention 2
{
  "intention_id": "int-batch-002",
  "corrélation_id": "batch-import-2026-01-26",
  "type": "CREATE_CONTENT",
  ...
}
```

**Avantages :**
- Traçabilité groupée
- Gestion d'erreurs simplifiée

**Inconvénients :**
- Pas de garantie d'ordre de traitement
- Pas de transaction atomique

---

## 8. Scénarios de performance

### 8.1 Scénario : Charge élevée

**Contexte :** 1000 intentions soumises simultanément.

**Comportement attendu :**
- Bonding Brother accepte toutes les intentions
- Traitement asynchrone en parallèle
- Chaque intention suit son propre flux
- Résultats retournés au fur et à mesure

**Métriques typiques :**
- Temps de validation : <10ms par intention
- Temps de traduction : <5ms par intention
- Temps de transmission : variable (dépend de l'autorité)
- Throughput : 100-500 intentions/seconde (selon configuration)

---

### 8.2 Scénario : Latence minimale

**Contexte :** Une intention nécessite une réponse rapide.

**Optimisations :**
- Utilisation de callback (pas de polling)
- Timeout court configuré
- Priorité élevée (si supporté)

**Métriques typiques :**
- Temps total : 100-500ms (si autorité rapide)
- Temps de traitement BB : <50ms
- Temps d'attente autorité : variable

---

## 9. Exemples de codes d'erreur

### 9.1 Erreur de validation

```json
{
  "statut": "REJETÉE",
  "erreur": {
    "code": "VAL-002",
    "message": "Le champ 'payload.titre' est obligatoire mais absent",
    "catégorie": "VALIDATION"
  }
}
```

### 9.2 Erreur de traduction

```json
{
  "statut": "REJETÉE",
  "erreur": {
    "code": "TRAD-001",
    "message": "Aucun mapping n'existe pour le type d'intention 'CUSTOM_ACTION' vers l'autorité 'kind_mother'",
    "catégorie": "TRADUCTION"
  }
}
```

### 9.3 Erreur d'autorité

```json
{
  "statut": "ERREUR",
  "erreur": {
    "code": "AUTH-001",
    "message": "L'autorité a refusé votre demande : permissions insuffisantes",
    "catégorie": "AUTORITÉ"
  }
}
```

### 9.4 Timeout

```json
{
  "statut": "ERREUR",
  "erreur": {
    "code": "TIMEOUT-002",
    "message": "L'autorité n'a pas répondu dans le délai imparti (30 secondes)",
    "catégorie": "TIMEOUT"
  }
}
```

---

## 10. Bonnes pratiques

### 10.1 Gestion des erreurs

**Recommandation :** Toujours gérer tous les statuts de résultat.

```typescript
switch (résultat.statut) {
  case "SUCCÈS":
    // Traiter le succès
    break;
  case "REFUSÉ":
    // Gérer le refus (afficher message, logger)
    break;
  case "ERREUR":
    // Gérer l'erreur (afficher message, logger, retry si transitoire)
    break;
  case "TIMEOUT":
    // Gérer le timeout (retry ou notification utilisateur)
    break;
}
```

---

### 10.2 Traçabilité

**Recommandation :** Toujours conserver les `intention_id` et `corrélation_id` pour le support.

```typescript
const intentionId = résultat.intention_id;
const corrélationId = intention.contexte.corrélation_id;

// Logger pour support
logger.info("Intention résolue", {
  intention_id: intentionId,
  corrélation_id: corrélationId,
  statut: résultat.statut
});
```

---

### 10.3 Gestion du mode offline

**Recommandation :** Toujours gérer les résultats différés.

```typescript
if (résultat.différé) {
  // Résultat d'une intention soumise en offline
  logger.info("Résultat différé reçu", {
    intention_id: résultat.intention_id,
    soumis_le: résultat.soumis_le,
    reçu_le: résultat.timestamp
  });
}
```

---

## 11. Statut contractuel

Ce document est **informatif, non normatif, et de statut EXEMPLES**. Il illustre l'utilisation de Bonding Brother mais ne remplace pas les contrats.

En cas de contradiction avec un contrat, le contrat prime toujours.

---

**Version :** 1.0  
**Date :** 2026-01-26  
**Statut :** EXEMPLES — Informatif  
**Dépendances :** 
- Bilateral Flow Contract v1.0
- Product Interface Contract v1.0
- Product-to-Ecosystem Flow v1.0
- Ecosystem-to-Product Flow v1.0
- Intent Model Contract v1.0
- Translation Contract v1.0