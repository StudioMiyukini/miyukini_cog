# Odoo POS Restaurant — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** du module POS Restaurant d'Odoo : dépendances obligatoires et optionnelles, flux de données, mécanismes d'intégration (POS de base, Appointments, IoT, comptabilité, produits) et recommandations pour Miyukini.

**Source d'analyse :** Documentation Odoo 19.0, structure module pos_restaurant.

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec point_of_sale et autres apps
- Flux de données (ordres, tables, réservations, impression)
- Mécanismes d'intégration (Booking, Preparation printers, Presets)
- APIs et hooks typiques
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Module Requis

**Dépendance centrale :**
- **point_of_sale** : POS Restaurant étend le POS standard. Tous les flux (ordres, paiements, produits, taxes, reçus, facturation) passent par les modèles et la logique du POS de base.

**Conséquences :**
- `pos.order` (et lignes) : modèle commun ; pos_restaurant ajoute la liaison table / floor.
- Configuration POS : option « Is a Bar/Restaurant » et paramètres (Floor Plans, Preparation Printers, Take out / Delivery, Booking, Tips, Bill Splitting, Early Receipt).
- Session POS : même mécanisme d’ouverture/fermeture de caisse ; les ordres restaurant sont des ordres POS avec contexte table/tab et cours.

### 1.2 Modules Optionnels

- **appointments** : Réservations (Booking). Activation de Booking dans le POS installe/utilise l’app Appointments ; types de rendez-vous, ressources (= tables), capacités.
- **iot** : Imprimantes de préparation connectées via IoT Box (cuisine, bar).
- **product** : Déjà dépendance du POS (catalogue, catégories). POS Restaurant utilise les **Printed Product Categories** pour router les ordres vers les bonnes imprimantes.
- **account** : Facturation des ordres POS (client, facture) ; même flux que le POS standard.
- **hr** (optionnel) : Gestion des employés pour attribution des ventes / droits (selon config Odoo).

---

## 2. Flux de Données

### 2.1 POS ↔ Restaurant

- **Sols / Tables** : `pos.floor`, `pos.table` créés en backend ou en frontend (Edit Plan) ; liés à une config POS (`pos.config`).
- **Ordres** : `pos.order` avec champs ou relations vers table/floor (ex. `table_id`, ou équivalent) ; « tab » = ordre sans table (nom libre).
- **État des tables** : dérivé en temps réel des ordres ouverts et des réservations (affichage plan).

### 2.2 Restaurant → Cuisine / Bar

- **Impression préparation** : à la validation (Order), les lignes sont envoyées aux imprimantes selon **Printed Product Categories** (catégorie produit → imprimante). Flux via driver imprimante (IoT ou Epson).
- **Cours** : structure des lignes par « course » ; envoi séquentiel (Order = cours 1 ; Fire Course 2, etc.) vers les mêmes imprimantes/catégories.
- **Annulation** : Cancel Order peut déclencher un ticket d’annulation vers l’imprimante configurée.

### 2.3 Restaurant → Comptabilité / Facturation

- Identique au POS standard : ordre payé → reçu ; option client + facture → génération `account.move` (facture).
- Aucun flux spécifique « restaurant » ; les montants, taxes, paiements suivent la logique POS + Accounting.

### 2.4 Restaurant ↔ Appointments (Booking)

- **Configuration** : type de rendez-vous avec ressources = tables ; capacité par ressource (Manage Capacities).
- **Réservations** : créées/éditées depuis le POS (écran Booking) ; stockées dans le modèle Appointments (rdv, ressources, créneaux).
- **Affichage plan** : statut « réservé », heure, retard, etc. lu depuis les rendez-vous liés aux tables (ressources).
- **Checked-In / No Show** : étapes du parcours rendez-vous ; mise à jour depuis le POS.

### 2.5 Takeout / Delivery

- **Presets** : Dine In / Takeout / Delivery ; champs (nom, client, créneau) et règles de capacité/heures selon configuration.
- **Données** : nom d’ordre (tab), partenaire (client), créneau ; peuvent alimenter un module livraison ou planning si présent (hors scope standard doc Odoo).

---

## 3. Mécanismes d'Intégration

### 3.1 Floor Plans et POS Config

- Les sols et tables sont filtrés par point de vente (config POS). Chaque config peut avoir ses propres plans.
- En session, le frontend charge les floors/tables de la config courante ; les mises à jour (Edit Plan) sont persistées dans les mêmes modèles.

### 3.2 Preparation Printers

- **Configuration** : POS Settings → Preparation → Preparation Printers ; création d’imprimantes (IoT ou Epson), association **Printed Product Categories**.
- **Runtime** : au clic Order (et Fire Course N), le backend ou le frontend envoie les lignes concernées au(x) printer(s) selon catégories des produits.
- **IoT** : communication avec IoT Box qui pilote l’imprimante physique ; **Epson** : envoi direct à l’IP configurée.

### 3.3 Booking (Appointments)

- **Ressources = tables** : dans le type de rendez-vous, on sélectionne les tables comme ressources ; capacité = 1 par table (ou plus si tables fusionnées).
- **Linked Resource** : pour réserver plusieurs tables ensemble (ex. grande table).
- **POS** : écran Booking lit/écrit les rendez-vous ; le plan des tables affiche les créneaux et états (Booked, Checked-In, No Show).

### 3.4 Presets et Contraintes

- **Take out / Delivery / Members** : réglage dans POS ; champs obligatoires et contrôles (créneaux, capacité) selon le preset sélectionné dans le registre.
- Les données saisies (client, créneau, nom) sont portées par l’ordre POS ; pas de modèle dédié « livraison » dans la doc standard.

### 3.5 Tips et Paiement

- **Tips** : produit spécial (ex. [TIPS] Tips) ou montant ; enregistré comme ligne ou paiement selon implémentation.
- **Add tip after payment** : spécifique US + terminaux Adyen/Stripe ; flux post-clôture vers le terminal.
- Intégration **payment** : même mécanisme que le POS standard (méthodes de paiement, terminaux).

---

## 4. APIs et Hooks (conceptuels)

- **Extension des vues POS** : pos_restaurant ajoute l’onglet/écran « Tables » (plan) et les actions Set Table, Set Tab, Course, Transfer/Merge, Split, Release table.
- **Hooks sur ordre** : à la validation (Order), envoi des lignes aux imprimantes ; à l’annulation, ticket d’annulation.
- **État des tables** : calcul côté frontend ou backend à partir des ordres ouverts + réservations (requêtes ou bus d’événements).
- **Booking** : utilisation de l’API Appointments (création/édition de rendez-vous, ressources, étapes).

---

## 5. Recommandations pour Miyukini

- **Couplage POS / Restaurant** : un Opérateur « POS Restaurant » qui étend le POS générique (MiyuPOSShop ou équivalent) avec Floor/Table, Course, Split, Presets ; pas de duplication des flux paiement/facturation.
- **Impression préparation** : abstraction « canal de préparation » (imprimante, écran, webhook) avec routage par catégorie ou tag ; indépendant du matériel Odoo/IoT.
- **Réservations** : contrat d’équipe avec un service RDV/Appointments (Miyukini Agenda ou équivalent) : ressources = tables, créneaux, étapes ; le plan des tables consomme les réservations en lecture.
- **Données Takeout/Delivery** : modéliser nom, client, créneau dans l’ordre ; prévoir extension vers un module livraison/planning si besoin, sans dépendance forte au POS Restaurant.
- **Intégrité** : garder une seule source de vérité pour les ordres (POS) et pour les tables (Floor/Table) ; les réservations comme couche complémentaire (ressources partagées).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
