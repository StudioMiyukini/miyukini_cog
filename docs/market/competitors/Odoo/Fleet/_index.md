# Odoo Fleet — Index de Documentation

## Contexte

Ce dossier contient l'**analyse complète** de l'application **Fleet** (Flotte véhicules) d'Odoo, réalisée selon la méthodologie standardisée. L'analyse couvre la logique métier, les parcours utilisateur, l'UI/UX, les intégrations, les spécifications Opérateurs Miyukini, l'intégration COG et les guides d'implémentation.

**Date d'analyse :** 2026-02-01  
**Source :** Documentation Odoo 19.0

---

## Structure de Documentation

### 00_logique_metier/
- **[Odoo Fleet - Logique Métier Complète](./00_logique_metier/Odoo%20Fleet%20-%20Logique%20Metier%20Complete.md)**
  - Modèles de données (fabricants, modèles, catégories, véhicules, contrats, services, odomètre)
  - Règles métier et contraintes (conducteur, alertes fin de contrat, coûts)
  - Workflows (ajout véhicule, contrats, services, sinistres, demande véhicule Belgique)
  - Analyse des coûts (total, par véhicule, par conducteur)
  - Gestion des documents (assurance, garantie, entretien)

### 01_parcours_utilisateur/
- **[Odoo Fleet - Parcours Utilisateur Détaillés](./01_parcours_utilisateur/Odoo%20Fleet%20-%20Parcours%20Utilisateur%20Detailles.md)**
  - Personas et rôles (Fleet Manager, Admin, Conducteur, Employé demandeur, Responsable contrat)
  - Parcours : configuration, ajout véhicule, contrats, services, sinistres
  - Scénarios : demande de véhicule (Belgique), alertes fin de contrat, analyse des coûts
  - Points de friction identifiés
  - Recommandations pour Miyukini

### 02_ui_ux/
- **[Odoo Fleet - Analyse UI/UX](./02_ui_ux/Odoo%20Fleet%20-%20Analyse%20UI%20UX.md)**
  - Structure de navigation et menus
  - Vues Fabricants, Modèles, Catégories, Véhicules
  - Formulaire véhicule (onglets Tax Info, Contract, Model, Note)
  - Services et sinistres (liste, formulaire, Kanban)
  - Contrats et alertes
  - Rapports et analyse des coûts
  - Configuration (Settings)

### 03_integrations/
- **[Odoo Fleet - Intégrations Cross-App](./03_integrations/Odoo%20Fleet%20-%20Integrations%20Cross%20App.md)**
  - Dépendances avec autres modules Odoo (Employees, Payroll, Accounting, Purchase, Inventory, Contacts)
  - Flux de données inter-apps
  - Mécanismes d'intégration (conducteur, coûts, demande véhicule)
  - Recommandations pour Miyukini

### 04_specifications_miyukini/
- **[Odoo Fleet - Spécifications Opérateurs Miyukini](./04_specifications_miyukini/Odoo%20Fleet%20-%20Specifications%20Operateurs%20Miyukini.md)**
  - Architecture Opérateurs (7 Opérateurs identifiés)
  - Équipe d'Opérateurs FleetService
  - Contrat d'Équipe
  - Mandats de Permission (Standard, Fleet Manager, Validation demande, Conducteur)
  - Niveaux de sécurité (1–3 selon données)
  - Intégration avec les Cores

### 05_integration_cog/
- **[Odoo Fleet - Guide Intégration COG](./05_integration_cog/Odoo%20Fleet%20-%20Guide%20Integration%20COG.md)**
  - Architecture d'intégration COG
  - Patterns WriteIntent et Mandates (création véhicule, contrat, service, demande véhicule)
  - Exemples de code (pseudo-code Rust)
  - Gestion des erreurs et rollback
  - Intégration avec Kits existants (MiyuContacts, MiyuHR, MiyuNotify)

