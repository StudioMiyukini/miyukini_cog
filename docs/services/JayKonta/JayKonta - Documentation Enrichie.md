# JayKonta - Documentation Enrichie

## Contexte

Ce document consolide la documentation enrichie du service JayKonta en s'appuyant sur :
- le document fondateur
- les references de securite, points d'entree, integration
- les analyses publics Purse et Account
- l'analyse PR concurrence web

JayKonta est un service COG unique avec deux points d'entree :
- JayBudget (Purse) pour le budget personnel et occasionnel
- JayKonta (Account) pour la comptabilite entreprise, devis et facturation

## Portee

- Perimetre : vision service, architecture fonctionnelle, modeles de capacites, gouvernance, securite, cadrage implementation
- Hors perimetre : details code crate par crate, schema SQL final, specifics UI pixel-perfect

## Objectifs produit

- Unifier budget personnel et comptabilite entreprise dans un meme socle gouverne
- Eviter la duplication de logique comptable dans les services consommateurs
- Garantir traçabilite, audit et residence des donnees sensibles
- Permettre des integrations robustes avec JayFestival, JayRDV et JayKoa

## Positionnement concurrence (synthese PR)

Le benchmark Odoo, QuickBooks, Xero, Zoho Books, FreshBooks, Sage, ERPNext, Akaunting et outils Purse (YNAB, Monarch, PocketGuard, Simplifi) confirme des attentes minimales :
- cycle complet devis vers facture vers paiement
- rapprochement bancaire et gestion des statuts de paiement
- tableaux de bord financiers et exports
- workflows mobiles pour la saisie rapide
- portail client ou partage externe controle

Differenciateur COG JayKonta :
- souverainete des donnees par environnement
- gouvernance explicite (StrongFather, Master Butler, WorrySentinel, KindMother)
- federation inter-COG sans fusion de gouvernance

## Architecture de service

### Domaines fonctionnels

- Domaine A : Accounting Core
- Domaine B : Billing and Collections
- Domaine C : Budget and Planning
- Domaine D : Reporting and Export
- Domaine E : Access, Security and Audit
- Domaine F : Inter-service Integrations

### Entites metier principales

- AccountProfile
- LedgerEntry
- BudgetEnvelope
- Quote
- Invoice
- PaymentRecord
- Counterparty
- ReportJob
- ExportArtifact
- AuditEvent
- IntegrationEvent

### Capacites transverses

- Classification securite par niveau WorrySentinel
- Residence des donnees via KindMother
- Mandats et permissions via StrongFather et Master Butler
- Journalisation forte des actions sensibles

## Catalogue capacites par point d'entree

### Purse (JayBudget)

- gestion mouvements personnels
- categories budget
- budgets occasionnels
- objectifs et alertes
- syntheses et exports personnels

### Account (JayKonta)

- grand livre et journal
- devis et conversion en facture
- emission facture et relances
- suivi encaissements
- rapports legaux et exports expert-comptable
- budget par projet / edition

## Gouvernance COG appliquee

### Regles de decision

- toute action d'ecriture est soumise a mandat
- les permissions sont explicites et revocables
- aucune autorite implicite depuis les services consommateurs

### Regles de persistance

- donnees niveau 2+ : residence selon contrat de service
- donnees niveau 3 : residence centralisee obligatoire
- write intents traces et horodates

### Regles de federation

- identite ne vaut pas autorite
- acces externe par visa de connexion
- bridge inter-COG transporte les intentions sans pouvoir decisionnel

## Integrations service a service

### JayFestival vers JayKonta

- budget edition via mouvements classes
- devis exposants
- factures exposants
- reporting budget edition

### JayRDV vers JayKonta

- devis prestations
- factures prestations
- suivi encaissements
- reporting revenus par periode

### JayKoa (optionnel)

- rappels d'echeances
- jalons temporels sur objectifs et paiements
- aucune copie canonique des donnees comptables

## Exigences non fonctionnelles

- securite : niveau 2 minimum Purse, niveau 2 a 3 Account
- audit : traçabilite complete des lectures critiques et ecritures
- performance : latence cible < 3 s sur ecrans principaux
- disponibilite : mode degrade en cas d'integrations indisponibles
- interoperabilite : formats export PDF/CSV standard

## UX fonctionnelle cible

### Principes

- parcours courts et clairs
- progression visible des statuts (devis, facture, paiement)
- feedback explicite sur les restrictions de securite
- affichage de contexte (point d'entree Purse vs Account)

### Ecrans coeur Purse

- tableau de bord
- mouvements
- budgets occasionnels
- objectifs
- alertes
- exports

### Ecrans coeur Account

- dashboard finance
- grand livre
- devis
- factures
- encaissements
- rapports
- exports

## Matrice besoins vers blocs fonctionnels

| Bloc | Purse | Account | Integrations |
|------|-------|---------|--------------|
| Auth et session | Oui | Oui | Oui |
| Permissions et roles | Basique | Avance | Oui |
| Mouvements | Oui | Oui | Oui |
| Devis | Non | Oui | Oui |
| Facturation | Non | Oui | Oui |
| Paiements | Non | Oui | Oui |
| Budgets occasionnels | Oui | Optionnel projet | Optionnel |
| Rapports | Oui | Oui | Oui |
| Export | Oui | Oui | Oui |
| Audit | Oui | Oui | Oui |

## Risques et mitigation

- Risque : sur-charge fonctionnelle Account
- Mitigation : phases de bornage strictes

- Risque : confusion Purse versus Account
- Mitigation : separation UX nette et mandats distincts

- Risque : incoherence entre services consommateurs
- Mitigation : contrats d'integration normatifs

- Risque : fuite de donnees sensibles
- Mitigation : classification niveau, audit, residence, chiffrement

## Livrables documentation enrichie

- JayKonta - Contrats Service Operateurs et Toolkits.md
- JayKonta - Bornage Implementation.md
- JayKonta - Plan Implementation.md
- references JayKonta existantes (securite, points d'entree, integration)
- publics Purse et Account (analyse, parcours, operateurs)

## References

- docs/services/JayKonta/JayKonta - Document Fondateur.md
- docs/services/JayKonta/reference/JayKonta - Niveaux Securite et Protection Donnees.md
- docs/services/JayKonta/reference/JayKonta - Points Entree JayBudget et JayKonta.md
- docs/services/JayKonta/reference/JayKonta - Integration Services.md
- docs/services/JayKonta/publics/Purse/Purse - Analyse des besoins.md
- docs/services/JayKonta/publics/Account/Account - Analyse des besoins.md
- docs/services/JayKonta/JayKonta - Analyse PR Concurrence Web.md

## Statut

- Version : 1.0
- Date : 2026-02-07
- Statut : Reference enrichie
