# Odoo Time Off — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Time Off (Congés / Absences) dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust (ou modules)
- Schémas de données
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des crates (proposition)

```
crates/
├── miyutimeoff/                        # Service Time Off (ou miyuleave)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── leave_request.rs            # Modèle et workflow demande
│   │   ├── leave_allocation.rs         # Modèle et solde
│   │   ├── leave_type.rs               # Modèle type de congé
│   │   ├── leave_accrual.rs            # Plans d'acquisition
│   │   ├── leave_calendar.rs           # Jours fériés, jours obligatoires
│   │   ├── balance.rs                  # Calcul solde
│   │   ├── validation.rs               # Workflow validation (approve/refuse)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyutimeoff-ui/                     # TimeOffUI (optionnel, ou intégré central)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── views/
│   │   │   ├── my_time_off.rs           # Soldes + demandes
│   │   │   ├── overview_calendar.rs     # Calendrier
│   │   │   ├── management.rs            # Demandes à approuver
│   │   │   ├── allocations.rs           # Liste + batch wizard
│   │   │   └── config.rs                # Types, accrual, fériés, jours obligatoires
│   │   └── admin_cell.rs
│   └── Cargo.toml
```

**Dépendances :**
- miyukini-kernel, miyukini-central (Cores)
- miyuhr ou équivalent (employés, départements, approbateur)
- miyuclock / resource (calendrier, calcul jours)
- miyunotify (notifications)
- miyuinvoice (analytic/timesheet si lignes congé)
- Optionnel : miyupayroll (work entries), calendar (événements)

### 1.2 Dépendances principales

**Cores Miyukini :**
- miyukini-kernel : Id, Logger, Clock, Config, Lifecycle
- miyukini-central : StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, BondingBrother, Caring Nanny

**Kits existants :**
- miyuhr : Employés, départements, Work Information (approbateur)
- miyuclock : Dates, timezone
- miyunotify : Notifications
- resource/calendar : Calendrier de travail, jours fériés (ou miyutimeoff.leave_calendar)
- miyuinvoice : Comptabilité analytique (lignes timesheet congé)
- miyupayroll (si présent) : Work entry types, work entries

---

## 2. Schémas de Données

### 2.1 LeaveRequest (demande de congé)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequest {
    pub id: LeaveRequestId,
    pub employee_id: EmployeeId,
    pub leave_type_id: LeaveTypeId,
    pub company_id: CompanyId,
    pub department_id: Option<DepartmentId>,
    pub manager_id: Option<UserId>,
    pub date_from: DateTime,
    pub date_to: DateTime,
    pub number_of_days: Decimal,
    pub number_of_hours: Option<Decimal>,
    pub state: LeaveState, // Draft, Confirm, Validate1, Validated, Refused, Cancel
    pub request_unit_half: bool,
    pub request_unit_hours: bool,
    pub attachment_ids: Vec<AttachmentId>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LeaveState {
    Draft,
    Confirm,
    Validate1,
    Validated,
    Refused,
    Cancel,
}
```

### 2.2 LeaveAllocation (allocation)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveAllocation {
    pub id: LeaveAllocationId,
    pub name: Option<String>,
    pub employee_id: EmployeeId,
    pub leave_type_id: LeaveTypeId,
    pub company_id: CompanyId,
    pub department_id: Option<DepartmentId>,
    pub number_of_days: Decimal,
    pub number_of_hours: Option<Decimal>,
    pub date_from: Date,
    pub date_to: Date,
    pub state: AllocationState, // Draft, Confirm, Validate1, Validated, Refused, Cancel
    pub accrual: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.3 LeaveType (type de congé)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveType {
    pub id: LeaveTypeId,
    pub name: String,
    pub company_id: Option<CompanyId>,
    pub approval: Approval,           // NoValidation, ByOfficer, ByApprover, Both
    pub time_off_officer_id: Option<UserId>,
    pub requires_allocation: bool,
    pub extra_days_allowed: bool,
    pub allocation_approval: Approval,
    pub take_time_off_in: TimeOffUnit, // Day, HalfDay, Hours
    pub deduct_extra_hours: bool,
    pub public_holiday_included: bool,
    pub allow_attachment: bool,
    pub kind: LeaveKind,              // WorkedTime, Absence
    pub allow_negative_cap: bool,
    pub max_negative_days: Option<Decimal>,
    pub work_entry_type_id: Option<WorkEntryTypeId>,
    pub timesheet_project_id: Option<ProjectId>,
    pub timesheet_task_id: Option<TaskId>,
    pub calendar_event_type_id: Option<CalendarEventTypeId>,
    pub color: Option<u32>,
    pub cover_image: Option<String>,
    pub active: bool,
}
```

