# JayBudget - Parcours, capacites et livrables (point d'entree Purse)

## Contexte

Ce document decrit les parcours utilisateur Purse et les livrables cibles.
Il aligne les flux UX avec les contrats et besoins enrichis.

## Portee

- In scope : onboarding, mouvements, budgets occasionnels, objectifs, alertes, rapports/export
- Out of scope : devis/facturation entreprise

## Parcours P1 - Onboarding Purse

### Etapes

1. Inscription/connexion
2. Choix contexte Purse
3. Arrivee dashboard

### Livrables

- ecran auth Purse
- ecran dashboard initial

### Contrats

- CK-SVC-01
- CK-SVC-02
- CK-TK-01

## Parcours P2 - Saisie budget quotidien

### Etapes

1. Ajouter mouvement
2. Choisir categorie
3. Valider
4. Voir impact solde

### Livrables

- formulaire mouvement rapide
- liste mouvements filtrable
- widget solde

### Contrats

- CK-OP-02
- CK-TK-11
- CK-AUD-01

## Parcours P3 - Budget occasionnel

### Etapes

1. Creer budget occasionnel
2. Affecter mouvements
3. Suivre depense/restant
4. Clore budget

### Livrables

- ecran liste budgets occasionnels
- ecran detail budget occasionnel

### Contrats

- CK-OP-03
- CK-TK-61

## Parcours P4 - Objectifs

### Etapes

1. Creer objectif
2. Suivre progression
3. Recevoir alertes seuil

### Livrables

- ecran objectifs
- indicateurs progression
- alertes objectifs

### Contrats

- CK-TK-61

## Parcours P5 - Rapports et export

### Etapes

1. Ouvrir rapport mensuel
2. Filtrer periode
3. Exporter PDF/CSV

### Livrables

- ecran rapports Purse
- module export personnel

### Contrats

- CK-TK-51
- CK-AUD-02

## Parcours P6 - Rappels optionnels

### Etapes

1. Activer rappels
2. Publier echeances vers JayKoa
3. Recevoir rappel

### Livrables

- option reminders
- liaison JayKoa de base

### Contrats

- CK-INT-03

## Matrice capacite vers livrable

| Capacite | Livrable principal | Priorite |
|----------|--------------------|----------|
| Auth Purse | Onboarding | P0 |
| Mouvements | Formulaire + historique | P0 |
| Categories | Gestion categories | P0 |
| Budgets occasionnels | Liste + detail | P1 |
| Objectifs | Module objectifs | P2 |
| Alertes | Preferences + notifications | P3 |
| Rapports/Export | Module reporting Purse | P2 |
| Rappels JayKoa | Option reminder | P3 |

## Criteres de validation parcours

- CV-1 : parcours P2 complet en moins de 30 secondes
- CV-2 : parcours P3 sans ambiguite de solde
- CV-3 : parcours P5 export audite

## Risques UX et reponses

- surcharge ecran principal
- reponse : widgets prioritaires, details deferes

- alert fatigue
- reponse : profilage alertes, seuils configurables

## References

- `docs/services/JayKonta/publics/Purse/Purse - Analyse des besoins.md`
- `docs/services/JayKonta/publics/Purse/Purse - Operateurs et Toolkits.md`
- `docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md`

## Statut

- Version : 2.0
- Date : 2026-02-07
- Statut : Parcours enrichis
