<!-- @id cert.critical_cyber.rgpd.002_obtention -->
<!-- @do verify_obtention_conditions_rgpd -->
<!-- @role cyber_compliance -->
<!-- @layer reference -->
<!-- @human Condition obtention RGPD (conformite) -->

# KC-RGPD-002 - Obtention pathway

> Fichier de connaissance atomique pour verification de controle certifiant.

## Conditions

- Obligations structurantes: registre, information, droits des personnes, contrats sous-traitants, securite.
- DPO obligatoire dans les cas prevus par le reglement.
- DPIA obligatoire en cas de risque eleve.

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
