# MiyuComptaLedger — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuComptaLedger. Référence technique des capacités atomiques de tenue des livres. Persistance = KindMother (WriteIntent). Validation rapprochement = StrongFather.

**Référence du kit :** [MiyuComptaLedger - Documentation Fondatrice](./MiyuComptaLedger%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.compta.bank.sync` | Synchro bancaire | Déclenche ou enregistre une synchronisation bancaire (API/EBICS/agrégateur) | 2 |
| `tool.compta.transaction.categorize` | Catégoriser écriture | Catégorise une écriture (exécution ; règles fournies) | 1–2 |
| `tool.compta.transaction.vat.resolve` | Résoudre TVA | Rattache un taux TVA à une écriture | 0–1 |
| `tool.compta.reconciliation.suggest` | Proposer rapprochements | Propose des rapprochements (sans décider) | 0–1 |
| `tool.compta.reconciliation.record` | Enregistrer rapprochement | Enregistre un rapprochement validé ; autorisation = StrongFather | 2 |
| `tool.company.structure.resolve` | Résoudre structure | Résout la structure juridique courante (micro, EURL, etc.) pour le contexte | 0–1 |
| `tool.company.structure.register` | Enregistrer structure | Enregistre une structure (WriteIntent KindMother) | 2 |
| `tool.company.siret.resolve` | Résoudre SIRET | Récupère les informations depuis SIRET/INSEE (lecture seule) | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
