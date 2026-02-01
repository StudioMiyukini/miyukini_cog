# Odoo Expenses — Guide Intégration COG

## Contexte

Ce document fournit un **guide pratique** pour intégrer les fonctionnalités Expenses (Notes de frais) dans l'architecture Miyukini COG, avec exemples de code pseudo-Rust et explications des patterns COG.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture d'intégration COG pour Expenses
- Patterns WriteIntent et Mandates (création, soumission, approbation, post)
- Exemples de code pseudo-Rust
- Gestion des gouvernances (StrongFather, KindMother, Master Butler, WorrySentinel)

---

## 1. Architecture d'Intégration

### 1.1 Vue d'ensemble

```
ExpenseUI → BondingBrother → ExpenseOperator ──► StrongFather (décision submit/approve/refuse/post)
                            ExpenseApprovalOperator   KindMother (WriteIntent)
                            ExpensePostOperator       Master Butler (permissions)
                                                      WorrySentinel (sécurité)
```

### 1.2 Flux typiques

1. **Création dépense** : Intention utilisateur → ExpenseUI → BondingBrother → ExpenseOperator → KindMother (WriteIntent Create).
2. **Soumission** : ExpenseOperator.action_submit → StrongFather (décision) → KindMother (WriteIntent Update state) → MiyuNotify (notification manager).
3. **Approbation** : ExpenseApprovalOperator.action_approve → StrongFather (décision) → Master Butler (can_approve) → KindMother (WriteIntent approval_state).
4. **Post** : ExpensePostOperator.action_post → StrongFather (décision) → KindMother (WriteIntent account.move / account.payment).

---

## 2. Patterns d'Intégration

### 2.1 Création de dépense

**Pattern :** WriteIntent Create + Mandate

```rust
// Pseudo-code Rust
pub struct CreateExpenseIntent {
    pub name: String,
    pub product_id: Option<ProductId>,
    pub date: Date,
    pub total_amount_currency: Decimal,
    pub currency_id: CurrencyId,
    pub quantity: Decimal,
    pub employee_id: EmployeeId,
    pub payment_mode: PaymentMode,
    pub attachment_ids: Vec<AttachmentId>,
    pub analytic_distribution: Option<AnalyticDistribution>,
}

impl ExpenseOperator {
    pub async fn create_expense(
        &self,
        intent: CreateExpenseIntent,
        mandate: Mandate,
    ) -> Result<Expense, ExpenseError> {
        mandate.validate_operators(&[self.id()])?;
        mandate.validate_flows(&["expense.create"])?;

        let permission = self.master_butler
            .check_permission(PermissionRequest {
                operator: self.id(),
                capability: "expense.create",
                resource: None,
            })
            .await?;
        if !permission.granted {
            return Err(ExpenseError::PermissionDenied);
        }

        let security_level = self.worry_sentinel
            .get_security_level(&intent)
            .await?;
        if security_level > mandate.max_security_level {
            return Err(ExpenseError::SecurityLevelExceeded);
        }

        let write_intent = WriteIntent {
            entity_type: "expense.expense",
            operation: WriteOperation::Create,
            data: ExpenseData {
                name: intent.name,
                product_id: intent.product_id,
                date: intent.date,
                total_amount_currency: intent.total_amount_currency,
                currency_id: intent.currency_id,
                quantity: intent.quantity,
                employee_id: intent.employee_id,
                payment_mode: intent.payment_mode,
                state: ExpenseState::Draft,
                attachment_ids: intent.attachment_ids,
                analytic_distribution: intent.analytic_distribution,
                company_id: self.get_company_id().await?,
            },
            security_level,
        };

        let expense = self.kind_mother.persist(write_intent).await?;
        Ok(expense)
    }
}
```

### 2.2 Soumission

**Pattern :** WriteIntent Update (state) + StrongFather (décision) + MiyuNotify

```rust
pub async fn submit_expenses(
    &self,
    expense_ids: Vec<ExpenseId>,
    mandate: Mandate,
) -> Result<(), ExpenseError> {
    mandate.validate_flows(&["expense.submit"])?;

    let expenses = self.kind_mother.load_expenses(&expense_ids).await?;
    for expense in &expenses {
        if expense.state != ExpenseState::Draft {
            return Err(ExpenseError::InvalidState);
        }
        if expense.product_id.is_none() {
            return Err(ExpenseError::CategoryRequired);
        }
    }

    let decision = self.strong_father
        .decide(DecisionRequest {
            action: "expense.submit",
            context: &expense_ids,
        })
        .await?;
    if !decision.allowed {
        return Err(ExpenseError::DecisionDenied);
    }

    let manager_id = self.resolve_manager_for_expenses(&expenses).await?;
    let update_intent = WriteIntent {
        entity_type: "expense.expense",
        operation: WriteOperation::Update,
        data: ExpenseData {
            state: ExpenseState::Submitted,
            approval_state: Some(ApprovalState::Submitted),
            manager_id: Some(manager_id),
            ..expenses[0].data
        },
        security_level: mandate.max_security_level,
    };

    self.kind_mother.persist(update_intent).await?;
    self.miyu_notify
        .notify_expense_submitted(manager_id, &expense_ids)
        .await?;
    Ok(())
}
```

### 2.3 Approbation

**Pattern :** StrongFather + Master Butler (can_approve) + KindMother (approval_state)

