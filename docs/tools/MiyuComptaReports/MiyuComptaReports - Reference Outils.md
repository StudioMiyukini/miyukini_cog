# MiyuComptaReports — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuComptaReports. Référence technique des capacités atomiques de rapports comptables. Données = KindMother (lecture). Autorisation export (liasse, ledger) = StrongFather.

**Référence du kit :** [MiyuComptaReports - Documentation Fondatrice](./MiyuComptaReports%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.compta.report.livre_recettes.generate` | Livre des recettes | Génère le livre des recettes ; données = KindMother | 1–2 |
| `tool.compta.report.balance.generate` | Bilan / CR | Génère bilan / compte de résultat ; lecture seule | 1–2 |
| `tool.compta.report.liasse.generate` | Liasse fiscale | Génère la liasse fiscale (export) ; autorisation = StrongFather | 2 |
| `tool.compta.report.cashflow.generate` | Flux de trésorerie | Génère un rapport flux de trésorerie / prévisionnel | 1–2 |
| `tool.compta.export.ledger` | Export écritures | Export des écritures (format fourni) ; autorisation = StrongFather | 2 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
