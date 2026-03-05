# Jay1Tribu â€” Document Conceptuel

## Contexte

**Jay1Tribu** est un **Service de messagerie pair-Ã -pair (P2P)** au sein de l'Ã©cosystÃ¨me Miyukini COG (Core-Orchestrated Governance Environment).

Son objectif est de fournir une messagerie instantanÃ©e entre COGs â€” discussions, envoi de fichiers et d'images â€” dans laquelle **les archives restent uniquement chez les participants**, dans leur base de donnÃ©es locale (leur COG). Aucun serveur central ne conserve les conversations Ã  l'insu des utilisateurs.

Jay1Tribu est conÃ§u comme un **Service Inter-COG** (Type 3) : les Ã©changes ont lieu entre COGs connectÃ©s au MWS (Miyukini Webway System). Le service s'appuie sur la prÃ©sence, la dÃ©couverte et le transport fournis par le Webway pour acheminer les messages, tout en garantissant que **tout ce qui transite est cryptÃ©** et que **seuls les participants conservent les donnÃ©es**.

> **Philosophie fondatrice :** Remplacer les systÃ¨mes de messagerie qui conservent les donnÃ©es Ã  l'insu de leurs utilisateurs. Avec Jay1Tribu, les discussions, fichiers et images sont hÃ©bergÃ©s chez les utilisateurs ; chaque COG ne garde que ce dont il a Ã©tÃ© partie prenante.

Ce document est le **document conceptuel** du Service Jay1Tribu : il en fixe la raison d'Ãªtre, le positionnement architectural, le modÃ¨le conceptuel (tribus, salons, amis, rÃ´les), les rÃ¨gles de confidentialitÃ© et de souverainetÃ© des donnÃ©es, et les interactions avec le MWS et les Cores. Il s'adresse aux Ã©quipes produit, architecture, sÃ©curitÃ© et Ã  toute partie prenante de l'Ã©cosystÃ¨me Miyukini.

## PortÃ©e / Scope

- **Applicable Ã  :** Vision produit, concepts mÃ©tier, architecture conceptuelle, modÃ¨le de donnÃ©es conceptuel, gouvernance, sÃ©curitÃ©
- **Audience :** Architectes, Ã©quipes produit, Ã©quipes sÃ©curitÃ©, parties prenantes
- **Statut :** Document conceptuel normatif â€” rÃ©fÃ©rence pour la conception du Service

### Hors pÃ©rimÃ¨tre

- Aucun code ni implÃ©mentation technique
- Aucun choix UI/UX dÃ©taillÃ©
- Aucun protocole de cryptographie spÃ©cifique (le principe Â« tout cryptÃ© Â» est fixÃ© ; les mÃ©canismes seront spÃ©cifiÃ©s ailleurs)
- Aucun schÃ©ma de base de donnÃ©es ni API

---

## 1. Contexte gÃ©nÃ©ral

Jay1Tribu offre une expÃ©rience fonctionnelle comparable Ã  des messageries de type Discord ou WhatsApp : salons de discussion, envoi de messages, de fichiers et d'images. La diffÃ©rence fondamentale est **la souverainetÃ© des donnÃ©es** :

| Principe | Description |
|----------|-------------|
| **Archives chez les participants** | Les historiques de discussion sont maintenus **uniquement** dans la base de donnÃ©es locale de chaque COG ayant participÃ© Ã  la conversation. |
| **Transit cryptÃ©** | L'ensemble des donnÃ©es qui transitent entre COGs (messages, mÃ©tadonnÃ©es, fichiers, images) est cryptÃ©. |
| **HÃ©bergement utilisateur** | Les discussions, fichiers et images restent hÃ©bergÃ©s chez les utilisateurs (leurs COGs) ; aucun stockage central obligatoire. |
| **Pas de conservation Ã  l'insu** | Aucun tiers (ni serveur Miyukini, ni relais MWS) ne conserve le contenu des conversations au-delÃ  du strict routage. |

Jay1Tribu respecte les Lois d'Autonomie Miyukini, en particulier :

