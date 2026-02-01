# Odoo Appraisals — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Appraisals (Évaluations) dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates)
- Spécifications des crates Rust
- Schémas de données (Appraisal, Goal, Plan, Template, EvaluationScale)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyuappraisals/                        # AppraisalOperator + cœur métier
│   ├── src/
│   │   ├── lib.rs
│   │   ├── appraisal.rs                   # Modèle Appraisal, états (Draft, Confirmed, Done)
│   │   ├── appraisal_plan.rs              # Plan (6m, 6m, 12m), automation
│   │   ├── evaluation_scale.rs           # Échelle de notation
│   │   ├── visibility.rs                  # Toggles visible to manager / employee
│   │   ├── confirm_done_reopen.rs        # Workflow confirm, mark as done, reopen
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuappraisals_templates/             # AppraisalTemplateOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── template.rs                   # Modèle Template (sections, questions)
│   │   ├── section.rs                    # Employee's Feedback / Manager's Feedback
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuappraisals_goals/                 # AppraisalGoalsOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── goal.rs                       # Goal (employee, manager, progress, deadline, tags)
│   │   ├── goal_tags.rs                  # Tags (External, Hard Skills, etc.)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuappraisals_360/                   # Appraisal360Operator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── feedback_request.rs          # Demande 360 (recipients, deadline)
│   │   ├── survey_360.rs                 # Lien survey / résultats (ou miyusurveys)
│   │   ├── dashboard.rs                 # Stats (Registered, Completed, Certified)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyuappraisals_ui/                    # AppraisalUI (frontend selon stack)
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── appraisal_dashboard.rs    # Cartes/liste, indicateur visible, activité
    │   │   ├── appraisal_form.rs         # Onglets Appraisal, Skills, Private Note
    │   │   ├── goals_views.rs            # Liste groupée par employé, formulaire goal
    │   │   └── config_views.rs           # Settings, Templates, Evaluation Scale, 360, Tags
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, TAMR)

**Kits existants :**
- `miyuhr` : Employee, Department, next_appraisal_date, skills (EmployeeSkillsOperator ou équivalent)
- `miyunotify` : Notifications (confirmation appraisal, Ask Feedback, réunion)
- `miyuvalidate` : Validation champs (dates, échelle)
- Optionnel : `miyusurveys` pour templates et 360 ; `miyuplanning` ou calendar pour Meeting

---

## 2. Schémas de Données

