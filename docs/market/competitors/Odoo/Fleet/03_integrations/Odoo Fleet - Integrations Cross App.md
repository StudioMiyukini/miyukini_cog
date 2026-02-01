# Odoo Fleet — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Fleet** (Flotte véhicules) d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 19.0 (Fleet, HR, Employees, Accounting, Purchase, Inventory, Payroll)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres modules Odoo (base, mail, hr, Employees, Accounting, Purchase, Inventory, Payroll)
- Flux de données inter-apps
- Mécanismes d'intégration (véhicule, conducteur, coûts)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Module Fleet (fleet) — base

**Contenu :**
- `fleet.vehicle.model.brand` : Fabricants
- `fleet.vehicle.model` : Modèles véhicules
- `fleet.vehicle.model.category` : Catégories modèles
- `fleet.vehicle` : Véhicules (parc)
- `fleet.vehicle.log.contract` : Contrats (assurance, leasing)
- `fleet.vehicle.log.services` : Services (entretiens, réparations, sinistres)
- `fleet.vehicle.odometer` : Relevés odomètre
- `fleet.service.type` : Types de service

**Dépendances typiques :**
- **base** : res.company, res.users, res.partner
- **mail** : Chatter, activités, notifications (optionnel sur véhicule, contrat, service)
- **hr** : Base HR (optionnel pour lien employé)

### 1.2 Modules consommateurs de Fleet

Les applications suivantes **consomment** ou **s’intègrent** avec Fleet :

| App | Usage principal |
|-----|------------------|
| **Employees (hr.employee)** | Conducteur = contact travail employé ; champ Fleet Mobility Card sur fiche employé |
| **Payroll** | Véhicule de fonction (avantage en nature) ; coût déprécié sur modèle (Belgique) ; demande de véhicule (New Vehicle Request) |
| **Accounting** | Coûts Fleet (contrats, services) pour export / écritures comptables ; prévisions |
| **Purchase** | Fournisseurs sur modèle véhicule ; création RFQ (demandes d’achat) pour véhicules |
| **Inventory** | Catégorie modèle : Max Weight, Max Volume pour capacité de chargement (livraisons, dispatch) |
| **Contacts (res.partner)** | Conducteur = res.partner ; fournisseurs = res.partner ; responsable contrat = res.users |

---

## 2. Flux de Données

### 2.1 Fleet comme source

```
fleet.vehicle (véhicule)
    ├── fleet.vehicle.model (model_id)
    │   ├── fleet.vehicle.model.brand (brand_id)
    │   ├── fleet.vehicle.model.category (categ_id)
    │   └── res.partner (vendor_ids — fournisseurs)
    ├── res.partner (driver_id — conducteur)
    ├── fleet.vehicle.log.contract (contrat actif)
    │   └── res.users (responsible_id)
    ├── fleet.vehicle.log.services (services / sinistres)
    │   ├── fleet.service.type (service_type_id)
    │   └── res.partner (vendor_id)
    └── fleet.vehicle.odometer (relevés)
```

**Flux sortants (Fleet → autres apps) :**
- **Employees** : Lien conducteur (driver_id = res.partner) ; si employé, Fleet Mobility Card (fleet.vehicle ou carte mobilité) sur fiche employé.
- **Payroll** : Modèle avec coût déprécié (Cost Depreciated, Total Cost Depreciated) pour configurateur paie ; demande de véhicule (Belgique) avec modèles « Can be requested ».
- **Accounting** : Coûts contrats et services (montants, véhicule, conducteur) pour écritures / analytique (selon version et modules).
- **Purchase** : Fournisseurs sur modèle (vendor_ids) pour création RFQ véhicules.
- **Inventory** : Max Weight, Max Volume sur catégorie (ou modèle) pour capacité chargement.

### 2.2 Fleet comme consommateur

