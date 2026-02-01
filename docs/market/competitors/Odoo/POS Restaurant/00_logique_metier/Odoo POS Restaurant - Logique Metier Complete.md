# Odoo POS Restaurant — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** du module **POS Restaurant** d'Odoo (Point of Sale — Restaurant / Bar). Il identifie les modèles de données, règles métier, workflows, et mécanismes spécifiques à la gestion de restaurant (sols, tables, commandes liées aux tables, transferts, cours, impression cuisine, addition) pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 19.0 — Point of Sale / Restaurant, et structure standard du module `pos_restaurant`.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données (pos.floor, pos.table, ordres POS liés aux tables)
- Règles métier (occupation table, transfert commande, cours, presets Dine In / Takeout / Delivery)
- Workflows (création commande → assignation table → validation → paiement → libération table)
- Impression cuisine et imprimantes de préparation
- Gestion des additions (split, impression avant paiement, pourboires)
- Réservations (Booking) et intégration Appointments
- Intégration avec POS de base (point_of_sale)

**Hors scope :**
- Détail technique du POS générique (document dédié POS Shop)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `pos.floor` (Sol / Étage)

**Rôle :** Représente un étage ou une zone du restaurant (ex. Rez-de-chaussée, Terrasse).

**Champs clés (déduits de la doc) :**
- `name` : Nom du sol (obligatoire)
- `pos_config_id` ou lien vers Point of Sale : Many2one vers configuration POS
- `table_ids` : One2many vers `pos.table`
- Image / fond de plan optionnel pour la vue graphique

**Règles métier :**
- Un sol appartient à un (ou plusieurs) points de vente configurés.
- Les tables sont organisées par sol pour la vue « plan des tables ».

### 1.2 Modèle `pos.table` (Table)

**Rôle :** Représente une table physique (numéro, forme, nombre de couverts, couleur, dimensions).

**Champs clés :**
- `name` / `number` : Numéro ou identifiant de la table
- `floor_id` : Many2one vers `pos.floor`
- `seats` : Nombre de couverts
- `shape` : Forme (square, round)
- `position_h`, `position_v` : Position sur le plan (coordonnées)
- `width`, `height` : Dimensions affichage
- `color` : Couleur sur le plan
- `active` : Table active ou non
- État dérivé : disponible / occupée / réservée (via ordres et booking)

**Règles métier :**
- Sélectionner une table sur le plan confirme l’occupation (liaison ordre ↔ table).
- Une table peut avoir plusieurs ordres ouverts (bouton + pour nouvel ordre sur la même table).
- Libération de table : action « Release table » lorsque le panier est vide.

### 1.3 Ordres POS et liaison Table

**Rôle :** Les ordres POS (`pos.order`) sont étendus pour être associés à une table ou à un « tab » (ordre sans table).

**Concepts :**
- **Set Table :** assigner l’ordre à une table existante (saisie numéro + Assign).
- **Set Tab :** ordre « sans table » (nom d’ordre libre, ex. « Bar », « Comptoir »).
- **New Order :** création d’un ordre direct non lié à une table ; possibilité d’assigner ensuite une table ou un tab.
- **Transfer / Merge :** transfert d’un ordre vers une autre table, ou fusion de commandes sur une table occupée.

**Règles métier :**
- Un ordre est soit lié à une table, soit à un « tab » (identifiant texte).
- Plusieurs ordres peuvent coexister pour la même table (multi-commandes).
- Le transfert déplace l’ordre (et donc l’occupation) vers la table cible ; la fusion regroupe les commandes sur une table.

---

## 2. Workflows et États

### 2.1 Parcours Commande Restaurant

1. **Ouverture contexte**
   - Depuis le plan des tables : clic sur une table → ouverture du registre POS avec table pré-sélectionnée (occupation confirmée).
   - Depuis le registre : « New Order » → panier vide ; « Set Table » ou « Set Tab » pour lier.

2. **Saisie**
   - Ajout de produits au panier.
   - Option **Course** : découpage en cours (Course 1, 2, …) ; envoi séquentiel en cuisine (Order envoie le premier cours ; « Fire Course 2 » ensuite, etc.).
   - **Presets** : Dine In / Takeout / Delivery — déterminent contraintes (table requise, nom, client, créneaux).

3. **Validation**
   - **Order** : valide la commande, envoi en cuisine si imprimante configurée, retour éventuel au plan des tables si écran par défaut = Tables.

4. **Paiement**
   - **Payment** : choix du mode de paiement, client optionnel, facture optionnelle.
   - **Split** : découpage de l’addition (par produit, sous-commande, transfert vers autre table).
   - **Tip** : ajout d’un pourboire (produit TIPS ou montant) avant ou après paiement (selon région et terminal).

5. **Fin de service**
   - Paiement validé → reçu imprimé (si Early Receipt Printing ou imprimante liée).
   - **Release table** : libération de la table lorsque le panier est vide.

### 2.2 Transfert et fusion

