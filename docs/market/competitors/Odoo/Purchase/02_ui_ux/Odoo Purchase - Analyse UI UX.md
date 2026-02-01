# Odoo Purchase — Analyse UI/UX

## Contexte

Ce document analyse l'**interface utilisateur et l'expérience utilisateur** de l'application **Purchase** d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les composants d'interface, patterns de navigation, formulaires, tableaux, rapports et mécanismes d'interaction pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/purchase/views`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (List, Kanban, Form, Calendar, Graph, Pivot, Activity)
- Composants d'interface spécialisés
- Patterns de navigation
- Formulaires et validations
- Tableaux et listes
- Design responsive et accessibilité

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Logique métier (document dédié)

---

## 1. Vues Principales

### 1.1 Vue Liste (List View) — `purchase.order`

**Fichier :** `purchase_views.xml` — `purchase_order_tree`, `purchase_order_kpis_tree`, `purchase_order_view_tree`

**Caractéristiques :**
- Vue principale pour RFQ et commandes d'achat
- Colonnes configurables (optional="show"/"hide")
- Multi-édition supportée (`multi_edit="1"`)
- Sample data activé
- Dashboard KPIs intégré (`js_class="purchase_dashboard_list"`)

**Colonnes principales :**
- `priority` : Priorité (widget `priority`, optional)
- `partner_ref` : Référence fournisseur (optional)
- `name` : Référence commande (readonly, decoration-info si draft/sent, decoration-bf)
- `date_order` : Date commande (readonly si cancel/purchase, optional, widget `remaining_days` si draft/sent)
- `date_approve` : Date approbation (optional, invisible si quotation_only)
- `partner_id` : Fournisseur (readonly)
- `company_id` : Entreprise (optional, groups multi-company)
- `user_id` : Acheteur (widget `many2one_avatar_user`, optional)
- `activity_ids` : Activités (widget `list_activity`, optional)
- `origin` : Source (optional)
- `amount_untaxed` : Montant HT (sum="Total Untaxed amount", widget `monetary`, optional)
- `amount_total` : Montant TTC (sum="Total amount", widget `monetary`, optional, decoration-bf si purchase)
- `currency_id` : Devise (column_invisible, readonly si cancel/purchase)
- `state` : État (widget `badge`, optional, decoration-success si purchase, decoration-warning si to approve, decoration-info si draft/sent)
- `date_planned` : Date prévue (optional, invisible si quotation_only)
- `invoice_status` : Statut facturation (widget `badge`, optional, decoration-success si invoiced, decoration-info si to invoice)

**Décorations visuelles :**
- `decoration-info` : État draft/sent (`state in ('draft','sent')`)
- `decoration-muted` : État cancel (`state == 'cancel'`)
- `decoration-bf` : Montant total si purchase (`state == 'purchase'`)

**Filtres principaux :**
- My Purchases / My Orders : Commandes assignées à moi
- Starred : Priorité urgente
- New : RFQ en draft
- Sent : RFQ envoyées
- Purchase Orders : Commandes confirmées
- Late : RFQ en retard
- Not Acknowledged : Commandes non reconnues
- Late Receipts : Réceptions en retard
- Waiting Bills : En attente facturation
- Bills Received : Facturées
- Order Date : Par date commande

**Groupements :**
- Par Vendor (`partner_id`)
- Par Buyer (`user_id`)
- Par Order Date (`date_order`)

**Actions groupées (header) :**
- "Create Bills" : Génération factures multiples
- "Cancel" : Annulation multiple (avec confirmation)

### 1.2 Vue Kanban — `purchase.order`

**Fichier :** `purchase_views.xml` — `view_purchase_order_kanban`

**Caractéristiques :**
- Vue mobile-friendly (`class="o_kanban_mobile"`)
- Classe JS spécialisée (`js_class="purchase_dashboard_kanban"`)
- Progressbar par activité (`activity_state`)
- Quick create désactivé (`quick_create="false"`)
- Sample data activé

**Structure de la carte :**
- En-tête : Priorité + Fournisseur (fw-bolder fs-5) + Montant total aligné droite (fw-bolder)
- Footer : Nom, Date commande, Activités + État (badge)

**Widgets spécialisés :**
- `kanban_activity` : Activités planifiées
- `label_selection` : État avec classes CSS (draft=default, cancel=default, approved=warning)
- `priority` : Widget priorité (étoiles)
- `monetary` : Montants avec devise

**Variantes :**
- **Sans Dashboard** (`purchase_order_view_kanban_without_dashboard`) : Sans classe JS dashboard

### 1.3 Vue Formulaire (Form View) — `purchase.order`

**Fichier :** `purchase_views.xml` — `purchase_order_form`