### 2.4 LeaveAccrualPlan, AccrualRule, PublicHoliday, MandatoryDay

- **LeaveAccrualPlan** : id, name, company_id, accrued_gain_time, carry_over_time, based_on_worked_time, milestone_transition
- **AccrualRule** : plan_id, employee_accrue (days/hours × amount × frequency), cap_accrued, start_accruing, carry_over, milestone_cap, carry_over_validity
- **PublicHoliday** : id, name, company_id, date_from, date_to, working_hours_id, work_entry_type_id
- **MandatoryDay** : id, name, company_id, department_ids, start_date, end_date, color

---

## 3. API et Contrats

### 3.1 LeaveRequestOperator

- `create_leave_request(intent, mandate) -> Result<LeaveRequest>`
- `submit_leave_request(leave_id, mandate) -> Result<LeaveRequest>`
- `approve_leave_request(leave_id, mandate) -> Result<LeaveRequest>`
- `refuse_leave_request(leave_id, reason?, mandate) -> Result<LeaveRequest>`
- `cancel_leave_request(leave_id, mandate) -> Result<LeaveRequest>`
- `get_leave_request(leave_id) -> Result<LeaveRequest>`
- `list_leave_requests(filters) -> Result<Vec<LeaveRequest>>`

### 3.2 LeaveAllocationOperator

- `create_allocation(intent, mandate) -> Result<LeaveAllocation>`
- `batch_create_allocations(intent, mandate) -> Result<Vec<LeaveAllocation>>`
- `validate_allocation(allocation_id, mandate) -> Result<LeaveAllocation>`
- `balance(employee_id, leave_type_id) -> Result<Balance>`
- `list_allocations(filters) -> Result<Vec<LeaveAllocation>>`

### 3.3 LeaveTypeOperator

- `create_leave_type(data, mandate) -> Result<LeaveType>`
- `update_leave_type(id, data, mandate) -> Result<LeaveType>`
- `get_leave_type(id) -> Result<LeaveType>`
- `list_leave_types(company_id?) -> Result<Vec<LeaveType>>`

### 3.4 LeaveAccrualOperator

- `create_plan(data, mandate) -> Result<LeaveAccrualPlan>`
- `add_rule(plan_id, rule, mandate) -> Result<AccrualRule>`
- `run_accrual(plan_id?, mandate) -> Result<Vec<LeaveAllocation>>` (cron/batch)
- `list_plans(company_id?) -> Result<Vec<LeaveAccrualPlan>>`

### 3.5 LeaveCalendarOperator

- `create_public_holiday(data, mandate) -> Result<PublicHoliday>`
- `create_mandatory_day(data, mandate) -> Result<MandatoryDay>`
- `list_public_holidays(company_id, date_from?, date_to?) -> Result<Vec<PublicHoliday>>`
- `list_mandatory_days(company_id, department_id?, date_from?, date_to?) -> Result<Vec<MandatoryDay>>`
- `calc_working_days(employee_id, date_from, date_to, leave_type?) -> Result<Decimal>` (exclut fériés, optionnellement jours obligatoires)

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (2–3 sprints)

**Objectif :** Demander un congé, valider/refuser, voir soldes et calendrier basique.

