# Odoo POS Shop — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités POS Shop dans l'architecture COG Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour POS Shop
- Patterns d'implémentation (Ouverture session, Vente, Paiement, Clôture session)
- Exemples de code (pseudo-code Rust)
- Gestion des WriteIntent et Mandats

---

## 1. Architecture d'Intégration

### 1.1 Vue d'Ensemble

```
PosUI → PosSession (open, close, cash_in_out)
     → PosOrder (create, update, validate, refund, invoice)
     → PosPayment (register payment)
     ↓
PosOrder → MiyuStore (products, pricelist)
         → MiyuContacts (customer)
         → MiyuInvoice (create invoice)
     ↓
StrongFather (Décisions)
KindMother (Persistance)
Master Butler (Permissions)
WorrySentinel (Sécurité caisse / paiements)
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : Ouverture de Session

```rust
// Dans PosSession
pub async fn open_session(
    &self,
    ctx: &OperatorContext,
    config_id: PosConfigId,
    opening_balance: Decimal,
) -> Result<SessionId, PosError> {
    // 1. Vérification permissions
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "pos.session.open").await?;

    // 2. Vérification sécurité
    worry_sentinel::check_security_level(ctx, SecurityLevel::Sensitive).await?;

    // 3. Vérification qu'aucune session n'est déjà ouverte pour ce POS / navigateur
    if pos_session::has_open_session(ctx, config_id).await? {
        return Err(PosError::SessionAlreadyOpen);
    }

    // 4. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::OpenPosSession {
            config_id: config_id.clone(),
            opening_balance,
        },
    ).await?;

    match decision {
        Decision::Approved => {
            // 5. WriteIntent vers KindMother
            let write_intent = WriteIntent {
                operation: WriteOperation::OpenSession {
                    config_id: config_id.clone(),
                    user_id: ctx.user_id().clone(),
                    opening_balance,
                    opened_at: miyuclock::now(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: mandate.id().clone(),
            };

            let session_id = kind_mother::write(ctx.environment_id(), write_intent).await?;
            Ok(session_id)
        }
        Decision::Rejected { reason } => Err(PosError::RejectedByGovernance(reason)),
    }
}
```

### 2.2 Pattern : Validation de Commande (Paiement Complet)

```rust
// Dans PosOrder
pub async fn validate_order(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
    payments: Vec<PaymentInput>,
) -> Result<(), PosError> {
    // 1. Lecture commande
    let order = pos_order::read(ctx, order_id).await?;

    if order.state != OrderState::Draft {
        return Err(PosError::InvalidState);
    }

    // 2. Vérification montants
    let total_paid: Decimal = payments.iter().map(|p| p.amount).sum();
    if total_paid < order.amount_total {
        return Err(PosError::InsufficientPayment {
            required: order.amount_total,
            received: total_paid,
        });
    }

    // 3. Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::ValidatePosOrder {
            order_id: order_id.clone(),
            amount_total: order.amount_total,
        },
    ).await?;

    match decision {
        Decision::Approved => {
            // 4. Enregistrement des paiements (PosPayment) → WriteIntent
            for payment in &payments {
                let payment_intent = WriteIntent {
                    operation: WriteOperation::CreatePayment {
                        order_id: order_id.clone(),
                        method_id: payment.method_id.clone(),
                        amount: payment.amount,
                    },
                    source: ctx.operator_id().clone(),
                    mandate_id: ctx.mandate()?.id().clone(),
                };
                kind_mother::write(ctx.environment_id(), payment_intent).await?;
            }

            // 5. Passage commande en done + génération mouvements stock
            let update_intent = WriteIntent {
                operation: WriteOperation::ValidateOrder {
                    order_id: order_id.clone(),
                    state: OrderState::Done,
                    validated_at: miyuclock::now(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            kind_mother::write(ctx.environment_id(), update_intent).await?;

            // 6. Déclenchement sortie stock (via KindMother ou module stock)
            stock::create_moves_from_pos_order(ctx, order_id).await?;

            Ok(())
        }
        Decision::Rejected { reason } => Err(PosError::RejectedByGovernance(reason)),
    }
}
```

### 2.3 Pattern : Clôture de Session

```rust
// Dans PosSession
pub async fn close_session(
    &self,
    ctx: &OperatorContext,
    session_id: SessionId,
    counted_amounts: HashMap<PaymentMethodId, Decimal>,
    allow_difference: bool,
) -> Result<(), PosError> {
    // 1. Lecture session
    let session = pos_session::read(ctx, session_id).await?;

    if session.state != SessionState::Opened {
        return Err(PosError::SessionNotOpen);
    }

    // 2. Calcul des écarts par méthode de paiement
    let differences = pos_payment::compute_differences(ctx, session_id, &counted_amounts).await?;

    if !allow_difference && differences.has_non_zero() {
        return Err(PosError::CashDifferenceNotAllowed { differences });
    }

    // 3. Décision StrongFather (clôture avec ou sans écart)
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::ClosePosSession {
            session_id: session_id.clone(),
            differences: differences.clone(),
        },
    ).await?;

    match decision {
        Decision::Approved => {
            // 4. Validation du relevé de caisse (KindMother)
            let write_intent = WriteIntent {
                operation: WriteOperation::CloseSession {
                    session_id: session_id.clone(),
                    closed_at: miyuclock::now(),
                    counted_amounts: counted_amounts.clone(),
                    differences,
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };

            kind_mother::write(ctx.environment_id(), write_intent).await?;

            // 5. Validation du statement (trésorerie / accounting)
            miyutreasury::validate_statement(ctx, session.statement_id()).await?;

            Ok(())
        }
        Decision::Rejected { reason } => Err(PosError::RejectedByGovernance(reason)),
    }
}
```

### 2.4 Pattern : Génération Facture depuis Ticket

```rust
// Dans PosOrder (ou délégation à MiyuInvoice)
pub async fn create_invoice_from_order(
    &self,
    ctx: &OperatorContext,
    order_id: OrderId,
) -> Result<InvoiceId, PosError> {
    let order = pos_order::read(ctx, order_id).await?;

    if order.state != OrderState::Done {
        return Err(PosError::OrderNotValidated);
    }
    if order.is_invoiced {
        return Err(PosError::AlreadyInvoiced);
    }

    // Décision StrongFather
    let decision = strong_father::decide(
        ctx,
        DecisionRequest::CreateInvoiceFromPosOrder {
            order_id: order_id.clone(),
        },
    ).await?;

    match decision {
        Decision::Approved => {
            // Préparation facture via MiyuInvoice
            let invoice_draft = miyuinvoice::prepare_invoice(
                ctx,
                InvoiceSource::PosOrder {
                    order_id: order_id.clone(),
                    lines: order.lines.clone(),
                    partner_id: order.partner_id.clone(),
                },
            ).await?;

            let write_intent = WriteIntent {
                operation: WriteOperation::CreateInvoice {
                    invoice: invoice_draft.clone(),
                    source_order_id: order_id.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };

            let invoice_id = kind_mother::write(ctx.environment_id(), write_intent).await?;

            // Lien bidirectionnel order ↔ invoice
            let link_intent = WriteIntent {
                operation: WriteOperation::LinkInvoice {
                    order_id: order_id.clone(),
                    invoice_id: invoice_id.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };
            kind_mother::write(ctx.environment_id(), link_intent).await?;

            Ok(invoice_id)
        }
        Decision::Rejected { reason } => Err(PosError::RejectedByGovernance(reason)),
    }
}
```

---

## 3. Intégration avec Kits Existants

### 3.1 MiyuStore (Prix et Produits)

```rust
pub async fn get_product_price(
    ctx: &OperatorContext,
    product_id: ProductId,
    pricelist_id: PricelistId,
    qty: f64,
    partner_id: Option<PartnerId>,
) -> Result<Decimal, PosError> {
    let price = miyustore::get_pricelist_price(
        ctx,
        product_id,
        pricelist_id,
        qty,
        partner_id,
    ).await?;
    Ok(price)
}
```

### 3.2 MiyuContacts (Client)

```rust
pub async fn set_order_customer(
    ctx: &OperatorContext,
    order_id: OrderId,
    partner_id: PartnerId,
) -> Result<(), PosError> {
    let write_intent = WriteIntent {
        operation: WriteOperation::UpdateOrderCustomer {
            order_id,
            partner_id,
        },
        source: ctx.operator_id().clone(),
        mandate_id: ctx.mandate()?.id().clone(),
    };
    kind_mother::write(ctx.environment_id(), write_intent).await?;
    Ok(())
}
```

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
