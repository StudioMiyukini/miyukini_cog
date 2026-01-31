# MiyuContacts — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.communication.contacts`

---

## Obligations spécifiques MiyuContacts

- **Décision** (ajout ami/ennemi autorisé, quota) = **StrongFather**.
- Toute écriture (friend add/remove, foe add/remove) = **WriteIntent** vers KindMother.
- Carnet d'adresses (pour MP) : réutilisation `tool.contacts.friend.list` ou outil dédié selon flux ; persistance = KindMother.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
