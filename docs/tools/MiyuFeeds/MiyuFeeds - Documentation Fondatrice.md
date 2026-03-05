# MiyuFeeds â€” Documentation Fondatrice

## 1. Contexte

**MiyuFeeds** est le **kit d'outils (Toolkit)** de flux ATOM (board, forum, topic) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de gÃ©nÃ©ration de flux ATOM pour board, forum et topic, alignÃ©s sur [Ã‰quivalents Moteur Forum](..//..//miyukini-webway-system//reference//_index.md).

Les donnÃ©es sous-jacentes (contenus board, forum, topic) appartiennent Ã  **KindMother**. MiyuFeeds expose des capacitÃ©s de **lecture** et de **gÃ©nÃ©ration de flux** ; les dÃ©cisions (qui peut accÃ©der Ã  quel flux) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuFeeds, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** les abonnements (MiyuNotify) ; l'implÃ©mentation dÃ©taillÃ©e (format ATOM, cache).

---

## 3. DÃ©finition canonique

> **MiyuFeeds est une composition officielle d'outils de flux ATOM (board, forum, topic), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuFeeds **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuFeeds **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques de gÃ©nÃ©ration de flux ; contenu fourni dans le flux ; dÃ©cision d'accÃ¨s = StrongFather.

**RÃ¨gle fondamentale :** Les Tools **lisent** les donnÃ©es KindMother et **produisent** un flux ATOM ; pas d'Ã©criture mÃ©tier.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.feeds` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuFeeds - Reference Outils](./MiyuFeeds%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.feed.atom.board` | GÃ©nÃ¨re un flux ATOM pour l'ensemble du board |
| `tool.feed.atom.forum` | GÃ©nÃ¨re un flux ATOM pour un forum donnÃ© |
| `tool.feed.atom.topic` | GÃ©nÃ¨re un flux ATOM pour un topic donnÃ© |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuFeeds en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision d'accÃ¨s au flux = StrongFather** ; lecture des donnÃ©es = KindMother ; pas d'Ã©criture mÃ©tier.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (flux public Ã  restreint selon politique) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es sources (board, forum, topic). Les Tools MiyuFeeds **lisent** ces donnÃ©es pour produire le flux ATOM ; ils n'Ã©crivent pas.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuFeeds - Tool Governance Compliance Contract](./contracts/governance/MiyuFeeds%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuFeeds sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuFeeds devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


