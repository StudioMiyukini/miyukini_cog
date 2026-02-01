# Odoo POS Restaurant — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités POS Restaurant dans l'architecture COG Miyukini : patterns WriteIntent/Mandats, exemples pseudo-Rust pour liaison table–ordre, transfert, cours, split et réservations.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d’intégration COG pour RestaurantService
- Patterns d’implémentation (Set Table, Transfer, Course, Split, Booking)
- Exemples de code (pseudo-code Rust)
- Intégration avec Kits existants (POS, Agenda)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
RestaurantUI → TableOrderBinding → FloorManager
             → OrderTransfer      → POS Order
             → CourseManager     → PreparationPrint
             → BillSplit         → POS Payment
             → RestaurantBooking → Miyukini Agenda (ressources)
             ↓
StrongFather (Décisions : assignation, transfert, split, réservation)
KindMother   (Persistance : floors, tables, bindings, courses, suborders)
Master Butler (Permissions)
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : Set Table (liaison ordre → table)

```rust
// Dans TableOrderBinding
pub async fn set_order_table(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
    table_id: TableId,
) -> Result<(), RestaurantError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "restaurant.binding.set_table").await?;
    worry_sentinel::check_security_level(ctx, SecurityLevel::Sensitive).await?;

    let table = floor_manager::get_table(ctx, table_id).await?;
    let order = pos_order::get(ctx, order_id).await?;

    let decision = strong_father::decide(
        ctx,
        DecisionRequest::AssignOrderToTable {
            order_id: &order_id,
            table_id: &table_id,
            table_state: table.state(),
        },
    ).await?;

    match decision {
        Decision::Approved => {
            let write_intent = WriteIntent {
                operation: WriteOperation::BindOrderToTable { order_id, table_id },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            kind_mother::write(ctx.environment_id(), write_intent).await?;
            Ok(())
        }
        Decision::Rejected { reason } => Err(RestaurantError::RejectedByGovernance(reason)),
    }
}
```

### 2.2 Pattern : Transfer Order to Table

```rust
// Dans OrderTransfer
pub async fn transfer_to_table(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
    target_table_id: TableId,
) -> Result<(), RestaurantError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "restaurant.transfer.to_table").await?;

    let target_table = floor_manager::get_table(ctx, target_table_id).await?;
    let binding = table_order_binding::get(ctx, order_id).await?;

    let decision = strong_father::decide(
        ctx,
        DecisionRequest::TransferOrder {
            order_id: &order_id,
            from_table_id: &binding.table_id(),
            to_table_id: &target_table_id,
            target_occupied: target_table.has_open_orders(),
        },
    ).await?;

    match decision {
        Decision::Approved => {
            let write_intent = WriteIntent {
                operation: WriteOperation::TransferOrder {
                    order_id,
                    new_table_id: target_table_id,
                    merge: target_table.has_open_orders(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            kind_mother::write(ctx.environment_id(), write_intent).await?;
            Ok(())
        }
        Decision::Rejected { reason } => Err(RestaurantError::RejectedByGovernance(reason)),
    }
}
```

### 2.3 Pattern : Fire Course (envoi cours vers préparation)

```rust
// Dans CourseManager
pub async fn fire_course(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
    course_number: u32,
) -> Result<(), RestaurantError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "restaurant.course.fire").await?;

    let order = pos_order::get(ctx, order_id).await?;
    let course_lines = order.lines_for_course(course_number);

    let write_intent = WriteIntent {
        operation: WriteOperation::MarkCourseSent { order_id, course_number },
        source: ctx.operator_id().clone(),
        mandate_id: mandate.id().clone(),
    };
    kind_mother::write(ctx.environment_id(), write_intent).await?;

    preparation_print::send(ctx, order_id, course_number, &course_lines).await?;
    Ok(())
}
```

### 2.4 Pattern : Split Bill (création sous-commande)

```rust
// Dans BillSplit
pub async fn split_suborder(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
    line_ids: Vec<LineId>,
) -> Result<SubOrderId, RestaurantError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "restaurant.split.suborder").await?;

    let order = pos_order::get(ctx, order_id).await?;
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::SplitOrder {
            order_id: &order_id,
            line_ids: &line_ids,
            amount: order.amount_for_lines(&line_ids),
        },
    ).await?;

    match decision {
        Decision::Approved => {
            let write_intent = WriteIntent {
                operation: WriteOperation::CreateSubOrder {
                    parent_order_id: order_id,
                    line_ids,
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            let sub_order_id = kind_mother::write(ctx.environment_id(), write_intent).await?;
            Ok(sub_order_id)
        }
        Decision::Rejected { reason } => Err(RestaurantError::RejectedByGovernance(reason)),
    }
}
```

### 2.5 Pattern : Create Booking (réservation table)

```rust
// Dans RestaurantBooking
pub async fn create_booking(
    &self,
    ctx: &OperatorContext,
    booking: BookingDraft,
) -> Result<BookingId, RestaurantError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "restaurant.booking.create").await?;

    let decision = strong_father::decide(
        ctx,
        DecisionRequest::CreateBooking {
            tables: &booking.table_ids,
            start: booking.start,
            duration: booking.duration,
            guests: booking.guests,
        },
    ).await?;

    match decision {
        Decision::Approved => {
            // Délégation possible au service Agenda si ressources partagées
            let write_intent = WriteIntent {
                operation: WriteOperation::CreateBooking { booking },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            let id = kind_mother::write(ctx.environment_id(), write_intent).await?;
            Ok(id)
        }
        Decision::Rejected { reason } => Err(RestaurantError::RejectedByGovernance(reason)),
    }
}
```

---

## 3. Gestion des WriteIntent

**Opérations KindMother typiques :**
- `CreateFloor`, `UpdateTable`, `DeleteTable` (FloorManager)
- `BindOrderToTable`, `BindOrderToTab`, `ReleaseTable` (TableOrderBinding)
- `TransferOrder`, `MergeOrders` (OrderTransfer)
- `AddCourse`, `MarkCourseSent` (CourseManager)
- `CreateSubOrder`, `PaySubOrder` (BillSplit)
- `CreateBooking`, `UpdateBookingStage` (RestaurantBooking)

Chaque opération est soumise à StrongFather lorsque la décision métier est requise (assignation, transfert, split, réservation) ; les opérations purement techniques (envoi préparation, marquage cours envoyé) peuvent ne pas passer par StrongFather si la politique le permet.

---

## 4. Intégration avec Kits existants

- **POS Order / POS Payment** : les ordres et paiements restent gérés par le POS ; Restaurant ajoute le contexte (table, tab, cours, sous-commandes) et les actions (Set Table, Split, etc.).
- **Miyukini Agenda** : si le service Agenda gère les rendez-vous et ressources, RestaurantBooking délègue la persistance des créneaux et des ressources (tables) via un contrat d’équipe ; les ressources « table » sont déclarées côté FloorManager et exposées à l’Agenda.
- **PreparationPrint** : kit ou adaptateur pour canaux (imprimante, écran) ; pas de WriteIntent métier, uniquement envoi vers périphériques ou API externes.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
