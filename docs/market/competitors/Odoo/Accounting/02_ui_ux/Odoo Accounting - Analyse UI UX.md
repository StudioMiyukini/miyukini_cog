# Odoo Accounting — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Accounting** d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les composants d'interface, patterns de navigation, formulaires, tableaux, rapports et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/account/views`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (List, Kanban, Form, Calendar, Graph, Pivot)
- Composants d'interface spécialisés
- Patterns de navigation
- Formulaires et validations
- Tableaux et listes
- Rapports et exports
- Design responsive et accessibilité

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vues Principales

### 1.1 Vue Liste (List View) — `account.move.line`

**Fichier :** `account_move_views.xml` — `view_move_line_tree`

**Caractéristiques :**
- Vue principale pour les lignes d'écriture comptable (Journal Items)
- Colonnes configurables (optional="show"/"hide")
- Multi-édition supportée (`multi_edit="1"`)
- Expansion contextuelle (`expand="context.get('expand', False)"`)
- Groupement par partenaire, journal entry, compte

**Colonnes principales :**
- `date` : Date comptable (readonly)
- `move_name` : Journal Entry (widget `open_move_widget`)
- `account_id` : Compte comptable
- `partner_id` : Partenaire (optional)
- `name` : Libellé (optional)
- `debit` : Débit (sum="Total Debit", readonly)
- `credit` : Crédit (sum="Total Credit", readonly)
- `balance` : Solde (sum="Total Balance", optional)
- `amount_residual` : Résiduel (sum="Total Residual", optional)
- `date_maturity` : Date d'échéance (decoration-danger si overdue)
- `matching_number` : Numéro de réconciliation

**Décorations visuelles :**
- `decoration-info` : État draft (`parent_state == 'draft'`)
- `decoration-danger` : Échéance dépassée (`date_maturity < today`)
- `decoration-muted` : Montant à zéro (`debit == 0` ou `credit == 0`)

**Filtres principaux :**
- Unposted / Posted
- To Review
- Unreconciled / With residual
- Par type de journal (Sales, Purchases, Bank, Cash, Miscellaneous)
- Par type de compte (Payable, Receivable, P&L Accounts)
- Par date (Date, Invoice Date)

**Groupements :**
- Journal Entry
- Account
- Partner
- Journal
- Date / Invoice Date
- Taxes
- Tax Grid
- Matching

### 1.2 Vue Kanban — `account.move`

**Fichier :** `account_move_views.xml` — `view_account_move_kanban`

**Caractéristiques :**
- Vue mobile-friendly (`class="o_kanban_mobile"`)
- Classe JS spécialisée (`js_class="account_documents_kanban"`)
- Badge "To Review" pour les entrées non vérifiées
- Affichage montant total avec devise

**Structure de la carte :**
- En-tête : Partenaire ou Journal (fw-bolder fs-5)
- Badge "To Review" (si `checked=False` et `state='posted'`)
- Montant total aligné à droite (fw-bolder)
- Footer : Nom, Date, Activités, État (badge)

**Widgets spécialisés :**
- `kanban_activity` : Activités planifiées
- `label_selection` : État (draft/posted) avec classes CSS

### 1.3 Vue Formulaire (Form View) — `account.move`

**Fichier :** `account_move_views.xml` — `view_move_form`

**Caractéristiques :**
- Classe JS spécialisée (`js_class="account_move_form"`)
- Header avec boutons d'action contextuels
- Alertes et avertissements dynamiques
- Ribbons de statut (Sent, Paid, In Payment, Partial, Reversed, Blocked)
- Formulaire structuré en groupes et onglets

**Header — Boutons d'action :**
- **Post** (`action_post`) : Valider l'écriture (hotkey `q`)
- **Confirm** (`action_post`) : Confirmer facture (hotkey `q`)
- **Send** (`action_invoice_sent`) : Envoyer facture (hotkey `y`)
- **Print** (`action_print_pdf`) : Imprimer PDF
- **Pay** (`action_register_payment`) : Enregistrer paiement (hotkey `g`)
- **Preview** (`preview_invoice`) : Prévisualiser facture (hotkey `o`)
- **Reverse Entry** : Créer contre-passation (hotkey `z`)
- **Credit Note** (`action_reverse`) : Créer avoir (hotkey `shift+n`)
- **Cancel** (`button_cancel`) : Annuler (hotkey `x`)
- **Reset to Draft** (`button_draft`) : Remettre en brouillon (hotkey `r`)
- **Lock** (`button_hash`) : Verrouiller (si hash table activé)
- **Request Cancel** (`button_request_cancel`) : Demander annulation (hotkey `w`)
- **Reviewed** (`button_set_checked`) : Marquer comme vérifié

