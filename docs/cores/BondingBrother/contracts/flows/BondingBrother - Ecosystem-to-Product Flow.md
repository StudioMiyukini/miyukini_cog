# BondingBrother - Ecosystem-to-Product Flow

## 1. Contexte

Ce document dÃ©finit le flux contractuel dÃ©taillÃ© des informations depuis l'Ã©cosystÃ¨me vers les produits via Bonding Brother. Il spÃ©cifie les Ã©tapes prÃ©cises, les transformations, les validations, et les garanties associÃ©es au flux Ã‰cosystÃ¨me â†’ Produit.

Ce document complÃ¨te la Section 5 de la [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) et s'appuie sur le [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) pour la vue d'ensemble, le [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md) pour les rÃ¨gles de traduction, et le [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md) pour les rÃ¨gles de filtrage.

Ce flux respecte les [Lois d'Autonomie SystÃ¨me](..//..//..//..//miyukini-webway-system//reference//_index.md) : les rÃ©sultats diffÃ©rÃ©s sont transmis mÃªme aprÃ¨s une pÃ©riode d'isolement (**LOI-2**, **LOI-3**).

## 2. PortÃ©e / Scope

Ce document couvre :
- Le flux complet Ã‰cosystÃ¨me â†’ Produit (Ã©tape par Ã©tape)
- Les types d'informations transmises (notifications, Ã©vÃ©nements, synchronisations)
- Les transformations appliquÃ©es Ã  chaque Ã©tape
- Les validations et vÃ©rifications effectuÃ©es
- Les rÃ¨gles de distribution aux produits
- Les garanties de traitement
- Les cas d'erreur et leur gestion

Ce document **ne couvre pas** :
- Le flux inverse Produit â†’ Ã‰cosystÃ¨me (voir [Product-to-Ecosystem Flow](./BondingBrother%20-%20Product-to-Ecosystem%20Flow.md))
- Les dÃ©tails de traduction (voir [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md))
- Les rÃ¨gles de filtrage (voir [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md))
- La gestion des erreurs (voir [Error & Rejection Model](../error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md))
- Les protocoles d'intÃ©gration avec les autoritÃ©s (voir les contrats d'intÃ©gration)

---

## 3. Principe fondamental

**Le flux Ã‰cosystÃ¨me â†’ Produit est unidirectionnel, asymÃ©trique, et toujours adaptatif.**

L'Ã©cosystÃ¨me (via les autoritÃ©s) Ã©met des informations dans son vocabulaire. Bonding Brother adapte ces informations au vocabulaire et aux attentes des produits, sans jamais demander aux produits de s'adapter Ã  l'Ã©cosystÃ¨me.

---

## 4. Types d'informations

### 4.1 Notifications

**DÃ©finition :** Informations proactives envoyÃ©es par l'Ã©cosystÃ¨me pour informer un produit d'un Ã©vÃ©nement ou d'un changement.

**Exemples :**
- Notification de crÃ©ation de contenu
- Notification de modification de hiÃ©rarchie
- Notification de changement de permission
- Notification de synchronisation disponible