**Caractéristiques :**
- Classe CSS spécialisée (`class="o_purchase_order"`)
- Header avec boutons d'action contextuels
- Alertes et avertissements dynamiques
- Badge "Locked" si commande verrouillée
- Formulaire structuré en groupes et onglets

**Header — Boutons d'action :**
- **Send RFQ** (`action_rfq_send`) : Envoyer RFQ (hotkey `g`, visible si draft)
- **Confirm Order** (`button_confirm`) : Confirmer commande (hotkey `q`, visible si sent/draft)
- **Approve Order** (`button_approve`) : Approuver commande (hotkey `z`, visible si to approve, groups manager)
- **Send PO** (`action_rfq_send`) : Envoyer commande (hotkey `g`, visible si purchase)
- **Acknowledge** (`action_acknowledge`) : Reconnaître (visible si purchase et pas acknowledged)
- **Set to Draft** (`button_draft`) : Remettre en brouillon (hotkey `o`, visible si cancel)
- **Print** (`print_quotation` / `action_report_purchase_order`) : Imprimer (hotkey `k`, visible selon état)
- **Cancel** (`button_cancel`) : Annuler (hotkey `x`, invisible si locked)
- **Lock** (`button_lock`) : Verrouiller (hotkey `l`, visible si purchase et lock_confirmed_po=lock)
- **Unlock** (`button_unlock`) : Déverrouiller (hotkey `l`, visible si locked, groups manager)

**Statusbar :**
- État : Draft → Sent → Purchase
- Readonly

**Alertes dynamiques :**
- `purchase_warning_text` : Avertissements partenaire/produits (alert-warning)
- Doublons (`duplicated_order_ids`) : Warning si commandes dupliquées détectées

**Widget spécialisé :**
- `purchase_file_uploader` : Upload fichiers factures (visible si purchase)

**Structure du formulaire :**

1. **Button Box (statistiques) :**
   - **Bill Matching** (`action_bill_matching`) : Matching factures (visible si purchase et factures disponibles)
   - **Vendor Bills** (`action_view_invoice`) : Nombre factures (visible si factures existent)
   - **Price Comparison** (`action_purchase_comparison`) : Comparaison prix (visible si `show_comparison`)

2. **Badge Locked :**
   - Affiché si `locked=True` (badge rounded-pill text-bg-secondary)

3. **En-tête :**
   - Label : "Request for Quotation" (si draft/sent) ou "Purchase Order" (sinon)
   - Titre : Priorité + Nom (référence)

4. **Groupe gauche :**
   - Fournisseur (`partner_id` avec widget `res_partner_many2one`, mode supplier)
   - Référence fournisseur (`partner_ref`)
   - Devise (`currency_id`, groups multi-currency)

5. **Groupe droit :**
   - Date commande (`date_order`, invisible si purchase)
   - Date approbation (`date_approve`, visible si purchase)
   - Date prévue (`date_planned`)
   - Rappel réception (`receipt_reminder_email`, `reminder_date_before_receipt`, groups send_reminder)
   - Widget `toaster_button` pour prévisualisation rappel

6. **Onglet Products :**
   - Widget spécialisé (`product_label_section_and_note_field_o2m`)
   - Mode list/kanban
   - Contrôles : Add a product, Add a section, Add a note, Catalog
   - Colonnes : Sequence, Product, Description, Date Planned, Analytic, Quantity, UOM, Received, Billed, Price, Discount, Taxes, Amount
   - Support sections et notes
   - Footer : Conditions générales (`note`) + Totaux taxes (`tax_totals` widget `account-tax-totals-field`)

7. **Onglet Other Information :**
   - Acheteur (`user_id` widget `many2one_avatar_user`)
   - Entreprise (`company_id`, groups multi-company)
   - Source (`origin`)
   - Statut facturation (`invoice_status`)
   - Conditions de paiement (`payment_term_id`)
   - Position fiscale (`fiscal_position_id`)

**Widgets spécialisés :**
- `res_partner_many2one` : Sélection fournisseur avec recherche avancée (mode supplier)
- `product_label_section_and_note_field_o2m` : Gestion lignes commande avec sections/notes
- `product_label_section_and_note_field` : Widget produit avec sections/notes
- `many2many_tax_tags` : Tags taxes avec couleurs
- `analytic_distribution` : Distribution analytique
- `many2one_uom` : Sélection UoM avec conversion
- `account-tax-totals-field` : Totaux taxes détaillés
- `toaster_button` : Bouton toast pour prévisualisation
- `purchase_file_uploader` : Upload fichiers factures

### 1.4 Vue Calendar — `purchase.order`

**Fichier :** `purchase_views.xml` — `purchase_order_calendar`

**Caractéristiques :**
- Vue calendrier pour commandes
- Date de début : `date_calendar_start` (date_approve si purchase, sinon date_order)
- Couleur : `partner_id`
- Masquage heure : `hide_time="true"`
- Limite événements : `event_limit="5"`
- Création désactivée : `create="false"`