**Statusbar :**
- État : Draft → Posted
- Widget spécialisé pour sécurité (`account_move_statusbar_secured` si groupe `account.group_account_secured`)

**Alertes dynamiques :**
- Duplicatas détectés (`duplicated_ref_ids`)
- Outstanding credits/debits pour factures
- Avertissement devise inactive (`display_inactive_currency_warning`)
- Alertes calculées depuis `_get_alerts()` (affichées dans `<div id="alerts">`)

**Ribbons de statut :**
- **Sent** : Facture envoyée (gris)
- **Paid** : Payée (vert)
- **In Payment** : En cours de paiement
- **Partial** : Partiellement payée (gris)
- **Reversed** : Contre-passée
- **Blocked** : Bloquée (rouge)
- **Invoicing App Legacy** : Générée via ancienne app Invoicing (info)

**Structure du formulaire :**

1. **Button Box** (statistiques) :
   - Payments (nombre)
   - Reconciled Items
   - Cash Basis Entries
   - Adjusting Entries

2. **En-tête** :
   - Type de document (receipt_selector widget)
   - Nom (numéro) avec placeholder "/"
   - Avertissement si numéro inférieur au maximum

3. **Groupe gauche** :
   - Customer/Vendor (`partner_id` avec widget `res_partner_many2one`)
   - Adresse de livraison (`partner_shipping_id`)
   - Bill Reference (`ref` pour factures fournisseur)
   - Auto-Complete (`invoice_vendor_bill_id`)

4. **Groupe droit** :
   - Invoice Date / Bill Date (`invoice_date`)
   - Accounting Date (`date`)
   - Payment Reference (`payment_reference`)
   - Partner Bank (`partner_bank_id`)
   - Due Date / Payment Terms (`invoice_date_due`, `invoice_payment_term_id`)
   - Taxable Supply Date (`taxable_supply_date`)
   - Delivery Date (`delivery_date`)
   - Journal / Currency (avec conversion de devise)

5. **Onglet Invoice Lines** :
   - Widget spécialisé (`product_label_section_and_note_field_o2m`)
   - Mode list/kanban
   - Contrôles : Add a line, Add a section, Add a note, Catalog
   - Colonnes : Sequence, Product, Description, Account, Analytic, Quantity, UOM, Price, Discount, Taxes, Amount
   - Support sections et notes

6. **Onglet Journal Items** :
   - Liste des lignes d'écriture comptable
   - Édition inline
   - Colonnes : Account, Partner, Label, Debit, Credit, Balance, Date, Due Date

7. **Onglet Outstanding** :
   - Outstanding payments/credits pour réconciliation

8. **Onglet Payments** :
   - Liste des paiements associés

9. **Onglet Other Info** :
   - Informations complémentaires (company, tags, etc.)

**Widgets spécialisés :**
- `receipt_selector` : Sélecteur de type de document (horizontal)
- `res_partner_many2one` : Sélection partenaire avec recherche avancée
- `product_label_section_and_note_field_o2m` : Gestion lignes facture avec sections/notes
- `open_move_widget` : Lien vers journal entry avec ouverture
- `many2many_tax_tags` : Tags taxes avec couleurs
- `analytic_distribution` : Distribution analytique
- `monetary` : Montants avec devise
- `remaining_days` : Jours restants jusqu'à échéance
- `account_pick_currency_date` : Sélection date pour taux de change

### 1.4 Vue Pivot — `account.move.line`

**Fichier :** `account_move_views.xml` — `view_move_line_pivot`

**Caractéristiques :**
- Analyse par journal et date
- Mesure : Balance
- Sample data activé (`sample="1"`)

**Structure :**
- Rows : `journal_id`
- Columns : `date`
- Measure : `balance`

### 1.5 Vue Graph — `account.move.line`

**Fichier :** `account_move_views.xml` — `account_move_line_graph_date`

**Caractéristiques :**
- Graphique par date
- Mesure : Balance (opérateur `+`)
- Sample data activé

**Structure :**
- X-axis : `date`
- Y-axis : `balance` (operator="+")

### 1.6 Vue Calendar — Non présente pour `account.move.line`

**Note :** Odoo Accounting n'utilise pas de vue Calendar pour les lignes d'écriture, mais utilise plutôt des filtres par date dans la vue Liste.

