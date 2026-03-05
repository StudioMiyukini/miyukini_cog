<!-- @id cert.critical_cyber.cisa.003_maintenance -->
<!-- @do verify_maintenance_conditions_cisa -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition maintien CISA -->

# KC-CISA-003 - Maintenance and cycle

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Maintien: CPE annuel minimal et CPE cumule sur cycle triennal.
- Paiement annuel de maintenance requis.
- Conformite a la politique d audit CPE ISACA requise.

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

- https://www.isaca.org/credentialing/cisa/cisa-exam-content-outline
- https://www.isaca.org/credentialing/cisa/get-cisa-certified
- https://www.isaca.org/credentialing/cisa/maintain-cisa-certification
