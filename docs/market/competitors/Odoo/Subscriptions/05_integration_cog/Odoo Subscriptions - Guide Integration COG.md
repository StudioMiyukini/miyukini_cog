# Odoo Subscriptions — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Subscriptions (Abonnements) dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Subscriptions
- Patterns WriteIntent et Mandates (création, renouvellement, upsell, clôture, facturation, paiement récurrent)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy, TAMR)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
SubscriptionUI → BondingBrother → SubscriptionOperator ──► StrongFather (décision confirm/renew/upsell/close)
                            RecurringPlanOperator         KindMother (WriteIntent)
                            SubscriptionBillingOperator   Master Butler (permissions)
                            SubscriptionPaymentOperator    WorrySentinel (sécurité)
                                                          Ever Buddy (planification, cycle de vie)
                                                          TAMR (résolution échec paiement)
```

### 1.2 Flux typiques

1. **Création abonnement** : Devis (Sales) + Recurring Plan → Confirm → SubscriptionOperator → StrongFather (décision confirm) → KindMother (WriteIntent order → abonnement In Progress).
2. **Renouvellement** : SubscriptionOperator.renew → StrongFather (renew) → KindMother (WriteIntent renewal quotation) → Workflow standard (Confirm → Facture → Paiement).
3. **Upsell** : SubscriptionOperator.upsell → StrongFather (upsell) → KindMother (WriteIntent upsell quotation) → Après confirmation, KindMother (fusion lignes à l’abonnement) ; SubscriptionBillingOperator (prorata services).
4. **Clôture** : SubscriptionOperator.close → StrongFather (close) → KindMother (WriteIntent status Churned/Closed + close reason).
5. **Facturation planifiée** : Ever Buddy (schedule) → SubscriptionBillingOperator.generate_invoice → StrongFather (bill) → KindMother (WriteIntent invoice) + MiyuInvoice.
6. **Paiement récurrent** : SubscriptionPaymentOperator.charge → StrongFather (charge) → MiyuBilling (prélèvement) ; en cas d’échec : KindMother (WriteIntent Payment Failure + Contract in exception).
7. **Résolution exception** : SubscriptionOperator.resolve_exception ou SubscriptionPaymentOperator.resolve_failure → StrongFather (resolve) → KindMother (WriteIntent exception levée) + TAMR (intervention humaine tracée).

---

## 2. Patterns d'Intégration

### 2.1 Création / confirmation d'abonnement

**Pattern :** WriteIntent Update (order state) + Mandate

```rust
// Pseudo-code Rust
pub struct ConfirmSubscriptionIntent {
    pub order_id: OrderId,
    pub recurring_plan_id: RecurringPlanId,
    pub partner_id: PartnerId,
    pub order_line_ids: Vec<OrderLineInput>,
}

impl SubscriptionOperator {
    pub async fn confirm_subscription(
        &self,
        intent: ConfirmSubscriptionIntent,
        mandate: Mandate,
    ) -> Result<SubscriptionOrder, SubscriptionError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["subscription.create"])?;

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "subscription.create",
                resource: Some(Resource::Order(intent.order_id)),
            })
            .await?;
        if !permission.granted {
            return Err(SubscriptionError::PermissionDenied);
        }

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "confirm_subscription",
                context: &intent,
                mandate: &mandate,
            })
            .await?;
        if !decision.allowed {
            return Err(SubscriptionError::DecisionDenied(decision.reason));
        }

        let write_intent = WriteIntent {
            entity_type: "subscription.order",
            operation: WriteOperation::Update,
            data: SubscriptionOrderData {
                state: SubscriptionState::InProgress,
                recurring_plan_id: intent.recurring_plan_id,
                next_invoice_date: self.compute_next_invoice_date(&intent).await?,
                contract_in_exception: false,
                ..existing_order.into()
            },
        };

        let result = self.kind_mother
            .submit_write_intent(write_intent, &mandate)
            .await?;
        Ok(result.into())
    }
}
```

### 2.2 Renouvellement (Renew)

**Pattern :** WriteIntent Create (renewal quotation) + Mandate

```rust
pub struct RenewSubscriptionIntent {
    pub subscription_order_id: SubscriptionOrderId,
}

