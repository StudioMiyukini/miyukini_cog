# Odoo Purchase — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Purchase** (Achats) d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/purchase`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (PurchaseOrder, PurchaseOrderLine)
- Règles métier et contraintes
- Workflows et transitions d'état (Draft → Sent → To Approve → Purchase → Cancel)
- Calculs de prix (fournisseur, taxes, remises)
- Gestion des quantités (commandées, reçues, facturées)
- Génération de factures fournisseur depuis les commandes
- Gestion des approbations et validations
- Intégration avec Accounting, Inventory, Product

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `purchase.order` (Commande d'Achat / RFQ)

**Rôle :** Modèle central de l'application Purchase. Représente soit une **Demande de Devis** (RFQ - Request for Quotation) soit une **Commande d'Achat** (Purchase Order).

**États (state) :**
- `draft` : Brouillon (RFQ)
- `sent` : RFQ envoyée au fournisseur
- `to approve` : En attente d'approbation
- `purchase` : Commande d'achat confirmée
- `cancel` : Annulée

**Champs clés :**

#### Identification
- `name` : Référence de la commande (obligatoire, indexé trigram, séquence auto, défaut 'New')
- `state` : État (draft, sent, to approve, purchase, cancel, indexé, tracking)
- `priority` : Priorité (0=Normal, 1=Urgent, indexé, défaut '0')
- `locked` : Boolean (commande verrouillée, ne peut pas être modifiée)
- `lock_confirmed_po` : Selection (règle de verrouillage depuis company, related)

#### Fournisseur et adresses
- `partner_id` : Many2one vers `res.partner` (fournisseur, obligatoire, indexé, tracking=1, check_company)
- `partner_ref` : Char (référence fournisseur, copié depuis devis fournisseur)
- `dest_address_id` : Many2one vers `res.partner` (adresse de dropship, check_company)

#### Dates
- `date_order` : Datetime (date de commande/devis, obligatoire, indexé, défaut maintenant)
- `date_approve` : Datetime (date de confirmation, readonly, indexé)
- `date_planned` : Datetime (date d'arrivée prévue, indexé, calculé depuis lignes)
- `date_calendar_start` : Datetime (date de début calendrier, calculé, stocké)

#### Acheteur et entreprise
- `user_id` : Many2one vers `res.users` (acheteur, indexé, tracking=2, défaut utilisateur courant, check_company)
- `company_id` : Many2one vers `res.company` (entreprise, obligatoire, indexé, défaut entreprise courante)
- `company_currency_id` : Many2one vers `res.currency` (devise entreprise, related)

#### Devise et montants
- `currency_id` : Many2one vers `res.currency` (devise, obligatoire, calculé depuis partner, stocké)
- `currency_rate` : Float (taux de change, calculé, stocké, digits=0)
- `amount_untaxed` : Monetary (montant HT, calculé, stocké, tracking=5)
- `amount_tax` : Monetary (montant taxes, calculé, stocké)
- `amount_total` : Monetary (montant TTC, calculé, stocké, tracking=4)
- `amount_total_cc` : Monetary (montant total en devise entreprise, calculé, stocké)
- `tax_totals` : Binary (détails taxes, calculé, exportable=False)

#### Lignes de commande
- `order_line` : One2many vers `purchase.order.line` (lignes de commande, copié)

#### Facturation
- `invoice_ids` : Many2many vers `account.move` (factures générées, calculé, stocké)
- `invoice_count` : Integer (nombre de factures, calculé, stocké, défaut 0)
- `invoice_status` : Selection (statut facturation : no, to invoice, invoiced, calculé, stocké, readonly, défaut 'no')
- `partner_bill_count` : Integer (nombre factures partenaire, related)

#### Conditions de paiement et fiscalité
- `payment_term_id` : Many2one vers `account.payment.term` (conditions de paiement)
- `fiscal_position_id` : Many2one vers `account.fiscal.position` (position fiscale)
- `tax_country_id` : Many2one vers `res.country` (pays fiscal, calculé, compute_sudo)
- `tax_calculation_rounding_method` : Selection (méthode arrondi taxes, related, readonly)
- `incoterm_id` : Many2one vers `account.incoterms` (incoterms)

#### Autres champs
- `origin` : Char (source, copié, aide="Référence du document qui a généré cette commande")
- `acknowledged` : Boolean (reconnu par fournisseur, copié, tracking)
- `note` : Html (conditions générales)
- `product_id` : Many2one vers `product.product` (produit, related depuis order_line)
- `duplicated_order_ids` : Many2many vers `purchase.order` (commandes dupliquées, calculé)
- `receipt_reminder_email` : Boolean (email rappel réception, calculé, stocké, readonly=False)
- `reminder_date_before_receipt` : Integer (jours avant réception, calculé, stocké, readonly=False)
- `is_late` : Boolean (en retard, store=False, search='_search_is_late')
- `show_comparison` : Boolean (afficher comparaison prix, calculé)
- `purchase_warning_text` : Text (avertissement achat, calculé)

**Règles métier :**
- Le `name` est généré automatiquement via séquence `purchase.order` si 'New'
- Un `purchase.order` ne peut être supprimé que s'il est en état `cancel` (`_unlink_if_cancelled`)
- Les produits dans `order_line` doivent appartenir à la même entreprise (`_check_order_line_company_id`)
- La devise est calculée depuis `partner_id.property_purchase_currency_id` ou devise entreprise
- Les montants sont recalculés automatiquement lors de modification des lignes (`_amount_all`)
- Le statut facturation est calculé selon quantités commandées vs facturées (`_get_invoiced`)

**Workflow d'états :**
```
draft → sent → to approve → purchase
  ↓       ↓         ↓
cancel  cancel    cancel
```

**Actions principales :**
1. **`action_rfq_send()`** : Envoie le RFQ au fournisseur (email)
   - Met à jour `state` = `sent` si `draft`
   - Ouvre composeur email avec template

2. **`button_confirm()`** : Confirme le RFQ
   - Vérifie erreurs (`_confirmation_error_message`)
   - Valide distribution analytique (`order_line._validate_analytic_distribution`)
   - Ajoute fournisseur aux produits (`_add_supplier_to_product`)
   - Si approbation autorisée : `button_approve()`
   - Sinon : `state` = `to approve`

3. **`button_approve()`** : Approuve la commande
   - Filtre commandes autorisées (`_approval_allowed`)
   - Met à jour `state` = `purchase`, `date_approve` = maintenant
   - Verrouille si `lock_confirmed_po` = `lock`

4. **`button_cancel()`** : Annule la commande
   - Vérifie que commande n'est pas verrouillée (`locked`)
   - Vérifie qu'aucune facture n'est en état non draft/cancel
   - Met à jour `state` = `cancel`

5. **`action_create_invoice()`** : Crée facture fournisseur
   - Prépare facture (`_prepare_invoice()`)
   - Prépare lignes facture (`line._prepare_account_move_line()`)
   - Groupe par (company_id, partner_id, currency_id)
   - Crée factures (`account.move`)
   - Convertit en avoir si montant négatif (`action_switch_move_type()`)
   - Retourne action vue factures

6. **`action_merge()`** : Fusionne plusieurs RFQ
   - Groupe par (partner_id, currency_id, dest_address_id)
   - Fusionne dans le RFQ le plus ancien
   - Fusionne lignes similaires (produit, UOM, analytique, remise, date)
   - Annule les autres RFQ
   - Retourne action vue RFQ fusionné

---

### 1.2 Modèle `purchase.order.line` (Ligne de Commande d'Achat)

**Rôle :** Représente une ligne de commande d'achat — produit, quantité, prix, taxes, dates.

**Champs clés :**

#### Identification
- `sequence` : Integer (ordre d'affichage, défaut 10)
- `name` : Text (description, obligatoire, calculé depuis produit, stocké, readonly=False)
- `display_type` : Selection (type affichage : line_section, line_subsection, line_note, défaut False)

#### Produit et quantités
- `product_id` : Many2one vers `product.product` (produit, domaine purchase_ok=True, change_default, indexé btree_not_null, ondelete restrict)
- `product_type` : Selection (type produit, related, readonly)
- `product_qty` : Float (quantité, digits Product Unit, obligatoire)
- `product_uom_qty` : Float (quantité totale en UoM produit, calculé, stocké)
- `product_uom_id` : Many2one vers `uom.uom` (unité de mesure, domaine depuis allowed_uom_ids, ondelete restrict)
- `allowed_uom_ids` : Many2many vers `uom.uom` (unités autorisées, calculé)

#### Prix et taxes
- `price_unit` : Float (prix unitaire, obligatoire, digits Product Price, aggregator avg, calculé, stocké, readonly=False)
- `price_unit_product_uom` : Float (prix unitaire en UoM produit, calculé)
- `price_unit_discounted` : Float (prix unitaire remisé, calculé)
- `technical_price_unit` : Float (prix technique pour calcul, aide)
- `discount` : Float (remise %, digits Discount, calculé, stocké, readonly=False)
- `tax_ids` : Many2many vers `account.tax` (taxes, contexte active_test=False)
- `tax_calculation_rounding_method` : Selection (méthode arrondi, related, readonly)

#### Montants
- `price_subtotal` : Monetary (sous-total HT, calculé, stocké)
- `price_total` : Monetary (total TTC, calculé, stocké)
- `price_tax` : Float (montant taxes, calculé, stocké)

#### Dates
- `date_planned` : Datetime (date d'arrivée prévue, indexé, calculé, stocké, readonly=False, aide="Date de livraison attendue du fournisseur")

#### Quantités reçues et facturées
- `qty_received_method` : Selection (méthode quantité reçue : manual, calculé, stocké)
- `qty_received` : Float (quantité reçue, calculé, inverse='_inverse_qty_received', compute_sudo, stocké, digits Product Unit)
- `qty_received_manual` : Float (quantité reçue manuelle, digits Product Unit, copié)
- `qty_invoiced` : Float (quantité facturée, calculé, stocké, digits Product Unit)
- `qty_to_invoice` : Float (quantité à facturer, calculé, stocké, readonly, digits Product Unit)
- `qty_received_at_date` : Float (quantité reçue à date, calculé, digits Product Unit)
- `qty_invoiced_at_date` : Float (quantité facturée à date, calculé, digits Product Unit)
- `amount_to_invoice_at_date` : Float (montant à facturer à date, calculé)

#### Relations
- `order_id` : Many2one vers `purchase.order` (commande, indexé, obligatoire, ondelete cascade)
- `company_id` : Many2one vers `res.company` (entreprise, related, stocké, readonly)
- `state` : Selection (état, related depuis order_id)
- `partner_id` : Many2one vers `res.partner` (fournisseur, related, stocké, readonly, indexé btree_not_null)
- `currency_id` : Many2one vers `res.currency` (devise, related)
- `date_order` : Datetime (date commande, related, readonly)
- `date_approve` : Datetime (date confirmation, related, readonly)
- `invoice_lines` : One2many vers `account.move.line` (lignes facture, readonly, copié=False)

#### Fournisseur sélectionné
- `selected_seller_id` : Many2one vers `product.supplierinfo` (fournisseur sélectionné, calculé, aide)

#### Autres
- `is_downpayment` : Boolean (acompte)
- `analytic_distribution` : Json (distribution analytique, hérité analytic.mixin)
- `parent_id` : Many2one vers `purchase.order.line` (ligne section parente, calculé)
- `purchase_line_warn_msg` : Text (message avertissement ligne, calculé)
- `product_template_attribute_value_ids` : Many2many (valeurs attributs template, related, readonly)
- `product_no_variant_attribute_value_ids` : Many2many (valeurs attributs non variantes)

**Règles métier :**
- Contrainte `_accountable_required_fields` : Si `display_type` NULL et pas `is_downpayment`, alors `product_id`, `product_uom_id`, `date_planned` obligatoires
- Contrainte `_non_accountable_null_fields` : Si `display_type` non NULL, alors `product_id`, `price_unit`, `product_uom_qty`, `product_uom_id`, `date_planned` doivent être NULL
- Un `purchase.order.line` ne peut être supprimé si `order_id.state` = `purchase` et pas section/note (`_unlink_except_purchase`)
- Le `price_unit` est calculé depuis `selected_seller_id` ou `product_id.standard_price` (`_compute_price_unit_and_date_planned_and_name`)
- La `date_planned` est calculée depuis `selected_seller_id.delay` ou date commande (`_get_date_planned`)
- Les quantités reçues sont calculées depuis `qty_received_manual` si méthode = 'manual', sinon depuis stock moves (si Inventory installé)
- Les quantités facturées sont calculées depuis `invoice_lines` (`_compute_qty_invoiced`)

**Calculs :**

1. **Prix unitaire** (`_compute_price_unit_and_date_planned_and_name`) :
   - Si `selected_seller_id` existe : prix depuis seller, converti devise, converti UoM
   - Sinon : `product_id.standard_price`, converti devise, converti UoM
   - Remise depuis `selected_seller_id.discount`

2. **Montants** (`_compute_amount`) :
   - Base ligne : `product_qty * price_unit * (1 - discount / 100)`
   - Taxes appliquées via `account.tax._add_tax_details_in_base_line`
   - `price_subtotal` = total_excluded_currency
   - `price_total` = total_included_currency
   - `price_tax` = `price_total - price_subtotal`

3. **Quantités** :
   - `qty_received` : Si méthode = 'manual' → `qty_received_manual`, sinon depuis stock moves
   - `qty_invoiced` : Somme quantités depuis `invoice_lines` (factures non cancel)
   - `qty_to_invoice` : Si `purchase_method` = 'purchase' → `product_qty - qty_invoiced`, sinon → `qty_received - qty_invoiced`

---

## 2. Workflows et Transitions d'État

### 2.1 Workflow Principal

**États et transitions :**

```
draft (RFQ Brouillon)
  ↓ [action_rfq_send]
sent (RFQ Envoyée)
  ↓ [button_confirm]
to approve (En Attente Approbation) [si double validation]
  ↓ [button_approve]
purchase (Commande Confirmée)
  ↓ [button_cancel]
cancel (Annulée)
```

**Conditions de transition :**

- **draft → sent** :
  - Action : `action_rfq_send()`
  - Condition : `state` = `draft`
  - Effet : Envoie email au fournisseur, met à jour `state` = `sent`

- **sent → to approve** :
  - Action : `button_confirm()`
  - Condition : `state` = `sent` ou `draft`, pas d'erreurs (`_confirmation_error_message`)
  - Effet : Si `_approval_allowed()` = False → `state` = `to approve`

- **sent → purchase** :
  - Action : `button_confirm()` puis `button_approve()`
  - Condition : `state` = `sent` ou `draft`, `_approval_allowed()` = True
  - Effet : `state` = `purchase`, `date_approve` = maintenant

- **to approve → purchase** :
  - Action : `button_approve()`
  - Condition : `state` = `to approve`, `_approval_allowed()` = True
  - Effet : `state` = `purchase`, `date_approve` = maintenant

- **→ cancel** :
  - Action : `button_cancel()`
  - Condition : `state` dans (`draft`, `sent`, `to approve`, `purchase`), pas `locked`, pas de factures non cancel
  - Effet : `state` = `cancel`

### 2.2 Système d'Approbation

**Règles d'approbation (`_approval_allowed`) :**
- Si `company_id.po_double_validation` = `one_step` → Toujours autorisé
- Si `company_id.po_double_validation` = `two_step` :
  - Si `amount_total` < `company_id.po_double_validation_amount` (converti devise) → Autorisé
  - Sinon → Nécessite `group_purchase_manager`
- Si utilisateur a `group_purchase_manager` → Toujours autorisé

**Double validation :**
- `one_step` : Confirmation directe (pas d'approbation)
- `two_step` : Confirmation → `to approve` → Approbation manager → `purchase`

---

## 3. Règles Métier et Contraintes

### 3.1 Contraintes de Données

**Contraintes sur `purchase.order` :**
- `name` : Obligatoire, unique (séquence)
- `partner_id` : Obligatoire, doit appartenir à `company_id` (check_company)
- `company_id` : Obligatoire
- `date_order` : Obligatoire
- Produits dans `order_line` doivent appartenir à `company_id` (`_check_order_line_company_id`)

**Contraintes sur `purchase.order.line` :**
- `product_id` : Obligatoire si pas `display_type` et pas `is_downpayment`
- `product_uom_id` : Obligatoire si pas `display_type` et pas `is_downpayment`
- `date_planned` : Obligatoire si pas `display_type` et pas `is_downpayment`
- `price_unit` : Obligatoire
- `product_qty` : Obligatoire, digits Product Unit
- Si `display_type` non NULL : `product_id`, `price_unit`, `product_uom_qty`, `product_uom_id`, `date_planned` doivent être NULL

### 3.2 Règles de Calcul

**Calcul montants commande (`_amount_all`) :**
- Filtre lignes non `display_type`
- Prépare base lignes pour calcul taxes (`_prepare_base_line_for_taxes_computation`)
- Applique taxes via `account.tax._add_tax_details_in_base_lines`
- Arrondit via `account.tax._round_base_lines_tax_details`
- Calcule totaux via `account.tax._get_tax_totals_summary`
- `amount_untaxed` = `base_amount_currency`
- `amount_tax` = `tax_amount_currency`
- `amount_total` = `total_amount_currency`

**Calcul statut facturation (`_get_invoiced`) :**
- Si `state` != `purchase` → `invoice_status` = `no`
- Si toutes lignes `qty_to_invoice` = 0 et `invoice_ids` existe → `invoice_status` = `invoiced`
- Si au moins une ligne `qty_to_invoice` > 0 → `invoice_status` = `to invoice`
- Sinon → `invoice_status` = `no`

**Calcul date prévue (`_compute_date_planned`) :**
- `date_planned` = minimum `date_planned` de toutes les lignes non `display_type`

**Calcul fournisseur sélectionné (`_compute_selected_seller_id`) :**
- Sélectionne seller depuis `product_id.seller_ids` selon :
  - `partner_id` = seller `partner_id`
  - `product_qty` >= seller `min_qty`
  - `date_order` dans période seller (`date_start`, `date_end`)
  - `product_uom_id` compatible
- Tri par `min_qty` croissant

### 3.3 Gestion des Doublons

**Détection (`_compute_duplicated_order_ids`) :**
- Basée sur `partner_ref` et `origin`
- Méthode : `_fetch_duplicate_orders()`
- Critères : même `company_id`, même `partner_id`, (`origin` = `name` autre) ou (`partner_ref` = `partner_ref` autre)
- Affichage dans `duplicated_order_ids`

---

## 4. Intégrations avec Autres Modules

### 4.1 Accounting

**Intégration :**
- Génération factures fournisseur : `action_create_invoice()`
- Lien bidirectionnel : `invoice_ids` ↔ `purchase_line_ids` sur `account.move.line`
- Synchronisation montants : `qty_invoiced`, `qty_to_invoice`
- Statut facturation : `invoice_status` (no, to invoice, invoiced)

**Champs liés :**
- `purchase_line_id` : Ligne commande liée (sur `account.move.line`)
- `invoice_origin` : Origine facture (référence commande)
- `invoice_payment_term_id` : Conditions de paiement (depuis commande)
- `fiscal_position_id` : Position fiscale (depuis commande)

### 4.2 Product

**Intégration :**
- Sélection produits depuis catalogue (`purchase_ok=True`)
- Calcul prix depuis `product.supplierinfo` (seller)
- Gestion variantes et attributs produits
- Ajout fournisseur aux produits (`_add_supplier_to_product`)

**Champs liés :**
- `product_id` : Produit sélectionné
- `selected_seller_id` : Seller sélectionné depuis `product.supplierinfo`
- `product_uom_id` : Unité de mesure

### 4.3 Inventory (si installé)

**Intégration :**
- Génération réception (picking) lors de confirmation
- `qty_received` calculé depuis les picking
- `date_planned` utilisé pour date de réception attendue

**Champs liés :**
- `qty_received` : Quantité reçue (depuis picking si méthode != 'manual')
- `qty_received_method` : Méthode calcul (manual ou depuis stock moves)

### 4.4 Portal

**Intégration :**
- Accès fournisseur à la commande via portail
- Reconnaissance (acknowledge) depuis portail
- Mise à jour dates prévues depuis portail

**Champs liés :**
- `acknowledged` : Reconnu par fournisseur
- `access_url` : URL portail (`/my/purchase/{id}`)

---

## 5. Considérations pour Miyukini COG

### 5.1 Architecture Opérateurs

**Opérateurs proposés :**
1. **PurchaseOrderOperator** : Gestion des commandes d'achat/RFQ
2. **PurchaseOrderLineOperator** : Gestion des lignes de commande
3. **PurchaseApprovalOperator** : Gestion des approbations
4. **PurchaseInvoiceOperator** : Génération de factures fournisseur depuis commandes
5. **PurchaseReceptionOperator** : Gestion des réceptions (si Inventory)
6. **PurchaseUI** : Interface utilisateur Purchase

### 5.2 Gouvernance COG

**StrongFather (Décisions) :**
- Autorisation de confirmation de commande
- Autorisation d'approbation (si double validation)
- Autorisation de modification de commande confirmée
- Autorisation de génération de facture
- Validation des montants importants

**KindMother (Persistance) :**
- Toutes les écritures via `WriteIntent`
- Commandes, lignes, factures

**Master Butler (Permissions) :**
- Permissions de création/modification/confirmation
- Permissions d'approbation (selon groupe)
- Permissions de génération facture

**WorrySentinel (Sécurité) :**
- Niveau sécurité : 2 (Sensitive) pour commandes
- Niveau sécurité : 3 (Critical) pour factures fournisseur
- Vérification modification commandes confirmées
- Isolation cross-équipe

**Ever Buddy (Cycle de Vie) :**
- Gestion transitions d'état (draft → sent → to approve → purchase → cancel)
- Gestion dépréciation/retrait fonctionnalités

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
