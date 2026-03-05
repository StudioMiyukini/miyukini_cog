# MiyuPM â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.communication.pm`

---

## Obligations spÃ©cifiques MiyuPM

- **DÃ©cision d'envoi** (destinataire, quota, autorisation) = **StrongFather**.
- Toute Ã©criture (message, dossier, brouillon) = **WriteIntent** vers KindMother.
- Flood control / anti-spam sur envoi = **MiyuAntiSpam** (tool.antispam.flood.check scope=pm) ; dÃ©cision bloquer = StrongFather.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

