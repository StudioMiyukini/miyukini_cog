# MiyuForum â€” Documentation Fondatrice

## 1. Contexte

**MiyuForum** est le **kit d'outils (Toolkit)** de structure forum (catÃ©gories, forums, topics, posts, sticky, annonces, suivi lu) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de gestion des catÃ©gories, boards, topics, posts et du suivi de lecture, alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

L'autoritÃ© sur les donnÃ©es (catÃ©gories, forums, topics, posts, readtrack) appartient Ã  **KindMother**. MiyuForum expose des capacitÃ©s d'exÃ©cution gouvernÃ©e ; les dÃ©cisions (crÃ©ation, dÃ©placement, modÃ©ration) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuForum, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** la modÃ©ration (voir MiyuModerationForum) ; la messagerie privÃ©e (MiyuPM) ; l'implÃ©mentation dÃ©taillÃ©e (schÃ©mas DB).

---

## 3. DÃ©finition canonique

> **MiyuForum est une composition officielle d'outils de structure forum (catÃ©gories, forums, topics, posts, suivi lu), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuForum **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuForum **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cisions (crÃ©ation, visibilitÃ©, sticky) = StrongFather.

**RÃ¨gle fondamentale :** Toute Ã©criture (category, board, topic, post, readtrack) = **WriteIntent** vers KindMother.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.community.forum` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `community` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuForum - Reference Outils](./MiyuForum%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.forum.category.*` | CatÃ©gories (list, get, create, update â€” WriteIntent KindMother) |
| `tool.forum.board.*` | Forums / boards (list, get, create, update) |
| `tool.forum.topic.*` | Topics (create, list, get, update, sticky, annonce) |
| `tool.forum.post.*` | Posts (create, list, get, update) |
| `tool.forum.readtrack.*` | Suivi de lecture (mark read, list) |
| `tool.forum.topic.export.*` | Export topic (ex. PDF, texte) |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuForum en contient plusieurs (rÃ©partis en familles category, board, topic, post, readtrack, export).

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : dÃ©cision (crÃ©ation topic/post, sticky, annonce) = StrongFather ; toute Ã©criture = WriteIntent KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **1 Ã  2** (contenu communautaire) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : catÃ©gories, forums, topics, posts, readtrack. Toute crÃ©ation, mise Ã  jour ou suppression passe par **WriteIntent** vers KindMother.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuForum - Tool Governance Compliance Contract](./contracts/governance/MiyuForum%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuForum sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuForum devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Ã‰quivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


