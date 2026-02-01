# Odoo Manufacturing — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Manufacturing** (MRP) d'Odoo (version 19.0), alignée sur la documentation officielle et les patterns Supply Chain. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, patterns `mrp` (Manufacturing / MRP)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (BOM, Manufacturing Order, Work Order, Work Center, Routing)
- Règles métier et contraintes
- Workflows et transitions d'état (draft → confirmed → progress → done)
- Nomenclatures (BOM) et structures multi-niveaux
- Ordres de fabrication et ordres de travail
- Postes de travail et dépendances entre opérations
- Sous-traitance, démontage (unbuild), sous-produits
- Plan directeur (MPS), backorders, lots et numéros de série
- Coûts de production et reporting (OEE, délais, allocation)

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `mrp.bom` (Nomenclature / Bill of Materials)

**Rôle :** Définit les **composants** et quantités nécessaires pour fabriquer un produit fini, ainsi que les opérations et postes de travail optionnels.

**Champs clés :**
- `product_tmpl_id` / `product_id` : Produit fabriqué (template ou variante)
- `product_qty` : Quantité de produit fini pour cette BOM
- `product_uom_id` : Unité de mesure
- `type` : normal, phantom, kit (type de BOM)
- `bom_line_ids` : One2many vers `mrp.bom.line` (composants)
- `routing_id` : Many2one vers `mrp.routing` (gamme optionnelle)
- `company_id` : Société
- `code` : Code nomenclature
- `active` : Nomenclature active
- `ready_to_produce` : all_available / asap (démarrage production)
- `consumption` : strict / flexible (consommation composants)
- `allow_operation_dependencies` : Dépendances entre opérations
- `picking_type_id` : Type d'opération stock (entrée/sortie)

**Types de BOM :**
- `normal` : BOM standard
- `phantom` : Sous-assemblage « virtuel » (non stocké comme tel)
- `kit` : Kit expédié sans fabrication (vente en kit)

**Règles métier :**
- Une BOM doit avoir au moins une ligne de composant (sauf cas particuliers)
- Plusieurs BOM peuvent exister pour un même produit (variantes, alternatives)
- BOM multi-niveaux : une ligne peut pointer vers un produit lui-même fabriqué (sous-assemblage)
- Consommation : strict = exacte, flexible = autorise dépassement
- Ready to produce : all_available = tous les composants disponibles, asap = dès que possible

### 1.2 Modèle `mrp.bom.line` (Ligne de Nomenclature)

**Rôle :** Une ligne = un composant avec quantité et optionnellement opération / emplacement.

**Champs clés :**
- `bom_id` : Nomenclature parente
- `product_id` : Produit composant
- `product_qty` : Quantité requise
- `product_uom_id` : Unité
- `operation_id` : Many2one vers `mrp.routing.workcenter` (opération qui consomme ce composant)
- `location_id` : Emplacement de prélèvement (optionnel)
- `bom_product_id` : Produit fabriqué (pour variantes)
- `sequence` : Ordre d'affichage

**Règles métier :**
- La quantité doit être > 0
- Si `operation_id` est renseigné, le composant est consommé à cette opération (sinon au démarrage ou selon config)

### 1.3 Modèle `mrp.production` (Ordre de Fabrication / Manufacturing Order)

**Rôle :** Représente un **ordre de fabrication** : fabriquer une quantité d'un produit fini à une date donnée, selon une BOM.

**États (state) :**
- `draft` : Brouillon
- `confirmed` : Confirmé (réservations / ordres de travail créés)
- `progress` : En cours
- `to_close` : À clôturer
- `done` : Terminé
- `cancel` : Annulé

**Champs clés :**
- `name` : Référence OF (séquence)
- `product_id` : Produit à fabriquer
- `product_qty` : Quantité à produire
- `product_uom_id` : Unité
- `bom_id` : Nomenclature utilisée
- `product_qty_produced` : Quantité déjà produite
- `state` : État
- `date_planned_start` / `date_planned_finished` : Dates planifiées
- `date_start` / `date_finished` : Dates réelles
- `move_raw_ids` : Mouvements stock matières (composants)
- `move_finished_ids` : Mouvements stock produits finis
- `workorder_ids` : Ordres de travail (si BOM avec gamme)
- `picking_type_id` : Type d'opération stock
- `location_src_id` / `location_dest_id` : Emplacements source / destination
- `origin` : Origine (commande vente, autre OF, etc.)
- `orderpoint_id` : Lien réapprovisionnement (si créé par règle)
- `procurement_group_id` : Groupe d'approvisionnement
- `backorder_sequence` : Numéro de backorder (0 = OF initial)
- `cost_share` : Part de coût (pour sous-produits)
- `allow_workorder_creation` : Création WO possible (selon BOM)

