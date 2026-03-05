<!-- @id cert.critical_cyber.iso27001.004_evidence -->
<!-- @do verify_evidence_requirements_iso27001 -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition preuves ISO/IEC 27001 -->

# KC-ISO27001-004 - Evidence requirements

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Preuves minimales: registre des risques, plan de traitement, SoA, preuves de fonctionnement des controles.
- Les controles Annex A doivent etre relies aux risques et objectifs.
- Le dossier d evidence doit etre versionne et auditable.

## Verification de controle

| Check | Question de verification | Preuve attendue | Decision |
|---|---|---|---|
| CTL-01 | La condition est-elle definie et documentee ? | Politique / procedure / reference | Conforme si oui |
| CTL-02 | La condition est-elle appliquee en operationnel ? | Trace execution / logs / tickets | Conforme si preuve recente |
| CTL-03 | La condition est-elle revue periodiquement ? | Revue datee + responsable | Non conforme si absence |

## Pratiques de preparation

- Cartographier la condition vers un controle operationnel concret.
- Assigner un owner et une periodicite de revue.
- Centraliser les preuves dans un dossier audit horodate.

## Sources officielles

- https://www.iso.org/certification.html
- https://www.iso.org/standard/27001
- https://www.iaf.nu/articles/ISO-IEC-270012022-APG-Documents/1415
- https://www.iaf.nu/articles/Determination-of-Audit-Time-of-Management-Systems/133
