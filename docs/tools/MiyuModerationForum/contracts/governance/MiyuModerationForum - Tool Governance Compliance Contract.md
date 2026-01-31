# MiyuModerationForum — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.moderation.forum`

---

## Obligations spécifiques MiyuModerationForum

- **Toute décision de modération** (lock, move, merge, split, delete, avertissement, bannissement) = **StrongFather**.
- Toute écriture (report, warning, ban, usernote, état topic/post) = **WriteIntent** vers KindMother.
- Seuls les Opérateurs autorisés (Mandat StrongFather) peuvent invoquer les Tools d'action (lock, move, merge, split, warning, ban).

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
