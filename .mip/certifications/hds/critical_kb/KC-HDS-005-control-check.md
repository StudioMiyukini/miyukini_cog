<!-- @id cert.critical_cyber.hds.005_control_check -->
<!-- @do verify_control_conformity_hds -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Verification controle HDS (France) -->

# KC-HDS-005 - Control conformity protocol

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Controle conforme uniquement si obligation legale + exigence HDS + preuve operationnelle sont toutes presentes.
- Manque de trace d acces ou de gestion incident = non conforme.
- Verifier aussi les clauses contractuelles de sous-traitance.

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

- https://esante.gouv.fr/labels-certifications/hds/certification-des-hebergeurs-de-donnees-de-sante
- https://www.legifrance.gouv.fr/loda/id/JORFTEXT000038135196/
- https://www.iso.org/standard/27001
