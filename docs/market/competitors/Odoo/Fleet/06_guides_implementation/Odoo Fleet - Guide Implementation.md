# Odoo Fleet — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Fleet (Flotte véhicules) dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates)
- Spécifications des crates Rust
- Schémas de données (Vehicle, Model, Contract, Service, Request)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyufleet/                              # FleetVehicleOperator + FleetModelOperator (référentiels)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── vehicle.rs                       # Modèle Vehicle, états actif/archivé
│   │   ├── model.rs                        # Modèle Model, Brand, Category
│   │   ├── service_type.rs                 # Types de service (Vidange, Accident, etc.)
│   │   ├── driver.rs                       # Assignation conducteur (PartnerId)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyufleet_contract/                     # FleetContractOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── contract.rs                     # Contrat (type, dates, montant, responsable)
│   │   ├── alert.rs                        # Enregistrement alertes fin de contrat
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyufleet_service/                      # FleetServiceOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── service.rs                      # Service (entretien, réparation, sinistre)
│   │   ├── odometer.rs                     # Relevés odomètre
│   │   ├── workflow.rs                     # États service (planifié, en cours, terminé)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyufleet_cost/                         # FleetCostOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── cost_aggregation.rs             # Agrégation coûts (contrats + services)
│   │   ├── report.rs                       # Total, par véhicule, par conducteur
│   │   ├── export.rs                       # Pivot, CSV
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyufleet_request/                      # FleetRequestOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── request.rs                      # Demande véhicule (employé, modèle, statut)
│   │   ├── eligibility.rs                  # Modèles éligibles, limites parc
│   │   ├── validation.rs                   # Validation RH / Fleet Manager
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyufleet_ui/                           # FleetUI (frontend selon stack)
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── vehicle_list.rs
    │   │   ├── vehicle_kanban.rs
    │   │   ├── vehicle_form.rs             # Onglets Tax Info, Contract, Model, Note
    │   │   ├── model_form.rs               # Fabricant, catégorie, engine, vendors
    │   │   ├── service_list.rs
    │   │   ├── service_kanban.rs            # Par stade
    │   │   ├── service_form.rs
    │   │   ├── contract_list.rs
    │   │   ├── contract_form.rs
    │   │   ├── cost_reports.rs             # Total, by vehicle, by driver, comparison
    │   │   ├── request_form.rs             # Demande véhicule (employé)
    │   │   └── config_views.rs             # Settings, Fabricants, Modèles, Catégories, Types service
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, Caring Nanny, TAMR)

**Kits existants :**
- `miyucontacts` : Conducteur (res.partner), fournisseurs, responsable (user)
- `miyunotify` : Alertes fin de contrat (email, in-app)
- `miyuhr` : Employé demandeur, work_contact_id (conducteur), Fleet Mobility Card
- `miyuvalidate` : Validation champs (immatriculation, VIN, etc.)

---

## 2. Schémas de Données

### 2.1 Modèle Vehicle

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: VehicleId,
    pub name: Option<String>,
    pub model_id: ModelId,
    pub license_plate: Option<String>,
    pub vin_sn: Option<String>,
    pub company_id: CompanyId,
    pub driver_id: Option<PartnerId>,
    pub active: bool,
    pub fiscality: Option<VehicleFiscality>,
    pub notes: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleFiscality {
    // Champs selon localisation (taxe véhicule, déduction, etc.)
}
```

### 2.2 Modèle Model, Brand, Category

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleModelBrand {
    pub id: BrandId,
    pub name: String,
    pub image_id: Option<ImageId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleModelCategory {
    pub id: CategoryId,
    pub name: String,
    pub sequence: u32,
    pub max_weight: Option<Decimal>,
    pub max_volume: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleModel {
    pub id: ModelId,
    pub name: String,
    pub brand_id: BrandId,
    pub vehicle_type: VehicleType, // Car | Bike
    pub category_id: Option<CategoryId>,
    pub model_year: Option<u16>,
    pub seats: Option<u8>,
    pub doors: Option<u8>,
    pub color: Option<String>,
    pub trailer_hitch: bool,
    pub fuel_type: Option<FuelType>,
    pub range_km: Option<u32>,
    pub co2_g_km: Option<u32>,
    pub emission_standard: Option<String>,
    pub transmission: Option<Transmission>,
    pub power_kw: Option<Decimal>,
    pub horsepower: Option<u16>,
    pub can_be_requested: bool,
    pub catalog_value: Option<Decimal>,
    pub co2_fee: Option<Decimal>,
    pub cost_depreciated: Option<Decimal>,
    pub total_cost_depreciated: Option<Decimal>,
    pub vendor_ids: Vec<PartnerId>,
    pub active: bool,
}
```

