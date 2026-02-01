# Odoo Time Off — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Time Off dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG
- Patterns WriteIntent et Mandates
- Exemples de code pseudo-Rust
- Gestion des gouvernances (validation, solde, intégrations)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
TimeOffUI → BondingBrother → LeaveRequestOperator / LeaveAllocationOperator / LeaveTypeOperator
                              → StrongFather (décision validation/refus)
                              → KindMother (WriteIntent demande, allocation, type)
                              → Master Butler (permissions poser/valider/configurer)
                              → WorrySentinel (sécurité)
                              → MiyuNotify (notifications)
```

### 1.2 Flux typiques

**Demande de congé (employé) :**
1. Intention utilisateur (dates, type) → TimeOffUI
2. Traduction → BondingBrother
3. Vérification solde → LeaveAllocationOperator.balance()
4. Vérification permissions → Master Butler
5. Création WriteIntent demande (draft) → KindMother
6. Soumission (confirm) → StrongFather (si No Validation → validate direct) sinon notification approbateur
7. À la validation : WriteIntent (validate) → KindMother ; notifications → MiyuNotify ; optionnel Calendar, Payroll, Timesheet

**Validation (approbateur) :**
1. Intention Approve/Refuse → TimeOffUI
2. BondingBrother → LeaveRequestOperator
3. Décision → StrongFather (allowed/denied)
4. Permission → Master Butler (valider/refuser)
5. WriteIntent (validate/refuse) → KindMother
6. Notification employé → MiyuNotify
7. Si validé : intégrations (Calendar, Payroll, Timesheet) selon configuration type

**Création allocation (officier) :**
1. Intention (employés, type, montant, période) → TimeOffUI
2. BondingBrother → LeaveAllocationOperator
3. Décision → StrongFather
4. Permission → Master Butler (Time Off Officer)
5. WriteIntent (allocation create + confirm) → KindMother
6. Optionnel validate direct si type "No Validation"
7. Notification employés (optionnel) → MiyuNotify

---

## 2. Patterns d'Intégration

### 2.1 Création demande de congé (employé)

**Pattern :** WriteIntent + Mandate + vérification solde

```rust
// Pseudo-code Rust
pub struct CreateLeaveRequestIntent {
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub date_from: DateTime,
    pub date_to: DateTime,
    pub request_unit_half: bool,  // demi-journées
    pub request_unit_hours: bool, // heures
    pub attachment_ids: Option<Vec<Uuid>>,
}

