# MiyuSocialFeed â€” Documentation Fondatrice

## 1. Contexte

**MiyuSocialFeed** est le **kit d'outils (Toolkit)** de fil d'actualitÃ© social (publication, flux, rÃ©actions, partages, commentaires) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de publication, de liste du flux, de rÃ©actions, de partages et de commentaires, alignÃ©s sur [Ã‰quivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (posts, flux, rÃ©actions, partages, commentaires) appartient Ã  **KindMother**. MiyuSocialFeed expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (publication autorisÃ©e, visibilitÃ©, modÃ©ration) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuSocialFeed, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** les stories (MiyuStory) ; la messagerie (MiyuSocialMessaging) ; l'implÃ©mentation dÃ©taillÃ©e (algorithme de flux).

---

## 3. DÃ©finition canonique

> **MiyuSocialFeed est une composition officielle d'outils de fil d'actualitÃ© social (publication, flux, rÃ©actions, partages, commentaires), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuSocialFeed **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuSocialFeed **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision (publication, visibilitÃ©) = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (post, rÃ©action, partage, commentaire) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.feed` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuSocialFeed - Reference Outils](./MiyuSocialFeed%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.social.post.create` | CrÃ©e une publication ; autorisation = StrongFather |
| `tool.social.post.update` | Met Ã  jour une publication |
| `tool.social.post.delete` | Supprime une publication ; dÃ©cision = StrongFather |
| `tool.social.post.get` | RÃ©cupÃ¨re une publication |
| `tool.social.feed.list` | Liste le flux (filtres, pagination fournis) |
| `tool.social.reaction.add` | Ajoute une rÃ©action (like, etc.) |
| `tool.social.reaction.remove` | Supprime une rÃ©action |
| `tool.social.reaction.list` | Liste les rÃ©actions d'un post |
| `tool.social.share.create` | CrÃ©e un partage ; autorisation = StrongFather |
| `tool.social.share.list` | Liste les partages |
| `tool.social.comment.create` | CrÃ©e un commentaire |
| `tool.social.comment.list` | Liste les commentaires d'un post |
| `tool.social.comment.delete` | Supprime un commentaire ; dÃ©cision = StrongFather |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSocialFeed en contient treize.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision (publication, visibilitÃ©, suppression) = StrongFather** ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  3** (contenu social, donnÃ©es personnelles) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : posts, flux, rÃ©actions, partages, commentaires. Toute crÃ©ation, mise Ã  jour ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuSocialFeed - Tool Governance Compliance Contract](./contracts/governance/MiyuSocialFeed%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuSocialFeed sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuSocialFeed devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Reseaux Sociaux | [Miyukini Conceptual References - Equivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


