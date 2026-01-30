# MiyuSearch — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuSearch. Référence technique des capacités atomiques de recherche. Index alimenté sous autorité KindMother ; critères fournis dans le flux.

**Référence du kit :** [MiyuSearch - Documentation Fondatrice](./MiyuSearch%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.search.index.update` | Mise à jour index | Met à jour l'index (document, champs, identifiant fournis) ; flux gouverné / WriteIntent selon implémentation | 0–2 |
| `tool.search.query.execute` | Exécuter requête | Exécute une requête full-text (terme(s), filtres, options fournis) ; retourne identifiants et scores | 0–2 |
| `tool.search.suggest` | Suggestions | Produit des suggestions (autocomplete) à partir d'un préfixe et options fournis | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
