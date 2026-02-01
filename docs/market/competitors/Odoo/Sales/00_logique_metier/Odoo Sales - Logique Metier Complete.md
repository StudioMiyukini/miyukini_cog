# Odoo Sales — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Sales** d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/sale`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (SaleOrder, SaleOrderLine)
- Règles métier et contraintes
- Workflows et transitions d'état (Draft → Sent → Sale → Cancel)
- Calculs de prix (pricelist, taxes, remises)
- Gestion des quantités (commandées, livrées, facturées)
- Génération de factures depuis les commandes
- Gestion des paiements et signatures
- Intégration avec CRM, Accounting, Inventory

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `sale.order` (Devis / Commande de Vente)

**Rôle :** Modèle central de l'application Sales. Représente soit un **Devis** (quotation) soit une **Commande de Vente** (sales order).

**États (state) :**
- `draft` : Brouillon (Devis)
- `sent` : Devis envoyé
- `sale` : Commande confirmée
- `cancel` : Annulé

**Champs clés :**

#### Identification
- `name` : Référence de la commande (obligatoire, indexé trigram, séquence auto)
- `state` : État (draft, sent, sale, cancel, indexé, tracking=3)
- `locked` : Boolean (commande verrouillée, ne peut pas être modifiée)

#### Client et adresses
- `partner_id` : Many2one vers `res.partner` (client, obligatoire, indexé, tracking=1)
- `partner_invoice_id` : Many2one vers `res.partner` (adresse de facturation, obligatoire)
- `partner_shipping_id` : Many2one vers `res.partner` (adresse de livraison, obligatoire)
- `client_order_ref` : Char (référence client)

#### Équipe et commercial
- `team_id` : Many2one vers `crm.team` (équipe commerciale, indexé, tracking)
- `user_id` : Many2one vers `res.users` (commercial, indexé, tracking=2)

