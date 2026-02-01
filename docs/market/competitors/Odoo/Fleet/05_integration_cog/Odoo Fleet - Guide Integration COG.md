# Odoo Fleet — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Fleet (Flotte véhicules) dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Fleet
- Patterns WriteIntent et Mandates (création véhicule, contrat, service, demande véhicule)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, Master Butler, WorrySentinel)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
FleetUI → BondingBrother → FleetVehicleOperator ──► StrongFather (décision create/update/archive)
                            FleetModelOperator       KindMother (WriteIntent)
                            FleetContractOperator    Master Butler (permissions)
                            FleetServiceOperator     WorrySentinel (sécurité)
                            FleetCostOperator        Caring Nanny (état contrats à échéance)
                            FleetRequestOperator     TAMR (validation humaine demande)
```

### 1.2 Flux typiques

1. **Création véhicule** : Intention utilisateur → FleetUI → BondingBrother → FleetVehicleOperator → StrongFather (décision) → KindMother (WriteIntent Create).
2. **Modification véhicule / assignation conducteur** : FleetVehicleOperator.update / assign_driver → Master Butler (permission) → KindMother (WriteIntent Update).
3. **Création contrat** : FleetContractOperator.create → StrongFather (décision) → KindMother (WriteIntent Create) ; alertes gérées par Caring Nanny + MiyuNotify.
4. **Création service** : FleetServiceOperator.create → StrongFather (décision) → KindMother (WriteIntent Create).
5. **Demande véhicule** : FleetRequestOperator.create → KindMother (WriteIntent) ; validation : FleetRequestOperator.validate → StrongFather (décision) → TAMR (validation humaine) → KindMother (WriteIntent Update) + FleetVehicleOperator.assign_driver.
6. **Lecture coûts** : FleetCostOperator.total / by_vehicle / by_driver → lecture KindMother (contrats, services) via Mandat ; périmètre selon Master Butler (tous vs mes véhicules).

---

## 2. Patterns d'Intégration

### 2.1 Création de véhicule

**Pattern :** WriteIntent Create + Mandate

```rust
// Pseudo-code Rust
pub struct CreateVehicleIntent {
    pub name: Option<String>,
    pub model_id: ModelId,
    pub license_plate: Option<String>,
    pub vin_sn: Option<String>,
    pub company_id: CompanyId,
    pub driver_id: Option<PartnerId>,
    // ... fiscality, contract ref, notes
}

impl FleetVehicleOperator {
    pub async fn create_vehicle(
        &self,
        intent: CreateVehicleIntent,
        mandate: Mandate,
    ) -> Result<Vehicle, FleetError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["vehicle.create"])?;

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "vehicle.create",
                resource: None,
            })
            .await?;
        if !permission.granted {
            return Err(FleetError::PermissionDenied);
        }

        let security_level = self.worry_sentinel
            .get_security_level(&intent)
            .await?;
        if security_level > mandate.max_security_level {
            return Err(FleetError::SecurityLevelExceeded);
        }

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "vehicle.create",
                context: &intent,
            })
            .await?;
        if !decision.allowed {
            return Err(FleetError::DecisionDenied(decision.reason));
        }

        let write_intent = VehicleWriteIntent::Create {
            name: intent.name,
            model_id: intent.model_id,
            license_plate: intent.license_plate,
            vin_sn: intent.vin_sn,
            company_id: intent.company_id,
            driver_id: intent.driver_id,
        };

        let vehicle = self.kind_mother
            .execute_write(VehicleWriteIntent::Create(write_intent), mandate.clone())
            .await?;

        Ok(vehicle)
    }
}
```

### 2.2 Assignation conducteur

**Pattern :** WriteIntent Update + Mandate

```rust
pub struct AssignDriverIntent {
    pub vehicle_id: VehicleId,
    pub driver_id: PartnerId,
}