### 06_guides_implementation/
- **[Odoo Fleet - Guide Implémentation](./06_guides_implementation/Odoo%20Fleet%20-%20Guide%20Implementation.md)**
  - Architecture technique détaillée (crates miyufleet, miyufleet_contract, miyufleet_service, miyufleet_cost, miyufleet_request, miyufleet_ui)
  - Schémas de données (Vehicle, Model, Contract, Service, Odometer, VehicleRequest)
  - API et contrats
  - Plan de développement par phases (MVP → Complet)
  - Bornage fonctionnel
  - Critères d'acceptation et risques

---

## Résumé Exécutif

### Fonctionnalités Principales Identifiées

1. **Véhicules**
   - Création/modification (modèle, immatriculation, VIN, conducteur)
   - Assignation conducteur (res.partner ; lien optionnel employé)
   - Onglets Fiscality, Contract, Model, Note ; documents (assurance, garantie)

2. **Référentiels**
   - Fabricants (préchargés + création) ; Modèles (création obligatoire) ; Catégories ; Types de service (Vidange, Révision, Accident)

3. **Contrats**
   - Assurance, leasing : type, dates, montant, responsable
   - Alertes fin de contrat (paramètre jours ; email au responsable)

4. **Services et sinistres**
   - Entretiens, réparations : type, date, coût, fournisseur, odomètre, description, notes
   - Sinistres : services avec type Accident (description, notes) ; plusieurs réparations avec mêmes notes pour lier au même sinistre
   - Relevés odomètre (véhicule, conducteur, valeur, date)

5. **Analyse des coûts**
   - Total, par véhicule, par conducteur, comparaison détaillée ; export (pivot, CSV)
   - Intégration Accounting (export / écritures selon version)

6. **Demande de véhicule (Belgique)**
   - Modèles éligibles (« Can be requested ») ; limites parc ; validation RH / Fleet Manager ; attribution véhicule et liaison conducteur (employé)

### Architecture Miyukini Proposée

**7 Opérateurs :**
- FleetVehicleOperator (véhicules)
- FleetModelOperator (fabricants, modèles, catégories, types de service)
- FleetContractOperator (contrats, alertes)
- FleetServiceOperator (services, odomètre)
- FleetCostOperator (coûts, rapports, export)
- FleetRequestOperator (demande véhicule)
- FleetUI (interface)

**1 Équipe d'Opérateurs :** FleetService

**Correspondance Miyukini :** **MiyuFleet** (ou **MiyukiniFleet**) — FleetService

**Niveaux de sécurité :** 1–3 selon action et données (Standard à Critical)

**Intégration Cores :**
- StrongFather : Décisions (création/modification véhicule, contrat, service ; validation demande véhicule)
- KindMother : Persistance (WriteIntent)
- Master Butler : Permissions (Fleet Manager, Admin, Conducteur, Responsable contrat)
- WorrySentinel : Sécurité (données véhicules, contrats, services, demandes)
- Caring Nanny : État contrats à échéance ; prochains entretiens (si planification)
- Ever Buddy : Cycle de vie (véhicules, contrats, services, modèles)
- TAMR : Validation humaine (RH / Fleet Manager) pour demande véhicule
- BondingBrother : Médiation entre FleetUI et les Opérateurs Fleet

---

## Statut de l'Analyse

| Document | Statut | Version |
|----------|--------|---------|
| Logique Métier | ✅ Complété | 1.0 |
| Parcours Utilisateur | ✅ Complété | 1.0 |
| UI/UX | ✅ Complété | 1.0 |
| Intégrations Cross-App | ✅ Complété | 1.0 |
| Spécifications Opérateurs Miyukini | ✅ Complété | 1.0 |
| Guide Intégration COG | ✅ Complété | 1.0 |
| Guide Implémentation | ✅ Complété | 1.0 |

---

## Prochaines Étapes

1. **Valider les spécifications** : Revue avec équipe technique
2. **Démarrer l'implémentation** : Phase 1 (MVP) selon guide
3. **Itérer** : Selon feedback et besoins utilisateurs

---

**Document** : Odoo Fleet — Index de Documentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : ✅ Analyse complète à 100% — référence pour implémentation Miyukini (MiyuFleet)
