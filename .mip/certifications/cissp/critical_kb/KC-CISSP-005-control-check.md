<!-- @id cert.critical_cyber.cissp.005_control_check -->
<!-- @do verify_control_conformity_cissp -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Verification controle CISSP -->

# KC-CISSP-005 - Control conformity protocol

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Pour verifier un controle, relier le controle a un domaine CISSP et au niveau de profondeur attendu.
- Controle acceptable si rationale risque + evidence operationnelle + gouvernance existent.
- Controle purement declaratif sans exploitation est non conforme.

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

- https://www.isc2.org/certifications/cissp/cissp-certification-exam-outline
- https://www.isc2.org/certifications/cissp/cissp-experience-requirements
- https://www.isc2.org/certifications/cissp/after-your-exam
