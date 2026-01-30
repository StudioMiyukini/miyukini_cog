# MiyuLocale — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuLocale. Référence technique des capacités atomiques de locale et internationalisation. Locale et catalogue fournis dans le flux ; pas de lecture base directe.

**Référence du kit :** [MiyuLocale - Documentation Fondatrice](./MiyuLocale%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.locale.date.format` | Formater date | Formate une date/heure selon locale et options fournis (format court/long, timezone) | 0–1 |
| `tool.locale.number.format` | Formater nombre | Formate un nombre selon locale et options fournis (devise, décimales, séparateurs) | 0–1 |
| `tool.locale.translate` | Traduire | Résout une clé de traduction dans un catalogue fourni ; retourne la chaîne ou la clé si absent | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