- **Modèles** : LeaveRequest, LeaveAllocation, LeaveType (champs essentiels)
- **Workflow** : draft → confirm → validate (une seule validation : By Officer ou No Validation)
- **Calcul** : nombre de jours entre date_from/date_to (calendrier employé, unité Day/HalfDay)
- **Solde** : somme allocations validées − somme congés validés par type et employé
- **UI** : Mes congés (soldes + liste demandes), formulaire demande, Overview calendrier (lecture)
- **Management** : liste demandes à approuver, boutons Approve/Refuse
- **Configuration** : types de congé (nom, unité, validation, requires allocation)
- **Intégrations** : HR (employé, approbateur), Notify (soumission, validation, refus)
- **Pas dans MVP** : double validation, accrual, jours fériés/jours obligatoires, Payroll/Timesheet/Calendar

### Phase 2 — Complet (2–3 sprints)

- **Double validation** : Validate1 → Validate (By Approver and Officer)
- **Jours fériés** : PublicHoliday, option "Public Holiday Included" sur type, exclusion du calcul
- **Jours obligatoires** : MandatoryDay, blocage des demandes sur ces dates
- **Allocations en lot** : wizard batch (employés, type, montant, période)
- **Rapports** : Time Off Summary, By Department, Analysis (filtres, export)
- **UI** : Configuration (fériés, jours obligatoires), rapports
- **Intégrations** : Calendar (événement à la validation si type configuré), Payroll (work entry type), Timesheet (projet/tâche)

### Phase 3 — Avancé (1–2 sprints)

- **Plans d'acquisition** : LeaveAccrualPlan, AccrualRule, cron/batch pour générer les allocations
- **Options avancées** : Extra days allowed, Negative cap, Deduct extra hours, Kind (Worked Time / Absence)
- **Annulation** : wizard Cancel Leave, recrédit solde, annulation work entries / événements / lignes timesheet
- **Multi-société** : company_id sur types, plans, fériés, jours obligatoires
- **Tests** : unitaires (calcul jours, solde, workflow), intégration (HR, Notify)

---

## 5. Bornage Fonctionnel

### MVP (Phase 1)

| Fonctionnalité | Inclus |
|----------------|--------|
| Types de congé (nom, unité, validation simple, requires allocation) | Oui |
| Demande de congé (dates, type, solde vérifié) | Oui |
| Workflow : draft → confirm → validate (ou refuse) | Oui |
| Une validation (No Validation ou By Time Off Officer) | Oui |
| Soldes par type et employé | Oui |
| Vue "Mes congés" (soldes + demandes) | Oui |
| Calendrier Overview (lecture) | Oui |
| Management (liste à approuver, Approve/Refuse) | Oui |
| Allocations manuelles (une par une) | Oui |
| Notifications (soumission, validation, refus) | Oui |
| Double validation | Non |
| Jours fériés / jours obligatoires | Non |
| Allocations en lot | Non |
| Accrual plans | Non |
| Payroll / Timesheet / Calendar | Non |
| Rapports (Summary, By Department, Analysis) | Non |

### Complet (Phase 2)

- Tout le MVP +
- Double validation, jours fériés, jours obligatoires, allocations en lot, rapports, intégrations Calendar/Payroll/Timesheet

### Avancé (Phase 3)

- Tout Complet +
- Accrual plans, options avancées (extra days, negative cap, deduct extra hours, kind), annulation complète, multi-société, tests

---

## 6. Risques et Points d'Attention

- **Calcul jours/heures** : dépendance forte au calendrier de travail (resource) et aux jours fériés ; bien définir l’API (calc_working_days) et les cas limites (timezone, demi-journées, heures).
- **Solde** : cohérence entre allocations et congés validés ; pas de double consommation (transactions, verrous ou versioning si besoin).
- **Validation** : approbateur manquant sur fiche employé si type "By Employee's Approver" ; comportement par défaut (refus, notification admin, délégation à l’officier) à trancher.
- **Intégrations** : Payroll et Timesheet peuvent être optionnels ; prévoir des feature flags ou modules optionnels pour ne pas bloquer le MVP.

---

## Références

- Odoo Time Off — Logique Métier, Parcours Utilisateur, UI/UX, Intégrations, Spécifications Opérateurs, Guide Intégration COG
- Odoo Project — Guide Implémentation (structure de document)
- Miyukini — Glossaire (WriteIntent, Mandat, Opérateur, Équipe)
