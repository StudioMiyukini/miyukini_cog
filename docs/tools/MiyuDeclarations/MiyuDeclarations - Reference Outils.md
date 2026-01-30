# MiyuDeclarations — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuDeclarations. Référence technique des capacités atomiques de déclarations fiscales et sociales. Persistance historique = KindMother (WriteIntent). Soumission = StrongFather.

**Référence du kit :** [MiyuDeclarations - Documentation Fondatrice](./MiyuDeclarations%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.compta.declaration.urssaf.prepare` | Préparer URSSAF | Prépare les données de déclaration URSSAF (CA, etc.) | 1–2 |
| `tool.compta.declaration.urssaf.submit` | Soumettre URSSAF | Soumet la déclaration URSSAF (télédéclaration) ; autorisation = StrongFather | 2 |
| `tool.compta.declaration.tva.prepare` | Préparer TVA | Prépare la déclaration TVA | 1–2 |
| `tool.compta.declaration.tva.submit` | Soumettre TVA | Soumet la déclaration TVA ; autorisation = StrongFather | 2 |
| `tool.compta.declaration.deadline.list` | Échéances | Liste les échéances fiscales et sociales (données fournies) | 0–1 |
| `tool.compta.declaration.list` | Historique déclarations | Liste l'historique des déclarations (filtres fournis) | 0–1 |
| `tool.compta.declaration.estimate.cotisations` | Estimateur cotisations | Calcule une estimation des cotisations (micro) à partir de CA fourni | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
