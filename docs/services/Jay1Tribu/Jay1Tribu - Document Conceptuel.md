# Jay1Tribu — Document Conceptuel

## Contexte

**Jay1Tribu** est un **Service de messagerie pair-à-pair (P2P)** au sein de l'écosystème Miyukini COG (Core-Orchestrated Governance Environment).

Son objectif est de fournir une messagerie instantanée entre COGs — discussions, envoi de fichiers et d'images — dans laquelle **les archives restent uniquement chez les participants**, dans leur base de données locale (leur COG). Aucun serveur central ne conserve les conversations à l'insu des utilisateurs.

Jay1Tribu est conçu comme un **Service Inter-COG** (Type 3) : les échanges ont lieu entre COGs connectés au MWS (Miyukini Webway System). Le service s'appuie sur la présence, la découverte et le transport fournis par le Webway pour acheminer les messages, tout en garantissant que **tout ce qui transite est crypté** et que **seuls les participants conservent les données**.

> **Philosophie fondatrice :** Remplacer les systèmes de messagerie qui conservent les données à l'insu de leurs utilisateurs. Avec Jay1Tribu, les discussions, fichiers et images sont hébergés chez les utilisateurs ; chaque COG ne garde que ce dont il a été partie prenante.

Ce document est le **document conceptuel** du Service Jay1Tribu : il en fixe la raison d'être, le positionnement architectural, le modèle conceptuel (tribus, salons, amis, rôles), les règles de confidentialité et de souveraineté des données, et les interactions avec le MWS et les Cores. Il s'adresse aux équipes produit, architecture, sécurité et à toute partie prenante de l'écosystème Miyukini.

## Portée / Scope

- **Applicable à :** Vision produit, concepts métier, architecture conceptuelle, modèle de données conceptuel, gouvernance, sécurité
- **Audience :** Architectes, équipes produit, équipes sécurité, parties prenantes
- **Statut :** Document conceptuel normatif — référence pour la conception du Service

### Hors périmètre

- Aucun code ni implémentation technique
- Aucun choix UI/UX détaillé
- Aucun protocole de cryptographie spécifique (le principe « tout crypté » est fixé ; les mécanismes seront spécifiés ailleurs)
- Aucun schéma de base de données ni API

---

## 1. Contexte général

Jay1Tribu offre une expérience fonctionnelle comparable à des messageries de type Discord ou WhatsApp : salons de discussion, envoi de messages, de fichiers et d'images. La différence fondamentale est **la souveraineté des données** :

| Principe | Description |
|----------|-------------|
| **Archives chez les participants** | Les historiques de discussion sont maintenus **uniquement** dans la base de données locale de chaque COG ayant participé à la conversation. |
| **Transit crypté** | L'ensemble des données qui transitent entre COGs (messages, métadonnées, fichiers, images) est crypté. |
| **Hébergement utilisateur** | Les discussions, fichiers et images restent hébergés chez les utilisateurs (leurs COGs) ; aucun stockage central obligatoire. |
| **Pas de conservation à l'insu** | Aucun tiers (ni serveur Miyukini, ni relais MWS) ne conserve le contenu des conversations au-delà du strict routage. |

Jay1Tribu respecte les Lois d'Autonomie Miyukini, en particulier :

- **LOI-2** — Le système accepte l'isolement comme état normal (messagerie possible uniquement lorsque les COGs concernés sont connectés ou lors de la reconnexion pour les tribus).
- **LOI-3** — L'état local est souverain : chaque COG est maître de ses archives.
- **LOI-6** — L'autonomie n'empêche pas la fédération : les COGs coopèrent via le MWS pour l'échange de messages.

---

## 2. Concepts fondamentaux

### 2.1 Salon de discussion

Un **salon** (ou **discussion**) est un espace de conversation auquel participent un ou plusieurs COGs (ou utilisateurs identifiés via leur COG).

- Un salon peut être **créé** par un COG ou **rejoint** sur invitation ou découverte (selon les règles du salon et de la tribu éventuelle).
- Les messages échangés dans un salon sont **cryptés en transit** et **archivés localement** par chaque participant dans son COG.
- Les salons peuvent être **directs** (deux participants) ou **collectifs** (groupe).

### 2.2 Envoi de fichiers et d'images

