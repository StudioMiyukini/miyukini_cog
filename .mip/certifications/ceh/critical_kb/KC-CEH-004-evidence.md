<!-- @id cert.critical_cyber.ceh.004_evidence -->
<!-- @do verify_evidence_requirements_ceh -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition preuves CEH -->

# KC-CEH-004 - Evidence requirements

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Preuves readiness: playbooks pentest, rapports techniques, remediation retestee.
- Verifier chaines d evidence: scope, regles d engagement, traces d execution.
- Une competence offensive sans cadre legal est non recevable.

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
