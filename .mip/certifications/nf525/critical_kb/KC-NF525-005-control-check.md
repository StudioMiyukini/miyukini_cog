<!-- @id cert.critical_cyber.nf525.005_control_check -->
<!-- @do verify_control_conformity_nf525 -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Verification controle NF525 / caisse securisee -->

# KC-NF525-005 - Control conformity protocol

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Controle conforme NF525 si 4 conditions legales + preuve version + mode de restitution sont verifies.
- Attestation non alignee a la version en production = non conforme.
- Verifier impacts de chaque patch sur validite de la preuve.

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