- **Fichiers :** tout type de fichier peut être envoyé **uniquement entre amis**. Le transfert de fichier n'est autorisé que si l'émetteur et chaque destinataire sont amis (relation bidirectionnelle dans la liste d'amis). Le fichier est transmis de pair à pair (ou vers chaque pair concerné), crypté, et stocké localement par l'émetteur et les destinataires qui l'acceptent.
- **Images :** les utilisateurs peuvent ajouter des images dans une discussion **entre amis**. Les images sont traitées comme des contenus cryptés et hébergés chez les participants ; pas de stockage central.

L'ensemble des données (messages, fichiers, images) qui **transitent** est crypté. Les données **au repos** dans chaque COG relèvent de la gouvernance locale (KindMother, WorrySentinel).

### 2.3 Tribu

Une **tribu** est un groupe dont les membres partagent un ensemble de discussions, ainsi que les fichiers et images associés.

- **Création :** un COG (ou un utilisateur) peut créer une tribu et en devenir l'administrateur (**Chef de tribu**).
- **Rôles :** des rôles peuvent être attribués aux membres d'une tribu par le Chef de tribu (ou par des administrateurs habilités). Les rôles définissent les permissions au sein de la tribu (création de salons, invitation, modération, etc.).
- **Partage à la reconnexion :** par défaut, les membres d'une tribu partagent les discussions, fichiers et images au sein de la tribu. Les membres qui n'ont pas encore pu voir certains messages, ou recevoir certains fichiers ou images, **les reçoivent dès leur reconnexion** — **si l'utilisateur qui possède ces contenus (l'émetteur) est lui-même connecté**. Sinon, la remise est différée jusqu'à une connexion simultanée ou à un mécanisme de relais autorisé (hors scope de ce document conceptuel).
- **Paramétrage individuel :** un utilisateur peut, par paramétrage individuel, restreindre ce qui est partagé à la reconnexion (par exemple ne pas synchroniser les fichiers lourds, ou limiter la rétention). Les règles exactes seront précisées dans une spécification fonctionnelle.

