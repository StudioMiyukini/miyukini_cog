# Odoo Expenses — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Expenses** (Notes de frais) d'Odoo (version 19.0), à partir du code source et de la documentation. Il identifie les vues, composants, patterns de navigation et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/hr_expense` (views, assets, static)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dashboard et entrées de menu
- Vues principales (List, Kanban, Form)
- Wizards (Refus, Post, Split, Duplicate)
- Composants et widgets (montants, statut, pièces jointes)
- Patterns de navigation et feedback
- Recommandations pour Miyukini

**Hors scope :**
- Implémentation technique détaillée (guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Structure de Navigation

### 1.1 Menu principal

- **Expenses** (app racine)
  - **My Expenses** (tableau de bord par défaut)
    - **My Expenses** : liste des dépenses de l’utilisateur (employé)
    - **My Reports** : liste des rapports de dépenses (si modèle report/sheet présent)
  - **To Process** : dépenses à approuver (approbateurs)
  - **All Expenses** : toutes les dépenses (droits étendus)
  - **Configuration** : catégories (produits expensables), règles société

### 1.2 Dashboard (get_expense_dashboard)

- **My Expenses** : trois blocs par statut
  - **To Submit** (draft) : montant total des dépenses en brouillon / à rapporter
  - **Waiting Approval** (submitted) : montant en attente d’approbation
  - **Waiting Reimbursement** (approved, own_account) : montant approuvé non encore payé
- Couleur / libellé selon statut (doc : To Report en bleu, autres en noir).
- **Create Report** : visible dès qu’il existe au moins une dépense To Report ou To Submit ; crée un rapport et y ajoute toutes les dépenses To Submit non déjà sur un rapport (ou sélection manuelle selon UX).

---

## 2. Vues Principales — Dépenses (hr.expense)

### 2.1 Vue Liste

**Usage :** Mes dépenses, To Process, All Expenses.

**Colonnes typiques :**
- **Description** (name)
- **Employee** (employee_id) — masqué en « My Expenses »
- **Category** (product_id)
- **Date** (date)
- **Total** (total_amount ou total_amount_currency) — format monétaire
- **Status** (state) — badge ou sélection (Draft, Submitted, Approved, Posted, In Payment, Paid, Refused)
- **Manager** (manager_id) — pour approbateurs
- **Company** (company_id) — multi-société
- **Paid By** (payment_mode) — Employee / Company
- **Journal** (journal_id) — optionnel
- **Attachment** : indicateur pièce jointe (icône trombone si attachment_ids)

**Filtres courants :**
- My Expenses (employee_id in user.employee_ids)
- To Submit (state = draft)
- To Process (state = submitted, pour approbateurs)
- Waiting Reimbursement (state = approved, payment_mode = own_account)
- By Status, By Employee, By Department, By Period

**Groupements :**
- Par statut (state)
- Par employé (employee_id)
- Par manager (manager_id)
- Par département (department_id)

**Actions de liste :**
- Submit (action_submit) — dépenses draft
- Approve (action_approve) — dépenses submitted
- Refuse (action_refuse) — dépenses submitted
- Post (action_post) — dépenses approved
- Reset to Draft (action_reset) — selon can_reset

### 2.2 Vue Kanban

**Usage :** Vue par statut (colonnes Draft, Submitted, Approved, Posted, Paid, Refused).

**Carte type :**
- En-tête : Catégorie (product_id) ou nom (name)
- Corps : Montant (total_amount_currency), Date, Employé (si pas « My Expenses »)
- Footer : Statut, Manager, activité « Review » si submitted
- Pièce jointe : miniature ou icône si message_main_attachment_id

**Drag & drop :** Déplacement entre colonnes pour changer d’état (soumission, approbation) si droits et workflow le permettent.

### 2.3 Vue Formulaire

**Structure type :**
- **En-tête** : Statut (state) en badge, boutons d’action (Submit, Approve, Refuse, Post, Reset, Split, Open Journal Entry)
- **Onglets** :
  - **Expense** (ou principal) :
    - Description (name), Catégorie (product_id), Date (date)
    - Employé (employee_id), Département (department_id), Manager (manager_id)
    - Quantity, Unit Price, Total (total_amount_currency / total_amount), Currency, Tax Included
    - Paid By (payment_mode) : Employee (to reimburse) / Company
    - Payment Method (payment_method_line_id) — si company_account
    - Vendor (vendor_id) — optionnel
    - Account (account_id), Taxes (tax_ids), Analytic Distribution (analytic_distribution)
    - Internal Notes (description)
  - **Receipt** : Pièce jointe principale (message_main_attachment_id), liste des pièces jointes (attachment_ids) ; zone glisser-déposer pour « Attach receipt »
  - **Chatter** : Messages, activités, historique

**Widgets spécifiques :**
- Montant : monetary (currency_id)
- Statut : badge coloré (state)
- Product : many2one avec domaine can_be_expensed
- Manager : many2one avec domaine selon groupes (share=False, expense_manager ou group_hr_expense_team_approver)

**Aides et contraintes :**
- Message « Tip: try sending receipts by email » (get_empty_list_help) avec lien mailto alias si activé.
- Alertes doublons / same receipt : liens vers duplicate_expense_ids et same_receipt_expense_ids (action_show_same_receipt_expense_ids, wizard approve duplicate).

---

## 3. Wizards

### 3.1 Refuse (hr.expense.refuse.wizard)

- **Champ** : Motif de refus (reason) — texte ou HTML.
- **Action** : Confirmation → _do_refuse(reason) sur les dépenses sélectionnées ; message posté avec template refuse_reason ; approval_state = refused ; activités mises à jour.

### 3.2 Post (hr.expense.post.wizard)

- **Contexte** : Dépenses approved, payment_mode = own_account (employee-paid).
- **Rôle** : Création des account.move (in_receipt) et action_post ; une entrée par employé (ou regroupement).
- **Champs possibles** : Résumé, journal (si choix proposé) ; pas de champs obligatoires supplémentaires dans le code analysé pour le wizard minimal.

### 3.3 Split (hr.expense.split.wizard)

- **Entrée** : Une dépense éditable, non postée/paid/in_payment.
- **Contenu** : Lignes hr.expense.split (deux par défaut : moitié montant arrondi haut/bas) ; liaison expense_id.
- **Action** : Création de nouvelles dépenses avec split_expense_origin_id = dépense courante ; suppression ou annulation de l’origine selon logique métier.

### 3.4 Approve Duplicate (hr.expense.approve.duplicate)

- **Déclenchement** : Lors de action_approve si duplicate_expense_ids non vides et state in (submitted, approved, posted, paid, in_payment).
- **Rôle** : Confirmer que les dépenses sont bien distinctes ; action_approve_duplicates enregistre un message de confirmation puis _do_approve.

---

## 4. Composants et Patterns

### 4.1 Pièces jointes et reçus

- **Main attachment** : Une pièce jointe peut être définie comme « reçu » principal (attach_document, _message_set_main_attachment_id).
- **Création depuis fichiers** : create_expense_from_attachments ; une dépense par pièce ; produit par défaut (EXP_GEN) ; nom « Untitled Expense [date] ».
- **Affichage** : Icône trombone en liste ; onglet Receipt en formulaire ; preview si type image/PDF.

### 4.2 Montants et devises

- **Multi-devises** : Affichage total_amount_currency + currency_id ; label_currency_rate (ex. « 1 USD = 0.92 EUR »).
- **Taxes** : Toujours « price included » côté métier ; affichage HT/TTC si besoin (untaxed_amount, tax_amount, total_amount).
- **Format** : Monetary avec company_currency_id ou currency_id selon contexte.

### 4.3 Statut et workflow

- **Badge state** : Draft (gris), Submitted (orange), Approved (vert), Posted (bleu), In Payment (bleu clair), Paid (vert foncé), Refused (rouge).
- **Boutons conditionnels** : Submit (draft), Approve/Refuse (submitted), Post (approved), Pay (posted own_account), Reset (draft/submitted/approved selon can_reset).
- **Activité** : « Review this expense » assignée au manager_id en submitted ; disparaît en approved ; supprimée en draft/refused.

### 4.4 Feedback et messages

- **Erreurs** : ValidationError (ex. « You can not submit an expense without a category », « Only draft expenses can have a total of 0 », « You cannot delete a posted or approved expense »).
- **Message édition** : « Uh-oh! You can’t edit this expense. Reach out to the administrators… » si modification champs protégés sans is_editable.
- **Empty list** : get_empty_list_help + astuce envoi reçus par email (mailto avec sujet exemple « Lunch with customer $12.32 »).

---

## 5. Rapports de dépenses (Expense Reports)

- **My Reports** : Liste de rapports (modèle type hr.expense.sheet ou équivalent selon version) ; colonnes Summary, Employee, Paid By, Company, Manager, Journal, Status.
- **Formulaire rapport** : Onglet Expense (lignes de dépenses) ; Expense Report Summary ; Manager, Journal ; bouton Submit To Manager.
- **Règles** : Seules les dépenses To Submit peuvent être ajoutées ; pas d’ajout de dépenses Approved ; soumission rapport par rapport (pas de batch).

---

## 6. Configuration

- **Catégories** : Produits (product.product) avec can_be_expensed = True ; configuration dans Expenses → Configuration ou Inventaire/Produits.
- **Société** : expense_account_id, expense_journal_id, company_expense_allowed_payment_method_line_ids (company_account).
- **Alias email** : use_mailgateway, mail_alias_expense pour création par email.

---

## 7. Recommandations pour Miyukini

### 7.1 Structure

- **Dashboard** : Trois blocs (To Submit, Waiting Approval, Waiting Reimbursement) avec montants et liens vers listes filtrées.
- **Vues Liste / Kanban / Formulaire** : Aligner colonnes et champs sur hr.expense ; statut et boutons conditionnels selon Mandat (Master Butler) et état (Ever Buddy).

### 7.2 UX

- **Saisie rapide** : Formulaire compact + zone pièce jointe (glisser-déposer) ; option « Créer depuis reçu » (équivalent create_expense_from_attachments).
- **Alertes** : Afficher clairement same_receipt et duplicate (Caring Nanny / WorrySentinel) sans bloquer par défaut ; confirmation explicite à l’approbation si doublons.
- **Wizards** : Refus (motif obligatoire), Post (employee-paid), Split (deux lignes), Approve duplicate (confirmation) ; tous avec traçabilité côté COG.

### 7.3 Accessibilité et responsive

- **Labels** : Tous les champs obligatoires et conditionnels (payment_method si company_account, etc.) avec labels et messages d’erreur explicites.
- **Mobile** : Kanban et formulaire adaptés (champs empilés, boutons d’action visibles).
- **Raccourcis** : Actions principales (Submit, Approve, Post) accessibles au clavier et depuis la liste (actions groupées).

### 7.4 Cohérence

- **Terminologie** : To Submit / Waiting Approval / Waiting Reimbursement (ou équivalents français) alignés avec les états state et approval_state.
- **Couleurs** : Palette cohérente Draft / Submitted / Approved / Posted / Paid / Refused pour badges et Kanban.

Ces recommandations alimentent le guide d’implémentation (composants UI, routes, états) et les spécifications Opérateurs Miyukini pour l’Opérateur d’interface Expenses.
