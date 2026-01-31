# MiyuCalc — Documentation Fondatrice

## 1. Contexte

**MiyuCalc** est le **kit d'outils (Toolkit)** de calcul et formatage numérique de l'écosystème Miyukini. Il intègre les outils d'évaluation d'expressions mathématiques, de formatage des nombres (devise, pourcentage, décimales, locale), et de conversion d'unités (longueur, masse, temps, etc.), sans logique métier — les données et règles sont fournies dans le flux gouverné.

L'autorité sur les données métier (barèmes, règles de calcul métier) appartient à **KindMother**. MiyuCalc expose des capacités d'exécution gouvernée (évaluer, formater, convertir) ; les décisions sur ce qui doit être calculé ou affiché relèvent de **StrongFather** et des Opérateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)

---

## 2. Portée / Scope

**Ce document définit :** l'identité et la définition canonique de MiyuCalc, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sécurité, la relation avec KindMother.

**Hors scope :** l'implémentation détaillée (moteur d'expressions, bibliothèques d'unités) ; les calculs métier spécifiques (voir MiyuTreasury, MiyuExpense, MiyuStore, etc.) qui orchestrent MiyuCalc avec leurs règles.

---

## 3. Définition canonique

> **MiyuCalc est une composition officielle d'outils de calcul et formatage numérique (évaluation d'expressions, formatage nombres, conversion d'unités), déclarée et gouvernée par l'environnement.**

- MiyuCalc **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrège des Tools existants.
- MiyuCalc **n'ajoute aucune logique métier** : il orchestre des capacités atomiques (évaluer une expression, formater un nombre, convertir une unité) ; les opérandes et options sont fournis dans le flux.

**Règle fondamentale :** Un Tool MiyuCalc exécute sur des **données fournies** (expression, valeur, unité) ; il ne lit pas la base ni ne décide des règles métier.

---

## 4. Identifiant et catalogue

| Élément | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.calc.miyucalc` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `calc` |
| **Catalogue** | Master Butler déclare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le détail de chaque outil est décrit dans [MiyuCalc - Reference Outils](./MiyuCalc%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.calc.expression.evaluate` | Évalue une expression mathématique fournie (sans décision métier) |
| `tool.calc.number.format` | Formate un nombre selon options fournies (devise, pourcentage, décimales, locale) |
| `tool.calc.unit.convert` | Convertit une valeur d'une unité vers une autre (données fournies) |
| `tool.calc.round` | Arrondit une valeur selon mode et précision fournis |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuCalc en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](../../reference/Miyukini%20Conceptual%20References%20-%20Tools%20et%20Toolkits.md)). Spécificité : opérandes et options fournis dans le flux ; aucune lecture base ; toute écriture métier = WriteIntent KindMother (MiyuCalc n'écrit pas).

---

## 7. Niveau de sécurité et états

| Élément | Valeur |
|---------|--------|
| **Niveau de sécurité du kit** | **0 à 1** (calculs et formatage ; pas de données sensibles par défaut) |
| **États autorisés** | Tous sauf restriction explicite |
| **États interdits** | Aucun (exécution locale, pas d'I/O externe critique) |

---

## 8. Relation avec KindMother

**KindMother** est l'autorité sur les données métier. MiyuCalc **n'écrit pas** et **ne lit pas** la base : il opère sur des valeurs et expressions **fournies dans le flux**. Les Opérateurs (ex. MiyuTreasury, MiyuStore) récupèrent les données via MiyuSQL sous autorité KindMother, puis appellent MiyuCalc pour calculer ou formater.

Les obligations de conformité détaillées sont dans [MiyuCalc - Tool Governance Compliance Contract](./contracts/governance/MiyuCalc%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

À l'implémentation : chaque Tool MiyuCalc est une unité logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

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