- **Transfer/Merge** (Actions) : choix de la table cible.
  - Table libre → transfert de l’ordre (et occupation).
  - Table occupée → fusion des ordres sur la table cible.

### 2.3 Annulation

- **Cancel Order** (Actions) : annulation de l’ordre ; si impression cuisine activée, ticket d’annulation envoyé à l’imprimante.

---

## 3. Règles Métier Spécifiques

### 3.1 Plan des tables (Floor plan)

- Affichage en temps réel : disponible / occupée / réservée / en retard (booking).
- Navigation entre étages via boutons (ex. Main Floor / Patio).
- **Table Selector** : saisie du numéro de table + Jump pour accéder directement.
- Création/édition des sols et tables depuis le backend (Configuration → Floor Plans) ou depuis le frontend POS (Edit Plan).

### 3.2 Cours (Courses)

- Découpage d’une commande en plusieurs envois cuisine (entrées, plats, desserts).
- **Course** : ajout de produits par cours ; **Order** envoie le premier cours en cuisine.
- **Fire Course 2, 3, …** : envoi des cours suivants à la demande.
- Transfert de produit ou d’un cours entier vers un autre cours (Actions → Transfer course).

### 3.3 Presets (Dine In / Takeout / Delivery)

- **Dine In** : assignation table ou tab obligatoire.
- **Takeout** : nom de l’ordre + date/heure de retrait.
- **Delivery** : client (contact) + créneau ; peut s’appuyer sur un module livraison.
- Contrôles de capacité et plages horaires selon configuration (Take out / Delivery / Members).

### 3.4 Impression cuisine (Preparation printers)

- **Printed Product Categories** : association catégories de produits → imprimantes (cuisine, bar, etc.).
- Envoi automatique à l’imprimante concernée lors du clic **Order** (ou selon configuration).
- Impression possible via IoT (imprimante connectée) ou Epson (IP).
- Réimpression : bouton (icône ordre) à côté de Payment pour réimprimer le dernier ordre cuisine.

### 3.5 Additions et paiement

- **Bill splitting** : Split → sélection de produits → Payment (paiement direct), Split Order (sous-commande), ou Transfer (vers une autre table). Chaque sous-commande doit être réglée avant de revenir à l’ordre principal.
- **Early Receipt Printing** : impression du reçu (addition) après paiement réussi ; ou impression manuelle « Print Full Receipt ».
- **Bill** (addition avant paiement) : impression d’une addition pour vérification client avant encaissement (configuration Bar/Restaurant requise).

### 3.6 Pourboires (Tips)

- Produit dédié (ex. [TIPS] Tips) ou montant libre.
- **Tip** avant validation du paiement : ajout du montant puis choix du mode de paiement pour commande + pourboire.
- **Add tip after payment** (US, terminaux Adyen/Stripe) : après clôture, proposition 15 % / 20 % / 25 % ou montant personnalisé.

### 3.7 Réservations (Booking)

- Option **Booking** dans les paramètres POS → intégration avec l’app **Appointments**.
- Type de rendez-vous + ressources = tables ; capacité par ressource.
- Depuis le plan : bouton **Booking** pour créer/éditer/supprimer des réservations ; étapes (Booked, Checked-In, No Show).
- Affichage sur les tables : réservé à telle heure, retard, etc.

---

## 4. Calculs et Données Dérivées

- **État table** : dérivé des ordres ouverts et des réservations (disponible / occupée / réservée / en retard).
- **Montants ordre** : identiques au POS standard (sous-total, taxes, total) ; split et sous-commandes réutilisent les mêmes calculs.
- **Cours** : structure de lignes regroupées par « course » pour l’envoi séquentiel en cuisine ; pas de calcul financier spécifique par cours.

---

## 5. Contraintes et Intégrité

- Une table ne peut pas être supprimée si des ordres ouverts y sont encore liés (ou règles équivalentes de cohérence).
- Suppression d’un sol ou d’une table : définitive (warning dans la doc).
- Presets : cohérence entre type (Dine In / Takeout / Delivery) et champs obligatoires (table/tab, nom, client, créneaux).
- Booking : cohérence entre ressources (tables) et types de rendez-vous ; capacité gérée par « Manage Capacities ».

---

## 6. Synthèse pour Miyukini

**Concepts à traduire en Opérateurs / Kits :**
- **FloorManager** : gestion des sols et tables (CRUD, plan, état en temps réel).
- **TableOrderBinding** : liaison ordre ↔ table / tab ; Set Table, Set Tab, Release table.
- **OrderTransfer** : transfert et fusion d’ordres entre tables.
- **CourseManager** : découpage en cours et envoi séquentiel cuisine.
- **PreparationPrint** : routage des ordres/cours vers imprimantes par catégorie.
- **BillSplit** : découpage d’addition (sous-commandes, transfert).
- **RestaurantPresets** : Dine In / Takeout / Delivery et contraintes associées.
- **RestaurantBooking** : réservations tables via type de rendez-vous et ressources (interop avec Miyukini Appointments si existant).

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