impl LeaveRequestOperator {
    pub async fn create_leave_request(
        &self,
        intent: CreateLeaveRequestIntent,
        mandate: Mandate,
    ) -> Result<LeaveRequest, TimeOffError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["leave_request.create"])?;

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "leave_request.create",
                resource: Some(Resource::Employee(intent.employee_id)),
            })
            .await?;
        if !permission.granted {
            return Err(TimeOffError::PermissionDenied);
        }

        let leave_type = self.leave_type_operator.get(intent.leave_type_id).await?;
        let number_of_days = self.calc_days(
            intent.employee_id,
            intent.date_from,
            intent.date_to,
            &leave_type,
        ).await?;

        let balance = self.leave_allocation_operator
            .balance(intent.employee_id, intent.leave_type_id).await?;
        if balance.available_days < number_of_days && !leave_type.extra_days_allowed && !leave_type.allow_negative_cap {
            return Err(TimeOffError::InsufficientBalance);
        }

        let write_intent = WriteIntent {
            entity_type: "hr.leave",
            operation: WriteOperation::Create,
            data: LeaveRequestData {
                employee_id: intent.employee_id,
                leave_type_id: intent.leave_type_id,
                date_from: intent.date_from,
                date_to: intent.date_to,
                number_of_days,
                state: LeaveState::Draft,
                request_unit_half: intent.request_unit_half,
                request_unit_hours: intent.request_unit_hours,
                attachment_ids: intent.attachment_ids,
            },
            security_level: SecurityLevel::S2,
        };

        let leave = self.kind_mother.submit(write_intent).await?;
        Ok(leave)
    }
}
```

### 2.2 Soumission et validation (workflow)

**Pattern :** StrongFather (décision) + KindMother (transition d'état) + MiyuNotify

```rust
pub async fn submit_leave_request(
    &self,
    leave_id: Uuid,
    mandate: Mandate,
) -> Result<LeaveRequest, TimeOffError> {
    let leave = self.kind_mother.get::<LeaveRequest>(leave_id).await?;
    if leave.state != LeaveState::Draft {
        return Err(TimeOffError::InvalidState);
    }

    let approval = self.leave_type_operator.get(leave.leave_type_id).await?.approval;
    let write_intent = match approval {
        Approval::NoValidation => {
            WriteIntent {
                entity_type: "hr.leave",
                operation: WriteOperation::Update,
                data: LeaveRequestData { state: LeaveState::Validated, ..leave.into() },
                security_level: SecurityLevel::S2,
            }
        }
        _ => {
            WriteIntent {
                entity_type: "hr.leave",
                operation: WriteOperation::Update,
                data: LeaveRequestData { state: LeaveState::Confirm, ..leave.into() },
                security_level: SecurityLevel::S2,
            }
        }
    };

    let updated = self.kind_mother.submit(write_intent).await?;
    if updated.state == LeaveState::Confirm {
        self.miyu_notify.notify_approvers(leave_id, NotifyEvent::LeaveRequestSubmitted).await?;
    }
    if updated.state == LeaveState::Validated {
        self.on_leave_validated(&updated).await?; // Calendar, Payroll, Timesheet
    }
    Ok(updated)
}

pub async fn approve_leave_request(
    &self,
    leave_id: Uuid,
    mandate: Mandate,
) -> Result<LeaveRequest, TimeOffError> {
    mandate.validate_flows(&["leave_request.approve"])?;
    let decision = self.strong_father
        .decide(DecisionRequest { action: "approve_leave", context: &leave_id })
        .await?;
    if !decision.allowed {
        return Err(TimeOffError::DecisionDenied);
    }

    let leave = self.kind_mother.get::<LeaveRequest>(leave_id).await?;
    let next_state = self.next_approval_state(&leave).await?; // validate1 or validate
    let write_intent = WriteIntent {
        entity_type: "hr.leave",
        operation: WriteOperation::Update,
        data: LeaveRequestData { state: next_state, ..leave.into() },
        security_level: SecurityLevel::S2,
    };
    let updated = self.kind_mother.submit(write_intent).await?;
    self.miyu_notify.notify_employee(leave.employee_id, NotifyEvent::LeaveRequestApproved).await?;
    if updated.state == LeaveState::Validated {
        self.on_leave_validated(&updated).await?;
    }
    Ok(updated)
}
```

### 2.3 Création allocation (batch)

**Pattern :** WriteIntent + StrongFather + KindMother

```rust
pub struct BatchAllocationIntent {
    pub employee_ids: Vec<Uuid>,
    pub leave_type_id: Uuid,
    pub number_of_days: Decimal,
    pub date_from: Date,
    pub date_to: Date,
}