```rust
pub async fn approve_expenses(
    &self,
    expense_ids: Vec<ExpenseId>,
    mandate: Mandate,
) -> Result<(), ExpenseError> {
    mandate.validate_flows(&["expense.approve"])?;

    let expenses = self.kind_mother.load_expenses(&expense_ids).await?;
    for expense in &expenses {
        if expense.state != ExpenseState::Submitted {
            return Err(ExpenseError::InvalidState);
        }
        if !self.master_butler.can_approve_expense(self.user_id(), expense).await? {
            return Err(ExpenseError::CannotApprove);
        }
    }

    let decision = self.strong_father
        .decide(DecisionRequest {
            action: "expense.approve",
            context: &expense_ids,
        })
        .await?;
    if !decision.allowed {
        return Err(ExpenseError::DecisionDenied);
    }

    let update_intent = WriteIntent {
        entity_type: "expense.expense",
        operation: WriteOperation::Update,
        data: ExpenseData {
            state: ExpenseState::Approved,
            approval_state: Some(ApprovalState::Approved),
            approval_date: Some(Utc::now()),
            manager_id: Some(self.user_id()),
            ..expenses[0].data
        },
        security_level: mandate.max_security_level,
    };

    self.kind_mother.persist(update_intent).await?;
    self.miyu_notify
        .notify_expense_approved(&expense_ids)
        .await?;
    Ok(())
}
```

### 2.4 Post (comptabilisation)

**Pattern :** KindMother (WriteIntent account.move / account.payment) + StrongFather

```rust
pub async fn post_expenses(
    &self,
    expense_ids: Vec<ExpenseId>,
    mandate: Mandate,
) -> Result<Vec<MoveId>, ExpenseError> {
    mandate.validate_flows(&["expense.post"])?;

    let expenses = self.kind_mother.load_expenses(&expense_ids).await?;
    for expense in &expenses {
        if expense.state != ExpenseState::Approved {
            return Err(ExpenseError::OnlyApprovedCanBePosted);
        }
    }

    let decision = self.strong_father
        .decide(DecisionRequest {
            action: "expense.post",
            context: &expense_ids,
        })
        .await?;
    if !decision.allowed {
        return Err(ExpenseError::DecisionDenied);
    }

    let company_paid = expenses.iter().filter(|e| e.payment_mode == PaymentMode::CompanyAccount);
    let employee_paid = expenses.iter().filter(|e| e.payment_mode == PaymentMode::OwnAccount);

    let mut move_ids = Vec::new();
    for expense in company_paid {
        let (move_intent, payment_intent) = self.prepare_company_paid_move(expense).await?;
        let move_id = self.kind_mother.persist_move(move_intent).await?;
        self.kind_mother.persist_payment(payment_intent).await?;
        move_ids.push(move_id);
    }

    for (company_id, group) in employee_paid.group_by_company() {
        let receipt_intent = self.prepare_receipt_vals(group).await?;
        let move_id = self.kind_mother.persist_move(receipt_intent).await?;
        move_ids.push(move_id);
    }

    let update_intent = WriteIntent {
        entity_type: "expense.expense",
        operation: WriteOperation::Update,
        data: ExpenseData {
            state: ExpenseState::Posted, // ou Paid si company_account
            account_move_id: Some(move_ids[0]), // selon logique 1 move par employé
            ..
        },
        security_level: 3,
    };
    self.kind_mother.persist(update_intent).await?;

    Ok(move_ids)
}
```

### 2.5 Refus avec motif

**Pattern :** KindMother (approval_state refused) + MiyuNotify (motif)

```rust
pub async fn refuse_expenses(
    &self,
    expense_ids: Vec<ExpenseId>,
    reason: String,
    mandate: Mandate,
) -> Result<(), ExpenseError> {
    mandate.validate_flows(&["expense.refuse"])?;
    // ... can_approve check, StrongFather decision ...

    let update_intent = WriteIntent {
        entity_type: "expense.expense",
        operation: WriteOperation::Update,
        data: ExpenseData {
            state: ExpenseState::Refused,
            approval_state: Some(ApprovalState::Refused),
            ..
        },
        security_level: mandate.max_security_level,
    };
    self.kind_mother.persist(update_intent).await?;
    self.miyu_notify
        .notify_expense_refused(&expense_ids, &reason)
        .await?;
    Ok(())
}
```

---

## 3. Gestion des Gouvernances

- **StrongFather** : Toute action modifiant l’état (submit, approve, refuse, reset, post) nécessite une décision ; le Mandat porte la liste des flux autorisés.
- **KindMother** : Aucune persistance (dépense, move, payment) sans WriteIntent validé ; pas de write direct sur les entités gouvernées.
- **Master Butler** : can_approve, can_reset, is_editable dérivés des groupes et de la hiérarchie (expense_manager_id, department, parent) ; vérification avant approve/refuse/reset.
- **WorrySentinel** : Niveau sécurité 2 (dépenses, montants, pièces jointes), niveau 3 (post/paiements) ; Mandat.max_security_level respecté.
- **Ever Buddy** : Transitions state cohérentes ; reset interdit si account_move posté (cohérence avec Comptabilité).

---

## 4. Synthèse

L’intégration COG pour Expenses repose sur des Mandats explicites, des WriteIntent pour toute persistance, et des appels StrongFather pour submit/approve/refuse/post. Les exemples pseudo-Rust ci-dessus illustrent les patterns à répliquer dans l’implémentation réelle (crates MiyuExpense, MiyukiniCentral). Ce guide complète les spécifications Opérateurs Miyukini et le guide d’implémentation.