---

## 2. Composants d'Interface Spécialisés

### 2.1 Widget `open_move_widget`

**Usage :** Affichage du nom de journal entry avec lien d'ouverture

**Caractéristiques :**
- Affiche le nom de l'écriture (`move_name`)
- Permet l'ouverture directe vers le formulaire
- Option `no_open` pour désactiver l'ouverture

### 2.2 Widget `many2many_tax_tags`

**Usage :** Affichage des tags taxes avec couleurs

**Caractéristiques :**
- Tags visuels colorés
- Readonly généralement
- Utilisé dans les lignes d'écriture et factures

### 2.3 Widget `analytic_distribution`

**Usage :** Distribution analytique des coûts

**Caractéristiques :**
- Widget interactif pour répartition analytique
- Support multi-comptes analytiques
- Options : `product_field`, `account_field`, `amount_field`
- Business domain : `invoice`, `bill`, `general`

### 2.4 Widget `product_label_section_and_note_field_o2m`

**Usage :** Gestion des lignes de facture avec sections et notes

**Caractéristiques :**
- Support sections (`line_section`)
- Support sous-sections (`line_subsection`)
- Support notes (`line_note`)
- Masquage composition/prices possible
- Contrôles : Add line, Add section, Add note, Catalog

### 2.5 Widget `account_move_form`

**Usage :** Formulaire d'écriture comptable avec logique métier

**Caractéristiques :**
- Gestion dynamique des boutons selon contexte
- Calcul automatique des alertes
- Gestion des outstanding payments
- Validation multi-devises

### 2.6 Widget `account_documents_kanban`

**Usage :** Vue Kanban spécialisée pour documents comptables

**Caractéristiques :**
- Badge "To Review" dynamique
- Affichage montant avec devise
- Gestion activités

---

## 3. Patterns de Navigation

### 3.1 Navigation Principale

**Menu Accounting :**
- Dashboard
- Customers → Invoices, Payments, Credit Notes
- Vendors → Bills, Payments, Credit Notes
- Accounting → Journal Entries, Journal Items, Chart of Accounts
- Reports → Financial Reports, Tax Reports
- Configuration

### 3.2 Navigation Contextuelle

**Depuis une facture :**
- Lien vers Journal Entry
- Lien vers Payments
- Lien vers Outstanding
- Lien vers Partner
- Lien vers Products (depuis lignes)

**Depuis une ligne d'écriture :**
- Lien vers Journal Entry (parent)
- Lien vers Account
- Lien vers Partner
- Lien vers Product (si applicable)
- Lien vers Analytic Lines

### 3.3 Actions Rapides

**Hotkeys :**
- `q` : Post/Confirm
- `y` : Send
- `g` : Pay
- `o` : Preview
- `z` : Reverse Entry
- `shift+n` : Credit Note
- `x` : Cancel
- `r` : Reset to Draft
- `w` : Request Cancel

---

## 4. Formulaires et Validations

### 4.1 Validation de Formulaire

**Validations automatiques :**
- Balance doit être équilibrée (débit = crédit)
- Date comptable cohérente avec date facture
- Devise active si multi-devises
- Compte compatible avec type de journal
- Taxes compatibles avec type de facture

**Messages d'erreur :**
- Alertes HTML dynamiques (`alerts` field)
- Messages inline sous les champs
- Popups pour erreurs bloquantes

### 4.2 Champs Conditionnels

**Visibilité conditionnelle :**
- `invisible="move_type not in ('out_invoice', ...)"` : Champs selon type
- `invisible="not invoice_has_outstanding"` : Outstanding si applicable
- `invisible="state != 'draft'"` : Champs éditables uniquement en draft
- `invisible="groups='...'"` : Champs selon groupes utilisateurs

**Readonly conditionnel :**
- `readonly="state != 'draft'"` : Non modifiable après validation
- `readonly="inalterable_hash != False"` : Verrouillé si hash activé
- `readonly="display_type != 'tax'"` : Certains champs selon type ligne

### 4.3 Auto-complétion

**Champs avec auto-complétion :**
- `partner_id` : Recherche partenaire avec contexte
- `product_id` : Recherche produit avec domaine selon type facture
- `account_id` : Recherche compte avec contexte partenaire
- `invoice_vendor_bill_id` : Auto-complétion depuis anciennes factures

---

## 5. Tableaux et Listes

### 5.1 Colonnes Configurables

