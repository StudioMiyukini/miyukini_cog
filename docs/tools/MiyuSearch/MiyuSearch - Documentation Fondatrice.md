# MiyuSearch — Documentation Fondatrice

## 1. Contexte

**MiyuSearch** est le **kit d'outils (Toolkit)** de recherche de l'écosystème Miyukini. Il intègre les outils d'indexation de contenu (full-text), d'exécution de requêtes de recherche, et de suggestions (autocomplete), sans logique métier — les données à indexer et les critères de recherche sont fournis dans le flux gouverné ; les décisions sur ce qui est « pertinent » ou affiché relèvent des Opérateurs.

L'autorité sur les données indexées appartient à **KindMother**. MiyuSearch expose des capacités d'exécution gouvernée (indexer, requêter, suggérer) ; les données indexées proviennent de KindMother (via MiyuSQL ou flux) ; MiyuSearch ne décide pas du périmètre ni du classement métier.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuSearch, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (moteur full-text, stockage index) ; le classement métier et les règles de pertinence (Opérateurs / StrongFather).

---

## 3. Définition canonique

> **MiyuSearch est une composition officielle d'outils de recherche (indexation, requête full-text, suggestions), déclarée et gouvernée par l'environnement.**

- MiyuSearch **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuSearch **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (indexer du contenu, exécuter une requête, produire des suggestions) ; périmètre et critères fournis dans le flux ; pas de décision sur la pertinence métier.

**Règle fondamentale :** Un Tool MiyuSearch exécute sur des **données et critères fournis** ; l'index est alimenté sous autorité KindMother ; le classement métier reste du ressort des Opérateurs.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.search.miyusearch` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `search` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuSearch - Reference Outils](./MiyuSearch%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.search.index.update` | Met à jour l'index (document/champ fournis) ; WriteIntent ou flux gouverné vers stockage index |
| `tool.search.query.execute` | Exécute une requête full-text (critères fournis) ; retourne des identifiants / scores |
| `tool.search.suggest` | Produit des suggestions (autocomplete) à partir d'un préfixe fourni |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuSearch en contient trois.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : index alimenté sous autorité KindMother ; requêtes et suggestions sur critères fournis ; pas de décision métier sur pertinence.

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 2** (contenu indexé peut être sensible) |
| **États autorisés** | Tous sauf restriction WorrySentinel |
| **États interdits** | Selon politique (ex. index en lecture seule en maintenance) |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données sources. L'index MiyuSearch est une **dérivation** des données KindMother ; mise à jour d'index = flux gouverné (WriteIntent ou mécanisme documenté). Les Tools lisent l'index pour requêter et suggérer ; pas d'écriture métier directe sur les données sources.

Les obligations de conformité détaillées sont dans [MiyuSearch - Tool Governance Compliance Contract](./contracts/governance/MiyuSearch%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

À l'implémentation : chaque Tool MiyuSearch est une unité logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

---

## 10. Références croisées

| Document | Lien |
|----------|------|
| Glossaire | [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md) |
| Tools et Toolkits | [Miyukini Conceptual References - Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md) |
| Tool Governance Contract | [Master Butler - Tool Governance Contract](../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Contract.md) |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence fondateur
