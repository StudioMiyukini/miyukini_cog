# Odoo Inventory — Spécifications Opérateurs Miyukini

## Contexte

Ce document définit les **spécifications d'Opérateurs Miyukini** pour implémenter les fonctionnalités équivalentes à l'application Inventory (Stock) d'Odoo, en respectant l'architecture COG et la gouvernance Miyukini.

**Références :**
- [Miyukini Conceptual References - Glossaire](../../../../reference/Miyukini%20Conceptual%20References%20-%20Glossaire.md)
- [Odoo Inventory - Logique Métier](../00_logique_metier/Odoo%20Inventory%20-%20Logique%20Metier%20Complete.md)
- [Odoo Inventory - Intégrations Cross-App](../03_integrations/Odoo%20Inventory%20-%20Integrations%20Cross%20App.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document définit :**
- Opérateurs identifiés pour équivalent Inventory
- Contrats d'équipe et Mandats de Permission
- Niveaux de sécurité
- Intégration avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel)

**Hors scope :**
- Implémentation technique détaillée (voir Guide d'Implémentation)
- Spécifications UI/UX (document dédié)

---

## 1. Architecture Opérateurs

### 1.1 Vue d'Ensemble

L'équivalent Inventory dans Miyukini s'appuie sur un **service COG Miyukini Inventory** (ou MiyuInventory), avec des **Opérateurs spécialisés** pour la gestion des stocks, transferts, emplacements et inventaires.

**Opérateurs identifiés :**

| Opérateur | Rôle | Type |
|-----------|------|------|
| **StockPicking** | Gestion des transferts (réception, livraison, interne) | Opérateur de Service |
| **StockMove** | Gestion des mouvements de stock | Opérateur de Service |
| **StockQuant** | Gestion des quantités en stock par emplacement | Opérateur de Service |
| **StockLocation** | Gestion des emplacements et hiérarchie | Opérateur de Service |
| **StockWarehouse** | Gestion des entrepôts et routes | Opérateur de Service |
| **StockLot** | Gestion des lots et numéros de série | Opérateur de Service |
| **StockPackage** | Gestion des colis et emballages | Opérateur de Service |
| **StockRule** | Gestion des règles d'approvisionnement | Opérateur de Service |
| **StockInventory** | Gestion des inventaires physiques | Opérateur de Service |
| **StockUI** | Interface utilisateur inventaire | Opérateur d'Interface |

### 1.2 Équipe d'Opérateurs : InventoryService

**Définition :**
> **InventoryService est une Équipe d'Opérateurs qui collabore sous règles explicites pour délivrer le service de gestion des stocks.**

**Composition :**
- StockPicking (niveau sécurité 2)
- StockMove (niveau sécurité 2)
- StockQuant (niveau sécurité 2)
- StockLocation (niveau sécurité 2)
- StockWarehouse (niveau sécurité 2)
- StockLot (niveau sécurité 2)
- StockPackage (niveau sécurité 1-2)
- StockRule (niveau sécurité 2)
- StockInventory (niveau sécurité 2)
- StockUI (niveau sécurité 1)

**Contrat d'Équipe :** Voir section 2

---

## 2. Opérateurs Détaillés

### 2.1 StockPicking

**Rôle :** Gestion des transferts de stock (réception, livraison, transfert interne).

**Capacités :**
- Création/modification de transferts
- Confirmation et réservation (sous gouvernance StrongFather si nécessaire)
- Validation des transferts
- Gestion des backorders et retours

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision de validation (optionnel selon politique)
- **KindMother** : Persistance des transferts (WriteIntent)
- **Master Butler** : Permissions de création/modification/validation
- **WorrySentinel** : Vérification niveau sécurité, état système

**Contrat d'équipe :**
- Consomme : StockMove, StockLocation, StockWarehouse, StockLot, StockPackage
- Expose : `picking.create`, `picking.confirm`, `picking.assign`, `picking.validate`, `picking.return`

**Mandat de Permission requis :**
- Création transfert : Mandat avec StockLocation + StockWarehouse
- Validation transfert : Mandat avec Master Butler (permissions)

### 2.2 StockMove

**Rôle :** Gestion des mouvements de stock (lignes de transfert).

**Capacités :**
- Création/modification de mouvements
- Réservation de stock (via StockQuant)
- Validation des mouvements
- Chaînage des mouvements (orig/dest)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des mouvements (WriteIntent)
- **Master Butler** : Permissions
- **WorrySentinel** : Vérification disponibilité, cohérence

**Contrat d'équipe :**
- Consomme : StockQuant (réservation), StockLot, StockPackage
- Expose : `move.confirm`, `move.assign`, `move.done`, `move.unreserve`

### 2.3 StockQuant

**Rôle :** Gestion des quantités physiques en stock par emplacement, lot, colis.

**Capacités :**
- Lecture des quantités disponibles/réservées
- Ajustement via inventaire physique
- Calcul de disponibilité et prévision

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des quants (WriteIntent pour ajustements)
- **Master Butler** : Permissions de lecture/ajustement
- **WorrySentinel** : Vérification avant ajustement (inventaire)

**Contrat d'équipe :**
- Consommé par : StockMove (réservation), StockInventory (ajustement)
- Expose : `quant.read`, `quant.apply_inventory`, `quant.available_quantity`

### 2.4 StockLocation

**Rôle :** Gestion des emplacements et hiérarchie (view, internal, supplier, customer, etc.).

**Capacités :**
- Création/modification d'emplacements
- Configuration stratégies de retrait (FIFO, LIFO, FEFO)
- Règles de rangement (putaway)
- Inventaire cyclique

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des emplacements
- **Master Butler** : Permissions de configuration
- **Ever Buddy** : Compatibilité hiérarchie (cycle de vie)

**Contrat d'équipe :**
- Consommé par : StockPicking, StockMove, StockQuant, StockWarehouse
- Expose : `location.get`, `location.children`, `location.putaway_rules`

### 2.5 StockWarehouse

**Rôle :** Gestion des entrepôts, routes et types d'opération.

**Capacités :**
- Création/modification d'entrepôts
- Configuration des étapes de réception/livraison
- Configuration des routes et règles
- Types d'opération (réception, livraison, transfert interne)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des entrepôts
- **Master Butler** : Permissions de configuration
- **Ever Buddy** : Compatibilité routes (cycle de vie)

**Contrat d'équipe :**
- Consommé par : StockPicking, StockLocation, StockRule
- Expose : `warehouse.get`, `warehouse.routes`, `warehouse.picking_types`

### 2.6 StockLot

**Rôle :** Gestion des lots et numéros de série (traçabilité).

**Capacités :**
- Création de lots/SN
- Consultation et historique
- Dates d'expiration/utilisation

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des lots
- **Master Butler** : Permissions
- **WorrySentinel** : Traçabilité (niveau selon secteur)

**Contrat d'équipe :**
- Consommé par : StockMove, StockQuant
- Expose : `lot.create`, `lot.get`, `lot.history`

### 2.7 StockPackage

**Rôle :** Gestion des colis et emballages.

**Capacités :**
- Création de colis
- Hiérarchie colis (colis dans colis)
- Poids et volume d'expédition

**Niveau de sécurité :** 1-2 (Standard à Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des colis
- **Master Butler** : Permissions

**Contrat d'équipe :**
- Consommé par : StockPicking, StockMove, StockQuant
- Expose : `package.create`, `package.get`, `package.put_in_pack`

### 2.8 StockRule

**Rôle :** Gestion des règles d'approvisionnement (pull, push, MTO, MTS).

**Capacités :**
- Création/modification de règles
- Déclenchement automatique d'approvisionnement
- Routes et séquences de règles

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **KindMother** : Persistance des règles
- **Master Butler** : Permissions de configuration
- **StrongFather** : Décision d'approvisionnement automatique (si politique)

**Contrat d'équipe :**
- Consommé par : StockMove (création automatique), StockWarehouse
- Expose : `rule.evaluate`, `rule.create_moves`

### 2.9 StockInventory

**Rôle :** Gestion des inventaires physiques et ajustements.

**Capacités :**
- Création de sessions d'inventaire
- Saisie des quantités comptées
- Application des ajustements (WriteIntent vers KindMother)
- Gestion des conflits (is_outdated)

**Niveau de sécurité :** 2 (Sensitive)

**Gouvernance :**
- **StrongFather** : Décision d'ajustement (si seuils dépassés)
- **KindMother** : Persistance des ajustements (WriteIntent)
- **Master Butler** : Permissions inventaire
- **WorrySentinel** : Vérification cohérence avant application

**Contrat d'équipe :**
- Consomme : StockQuant (lecture/ajustement)
- Expose : `inventory.create`, `inventory.count`, `inventory.apply`, `inventory.clear`

**Mandat de Permission requis :**
- Application ajustement : Mandat avec StockQuant + KindMother (WriteIntent)

### 2.10 StockUI

**Rôle :** Interface utilisateur pour les opérations d'inventaire.

**Capacités :**
- Affichage des transferts, mouvements, quants
- Formulaires de saisie (réception, livraison, inventaire)
- Rapports et tableaux de bord
- Intégration scan code-barres

**Niveau de sécurité :** 1 (Standard pour UI, données selon Opérateurs)

**Gouvernance :**
- **Master Butler** : Permissions d'accès UI
- **WorrySentinel** : Niveau sécurité selon données affichées

**Contrat d'équipe :**
- Consomme : Tous les autres Opérateurs InventoryService
- Expose : Interface web/mobile

---

## 3. Contrat d'Équipe InventoryService

### 3.1 Définition

**Contrat d'Équipe :** InventoryService

**Opérateurs membres :**
- StockPicking, StockMove, StockQuant, StockLocation, StockWarehouse
- StockLot, StockPackage, StockRule, StockInventory, StockUI

### 3.2 Flux Autorisés

**Flux de données :**

```
StockUI → StockPicking → StockMove → StockQuant (réservation)
StockUI → StockPicking → KindMother (WriteIntent validation)
StockUI → StockInventory → StockQuant → KindMother (WriteIntent ajustement)
StockUI → StockLocation / StockWarehouse → KindMother (lecture/configuration)
StockRule → StockMove (création automatique) → KindMother (WriteIntent)
```

**Règles :**
- StockUI ne communique jamais directement avec KindMother (toujours via Opérateurs)
- Toute modification de quantité (validation move, ajustement inventaire) = WriteIntent vers KindMother
- StockRule peut créer des moves sous Mandat de Permission (approvisionnement automatique)

### 3.3 Types d'Échanges

**Types de données :**
- Transferts (StockPicking)
- Mouvements (StockMove)
- Quantités (StockQuant)
- Emplacements (StockLocation)
- Entrepôts (StockWarehouse)
- Lots/SN (StockLot)
- Colis (StockPackage)
- Règles (StockRule)
- Sessions d'inventaire (StockInventory)

**Niveau de validation requis :**
- Création transfert : Validation Master Butler (permissions)
- Validation transfert : KindMother (WriteIntent) + mise à jour quants
- Ajustement inventaire : Mandat avec StockInventory + KindMother (WriteIntent)
- Configuration emplacements/entrepôts : Master Butler + KindMother (WriteIntent)

### 3.4 Conditions Préalables

**Avant activation de l'Équipe :**
1. Au moins un entrepôt configuré (StockWarehouse)
2. Emplacements créés (StockLocation)
3. Mandats de Permission émis (StrongFather)
4. Niveaux de sécurité définis (WorrySentinel)
5. Intégration MiyuStore (produits) pour quantités et traçabilité

---

## 4. Mandats de Permission

### 4.1 Mandat Standard : InventoryService Standard

**Émis par :** StrongFather

**Opérateurs autorisés :**
- StockUI (lecture/écriture)
- StockPicking (création, confirmation, validation)
- StockMove (réservation, validation)
- StockQuant (lecture)
- StockLocation (lecture)
- StockWarehouse (lecture)
- StockLot (création, lecture)
- StockPackage (création, lecture)
- StockInventory (création, comptage, application)

**Flux autorisés :**
- StockUI → StockPicking → KindMother (WriteIntent)
- StockUI → StockInventory → StockQuant → KindMother (WriteIntent ajustement)

**Types de données :**
- Transferts, mouvements, quants, emplacements, lots, colis, inventaires

**Niveau de sécurité maximum :** 2 (Sensitive)

**Conditions de validité :**
- Utilisateur authentifié
- Environnement en état T0 (Normal) ou T1 (Instable)
- Permissions Master Butler accordées

**Révocation :**
- Fin de session utilisateur
- Changement d'état système (T2-T4)
- Violation de règle WorrySentinel

### 4.2 Mandat Configuration : InventoryService Configuration

**Émis par :** StrongFather

**Opérateurs autorisés :**
- StockLocation (modification emplacements)
- StockWarehouse (modification entrepôts)
- StockRule (modification règles)

**Flux autorisés :**
- StockLocation → KindMother (WriteIntent)
- StockWarehouse → KindMother (WriteIntent)
- StockRule → KindMother (WriteIntent)

**Niveau de sécurité maximum :** 2 (Sensitive)

**Conditions de validité :**
- Utilisateur avec rôle gestionnaire stock / logistique
- Environnement en état T0
- Validation WorrySentinel (emplacements/entrepôts non utilisés si modification structure)

**Révocation :**
- Fin de session
- Changement d'état système

### 4.3 Mandat Approvisionnement Automatique

**Émis par :** StrongFather

**Opérateurs autorisés :**
- StockRule (création moves automatique)

**Flux autorisés :**
- StockRule → StockMove (création) → KindMother (WriteIntent)

**Niveau de sécurité maximum :** 2 (Sensitive)

**Conditions de validité :**
- Règles d'approvisionnement activées
- Mandat limité dans le temps ou par nombre d'opérations

**Révocation :**
- Fin de validité
- Désactivation des règles

---

## 5. Niveaux de Sécurité

### 5.1 Classification des Données

| Type de donnée | Niveau | Justification |
|----------------|--------|---------------|
| Quantités en stock | 2 (Sensitive) | Données métier sensibles |
| Transferts et mouvements | 2 (Sensitive) | Traçabilité et valorisation |
| Emplacements / Entrepôts | 2 (Sensitive) | Structure organisationnelle |
| Lots / Numéros de série | 2 (Sensitive) | Traçabilité réglementaire |
| Colis | 1-2 (Standard à Sensitive) | Selon contenu |
| Règles d'approvisionnement | 2 (Sensitive) | Configuration critique |
| Interface utilisateur | 1 (Standard) | UI seule, données selon Opérateurs |

### 5.2 Mesures de Protection

**Niveau 1 (Standard) :**
- Contrôle d'accès (Mandat, Master Butler)
- Traçabilité des accès

**Niveau 2 (Sensitive) :**
- Résidence centralisée (KindMother)
- Chiffrement en transit
- Audit des lectures/écritures
- Mandats de Permission requis pour modifications

---

## 6. Intégration avec les Cores

### 6.1 StrongFather

**Rôle :** Décision stratégique

**Interventions :**
- Émission de Mandats de Permission pour InventoryService
- Décision d'ajustement inventaire (si seuils ou politique)
- Révocation de Mandats si nécessaire

**Règles :**
- StrongFather ne modifie jamais les données (KindMother)
- StrongFather ne persiste jamais (KindMother)

### 6.2 KindMother

**Rôle :** Autorité absolue des données

**Responsabilités :**
- Persistance des transferts, mouvements, quants (WriteIntent)
- Persistance des emplacements, entrepôts, lots, colis, règles
- Cohérence des quantités (réservation, disponibilité)

**Règles :**
- Toute modification de stock = WriteIntent vers KindMother
- KindMother valide la cohérence avant persistance

### 6.3 Master Butler

**Rôle :** Registre des capacités et permissions

**Responsabilités :**
- Déclaration des Opérateurs InventoryService
- Déclaration des capacités (picking.create, move.assign, inventory.apply, etc.)
- Gestion des permissions utilisateur
- Validation des Mandats de Permission

### 6.4 WorrySentinel

**Rôle :** Gouvernance de sécurité

**Responsabilités :**
- Niveau de sécurité des données stock
- Vérification état système avant opérations critiques (inventaire, ajustements)
- Blocage si état dégradé (T2-T4)

### 6.5 Ever Buddy

**Rôle :** Cycle de vie et compatibilité

**Responsabilités :**
- Versions des structures (emplacements, entrepôts, routes)
- Dépréciation des règles ou types d'opération
- Compatibilité des données lors des évolutions

---

## 7. Intégrations avec Autres Services Miyukini

### 7.1 MiyuStore (Product)

**Flux :**
- Produits et variantes : lecture depuis MiyuStore
- Quantités disponibles : calcul depuis StockQuant, exposé sur produit (qty_available, virtual_available)
- Traçabilité (lots/SN) : lien produit ↔ StockLot

**Contrat :** InventoryService consomme MiyuStore (lecture). MiyuStore peut étendre produit avec champs quantités (calculés via InventoryService).

### 7.2 Miyukini Sales

**Flux :**
- Commande client confirmée → création StockPicking (livraison) via BondingBrother
- Validation livraison → mise à jour qty_delivered sur ligne commande

**Contrat :** Mandat de Permission entre Sales et InventoryService pour création/validation transferts.

### 7.3 MiyuPurchase (si développé)

**Flux :**
- Commande fournisseur confirmée → création StockPicking (réception)
- Validation réception → mise à jour qty_received sur ligne commande

**Contrat :** Mandat de Permission entre Purchase et InventoryService.

### 7.4 MiyuInvoice / MiyuAccounting (si développé)

**Flux :**
- Validation StockMove → écritures comptables (valorisation stock)

**Contrat :** WriteIntent ou événements vers Accounting pour écritures de stock.

### 7.5 MiyuContacts (Partners)

**Flux :**
- Emplacements par défaut (client, fournisseur) sur partenaire
- Partenaire sur StockPicking (livraison/réception)

**Contrat :** Lecture partenaire depuis MiyuContacts.

---

## 8. Conclusion

Les spécifications Opérateurs Miyukini pour Inventory couvrent :

- **10 Opérateurs** : StockPicking, StockMove, StockQuant, StockLocation, StockWarehouse, StockLot, StockPackage, StockRule, StockInventory, StockUI
- **Équipe InventoryService** avec Contrat d'Équipe et flux autorisés
- **Mandats de Permission** : Standard, Configuration, Approvisionnement automatique
- **Niveaux de sécurité** : 1 (UI) à 2 (Sensitive) pour données stock
- **Intégration Cores** : StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy
- **Intégration services** : MiyuStore, Miyukini Sales, MiyuPurchase, MiyuInvoice, MiyuContacts

**Correspondance Miyukini :** `MiyukiniInventory` ou `MiyuInventory` (service). Crate existant : `miyuposinventory` (POS Inventory) — à étendre ou à distinguer d’un module Inventory générique.

---

**Date de création :** 2026-02-01  
**Version :** 1.0  
**Statut :** Document d'analyse complète