**Système optional :**
- `optional="show"` : Visible par défaut, peut être masqué
- `optional="hide"` : Masqué par défaut, peut être affiché
- `optional="conditional"` : Affiché selon contexte

**Colonnes principales (move.line) :**
- Date, Journal Entry, Account, Partner, Label, Debit, Credit, Balance, Due Date, Matching

### 5.2 Multi-édition

**Support multi-édition :**
- `multi_edit="1"` activé sur liste
- Sélection multiple avec cases à cocher
- Édition groupée des champs compatibles
- Validation groupée

### 5.3 Groupement

**Groupements disponibles :**
- Par Journal Entry
- Par Account
- Par Partner
- Par Journal
- Par Date (jour, mois, année)
- Par Taxes
- Par Tax Grid
- Par Matching

### 5.4 Filtres Avancés

**Filtres prédéfinis :**
- Unposted / Posted
- To Review
- Unreconciled / With residual
- Par type journal (Sales, Purchases, Bank, Cash, Misc)
- Par type compte (Payable, Receivable, P&L)
- Par date (Date, Invoice Date, Due Date)
- Par devise (si multi-devises)

**Filtres personnalisés :**
- Recherche texte (nom, référence, partenaire, compte)
- Filtres de domaine complexes
- Filtres de date avec périodes prédéfinies

---

## 6. Rapports et Exports

### 6.1 Rapports PDF

**Rapports disponibles :**
- Invoice Report (`report_invoice.xml`)
- Payment Receipt Templates (`report_payment_receipt_templates.xml`)
- Statement Report (`report_statement.xml`)
- Report Templates (`report_templates.xml`)

**Caractéristiques :**
- Templates personnalisables
- Multi-langues
- Multi-entreprises
- Signatures et logos

### 6.2 Exports

**Formats d'export :**
- CSV (depuis liste)
- Excel (depuis liste)
- PDF (rapports)

**Données exportables :**
- Toutes les colonnes visibles
- Filtres appliqués
- Groupements préservés

---

## 7. Design Responsive et Accessibilité

### 7.1 Responsive Design

**Vue mobile :**
- Kanban mobile-friendly (`o_kanban_mobile`)
- Formulaires adaptatifs
- Navigation simplifiée
- Touch-friendly buttons

**Breakpoints :**
- Desktop : Vue complète
- Tablet : Colonnes adaptatives
- Mobile : Vue Kanban prioritaire

### 7.2 Accessibilité

**Hotkeys :**
- Navigation clavier complète
- Raccourcis clavier pour actions principales
- Focus management

**ARIA :**
- Labels appropriés
- Roles sémantiques
- États annoncés

**Contraste :**
- Couleurs de décoration cohérentes
- Badges avec contrastes suffisants
- Alertes visuelles claires

---

## 8. Recommandations pour Miyukini

### 8.1 Composants à Implémenter

**Vues principales :**
1. **Liste d'écritures** : Vue principale avec colonnes configurables
2. **Formulaire d'écriture** : Formulaire structuré avec onglets
3. **Kanban mobile** : Vue mobile-friendly
4. **Pivot** : Analyse par dimensions
5. **Graph** : Visualisations temporelles

**Widgets spécialisés :**
1. **Journal Entry Widget** : Lien vers écriture parente
2. **Tax Tags Widget** : Affichage tags taxes
3. **Analytic Distribution Widget** : Répartition analytique
4. **Product Lines Widget** : Gestion lignes avec sections/notes
5. **Outstanding Widget** : Gestion outstanding payments

### 8.2 Patterns à Adopter

**Navigation :**
- Menu hiérarchique clair
- Breadcrumbs contextuels
- Actions rapides accessibles
- Hotkeys pour actions fréquentes

**Formulaires :**
- Validation en temps réel
- Messages d'erreur clairs
- Auto-complétion intelligente
- Champs conditionnels

**Tableaux :**
- Colonnes configurables
- Multi-édition
- Groupements flexibles
- Filtres avancés

**Responsive :**
- Design mobile-first
- Vue Kanban pour mobile
- Navigation adaptative
- Touch-friendly

### 8.3 Améliorations Possibles

**UX :**
- Workflow guidé pour création écriture
- Assistant de réconciliation
- Prévisualisation avant validation
- Undo/Redo pour modifications

**Performance :**
- Lazy loading des lignes
- Pagination intelligente
- Cache des filtres fréquents
- Optimisation requêtes

**Accessibilité :**
- Support lecteurs d'écran complet
- Navigation clavier exhaustive
- Contraste amélioré
- Textes alternatifs complets

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
