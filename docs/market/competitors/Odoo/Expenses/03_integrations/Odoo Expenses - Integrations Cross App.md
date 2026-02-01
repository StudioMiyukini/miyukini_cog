# Odoo Expenses — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Expenses** (Notes de frais) d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Code source GitHub Odoo 19.0 (hr_expense __manifest__, models, dépendances)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres modules Odoo (hr, account, mail, product, analytic)
- Flux de données inter-apps
- Mécanismes d'intégration (héritages, related, compute, appels)
- APIs et hooks utilisés
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules requis (__manifest__.py)

**Dépendances explicites typiques :**
- **base** : Modèles de base, res.company, res.users
- **hr** : hr.employee, hr.department (employé, département, expense_manager_id, filter_for_expense)
- **account** : account.move, account.payment, account.journal, account.account, account.tax, account.payment.method.line (écritures, paiements, comptes, taxes)
- **mail** : mail.thread, mail.activity.mixin, chatter, notifications, activités
- **product** : product.product (catégories expensables, can_be_expensed, standard_price, uom_id, supplier_taxes_id)
- **analytic** : analytic.mixin (analytic_distribution), account.analytic.distribution.model

**Optionnel selon versions :**
- **web** : Interface web, actions, vues
- **base_setup** : Paramètres société (company_expense_*, use_mailgateway)

### 1.2 Flux de données

```
HR (employee, department) ──► hr.expense (employé, manager, département)
Product (catégories)       ──► hr.expense (product_id, prix, taxes, compte)
Analytic                   ──► hr.expense (analytic_distribution)
Mail                       ──► hr.expense (chatter, activités, alias)
Account                    ◄── hr.expense (account_move_id, paiements, écritures)
```

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec HR (hr)

**Flux :**
- **Employé** : hr.expense.employee_id (requis), département et manager dérivés (department_id, manager_id) ; default_employee_id depuis user.employee_id ; domaine filter_for_expense.
- **Manager / approbation** : manager_id = expense_manager_id ou department.manager_id ou parent_id (team leader) ; groupe group_hr_expense_team_approver pour périmètre « child_of user.employee_ids ».
- **Contact travail** : employee_id.work_contact_id pour in_receipt (partner_id), partner_bank_id (primary_bank_account_id) ; obligatoire pour poster en own_account.

**Modèles consommés :**
- hr.employee (employee_id, department_id, expense_manager_id, parent_id, user_id, work_contact_id, company_id)
- hr.department (manager_id)

**Recommandations Miyukini :**
- Opérateur Expenses consomme un équivalent « Employé » (MiyuHR ou MiyuContacts) pour identité, hiérarchie et contact de remboursement.
- Décision d’approbation (StrongFather) s’appuie sur une notion de manager / expense_manager / département cohérente avec la structure RH.

### 2.2 Intégration avec Account (account)

**Flux :**
- **Écritures** : hr.expense.account_move_id (One2many inverse : expense_ids sur account.move). Création soit in_receipt (employee-paid) via _prepare_receipts_vals / _prepare_move_lines_vals, soit paiement + move (company_account) via _create_company_paid_moves / _prepare_payments_vals.
- **Paiements** : account.payment (origin_payment_id sur move) ; une dépense par paiement (_check_o2o_payment). action_pay = account_move_id.action_register_payment.
- **Comptes** : account_id (compte de charge), _get_base_account() (dépense → produit → société → journal purchase). Domaine account : hors receivable, payable, cash, credit_card.
- **Taxes** : account.tax (type purchase), _prepare_base_line_for_taxes_computation, price included.
- **Journaux** : journal_id (related payment_method_line_id.journal_id) ; selectable_payment_method_line_ids depuis company_expense_allowed_payment_method_line_ids ou recherche outbound actifs.
- **État** : state dépense calculé depuis account_move_id.state et payment_state (draft → posted, not_paid → posted, in_payment/partial → in_payment, sinon paid).

**Modèles consommés / créés :**
- account.move (in_receipt, draft/posted/cancel)
- account.payment (company_account)
- account.journal, account.account, account.tax, account.payment.method.line
- account.analytic.distribution.model (répartition analytique)

**Recommandations Miyukini :**
- Intégration forte avec module Comptabilité (MiyuInvoice / MiyuCptaLedger) : WriteIntent pour création moves et paiements ; pas de post sans Mandat et sans état approved.
- Une dépense = une ou plusieurs lignes move + un paiement optionnel ; contrainte « une dépense par paiement » peut être reproduite ou assouplie selon besoin.

### 2.3 Intégration avec Mail (mail)

**Flux :**
- **Thread** : Héritage mail.thread.main.attachment ; message_ids, message_main_attachment_id (reçu principal), follower_ids.
- **Activités** : Héritage mail.activity.mixin ; activité « Review this expense » (mail_act_expense_approval) en submitted pour manager_id ; feedback en approved ; unlink en draft/refused.
- **Alias** : message_new sur alias dépenses ; _get_employee_from_email, _parse_expense_subject (produit, prix, devise) ; création dépense + _send_expense_success_mail.
- **Tracking** : state, employee_id, manager_id, total_amount_currency, total_amount, payment_mode (tracking=True).
- **Subtypes** : mt_expense_reset, mt_expense_refused, mt_expense_paid, mt_expense_approved, mt_expense_entry_draft, mt_expense_entry_delete.

