# Odoo Purchase — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Purchase** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

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

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances explicites (`__manifest__.py`) :**
- `account` : Comptabilité (facturation fournisseur)

### 1.2 Modules Optionnels

**Dépendances optionnelles :**
- `stock` : Intégration stock (réceptions, picking)
- `product` : Produits et catalogues (déjà dépendance de account)
- `portal` : Portail fournisseur (consultation, reconnaissance)
- `mail` : Messagerie et activités (déjà dépendance de account)
- `analytic` : Comptabilité analytique (déjà dépendance de account)

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Accounting

**Flux :**
```
Purchase Order → Vendor Bill (account.move)
```

**Mécanismes :**
- Génération factures fournisseur depuis commandes : `action_create_invoice()`
- Lien bidirectionnel : `invoice_ids` ↔ `purchase_line_ids` sur `account.move.line`
- Synchronisation montants : `qty_invoiced`, `qty_to_invoice` sur `purchase.order.line`
- Statut facturation : `invoice_status` (no, to invoice, invoiced)

**Champs liés :**
- `purchase_line_id` : Ligne commande liée (sur `account.move.line`)
- `invoice_origin` : Origine facture (référence commande)
- `invoice_payment_term_id` : Conditions de paiement (depuis commande)
- `fiscal_position_id` : Position fiscale (depuis commande)
- `partner_bank_id` : Banque partenaire (depuis commande)

**Hooks utilisés :**
- `purchase.order._prepare_invoice()` : Préparation facture depuis commande
- `purchase.order.line._prepare_account_move_line()` : Préparation lignes facture
- `account.move._get_invoice_reference()` : Référence facture depuis commande

**Bill Matching :**
- Modèle `purchase.bill.line.match` : Matching factures avec commandes
- Vue liste factures non matchées
- Matching automatique ou manuel par référence/montant/lignes

**Recommandations pour Miyukini :**
- Intégration native avec Miyukini Accounting
- Lien bidirectionnel commande ↔ facture
- Synchronisation automatique des montants
- Génération factures depuis commandes confirmées
- Outil de matching factures intelligent

### 2.2 Intégration avec Inventory (si installé)

**Flux :**
```
Purchase Order → Stock Picking (Réception) → Stock Quant
```

**Mécanismes :**
- Génération picking réception lors de confirmation commande
- Lien : `picking_id` sur `purchase.order.line` (via `stock.move`)
- Calcul quantités reçues : `qty_received` depuis picking
- `qty_received_method` : 'manual' pour produits consu/service, sinon depuis stock moves

**Champs liés :**
- `qty_received` : Quantité reçue (depuis picking si méthode != 'manual')
- `qty_received_method` : Méthode calcul (manual ou depuis stock moves)
- `date_planned` : Date de réception prévue (utilisée pour picking)

**Recommandations pour Miyukini :**
- Intégration avec module Inventory (si développé)
- Lien réception ↔ commande
- Calcul automatique quantités reçues
- Synchronisation dates prévues

### 2.3 Intégration avec Product

**Flux :**
```
Product → Purchase Order Line → Supplier Info Update
```

**Mécanismes :**
- Sélection produits depuis catalogue (`purchase_ok=True`)
- Calcul prix depuis `product.supplierinfo` (seller)
- Ajout fournisseur aux produits (`_add_supplier_to_product`)
- Gestion variantes et attributs produits

**Champs liés :**
- `product_id` : Produit sélectionné
- `selected_seller_id` : Seller sélectionné depuis `product.supplierinfo`
- `product_uom_id` : Unité de mesure
- `product.supplierinfo` : Informations fournisseur produits

**Recommandations pour Miyukini :**
- Intégration native avec MiyuStore
- Calcul prix automatique depuis seller
- Ajout fournisseur aux produits automatique
- Support variantes et attributs

### 2.4 Intégration avec Portal

**Flux :**
```
Purchase Order → Portal → Vendor View
```

**Mécanismes :**
- Accès fournisseur aux commandes via portail
- Reconnaissance (acknowledge) depuis portail
- Mise à jour dates prévues depuis portail