impl SubscriptionOperator {
    pub async fn renew(
        &self,
        intent: RenewSubscriptionIntent,
        mandate: Mandate,
    ) -> Result<RenewalQuotation, SubscriptionError> {
        mandate.validate_flows(&["subscription.renew"])?;

        let subscription = self.get_subscription(intent.subscription_order_id).await?;
        self.check_renew_prerequisites(&subscription)?; // confirmé, plan configuré, première facture payée

        let decision = self.strong_father
            .decide(DecisionRequest {
                action: "renew_subscription",
                context: &intent,
                mandate: &mandate,
            })
            .await?;
        if !decision.allowed {
            return Err(SubscriptionError::DecisionDenied(decision.reason));
        }

        let renewal_data = self.prepare_renewal_quotation(&subscription).await?;
        let write_intent = WriteIntent {
            entity_type: "sale.order",
            operation: WriteOperation::Create,
            data: renewal_data, // tag Renewal Quotation, dates start/next invoice en chatter
        };

        let result = self.kind_mother
            .submit_write_intent(write_intent, &mandate)
            .await?;
        self.link_renewal_to_subscription(result.id(), intent.subscription_order_id).await?;
        Ok(result.into())
    }
}
```

### 2.3 Upsell

**Pattern :** WriteIntent Create (upsell quotation) + après confirmation WriteIntent Update (fusion lignes)

```rust
pub struct UpsellSubscriptionIntent {
    pub subscription_order_id: SubscriptionOrderId,
    pub new_line_ids: Vec<OrderLineInput>,
}

impl SubscriptionOperator {
    pub async fn create_upsell_quotation(
        &self,
        intent: UpsellSubscriptionIntent,
        mandate: Mandate,
    ) -> Result<UpsellQuotation, SubscriptionError> {
        mandate.validate_flows(&["subscription.upsell"])?;
        // Vérifier abonnement déjà facturé
        let decision = self.strong_father.decide(...).await?;
        let write_intent = WriteIntent {
            entity_type: "sale.order",
            operation: WriteOperation::Create,
            data: upsell_quotation_data, // tag Upsell, lignes initiales + new_line_ids, avertissement prorata
        };
        self.kind_mother.submit_write_intent(write_intent, &mandate).await
    }

    pub async fn confirm_upsell(
        &self,
        upsell_order_id: OrderId,
        mandate: Mandate,
    ) -> Result<(), SubscriptionError> {
        // Après confirmation client : fusion des lignes upsell dans l'abonnement ; prorata via SubscriptionBillingOperator
        let write_intent = WriteIntent {
            entity_type: "subscription.order",
            operation: WriteOperation::Update,
            data: SubscriptionOrderData {
                merge_upsell_lines: upsell_order_id,
                ..
            },
        };
        self.kind_mother.submit_write_intent(write_intent, &mandate).await?;
        self.subscription_billing.prorata_apply(upsell_order_id).await?;
        Ok(())
    }
}
```

### 2.4 Clôture (Close)

**Pattern :** WriteIntent Update (status + close reason) + Mandate

```rust
pub struct CloseSubscriptionIntent {
    pub subscription_order_id: SubscriptionOrderId,
    pub close_reason_id: CloseReasonId,
    pub reason_text: Option<String>, // admin peut saisir du texte libre
}

impl SubscriptionOperator {
    pub async fn close(
        &self,
        intent: CloseSubscriptionIntent,
        mandate: Mandate,
    ) -> Result<(), SubscriptionError> {
        mandate.validate_flows(&["subscription.close"])?;
        let decision = self.strong_father.decide(...).await?;
        let write_intent = WriteIntent {
            entity_type: "subscription.order",
            operation: WriteOperation::Update,
            data: SubscriptionOrderData {
                state: SubscriptionState::Churned, // ou Closed si portail client
                close_reason_id: intent.close_reason_id,
                close_reason_text: intent.reason_text,
                ..
            },
        };
        self.kind_mother.submit_write_intent(write_intent, &mandate).await
    }
}
```

### 2.5 Résolution exception (Contract in exception)

**Pattern :** WriteIntent Update (contract_in_exception = false) + TAMR (intervention humaine)

```rust
pub struct ResolveExceptionIntent {
    pub subscription_order_id: SubscriptionOrderId,
    pub payment_was_received: bool,
    pub manual_invoice_id: Option<InvoiceId>, // si paiement reçu mais non enregistré
}