- **LOI-2** â€” Le systÃ¨me accepte l'isolement comme Ã©tat normal (messagerie possible uniquement lorsque les COGs concernÃ©s sont connectÃ©s ou lors de la reconnexion pour les tribus).
- **LOI-3** â€” L'Ã©tat local est souverain : chaque COG est maÃ®tre de ses archives.
- **LOI-6** â€” L'autonomie n'empÃªche pas la fÃ©dÃ©ration : les COGs coopÃ¨rent via le MWS pour l'Ã©change de messages.

---

## 2. Concepts fondamentaux

### 2.1 Salon de discussion

Un **salon** (ou **discussion**) est un espace de conversation auquel participent un ou plusieurs COGs (ou utilisateurs identifiÃ©s via leur COG).

- Un salon peut Ãªtre **crÃ©Ã©** par un COG ou **rejoint** sur invitation ou dÃ©couverte (selon les rÃ¨gles du salon et de la tribu Ã©ventuelle).
- Les messages Ã©changÃ©s dans un salon sont **cryptÃ©s en transit** et **archivÃ©s localement** par chaque participant dans son COG.
- Les salons peuvent Ãªtre **directs** (deux participants) ou **collectifs** (groupe).

### 2.2 Envoi de fichiers et d'images

- **Fichiers :** tout type de fichier peut Ãªtre envoyÃ© **uniquement entre amis**. Le transfert de fichier n'est autorisÃ© que si l'Ã©metteur et chaque destinataire sont amis (relation bidirectionnelle dans la liste d'amis). Le fichier est transmis de pair Ã  pair (ou vers chaque pair concernÃ©), cryptÃ©, et stockÃ© localement par l'Ã©metteur et les destinataires qui l'acceptent.
- **Images :** les utilisateurs peuvent ajouter des images dans une discussion **entre amis**. Les images sont traitÃ©es comme des contenus cryptÃ©s et hÃ©bergÃ©s chez les participants ; pas de stockage central.

L'ensemble des donnÃ©es (messages, fichiers, images) qui **transitent** est cryptÃ©. Les donnÃ©es **au repos** dans chaque COG relÃ¨vent de la gouvernance locale (KindMother, WorrySentinel).

### 2.3 Tribu

Une **tribu** est un groupe dont les membres partagent un ensemble de discussions, ainsi que les fichiers et images associÃ©s.

- **CrÃ©ation :** un COG (ou un utilisateur) peut crÃ©er une tribu et en devenir l'administrateur (**Chef de tribu**).
- **RÃ´les :** des rÃ´les peuvent Ãªtre attribuÃ©s aux membres d'une tribu par le Chef de tribu (ou par des administrateurs habilitÃ©s). Les rÃ´les dÃ©finissent les permissions au sein de la tribu (crÃ©ation de salons, invitation, modÃ©ration, etc.).
- **Partage Ã  la reconnexion :** par dÃ©faut, les membres d'une tribu partagent les discussions, fichiers et images au sein de la tribu. Les membres qui n'ont pas encore pu voir certains messages, ou recevoir certains fichiers ou images, **les reÃ§oivent dÃ¨s leur reconnexion** â€” **si l'utilisateur qui possÃ¨de ces contenus (l'Ã©metteur) est lui-mÃªme connectÃ©**. Sinon, la remise est diffÃ©rÃ©e jusqu'Ã  une connexion simultanÃ©e ou Ã  un mÃ©canisme de relais autorisÃ© (hors scope de ce document conceptuel).
- **ParamÃ©trage individuel :** un utilisateur peut, par paramÃ©trage individuel, restreindre ce qui est partagÃ© Ã  la reconnexion (par exemple ne pas synchroniser les fichiers lourds, ou limiter la rÃ©tention). Les rÃ¨gles exactes seront prÃ©cisÃ©es dans une spÃ©cification fonctionnelle.

