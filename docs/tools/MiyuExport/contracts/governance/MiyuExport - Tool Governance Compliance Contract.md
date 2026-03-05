# MiyuExport â€” Tool Governance Compliance Contract

## Contexte

ConformitÃ© aux obligations communes : [Master Butler - Tool Governance Compliance Template](..//..//..//..//cores//MasterButler//contracts//tools//Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.export.miyuexport`

---

## Obligations spÃ©cifiques MiyuExport

- MiyuExport **ne lit pas** la base directement : donnÃ©es Ã  exporter fournies dans le flux (aprÃ¨s lecture via MiyuSQL sous autoritÃ© KindMother).
- **DÃ©cision d'export** (pÃ©rimÃ¨tre, destinataire) = **StrongFather** ; les Tools exÃ©cutent la gÃ©nÃ©ration sur donnÃ©es fournies.
- Aucune Ã©criture en base mÃ©tier : produit un flux binaire (CSV, XLSX, PDF).

---

**Date de crÃ©ation :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformitÃ©

