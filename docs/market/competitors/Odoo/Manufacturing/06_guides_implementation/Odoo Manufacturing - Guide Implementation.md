# Odoo Manufacturing — Guide d'Implémentation

## Contexte

Ce document fournit un **guide d'implémentation technique** pour développer l'équivalent Manufacturing (MRP) dans Miyukini, avec **bornage fonctionnel**, **spécifications techniques** et **plan de développement**.

**Références :**
- [Logique Métier](../00_logique_metier/Odoo%20Manufacturing%20-%20Logique%20Metier%20Complete.md)
- [Spécifications Opérateurs](../04_specifications_miyukini/Odoo%20Manufacturing%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Guide Intégration COG](../05_integration_cog/Odoo%20Manufacturing%20-%20Guide%20Integration%20COG.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique et structure des crates Rust
- Schémas de données (BOM, OF, WO, poste, gamme)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel et critères d'acceptation

**Hors scope :**
- Code source complet (sera dans les crates)
- Tests unitaires détaillés (sera dans les tests)

---

## 1. Architecture Technique

### 1.1 Structure des crates proposées

```
crates/
├── miyumanufacturing-bom/           # ManufacturingBOM
│   ├── src/
│   │   ├── lib.rs
│   │   ├── bom.rs
│   │   ├── bom_line.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumanufacturing-routing/       # ManufacturingRouting
│   ├── src/
│   │   ├── lib.rs
│   │   ├── routing.rs
│   │   ├── routing_workcenter.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumanufacturing-workcenter/    # ManufacturingWorkCenter
│   ├── src/
│   │   ├── lib.rs
│   │   ├── workcenter.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumanufacturing-order/         # ManufacturingOrder
│   ├── src/
│   │   ├── lib.rs
│   │   ├── production.rs
│   │   ├── moves.rs                 # Délégation Stock / WriteIntent
│   │   ├── confirm.rs
│   │   ├── close_backorder.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumanufacturing-workorder/     # ManufacturingWorkOrder
│   ├── src/
│   │   ├── lib.rs
│   │   ├── workorder.rs
│   │   ├── start_finish.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyumanufacturing-planning/      # ManufacturingPlanning
│   ├── src/
│   │   ├── lib.rs
│   │   ├── mps.rs
│   │   ├── propose.rs
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyumanufacturing-reporting/     # ManufacturingReporting
    ├── src/
    │   ├── lib.rs
    │   ├── oee.rs
    │   ├── delays.rs
    │   ├── allocation.rs
    │   └── admin_cell.rs
    └── Cargo.toml
```

Les Opérateurs d'interface (ManufacturingUI, ManufacturingShopFloor) peuvent vivre dans une crate front (web/desktop) ou dans miyukini-central selon l'architecture globale.

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Id, Logger, Clock
- `miyukini-central` : StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy

**Supply Chain / Stock :**
- `miyuinventory` ou équivalent : mouvements, emplacements, réservations (contrat WriteIntent)

**Produits :**
- `miyustore` ou équivalent : product, UoM, variantes

**Externes :**
- `serde`, `chrono`, `uuid`, `rust_decimal` (quantités), `thiserror`, `async-trait`

---

## 2. Schémas de données

### 2.1 BOM et lignes

```rust
// miyumanufacturing-bom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bom {
    pub id: BomId,
    pub product_id: ProductId,
    pub product_qty: Decimal,
    pub product_uom_id: UomId,
    pub bom_type: BomType, // Normal, Phantom, Kit
    pub routing_id: Option<RoutingId>,
    pub consumption: ConsumptionMode, // Strict, Flexible
    pub ready_to_produce: ReadyToProduce, // AllAvailable, Asap
    pub active: bool,
    pub company_id: CompanyId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomLine {
    pub id: BomLineId,
    pub bom_id: BomId,
    pub product_id: ProductId,
    pub product_qty: Decimal,
    pub product_uom_id: UomId,
    pub operation_id: Option<RoutingWorkcenterId>,
    pub sequence: u32,
}
```

### 2.2 Ordre de fabrication (Production)

```rust
// miyumanufacturing-order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Production {
    pub id: ProductionId,
    pub name: String,                    // Séquence
    pub product_id: ProductId,
    pub product_qty: Decimal,
    pub product_uom_id: UomId,
    pub bom_id: BomId,
    pub state: ProductionState,
    pub product_qty_produced: Decimal,
    pub date_planned_start: DateTime<Utc>,
    pub date_planned_finished: DateTime<Utc>,
    pub date_start: Option<DateTime<Utc>>,
    pub date_finished: Option<DateTime<Utc>>,
    pub picking_type_id: PickingTypeId,
    pub location_src_id: LocationId,
    pub location_dest_id: LocationId,
    pub origin: Option<String>,
    pub backorder_sequence: u32,
    pub company_id: CompanyId,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ProductionState {
    Draft,
    Confirmed,
    Progress,
    ToClose,
    Done,
    Cancel,
}
```

### 2.3 Ordre de travail (WorkOrder)

```rust
// miyumanufacturing-workorder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    pub id: WorkOrderId,
    pub production_id: ProductionId,
    pub workcenter_id: WorkCenterId,
    pub operation_id: RoutingWorkcenterId,
    pub name: String,
    pub state: WorkOrderState,
    pub qty_production: Decimal,
    pub qty_produced: Decimal,
    pub date_planned_start: DateTime<Utc>,
    pub date_planned_finished: DateTime<Utc>,
    pub date_start: Option<DateTime<Utc>>,
    pub date_finished: Option<DateTime<Utc>>,
    pub duration_expected: Option<Duration>,
    pub duration: Option<Duration>,
    pub blocked_by_workorder_ids: Vec<WorkOrderId>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WorkOrderState {
    Pending,
    Ready,
    Progress,
    Done,
    Cancel,
}
```

### 2.4 Poste de travail et gamme

```rust
// miyumanufacturing-workcenter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCenter {
    pub id: WorkCenterId,
    pub name: String,
    pub code: Option<String>,
    pub capacity: Decimal,
    pub time_efficiency: Decimal,
    pub costs_hour: Option<Decimal>,
    pub resource_calendar_id: Option<ResourceCalendarId>,
    pub company_id: CompanyId,
}

// miyumanufacturing-routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routing {
    pub id: RoutingId,
    pub name: String,
    pub company_id: CompanyId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingWorkcenter {
    pub id: RoutingWorkcenterId,
    pub routing_id: RoutingId,
    pub workcenter_id: WorkCenterId,
    pub name: String,
    pub sequence: u32,
    pub time_cycle_manual: Option<Duration>,
    pub time_mode: TimeMode,
    pub batch_size: Option<Decimal>,
    pub blocked_by_operation_ids: Vec<RoutingWorkcenterId>,
}
```

---

## 3. API et contrats

### 3.1 ManufacturingOrder

- `mo.create(input: CreateProductionInput) -> Result<ProductionId>`
- `mo.confirm(production_id: ProductionId) -> Result<()>`
- `mo.close(production_id: ProductionId, create_backorder: bool) -> Result<()>`
- `mo.backorder(production_id: ProductionId) -> Result<ProductionId>`
- `mo.cancel(production_id: ProductionId) -> Result<()>`
- `mo.get(production_id: ProductionId) -> Result<Production>`
- `mo.list(filters: ProductionFilters) -> Result<Vec<Production>>`

### 3.2 ManufacturingWorkOrder

- `wo.start(workorder_id: WorkOrderId) -> Result<()>`
- `wo.finish(workorder_id: WorkOrderId, qty_produced: Decimal, duration: Option<Duration>) -> Result<()>`
- `wo.get(workorder_id: WorkOrderId) -> Result<WorkOrder>`
- `wo.list_by_workcenter(workcenter_id: WorkCenterId, state: Option<WorkOrderState>) -> Result<Vec<WorkOrder>>`

### 3.3 ManufacturingBOM

- `bom.get(bom_id: BomId) -> Result<Bom>`
- `bom.get_by_product(product_id: ProductId, variant_id: Option<ProductId>) -> Result<Vec<Bom>>`
- `bom.create(input: CreateBomInput) -> Result<BomId>`
- `bom.update(bom_id: BomId, patch: BomPatch) -> Result<()>`

### 3.4 ManufacturingPlanning

- `mps.compute(filters: MpsFilters) -> Result<MpsView>`
- `mps.propose_mo(filters: MpsFilters) -> Result<Vec<MpsProposal>>`
- `mps.create_mo(proposal: MpsProposal) -> Result<ProductionId>`

---

## 4. Plan de développement par phases

### Phase 1 — MVP (BOM + OF simple, sans gamme)

- **Objectif :** Créer et confirmer des OF à partir d'une BOM sans opérations (pas de WO).
- **Crates :** miyumanufacturing-bom, miyumanufacturing-order, intégration Stock (moves raw + finished).
- **Fonctionnalités :** BOM + lignes ; OF draft → confirmed → done ; mouvements matières et produit fini ; réservation composants.
- **Hors scope :** Gammes, WO, postes, MPS, sous-traitance, backorder, unbuild.

**Durée estimée :** 4–6 semaines.

### Phase 2 — Gammes et ordres de travail

- **Objectif :** BOM avec gamme ; création WO à la confirmation ; démarrage/fin WO.
- **Crates :** miyumanufacturing-routing, miyumanufacturing-workcenter, miyumanufacturing-workorder.
- **Fonctionnalités :** Routing + opérations ; postes ; WO créés à la confirmation OF ; états WO (pending → ready → progress → done) ; dépendances entre WO ; consommation par opération (optionnel).
- **Hors scope :** MPS, Shop Floor UI, OEE, sous-traitance.

**Durée estimée :** 4–6 semaines.

### Phase 3 — Planification et rapports

- **Objectif :** MPS, backorder, rapports délais/allocation/coûts.
- **Crates :** miyumanufacturing-planning, miyumanufacturing-reporting.
- **Fonctionnalités :** Calcul besoins, proposition OF, création OF depuis MPS ; backorder à la clôture ; rapports délais, allocation, coûts OF ; OEE basique (disponibilité, performance, qualité).
- **Hors scope :** Sous-traitance, unbuild, by-products, IoT.

**Durée estimée :** 3–4 semaines.

### Phase 4 — Atelier et avancé

- **Objectif :** Interface Shop Floor, sous-traitance, unbuild, lots/séries.
- **Fonctionnalités :** Vue tableau de bord poste (WO par poste, démarrer/terminer, quantités/temps) ; Mandat poste ; sous-traitance (moves + lien Purchase si applicable) ; unbuild ; traçabilité lots/séries sur OF/WO/moves.
- **Durée estimée :** 4–6 semaines.

---

## 5. Bornage fonctionnel

### 5.1 In scope (prioritaire)

- BOM (normal, phantom, kit) et lignes ; consommation strict/flexible.
- OF : draft, confirmed, progress, to_close, done, cancel.
- Mouvements stock (raw, finished) créés à la confirmation ; réservation ; done à la clôture.
- WO (si gamme) : création à la confirmation, dépendances, démarrage/fin, quantités et temps.
- Postes de travail et gammes (routing + opérations).
- Backorder à la clôture partielle.
- MPS : calcul besoins, proposition OF, création OF validée par StrongFather.
- Rapports : délais, allocation, coûts OF ; OEE basique.
- Mandats : Standard Manufacturing, Poste (Shop Floor), Validation (clôture/backorder).

### 5.2 Out of scope (v1) ou optionnel

- PLM (révisions BOM) : module dédié.
- Qualité (contrôles qualité sur WO) : intégration module Quality si existant.
- Maintenance (demandes depuis WO) : intégration module Maintenance.
- IoT / postes connectés : module dédié ou extension.
- Sous-traitance avancée (dropship, resupply) : Phase 4 ou module.
- By-products et cost_share avancés : Phase 4.
- Multi-société / multi-entrepôt : à aligner avec Inventory.

---

## 6. Critères d'acceptation (MVP)

- Un utilisateur avec Mandat Manufacturing peut créer une BOM (produit fini + lignes composants) et un OF (produit, quantité, BOM).
- La confirmation de l'OF crée les mouvements stock (raw + finished) et les réserve (via KindMother / Stock).
- La clôture de l'OF marque les mouvements en done et met à jour les quants.
- Aucun mouvement n'est créé sans décision StrongFather (confirmation) et sans WriteIntent KindMother.
- Un utilisateur sans Mandat ne peut pas confirmer ni clôturer un OF.

---

## 7. Risques et mitigation

- **Complexité Stock :** S’appuyer sur le contrat Inventory existant ; ne pas dupliquer la logique de réservation. Mitigation : interface claire ManufacturingOrder ↔ Stock (WriteIntent).
- **Performance MPS :** Calcul des besoins sur gros volumes. Mitigation : indexation, cache, calcul incrémental.
- **Concurrence atelier :** Double démarrage/fin WO. Mitigation : optimistic locking (version sur WO), Mandat poste strict (un WO = un poste).

---

**Document** : Odoo Manufacturing — Guide d'Implémentation  
**Version** : 1.0  
**Date** : 2026-02-01
