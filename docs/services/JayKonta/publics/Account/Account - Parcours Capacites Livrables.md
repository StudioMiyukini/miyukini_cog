# JayKonta - Parcours, capacites et livrables (point d'entree Account)

## Contexte

Ce document traduit les besoins Account en parcours operationnels et livrables concrets.
Il est aligne avec les contrats CK-OP/CK-TK et le plan d'implementation.

## Portee

- In scope : onboarding, compta coeur, devis, factures, paiements, reporting, integrations
- Out of scope : details UX pixel-level, schemas API bas niveau

## Parcours P1 - Onboarding Account

### Etapes

1. Authentification et selection contexte Account
2. Validation profil legal si requis
3. Attribution role (admin/comptable/lecture)
4. Ouverture dashboard finance

### Livrables

- ecran onboarding Account
- gestion roles/permissions
- trace audit ouverture session

### Contrats

- CK-SVC-01
- CK-SVC-02
- CK-TK-01
- CK-SEC-02

## Parcours P2 - Comptabilite coeur

### Etapes

1. Saisie mouvement revenu/depense
2. Ventilation categorie/projet
3. Consultation journal/grand livre
4. Correction/annulation gouvernee

### Livrables

- formulaire mouvement
- vues journal/grand livre
- export comptable de base

### Contrats

- CK-TK-11
- CK-SEC-01
- CK-AUD-01

## Parcours P3 - Devis

### Etapes

1. Creation devis
2. Envoi devis
3. Suivi statut
4. Conversion devis vers facture

### Livrables

- ecran devis liste/detail
- action envoyer
- action convertir en facture

### Contrats

- CK-OP-11
- CK-OP-12
- CK-TK-21
- CK-AUD-03

## Parcours P4 - Facturation et encaissement

### Etapes

1. Emission facture
2. Envoi facture
3. Relance impaye
4. Enregistrement paiement
5. Mise a jour statut facture

### Livrables

- ecran factures et statuts
- relances manuelles/automatiques
- ecran encaissements

### Contrats

- CK-OP-13
- CK-OP-14
- CK-TK-31
- CK-TK-41
- CK-SEC-03

## Parcours P5 - Reporting et export

### Etapes

1. Consultation dashboard
2. Generation rapport legal
3. Export PDF/CSV scope controle
4. Journalisation export

### Livrables

- dashboard finance
- module rapports
- module export

### Contrats

- CK-OP-15
- CK-TK-51
- CK-AUD-02

## Parcours P6 - Integrations metier

### JayFestival

- quote.create
- invoice.emit
- budget.movements.record
- restitution reporting edition

Contrats : CK-INT-01

### JayRDV

- quote.create
- invoice.emit
- payment.record

Contrats : CK-INT-02

## Matrice capacite vers livrable

| Capacite | Livrable principal | Priorite |
|----------|--------------------|----------|
| Auth et contexte Account | Onboarding + roles | P0 |
| Mouvements/journal | Formulaire + vues GL | P0 |
| Devis | Module devis | P1 |
| Facturation | Module factures | P0 |
| Paiements | Module encaissements | P0 |
| Rapports | Dashboard + rapports legaux | P1 |
| Export | Export PDF/CSV | P1 |
| Integrations | Endpoints contractuels | P1 |

## Criteres de validation parcours

- CV-1 : parcours P1 complet en moins de 5 minutes
- CV-2 : parcours P3 -> P4 sans ressaisie
- CV-3 : parcours P5 exporte un rapport audite
- CV-4 : parcours P6 passe les tests de contrats

## Risques UX et reponses

- confusion statut devis/facture
- reponse : badges et timeline statut explicites

- surcharge dashboard
- reponse : widgets prioritaires et filtres simples

## References

- `docs/services/JayKonta/publics/Account/Account - Analyse des besoins.md`
- `docs/services/JayKonta/publics/Account/Account - Operateurs et Toolkits.md`
- `docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md`

## Statut

- Version : 2.0
- Date : 2026-02-07
- Statut : Parcours enrichis