**Règles métier :**
- En mode « simple » (BOM sans gamme), pas d'ordres de travail : consommation et production gérées directement sur l'OF
- En mode « avec gamme », les work orders sont créés à partir du routing de la BOM
- Réservation des composants à la confirmation (stock.move)
- Clôture : lorsque quantité produite atteinte ou OF marqué comme terminé (avec éventuel backorder)
- Un OF peut être créé depuis une commande vente, un point de commande, ou manuellement

### 1.4 Modèle `mrp.workorder` (Ordre de Travail / Work Order)

**Rôle :** Représente une **opération** sur un ordre de fabrication : une étape à un poste de travail, avec temps et instructions.

**États (state) :**
- `pending` : En attente
- `ready` : Prêt (précédences OK)
- `progress` : En cours
- `done` : Terminé
- `cancel` : Annulé

**Champs clés :**
- `name` : Libellé opération
- `production_id` : Ordre de fabrication parent
- `workcenter_id` : Poste de travail
- `operation_id` : Many2one vers `mrp.routing.workcenter` (définition opération)
- `product_id` : Produit fabriqué (copie OF)
- `qty_production` : Quantité à produire
- `qty_produced` : Quantité produite
- `state` : État
- `date_planned_start` / `date_planned_finished` : Planifiées
- `date_start` / `date_finished` : Réelles
- `duration_expected` : Durée prévue
- `duration` : Durée réelle (calculée)
- `next_work_order_id` : Dépendance (opération suivante)
- `blocked_by_workorder_ids` : Dépendances (doit attendre ces WO)
- `worksheet` : Instructions (HTML / PDF)
- `quality_alert_ids` : Alertes qualité
- `maintenance_requests_ids` : Demandes maintenance
- `time_ids` : Détail des temps (début/fin par utilisateur)

**Règles métier :**
- Les WO sont créés à partir du routing de la BOM à la confirmation de l'OF
- Dépendances : un WO peut être bloqué par d'autres WO (blocked_by_workorder_ids)
- Consommation des composants : à l'opération définie sur la ligne BOM, ou au démarrage selon config
- Production : enregistrée au niveau WO ou au niveau OF selon le flux (one-step, two-step, three-step)

### 1.5 Modèle `mrp.workcenter` (Poste de Travail)

**Rôle :** Représente un **poste de travail** (machine, ligne, cellule) avec capacité et coûts.

**Champs clés :**
- `name` : Nom du poste
- `code` : Code court
- `capacity` : Capacité (nombre de cycles simultanés, ex. 1)
- `time_efficiency` : Efficacité (100 % = nominal)
- `costs_hour` : Coût horaire
- `costs_hour_account_id` : Compte analytique coût
- `resource_calendar_id` : Calendrier (disponibilité)
- `routing_line_ids` : One2many vers `mrp.routing.workcenter` (opérations qui utilisent ce poste)
- `alternative_workcenter_ids` : Postes alternatifs
- `company_id` : Société

**Règles métier :**
- Utilisé dans les gammes (routing) pour définir où et combien de temps
- Capacité et calendrier permettent le calcul des plages disponibles (planification)
- Coûts utilisés pour valoriser les ordres de fabrication

### 1.6 Modèle `mrp.routing` (Gamme) et `mrp.routing.workcenter` (Opération de gamme)

**Rôle :**
- **mrp.routing** : Ensemble d'opérations (gamme) pour fabriquer un produit.
- **mrp.routing.workcenter** : Une opération = un poste de travail + temps de réglage et temps par unité.

**Champs clés (routing.workcenter) :**
- `routing_id` : Gamme
- `workcenter_id` : Poste de travail
- `name` : Libellé opération
- `sequence` : Ordre
- `time_cycle_manual` : Durée manuelle (fixe)
- `time_mode` : manual / auto (calcul depuis product)
- `time_mode_batch` : Nombre d'unités pour le cycle
- `batch_size` : Taille de lot pour le temps
- `bom_id` : Lien BOM (pour composants par opération)
- `worksheet_type` : text / pdf (instructions)
- `worksheet_google_slide` : Lien Google Slide (optionnel)
- `blocked_by_operation_ids` : Dépendances (opérations qui doivent être terminées avant)

**Règles métier :**
- Une BOM peut avoir une gamme (routing_id) : les WO sont créés à partir de ces opérations
- Dépendances entre opérations : blocked_by_operation_ids → répliqué sur workorder (blocked_by_workorder_ids)
- Temps : réglage (setup) + temps par unité ou temps cycle

### 1.7 Modèles `stock.move` (Mouvements) liés à la fabrication

**Rôle :** Les mouvements de stock pour matières (raw) et produits finis (finished) sont des `stock.move` avec `raw_material_production_id` ou `production_id` (produit fini).