En résumé : les tribus permettent de **partager le contexte des conversations** et des médias entre membres, avec une **synchronisation à la reconnexion** (sous réserve de la disponibilité de l'émetteur et du paramétrage).

### 2.4 Amis

- Un **COG** (ou un **utilisateur** identifié via son COG) peut disposer d'une **liste d'amis**.
- La liste d'amis permet :
  - de **voir rapidement** si un ami est connecté (présence via le MWS) ;
  - d'**initier une discussion directe** plus rapidement, sans passer par la découverte d'un salon ou d'une tribu.

Les amis ne sont pas nécessairement dans la même tribu ; la relation « ami » est indépendante et facilite le contact et la visibilité de la présence.

### 2.5 Présence et connectivité

- La **présence** (en ligne / hors ligne) des COGs et utilisateurs s'appuie sur le MWS (présence, découverte, transport). Jay1Tribu **consomme** cette capacité ; il ne définit pas le protocole de présence.
- Les messages ne peuvent être livrés que si le ou les destinataires sont joignables (connectés au maillage ou via un mécanisme de relais autorisé). Les tribus permettent de **différer la livraison** jusqu'à la reconnexion des membres concernés, lorsque l'émetteur est également connecté.

---

## 3. Rôle architectural

### 3.1 Type de Service

Jay1Tribu est un **Service Inter-COG** (Type 3) :

- **Espace Central (Miyukini Central) :** gestion des tribus, des salons, de la liste d'amis, des paramètres et de l'interface utilisateur du COG.
- **Protocoles Inter-COG :** échange de messages, fichiers et images entre COGs, sur le MWS, avec cryptage de bout en bout (ou équivalent selon la spécification technique future).

### 3.2 Positionnement dans la Pyramide Miyukini

| Strate | Élément | Rôle vis-à-vis de Jay1Tribu |
|--------|---------|-----------------------------|
| **7** | Opérateurs Jay1Tribu | Exécutent la messagerie, la gestion des tribus, des salons et des amis pour le compte de l'utilisateur |
| **6** | Outils & Kits d'Outils | Capacités exécutables (chiffrement, transfert de fichiers, etc.) utilisées par les Opérateurs |
| **5** | BondingBrother | Médiation entre les Opérateurs et les Cores |
| **4** | Cores | Gouvernent le comportement (StrongFather, KindMother, Border Guard, WorrySentinel, Master Butler, etc.) |

### 3.3 Dépendance au MWS

Jay1Tribu **s'appuie sur le MWS** pour :

- la **présence** des COGs et la découverte des pairs ;
- le **transport** des messages et des métadonnées (canaux, tribus, rôles) ;
- le **transport** des fichiers et images (ou références sécurisées selon l'architecture retenue).

Le MWS ne stocke pas le contenu des messages ; il assure le routage entre COGs. Le cryptage est de la responsabilité de Jay1Tribu (ou des Outils qu'il utilise), en conformité avec WorrySentinel et Border Guard.

---

## 4. Persistance et souveraineté des données

### 4.1 Autorité de persistance locale

**KindMother** est l'autorité exclusive de persistance **au sein de chaque COG**. Toute écriture locale (archives de discussion, fichiers et images conservés, liste d'amis, paramètres de tribu) passe par des **Intentions d'Écriture (WriteIntent)** soumises à KindMother.

### 4.2 Règle d'archivage

- **Les archives des discussions sont maintenues uniquement par les participants**, dans leur base de données locale (leur COG).
- Aucun serveur central Miyukini ne conserve le contenu des conversations.
- Les relais MWS (Relay, Tracker) ne conservent pas le contenu des messages ; ils peuvent être nécessaires pour le routage, sous contraintes de confidentialité et de conformité (à préciser dans la spécification sécurité).

### 4.3 Données hébergées chez l'utilisateur

Les discussions, fichiers et images sont **hébergés chez les utilisateurs** (leurs COGs). Chaque COG :

- conserve les messages des salons auxquels il a participé ;
- conserve les fichiers et images qu'il a envoyés ou reçus (selon la politique de rétention locale) ;
- ne partage vers d'autres COGs que ce qui est explicitement envoyé ou synchronisé (tribu, à la reconnexion), dans le respect des paramétrages et des rôles.

---

## 5. Sécurité et gouvernance

### 5.1 Cryptage

- **En transit :** l'ensemble des données qui transitent entre COGs (messages, fichiers, images, métadonnées sensibles) est crypté. Les mécanismes (chiffrement de bout en bout, clés, gestion des identités) seront définis dans une spécification technique et sécurité.
- **Au repos :** la classification et le chiffrement au repos dans chaque COG relèvent de WorrySentinel et KindMother (niveaux de sécurité, politique de résidence des données).

### 5.2 Gouvernance par les Cores

| Core | Rôle vis-à-vis de Jay1Tribu |
|------|-----------------------------|
| **StrongFather** | Décide si une action est autorisée (création de tribu, envoi de message, invitation, attribution de rôles). Émet les Mandats de Permission pour les échanges Inter-COG. |
| **KindMother** | Autorité de persistance locale. Valide, refuse ou reporte les WriteIntent pour les archives et les fichiers stockés localement. |
| **Master Butler** | Registre des capacités et permissions (qui peut créer un salon, inviter, être Chef de tribu, envoyer des fichiers, etc.). |
| **WorrySentinel** | Niveaux de sécurité des contenus, règles de rétention, politique de chiffrement. |
| **Border Guard** | Frontières Inter-COG : qui peut communiquer avec qui, règles de confiance entre COGs. |
| **Caring Nanny** | Observation de l'état du système ; peut restreindre les échanges en cas d'environnement dégradé. |
| **Ever Buddy** | Évolution du Service (versions, compatibilité, dépréciation). |
| **TAMR** | Points d'intervention humaine (modération, litiges, révocation d'accès). |

### 5.3 Rôles au sein d'une tribu

- **Chef de tribu :** administrateur de la tribu ; peut attribuer des rôles, gérer les membres, les salons et les paramètres de la tribu.
- **Rôles personnalisés :** définis par le Chef de tribu (ou par délégation) pour attribuer des permissions (création de salons, invitation, modération, etc.). La liste et la sémantique des rôles seront précisées dans une spécification fonctionnelle.

---

## 6. Capacités exposées (vision conceptuelle)

| Capacité | Description |
|----------|-------------|
| **Créer ou rejoindre un salon** | Création d'une discussion ou adhésion à un salon existant (direct ou collectif). |
| **Envoyer des messages** | Envoi de messages texte (et métadonnées associées) dans un salon ; transit crypté ; archivage local chez chaque participant. |
| **Envoyer des fichiers** | Envoi de fichiers vers un ou plusieurs membres d'une discussion ; transit crypté ; stockage local chez l'émetteur et les destinataires. |
| **Ajouter des images** | Ajout d'images dans une discussion ; transit crypté ; hébergement chez les participants. |
| **Créer une tribu** | Création d'une tribu avec un Chef de tribu ; définition des rôles et des règles de partage. |
| **Rejoindre une tribu** | Adhésion à une tribu (sur invitation ou découverte selon les règles). |
| **Partage à la reconnexion** | Pour les membres d'une tribu : réception des messages, fichiers et images non encore vus à la reconnexion, si l'émetteur est connecté (sauf paramétrage contraire). |
| **Attribution de rôles** | Le Chef de tribu (ou les administrateurs) attribuent des rôles aux membres de la tribu. |
| **Liste d'amis** | Gestion d'une liste d'amis ; consultation de la présence (connecté / hors ligne) ; initiation rapide d'une discussion directe. |

Ces capacités sont déclarées et gouvernées par les Cores (Master Butler, StrongFather) et soumises aux politiques de sécurité (WorrySentinel, Border Guard).

---

## 7. Contraintes et invariants

Les contraintes suivantes sont **non négociables** pour toute conception et toute évolution de Jay1Tribu.

| # | Contrainte | Description |
|---|------------|-------------|
| **C-1** | Pas d'archives centrales de contenu | Les archives des discussions ne sont maintenues que chez les participants (leurs COGs). |
| **C-2** | Transit crypté | Tout message, fichier et image en transit entre COGs est crypté. |
| **C-3** | Hébergement utilisateur | Les discussions, fichiers et images restent hébergés chez les utilisateurs (leurs COGs). |
| **C-4** | Persistance locale via KindMother | Toute écriture locale passe par KindMother (WriteIntent). |
| **C-5** | Service Inter-COG | Jay1Tribu déclare son type (Type 3) et prévoit les espaces Central et Inter-COG. |
| **C-6** | Tribus et reconnexion | La livraison différée (tribu) est conditionnée par la reconnexion et la disponibilité de l'émetteur ; paramétrage individuel possible. |
| **C-7** | Rôles gouvernés | Les rôles au sein d'une tribu sont attribués par le Chef de tribu (ou délégation) et gouvernés par Master Butler / StrongFather. |
| **C-8** | Liste d'amis et présence | La liste d'amis et la présence s'appuient sur le MWS ; pas de duplication de la logique de présence. |
| **C-9** | Transferts réservés aux amis | Les transferts de fichier ne peuvent se faire qu'entre amis ; l'émetteur et chaque destinataire doivent être amis. |

---

## 8. Résumé exécutif

**Jay1Tribu** est le **service de messagerie pair-à-pair** de l'écosystème Miyukini : salons, tribus, amis, envoi de messages, fichiers et images, avec **archives uniquement chez les participants** et **transit crypté**.

- **Tribus** : partage des discussions et des médias entre membres ; synchronisation à la reconnexion (si l'émetteur est connecté) ; rôles gérés par le Chef de tribu.
- **Amis** : liste d'amis, présence, discussion directe rapide.
- **Souveraineté** : pas de conservation des données à l'insu des utilisateurs ; tout reste hébergé chez eux, dans leur COG.

Jay1Tribu s'appuie sur le **MWS** pour la présence et le transport, et sur les **Cores** pour la gouvernance, la persistance (KindMother) et la sécurité (WorrySentinel, Border Guard).

---

## 9. Références

| Document | Rôle |
|----------|------|
| [Miyukini Conceptual References — Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) | Terminologie officielle (Service, Opérateur, COG, Cores, MWS, WriteIntent, etc.) |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Présence, découverte, transport des COGs |
| [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) | Service Inter-COG (Type 3) |
| Architecture Miyukini (skill miyukini-architecture) | Pyramide, Cores, Lois d'Autonomie |

---

**Document** : Jay1Tribu — Document Conceptuel  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Document conceptuel normatif — référence pour la conception du Service