En rÃ©sumÃ© : les tribus permettent de **partager le contexte des conversations** et des mÃ©dias entre membres, avec une **synchronisation Ã  la reconnexion** (sous rÃ©serve de la disponibilitÃ© de l'Ã©metteur et du paramÃ©trage).

### 2.4 Amis

- Un **COG** (ou un **utilisateur** identifiÃ© via son COG) peut disposer d'une **liste d'amis**.
- La liste d'amis permet :
  - de **voir rapidement** si un ami est connectÃ© (prÃ©sence via le MWS) ;
  - d'**initier une discussion directe** plus rapidement, sans passer par la dÃ©couverte d'un salon ou d'une tribu.

Les amis ne sont pas nÃ©cessairement dans la mÃªme tribu ; la relation Â« ami Â» est indÃ©pendante et facilite le contact et la visibilitÃ© de la prÃ©sence.

### 2.5 PrÃ©sence et connectivitÃ©

- La **prÃ©sence** (en ligne / hors ligne) des COGs et utilisateurs s'appuie sur le MWS (prÃ©sence, dÃ©couverte, transport). Jay1Tribu **consomme** cette capacitÃ© ; il ne dÃ©finit pas le protocole de prÃ©sence.
- Les messages ne peuvent Ãªtre livrÃ©s que si le ou les destinataires sont joignables (connectÃ©s au maillage ou via un mÃ©canisme de relais autorisÃ©). Les tribus permettent de **diffÃ©rer la livraison** jusqu'Ã  la reconnexion des membres concernÃ©s, lorsque l'Ã©metteur est Ã©galement connectÃ©.

---

## 3. RÃ´le architectural

### 3.1 Type de Service

Jay1Tribu est un **Service Inter-COG** (Type 3) :

- **Espace Central (Miyukini Central) :** gestion des tribus, des salons, de la liste d'amis, des paramÃ¨tres et de l'interface utilisateur du COG.
- **Protocoles Inter-COG :** Ã©change de messages, fichiers et images entre COGs, sur le MWS, avec cryptage de bout en bout (ou Ã©quivalent selon la spÃ©cification technique future).

### 3.2 Positionnement dans la Pyramide Miyukini

| Strate | Ã‰lÃ©ment | RÃ´le vis-Ã -vis de Jay1Tribu |
|--------|---------|-----------------------------|
| **7** | OpÃ©rateurs Jay1Tribu | ExÃ©cutent la messagerie, la gestion des tribus, des salons et des amis pour le compte de l'utilisateur |
| **6** | Outils & Kits d'Outils | CapacitÃ©s exÃ©cutables (chiffrement, transfert de fichiers, etc.) utilisÃ©es par les OpÃ©rateurs |
| **5** | BondingBrother | MÃ©diation entre les OpÃ©rateurs et les Cores |
| **4** | Cores | Gouvernent le comportement (StrongFather, KindMother, Border Guard, WorrySentinel, Master Butler, etc.) |

### 3.3 DÃ©pendance au MWS

Jay1Tribu **s'appuie sur le MWS** pour :

- la **prÃ©sence** des COGs et la dÃ©couverte des pairs ;
- le **transport** des messages et des mÃ©tadonnÃ©es (canaux, tribus, rÃ´les) ;
- le **transport** des fichiers et images (ou rÃ©fÃ©rences sÃ©curisÃ©es selon l'architecture retenue).

Le MWS ne stocke pas le contenu des messages ; il assure le routage entre COGs. Le cryptage est de la responsabilitÃ© de Jay1Tribu (ou des Outils qu'il utilise), en conformitÃ© avec WorrySentinel et Border Guard.

---

## 4. Persistance et souverainetÃ© des donnÃ©es

### 4.1 AutoritÃ© de persistance locale

**KindMother** est l'autoritÃ© exclusive de persistance **au sein de chaque COG**. Toute Ã©criture locale (archives de discussion, fichiers et images conservÃ©s, liste d'amis, paramÃ¨tres de tribu) passe par des **Intentions d'Ã‰criture (WriteIntent)** soumises Ã  KindMother.

### 4.2 RÃ¨gle d'archivage

- **Les archives des discussions sont maintenues uniquement par les participants**, dans leur base de donnÃ©es locale (leur COG).
- Aucun serveur central Miyukini ne conserve le contenu des conversations.
- Les relais MWS (Relay, Tracker) ne conservent pas le contenu des messages ; ils peuvent Ãªtre nÃ©cessaires pour le routage, sous contraintes de confidentialitÃ© et de conformitÃ© (Ã  prÃ©ciser dans la spÃ©cification sÃ©curitÃ©).

### 4.3 DonnÃ©es hÃ©bergÃ©es chez l'utilisateur

Les discussions, fichiers et images sont **hÃ©bergÃ©s chez les utilisateurs** (leurs COGs). Chaque COG :

- conserve les messages des salons auxquels il a participÃ© ;
- conserve les fichiers et images qu'il a envoyÃ©s ou reÃ§us (selon la politique de rÃ©tention locale) ;
- ne partage vers d'autres COGs que ce qui est explicitement envoyÃ© ou synchronisÃ© (tribu, Ã  la reconnexion), dans le respect des paramÃ©trages et des rÃ´les.

---

## 5. SÃ©curitÃ© et gouvernance

### 5.1 Cryptage

- **En transit :** l'ensemble des donnÃ©es qui transitent entre COGs (messages, fichiers, images, mÃ©tadonnÃ©es sensibles) est cryptÃ©. Les mÃ©canismes (chiffrement de bout en bout, clÃ©s, gestion des identitÃ©s) seront dÃ©finis dans une spÃ©cification technique et sÃ©curitÃ©.
- **Au repos :** la classification et le chiffrement au repos dans chaque COG relÃ¨vent de WorrySentinel et KindMother (niveaux de sÃ©curitÃ©, politique de rÃ©sidence des donnÃ©es).

### 5.2 Gouvernance par les Cores

| Core | RÃ´le vis-Ã -vis de Jay1Tribu |
|------|-----------------------------|
| **StrongFather** | DÃ©cide si une action est autorisÃ©e (crÃ©ation de tribu, envoi de message, invitation, attribution de rÃ´les). Ã‰met les Mandats de Permission pour les Ã©changes Inter-COG. |
| **KindMother** | AutoritÃ© de persistance locale. Valide, refuse ou reporte les WriteIntent pour les archives et les fichiers stockÃ©s localement. |
| **Master Butler** | Registre des capacitÃ©s et permissions (qui peut crÃ©er un salon, inviter, Ãªtre Chef de tribu, envoyer des fichiers, etc.). |
| **WorrySentinel** | Niveaux de sÃ©curitÃ© des contenus, rÃ¨gles de rÃ©tention, politique de chiffrement. |
| **Border Guard** | FrontiÃ¨res Inter-COG : qui peut communiquer avec qui, rÃ¨gles de confiance entre COGs. |
| **Caring Nanny** | Observation de l'Ã©tat du systÃ¨me ; peut restreindre les Ã©changes en cas d'environnement dÃ©gradÃ©. |
| **Ever Buddy** | Ã‰volution du Service (versions, compatibilitÃ©, dÃ©prÃ©ciation). |
| **TAMR** | Points d'intervention humaine (modÃ©ration, litiges, rÃ©vocation d'accÃ¨s). |

### 5.3 RÃ´les au sein d'une tribu

- **Chef de tribu :** administrateur de la tribu ; peut attribuer des rÃ´les, gÃ©rer les membres, les salons et les paramÃ¨tres de la tribu.
- **RÃ´les personnalisÃ©s :** dÃ©finis par le Chef de tribu (ou par dÃ©lÃ©gation) pour attribuer des permissions (crÃ©ation de salons, invitation, modÃ©ration, etc.). La liste et la sÃ©mantique des rÃ´les seront prÃ©cisÃ©es dans une spÃ©cification fonctionnelle.

---

## 6. CapacitÃ©s exposÃ©es (vision conceptuelle)

| CapacitÃ© | Description |
|----------|-------------|
| **CrÃ©er ou rejoindre un salon** | CrÃ©ation d'une discussion ou adhÃ©sion Ã  un salon existant (direct ou collectif). |
| **Envoyer des messages** | Envoi de messages texte (et mÃ©tadonnÃ©es associÃ©es) dans un salon ; transit cryptÃ© ; archivage local chez chaque participant. |
| **Envoyer des fichiers** | Envoi de fichiers vers un ou plusieurs membres d'une discussion ; transit cryptÃ© ; stockage local chez l'Ã©metteur et les destinataires. |
| **Ajouter des images** | Ajout d'images dans une discussion ; transit cryptÃ© ; hÃ©bergement chez les participants. |
| **CrÃ©er une tribu** | CrÃ©ation d'une tribu avec un Chef de tribu ; dÃ©finition des rÃ´les et des rÃ¨gles de partage. |
| **Rejoindre une tribu** | AdhÃ©sion Ã  une tribu (sur invitation ou dÃ©couverte selon les rÃ¨gles). |
| **Partage Ã  la reconnexion** | Pour les membres d'une tribu : rÃ©ception des messages, fichiers et images non encore vus Ã  la reconnexion, si l'Ã©metteur est connectÃ© (sauf paramÃ©trage contraire). |
| **Attribution de rÃ´les** | Le Chef de tribu (ou les administrateurs) attribuent des rÃ´les aux membres de la tribu. |
| **Liste d'amis** | Gestion d'une liste d'amis ; consultation de la prÃ©sence (connectÃ© / hors ligne) ; initiation rapide d'une discussion directe. |

Ces capacitÃ©s sont dÃ©clarÃ©es et gouvernÃ©es par les Cores (Master Butler, StrongFather) et soumises aux politiques de sÃ©curitÃ© (WorrySentinel, Border Guard).

---

## 7. Contraintes et invariants

Les contraintes suivantes sont **non nÃ©gociables** pour toute conception et toute Ã©volution de Jay1Tribu.

| # | Contrainte | Description |
|---|------------|-------------|
| **C-1** | Pas d'archives centrales de contenu | Les archives des discussions ne sont maintenues que chez les participants (leurs COGs). |
| **C-2** | Transit cryptÃ© | Tout message, fichier et image en transit entre COGs est cryptÃ©. |
| **C-3** | HÃ©bergement utilisateur | Les discussions, fichiers et images restent hÃ©bergÃ©s chez les utilisateurs (leurs COGs). |
| **C-4** | Persistance locale via KindMother | Toute Ã©criture locale passe par KindMother (WriteIntent). |
| **C-5** | Service Inter-COG | Jay1Tribu dÃ©clare son type (Type 3) et prÃ©voit les espaces Central et Inter-COG. |
| **C-6** | Tribus et reconnexion | La livraison diffÃ©rÃ©e (tribu) est conditionnÃ©e par la reconnexion et la disponibilitÃ© de l'Ã©metteur ; paramÃ©trage individuel possible. |
| **C-7** | RÃ´les gouvernÃ©s | Les rÃ´les au sein d'une tribu sont attribuÃ©s par le Chef de tribu (ou dÃ©lÃ©gation) et gouvernÃ©s par Master Butler / StrongFather. |
| **C-8** | Liste d'amis et prÃ©sence | La liste d'amis et la prÃ©sence s'appuient sur le MWS ; pas de duplication de la logique de prÃ©sence. |
| **C-9** | Transferts rÃ©servÃ©s aux amis | Les transferts de fichier ne peuvent se faire qu'entre amis ; l'Ã©metteur et chaque destinataire doivent Ãªtre amis. |

---

## 8. RÃ©sumÃ© exÃ©cutif

**Jay1Tribu** est le **service de messagerie pair-Ã -pair** de l'Ã©cosystÃ¨me Miyukini : salons, tribus, amis, envoi de messages, fichiers et images, avec **archives uniquement chez les participants** et **transit cryptÃ©**.

- **Tribus** : partage des discussions et des mÃ©dias entre membres ; synchronisation Ã  la reconnexion (si l'Ã©metteur est connectÃ©) ; rÃ´les gÃ©rÃ©s par le Chef de tribu.
- **Amis** : liste d'amis, prÃ©sence, discussion directe rapide.
- **SouverainetÃ©** : pas de conservation des donnÃ©es Ã  l'insu des utilisateurs ; tout reste hÃ©bergÃ© chez eux, dans leur COG.

Jay1Tribu s'appuie sur le **MWS** pour la prÃ©sence et le transport, et sur les **Cores** pour la gouvernance, la persistance (KindMother) et la sÃ©curitÃ© (WorrySentinel, Border Guard).

---

## 9. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Miyukini Conceptual References â€” Glossaire](..//..//miyukini-webway-system//reference//_index.md) | Terminologie officielle (Service, OpÃ©rateur, COG, Cores, MWS, WriteIntent, etc.) |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | PrÃ©sence, dÃ©couverte, transport des COGs |
| [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md) | Service Inter-COG (Type 3) |
| Architecture Miyukini (skill miyukini-architecture) | Pyramide, Cores, Lois d'Autonomie |

---

**Document** : Jay1Tribu â€” Document Conceptuel  
**Version** : 1.0  
**Date** : 2026-02-14  
**Statut** : Document conceptuel normatif â€” rÃ©fÃ©rence pour la conception du Service