**Champs affichés :**
- `partner_ref` : Référence fournisseur
- `amount_total` : Montant total (widget `monetary`)
- `partner_id` : Fournisseur (filtre)

### 1.5 Vue Graph — `purchase.order`

**Fichier :** `purchase_views.xml` — `purchase_order_graph`

**Caractéristiques :**
- Graphique par fournisseur
- Mesure : `amount_total`
- Sample data activé

**Structure :**
- X-axis : `partner_id`
- Y-axis : `amount_total` (type="measure")

### 1.6 Vue Pivot — `purchase.order`

**Fichier :** `purchase_views.xml` — `purchase_order_pivot`

**Caractéristiques :**
- Analyse par fournisseur
- Mesure : `amount_total`
- Display quantity activé (`display_quantity="1"`)
- Sample data activé

**Structure :**
- Rows : `partner_id`
- Measure : `amount_total`

### 1.7 Vue Activity — `purchase.order`

**Fichier :** `purchase_views.xml` — `purchase_order_view_activity`

**Caractéristiques :**
- Vue activité pour commandes
- Affichage : Nom, Montant total, Fournisseur, État

**Structure :**
- Box activité avec nom (bold), montant total, fournisseur (muted), badge état

### 1.8 Vue Historique — `purchase.order.line`

**Fichier :** `purchase_views.xml` — `purchase_history_tree`, `purchase_history_pivot`, `purchase_history_graph`

**Caractéristiques :**
- Historique des achats par produit
- Vue liste avec décoration si commande active
- Action ouverture commande (`action_open_order`)
- Pivot et Graph pour analyses

**Colonnes :**
- `order_id` : Commande (widget `many2one`)
- `date_approve` : Date approbation
- `partner_id` : Fournisseur
- `product_uom_qty` : Quantité
- `price_unit_product_uom` : Prix unitaire (widget `monetary`)
- `price_subtotal` : Total HT (widget `monetary`)

---

## 2. Composants d'Interface Spécialisés

### 2.1 Widget `purchase_dashboard_kanban`

**Usage :** Vue Kanban avec dashboard KPIs

**Caractéristiques :**
- Dashboard intégré avec métriques
- KPIs : Draft, Sent, Late, Not Acknowledged, Late Receipts
- Filtres par priorité

### 2.2 Widget `purchase_dashboard_list`

**Usage :** Vue Liste avec dashboard KPIs

**Caractéristiques :**
- Tableau KPIs intégré
- Métriques globales et personnelles
- Filtres rapides

### 2.3 Widget `purchase_file_uploader`

**Usage :** Upload fichiers factures depuis commande

**Caractéristiques :**
- Upload fichiers factures
- Génération factures depuis fichiers
- Visible uniquement si `state` = `purchase`

### 2.4 Widget `toaster_button`

**Usage :** Bouton toast pour prévisualisation email rappel

**Caractéristiques :**
- Envoi email prévisualisation à utilisateur courant
- Toast message de confirmation
- Visible si `receipt_reminder_email` activé

### 2.5 Widget `product_label_section_and_note_field_o2m`

**Usage :** Gestion des lignes de commande avec sections et notes

**Caractéristiques :**
- Support sections (`line_section`)
- Support sous-sections (`line_subsection`)
- Support notes (`line_note`)
- Catalogue intégré (`action_add_from_catalog`)
- Options : `subsections: True`

---

## 3. Patterns de Navigation

### 3.1 Navigation Principale

**Menu Purchase :**
- Orders → Requests for Quotation, Purchase Orders
- Products → Products, Product Variants
- Vendors → Fournisseurs
- Configuration → Pricelists, Products, Attributes, Units & Packagings

### 3.2 Navigation Contextuelle

**Depuis une commande :**
- Lien vers Fournisseur
- Lien vers Acheteur
- Lien vers Factures (`action_view_invoice`)
- Lien vers Bill Matching (`action_bill_matching`)
- Lien vers Price Comparison (`action_purchase_comparison`)
- Lien vers Historique produit (`action_purchase_history`)

**Depuis une ligne :**
- Lien vers Commande (`action_open_order`)
- Lien vers Produit
- Lien vers Factures (`invoice_lines`)

### 3.3 Actions Rapides

**Hotkeys :**
- `g` : Send RFQ / Send PO
- `q` : Confirm Order
- `z` : Approve Order
- `o` : Set to Draft
- `k` : Print
- `x` : Cancel
- `l` : Lock / Unlock

---

## 4. Formulaires et Validations

### 4.1 Validation de Formulaire

**Validations automatiques :**
- Produit obligatoire si pas section/note
- Date prévue obligatoire si pas section/note
- UoM obligatoire si pas section/note
- Prix unitaire obligatoire
- Quantité obligatoire
- Fournisseur obligatoire
- Entreprise obligatoire

