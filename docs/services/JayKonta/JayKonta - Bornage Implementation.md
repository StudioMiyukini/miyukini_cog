# JayKonta - Bornage Implementation

## Contexte

Ce document fixe le bornage de l'implementation JayKonta :
- perimetre en phase 1, phase 2, phase 3
- limites explicites
- dependances
- criteres de livraison

Il est aligne sur la documentation JayKonta existante et sur le benchmark PR.

## Portee

- Perimetre : implementation des operateurs et toolkits du service JayKonta
- Hors perimetre : migration inter-version COG complete, connectors bancaires proprietaires complets en phase 1

## Principes de bornage

- prioriser les flux coeur a forte valeur metier
- limiter la dette d'integration en phase 1
- garder une separation nette Purse versus Account
- appliquer securite et audit des le premier lot

## 1. Phase 1 - Coeur fonctionnel (in scope)

### Purse

- compte et session Purse
- mouvements et categories
- budgets occasionnels
- objectifs et alertes
- rapport synthese et export CSV/PDF

### Account

- compte et roles Account
- grand livre et journal
- devis creation, envoi, conversion
- facture emission, relances, encaissements
- dashboard finance de base
- rapports legaux de base et export

### Integrations

- JayFestival : quote, invoice, budget by edition
- JayRDV : quote, invoice, payment
- JayKoa : reminders (optionnel)

### Gouvernance obligatoire

- mandat sur toutes ecritures
- audit sur actions critiques
- classification donnees niveau 2+ active

## 2. Phase 1 - Hors scope

- OCR avancee de justificatifs
- rapprochement bancaire automatise complet multi-connecteurs
- multi-entite avancee avec consolidation inter-filiales
- portail client complet libre-service multi-role
- fiscalite internationale avancee

## 3. Phase 2 - Extension fonctionnelle

- rapprochement bancaire semi-automatise
- import releves et regles de categorisation
- AP structure (fournisseurs, bills, echeanciers)
- cash-flow previsionnel
- vues analytiques avancees

## 4. Phase 3 - Industrialisation

- optimisation perf a grande volumetrie
- durcissement multi-COG inter-organisations
- connecteurs et contrats d'integration supplementaires
- suite tests non-regression complete

## 5. Dependances

### Dependances internes

- Miyauth
- Master Butler
- StrongFather
- KindMother
- WorrySentinel
- Miyunotify

### Dependances inter-services

- JayFestival
- JayRDV
- JayKoa (optionnel)

## 6. Livrables obligatoires phase 1

- operateurs Purse et Account implementes
- toolkits coeur implementes
- contrats normatifs valides
- ecrans metiers clefs disponibles
- tests unitaires coeur metier
- trace audit operationnelle

## 7. Criteres de fin de phase 1

- CF-01 : parcours Purse complet operable de bout en bout
- CF-02 : parcours Account devis->facture->paiement operable
- CF-03 : integration JayFestival operative sur 3 appels coeur
- CF-04 : integration JayRDV operative sur 3 appels coeur
- CF-05 : export rapports operable avec restrictions de scope
- CF-06 : audit et securite conformes aux niveaux definis

## 8. Risques de phase et mitigation

- risque de confusion Purse/Account
- mitigation : separation routes, permissions et etiquetage UX

- risque de retard integration
- mitigation : contrats d'integration minimaux puis extension

- risque de performance rapports
- mitigation : vues agreges et limitation des exports lourds en phase 1

## 9. Hors scope explicite global

- implementation full ERP hors finance
- orchestration logistique non finance
- moteur IA prediction avancee
- connecteurs bancaires premium non critiques

## References

- docs/services/JayKonta/JayKonta - Documentation Enrichie.md
- docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md
- docs/services/JayKonta/reference/JayKonta - Integration Services.md
- docs/services/JayKonta/publics/Account/Account - Parcours Capacites Livrables.md
- docs/services/JayKonta/publics/Purse/Purse - Parcours Capacites Livrables.md

## Statut

- Version : 1.0
- Date : 2026-02-07
- Statut : Reference bornage implementation
