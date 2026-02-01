# Odoo POS Restaurant — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes au module POS Restaurant d'Odoo (plan des tables, liaison ordre–table, transferts, cours, impression cuisine, split addition, presets, réservations).

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour l’équivalent POS Restaurant
- Contrats d’équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores et avec les services POS / Agenda

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **FloorManager** | Gestion des sols et tables (CRUD, plan, état) | Opérateur de Service |
| **TableOrderBinding** | Liaison ordre ↔ table / tab ; Set Table, Set Tab, Release | Opérateur de Service |
| **OrderTransfer** | Transfert et fusion d’ordres entre tables | Opérateur de Service |
| **CourseManager** | Découpage en cours et envoi séquentiel cuisine | Opérateur de Service |
| **PreparationPrint** | Routage ordres/cours vers canaux préparation (imprimantes, écran) | Opérateur de Service |
| **BillSplit** | Découpage d’addition (sous-commandes, transfert) | Opérateur de Service |
| **RestaurantPresets** | Dine In / Takeout / Delivery et contraintes | Opérateur de Service |
| **RestaurantBooking** | Réservations tables (ressources, créneaux) | Opérateur de Service |
| **RestaurantUI** | Interface plan des tables, registre restaurant, ordres | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : RestaurantService

**Définition :**
> **RestaurantService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion restaurant (tables, commandes liées aux tables, cours, préparation, additions, réservations) au sein du POS.**

**Composition :**
- FloorManager (niveau sécurité 1–2)
- TableOrderBinding (niveau sécurité 2)
- OrderTransfer (niveau sécurité 2)
- CourseManager (niveau sécurité 1)
- PreparationPrint (niveau sécurité 1)
- BillSplit (niveau sécurité 2)
- RestaurantPresets (niveau sécurité 1)
- RestaurantBooking (niveau sécurité 2, interop Agenda)
- RestaurantUI (niveau sécurité 1)

---

## 2. Opérateurs Détaillés

### 2.1 FloorManager

**Rôle :** Gestion des sols (étages) et des tables : création, modification, suppression, plan visuel, état en temps réel (disponible / occupée / réservée).

**Capacités :**
- CRUD sols et tables (nom, forme, places, position, couleur, actif)
- Exposition du plan (données pour affichage)
- Calcul d’état des tables à partir des ordres et des réservations

