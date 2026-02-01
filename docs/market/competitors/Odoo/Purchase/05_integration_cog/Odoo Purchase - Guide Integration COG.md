# Odoo Purchase — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Purchase dans l'architecture COG Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG
- Patterns d'implémentation
- Exemples de code (pseudo-code Rust)
- Gestion des WriteIntent
- Gestion des Mandats

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
PurchaseUI → PurchaseOrder → PurchaseOrderLine → MiyuStore
         → PurchaseApproval → StrongFather
         → PurchaseInvoice → MiyuInvoice
         → PurchaseReception → MiyuInventory (si développé)
         ↓
StrongFather (Décisions)
KindMother (Persistance)
Master Butler (Permissions)
WorrySentinel (Sécurité)
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : Création RFQ

```rust
// Dans PurchaseOrder
pub async fn create_rfq(
    &self,
    ctx: &OperatorContext,
    rfq: RFQDraft,
) -> Result<OrderId, PurchaseError> {
    // 1. Vérification permissions
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "purchase.order.create").await?;
    
    // 2. Vérification sécurité
    worry_sentinel::check_security_level(ctx, SecurityLevel::Sensitive).await?;
    
    // 3. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::CreateRFQ { rfq: &rfq },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 4. WriteIntent vers KindMother
            let write_intent = WriteIntent {
                operation: WriteOperation::CreateOrder {
                    order: rfq.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            
            let order_id = kind_mother::write(ctx.environment_id(), write_intent).await?;
            Ok(order_id)
        }
        Decision::Rejected { reason } => {
            Err(PurchaseError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.2 Pattern : Envoi RFQ

```rust
// Dans PurchaseOrder
pub async fn send_rfq(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
) -> Result<(), PurchaseError> {
    // 1. Lecture commande
    let order = purchase_order::read(ctx, order_id).await?;
    
    // 2. Vérifications préalables
    if order.state != OrderState::Draft {
        return Err(PurchaseError::InvalidState);
    }
    
    // 3. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::SendRFQ { order_id: &order_id },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 4. Envoi email via MiyuNotify
            let email_intent = EmailIntent {
                to: order.partner_id.email(),
                template: "purchase_rfq_template",
                context: order.clone(),
            };
            
            miyu_notify::send_email(ctx, email_intent).await?;
            
            // 5. Mise à jour état
            let write_intent = WriteIntent {
                operation: WriteOperation::UpdateOrder {
                    order_id: order_id.clone(),
                    updates: OrderUpdates {
                        state: OrderState::Sent,
                    },
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            
            kind_mother::write(ctx.environment_id(), write_intent).await?;
            Ok(())
        }
        Decision::Rejected { reason } => {
            Err(PurchaseError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.3 Pattern : Confirmation Commande

```rust
// Dans PurchaseOrder
pub async fn confirm_order(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
) -> Result<(), PurchaseError> {
    // 1. Lecture commande
    let order = purchase_order::read(ctx, order_id).await?;
    
    // 2. Vérifications préalables
    if order.state != OrderState::Draft && order.state != OrderState::Sent {
        return Err(PurchaseError::InvalidState);
    }
    
    // 3. Validation lignes
    for line in &order.lines {
        if line.product_id.is_none() && !line.is_section() {
            return Err(PurchaseError::MissingProduct);
        }
    }
    
    // 4. Vérification approbation requise
    let approval_required = purchase_approval::check_approval_required(
        ctx,
        &order,
    ).await?;
    
    if approval_required {
        // Mise à jour état "to approve"
        let write_intent = WriteIntent {
            operation: WriteOperation::UpdateOrder {
                order_id: order_id.clone(),
                updates: OrderUpdates {
                    state: OrderState::ToApprove,
                },
            },
            source: ctx.operator_id().clone(),
            mandate_id: ctx.mandate()?.id().clone(),
        };
        
        kind_mother::write(ctx.environment_id(), write_intent).await?;
        
        // Notification approbation
        purchase_approval::notify_approvers(ctx, order_id).await?;
        
        return Ok(());
    }
    
    // 5. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::ConfirmPurchaseOrder { order_id: &order_id },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 6. Ajout fournisseur aux produits
            for line in &order.lines {
                if let Some(product_id) = line.product_id {
                    miyu_store::add_supplier_to_product(
                        ctx,
                        product_id,
                        order.partner_id.clone(),
                        line.price_unit,
                        order.currency_id.clone(),
                    ).await?;
                }
            }
            
            // 7. Mise à jour état
            let write_intent = WriteIntent {
                operation: WriteOperation::UpdateOrder {
                    order_id: order_id.clone(),
                    updates: OrderUpdates {
                        state: OrderState::Purchase,
                        date_approve: Some(DateTime::now()),
                    },
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            
            kind_mother::write(ctx.environment_id(), write_intent).await?;
            
            // 8. Création réception (si Inventory)
            if miyu_inventory::is_available(ctx).await? {
                purchase_reception::create_from_order(ctx, order_id).await?;
            }
            
            Ok(())
        }
        Decision::Rejected { reason } => {
            Err(PurchaseError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.4 Pattern : Approbation Commande

```rust
// Dans PurchaseApproval
pub async fn approve_order(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
) -> Result<(), PurchaseError> {
    // 1. Vérification permissions
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "purchase.approval.approve").await?;
    
    // 2. Lecture commande
    let order = purchase_order::read(ctx, order_id).await?;
    
    // 3. Vérification état
    if order.state != OrderState::ToApprove {
        return Err(PurchaseError::InvalidState);
    }
    
    // 4. Vérification niveau sécurité selon montant
    let security_level = if order.amount_total > THRESHOLD_HIGH {
        SecurityLevel::Critical
    } else {
        SecurityLevel::Sensitive
    };
    
    worry_sentinel::check_security_level(ctx, security_level).await?;
    
    // 5. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::ApprovePurchaseOrder { order_id: &order_id },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 6. Mise à jour état
            let write_intent = WriteIntent {
                operation: WriteOperation::UpdateOrder {
                    order_id: order_id.clone(),
                    updates: OrderUpdates {
                        state: OrderState::Purchase,
                        date_approve: Some(DateTime::now()),
                    },
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            
            kind_mother::write(ctx.environment_id(), write_intent).await?;
            
            // 7. Notification acheteur
            purchase_order::notify_buyer(ctx, order_id, "approved").await?;
            
            // 8. Création réception (si Inventory)
            if miyu_inventory::is_available(ctx).await? {
                purchase_reception::create_from_order(ctx, order_id).await?;
            }
            
            Ok(())
        }
        Decision::Rejected { reason } => {
            Err(PurchaseError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.5 Pattern : Génération Facture

```rust
// Dans PurchaseInvoice
pub async fn create_invoice_from_order(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
) -> Result<InvoiceId, PurchaseError> {
    // 1. Vérification permissions
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "purchase.invoice.create").await?;
    
    // 2. Vérification sécurité
    worry_sentinel::check_security_level(ctx, SecurityLevel::Critical).await?;
    
    // 3. Lecture commande
    let order = purchase_order::read(ctx, order_id).await?;
    
    // 4. Vérification état
    if order.state != OrderState::Purchase {
        return Err(PurchaseError::InvalidState);
    }
    
    // 5. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::CreateInvoiceFromOrder { order_id: &order_id },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 6. Préparation facture
            let invoice_data = miyu_invoice::prepare_invoice_from_purchase_order(
                ctx,
                &order,
            ).await?;
            
            // 7. Création facture via MiyuInvoice
            let invoice_id = miyu_invoice::create_invoice(
                ctx,
                invoice_data,
            ).await?;
            
            // 8. Lien bidirectionnel
            let write_intent = WriteIntent {
                operation: WriteOperation::LinkInvoiceToOrder {
                    order_id: order_id.clone(),
                    invoice_id: invoice_id.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            
            kind_mother::write(ctx.environment_id(), write_intent).await?;
            
            // 9. Mise à jour statut facturation
            purchase_order::update_invoice_status(ctx, order_id).await?;
            
            Ok(invoice_id)
        }
        Decision::Rejected { reason } => {
            Err(PurchaseError::RejectedByGovernance(reason))
        }
    }
}
```

---

## 3. Gestion des WriteIntent

### 3.1 Structure WriteIntent

```rust
pub struct WriteIntent {
    pub operation: WriteOperation,
    pub source: OperatorId,
    pub mandate_id: MandateId,
    pub timestamp: DateTime,
}

pub enum WriteOperation {
    CreateOrder { order: Order },
    UpdateOrder { order_id: OrderId, updates: OrderUpdates },
    CreateLine { order_id: OrderId, line: OrderLine },
    UpdateLine { line_id: LineId, updates: LineUpdates },
    LinkInvoiceToOrder { order_id: OrderId, invoice_id: InvoiceId },
}
```

### 3.2 Validation WriteIntent

```rust
// Dans KindMother
pub async fn validate_write_intent(
    &self,
    ctx: &EnvironmentContext,
    intent: &WriteIntent,
) -> Result<ValidationResult, KindMotherError> {
    // 1. Vérification mandate valide
    let mandate = strong_father::get_mandate(ctx, &intent.mandate_id).await?;
    if !mandate.is_valid() {
        return Err(KindMotherError::InvalidMandate);
    }
    
    // 2. Vérification permissions
    master_butler::check_write_permission(
        ctx,
        &intent.source,
        &intent.operation,
    ).await?;
    
    // 3. Validation données
    match &intent.operation {
        WriteOperation::CreateOrder { order } => {
            validate_order(order)?;
        }
        WriteOperation::UpdateOrder { updates } => {
            validate_order_updates(updates)?;
        }
        // ...
    }
    
    Ok(ValidationResult::Approved)
}
```

---

## 4. Gestion des Mandats

### 4.1 Création Mandat

```rust
// Dans StrongFather
pub async fn create_purchase_mandate(
    &self,
    ctx: &EnvironmentContext,
    request: MandateRequest,
) -> Result<MandateId, StrongFatherError> {
    // 1. Validation requête
    validate_mandate_request(&request)?;
    
    // 2. Création mandat
    let mandate = Mandate {
        id: MandateId::generate(),
        operators: request.operators,
        flows: request.flows,
        security_level: request.security_level,
        valid_until: request.valid_until,
        created_at: DateTime::now(),
    };
    
    // 3. Persistance
    let write_intent = WriteIntent {
        operation: WriteOperation::CreateMandate { mandate: mandate.clone() },
        source: ctx.core_id().clone(),
        mandate_id: MandateId::system(),
    };
    
    kind_mother::write(ctx.environment_id(), write_intent).await?;
    
    Ok(mandate.id)
}
```

### 4.2 Utilisation Mandat

```rust
// Dans PurchaseOrder
pub async fn create_with_mandate(
    &self,
    ctx: &OperatorContext,
    mandate_id: MandateId,
    rfq: RFQDraft,
) -> Result<OrderId, PurchaseError> {
    // 1. Récupération mandat
    let mandate = strong_father::get_mandate(ctx, &mandate_id).await?;
    
    // 2. Vérification validité
    if !mandate.is_valid() {
        return Err(PurchaseError::InvalidMandate);
    }
    
    // 3. Vérification opérateur autorisé
    if !mandate.allows_operator(ctx.operator_id()) {
        return Err(PurchaseError::UnauthorizedOperator);
    }
    
    // 4. Création avec mandat
    self.create_rfq_with_mandate(ctx, mandate, rfq).await
}
```

---

## 5. Intégration avec Autres Services

### 5.1 Intégration MiyuStore

```rust
// Calcul prix depuis seller
pub async fn get_product_price(
    ctx: &OperatorContext,
    product_id: ProductId,
    partner_id: PartnerId,
    quantity: f64,
    date: DateTime,
) -> Result<Price, PurchaseError> {
    // 1. Sélection seller
    let seller = miyu_store::select_seller(
        ctx,
        product_id,
        partner_id,
        quantity,
        date,
    ).await?;
    
    // 2. Calcul prix
    let price = seller.price * (1.0 - seller.discount / 100.0);
    
    // 3. Conversion devise si nécessaire
    let price_converted = miyu_accounting::convert_currency(
        ctx,
        price,
        seller.currency_id,
        ctx.company_currency_id(),
        date,
    ).await?;
    
    Ok(price_converted)
}
```

### 5.2 Intégration MiyuInvoice

```rust
// Préparation facture depuis commande
pub async fn prepare_invoice_from_order(
    ctx: &OperatorContext,
    order: &Order,
) -> Result<InvoiceData, PurchaseError> {
    let invoice_data = InvoiceData {
        move_type: MoveType::InInvoice,
        partner_id: order.partner_id.clone(),
        currency_id: order.currency_id.clone(),
        invoice_origin: order.name.clone(),
        invoice_payment_term_id: order.payment_term_id.clone(),
        fiscal_position_id: order.fiscal_position_id.clone(),
        invoice_line_ids: vec![],
    };
    
    // Préparation lignes
    for line in &order.lines {
        if line.qty_to_invoice > 0.0 {
            let invoice_line = InvoiceLineData {
                product_id: line.product_id.clone(),
                product_uom_id: line.product_uom_id.clone(),
                quantity: line.qty_to_invoice,
                price_unit: line.price_unit,
                discount: line.discount,
                tax_ids: line.tax_ids.clone(),
                purchase_line_id: Some(line.id.clone()),
            };
            
            invoice_data.invoice_line_ids.push(invoice_line);
        }
    }
    
    Ok(invoice_data)
}
```

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
