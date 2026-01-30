# MiyuText — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuText. Référence technique des capacités atomiques de traitement de texte. Contenu et options fournis dans le flux ; pas de lecture base.

**Référence du kit :** [MiyuText - Documentation Fondatrice](./MiyuText%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.text.markdown.render` | Rendu markdown | Rend du markdown fourni en HTML ; options (extensions, safe) fournies ; ne décide pas du contenu | 0–1 |
| `tool.text.replace` | Recherche/remplacement | Recherche et remplacement dans une chaîne (littéral ou regex fournis) ; retourne la chaîne modifiée | 0–1 |
| `tool.text.template.apply` | Template texte | Substitue des placeholders (ex. `{{ name }}`) dans un template avec des données fournies | 0–1 |
| `tool.text.sanitize` | Sanitization | Sanitise une chaîne pour affichage sécurisé (XSS, échappement HTML) ; politique fournie | 1–2 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
