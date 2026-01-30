# MiyuExport — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuExport. Référence technique des capacités atomiques d'export et de génération de documents. Données et options fournies dans le flux ; pas de lecture base directe.

**Référence du kit :** [MiyuExport - Documentation Fondatrice](./MiyuExport%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.export.csv.generate` | Générer CSV | Génère un fichier CSV à partir de données (tableau) et options fournis (délimiteur, encodage, locale) | 1–3 |
| `tool.export.xlsx.generate` | Générer XLSX | Génère un fichier Excel (XLSX) à partir de données et options fournis (feuilles, en-têtes, format) | 1–3 |
| `tool.export.pdf.render` | Rendre PDF | Rend un PDF à partir d'un template et de données fournis ; ne décide pas du contenu | 1–3 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
