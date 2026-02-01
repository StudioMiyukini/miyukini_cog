# Odoo Project — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Project dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust
- Schémas de données
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyukini-project/                  # ProjectOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── project.rs                 # Modèle Project
│   │   ├── visibility.rs              # Gestion visibilité
│   │   ├── collaborator.rs            # Gestion collaborateurs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-task/                     # TaskOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── task.rs                    # Modèle Task
│   │   ├── state.rs                   # États et transitions
│   │   ├── dependencies.rs            # Gestion dépendances
│   │   ├── recurrence.rs              # Gestion récurrence
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-milestone/                # MilestoneOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── milestone.rs               # Modèle Milestone
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-project-update/           # ProjectUpdateOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── update.rs                  # Modèle Update
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-project-ui/               # ProjectUI
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── project_kanban.rs
    │   │   ├── project_list.rs
    │   │   ├── task_kanban.rs
    │   │   └── task_list.rs
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyucontacts` : Contacts/clients
- `miyuinvoice` : Compte analytique
- `miyuclock` : Dates et calendrier
- `miyunotify` : Notifications
- `miyuportal` : Accès portail
- `miyumedia` : Pièces jointes

---

## 2. Schémas de Données

### 2.1 Modèle Project

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
    pub sequence: u32,
    pub is_template: bool,
    
    // Client et entreprise
    pub partner_id: Option<PartnerId>,
    pub company_id: CompanyId,
    pub currency_id: CurrencyId,
    
    // Comptabilité analytique
    pub account_id: Option<AnalyticAccountId>,
    pub analytic_account_balance: Option<Decimal>,
    
    // Gestionnaire et favoris
    pub user_id: UserId,
    pub favorite_user_ids: Vec<UserId>,
    
    // Configuration
    pub label_tasks: String,
    pub color: Option<u32>,
    pub privacy_visibility: PrivacyVisibility,
    
    // Dates
    pub date_start: Option<Date>,
    pub date: Option<Date>,
    
    // Fonctionnalités
    pub allow_task_dependencies: bool,
    pub allow_milestones: bool,
    pub allow_recurring_tasks: bool,
    
    // Stages
    pub stage_id: Option<ProjectStageId>,
    pub type_ids: Vec<TaskTypeId>,
    
    // Tâches
    pub task_count: u32,
    pub open_task_count: u32,
    pub closed_task_count: u32,
    pub task_completion_percentage: f64,
    
    // Tags
    pub tag_ids: Vec<TagId>,
    
    // Mises à jour
    pub update_count: u32,
    pub last_update_id: Option<UpdateId>,
    pub last_update_status: UpdateStatus,
    
    // Jalons
    pub milestone_count: u32,
    pub milestone_count_reached: u32,
    pub milestone_progress: u32,
    
    // Partage
    pub collaborator_count: u32,
    
    // Métadonnées
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyVisibility {
    Followers,
    InvitedUsers,
    Employees,
    Portal,
}
```

