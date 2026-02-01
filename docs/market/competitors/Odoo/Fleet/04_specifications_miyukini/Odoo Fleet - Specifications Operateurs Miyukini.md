# Odoo Fleet — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application **Fleet** (Flotte véhicules) d'Odoo.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Fleet
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores

---

## 1. Architecture Opérateurs

### 1.1 Vue d'ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **FleetVehicleOperator** | Gestion des véhicules (création, édition, assignation conducteur, documents) | Opérateur de Service |
| **FleetModelOperator** | Gestion des fabricants, modèles et catégories (référentiels) | Opérateur de Domaine |
| **FleetContractOperator** | Gestion des contrats (assurance, leasing), alertes fin de contrat | Opérateur de Service |
| **FleetServiceOperator** | Services (entretiens, réparations), sinistres, relevés odomètre | Opérateur de Service |
| **FleetCostOperator** | Analyse des coûts (par véhicule, conducteur, période), export | Opérateur de Domaine |
| **FleetRequestOperator** | Demande de véhicule (éligibilité, validation, attribution) | Opérateur de Service |
| **FleetUI** | Interface utilisateur Fleet | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : FleetService

**Définition :**
> **FleetService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion de flotte (véhicules, modèles, contrats, services, sinistres, coûts, demande de véhicule).**

**Composition :**
- FleetVehicleOperator (niveau sécurité 2)
- FleetModelOperator (niveau sécurité 1–2)
- FleetContractOperator (niveau sécurité 2)
- FleetServiceOperator (niveau sécurité 2)
- FleetCostOperator (niveau sécurité 2)
- FleetRequestOperator (niveau sécurité 2–3 selon données employé)
- FleetUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 FleetVehicleOperator

**Rôle :** Gestion des véhicules (création, édition, assignation conducteur, documents, archivage).

**Capacités :**
- Création / modification de véhicules (modèle, immatriculation, VIN, société, conducteur)
- Assignation conducteur (res.partner ; lien optionnel employé)
- Onglets Fiscality, Contract, Model, Note ; pièces jointes (assurance, garantie, carnets)
- Archivage (active = false)
- Exposition des véhicules pour Services, Contrats, Odometer, Coûts

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décisions (création, modification, archivage) ; validation assignation conducteur
- **KindMother** : Persistance des véhicules (WriteIntent)
- **Master Butler** : Permissions (Fleet Manager, Admin, Conducteur en lecture sur ses véhicules)
- **WorrySentinel** : Niveau sécurité données véhicule ; isolation multi-société
- **Ever Buddy** : Cycle de vie (actif / archivé)

**Contrat d'équipe :**
- Consomme : FleetModelOperator (modèles), MiyuContacts (conducteur, fournisseurs), FleetContractOperator (contrat actif)
- Expose : `vehicle.create`, `vehicle.update`, `vehicle.archive`, `vehicle.assign_driver`, `vehicle.list`, `vehicle.get_by_id`

**Mandat de Permission requis :**
- Création véhicule : Mandat avec KindMother (WriteIntent) + Master Butler (vehicle.create)
- Modification véhicule : Mandat avec KindMother (WriteIntent) + Master Butler (vehicle.update)
- Assignation conducteur : Mandat avec Master Butler (vehicle.assign_driver)

### 2.2 FleetModelOperator

**Rôle :** Gestion des référentiels (fabricants, modèles, catégories de modèles).

**Capacités :**
- Fabricants (création, édition, logo) ; modèles (création, édition : informations, moteur, fiscalité si localisation, fournisseurs)
- Catégories de modèles (création, ordre) ; types véhicule car/bike (fixes)
- Types de service (Vidange, Révision, Accident - Faute / Sans faute, etc.)
- Exposition des modèles et catégories pour FleetVehicleOperator et FleetServiceOperator

