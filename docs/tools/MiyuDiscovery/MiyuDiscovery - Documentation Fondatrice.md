# MiyuDiscovery â€” Documentation Fondatrice

## 1. Contexte

**MiyuDiscovery** est le **kit d'outils (Toolkit)** de dÃ©couverte sociale (hashtags, tendances, explore, recherche) de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils de hashtags, de tendances, de liste explore et de recherche sociale, alignÃ©s sur [Ã‰quivalents Reseaux Sociaux](..//..//miyukini-webway-system//reference//_index.md).

Les donnÃ©es sous-jacentes (posts, hashtags, tendances) appartiennent Ã  **KindMother**. MiyuDiscovery expose des capacitÃ©s de **lecture** et de **recherche** ; les dÃ©cisions (qui peut voir quelles tendances, politique explore) relÃ¨vent de **StrongFather**.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuDiscovery, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'indexation full-text (MiyuSearch) ; l'implÃ©mentation dÃ©taillÃ©e (algorithme tendances, explore).

---

## 3. DÃ©finition canonique

> **MiyuDiscovery est une composition officielle d'outils de dÃ©couverte sociale (hashtags, tendances, explore, recherche), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuDiscovery **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuDiscovery **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques ; dÃ©cision (politique explore, tendances) = StrongFather.

**RÃ¨gle fondamentale :** Les Tools **lisent** les donnÃ©es KindMother (posts, hashtags, tendances) ; pas d'Ã©criture mÃ©tier (sauf si compteurs hashtags/trending sont persistÃ©s ; alors WriteIntent KindMother).

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.social.discovery` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `social` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuDiscovery - Reference Outils](./MiyuDiscovery%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.social.hashtag.list` | Liste les hashtags (filtres fournis) |
| `tool.social.hashtag.get` | RÃ©cupÃ¨re un hashtag et ses posts |
| `tool.social.hashtag.trending` | Liste les hashtags tendance |
| `tool.social.trending.list` | Liste les tendances (posts, sujets) ; politique = StrongFather |
| `tool.social.discover.list` | Liste le contenu explore (filtres fournis) |
| `tool.social.search` | Recherche sociale (scope=social) ; ou rÃ©utilisation MiyuSearch scope=social |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuDiscovery en contient six.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : **dÃ©cision (politique explore, tendances) = StrongFather** ; lecture = KindMother.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (contenu public Ã  restreint selon politique) |
| **Ã‰tats autorisÃ©s** | `HEALTHY`, `DEGRADED` |
| **Ã‰tats interdits** | `SECURITY_LOCKDOWN`, `MAINTENANCE` |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es : posts, hashtags, tendances. Les Tools MiyuDiscovery **lisent** ces donnÃ©es pour produire listes et recherche ; Ã©criture Ã©ventuelle (compteurs hashtags) = WriteIntent KindMother si dÃ©finie.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuDiscovery - Tool Governance Compliance Contract](./contracts/governance/MiyuDiscovery%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

La documentation et la future implÃ©mentation de MiyuDiscovery sont conÃ§ues pour Ãªtre **compatibles MIP v1** (Miyukini Index Protocol). Ã€ l'implÃ©mentation, le code fournissant les Tools MiyuDiscovery devra Ãªtre balisÃ© MSCM afin que l'index MIP (blocks.json, domains.json, layers.json) soit gÃ©nÃ©rÃ© selon le [Protocole MIP v1](..//..//contrats//Miyukini%20Prompt%20Protocol%20-%20Ecriture%20Documentation%20Conceptuelle.md).

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


