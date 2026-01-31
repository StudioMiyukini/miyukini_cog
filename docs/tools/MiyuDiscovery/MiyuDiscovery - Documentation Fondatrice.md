# MiyuDiscovery — Documentation Fondatrice

## 1. Contexte

**MiyuDiscovery** est le **kit d'outils (Toolkit)** de découverte sociale (hashtags, tendances, explore, recherche) de l'écosystème Miyukini. Il intègre les outils de hashtags, de tendances, de liste explore et de recherche sociale, alignés sur [Équivalents Reseaux Sociaux](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Reseaux%20Sociaux.md).

Les données sous-jacentes (posts, hashtags, tendances) appartiennent à **KindMother**. MiyuDiscovery expose des capacités de **lecture** et de **recherche** ; les décisions (qui peut voir quelles tendances, politique explore) relèvent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuDiscovery, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'indexation full-text (MiyuSearch) ; l'implémentation détaillée (algorithme tendances, explore).

---

## 3. Définition canonique

> **MiyuDiscovery est une composition officielle d'outils de découverte sociale (hashtags, tendances, explore, recherche), déclarée et gouvernée par l'environnement.**

- MiyuDiscovery **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuDiscovery **n'ajoute aucune logique métier** : il orchestre des capacités atomiques ; décision (politique explore, tendances) = StrongFather.

**Règle fondamentale :** Les Tools **lisent** les données KindMother (posts, hashtags, tendances) ; pas d'écriture métier (sauf si compteurs hashtags/trending sont persistés ; alors WriteIntent KindMother).

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.discovery` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuDiscovery - Reference Outils](./MiyuDiscovery%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.social.hashtag.list` | Liste les hashtags (filtres fournis) |
| `tool.social.hashtag.get` | Récupère un hashtag et ses posts |
| `tool.social.hashtag.trending` | Liste les hashtags tendance |
| `tool.social.trending.list` | Liste les tendances (posts, sujets) ; politique = StrongFather |
| `tool.social.discover.list` | Liste le contenu explore (filtres fournis) |
| `tool.social.search` | Recherche sociale (scope=social) ; ou réutilisation MiyuSearch scope=social |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuDiscovery en contient six.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : **décision (politique explore, tendances) = StrongFather** ; lecture = KindMother.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (contenu public à restreint selon politique) |
| **États autorisés** | `HEALTHY`, `DEGRADED` |
| **États interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données : posts, hashtags, tendances. Les Tools MiyuDiscovery **lisent** ces données pour produire listes et recherche ; écriture éventuelle (compteurs hashtags) = WriteIntent KindMother si définie.

Les obligations de conformité détaillées sont dans [MiyuDiscovery - Tool Governance Compliance Contract](./contracts/governance/MiyuDiscovery%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implémentation de MiyuDiscovery sont conçues pour être **compatibles MIP v1** (Miyukini Index Protocol). À l'implémentation, le code fournissant les Tools MiyuDiscovery devra être balisé MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit généré selon le [Protocole MIP v1](../../protocols/Miyukini%20Prompt%20Protocol%20-%20MIP%20v1%20MSCM%20Index%20Protocol.md).

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Équivalents Reseaux Sociaux | [Miyukini Conceptual References - Equivalents Reseaux Sociaux](../../reference/Miyukini%20Conceptual%20References%20-%20Equivalents%20Reseaux%20Sociaux.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