### 2.2 Modèle Task

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
    pub sequence: u32,
    
    // Priorité
    pub priority: Priority,
    
    // Stage et état
    pub stage_id: Option<TaskTypeId>,
    pub state: TaskState,
    pub is_closed: bool,
    
    // Dates
    pub create_date: DateTime,
    pub write_date: DateTime,
    pub date_end: Option<DateTime>,
    pub date_assign: Option<DateTime>,
    pub date_deadline: Option<DateTime>,
    pub date_last_stage_update: Option<DateTime>,
    
    // Projet et hiérarchie
    pub project_id: Option<ProjectId>,
    pub display_in_project: bool,
    pub parent_id: Option<TaskId>,
    pub subtask_count: u32,
    pub closed_subtask_count: u32,
    pub subtask_completion_percentage: f64,
    pub subtask_allocated_hours: Decimal,
    
    // Assignation
    pub user_ids: Vec<UserId>,
    
    // Client
    pub partner_id: Option<PartnerId>,
    
    // Entreprise
    pub company_id: CompanyId,
    
    // Tags
    pub tag_ids: Vec<TagId>,
    
    // Temps alloué
    pub allocated_hours: Decimal,
    
    // Jalon
    pub milestone_id: Option<MilestoneId>,
    
    // Dépendances
    pub depend_on_ids: Vec<TaskId>,
    pub depend_on_count: u32,
    pub closed_depend_on_count: u32,
    pub dependent_ids: Vec<TaskId>,
    pub dependent_tasks_count: u32,
    
    // Récurrence
    pub recurring_task: bool,
    pub recurring_count: u32,
    pub recurrence_id: Option<RecurrenceId>,
    pub repeat_interval: u32,
    pub repeat_unit: RepeatUnit,
    pub repeat_type: RepeatType,
    pub repeat_until: Option<Date>,
    
    // Métriques temps
    pub working_hours_open: Decimal,
    pub working_hours_close: Decimal,
    pub working_days_open: Decimal,
    pub working_days_close: Decimal,
    
    // Métadonnées
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskState {
    InProgress,
    ChangesRequested,
    Approved,
    Done,
    Canceled,
    Waiting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,      // 0
    Medium,   // 1
    High,     // 2
    Urgent,   // 3
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatUnit {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatType {
    Forever,
    Until,
}
```

### 2.3 Modèle Milestone

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: MilestoneId,
    pub name: String,
    pub project_id: ProjectId,
    pub deadline: Option<Date>,
    pub is_reached: bool,
    pub reached_date: Option<Date>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.4 Modèle ProjectUpdate

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectUpdate {
    pub id: UpdateId,
    pub name: String,
    pub project_id: ProjectId,
    pub status: UpdateStatus,
    pub description: Option<String>,
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateStatus {
    OnTrack,
    AtRisk,
    OffTrack,
    OnHold,
    Done,
    ToDefine,
}
```

---

## 3. API et Contrats

### 3.1 ProjectOperator API

```rust
#[async_trait]
pub trait ProjectOperatorTrait {
    // CRUD
    async fn create_project(
        &self,
        intent: CreateProjectIntent,
        mandate: Mandate,
    ) -> Result<Project, ProjectError>;
    
    async fn get_project(
        &self,
        project_id: ProjectId,
        mandate: Mandate,
    ) -> Result<Project, ProjectError>;
    
    async fn update_project(
        &self,
        project_id: ProjectId,
        updates: ProjectUpdates,
        mandate: Mandate,
    ) -> Result<Project, ProjectError>;
    
    async fn delete_project(
        &self,
        project_id: ProjectId,
        mandate: Mandate,
    ) -> Result<(), ProjectError>;
    
    // Actions
    async fn share_project(
        &self,
        project_id: ProjectId,
        collaborator_ids: Vec<PartnerId>,
        limited_access: bool,
        mandate: Mandate,
    ) -> Result<Project, ProjectError>;
    
    async fn archive_project(
        &self,
        project_id: ProjectId,
        mandate: Mandate,
    ) -> Result<Project, ProjectError>;
    
    // Queries
    async fn list_projects(
        &self,
        filters: ProjectFilters,
        mandate: Mandate,
    ) -> Result<Vec<Project>, ProjectError>;
    
    async fn get_project_metrics(
        &self,
        project_id: ProjectId,
        mandate: Mandate,
    ) -> Result<ProjectMetrics, ProjectError>;
}
```

### 3.2 TaskOperator API

```rust
#[async_trait]
pub trait TaskOperatorTrait {
    // CRUD
    async fn create_task(
        &self,
        intent: CreateTaskIntent,
        mandate: Mandate,
    ) -> Result<Task, TaskError>;
    
    async fn get_task(
        &self,
        task_id: TaskId,
        mandate: Mandate,
    ) -> Result<Task, TaskError>;
    
    async fn update_task(
        &self,
        task_id: TaskId,
        updates: TaskUpdates,
        mandate: Mandate,
    ) -> Result<Task, TaskError>;
    
    async fn delete_task(
        &self,
        task_id: TaskId,
        mandate: Mandate,
    ) -> Result<(), TaskError>;
    
    // Actions
    async fn assign_task(
        &self,
        task_id: TaskId,
        user_ids: Vec<UserId>,
        mandate: Mandate,
    ) -> Result<Task, TaskError>;
    
    async fn close_task(
        &self,
        task_id: TaskId,
        state: TaskState,
        mandate: Mandate,
    ) -> Result<Task, TaskError>;
    
    async fn add_dependency(
        &self,
        task_id: TaskId,
        depend_on_id: TaskId,
        mandate: Mandate,
    ) -> Result<Task, TaskError>;
    
    async fn create_subtask(
        &self,
        parent_id: TaskId,
        intent: CreateTaskIntent,
        mandate: Mandate,
    ) -> Result<Task, TaskError>;
    
    // Queries
    async fn list_tasks(
        &self,
        filters: TaskFilters,
        mandate: Mandate,
    ) -> Result<Vec<Task>, TaskError>;
    
    async fn get_task_dependencies(
        &self,
        task_id: TaskId,
        mandate: Mandate,
    ) -> Result<Vec<Task>, TaskError>;
    
    async fn get_subtasks(
        &self,
        task_id: TaskId,
        mandate: Mandate,
    ) -> Result<Vec<Task>, TaskError>;
}
```

---

## 4. Plan de Développement par Phases

### Phase 1 : MVP (Minimum Viable Product)

**Objectif :** Fonctionnalités de base projets et tâches

**Fonctionnalités :**
- ✅ Création/modification projets
- ✅ Création/modification tâches
- ✅ Assignation tâches
- ✅ États tâches basiques (In Progress, Done, Canceled)
- ✅ Vue Liste projets/tâches
- ✅ Vue Formulaire projets/tâches

**Opérateurs :**
- ProjectOperator (MVP)
- TaskOperator (MVP)
- ProjectUI (MVP)

**Kits utilisés :**
- MiyuContacts (clients)
- MiyuClock (dates)
- MiyuNotify (notifications basiques)

**Durée estimée :** 4-6 semaines

### Phase 2 : Fonctionnalités Avancées

**Objectif :** Fonctionnalités avancées projets et tâches

**Fonctionnalités :**
- ✅ Gestion sous-tâches
- ✅ Gestion dépendances
- ✅ Gestion jalons
- ✅ Gestion récurrence
- ✅ Mises à jour projet
- ✅ Partage projet (collaborateurs)
- ✅ Vue Kanban projets/tâches
- ✅ Vue Calendrier tâches

**Opérateurs :**
- MilestoneOperator
- ProjectUpdateOperator
- ProjectCollaboratorOperator

**Durée estimée :** 6-8 semaines

### Phase 3 : Intégrations

**Objectif :** Intégrations avec autres modules

**Fonctionnalités :**
- ✅ Intégration comptabilité analytique (rentabilité)
- ✅ Intégration Sales (création projet depuis commande)
- ✅ Intégration Timesheet (saisie temps)
- ✅ Intégration Rating (feedback)
- ✅ Rapports et analyses

**Durée estimée :** 4-6 semaines

### Phase 4 : Optimisations et Polish

**Objectif :** Optimisations et améliorations UX

**Fonctionnalités :**
- ✅ Performance (cache, pagination)
- ✅ UX améliorée (drag & drop, quick create)
- ✅ Rapports avancés (graphiques, pivot)
- ✅ Templates projets/tâches
- ✅ Export/import

**Durée estimée :** 4-6 semaines

---

## 5. Bornage Fonctionnel

### 5.1 MVP (Phase 1)

**Inclus :**
- CRUD projets/tâches
- Assignation tâches
- États basiques
- Vues Liste/Formulaire

**Exclu :**
- Sous-tâches
- Dépendances
- Jalons
- Récurrence
- Partage projet
- Mises à jour projet
- Vues Kanban/Calendrier
- Intégrations avancées

### 5.2 Version Complète (Phase 4)

**Inclus :**
- Toutes fonctionnalités MVP
- Sous-tâches
- Dépendances
- Jalons
- Récurrence
- Partage projet
- Mises à jour projet
- Toutes vues (Kanban, Liste, Formulaire, Calendrier, Graphique, Pivot)
- Intégrations complètes
- Templates
- Rapports avancés

**Exclu (hors scope initial) :**
- Gestion temps avancée (nécessite Timesheet)
- Facturation projet (nécessite Accounting)
- Gestion ressources avancée (nécessite HR)

---

## 6. Considérations Techniques

### 6.1 Performance

**Optimisations :**
- Cache projets/tâches fréquemment accédés
- Pagination pour listes importantes
- Index base de données sur champs recherchés
- Calculs métriques en arrière-plan

### 6.2 Sécurité

**Mesures :**
- Validation WriteIntent avant persistance
- Vérification permissions Master Butler
- Vérification sécurité WorrySentinel
- Isolation cross-équipe
- Audit des actions importantes

### 6.3 Scalabilité

**Considérations :**
- Partitionnement par entreprise
- Archivage projets/tâches anciens
- Optimisation requêtes complexes
- Cache distribué (si applicable)

---

## 7. Tests

### 7.1 Tests Unitaires

**Couverture :**
- Modèles de données
- Calculs métriques
- Transitions d'état
- Validations

### 7.2 Tests d'Intégration

**Couverture :**
- Intégration avec Cores
- Intégration avec Kits
- WriteIntents
- Mandates

### 7.3 Tests End-to-End

**Couverture :**
- Parcours utilisateur complets
- Création projet → Création tâches → Assignation → Fermeture
- Partage projet → Accès collaborateur

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
