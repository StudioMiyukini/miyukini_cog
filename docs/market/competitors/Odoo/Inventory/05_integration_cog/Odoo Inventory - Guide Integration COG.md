# Odoo Inventory — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Inventory (Stock) dans l'architecture COG Miyukini, en respectant la gouvernance, les WriteIntent, et les Mandats de Permission.

**Références :**
- [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Inventory%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Odoo Inventory - Logique Métier](../00_logique_metier/Odoo%20Inventory%20-%20Logique%20Metier%20Complete.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Inventory
- Patterns d'implémentation (confirmation, réservation, validation, inventaire)
- Exemples de code (pseudo-code Rust)
- Gestion des WriteIntent
- Gestion des Mandats
- Gestion des erreurs et rollback

**Hors scope :**
- Implémentation complète (voir Guide d'Implémentation)
- Spécifications UI/UX

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
┌─────────────────────────────────────────────────────────────┐
│                    StockUI (Opérateur Interface)             │
│                    Niveau sécurité: 1                        │
└───────────────────────┬─────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┬───────────────┐
        │               │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐ ┌──────▼──────┐
│ StockPicking │ │ StockMove   │ │ StockQuant  │ │StockInventory│
│   (S2)       │ │   (S2)      │ │   (S2)      │ │   (S2)       │
└───────┬──────┘ └──────┬──────┘ └──────┬──────┘ └──────┬──────┘
        │               │               │               │
        └───────────────┼───────────────┴───────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌─────▼──────┐
│StrongFather  │ │ KindMother  │ │MasterButler│
│  (Mandats)   │ │ (Persistance)│ │(Permissions)│
└──────────────┘ └──────────────┘ └────────────┘
```

### 1.2 Flux de Données Standard

**Validation d'un transfert (picking) :**
```
StockUI → StockPicking → Master Butler (permissions)
       → WorrySentinel (sécurité) → StockMove._action_done()
       → KindMother (WriteIntent) → Mise à jour StockQuant
```

**Application d'un ajustement inventaire :**
```
StockUI → StockInventory → Master Butler (permissions)
       → WorrySentinel (sécurité) → StockQuant.apply_inventory
       → KindMother (WriteIntent) → Création StockMove d'ajustement
```

**Réservation de stock :**
```
StockPicking.action_assign() → StockMove._action_assign()
       → Lecture StockQuant (KindMother) → Création StockMoveLine
       → KindMother (WriteIntent réservation) → Mise à jour reserved_quantity
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : WriteIntent Validation Transfert vers KindMother

**Principe :** Toute validation de transfert (picking done) passe par WriteIntent vers KindMother pour mise à jour des quants.

**Pseudo-code Rust :**

```rust
// Dans StockPicking
pub async fn validate_picking(
    &self,
    ctx: &OperatorContext,
    picking_id: PickingId,
) -> Result<PickingId, InventoryError> {
    // 1. Vérification permissions (Master Butler)
    let mandate = ctx.mandate()?;
    master_butler::check_capability(
        &mandate,
        "picking.validate",
    ).await?;
    
    // 2. Vérification sécurité (WorrySentinel)
    worry_sentinel::check_security_level(
        &ctx.environment_id(),
        SecurityLevel::Sensitive,
    ).await?;
    
    // 3. Lecture du picking (KindMother)
    let picking = kind_mother::read::<Picking>(
        &ctx.environment_id(),
        picking_id.clone(),
    ).await?;
    
    if picking.state != PickingState::Assigned && picking.state != PickingState::Waiting {
        return Err(InventoryError::InvalidState(picking.state));
    }
    
    // 4. WriteIntent validation vers KindMother (moves done + quants mis à jour)
    let write_intent = WriteIntent {
        operation: WriteOperation::ValidatePicking {
            picking_id: picking_id.clone(),
            move_lines: picking.move_line_ids.clone(), // quantités faites
        },
        source: ctx.operator_id().clone(),
        mandate_id: mandate.id().clone(),
    };
    
    kind_mother::write(
        &ctx.environment_id(),
        write_intent,
    ).await?;
    
    Ok(picking_id)
}
```

### 2.2 Pattern : Réservation de Stock (Assign)

**Principe :** La réservation crée des move_lines et met à jour reserved_quantity sur les quants via KindMother.

**Pseudo-code Rust :**

```rust
// Dans StockMove
pub async fn action_assign(
    &self,
    ctx: &OperatorContext,
    move_id: MoveId,
) -> Result<(), InventoryError> {
    // 1. Lecture du move (KindMother)
    let move_ = kind_mother::read::<StockMove>(
        &ctx.environment_id(),
        move_id.clone(),
    ).await?;
    
    // 2. Calcul disponibilité (lecture quants KindMother)
    let available = self.get_available_quantity(
        ctx,
        move_.product_id(),
        move_.location_id(),
        move_.lot_id(),
        move_.package_id(),
        move_.owner_id(),
    ).await?;
    
    if available < move_.product_uom_qty() {
        return Err(InventoryError::InsufficientQuantity {
            required: move_.product_uom_qty(),
            available,
        });
    }
    
    // 3. Stratégie de retrait (FIFO, LIFO, etc.) → sélection des quants
    let quants_to_reserve = self.gather_quants(
        ctx,
        &move_,
        move_.product_uom_qty(),
    ).await?;
    
    // 4. WriteIntent réservation vers KindMother (move_lines + reserved_quantity)
    let write_intent = WriteIntent {
        operation: WriteOperation::ReserveMove {
            move_id: move_id.clone(),
            move_lines: quants_to_reserve.iter()
                .map(|q| MoveLineDraft {
                    product_id: move_.product_id(),
                    location_id: q.location_id(),
                    location_dest_id: move_.location_dest_id(),
                    quantity: q.quantity_to_reserve(),
                    lot_id: q.lot_id(),
                    package_id: q.package_id(),
                    owner_id: q.owner_id(),
                })
                .collect(),
        },
        source: ctx.operator_id().clone(),
        mandate_id: ctx.mandate()?.id().clone(),
    };
    
    kind_mother::write(
        &ctx.environment_id(),
        write_intent,
    ).await?;
    
    Ok(())
}
```

### 2.3 Pattern : Ajustement Inventaire (Apply Inventory)

**Principe :** L'application d'un inventaire physique crée des StockMove d'ajustement via WriteIntent vers KindMother.

**Pseudo-code Rust :**

```rust
// Dans StockInventory
pub async fn apply_inventory(
    &self,
    ctx: &OperatorContext,
    quant_id: QuantId,
    inventory_quantity: Decimal,
) -> Result<(), InventoryError> {
    // 1. Vérification permissions (Master Butler)
    let mandate = ctx.mandate()?;
    master_butler::check_capability(
        &mandate,
        "inventory.apply",
    ).await?;
    
    // 2. Lecture du quant (KindMother)
    let quant = kind_mother::read::<StockQuant>(
        &ctx.environment_id(),
        quant_id.clone(),
    ).await?;
    
    let diff = inventory_quantity - quant.quantity();
    if diff == Decimal::ZERO {
        return Ok(());
    }
    
    // 3. Décision StrongFather (optionnel, si seuils ou politique)
    let decision = strong_father::decide(
        &ctx.environment_id(),
        DecisionRequest::ApplyInventoryAdjustment {
            quant_id: quant_id.clone(),
            current_quantity: quant.quantity(),
            new_quantity: inventory_quantity,
            diff,
        },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 4. WriteIntent ajustement vers KindMother
            let write_intent = WriteIntent {
                operation: WriteOperation::ApplyInventory {
                    quant_id: quant_id.clone(),
                    inventory_quantity,
                    diff,
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            
            kind_mother::write(
                &ctx.environment_id(),
                write_intent,
            ).await?;
            
            Ok(())
        }
        Decision::Rejected { reason } => {
            Err(InventoryError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.4 Pattern : Création Transfert depuis Commande (Sales)

**Principe :** La création d'un picking de livraison depuis une commande client passe par BondingBrother (médiation) et WriteIntent vers KindMother.

**Pseudo-code Rust :**

```rust
// Dans BondingBrother (médiation Sales → Inventory)
pub async fn on_sale_order_confirmed(
    &self,
    ctx: &OperatorContext,
    order_id: SaleOrderId,
) -> Result<PickingId, MediationError> {
    // 1. Lecture commande (KindMother, via Sales)
    let order = kind_mother::read::<SaleOrder>(
        &ctx.environment_id(),
        order_id.clone(),
    ).await?;
    
    // 2. Demande Mandat pour InventoryService (StrongFather)
    let mandate = strong_father::request_mandate(
        &ctx.environment_id(),
        MandateRequest {
            operators: vec![
                OperatorId::from("stock.picking"),
                ctx.operator_id().clone(),
            ],
            capabilities: vec!["picking.create".into()],
            security_level: SecurityLevel::Sensitive,
            duration: Duration::hours(1),
        },
    ).await?;
    
    // 3. Création brouillon picking (WriteIntent KindMother)
    let picking_draft = PickingDraft {
        picking_type_code: PickingTypeCode::Outgoing,
        partner_id: order.partner_id(),
        location_id: order.warehouse_id().lot_stock_id(),
        location_dest_id: order.partner_id().property_stock_customer(),
        origin: order.name().clone(),
        move_ids: order.line_ids().iter()
            .filter(|l| l.product_id().is_storable())
            .map(|l| MoveDraft {
                product_id: l.product_id(),
                product_uom_qty: l.product_uom_qty(),
                location_id: order.warehouse_id().lot_stock_id(),
                location_dest_id: order.partner_id().property_stock_customer(),
            })
            .collect(),
    };
    
    let write_intent = WriteIntent {
        operation: WriteOperation::CreatePicking {
            picking: picking_draft,
        },
        source: ctx.operator_id().clone(),
        mandate_id: mandate.id().clone(),
    };
    
    let picking_id = kind_mother::write(
        &ctx.environment_id(),
        write_intent,
    ).await?;
    
    // 4. Confirmation picking (StockPicking.action_confirm)
    stock_picking::action_confirm(ctx, picking_id.clone()).await?;
    
    Ok(picking_id)
}
```

---

## 3. Gestion des Mandats de Permission

### 3.1 Obtention d'un Mandat InventoryService

**Pattern standard :**

```rust
// Dans StockPicking
pub async fn ensure_mandate(
    &self,
    ctx: &OperatorContext,
) -> Result<Mandate, InventoryError> {
    if let Some(mandate) = ctx.mandate() {
        if mandate.is_valid() && mandate.has_operator("stock.picking") {
            return Ok(mandate);
        }
    }
    
    let mandate = strong_father::request_mandate(
        &ctx.environment_id(),
        MandateRequest {
            operators: vec![
                ctx.operator_id().clone(),
                OperatorId::from("stock.picking"),
            ],
            capabilities: vec![
                "picking.create".into(),
                "picking.confirm".into(),
                "picking.validate".into(),
            ],
            security_level: SecurityLevel::Sensitive,
            duration: Duration::hours(8),
        },
    ).await?;
    
    Ok(mandate)
}
```

### 3.2 Utilisation d'un Mandat dans les Méthodes

**Pattern :** Chaque opération sensible (création, validation, inventaire) vérifie le mandat et les capacités via Master Butler.

```rust
pub async fn validate_picking(
    &self,
    ctx: &OperatorContext,
    picking_id: PickingId,
) -> Result<PickingId, InventoryError> {
    let mandate = self.ensure_mandate(ctx).await?;
    master_butler::check_capability_with_mandate(
        &mandate,
        "picking.validate",
    ).await?;
    
    // ... implémentation
}
```

---

## 4. Gestion des Erreurs et Rollback

### 4.1 Pattern : Rollback sur Erreur

**Principe :** En cas d'erreur après WriteIntent (ex. validation partielle), KindMother gère le rollback ou la compensation (backorder).

**Pseudo-code Rust :**

```rust
pub async fn validate_picking_with_backorder(
    &self,
    ctx: &OperatorContext,
    picking_id: PickingId,
    create_backorder: bool,
) -> Result<ValidateResult, InventoryError> {
    let write_intent = WriteIntent {
        operation: WriteOperation::ValidatePickingPartial {
            picking_id: picking_id.clone(),
            create_backorder,
        },
        source: ctx.operator_id().clone(),
        mandate_id: ctx.mandate()?.id().clone(),
    };
    
    match kind_mother::write(
        &ctx.environment_id(),
        write_intent,
    ).await {
        Ok(result) => Ok(result),
        Err(e) => {
            // KindMother a effectué rollback automatique
            Err(InventoryError::WriteFailed(e))
        }
    }
}
```

### 4.2 Types d'Erreurs Gouvernance

**Types d'erreurs :**

```rust
#[derive(Debug, Error)]
pub enum InventoryError {
    #[error("Rejected by governance: {0}")]
    RejectedByGovernance(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Insufficient quantity: required {required}, available {available}")]
    InsufficientQuantity { required: Decimal, available: Decimal },
    
    #[error("Invalid state: {0:?}")]
    InvalidState(PickingState),
    
    #[error("Write failed: {0}")]
    WriteFailed(#[from] KindMotherError),
    
    #[error("Picking not found: {0}")]
    PickingNotFound(PickingId),
    
    #[error("Quant not found: {0}")]
    QuantNotFound(QuantId),
    
    // ... autres erreurs
}
```

---

## 5. Intégration avec les Kits Existants

### 5.1 MiyuStore (Product) — Quantités

**Pattern :** Les quantités disponibles sur produit sont calculées depuis StockQuant (KindMother) et exposées en lecture via InventoryService.

```rust
// Dans MiyuStore (extension produit) ou StockQuant
pub async fn compute_qty_available(
    ctx: &OperatorContext,
    product_id: ProductId,
    location_ids: &[LocationId],
) -> Result<Decimal, InventoryError> {
    let quants = kind_mother::search::<StockQuant>(
        &ctx.environment_id(),
        QuantFilter {
            product_id: Some(product_id),
            location_id: location_ids.to_vec(),
        },
    ).await?;
    
    let available: Decimal = quants.iter()
        .map(|q| q.quantity() - q.reserved_quantity())
        .sum();
    
    Ok(available)
}
```

### 5.2 MiyuPOSInventory (crate existant)

**Note :** Le crate `miyuposinventory` existe pour le contexte POS. Pour un module Inventory générique (équivalent Odoo Stock), les patterns ci-dessus s'appliquent ; l'implémentation peut soit étendre miyuposinventory, soit créer un module dédié (ex. miyuinventory) et réutiliser les concepts (transfert, move, quant, inventaire).

---

## 6. Conclusion

Le guide d'intégration COG pour Inventory couvre :

- **Architecture** : StockUI → StockPicking / StockMove / StockQuant / StockInventory → KindMother (WriteIntent)
- **Patterns** : Validation transfert, réservation, ajustement inventaire, création depuis commande
- **Mandats** : Obtention et utilisation pour opérations sensibles
- **Erreurs** : Rollback géré par KindMother, types d'erreurs gouvernance
- **Intégrations** : MiyuStore (quantités), Sales (création livraison), BondingBrother (médiation)

---

**Date de création :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document d'analyse complète
