# MiyuExport — Tool Governance Compliance Contract

## Contexte

Conformité aux obligations communes : [Master Butler - Tool Governance Compliance Template](../../../../core/MasterButler/contracts/tools/Master%20Butler%20-%20Tool%20Governance%20Compliance%20Template.md).

**ToolkitId :** `toolkit.export.miyuexport`

---

## Obligations spécifiques MiyuExport

- MiyuExport **ne lit pas** la base directement : données à exporter fournies dans le flux (après lecture via MiyuSQL sous autorité KindMother).
- **Décision d'export** (périmètre, destinataire) = **StrongFather** ; les Tools exécutent la génération sur données fournies.
- Aucune écriture en base métier : produit un flux binaire (CSV, XLSX, PDF).

---

**Date de création :** 2026-01-30  
**Version :** 1.0  
**Statut :** Contrat de conformité
