# Odoo Manufacturing — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Manufacturing (MRP) dans l'architecture COG Miyukini, en respectant la gouvernance, les WriteIntent et les Mandats de Permission.

**Références :**
- [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Manufacturing%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Odoo Manufacturing - Logique Métier](../00_logique_metier/Odoo%20Manufacturing%20-%20Logique%20Metier%20Complete.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG Manufacturing
- Patterns d'implémentation (WriteIntent, Mandats)
- Exemples de code (pseudo-code Rust)
- Gestion des erreurs et rollback
- Intégration Stock et Planification

**Hors scope :**
- Implémentation complète (voir Guide d'Implémentation)
- Spécifications UI/UX

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────────────┐
│  ManufacturingUI (bureau)     ManufacturingShopFloor (atelier)     │
│  Niveau sécurité: 1                        Niveau sécurité: 2       │
└───────────────────────┬─────────────────────────────┬───────────────┘
                        │                             │
    ┌───────────────────┼─────────────────────────────┼───────────────────┐
    │                   │                             │                   │
    ▼                   ▼                             ▼                   ▼
┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│Manufacturing│  │Manufacturing│  │Manufacturing│  │Manufacturing│  │Manufacturing│
│   Order     │  │  WorkOrder  │  │  Planning   │  │    BOM      │  │  Reporting  │
│    (S2)     │  │    (S2)     │  │    (S2)     │  │   (S2)      │  │   (S1-2)    │
└──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └─────────────┘
       │                │                │                │
       └────────────────┼────────────────┼────────────────┘
                        │                │
       ┌────────────────┼────────────────┼────────────────┐
       │                │                │                │
       ▼                ▼                ▼                ▼
┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐
│StrongFather │  │ KindMother  │  │Master Butler│  │WorrySentinel │
│ (Décision)  │  │(Persistance)│  │(Permissions)│  │  (Sécurité)   │
└─────────────┘  └──────┬──────┘  └─────────────┘  └─────────────┘
                        │
                        ▼
               ┌─────────────┐
               │  Stock /    │
               │  Inventory  │
               │  (moves)    │
               └─────────────┘
```

### 1.2 Flux de données standard

**Confirmation d'un OF :**
```
ManufacturingUI → ManufacturingOrder → Master Butler (permissions)
                → WorrySentinel (sécurité)
                → StrongFather (décision confirmation)
                → ManufacturingBOM / ManufacturingRouting (lecture BOM, gamme)
                → KindMother (WriteIntent : moves raw + finished, création WO si gamme)
                → Stock (moves réservés)
```

**Démarrage d'un WO (atelier) :**
```
ManufacturingShopFloor → ManufacturingWorkOrder → Master Butler (Mandat poste)
                       → StrongFather (décision démarrer si seuils)
                       → KindMother (WriteIntent : état WO, date_start)
```

**Clôture d'un OF :**
```
ManufacturingUI → ManufacturingOrder → StrongFather (décision clôturer / backorder)
                → KindMother (WriteIntent : moves done, état OF done)
                → Stock (quants mis à jour)
```

---

## 2. Patterns d'implémentation

### 2.1 Pattern : WriteIntent mouvements stock (KindMother)

**Principe :** Toute création ou modification de mouvement de stock lié à un OF passe par WriteIntent vers KindMother (ou vers l'Opérateur Stock gouverné par KindMother).

**Pseudo-code Rust :**

```rust
// Dans ManufacturingOrder
pub async fn confirm_production(
    &self,
    ctx: &OperatorContext,
    production_id: ProductionId,
) -> Result<(), ManufacturingError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "mo.confirm").await?;
    worry_sentinel::check_security_level(&mandate, 2).await?;

    let production = self.get_production(production_id).await?;
    let bom = manufacturing_bom::get(&production.bom_id).await?;
    strong_father::decide_confirm_mo(&mandate, &production).await?;

    // WriteIntent : mouvements matières
    for line in &bom.lines {
        let move_intent = StockMoveIntent {
            product_id: line.product_id,
            quantity: line.product_qty * production.product_qty,
            location_src: production.location_src_id,
            location_dest: production.location_dest_id,
            raw_material_production_id: Some(production_id),
            workorder_id: None, // ou selon opération
        };
        kind_mother::write_intent(ctx, move_intent).await?;
    }

    // WriteIntent : mouvement produit fini
    let finished_move = StockMoveIntent {
        product_id: production.product_id,
        quantity: production.product_qty,
        location_src: production.location_src_id,
        location_dest: production.location_dest_id,
        production_id: Some(production_id),
        ..
    };
    kind_mother::write_intent(ctx, finished_move).await?;

    // Création WO si gamme
    if let Some(routing_id) = bom.routing_id {
        self.create_work_orders(ctx, production_id, routing_id).await?;
    }

    kind_mother::write_intent(ctx, ProductionStateIntent {
        production_id,
        state: ProductionState::Confirmed,
    }).await?;

    Ok(())
}
```

### 2.2 Pattern : Mandat poste (Shop Floor)

**Principe :** ManufacturingShopFloor ne peut agir que sur les WO du poste assigné à l'utilisateur, avec un Mandat limité (démarrer, terminer, saisir quantités/temps).

**Pseudo-code Rust :**

```rust
// Dans ManufacturingWorkOrder
pub async fn start_workorder(
    &self,
    ctx: &OperatorContext,
    workorder_id: WorkOrderId,
) -> Result<(), ManufacturingError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "wo.start").await?;
    master_butler::check_workcenter_scope(&mandate, workorder_id).await?;

    let wo = self.get_workorder(workorder_id).await?;
    if wo.state != WorkOrderState::Ready {
        return Err(ManufacturingError::InvalidState);
    }
    strong_father::decide_start_wo(&mandate, &wo).await?;

    kind_mother::write_intent(ctx, WorkOrderStateIntent {
        workorder_id,
        state: WorkOrderState::Progress,
        date_start: Some(Clock::now()),
    }).await?;

    Ok(())
}
```

### 2.3 Pattern : Création OF depuis MPS (StrongFather)

**Principe :** La planification propose des OF ; la création effective est une décision StrongFather (ou délégation via Mandat avec seuils).

**Pseudo-code Rust :**

```rust
// Dans ManufacturingPlanning
pub async fn create_mo_from_mps(
    &self,
    ctx: &OperatorContext,
    proposal: MpsProposal, // product_id, quantity, date, origin
) -> Result<ProductionId, ManufacturingError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "mps.create_mo").await?;
    strong_father::decide_create_mo_from_mps(&mandate, &proposal).await?;

    manufacturing_order::create_production(ctx, CreateProductionInput {
        product_id: proposal.product_id,
        product_qty: proposal.quantity,
        bom_id: proposal.bom_id,
        origin: proposal.origin,
        date_planned_start: proposal.date_start,
        date_planned_finished: proposal.date_finished,
    }).await
}
```

---

## 3. Gestion des erreurs et rollback

- **Réservation impossible :** Si KindMother ou Stock refuse le WriteIntent (composants indisponibles), retourner une erreur explicite ; ne pas confirmer l'OF. L'utilisateur peut ajuster quantités ou dates.
- **Annulation OF :** StrongFather décide annulation → KindMother WriteIntent pour annuler les moves et passer l'OF en cancel. Libération des réservations (Stock).
- **Rollback partiel :** Si création WO échoue après création des moves, soit rollback des moves (transaction), soit OF en état « à corriger » selon politique.
- **Concurrence :** Optimistic locking sur OF et WO (version ou updated_at) pour éviter double clôture ou double démarrage.

---

## 4. Intégration avec Kits existants

- **MiyuInventory / LogisticsSteward :** Les mouvements créés par Manufacturing sont des stock.move ; le même contrat Stock (WriteIntent) s'applique. Pas de contournement.
- **Miyukini Sales :** Création d'OF avec origin = commande ; lien optionnel sale_order_id. Mandat entre Sales et Manufacturing pour « créer OF pour cette commande ».
- **Réapprovisionnement :** Règle « Manufacture » appelle ManufacturingOrder.create (ou Planning.create_mo_from_mps) avec Mandat ; StrongFather peut automatiser selon seuils (ex. créer OF si besoin < X jours).

---

## 5. Tests d'intégration COG

- **Confirmation OF :** Vérifier que les moves sont créés et réservés, WO créés si gamme, état OF = confirmed.
- **Démarrage WO :** Vérifier Mandat poste (refus si WO d'un autre poste), état WO = progress, date_start renseignée.
- **Clôture OF :** Vérifier moves en done, quants Stock mis à jour, état OF = done.
- **Backorder :** Vérifier nouvel OF créé avec reliquat, lien backorder_id, moves répartis.
- **Refus StrongFather :** Simuler refus (ex. seuil dépassé) et vérifier que l'OF n'est pas confirmé et qu'aucun move n'est créé.

---

**Document** : Odoo Manufacturing — Guide Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01
