# Odoo Inventory — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Inventory** (Stock) d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Code source GitHub Odoo 19.0

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration
- APIs et hooks utilisés
- Événements partagés
- Extensions de modèles partagés

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances explicites (`__manifest__.py`) :**
- `base` : Fonctionnalités de base (partners, companies, users, currencies)
- `product` : Gestion des produits (product.product, product.template, product.category)
- `barcodes_gs1_nomenclature` : Support codes-barres GS1 (traçabilité)
- `digest` : Rapports digest (résumés périodiques)
- `web` : Framework web

### 1.2 Modules Optionnels

**Dépendances optionnelles :**
- `sale` : Intégration ventes (livraisons depuis commandes)
- `purchase` : Intégration achats (réceptions depuis commandes fournisseur)
- `account` : Intégration comptable (écritures de stock)
- `mrp` : Intégration manufacturing (consommation composants, réception produits finis)
- `stock_account` : Écritures comptables automatiques (si Accounting installé)
- `stock_barcode` : Interface codes-barres pour opérations stock
- `stock_picking_batch` : Traitement par lots de pickings

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Product

**Flux :**
```
Product → Stock Quantities → Stock Moves
```

**Mécanismes :**
- Extension de `product.product` et `product.template` avec champs stock
- Calcul automatique des quantités disponibles
- Gestion de la traçabilité (lots, numéros de série)
- Routes d'approvisionnement par produit

**Champs ajoutés à `product.product` :**
- `stock_quant_ids` : One2many vers `stock.quant` (quantités en stock)
- `stock_move_ids` : One2many vers `stock.move` (mouvements de stock)
- `qty_available` : Float (quantité disponible, calculé, recherchable, inversible)
- `virtual_available` : Float (quantité prévisionnelle, calculé, recherchable)
- `free_qty` : Float (quantité libre = disponible - réservée, calculé, recherchable)
- `incoming_qty` : Float (quantité entrante planifiée, calculé, recherchable)
- `outgoing_qty` : Float (quantité sortante planifiée, calculé, recherchable)
- `orderpoint_ids` : One2many vers `stock.warehouse.orderpoint` (règles de réapprovisionnement)
- `putaway_rule_ids` : One2many vers `stock.putaway.rule` (règles de rangement)
- `lot_properties_definition` : PropertiesDefinition (propriétés personnalisées pour lots)

