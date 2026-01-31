# MiyuFeeds — Documentation Fondatrice

## 1. Contexte

**MiyuFeeds** est le **kit d'outils (Toolkit)** de flux ATOM (board, forum, topic) de l'écosystème Miyukini. Il intègre les outils de génération de flux ATOM pour board, forum et topic, alignés sur [Équivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md).

Les données sous-jacentes (contenus board, forum, topic) appartiennent à **KindMother**. MiyuFeeds expose des capacités de **lecture** et de **génération de flux** ; les décisions (qui peut accéder à quel flux) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuFeeds, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** les abonnements (MiyuNotify) ; l'implémentation détaillée (format ATOM, cache).

---

## 3. Définition canonique

> **MiyuFeeds est une composition officielle d'outils de flux ATOM (board, forum, topic), déclarée et gouvernée par l'environnement.**

- MiyuFeeds **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuFeeds **n'ajoute aucune logique métier** : il orchestre des capacités atomiques de génération de flux ; contenu fourni dans le flux ; décision d'accès = StrongFather.

**Règle fondamentale :** Les Tools **lisent** les données KindMother et **produisent** un flux ATOM ; pas d'écriture métier.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.content.feeds` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `content` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuFeeds - Reference Outils](./MiyuFeeds%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.feed.atom.board` | Génère un flux ATOM pour l'ensemble du board |
| `tool.feed.atom.forum` | Génère un flux ATOM pour un forum donné |
| `tool.feed.atom.topic` | Génère un flux ATOM pour un topic donné |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuFeeds en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision d'accès au flux = StrongFather** ; lecture des données = KindMother ; pas d'écriture métier.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (flux public à restreint selon politique) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données sources (board, forum, topic). Les Tools MiyuFeeds **lisent** ces données pour produire le flux ATOM ; ils n'écrivent pas.

Les obligations de conformité détaillées sont dans [MiyuFeeds - Tool Governance Compliance Contract](./contracts/governance/MiyuFeeds%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuFeeds sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuFeeds devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Moteur Forum | [Miyukini Conceptual References - Equivalents Moteur Forum](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Moteur%20Forum.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
