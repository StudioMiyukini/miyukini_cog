# MiyuJobs — Référence des outils

## 1. Contexte

Ce document décrit **chaque outil (Tool)** composant le kit MiyuJobs. Référence technique des capacités atomiques de planification et de file d'attente. Décision de planifier/enfiler = StrongFather ; contenu fourni dans le flux.

**Référence du kit :** [MiyuJobs - Documentation Fondatrice](./MiyuJobs%20-%20Documentation%20Fondatrice.md)

---

## 2. Tableau des outils

| ToolId | Nom lisible | Action | Niveau sécurité |
|--------|-------------|--------|------------------|
| `tool.jobs.schedule.at` | Planifier à | Planifie une exécution à une date/heure fournie ; autorisation = StrongFather | 1–2 |
| `tool.jobs.schedule.cron` | Planifier cron | Planifie une exécution selon expression cron fournie ; autorisation = StrongFather | 1–2 |
| `tool.jobs.queue.enqueue` | Enfiler | Enfile une tâche (payload, queue, options fournis) ; autorisation = StrongFather | 1–2 |
| `tool.jobs.queue.process` | Traiter queue | Traite une tâche (ou un lot) depuis une queue ; handler fourni dans le flux | 1–2 |

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Document de référence
