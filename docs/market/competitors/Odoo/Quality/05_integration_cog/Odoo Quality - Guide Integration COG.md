# Odoo Quality — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Quality dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Quality
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust (création QCP, création contrôle, traitement contrôle Pass/Fail, création alerte)
- Gestion des gouvernances

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
QualityUI → BondingBrother → QualityCheckOperator / QualityControlPointOperator / QualityAlertOperator
                                              → StrongFather (décision)
                                              → KindMother (WriteIntent)
                                              → Master Butler (permissions)
                                              → WorrySentinel (sécurité)
                                              → QualityMetricsOperator (lecture)
Événement MO / Picking / WO → QualityCheckOperator (évaluation QCP) → création contrôles
```

### 1.2 Flux Typiques

**Création d'un QCP :**
1. Intention utilisateur → QualityUI
2. Traduction intention → BondingBrother
3. Décision → StrongFather
4. Permissions → Master Butler
5. Sécurité → WorrySentinel
6. Persistance → KindMother (WriteIntent)

**Création automatique de contrôles (QCP) :**
1. Événement (MO confirmé, picking validé, WO démarré) → MiyuManufacturing / MiyuInventory
2. Notification vers QualityCheckOperator (ou évaluation par QualityCheckOperator des ordres concernés)
3. QualityCheckOperator consulte QualityControlPointOperator (évaluation des QCP applicables)
4. Pour chaque QCP déclenché : Décision → StrongFather ; WriteIntent → KindMother (création contrôles)

**Traitement d'un contrôle (Pass / Fail) :**
1. Intention utilisateur (Pass ou Fail) → QualityUI (ou pop-up sur ordre / Shop Floor)
2. BondingBrother → QualityCheckOperator
3. Décision → StrongFather
4. WriteIntent (mise à jour état du contrôle) → KindMother
5. Si Fail et « Message If Failure » configuré : création alerte ou notification → QualityAlertOperator / MiyuNotify

**Création d'une alerte qualité :**
1. Intention utilisateur (depuis app Quality, MO, picking ou Shop Floor) → QualityUI ou contexte ordre
2. BondingBrother → QualityAlertOperator
3. Décision → StrongFather
4. WriteIntent → KindMother
5. Notification assignation (si responsable) → MiyuNotify

**Lecture rapports (métriques) :**
1. QualityUI demande taux de conformité / statut contrôles / causes défauts
2. QualityMetricsOperator : lecture contrôles et alertes via KindMother (ou lecture directe selon design)
3. Calculs (conformité, agrégations) ; exposition en lecture seule

---

## 2. Patterns d'Intégration

### 2.1 Création d'un point de contrôle qualité (QCP)

**Pattern :** WriteIntent + Mandate

```rust
pub struct CreateQualityControlPointIntent {
    pub title: String,
    pub company_id: Uuid,
    pub operation_ids: Vec<Uuid>,           // Manufacturing, Receipt, Delivery, etc.
    pub work_order_operation_id: Option<Uuid>,
    pub product_ids: Vec<Uuid>,
    pub category_ids: Vec<Uuid>,
    pub control_per: ControlPer,             // Operation | Product | Quantity
    pub partial_percentage: Option<f64>,
    pub control_frequency: ControlFrequency, // All | Randomly | Periodically
    pub random_percentage: Option<f64>,
    pub period_value: Option<u32>,
    pub period_unit: PeriodUnit,             // Days | Weeks | Months
    pub check_type: QualityCheckType,        // Instructions | PassFail | Measure | Picture | Worksheet | Spreadsheet | RegisterProduction | PrintLabel
    pub template_id: Option<Uuid>,
    pub team_id: Uuid,
    pub responsible_id: Option<Uuid>,
    pub instructions: Option<String>,
    pub message_if_failure: Option<String>,
}

