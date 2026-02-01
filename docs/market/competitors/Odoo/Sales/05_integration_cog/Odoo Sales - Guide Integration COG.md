# Odoo Sales — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Sales dans l'architecture COG Miyukini.

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
SalesUI → SalesOrder → SalesOrderLine → SalesPricelist
       → SalesInvoice → MiyuInvoice
       → SalesPayment
       ↓
StrongFather (Décisions)
KindMother (Persistance)
Master Butler (Permissions)
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : Création Devis

```rust
// Dans SalesOrder
pub async fn create_quotation(
    &self,
    ctx: &OperatorContext,
    quotation: QuotationDraft,
) -> Result<OrderId, SalesError> {
    // 1. Vérification permissions
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "sales.order.create").await?;
    
    // 2. Vérification sécurité
    worry_sentinel::check_security_level(ctx, SecurityLevel::Sensitive).await?;
    
    // 3. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::CreateQuotation { quotation: &quotation },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 4. WriteIntent vers KindMother
            let write_intent = WriteIntent {
                operation: WriteOperation::CreateOrder {
                    order: quotation.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };
            
            let order_id = kind_mother::write(ctx.environment_id(), write_intent).await?;
            Ok(order_id)
        }
        Decision::Rejected { reason } => {
            Err(SalesError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.2 Pattern : Confirmation Commande

```rust
// Dans SalesOrder
pub async fn confirm_order(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
) -> Result<(), SalesError> {
    // 1. Lecture commande
    let order = sales_order::read(ctx, order_id).await?;
    
    // 2. Vérifications préalables
    if order.state != OrderState::Draft && order.state != OrderState::Sent {
        return Err(SalesError::InvalidState);
    }
    
    // 3. Validation lignes
    for line in &order.lines {
        if line.product_id.is_none() && !line.is_section() {
            return Err(SalesError::MissingProduct);
        }
    }
    
    // 4. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::ConfirmOrder { order_id: order_id.clone() },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 5. Validation cycle de vie (Ever Buddy)
            ever_buddy::validate_lifecycle_transition(
                ctx,
                LifecycleTransition::SentToSale { order_id: order_id.clone() },
            ).await?;
            
            // 6. WriteIntent vers KindMother
            let write_intent = WriteIntent {
                operation: WriteOperation::ConfirmOrder {
                    order_id: order_id.clone(),
                    date_order: miyuclock::now(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            
            kind_mother::write(ctx.environment_id(), write_intent).await?;
            
            Ok(())
        }
        Decision::Rejected { reason } => {
            Err(SalesError::RejectedByGovernance(reason))
        }
    }
}
```

### 2.3 Pattern : Génération Facture

```rust
// Dans SalesInvoice
pub async fn create_invoice_from_order(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
    grouped: bool,
) -> Result<InvoiceId, SalesError> {
    // 1. Lecture commande
    let order = sales_order::read(ctx, order_id).await?;
    
    // 2. Vérification état
    if order.state != OrderState::Sale {
        return Err(SalesError::OrderNotConfirmed);
    }
    
    // 3. Récupération lignes facturables
    let invoiceable_lines = order.lines.iter()
        .filter(|line| line.qty_to_invoice > 0.0)
        .collect::<Vec<_>>();
    
    if invoiceable_lines.is_empty() {
        return Err(SalesError::NothingToInvoice);
    }
    
    // 4. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::CreateInvoice {
            order_id: order_id.clone(),
            lines: invoiceable_lines.len(),
        },
    ).await?;
    
    match decision {
        Decision::Approved => {
            // 5. Préparation facture via MiyuInvoice
            let invoice_draft = miyuinvoice::prepare_invoice(
                ctx,
                InvoiceSource::SalesOrder {
                    order_id: order_id.clone(),
                    lines: invoiceable_lines,
                },
            ).await?;
            
            // 6. WriteIntent création facture
            let write_intent = WriteIntent {
                operation: WriteOperation::CreateInvoice {
                    invoice: invoice_draft.clone(),
                    source_order_id: order_id.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            
            let invoice_id = kind_mother::write(ctx.environment_id(), write_intent).await?;
            
            // 7. Mise à jour statut facturation commande
            let update_intent = WriteIntent {
                operation: WriteOperation::UpdateInvoiceStatus {
                    order_id: order_id.clone(),
                    invoice_id: invoice_id.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            
            kind_mother::write(ctx.environment_id(), update_intent).await?;
            
            Ok(invoice_id)
        }
        Decision::Rejected { reason } => {
            Err(SalesError::RejectedByGovernance(reason))
        }
    }
}
```

---

## 3. Intégration avec Kits Existants

### 3.1 Intégration MiyuInvoice

```rust
// Génération facture
pub async fn generate_invoice(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
) -> Result<InvoiceId, SalesError> {
    let order = sales_order::read(ctx, order_id).await?;
    
    // Utilisation MiyuInvoice pour génération
    let invoice_id = miyuinvoice::create_from_order(
        ctx,
        order.clone(),
    ).await?;
    
    // Lien bidirectionnel
    let write_intent = WriteIntent {
        operation: WriteOperation::LinkInvoice {
            order_id: order_id.clone(),
            invoice_id: invoice_id.clone(),
        },
        source: ctx.operator_id().clone(),
        mandate_id: ctx.mandate()?.id().clone(),
    };
    
    kind_mother::write(ctx.environment_id(), write_intent).await?;
    
    Ok(invoice_id)
}
```

### 3.2 Intégration MiyuStore

```rust
// Calcul prix depuis pricelist
pub async fn compute_line_price(
    &self,
    ctx: &OperatorContext,
    product_id: ProductId,
    quantity: f64,
    pricelist_id: PricelistId,
) -> Result<Price, SalesError> {
    // Utilisation MiyuStore pour prix
    let price = miyustore::get_pricelist_price(
        ctx,
        product_id,
        quantity,
        pricelist_id,
    ).await?;
    
    Ok(price)
}
```

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
