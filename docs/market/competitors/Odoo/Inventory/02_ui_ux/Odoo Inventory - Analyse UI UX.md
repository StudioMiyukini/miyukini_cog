# Odoo Inventory — Analyse UI/UX Complète

## Contexte

Ce document analyse en profondeur l'**interface utilisateur et l'expérience utilisateur** de l'application Inventory d'Odoo, extraite du code source GitHub (vues XML, templates, composants).

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/stock/views`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Vues principales (Liste, Formulaire, Kanban, Calendrier, Graphique, Pivot)
- Composants d'interface (formulaires, champs, widgets spécialisés)
- Patterns de navigation
- Responsive design et mobile
- Accessibilité (hotkeys, raccourcis)
- Recommandations pour Miyukini

---

## 1. Vues Principales

### 1.1 Vue Liste (Tree/List) — `stock.picking`

**Fichier :** `stock_picking_views.xml` → `vpicktree`

**Caractéristiques :**
- Colonnes principales : `name` (référence), `location_id`, `location_dest_id`, `partner_id`, `scheduled_date`, `state`, `products_availability`
- Colonnes optionnelles : `user_id`, `date_deadline`, `date_done`, `origin`, `backorder_id`, `picking_type_id`, `company_id`
- Décoration : `decoration-danger` pour `state` = `cancel`, `decoration-info` pour `assigned`, `decoration-muted` pour `draft`, `decoration-success` pour `done`, `decoration-warning` pour autres états
- Boutons header : "Unreserve", "Check Availability" (actions groupées)
- Widget spécialisé : `js_class="stock_list_view"` pour fonctionnalités avancées

**Widgets utilisés :**
- `priority` : Widget de priorité (0=Normal, 1=Urgent)
- `many2one_avatar_user` : Avatar utilisateur pour `user_id`
- `remaining_days` : Affichage jours restants pour `scheduled_date` et `date_deadline`
- `badge` : Badge d'état avec couleurs
- `stock_rescheduling_popover` : Popover pour réordonnancement (`json_popover`)
- `activity_exception` : Décoration d'exception d'activité

**Colonnes spéciales :**
- `products_availability` : Statut de disponibilité avec décoration (success=available, warning=expected, danger=late)
- `products_availability_state` : État de disponibilité (lazy loading)
- `json_popover` : Popover de réordonnancement (invisible si pas de retard)

**Recommandations pour Miyukini :**
- Vue liste claire avec colonnes essentielles (référence, emplacements, dates, état)
- Indicateurs visuels de disponibilité et retards
- Actions groupées (annulation réservation, vérification disponibilité)
- Lazy loading pour performances

---

### 1.2 Vue Kanban — `stock.picking`

**Fichier :** `stock_picking_views.xml` → `stock_picking_kanban`

**Caractéristiques :**
- Classe mobile : `o_kanban_mobile` pour optimisation mobile
- Progressbar pour activités : `activity_state` avec couleurs (planned=success, today=warning, overdue=danger)
- Cards avec informations essentielles : `name`, `partner_id`, `scheduled_date`, `state`, `picking_properties`
- Widget `kanban_activity` pour activités
- Widget `stock_rescheduling_popover` pour réordonnancement

**Structure card :**
```xml
<div class="d-flex mb-1">
    <field name="priority" widget="priority"/>
    <field name="name" class="fw-bold fs-5 ms-1"/>
    <field name="state" widget="label_selection" class="ms-auto"/>
</div>
<field name="picking_properties" widget="properties"/>
<footer>
    <div class="d-flex">
        <field name="partner_id"/>
        <field name="activity_ids" widget="kanban_activity"/>
        <field name="json_popover" widget="stock_rescheduling_popover"/>
    </div>
    <div class="d-flex ms-auto mt-1 align-items-center">
        <field name="scheduled_date" options="{'show_time': false}"/>
        <field name="user_id" widget="many2one_avatar_user"/>
    </div>
