# MiyuCalc â€” Documentation Fondatrice

## 1. Contexte

**MiyuCalc** est le **kit d'outils (Toolkit)** de calcul et formatage numÃ©rique de l'Ã©cosystÃ¨me Miyukini. Il intÃ¨gre les outils d'Ã©valuation d'expressions mathÃ©matiques, de formatage des nombres (devise, pourcentage, dÃ©cimales, locale), et de conversion d'unitÃ©s (longueur, masse, temps, etc.), sans logique mÃ©tier â€” les donnÃ©es et rÃ¨gles sont fournies dans le flux gouvernÃ©.

L'autoritÃ© sur les donnÃ©es mÃ©tier (barÃ¨mes, rÃ¨gles de calcul mÃ©tier) appartient Ã  **KindMother**. MiyuCalc expose des capacitÃ©s d'exÃ©cution gouvernÃ©e (Ã©valuer, formater, convertir) ; les dÃ©cisions sur ce qui doit Ãªtre calculÃ© ou affichÃ© relÃ¨vent de **StrongFather** et des OpÃ©rateurs.

**Terminologie officielle :** [Miyukini Conceptual References - Glossaire](..//..//miyukini-webway-system//reference//_index.md)

---

## 2. PortÃ©e / Scope

**Ce document dÃ©finit :** l'identitÃ© et la dÃ©finition canonique de MiyuCalc, le ToolkitId, la liste des outils composants, la gouvernance, le niveau de sÃ©curitÃ©, la relation avec KindMother.

**Hors scope :** l'implÃ©mentation dÃ©taillÃ©e (moteur d'expressions, bibliothÃ¨ques d'unitÃ©s) ; les calculs mÃ©tier spÃ©cifiques (voir MiyuTreasury, MiyuExpense, MiyuStore, etc.) qui orchestrent MiyuCalc avec leurs rÃ¨gles.

---

## 3. DÃ©finition canonique

> **MiyuCalc est une composition officielle d'outils de calcul et formatage numÃ©rique (Ã©valuation d'expressions, formatage nombres, conversion d'unitÃ©s), dÃ©clarÃ©e et gouvernÃ©e par l'environnement.**

- MiyuCalc **n'est pas** un nouveau Tool : c'est un **Kit d'Outils (Toolkit)** qui agrÃ¨ge des Tools existants.
- MiyuCalc **n'ajoute aucune logique mÃ©tier** : il orchestre des capacitÃ©s atomiques (Ã©valuer une expression, formater un nombre, convertir une unitÃ©) ; les opÃ©randes et options sont fournis dans le flux.

**RÃ¨gle fondamentale :** Un Tool MiyuCalc exÃ©cute sur des **donnÃ©es fournies** (expression, valeur, unitÃ©) ; il ne lit pas la base ni ne dÃ©cide des rÃ¨gles mÃ©tier.

---

## 4. Identifiant et catalogue

| Ã‰lÃ©ment | Valeur |
|--------|--------|
| **ToolkitId** | `toolkit.calc.miyucalc` |
| **Format** | `toolkit.<domain>.<name>` (conforme Master Butler) |
| **Domaine** | `calc` |
| **Catalogue** | Master Butler dÃ©clare le Toolkit et la liste des Tools composants. |

---

## 5. Liste des outils composants

Le dÃ©tail de chaque outil est dÃ©crit dans [MiyuCalc - Reference Outils](./MiyuCalc%20-%20Reference%20Outils.md).

| ToolId | Description courte |
|--------|---------------------|
| `tool.calc.expression.evaluate` | Ã‰value une expression mathÃ©matique fournie (sans dÃ©cision mÃ©tier) |
| `tool.calc.number.format` | Formate un nombre selon options fournies (devise, pourcentage, dÃ©cimales, locale) |
| `tool.calc.unit.convert` | Convertit une valeur d'une unitÃ© vers une autre (donnÃ©es fournies) |
| `tool.calc.round` | Arrondit une valeur selon mode et prÃ©cision fournis |

**Invariant (Toolkit Composition Contract) :** Un Toolkit contient au moins deux Tools. MiyuCalc en contient quatre.

---

## 6. Gouvernance

Flux de gouvernance standard (voir [Tools et Toolkits](..//..//miyukini-webway-system//reference//_index.md)). SpÃ©cificitÃ© : opÃ©randes et options fournis dans le flux ; aucune lecture base ; toute Ã©criture mÃ©tier = WriteIntent KindMother (MiyuCalc n'Ã©crit pas).

---

## 7. Niveau de sÃ©curitÃ© et Ã©tats

| Ã‰lÃ©ment | Valeur |
|---------|--------|
| **Niveau de sÃ©curitÃ© du kit** | **0 Ã  1** (calculs et formatage ; pas de donnÃ©es sensibles par dÃ©faut) |
| **Ã‰tats autorisÃ©s** | Tous sauf restriction explicite |
| **Ã‰tats interdits** | Aucun (exÃ©cution locale, pas d'I/O externe critique) |

---

## 8. Relation avec KindMother

**KindMother** est l'autoritÃ© sur les donnÃ©es mÃ©tier. MiyuCalc **n'Ã©crit pas** et **ne lit pas** la base : il opÃ¨re sur des valeurs et expressions **fournies dans le flux**. Les OpÃ©rateurs (ex. MiyuTreasury, MiyuStore) rÃ©cupÃ¨rent les donnÃ©es via MiyuSQL sous autoritÃ© KindMother, puis appellent MiyuCalc pour calculer ou formater.

Les obligations de conformitÃ© dÃ©taillÃ©es sont dans [MiyuCalc - Tool Governance Compliance Contract](./contracts/governance/MiyuCalc%20-%20Tool%20Governance%20Compliance%20Contract.md).

---

## 9. Alignement MIP

Ã€ l'implÃ©mentation : chaque Tool MiyuCalc est une unitÃ© logique pouvant devenir un **bloc MSCM** : `id`, `do`, `role`, `layer` pour alimenter blocks.json.

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


