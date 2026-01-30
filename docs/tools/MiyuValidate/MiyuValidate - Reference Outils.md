# MiyuValidate — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuValidate. Référence technique des capacités atomiques de validation et sanitization. Schéma et politique fournis dans le flux ; pas de lecture base directe.

**Référence du kit :** [MiyuValidate - Documentation Fondatrice](./MiyuValidate%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.validate.schema.check` | Valider schéma | Valide des données selon un schéma fourni (ex. JSON Schema) ; retourne succès ou liste d'erreurs | 0–2 |
| `tool.validate.sanitize` | Sanitiser | Sanitise une valeur selon type et politique fournis (string, nombre, liste, échappement HTML, trim) | 0–2 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
