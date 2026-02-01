# Odoo Inventory — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Inventory (Stock) d'Odoo dans Miyukini, avec **bornage fonctionnel**, **spécifications techniques**, et **plan de développement**.

**Références :**
- [Logique Métier](../00_logique_metier/Odoo%20Inventory%20-%20Logique%20Metier%20Complete.md)
- [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Inventory%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Guide Intégration COG](../05_integration_cog/Odoo%20Inventory%20-%20Guide%20Integration%20COG.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust (ou extension de miyuposinventory)
- Schémas de données (Picking, Move, MoveLine, Quant, Location, Warehouse)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)
- Critères d'acceptation

**Hors scope :**
- Implémentation complète du code (sera dans les crates)
- Tests unitaires détaillés (sera dans les tests)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

**Option A — Extension du crate existant miyuposinventory :**

Le crate `miyuposinventory` existe déjà (transfert, stock, count, valuation, etc.). L'équivalent Odoo Inventory complet peut soit **étendre** ce crate avec les concepts manquants (règles d'approvisionnement, multi-entrepôts avancé, putaway, etc.), soit coexister avec un module générique.

**Option B — Crates dédiés Inventory (équivalent Odoo Stock) :**

```
crates/
├── miyuinventory-picking/         # StockPicking Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── picking.rs              # Modèle Picking
│   │   ├── workflow.rs             # Confirm, Assign, Validate
│   │   ├── backorder.rs            # Backorders
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuinventory-move/             # StockMove / StockMoveLine
│   ├── src/
│   │   ├── lib.rs
│   │   ├── move.rs                 # Modèle Move
│   │   ├── move_line.rs            # Modèle MoveLine
│   │   ├── reservation.rs          # _action_assign, stratégies retrait
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuinventory-quant/            # StockQuant
│   ├── src/
│   │   ├── lib.rs
│   │   ├── quant.rs                # Modèle Quant
│   │   ├── availability.rs        # available_quantity, forecast
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuinventory-location/        # StockLocation, StockWarehouse
│   ├── src/
│   │   ├── lib.rs
│   │   ├── location.rs             # Modèle Location
│   │   ├── warehouse.rs            # Modèle Warehouse
│   │   ├── putaway.rs              # Règles de rangement
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuinventory-lot/              # StockLot
│   ├── src/
│   │   ├── lib.rs
│   │   ├── lot.rs                  # Modèle Lot/SN
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuinventory-rule/             # StockRule (règles d'approvisionnement)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── rule.rs                 # Modèle Rule
│   │   ├── route.rs                # Route
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyuinventory-inventory/        # Inventaire physique (déjà partiel dans miyuposinventory)
    ├── src/
    │   ├── lib.rs
    │   ├── session.rs              # Session inventaire
    │   ├── apply.rs                # Apply inventory
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel (Id, Logger, Clock)
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)
- `miyukini-admin` : Admin cell

**Kits existants :**
- `miyustore` : Produits (product_id, is_storable, tracking)
- `miyuposinventory` : POS Inventory (transfert, count, valuation, etc.) — réutilisation possible
- `miyulocale` : Unités de mesure (UoM)

**Externes :**
- `serde` : Sérialisation
- `chrono` : Dates
- `rust_decimal` : Quantités décimales
- `uuid` : Identifiants

---

## 2. Schémas de Données

### 2.1 Modèle Picking (Transfert)

```rust
// miyuinventory-picking/src/picking.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picking {
    pub id: PickingId,
    pub name: Option<String>,              // Référence (séquence)
    pub origin: Option<String>,            // Document source (SO001, PO002)
    pub state: PickingState,
    pub picking_type_id: PickingTypeId,
    pub location_id: LocationId,
    pub location_dest_id: LocationId,
    pub picking_type_code: PickingTypeCode,
    pub move_ids: Vec<MoveId>,
    pub move_line_ids: Vec<MoveLineId>,
    pub scheduled_date: DateTime<Utc>,
    pub date_deadline: Option<DateTime<Utc>>,
    pub date_done: Option<DateTime<Utc>>,
    pub partner_id: Option<PartnerId>,
    pub user_id: Option<UserId>,
    pub owner_id: Option<PartnerId>,
    pub move_type: MoveType,               // direct, one
    pub backorder_id: Option<PickingId>,
    pub return_id: Option<PickingId>,
    pub company_id: CompanyId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PickingState {
    Draft,
    Waiting,
    Confirmed,
    Assigned,
    Done,
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PickingTypeCode {
    Incoming,
    Outgoing,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoveType {
    Direct,   // Livraison partielle autorisée
    One,      // Tout en une fois
}
```

### 2.2 Modèle Move (Mouvement de Stock)

```rust
// miyuinventory-move/src/move.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMove {
    pub id: MoveId,
    pub reference: Option<String>,
    pub product_id: ProductId,
    pub product_uom_qty: Decimal,           // Demande
    pub quantity: Decimal,                  // Fait (somme move_lines)
    pub product_uom: UomId,
    pub location_id: LocationId,
    pub location_dest_id: LocationId,
    pub picking_id: Option<PickingId>,
    pub picking_type_id: Option<PickingTypeId>,
    pub state: MoveState,
    pub move_line_ids: Vec<MoveLineId>,
    pub date: DateTime<Utc>,
    pub date_deadline: Option<DateTime<Utc>>,
    pub procure_method: ProcureMethod,      // make_to_stock, make_to_order
    pub rule_id: Option<RuleId>,
    pub lot_ids: Vec<LotId>,
    pub owner_id: Option<PartnerId>,
    pub price_unit: Option<Decimal>,
    pub company_id: CompanyId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoveState {
    Draft,
    Waiting,
    Confirmed,
    PartiallyAvailable,
    Assigned,
    Done,
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcureMethod {
    MakeToStock,
    MakeToOrder,
}
```

### 2.3 Modèle MoveLine (Ligne de Mouvement)

```rust
// miyuinventory-move/src/move_line.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveLine {
    pub id: MoveLineId,
    pub move_id: MoveId,
    pub product_id: ProductId,
    pub product_uom_id: UomId,
    pub quantity: Decimal,
    pub location_id: LocationId,
    pub location_dest_id: LocationId,
    pub lot_id: Option<LotId>,
    pub package_id: Option<PackageId>,
    pub result_package_id: Option<PackageId>,
    pub owner_id: Option<PartnerId>,
    pub picked: bool,
}
```

### 2.4 Modèle Quant (Quantité en Stock)

```rust
// miyuinventory-quant/src/quant.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockQuant {
    pub id: QuantId,
    pub product_id: ProductId,
    pub location_id: LocationId,
    pub lot_id: Option<LotId>,
    pub package_id: Option<PackageId>,
    pub owner_id: Option<PartnerId>,
    pub quantity: Decimal,
    pub reserved_quantity: Decimal,
    pub in_date: DateTime<Utc>,
    pub inventory_quantity: Option<Decimal>,  // Comptée (inventaire)
    pub company_id: CompanyId,
}

impl StockQuant {
    pub fn available_quantity(&self) -> Decimal {
        self.quantity - self.reserved_quantity
    }
}
```

### 2.5 Modèle Location (Emplacement)

```rust
// miyuinventory-location/src/location.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub name: String,
    pub complete_name: String,               // Chemin hiérarchique
    pub location_id: Option<LocationId>,     // Parent
    pub usage: LocationUsage,
    pub company_id: Option<CompanyId>,
    pub warehouse_id: Option<WarehouseId>,
    pub removal_strategy_id: Option<RemovalStrategyId>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationUsage {
    Supplier,
    View,
    Internal,
    Customer,
    Inventory,
    Production,
    Transit,
}
```

### 2.6 Modèle Warehouse (Entrepôt)

```rust
// miyuinventory-location/src/warehouse.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub id: WarehouseId,
    pub name: String,
    pub code: String,
    pub company_id: CompanyId,
    pub view_location_id: LocationId,
    pub lot_stock_id: LocationId,
    pub wh_input_stock_loc_id: Option<LocationId>,
    pub wh_output_stock_loc_id: Option<LocationId>,
    pub reception_steps: ReceptionSteps,
    pub delivery_steps: DeliverySteps,
    pub in_type_id: PickingTypeId,
    pub out_type_id: PickingTypeId,
    pub route_ids: Vec<RouteId>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReceptionSteps {
    OneStep,
    TwoSteps,
    ThreeSteps,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliverySteps {
    ShipOnly,
    PickShip,
    PickPackShip,
}
```

---

## 3. API et Contrats

### 3.1 StockPicking API

```rust
// miyuinventory-picking/src/lib.rs

pub struct StockPickingOperator { /* ... */ }

impl StockPickingOperator {
    pub async fn create(
        &self,
        ctx: &OperatorContext,
        picking: PickingDraft,
    ) -> Result<PickingId, InventoryError>;
    
    pub async fn action_confirm(
        &self,
        ctx: &OperatorContext,
        picking_id: PickingId,
    ) -> Result<(), InventoryError>;
    
    pub async fn action_assign(
        &self,
        ctx: &OperatorContext,
        picking_id: PickingId,
    ) -> Result<(), InventoryError>;
    
    pub async fn button_validate(
        &self,
        ctx: &OperatorContext,
        picking_id: PickingId,
        create_backorder: Option<bool>,
    ) -> Result<ValidateResult, InventoryError>;
    
    pub async fn action_cancel(
        &self,
        ctx: &OperatorContext,
        picking_id: PickingId,
    ) -> Result<(), InventoryError>;
    
    pub async fn action_return(
        &self,
        ctx: &OperatorContext,
        picking_id: PickingId,
    ) -> Result<PickingId, InventoryError>;
    
    pub async fn get(
        &self,
        ctx: &OperatorContext,
        picking_id: PickingId,
    ) -> Result<Picking, InventoryError>;
    
    pub async fn list(
        &self,
        ctx: &OperatorContext,
        filters: PickingFilters,
    ) -> Result<Vec<Picking>, InventoryError>;
}
```

### 3.2 StockMove API

```rust
pub struct StockMoveOperator { /* ... */ }

impl StockMoveOperator {
    pub async fn _action_confirm(&self, ctx: &OperatorContext, move_id: MoveId) -> Result<(), InventoryError>;
    pub async fn _action_assign(&self, ctx: &OperatorContext, move_id: MoveId) -> Result<(), InventoryError>;
    pub async fn _action_done(&self, ctx: &OperatorContext, move_id: MoveId) -> Result<(), InventoryError>;
    pub async fn _do_unreserve(&self, ctx: &OperatorContext, move_id: MoveId) -> Result<(), InventoryError>;
    pub async fn get_available_quantity(
        &self,
        ctx: &OperatorContext,
        product_id: ProductId,
        location_id: LocationId,
        lot_id: Option<LotId>,
        package_id: Option<PackageId>,
        owner_id: Option<PartnerId>,
    ) -> Result<Decimal, InventoryError>;
}
```

### 3.3 StockQuant API

```rust
pub struct StockQuantOperator { /* ... */ }

impl StockQuantOperator {
    pub async fn get(
        &self,
        ctx: &OperatorContext,
        quant_id: QuantId,
    ) -> Result<StockQuant, InventoryError>;
    
    pub async fn search(
        &self,
        ctx: &OperatorContext,
        filter: QuantFilter,
    ) -> Result<Vec<StockQuant>, InventoryError>;
    
    pub async fn apply_inventory(
        &self,
        ctx: &OperatorContext,
        quant_id: QuantId,
        inventory_quantity: Decimal,
    ) -> Result<(), InventoryError>;
    
    pub async fn clear_inventory_quantity(
        &self,
        ctx: &OperatorContext,
        quant_id: QuantId,
    ) -> Result<(), InventoryError>;
}
```

### 3.4 StockInventory API (Inventaire physique)

```rust
pub struct StockInventoryOperator { /* ... */ }

impl StockInventoryOperator {
    pub async fn create_session(
        &self,
        ctx: &OperatorContext,
        location_id: LocationId,
        product_ids: Option<Vec<ProductId>>,
    ) -> Result<InventorySessionId, InventoryError>;
    
    pub async fn set_count(
        &self,
        ctx: &OperatorContext,
        quant_id: QuantId,
        inventory_quantity: Decimal,
    ) -> Result<(), InventoryError>;
    
    pub async fn apply_all(
        &self,
        ctx: &OperatorContext,
        session_id: InventorySessionId,
    ) -> Result<ApplyResult, InventoryError>;
    
    pub async fn clear_all(
        &self,
        ctx: &OperatorContext,
        session_id: InventorySessionId,
    ) -> Result<(), InventoryError>;
}
```

---

## 4. Plan de Développement par Phases

### 4.1 Phase 1 : MVP (Minimum Viable Product)

**Objectif :** Implémenter les fonctionnalités de base pour validation du concept.

**Fonctionnalités incluses :**
- [ ] Emplacements (Location) et hiérarchie (view, internal, supplier, customer)
- [ ] Entrepôt unique avec stock principal
- [ ] Transferts (Picking) : réception, livraison, transfert interne
- [ ] Mouvements (Move) et lignes (MoveLine)
- [ ] Quantités (Quant) : quantity, reserved_quantity, available_quantity
- [ ] Workflow : Draft → Confirmed → Assigned → Done
- [ ] Réservation simple (FIFO par défaut)
- [ ] Inventaire physique : saisie quantité comptée, application ajustement
- [ ] Intégration MiyuStore : qty_available sur produit

**Fonctionnalités exclues :**
- ❌ Lots et numéros de série
- ❌ Colis et emballages
- ❌ Règles d'approvisionnement (Stock Rule)
- ❌ Multi-entrepôts
- ❌ Putaway (règles de rangement)
- ❌ Backorders / Retours
- ❌ Cross-dock

**Durée estimée :** 6-8 semaines

**Critères d'acceptation :**
- Création d'un transfert (réception/livraison/interne)
- Confirmation et réservation
- Validation avec mise à jour des quants
- Inventaire physique avec application d'ajustement
- Quantité disponible correcte sur produit (MiyuStore)

### 4.2 Phase 2 : Fonctionnalités Essentielles

**Objectif :** Ajouter lots/SN, colis, backorders, retours.

**Fonctionnalités incluses :**
- [ ] Lots et numéros de série (StockLot)
- [ ] Traçabilité produit (lot, serial)
- [ ] Colis (StockPackage) et "Put in Pack"
- [ ] Backorders (livraison/réception partielle)
- [ ] Retours (action_return)
- [ ] Stratégies de retrait (FIFO, LIFO, FEFO)
- [ ] Règles de rangement (putaway) basiques

**Durée estimée :** 6-8 semaines

**Critères d'acceptation :**
- Création de lots à la réception
- Sélection de lots à la livraison
- Création de colis et association aux move_lines
- Backorder créé si validation partielle
- Retour créé depuis un picking done
- Putaway appliqué à la réception

### 4.3 Phase 3 : Multi-entrepôts et Règles

**Objectif :** Multi-entrepôts, routes, règles d'approvisionnement.

**Fonctionnalités incluses :**
- [ ] Multi-entrepôts (Warehouse)
- [ ] Étapes de réception (1, 2, 3 étapes)
- [ ] Étapes de livraison (ship only, pick+ship, pick+pack+ship)
- [ ] Routes (Stock Route)
- [ ] Règles d'approvisionnement (pull, push, MTO, MTS)
- [ ] Réapprovisionnement inter-entrepôts
- [ ] Inventaire cyclique par emplacement

**Durée estimée :** 8-10 semaines

**Critères d'acceptation :**
- Création de plusieurs entrepôts avec emplacements
- Routes et règles créées automatiquement
- Approvisionnement automatique (MTO) déclenché par demande
- Transfert inter-entrepôts fonctionnel
- Inventaire cyclique planifié par emplacement

### 4.4 Phase 4 : Intégrations et Optimisations

**Objectif :** Intégrations Sales/Purchase/Accounting, performance, UI.

**Fonctionnalités incluses :**
- [ ] Intégration Miyukini Sales : création livraison depuis commande
- [ ] Intégration Purchase : création réception depuis commande fournisseur
- [ ] Intégration Accounting : écritures de stock à validation move
- [ ] Interface StockUI (listes, formulaires, Kanban)
- [ ] Scan code-barres (produits, lots, emplacements)
- [ ] Rapports (valuation, mouvement, inventaire)
- [ ] Optimisations (cache quants, fusion moves, nettoyage)

**Durée estimée :** 8-10 semaines

---

## 5. Bornage Fonctionnel

### 5.1 MVP (Phase 1)

**Inclus :**
- ✅ Emplacements et entrepôt unique
- ✅ Transferts (réception, livraison, interne)
- ✅ Mouvements et quants
- ✅ Workflow Confirm → Assign → Validate
- ✅ Réservation FIFO
- ✅ Inventaire physique et ajustement
- ✅ Quantités sur produit (MiyuStore)

**Exclus :**
- ❌ Lots/SN
- ❌ Colis
- ❌ Règles d'approvisionnement
- ❌ Multi-entrepôts
- ❌ Backorders / Retours
- ❌ Intégrations Sales/Purchase/Accounting

### 5.2 Version Complète (Phases 1 à 4)

**Inclus :**
- ✅ Toutes fonctionnalités MVP
- ✅ Lots et numéros de série
- ✅ Colis et putaway
- ✅ Backorders et retours
- ✅ Multi-entrepôts, routes, règles
- ✅ Intégrations Sales, Purchase, Accounting
- ✅ StockUI et code-barres
- ✅ Rapports et optimisations

**Exclus (hors scope) :**
- ❌ Manufacturing (consommation composants, réception produits finis) — module séparé
- ❌ Cross-dock avancé (simplifié en phase 3 si nécessaire)
- ❌ IoT / terminaux dédiés (intégration future)

---

## 6. Correspondance Miyukini et Crate Existant

**Service Miyukini proposé :** `MiyukiniInventory` ou `MiyuInventory`

**Crate existant :** `miyuposinventory` — couvre déjà transfert, stock, count, valuation, purchase_order, etc. Pour un équivalent Odoo Stock complet, les options sont :

1. **Étendre miyuposinventory** : Ajouter règles d'approvisionnement, multi-entrepôts avancé, putaway, backorders/retours, et aligner les noms de concepts sur ce guide.
2. **Nouveau module miyuinventory** : Crates dédiés comme décrit ci-dessus, en réutilisant les patterns et en s’interfaçant avec KindMother ; coexistence avec miyuposinventory pour le contexte POS.

**Opérateurs proposés :** StockPicking, StockMove, StockQuant, StockLocation, StockWarehouse, StockLot, StockPackage, StockRule, StockInventory, StockUI

**Équipe d'Opérateurs :** InventoryService

---

**Date de création :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document d'analyse complète
