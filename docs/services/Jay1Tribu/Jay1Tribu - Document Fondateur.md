# Jay1Tribu â€” Document Fondateur

## Contexte

**Jay1Tribu** est le **Service de messagerie pair-Ã -pair (P2P)** de l'Ã©cosystÃ¨me Miyukini COG. Il fÃ©dÃ¨re les utilisateurs en communautÃ©s souveraines : tribus, salons de discussion, liste d'amis, envoi de messages, fichiers et images â€” avec **archives uniquement chez les participants** et **transit cryptÃ©**.

Ce document pose la vision et les principes fondateurs. Pour les concepts dÃ©taillÃ©s (tribus, rÃ´les, persistance, Cores), voir le [Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md).

## PortÃ©e / Scope

- **Applicable Ã  :** Vision produit, principes non nÃ©gociables, positionnement du service.
- **Audience :** Parties prenantes, Ã©quipes produit, architecture.
- **Statut :** Document fondateur normatif.

---

## 1. Vision

> Remplacer les systÃ¨mes de messagerie qui conservent les donnÃ©es Ã  l'insu de leurs utilisateurs. Avec Jay1Tribu, les discussions, fichiers et images sont hÃ©bergÃ©s chez les utilisateurs ; chaque COG ne garde que ce dont il a Ã©tÃ© partie prenante.

| Principe | Description |
|----------|-------------|
| **Archives chez les participants** | Les historiques sont maintenus **uniquement** dans la base locale de chaque COG ayant participÃ© Ã  la conversation. |
| **Transit cryptÃ©** | Toutes les donnÃ©es Ã©changÃ©es entre COGs (messages, mÃ©tadonnÃ©es, fichiers, images) sont cryptÃ©es. |
| **HÃ©bergement utilisateur** | Aucun serveur central ne conserve le contenu des conversations. Les relais MWS routent sans stocker. |
| **Pas de conservation Ã  l'insu** | Aucun tiers ne conserve le contenu au-delÃ  du strict routage. |

---

## 2. Type de Service

**Service Inter-COG (Type 3)** :

- **Espace Miyukini Central :** gestion des tribus, salons, liste d'amis, paramÃ¨tres, interface.
- **Protocoles Inter-COG :** Ã©change de messages, fichiers et images entre COGs via le MWS, avec cryptage.

---

## 3. CapacitÃ©s clÃ©s

| CapacitÃ© | Description |
|----------|-------------|
| **Salons** | Discussions directes (2 participants) ou collectives ; messages, fichiers, images ; archivage local chez chaque participant. |
| **Tribus** | Groupes partageant discussions et mÃ©dias ; Chef de tribu ; rÃ´les ; synchronisation Ã  la reconnexion (si l'Ã©metteur est connectÃ©). |
| **Amis** | Liste d'amis ; prÃ©sence (via MWS) ; initiation rapide d'une discussion directe. |
| **Partage Ã  la reconnexion** | Pour les membres d'une tribu : rÃ©ception des contenus non encore vus Ã  la reconnexion, sous rÃ©serve de la disponibilitÃ© de l'Ã©metteur. |

---

## 4. DÃ©pendances

- **MWS (Miyukini Webway System) :** prÃ©sence, dÃ©couverte, transport. Jay1Tribu consomme ces capacitÃ©s ; il ne dÃ©finit pas le protocole de prÃ©sence.
- **Cores :** KindMother (persistance locale), StrongFather (autorisation), Master Butler (permissions), WorrySentinel (sÃ©curitÃ©), Border Guard (frontiÃ¨res Inter-COG).

---

## 5. Lois d'Autonomie

Jay1Tribu respecte en particulier :

- **LOI-2** â€” Le systÃ¨me accepte l'isolement ; messagerie possible lorsque les COGs sont connectÃ©s ou Ã  la reconnexion pour les tribus.
- **LOI-3** â€” L'Ã©tat local est souverain : chaque COG est maÃ®tre de ses archives.
- **LOI-6** â€” L'autonomie n'empÃªche pas la fÃ©dÃ©ration : les COGs coopÃ¨rent via le MWS pour l'Ã©change.

---

## 6. RÃ©sumÃ©

**Jay1Tribu** = messagerie P2P Miyukini : salons, tribus, amis, messages/fichiers/images, **archives chez les participants**, **transit cryptÃ©**, gouvernance par les Cores et transport par le MWS.

---

## 7. RÃ©fÃ©rences

| Document | RÃ´le |
|----------|------|
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts complets (tribus, salons, amis, rÃ´les, persistance, contraintes). |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | PrÃ©sence, dÃ©couverte, transport. |
| [Types de Services et Espaces](..//..//miyukini-webway-system//reference//_index.md) | Service Inter-COG (Type 3). |

---

**Document** : Jay1Tribu â€” Document Fondateur  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Document fondateur normatif

