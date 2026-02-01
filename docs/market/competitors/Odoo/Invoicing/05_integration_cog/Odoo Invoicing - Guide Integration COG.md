# Odoo Invoicing — Guide d'Intégration COG

## Contexte

Ce document fournit un **guide pratique d'intégration** pour implémenter les fonctionnalités Invoicing dans l'architecture COG Miyukini : gouvernance, WriteIntent, Mandats de Permission, et réutilisation de MiyuInvoice.

**Références :**
- [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Invoicing%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Odoo Invoicing - Logique Métier](../00_logique_metier/Odoo%20Invoicing%20-%20Logique%20Metier%20Complete.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG (InvoiceService)
- Patterns d'implémentation (WriteIntent, Mandats, validation, envoi, paiement)
- Exemples de code (pseudo-code Rust)
- Gestion des erreurs et rollback
- Intégration avec MiyuInvoice et Cores

**Hors scope :**
- Implémentation complète (voir Guide d'Implémentation)
- Spécifications UI/UX

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
┌─────────────────────────────────────────────────────────────┐
│                    InvoiceUI (Opérateur Interface)         │
│                    Niveau sécurité: 1                       │
└───────────────────────┬─────────────────────────────────────┘
                        │
        ┌───────────────┼───────────────┬───────────────┐
        │               │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌─────▼──────┐ ┌──────▼──────┐
│InvoiceLedger │ │InvoicePayment│ │InvoiceSend │ │InvoiceTerms │
│   (S2)       │ │   (S2-S3)    │ │  (S1-S2)   │ │   (S2)      │
└───────┬──────┘ └──────┬──────┘ └─────┬──────┘ └──────┬──────┘
        │               │               │               │
        └───────────────┼───────────────┴───────────────┘
                        │
        ┌───────────────┼───────────────┐
        │               │               │
┌───────▼──────┐ ┌──────▼──────┐ ┌─────▼──────┐
│StrongFather  │ │ KindMother  │ │ Ever Buddy │
│  (Décision)  │ │ (Persistance)│ │ (Séquences)│
└──────────────┘ └──────────────┘ └────────────┘
```

### 1.2 Flux de Données Standard

**Création de facture (brouillon) :**
```
InvoiceUI → InvoiceLedger → Master Butler (permissions)
         → WorrySentinel (sécurité) → KindMother (WriteIntent) → Persistance
```

**Validation de facture :**
```
InvoiceUI → InvoiceLedger → StrongFather (décision validation)
         → Ever Buddy (séquence numéro) → KindMother (WriteIntent) → Persistance
```

**Envoi de facture :**
```
InvoiceUI → InvoiceSend → KindMother (lecture facture) → Génération PDF
         → StrongFather (décision envoi si politique) → Envoi email
         → KindMother (WriteIntent "marquer envoyé") → Persistance
```

**Enregistrement paiement :**
```
InvoiceUI → InvoicePayment → StrongFather (décision si politique)
         → KindMother (WriteIntent paiement + réconciliation) → Persistance
```

---

## 2. Patterns d'Implémentation

### 2.1 Pattern : WriteIntent Facture vers KindMother

**Principe :** Toute création ou modification de facture passe par WriteIntent vers KindMother.

**Pseudo-code Rust :**

```rust
// Dans InvoiceLedger
pub async fn create_invoice(
    &self,
    ctx: &OperatorContext,
    draft: InvoiceDraft,
) -> Result<InvoiceId, InvoiceError> {
    // 1. Vérification permissions (Master Butler)
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "invoice.create").await?;

    // 2. Vérification sécurité (WorrySentinel)
    worry_sentinel::check_security_level(
        &ctx.environment_id(),
        SecurityLevel::Sensitive,
    ).await?;

    // 3. Calculs (MiyuInvoice ou local)
    let lines = miyuinvoice::compute_lines(&draft.lines).await?;
    let totals = miyuinvoice::compute_totals(&lines).await?;

    // 4. WriteIntent vers KindMother
    let write_intent = WriteIntent {
        operation: WriteOperation::CreateInvoice {
            partner_id: draft.partner_id,
            invoice_date: draft.invoice_date,
            payment_term_id: draft.payment_term_id,
            lines,
            totals,
        },
        source: ctx.operator_id().clone(),
        mandate_id: mandate.id().clone(),
    };

    let invoice_id = kind_mother::write(&ctx.environment_id(), write_intent).await?;
    Ok(invoice_id)
}
```

### 2.2 Pattern : Validation avec Séquence (Ever Buddy)

**Principe :** La validation de facture génère un numéro de séquence via Ever Buddy.

**Pseudo-code Rust :**

```rust
// Dans InvoiceLedger
pub async fn validate_invoice(
    &self,
    ctx: &OperatorContext,
    invoice_id: InvoiceId,
) -> Result<ValidatedInvoice, InvoiceError> {
    let invoice = kind_mother::read::<Invoice>(&ctx.environment_id(), invoice_id.clone()).await?;

    if !invoice.is_balanced() {
        return Err(InvoiceError::Unbalanced);
    }

    let decision = strong_father::decide(
        &ctx.environment_id(),
        DecisionRequest::ValidateInvoice { invoice_id: invoice_id.clone() },
    ).await?;

    match decision {
        Decision::Approved => {
            let sequence_number = ever_buddy::generate_sequence(
                &ctx.environment_id(),
                journal_sequence_id(&invoice.journal_id()),
                invoice.invoice_date(),
            ).await?;

            let write_intent = WriteIntent {
                operation: WriteOperation::ValidateInvoice {
                    invoice_id: invoice_id.clone(),
                    sequence_number: sequence_number.clone(),
                },
                source: ctx.operator_id().clone(),
                mandate_id: ctx.mandate()?.id().clone(),
            };

            let validated = kind_mother::write(&ctx.environment_id(), write_intent).await?;
            Ok(validated)
        }
        Decision::Rejected { reason } => Err(InvoiceError::RejectedByGovernance(reason)),
    }
}
```

### 2.3 Pattern : Envoi de Facture (InvoiceSend)

**Principe :** Lecture facture (KindMother), génération PDF, décision StrongFather (optionnel), envoi email, WriteIntent "marquer envoyé".

**Pseudo-code Rust :**

```rust
// Dans InvoiceSend
pub async fn send_invoice(
    &self,
    ctx: &OperatorContext,
    invoice_id: InvoiceId,
    options: SendOptions,
) -> Result<(), InvoiceError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "invoice.send").await?;

    let invoice = kind_mother::read::<Invoice>(&ctx.environment_id(), invoice_id.clone()).await?;
    let pdf = miyuinvoice::render_pdf(&invoice).await?;

    if options.require_approval {
        let decision = strong_father::decide(
            &ctx.environment_id(),
            DecisionRequest::SendInvoice { invoice_id: invoice_id.clone() },
        ).await?;
        if let Decision::Rejected { reason } = decision {
            return Err(InvoiceError::RejectedByGovernance(reason));
        }
    }

    email::send(
        &invoice.partner_email(),
        options.subject,
        options.body,
        Attachment::Pdf(pdf),
    ).await?;

    let write_intent = WriteIntent {
        operation: WriteOperation::MarkInvoiceSent { invoice_id: invoice_id.clone() },
        source: ctx.operator_id().clone(),
        mandate_id: mandate.id().clone(),
    };
    kind_mother::write(&ctx.environment_id(), write_intent).await?;
    Ok(())
}
```

### 2.4 Pattern : Enregistrement Paiement et Réconciliation

**Principe :** Décision StrongFather (si politique), WriteIntent paiement + réconciliation vers KindMother.

**Pseudo-code Rust :**

```rust
// Dans InvoicePayment
pub async fn record_payment(
    &self,
    ctx: &OperatorContext,
    payment: PaymentDraft,
    invoice_ids: Vec<InvoiceId>,
) -> Result<PaymentId, InvoiceError> {
    let mandate = ctx.mandate()?;
    master_butler::check_capability(&mandate, "payment.record").await?;

    if payment.amount > threshold() {
        let decision = strong_father::decide(
            &ctx.environment_id(),
            DecisionRequest::RecordPayment {
                amount: payment.amount,
                invoice_ids: invoice_ids.clone(),
            },
        ).await?;
        if let Decision::Rejected { reason } = decision {
            return Err(InvoiceError::RejectedByGovernance(reason));
        }
    }

    let write_intent = WriteIntent {
        operation: WriteOperation::RecordPaymentAndReconcile {
            payment,
            invoice_ids,
        },
        source: ctx.operator_id().clone(),
        mandate_id: mandate.id().clone(),
    };

    let payment_id = kind_mother::write(&ctx.environment_id(), write_intent).await?;
    Ok(payment_id)
}
```

---

## 3. Gestion des Mandats de Permission

### 3.1 Obtention d'un Mandat (InvoiceService)

**Pattern :** Demande de mandat à StrongFather avec opérateurs et capacités InvoiceService.

```rust
pub async fn ensure_invoice_mandate(
    ctx: &OperatorContext,
    capabilities: Vec<&str>,
) -> Result<Mandate, InvoiceError> {
    if let Some(mandate) = ctx.mandate() {
        if mandate.is_valid() && mandate.has_capabilities(&capabilities) {
            return Ok(mandate);
        }
    }

    let mandate = strong_father::request_mandate(
        &ctx.environment_id(),
        MandateRequest {
            operators: vec![
                ctx.operator_id().clone(),
                OperatorId::from("invoice.ledger"),
            ],
            capabilities: capabilities.into_iter().map(String::from).collect(),
            security_level: SecurityLevel::Sensitive,
            duration: Duration::hours(8),
        },
    ).await?;

    Ok(mandate)
}
```

### 3.2 Utilisation dans les Méthodes

**Pattern :** Vérification du mandat au début de chaque action (création, validation, envoi, paiement).

```rust
pub async fn create_invoice(&self, ctx: &OperatorContext, draft: InvoiceDraft) -> Result<InvoiceId, InvoiceError> {
    let mandate = self.ensure_invoice_mandate(ctx, &["invoice.create"]).await?;
    master_butler::check_capability_with_mandate(&mandate, "invoice.create").await?;
    // ... reste
}
```

---

## 4. Gestion des Erreurs et Rollback

### 4.1 Rollback sur Erreur

**Principe :** En cas d'échec après WriteIntent, KindMother gère le rollback. Les factures restent en état cohérent (brouillon ou validé selon l’étape).

**Exemple :** Si `validate_invoice` échoue après génération de la séquence mais avant WriteIntent, aucune écriture n’est persistée. Si WriteIntent échoue, KindMother annule la transaction.

### 4.2 Erreurs Métier

- `InvoiceError::Unbalanced` : Facture non équilibrée
- `InvoiceError::RejectedByGovernance(reason)` : StrongFather a refusé
- `InvoiceError::AlreadyPosted` : Facture déjà validée
- `InvoiceError::PaymentStateBlocked` : Facture bloquée ou déjà payée
- `InvoiceError::WriteFailed(e)` : Échec KindMother

---

## 5. Intégration avec Kits Existants

### 5.1 MiyuInvoice

- **Calcul des lignes** : `miyuinvoice::compute_lines` (quantité, prix, remise, taxes)
- **Calcul des totaux** : `miyuinvoice::compute_totals` (HT, TTC, taxes)
- **Génération PDF** : `miyuinvoice::render_pdf` (template facture)
- **Conditions de paiement** : Réutilisation des outils MiyuInvoice ou délégation à InvoiceTerms (Ever Buddy pour échéances si besoin)

### 5.2 MiyuComptaLedger (optionnel)

- Si Accounting est présent : comptabilisation des factures validées via AccountLedger / KindMother
- InvoiceLedger peut émettre des WriteIntent vers le grand livre (AccountLedger) ou déléguer à un service commun

### 5.3 Tests d'Intégration COG

- **Création facture** : InvoiceUI → InvoiceLedger → KindMother (WriteIntent) → lecture et vérification
- **Validation** : StrongFather décide → Ever Buddy séquence → KindMother WriteIntent → vérification numéro et état
- **Envoi** : InvoiceSend → PDF + email → WriteIntent "marquer envoyé" → vérification statut
- **Paiement** : InvoicePayment → WriteIntent réconciliation → vérification payment_state des factures

---

## 6. Conclusion

L’intégration COG pour **Invoicing** repose sur **InvoiceService** (InvoiceLedger, InvoicePayment, InvoiceSend, InvoiceTerms, InvoiceUI), **WriteIntent** vers **KindMother**, **décisions StrongFather** (validation, envoi, paiement selon politique), **séquences Ever Buddy** et **Mandats de Permission**. La réutilisation de **MiyuInvoice** pour les calculs et le PDF permet de ne pas dupliquer la logique facturation tout en respectant la gouvernance Miyukini.

**Prochaines étapes :** Voir [Guide d'Implémentation](../06_guides_implementation/Odoo%20Invoicing%20-%20Guide%20Implementation.md).

---

**Document** : Odoo Invoicing — Guide d'Intégration COG  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Référence pour implémentation Miyukini
