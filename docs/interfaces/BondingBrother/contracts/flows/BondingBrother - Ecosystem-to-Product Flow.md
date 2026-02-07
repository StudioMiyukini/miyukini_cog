# BondingBrother - Ecosystem-to-Product Flow

## 1. Contexte

Ce document définit le flux contractuel détaillé des informations depuis l'écosystème vers les produits via Bonding Brother. Il spécifie les étapes précises, les transformations, les validations, et les garanties associées au flux Écosystème → Produit.

Ce document complète la Section 5 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour la vue d'ensemble, le [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md) pour les règles de traduction, et le [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md) pour les règles de filtrage.

Ce flux respecte les [Lois d'Autonomie Système](../../../../reference/Miyukini%20Conceptual%20References%20-%20Lois%20Autonomie%20Systeme.md) : les résultats différés sont transmis même après une période d'isolement (**LOI-2**, **LOI-3**).

## 2. Portée / Scope

Ce document couvre :
- Le flux complet Écosystème → Produit (étape par étape)
- Les types d'informations transmises (notifications, événements, synchronisations)
- Les transformations appliquées à chaque étape
- Les validations et vérifications effectuées
- Les règles de distribution aux produits
- Les garanties de traitement
- Les cas d'erreur et leur gestion

Ce document **ne couvre pas** :
- Le flux inverse Produit → Écosystème (voir [Product-to-Ecosystem Flow](./BondingBrother%20-%20Product-to-Ecosystem%20Flow.md))
- Les détails de traduction (voir [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md))
- Les règles de filtrage (voir [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md))
- La gestion des erreurs (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md))
- Les protocoles d'intégration avec les autorités (voir les contrats d'intégration)

---

## 3. Principe fondamental

**Le flux Écosystème → Produit est unidirectionnel, asymétrique, et toujours adaptatif.**

L'écosystème (via les autorités) émet des informations dans son vocabulaire. Bonding Brother adapte ces informations au vocabulaire et aux attentes des produits, sans jamais demander aux produits de s'adapter à l'écosystème.

---

## 4. Types d'informations

### 4.1 Notifications

**Définition :** Informations proactives envoyées par l'écosystème pour informer un produit d'un événement ou d'un changement.

**Exemples :**
- Notification de création de contenu
- Notification de modification de hiérarchie
- Notification de changement de permission
- Notification de synchronisation disponible

