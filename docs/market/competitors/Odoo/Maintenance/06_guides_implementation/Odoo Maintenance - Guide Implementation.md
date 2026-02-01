# Odoo Maintenance — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Maintenance dans Miyukini.

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

### 1.1 Structure des Crates (Proposition)

```
crates/
├── miyumaintenance-equipment/       # EquipmentOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── equipment.rs             # Modèle Equipment
│   │   ├── category.rs              # Lien catégorie
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumaintenance-request/         # MaintenanceRequestOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── request.rs               # Modèle MaintenanceRequest
│   │   ├── stage.rs                 # Workflow stages
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumaintenance-team/             # MaintenanceTeamOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── team.rs                  # Modèle MaintenanceTeam
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumaintenance-category/         # EquipmentCategoryOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── category.rs              # Modèle EquipmentCategory
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumaintenance-metrics/          # MaintenanceMetricsOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── metrics.rs               # MTBF, MTTR, Latest Failure, Estimated Next Failure
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyumaintenance-ui/               # MaintenanceUI
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── request_kanban.rs
    │   │   ├── request_form.rs
    │   │   ├── equipment_list.rs
    │   │   ├── equipment_form.rs
    │   │   ├── calendar.rs
    │   │   ├── teams_config.rs
    │   │   └── categories_config.rs
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyucontacts` : Contacts / fournisseurs
- `miyuclock` : Dates et calendrier
- `miyunotify` : Notifications, chatter, followers
- `miyumedia` : Pièces jointes (instructions PDF)

**Optionnels :**
- Intégration Manufacturing (work center, MO, WO) : crate ou module équivalent MRP
- Intégration HR (département, employé) : MiyuHR ou équivalent

---

## 2. Schémas de Données

