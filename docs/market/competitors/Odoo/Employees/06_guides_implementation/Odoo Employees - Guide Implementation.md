# Odoo Employees — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Employees (Employés) dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates)
- Spécifications des crates Rust
- Schémas de données (Employee, Department, Skills, Presence)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyuhr/                              # EmployeeOperator + DepartmentOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── employee.rs                   # Modèle Employee, états actif/inactif
│   │   ├── department.rs                # Modèle Department, hiérarchie
│   │   ├── approvers.rs                 # Résolution approbateurs (expense, time_off, etc.)
│   │   ├── hierarchy.rs                 # parent_id, coach_id, manager via department
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuhr_skills/                       # EmployeeSkillsOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── skill_type.rs                # Skill Types, compétences, niveaux
│   │   ├── employee_skill.rs            # Attribution compétences aux employés
│   │   ├── resume_line.rs               # Expériences, éducation, certifications
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuhr_presence/                     # EmployeePresenceOperator (ou miyuattendances)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── presence_config.rs           # Mode (attendances, user_status, advanced)
│   │   ├── presence_status.rs           # Calcul et exposition statut
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuhr_equipment/                    # EmployeeEquipmentOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── equipment.rs                  # Équipement, attribution employé
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuhr_offboarding/                   # EmployeeOffboardingOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── offboarding_workflow.rs       # Étapes, checklist
│   │   ├── steps.rs                     # Désactivation, récupération, révocation
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyuhr_ui/                           # EmployeeUI (frontend selon stack)
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── employee_list.rs
    │   │   ├── employee_kanban.rs
    │   │   ├── employee_form.rs         # Onglets Général, Résumé, Work Info, Private, Payroll, Settings
    │   │   ├── department_views.rs
    │   │   └── config_views.rs           # Settings, Skill Types, Work Locations
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, Caring Nanny, TAMR)

**Kits existants :**
- `miyucontacts` : work_contact, partenaires, adresses
- `miyunotify` : Notifications, activités (chatter)
- `miyuattendances` : Pointages (si mode présence par pointage)
- `miyuvalidate` : Validation champs (email, téléphone, etc.)

---

## 2. Schémas de Données

### 2.1 Modèle Employee

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub id: EmployeeId,
    pub name: String,
    pub company_id: CompanyId,
    pub department_id: Option<DepartmentId>,
    pub job_position_text: Option<String>,
    pub job_id: Option<JobId>,
    pub parent_id: Option<EmployeeId>,
    pub coach_id: Option<EmployeeId>,
    pub user_id: Option<UserId>,
    pub work_contact_id: Option<PartnerId>,
    pub image_1920: Option<ImageId>,
    pub barcode: Option<String>,
    pub pin: Option<String>,
    pub active: bool,

    pub work_email: Option<String>,
    pub work_phone: Option<String>,
    pub work_mobile: Option<String>,
    pub address_id: Option<PartnerId>,
    pub work_location_id: Option<WorkLocationId>,
    pub resource_calendar_id: Option<CalendarId>,
    pub tz: Option<String>,
    pub expense_approver_id: Option<UserId>,
    pub time_off_approver_id: Option<UserId>,
    pub timesheet_approver_id: Option<UserId>,
    pub attendance_approver_id: Option<UserId>,
    pub remote_work_schedule: Option<RemoteWorkSchedule>, // lieu par jour

    pub address_home_id: Option<PartnerId>,
    pub private_email: Option<String>,
    pub private_phone: Option<String>,
    pub bank_account_id: Option<BankAccountId>,
    pub km_home_work: Option<Decimal>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub marital_status: Option<MaritalStatus>,
    pub spouse_name: Option<String>,
    pub spouse_birthdate: Option<Date>,
    pub children_count: Option<u32>,
    pub nationality_id: Option<CountryId>,
    pub identification_id: Option<String>,
    pub ssn_no: Option<String>,
    pub passport_no: Option<String>,
    pub gender: Option<Gender>,
    pub birthdate: Option<Date>,
    pub place_of_birth: Option<String>,
    pub country_of_birth_id: Option<CountryId>,
    pub non_resident: bool,
    pub certificate_level: Option<CertificateLevel>,
    pub field_of_study: Option<String>,
    pub school: Option<String>,
    pub visa_no: Option<String>,
    pub work_permit_no: Option<String>,
    pub visa_expiration: Option<Date>,
    pub work_permit_expiration: Option<Date>,
    pub work_permit_attachment_id: Option<AttachmentId>,

    pub legal_name: Option<String>,
    pub payslip_lang: Option<String>,
    pub registration_number: Option<String>,

    pub employee_type: EmployeeType,
    pub hourly_cost: Option<Decimal>,
    pub fleet_mobility_card: Option<String>,
}
```

### 2.2 Modèle Department

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub id: DepartmentId,
    pub name: String,
    pub manager_id: Option<EmployeeId>,
    pub parent_id: Option<DepartmentId>,
    pub company_id: CompanyId,
}
```

