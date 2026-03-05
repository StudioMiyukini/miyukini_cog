<!-- @id cert.critical_cyber.rgpd.001_entry -->
<!-- @do verify_entry_conditions_rgpd -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition entree RGPD (conformite) -->

# KC-RGPD-001 - Entry conditions

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- RGPD est une obligation legale continue et non une certification de personne.
- Le perimetre couvre responsable de traitement et sous-traitants.
- Les traitements doivent avoir finalite, base legale et minimisation.

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

- https://www.cnil.fr/fr/rgpd-de-quoi-parle-t-on
- https://www.cnil.fr/fr/les-sanctions-prononcees-par-la-cnil
- https://eur-lex.europa.eu/eli/reg/2016/679/oj
