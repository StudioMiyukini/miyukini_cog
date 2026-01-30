# MiyuCalc — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuCalc. Référence technique des capacités atomiques de calcul et formatage numérique. Opérandes et options fournis dans le flux ; pas de lecture base.

**Référence du kit :** [MiyuCalc - Documentation Fondatrice](./MiyuCalc%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.calc.expression.evaluate` | Évaluer expression | Évalue une expression mathématique fournie (chaîne ou AST) ; retourne le résultat ; pas de décision métier | 0–1 |
| `tool.calc.number.format` | Formater nombre | Formate un nombre selon options fournies (devise, pourcentage, décimales, séparateurs, locale) | 0–1 |
| `tool.calc.unit.convert` | Convertir unité | Convertit une valeur d'une unité vers une autre (ex. km → miles, °C → °F) ; unités et valeur fournies | 0–1 |
| `tool.calc.round` | Arrondir | Arrondit une valeur selon mode (défaut, plancher, plafond, troncature) et précision fournis | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
