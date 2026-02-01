# Odoo Maintenance — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Maintenance dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Maintenance
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (création équipement, création demande, changement de stage, calcul métriques)
- Gestion des gouvernances

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
MaintenanceUI → BondingBrother → MaintenanceRequestOperator / EquipmentOperator
                                              → StrongFather (décision)
                                              → KindMother (WriteIntent)
                                              → Master Butler (permissions)
                                              → WorrySentinel (sécurité)
                                              → MaintenanceMetricsOperator (lecture)
```

### 1.2 Flux Typiques

**Création demande de maintenance :**
1. Intention utilisateur → MaintenanceUI
2. Traduction intention → BondingBrother
3. Vérification accès équipement (Follower ou Equipment Manager) → Master Butler
4. Demande décision → StrongFather
5. Vérification permissions → Master Butler
6. Vérification sécurité → WorrySentinel
7. Persistance → KindMother (WriteIntent)
8. Notification (assignation) → MiyuNotify

**Création équipement :**
1. Intention utilisateur → MaintenanceUI
2. BondingBrother → EquipmentOperator
3. Décision → StrongFather
4. Permissions → Master Butler
5. Sécurité → WorrySentinel
6. Persistance → KindMother (WriteIntent)

**Changement de stage (demande) :**
1. Intention (glisser-déposer ou formulaire) → MaintenanceUI
2. BondingBrother → MaintenanceRequestOperator
3. Décision → StrongFather
4. WriteIntent (mise à jour stage) → KindMother
5. Recalcul métriques équipement (si stage Repaired/Done) → MaintenanceMetricsOperator (lecture + calcul) puis KindMother (mise à jour champs métriques équipement si prévu)

**Lecture métriques :**
1. Affichage équipement → MaintenanceUI
2. MaintenanceMetricsOperator : lecture demandes terminées + calcul MTBF, MTTR, Latest Failure, Estimated Next Failure
3. Pas d'écriture ; exposition en lecture seule

---

## 2. Patterns d'Intégration

### 2.1 Création d'un équipement

**Pattern :** WriteIntent + Mandate

```rust
// Pseudo-code Rust
pub struct CreateEquipmentIntent {
    pub name: String,
    pub category_id: Uuid,
    pub company_id: Uuid,
    pub used_by: UsedBy,  // Department | Employee | Other
    pub department_id: Option<Uuid>,
    pub employee_id: Option<Uuid>,
    pub maintenance_team_id: Option<Uuid>,
    pub technician_id: Option<Uuid>,
    pub used_in_location: Option<String>,
    pub workcenter_id: Option<Uuid>,
    pub expected_mtbf: Option<f64>,
    pub vendor_id: Option<Uuid>,
    pub model: Option<String>,
    pub serial_no: Option<String>,
    pub effective_date: Option<Date>,
    pub cost: Option<Decimal>,
    pub warranty_expiration_date: Option<Date>,
}

impl EquipmentOperator {
    pub async fn create_equipment(
        &self,
        intent: CreateEquipmentIntent,
        mandate: Mandate,
    ) -> Result<Equipment, MaintenanceError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["equipment.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_equipment",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(MaintenanceError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "equipment.create",
                resource: None,
            })
            .await?;

        if !permission.granted {
            return Err(MaintenanceError::PermissionDenied);
        }

        let security_level = self.worry_sentinel
            .get_security_level(&intent)
            .await?;

        if security_level > mandate.max_security_level {
            return Err(MaintenanceError::SecurityLevelExceeded);
        }

        let write_intent = WriteIntent {
            entity_type: "maintenance.equipment",
            operation: WriteOperation::Create,
            data: EquipmentData {
                name: intent.name,
                category_id: intent.category_id,
                company_id: intent.company_id,
                used_by: intent.used_by,
                department_id: intent.department_id,
                employee_id: intent.employee_id,
                maintenance_team_id: intent.maintenance_team_id,
                technician_id: intent.technician_id,
                used_in_location: intent.used_in_location,
                workcenter_id: intent.workcenter_id,
                expected_mtbf: intent.expected_mtbf,
                vendor_id: intent.vendor_id,
                model: intent.model,
                serial_no: intent.serial_no,
                effective_date: intent.effective_date,
                cost: intent.cost,
                warranty_expiration_date: intent.warranty_expiration_date,
                ..Default::default()
            },
            security_level,
        };

        let equipment = self.kind_mother.persist(write_intent).await?;
        Ok(equipment)
    }
}
```

### 2.2 Création d'une demande de maintenance

**Pattern :** WriteIntent + Mandate + vérification accès équipement

```rust
pub struct CreateMaintenanceRequestIntent {
    pub name: String,
    pub for_type: ForType,  // Equipment | WorkCenter
    pub equipment_id: Option<Uuid>,
    pub workcenter_id: Option<Uuid>,
    pub maintenance_type: MaintenanceType,  // Corrective | Preventive
    pub maintenance_team_id: Uuid,
    pub user_id: Option<Uuid>,
    pub schedule_date: Option<DateTime>,
    pub duration: Option<Duration>,
    pub priority: u8,  // 0-3
    pub block_workcenter: bool,
    pub description: Option<String>,
    pub manufacturing_order_id: Option<Uuid>,
    pub workorder_id: Option<Uuid>,
}

