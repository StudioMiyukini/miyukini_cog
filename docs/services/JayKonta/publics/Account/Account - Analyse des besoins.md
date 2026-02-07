# JayKonta - Analyse des besoins (point d'entree Account)

## Contexte

Ce document formalise l'analyse des besoins du point d'entree Account du service COG JayKonta.
Le perimetre couvre la comptabilite entreprise, le cycle devis vers facture vers paiement, les rapports et les integrations metier.

References principales :
- `docs/services/JayKonta/JayKonta - Document Fondateur.md`
- `docs/services/JayKonta/JayKonta - Contrats Service Operateurs et Toolkits.md`
- `docs/services/JayKonta/reference/JayKonta - Integration Services.md`
- `docs/services/JayKonta/reference/JayKonta - Niveaux Securite et Protection Donnees.md`

## Portee

- Public : professionnels, associations, TPE/PME, organisateurs
- In scope : compte Account, roles, grand livre, devis, factures, paiements, rapports, export, integrations JayFestival/JayRDV
- Out of scope : budget personnel Purse, implementation technique detaillee API/DB

## Objectifs metier

- O1 : tenir une comptabilite exploitable sans double saisie
- O2 : executer le cycle commercial devis vers facture vers encaissement
- O3 : produire des rapports legaux et de pilotage
- O4 : supporter les integrations JayFestival et JayRDV sous contrat
- O5 : garantir securite, residence et audit de niveau 2 a 3

## Personas et attentes

| Persona | Attente principale | Risque de non couverture |
|---------|--------------------|--------------------------|
| TPE / Independant | Facturer vite et suivre les impayes | abandon outil, tableur externe |
| Association | Budget par projet/edition + facturation simple | compta dispersee |
| Organisateur evenement | Budget edition, devis/factures exposants | duplication entre outils |
| Professionnel RDV | Facturation prestation et suivi paiements | erreur de reconciliation |
| Comptable interne | Journal fiable, export, audit | non conformite, perte de trace |

## Besoins fonctionnels

### B1. Acces et gouvernance

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| MAC-01 | Creation/connexion compte Account | CK-SVC-01, CK-TK-01 |
| MAC-02 | Roles et permissions (admin/comptable/lecture) | CK-SVC-02, CK-SEC-02 |
| MAC-03 | Contexte Account explicite et isole | CK-SVC-02 |

### B2. Comptabilite coeur

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| MAC-04 | Enregistrer mouvements (revenu/depense) | CK-OP-11, CK-TK-11 |
| MAC-05 | Ventiler par categorie/projet/edition | CK-TK-11 |
| MAC-06 | Consulter journal et grand livre | CK-TK-11 |

### B3. Cycle devis

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| MAC-07 | Creer devis conforme | CK-OP-11, CK-TK-21 |
| MAC-08 | Envoyer et suivre statut devis | CK-TK-21 |
| MAC-09 | Convertir devis vers facture | CK-OP-12, CK-TK-21 |
| MAC-10 | Exposer quote.create aux integrations | CK-INT-01, CK-INT-02 |

### B4. Cycle facturation et paiement

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| MAC-11 | Emettre facture | CK-OP-13, CK-TK-31 |
| MAC-12 | Relancer impayes | CK-TK-31 |
| MAC-13 | Enregistrer encaissements | CK-OP-14, CK-TK-41 |
| MAC-14 | Exposer invoice.emit aux integrations | CK-INT-01, CK-INT-02 |

### B5. Reporting et export

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| MAC-15 | Dashboard financier | CK-TK-51 |
| MAC-16 | Rapports legaux | CK-OP-15, CK-TK-51 |
| MAC-17 | Exports controle´s (PDF/CSV) | CK-TK-51, CK-AUD-02 |

### B6. Integration evenementielle

| ID | Besoin | Contrat associe |
|----|--------|-----------------|
| MAC-18 | Budget par edition (JayFestival) | CK-INT-01 |
| MAC-19 | Restitution budget organisateur | CK-INT-01 |

## Besoins non fonctionnels

| ID | Exigence | Cible |
|----|----------|-------|
| NFR-MAC-01 | Classification securite | Niveau 2 minimum, 3 sur paiements et pieces critiques |
| NFR-MAC-02 | Residence donnees | Centralisee pour classes 3 |
| NFR-MAC-03 | Secrets paiement | Pas de stockage en clair |
| NFR-MAC-04 | Audit | Journal complet des ecritures et exports |
| NFR-MAC-05 | Performance ecran dashboard | < 3 s mediane |
| NFR-MAC-06 | Emission facture | < 3 s mediane |
| NFR-MAC-07 | Disponibilite degradee | lecture possible si integration externe indisponible |
| NFR-MAC-08 | Conformite export | scope minimal et traçabilite |

## Priorisation

| Priorite | Besoins |
|----------|---------|
| P0 | MAC-01 a MAC-06, MAC-11 a MAC-13 |
| P1 | MAC-07 a MAC-10 |
| P2 | MAC-14 a MAC-17 |
| P3 | MAC-18 a MAC-19 |

## Criteres d'acceptation globaux

- CA-1 : un compte Account peut emettre un devis et le convertir en facture
- CA-2 : une facture peut passer en payee/partielle avec trace audit
- CA-3 : journal et grand livre sont coherents sur une periode
- CA-4 : export PDF/CSV respecte le scope autorise
- CA-5 : appels JayFestival/JayRDV passent par les contrats CK-INT

## Definition of done (document)

- besoins listes et traces vers contrats
- priorisation explicite
- criteres mesurables presents
- dependances et risques identifies

## Risques et mitigations

- R1 : confusion Purse/Account
- mitigation : separation de contexte et permissions strictes

- R2 : surcharge phase 1
- mitigation : limiter P0/P1, reporter enrichissements phase 2

- R3 : regressions integration
- mitigation : tests de contrats CK-INT automatiques

## References complementaires

- `docs/services/JayKonta/JayKonta - Bornage Implementation.md`
- `docs/services/JayKonta/JayKonta - Plan Implementation.md`
- `docs/services/JayKonta/publics/Account/Account - Operateurs et Toolkits.md`
- `docs/services/JayKonta/publics/Account/Account - Parcours Capacites Livrables.md`

## Statut

- Version : 2.0
- Date : 2026-02-07
- Statut : Analyse enrichie