**Messages d'erreur :**
- Alertes HTML dynamiques (`purchase_warning_text`)
- Messages inline sous les champs
- Popups pour erreurs bloquantes

### 4.2 Champs Conditionnels

**Visibilité conditionnelle :**
- `invisible="state != 'draft'"` : Champs selon état
- `invisible="state != 'purchase'"` : Champs uniquement si purchase
- `invisible="not receipt_reminder_email"` : Rappel si activé
- `invisible="groups='...'"` : Champs selon groupes utilisateurs

**Readonly conditionnel :**
- `readonly="state in ['cancel', 'purchase']"` : Non modifiable après confirmation
- `readonly="locked"` : Non modifiable si verrouillé
- `readonly="qty_invoiced != 0"` : Prix non modifiable si facturé
- `readonly="invoice_status == 'invoiced' or locked"` : Conditions paiement non modifiables si facturé/verrouillé

### 4.3 Auto-complétion

**Champs avec auto-complétion :**
- `partner_id` : Recherche fournisseur avec contexte (mode supplier)
- `product_id` : Recherche produit avec domaine purchase_ok
- `order_line` : Catalogue produits avec contexte fournisseur

---

## 5. Tableaux et Listes

### 5.1 Colonnes Configurables

**Système optional :**
- `optional="show"` : Visible par défaut, peut être masqué
- `optional="hide"` : Masqué par défaut, peut être affiché

**Colonnes principales :**
- Priority, Partner Ref, Name, Date Order, Date Approve, Partner, Company, User, Activities, Origin, Amount Untaxed, Amount Total, Currency, State, Date Planned, Invoice Status

### 5.2 Multi-édition

**Support multi-édition :**
- `multi_edit="1"` activé sur liste
- Sélection multiple avec cases à cocher
- Édition groupée des champs compatibles
- Actions groupées (Cancel, Create Bills)

### 5.3 Groupement

**Groupements disponibles :**
- Par Vendor (`partner_id`)
- Par Buyer (`user_id`)
- Par Order Date (`date_order`)

### 5.4 Filtres Avancés

**Filtres prédéfinis :**
- My Purchases / My Orders
- Starred (priorité urgente)
- New / Sent / Purchase Orders
- Late / Not Acknowledged / Late Receipts
- Waiting Bills / Bills Received
- Order Date (avec périodes)

**Filtres personnalisés :**
- Recherche texte (nom, référence fournisseur, fournisseur)
- Filtres de domaine complexes
- Filtres de date avec périodes prédéfinies

---

## 6. Design Responsive et Accessibilité

### 6.1 Responsive Design

**Vue mobile :**
- Kanban mobile-friendly (`o_kanban_mobile`)
- Formulaires adaptatifs
- Navigation simplifiée
- Touch-friendly buttons

**Breakpoints :**
- Desktop : Vue complète
- Tablet : Colonnes adaptatives
- Mobile : Vue Kanban prioritaire

### 6.2 Accessibilité

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

## 7. Recommandations pour Miyukini

### 7.1 Composants à Implémenter

**Vues principales :**
1. **Liste commandes** : Vue principale avec colonnes configurables et KPIs
2. **Formulaire commande** : Formulaire structuré avec onglets
3. **Kanban mobile** : Vue mobile-friendly avec dashboard
4. **Calendar** : Calendrier par dates prévues
5. **Pivot** : Analyse par fournisseur
6. **Graph** : Visualisations par fournisseur
7. **Activity** : Vue activité
8. **Historique** : Historique achats par produit

**Widgets spécialisés :**
1. **Purchase Dashboard Widget** : KPIs intégrés
2. **File Uploader Widget** : Upload fichiers factures
3. **Product Catalog Widget** : Catalogue produits achats
4. **Bill Matching Widget** : Matching factures
5. **Price Comparison Widget** : Comparaison prix
6. **Reminder Widget** : Rappels réception

### 7.2 Patterns à Adopter

**Navigation :**
- Menu hiérarchique clair (RFQ, Purchase Orders, Products, Vendors)
- Breadcrumbs contextuels
- Actions rapides accessibles
- Hotkeys pour actions fréquentes

**Formulaires :**
- Validation en temps réel
- Messages d'erreur clairs
- Auto-complétion intelligente
- Champs conditionnels selon état

**Tableaux :**
- Colonnes configurables
- Multi-édition
- Groupements flexibles
- Filtres avancés
- Dashboard KPIs intégré

**Responsive :**
- Design mobile-first
- Vue Kanban pour mobile
- Navigation adaptative
- Touch-friendly

### 7.3 Améliorations Possibles

**UX :**
- Assistant guidé création RFQ
- Comparaison prix automatique
- Matching factures intelligent
- Workflow guidé approbation

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