### 2.1 Modèle Equipment

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub id: Uuid,
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
    pub description: Option<String>,
    // Product info
    pub vendor_id: Option<Uuid>,
    pub vendor_reference: Option<String>,
    pub model: Option<String>,
    pub serial_no: Option<String>,
    pub effective_date: Option<Date>,
    pub cost: Option<Decimal>,
    pub warranty_expiration_date: Option<Date>,
    // Metrics (editable only expected_mtbf; others computed)
    pub expected_mtbf: Option<f64>,
    pub mtbf: Option<f64>,
    pub mttr: Option<f64>,
    pub latest_failure_date: Option<Date>,
    pub estimated_next_failure_date: Option<Date>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.2 Modèle MaintenanceRequest

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceRequest {
    pub id: Uuid,
    pub name: String,
    pub create_uid: Uuid,
    pub request_date: Date,
    pub for_type: ForType,  // Equipment | WorkCenter
    pub equipment_id: Option<Uuid>,
    pub workcenter_id: Option<Uuid>,
    pub worksheet_template_id: Option<Uuid>,
    pub maintenance_type: MaintenanceType,  // Corrective | Preventive
    pub manufacturing_order_id: Option<Uuid>,
    pub workorder_id: Option<Uuid>,
    pub maintenance_team_id: Uuid,
    pub user_id: Option<Uuid>,
    pub schedule_date: Option<DateTime>,
    pub duration: Option<Duration>,
    pub block_workcenter: bool,
    pub priority: u8,  // 0-3
    pub stage_id: Uuid,
    pub description: Option<String>,
    pub instructions_pdf_id: Option<Uuid>,
    pub instructions_slide_url: Option<String>,
    pub instructions_text: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.3 Modèle MaintenanceTeam

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceTeam {
    pub id: Uuid,
    pub name: String,
    pub member_ids: Vec<Uuid>,
    pub company_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.4 Modèle EquipmentCategory

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentCategory {
    pub id: Uuid,
    pub name: String,
    pub user_id: Option<Uuid>,
    pub company_id: Uuid,
    pub alias_id: Option<Uuid>,
    pub comment: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.5 Stages (MaintenanceRequest)

```rust
pub const STAGE_NEW: &str = "new";
pub const STAGE_IN_PROGRESS: &str = "in_progress";
pub const STAGE_REPAIRED: &str = "repaired";
pub const STAGE_SCRAP: &str = "scrap";
```

---

## 3. API et Contrats

### 3.1 EquipmentOperator

- `create_equipment(intent, mandate) -> Result<Equipment>`
- `update_equipment(id, intent, mandate) -> Result<Equipment>`
- `read_equipment(id) -> Result<Equipment>`
- `list_equipments(filters) -> Result<Vec<Equipment>>`

### 3.2 MaintenanceRequestOperator

- `create_request(intent, mandate, requestor_id) -> Result<MaintenanceRequest>`
- `update_request(id, intent, mandate) -> Result<MaintenanceRequest>`
- `change_stage(id, new_stage_id, mandate) -> Result<MaintenanceRequest>`
- `read_request(id) -> Result<MaintenanceRequest>`
- `list_requests(filters) -> Result<Vec<MaintenanceRequest>>`

### 3.3 MaintenanceTeamOperator

- `create_team(intent, mandate) -> Result<MaintenanceTeam>`
- `update_team(id, intent, mandate) -> Result<MaintenanceTeam>`
- `read_team(id) -> Result<MaintenanceTeam>`
- `list_teams(filters) -> Result<Vec<MaintenanceTeam>>`

### 3.4 EquipmentCategoryOperator

- `create_category(intent, mandate) -> Result<EquipmentCategory>`
- `update_category(id, intent, mandate) -> Result<EquipmentCategory>`
- `read_category(id) -> Result<EquipmentCategory>`
- `list_categories(filters) -> Result<Vec<EquipmentCategory>>`

### 3.5 MaintenanceMetricsOperator

- `get_equipment_metrics(equipment_id, mandate) -> Result<EquipmentMetrics>`
- `recompute_equipment_metrics(equipment_id) -> Result<()>` (appelé après passage demande en Repaired/Done)

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (Équipements + Demandes + Workflow)

- **Équipements** : CRUD équipement, catégorie, équipe ; champs de base (nom, catégorie, équipe, technicien, lieu).
- **Demandes** : Création demande pour équipement uniquement (For = Equipment) ; type Corrective / Preventive ; équipe, responsable, date prévue, durée, priorité ; workflow stages (New, In Progress, Repaired, Scrap).
- **Métriques** : Expected MTBF éditable ; MTBF, MTTR, Latest Failure, Estimated Next Failure calculés et en lecture seule.
- **UI** : Liste / formulaire équipements ; Kanban + formulaire demandes ; configuration équipes et catégories.
- **Droits** : Equipment Manager (accès tous) ou Follower (création demande pour équipements suivis uniquement).

**Livrables :** Crates Equipment, Request, Team, Category, Metrics, UI ; intégration KindMother, StrongFather, Master Butler, WorrySentinel, MiyuNotify.

### Phase 2 — Calendrier + Instructions

- Calendrier des maintenances (Scheduled Date) ; popover détail ; sidebar techniciens.
- Instructions demande : PDF, lien (Google Slide), texte ; stockage pièces jointes (MiyuMedia).
- Option « Custom Maintenance Worksheets » (template feuille de travail) si besoin.

**Livrables :** Vue calendrier ; champs et stockage instructions ; paramètre worksheets.

### Phase 3 — Work Center et Manufacturing (optionnel)

- Champ Work Center sur équipement ; demande pour Work Center (For = Work Center) ; option Block Workcenter.
- Lien Manufacturing Order / Work Order sur demande.
- Vue « Work Centers » et onglet Equipment sur centre de travail (intégration module Manufacturing).

**Livrables :** Contrat d'équipe avec MiyuManufacturing (ou équivalent) ; champs workcenter_id, manufacturing_order_id, workorder_id, block_workcenter.

### Phase 4 — HR (optionnel)

- Champs Department / Employee sur équipement (Used By) ; intégration MiyuHR.

**Livrables :** Champs department_id, employee_id ; filtres / groupements par département / employé.

---

## 5. Bornage Fonctionnel

| Fonctionnalité | MVP | Phase 2 | Phase 3 | Phase 4 |
|----------------|-----|---------|---------|--------|
| CRUD Équipement | ✅ | ✅ | ✅ | ✅ |
| CRUD Catégorie | ✅ | ✅ | ✅ | ✅ |
| CRUD Équipe | ✅ | ✅ | ✅ | ✅ |
| Demande (Equipment) | ✅ | ✅ | ✅ | ✅ |
| Demande (Work Center) | ❌ | ❌ | ✅ | ✅ |
| Workflow stages | ✅ | ✅ | ✅ | ✅ |
| Métriques (MTBF, MTTR, etc.) | ✅ | ✅ | ✅ | ✅ |
| Calendrier maintenances | ❌ | ✅ | ✅ | ✅ |
| Instructions (PDF, lien, texte) | ❌ | ✅ | ✅ | ✅ |
| Block Workcenter / MO-WO | ❌ | ❌ | ✅ | ✅ |
| Used By (Department / Employee) | ❌ | ❌ | ❌ | ✅ |
| Follower = droit création demande | ✅ | ✅ | ✅ | ✅ |

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