**Champs ajoutés à `product.template` :**
- `is_storable` : Boolean (suivi de stock activé, calculé depuis `type`, tracking)
- `responsible_id` : Many2one vers `res.users` (responsable logistique)
- `property_stock_production` : Many2one vers `stock.location` (emplacement production, company_dependent)
- `property_stock_inventory` : Many2one vers `stock.location` (emplacement inventaire, company_dependent)
- `sale_delay` : Integer (délai de livraison client, en jours)
- `tracking` : Selection (traçabilité : serial, lot, none, calculé depuis `is_storable`)
- `lot_sequence_id` : Many2one vers `ir.sequence` (séquence pour numéros de lot/série)
- `serial_prefix_format` : Char (format préfixe sérial, calculé depuis `lot_sequence_id.prefix`)
- `next_serial` : Char (prochain numéro de série, calculé)
- `description_picking` : Text (description sur picking, traduisible)
- `description_pickingout` : Text (description sur livraisons, traduisible)
- `description_pickingin` : Text (description sur réceptions, traduisible)
- `route_ids` : Many2many vers `stock.route` (routes d'approvisionnement)
- `qty_available`, `virtual_available`, `incoming_qty`, `outgoing_qty` : Float (quantités, calculées depuis variants)

**Champs ajoutés à `product.category` :**
- `route_ids` : Many2many vers `stock.route` (routes héritables par catégorie)
- `removal_strategy_id` : Many2one vers `product.removal` (stratégie de retrait forcée)
- `parent_route_ids` : Many2many vers `stock.route` (routes parentes, calculé)
- `total_route_ids` : Many2many vers `stock.route` (routes totales = routes + routes parentes, calculé, recherchable)
- `putaway_rule_ids` : One2many vers `stock.putaway.rule` (règles de rangement par catégorie)
- `packaging_reserve_method` : Selection (méthode de réservation d'emballages : full, partial)

**Méthodes clés :**
- `_compute_quantities()` : Calcul des quantités disponibles/prévisionnelles/entrantes/sortantes
- `_compute_quantities_dict()` : Calcul optimisé par dictionnaire
- `_get_domain_locations()` : Parsing contexte pour déterminer emplacements à considérer
- `_search_qty_available()` : Recherche optimisée sur quantités disponibles (bypass moves si pas de dates)
- `_inverse_qty_available()` : Ajustement manuel de quantité (crée `stock.quant` si nécessaire)
- `action_open_quants()` : Action pour ouvrir la vue des quantités
- `action_view_orderpoints()` : Action pour voir les règles de réapprovisionnement
- `action_view_routes()` : Action pour voir les routes d'approvisionnement

**Hooks utilisés :**
- `_onchange_tracking()` : Avertissement si changement traçabilité avec stock existant
- `_onchange_type()` : Avertissement si changement type produit avec mouvements existants
- `write()` : Validation si changement `company_id` avec mouvements/quants dans autre entreprise
- `_filter_to_unlink()` : Empêche suppression si lots liés

**Recommandations pour Miyukini :**
- Intégration native avec MiyuStore (produits)
- Calcul automatique des quantités depuis StockQuants
- Gestion traçabilité intégrée (lots, séries)
- Routes d'approvisionnement configurables par produit/catégorie
- Synchronisation bidirectionnelle quantités ↔ produits

### 2.2 Intégration avec Sales

**Flux :**
```
Sales Order → Stock Picking (outgoing) → Stock Move → Stock Quant
```

**Mécanismes :**
- Génération automatique de `stock.picking` (livraison) depuis `sale.order` confirmé
- Création de `stock.move` depuis `sale.order.line`
- Lien via `origin` = `sale.order.name`
- Synchronisation dates de livraison promise
- Réservation automatique lors de la confirmation

**Champs liés :**
- `stock.picking.origin` : Référence commande (ex: "SO001")
- `stock.picking.partner_id` : Client depuis `sale.order.partner_id`
- `stock.picking.date_deadline` : Date de livraison promise depuis `sale.order.commitment_date`
- `stock.move.sale_line_id` : Many2one vers `sale.order.line` (lien direct)
- `stock.move.origin` : Référence commande (héritée du picking)

**Hooks utilisés :**
- `sale.order._action_confirm()` : Création picking de livraison si `sale_stock` installé
- `sale.order.line._prepare_procurement_values()` : Préparation valeurs pour approvisionnement
- `stock.picking._action_done()` : Mise à jour `qty_delivered` sur `sale.order.line`
- `stock.move._action_done()` : Mise à jour statut livraison sur commande

**Événements :**
- Confirmation commande → Création picking livraison
- Validation picking → Mise à jour `qty_delivered` sur lignes commande
- Annulation picking → Mise à jour statut commande

**Recommandations pour Miyukini :**
- Intégration native avec Miyukini Sales
- Génération automatique de Transferts depuis Commandes confirmées
- Lien bidirectionnel commande ↔ transfert
- Synchronisation automatique des quantités livrées
- Gestion backorders si livraison partielle

### 2.3 Intégration avec Purchase

**Flux :**
```
Purchase Order → Stock Picking (incoming) → Stock Move → Stock Quant
```

**Mécanismes :**
- Génération automatique de `stock.picking` (réception) depuis `purchase.order` confirmé
- Création de `stock.move` depuis `purchase.order.line`
- Lien via `origin` = `purchase.order.name`
- Synchronisation dates de réception planifiée
- Réception automatique lors de la validation picking

**Champs liés :**
- `stock.picking.origin` : Référence commande fournisseur (ex: "PO002")
- `stock.picking.partner_id` : Fournisseur depuis `purchase.order.partner_id`
- `stock.picking.date_deadline` : Date de réception promise depuis `purchase.order.date_planned`
- `stock.move.purchase_line_id` : Many2one vers `purchase.order.line` (lien direct)
- `stock.move.origin` : Référence commande (héritée du picking)

**Hooks utilisés :**
- `purchase.order.button_confirm()` : Création picking de réception si `purchase_stock` installé
- `purchase.order.line._prepare_stock_moves()` : Préparation moves depuis lignes commande
- `stock.picking._action_done()` : Mise à jour `qty_received` sur `purchase.order.line`
- `stock.move._action_done()` : Mise à jour statut réception sur commande

**Événements :**
- Confirmation commande fournisseur → Création picking réception
- Validation picking → Mise à jour `qty_received` sur lignes commande
- Annulation picking → Mise à jour statut commande

**Recommandations pour Miyukini :**
- Intégration native avec module Purchase (si développé)
- Génération automatique de Transferts depuis Commandes fournisseur confirmées
- Lien bidirectionnel commande fournisseur ↔ transfert
- Synchronisation automatique des quantités reçues
- Gestion backorders si réception partielle

### 2.4 Intégration avec Accounting

**Flux :**
```
Stock Move → Account Move (écriture de stock)
```

**Mécanismes :**
- Génération automatique d'écritures comptables (`account.move`) lors de `stock.move._action_done()`
- Écritures créées uniquement si `stock_account` installé
- Lignes d'écriture (`account.move.line`) pour variations de stock
- Comptes comptables par défaut depuis produits/emplacements

**Champs liés :**
- `stock.move.account_move_ids` : Many2many vers `account.move` (écritures générées)
- `stock.move.account_move_line_ids` : Many2many vers `account.move.line` (lignes d'écriture)
- `stock.location.valuation_in_account_id` : Many2one vers `account.account` (compte entrée)
- `stock.location.valuation_out_account_id` : Many2one vers `account.account` (compte sortie)
- `product.product.property_stock_account_input` : Many2one vers `account.account` (compte entrée produit, company_dependent)
- `product.product.property_stock_account_output` : Many2one vers `account.account` (compte sortie produit, company_dependent)

**Hooks utilisés :**
- `stock.move._action_done()` : Création écritures comptables si `stock_account` installé
- `stock.move._create_accounting_entries()` : Création lignes d'écriture
- `stock.quant._create_accounting_entries()` : Création écritures pour ajustements inventaire
- `stock.location._get_valuation_account()` : Récupération compte comptable par défaut

**Événements :**
- Validation move → Création écriture comptable
- Ajustement inventaire → Création écriture d'inventaire
- Annulation move → Contre-passation écriture

**Recommandations pour Miyukini :**
- Intégration avec MiyuInvoice / MiyuAccounting (si développé)
- Génération automatique d'écritures depuis mouvements stock
- Comptes comptables configurables par produit/emplacement
- Traçabilité complète stock ↔ comptabilité

### 2.5 Intégration avec Manufacturing (MRP)

**Flux :**
```
MRP Production → Stock Move (consommation composants)
MRP Production → Stock Picking (réception produits finis)
```

**Mécanismes :**
- Création automatique de `stock.move` pour consommation composants depuis `mrp.production`
- Création automatique de `stock.picking` pour réception produits finis
- Lien via `origin` = `mrp.production.name`
- Gestion des emplacements production

**Champs liés :**
- `stock.picking.origin` : Référence ordre de fabrication (ex: "MO001")
- `stock.move.raw_material_production_id` : Many2one vers `mrp.production` (consommation composants)
- `stock.move.production_id` : Many2one vers `mrp.production` (réception produits finis)
- `stock.location.usage` = `'production'` : Emplacement production

**Hooks utilisés :**
- `mrp.production._action_confirm()` : Création moves consommation composants
- `mrp.production.button_mark_done()` : Création picking réception produits finis
- `stock.move._action_done()` : Mise à jour statut production

**Événements :**
- Confirmation production → Création moves consommation
- Fin production → Création picking réception
- Validation picking → Mise à jour production

**Recommandations pour Miyukini :**
- Intégration avec module Manufacturing (si développé)
- Génération automatique de Transferts depuis Ordres de fabrication
- Gestion consommation composants et réception produits finis
- Lien bidirectionnel production ↔ transferts

### 2.6 Intégration avec Partners (res.partner)

**Flux :**
```
Partner → Default Stock Locations → Stock Picking
```

**Mécanismes :**
- Extension de `res.partner` avec emplacements stock par défaut
- Utilisation pour pickings entrants/sortants automatiques
- Gestion multi-entreprise

**Champs ajoutés à `res.partner` :**
- `property_stock_customer` : Many2one vers `stock.location` (emplacement client par défaut, company_dependent)
- `property_stock_supplier` : Many2one vers `stock.location` (emplacement fournisseur par défaut, company_dependent)
- `picking_warn_msg` : Text (message d'avertissement sur pickings)

**Hooks utilisés :**
- `res.company._create_transit_location()` : Création emplacement transit inter-entrepôts
- `res.company._set_per_company_inter_company_locations()` : Configuration emplacements inter-entreprises

**Recommandations pour Miyukini :**
- Intégration avec MiyuContacts (partenaires)
- Emplacements par défaut configurables par partenaire
- Gestion multi-entreprise si nécessaire

### 2.7 Intégration avec Company (res.company)

**Flux :**
```
Company → Stock Configuration → Default Locations → Stock Operations
```

**Mécanismes :**
- Extension de `res.company` avec configurations stock
- Création automatique d'emplacements par défaut lors de création entreprise
- Gestion emplacements transit, inventaire, production, scrap

**Champs ajoutés à `res.company` :**
- `internal_transit_location_id` : Many2one vers `stock.location` (emplacement transit inter-entrepôts)
- `stock_move_email_validation` : Boolean (validation email picking)
- `stock_mail_confirmation_template_id` : Many2one vers `mail.template` (template email confirmation picking)
- `annual_inventory_month` : Selection (mois inventaire annuel : 1-12)
- `annual_inventory_day` : Integer (jour du mois inventaire annuel)
- `horizon_days` : Float (horizon réapprovisionnement, défaut 365 jours)
- `stock_text_confirmation` : Boolean (confirmation SMS picking)
- `stock_confirmation_type` : Selection (type confirmation : sms)

**Méthodes clés :**
- `_create_transit_location()` : Création emplacement transit inter-entrepôts
- `_create_inventory_loss_location()` : Création emplacement pertes inventaire
- `_create_production_location()` : Création emplacement production
- `_create_scrap_location()` : Création emplacement scrap
- `_create_scrap_sequence()` : Création séquence scrap
- `create_missing_warehouse()` : Création entrepôt manquant pour première entreprise
- `_create_per_company_locations()` : Création emplacements par entreprise
- `_create_per_company_sequences()` : Création séquences par entreprise
- `_create_per_company_picking_types()` : Création types picking par entreprise
- `_create_per_company_rules()` : Création règles par entreprise

**Hooks utilisés :**
- `res.company.create()` : Création automatique emplacements/séquences/picking types/règles lors création entreprise
- `res.company._set_per_company_inter_company_locations()` : Configuration emplacements inter-entreprises

**Recommandations pour Miyukini :**
- Configuration stock par entreprise (si multi-entreprise)
- Création automatique emplacements par défaut
- Gestion emplacements transit/inventaire/production/scrap

### 2.8 Intégration avec Users (res.users)

**Flux :**
```
User → Default Warehouse → Stock Operations
```

**Mécanismes :**
- Extension de `res.users` avec entrepôt par défaut
- Utilisation pour filtrage automatique des opérations

**Champs ajoutés à `res.users` :**
- `default_warehouse_id` : Many2one vers `stock.warehouse` (entrepôt par défaut, calculé)

**Méthodes clés :**
- `_get_default_warehouse_id()` : Récupération entrepôt par défaut (premier entrepôt de l'entreprise)

**Recommandations pour Miyukini :**
- Entrepôt par défaut par utilisateur (si multi-entrepôts)
- Filtrage automatique des opérations par entrepôt utilisateur

---

## 3. Flux de Données Inter-Apps

### 3.1 Flux Principal Ventes

```
Sales Order (sale)
    ↓
Stock Picking (outgoing) [si sale_stock installé]
    ↓
Stock Move (outgoing)
    ↓
Stock Quant (réduction quantité)
    ↓
Account Move (écriture stock) [si stock_account installé]
```

### 3.2 Flux Principal Achats

```
Purchase Order (purchase)
    ↓
Stock Picking (incoming) [si purchase_stock installé]
    ↓
Stock Move (incoming)
    ↓
Stock Quant (augmentation quantité)
    ↓
Account Move (écriture stock) [si stock_account installé]
```

### 3.3 Flux Manufacturing

```
MRP Production (mrp)
    ↓
Stock Move (consommation composants)
    ↓
Stock Quant (réduction composants)
    ↓
Stock Picking (réception produits finis)
    ↓
Stock Move (réception produits finis)
    ↓
Stock Quant (augmentation produits finis)
    ↓
Account Move (écriture stock) [si stock_account installé]
```

### 3.4 Flux Inventaire Physique

```
Physical Inventory (stock)
    ↓
Stock Quant (ajustement quantité)
    ↓
Account Move (écriture inventaire) [si stock_account installé]
```

### 3.5 Flux Données Partagées

**Données partagées :**
- **Product** : Produits partagés entre Sales, Purchase, Stock, Accounting, Manufacturing
- **Partner** : Clients/Fournisseurs partagés entre Sales, Purchase, Stock, Accounting
- **Company** : Entreprises partagées entre toutes les apps
- **User** : Utilisateurs partagés entre toutes les apps
- **Currency** : Devises partagées entre toutes les apps
- **UoM** : Unités de mesure partagées entre Product, Stock, Sales, Purchase
- **Location** : Emplacements utilisés uniquement par Stock (mais référencés par Product)
- **Warehouse** : Entrepôts utilisés uniquement par Stock
- **Stock Route** : Routes d'approvisionnement utilisées uniquement par Stock (mais référencées par Product)

---

## 4. Mécanismes d'Intégration

### 4.1 Hooks et Overrides

**Hooks utilisés :**
- `_action_confirm()` : Confirmation document source → Création picking
- `_action_done()` : Validation picking → Mise à jour document source
- `_prepare_procurement_values()` : Préparation valeurs pour approvisionnement
- `_prepare_stock_moves()` : Préparation moves depuis document source
- `_create_accounting_entries()` : Création écritures comptables depuis moves
- `_get_valuation_account()` : Récupération compte comptable par défaut

**Overrides :**
- `product.product` : Champs quantités, traçabilité, routes
- `product.template` : Champs stock, traçabilité, routes
- `product.category` : Routes, stratégie retrait, règles rangement
- `res.partner` : Emplacements par défaut
- `res.company` : Configurations stock, emplacements par défaut
- `res.users` : Entrepôt par défaut
- `stock.move` : Lien vers `sale.order.line`, `purchase.order.line`, `mrp.production`
- `stock.picking` : Lien via `origin` vers documents sources

### 4.2 Événements et Signaux

**Événements :**
- Confirmation commande → Création picking livraison
- Confirmation commande fournisseur → Création picking réception
- Confirmation production → Création moves consommation
- Validation picking → Mise à jour quantités livrées/reçues sur commandes
- Validation move → Création écriture comptable (si `stock_account`)
- Ajustement inventaire → Création écriture inventaire (si `stock_account`)
- Annulation picking → Mise à jour statut commande
- Annulation move → Contre-passation écriture

**Signaux :**
- `onchange_product_id` : Mise à jour comptes comptables par défaut
- `onchange_location_id` : Mise à jour comptes comptables par défaut
- `onchange_tracking` : Avertissement si changement traçabilité avec stock existant
- `onchange_type` : Avertissement si changement type produit avec mouvements existants

### 4.3 APIs et Méthodes Publiques

**Méthodes principales :**
- `stock.picking._action_confirm()` : Confirmation picking
- `stock.picking._action_assign()` : Attribution picking (réservation)
- `stock.picking._action_done()` : Validation picking
- `stock.move._action_confirm()` : Confirmation move
- `stock.move._action_assign()` : Attribution move (réservation)
- `stock.move._action_done()` : Validation move
- `stock.quant._apply_inventory()` : Application ajustement inventaire
- `product.product._compute_quantities()` : Calcul quantités disponibles
- `product.product._inverse_qty_available()` : Ajustement manuel quantité

**APIs externes :**
- Pas d'API REST publique dans Stock core
- APIs via `stock_barcode` pour interface codes-barres
- APIs via `stock_picking_batch` pour traitement par lots

---

## 5. Intégrations avec Services Externes

### 5.1 Codes-Barres

**Intégration :**
- `stock_barcode` : Module interface codes-barres
- Support codes-barres GS1 (`barcodes_gs1_nomenclature`)
- Scan codes-barres pour opérations stock (réception, picking, inventaire)

**Flux :**
```
Barcode Scan → Product Identification → Stock Operation
```

### 5.2 Messagerie

**Intégration :**
- `mail` : Module messagerie (dépendance implicite)
- Emails de confirmation picking (si `stock_move_email_validation` activé)
- Templates email configurables par entreprise

**Flux :**
```
Stock Picking Done → Email Confirmation → Customer/Supplier
```

### 5.3 Rapports Digest

**Intégration :**
- `digest` : Module rapports digest (dépendance explicite)
- Résumés périodiques des opérations stock

---

## 6. Recommandations pour Miyukini

### 6.1 Intégrations Prioritaires

**Intégrations natives :**
1. **MiyuStore** : Produits et quantités (priorité absolue)
2. **Miyukini Sales** : Génération transferts depuis commandes
3. **MiyuInvoice / MiyuAccounting** : Écritures comptables depuis mouvements (si développé)
4. **MiyuContacts** : Partenaires et emplacements par défaut
5. **Module Purchase** : Génération transferts depuis commandes fournisseur (si développé)
6. **Module Manufacturing** : Consommation composants et réception produits finis (si développé)

### 6.2 Patterns d'Intégration

**Actions :**
- Lien bidirectionnel document source ↔ transfert
- Synchronisation automatique des quantités livrées/reçues
- Génération automatique de transferts depuis documents sources
- Calcul automatique des quantités depuis StockQuants
- Gestion traçabilité intégrée (lots, séries)

**Gouvernance COG :**
- StrongFather : Autorisation création transfert depuis document source
- KindMother : Persistance via WriteIntent (StockPicking, StockMove, StockQuant)
- Master Butler : Permissions opérations stock
- WorrySentinel : Sécurité données stock (niveaux S2-S4 selon criticité)
- Caring Nanny : Observation état stock (dégradé si incohérences)

### 6.3 Architecture d'Intégration

**Opérateurs proposés :**
1. **StockPickingOperator** : Gestion transferts (picking)
2. **StockMoveOperator** : Gestion mouvements individuels
3. **StockQuantOperator** : Gestion quantités en stock
4. **StockLocationOperator** : Gestion emplacements
5. **StockWarehouseOperator** : Gestion entrepôts
6. **StockLotOperator** : Gestion lots et numéros de série
7. **StockPackageOperator** : Gestion colis et emballages
8. **StockRuleOperator** : Gestion règles d'approvisionnement
9. **StockInventoryOperator** : Gestion inventaires physiques
10. **StockUI** : Interface utilisateur

**Intégrations via BondingBrother :**
- Traduction intentions depuis Sales/Purchase/Manufacturing
- Traduction réponses vers sources
- Médiation sans autorité

**Extensions de modèles partagés :**
- `MiyuStore.Product` : Champs quantités, traçabilité, routes
- `MiyuContacts.Partner` : Emplacements par défaut
- `MiyuCompany.Company` : Configurations stock (si multi-entreprise)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
