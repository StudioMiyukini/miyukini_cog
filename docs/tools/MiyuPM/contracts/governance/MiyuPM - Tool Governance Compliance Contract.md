# MiyuPM — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.communication.pm`

---

## Obligations spécifiques MiyuPM

- **Décision d'envoi** (destinataire, quota, autorisation) = **StrongFather**.
- Toute écriture (message, dossier, brouillon) = **WriteIntent** vers KindMother.
- Flood control / anti-spam sur envoi = **MiyuAntiSpam** (tool.antispam.flood.check scope=pm) ; décision bloquer = StrongFather.

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
