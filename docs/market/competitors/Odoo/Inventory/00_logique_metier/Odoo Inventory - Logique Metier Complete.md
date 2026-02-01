# Odoo Inventory — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Inventory** (Stock) d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/stock`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (StockPicking, StockMove, StockMoveLine, StockQuant, StockLocation, StockWarehouse)
- Règles métier et contraintes
- Workflows et transitions d'état (Draft → Confirmed → Assigned → Done)
- Gestion des réservations et disponibilités
- Stratégies de retrait (FIFO, LIFO, FEFO, Least Packages)
- Gestion des lots et numéros de série
- Gestion des emplacements et entrepôts
- Système de règles d'approvisionnement (Stock Rules)
- Inventaire physique et ajustements
- Gestion des colis et emballages
- Traçabilité et historique

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `stock.picking` (Transfert / Bon de Livraison)

**Rôle :** Représente un **transfert de stock** (picking) — opération de déplacement de produits d'un emplacement source vers un emplacement destination. Peut être une réception, une livraison, ou un transfert interne.

**États (state) :**
- `draft` : Brouillon
- `waiting` : En attente d'une autre opération
- `confirmed` : Confirmé, en attente de disponibilité
- `assigned` : Prêt (produits réservés)
- `done` : Terminé
- `cancel` : Annulé

**Champs clés :**

