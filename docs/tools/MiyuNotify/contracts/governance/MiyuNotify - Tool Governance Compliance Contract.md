# MiyuNotify â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.notify.miyunotify`

---

## Obligations spÃ©cifiques MiyuNotify

- **DÃ©cision d'envoi** (email, push) = **StrongFather** ; les Tools exÃ©cutent l'envoi aprÃ¨s autorisation.
- `tool.notify.inbox.write` : Ã©criture = **WriteIntent** vers KindMother.
- Aucun Tool ne dÃ©cide Ã  qui envoyer ni quel contenu : destinataire et contenu fournis dans le flux gouvernÃ©.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

