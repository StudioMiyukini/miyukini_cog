# MiyuJobs — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.jobs.miyujobs`

---

## Obligations spécifiques MiyuJobs

- **Décision de planifier** (schedule.at, schedule.cron) = **StrongFather** ; les Tools exécutent la planification après autorisation.
- **Décision d'enfiler** (queue.enqueue) = **StrongFather** ; contenu de la tâche fourni dans le flux gouverné.
- Persistance des jobs (queue, historique) = **WriteIntent** vers KindMother ou stockage gouverné selon implémentation.
- MiyuJobs n'exécute pas la logique métier des tâches : il planifie, enfile et déclenche le traitement ; handler fourni dans le flux.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