**Niveau de sécurité :** 1–2 (Standard à Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des sols et tables (WriteIntent)
- **Master Butler** : Permissions de configuration (création/modification plan)
- **WorrySentinel** : Niveau sécurité selon exposition (back-office vs POS)

**Contrat d’équipe :**
- Consommé par : TableOrderBinding, RestaurantUI, RestaurantBooking (ressources = tables)
- Expose : `floor.list`, `floor.create`, `table.update`, `plan.state`

### 2.2 TableOrderBinding

**Rôle :** Liaison ordre POS ↔ table ou tab ; Set Table, Set Tab, Release table.

**Capacités :**
- Assigner un ordre à une table (par id ou numéro)
- Assigner un ordre à un « tab » (nom libre, sans table)
- Libérer une table (Release) lorsque l’ordre est réglé / panier vide
- Fournir le contexte table/tab pour l’ordre courant

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision d’assignation (table déjà occupée, conflits)
- **KindMother** : Persistance de la liaison ordre–table (WriteIntent)
- **Master Butler** : Permissions de lier/délier

**Contrat d’équipe :**
- Consomme : FloorManager (tables), POS Order (ordre)
- Expose : `binding.set_table`, `binding.set_tab`, `binding.release`

### 2.3 OrderTransfer

**Rôle :** Transfert d’un ordre vers une autre table ; fusion de deux ordres sur une même table.

**Capacités :**
- Transfert : déplacer l’ordre (et l’occupation) vers une table cible libre
- Fusion : regrouper les lignes d’un ordre avec un ordre existant sur une table occupée

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Validation du transfert/fusion (cohérence montants, états)
- **KindMother** : Mise à jour des ordres et liaisons (WriteIntent)
- **Master Butler** : Permission de transfert/fusion

**Contrat d’équipe :**
- Consomme : TableOrderBinding, FloorManager, POS Order
- Expose : `transfer.to_table`, `merge.into_order`

### 2.4 CourseManager

**Rôle :** Découpage d’une commande en cours (entrées, plats, desserts) et envoi séquentiel vers la préparation.

**Capacités :**
- Attacher des lignes à un numéro de cours (Course 1, 2, …)
- Déclencher l’envoi d’un cours (Order = cours 1 ; Fire Course 2, etc.)
- Transfert de ligne ou de cours vers un autre cours

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **KindMother** : Persistance de la structure cours (WriteIntent)
- **Master Butler** : Permissions de modification des cours
- **PreparationPrint** : Consommation des cours pour envoi aux canaux

**Contrat d’équipe :**
- Consomme : POS Order (lignes)
- Expose : `course.add`, `course.fire`, `course.transfer`
- Alimente : PreparationPrint

### 2.5 PreparationPrint

**Rôle :** Routage des ordres/cours vers les canaux de préparation (imprimantes, écran cuisine, webhook).

**Capacités :**
- Configuration : catégorie produit (ou tag) → canal (imprimante, écran)
- Envoi des lignes d’un cours ou d’un ordre au(x) canal(aux) concerné(s)
- Gestion des annulations (ticket d’annulation)

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions de configuration des canaux
- Pas de décision métier sur les montants ; uniquement routage

**Contrat d’équipe :**
- Consommé par : CourseManager, POS Order (validation)
- Expose : `prepare.send`, `prepare.cancel`
- Consomme : catalogue produits (catégories)

### 2.6 BillSplit

**Rôle :** Découpage de l’addition : sous-commandes, paiement par part, transfert de lignes vers une autre table.

**Capacités :**
- Split : sélection de lignes → création de sous-commande ou paiement direct ou transfert
- Gestion des états (sous-commande à régler avant retour à l’ordre principal)
- Montants et taxes cohérents par part

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Validation du split (montants, cohérence)
- **KindMother** : Persistance des sous-commandes et liens (WriteIntent)
- **Master Butler** : Permission de split et paiement

**Contrat d’équipe :**
- Consomme : POS Order, POS Payment
- Expose : `split.suborder`, `split.pay_part`, `split.transfer`

### 2.7 RestaurantPresets

**Rôle :** Presets Dine In / Takeout / Delivery : contraintes et champs obligatoires (table/tab, nom, client, créneau).

**Capacités :**
- Définition des presets (actifs, champs requis, règles capacité/heures)
- Validation des données selon preset sélectionné
- Application des limites (créneaux, quantité) si configurées

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **KindMother** : Persistance des configurations preset
- **Master Butler** : Permissions de configuration
- **Caring Nanny** : État (heures d’ouverture, capacité) pour blocage si dégradé

**Contrat d’équipe :**
- Consommé par : TableOrderBinding, RestaurantUI
- Expose : `preset.list`, `preset.validate`

### 2.8 RestaurantBooking

**Rôle :** Réservations de tables (ressources = tables) ; interop avec service Agenda/RDV.

**Capacités :**
- Création/édition/suppression de réservations (nom, heure, convives, durée, tables)
- Gestion des étapes (Booked, Checked-In, No Show)
- Exposition des réservations pour le plan des tables (affichage statut)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de création/modification réservation (conflits, capacité)
- **KindMother** : Persistance des réservations ou délégation au service Agenda (WriteIntent)
- **Master Butler** : Permissions de réserver

**Contrat d’équipe :**
- Consomme : FloorManager (tables comme ressources), Miyukini Agenda (si existant) pour créneaux et types de rdv
- Expose : `booking.create`, `booking.update`, `booking.list_by_floor`
- Équipe : collaboration avec l’Opérateur Agenda pour ressources partagées

### 2.9 RestaurantUI

**Rôle :** Interface utilisateur : plan des tables, registre restaurant (Set Table, Set Tab, Course, Split, etc.), liste des ordres.

**Capacités :**
- Affichage du plan (états, navigation étages, Table Selector)
- Actions : New Order, Set Table, Set Tab, Order, Transfer/Merge, Course, Split, Payment, Release table
- Édition du plan (Edit Plan) si mandat

**Niveau de sécurité :** 1 (Standard)

**Gouvernance :**
- **Master Butler** : Permissions d’affichage et d’action selon rôle (serveur, maître d’hôtel, admin)
- **WorrySentinel** : Pas d’élévation de niveau ; lecture seule ou actions mandatées

**Contrat d’équipe :**
- Consomme : FloorManager, TableOrderBinding, OrderTransfer, CourseManager, BillSplit, RestaurantPresets, RestaurantBooking, POS Order/Payment
- Expose : écrans et actions utilisateur (pas d’API métier directe)

---

## 3. Contrat d'Équipe RestaurantService

**Flux autorisés (résumé) :**
- RestaurantUI → TableOrderBinding, OrderTransfer, CourseManager, BillSplit, RestaurantBooking
- TableOrderBinding → FloorManager, POS Order
- OrderTransfer → TableOrderBinding, FloorManager, POS Order
- CourseManager → POS Order, PreparationPrint
- BillSplit → POS Order, POS Payment
- RestaurantBooking → FloorManager, Agenda (ressources)
- PreparationPrint → canaux externes (imprimantes, écran)

**Mandats de Permission typiques :**
- **Service en salle** : Mandat avec TableOrderBinding, CourseManager, BillSplit, FloorManager (lecture), POS Order/Payment.
- **Gestion du plan** : Mandat avec FloorManager (écriture), TableOrderBinding (lecture).
- **Réservations** : Mandat avec RestaurantBooking, FloorManager (lecture), Agenda si fédéré.
- **Configuration** : Mandat avec FloorManager, PreparationPrint, RestaurantPresets (écriture).

---

## 4. Intégration avec les Cores

- **StrongFather** : Décisions sur assignation table, transfert/fusion, split, réservation.
- **KindMother** : Toute persistance (sols, tables, liaisons, cours, sous-commandes, réservations) via WriteIntent.
- **Master Butler** : Permissions et capacités (floor.manage, binding.set_table, transfer, course.fire, split, booking.create, prepare.send).
- **WorrySentinel** : Niveaux de sécurité par opérateur ; pas d’élévation ; audit des actions sensibles (split, transfert, réservation).
- **Ever Buddy** : Cycle de vie des ordres et des réservations (états, historique).
- **Caring Nanny** : État du système (imprimantes, capacité) pour bloquer ou dégrader si besoin.

---

## 5. Correspondance Odoo → Miyukini

| Odoo | Miyukini |
|------|----------|
| pos.floor, pos.table | FloorManager |
| Set Table, Set Tab, Release table | TableOrderBinding |
| Transfer/Merge | OrderTransfer |
| Courses, Fire Course | CourseManager |
| Preparation Printers, Printed Product Categories | PreparationPrint |
| Split (bill) | BillSplit |
| Dine In / Takeout / Delivery presets | RestaurantPresets |
| Booking (Appointments) | RestaurantBooking + Agenda |
| Plan des tables + Registre + Orders | RestaurantUI |

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