**Templates :**
- `portal_templates.xml` : Templates portail pour commandes

**Recommandations pour Miyukini :**
- Portail fournisseur pour consultation commandes
- Reconnaissance facile
- Mise à jour dates prévues
- Notifications temps réel

---

## 3. Flux de Données Inter-Apps

### 3.1 Flux Principal Achats

```
Product (product)
    ↓
Purchase Order (purchase)
    ↓
Stock Picking (si stock)
    ↓
Vendor Bill (account.move)
    ↓
Account Payment (account.payment)
```

### 3.2 Flux Données Partagées

**Données partagées :**
- **Partner** : Fournisseur partagé entre Purchase, Accounting, Inventory
- **Product** : Produits partagés entre Purchase, Inventory, Accounting
- **Currency** : Devises partagées entre toutes les apps
- **Company** : Entreprises partagées entre toutes les apps
- **Invoice** : Factures partagées entre Purchase et Accounting

---

## 4. Mécanismes d'Intégration

### 4.1 Hooks et Overrides

**Hooks utilisés :**
- `purchase.order._prepare_invoice()` : Préparation facture depuis commande
- `purchase.order.line._prepare_account_move_line()` : Préparation lignes facture
- `purchase.order._add_supplier_to_product()` : Ajout fournisseur aux produits
- `account.move._get_invoice_reference()` : Référence facture depuis commande

**Overrides :**
- `account.move` : Lien vers `purchase_line_ids`
- `account.move.line` : Lien vers `purchase_line_id`
- `product.product` : Ajout seller lors confirmation commande
- `res.partner` : Champs comptables (supplier_invoice_count)

### 4.2 Événements et Signaux

**Événements :**
- Confirmation commande → Génération picking (si Inventory)
- Confirmation commande → Ajout fournisseur aux produits
- Génération facture → Mise à jour statut facturation
- Réception produits → Mise à jour quantités reçues

**Signaux :**
- `onchange_partner_id` : Mise à jour devise, conditions paiement, position fiscale
- `onchange_product_id` : Mise à jour prix, UoM, taxes, date prévue

### 4.3 APIs et Méthodes Publiques

**Méthodes principales :**
- `purchase.order.action_create_invoice()` : Création factures depuis commandes
- `purchase.order.button_confirm()` : Confirmation commande
- `purchase.order.button_approve()` : Approbation commande
- `purchase.order.action_bill_matching()` : Matching factures
- `purchase.order.action_merge()` : Fusion RFQ

**APIs externes :**
- Pas d'API REST publique dans Purchase core
- APIs via `portal` pour consultation fournisseur

---

## 5. Recommandations pour Miyukini

### 5.1 Intégrations Prioritaires

**Intégrations natives :**
1. **Miyukini Accounting** : Génération factures fournisseur depuis commandes
2. **MiyuStore** : Produits et sellers
3. **MiyuContacts** : Fournisseurs
4. **MiyuInventory** : Réceptions (si développé)
5. **MiyuPortal** : Portail fournisseur (si nécessaire)

### 5.2 Patterns d'Intégration

**Actions :**
- Lien bidirectionnel commande ↔ facture
- Synchronisation automatique des montants
- Génération automatique depuis commandes confirmées
- Matching automatique factures

**Gouvernance COG :**
- StrongFather : Autorisation génération facture, approbation
- KindMother : Persistance via WriteIntent
- Master Butler : Permissions achats
- WorrySentinel : Sécurité données financières

### 5.3 Architecture d'Intégration

**Opérateurs proposés :**
1. **PurchaseOrderOperator** : Gestion commandes d'achat/RFQ
2. **PurchaseOrderLineOperator** : Gestion lignes de commande
3. **PurchaseApprovalOperator** : Gestion approbations
4. **PurchaseInvoiceOperator** : Génération factures depuis commandes
5. **PurchaseReceptionOperator** : Gestion réceptions (si Inventory)
6. **PurchaseUI** : Interface utilisateur

**Intégrations via BondingBrother :**
- Traduction intentions depuis Accounting/Inventory
- Traduction réponses vers sources
- Médiation sans autorité

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
