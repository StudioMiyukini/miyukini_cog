# Odoo Manufacturing — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Manufacturing** (MRP) d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, patterns mrp / stock / purchase / sale.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo (Stock, Purchase, Sale, Inventory)
- Flux de données inter-apps
- Mécanismes d'intégration (création OF, réservations, sous-traitance)
- Événements et synchronisation

**Hors scope :**
- Détail des modèles internes Manufacturing (document Logique Métier)

---

## 1. Dépendances Principales

### 1.1 Modules requis

**Dépendances explicites (typiques du module MRP Odoo) :**
- `base` : Partenaires, sociétés, utilisateurs
- `stock` : Mouvements, emplacements, entrepôts, réservations, lots/séries
- `product` : Produits, variantes, UoM
- `mail` : Activités, suivi, notifications
- `mrp` (core manufacturing) : BOM, OF, WO, postes, gammes

### 1.2 Modules optionnels

- `purchase` : Achats, sous-traitance (commandes fournisseur pour sous-traitance), réception
- `sale` : Commandes client, création OF depuis vente, origine OF
- `sale_management` : Workflows vente avancés
- `mrp_plm` : PLM (ingénierie, révisions BOM)
- `quality` : Contrôles qualité sur WO
- `maintenance` : Demandes maintenance depuis WO
- `hr` / `mrp_workorder` : Temps et ressources humaines sur WO
- `iot` : Postes connectés (scan, capteurs)
- `account` : Coûts, comptabilité analytique

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Stock (Inventory)

**Flux :**
- Manufacturing consomme et produit des **stock.move** (matières et produits finis).
- OF confirmé → création des `stock.move` (raw + finished) selon BOM.
- Réservation des composants selon règles stock (FIFO, FEFO, etc.).
- Clôture OF → moves en `done`, mise à jour des quants.

**Modèles partagés / étendus :**
- `stock.move` : champs `raw_material_production_id`, `production_id`, `workorder_id`, `bom_line_id`
- `stock.picking` : réceptions / livraisons liées (sous-traitance, retours)
- `stock.location` : emplacements production, sous-traitant, rebut
- `stock.rule` : règles d’approvisionnement « Manufacture » pour créer des OF

**Règles métier :**
- Les mouvements OF respectent les types d’opération (picking_type_id) et emplacements configurés.
- Traçabilité : lots/séries sur moves et propagation vers OF/WO.

**Recommandations Miyukini :**
- Réutiliser le même modèle de mouvements que MiyuInventory / LogisticsSteward.
- WriteIntent pour toute création/modification de move liée à un OF.

### 2.2 Intégration avec Purchase

**Flux :**
- **Sous-traitance :** BOM (ou ligne) en sous-traitance → création ou liaison à une commande fournisseur ; envoi composants au sous-traitant, réception produit fini.
- **Composants achetés :** besoin net (OF confirmés) alimente les propositions d’achat ou les commandes (selon règles d’approvisionnement).

**Champs / liens :**
- `purchase.order` / `purchase.order.line` liés à l’OF ou au bon de sous-traitance
- Lien entre `stock.move` (réception produit fini) et OF sous-traité

**Recommandations Miyukini :**
- Définir clairement qui crée la « commande sous-traitant » (Manufacturing vs Purchase) et comment le Stock enregistre les entrées/sorties.

### 2.3 Intégration avec Sales

**Flux :**
- Commande client (sale.order) → besoin en produit fini.
- Création d’OF avec **origin** = référence commande (manuel ou MPS / automate).
- Suivi : livraison prévue liée à la date fin OF.

**Champs / liens :**
- `mrp.production.origin` : nom de la commande
- Liens optionnels `sale.order` ↔ `mrp.production` (champs relationnels si module sale_mrp)
- Propagation « livrable quand » depuis OF vers commande (date promise)

**Recommandations Miyukini :**
- Lien explicite Commande → OF (Mandat entre Sales et Manufacturing pour créer OF).
- StrongFather pour valider la création d’OF depuis une commande.

### 2.4 Intégration avec Inventory (réapprovisionnement)