impl MaintenanceRequestOperator {
    pub async fn create_request(
        &self,
        intent: CreateMaintenanceRequestIntent,
        mandate: Mandate,
        requestor_id: UserId,
    ) -> Result<MaintenanceRequest, MaintenanceError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["request.create"])?;

        // Vérification accès équipement (Follower ou Equipment Manager)
        if intent.for_type == ForType::Equipment {
            let equipment_id = intent.equipment_id.ok_or(MaintenanceError::MissingEquipment)?;
            let can_create = self.master_butler
                .can_create_request_for_equipment(requestor_id, equipment_id)
                .await?;  // Follower ou equipment_manager
            if !can_create {
                return Err(MaintenanceError::AccessDeniedToEquipment);
            }
        }

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_maintenance_request",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(MaintenanceError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "maintenance.request",
            operation: WriteOperation::Create,
            data: MaintenanceRequestData {
                name: intent.name,
                for_type: intent.for_type,
                equipment_id: intent.equipment_id,
                workcenter_id: intent.workcenter_id,
                maintenance_type: intent.maintenance_type,
                maintenance_team_id: intent.maintenance_team_id,
                user_id: intent.user_id,
                schedule_date: intent.schedule_date,
                duration: intent.duration,
                priority: intent.priority,
                block_workcenter: intent.block_workcenter,
                description: intent.description,
                manufacturing_order_id: intent.manufacturing_order_id,
                workorder_id: intent.workorder_id,
                stage_id: default_stage_new_request(),
                request_date: Utc::now().date(),
                create_uid: requestor_id,
                ..Default::default()
            },
            security_level: 2,
        };

        let request = self.kind_mother.persist(write_intent).await?;

        // Notification assignation si user_id présent
        if let Some(user_id) = intent.user_id {
            self.miyu_notify
                .notify_assignment(NotifyAssignmentRequest {
                    resource_type: "maintenance.request",
                    resource_id: request.id,
                    assignee_id: user_id,
                })
                .await?;
        }

        Ok(request)
    }
}
```

### 2.3 Changement de stage (demande)

**Pattern :** WriteIntent + mise à jour métriques équipement (si Repaired/Done)

```rust
impl MaintenanceRequestOperator {
    pub async fn change_stage(
        &self,
        request_id: Uuid,
        new_stage_id: Uuid,
        mandate: Mandate,
    ) -> Result<MaintenanceRequest, MaintenanceError> {
        mandate.validate_flows(&["request.update"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "change_maintenance_request_stage",
                context: &ChangeStageContext { request_id, new_stage_id },
            })
            .await?;

        if !decision.allowed {
            return Err(MaintenanceError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "maintenance.request",
            operation: WriteOperation::Update,
            data: MaintenanceRequestData {
                id: request_id,
                stage_id: new_stage_id,
                ..Default::default()
            },
            security_level: 2,
        };

        let updated = self.kind_mother.persist(write_intent).await?;

        // Si stage = Repaired ou Done : déclencher recalcul métriques équipement
        if is_stage_repaired_or_done(new_stage_id) {
            if let Some(equipment_id) = updated.equipment_id {
                self.maintenance_metrics
                    .recompute_equipment_metrics(equipment_id)
                    .await?;  // Lit les demandes, calcule MTBF/MTTR/Latest/Estimated, puis WriteIntent sur equipment
            }
        }

        Ok(updated)
    }
}
```

### 2.4 Lecture des métriques (MaintenanceMetricsOperator)

**Pattern :** Lecture uniquement ; pas de WriteIntent depuis cet Opérateur (recalcul peut mettre à jour équipement via KindMother si design le prévoit)

```rust
impl MaintenanceMetricsOperator {
    pub async fn get_equipment_metrics(
        &self,
        equipment_id: Uuid,
        mandate: Mandate,
    ) -> Result<EquipmentMetrics, MaintenanceError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["metrics.read"])?;

        let equipment = self.kind_mother
            .read_equipment(equipment_id)
            .await?;

        let completed_requests = self.kind_mother
            .read_maintenance_requests_for_equipment(equipment_id, CompletedOnly)
            .await?;

        let mtbf = compute_mtbf(&completed_requests);
        let mttr = compute_mttr(&completed_requests);
        let latest_failure = latest_failure_date(&completed_requests);
        let estimated_next_failure = latest_failure.map(|d| d + mtbf_days(mtbf));

        Ok(EquipmentMetrics {
            expected_mtbf: equipment.expected_mtbf,
            mtbf,
            mttr,
            latest_failure,
            estimated_next_failure,
        })
    }
}
```

---

## 3. Gestion des Gouvernances

- **StrongFather** : Toute création / modification équipement, demande, équipe, catégorie ; changement de stage.
- **KindMother** : Seule autorité d'écriture (WriteIntent) pour équipements, demandes, équipes, catégories.
- **Master Butler** : Permissions « equipment.create », « request.create », « request.update », « team.create », « category.create », « metrics.read » ; règle « can_create_request_for_equipment » (Follower ou Equipment Manager).
- **WorrySentinel** : Niveau de sécurité 2 pour données Maintenance ; pas de descente de niveau.
- **Ever Buddy** : Cycle de vie des entités (équipement, demande) et transitions de stage (demande).

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