### 2.3 Modèle SkillType / EmployeeSkill / ResumeLine

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillType {
    pub id: SkillTypeId,
    pub name: String,
    pub skill_ids: Vec<SkillId>,
    pub level_ids: Vec<SkillLevelId>,
    pub default_level_id: Option<SkillLevelId>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeSkill {
    pub employee_id: EmployeeId,
    pub skill_type_id: SkillTypeId,
    pub skill_id: SkillId,
    pub level_id: SkillLevelId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeLine {
    pub id: ResumeLineId,
    pub employee_id: EmployeeId,
    pub title: String,
    pub line_type: ResumeLineType, // Experience, Education, Social Media, Internal Certification
    pub display_type: DisplayType, // Classic, Certification
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub description: Option<String>,
}
```

### 2.4 Présence (PresenceStatus)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceStatus {
    pub employee_id: EmployeeId,
    pub status: PresenceState, // Present, Absent, OutsideWorkingHours
    pub location_icon: Option<WorkLocationIcon>, // Home, Office, Other
    pub computed_at: DateTime,
}
```

---

## 3. API et Contrats

### 3.1 EmployeeOperator

- `create_employee(intent, mandate) -> Result<Employee>`
- `update_employee(employee_id, patch, mandate) -> Result<Employee>`
- `get_employee(employee_id, mandate) -> Result<Employee>`
- `list_employees(filter, mandate) -> Result<Vec<Employee>>`
- `deactivate_employee(employee_id, mandate) -> Result<()>`
- `get_approvers(employee_id, mandate) -> Result<EmployeeApprovers>`
- `get_hierarchy(employee_id, mandate) -> Result<EmployeeHierarchy>`

### 3.2 DepartmentOperator

- `create_department(intent, mandate) -> Result<Department>`
- `update_department(id, patch, mandate) -> Result<Department>`
- `list_departments(filter, mandate) -> Result<Vec<Department>>`
- `get_manager(department_id, mandate) -> Result<Option<EmployeeId>>`

### 3.3 EmployeePresenceOperator

- `get_presence_status(employee_id, mandate) -> Result<PresenceStatus>`
- `get_bulk_presence_status(employee_ids, mandate) -> Result<Vec<PresenceStatus>>`

### 3.4 EmployeeOffboardingOperator

- `start_offboarding(employee_id, mandate) -> Result<OffboardingWorkflow>`
- `complete_offboarding(workflow_id, mandate) -> Result<()>`
- `cancel_offboarding(workflow_id, mandate) -> Result<()>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (Bornage minimal)

**Objectif :** Fiches employés et départements utilisables par les autres Opérateurs (Expenses, Time Off, etc.).

- **Contenu :**
  - miyuhr : Employee (champs généraux + work contact + approbateurs), Department
  - CRUD employé et département
  - Hiérarchie (parent_id, department_id, manager via département)
  - Intégration KindMother (WriteIntent), Master Butler (permissions de base), StrongFather (décision create/update)
  - Pas de Private Information / Payroll en MVP
- **Livrables :** Crates miyuhr, API create/update/get/list, contrat d’équipe EmployeeService (EmployeeOperator, DepartmentOperator)
- **Critères d’acceptation :** Création fiche employé avec nom, société, département, manager, approbateurs ; liste et filtre par département

### Phase 2 — Données travail et présence

- **Contenu :**
  - Work Information complète (horaires, télétravail, lieu par jour si config activée)
  - EmployeePresenceOperator (modes attendances / user status)
  - Configuration Presence Display, Remote Work
  - Work Locations (référentiel)
- **Livrables :** miyuhr_presence (ou extension miyuattendances), config Work Locations, indicateurs présence sur Kanban
- **Critères d’acceptation :** Statut présence affiché ; télétravail configurable par employé

### Phase 3 — Compétences et référentiels

- **Contenu :**
  - EmployeeSkillsOperator : Skill Types, niveaux, compétences, attribution aux employés
  - Lignes de résumé (expérience, éducation, certifications)
  - Configuration Skill Types
- **Livrables :** miyuhr_skills, onglet Résumé sur formulaire employé
- **Critères d’acceptation :** Création type de compétence, attribution compétences à un employé, affichage résumé

### Phase 4 — Données privées et paie

- **Contenu :**
  - Private Information (adresse, banque, urgence, citoyenneté, permis de travail)
  - Champs Payroll (legal_name, payslip_lang, registration_number) + extension localisation si besoin
  - Niveau sécurité 3 (WorrySentinel) pour ces données
  - Option Employee Editing (self-edit)
- **Livrables :** Champs privés/paie dans miyuhr, permissions et mandats niveau 3
- **Critères d’acceptation :** Saisie données privées réservée aux droits élevés ; self-edit limité aux champs autorisés

### Phase 5 — Équipements et offboarding

- **Contenu :**
  - EmployeeEquipmentOperator : attribution, récupération
  - EmployeeOffboardingOperator : workflow, désactivation, récupération équipements, révocation mandats
  - Rapport retention (analyse départs)
- **Livrables :** miyuhr_equipment, miyuhr_offboarding, rapport retention
- **Critères d’acceptation :** Processus offboarding complet ; équipements récupérés ; mandats révoqués

---

## 5. Bornage Fonctionnel

### In scope (équivalent Odoo Employees)

- Fiche employé (général, work information, private, payroll, settings)
- Départements et hiérarchie
- Approbateurs par domaine (Expense, Time Off, Timesheet, Attendance)
- Présence (3 modes : attendances, user status, avancé)
- Compétences (Skill Types, niveaux, attribution), résumé (expériences, éducation, certifications)
- Work Locations, Remote Work (lieu par jour)
- Horaires (référentiel resource.calendar / Working Schedules — consommé depuis Payroll ou module dédié)
- Équipements (attribution, récupération)
- Offboarding (workflow, désactivation, récupération, révocation)
- Rapport de rétention
- Configuration (Presence Display, Skills Management, Remote Work, Employee Editing, Company Working Hours)
- Multi-société (company_id)

### Hors scope (dans ce bornage)

- Payroll (contrats, paie, bulletins) — module séparé
- Recruitment (hr.job) — réutilisation en tant que référentiel externe
- Appraisals (évaluations) — module séparé
- Planning (rôles) — réutilisation en tant que référentiel externe
- Fleet / Manufacturing (champs spécifiques) — intégration ultérieure si besoin

---

## 6. Risques et Mitigation

| Risque | Mitigation |
|--------|------------|
| Données privées / paie sensibles | Niveau sécurité 3, audit WorrySentinel, accès strictement mandaté |
| Dépendance Payroll pour horaires | Contrat clair : référentiel horaires fourni par Payroll ou module dédié ; miyuhr ne fait que consommer |
| Approbateurs multi-apps | Documenter droits requis par app ; Master Butler vérifie par capacité (expense.approve, time_off.approve, etc.) |
| Offboarding partiel (échec en cours) | Workflow avec étapes compensables ; journalisation ; reprise possible |

---

**Document** : Odoo Employees — Guide d'Implémentation avec Bornage  
**Version** : 1.0  
**Date** : 2026-02-01
