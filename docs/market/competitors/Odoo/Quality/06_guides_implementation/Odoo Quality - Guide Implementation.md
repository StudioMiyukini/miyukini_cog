# Odoo Quality — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Quality dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust (ou modules)
- Schémas de données (QCP, QualityCheck, QualityAlert, QualityTeam, FailureLocation)
- API et contrats
- Plan de développement par phases (MVP → Inventory → Manufacturing → Shop Floor)
- Bornage fonctionnel

---

## 1. Architecture Technique

### 1.1 Structure des Crates (Proposition)

```
crates/
├── miyuquality-qcp/                    # QualityControlPointOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── qcp.rs                      # Modèle QualityControlPoint
│   │   ├── evaluate.rs                # Évaluation QCP applicables à un ordre
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuquality-check/                 # QualityCheckOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── check.rs                   # Modèle QualityCheck
│   │   ├── trigger.rs                 # Création auto depuis QCP (ordre)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuquality-alert/                 # QualityAlertOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── alert.rs                   # Modèle QualityAlert
│   │   ├── stage.rs                   # Workflow stages
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuquality-team/                  # QualityTeamOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── team.rs                    # Modèle QualityTeam
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuquality-failure-location/      # FailureLocationOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── failure_location.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuquality-metrics/               # QualityMetricsOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── metrics.rs                 # Conformité, causes, rapports
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyuquality-ui/                    # QualityUI
    ├── src/
    │   ├── lib.rs
    │   ├── views/
    │   │   ├── qcp_list.rs
    │   │   ├── qcp_form.rs
    │   │   ├── check_list.rs
    │   │   ├── check_form.rs
    │   │   ├── alert_kanban.rs
    │   │   ├── alert_form.rs
    │   │   ├── teams_config.rs
    │   │   ├── failure_locations_config.rs
    │   │   ├── templates_config.rs
    │   │   └── reports.rs
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyunotify` : Notifications, chatter, followers
- `miyumedia` : Pièces jointes (photos contrôles, templates)

**Optionnels :**
- Intégration Inventory (pickings) : MiyuInventory ou équivalent
- Intégration Manufacturing (MO, WO, Shop Floor) : MiyuManufacturing ou équivalent

---

## 2. Schémas de Données