**Flux entrants (autres apps → Fleet) :**
- **Contacts (res.partner)** : Conducteur (driver_id), fournisseurs (vendor_id sur service, vendor_ids sur modèle), responsable contrat (res.users lié à res.partner).
- **Employees (hr.employee)** : Lien conducteur ↔ employé via work_contact_id ou champ Fleet Mobility Card ; employé comme demandeur (New Vehicle Request).
- **Payroll** : Working Schedules, contrats employé ; modèles avec « Can be requested » et coût déprécié pour avantage en nature.
- **Configuration** : End Date Contract Alert, New Vehicle Request (Belgique) ; localisation pour champs fiscaux (CO2 fee, Tax Deduction, Salary tab).

---

## 3. Intégrations Détaillées

### 3.1 Employees (HR)

**Données partagées :**
- **Conducteur** : fleet.vehicle.driver_id = res.partner ; si employé, ce partenaire est souvent work_contact_id de hr.employee.
- **Fleet Mobility Card** : Sur la fiche employé (hr.employee), champ ou lien vers le véhicule / carte mobilité (fleet) pour identifier le véhicule de fonction de l’employé.
- **Demande de véhicule** : Employé comme demandeur ; validation par RH / Fleet manager ; attribution véhicule et liaison conducteur (employé).

**Règles :**
- Un véhicule a un seul conducteur (res.partner) ; l’employé est relié via son contact travail.
- Si Employees n’est pas installé, le conducteur reste un contact (res.partner) sans lien structuré à un employé.

### 3.2 Payroll

**Données partagées :**
- **Véhicule de fonction (avantage en nature)** : Modèle (fleet.vehicle.model) avec onglet Salary (Belgique) : Can be requested, Catalog Value (VAT Incl.), CO2 fee, Cost (Depreciated), Total Cost (Depreciated). Ces valeurs alimentent le configurateur de paie (salaire brut/net).
- **Dépréciation** : Ne se calcule pas sur le modèle seul mais sur le **contrat** lié au véhicule (fleet.vehicle.log.contract).
- **New Vehicle Request** (Belgique) : Limites sur les demandes selon disponibilité du parc ; modèles avec « Can be requested » = true ; workflow demande → validation → attribution.

**Règles :**
- Type véhicule (car / bike) fixe pour compatibilité Paie (avantage en nature).
- Coût déprécié : impact sur salaire selon localisation et règles fiscales.

### 3.3 Accounting

**Données partagées :**
- **Coûts Fleet** : Montants des contrats (fleet.vehicle.log.contract) et des services (fleet.vehicle.log.services) ; véhicule, conducteur, fournisseur, date.
- **Usage** : Export des coûts pour écritures comptables ; rapports analytiques (coût par véhicule, par conducteur, par période) ; prévisions.

**Règles :**
- Selon version et modules : lien direct (compte analytique, journal) ou export manuel (CSV, pivot) vers Accounting.
- Séparation coûts contrats vs services dans les rapports Fleet (Total costs, Cost by vehicle, Cost by driver).

### 3.4 Purchase

**Données partagées :**
- **Fournisseurs** : Sur le modèle véhicule (fleet.vehicle.model), onglet Vendors : vendor_ids (res.partner). Liste des fournisseurs auprès desquels le véhicule peut être acheté.
- **Usage** : Création de demandes d’achat (RFQ) pour véhicules depuis l’app Purchase ; sélection du modèle et du fournisseur.

**Règles :**
- Les fournisseurs sont des contacts (Contacts) ; ajout depuis le formulaire modèle (Add Vendors) ou création d’un nouveau contact.
- Pas de limite sur le nombre de fournisseurs par modèle.

### 3.5 Inventory

**Données partagées :**
- **Capacité de chargement** : Sur la catégorie de modèle (fleet.vehicle.model.category) ou sur le modèle : champs **Max Weight** et **Max Volume** (si module Inventory installé).
- **Usage** : Gestion des livraisons (dispatch) ; affichage de l’espace et du poids restants pour charger les produits dans le véhicule.

**Règles :**
- Optionnelle ; champs visibles uniquement si Inventory est installé.
- Unité : selon configuration (kg, m³, etc.).

