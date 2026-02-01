# Odoo Inventory — Index Documentation

## Contexte

Ce document sert d'**index central** pour l'analyse complète de l'application **Inventory** (Stock) d'Odoo, extraite du code source GitHub.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/stock`

**Date d'analyse :** 2026-02-01

---

## Structure de Documentation

Cette analyse complète de l'application Inventory est organisée en **7 documents** couvrant tous les aspects nécessaires pour comprendre et implémenter un équivalent dans l'écosystème Miyukini.

### Documents disponibles

#### 1. Logique Métier Complète
**Fichier :** `00_logique_metier/Odoo Inventory - Logique Metier Complete.md`

**Contenu :**
- Modèles de données principaux (StockPicking, StockMove, StockMoveLine, StockQuant, StockLocation, StockWarehouse)
- Règles métier et contraintes
- Workflows et transitions d'état (Draft → Confirmed → Assigned → Done)
- Gestion des réservations et stratégies de retrait (FIFO, LIFO, FEFO)
- Gestion des lots et numéros de série
- Règles d'approvisionnement et inventaire physique
- Backorders, retours, cross-dock

**Statut :** Complété

---

#### 2. Parcours Utilisateur Détaillés
**Fichier :** `01_parcours_utilisateur/Odoo Inventory - Parcours Utilisateur Detailles.md`

**Contenu :**
- Personas et rôles (Préparateur, Réceptionniste, Gestionnaire de Stock, Responsable Logistique)
- Parcours d'onboarding
- Scénarios d'usage (réception, préparation commande, transfert interne, inventaire physique, backorders, retours)
- Points de friction identifiés
- Recommandations pour Miyukini

**Statut :** Complété

---

#### 3. Analyse UI/UX
**Fichier :** `02_ui_ux/Odoo Inventory - Analyse UI UX.md`

**Contenu :**
- Vues principales (Liste, Kanban, Formulaire, Calendrier, Inventaire éditable, Pivot, Graphique)
- Widgets spécialisés (stock_move_one2many, forecast_widget, counted_quantity_widget, package_m2o, stock_rescheduling_popover)
- Patterns de navigation et filtres
- Responsive et mobile
- Accessibilité (hotkeys)
- Recommandations pour Miyukini

**Statut :** Complété

---

#### 4. Intégrations Cross-App
**Fichier :** `03_integrations/Odoo Inventory - Integrations Cross App.md`

**Contenu :**
- Dépendances (Product, Sales, Purchase, Accounting, MRP, Partners, Company, Users)
- Flux de données inter-apps
- Mécanismes d'intégration (hooks, overrides, événements)
- APIs et méthodes publiques
- Recommandations pour Miyukini

**Statut :** Complété

---

#### 5. Spécifications Opérateurs Miyukini
**Fichier :** `04_specifications_miyukini/Odoo Inventory - Specifications Operateurs Miyukini.md`

**Contenu :**
- Opérateurs identifiés (StockPicking, StockMove, StockQuant, StockLocation, StockWarehouse, StockLot, StockPackage, StockRule, StockInventory, StockUI)
- Contrat d'équipe InventoryService
- Mandats de Permission (Standard, Configuration, Approvisionnement automatique)
- Niveaux de sécurité
- Intégration avec les Cores (StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy)
- Intégrations avec autres services Miyukini (MiyuStore, Miyukini Sales, MiyuPurchase, MiyuInvoice, MiyuContacts)

**Statut :** Complété

---

#### 6. Guide Intégration COG
**Fichier :** `05_integration_cog/Odoo Inventory - Guide Integration COG.md`

**Contenu :**
- Architecture d'intégration COG
- Patterns d'implémentation (validation transfert, réservation, ajustement inventaire, création depuis commande)
- Exemples de code (pseudo-code Rust)
- Gestion des WriteIntent et Mandats
- Gestion des erreurs et rollback
- Intégration avec MiyuStore et miyuposinventory

**Statut :** Complété

---

#### 7. Guide Implémentation
**Fichier :** `06_guides_implementation/Odoo Inventory - Guide Implementation.md`

**Contenu :**
- Architecture technique détaillée (crates Rust)
- Schémas de données (Picking, Move, MoveLine, Quant, Location, Warehouse)
- API et contrats
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel
- Correspondance avec crate existant miyuposinventory

**Statut :** Complété

---

## Statut Global

**Analyse complète à 100% (7/7 documents)**

Tous les documents de l'analyse Inventory sont complétés et prêts pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

---

## Correspondance Miyukini

**Service Miyukini proposé :** `MiyukiniInventory` ou `MiyuInventory`

**Opérateurs proposés :**
- StockPickingOperator
- StockMoveOperator
- StockQuantOperator
- StockLocationOperator
- StockWarehouseOperator
- StockLotOperator
- StockPackageOperator
- StockRuleOperator
- StockInventoryOperator
- StockUI

**Équipe d'Opérateurs :** InventoryService

**Crate existant :** `miyuposinventory` (POS Inventory) — peut être étendu ou coexistir avec un module Inventory générique.

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