**Champs pertinents (côté MRP) :**
- `raw_material_production_id` : OF parent (mouvement matière)
- `production_id` : OF parent (mouvement produit fini)
- `workorder_id` : WO où le composant est consommé (si consommation par opération)
- `product_id`, `product_uom_qty`, `quantity` (fait), `state` : Idem Inventory
- `bom_line_id` : Ligne de BOM source (optionnel)
- `unit_factor` : Facteur par rapport à la quantité OF (pour sous-produits / coûts)

**Règles métier :**
- Créés à la confirmation de l'OF à partir des lignes BOM
- Réservation (assign) selon stratégie stock
- Consommation : done sur le move quand les composants sont prélevés (ou au WO)
- Produit fini : move vers emplacement de production, puis done quand production validée

---

## 2. Workflows et Transitions d'État

### 2.1 Ordre de Fabrication (mrp.production)

```
draft → confirmed → progress → to_close → done
         ↓            ↓           ↓
       cancel      cancel      cancel
```

- **draft** : Saisie, choix BOM, quantité, dates. Modifiable.
- **confirmed** : BOM et quantité figés (selon paramétrage), mouvements stock créés, WO créés si gamme. Réservations.
- **progress** : Au moins un WO en cours ou consommation/production démarrée (mode simple).
- **to_close** : Toute la quantité produite ou marquée à clôturer (backorder possible).
- **done** : OF clôturé. Mouvements et WO en done.
- **cancel** : Annulation ; mouvements annulés, réservations libérées.

### 2.2 Ordre de Travail (mrp.workorder)

```
pending → ready → progress → done
   ↓         ↓         ↓
cancel    cancel    cancel
```

- **pending** : Créé, en attente de dépendances (blocked_by_workorder_ids).
- **ready** : Dépendances satisfaites, peut être démarré.
- **progress** : Démarré (date_start renseignée), en cours.
- **done** : Terminé (date_finished, durée, quantité produite enregistrées).

### 2.3 Backorders et Split

- Si à la clôture d'un OF la quantité produite est inférieure à la quantité demandée, Odoo peut créer un **backorder** (nouvel OF avec le reliquat, même BOM, lien backorder_id).
- **Split** : découper un OF en plusieurs (quantités).
- **Merge** : fusionner des OF (même produit, même BOM).

---

## 3. Règles Métier Transverses

### 3.1 Planification et dates

- **date_planned_start** / **date_planned_finished** : planification OF et WO.
- **MPS (Master Production Schedule)** : vue planification pour lancer les OF selon demande (ventes, stock) et capacités.
- Dépendances entre WO : un WO ne passe en **ready** que lorsque les WO bloquants sont **done**.

### 3.2 Consommation et scrap

- **Consommation** : strict (quantité BOM) ou flexible (dépassement autorisé).
- **Scrap** : produits mis au rebut pendant la fabrication ; mouvements scrap ou lignes dédiées, impact stock et coût.
- **By-products** : produits co-produits (quantité et coût partiel via cost_share).

### 3.3 Sous-traitance

- BOM ou ligne BOM avec **subcontracting** : les composants sont envoyés au sous-traitant, le produit fini (ou la finition) revient.
- Mouvements : sortie vers emplacement sous-traitant, réception produit fini.
- Lien purchase (commande fournisseur) pour suivre la sous-traitance.

### 3.4 Unbuild (démontage)

- **Unbuild** : démonter un produit fini en composants (inverse de la BOM). Crée des mouvements stock (produit fini → sortie, composants → entrée) et peut créer des OF « unbuild » ou un modèle dédié.

### 3.5 Lots et numéros de série

- Produits en lot/série : traçabilité sur moves et WO (lot_id, serial_id).
- Fabrication avec attribution de lot/série au produit fini et aux composants consommés.

### 3.6 Coûts

- **Coût matière** : valorisation des composants (coût standard ou moyen selon stock).
- **Coût poste** : durée WO × coût horaire poste.
- **Coût OF** : somme matières + postes ; répartition possible sur sous-produits (cost_share).
- Comptabilité analytique : comptes et répartition sur OF.

---

## 4. Points d'Attention pour Miyukini

- **Opérateurs** : distinguer Nomenclature (BOM), Ordre de fabrication (OF), Ordre de travail (WO), Poste de travail (Work Center), Planification (MPS), Reporting (OEE, délais, allocation).
- **KindMother** : toute création/modification de stock (moves) et d’états OF/WO doit passer par WriteIntent.
- **StrongFather** : décisions de confirmation OF, clôture, backorder, annulation.
- **Inventory** : Manufacturing s’appuie sur les mêmes concepts que Inventory (stock.move, locations, picking_type) ; cohérence avec MiyuInventory / LogisticsSteward.
- **Traçabilité** : lots/séries et historique des mouvements pour conformité et rappel.
- **Sous-traitance** : frontière avec Purchase et Stock ; définir qui émet la demande (OF sous-traité vs commande fournisseur).

---

**Document** : Odoo Manufacturing — Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01