impl QualityControlPointOperator {
    pub async fn create_qcp(
        &self,
        intent: CreateQualityControlPointIntent,
        mandate: Mandate,
    ) -> Result<QualityControlPoint, QualityError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["qcp.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_quality_control_point",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(QualityError::DecisionDenied);
        }

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "qcp.create",
                resource: None,
            })
            .await?;

        if !permission.granted {
            return Err(QualityError::PermissionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "quality.control_point",
            operation: WriteOperation::Create,
            data: QualityControlPointData {
                title: intent.title,
                company_id: intent.company_id,
                operation_ids: intent.operation_ids,
                work_order_operation_id: intent.work_order_operation_id,
                product_ids: intent.product_ids,
                category_ids: intent.category_ids,
                control_per: intent.control_per,
                partial_percentage: intent.partial_percentage,
                control_frequency: intent.control_frequency,
                random_percentage: intent.random_percentage,
                period_value: intent.period_value,
                period_unit: intent.period_unit,
                check_type: intent.check_type,
                template_id: intent.template_id,
                team_id: intent.team_id,
                responsible_id: intent.responsible_id,
                instructions: intent.instructions,
                message_if_failure: intent.message_if_failure,
                ..Default::default()
            },
            security_level: 2,
        };

        let qcp = self.kind_mother.persist(write_intent).await?;
        Ok(qcp)
    }
}
```

### 2.2 Traitement d'un contrôle (Pass / Fail)

**Pattern :** WriteIntent + déclenchement « Message If Failure » (alerte) si Fail

```rust
impl QualityCheckOperator {
    pub async fn process_check(
        &self,
        check_id: Uuid,
        result: CheckResult,  // Pass | Fail
        measure_value: Option<f64>,  // si type Measure
        mandate: Mandate,
    ) -> Result<QualityCheck, QualityError> {
        mandate.validate_flows(&["check.process"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "process_quality_check",
                context: &ProcessCheckContext { check_id, result },
            })
            .await?;

        if !decision.allowed {
            return Err(QualityError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "quality.check",
            operation: WriteOperation::Update,
            data: QualityCheckData {
                id: check_id,
                state: match result {
                    CheckResult::Pass => CheckState::Passed,
                    CheckResult::Fail => CheckState::Failed,
                },
                measure_value,
                processed_at: Utc::now(),
                ..Default::default()
            },
            security_level: 2,
        };

        let updated = self.kind_mother.persist(write_intent).await?;

        if result == CheckResult::Fail {
            if let Some(qcp_id) = updated.control_point_id {
                let qcp = self.kind_mother.read_qcp(qcp_id).await?;
                if let Some(msg) = qcp.message_if_failure {
                    // Créer alerte qualité si configuré (ex. "Create a quality alert")
                    self.quality_alert
                        .create_alert_from_check_failure(updated.id, &msg, mandate.clone())
                        .await?;
                }
                self.miyu_notify.notify_check_failed(updated.id).await?;
            }
        }

        Ok(updated)
    }
}
```

### 2.3 Création d'une alerte qualité

**Pattern :** WriteIntent + Mandate

```rust
pub struct CreateQualityAlertIntent {
    pub title: String,
    pub product_id: Option<Uuid>,
    pub work_center_id: Option<Uuid>,
    pub picking_id: Option<Uuid>,
    pub production_order_id: Option<Uuid>,
    pub team_id: Uuid,
    pub responsible_id: Option<Uuid>,
    pub tags: Vec<Uuid>,
    pub root_cause_id: Option<Uuid>,
    pub priority: u8,
    pub description: Option<String>,
    pub corrective_actions: Option<String>,
    pub preventive_actions: Option<String>,
    pub company_id: Uuid,
}

impl QualityAlertOperator {
    pub async fn create_alert(
        &self,
        intent: CreateQualityAlertIntent,
        mandate: Mandate,
    ) -> Result<QualityAlert, QualityError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["alert.create"])?;

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "create_quality_alert",
                context: &intent,
            })
            .await?;

        if !decision.allowed {
            return Err(QualityError::DecisionDenied);
        }

        let write_intent = WriteIntent {
            entity_type: "quality.alert",
            operation: WriteOperation::Create,
            data: QualityAlertData {
                title: intent.title,
                product_id: intent.product_id,
                work_center_id: intent.work_center_id,
                picking_id: intent.picking_id,
                production_order_id: intent.production_order_id,
                team_id: intent.team_id,
                responsible_id: intent.responsible_id,
                tags: intent.tags,
                root_cause_id: intent.root_cause_id,
                priority: intent.priority,
                description: intent.description,
                corrective_actions: intent.corrective_actions,
                preventive_actions: intent.preventive_actions,
                company_id: intent.company_id,
                stage_id: default_stage_new(),
                ..Default::default()
            },
            security_level: 2,
        };

        let alert = self.kind_mother.persist(write_intent).await?;

        if let Some(resp_id) = intent.responsible_id {
            self.miyu_notify
                .notify_assignment(NotifyAssignmentRequest {
                    resource_type: "quality.alert",
                    resource_id: alert.id,
                    assignee_id: resp_id,
                })
                .await?;
        }

        Ok(alert)
    }
}
```

### 2.4 Création automatique de contrôles (évaluation QCP)

**Pattern :** Événement ordre → évaluation QCP → création contrôles (WriteIntent en lot)

```rust
impl QualityCheckOperator {
    pub async fn on_order_created_or_confirmed(
        &self,
        order_context: OrderContext,  // MO | Picking | WorkOrder
        mandate: Mandate,
    ) -> Result<Vec<QualityCheck>, QualityError> {
        mandate.validate_flows(&["check.create"])?;

        let applicable_qcps = self.quality_control_point
            .evaluate_applicable_qcps(&order_context)
            .await?;

        let mut created = Vec::new();
        for qcp in applicable_qcps {
            if !self.should_trigger_for_frequency(&qcp, &order_context).await? {
                continue;
            }

            let count = self.control_per_count(&qcp, &order_context).await?;
            for _ in 0..count {
                let decision = self.strong_father
                    .decide(DecisionRequest {
                        action: "create_quality_check_from_qcp",
                        context: &CreateCheckFromQcpContext { qcp_id: qcp.id, order_context: &order_context },
                    })
                    .await?;

                if !decision.allowed {
                    continue;
                }

                let write_intent = WriteIntent {
                    entity_type: "quality.check",
                    operation: WriteOperation::Create,
                    data: QualityCheckData {
                        control_point_id: Some(qcp.id),
                        picking_id: order_context.picking_id(),
                        production_order_id: order_context.production_order_id(),
                        workorder_id: order_context.workorder_id(),
                        product_ids: order_context.product_ids(),
                        team_id: qcp.team_id,
                        check_type: qcp.check_type,
                        template_id: qcp.template_id,
                        state: CheckState::Pending,
                        ..Default::default()
                    },
                    security_level: 2,
                };

                let check = self.kind_mother.persist(write_intent).await?;
                created.push(check);
            }
        }

        Ok(created)
    }
}
```

---

## 3. Gestion des Gouvernances

- **StrongFather** : Toute création / modification QCP, contrôle, alerte, équipe, Failure Location ; traitement contrôle (Pass/Fail) ; création automatique de contrôles depuis QCP.
- **KindMother** : Seule autorité d'écriture (WriteIntent) pour QCP, contrôles, alertes, équipes, Failure Locations.
- **Master Butler** : Permissions « qcp.create », « check.create », « check.process », « alert.create », « team.create », « failure_location.create », « metrics.read » ; rôles Quality User / Quality Manager.
- **WorrySentinel** : Niveau de sécurité 2 pour données Quality ; pas de descente de niveau.
- **Ever Buddy** : Cycle de vie QCP, contrôles (pending → passed/failed), alertes (stages).

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
