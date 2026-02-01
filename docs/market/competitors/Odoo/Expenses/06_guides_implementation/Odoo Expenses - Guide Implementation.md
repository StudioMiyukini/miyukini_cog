# Odoo Expenses — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Expenses (Notes de frais) dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates)
- Spécifications des crates Rust
- Schémas de données (Expense, états, paiement)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyuexpense/                        # ExpenseOperator + ExpenseApprovalOperator
│   ├── src/
│   │   ├── lib.rs
│   │   ├── expense.rs                  # Modèle Expense, états
│   │   ├── approval.rs                 # Workflow approve/refuse/reset
│   │   ├── submit.rs                   # Soumission, manager resolution
│   │   ├── duplicate.rs                # Détection doublons / same receipt
│   │   ├── split.rs                    # Split wizard
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuexpense_post/                   # ExpensePostOperator (optionnel, ou dans miyuinvoice)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── post.rs                     # Post company / employee paid
│   │   ├── receipts.rs                 # _prepare_receipts_vals
│   │   ├── payments.rs                 # _prepare_payments_vals (company)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuexpense_category/               # ExpenseCategoryOperator (ou réutiliser product)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── category.rs                 # Catégories expensables
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyuexpense_ui/                     # ExpenseUI (frontend selon stack)
    ├── src/
    │   ├── lib.rs
    │   ├── dashboard.rs
    │   ├── views/
    │   │   ├── expense_list.rs
    │   │   ├── expense_kanban.rs
    │   │   ├── expense_form.rs
    │   │   └── wizards.rs
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)

**Kits existants :**
- `miyucontacts` / `miyuhr` : Employé, work_contact, hiérarchie
- `miyuinvoice` / `miyucptaledger` : Écritures, paiements, comptes, taxes
- `miyunotify` : Notifications, activités
- `miyumedia` : Pièces jointes
- `miyuvalidate` : Validation montants, devises

---

## 2. Schémas de Données