**CaractÃ©ristiques :**
- Proactives (initiÃ©es par l'Ã©cosystÃ¨me)
- CiblÃ©es (destinÃ©es Ã  un produit spÃ©cifique)
- Asynchrones (pas de rÃ©ponse attendue)

### 4.2 Ã‰vÃ©nements

**DÃ©finition :** Informations sur des Ã©vÃ©nements survenus dans l'Ã©cosystÃ¨me qui peuvent intÃ©resser un produit.

**Exemples :**
- Ã‰vÃ©nement de publication
- Ã‰vÃ©nement de suppression
- Ã‰vÃ©nement de migration
- Ã‰vÃ©nement de synchronisation

**CaractÃ©ristiques :**
- Ã‰mis par les autoritÃ©s
- Potentiellement multi-destinataires
- Asynchrones

### 4.3 Synchronisations

**DÃ©finition :** Informations de synchronisation envoyÃ©es pour mettre Ã  jour un produit avec l'Ã©tat actuel de l'Ã©cosystÃ¨me.

**Exemples :**
- Synchronisation de contenu
- Synchronisation de hiÃ©rarchie
- Synchronisation de permissions
- Synchronisation d'Ã©tat

**CaractÃ©ristiques :**
- Initiatives de l'Ã©cosystÃ¨me
- Peuvent Ãªtre diffÃ©rÃ©es (mode offline)
- Peuvent Ãªtre incrÃ©mentielles ou complÃ¨tes

### 4.4 RÃ©sultats diffÃ©rÃ©s

**DÃ©finition :** RÃ©sultats d'intentions prÃ©cÃ©demment soumises, disponibles aprÃ¨s une pÃ©riode d'attente ou une reconnexion.

**Exemples :**
- RÃ©sultat d'une intention soumise en mode offline
- RÃ©sultat d'une intention Ã  traitement long
- RÃ©sultat d'une intention diffÃ©rÃ©e

**CaractÃ©ristiques :**
- LiÃ©es Ã  une intention prÃ©cÃ©dente
- Peuvent Ãªtre diffÃ©rÃ©es
- Asynchrones

---

## 5. Vue d'ensemble du flux

Le flux Ã‰cosystÃ¨me â†’ Produit traverse les Ã©tapes suivantes dans l'ordre strict :

```
AUTORITÃ‰ (Kind Mother ou Strong Father)
  â”‚
  â–¼
[1] RÃ©ception de l'information
  â”‚
  â–¼
[2] Validation structurelle
  â”‚
  â–¼
[3] Identification des produits cibles
  â”‚
  â–¼
[4] Filtrage d'entrÃ©e
  â”‚
  â–¼
[5] Journalisation
  â”‚
  â–¼
[6] Traduction information â†’ message
  â”‚
  â–¼
[7] Filtrage de sortie
  â”‚
  â–¼
[8] Distribution aux produits
  â”‚
  â–¼
[9] Transmission aux produits
  â”‚
  â–¼
PRODUITS
```

---

## 6. Ã‰tapes dÃ©taillÃ©es

### 6.1 Ã‰tape 1 : RÃ©ception de l'information

**DÃ©clencheur :** Une autoritÃ© (Kind Mother ou Strong Father) Ã©met une information via son adaptateur.

**Action :** Bonding Brother reÃ§oit l'information dans le vocabulaire et le format de l'autoritÃ©.

**Types d'informations reÃ§ues :**
- Notifications
- Ã‰vÃ©nements
- Synchronisations
- RÃ©sultats diffÃ©rÃ©s

**Validation :** Aucune validation Ã  ce stade, uniquement rÃ©ception.

**RÃ©sultat :** Information reÃ§ue, Ã©tat `REÃ‡UE`.

**RÃ¨gle REC-01 : RÃ©ception immÃ©diate**

Bonding Brother accepte immÃ©diatement toute information structurellement valide, mÃªme si la validation sÃ©mantique Ã©choue plus tard.

**RÃ¨gle REC-02 : Pas de rejet prÃ©coce**

Aucun rejet n'est effectuÃ© Ã  cette Ã©tape, sauf si l'information n'est pas un format valide.

---

### 6.2 Ã‰tape 2 : Validation structurelle

**DÃ©clencheur :** Information reÃ§ue et parsÃ©e.

**Action :** Bonding Brother valide la structure de l'information selon le schÃ©ma dÃ©fini par l'autoritÃ© Ã©mettrice.

**Validations effectuÃ©es :**
- Format valide (JSON, protobuf, etc.)
- PrÃ©sence des champs obligatoires
- Types de donnÃ©es conformes
- CohÃ©rence des mÃ©tadonnÃ©es

**RÃ©sultat :**
- Si validation rÃ©ussie : Ã‰tat `VALIDÃ‰E`, passage Ã  l'Ã©tape suivante
- Si validation Ã©choue : Ã‰tat `REJETÃ‰E`, information ignorÃ©e (journalisÃ©e pour audit)

**RÃ¨gle VAL-01 : Validation stricte**

Toute information non conforme est rejetÃ©e immÃ©diatement, sans tentative de correction.

**RÃ¨gle VAL-02 : Pas de validation mÃ©tier**

Bonding Brother ne valide pas le contenu mÃ©tier de l'information. Il valide uniquement la structure.

---

### 6.3 Ã‰tape 3 : Identification des produits cibles

**DÃ©clencheur :** Information validÃ©e structurellement.

**Action :** Bonding Brother identifie les produits qui doivent recevoir cette information.

**MÃ©thodes d'identification :**
- **Explicite :** L'autoritÃ© spÃ©cifie explicitement les produits cibles
- **Implicite :** Bonding Brother dÃ©termine les produits cibles selon des rÃ¨gles :
  - Produits abonnÃ©s Ã  un type d'Ã©vÃ©nement
  - Produits concernÃ©s par une ressource
  - Produits ayant soumis une intention liÃ©e
  - Produits dans un contexte spÃ©cifique

**RÃ©sultat :** Liste de produits cibles identifiÃ©e, passage Ã  l'Ã©tape suivante.

**RÃ¨gle IDENT-01 : Identification dÃ©terministe**

L'identification des produits cibles est dÃ©terministe : pour une mÃªme information, les mÃªmes produits sont identifiÃ©s.

**RÃ¨gle IDENT-02 : Pas de produits inconnus**

Seuls les produits enregistrÃ©s et actifs peuvent Ãªtre cibles. Les produits inconnus sont ignorÃ©s.

**RÃ¨gle IDENT-03 : Liste vide autorisÃ©e**

Si aucun produit n'est identifiÃ©, l'information est journalisÃ©e mais non distribuÃ©e.

---

### 6.4 Ã‰tape 4 : Filtrage d'entrÃ©e

**DÃ©clencheur :** Produits cibles identifiÃ©s.

**Action :** Bonding Brother applique les rÃ¨gles de filtrage d'entrÃ©e pour dÃ©terminer si l'information doit Ãªtre distribuÃ©e.

**Filtrages appliquÃ©s :**
- VÃ©rification de la pertinence pour chaque produit
- Application des rÃ¨gles de sÃ©curitÃ©
- VÃ©rification des permissions de rÃ©ception
- Filtrage des informations obsolÃ¨tes ou redondantes

**RÃ©sultat :**
- Si filtrage accepte : Ã‰tat `FILTRÃ‰E`, passage Ã  l'Ã©tape suivante
- Si filtrage rejette pour tous les produits : Ã‰tat `REJETÃ‰E`, information non distribuÃ©e

**RÃ¨gle FILT-01 : Filtrage par produit**

Le filtrage est effectuÃ© pour chaque produit cible individuellement. Un produit peut recevoir l'information mÃªme si d'autres la rejettent.

**RÃ¨gle FILT-02 : Pas de dÃ©cision mÃ©tier**

Le filtrage ne prend pas de dÃ©cision mÃ©tier. Il applique uniquement des rÃ¨gles techniques et de sÃ©curitÃ©.

---

### 6.5 Ã‰tape 5 : Journalisation

**DÃ©clencheur :** Information filtrÃ©e et prÃªte pour distribution.

**Action :** Bonding Brother journalise l'information complÃ¨te dans le journal d'audit.

**Informations journalisÃ©es :**
- Information complÃ¨te (structure + donnÃ©es)
- AutoritÃ© Ã©mettrice
- Produits cibles identifiÃ©s
- Timestamp de rÃ©ception
- Ã‰tat actuel (`JOURNALISÃ‰E`)

**RÃ©sultat :** Information journalisÃ©e, Ã©tat `JOURNALISÃ‰E`, passage Ã  l'Ã©tape suivante.

**RÃ¨gle JOUR-01 : Journalisation systÃ©matique**

Toute information qui atteint cette Ã©tape est journalisÃ©e, sans exception.

**RÃ¨gle JOUR-02 : ImmuabilitÃ©**

Une fois journalisÃ©e, l'information ne peut Ãªtre modifiÃ©e.

**RÃ¨gle JOUR-03 : TraÃ§abilitÃ© complÃ¨te**

Le journal permet de tracer l'information complÃ¨te depuis sa rÃ©ception jusqu'Ã  sa distribution.

---

### 6.6 Ã‰tape 6 : Traduction information â†’ message

**DÃ©clencheur :** Information journalisÃ©e et prÃªte pour traduction.

**Action :** Bonding Brother traduit l'information (vocabulaire autoritÃ©) en message (vocabulaire produit) selon les rÃ¨gles du Translation Contract.

**Transformations appliquÃ©es :**
- Mapping des types d'information vers les types de message
- Traduction champ par champ des donnÃ©es
- Adaptation du format au vocabulaire du produit
- PrÃ©servation de la sÃ©mantique

**RÃ©sultat :** Message traduit pour chaque produit cible, prÃªt pour filtrage.

**RÃ¨gle TRAD-01 : Traduction par produit**

Chaque produit peut recevoir une traduction adaptÃ©e Ã  son vocabulaire spÃ©cifique.

**RÃ¨gle TRAD-02 : FidÃ©litÃ© sÃ©mantique**

La traduction prÃ©serve intÃ©gralement la sÃ©mantique de l'information. Aucune interprÃ©tation ni enrichissement mÃ©tier n'est autorisÃ©.

**RÃ¨gle TRAD-03 : DÃ©terminisme**

Pour une mÃªme information et un mÃªme produit, la traduction produit toujours le mÃªme message.

---

### 6.7 Ã‰tape 7 : Filtrage de sortie

**DÃ©clencheur :** Message traduit et prÃªt.

**Action :** Bonding Brother applique les rÃ¨gles de filtrage de sortie pour chaque produit cible.

**Filtrages appliquÃ©s :**
- Suppression des informations sensibles non autorisÃ©es pour le produit
- Adaptation des donnÃ©es selon les permissions du produit
- Projection des champs nÃ©cessaires uniquement
- Masquage des dÃ©tails internes de l'autoritÃ©

**RÃ©sultat :** Message filtrÃ© pour chaque produit, prÃªt pour transmission.

**RÃ¨gle FILT-S-01 : Filtrage protecteur**

Le filtrage de sortie protÃ¨ge les autoritÃ©s en ne transmettant que les informations autorisÃ©es.

**RÃ¨gle FILT-S-02 : Respect des permissions**

Le filtrage respecte les permissions de chaque produit. Les informations non autorisÃ©es sont omises.

**RÃ¨gle FILT-S-03 : Filtrage par produit**

Chaque produit peut recevoir une version filtrÃ©e diffÃ©rente selon ses permissions.

---

### 6.8 Ã‰tape 8 : Distribution aux produits

**DÃ©clencheur :** Messages filtrÃ©s et prÃªts.

**Action :** Bonding Brother distribue les messages aux produits cibles via l'interface `INotificationSubscription` ou `IResultConsumption`.

**Modes de distribution :**
- **Push :** Transmission immÃ©diate au produit (si connectÃ©)
- **Queue :** Mise en file d'attente pour transmission ultÃ©rieure (si dÃ©connectÃ©)
- **Broadcast :** Distribution Ã  tous les produits cibles simultanÃ©ment

**RÃ©sultat :** Messages distribuÃ©s aux produits, passage Ã  l'Ã©tape suivante.

**RÃ¨gle DIST-01 : Distribution asynchrone**

La distribution est asynchrone. Bonding Brother ne bloque pas en attendant la rÃ©ception par les produits.

**RÃ¨gle DIST-02 : Gestion des dÃ©connexions**

Si un produit est dÃ©connectÃ©, le message est mis en queue et transmis lors de la reconnexion.

**RÃ¨gle DIST-03 : Pas de garantie de rÃ©ception**

Bonding Brother garantit la transmission, pas la rÃ©ception. Les produits peuvent Ãªtre dÃ©connectÃ©s ou indisponibles.

---

### 6.9 Ã‰tape 9 : Transmission aux produits

**DÃ©clencheur :** Messages distribuÃ©s et produits identifiÃ©s.

**Action :** Bonding Brother transmet le message Ã  chaque produit cible via l'interface appropriÃ©e.

**Interfaces utilisÃ©es :**
- `INotificationSubscription` : Pour les notifications et Ã©vÃ©nements
- `IResultConsumption` : Pour les rÃ©sultats diffÃ©rÃ©s
- `ISyncSubscription` : Pour les synchronisations

**Contenu transmis :**
- Type de message
- DonnÃ©es traduites et filtrÃ©es
- MÃ©tadonnÃ©es de traÃ§abilitÃ©
- Timestamp

**RÃ©sultat :** Messages transmis aux produits, Ã©tat `TRANSMISE`, cycle complet terminÃ©.

**RÃ¨gle TRANS-01 : Transmission fidÃ¨le**

Le message est transmis intÃ©gralement au produit, sans modification supplÃ©mentaire.

**RÃ¨gle TRANS-02 : Journalisation finale**

La transmission du message est journalisÃ©e pour complÃ©ter la traÃ§abilitÃ©.

**RÃ¨gle TRANS-03 : Gestion des erreurs de transmission**

Si la transmission Ã©choue pour un produit, le message est mis en queue pour retry ultÃ©rieur.

---

## 7. Garanties du flux

### 7.1 Garantie d'ordre

**Engagement :** Les Ã©tapes du flux sont exÃ©cutÃ©es dans l'ordre strict dÃ©fini. Aucune Ã©tape ne peut Ãªtre sautÃ©e ou rÃ©ordonnÃ©e.

**Exception :** En cas d'erreur, le flux peut Ãªtre interrompu et l'information peut Ãªtre ignorÃ©e ou mise en queue.

### 7.2 Garantie de traÃ§abilitÃ©

**Engagement :** Toute information qui traverse le flux est traÃ§able de bout en bout. Le journal contient toutes les informations nÃ©cessaires pour reconstruire le flux complet.

### 7.3 Garantie de fidÃ©litÃ©

**Engagement :** La sÃ©mantique de l'information est prÃ©servÃ©e lors de la traduction et de la transmission. Les donnÃ©es sont adaptÃ©es au format, pas au contenu.

### 7.4 Garantie de distribution

**Engagement :** Bonding Brother garantit la transmission de l'information aux produits cibles identifiÃ©s, sous rÃ©serve de leur disponibilitÃ©.

---

## 8. Gestion des erreurs

### 8.1 Points d'Ã©chec

Le flux peut Ã©chouer aux Ã©tapes suivantes :
- **Ã‰tape 2** : Validation structurelle Ã©chouÃ©e â†’ Information ignorÃ©e
- **Ã‰tape 3** : Aucun produit cible identifiÃ© â†’ Information journalisÃ©e mais non distribuÃ©e
- **Ã‰tape 4** : Filtrage rejette pour tous les produits â†’ Information non distribuÃ©e
- **Ã‰tape 6** : Traduction Ã©chouÃ©e â†’ Information ignorÃ©e pour le produit concernÃ©
- **Ã‰tape 9** : Transmission Ã©chouÃ©e â†’ Message mis en queue pour retry

### 8.2 Traitement des erreurs

**RÃ¨gle ERR-01 : Journalisation des erreurs**

Toutes les erreurs sont journalisÃ©es pour audit et analyse.

**RÃ¨gle ERR-02 : Pas de retry automatique pour erreurs structurelles**

Les erreurs de validation ou de traduction ne sont pas retentÃ©es automatiquement (ce ne sont pas des erreurs transitoires).

**RÃ¨gle ERR-03 : Retry pour erreurs de transmission**

Les erreurs de transmission sont retentÃ©es lors de la reconnexion du produit.

**RÃ¨gle ERR-04 : Isolation des erreurs**

Une erreur pour un produit n'affecte pas la distribution aux autres produits.

---

## 9. Mode offline

### 9.1 Comportement en mode offline

En mode offline, les Ã©tapes 8 et 9 peuvent Ãªtre diffÃ©rÃ©es :

- **Ã‰tape 8** : La distribution est mise en queue
- **Ã‰tape 9** : La transmission est diffÃ©rÃ©e jusqu'Ã  la reconnexion

Les Ã©tapes 1 Ã  7 continuent de fonctionner normalement.

### 9.2 Synchronisation Ã  la reconnexion

Lors de la reconnexion d'un produit, Bonding Brother :
1. Transmet tous les messages en queue pour ce produit
2. Respecte l'ordre chronologique
3. GÃ¨re les doublons et les messages obsolÃ¨tes

Voir [Sync & Reconnection Contract](../offline/BondingBrother%20-%20Sync%20%26%20Reconnection%20Contract.md) pour les dÃ©tails.

---

## 10. Performance et limites

### 10.1 DÃ©lais

**DÃ©lai de traitement :** Le dÃ©lai total dÃ©pend de :
- Temps de validation (instantanÃ©)
- Temps d'identification des produits (instantanÃ©)
- Temps de filtrage (instantanÃ©)
- Temps de traduction (instantanÃ©)
- Temps de filtrage de sortie (instantanÃ©)
- Temps de transmission aux produits (variable)

**DÃ©lai de distribution :** Asynchrone, non bloquant

### 10.2 Limites

**Taille maximale d'information :** 1 MB (configurable)
**Nombre de produits cibles :** IllimitÃ© (sous rÃ©serve de ressources)
**Taille de la queue :** Configurable par produit

---

## 11. Exemples

### 11.1 Flux complet : Notification de crÃ©ation de contenu

```
1. Kind Mother Ã©met notification : content crÃ©Ã© (content_id = "content-123")
2. Validation structurelle : âœ…
3. Identification produits : ["miyukini-cms", "miyukini-publisher"]
4. Filtrage d'entrÃ©e : âœ… pour les deux produits
5. Journalisation : âœ…
6. Traduction : content crÃ©Ã© â†’ notification_content_created (vocabulaire produit)
7. Filtrage de sortie : âœ… (donnÃ©es adaptÃ©es selon permissions)
8. Distribution : Push aux deux produits
9. Transmission : âœ… aux deux produits
```

### 11.2 Flux avec filtrage : Information non pertinente

```
1. Strong Father Ã©met Ã©vÃ©nement : permission modifiÃ©e (user_id = "user-456")
2. Validation structurelle : âœ…
3. Identification produits : ["miyukini-cms", "miyukini-auth"]
4. Filtrage d'entrÃ©e : 
   - "miyukini-cms" : âœ… (intÃ©ressÃ© par les permissions)
   - "miyukini-auth" : âŒ (non concernÃ© par cette modification)
5. Journalisation : âœ…
6. Traduction : Pour "miyukini-cms" uniquement
7. Filtrage de sortie : âœ…
8. Distribution : Push Ã  "miyukini-cms" uniquement
9. Transmission : âœ… Ã  "miyukini-cms"
```

### 11.3 Flux avec queue : Produit dÃ©connectÃ©

```
1-7. (identique Ã  l'exemple 11.1)
8. Distribution : 
   - "miyukini-cms" : Push immÃ©diat (connectÃ©) âœ…
   - "miyukini-publisher" : Queue (dÃ©connectÃ©) â³
9. Transmission : 
   - "miyukini-cms" : âœ…
   - "miyukini-publisher" : Mise en queue
   
[Reconnexion de "miyukini-publisher"]
â†’ Transmission du message en queue âœ…
```

---

## 12. Statut contractuel

Ce document est **contractuel, normatif, et de statut CONTRAT**. Il Ã©tablit le flux dÃ©taillÃ© que Bonding Brother doit respecter pour traiter les informations de l'Ã©cosystÃ¨me vers les produits.

Toute implÃ©mentation du flux Ã‰cosystÃ¨me â†’ Produit doit respecter ce contrat. Toute violation entraÃ®ne un comportement non conforme.

---

**Version :** 2.0  
**Date :** 2026-01-28  
**Statut :** CONTRAT â€” Normatif  
**DÃ©pendances :** 
- [Documentation Fondatrice](../../foundation/BondingBrother%20-%20Documentation%20Fondatrice.md) v2.0 (Section 5)
- [Bilateral Flow Contract](./BondingBrother%20-%20Bilateral%20Flow%20Contract.md) v2.0
- [Translation Contract](../intent/BondingBrother%20-%20Translation%20Contract.md) v2.0
- [Filtering & Projection Contract](../intent/BondingBrother%20-%20Filtering%20%26%20Projection%20Contract.md) v2.0
- [Error & Rejection Model](../error/BondingBrother%20-%20Error%20%26%20Rejection%20Model.md) v2.0

