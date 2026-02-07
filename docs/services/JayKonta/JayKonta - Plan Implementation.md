# JayKonta - Plan Implementation

## Contexte

Ce document est le plan d'implementation de JayKonta, organise par phases et taches.
Le plan couvre Purse, Account, integrations et verification.

## Portee

- Perimetre : lotissement implementation operateurs, toolkits, ecrans fonctionnels, integration et verification
- Hors perimetre : sujets explicitement classes phase 2 ou 3 dans le bornage

## Regles de travail

- 1 tache = 1 fichier principal livre
- 1 bloc fonctionnel = 1 id de tracabilite
- tests obligatoires quand logique metier modifiee
- aucune ecriture sensible hors mandat et audit

## Phase 0 - Preparation

### Objectif

- verrouiller perimetre, dependances, nomenclature

### Taches

- [001] valider mapping besoins Purse et Account
- [002] valider contrats normatifs CK-*
- [003] valider criteres CF-01 a CF-06

## Phase 1 - Fondations service

### Objectif

- etablir ossature commune JayKonta

### Taches

- [101] modeles metier coeur (AccountProfile, LedgerEntry, Quote, Invoice, PaymentRecord)
- [102] couche auth contextuelle Purse/Account
- [103] couche permissions et mandats
- [104] couche audit minimale

## Phase 2 - Purse (JayBudget)

### Objectif

- livrer parcours Purse bout en bout

### Taches

- [201] ecrans Purse dashboard et mouvements
- [202] budgets occasionnels et objectifs
- [203] alertes et rappels optionnels JayKoa
- [204] rapports Purse et export

### Criteres

- parcours Purse complet operable
- niveau securite 2 applique

## Phase 3 - Account (JayKonta)

### Objectif

- livrer parcours comptable entreprise coeur

### Taches

- [301] grand livre et journal
- [302] devis (create, send, convert)
- [303] factures (emit, remind)
- [304] encaissements et statuts facture

### Criteres

- flux devis->facture->paiement operationnel
- audit complet des ecritures critiques

## Phase 4 - Reporting and Export

### Objectif

- livrer dashboards et rapports conformes

### Taches

- [401] dashboard finance Account
- [402] rapports legaux de base
- [403] export CSV/PDF scope controle
- [404] controle performance sur rapports

## Phase 5 - Integrations inter-services

### Objectif

- brancher JayFestival et JayRDV sur contrats minimaux

### Taches

- [501] endpoint contractuel quote.create pour integrations
- [502] endpoint contractuel invoice.emit pour integrations
- [503] endpoint contractuel budget.movements.record edition
- [504] endpoint contractuel payment.record

### Criteres

- JayFestival integrable sans duplication logique comptable
- JayRDV integrable sans duplication logique comptable

## Phase 6 - Durcissement securite

### Objectif

- completer controle securite et federation COG

### Taches

- [601] enforcement classifications niveau 2 et 3
- [602] controles residence donnees selon policies
- [603] traces audit export et conversion devis->facture
- [604] tests refus erreurs contractuelles

## Phase 7 - Verification et recette

### Objectif

- valider qualite, regressions et conformite

### Taches

- [701] tests unitaires coeur metier
- [702] tests integration JayFestival et JayRDV
- [703] verification criteres CF-01 a CF-06
- [704] rapport de verification final

## Backlog phase 2 et 3

- OCR justificatifs
- rapprochement bancaire avance
- AP complet et echeanciers fournisseurs
- cash flow forecasting avance
- multi-entite consolidation avancee

## Checklist de livraison

- contrats CK-* publies
- operateurs Purse et Account actifs
- toolkits coeur actifs
- integrations minimales actives
- audit actif
- tests passes

## References

- docs/services/JayKonta/JayKonta - Bornage Implementation.md
- docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md
- docs/services/JayKonta/JayKonta - Documentation Enrichie.md
- docs/services/JayKonta/reference/JayKonta - Integration Services.md

## Statut

- Version : 1.0
- Date : 2026-02-07
- Statut : Plan implementation reference
