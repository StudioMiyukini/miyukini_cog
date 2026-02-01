# Odoo Sales — Analyse UI/UX Complète

## Contexte

Ce document analyse en profondeur l'**interface utilisateur et l'expérience utilisateur** de l'application Sales d'Odoo, extraite du code source GitHub (vues XML, templates, composants).

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/sale/views`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (Liste, Formulaire, Kanban, Calendrier, Graphique, Pivot)
- Composants d'interface (formulaires, champs, widgets)
- Patterns de navigation
- Responsive design et mobile
- Accessibilité
- Recommandations pour Miyukini

---

## 1. Vues Principales

### 1.1 Vue Liste (Tree/List)

**Fichier :** `sale_order_views.xml` → `sale_order_tree`

**Caractéristiques :**
- Colonnes principales : `name` (référence), `date_order`, `partner_id`, `amount_total`, `state`, `invoice_status`
- Colonnes optionnelles : `user_id`, `team_id`, `commitment_date`, `expected_date`, `tag_ids`
- Décoration : `decoration-muted` pour état `cancel`
- Bouton header : "Create Invoices" (action groupée)
- Tri et filtres intégrés

**Widgets utilisés :**
- `monetary` pour montants
- `badge` pour états
- `many2one_avatar_user` pour commercial
- `list_activity` pour activités

**Recommandations pour Miyukini :**
- Vue liste claire avec colonnes essentielles
- Filtres rapides par état, commercial, client
- Actions groupées (création factures multiples)

### 1.2 Vue Kanban

**Fichier :** `sale_order_views.xml` → `view_sale_order_kanban`

**Caractéristiques :**
- Cards avec informations essentielles : `partner_id`, `amount_total`, `name`, `date_order`
- Progressbar pour activités (`activity_state`)
- Badge d'état avec couleurs (draft=info, sale=success, cancel=default)
- Widget `kanban_activity` pour activités
- Classe mobile : `o_kanban_mobile`

**Structure card :**
```xml
<div class="d-flex align-items-baseline mb-2">
    <field name="partner_id" class="fw-bolder fs-5 me-2"/>
    <field name="amount_total" widget="monetary" class="fw-bolder ms-auto"/>
</div>
<footer>
    <div>name, date_order, activities</div>
    <field name="state" widget="label_selection"/>