**Modèles / données :**
- mail.message, mail.activity, mail.mail
- ir.attachment (res_model=hr.expense, res_id)
- mail.alias (mail_alias_expense)

**Recommandations Miyukini :**
- MiyuNotify pour notifications (soumis, approuvé, refusé, payé) et activités « À approuver ».
- Fil de discussion et pièces jointes via Opérateur ou Kit commun (MiyuMedia / chatter équivalent) ; création par email = façade optionnelle (BondingBrother traduit email → intention création dépense).

### 2.4 Intégration avec Product (product)

**Flux :**
- **Catégories** : product_id (product.product) avec can_be_expensed = True ; domaine sur les vues.
- **Prix** : price_unit depuis product.standard_price (product_has_cost) ou total_amount/quantity ; product_uom_id = product_id.uom_id.
- **Taxes** : tax_ids depuis product_id.supplier_taxes_id (filtré société).
- **Compte** : account_id depuis product.product_tmpl_id._get_product_accounts()['expense'].
- **Description** : product_description (compute) pour aide à l’affichage.

**Modèles consommés :**
- product.product (product_id, can_be_expensed, standard_price, uom_id, supplier_taxes_id, product_tmpl_id)
- uom.uom (product_uom_id)

**Recommandations Miyukini :**
- Catalogue « Catégories de dépenses » (produits ou entité dédiée) avec flag expensable ; lien vers compte de charge et taxes par défaut.
- Master Butler peut exposer la capacité « category.expense » ; KindMother persiste la dépense avec référence catégorie.

### 2.5 Intégration avec Analytic (analytic)

**Flux :**
- **Mixin** : analytic.mixin sur hr.expense ; champ analytic_distribution.
- **Calcul** : _compute_analytic_distribution depuis account.analytic.distribution.model._get_distribution (product_id, product_categ_id, partner_id=work_contact_id, account_prefix=account_id.code, company_id).
- **Propagation** : analytic_distribution copié sur les lignes d’écriture (_prepare_move_lines_vals, _prepare_payments_vals / base_line).

**Modèles consommés :**
- account.analytic.distribution.model
- account.analytic.account (lignes analytiques créées côté account.move.line)

**Recommandations Miyukini :**
- Répartition analytique optionnelle ; calcul par règles (projet, centre, contrat) équivalent distribution model.
- Intégration avec module Analytique / Projet (MiyuComptaReports, MiyukiniProject) pour coûts par projet ou centre.

---

## 3. Hooks et Méthodes Partagées

### 3.1 Comptabilité

- **res.currency** : _get_conversion_rate (date, company) pour currency_rate.
- **account.tax** : _prepare_base_line_for_taxes_computation, _add_tax_details_in_base_line, _round_base_lines_tax_details, _prepare_tax_lines, _add_accounting_data_in_base_lines_tax_details.
- **account.move** : action_post, _reverse_moves (reset) ; payment_state, amount_residual.
- **account.payment** : action_post (company-paid).

### 3.2 Produit

- **product.product** : _get_product_accounts()['expense'], _price_compute('standard_price', uom, company).

### 3.3 Mail

- **mail.thread** : message_post, _message_set_main_attachment_id, _track_subtype, _message_auto_subscribe_followers.
- **mail.activity** : activity_schedule, activity_feedback, activity_unlink.

### 3.4 Configuration

- **res.company** : expense_account_id, expense_journal_id, company_expense_allowed_payment_method_line_ids.
- **ir.config_parameter** : hr_expense.use_mailgateway.
- **ir.attachment** : res_model, res_id, checksum (same_receipt_expense_ids).

---

## 4. Synthèse des Dépendances

| Module   | Rôle principal |
|----------|----------------|
| hr       | Employé, département, manager, work_contact |
| account  | Écritures, paiements, comptes, taxes, journaux |
| mail     | Chatter, activités, alias, tracking |
| product  | Catégories expensables, prix, taxes, compte |
| analytic | Répartition analytique |

---

## 5. Recommandations pour Miyukini

- **Équipe d’Opérateurs** : ExpenseOperator (dépenses), ApprovalOperator (workflow approbation), et intégration avec Opérateur Comptabilité (écritures/paiements), Opérateur RH/Contacts (employé, manager), Opérateur Produit (catégories), Opérateur Analytique (répartition).
- **Contrats d’équipe** : Flux explicites Expense → KindMother (persistance), Expense → StrongFather (approbation), Expense → Comptabilité (post), Expense → MiyuNotify (notifications).
- **Mandats** : Post et Approve sous Mandat ; création écritures et paiements uniquement après décision StrongFather et persistance KindMother (WriteIntent).
- **Sécurité** : WorrySentinel sur niveau de données (montants, pièces jointes) ; Master Butler sur capacités submit/approve/post/reset selon groupe et hiérarchie.

Ces recommandations alimentent les spécifications Opérateurs Miyukini et le guide d’intégration COG pour Expenses.
