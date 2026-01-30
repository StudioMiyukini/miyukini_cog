# MiyuExpense — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuExpense. Référence technique des capacités atomiques de notes de frais et indemnités. Persistance = KindMother (WriteIntent). Validation et export = StrongFather.

**Référence du kit :** [MiyuExpense - Documentation Fondatrice](./MiyuExpense%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.expense.receipt.capture` | Capturer justificatif | Enregistre un justificatif (photo/scan) ; WriteIntent KindMother | 1–2 |
| `tool.expense.receipt.extract` | Extraire OCR | Extrait les données d'un justificatif par OCR (exécution seule) | 0–1 |
| `tool.expense.claim.create` | Créer note de frais | Crée une note de frais à partir de données fournies | 1–2 |
| `tool.expense.claim.update` | Mettre à jour note | Met à jour une note de frais | 1–2 |
| `tool.expense.claim.list` | Lister notes | Liste les notes de frais (filtres fournis) | 0–1 |
| `tool.expense.claim.validate` | Valider note | Valide une note de frais (workflow ; décision = StrongFather) | 2 |
| `tool.expense.mileage.calculate` | Calcul indemnités km | Calcule les indemnités kilométriques selon barème fourni | 0–1 |
| `tool.expense.mileage.export` | Export indemnités km | Export PDF/CSV des indemnités pour administration | 1 |
| `tool.expense.claim.export` | Export vers compta | Export des notes de frais vers compta ; autorisation = StrongFather | 2 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
