# Odoo Employees — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Employees (Employés) dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Employees
- Patterns WriteIntent et Mandates (création fiche, modification, offboarding, présence)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, Master Butler, WorrySentinel)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
EmployeeUI → BondingBrother → EmployeeOperator ──► StrongFather (décision create/update/deactivate)
                            DepartmentOperator     KindMother (WriteIntent)
                            EmployeeSkillsOperator Master Butler (permissions)
                            EmployeeOffboardingOperator  WorrySentinel (sécurité)
                            EmployeePresenceOperator     Caring Nanny (état présence)
```

### 1.2 Flux typiques

1. **Création fiche employé** : Intention utilisateur → EmployeeUI → BondingBrother → EmployeeOperator → StrongFather (décision) → KindMother (WriteIntent Create).
2. **Modification fiche** : EmployeeOperator.update → Master Butler (permission update ou self_edit) → KindMother (WriteIntent Update).
3. **Lecture présence** : EmployeeUI → EmployeePresenceOperator → Caring Nanny / MiyuAttendances / Session → retour statut.
4. **Offboarding** : EmployeeOffboardingOperator.start → StrongFather (décision) → KindMother (WriteIntent désactivation) + EmployeeEquipmentOperator (récupération) + révocation mandats.

---

## 2. Patterns d'Intégration

### 2.1 Création de fiche employé

**Pattern :** WriteIntent Create + Mandate

```rust
// Pseudo-code Rust
pub struct CreateEmployeeIntent {
    pub name: String,
    pub company_id: CompanyId,
    pub department_id: Option<DepartmentId>,
    pub job_position: Option<String>,
    pub job_id: Option<JobId>,
    pub parent_id: Option<EmployeeId>,
    pub coach_id: Option<EmployeeId>,
    pub work_email: Option<String>,
    pub work_phone: Option<String>,
    pub work_mobile: Option<String>,
    pub work_contact_id: Option<PartnerId>,
    pub resource_calendar_id: Option<CalendarId>,
    pub tz: Option<String>,
    pub expense_approver_id: Option<UserId>,
    pub time_off_approver_id: Option<UserId>,
    pub timesheet_approver_id: Option<UserId>,
    pub attendance_approver_id: Option<UserId>,
    // ... autres champs work information, private (niveau 3), etc.
}