</footer>
```

**Recommandations pour Miyukini :**
- Cards visuelles avec informations clés
- Drag & drop pour changement d'état (si workflow)
- Responsive mobile optimisé

### 1.3 Vue Formulaire

**Fichier :** `sale_order_views.xml` → `view_order_form`

**Structure :**

#### Header
- Statusbar avec états : `draft`, `sent`, `sale`
- Boutons contextuels selon état :
  - Draft : "Send", "Send PRO-FORMA", "Confirm", "Print", "Preview", "Cancel"
  - Sent : "Confirm", "Send", "Cancel"
  - Sale : "Create Invoice", "Lock", "Unlock", "Cancel"
- Hotkeys : `g` (Send), `q` (Confirm/Create Invoice), `x` (Cancel), `i` (Print)

#### Alertes
- `sale_warning_text` : Alertes partenaire/produits
- `partner_credit_warning` : Avertissement crédit client
- Produits archivés : Warning si produits archivés dans devis
- Doublons : Warning si commandes dupliquées détectées

#### Sheet (Contenu principal)

**Section Header :**
- Titre : `name` (référence commande)
- Badge "Locked" si verrouillé
- Stat button : Nombre de factures (`invoice_count`)

**Groupe Partner Details :**
- `partner_id` : Client (widget `res_partner_many2one`)
- `partner_invoice_id` : Adresse facturation (si `group_delivery_invoice_address`)
- `partner_shipping_id` : Adresse livraison (si `group_delivery_invoice_address`)

**Groupe Order Details :**
- `validity_date` : Date validité (si draft/sent)
- `date_order` : Date commande/devis (selon état)
- `pricelist_id` : Liste de prix (avec bouton "Update Prices")
- `payment_term_id` : Conditions de paiement

**Notebook Pages :**

1. **Order Lines** :
   - Field `order_line` avec widget `sol_o2m` (Sale Order Line One2Many)
   - Mode : `list,kanban`
   - Options : `hide_composition`, `hide_prices`, `subsections`
   - Contrôles : "Add a product", "Add a section", "Add a note", "Catalog"
   - Bouton "Discount" (si `group_discount_per_so_line`)
   - Section totaux avec `tax_totals` widget
   - Field `note` : Conditions générales

2. **Other Info** :
   - **Sales** : `user_id`, `team_id`, `company_id`, `require_signature`, `require_payment`, `prepayment_percent`, `reference`, `client_order_ref`, `tag_ids`
   - **Invoicing** : `fiscal_position_id` (avec bouton "Update Taxes"), `preferred_payment_method_line_id`, `journal_id`, `invoice_status`
   - **Shipping** : `commitment_date`, `expected_date`
   - **Tracking** : `origin`, `campaign_id`, `medium_id`, `source_id`

3. **Customer Signature** (si requis) :
   - `signed_by`, `signed_on`, `signature` (widget `image`)

**Chatter :**
- Section chatter pour messages et activités

**Recommandations pour Miyukini :**
- Formulaire structuré avec sections claires
- Hotkeys pour actions fréquentes
- Alertes contextuelles visibles
- Widgets spécialisés pour lignes de commande

### 1.4 Vue Calendrier

**Fichier :** `sale_order_views.xml` → `view_sale_order_calendar`

**Caractéristiques :**
- Mode : `month`
- Date start : `activity_date_deadline`
- Couleur : `state`
- Event limit : 5
- Champs affichés : `partner_id` (avatar), `amount_total`, `payment_term_id`
- Icônes : `fa-clock-o` pour activités, `fa-users` pour partenaire

**Recommandations pour Miyukini :**
- Vue calendrier pour dates de livraison
- Filtres par état, commercial, équipe

### 1.5 Vue Graphique

**Fichier :** `sale_order_views.xml` → `view_sale_order_graph`

**Caractéristiques :**
- Mesure : `amount_total`
- Dimension : `partner_id`
- Sample : 1 (échantillonnage)

**Recommandations pour Miyukini :**
- Graphiques de performance (CA par client, commercial, période)
- Export données

### 1.6 Vue Pivot

**Fichier :** `sale_order_views.xml` → `view_sale_order_pivot`

**Caractéristiques :**
- Row : `date_order`
- Measure : `amount_total`
- Sample : 1

**Recommandations pour Miyukini :**
- Tableaux croisés dynamiques pour analyses
- Dimensions multiples (date, client, commercial, produit)

---

## 2. Composants d'Interface

### 2.1 Widgets Spécialisés

#### `sol_o2m` (Sale Order Line One2Many)
- Widget spécialisé pour lignes de commande
- Support sections, sous-sections, notes
- Mode list et kanban
- Options de masquage (composition, prix)

#### `sol_product_many2one`
- Widget de sélection produit spécialisé pour lignes
- Support variantes, attributs
- Avertissements produits archivés

#### `sol_text`
- Widget texte pour description ligne
- Multiligne

#### `many2one_barcode`
- Sélection produit via code-barres
- Sur champ `product_id`

#### `many2one_uom`
- Sélection unité de mesure
- Intégré avec produits

#### `many2many_tax_tags`
- Sélection taxes avec tags visuels
- Options : `no_create`

#### `account-tax-totals-field`
- Affichage totaux taxes
- Format structuré

**Recommandations pour Miyukini :**
- Widgets spécialisés pour lignes de commande
- Support code-barres pour sélection produits
- Affichage taxes clair et structuré

### 2.2 Champs et Validations

**Champs avec décorations :**
- `state` : Badge avec couleurs selon état
- `invoice_status` : Badge avec couleurs (invoiced=success, to invoice=info, upselling=warning)
- `amount_total` : Gras si `invoice_status == 'to invoice'`

**Champs conditionnels :**
- `date_order` : Label change selon état ("Quotation Date" vs "Order Date")
- `qty_delivered`, `qty_invoiced` : Visibles uniquement si `state == 'sale'`
- `validity_date` : Invisible si `state == 'sale'`

**Readonly selon état :**
- Champs verrouillés si `state == 'cancel'` ou `locked`
- `product_id` non modifiable si `qty_invoiced > 0` ou `qty_delivered > 0`

**Recommandations pour Miyukini :**
- Champs readonly intelligents selon état
- Décorations visuelles pour statuts
- Validations contextuelles

---

## 3. Patterns de Navigation

### 3.1 Menus

**Fichier :** `sale_menus.xml`

**Menus principaux :**
- **Sales** (menu racine)
  - **Quotations** : Liste devis
  - **Sales Orders** : Liste commandes
  - **Orders to Invoice** : Commandes à facturer
  - **Orders to Upsell** : Opportunités upselling
  - **Configuration** : Pricelists, équipes, etc.

**Recommandations pour Miyukini :**
- Menu clair avec séparation devis/commandes
- Accès rapide aux actions fréquentes (à facturer, upselling)

### 3.2 Actions et Filtres

**Filtres par défaut :**
- Quotations : `search_default_my_quotation` (mes devis)
- Sales Orders : `search_default_sales` (commandes confirmées)

**Filtres disponibles :**
- Par commercial (`user_id`)
- Par client (`partner_id`)
- Par équipe (`team_id`)
- Par produit (`order_line.product_id`)
- Par état (`state`)
- Par statut facturation (`invoice_status`)
- Par date (`date_order`, `create_date`)

**Groupements :**
- Par commercial
- Par client
- Par date commande
- Par méthode de paiement

**Recommandations pour Miyukini :**
- Filtres rapides par défaut (mes devis, mes commandes)
- Recherche multi-critères
- Groupements utiles pour analyses

---

## 4. Responsive Design et Mobile

### 4.1 Classes Mobile

**Kanban :**
- Classe `o_kanban_mobile` pour optimisation mobile
- Cards adaptatives

**Liste :**
- Colonnes optionnelles masquables
- Responsive avec `optional="show/hide"`

**Formulaire :**
- Layout adaptatif avec groupes
- Sections empilables sur mobile

**Recommandations pour Miyukini :**
- Design mobile-first
- Cards adaptatives
- Navigation tactile optimisée

---

## 5. Accessibilité

### 5.1 Hotkeys

- `g` : Envoyer devis
- `q` : Confirmer / Créer facture
- `x` : Annuler
- `i` : Imprimer
- `shift+g` : Capture transaction
- `shift+v` : Void transaction

### 5.2 Labels et Aides

- Labels clairs pour tous les champs
- Placeholders informatifs ("Type to find a customer...")
- Help text sur boutons ("Recompute all prices...")
- Confirmations pour actions destructives

**Recommandations pour Miyukini :**
- Hotkeys pour actions fréquentes
- Labels et aides contextuelles
- Confirmations pour actions critiques

---

## 6. Recommandations pour Miyukini

### 6.1 Interface Utilisateur

**Actions :**
- Formulaire structuré avec sections claires
- Widgets spécialisés pour lignes de commande
- Alertes contextuelles visibles
- Hotkeys pour productivité

### 6.2 Expérience Utilisateur

**Actions :**
- Workflow guidé (devis → commande → facture)
- Pré-remplissage intelligent depuis CRM
- Templates de devis réutilisables
- Actions groupées (création factures multiples)

### 6.3 Mobile

**Actions :**
- Design mobile-first
- Cards adaptatives
- Navigation tactile optimisée

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
