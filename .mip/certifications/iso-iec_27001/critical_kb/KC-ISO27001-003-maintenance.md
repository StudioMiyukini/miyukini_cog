<!-- @id cert.critical_cyber.iso27001.003_maintenance -->
<!-- @do verify_maintenance_conditions_iso27001 -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition maintien ISO/IEC 27001 -->

# KC-ISO27001-003 - Maintenance and cycle

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Surveillance periodique obligatoire pendant le cycle.
- Recertification au terme du cycle de certification.
- Le planning d audit doit etre compatible avec les exigences IAF.

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
