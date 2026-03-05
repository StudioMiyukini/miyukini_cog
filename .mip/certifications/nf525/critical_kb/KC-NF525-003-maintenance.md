<!-- @id cert.critical_cyber.nf525.003_maintenance -->
<!-- @do verify_maintenance_conditions_nf525 -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition maintien NF525 / caisse securisee -->

# KC-NF525-003 - Maintenance and cycle

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Conservation et archivage doivent permettre controle fiscal et lecture des archives.
- Periodicite d archivage et tracabilite de purge doivent etre formalisees.
- Les donnees d origine et totaux de controle doivent rester intgres.

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

- https://bofip.impots.gouv.fr/bofip/10691-PGP.html/identifiant%3DBOI-TVA-DECLA-30-10-30-20210519
- https://www.legifrance.gouv.fr/codes/article_lc/LEGIARTI000036432356/
- https://www.lne.fr/fr/actualites/conformite-logiciels-caisse-engagez-demarche-avant-31-aout