impl FleetVehicleOperator {
    pub async fn assign_driver(
        &self,
        intent: AssignDriverIntent,
        mandate: Mandate,
    ) -> Result<Vehicle, FleetError> {
        mandate.validate_flows(&["vehicle.assign_driver"])?;

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "vehicle.assign_driver",
                resource: Some(ResourceId::Vehicle(intent.vehicle_id)),
            })
            .await?;
        if !permission.granted {
            return Err(FleetError::PermissionDenied);
        }

        let write_intent = VehicleWriteIntent::Update {
            vehicle_id: intent.vehicle_id,
            driver_id: Some(intent.driver_id),
            ..Default::default()
        };

        let vehicle = self.kind_mother
            .execute_write(write_intent, mandate.clone())
            .await?;

        Ok(vehicle)
    }
}
```

### 2.3 Création de contrat et alerte fin de contrat

**Pattern :** WriteIntent Create + Caring Nanny (état) + MiyuNotify (notification)

```rust
pub struct CreateContractIntent {
    pub vehicle_id: VehicleId,
    pub contract_type: ContractType,
    pub start_date: Date,
    pub expiration_date: Date,
    pub amount: Decimal,
    pub responsible_id: UserId,
}

impl FleetContractOperator {
    pub async fn create_contract(
        &self,
        intent: CreateContractIntent,
        mandate: Mandate,
    ) -> Result<Contract, FleetError> {
        mandate.validate_flows(&["contract.create"])?;
        // ... permission, decision, write_intent

        let contract = self.kind_mother
            .execute_write(ContractWriteIntent::Create { ... }, mandate.clone())
            .await?;

        // Enregistrement pour alerte : Caring Nanny + MiyuNotify
        let days_before = self.config.end_date_contract_alert_days;
        self.caring_nanny
            .register_alert(AlertRegistration {
                resource: ResourceId::Contract(contract.id),
                trigger_at: contract.expiration_date - days_before,
                action: AlertAction::NotifyContractExpiring {
                    contract_id: contract.id,
                    responsible_id: intent.responsible_id,
                },
            })
            .await?;

        Ok(contract)
    }
}
```

### 2.4 Création de service (entretien / sinistre)

**Pattern :** WriteIntent Create + Mandate

```rust
pub struct CreateServiceIntent {
    pub vehicle_id: VehicleId,
    pub service_type_id: ServiceTypeId,
    pub date: Date,
    pub amount: Option<Decimal>,
    pub vendor_id: Option<PartnerId>,
    pub driver_id: Option<PartnerId>,
    pub odometer: Option<u32>,
    pub description: Option<String>,
    pub notes: Option<String>,
}

impl FleetServiceOperator {
    pub async fn create_service(
        &self,
        intent: CreateServiceIntent,
        mandate: Mandate,
    ) -> Result<Service, FleetError> {
        mandate.validate_flows(&["service.create"])?;
        // ... permission, decision, write_intent

        let service = self.kind_mother
            .execute_write(ServiceWriteIntent::Create { ... }, mandate.clone())
            .await?;

        Ok(service)
    }
}
```

### 2.5 Demande de véhicule et validation

**Pattern :** WriteIntent Create (demande) + StrongFather + TAMR (validation humaine) + WriteIntent Update (attribution)

```rust
// Création demande (employé demandeur)
impl FleetRequestOperator {
    pub async fn create_request(
        &self,
        intent: CreateRequestIntent { employee_id, model_id },
        mandate: Mandate,
    ) -> Result<VehicleRequest, FleetError> {
        mandate.validate_flows(&["request.create"])?;
        // Vérifier éligibilité modèle (can_be_requested) et limites parc
        let eligible = self.fleet_model_operator
            .is_model_eligible_for_request(intent.model_id)
            .await?;
        if !eligible {
            return Err(FleetError::ModelNotEligible);
        }
        let within_limits = self.check_request_limits(intent.employee_id).await?;
        if !within_limits {
            return Err(FleetError::RequestLimitExceeded);
        }
        let request = self.kind_mother
            .execute_write(RequestWriteIntent::Create { ... }, mandate.clone())
            .await?;
        Ok(request)
    }