### 2.1 Modèle Expense

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: ExpenseId,
    pub name: String,
    pub date: Date,
    pub description: Option<String>,

    pub employee_id: EmployeeId,
    pub department_id: Option<DepartmentId>,
    pub manager_id: Option<UserId>,
    pub company_id: CompanyId,

    pub product_id: Option<ProductId>,
    pub product_uom_id: Option<UomId>,
    pub quantity: Decimal,
    pub price_unit: Decimal,
    pub total_amount_currency: Decimal,
    pub total_amount: Decimal,
    pub currency_id: CurrencyId,
    pub company_currency_id: CurrencyId,
    pub tax_ids: Vec<TaxId>,
    pub tax_amount_currency: Decimal,
    pub tax_amount: Decimal,
    pub untaxed_amount_currency: Decimal,
    pub untaxed_amount: Decimal,
    pub account_id: Option<AccountId>,
    pub analytic_distribution: Option<AnalyticDistribution>,

    pub state: ExpenseState,
    pub approval_state: Option<ApprovalState>,
    pub approval_date: Option<DateTime>,
    pub payment_mode: PaymentMode,
    pub payment_method_line_id: Option<PaymentMethodLineId>,
    pub account_move_id: Option<MoveId>,
    pub amount_residual: Option<Decimal>,
    pub vendor_id: Option<PartnerId>,

    pub attachment_ids: Vec<AttachmentId>,
    pub message_main_attachment_id: Option<AttachmentId>,
    pub split_expense_origin_id: Option<ExpenseId>,

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpenseState {
    Draft,
    Submitted,
    Approved,
    Posted,
    InPayment,
    Paid,
    Refused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalState {
    Submitted,
    Approved,
    Refused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMode {
    OwnAccount,   // Employee to reimburse
    CompanyAccount,
}
```

### 2.2 Contraintes et règles

- **state** : Calculé ou persistant selon choix (compute côté serveur ou stocké).
- **approval_state** : Soumis / Approuvé / Refusé ; cohérent avec state.
- **Un seul account_move_id par dépense** pour employee-paid ; une dépense par paiement pour company-paid.
- **work_contact_id** obligatoire sur employé pour post own_account.
- **product_id** obligatoire à la soumission (pas en draft si création par email).

---

## 3. API et Contrats

### 3.1 ExpenseOperator

- `create_expense(intent, mandate) -> Result<Expense>`
- `update_expense(id, intent, mandate) -> Result<Expense>`
- `submit_expenses(ids, mandate) -> Result<()>`
- `create_from_attachments(attachment_ids, mandate) -> Result<Vec<ExpenseId>>`
- `split_expense(id, mandate) -> Result<(ExpenseId, ExpenseId)>`
- `get_expense_dashboard(user_id) -> Result<ExpenseDashboard>`
- `list_duplicate_expense_ids(expense_id) -> Result<Vec<ExpenseId>>`
- `list_same_receipt_expense_ids(expense_id) -> Result<Vec<ExpenseId>>`

### 3.2 ExpenseApprovalOperator

- `approve_expenses(ids, mandate) -> Result<()>`
- `refuse_expenses(ids, reason, mandate) -> Result<()>`
- `reset_expenses(ids, mandate) -> Result<()>`
- `get_default_responsible_for_approval(expense_id) -> Result<Option<UserId>>`
- `can_approve(user_id, expense_id) -> Result<bool>`
- `can_reset(user_id, expense_id) -> Result<bool>`

### 3.3 ExpensePostOperator

- `post_expenses(ids, mandate) -> Result<Vec<MoveId>>`
- `register_payment(expense_id, mandate) -> Result<PaymentId>`
- `prepare_receipt_vals(expense_ids) -> Result<ReceiptVals>`
- `prepare_payments_vals(expense) -> Result<(MoveVals, PaymentVals)>`

### 3.4 ExpenseCategoryOperator

- `list_expensable_categories(company_id) -> Result<Vec<Product>>`
- `get_defaults(product_id, company_id) -> Result<(Decimal, Vec<TaxId>, AccountId)>`

---

## 4. Plan de Développement par Phases

### Phase 1 — MVP (2–3 semaines)

- **Modèle Expense** : Champs essentiels (name, date, employee_id, product_id, total_amount_currency, currency_id, state, approval_state, payment_mode).
- **ExpenseOperator** : create, update, submit (sans rapports ; dépense par dépense).
- **ExpenseApprovalOperator** : approve, refuse, reset (can_approve, can_reset basiques).
- **ExpenseUI** : Dashboard 3 blocs, liste Mes dépenses, formulaire dépense, boutons Submit / Approve / Refuse / Reset.
- **Intégration** : KindMother (persistance), StrongFather (décision), Master Butler (permissions basiques), MiyuNotify (notifications simples).
- **Hors scope MVP** : Post comptabilité, création depuis PJ/email, split, doublons/same receipt, multi-devises avancé, rapports.

### Phase 2 — Comptabilité et post (2 semaines)

- **ExpensePostOperator** : post (company_account + own_account), register_payment.
- **Intégration** : MiyuInvoice / MiyuCptaLedger (moves, payments).
- **Contraintes** : work_contact_id, compte de charge, une dépense par paiement (company).
- **UI** : Bouton Post, Pay ; ouverture écriture / paiement.

### Phase 3 — Confort et traçabilité (1–2 semaines)

- **Création depuis pièces jointes** : create_from_attachments.
- **Création depuis email** : alias, message_new, _parse_expense_subject.
- **Split** : action_split_wizard, hr.expense.split.
- **Doublons / same receipt** : duplicate_expense_ids, same_receipt_expense_ids ; alertes UI et wizard approve duplicate.
- **Multi-devises** : currency_rate, label_currency_rate, calculs total_amount.
- **Rapports** (optionnel) : regroupement « rapport » pour soumission en lot (modèle sheet ou agrégat UI).

### Phase 4 — Configuration et évolutions (1 semaine)

- **ExpenseCategoryOperator** : catégories expensables, prix/taxes/compte par défaut.
- **Configuration société** : expense_account_id, expense_journal_id, company_expense_allowed_payment_method_line_ids.
- **Activités** : « Review this expense » assignée au manager_id en submitted ; feedback / unlink selon état.
- **Tests** : Unitaires (états, calculs, droits), intégration (submit → approve → post).

---

## 5. Bornage Fonctionnel

### MVP (livrable Phase 1)

- Saisie dépense (catégorie, date, montant, employé, payé par).
- Soumission dépense (draft → submitted).
- Approbation / refus (submitted → approved / refused).
- Reset (submitted / approved → draft) si pas d’écriture postée.
- Dashboard (To Submit, Waiting Approval, Waiting Reimbursement).
- Notifications basiques (soumis, approuvé, refusé).

### Complet (livrable Phase 4)

- Tout le MVP.
- Post (company_account et own_account), remboursement (action_pay).
- Création depuis pièces jointes et email.
- Split, alertes doublons et same receipt.
- Multi-devises, taxes, analytique.
- Rapports de dépenses (regroupement soumission).
- Configuration catégories et société.
- Activités et historique complets.

---

## 6. Synthèse

Le guide d’implémentation Expenses repose sur les crates **miyuexpense**, **miyuexpense_post**, **miyuexpense_category** et **miyuexpense_ui**, avec schémas Expense/ExpenseState/ApprovalState/PaymentMode, API et contrats définis, et un plan en 4 phases (MVP → Complet). Le bornage MVP vs Complet permet de livrer une première version utilisable (saisie, soumission, approbation) puis d’ajouter comptabilité, confort et configuration. Ce document complète la logique métier, les parcours utilisateur, l’UI/UX, les intégrations cross-app, les spécifications Opérateurs Miyukini et le guide d’intégration COG pour Expenses.
