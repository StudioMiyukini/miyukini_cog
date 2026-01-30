# MiyuHR — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuHR. Référence technique des capacités atomiques RH (pointeuse, planning). Persistance = KindMother (WriteIntent). Permissions = Master Butler + StrongFather.

**Référence du kit :** [MiyuHR - Documentation Fondatrice](./MiyuHR%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.hr.time_clock.in` | Entrée (début shift) | Enregistre une entrée (début de shift) ; heures = KindMother | 1–2 |
| `tool.hr.time_clock.out` | Sortie (fin shift) | Enregistre une sortie (fin de shift) | 1–2 |
| `tool.hr.schedule.get` | Lecture planning | Retourne le planning (shifts) pour un employé/période | 0–1 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
