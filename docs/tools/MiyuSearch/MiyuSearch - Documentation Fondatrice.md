# MiyuSearch â€” Documentation Fondatrice

## 1. Contexte

**MiyuSearch** est le **kit d'outils (Toolkit)** de recherche de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'indexation de contenu (full-text), d'exÃ©cution de requÃªtes de recherche, et de suggestions (autocomplete), sans logique mÃ©tier â€” les donnÃ©es Ã  indexer et les critÃ¨res de recherche sont fournis dans le flux gouvernÃ© ; les dÃ©cisions sur ce qui est Â« pertinent Â» ou affichÃ© relÃ¨vent des OpÃ©rateurs.

L'autoritÃ© sur les donnÃ©es indexÃ©es appartient Ã  **KindMother**. MiyuSearch expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (indexer, requÃªter, suggÃ©rer) ; les donnÃ©es indexÃ©es proviennent de KindMother (via MiyuSQL ou flux) ; MiyuSearch ne dÃ©cide pas du pÃ©rimÃ¨tre ni du classement mÃ©tier.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuSearch, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (moteur full-text, stockage index) ; le classement mÃ©tier et les rÃ¨gles de pertinence (OpÃ©rateurs / StrongFather).

---

## 3. DÃ©finition canonique

> **MiyuSearch est une composition officielle d'outils de recherche (indexation, requÃªte full-text, suggestions), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuSearch **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuSearch **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (indexer du contenu, exÃ©cuter une requÃªte, produire des suggestions) ; pÃ©rimÃ¨tre et critÃ¨res fournis dans le flux ; pas de dÃ©cision sur la pertinence mÃ©tier.

**RÃ¨gle fondamentale :** Un Tool MiyuSearch exÃ©cute sur des **donnÃ©es et critÃ¨res fournis** ; l'index est alimentÃ© sous autoritÃ© KindMother ; le classement mÃ©tier reste du ressort des OpÃ©rateurs.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.search.miyusearch` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `search` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuSearch - Reference Outils](./MiyuSearch%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.search.index.update` | Met Ã  jour l'index (document/champ fournis) ; WriteIntent ou flux gouvernÃ© vers stockage index |
| `tool.search.query.execute` | ExÃ©cute une requÃªte full-text (critÃ¨res fournis) ; retourne des identifiants / scores |
| `tool.search.suggest` | Produit des suggestions (autocomplete) Ã  partir d'un prÃ©fixe fourni |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSearch en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : index alimentÃ© sous autoritÃ© KindMother ; requÃªtes et suggestions sur critÃ¨res fournis ; pas de dÃ©cision mÃ©tier sur pertinence.

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  2** (contenu indexÃ© peut Ãªtre sensible) |
| **Ã‰tats autorisÃ©s** | Tous sauf restriction WorrySentinel |
| **Ã‰tats interdits** | Selon politique (ex. index en lecture seule en maintenance) |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es sources. L'index MiyuSearch est une **dÃ©rivation** des donnÃ©es KindMother ; mise Ã  jour d'index = flux gouvernÃ© (WriteIntent ou mÃ©canisme documentÃ©). Les Tools lisent l'index pour requÃªter et suggÃ©rer ; pas d'Ã©criture mÃ©tier directe sur les donnÃ©es sources.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuSearch - Tool Governance Compliance Contract](./contracts/governance/MiyuSearch%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

Ã€ l'implÃ©mentation : chaque Tool MiyuSearch est une unitÃ© logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. RÃ©fÃ©rences croisÃ©es

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de rÃ©fÃ©rence fondateur


