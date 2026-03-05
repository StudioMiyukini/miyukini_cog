# MiyuPolls â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.content.polls`

---

## Obligations spÃ©cifiques MiyuPolls

- **DÃ©cision** (crÃ©ation sondage, vote autorisÃ©, clÃ´ture) = **StrongFather**.
- Toute Ã©criture (sondage, vote) = **WriteIntent** vers KindMother.
- RÃ¨gles d'unicitÃ© de vote (un vote par utilisateur par sondage) = politique StrongFather ; l'outil exÃ©cute la vÃ©rification fournie.

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

