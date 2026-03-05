<!-- @id cert.critical_cyber.ceh.001_entry -->
<!-- @do verify_entry_conditions_ceh -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition entree CEH -->

# KC-CEH-001 - Entry conditions

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Deux voies: formation officielle EC-Council ou eligibility sans formation.
- Sans formation officielle: experience InfoSec verifiable requise (2 ans).
- Dossier d eligibility et frais associes selon politique EC-Council.

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