### 2.1 Modèle Appraisal

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appraisal {
    pub id: AppraisalId,
    pub employee_id: EmployeeId,
    pub manager_id: Option<EmployeeId>,
    pub department_id: Option<DepartmentId>,
    pub job_position: Option<String>,
    pub company_id: CompanyId,
    pub appraisal_date: Date,
    pub next_appraisal_date: Option<Date>,  // ou enum Ongoing
    pub template_id: TemplateId,
    pub state: AppraisalState,
    pub employee_feedback_visible_to_manager: bool,
    pub manager_feedback_visible_to_employee: bool,
    pub final_rating_id: Option<EvaluationScaleId>,
    pub private_note: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub done_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppraisalState {
    Draft,
    Confirmed,
    Done,
}
```

### 2.2 Modèle Goal

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub name: String,
    pub employee_id: EmployeeId,
    pub manager_id: Option<EmployeeId>,
    pub progress: GoalProgress,  // 0, 25, 50, 75, 100
    pub deadline: Date,
    pub tags: Vec<TagId>,
    pub description: Option<String>,
    pub is_done: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum GoalProgress { P0, P25, P50, P75, P100 }
```

### 2.3 Modèle AppraisalPlan

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppraisalPlan {
    pub id: AppraisalPlanId,
    pub company_id: CompanyId,
    pub first_appraisal_months: u32,   // ex. 6
    pub second_appraisal_months: u32,  // ex. 6
    pub recurring_appraisal_months: u32, // ex. 12
    pub automation_enabled: bool,
}
```

### 2.4 Modèle Template

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppraisalTemplate {
    pub id: TemplateId,
    pub name: String,
    pub company_id: Option<CompanyId>,
    pub department_id: Option<DepartmentId>,
    pub employee_sections: Vec<Section>,
    pub manager_sections: Vec<Section>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: SectionId,
    pub title: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: QuestionId,
    pub label: String,
    pub kind: QuestionKind,  // Text, LongText, Scale, etc.
}
```

### 2.5 Modèle EvaluationScale

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationScale {
    pub id: EvaluationScaleId,
    pub name: String,
    pub sequence: u32,
}
```

---

## 3. API et Contrats

### 3.1 AppraisalOperator (miyuappraisals)

| Méthode | Description | Mandat |
|---------|-------------|--------|
| `create_appraisal(intent, mandate)` | Création appraisal (draft) | appraisal.create |
| `confirm_appraisal(id, mandate)` | Passage confirmed ; notification employé | appraisal.confirm |
| `update_employee_feedback(id, answers, visible_to_manager, mandate)` | Réponses employé + toggle | appraisal.update_own |
| `update_manager_feedback(id, answers, visible_to_employee, mandate)` | Réponses manager + toggle | appraisal.update_manager |
| `set_final_rating(id, rating_id, private_note, mandate)` | Note finale + note privée | appraisal.rate |
| `mark_as_done(id, mandate)` | Clôture ; next_appraisal_date + sync skills | appraisal.done |
| `reopen(id, mandate)` | Réouverture (done → confirmed) | appraisal.reopen |
| `get_appraisal(id, mandate)` | Lecture (selon périmètre manager/employé) | appraisal.read |
| `list_appraisals(filters, mandate)` | Liste (manager : ses équipes ; employé : les siennes) | appraisal.read |

### 3.2 AppraisalGoalsOperator (miyuappraisals_goals)

| Méthode | Description | Mandat |
|---------|-------------|--------|
| `create_goal(intent, mandate)` | Création objectif | goal.create |
| `update_goal(id, progress, description, mandate)` | Mise à jour avancement | goal.update |
| `mark_goal_done(id, mandate)` | 100 % et terminé | goal.done |
| `list_goals_by_employee(employee_id, mandate)` | Liste objectifs d’un employé | goal.read |

### 3.3 AppraisalPlanOperator (miyuappraisals)

| Méthode | Description | Mandat |
|---------|-------------|--------|
| `get_plan(company_id, mandate)` | Lecture plan | plan.read |
| `update_plan(plan, mandate)` | Modification plan (mois, automation) | plan.update |
| `run_automation(mandate)` | Cron : création/confirmation appraisals selon plan | plan.run |

### 3.4 Appraisal360Operator (miyuappraisals_360)

| Méthode | Description | Mandat |
|---------|-------------|--------|
| `request_feedback(appraisal_id, recipients, message, deadline, mandate)` | Envoi demande 360 | 360.request |
| `get_results(survey_id, mandate)` | Résultats survey 360 | 360.read |
| `dashboard(mandate)` | Liste surveys + stats (Registered, Completed, etc.) | 360.read |

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (2–3 semaines)

- **Crates** : miyuappraisals (appraisal, plan, evaluation_scale), miyuappraisals_templates (template minimal), miyuappraisals_ui (dashboard + formulaire appraisal basique).
- **Fonctionnalités** : Création appraisal (draft) ; Confirm ; auto-évaluation (Employee's Feedback) et Manager's Feedback (sections simples) ; toggles visibilité ; Final Rating (échelle par défaut) ; Mark as Done ; Next Appraisal Date mise à jour (sans plan automatique).
- **Intégrations** : MiyuHR (employé, manager, next_appraisal_date) ; MiyuNotify (confirmation).
- **Hors scope MVP** : Goals, 360, Skills dans appraisal, Plan automation, Private Note (ou champ simple sans WorrySentinel avancé), Reopen.

### Phase 2 — Goals et Skills (1–2 semaines)

- **Crates** : miyuappraisals_goals.
- **Fonctionnalités** : CRUD goals (avancement, deadline, tags) ; Mark as Done ; onglet Skills dans l’appraisal (lecture depuis fiche employé ; mise à jour niveau + justification ; report sur fiche après Done).
- **Intégrations** : MiyuHR (EmployeeSkillsOperator ou équivalent).

### Phase 3 — Plan et Templates avancés (1 semaine)

- **Fonctionnalités** : Appraisal Plan (6m, 6m, 12m) ; Appraisals Automation (cron création/confirmation) ; mise à jour next_appraisal_date en masse si plan modifié ; templates par département ; Evaluation Scale personnalisable.
- **Intégrations** : Cron / scheduler pour automation.

### Phase 4 — 360 et Private Note (1–2 semaines)

- **Crates** : miyuappraisals_360.
- **Fonctionnalités** : Ask Feedback (destinataires, message, deadline) ; envoi email via MiyuNotify ; dashboard 360 (stats) ; See Results (ou lien MiyuSurveys). Private Note (onglet réservé managers ; WorrySentinel niveau 3).
- **Intégrations** : MiyuNotify ; MiyuSurveys ou stockage interne réponses.

### Phase 5 — Réunion et Reopen (1 semaine)

- **Fonctionnalités** : Création activité Meeting depuis appraisal (smart button Meetings / No Meeting) ; option vidéocall ; Reopen (Done → Confirmed) puis modifications et Mark as Done.
- **Intégrations** : MiyuPlanning ou calendar.

### Phase 6 — Analyse et rapports

- **Fonctionnalités** : Appraisal analysis (filtres par statut, « view only user's appraisals ») ; Skills evolution (rapport amélioration, identification compétences).
- **Intégrations** : Données déjà présentes ; vues et exports (PDF si besoin).

---

## 5. Bornage Fonctionnel

### MVP (Phase 1)

- Création et confirmation d’une appraisal.
- Remplissage Employee's Feedback et Manager's Feedback (sections simples).
- Toggles visible to manager / visible to employee.
- Final Rating (échelle par défaut).
- Mark as Done et mise à jour next_appraisal_date (calcul manuel ou simple règle 12 mois).
- Notification employé à la confirmation.

### Complet (Phases 1–6)

- Tout le workflow (draft → confirmed → done ; reopen).
- Goals (CRUD, Mark as Done).
- Skills dans l’appraisal (sync fiche employé après Done).
- Plan et automation (6m, 6m, 12m ; cron).
- Templates par département ; Evaluation Scale personnalisable.
- 360 (Ask Feedback, dashboard, résultats).
- Private Note (managers uniquement ; niveau sécurité 3).
- Réunion (Meeting) et Reopen.
- Analyse et Skills evolution.

---

## 6. Critères d'Acceptation (MVP)

- Un manager peut créer une appraisal (employé, date, template) et la confirmer ; l’employé reçoit une notification.
- L’employé peut remplir son feedback et le rendre visible au manager ; le manager peut remplir son feedback et le rendre visible à l’employé.
- Le manager peut attribuer une note finale et marquer l’appraisal comme Done ; next_appraisal_date sur la fiche employé est mise à jour.
- Les droits (manager voit ses équipes, employé voit ses appraisals) sont respectés (Master Butler).

---

## 7. Risques et Mitigation

| Risque | Mitigation |
|--------|------------|
| Dépendance forte à MiyuHR (skills, next_appraisal_date) | Contrat d’équipe clair ; API stables ; fallback lecture seule si MiyuHR absent |
| Templates complexes (Surveys) | MVP avec template minimal natif ; Phase 4 option MiyuSurveys |
| Données sensibles (Private Note, rating) | WorrySentinel niveau 3 ; audit des accès ; Mandat max_security_level |
| Cron automation (plan) | Documentation ; test en staging ; rollback possible (désactiver automation) |

---

**Document** : Odoo Appraisals — Guide d'Implémentation avec Bornage  
**Version** : 1.0  
**Date** : 2026-02-01