**Niveau de sécurité :** 1–2 (Standard à Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification référentiels
- **KindMother** : Persistance (WriteIntent)
- **Master Butler** : Permissions (Fleet Manager, Admin)
- **WorrySentinel** : Isolation multi-société
- **Ever Buddy** : Cycle de vie (actif / inactif) pour modèles et types

**Contrat d'équipe :**
- Consommé par : FleetVehicleOperator (model_id), FleetServiceOperator (service_type_id)
- Expose : `model.create`, `model.update`, `model.list`, `brand.list`, `category.list`, `service_type.list`

### 2.3 FleetContractOperator

**Rôle :** Gestion des contrats véhicule (assurance, leasing), alertes fin de contrat.

**Capacités :**
- Création / modification / clôture de contrats (type, véhicule, dates, montant, responsable)
- Paramètre « jours avant échéance » pour alerte (global ou par type selon évolution)
- Notification au responsable (email + in-app via MiyuNotify)
- Exposition des contrats actifs et à échéance pour FleetVehicleOperator et FleetCostOperator

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification/clôture contrat
- **KindMother** : Persistance (WriteIntent)
- **Master Butler** : Permissions (Fleet Manager, Admin, Responsable lecture sur ses contrats)
- **WorrySentinel** : Données contrat (montants, responsable) ; isolation multi-société
- **Ever Buddy** : Cycle de vie (actif / expiré / clôturé)
- **Caring Nanny** : État « contrats à échéance » pour alertes

**Contrat d'équipe :**
- Consomme : FleetVehicleOperator (véhicule), MiyuContacts (responsible = user), MiyuNotify (alertes)
- Expose : `contract.create`, `contract.update`, `contract.close`, `contract.list_expiring`, `contract.get_by_vehicle`

**Mandat de Permission requis :**
- Création/modification contrat : Mandat avec KindMother (WriteIntent) + Master Butler (contract.create/update)
- Clôture : Mandat avec StrongFather (décision) + KindMother (WriteIntent)

### 2.4 FleetServiceOperator

**Rôle :** Services (entretiens, réparations), sinistres (Accident), relevés odomètre.

**Capacités :**
- Création / modification de services (véhicule, type, date, coût, fournisseur, conducteur, odomètre, description, notes)
- Sinistres : services avec type Accident (description, notes pour détails) ; liaison plusieurs réparations par mêmes notes
- Relevés odomètre (véhicule, conducteur, valeur, date)
- Workflow services : stades (planifié, en cours, terminé) — Kanban
- Exposition des services et odomètre pour FleetCostOperator et rapports

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision création/modification service
- **KindMother** : Persistance (WriteIntent)
- **Master Butler** : Permissions (Fleet Manager, Admin, Conducteur possible en saisie odomètre / déclaration sinistre selon droits)
- **WorrySentinel** : Données coûts, fournisseur ; isolation multi-société
- **Ever Buddy** : Cycle de vie (états service)
- **Caring Nanny** : État « prochains entretiens » si planification

**Contrat d'équipe :**
- Consomme : FleetVehicleOperator (véhicule), FleetModelOperator (service_type), MiyuContacts (vendor, driver)
- Expose : `service.create`, `service.update`, `service.list`, `odometer.log`, `service.list_by_vehicle`, `service.list_accidents`

**Mandat de Permission requis :**
- Création service : Mandat avec KindMother (WriteIntent) + Master Butler (service.create)
- Saisie odomètre : Mandat avec Master Butler (odometer.log) — possible pour conducteur sur ses véhicules

### 2.5 FleetCostOperator

**Rôle :** Analyse des coûts (total, par véhicule, par conducteur, par période), export et rapports.

**Capacités :**
- Agrégation coûts contrats + services sur période
- Coût par véhicule ; coût par conducteur
- Comparaison détaillée ; export (pivot, CSV)
- Exposition des données pour module Comptabilité / Analytique (Miyukini) si intégration

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Pas de décision directe ; lecture agrégée
- **KindMother** : Lecture des données (contrats, services) via Mandat
- **Master Butler** : Permissions (Fleet Manager, Admin pour rapports complets ; Conducteur éventuellement limité à ses véhicules)
- **WorrySentinel** : Données agrégées sensibles ; isolation multi-société

**Contrat d'équipe :**
- Consomme : FleetContractOperator (contrats), FleetServiceOperator (services), FleetVehicleOperator (véhicules)
- Expose : `cost.total`, `cost.by_vehicle`, `cost.by_driver`, `cost.detailed_comparison`, `cost.export`

**Mandat de Permission requis :**
- Lecture rapports coûts : Mandat avec Master Butler (cost.read) ; périmètre selon rôle (tous véhicules vs mes véhicules)

### 2.6 FleetRequestOperator

**Rôle :** Demande de véhicule (employé demandeur) : éligibilité des modèles, limites parc, validation, attribution.

**Capacités :**
- Liste des modèles éligibles (« Can be requested »)
- Création demande (employé, modèle) ; validation (RH / Fleet Manager) ; attribution véhicule et liaison conducteur (employé)
- Limites selon disponibilité du parc (paramètre Configuration)
- Exposition du statut demande (en attente, acceptée, refusée)

**Niveau de sécurité :** 2–3 (Sensitive à Critical selon données employé)

**Gouvernance :**
- **StrongFather** : Décision validation (accepter / refuser) ; décision attribution véhicule
- **KindMother** : Persistance des demandes (WriteIntent) ; mise à jour véhicule (assignation conducteur)
- **Master Butler** : Permissions (demandeur : create request, read own ; validateur : validate, assign)
- **WorrySentinel** : Données employé et parc ; isolation multi-société
- **TAMR** : Validation humaine (RH / Fleet Manager) pour accepter ou refuser

**Contrat d'équipe :**
- Consomme : FleetVehicleOperator (attribution), FleetModelOperator (modèles éligibles), MiyuHR (employé demandeur)
- Expose : `request.create`, `request.validate`, `request.reject`, `request.assign_vehicle`, `request.list_eligible_models`, `request.list_my_requests`

**Mandat de Permission requis :**
- Création demande : Mandat avec KindMother (WriteIntent) + Master Butler (request.create)
- Validation : Mandat avec StrongFather (décision) + KindMother (WriteIntent) + Master Butler (request.validate)
- Attribution : Mandat avec FleetVehicleOperator (assign_driver) + KindMother (WriteIntent)

### 2.7 FleetUI

**Rôle :** Interface utilisateur Fleet (tableaux de bord, listes, formulaires, rapports).

**Capacités :**
- Navigation : Véhicules, Services, Contrats, Rapports, Configuration (Fabricants, Modèles, Catégories, Types de service)
- Formulaire véhicule (onglets Tax Info, Contract, Model, Note) ; formulaire modèle ; formulaire service ; formulaire contrat
- Listes et Kanban (véhicules, services par stade) ; filtres et recherche
- Rapports coûts (Total, Par véhicule, Par conducteur, Comparaison détaillée) ; export
- Configuration : Settings (alertes, demande véhicule) ; création rapide conducteur, fournisseur
- Tableau de bord Fleet Manager : véhicules actifs, contrats à échéance, prochains entretiens, coûts du mois (recommandation Miyukini)

**Niveau de sécurité :** 1 (Standard) pour affichage ; délégation des écritures aux Opérateurs avec Mandat

**Gouvernance :**
- **Master Butler** : Permissions (affichage selon rôle : Admin, Fleet Manager, Conducteur « Mes véhicules »)
- **BondingBrother** : Médiation entre intention utilisateur et FleetVehicleOperator, FleetContractOperator, FleetServiceOperator, FleetCostOperator, FleetRequestOperator, FleetModelOperator

**Contrat d'équipe :**
- Consomme : Tous les Opérateurs Fleet (lecture et actions via Mandats)
- Expose : Pages et composants UI ; pas d’API métier directe (tout passe par BondingBrother)

---

## 3. Contrat d'Équipe FleetService

### 3.1 Flux autorisés

- FleetUI → BondingBrother → FleetVehicleOperator (CRUD véhicules, assignation conducteur)
- FleetUI → BondingBrother → FleetModelOperator (CRUD fabricants, modèles, catégories, types de service)
- FleetUI → BondingBrother → FleetContractOperator (CRUD contrats, alertes)
- FleetUI → BondingBrother → FleetServiceOperator (CRUD services, odomètre)
- FleetUI → BondingBrother → FleetCostOperator (lecture coûts, export)
- FleetUI → BondingBrother → FleetRequestOperator (demande, validation, attribution)
- FleetContractOperator → MiyuNotify (envoi alertes fin de contrat)
- FleetVehicleOperator → FleetModelOperator (lecture modèles) ; FleetVehicleOperator → MiyuContacts (conducteur)
- FleetServiceOperator → FleetVehicleOperator (lecture véhicule) ; FleetServiceOperator → MiyuContacts (vendor, driver)
- FleetCostOperator → FleetContractOperator, FleetServiceOperator, FleetVehicleOperator (lecture)
- FleetRequestOperator → FleetVehicleOperator (assignation), MiyuHR (employé)

### 3.2 Types de données échangeables

- Véhicule, Modèle, Fabricant, Catégorie, Type de service (référentiels)
- Contrat (type, dates, montant, responsable)
- Service (véhicule, type, date, coût, fournisseur, conducteur, odomètre, description, notes)
- Relevé odomètre (véhicule, conducteur, valeur, date)
- Coûts agrégés (total, par véhicule, par conducteur)
- Demande véhicule (employé, modèle, statut, véhicule attribué)

### 3.3 Niveau de sécurité maximum

- Standard (1) : Lecture listes publiques (modèles, fabricants)
- Sensitive (2) : Véhicules, contrats, services, coûts, demandes
- Critical (3) : Validation demande et attribution (données employé + parc)

### 3.4 Conditions de validité

- Mandat valide émis par StrongFather pour les actions d’écriture
- Permissions Master Butler conformes au rôle (Fleet Manager, Admin, Conducteur, Responsable contrat)
- WorrySentinel : périmètre société et niveau de sécurité respectés

---

## 4. Mandats de Permission

### 4.1 Standard (lecture / liste)

- **Usage** : Lecture des véhicules, contrats, services, modèles (selon périmètre : tous vs mes véhicules).
- **Contenu** : Flux lecture ; niveau sécurité max 2 ; opérateurs FleetCostOperator, FleetVehicleOperator (read), FleetContractOperator (read), FleetServiceOperator (read).

### 4.2 Fleet Manager (écriture véhicules, contrats, services)

- **Usage** : Création/modification véhicules, contrats, services ; gestion référentiels (modèles, types de service).
- **Contenu** : Flux create/update sur FleetVehicleOperator, FleetContractOperator, FleetServiceOperator, FleetModelOperator ; niveau sécurité max 2 ; durée selon session ou politique.

### 4.3 Validation demande véhicule

- **Usage** : RH / Fleet Manager valide ou refuse une demande ; attribution véhicule et liaison conducteur.
- **Contenu** : Flux request.validate, request.reject, request.assign_vehicle ; StrongFather (décision) ; KindMother (WriteIntent) ; niveau sécurité max 3 ; révocation à la fin de l’action.

### 4.4 Conducteur (saisie odomètre, déclaration sinistre)

- **Usage** : Conducteur saisit les relevés odomètre et/ou déclare un sinistre sur ses véhicules.
- **Contenu** : Flux odometer.log, service.create (limité à ses véhicules) ; niveau sécurité max 2 ; opérateurs FleetServiceOperator avec restriction périmètre.

---

## 5. Intégration avec les Cores

| Core | Rôle dans Fleet |
|------|------------------|
| **StrongFather** | Décisions création/modification véhicule, contrat, service ; validation et attribution demande véhicule |
| **KindMother** | Persistance véhicules, modèles, contrats, services, demandes (WriteIntent) |
| **Master Butler** | Permissions (Fleet Manager, Admin, Conducteur, Responsable contrat) ; capacités vehicle.*, contract.*, service.*, cost.*, request.* |
| **WorrySentinel** | Niveau sécurité données (Sensitive, Critical pour demande) ; isolation multi-société |
| **Caring Nanny** | État contrats à échéance ; état prochains entretiens (si planification) |
| **Ever Buddy** | Cycle de vie véhicules (actif/archivé), contrats (actif/expiré/clôturé), services (états), modèles et types |
| **TAMR** | Validation humaine (RH / Fleet Manager) pour accepter ou refuser demande véhicule |
| **Border Guard** | Frontières du service Fleet (pas d’exécution hors Mandat) |
| **BondingBrother** | Médiation entre FleetUI et les Opérateurs Fleet |

---

## 6. Correspondance Miyukini

**Service Miyukini équivalent :** **MiyuFleet** (ou **MiyukiniFleet**) — FleetService

**Équipe d'Opérateurs :** FleetService (7 Opérateurs)

**Niveaux de sécurité :** 1 (Standard) à 3 (Critical) selon action et données

**Intégration Cores :** StrongFather, KindMother, Master Butler, WorrySentinel, Caring Nanny, Ever Buddy, TAMR, Border Guard, BondingBrother

---

**Document** : Odoo Fleet — Spécifications Opérateurs Miyukini  
**Version** : 1.0  
**Date** : 2026-02-01