### 2.1 Modèle QualityControlPoint

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityControlPoint {
    pub id: Uuid,
    pub title: String,
    pub company_id: Uuid,
    pub operation_ids: Vec<Uuid>,           // Receipt, Delivery, Manufacturing, etc.
    pub work_order_operation_id: Option<Uuid>,
    pub product_ids: Vec<Uuid>,
    pub category_ids: Vec<Uuid>,
    pub control_per: ControlPer,             // Operation | Product | Quantity
    pub partial_percentage: Option<f64>,
    pub control_frequency: ControlFrequency, // All | Randomly | Periodically
    pub random_percentage: Option<f64>,
    pub period_value: Option<u32>,
    pub period_unit: PeriodUnit,             // Days | Weeks | Months
    pub check_type: QualityCheckType,
    pub template_id: Option<Uuid>,
    pub team_id: Uuid,
    pub responsible_id: Option<Uuid>,
    pub instructions: Option<String>,
    pub message_if_failure: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.2 Modèle QualityCheck

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCheck {
    pub id: Uuid,
    pub control_point_id: Option<Uuid>,
    pub control_per: ControlPer,
    pub picking_id: Option<Uuid>,
    pub production_order_id: Option<Uuid>,
    pub workorder_id: Option<Uuid>,
    pub product_ids: Vec<Uuid>,
    pub lot_serial_id: Option<Uuid>,
    pub check_type: QualityCheckType,
    pub template_id: Option<Uuid>,
    pub team_id: Uuid,
    pub company_id: Uuid,
    pub state: CheckState,                  // Pending | Passed | Failed
    pub measure_value: Option<f64>,
    pub instructions: Option<String>,
    pub notes: Option<String>,
    pub processed_at: Option<DateTime>,
    pub processed_by: Option<Uuid>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.3 Modèle QualityAlert

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityAlert {
    pub id: Uuid,
    pub title: String,
    pub product_id: Option<Uuid>,
    pub product_variant_id: Option<Uuid>,
    pub work_center_id: Option<Uuid>,
    pub picking_id: Option<Uuid>,
    pub production_order_id: Option<Uuid>,
    pub team_id: Uuid,
    pub responsible_id: Option<Uuid>,
    pub tags: Vec<Uuid>,
    pub root_cause_id: Option<Uuid>,
    pub failure_location_id: Option<Uuid>,
    pub priority: u8,                       // 1-3
    pub stage_id: Uuid,
    pub description: Option<String>,
    pub corrective_actions: Option<String>,
    pub preventive_actions: Option<String>,
    pub company_id: Uuid,
    pub vendor_id: Option<Uuid>,
    pub date_assigned: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.4 Modèle QualityTeam

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTeam {
    pub id: Uuid,
    pub name: String,
    pub member_ids: Vec<Uuid>,
    pub company_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.5 Modèle FailureLocation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureLocation {
    pub id: Uuid,
    pub name: String,
    pub company_id: Option<Uuid>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.6 États et Types

```rust
pub enum ControlPer { Operation, Product, Quantity }
pub enum ControlFrequency { All, Randomly, Periodically }
pub enum PeriodUnit { Days, Weeks, Months }
pub enum QualityCheckType {
    Instructions,
    TakePicture,
    PrintLabel,
    PassFail,
    Measure,
    Worksheet,
    Spreadsheet,
    RegisterProduction,
}
pub enum CheckState { Pending, Passed, Failed }
```

---

## 3. API et Contrats

### 3.1 QualityControlPointOperator

- `create_qcp(intent, mandate) -> Result<QualityControlPoint>`
- `update_qcp(id, intent, mandate) -> Result<QualityControlPoint>`
- `read_qcp(id) -> Result<QualityControlPoint>`
- `list_qcps(filters) -> Result<Vec<QualityControlPoint>>`
- `evaluate_applicable_qcps(order_context) -> Result<Vec<QualityControlPoint>>`

### 3.2 QualityCheckOperator

- `create_check_manual(intent, mandate) -> Result<QualityCheck>`
- `on_order_created_or_confirmed(order_context, mandate) -> Result<Vec<QualityCheck>>`
- `process_check(check_id, result, measure_value, mandate) -> Result<QualityCheck>`
- `read_check(id) -> Result<QualityCheck>`
- `list_checks(filters) -> Result<Vec<QualityCheck>>`
- `list_checks_by_order(picking_id | production_order_id | workorder_id) -> Result<Vec<QualityCheck>>`

### 3.3 QualityAlertOperator

- `create_alert(intent, mandate) -> Result<QualityAlert>`
- `update_alert(id, intent, mandate) -> Result<QualityAlert>`
- `change_stage(id, new_stage_id, mandate) -> Result<QualityAlert>`
- `read_alert(id) -> Result<QualityAlert>`
- `list_alerts(filters) -> Result<Vec<QualityAlert>>`

### 3.4 QualityTeamOperator

- `create_team(intent, mandate) -> Result<QualityTeam>`
- `update_team(id, intent, mandate) -> Result<QualityTeam>`
- `read_team(id) -> Result<QualityTeam>`
- `list_teams(filters) -> Result<Vec<QualityTeam>>`

### 3.5 FailureLocationOperator

- `create_failure_location(intent, mandate) -> Result<FailureLocation>`
- `update_failure_location(id, intent, mandate) -> Result<FailureLocation>`
- `read_failure_location(id) -> Result<FailureLocation>`
- `list_failure_locations(filters) -> Result<Vec<FailureLocation>>`

### 3.6 QualityMetricsOperator

- `conformity_rate(filters, period) -> Result<ConformityRate>`
- `checks_by_status(filters) -> Result<Vec<CheckStatusCount>>`
- `alerts_by_cause(filters) -> Result<Vec<CauseCount>>`
- `failure_locations_report(filters) -> Result<Vec<FailureLocationReport>>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (QCP + Contrôles + Alertes + Équipes)

- **QCP** : CRUD ; opérations (Receipt, Delivery en priorité), produits/catégories, Control Per (Operation/Product/Quantity), Control Frequency (All), type (Instructions, Pass-Fail, Measure).
- **Contrôles** : Création automatique depuis QCP sur événement picking (si intégration Inventory) ; création manuelle ; traitement Pass/Fail/Validate.
- **Alertes** : CRUD ; Kanban stages ; champs titre, produit, picking, équipe, responsable, priorité, cause racine, description, actions correctives/préventives.
- **Équipes** : CRUD équipes qualité.
- **UI** : Liste/formulaire QCP, liste/formulaire Quality Checks, Kanban/formulaire Quality Alerts, configuration équipes.
- **Droits** : Quality User (contrôles, alertes), Quality Manager (QCP, équipes, config).

**Livrables :** Crates QCP, Check, Alert, Team, UI ; intégration KindMother, StrongFather, Master Butler, WorrySentinel, MiyuNotify.

### Phase 2 — Inventory (Pickings)

- Déclenchement création contrôles sur événements picking (réception, livraison, retour) selon QCP.
- Bouton « Quality Checks » sur picking (pop-up ou page) ; bouton « Quality Alert ».
- « # To Process » ou équivalent pour filtrer ordres avec contrôles.

**Livrables :** Contrat d'équipe avec MiyuInventory ; événements picking → QualityCheckOperator ; UI contexte picking.

### Phase 3 — Manufacturing (MO)

- QCP sur opération Manufacturing ; création contrôles à la confirmation MO.
- Bouton Quality Checks et Quality Alert sur MO.
- Types de contrôle : Worksheet, Spreadsheet (templates), Take a Picture, Print label, Register Production.

**Livrables :** Contrat d'équipe avec MiyuManufacturing ; événements MO → QualityCheckOperator ; UI contexte MO.

### Phase 4 — Work Orders et Shop Floor (optionnel)

- QCP avec **Work Order Operation** : contrôles créés pour une opération de travail précise.
- Traitement des contrôles dans le module Shop Floor (étape sur la carte work order).
- Création alerte depuis Shop Floor (menu ⋮ sur carte WO ; Product et Work Center pré-remplis).

**Livrables :** Intégration work order ; UI Shop Floor pour étape contrôle et alerte.

### Phase 5 — Failure Locations et Rapports

- CRUD Failure Locations ; champ cause racine / lieu sur alertes.
- QualityMetricsOperator : taux de conformité, contrôles par statut, alertes par cause, rapports par Failure Location.
- Tableaux de bord et rapports personnalisables.

**Livrables :** Crate FailureLocation ; QualityMetricsOperator ; vues rapports et configuration Failure Locations.

---

## 5. Bornage Fonctionnel

| Fonctionnalité | MVP | Phase 2 | Phase 3 | Phase 4 | Phase 5 |
|----------------|-----|---------|---------|---------|---------|
| CRUD QCP | ✅ | ✅ | ✅ | ✅ | ✅ |
| QCP opérations Stock | ❌ | ✅ | ✅ | ✅ | ✅ |
| QCP opération Manufacturing | ❌ | ❌ | ✅ | ✅ | ✅ |
| QCP Work Order Operation | ❌ | ❌ | ❌ | ✅ | ✅ |
| Contrôles auto (picking) | ❌ | ✅ | ✅ | ✅ | ✅ |
| Contrôles auto (MO) | ❌ | ❌ | ✅ | ✅ | ✅ |
| Contrôles auto (WO) | ❌ | ❌ | ❌ | ✅ | ✅ |
| Contrôles manuels | ✅ | ✅ | ✅ | ✅ | ✅ |
| Types Instructions, Pass-Fail, Measure | ✅ | ✅ | ✅ | ✅ | ✅ |
| Types Picture, Worksheet, Spreadsheet | ❌ | ❌ | ✅ | ✅ | ✅ |
| CRUD Alertes + Kanban | ✅ | ✅ | ✅ | ✅ | ✅ |
| CRUD Équipes | ✅ | ✅ | ✅ | ✅ | ✅ |
| Failure Locations | ❌ | ❌ | ❌ | ❌ | ✅ |
| Rapports / Métriques | ❌ | ❌ | ❌ | ❌ | ✅ |
| Message If Failure (création alerte) | ✅ | ✅ | ✅ | ✅ | ✅ |

---

**Document rédigé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
