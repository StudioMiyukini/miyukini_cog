# MiyuPosAnalytics — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuPosAnalytics. Référence technique des capacités atomiques d'analytics ventes ; clôture shift = autorisation StrongFather. Données = KindMother.

**Référence du kit :** [MiyuPosAnalytics - Documentation Fondatrice](./MiyuPosAnalytics%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.analytics.sales.trend` | Tendance ventes | Retourne tendance ventes (période, comparaison) ; données = KindMother | 0–1 |
| `tool.analytics.sales.by_item` | Ventes par article | Retourne ventes par article (top N, filtres) ; lecture seule | 0–1 |
| `tool.analytics.sales.by_employee` | Ventes par employé | Retourne les ventes agrégées par employé (filtres fournis) | 0–1 |
| `tool.analytics.cash.discrepancy` | Écart caisse | Retourne l'écart caisse pour un shift | 0–1 |
| `tool.analytics.tax.report` | Rapport taxes | Retourne rapport taxes (période, filtres) | 0–1 |
| `tool.pos.shift.close` | Clôture shift | Clôture un shift caisse (comptage, écart) ; autorisation = StrongFather | 2 |
| `tool.data.export.spreadsheet` | Export tableur | Exporte des données en format tableur (données fournies) | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