pub async fn batch_create_allocations(
    &self,
    intent: BatchAllocationIntent,
    mandate: Mandate,
) -> Result<Vec<LeaveAllocation>, TimeOffError> {
    mandate.validate_flows(&["allocation.batch_create"])?;
    let decision = self.strong_father
        .decide(DecisionRequest { action: "batch_create_allocations", context: &intent })
        .await?;
    if !decision.allowed {
        return Err(TimeOffError::DecisionDenied);
    }

    let mut allocations = Vec::new();
    for employee_id in intent.employee_ids {
        let write_intent = WriteIntent {
            entity_type: "hr.leave.allocation",
            operation: WriteOperation::Create,
            data: LeaveAllocationData {
                employee_id,
                leave_type_id: intent.leave_type_id,
                number_of_days: intent.number_of_days,
                date_from: intent.date_from,
                date_to: intent.date_to,
                state: AllocationState::Draft,
            },
            security_level: SecurityLevel::S2,
        };
        let alloc = self.kind_mother.submit(write_intent).await?;
        allocations.push(alloc);
    }
    Ok(allocations)
}
```

### 2.4 Calcul solde et jours (sans WriteIntent)

**Pattern :** Lecture KindMother + calcul côté Opérateur (pas de décision, pas d'écriture)

```rust
pub async fn balance(&self, employee_id: Uuid, leave_type_id: Uuid) -> Result<Balance, TimeOffError> {
    let allocations = self.kind_mother
        .query::<LeaveAllocation>()
        .filter(employee_id, leave_type_id, AllocationState::Validated)
        .await?;
    let leaves = self.kind_mother
        .query::<LeaveRequest>()
        .filter(employee_id, leave_type_id, LeaveState::Validated)
        .await?;
    let total_allocated: Decimal = allocations.iter().map(|a| a.number_of_days).sum();
    let total_used: Decimal = leaves.iter().map(|l| l.number_of_days).sum();
    Ok(Balance {
        available_days: total_allocated - total_used,
        total_allocated,
        total_used,
    })
}
```

---

## 3. Intégrations post-validation (Calendar, Payroll, Timesheet)

**Pattern :** Après KindMother.submit(validate), appels gouvernés vers autres Opérateurs avec Mandat

```rust
async fn on_leave_validated(&self, leave: &LeaveRequest) -> Result<(), TimeOffError> {
    let leave_type = self.leave_type_operator.get(leave.leave_type_id).await?;
    if let Some(calendar_event_type_id) = leave_type.calendar_event_type_id {
        self.calendar_operator
            .create_event_from_leave(leave, calendar_event_type_id, self.mandate()).await?;
    }
    if let Some(work_entry_type_id) = leave_type.work_entry_type_id {
        self.payroll_operator
            .create_work_entries_for_leave(leave, work_entry_type_id, self.mandate()).await?;
    }
    if let (Some(project_id), Some(task_id)) = (leave_type.timesheet_project_id, leave_type.timesheet_task_id) {
        self.timesheet_operator
            .create_lines_for_leave(leave, project_id, task_id, self.mandate()).await?;
    }
    Ok(())
}
```

Chaque appel (Calendar, Payroll, Timesheet) s'appuie sur un Mandat de Permission autorisant l'Opérateur Time Off à demander la création d'événements / work entries / lignes analytic.

---

## 4. Révocation et annulation

- **Annulation congé** : WriteIntent (state = Cancel) → KindMother ; recrédit solde (lecture + pas d'écriture allocation, le solde est recalculé à la lecture) ; suppression ou annulation des work entries / événements / lignes timesheet selon contrats avec Payroll, Calendar, Timesheet.
- **Révocation Mandat** : si le Mandat est révoqué (fin de session, alerte WorrySentinel, etc.), les actions en cours (validation, création allocation) sont refusées ; pas de persistance sans Mandat valide.

---

## 5. Résumé des patterns

| Action | StrongFather | KindMother | Master Butler | MiyuNotify |
|--------|--------------|------------|---------------|------------|
| Créer demande | — | Create (draft) | create | — |
| Soumettre | — (ou auto-validate si No Validation) | Update (confirm/validate) | — | Approvers / Employee |
| Approuver/Refuser | Decide | Update (validate/refuse) | approve | Employee |
| Créer allocation | Decide (batch) | Create | Time Off Officer | Optionnel |
| Configurer type | Decide | Create/Update | configure | — |

---

## Références

- Glossaire Miyukini (WriteIntent, Mandat de Permission, KindMother, StrongFather)
- Odoo Time Off — Logique Métier, Spécifications Opérateurs Miyukini
- Odoo Project — Guide Intégration COG (structure de document)
