# Jay1Tribu — Document Fondateur

## Contexte

**Jay1Tribu** est le **Service de messagerie pair-à-pair (P2P)** de l'écosystème Miyukini COG. Il fédère les utilisateurs en communautés souveraines : tribus, salons de discussion, liste d'amis, envoi de messages, fichiers et images — avec **archives uniquement chez les participants** et **transit crypté**.

Ce document pose la vision et les principes fondateurs. Pour les concepts détaillés (tribus, rôles, persistance, Cores), voir le [Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md).

## Portée / Scope

- **Applicable à :** Vision produit, principes non négociables, positionnement du service.
- **Audience :** Parties prenantes, équipes produit, architecture.
- **Statut :** Document fondateur normatif.

---

## 1. Vision

> Remplacer les systèmes de messagerie qui conservent les données à l'insu de leurs utilisateurs. Avec Jay1Tribu, les discussions, fichiers et images sont hébergés chez les utilisateurs ; chaque COG ne garde que ce dont il a été partie prenante.

| Principe | Description |
|----------|-------------|
| **Archives chez les participants** | Les historiques sont maintenus **uniquement** dans la base locale de chaque COG ayant participé à la conversation. |
| **Transit crypté** | Toutes les données échangées entre COGs (messages, métadonnées, fichiers, images) sont cryptées. |
| **Hébergement utilisateur** | Aucun serveur central ne conserve le contenu des conversations. Les relais MWS routent sans stocker. |
| **Pas de conservation à l'insu** | Aucun tiers ne conserve le contenu au-delà du strict routage. |

---

## 2. Type de Service

**Service Inter-COG (Type 3)** :

- **Espace Miyukini Central :** gestion des tribus, salons, liste d'amis, paramètres, interface.
- **Protocoles Inter-COG :** échange de messages, fichiers et images entre COGs via le MWS, avec cryptage.

---

## 3. Capacités clés

| Capacité | Description |
|----------|-------------|
| **Salons** | Discussions directes (2 participants) ou collectives ; messages, fichiers, images ; archivage local chez chaque participant. |
| **Tribus** | Groupes partageant discussions et médias ; Chef de tribu ; rôles ; synchronisation à la reconnexion (si l'émetteur est connecté). |
| **Amis** | Liste d'amis ; présence (via MWS) ; initiation rapide d'une discussion directe. |
| **Partage à la reconnexion** | Pour les membres d'une tribu : réception des contenus non encore vus à la reconnexion, sous réserve de la disponibilité de l'émetteur. |

---

## 4. Dépendances

- **MWS (Miyukini Webway System) :** présence, découverte, transport. Jay1Tribu consomme ces capacités ; il ne définit pas le protocole de présence.
- **Cores :** KindMother (persistance locale), StrongFather (autorisation), Master Butler (permissions), WorrySentinel (sécurité), Border Guard (frontières Inter-COG).

---

## 5. Lois d'Autonomie

Jay1Tribu respecte en particulier :

- **LOI-2** — Le système accepte l'isolement ; messagerie possible lorsque les COGs sont connectés ou à la reconnexion pour les tribus.
- **LOI-3** — L'état local est souverain : chaque COG est maître de ses archives.
- **LOI-6** — L'autonomie n'empêche pas la fédération : les COGs coopèrent via le MWS pour l'échange.

---

## 6. Résumé

**Jay1Tribu** = messagerie P2P Miyukini : salons, tribus, amis, messages/fichiers/images, **archives chez les participants**, **transit crypté**, gouvernance par les Cores et transport par le MWS.

---

## 7. Références

| Document | Rôle |
|----------|------|
| [Jay1Tribu - Document Conceptuel](./Jay1Tribu%20-%20Document%20Conceptuel.md) | Concepts complets (tribus, salons, amis, rôles, persistance, contraintes). |
| [MWS - Document Fondateur](../../miyukini-webway-system/MWS%20-%20Document%20Fondateur.md) | Présence, découverte, transport. |
| [Types de Services et Espaces](../../reference/Miyukini%20Conceptual%20References%20-%20Types%20de%20Services%20et%20Espaces.md) | Service Inter-COG (Type 3). |

---

**Document** : Jay1Tribu — Document Fondateur  
**Version** : 1.0  
**Date** : 2026-02-15  
**Statut** : Document fondateur normatif