#### Dates
- `create_date` : Datetime (date de création, indexé)
- `date_order` : Datetime (date de commande, obligatoire, date de confirmation si confirmé)
- `commitment_date` : Datetime (date de livraison promise)
- `validity_date` : Date (date d'expiration du devis, calculé)
- `expected_date` : Datetime (date de livraison attendue, calculé)

#### Prix et devises
- `pricelist_id` : Many2one vers `product.pricelist` (liste de prix, tracking=1)
- `currency_id` : Many2one vers `res.currency` (devise, calculé depuis pricelist)
- `currency_rate` : Float (taux de change, calculé)
- `amount_untaxed` : Monetary (montant HT, calculé, tracking=5)
- `amount_tax` : Monetary (montant taxes, calculé)
- `amount_total` : Monetary (montant TTC, calculé, tracking=4)
- `amount_undiscounted` : Float (montant avant remise, calculé)

#### Lignes de commande
- `order_line` : One2many vers `sale.order.line` (lignes de commande)

#### Facturation
- `invoice_ids` : Many2many vers `account.move` (factures générées, calculé)
- `invoice_count` : Integer (nombre de factures, calculé)
- `invoice_status` : Selection (statut facturation : upselling, invoiced, to invoice, no, calculé)
- `amount_invoiced` : Monetary (montant déjà facturé, calculé)
- `amount_to_invoice` : Monetary (montant à facturer, calculé)
- `journal_id` : Many2one vers `account.journal` (journal de facturation)

#### Paiement et signature
- `require_signature` : Boolean (signature en ligne requise, calculé)
- `require_payment` : Boolean (paiement en ligne requis, calculé)
- `prepayment_percent` : Float (pourcentage d'acompte requis, calculé)
- `signature` : Image (signature client)
- `signed_by` : Char (signé par)
- `signed_on` : Datetime (date de signature)
- `transaction_ids` : Many2many vers `payment.transaction` (transactions de paiement)
- `amount_paid` : Float (montant payé via transactions, calculé)

#### Conditions de paiement et fiscalité
- `payment_term_id` : Many2one vers `account.payment.term` (conditions de paiement)
- `preferred_payment_method_line_id` : Many2one vers `account.payment.method.line` (méthode de paiement préférée)
- `fiscal_position_id` : Many2one vers `account.fiscal.position` (position fiscale)

#### Métadonnées
- `note` : Html (conditions générales, calculé)
- `origin` : Char (document source)
- `reference` : Char (référence de paiement)
- `tag_ids` : Many2many vers `crm.tag` (tags)

#### UTMs (tracking marketing)
- `campaign_id`, `medium_id`, `source_id` : Many2one vers `utm.*` (tracking marketing)

**Règles métier :**

1. **Contrainte de date de confirmation :**
   - Si `state = 'sale'`, alors `date_order` est obligatoire

2. **Contrainte de pricelist :**
   - Ne peut pas changer de pricelist si `state = 'sale'`

3. **Contrainte de verrouillage :**
   - Si `locked = True`, certaines modifications sont interdites

4. **Calcul des montants :**
   - `amount_untaxed` = somme des `price_subtotal` des lignes
   - `amount_tax` = somme des `price_tax` des lignes
   - `amount_total` = `amount_untaxed` + `amount_tax`
   - Calcul via moteur de taxes avec prise en compte des remises globales et acomptes

5. **Statut de facturation :**
   - `invoiced` : Toutes les lignes sont facturées
   - `to invoice` : Des lignes sont à facturer
   - `upselling` : Toutes les lignes sont facturées mais livrées en plus (opportunité upselling)
   - `no` : Rien à facturer (pas confirmé ou pas de lignes facturables)

### 1.2 Modèle `sale.order.line` (Ligne de Commande)

**Rôle :** Représente une ligne de commande (produit, quantité, prix, taxes).

**Champs clés :**

#### Référence commande
- `order_id` : Many2one vers `sale.order` (commande parente, obligatoire, indexé)
- `sequence` : Integer (ordre d'affichage, défaut 10)

#### Produit
- `product_id` : Many2one vers `product.product` (produit, indexé, ondelete='restrict')
- `product_template_id` : Many2one vers `product.template` (template produit, calculé)
- `name` : Text (description, calculé, obligatoire)
- `product_uom_qty` : Float (quantité, obligatoire, défaut 1.0)
- `product_uom_id` : Many2one vers `uom.uom` (unité de mesure, calculé)

#### Prix et remises
- `price_unit` : Float (prix unitaire, calculé, obligatoire)
- `technical_price_unit` : Float (prix unitaire technique, cache)
- `discount` : Float (remise %, calculé)
- `pricelist_item_id` : Many2one vers `product.pricelist.item` (règle de pricelist utilisée, calculé)

#### Montants
- `price_subtotal` : Monetary (sous-total HT, calculé)
- `price_tax` : Float (total taxes, calculé)
- `price_total` : Monetary (total TTC, calculé)
- `price_reduce_taxexcl` : Monetary (prix réduit HT, calculé)
- `price_reduce_taxinc` : Monetary (prix réduit TTC, calculé)

#### Taxes
- `tax_ids` : Many2many vers `account.tax` (taxes, calculé)

#### Quantités
- `qty_delivered` : Float (quantité livrée, calculé, défaut 0.0)
- `qty_delivered_method` : Selection (méthode de calcul livraison : manual, analytic)
- `qty_invoiced` : Float (quantité facturée, calculé)
- `qty_to_invoice` : Float (quantité à facturer, calculé)

#### Statut facturation
- `invoice_status` : Selection (statut : upselling, invoiced, to invoice, no, calculé)
- `invoice_lines` : Many2many vers `account.move.line` (lignes de facture liées)

#### Montants facturés
- `untaxed_amount_invoiced` : Monetary (montant HT facturé, calculé)
- `amount_invoiced` : Monetary (montant TTC facturé, calculé)
- `untaxed_amount_to_invoice` : Monetary (montant HT à facturer, calculé)
- `amount_to_invoice` : Monetary (montant TTC à facturer, calculé)

#### Délai et livraison
- `customer_lead` : Float (délai de livraison en jours, calculé, obligatoire)

#### Types de ligne
- `display_type` : Selection (type : line_section, line_subsection, line_note, ou False pour produit)
- `is_downpayment` : Boolean (acompte)
- `is_expense` : Boolean (ligne provenant d'une note de frais)

#### Produits configurables
- `product_custom_attribute_value_ids` : One2many vers `product.attribute.custom.value` (valeurs custom)
- `product_no_variant_attribute_value_ids` : Many2many vers `product.template.attribute.value` (attributs no_variant)
- `is_configurable_product` : Boolean (produit configurable, calculé)

#### Produits combo
- `combo_item_id` : Many2one vers `product.combo.item` (item combo)
- `linked_line_id` : Many2one vers `sale.order.line` (ligne liée pour combo/option)
- `linked_line_ids` : One2many vers `sale.order.line` (lignes liées)

#### Sections
- `parent_id` : Many2one vers `sale.order.line` (section parente, calculé)
- `collapse_prices` : Boolean (masquer prix dans rapports)
- `collapse_composition` : Boolean (masquer composition dans rapports)

**Règles métier :**

1. **Contrainte de champs obligatoires :**
   - Si `display_type` est NULL et `is_downpayment` est False, alors `product_id` et `product_uom_id` sont obligatoires

2. **Contrainte de champs null pour sections :**
   - Si `display_type` est défini, alors `product_id`, `price_unit`, `product_uom_qty`, `product_uom_id` doivent être NULL

3. **Calcul du prix unitaire :**
   - Basé sur la pricelist (`pricelist_id`)
   - Prend en compte les quantités (`product_uom_qty`)
   - Prend en compte les attributs produits (no_variant, custom)
   - Prend en compte la position fiscale pour les taxes incluses
   - Ne recalcule pas si `technical_price_unit` != `price_unit` (prix manuel)
   - Ne recalcule pas si `qty_invoiced > 0` (déjà facturé)

4. **Calcul de la remise :**
   - Basé sur la pricelist
   - `discount = (base_price - pricelist_price) / base_price * 100`
   - Ne s'affiche que si positif (surcharge si négatif)

5. **Calcul des montants :**
   - Via moteur de taxes (`account.tax`)
   - `price_subtotal` = montant HT après remise
   - `price_total` = montant TTC après remise
   - `price_tax` = `price_total` - `price_subtotal`

6. **Calcul quantité à facturer :**
   - Si `invoice_policy = 'order'` : `qty_to_invoice = product_uom_qty - qty_invoiced`
   - Si `invoice_policy = 'delivery'` : `qty_to_invoice = qty_delivered - qty_invoiced`

7. **Statut de facturation :**
   - `invoiced` : `qty_invoiced >= product_uom_qty`
   - `to invoice` : `qty_to_invoice > 0`
   - `upselling` : `qty_delivered > product_uom_qty` et `invoice_policy = 'order'`
   - `no` : Sinon

---

## 2. Workflows et Transitions d'État

### 2.1 Cycle de Vie Devis → Commande

**Workflow principal :**

```
[Draft] → [Sent] → [Sale] → [Facturation] → [Livraison]
         ↓
      [Cancel]
```

**Étapes détaillées :**

1. **Création Devis (Draft) :**
   - `state = 'draft'`
   - `name` généré automatiquement via séquence
   - `date_order` = maintenant
   - `validity_date` calculé selon `company_id.quotation_validity_days`
   - `pricelist_id` calculé depuis `partner_id.property_product_pricelist`
   - `fiscal_position_id` calculé depuis `partner_shipping_id`
   - `payment_term_id` calculé depuis `partner_id.property_payment_term_id`

2. **Envoi Devis (Sent) :**
   - Méthode : `action_quotation_send()`
   - Actions :
     - Génération email avec template
     - `state` → `sent` (si marqué comme envoyé)
     - Création token portal si nécessaire

3. **Confirmation Commande (Sale) :**
   - Méthode : `action_confirm()`
   - Actions :
     - Validation des lignes (produits requis)
     - `state` → `sale'`
     - `date_order` → maintenant (date de confirmation)
     - Verrouillage si `group_auto_done_setting` activé
     - Génération documents liés (picking, invoice si auto)
     - Envoi email de confirmation si `send_email=True`

4. **Annulation :**
   - Méthode : `action_cancel()`
   - Actions :
     - Annulation des factures brouillons liées
     - `state` → `cancel'`

### 2.2 Génération de Factures

**Méthode :** `_create_invoices(grouped=False, final=False)`

**Processus :**

1. **Préparation des factures :**
   - Pour chaque commande : `_prepare_invoice()`
   - Récupération des lignes facturables : `_get_invoiceable_lines(final)`
   - Préparation des lignes de facture : `_prepare_invoice_line()`

2. **Groupement (si `grouped=False`) :**
   - Groupement par clés : `company_id`, `partner_id`, `partner_shipping_id`, `currency_id`, `fiscal_position_id`
   - Fusion des lignes dans une facture groupée

3. **Création des factures :**
   - Création via `account.move` (sudo pour permissions)
   - Lien bidirectionnel : `invoice_line_ids.sale_line_ids`

4. **Gestion des remboursements :**
   - Si `final=True` et montant total négatif, conversion en avoir (refund)

**Lignes facturables :**
- Lignes avec `qty_to_invoice > 0` ou `display_type = 'line_note'`
- Exclusion des lignes `is_downpayment` (gérées séparément)
- Prise en compte des sections et sous-sections

**Acomptes :**
- Section dédiée "Down Payments" créée automatiquement
- Lignes d'acompte avec `quantity = -1.0` pour annuler les acomptes précédents

### 2.3 Gestion des Paiements et Signatures

**Signature en ligne :**
- `require_signature` : Calculé depuis `company_id.portal_confirmation_sign`
- `_has_to_be_signed()` : Vérifie si signature requise et non signée
- `signature` : Image de la signature
- `signed_by`, `signed_on` : Métadonnées

**Paiement en ligne :**
- `require_payment` : Calculé depuis `company_id.portal_confirmation_pay`
- `prepayment_percent` : Pourcentage d'acompte requis
- `transaction_ids` : Transactions de paiement liées
- `amount_paid` : Somme des transactions autorisées/terminées
- `_has_to_be_paid()` : Vérifie si paiement requis et non payé
- `_is_confirmation_amount_reached()` : Vérifie si montant d'acompte atteint

**Confirmation automatique :**
- Si signature ET paiement requis : confirmation automatique après signature + paiement
- Si paiement seul : confirmation après paiement de l'acompte

---

## 3. Calculs et Algorithmes

### 3.1 Calcul des Prix (Pricelist)

**Méthode :** `_compute_price_unit()`

**Processus :**

1. **Prix de base :**
   - `_get_display_price()` : Prix depuis pricelist
   - Prend en compte quantité, UOM, date, devise
   - Prend en compte attributs produits (no_variant, custom)

2. **Ajustement taxes incluses :**
   - Si taxes incluses dans prix produit, conversion HT via `_get_tax_included_unit_price_from_price()`

3. **Cache prix manuel :**
   - `technical_price_unit` : Cache du prix calculé
   - Si `technical_price_unit != price_unit` : prix manuel, ne pas recalculer

**Règles de pricelist :**
- Recherche de règle via `pricelist_id._get_product_rule()`
- Prend en compte quantité, UOM, date
- Application remise si règle trouvée

### 3.2 Calcul des Taxes

**Méthode :** Via `account.tax._compute_all()`

**Processus :**

1. **Préparation base line :**
   - `_prepare_base_line_for_taxes_computation()`
   - Prix unitaire, quantité, taxes, devise, partenaire

2. **Application position fiscale :**
   - `fiscal_position_id.map_tax(taxes)` : Mapping des taxes selon position fiscale

3. **Calcul taxes :**
   - `AccountTax._add_tax_details_in_base_line()`
   - `AccountTax._round_base_lines_tax_details()`
   - Résultat : `total_excluded`, `total_included`, `tax_amount`

4. **Remises globales :**
   - Prise en compte des remises globales (early payment discount)
   - Lignes spéciales créées pour le calcul

### 3.3 Calcul des Quantités

**Quantité livrée (`qty_delivered`) :**

- **Méthode manuelle :** Saisie manuelle
- **Méthode analytique :** Somme des `unit_amount` des lignes analytiques (`account.analytic.line`)
- **Méthode stock :** Via `sale_stock` : somme des quantités des picking confirmés

**Quantité facturée (`qty_invoiced`) :**

- Somme des quantités des lignes de facture liées (`invoice_lines`)
- Prise en compte des avoirs (soustraits)
- Conversion UOM si nécessaire

**Quantité à facturer (`qty_to_invoice`) :**

- Si `invoice_policy = 'order'` : `product_uom_qty - qty_invoiced`
- Si `invoice_policy = 'delivery'` : `qty_delivered - qty_invoiced`

### 3.4 Calcul Date de Livraison Attendue

**Méthode :** `_compute_expected_date()`

**Formule :**
```
expected_date = date_order + customer_lead (en jours)
```

**Règles :**
- Prend le minimum des `customer_lead` des lignes
- Uniquement pour produits `type = 'consu'` (consommables)
- Exclut les lignes de type section/note

---

## 4. Règles Métier Spécifiques

### 4.1 Gestion des Pricelists

**Règles :**
- `pricelist_id` calculé depuis `partner_id.property_product_pricelist`
- Ne peut pas changer si `state = 'sale'`
- Changement de pricelist : recalcul des prix des nouvelles lignes uniquement
- Option `show_update_pricelist` : Avertissement si pricelist changé avec lignes existantes

**Recalcul des prix :**
- Méthode : `action_update_prices()`
- Recalcule `price_unit` et `discount` pour toutes les lignes
- Réinitialise `discount` à 0 avant recalcul

### 4.2 Gestion des Positions Fiscales

**Règles :**
- `fiscal_position_id` calculé depuis `partner_shipping_id` et `company_id`
- Changement de position fiscale : recalcul des taxes
- Option `show_update_fpos` : Avertissement si position fiscale changée avec lignes existantes

**Recalcul des taxes :**
- Méthode : `action_update_taxes()`
- Recalcule `tax_ids` pour toutes les lignes

### 4.3 Gestion des Lignes

**Types de lignes :**
- **Section (`line_section`)** : Titre de section, pas de produit
- **Sous-section (`line_subsection`)** : Sous-titre, pas de produit
- **Note (`line_note`)** : Note facturable, pas de produit
- **Produit** : Ligne normale avec produit

**Règles de modification :**
- Lignes confirmées (`state = 'sale'`) : Modification limitée
- Si `qty_invoiced > 0` ou `qty_delivered > 0` : Produit non modifiable
- Si `locked = True` : Champs protégés non modifiables
- Suppression : Interdite si `state = 'sale'` et `invoice_lines` ou `qty_delivered > 0`

**Acomptes :**
- `is_downpayment = True`
- Création via wizard `sale.advance.payment.inv`
- Non copiés lors de duplication
- Gestion séparée dans les factures (section dédiée)

### 4.4 Gestion Multi-Company

**Règles :**
- `company_id` obligatoire
- Produits doivent appartenir à la même entreprise ou être accessibles
- Contrainte : `_check_order_line_company_id()`

### 4.5 Gestion des Doublons

**Détection :**
- Basée sur `client_order_ref` et `origin`
- Méthode : `_fetch_duplicate_orders()`
- Affichage dans `duplicated_order_ids`

---

## 5. Intégrations avec Autres Modules

### 5.1 CRM

**Intégration :**
- `team_id` : Équipe commerciale depuis CRM
- `opportunity_id` : Lien vers opportunity (champ sur `sale.order`)
- Conversion Opportunity → Quotation

### 5.2 Accounting

**Intégration :**
- Génération factures : `_create_invoices()`
- Lien bidirectionnel : `invoice_ids` ↔ `sale_line_ids`
- Comptabilisation automatique selon configuration

### 5.3 Inventory (sale_stock)

**Intégration :**
- Génération picking (bon de livraison) lors de confirmation
- `qty_delivered` calculé depuis les picking
- `expected_date` calculé depuis les délais produits

### 5.4 Payment

**Intégration :**
- `transaction_ids` : Transactions de paiement
- Paiement en ligne pour confirmation
- Capture/void des transactions

### 5.5 Portal

**Intégration :**
- Accès client au devis/commande
- Signature en ligne
- Paiement en ligne
- Consultation historique

---

## 6. Considérations pour Miyukini COG

### 6.1 Architecture Opérateurs

**Opérateurs proposés :**
1. **SalesOrderOperator** : Gestion des devis/commandes
2. **SalesOrderLineOperator** : Gestion des lignes de commande
3. **SalesPricelistOperator** : Gestion des listes de prix
4. **SalesInvoiceOperator** : Génération de factures depuis commandes
5. **SalesPaymentOperator** : Gestion des paiements et signatures
6. **SalesUI** : Interface utilisateur Sales

### 6.2 Gouvernance COG

**StrongFather (Décisions) :**
- Autorisation de confirmation de commande
- Autorisation de modification de commande confirmée
- Autorisation de génération de facture
- Validation des remises importantes

**KindMother (Persistance) :**
- Toutes les écritures via `WriteIntent`
- Devis, Commandes, Lignes

**Master Butler (Capacités) :**
- Déclaration des capacités Sales
- Permissions d'accès aux devis/commandes
- Permissions de confirmation

**WorrySentinel (Sécurité) :**
- Niveau de sécurité : 2-3 (données commerciales sensibles)
- Validation des accès cross-équipe
- Audit des confirmations et facturations

**Ever Buddy (Cycle de vie) :**
- Gestion des transitions Draft → Sent → Sale → Cancel
- Gestion des versions de pricelist
- Migration des données lors de changements de structure

### 6.3 Intégrations Miyukini

**MiyuInvoice :**
- Utilisation pour génération factures depuis commandes
- Lien bidirectionnel commande ↔ facture

**MiyuContacts :**
- Utilisation pour `partner_id` (clients)
- Synchronisation adresses facturation/livraison

**MiyuStore :**
- Utilisation pour produits et catalogues
- Intégration avec pricelist

**MiyuBooking :**
- Lien avec rendez-vous commerciaux

**MiyuClock :**
- Gestion des dates (`date_order`, `commitment_date`, `expected_date`)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
