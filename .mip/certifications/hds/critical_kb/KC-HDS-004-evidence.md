<!-- @id cert.critical_cyber.hds.004_evidence -->
<!-- @do verify_evidence_requirements_hds -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition preuves HDS (France) -->

# KC-HDS-004 - Evidence requirements

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Preuves minimales: politique SSI, gestion acces, journalisation, chiffrement, gestion incidents.
- Traiter explicitement exigences sante: confidentialite, tracabilite, continuites.
- Conserver les preuves de supervision et revues periodiques.

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