impl SubscriptionOperator {
    pub async fn resolve_exception(
        &self,
        intent: ResolveExceptionIntent,
        mandate: Mandate,
    ) -> Result<(), SubscriptionError> {
        mandate.validate_flows(&["subscription.resolve_exception"])?;
        self.tamr.record_human_intervention("resolve_payment_failure", &intent).await?;
        let decision = self.strong_father.decide(...).await?;
        if intent.payment_was_received && intent.manual_invoice_id.is_none() {
            // Créer/facturer manuellement si besoin
        }
        let write_intent = WriteIntent {
            entity_type: "subscription.order",
            operation: WriteOperation::Update,
            data: SubscriptionOrderData {
                contract_in_exception: false,
                payment_failure_tag: false,
                ..
            },
        };
        self.kind_mother.submit_write_intent(write_intent, &mandate).await
    }
}
```

### 2.6 Facturation planifiée (SubscriptionBillingOperator)

**Pattern :** Ever Buddy schedule → StrongFather (bill) → KindMother (invoice)

```rust
impl SubscriptionBillingOperator {
    pub async fn generate_scheduled_invoice(
        &self,
        subscription_id: SubscriptionOrderId,
        mandate: Mandate,
    ) -> Result<InvoiceId, SubscriptionError> {
        let subscription = self.get_subscription(subscription_id).await?;
        if subscription.contract_in_exception {
            return Err(SubscriptionError::ContractInException);
        }
        if subscription.next_invoice_date > today() {
            return Err(SubscriptionError::NotDueYet);
        }
        let decision = self.strong_father.decide(...).await?;
        let invoice_data = self.prepare_invoice_from_subscription(&subscription).await?;
        let write_intent = WriteIntent {
            entity_type: "account.move",
            operation: WriteOperation::Create,
            data: invoice_data,
        };
        let result = self.kind_mother.submit_write_intent(write_intent, &mandate).await?;
        self.update_next_invoice_date(subscription_id, &subscription).await?;
        Ok(result.id())
    }
}
```

### 2.7 Paiement récurrent (SubscriptionPaymentOperator)

**Pattern :** Tokenisation (KindMother + MiyuBilling) ; Charge (StrongFather + MiyuBilling) ; en échec → WriteIntent exception

```rust
impl SubscriptionPaymentOperator {
    pub async fn charge_recurring(
        &self,
        subscription_id: SubscriptionOrderId,
        invoice_id: InvoiceId,
        mandate: Mandate,
    ) -> Result<PaymentId, SubscriptionError> {
        let token = self.get_customer_token(subscription_id).await?;
        let decision = self.strong_father.decide(...).await?;
        match self.miyu_billing.charge_with_token(token, invoice_id).await {
            Ok(payment_id) => Ok(payment_id),
            Err(_) => {
                let write_intent = WriteIntent {
                    entity_type: "subscription.order",
                    operation: WriteOperation::Update,
                    data: SubscriptionOrderData {
                        contract_in_exception: true,
                        payment_failure_tag: true,
                        ..
                    },
                };
                self.kind_mother.submit_write_intent(write_intent, &mandate).await?;
                Err(SubscriptionError::PaymentFailed)
            }
        }
    }
}
```

---

## 3. Intégration avec Kits existants

- **Miyukini Sales / MiyuStore** : Devis et commandes (sale.order) ; SubscriptionOperator consomme et étend par « type » abonnement + plan récurrent.
- **MiyuInvoice** : Factures et écritures ; SubscriptionBillingOperator déclenche la création des factures récurrentes et le prorata.
- **MiyuBilling** : Tokenisation et prélèvement ; SubscriptionPaymentOperator délègue tokenize et charge.
- **MiyuNotify** : Notifications (facture envoyée, échec paiement, renouvellement) ; templates configurés dans le plan.
- **MiyuContacts** : Client (partner) et multi-société (Company sur le plan).
- **Ever Buddy** : Planification des échéances (next invoice date) et exécution des tâches planifiées (génération factures, renouvellements).
- **TAMR** : Enregistrement de l’intervention humaine lors de la résolution d’un échec de paiement.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
