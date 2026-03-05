<!-- @id cert.critical_cyber.ceh.005_control_check -->
<!-- @do verify_control_conformity_ceh -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Verification controle CEH -->

# KC-CEH-005 - Control conformity protocol

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Controle conforme CEH si legalite + methode + preuves + recommandations sont presentes.
- Test hors perimetre autorise = non conforme critique.
- Rapport sans preuve reproductible = non conforme.

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

- https://cert.eccouncil.org/application-process-eligibility.html
- https://www.eccouncil.org/train-certify/certified-ethical-hacker-ceh/
- https://www.eccouncil.org/wp-content/uploads/2023/04/CEH-Exam-Blueprint-v4.0.pdf