### 2.3 Modèle Contract

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleContract {
    pub id: ContractId,
    pub vehicle_id: VehicleId,
    pub contract_type: ContractType,
    pub company_id: CompanyId,
    pub amount: Decimal,
    pub start_date: Date,
    pub expiration_date: Date,
    pub responsible_id: UserId,
    pub state: ContractState, // Active | Expired | Closed
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
```

### 2.4 Modèle Service

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleService {
    pub id: ServiceId,
    pub vehicle_id: VehicleId,
    pub service_type_id: ServiceTypeId,
    pub date: Date,
    pub amount: Option<Decimal>,
    pub vendor_id: Option<PartnerId>,
    pub driver_id: Option<PartnerId>,
    pub odometer: Option<u32>,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub state: ServiceState, // Draft | Planned | InProgress | Done
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceType {
    pub id: ServiceTypeId,
    pub name: String,
    pub is_accident: bool,
}
```

### 2.5 Modèle Odometer

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleOdometer {
    pub id: OdometerId,
    pub vehicle_id: VehicleId,
    pub driver_id: Option<PartnerId>,
    pub value: u32,
    pub date: Date,
    pub created_at: Timestamp,
}
```

### 2.6 Modèle VehicleRequest

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleRequest {
    pub id: RequestId,
    pub employee_id: EmployeeId,
    pub model_id: ModelId,
    pub state: RequestState, // Pending | Accepted | Rejected
    pub vehicle_id: Option<VehicleId>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
```

---

## 3. API et Contrats

### 3.1 FleetVehicleOperator

- `vehicle.create(intent, mandate) -> Result<Vehicle>`
- `vehicle.update(vehicle_id, intent, mandate) -> Result<Vehicle>`
- `vehicle.archive(vehicle_id, mandate) -> Result<()>`
- `vehicle.assign_driver(vehicle_id, driver_id, mandate) -> Result<Vehicle>`
- `vehicle.list(filters, mandate) -> Result<Vec<Vehicle>>`
- `vehicle.get_by_id(vehicle_id, mandate) -> Result<Vehicle>`

### 3.2 FleetModelOperator

- `model.create(intent, mandate) -> Result<VehicleModel>`
- `model.update(model_id, intent, mandate) -> Result<VehicleModel>`
- `model.list(filters, mandate) -> Result<Vec<VehicleModel>>`
- `brand.list(mandate) -> Result<Vec<VehicleModelBrand>>`
- `category.list(mandate) -> Result<Vec<VehicleModelCategory>>`
- `service_type.list(mandate) -> Result<Vec<ServiceType>>`
- `model.is_eligible_for_request(model_id) -> Result<bool>`

### 3.3 FleetContractOperator

- `contract.create(intent, mandate) -> Result<VehicleContract>`
- `contract.update(contract_id, intent, mandate) -> Result<VehicleContract>`
- `contract.close(contract_id, mandate) -> Result<VehicleContract>`
- `contract.list_expiring(days, mandate) -> Result<Vec<VehicleContract>>`
- `contract.get_by_vehicle(vehicle_id, mandate) -> Result<Vec<VehicleContract>>`

### 3.4 FleetServiceOperator

- `service.create(intent, mandate) -> Result<VehicleService>`
- `service.update(service_id, intent, mandate) -> Result<VehicleService>`
- `service.list(filters, mandate) -> Result<Vec<VehicleService>>`
- `odometer.log(vehicle_id, value, date, driver_id, mandate) -> Result<VehicleOdometer>`
- `service.list_by_vehicle(vehicle_id, mandate) -> Result<Vec<VehicleService>>`
- `service.list_accidents(filters, mandate) -> Result<Vec<VehicleService>>`

### 3.5 FleetCostOperator

- `cost.total(period_start, period_end, mandate) -> Result<Decimal>`
- `cost.by_vehicle(period_start, period_end, mandate) -> Result<Vec<CostByVehicle>>`
- `cost.by_driver(period_start, period_end, mandate) -> Result<Vec<CostByDriver>>`
- `cost.detailed_comparison(filters, mandate) -> Result<DetailedCostReport>`
- `cost.export(format, filters, mandate) -> Result<Bytes>`

### 3.6 FleetRequestOperator

- `request.create(employee_id, model_id, mandate) -> Result<VehicleRequest>`
- `request.validate(request_id, accept, vehicle_id, mandate) -> Result<VehicleRequest>`
- `request.list_eligible_models(mandate) -> Result<Vec<VehicleModel>>`
- `request.list_my_requests(employee_id, mandate) -> Result<Vec<VehicleRequest>>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (Fondations)

**Objectif :** Parc véhicules, modèles, conducteur, contrats et services de base.

**Livrables :**
- Crates : `miyufleet` (véhicules, modèles, fabricants, catégories, types de service)
- Crates : `miyufleet_contract` (contrats, alertes fin de contrat)
- Crates : `miyufleet_service` (services, odomètre)
- API : vehicle CRUD, model CRUD, contract CRUD, service CRUD, odometer.log
- Intégration : MiyuContacts (conducteur, fournisseurs), MiyuNotify (alertes)
- UI : Liste véhicules, formulaire véhicule (onglets de base), liste contrats, liste services, formulaire service, Configuration (modèles, types de service)

**Durée estimée :** 4–6 semaines

### Phase 2 — Coûts et Rapports

**Objectif :** Analyse des coûts et export.

**Livrables :**
- Crate : `miyufleet_cost` (agrégation, rapports, export)
- API : cost.total, cost.by_vehicle, cost.by_driver, cost.export
- UI : Rapports coûts (Total, Par véhicule, Par conducteur, Comparaison détaillée), export pivot/CSV
- Intégration optionnelle : module Comptabilité / Analytique (export coûts)

**Durée estimée :** 2–3 semaines

### Phase 3 — Demande de véhicule

**Objectif :** Flux demande véhicule (employé), validation RH / Fleet Manager, attribution.

**Livrables :**
- Crate : `miyufleet_request` (demande, éligibilité, limites parc, validation)
- API : request.create, request.validate, request.list_eligible_models, request.list_my_requests
- UI : Formulaire demande véhicule (employé), écran validation (RH / Fleet Manager), attribution véhicule
- Intégration : MiyuHR (employé, work_contact_id), FleetVehicleOperator (assign_driver)
- TAMR : Validation humaine (accepter / refuser)

**Durée estimée :** 3–4 semaines

### Phase 4 — Compléments et Optimisations

**Objectif :** Kanban services, tableau de bord Fleet Manager, fiscalité avancée, intégration Paie (avantage en nature).

**Livrables :**
- UI : Kanban services (stades planifié / en cours / terminé), tableau de bord (véhicules actifs, contrats à échéance, prochains entretiens, coûts du mois)
- Fiscalité : Champs selon localisation (CO2 fee, Tax Deduction, Salary tab sur modèle si Belgique)
- Intégration Paie : Modèle avec cost_depreciated, can_be_requested ; dépréciation basée sur contrat (optionnel selon priorité)
- Inventory : Max Weight, Max Volume sur catégorie/modèle pour capacité chargement (optionnel)

**Durée estimée :** 2–3 semaines

---

## 5. Bornage Fonctionnel

### 5.1 MVP (Phase 1)

**In scope :**
- Véhicules : création, édition, archivage, assignation conducteur
- Modèles : fabricants, modèles, catégories, types de service (dont Accident)
- Contrats : création, édition, clôture, responsable, alerte fin de contrat (email)
- Services : création, édition, type, date, coût, fournisseur, conducteur, odomètre, description, notes
- Relevés odomètre
- Configuration : Settings (jours alerte), Fabricants, Modèles, Catégories, Types de service

**Out of scope (MVP) :**
- Demande de véhicule (employé)
- Rapports coûts (Total, Par véhicule, Par conducteur, Export)
- Kanban services
- Tableau de bord Fleet Manager
- Fiscalité avancée (Belgique)
- Intégration Paie (avantage en nature)
- Max Weight / Max Volume (Inventory)

### 5.2 Complet (Phases 1–4)

**In scope :**
- Tout le MVP
- Rapports coûts et export
- Demande de véhicule (éligibilité, validation, attribution)
- Kanban services, tableau de bord
- Fiscalité selon localisation (optionnel)
- Intégration Paie (optionnel)
- Intégration Inventory (optionnel)

### 5.3 Critères d'acceptation (MVP)

- Un véhicule peut être créé à partir d’un modèle existant (modèle, immatriculation, VIN, conducteur).
- Un conducteur est un contact (MiyuContacts) ; assignation sur la fiche véhicule.
- Un contrat peut être créé (type, dates, montant, responsable) ; alerte email au responsable X jours avant échéance.
- Un service peut être créé (véhicule, type, date, coût, fournisseur, odomètre, description, notes) ; les sinistres sont des services avec type Accident.
- Les relevés odomètre peuvent être saisis (véhicule, valeur, date, conducteur).
- Configuration : Fabricants, Modèles, Catégories, Types de service ; paramètre End Date Contract Alert.

### 5.4 Risques et mitigation

| Risque | Mitigation |
|--------|------------|
| Complexité référentiels (modèles, fiscalité locale) | Phase 1 avec champs minimaux ; extension fiscalité en Phase 4 |
| Alertes fin de contrat (scheduling) | Utiliser Caring Nanny + MiyuNotify ; job périodique ou événementiel selon stack |
| Demande véhicule (workflow RH) | TAMR pour validation humaine ; bien définir rôles (validateur vs demandeur) |
| Export coûts vers Comptabilité | API export (CSV/pivot) en Phase 2 ; liaison automatique avec module Comptabilité si priorité |

---

**Document** : Odoo Fleet — Guide d'Implémentation avec Bornage  
**Version** : 1.0  
**Date** : 2026-02-01