    // Validation (RH / Fleet Manager)
    pub async fn validate_request(
        &self,
        request_id: RequestId,
        accept: bool,
        vehicle_id: Option<VehicleId>, // si accept, véhicule à attribuer
        mandate: Mandate,
    ) -> Result<VehicleRequest, FleetError> {
        mandate.validate_flows(&["request.validate"])?;
        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "request.validate",
                context: &(request_id, accept, vehicle_id),
            })
            .await?;
        if !decision.allowed {
            return Err(FleetError::DecisionDenied(decision.reason));
        }
        // TAMR : validation humaine (RH / Fleet Manager)
        let human_approval = self.tamr
            .require_human_approval(ApprovalRequest {
                action: "request.validate",
                context: format!("Demande véhicule {} - Accepter: {} - Véhicule: {:?}", request_id, accept, vehicle_id),
            })
            .await?;
        if !human_approval.approved {
            return Err(FleetError::HumanApprovalDenied);
        }
        let request = self.kind_mother
            .execute_write(RequestWriteIntent::Update {
                request_id,
                state: if accept { RequestState::Accepted } else { RequestState::Rejected },
                vehicle_id,
            }, mandate.clone())
            .await?;
        if accept && vehicle_id.is_some() {
            // Attribution conducteur (employé → contact travail)
            let employee = self.miyu_hr.get_employee(intent.employee_id).await?;
            let driver_id = employee.work_contact_id.ok_or(FleetError::NoWorkContact)?;
            self.fleet_vehicle_operator
                .assign_driver(AssignDriverIntent { vehicle_id: vehicle_id.unwrap(), driver_id }, mandate.clone())
                .await?;
        }
        Ok(request)
    }
}
```

### 2.6 Lecture des coûts (périmètre selon rôle)

**Pattern :** Mandate + Master Butler (périmètre) + KindMother (lecture)

```rust
impl FleetCostOperator {
    pub async fn cost_by_vehicle(
        &self,
        period_start: Date,
        period_end: Date,
        mandate: Mandate,
    ) -> Result<Vec<CostByVehicle>, FleetError> {
        mandate.validate_flows(&["cost.read"])?;
        let scope = self.master_butler
            .get_scope(PermissionRequest {
                operator: self.id(),
                capability: "cost.read",
                resource: None,
            })
            .await?;
        // scope = AllVehicles ou MyVehicles (conducteur)
        let vehicle_ids = match scope {
            Scope::AllVehicles => self.kind_mother.list_vehicle_ids(mandate.company_id).await?,
            Scope::MyVehicles => self.kind_mother.list_vehicle_ids_for_driver(mandate.user_id).await?,
        };
        let costs = self.kind_mother
            .aggregate_costs_by_vehicle(vehicle_ids, period_start, period_end, mandate.clone())
            .await?;
        Ok(costs)
    }
}
```

---

## 3. Gestion des Erreurs et Rollback

### 3.1 Erreurs typiques

- **PermissionDenied** : Master Butler refuse l’action (capability ou resource).
- **SecurityLevelExceeded** : WorrySentinel détecte un niveau de sécurité supérieur au mandat.
- **DecisionDenied** : StrongFather refuse la décision (raison dans decision.reason).
- **HumanApprovalDenied** : TAMR refuse (validation humaine refusée).
- **ModelNotEligible** : Modèle non éligible pour demande véhicule (can_be_requested = false).
- **RequestLimitExceeded** : Limites parc ou politique dépassées pour demande véhicule.
- **KindMotherError** : Échec persistance (WriteIntent refusé ou conflit).

### 3.2 Rollback

- Les WriteIntent sont exécutés dans une transaction côté KindMother ; en cas d’échec, rollback automatique.
- Pour les flux multi-opérateurs (ex. validation demande + assignation conducteur), prévoir une compensation (ex. remettre la demande en « en attente » si assignation échoue) ou une transaction distribuée selon capacités Miyukini.

---

## 4. Intégration avec Kits existants

### 4.1 MiyuContacts

- **Conducteur** : driver_id = PartnerId ; création « Create a new driver » → MiyuContacts.create_partner.
- **Fournisseur** : vendor_id = PartnerId sur service ; vendor_ids sur modèle → MiyuContacts.
- **Responsable contrat** : responsible_id = UserId ; utilisateur lié à un res.partner (MiyuContacts).

### 4.2 MiyuHR

- **Employé conducteur** : Lien employé ↔ conducteur via work_contact_id ; champ Fleet Mobility Card sur fiche employé (véhicule ou carte).
- **Demande véhicule** : Employé demandeur ; validation RH ; attribution véhicule et liaison conducteur (employé) → MiyuHR.get_employee, work_contact_id.

### 4.3 MiyuNotify

- **Alertes fin de contrat** : Envoi email au responsable (responsible_id) X jours avant expiration ; notification in-app si supportée (MiyuNotify.send_email, MiyuNotify.notify_user).

### 4.4 Comptabilité / Analytique (si module Miyukini)

- **Export coûts** : FleetCostOperator expose cost.total, cost.by_vehicle, cost.by_driver, cost.export ; le module Comptabilité consomme ces données pour écritures analytiques ou import CSV.

---

**Document** : Odoo Fleet — Guide Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01