impl EmployeeOperator {
    pub async fn create_employee(
        &self,
        intent: CreateEmployeeIntent,
        mandate: Mandate,
    ) -> Result<Employee, EmployeeError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["employee.create"])?;

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "employee.create",
                resource: None,
            })
            .await?;
        if !permission.granted {
            return Err(EmployeeError::PermissionDenied);
        }

        let security_level = self.worry_sentinel
            .get_security_level(&intent)
            .await?;
        if security_level > mandate.max_security_level {
            return Err(EmployeeError::SecurityLevelExceeded);
        }

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "employee.create",
                context: &intent,
            })
            .await?;
        if !decision.approved {
            return Err(EmployeeError::DecisionRejected);
        }

        let write_intent = WriteIntent {
            entity_type: "hr.employee",
            operation: WriteOperation::Create,
            data: EmployeeData::from_intent(intent),
        };

        self.kind_mother.persist(write_intent).await
    }
}
```

### 2.2 Modification de fiche (avec self-edit)

**Pattern :** WriteIntent Update + Mandate (vérification self_edit ou Officer)

```rust
pub async fn update_employee(
    &self,
    employee_id: EmployeeId,
    patch: EmployeePatch,
    mandate: Mandate,
) -> Result<Employee, EmployeeError> {
    mandate.validate_flows(&["employee.update"])?;

    let employee = self.kind_mother.get::<Employee>(employee_id).await?;
    let is_self_edit = mandate.user_employee_id() == Some(employee_id);

    let capability = if is_self_edit {
        "employee.self_edit"
    } else {
        "employee.update"
    };

    let permission = self.master_butler
        .check_permission(PermissionRequest {
            operator: self.id(),
            capability,
            resource: Some(ResourceId::Employee(employee_id)),
        })
        .await?;
    if !permission.granted {
        return Err(EmployeeError::PermissionDenied);
    }

    if patch.contains_private_or_payroll() {
        let level = self.worry_sentinel.get_security_level(&patch).await?;
        if level > mandate.max_security_level {
            return Err(EmployeeError::SecurityLevelExceeded);
        }
    }

    let write_intent = WriteIntent {
        entity_type: "hr.employee",
        operation: WriteOperation::Update,
        entity_id: Some(employee_id),
        data: patch.into_data(),
    };

    self.kind_mother.persist(write_intent).await
}
```

### 2.3 Lecture présence

**Pattern :** Observation (Caring Nanny) — pas de WriteIntent

```rust
impl EmployeePresenceOperator {
    pub async fn get_presence_status(
        &self,
        employee_id: EmployeeId,
        mandate: Mandate,
    ) -> Result<PresenceStatus, PresenceError> {
        mandate.validate_flows(&["presence.get_status"])?;

        let config = self.get_presence_display_config().await?;
        let status = match config.mode {
            PresenceMode::Attendances => {
                self.miyu_attendances
                    .get_current_attendance(employee_id)
                    .await
                    .map(|a| PresenceStatus::from_attendance(a))
            }
            PresenceMode::UserStatus => {
                self.session_service
                    .get_user_status_for_employee(employee_id)
                    .await
                    .map(|s| PresenceStatus::from_session(s))
            }
            PresenceMode::Advanced => {
                self.advanced_presence
                    .compute(employee_id, &config)
                    .await
            }
        }?;

        Ok(status)
    }
}
```

### 2.4 Offboarding

**Pattern :** StrongFather (décision) + KindMother (désactivation) + Révocation mandats + Equipment recover

```rust
pub async fn start_offboarding(
    &self,
    employee_id: EmployeeId,
    mandate: Mandate,
) -> Result<OffboardingWorkflow, OffboardingError> {
    mandate.validate_operators(&[self.id()])?;
    mandate.validate_flows(&["offboarding.start"])?;

    let permission = self.master_butler
        .check_permission(PermissionRequest {
            operator: self.id(),
            capability: "offboarding.start",
            resource: Some(ResourceId::Employee(employee_id)),
        })
        .await?;
    if !permission.granted {
        return Err(OffboardingError::PermissionDenied);
    }

    let decision = self.strong_father
        .decide(DecisionRequest {
            action: "offboarding.start",
            context: &employee_id,
        })
        .await?;
    if !decision.approved {
        return Err(OffboardingError::DecisionRejected);
    }

    let workflow = OffboardingWorkflow::create(employee_id);
    self.kind_mother.persist(workflow.write_intent()).await?;

    self.employee_equipment.recover_all(employee_id, &mandate).await?;
    self.strong_father.revoke_mandates_for_employee(employee_id).await?;

    let deactivate_intent = WriteIntent {
        entity_type: "hr.employee",
        operation: WriteOperation::Update,
        entity_id: Some(employee_id),
        data: EmployeeData { active: false, ..default() },
    };
    self.kind_mother.persist(deactivate_intent).await?;

    Ok(workflow)
}
```

---

## 3. Gestion des Erreurs et Rollback

- **PermissionDenied** : Master Butler refuse → pas d’appel KindMother.
- **DecisionRejected** : StrongFather refuse → pas de WriteIntent.
- **SecurityLevelExceeded** : WorrySentinel → pas de persistance des données sensibles.
- **Rollback** : En cas d’échec après plusieurs WriteIntent (ex. offboarding), utiliser des compensations (ré-activation employé, annulation récupération équipements) ou transactions atomiques si le Kernel le supporte.

---

## 4. Intégration avec Kits existants

- **MiyuContacts** : work_contact_id, res.partner (adresses travail / privées).
- **MiyuAttendances** : pointages pour mode présence « Based on attendances ».
- **MiyuNotify** : Chatter / activités sur fiche employé (si mail.thread).
- **MiyuInvoice / Payroll** : pas dans ce document ; contrat futur pour paie, contrats, horaires (resource.calendar).

---

**Document** : Odoo Employees — Guide Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01