### 3.6 Contacts (res.partner)

**Données partagées :**
- **Conducteur** : driver_id sur fleet.vehicle = res.partner. Création « Create a new driver » ouvre le formulaire contact.
- **Fournisseur** : vendor_id sur fleet.vehicle.log.services ; vendor_ids sur fleet.vehicle.model.
- **Responsable contrat** : responsible_id sur fleet.vehicle.log.contract = res.users (utilisateur Odoo) ; l’utilisateur est lié à un res.partner.

**Règles :**
- Conducteur et fournisseurs sont toujours des res.partner.
- Responsable contrat doit être un utilisateur (res.users) pour recevoir les emails d’alerte.

---

## 4. Schéma des Flux (Résumé)

```
                    ┌─────────────────┐
                    │   Contacts      │
                    │  (res.partner)  │
                    └────────┬────────┘
                             │ driver_id, vendor_id(s), responsible (user)
                             ▼
┌──────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Employees   │────▶│     Fleet      │◀───│   Accounting    │
│ (conducteur, │     │ (véhicules,    │     │ (coûts, export) │
│  Fleet Card, │     │  contrats,     │     └─────────────────┘
│  New Req.)   │     │  services)     │
└──────────────┘     └────────┬────────┘
                             │
              ┌──────────────┼──────────────┐
              ▼              ▼              ▼
     ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
     │  Payroll    │ │  Purchase   │ │  Inventory  │
     │ (avantage   │ │ (RFQ        │ │ (Max Weight │
     │  en nature, │ │  véhicules) │ │  Max Volume)│
     │  New Req.)  │ │             │ │             │
     └─────────────┘ └─────────────┘ └─────────────┘
```

---

## 5. Recommandations pour Miyukini

### 5.1 Intégrations à prévoir

- **MiyuHR (Employees)** : Lien conducteur ↔ employé (contact travail) ; champ « Véhicule de fonction » ou « Fleet Mobility Card » sur fiche employé ; flux « Demande de véhicule » (éligibilité, validation, attribution).
- **MiyuContacts** : Conducteur, fournisseurs, responsable contrat (utilisateur) ; création rapide contact depuis Fleet.
- **Comptabilité / Analytique** : Export des coûts Fleet (contrats + services) par véhicule, conducteur, période ; écritures analytiques ou export pivot/CSV ; prévisions.
- **Achat (Purchase)** : Fournisseurs sur modèle véhicule ; création de demandes d’achat (RFQ) pour véhicules depuis le module Achat.
- **Inventory (livraisons)** : Capacité de chargement (poids, volume) sur catégorie ou modèle pour dispatch et chargement.
- **Paie (si avantage en nature)** : Modèle avec coût déprécié et « Can be requested » ; dépréciation basée sur le contrat véhicule ; workflow demande de véhicule.

### 5.2 Contrats d’équipe (Miyukini)

- **FleetService** consomme : MiyuContacts (conducteur, fournisseurs, responsable), MiyuHR (employé conducteur, demande véhicule), MiyuNotify (alertes fin de contrat).
- **FleetService** expose : véhicules, contrats, services, coûts (par véhicule, conducteur, période) ; API pour Accounting et Purchase si besoin.
- **Mandats** : Création/modification véhicule, contrat, service selon permissions (Fleet Manager, Admin) ; lecture « Mes véhicules » pour conducteur ; validation demande véhicule (RH / Fleet).

### 5.3 Flux de données

- **Conducteur** : Toujours un MiyuContacts (partner) ; lien optionnel vers MiyuHR (employee) pour rapport coût par employé et demande de véhicule.
- **Alertes** : Paramètre « jours avant échéance » par type de contrat (évolution) ; responsable = utilisateur ; notification email + in-app (MiyuNotify).
- **Coûts** : Agrégation contrats + services ; export vers module Comptabilité / Analytique (Miyukini) pour ventilation automatique par véhicule, conducteur, compte.

---

**Document** : Odoo Fleet — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01
