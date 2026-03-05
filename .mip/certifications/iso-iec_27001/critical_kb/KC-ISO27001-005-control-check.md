<!-- @id cert.critical_cyber.iso27001.005_control_check -->
<!-- @do verify_control_conformity_iso27001 -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Verification controle ISO/IEC 27001 -->

# KC-ISO27001-005 - Control conformity protocol

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Decision de conformite controle: conforme / partiel / non conforme.
- Chaque controle audite doit avoir preuve + proprietaire + periodicite.
- Absence de preuve ou preuve obsolete = non conformite majeure.

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