</footer>
```

**Recommandations pour Miyukini :**
- Cards visuelles avec informations clés (référence, partenaire, date, état)
- Indicateurs de priorité et activités
- Responsive mobile optimisé
- Popover contextuel pour actions rapides

---

### 1.3 Vue Formulaire — `stock.picking`

**Fichier :** `stock_picking_views.xml` → `view_picking_form`

**Structure :**

#### Header
- Statusbar avec états selon `picking_type_code` :
  - `incoming` : `draft`, `assigned`, `done`
  - Autres : `draft`, `confirmed`, `assigned`, `done`
- Boutons contextuels selon état :
  - Draft : "Mark as Todo" (`action_confirm`, hotkey `q`)
  - Confirmed/Waiting/Assigned : "Check Availability" (`action_assign`, hotkey `w`)
  - Assigned/Waiting : "Validate" (`button_validate`, hotkey `v`)
  - Assigned : "Print" (`do_print_picking`, hotkey `o`)
  - Done : "Print" (rapport), "Return" (`act_stock_return_picking`, hotkey `k`)
  - Outgoing : Widget `signature` pour signature client
- Hotkeys : `q` (Confirm), `w` (Check Availability), `v` (Validate), `o` (Print), `k` (Return), `x` (Cancel)

#### Alertes
- `picking_warning_text` : Instructions de picking depuis partenaire (si `group_stock_warning_stock`)

#### Sheet (Contenu principal)

**Button Box (Statistiques) :**
- Bouton "Returns" : Nombre de retours (`return_count`)
- Bouton "Scraps" : Opérations de casse (`has_scrap_move`)
- Bouton "Packages" : Nombre de colis (`packages_count`, visible si `group_tracking_lot`)
- Bouton "Traceability" : Rapport de traçabilité (si `has_tracking` et `group_production_lot`)
- Bouton "Allocation" : Rapport d'allocation (si `show_allocation` et `group_reception_report`)
- Bouton "Operations" : Vue liste des moves (si `group_no_one`)
- Bouton "Moves" : Vue détaillée des move lines
- Bouton "Next Transfer" : Transferts suivants (si `show_next_pickings`)

**Titre :**
- Priorité (`priority` widget)
- Nom (`name`) ou "New [picking_type_code]" si nouveau

**Groupes de champs :**
- **Gauche :**
  - `partner_id` : Label dynamique selon `picking_type_code` (Delivery Address / Receive From / Contact)
  - `picking_type_id` : Type d'opération (readonly si done/cancel)
  - `location_id` / `location_dest_id` : Emplacements (selon groupes et `picking_type_code`)
  - `backorder_id` : Backorder de (si existe)
- **Droite :**
  - `scheduled_date` : Date planifiée (readonly si `not is_date_editable`, décoration warning/danger si retard)
  - `json_popover` : Popover réordonnancement (si retard)
  - `date_deadline` : Date limite (décoration danger si dépassée)
  - `products_availability` : Disponibilité produits (décoration selon état)
  - `date_done` : Date effective (si done)
  - `origin` : Document source
  - `owner_id` : Propriétaire (si `group_tracking_owner` et incoming)

**Propriétés :**
- `picking_properties` : Propriétés dynamiques (2 colonnes)

**Notebook (Onglets) :**
- **Operations** : Liste/Kanban des `move_ids` avec widget `stock_move_one2many`
- **Additional Info** : `move_type`, `user_id`, `company_id`
- **Note** : `note` (notes internes)

**Widget spécialisé `stock_move_one2many` :**
- Liste éditable en bas (`editable="bottom"`)
- Colonnes principales : `product_id`, `forecast_availability`, `description_picking`, `product_uom_qty` (Demand), `quantity` (Quantity), `product_uom`, `picked`, `lot_ids`
- Widget `forecast_widget` pour disponibilité prévue
- Widget `move_product_label_field` pour `product_id`
- Bouton "Details" (`action_show_details`) pour opérations détaillées
- Bouton "Put in Pack" pour mise en colis (hotkey `shift+g`)

**Recommandations pour Miyukini :**
- Formulaire structuré avec header contextuel
- Statusbar claire avec états visuels
- Hotkeys pour actions fréquentes
- Widgets spécialisés pour opérations de stock
- Onglets pour organisation du contenu

---

### 1.4 Vue Calendrier — `stock.picking`

**Fichier :** `stock_picking_views.xml` → `stock_picking_calendar`

**Caractéristiques :**
- Date de départ : `scheduled_date`
- Couleur : `partner_id` (couleur selon partenaire)
- Limite d'événements : 5 par jour
- Quick create : Désactivé (`quick_create="0"`)
- Filtres : `partner_id`, `picking_type_id`, `state`, `picking_properties`

**Recommandations pour Miyukini :**
- Vue calendrier pour planification
- Couleurs par partenaire ou type d'opération
- Limite d'affichage pour performance

---

### 1.5 Vue Inventaire (Quants) — `stock.quant`

**Fichier :** `stock_quant_views.xml` → `view_stock_quant_tree_inventory_editable`

**Caractéristiques :**
- Liste éditable en bas (`editable="bottom"`)
- Mode inventaire : `inventory_mode` activé
- Colonnes principales : `location_id`, `product_id`, `lot_id`, `package_id`, `owner_id`, `inventory_date`, `quantity` (On Hand), `inventory_quantity` (Counted), `inventory_diff_quantity` (Difference)
- Décoration : `decoration-warning` pour `is_outdated`, `decoration-danger` pour `sn_duplicated` ou `inventory_diff_quantity < 0`, `decoration-success` pour `inventory_diff_quantity > 0`
- Widget spécialisé : `counted_quantity_widget` pour `inventory_quantity`
- Boutons header : "Apply All", "Apply", "Clear", "Request a Count"
- Boutons ligne : "Apply" (`action_apply_inventory`), "Clear" (`action_clear_inventory_quantity`), "History" (`action_inventory_history`)

**Widget `counted_quantity_widget` :**
- Widget personnalisé pour saisie quantité comptée
- Validation en temps réel
- Calcul automatique de l'écart

**Recommandations pour Miyukini :**
- Interface d'inventaire claire et éditable
- Widgets spécialisés pour comptage
- Indicateurs visuels d'écarts et conflits
- Actions rapides (Apply, Clear)

---

### 1.6 Vue Liste Quants (Rapport) — `stock.quant`

**Fichier :** `stock_quant_views.xml` → `view_stock_quant_tree_editable`

**Caractéristiques :**
- Liste éditable en bas (`editable="bottom"`)
- Création/édition activées (`create="1" edit="1"`)
- Colonnes principales : `location_id`, `product_id`, `package_id`, `lot_id`, `owner_id`, `inventory_quantity_auto_apply`, `reserved_quantity`, `available_quantity`, `product_uom_id`
- Sommes : Total On Hand, Total Reserved, Total Available
- Boutons ligne : "History" (`action_view_stock_moves`), "Replenishment" (`action_view_orderpoints`)
- Widget `package_m2o` pour `package_id`

**Recommandations pour Miyukini :**
- Vue rapport avec sommes
- Actions contextuelles (historique, réapprovisionnement)
- Widgets spécialisés pour colis

---

### 1.7 Vue Pivot — `stock.quant`

**Fichier :** `stock_quant_views.xml` → `view_stock_quant_pivot`

**Caractéristiques :**
- Lignes : `product_id`
- Colonnes : `location_id`
- Mesure : `quantity`

**Recommandations pour Miyukini :**
- Vue pivot pour analyse multi-dimensionnelle
- Groupements par produit, emplacement, lot, etc.

---

### 1.8 Vue Graphique — `stock.quant`

**Fichier :`stock_quant_views.xml` → `stock_quant_view_graph`

**Caractéristiques :**
- Axe X : `location_id`
- Mesure : `quantity`

**Recommandations pour Miyukini :**
- Graphiques pour visualisation des stocks
- Différents types (barres, lignes, camembert)

---

## 2. Widgets Spécialisés

### 2.1 Widget `stock_move_one2many`

**Usage :** Liste/Kanban des moves dans un picking

**Fonctionnalités :**
- Édition inline des moves
- Widget `forecast_widget` pour disponibilité prévue
- Widget `move_product_label_field` pour sélection produit
- Gestion des lots/SN avec `many2many_tags`
- Bouton "Details" pour opérations détaillées

**Recommandations pour Miyukini :**
- Widget spécialisé pour gestion des mouvements
- Édition inline pour rapidité
- Prévisualisation de disponibilité

---

### 2.2 Widget `forecast_widget`

**Usage :** Affichage disponibilité prévue sur `stock.move`

**Fonctionnalités :**
- Affiche `forecast_availability` et `forecast_expected_date`
- Indicateurs visuels (disponible, attendu, en retard)
- Popover avec détails

**Recommandations pour Miyukini :**
- Widget de prévision intégré
- Indicateurs visuels clairs

---

### 2.3 Widget `counted_quantity_widget`

**Usage :** Saisie quantité comptée dans inventaire

**Fonctionnalités :**
- Validation en temps réel
- Calcul automatique de l'écart
- Formatage selon UoM

**Recommandations pour Miyukini :**
- Widget spécialisé pour inventaire
- Validation et feedback immédiat

---

### 2.4 Widget `package_m2o` / `package_m2m`

**Usage :** Sélection de colis

**Fonctionnalités :**
- Affichage hiérarchique des colis
- Recherche par nom
- Création rapide

**Recommandations pour Miyukini :**
- Widget spécialisé pour colis
- Gestion hiérarchique

---

### 2.5 Widget `stock_rescheduling_popover`

**Usage :** Popover de réordonnancement

**Fonctionnalités :**
- Affiche `delay_alert_date`
- Liste des éléments en retard (`late_elements`)
- Actions de réordonnancement

**Recommandations pour Miyukini :**
- Popover contextuel pour actions rapides
- Informations pertinentes sans surcharge

---

### 2.6 Widget `move_product_label_field`

**Usage :** Sélection produit dans move

**Fonctionnalités :**
- Affichage avec code-barres
- Recherche optimisée
- Validation produit stockable

**Recommandations pour Miyukini :**
- Widget spécialisé pour sélection produit
- Intégration code-barres

---

## 3. Patterns de Navigation

### 3.1 Menu Principal

**Structure :**
- Inventaire → Opérations → Réceptions (`action_picking_tree_incoming`)
- Inventaire → Opérations → Livraisons (`action_picking_tree_outgoing`)
- Inventaire → Opérations → Transferts Internes (`action_picking_tree_internal`)
- Inventaire → Opérations → Tous les Transferts (`action_picking_tree_all`)
- Inventaire → Ajustements → Inventaire Physique (`action_view_inventory_tree`)
- Inventaire → Rapports → Emplacements (`action_view_quants`)

**Recommandations pour Miyukini :**
- Navigation claire par type d'opération
- Accès rapide aux fonctions principales

---

### 3.2 Filtres et Recherche

**Filtres principaux :**
- Par état : Draft, Waiting, Ready, Done, Cancel
- Par type : Receipts, Deliveries, Internal
- Par date : Before, Yesterday, Today, Tomorrow, Late
- Par disponibilité : Late Availability
- Par backorder : Backorders
- Par activités : My Activities, Late Activities, Today Activities

**Recherche :**
- Par référence (`name`) ou origine (`origin`)
- Par partenaire (`partner_id`)
- Par produit (`product_id`)
- Par lot/SN (`lot_id`)
- Par colis (`package_id`)

**Groupements :**
- Par état (`state`)
- Par date planifiée (`scheduled_date`)
- Par document source (`origin`)
- Par type d'opération (`picking_type_id`)
- Par pays destination (`partner_country_id`)
- Par propriétés (`picking_properties`)

**Recommandations pour Miyukini :**
- Filtres contextuels selon vue
- Recherche multi-critères
- Groupements flexibles

---

### 3.3 Actions Rapides

**Depuis liste :**
- Sélection multiple → Actions groupées (Unreserve, Check Availability)
- Clic droit → Menu contextuel
- Hotkeys : `q` (Validate), `w` (Check Availability), etc.

**Depuis formulaire :**
- Boutons header selon état
- Hotkeys pour actions fréquentes
- Widgets contextuels (signature, popover)

**Recommandations pour Miyukini :**
- Actions rapides accessibles
- Hotkeys pour productivité
- Menu contextuel riche

---

## 4. Responsive Design et Mobile

### 4.1 Optimisations Mobile

**Kanban :**
- Classe `o_kanban_mobile` pour optimisation mobile
- Cards adaptées petit écran
- Gestes tactiles

**Liste :**
- Colonnes adaptatives
- Colonnes optionnelles masquables
- Scroll horizontal si nécessaire

**Formulaire :**
- Layout adaptatif
- Champs empilés sur mobile
- Boutons accessibles

**Recommandations pour Miyukini :**
- Design mobile-first pour opérations terrain
- Interface tactile optimisée
- Performance sur mobile

---

### 4.2 Code-Barres

**Intégration :**
- App Barcode recommandée
- Scan code-barres pour produits, lots, emplacements, colis
- Actions rapides depuis scan

**Recommandations pour Miyukini :**
- Intégration native code-barres
- Scan pour toutes les entités (produits, lots, emplacements)
- Actions automatiques depuis scan

---

## 5. Accessibilité

### 5.1 Hotkeys (Raccourcis Clavier)

**Picking :**
- `q` : Confirmer / Valider
- `w` : Vérifier disponibilité
- `v` : Valider
- `o` : Imprimer
- `k` : Retour
- `x` : Annuler
- `shift+g` : Mettre en colis

**Wizard Backorder :**
- `q` : Créer backorder
- `w` : Pas de backorder
- `x` : Annuler

**Wizard Retour :**
- `q` : Retourner

**Recommandations pour Miyukini :**
- Hotkeys cohérents avec Odoo
- Raccourcis pour actions fréquentes
- Documentation accessible

---

### 5.2 Indicateurs Visuels

**Couleurs :**
- Success (vert) : Disponible, Done
- Warning (orange) : Expected, Waiting
- Danger (rouge) : Late, Cancel, Not Available
- Info (bleu) : Assigned
- Muted (gris) : Draft

**Décoration :**
- `decoration-success` : État positif
- `decoration-warning` : Attention requise
- `decoration-danger` : Problème critique
- `decoration-info` : Information
- `decoration-muted` : Inactif

**Recommandations pour Miyukini :**
- Palette de couleurs cohérente
- Indicateurs visuels clairs
- Accessibilité (contraste, daltonisme)

---

### 5.3 Messages et Alertes

**Types :**
- Alertes partenaire (`picking_warning_text`)
- Warnings produits (archivés, indisponibles)
- Erreurs validation (quantités, lots manquants)
- Confirmations (backorder, retour)

**Recommandations pour Miyukini :**
- Messages clairs et actionnables
- Alertes contextuelles
- Feedback immédiat

---

## 6. Composants d'Interface Avancés

### 6.1 Statusbar

**Usage :** Affichage état avec transitions visuelles

**États affichés :**
- Incoming : `draft`, `assigned`, `done`
- Autres : `draft`, `confirmed`, `assigned`, `done`

**Recommandations pour Miyukini :**
- Statusbar claire avec états visuels
- Transitions animées
- Clic pour changer d'état (si autorisé)

---

### 6.2 Progressbar

**Usage :** Progression activités dans Kanban

**États :**
- Planned : Success (vert)
- Today : Warning (orange)
- Overdue : Danger (rouge)

**Recommandations pour Miyukini :**
- Progressbar pour indicateurs de progression
- Couleurs cohérentes

---

### 6.3 Properties Widget

**Usage :** Propriétés dynamiques sur picking

**Fonctionnalités :**
- Champs personnalisables selon `picking_type_id`
- Types : texte, nombre, date, sélection, etc.
- Affichage en 2 colonnes

**Recommandations pour Miyukini :**
- Système de propriétés extensible
- Configuration par type d'opération

---

## 7. Patterns d'Interaction

### 7.1 Édition Inline

**Usage :** Modification directe dans liste

**Exemples :**
- Quantités dans moves
- Quantités comptées dans inventaire
- Lots/SN dans moves

**Recommandations pour Miyukini :**
- Édition inline pour rapidité
- Validation en temps réel
- Annulation facile (ESC)

---

### 7.2 Actions Groupées

**Usage :** Actions sur sélection multiple

**Exemples :**
- Unreserve plusieurs pickings
- Check Availability plusieurs pickings
- Apply All dans inventaire

**Recommandations pour Miyukini :**
- Actions groupées pour efficacité
- Confirmation pour actions destructives

---

### 7.3 Wizards Contextuels

**Usage :** Dialogs pour décisions

**Exemples :**
- Wizard backorder (`stock.backorder.confirmation`)
- Wizard retour (`stock.return.picking`)
- Wizard conflit inventaire (`stock.inventory.conflict`)

**Recommandations pour Miyukini :**
- Wizards pour décisions complexes
- Interface claire avec explications
- Actions rapides (hotkeys)

---

## 8. Performance et Optimisations

### 8.1 Lazy Loading

**Usage :** Chargement différé de données

**Exemples :**
- `products_availability_state` : Lazy loading
- `forecast_availability` : Calcul à la demande

**Recommandations pour Miyukini :**
- Lazy loading pour colonnes lourdes
- Pagination intelligente
- Cache des données fréquentes

---

### 8.2 Sample Data

**Usage :** Données d'exemple pour performance

**Exemples :**
- `sample="1"` sur listes et kanban
- Affichage de quelques enregistrements pour layout

**Recommandations pour Miyukini :**
- Sample data pour développement
- Performance optimisée en production

---

## 9. Recommandations pour Miyukini

### 9.1 Interface Mobile-First

**Recommandation :** Prioriser l'interface mobile pour opérations terrain.

**Justification :** Préparateurs et réceptionnistes travaillent sur mobile/tablette.

**Implémentation :**
- Design responsive avec breakpoints adaptés
- Gestes tactiles (swipe, tap, long-press)
- Scan code-barres natif
- Interface simplifiée pour mobile

---

### 9.2 Widgets Spécialisés

**Recommandation :** Créer des widgets spécialisés pour opérations de stock.

**Justification :** Améliore l'expérience utilisateur et réduit les erreurs.

**Implémentation :**
- Widget `forecast_widget` pour disponibilité prévue
- Widget `counted_quantity_widget` pour inventaire
- Widget `package_widget` pour colis
- Widget `lot_selector` pour lots/SN

---

### 9.3 Feedback Visuel Immédiat

**Recommandation :** Fournir un feedback visuel immédiat sur toutes les actions.

**Justification :** Réduit les erreurs et améliore la confiance.

**Implémentation :**
- Indicateurs de disponibilité en temps réel
- Décoration visuelle pour états et écarts
- Messages de confirmation/erreur clairs
- Animations subtiles pour transitions

---

### 9.4 Hotkeys et Raccourcis

**Recommandation :** Implémenter des hotkeys cohérents pour actions fréquentes.

**Justification :** Améliore la productivité des utilisateurs expérimentés.

**Implémentation :**
- Hotkeys standards : `q` (Valider), `w` (Vérifier), `v` (Valider), `x` (Annuler)
- Hotkeys contextuels selon vue
- Documentation accessible (aide contextuelle)

---

### 9.5 Intégration Code-Barres

**Recommandation :** Intégration native du scan code-barres.

**Justification :** Essentiel pour opérations d'entrepôt.

**Implémentation :**
- Scan pour produits, lots, emplacements, colis
- Actions automatiques depuis scan
- Validation en temps réel
- Support GS1 et formats standards

---

### 9.6 Vue Kanban Optimisée

**Recommandation :** Vue Kanban optimisée pour suivi opérations.

**Justification :** Vue visuelle efficace pour gestion opérationnelle.

**Implémentation :**
- Cards avec informations essentielles
- Progressbar pour activités
- Drag & drop pour changement d'état (si workflow)
- Filtres par colonne

---

### 9.7 Interface d'Inventaire Intuitive

**Recommandation :** Interface d'inventaire claire et intuitive.

**Justification :** Réduit les erreurs de comptage.

**Implémentation :**
- Liste éditable avec widget spécialisé
- Indicateurs visuels d'écarts
- Actions rapides (Apply, Clear)
- Import/export Excel

---

## 10. Conclusion

L'interface utilisateur d'Odoo Inventory révèle :

- **Diversité des vues** : Liste, Kanban, Formulaire, Calendrier, Pivot, Graphique
- **Widgets spécialisés** : Forecast, Counted Quantity, Package, etc.
- **Optimisations mobile** : Interface adaptée terrain
- **Accessibilité** : Hotkeys, indicateurs visuels, messages clairs
- **Performance** : Lazy loading, sample data, optimisations

L'implémentation Miyukini devra :
- Prioriser l'interface mobile pour opérations terrain
- Créer des widgets spécialisés pour opérations de stock
- Fournir un feedback visuel immédiat
- Intégrer le scan code-barres nativement
- Optimiser les performances pour grands volumes

---

**Date de création :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document d'analyse complète