**Flux :**
- **Points de commande / réapprovisionnement :** règle stock « Manufacture » → proposition ou création automatique d’OF.
- `stock.warehouse.orderpoint` : produit fabriqué, route « Manufacture » → création `mrp.production` (ou wizard).

**Modèles :**
- `stock.rule` : type « manufacture », action = créer OF
- `procurement.group` : regroupement des besoins (plusieurs lignes → un OF possible)
- `mrp.production.orderpoint_id` : lien OF ↔ orderpoint

**Recommandations Miyukini :**
- Un seul moteur de règles d’approvisionnement (Inventory) qui appelle Manufacturing pour « fabriquer » (création OF sous gouvernance).

### 2.5 Intégration avec Quality / Maintenance / HR (optionnel)

- **Quality :** points de contrôle sur WO ou sur OF ; alertes qualité depuis le poste.
- **Maintenance :** demande de maintenance depuis WO (panne, entretien).
- **HR / Timesheet :** temps passé par opérateur sur WO (time_ids).

Ces intégrations sont optionnelles mais courantes en atelier ; prévoir des Mandats et flux de données limités (qui peut créer une alerte, qui peut lier un temps, etc.).

---

## 3. Mécanismes d'Intégration

### 3.1 Création d’OF

- **Manuelle :** utilisateur choisit produit, quantité, BOM.
- **Depuis vente :** bouton « Fabrication » sur commande ou wizard « Créer OF ».
- **Depuis MPS :** plan directeur propose des OF ; l’utilisateur valide et crée.
- **Depuis réapprovisionnement :** règle stock « Manufacture » + orderpoint → création OF (ou proposition).

Dans tous les cas, l’OF créé doit être conforme aux règles Manufacturing (BOM valide, produit fabriqué, etc.) et les mouvements stock créés à la confirmation.

### 3.2 Réservations et disponibilité

- Manufacturing s’appuie sur le moteur de réservation Stock (stock.move → reservation, assign).
- Alertes « composants manquants » ou « dates retard » basées sur les quants et les mouvements.
- Rapports d’allocation (composants par OF) utilisent les mêmes données Stock.

### 3.3 Sous-traitance

- Sortie composants : move vers emplacement sous-traitant (ou partenaire).
- Réception produit fini : move entrant depuis sous-traitant, lié à l’OF.
- Purchase : commande fournisseur pour suivre la sous-traitance (coût, délai, réception).

### 3.4 Coûts et comptabilité

- **Account** : coûts OF (matière, main-d’œuvre, overhead) et écritures analytiques.
- Manufacturing calcule les coûts ; Account enregistre (ou consomme) les écritures selon configuration.
- Intégration à prévoir côté Miyukini avec le service Comptabilité / Coûts (Mandat en écriture analytique).

---

## 4. Synthèse des Flux

| Source        | Cible          | Donnée / événement                          |
|---------------|----------------|---------------------------------------------|
| Sale          | Manufacturing  | Besoin produit → création OF (origin SO)    |
| Stock (rule)  | Manufacturing  | Orderpoint / besoin → création OF           |
| Manufacturing | Stock          | Moves matières + finis (création / done)     |
| Manufacturing | Purchase       | Commande sous-traitant, réception          |
| Manufacturing | Account        | Coûts OF, analytique                        |
| Quality/Maintenance | Manufacturing | Alertes liées aux WO                        |

---

## 5. Points d'Attention pour Miyukini

- **BondingBrother** : traduction des intentions « fabriquer pour cette commande » ou « couvrir ce besoin » en création d’OF avec Mandat.
- **KindMother** : tous les moves (Stock) et états OF/WO sont des écritures gouvernées (WriteIntent).
- **StrongFather** : décision de créer un OF, de confirmer, de clôturer ou de créer un backorder.
- **Cohérence Supply Chain** : un seul référentiel Stock (MiyuInventory) ; Manufacturing consomme et produit via les mêmes contrats que les autres flux (achat, vente, transferts).

---

**Document** : Odoo Manufacturing — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01