**Caractéristiques :**
- Proactives (initiées par l'écosystème)
- Ciblées (destinées à un produit spécifique)
- Asynchrones (pas de réponse attendue)

### 4.2 Événements

**Définition :** Informations sur des événements survenus dans l'écosystème qui peuvent intéresser un produit.

**Exemples :**
- Événement de publication
- Événement de suppression
- Événement de migration
- Événement de synchronisation

**Caractéristiques :**
- Émis par les autorités
- Potentiellement multi-destinataires
- Asynchrones

### 4.3 Synchronisations

**Définition :** Informations de synchronisation envoyées pour mettre à jour un produit avec l'état actuel de l'écosystème.

**Exemples :**
- Synchronisation de contenu
- Synchronisation de hiérarchie
- Synchronisation de permissions
- Synchronisation d'état

**Caractéristiques :**
- Initiatives de l'écosystème
- Peuvent être différées (mode offline)
- Peuvent être incrémentielles ou complètes

### 4.4 Résultats différés

**Définition :** Résultats d'intentions précédemment soumises, disponibles après une période d'attente ou une reconnexion.

**Exemples :**
- Résultat d'une intention soumise en mode offline
- Résultat d'une intention à traitement long
- Résultat d'une intention différée

**Caractéristiques :**
- Liées à une intention précédente
- Peuvent être différées
- Asynchrones

---

## 5. Vue d'ensemble du flux

Le flux Écosystème → Produit traverse les étapes suivantes dans l'ordre strict :

```
AUTORITÉ (Kind Mother ou Strong Father)
  │
  ▼
[1] Réception de l'information
  │
  ▼
[2] Validation structurelle
  │
  ▼
[3] Identification des produits cibles
  │
  ▼
[4] Filtrage d'entrée
  │
  ▼
[5] Journalisation
  │
  ▼
[6] Traduction information → message
  │
  ▼
[7] Filtrage de sortie
  │
  ▼
[8] Distribution aux produits
  │
  ▼
[9] Transmission aux produits
  │
  ▼
PRODUITS
```

---

## 6. Étapes détaillées

### 6.1 Étape 1 : Réception de l'information

**Déclencheur :** Une autorité (Kind Mother ou Strong Father) émet une information via son adaptateur.

**Action :** Bonding Brother reçoit l'information dans le vocabulaire et le format de l'autorité.

**Types d'informations reçues :**
- Notifications
- Événements
- Synchronisations
- Résultats différés

**Validation :** Aucune validation à ce stade, uniquement réception.

**Résultat :** Information reçue, état `REÇUE`.

**Règle REC-01 : Réception immédiate**

Bonding Brother accepte immédiatement toute information structurellement valide, même si la validation sémantique échoue plus tard.

**Règle REC-02 : Pas de rejet précoce**

Aucun rejet n'est effectué à cette étape, sauf si l'information n'est pas un format valide.

---

### 6.2 Étape 2 : Validation structurelle

**Déclencheur :** Information reçue et parsée.

**Action :** Bonding Brother valide la structure de l'information selon le schéma défini par l'autorité émettrice.

**Validations effectuées :**
- Format valide (JSON, protobuf, etc.)
- Présence des champs obligatoires
- Types de données conformes
- Cohérence des métadonnées

**Résultat :**
- Si validation réussie : État `VALIDÉE`, passage à l'étape suivante
- Si validation échoue : État `REJETÉE`, information ignorée (journalisée pour audit)

**Règle VAL-01 : Validation stricte**

Toute information non conforme est rejetée immédiatement, sans tentative de correction.

**Règle VAL-02 : Pas de validation métier**

Bonding Brother ne valide pas le contenu métier de l'information. Il valide uniquement la structure.

---

### 6.3 Étape 3 : Identification des produits cibles

**Déclencheur :** Information validée structurellement.

**Action :** Bonding Brother identifie les produits qui doivent recevoir cette information.

**Méthodes d'identification :**
- **Explicite :** L'autorité spécifie explicitement les produits cibles
- **Implicite :** Bonding Brother détermine les produits cibles selon des règles :
  - Produits abonnés à un type d'événement
  - Produits concernés par une ressource
  - Produits ayant soumis une intention liée
  - Produits dans un contexte spécifique

**Résultat :** Liste de produits cibles identifiée, passage à l'étape suivante.

**Règle IDENT-01 : Identification déterministe**

L'identification des produits cibles est déterministe : pour une même information, les mêmes produits sont identifiés.

**Règle IDENT-02 : Pas de produits inconnus**

Seuls les produits enregistrés et actifs peuvent être cibles. Les produits inconnus sont ignorés.

**Règle IDENT-03 : Liste vide autorisée**

Si aucun produit n'est identifié, l'information est journalisée mais non distribuée.

---

### 6.4 Étape 4 : Filtrage d'entrée

**Déclencheur :** Produits cibles identifiés.

**Action :** Bonding Brother applique les règles de filtrage d'entrée pour déterminer si l'information doit être distribuée.

**Filtrages appliqués :**
- Vérification de la pertinence pour chaque produit
- Application des règles de sécurité
- Vérification des permissions de réception
- Filtrage des informations obsolètes ou redondantes

**Résultat :**
- Si filtrage accepte : État `FILTRÉE`, passage à l'étape suivante
- Si filtrage rejette pour tous les produits : État `REJETÉE`, information non distribuée

**Règle FILT-01 : Filtrage par produit**

Le filtrage est effectué pour chaque produit cible individuellement. Un produit peut recevoir l'information même si d'autres la rejettent.

**Règle FILT-02 : Pas de décision métier**

Le filtrage ne prend pas de décision métier. Il applique uniquement des règles techniques et de sécurité.

---

### 6.5 Étape 5 : Journalisation

**Déclencheur :** Information filtrée et prête pour distribution.

**Action :** Bonding Brother journalise l'information complète dans le journal d'audit.

**Informations journalisées :**
- Information complète (structure + données)
- Autorité émettrice
- Produits cibles identifiés
- Timestamp de réception
- État actuel (`JOURNALISÉE`)

**Résultat :** Information journalisée, état `JOURNALISÉE`, passage à l'étape suivante.

**Règle JOUR-01 : Journalisation systématique**

Toute information qui atteint cette étape est journalisée, sans exception.

**Règle JOUR-02 : Immuabilité**

Une fois journalisée, l'information ne peut être modifiée.

**Règle JOUR-03 : Traçabilité complète**

Le journal permet de tracer l'information complète depuis sa réception jusqu'à sa distribution.

---

### 6.6 Étape 6 : Traduction information → message

**Déclencheur :** Information journalisée et prête pour traduction.

**Action :** Bonding Brother traduit l'information (vocabulaire autorité) en message (vocabulaire produit) selon les règles du Translation Contract.

**Transformations appliquées :**
- Mapping des types d'information vers les types de message
- Traduction champ par champ des données
- Adaptation du format au vocabulaire du produit
- Préservation de la sémantique

**Résultat :** Message traduit pour chaque produit cible, prêt pour filtrage.

**Règle TRAD-01 : Traduction par produit**

Chaque produit peut recevoir une traduction adaptée à son vocabulaire spécifique.

**Règle TRAD-02 : Fidélité sémantique**

La traduction préserve intégralement la sémantique de l'information. Aucune interprétation ni enrichissement métier n'est autorisé.

**Règle TRAD-03 : Déterminisme**

Pour une même information et un même produit, la traduction produit toujours le même message.

---

### 6.7 Étape 7 : Filtrage de sortie

**Déclencheur :** Message traduit et prêt.

**Action :** Bonding Brother applique les règles de filtrage de sortie pour chaque produit cible.

**Filtrages appliqués :**
- Suppression des informations sensibles non autorisées pour le produit
- Adaptation des données selon les permissions du produit
- Projection des champs nécessaires uniquement
- Masquage des détails internes de l'autorité

**Résultat :** Message filtré pour chaque produit, prêt pour transmission.

**Règle FILT-S-01 : Filtrage protecteur**

Le filtrage de sortie protège les autorités en ne transmettant que les informations autorisées.

**Règle FILT-S-02 : Respect des permissions**

Le filtrage respecte les permissions de chaque produit. Les informations non autorisées sont omises.

**Règle FILT-S-03 : Filtrage par produit**

Chaque produit peut recevoir une version filtrée différente selon ses permissions.

---

### 6.8 Étape 8 : Distribution aux produits

**Déclencheur :** Messages filtrés et prêts.

**Action :** Bonding Brother distribue les messages aux produits cibles via l'interface `INotificationSubscription` ou `IResultConsumption`.

**Modes de distribution :**
- **Push :** Transmission immédiate au produit (si connecté)
- **Queue :** Mise en file d'attente pour transmission ultérieure (si déconnecté)
- **Broadcast :** Distribution à tous les produits cibles simultanément

**Résultat :** Messages distribués aux produits, passage à l'étape suivante.

**Règle DIST-01 : Distribution asynchrone**

La distribution est asynchrone. Bonding Brother ne bloque pas en attendant la réception par les produits.

**Règle DIST-02 : Gestion des déconnexions**

Si un produit est déconnecté, le message est mis en queue et transmis lors de la reconnexion.

**Règle DIST-03 : Pas de garantie de réception**

Bonding Brother garantit la transmission, pas la réception. Les produits peuvent être déconnectés ou indisponibles.

---

### 6.9 Étape 9 : Transmission aux produits

**Déclencheur :** Messages distribués et produits identifiés.

**Action :** Bonding Brother transmet le message à chaque produit cible via l'interface appropriée.

**Interfaces utilisées :**
- `INotificationSubscription` : Pour les notifications et événements
- `IResultConsumption` : Pour les résultats différés
- `ISyncSubscription` : Pour les synchronisations

**Contenu transmis :**
- Type de message
- Données traduites et filtrées
- Métadonnées de traçabilité
- Timestamp

**Résultat :** Messages transmis aux produits, état `TRANSMISE`, cycle complet terminé.

**Règle TRANS-01 : Transmission fidèle**

Le message est transmis intégralement au produit, sans modification supplémentaire.

**Règle TRANS-02 : Journalisation finale**

La transmission du message est journalisée pour compléter la traçabilité.

**Règle TRANS-03 : Gestion des erreurs de transmission**

Si la transmission échoue pour un produit, le message est mis en queue pour retry ultérieur.

---

## 7. Garanties du flux

### 7.1 Garantie d'ordre

**Engagement :** Les étapes du flux sont exécutées dans l'ordre strict défini. Aucune étape ne peut être sautée ou réordonnée.

**Exception :** En cas d'erreur, le flux peut être interrompu et l'information peut être ignorée ou mise en queue.

### 7.2 Garantie de traçabilité

**Engagement :** Toute information qui traverse le flux est traçable de bout en bout. Le journal contient toutes les informations nécessaires pour reconstruire le flux complet.

### 7.3 Garantie de fidélité

**Engagement :** La sémantique de l'information est préservée lors de la traduction et de la transmission. Les données sont adaptées au format, pas au contenu.

### 7.4 Garantie de distribution

**Engagement :** Bonding Brother garantit la transmission de l'information aux produits cibles identifiés, sous réserve de leur disponibilité.

---

## 8. Gestion des erreurs

### 8.1 Points d'échec

Le flux peut échouer aux étapes suivantes :
- **Étape 2** : Validation structurelle échouée → Information ignorée
- **Étape 3** : Aucun produit cible identifié → Information journalisée mais non distribuée
- **Étape 4** : Filtrage rejette pour tous les produits → Information non distribuée
- **Étape 6** : Traduction échouée → Information ignorée pour le produit concerné
- **Étape 9** : Transmission échouée → Message mis en queue pour retry

### 8.2 Traitement des erreurs

**Règle ERR-01 : Journalisation des erreurs**

Toutes les erreurs sont journalisées pour audit et analyse.

**Règle ERR-02 : Pas de retry automatique pour erreurs structurelles**

Les erreurs de validation ou de traduction ne sont pas retentées automatiquement (ce ne sont pas des erreurs transitoires).

**Règle ERR-03 : Retry pour erreurs de transmission**

Les erreurs de transmission sont retentées lors de la reconnexion du produit.

**Règle ERR-04 : Isolation des erreurs**

Une erreur pour un produit n'affecte pas la distribution aux autres produits.

---

## 9. Mode offline

### 9.1 Comportement en mode offline

En mode offline, les étapes 8 et 9 peuvent être différées :

- **Étape 8** : La distribution est mise en queue
- **Étape 9** : La transmission est différée jusqu'à la reconnexion

Les étapes 1 à 7 continuent de fonctionner normalement.

### 9.2 Synchronisation à la reconnexion

Lors de la reconnexion d'un produit, Bonding Brother :
1. Transmet tous les messages en queue pour ce produit
2. Respecte l'ordre chronologique
3. Gère les doublons et les messages obsolètes

Voir [Sync & Reconnection Contract](../offline/BondingBrother%20-%20Sync%20%26%20Reconnection%20Contract.md) pour les détails.

---

## 10. Performance et limites

### 10.1 Délais

**Délai de traitement :** Le délai total dépend de :
- Temps de validation (instantané)
- Temps d'identification des produits (instantané)
- Temps de filtrage (instantané)
- Temps de traduction (instantané)
- Temps de filtrage de sortie (instantané)
- Temps de transmission aux produits (variable)

**Délai de distribution :** Asynchrone, non bloquant

### 10.2 Limites

**Taille maximale d'information :** 1 MB (configurable)
**Nombre de produits cibles :** Illimité (sous réserve de ressources)
**Taille de la queue :** Configurable par produit

---

## 11. Exemples

### 11.1 Flux complet : Notification de création de contenu

```
1. Kind Mother émet notification : content créé (content_id = "content-123")
2. Validation structurelle : ✅
3. Identification produits : ["miyukini-cms", "miyukini-publisher"]
4. Filtrage d'entrée : ✅ pour les deux produits
5. Journalisation : ✅
6. Traduction : content créé → notification_content_created (vocabulaire produit)
7. Filtrage de sortie : ✅ (données adaptées selon permissions)
8. Distribution : Push aux deux produits
9. Transmission : ✅ aux deux produits
```

### 11.2 Flux avec filtrage : Information non pertinente

```
1. Strong Father émet événement : permission modifiée (user_id = "user-456")
2. Validation structurelle : ✅
3. Identification produits : ["miyukini-cms", "miyukini-auth"]
4. Filtrage d'entrée : 
   - "miyukini-cms" : ✅ (intéressé par les permissions)
   - "miyukini-auth" : ❌ (non concerné par cette modification)
5. Journalisation : ✅
6. Traduction : Pour "miyukini-cms" uniquement
7. Filtrage de sortie : ✅
8. Distribution : Push à "miyukini-cms" uniquement
9. Transmission : ✅ à "miyukini-cms"
```

### 11.3 Flux avec queue : Produit déconnecté

```
1-7. (identique à l'exemple 11.1)
8. Distribution : 
   - "miyukini-cms" : Push immédiat (connecté) ✅
   - "miyukini-publisher" : Queue (déconnecté) ⏳
9. Transmission : 
   - "miyukini-cms" : ✅
   - "miyukini-publisher" : Mise en queue
   
[Reconnexion de "miyukini-publisher"]
→ Transmission du message en queue ✅
```

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il établit le flux détaillé que Bonding Brother doit respecter pour traiter les informations de l'écosystème vers les produits.

Toute implémentation du flux Écosystème → Produit doit respecter ce contrat. Toute violation entraîne un comportement non conforme.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT — Normatif  
**Dépendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0 (Section 5)
- [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) v2.0
- [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md) v2.0
- [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md) v2.0
- [Error & Rejection Model](../error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md) v2.0
