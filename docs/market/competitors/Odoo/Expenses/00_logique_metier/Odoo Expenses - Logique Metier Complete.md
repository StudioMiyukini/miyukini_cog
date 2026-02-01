# Odoo Expenses — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Expenses** (Notes de frais) d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/hr_expense`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèle principal hr.expense (dépense)
- Règles métier et contraintes (montant, approbation, édition)
- Workflow d'états (draft → submitted → approved → posted → paid / refused)
- Modes de paiement (employé à rembourser / entreprise)
- Calcul des montants, taxes, multi-devises
- Détection doublons et reçus identiques
- Split de dépenses, création depuis pièces jointes / email
- Intégration Accounting (écritures, paiements), HR (employé, département), Mail

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `hr.expense` (Dépense)

**Rôle :** Représente une **note de frais** — une dépense engagée par un employé, soumise à approbation et remboursement ou comptabilisation.

**Héritages :**
- `mail.thread.main.attachment` : Fil de discussion, pièce jointe principale
- `mail.activity.mixin` : Activités (ex. « Review this expense »)
- `analytic.mixin` : Répartition analytique

**Champs clés :**

#### Identification et description
- `name` : Description (compute + store + readonly=False, requis, copié)
- `date` : Date de la dépense (Date, défaut aujourd'hui)
- `description` : Notes internes (Text)
- `message_main_attachment_checksum` : Checksum pièce jointe principale (related)
- `nb_attachment` : Nombre de pièces jointes (compute)
- `attachment_ids` : Pièces jointes (One2many ir.attachment)

#### Employé et hiérarchie
- `employee_id` : Employé (Many2one hr.employee, requis, default _default_employee_id, check_company, domaine filter_for_expense, tracking)
- `department_id` : Département (compute depuis employee_id, store)
- `manager_id` : Manager responsable approbation (compute depuis employee_id, domaine selon droits, tracking)
- `company_id` : Société (requis, readonly, défaut env.company)

#### Catégorie et montants
- `product_id` : Catégorie (Many2one product.product, can_be_expensed, ondelete restrict) — optionnel en création via alias mail
- `product_description` : Description produit (compute)
- `product_uom_id` : Unité (compute depuis product_id)
- `product_has_cost` : Produit a un coût (standard_price) (compute)
- `product_has_tax` : Taxes définies sur le produit (compute)
- `quantity` : Quantité (requis, défaut 1, digits Product Unit)
- `price_unit` : Prix unitaire (compute, store, readonly, digits Product Price)
- `total_amount_currency` : Total en devise (compute, store, readonly=False, tracking)
- `total_amount` : Total en devise société (compute, inverse, store, tracking)
- `untaxed_amount_currency` / `untaxed_amount` : Total HT (compute)
- `tax_amount_currency` / `tax_amount` : Montant TVA (compute)
- `currency_id` : Devise (compute, store, requis, défaut company)
- `company_currency_id` : Devise société (related)
- `is_multiple_currency` : Dépense en devise étrangère (compute)
- `currency_rate` : Taux de change (compute, tracking)
- `label_currency_rate` : Libellé taux (compute)
- `tax_ids` : Taxes incluses (Many2many account.tax, type purchase, compute + store + readonly=False)
- `account_id` : Compte de charge (Many2one account.account, compute, domaine hors receivable/payable/cash/credit_card)

#### État et approbation
- `state` : Statut (Selection, compute store readonly, index, copy=False, default draft, tracking)
  - **Pre-approbation :** `draft`
  - **Approbation :** `submitted`, `approved`, `posted`
  - **Paiement :** `in_payment`, `paid`
  - **Refus :** `refused`
- `approval_state` : État d'approbation (submitted / approved / refused, copy=False, readonly)
- `approval_date` : Date d'approbation (Datetime, readonly)
- `is_editable` : Modifiable par l'utilisateur courant (compute)
- `can_reset` : Peut remettre en brouillon (compute)
- `can_approve` : Peut approuver (compute)

#### Comptabilité et paiement
- `journal_id` : Journal (related payment_method_line_id.journal_id)
- `selectable_payment_method_line_ids` : Méthodes de paiement sélectionnables (compute_sudo)
- `payment_method_line_id` : Méthode de paiement (compute, store, readonly=False, domaine)
- `account_move_id` : Écriture comptable / facture (Many2one account.move, readonly, index btree_not_null)
- `amount_residual` : Montant dû (related account_move_id.amount_residual)
- `payment_mode` : Payé par — `own_account` (employé à rembourser) / `company_account` (entreprise)
- `vendor_id` : Fournisseur (Many2one res.partner)

#### Sécurité et aide
- `duplicate_expense_ids` : Dépenses potentielles doublons (compute) — même employé, produit, date, montant, devise
- `same_receipt_expense_ids` : Dépenses avec même reçu (checksum pièce jointe) (compute)
- `split_expense_origin_id` : Dépense d'origine en cas de split (Many2one hr.expense)
- `former_sheet_id` : Ancien report (legacy, Integer)

**Règles métier :**
- `_check_non_zero` : Seules les dépenses en brouillon sans approval_state peuvent avoir un total à 0.
- `_check_o2o_payment` : Une seule dépense peut être liée à un paiement donné (origin_payment_id.expense_ids).
- `_unlink_except_approved` : Impossible de supprimer une dépense approuvée, postée, en paiement ou payée.
- Édition restreinte via `is_editable` (droits selon groupe, employé, manager, état).
- Champs de sécurité (`is_editable`, `can_approve`, etc.) non modifiables manuellement dans `write`.

**Workflow des états (state) :**
- **draft** : Brouillon, modifiable par l'employé.
- **submitted** : Soumis au manager, en attente d'approbation.
- **approved** : Approuvé, prêt à être posté en comptabilité.
- **posted** : Écriture comptable créée (in_receipt pour employé, ou paiement pour company).
- **in_payment** : Paiement en cours (employee-paid).
- **paid** : Payé / remboursé ou déjà payé par l'entreprise (company_account).
- **refused** : Refusé par le manager.

Le `state` est **calculé** à partir de `approval_state` et de `account_move_id` (état et payment_state du move). Priorité : move cancel → paid si company_account → sinon état du move (draft → posted, not_paid → posted, in_payment/partial → in_payment, sinon paid) ; sans move : state = approval_state ou draft.

---

## 2. Règles Métier Détaillées

### 2.1 Droits et approbation

**Groupes :**
- `group_hr_expense_team_approver` : Approbateur équipe (peut approuver sous conditions hiérarchie/département).
- `group_hr_expense_user` : Utilisateur notes de frais (tous les approbateurs).
- `group_hr_expense_manager` : Manager / admin (toujours éditer, approuver, reset).

**Règles (docstring HrExpense) :**
- **Submit** : Employee (ses propres dépenses) ; Officer (si manager de l'employé, manager du département ou expense manager de l'employé) ; Manager (toujours).
- **Approve** : Officer (pas les siennes, et doit être expense manager / manager / département) ; Manager (toujours).
- **Post** : Billing accountant, state == approved.
- **Cancel/Refuse** : Officer (mêmes conditions qu'approuver) ; Manager (toujours).
- **Reset** : Officer / Manager / employé concerné (selon `can_reset`), et uniquement si pas d'écriture postée.

**Calcul du responsable d'approbation (`_get_default_responsible_for_approval`)** :  
Expense manager de l'employé → Manager du département (si dans group approver) → Parent (team leader) → vide.

**Auto-validation** : Si pas de manager_id ni expense_manager_id, ou si manager = employé lui-même, la dépense peut être auto-validée à la soumission (`_can_be_autovalidated`).

### 2.2 Contraintes d'édition

- `is_editable` : True pour admin ; pour son propre brouillon ; pour manager/officer/approver sur dépenses des autres (draft/submitted/approved selon règles).
- Champs `tax_ids`, `analytic_distribution`, `account_id`, `manager_id` ne sont modifiables que si `is_editable` (ou sudo).

### 2.3 Calculs financiers

- **Prix unitaire** : En brouillon, soit depuis `product_id.standard_price` (si product_has_cost), soit total_amount / quantity.
- **Total en devise** : Pour produit avec coût : base ligne taxes (price_unit, quantity) → total TTC en devise. Sinon saisi manuellement (total_amount_currency).
- **Total société** : Si multi-devise : total_amount_currency * currency_rate puis recalcul taxes en devise société ; sinon total_amount = total_amount_currency.
- **Taux** : `_set_expense_currency_rate` utilise `res.currency._get_conversion_rate` (date = date de la dépense).
- **Taxes** : Toujours en mode « price included » pour les notes de frais ; `_prepare_base_line_for_taxes_computation` avec `special_mode='total_included'`, `rate=currency_rate`.

### 2.4 Comptabilité

- **Compte de charge** : `_get_base_account()` : account_id de la dépense → compte produit (expense) → company.expense_account_id → journal (purchase) default_account_id. Obligatoire avant post.
- **Employee-paid (own_account)** : Création d’un `account.move` type `in_receipt` par employé (ou regroupement), lignes depuis `_prepare_move_lines_vals`, partenaire = work_contact_id employé, `_prepare_receipts_vals` → `action_post` sur les moves.
- **Company-paid (company_account)** : `_create_company_paid_moves` crée un paiement (account.payment) + move associé ; une seule dépense par paiement (`_check_o2o_payment`).
- **Répartition analytique** : `_compute_analytic_distribution` via `account.analytic.distribution.model._get_distribution` (product, category, partner, account_prefix, company).

### 2.5 Doublons et reçus

- **duplicate_expense_ids** : Requête SQL sur même (employee_id, product_id, date, total_amount_currency, company_id, currency_id), COUNT > 1.
- **same_receipt_expense_ids** : Par checksum des pièces jointes (res_model=hr.expense) ; dépenses avec au moins une pièce jointe de même checksum (hors split_expense_origin_id).
- À l’approbation, si des doublons existent (state submitted/approved/posted/paid/in_payment), ouverture du wizard « approve duplicate » avant `_do_approve`.

### 2.6 Split

- `action_split_wizard` : Interdit si state in (posted, paid, in_payment) ou si non éditable. Crée `hr.expense.split` (deux lignes, moitié montant arrondi haut/bas) et `hr.expense.split.wizard`. Les nouvelles dépenses ont `split_expense_origin_id` = dépense courante.

### 2.7 Création depuis pièces jointes / email

- **create_expense_from_attachments** : Pièces jointes sans res_id/res_model ou res_model=hr.expense ; pour chaque pièce, création d’un hr.expense avec produit par défaut (EXP_GEN si existe), nom « Untitled Expense [date] », attachment lié et défini comme main attachment.
- **message_new** (alias mail) : Extraction employé depuis email, parsing du sujet (`_parse_expense_subject`) pour produit (code/nom), prix et devise → création dépense avec `_send_expense_success_mail`.

---

## 3. Actions Principales

| Action | Méthode | Rôle |
|--------|--------|------|
| Soumettre | `action_submit` | Passe en submitted (ou approved si autovalidation), met à jour activités / mails |
| Approuver | `action_approve` | Vérifie droits, distribution analytique, doublons éventuels, puis `_do_approve` |
| Refuser | `action_refuse` | Ouvre wizard de refus, puis `_do_refuse(reason)` |
| Post | `action_post` | Company-paid : création moves + paiements ; Employee-paid : ouverture wizard `hr.expense.post.wizard` |
| Payer | `action_pay` | Raccourci vers enregistrement paiement sur account_move_id |
| Reset | `action_reset` | Annulation des moves (reverse si postés), `_do_reset_approval` |
| Split | `action_split_wizard` | Wizard de partage en deux dépenses |
| Ouvrir écriture | `action_open_account_move` | Ouvre account.move (own_account) ou account.payment (company_account) |
| Créer depuis PJ | `create_expense_from_attachments` | Création une dépense par pièce jointe |

---

## 4. Intégration Mail et Activités

- **Activité « Review this expense »** : Planifiée en submitted pour `manager_id` (ou responsable par défaut) ; marquée faite en approved ; supprimée en draft/refused.
- **update_activities_and_mails** : Appelé après submit/approve/reset/refuse ; envoi possible d’emails aux managers (template submitted_expenses) selon version module.
- **Subtypes** : mt_expense_reset (draft), mt_expense_refused (refused), mt_expense_paid (paid), mt_expense_approved (approved), mt_expense_entry_draft / mt_expense_entry_delete (retour à approved depuis posted/paid).
- **get_empty_list_help** : Affiche astuce « Envoyer reçus par email » avec alias mailto si mailgateway activé.

---

## 5. Dashboard et Statistiques

- **get_expense_dashboard** : Pour l’utilisateur courant (employee_ids), agrège par state (draft, submitted, approved) les montants (total_amount) pour « To Submit », « Waiting Approval », « Waiting Reimbursement » (approved + own_account uniquement).

---

## 6. Synthèse pour Miyukini

- **Entité centrale** : Une dépense = un enregistrement type hr.expense avec workflow draft → submitted → approved → posted → paid / refused.
- **Deux flux paiement** : Employé à rembourser (in_receipt + paiement ultérieur) vs entreprise paie (paiement outbound direct).
- **Gouvernance** : Droits par groupe (employee, officer, manager) et hiérarchie (manager_id, expense_manager_id, département).
- **Comptabilité** : Intégration forte avec account (moves, payments, taxes, analytique).
- **Traçabilité** : Mail thread, activités, détection doublons/reçus, split tracé via split_expense_origin_id.

Ces éléments constituent la base d’un équivalent **MiyukiniExpenses** / **MiyuExpense** avec Opérateurs dédiés (dépense, approbation, comptabilité) et Mandats StrongFather / KindMother / Master Butler / WorrySentinel.