#### Identification
- `name` : Référence du transfert (obligatoire, séquence auto, indexé trigram)
- `origin` : Document source (indexé trigram, ex: "SO001", "PO002")
- `state` : État (calculé depuis les moves, indexé, tracking)
- `picking_type_id` : Many2one vers `stock.picking.type` (type d'opération, obligatoire, indexé, tracking)

#### Emplacements
- `location_id` : Many2one vers `stock.location` (emplacement source, obligatoire, check_company)
- `location_dest_id` : Many2one vers `stock.location` (emplacement destination, obligatoire, check_company)
- `picking_type_code` : Selection (code du type : incoming, outgoing, internal, calculé)

#### Mouvements et lignes
- `move_ids` : One2many vers `stock.move` (mouvements de stock)
- `move_line_ids` : One2many vers `stock.move.line` (opérations détaillées)

#### Dates et priorités
- `scheduled_date` : Datetime (date planifiée, indexé, tracking, calculé depuis moves)
- `date_deadline` : Datetime (date limite, calculé)
- `date_done` : Datetime (date de traitement effectif)
- `has_deadline_issue` : Boolean (retard détecté, calculé)
- `priority` : Selection (priorité : 0=Normal, 1=Urgent, hérité du picking_type)

#### Partenaire et responsable
- `partner_id` : Many2one vers `res.partner` (contact, check_company, indexé)
- `user_id` : Many2one vers `res.users` (responsable, tracking)
- `owner_id` : Many2one vers `res.partner` (propriétaire des produits, check_company)

#### Politique de livraison
- `move_type` : Selection (politique : direct=partiel autorisé, one=tout en une fois, hérité du picking_type)

#### Poids et volumes
- `weight_bulk` : Float (poids des produits non emballés, calculé)
- `shipping_weight` : Float (poids total pour expédition, calculé, stocké)
- `shipping_volume` : Float (volume total, calculé)

#### Colis
- `packages_count` : Integer (nombre de colis, calculé)
- `package_history_ids` : Many2many vers `stock.package.history` (historique des colis transférés)

#### Disponibilité
- `products_availability` : Char (statut de disponibilité : Available, Not Available, Exp date, calculé)
- `products_availability_state` : Selection (available, expected, late, calculé, recherchable)

#### Backorders et retours
- `backorder_id` : Many2one vers `stock.picking` (backorder de, indexé, readonly)
- `backorder_ids` : One2many vers `stock.picking` (backorders créés)
- `return_id` : Many2one vers `stock.picking` (retour de, indexé, readonly)
- `return_ids` : One2many vers `stock.picking` (retours créés)
- `return_count` : Integer (nombre de retours, calculé)

#### Verrouillage et signature
- `is_locked` : Boolean (verrouillé, par défaut True, empêche modification des quantités initiales)
- `signature` : Image (signature, copie interdite)
- `is_signed` : Boolean (signé, calculé depuis signature)
- `printed` : Boolean (imprimé)

#### Propriétés et notes
- `picking_properties` : Properties (propriétés dynamiques, définition depuis picking_type)
- `note` : Html (notes)

**Règles métier :**
- Le `state` est calculé depuis les `move_ids` selon la logique suivante :
  - Si tous les moves sont `draft` → `draft`
  - Si tous les moves sont `cancel` → `cancel`
  - Si tous les moves sont `done` ou `cancel` (sans scrap) → `done`
  - Sinon, état le plus avancé parmi les moves (assigned > waiting > confirmed)
- La `scheduled_date` est calculée depuis les moves :
  - `direct` : minimum des dates des moves
  - `one` : maximum des dates des moves
- Un picking ne peut pas être modifié si `state` = `done` ou `cancel` (sauf si `is_locked` = False)
- Les backorders sont créés automatiquement si `create_backorder` = `ask` ou `always` et qu'il reste des quantités non traitées

---

### 1.2 Modèle `stock.move` (Mouvement de Stock)

**Rôle :** Représente un **mouvement de stock** — intention de déplacer une quantité d'un produit d'un emplacement source vers un emplacement destination. Un mouvement peut être réservé (assigned) ou non (confirmed/waiting).

**États (state) :**
- `draft` : Nouveau
- `waiting` : En attente d'un autre mouvement
- `confirmed` : Confirmé, en attente de réservation
- `partially_available` : Partiellement disponible
- `assigned` : Disponible (réservé)
- `done` : Terminé
- `cancel` : Annulé

**Champs clés :**

#### Identification
- `reference` : Char (référence, calculé depuis picking/scrap/inventory)
- `sequence` : Integer (ordre d'affichage, défaut 10)
- `priority` : Selection (priorité : 0=Normal, 1=Urgent, hérité du picking)

#### Produit et quantités
- `product_id` : Many2one vers `product.product` (produit, obligatoire, indexé, check_company)
- `product_uom_qty` : Float (demande initiale, digits Product Unit, obligatoire)
- `product_qty` : Float (quantité réelle en UoM produit, calculé depuis product_uom_qty)
- `product_uom` : Many2one vers `uom.uom` (unité de mesure, obligatoire, calculé depuis product)
- `quantity` : Float (quantité faite, calculé depuis move_line_ids, digits Product Unit)
- `picked` : Boolean (coché, calculé depuis move_line_ids, stocké)

#### Emplacements
- `location_id` : Many2one vers `stock.location` (emplacement source, obligatoire, indexé, check_company)
- `location_dest_id` : Many2one vers `stock.location` (emplacement destination intermédiaire, obligatoire, indexé)
- `location_final_id` : Many2one vers `stock.location` (emplacement destination final, indexé, check_company)
- `location_usage` : Selection (type emplacement source, calculé)
- `location_dest_usage` : Selection (type emplacement destination, calculé)

#### Picking et type
- `picking_id` : Many2one vers `stock.picking` (transfert, indexé, check_company)
- `picking_type_id` : Many2one vers `stock.picking.type` (type d'opération, calculé depuis picking, check_company)
- `picking_code` : Selection (code du picking type, calculé)

#### Chaining (chaînage)
- `move_orig_ids` : Many2many vers `stock.move` (mouvements d'origine, via stock_move_move_rel)
- `move_dest_ids` : Many2many vers `stock.move` (mouvements de destination)

#### Dates
- `date` : Datetime (date planifiée, indexé, obligatoire, défaut maintenant)
- `date_deadline` : Datetime (date limite, readonly)
- `delay_alert_date` : Datetime (alerte retard, calculé)

#### Réservation et disponibilité
- `availability` : Float (quantité disponible pour réservation, calculé)
- `forecast_availability` : Float (disponibilité prévue, calculé, digits Product Unit)
- `forecast_expected_date` : Datetime (date prévue de disponibilité, calculé)
- `reservation_date` : Date (date de réservation, calculé selon reservation_method)

#### Méthode d'approvisionnement
- `procure_method` : Selection (make_to_stock=par défaut, make_to_order=approvisionnement avancé, défaut make_to_stock)
- `rule_id` : Many2one vers `stock.rule` (règle de stock qui a créé ce move)

#### Lots et numéros de série
- `has_tracking` : Selection (produit avec traçabilité : none, lot, serial, calculé)
- `lot_ids` : Many2many vers `stock.lot` (numéros de série/lots, calculé depuis move_line_ids)
- `next_serial` : Char (premier SN/Lot)
- `next_serial_count` : Integer (nombre de SN/Lots à générer)

#### Lignes de mouvement
- `move_line_ids` : One2many vers `stock.move.line` (lignes détaillées)
- `move_lines_count` : Integer (nombre de lignes, calculé)

#### Propriétaire et restrictions
- `restrict_partner_id` : Many2one vers `res.partner` (propriétaire restrictif, check_company, indexé)
- `owner_id` : Many2one vers `res.partner` (propriétaire, calculé depuis picking)

#### Routes et entrepôt
- `route_ids` : Many2many vers `stock.route` (routes préférées)
- `warehouse_id` : Many2one vers `stock.warehouse` (entrepôt, calculé)

#### Prix
- `price_unit` : Float (prix unitaire, copie interdite, utilisé pour valorisation)

#### Scrap et inventaire
- `scrap_id` : Many2one vers `stock.scrap` (opération de casse, readonly, indexé)
- `is_inventory` : Boolean (inventaire)
- `inventory_name` : Char (nom inventaire, readonly)

#### Références
- `reference_ids` : Many2many vers `stock.reference` (références, calculé depuis picking)

#### Emballage
- `packaging_uom_id` : Many2one vers `uom.uom` (unité d'emballage, calculé depuis move_orig/move_dest)
- `packaging_uom_qty` : Float (quantité en unité d'emballage, calculé)

**Règles métier :**
- Le `state` est calculé selon :
  - `draft` si le move n'est pas confirmé
  - `waiting` si `move_orig_ids` existe et n'est pas `done`
  - `confirmed` si confirmé mais pas réservé
  - `partially_available` si partiellement réservé
  - `assigned` si entièrement réservé
  - `done` si traité
  - `cancel` si annulé
- La `quantity` (quantité faite) est la somme des `quantity` des `move_line_ids`
- La réservation se fait via `_action_assign()` qui crée des `stock.move.line` avec réservation de `stock.quant`
- Les moves peuvent être chaînés : `move_orig_ids` → `move` → `move_dest_ids`
- Un move `make_to_order` déclenche un approvisionnement automatique via les règles de stock

---

### 1.3 Modèle `stock.move.line` (Ligne de Mouvement / Opération Détaillée)

**Rôle :** Représente une **opération détaillée** de mouvement — ligne concrète avec quantité, lot, colis, emplacements précis. Une ligne correspond à un prélèvement réel depuis un quant spécifique.

**Champs clés :**

#### Identification
- `picking_id` : Many2one vers `stock.picking` (transfert)
- `move_id` : Many2one vers `stock.move` (mouvement, obligatoire, indexé)

#### Produit et quantités
- `product_id` : Many2one vers `product.product` (produit, obligatoire, check_company)
- `product_uom_id` : Many2one vers `uom.uom` (unité de mesure, obligatoire)
- `quantity` : Float (quantité faite, digits Product Unit)
- `quantity_product_uom` : Float (quantité en UoM produit, calculé)
- `picked` : Boolean (coché, indique si la ligne a été prélevée)

#### Emplacements
- `location_id` : Many2one vers `stock.location` (emplacement source, obligatoire, indexé)
- `location_dest_id` : Many2one vers `stock.location` (emplacement destination, obligatoire, indexé)

#### Lot et numéro de série
- `lot_id` : Many2one vers `stock.lot` (lot/numéro de série, check_company)
- `lot_name` : Char (nom de lot à créer, utilisé si use_create_lots)

#### Colis
- `package_id` : Many2one vers `stock.package` (colis source, check_company)
- `result_package_id` : Many2one vers `stock.package` (colis destination, check_company)

#### Propriétaire
- `owner_id` : Many2one vers `res.partner` (propriétaire, check_company)

#### État
- `state` : Selection (état, calculé depuis move.state)

#### Date
- `date` : Datetime (date, hérité du move.date)

#### Inventaire
- `is_inventory` : Boolean (ligne d'inventaire)

**Règles métier :**
- Une `move_line` représente un prélèvement réel depuis un `stock.quant` spécifique
- Les `move_line_ids` sont créées lors de la réservation (`_action_assign`)
- La `quantity` peut être modifiée manuellement si `picked` = False
- Les `move_line_ids` sont supprimées lors de l'annulation de réservation (`_do_unreserve`)

---

### 1.4 Modèle `stock.quant` (Quantité de Stock)

**Rôle :** Représente une **quantité physique** de stock — stock réel d'un produit dans un emplacement spécifique, avec lot, colis et propriétaire. C'est l'unité atomique de stock.

**Champs clés :**

#### Identification
- `product_id` : Many2one vers `product.product` (produit, obligatoire, indexé, check_company)
- `location_id` : Many2one vers `stock.location` (emplacement, obligatoire, indexé, ondelete restrict)
- `lot_id` : Many2one vers `stock.lot` (lot/numéro de série, indexé, ondelete restrict, check_company)
- `package_id` : Many2one vers `stock.package` (colis, indexé, ondelete restrict, check_company)
- `owner_id` : Many2one vers `res.partner` (propriétaire, indexé)

#### Quantités
- `quantity` : Float (quantité en stock, readonly, digits Product Unit)
- `reserved_quantity` : Float (quantité réservée, readonly, défaut 0.0, digits Product Unit)
- `available_quantity` : Float (quantité disponible = quantity - reserved_quantity, calculé, digits Product Unit)

#### Date
- `in_date` : Datetime (date d'entrée, readonly, obligatoire, défaut maintenant)

#### Inventaire physique
- `inventory_quantity` : Float (quantité comptée, digits Product Unit)
- `inventory_quantity_auto_apply` : Float (quantité inventoriée auto-appliquée, calculé, inverse)
- `inventory_diff_quantity` : Float (écart = inventory_quantity - quantity, calculé, stocké)
- `inventory_date` : Date (date planifiée d'inventaire, calculé, stocké)
- `last_count_date` : Date (dernière date de comptage, calculé)
- `inventory_quantity_set` : Boolean (quantité inventaire définie, calculé, stocké)
- `is_outdated` : Boolean (quantité dépassée depuis dernier comptage, calculé, recherchable)
- `user_id` : Many2one vers `res.users` (utilisateur assigné au comptage)

#### Traçabilité
- `tracking` : Selection (traçabilité produit : none, lot, serial, calculé)
- `sn_duplicated` : Boolean (numéro de série dupliqué, calculé)

#### Propriétés
- `lot_properties` : Properties (propriétés du lot, readonly, définition depuis product)

**Règles métier :**
- Un quant est unique par combinaison (product_id, location_id, lot_id, package_id, owner_id)
- La `quantity` ne peut être modifiée que via `_update_available_quantity()` qui crée des `stock.move`
- La `reserved_quantity` est mise à jour lors de la réservation/annulation de réservation
- En mode inventaire (`inventory_mode`), `inventory_quantity` peut être définie et appliquée pour créer un ajustement
- Les quants avec `quantity` = 0 et `reserved_quantity` = 0 sont automatiquement supprimés (`_unlink_zero_quants`)

---

### 1.5 Modèle `stock.location` (Emplacement)

**Rôle :** Représente un **emplacement de stock** — lieu physique ou virtuel où les produits peuvent être stockés ou transitent.

**Types d'usage (usage) :**
- `supplier` : Fournisseur (virtuel, source pour réceptions)
- `view` : Vue (virtuel, agrégation hiérarchique)
- `internal` : Interne (physique, stock réel)
- `customer` : Client (virtuel, destination pour livraisons)
- `inventory` : Perte d'inventaire (virtuel, contrepartie pour ajustements)
- `production` : Production (virtuel, contrepartie pour production)
- `transit` : Transit (virtuel, inter-entreprises/inter-entrepôts)

**Champs clés :**

#### Identification
- `name` : Char (nom, obligatoire)
- `complete_name` : Char (nom complet avec hiérarchie, calculé récursif, stocké)
- `barcode` : Char (code-barres, unique par company)
- `active` : Boolean (actif, défaut True)

#### Hiérarchie
- `location_id` : Many2one vers `stock.location` (emplacement parent, indexé, check_company)
- `child_ids` : One2many vers `stock.location` (emplacements enfants)
- `parent_path` : Char (chemin hiérarchique, indexé)
- `child_internal_location_ids` : Many2many (emplacements internes descendants, calculé récursif)

#### Configuration
- `usage` : Selection (type d'usage, obligatoire, indexé, défaut internal)
- `company_id` : Many2one vers `res.company` (entreprise, indexé)
- `warehouse_id` : Many2one vers `stock.warehouse` (entrepôt, calculé, stocké)

#### Stratégies
- `removal_strategy_id` : Many2one vers `product.removal` (stratégie de retrait : FIFO, LIFO, FEFO, Closest, Least Packages)
- `putaway_rule_ids` : One2many vers `stock.putaway.rule` (règles de rangement)

#### Inventaire cyclique
- `cyclic_inventory_frequency` : Integer (fréquence inventaire en jours, défaut 0)
- `last_inventory_date` : Date (dernière date d'inventaire, readonly)
- `next_inventory_date` : Date (prochaine date prévue, calculé, stocké)

#### Stockage
- `storage_category_id` : Many2one vers `stock.storage.category` (catégorie de stockage, check_company, indexé)
- `replenish_location` : Boolean (déclenche réapprovisionnement, calculé, stocké)

#### Poids
- `net_weight` : Float (poids net actuel, calculé)
- `forecast_weight` : Float (poids prévu, calculé)
- `is_empty` : Boolean (vide, calculé, recherchable)

#### Relations
- `quant_ids` : One2many vers `stock.quant` (quants dans cet emplacement)
- `outgoing_move_line_ids` : One2many vers `stock.move.line` (lignes sortantes)
- `incoming_move_line_ids` : One2many vers `stock.move.line` (lignes entrantes)

**Règles métier :**
- Les emplacements `view` ne peuvent pas contenir de produits (`quant_ids` vide)
- Un emplacement ne peut pas être supprimé s'il contient des quants avec `quantity` > 0
- La `removal_strategy_id` détermine l'ordre de prélèvement des quants lors de la réservation
- Les `putaway_rule_ids` déterminent où ranger un produit lors de la réception
- Un emplacement `internal` ou `transit` peut avoir un inventaire cyclique configuré

---

### 1.6 Modèle `stock.warehouse` (Entrepôt)

**Rôle :** Représente un **entrepôt** — structure organisationnelle regroupant des emplacements et définissant les routes d'approvisionnement et de livraison.

**Champs clés :**

#### Identification
- `name` : Char (nom, obligatoire, unique par company)
- `code` : Char (code court, obligatoire, taille 5, unique par company)
- `active` : Boolean (actif, défaut True)
- `sequence` : Integer (ordre d'affichage, défaut 10)

#### Entreprise
- `company_id` : Many2one vers `res.company` (entreprise, readonly, obligatoire)
- `partner_id` : Many2one vers `res.partner` (adresse, check_company)

#### Emplacements
- `view_location_id` : Many2one vers `stock.location` (emplacement vue racine, obligatoire, check_company, indexé)
- `lot_stock_id` : Many2one vers `stock.location` (emplacement stock principal, obligatoire, check_company)
- `wh_input_stock_loc_id` : Many2one vers `stock.location` (emplacement entrée, check_company)
- `wh_qc_stock_loc_id` : Many2one vers `stock.location` (emplacement contrôle qualité, check_company)
- `wh_output_stock_loc_id` : Many2one vers `stock.location` (emplacement sortie, check_company)
- `wh_pack_stock_loc_id` : Many2one vers `stock.location` (emplacement emballage, check_company)

#### Routes
- `route_ids` : Many2many vers `stock.route` (routes par défaut, check_company)
- `reception_route_id` : Many2one vers `stock.route` (route de réception, ondelete restrict)
- `delivery_route_id` : Many2one vers `stock.route` (route de livraison, ondelete restrict)
- `resupply_wh_ids` : Many2many vers `stock.warehouse` (entrepôts d'approvisionnement)
- `resupply_route_ids` : One2many vers `stock.route` (routes d'approvisionnement)

#### Étapes de réception
- `reception_steps` : Selection (one_step, two_steps, three_steps, défaut one_step, obligatoire)

#### Étapes de livraison
- `delivery_steps` : Selection (ship_only, pick_ship, pick_pack_ship, défaut ship_only, obligatoire)

#### Types d'opération
- `in_type_id` : Many2one vers `stock.picking.type` (type réception, check_company)
- `out_type_id` : Many2one vers `stock.picking.type` (type livraison, check_company)
- `pick_type_id` : Many2one vers `stock.picking.type` (type prélèvement, check_company)
- `pack_type_id` : Many2one vers `stock.picking.type` (type emballage, check_company)
- `int_type_id` : Many2one vers `stock.picking.type` (type transfert interne, check_company)
- `qc_type_id` : Many2one vers `stock.picking.type` (type contrôle qualité, check_company)
- `store_type_id` : Many2one vers `stock.picking.type` (type stockage, check_company)
- `xdock_type_id` : Many2one vers `stock.picking.type` (type cross-dock, check_company)

#### Règles
- `mto_pull_id` : Many2one vers `stock.rule` (règle MTO, copie interdite)

**Règles métier :**
- Un entrepôt est créé avec une hiérarchie complète d'emplacements selon `reception_steps` et `delivery_steps`
- Les routes sont créées automatiquement selon la configuration
- Les types d'opération sont créés avec des séquences automatiques
- Un entrepôt peut s'approvisionner depuis d'autres entrepôts via `resupply_wh_ids`

---

### 1.7 Modèle `stock.picking.type` (Type d'Opération)

**Rôle :** Définit un **type d'opération** — configuration d'un type de transfert (réception, livraison, transfert interne) avec ses règles et comportements.

**Champs clés :**

#### Identification
- `name` : Char (nom, obligatoire, traduisible)
- `code` : Selection (incoming, outgoing, internal, obligatoire, défaut incoming)
- `sequence_code` : Char (préfixe séquence, obligatoire)
- `sequence_id` : Many2one vers `ir.sequence` (séquence de référence, check_company)
- `barcode` : Char (code-barres)

#### Emplacements par défaut
- `default_location_src_id` : Many2one vers `stock.location` (emplacement source par défaut, obligatoire, check_company)
- `default_location_dest_id` : Many2one vers `stock.location` (emplacement destination par défaut, obligatoire, check_company)

#### Entreposage
- `warehouse_id` : Many2one vers `stock.warehouse` (entrepôt, calculé, stocké, ondelete cascade, check_company)

#### Configuration lots
- `use_create_lots` : Boolean (créer nouveaux lots, calculé depuis code, stocké)
- `use_existing_lots` : Boolean (utiliser lots existants, calculé depuis code, stocké)

#### Réservation
- `reservation_method` : Selection (at_confirm, manual, by_date, obligatoire, défaut at_confirm)
- `reservation_days_before` : Integer (jours avant date planifiée pour réservation)
- `reservation_days_before_priority` : Integer (jours pour produits prioritaires)

#### Politique de livraison
- `move_type` : Selection (direct, one, défaut direct, obligatoire)

#### Retours
- `return_picking_type_id` : Many2one vers `stock.picking.type` (type pour retours, indexé, check_company)

#### Compteurs Kanban
- `count_picking_draft` : Integer (nombre brouillons, calculé)
- `count_picking_ready` : Integer (nombre prêts, calculé)
- `count_picking` : Integer (nombre en cours, calculé)
- `count_picking_waiting` : Integer (nombre en attente, calculé)
- `count_picking_late` : Integer (nombre en retard, calculé)
- `count_picking_backorders` : Integer (nombre backorders, calculé)
- `count_move_ready` : Integer (nombre moves prêts, calculé)

#### Backorders
- `create_backorder` : Selection (ask, always, never, obligatoire, défaut ask)

#### Impression automatique
- `auto_print_delivery_slip` : Boolean (imprimer bon de livraison automatiquement)
- `auto_print_return_slip` : Boolean (imprimer bon de retour automatiquement)
- `auto_print_product_labels` : Boolean (imprimer étiquettes produits automatiquement)
- `auto_print_lot_labels` : Boolean (imprimer étiquettes lots automatiquement)
- `auto_print_reception_report` : Boolean (imprimer rapport réception automatiquement)
- `auto_show_reception_report` : Boolean (afficher rapport réception à validation)

#### Favoris
- `favorite_user_ids` : Many2many vers `res.users` (utilisateurs favoris)
- `is_favorite` : Boolean (favori pour utilisateur courant, calculé, recherchable)

**Règles métier :**
- Le `code` détermine les emplacements par défaut et les comportements
- `use_create_lots` = True pour `incoming`, `use_existing_lots` = True pour `outgoing`
- La `reservation_method` détermine quand la réservation se fait :
  - `at_confirm` : à la confirmation
  - `manual` : manuellement
  - `by_date` : selon `reservation_date` calculé

---

### 1.8 Modèle `stock.lot` (Lot / Numéro de Série)

**Rôle :** Représente un **lot** ou un **numéro de série** — identifiant unique pour un groupe de produits ou un produit individuel avec traçabilité.

**Champs clés :**

#### Identification
- `name` : Char (nom du lot/SN, obligatoire, indexé)
- `product_id` : Many2one vers `product.product` (produit, obligatoire, check_company)
- `company_id` : Many2one vers `res.company` (entreprise, check_company)

#### Dates
- `expiration_date` : Datetime (date d'expiration)
- `use_date` : Datetime (date d'utilisation)
- `removal_date` : Datetime (date de retrait)
- `alert_date` : Datetime (date d'alerte)

#### Propriétés
- `lot_properties` : Properties (propriétés dynamiques, définition depuis product)

**Règles métier :**
- Un lot est unique par `product_id` + `name` + `company_id`
- Les lots sont créés automatiquement lors de la réception si `use_create_lots` = True
- Les numéros de série (`tracking` = `serial`) doivent être uniques et avoir `quantity` = 1 dans les quants

---

### 1.9 Modèle `stock.package` (Colis)

**Rôle :** Représente un **colis** — conteneur physique regroupant des produits pour le transport ou le stockage.

**Champs clés :**

#### Identification
- `name` : Char (nom/référence, obligatoire)
- `package_type_id` : Many2one vers `stock.package.type` (type de colis, check_company)
- `location_id` : Many2one vers `stock.location` (emplacement, indexé)
- `packing_date` : Datetime (date d'emballage)

#### Hiérarchie
- `parent_package_id` : Many2one vers `stock.package` (colis parent)
- `child_ids` : One2many vers `stock.package` (colis enfants)

#### Poids
- `shipping_weight` : Float (poids expédition)

#### Relations
- `quant_ids` : One2many vers `stock.quant` (quants dans ce colis)
- `picking_ids` : Many2many vers `stock.picking` (transferts contenant ce colis)

**Règles métier :**
- Un colis peut contenir d'autres colis (hiérarchie)
- Les colis sont créés lors de l'opération "Put in Pack"
- Le poids d'expédition peut être défini manuellement ou calculé depuis les produits

---

### 1.10 Modèle `stock.rule` (Règle de Stock)

**Rôle :** Définit une **règle d'approvisionnement** — automatise la création de mouvements de stock selon des conditions (route, produit, emplacement).

**Champs clés :**

#### Identification
- `name` : Char (nom, obligatoire)
- `active` : Boolean (actif, défaut True)
- `sequence` : Integer (ordre, défaut 0)

#### Route
- `route_id` : Many2one vers `stock.route` (route, ondelete cascade)

#### Emplacements
- `location_src_id` : Many2one vers `stock.location` (emplacement source)
- `location_dest_id` : Many2one vers `stock.location` (emplacement destination)
- `location_dest_from_rule` : Boolean (destination depuis règle)

#### Type d'opération
- `picking_type_id` : Many2one vers `stock.picking.type` (type d'opération)

#### Action
- `action` : Selection (pull, push, pull_push, pull_mts_else_mto)

#### Méthode d'approvisionnement
- `procure_method` : Selection (make_to_stock, make_to_order, mts_else_mto)

#### Entreposage
- `warehouse_id` : Many2one vers `stock.warehouse` (entrepôt, check_company)

#### Domaine
- `push_domain` : Char (domaine pour push rules)

**Règles métier :**
- Les règles `pull` créent des mouvements vers l'emplacement destination
- Les règles `push` créent des mouvements depuis l'emplacement source
- Les règles sont évaluées dans l'ordre de `sequence`
- Une règle `make_to_order` déclenche un approvisionnement automatique

---

## 2. Workflows et Transitions d'État

### 2.1 Workflow `stock.picking`

**États et transitions :**

```
draft → confirmed → assigned → done
  ↓         ↓          ↓
cancel    cancel    cancel
```

**Actions principales :**

1. **`action_confirm()`** : Confirme le picking
   - Confirme tous les `move_ids` en état `draft`
   - Déclenche le scheduler pour les moves sans stock suffisant
   - Retourne True

2. **`action_assign()`** : Vérifie la disponibilité
   - Confirme les picks en `draft`
   - Trie les moves par priorité et date
   - Appelle `_action_assign()` sur les moves
   - Retourne True

3. **`button_validate()`** : Valide le picking
   - Vérifie les quantités et lots (sanity check)
   - Exécute les wizards de pré-validation
   - Appelle `_action_done()` sur les moves
   - Crée des backorders si nécessaire
   - Retourne True ou une action de rapport

4. **`action_cancel()`** : Annule le picking
   - Annule tous les `move_ids`
   - Verrouille le picking (`is_locked` = True)
   - Retourne True

5. **`action_split_transfer()`** : Divise le picking
   - Crée un backorder avec les quantités non faites
   - Retourne le nouveau picking

**Règles de calcul d'état :**

- `draft` : Si au moins un move est `draft`
- `cancel` : Si tous les moves sont `cancel`
- `done` : Si tous les moves sont `done` ou `cancel` (sans scrap)
- `waiting` : Si au moins un move est `waiting`
- `confirmed` : Si au moins un move est `confirmed` ou `partially_available`
- `assigned` : Si tous les moves sont `assigned` ou si `move_type` = `one` et tous les moves ont `product_uom_qty` > 0

---

### 2.2 Workflow `stock.move`

**États et transitions :**

```
draft → confirmed → assigned → done
  ↓         ↓          ↓
waiting  waiting    cancel
  ↓         ↓
cancel    cancel
```

**Actions principales :**

1. **`_action_confirm()`** : Confirme le move
   - Met à jour `state` selon les conditions
   - Crée des procurements si `procure_method` = `make_to_order`
   - Assigne le move à un picking existant ou crée un nouveau
   - Fusionne les moves similaires si `merge` = True
   - Retourne le recordset des moves (fusionnés)

2. **`_action_assign()`** : Réserve le stock
   - Vérifie la disponibilité via `_get_available_quantity()`
   - Crée des `move_line_ids` avec réservation de quants
   - Met à jour `state` selon le résultat (assigned, partially_available, confirmed)
   - Retourne True

3. **`_action_done()`** : Finalise le move
   - Crée les `move_line_ids` manquantes si nécessaire
   - Met à jour les quants (source -quantity, destination +quantity)
   - Met à jour `state` = `done`
   - Déclenche les règles push si nécessaire
   - Retourne True

4. **`_do_unreserve()`** : Annule la réservation
   - Supprime les `move_line_ids` non `picked`
   - Met à jour `state` selon le résultat
   - Retourne True

**Règles de calcul d'état :**

- `draft` : État initial
- `waiting` : Si `move_orig_ids` existe et n'est pas `done`
- `confirmed` : Si confirmé mais pas réservé
- `partially_available` : Si partiellement réservé
- `assigned` : Si entièrement réservé
- `done` : Si traité
- `cancel` : Si annulé

---

## 3. Règles Métier et Contraintes

### 3.1 Réservation de Stock

**Principe :** La réservation (`_action_assign`) crée des `move_line_ids` qui réservent des `stock.quant` spécifiques.

**Stratégie de retrait :** Déterminée par `removal_strategy_id` sur `stock.location` :
- **FIFO** (First In First Out) : `in_date ASC, id`
- **LIFO** (Last In First Out) : `in_date DESC, id DESC`
- **FEFO** (First Expiry First Out) : `removal_date ASC` (si expiration activée)
- **Closest** : Tri par `complete_name`
- **Least Packages** : Algorithme A* pour minimiser le nombre de colis

**Processus de réservation :**

1. Calcul de la quantité disponible via `_get_available_quantity()`
2. Rassemblement des quants via `_gather()` avec stratégie de retrait
3. Création de `move_line_ids` pour chaque quant réservé
4. Mise à jour de `reserved_quantity` sur les quants
5. Mise à jour de `state` du move

**Règles :**
- Un quant ne peut pas être réservé si `available_quantity` < quantité demandée
- Les quants avec `lot_id` doivent correspondre au lot demandé
- Les quants avec `package_id` peuvent être réservés ensemble
- Les quants avec `owner_id` doivent correspondre au propriétaire

---

### 3.2 Gestion des Lots et Numéros de Série

**Traçabilité produit :**
- `none` : Pas de traçabilité
- `lot` : Traçabilité par lot (plusieurs unités par lot)
- `serial` : Traçabilité par numéro de série (1 unité = 1 SN)

**Règles :**
- Un produit avec `tracking` = `serial` doit avoir `quantity` = 1 dans chaque quant
- Un numéro de série ne peut pas être dupliqué dans les emplacements internes/transit
- Les lots sont créés automatiquement lors de la réception si `use_create_lots` = True
- Les lots existants sont sélectionnés lors de la livraison si `use_existing_lots` = True

**Génération de numéros de série :**
- `_generate_serial_numbers()` : Génère des SN depuis un pattern et un count
- Utilise `stock.lot.generate_lot_names()` pour créer les noms
- Crée des `move_line_ids` avec `quantity` = 1 pour chaque SN

---

### 3.3 Gestion des Colis

**Création de colis :**
- `action_put_in_pack()` : Crée un colis depuis les `move_line_ids`
- Les colis peuvent être hiérarchiques (colis dans colis)
- Un colis peut être déplacé entier si `show_entire_packs` = True

**Règles :**
- Un colis doit avoir un `location_id`
- Le poids d'expédition peut être défini manuellement ou calculé
- Les colis peuvent avoir un `package_type_id` pour règles de route

---

### 3.4 Inventaire Physique

**Mode inventaire (`inventory_mode`) :**
- Permet de définir `inventory_quantity` sur les quants
- Crée automatiquement des `stock.move` pour ajuster les écarts
- Les quants peuvent être assignés à un utilisateur (`user_id`)

**Processus :**
1. Définition de `inventory_quantity` sur les quants
2. Calcul automatique de `inventory_diff_quantity`
3. Application via `action_apply_inventory()` qui crée des moves
4. Validation des moves pour finaliser l'ajustement

**Inventaire cyclique :**
- `cyclic_inventory_frequency` : Fréquence en jours sur `stock.location`
- `inventory_date` : Date planifiée calculée automatiquement
- `last_count_date` : Dernière date de comptage depuis les moves d'inventaire

---

### 3.5 Stratégies de Rangement (Putaway)

**Règles de rangement (`putaway_rule_ids`) :**
- Déterminent où ranger un produit lors de la réception
- Basées sur produit, catégorie, type de colis
- Prendent en compte la capacité et le poids de l'emplacement

**Processus :**
1. `_get_putaway_strategy()` : Trouve l'emplacement de rangement
2. Évalue les règles selon produit/catégorie/colis
3. Vérifie la capacité disponible (`_check_can_be_used()`)
4. Retourne l'emplacement de destination

---

### 3.6 Règles d'Approvisionnement (Stock Rules)

**Types de règles :**
- **Pull** : Crée un mouvement vers l'emplacement destination
- **Push** : Crée un mouvement depuis l'emplacement source
- **Pull Push** : Combine pull et push
- **MTS Else MTO** : Make To Stock si disponible, sinon Make To Order

**Méthodes d'approvisionnement :**
- **Make To Stock (MTS)** : Prend depuis le stock disponible
- **Make To Order (MTO)** : Crée un approvisionnement automatique

**Processus :**
1. Évaluation des règles selon route, produit, emplacement
2. Création de `stock.move` si conditions remplies
3. Confirmation automatique des moves créés
4. Chaînage des moves si nécessaire

---

## 4. Calculs et Formules

### 4.1 Calcul de Disponibilité

**Formule :**
```
available_quantity = quantity - reserved_quantity
```

**Où :**
- `quantity` : Quantité totale en stock (depuis `stock.quant`)
- `reserved_quantity` : Quantité déjà réservée sur d'autres moves

**Calcul pour un move :**
- `availability` = min(`product_qty`, `_get_available_quantity(location_id)`)
- Prend en compte les lots, colis, propriétaires si spécifiés

---

### 4.2 Calcul de Disponibilité Prévisionnelle

**Formule :**
```
forecast_availability = virtual_available - outgoing_moves + incoming_moves
```

**Où :**
- `virtual_available` : Stock virtuel (quantité - réservations + commandes entrantes)
- `outgoing_moves` : Mouvements sortants confirmés
- `incoming_moves` : Mouvements entrants confirmés

**Calcul par entrepôt :**
- Prend en compte les moves en attente et confirmés
- Calcule une date prévue (`forecast_expected_date`) si stock insuffisant

---

### 4.3 Calcul de Poids

**Poids net (`net_weight`) :**
```
net_weight = sum(quant.quantity * product.weight for quant in location.quant_ids)
```

**Poids prévu (`forecast_weight`) :**
```
forecast_weight = net_weight 
                - sum(outgoing_move_line.quantity_product_uom * product.weight)
                + sum(incoming_move_line.quantity_product_uom * product.weight)
```

**Poids d'expédition (`shipping_weight`) :**
```
shipping_weight = weight_bulk 
                + sum(package.shipping_weight for package in packages)
```

---

### 4.4 Calcul de Date Planifiée

**Pour un picking (`scheduled_date`) :**
- Si `move_type` = `direct` : `min(move.date for move in move_ids)`
- Si `move_type` = `one` : `max(move.date for move in move_ids)`

**Pour un move (`date`) :**
- Défaut : `fields.Datetime.now()`
- Peut être modifié manuellement
- Propagé depuis le picking si modifié

---

## 5. Intégrations avec Autres Modules

### 5.1 Intégration avec Sales

**Flux :**
- `sale.order` → `stock.picking` (livraison)
- `sale.order.line` → `stock.move` (mouvement de livraison)
- La confirmation d'une commande crée automatiquement un picking `outgoing`
- Le picking est lié via `origin` = `sale.order.name`

**Données partagées :**
- `partner_id` : Client depuis `sale.order.partner_id`
- `date_deadline` : Date de livraison promise depuis `sale.order.commitment_date`
- `move_ids.product_id` : Produits depuis `sale.order.line.product_id`

---

### 5.2 Intégration avec Purchase

**Flux :**
- `purchase.order` → `stock.picking` (réception)
- `purchase.order.line` → `stock.move` (mouvement de réception)
- La confirmation d'une commande crée automatiquement un picking `incoming`
- Le picking est lié via `origin` = `purchase.order.name`

**Données partagées :**
- `partner_id` : Fournisseur depuis `purchase.order.partner_id`
- `date_deadline` : Date de réception promise depuis `purchase.order.date_planned`
- `move_ids.product_id` : Produits depuis `purchase.order.line.product_id`

---

### 5.3 Intégration avec Accounting

**Flux :**
- `stock.move` → `account.move` (écriture de stock)
- La valorisation du stock crée des écritures comptables
- Les méthodes de valorisation (FIFO, Average, Standard) déterminent le coût

**Données partagées :**
- `price_unit` : Prix unitaire sur `stock.move` pour valorisation
- `account.move.line` : Lignes d'écriture créées lors de `_action_done()`

---

### 5.4 Intégration avec Manufacturing

**Flux :**
- `mrp.production` → `stock.picking` (réception produits finis)
- `mrp.production` → `stock.move` (consommation composants)
- Les composants sont prélevés depuis le stock
- Les produits finis sont stockés dans le stock

**Données partagées :**
- `location_id` : Emplacement source pour composants
- `location_dest_id` : Emplacement destination pour produits finis
- `move_ids` : Mouvements créés automatiquement depuis la nomenclature

---

## 6. Mécanismes Avancés

### 6.1 Backorders

**Définition :** Un backorder est un picking créé automatiquement pour les quantités non traitées lors de la validation.

**Création :**
- Si `create_backorder` = `ask` : Wizard de confirmation
- Si `create_backorder` = `always` : Création automatique
- Si `create_backorder` = `never` : Pas de backorder, quantités annulées

**Processus :**
1. Validation du picking avec quantités partielles
2. Création d'un nouveau picking (`backorder_id` lié)
3. Déplacement des moves non terminés vers le backorder
4. Le backorder reprend le workflow normal

---

### 6.2 Retours

**Définition :** Un retour est un picking créé pour retourner des produits déjà livrés.

**Création :**
- Via `action_return()` sur un picking `done`
- Crée un picking avec `return_id` lié au picking original
- Inverse les emplacements source/destination
- Les moves sont marqués comme `origin_returned_move_id`

**Processus :**
1. Sélection du picking à retourner
2. Création d'un nouveau picking avec type de retour
3. Création de moves inversés
4. Validation normale du picking de retour

---

### 6.3 Cross-Dock

**Définition :** Le cross-dock permet de transférer directement de l'entrée vers la sortie sans passer par le stock.

**Configuration :**
- `xdock_type_id` : Type d'opération cross-dock
- Activé si `reception_steps` != `one_step` et `delivery_steps` != `ship_only`

**Processus :**
1. Réception dans `wh_input_stock_loc_id`
2. Transfert direct vers `wh_output_stock_loc_id` (cross-dock)
3. Livraison depuis `wh_output_stock_loc_id`

---

### 6.4 Inter-Entrepôts

**Définition :** Approvisionnement d'un entrepôt depuis un autre entrepôt.

**Configuration :**
- `resupply_wh_ids` : Entrepôts d'approvisionnement
- Crée automatiquement des routes inter-entrepôts
- Utilise des emplacements de transit

**Processus :**
1. Demande de stock dans l'entrepôt destinataire
2. Création d'un mouvement vers l'emplacement de transit
3. Livraison depuis l'entrepôt fournisseur vers le transit
4. Réception dans l'entrepôt destinataire depuis le transit

---

## 7. Contraintes et Validations

### 7.1 Contraintes sur `stock.picking`

- `name` doit être unique par `company_id`
- `picking_type_id` ne peut pas être modifié si `state` = `done` ou `cancel`
- `location_id` et `location_dest_id` doivent appartenir à la même `company_id`

---

### 7.2 Contraintes sur `stock.move`

- `product_id` doit être un produit stockable (`is_storable` = True)
- `location_id` et `location_dest_id` doivent appartenir à la même `company_id`
- `product_uom_qty` doit être >= 0
- `quantity` (fait) ne peut pas dépasser `product_uom_qty` (demandé)

---

### 7.3 Contraintes sur `stock.quant`

- `product_id` doit être un produit stockable
- `location_id.usage` ne peut pas être `view`
- `lot_id.product_id` doit correspondre à `product_id` si défini
- `quantity` peut être négative si `allow_negative` = True (sinon >= 0)

---

### 7.4 Contraintes sur `stock.location`

- Un emplacement `view` ne peut pas contenir de quants
- Un emplacement ne peut pas être supprimé s'il contient des quants avec `quantity` > 0
- Un emplacement ne peut pas être archivé s'il est utilisé par un entrepôt actif

---

## 8. Performance et Optimisations

### 8.1 Cache de Quants

**Mécanisme :** Un cache de quants (`quants_cache`) est utilisé lors des opérations batch pour éviter les recherches répétées.

**Utilisation :**
- Passé dans le contexte lors des opérations batch
- Clé : `(product_id, location_id, lot_id, package_id, owner_id)`
- Valeur : Recordset de quants correspondants

---

### 8.2 Fusion de Moves

**Mécanisme :** Les moves similaires sont fusionnés automatiquement lors de `_action_confirm()` si `merge` = True.

**Critères de fusion :**
- Même `product_id`, `location_id`, `location_dest_id`
- Même `product_uom`, `price_unit`
- Même `procure_method`, `restrict_partner_id`
- Dates compatibles selon configuration

**Processus :**
1. Groupement des moves par critères
2. Fusion des quantités (`product_uom_qty` additionné)
3. Déplacement des `move_line_ids` vers le move conservé
4. Suppression des moves fusionnés

---

### 8.3 Nettoyage Automatique

**Tâches automatiques :**
- `_merge_quants()` : Fusionne les quants dupliqués
- `_clean_reservations()` : Nettoie les réservations orphelines
- `_unlink_zero_quants()` : Supprime les quants avec quantité = 0

**Déclenchement :**
- Via `_quant_tasks()` appelé avant certaines vues
- Peut être désactivé via paramètre `stock.skip_quant_tasks`

---

## 9. Sécurité et Permissions

### 9.1 Groupes d'Utilisateurs

- `stock.group_stock_user` : Utilisateur de stock (accès de base)
- `stock.group_stock_manager` : Gestionnaire de stock (accès complet)
- `stock.group_stock_multi_locations` : Multi-emplacements
- `stock.group_stock_multi_warehouses` : Multi-entrepôts
- `stock.group_tracking_lot` : Traçabilité lots
- `stock.group_tracking_owner` : Traçabilité propriétaire
- `stock.group_reception_report` : Rapport de réception

---

### 9.2 Restrictions d'Accès

- Les quants ne peuvent être modifiés qu'en mode inventaire (`inventory_mode`)
- Les champs de quant sont restreints en mode inventaire
- Les emplacements peuvent être restreints par groupe d'utilisateurs

---

## 10. Conclusion

L'application **Inventory** d'Odoo est un système complexe de gestion de stock avec :

- **Modèles hiérarchiques** : Picking → Move → MoveLine → Quant
- **Workflows robustes** : États calculés automatiquement depuis les mouvements
- **Réservation intelligente** : Stratégies de retrait configurables
- **Traçabilité complète** : Lots, numéros de série, historique
- **Automatisation** : Règles d'approvisionnement, routes, cross-dock
- **Intégrations** : Sales, Purchase, Accounting, Manufacturing

Cette architecture servira de référence pour l'implémentation d'un équivalent dans l'écosystème Miyukini, en respectant les principes de gouvernance COG et la séparation des responsabilités.

---

**Date de création :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document d'analyse complète
