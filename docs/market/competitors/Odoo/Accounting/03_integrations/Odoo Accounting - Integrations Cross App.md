# Odoo Accounting — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application **Accounting** d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

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
- `base` : Fonctionnalités de base (partners, companies, currencies)
- `mail` : Messagerie et activités
- `portal` : Portail client (consultation factures)
- `web` : Framework web
- `utm` : Tracking marketing (campaigns, medium, source)

### 1.2 Modules Optionnels

**Dépendances optionnelles :**
- `sale` : Intégration ventes (factures depuis commandes)
- `purchase` : Intégration achats (factures depuis commandes fournisseur)
- `stock` : Intégration stock (factures depuis livraisons)
- `project` : Intégration projets (timesheet, facturation)
- `expense` : Intégration notes de frais
- `crm` : Intégration CRM (factures depuis opportunities)
- `website` : Portail web public
- `l10n_*` : Localisations comptables (plans comptables par pays)

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Sales

**Flux :**
```
Sales Order → Invoice (account.move)
```

**Mécanismes :**
- Génération factures depuis commandes : `sale.order._create_invoices()`
- Lien bidirectionnel : `invoice_ids` sur `sale.order` ↔ `sale_line_ids` sur `account.move.line`
- Synchronisation montants : `amount_invoiced`, `amount_to_invoice` sur `sale.order`
- Statut facturation : `invoice_status` (invoiced, to invoice, upselling, no)

**Champs liés :**
- `sale_line_ids` : Lignes de commande liées (Many2many sur `account.move.line`)
- `invoice_origin` : Origine facture (référence commande)
- `invoice_payment_term_id` : Conditions de paiement (depuis commande)
- `invoice_partner_display_name` : Nom partenaire (depuis commande)

**Hooks utilisés :**
- `sale.order._prepare_invoice()` : Préparation facture depuis commande
- `sale.order._prepare_invoice_line()` : Préparation lignes facture
- `account.move._get_invoice_reference()` : Référence facture depuis commande

**Recommandations pour Miyukini :**
- Intégration native avec Miyukini Sales
- Lien bidirectionnel commande ↔ facture
- Synchronisation automatique des montants
- Génération factures depuis commandes confirmées

### 2.2 Intégration avec Purchase

**Flux :**
```
Purchase Order → Vendor Bill (account.move)
```

**Mécanismes :**
- Génération factures fournisseur depuis commandes : `purchase.order._create_invoices()`
- Lien bidirectionnel : `invoice_ids` sur `purchase.order` ↔ `purchase_line_ids` sur `account.move.line`
- Synchronisation montants : `amount_invoiced`, `amount_to_invoice` sur `purchase.order`
- Statut facturation : `invoice_status` (invoiced, to invoice, no)

**Champs liés :**
- `purchase_line_ids` : Lignes de commande fournisseur liées (Many2many sur `account.move.line`)
- `invoice_origin` : Origine facture (référence commande fournisseur)
- `invoice_payment_term_id` : Conditions de paiement (depuis commande)
- `invoice_partner_display_name` : Nom fournisseur (depuis commande)

**Hooks utilisés :**
- `purchase.order._prepare_invoice()` : Préparation facture fournisseur
- `purchase.order._prepare_invoice_line()` : Préparation lignes facture
- `account.move._get_invoice_reference()` : Référence facture depuis commande

**Recommandations pour Miyukini :**
- Intégration native avec module Purchase (si développé)
- Lien bidirectionnel commande fournisseur ↔ facture
- Synchronisation automatique des montants

### 2.3 Intégration avec Stock

**Flux :**
```
Stock Picking → Invoice (account.move)
```

**Mécanismes :**
- Génération factures depuis livraisons (si `sale_stock` installé)
- Lien : `stock_picking_id` sur `account.move.line`
- Calcul quantités livrées : `qty_delivered` depuis picking

**Champs liés :**
- `stock_picking_id` : Livraison liée
- `qty_delivered` : Quantité livrée (depuis picking)

**Recommandations pour Miyukini :**
- Intégration avec module Stock (si développé)
- Lien livraison ↔ facture
- Calcul automatique quantités livrées

### 2.4 Intégration avec Project

**Flux :**
```
Project Task → Timesheet → Invoice Line (account.move.line)
```

**Mécanismes :**
- Facturation timesheet depuis tâches projet
- Lien : `project_id`, `task_id` sur `account.move.line`
- Calcul montants depuis heures travaillées

**Champs liés :**
- `project_id` : Projet lié
- `task_id` : Tâche liée
- `timesheet_ids` : Timesheets liés

**Recommandations pour Miyukini :**
- Intégration avec module Project (si développé)
- Facturation timesheet depuis projets
- Lien projet/tâche ↔ facture

### 2.5 Intégration avec Expense

**Flux :**
```
Expense → Expense Report → Invoice (account.move)
```

**Mécanismes :**
- Génération factures depuis notes de frais
- Lien : `expense_sheet_id` sur `account.move`
- Comptabilisation automatique des frais

**Champs liés :**
- `expense_sheet_id` : Note de frais liée
- `expense_line_ids` : Lignes de frais liées

**Recommandations pour Miyukini :**
- Intégration avec module Expense (si développé)
- Génération factures depuis notes de frais
- Comptabilisation automatique

### 2.6 Intégration avec CRM

**Flux :**
```
CRM Opportunity → Sales Order → Invoice (account.move)
```

**Mécanismes :**
- Factures générées indirectement via Sales depuis CRM
- Tracking marketing : `campaign_id`, `medium_id`, `source_id` sur factures
- Lien équipe commerciale : `team_id`, `user_id`

**Champs liés :**
- `campaign_id` : Campagne marketing
- `medium_id` : Support marketing
- `source_id` : Source marketing
- `team_id` : Équipe commerciale (depuis CRM)
- `user_id` : Commercial (depuis CRM)

**Recommandations pour Miyukini :**
- Intégration indirecte via Miyukini Sales
- Tracking marketing intégré
- Lien équipe commerciale

### 2.7 Intégration avec Portal

**Flux :**
```
Invoice → Portal → Customer View
```

**Mécanismes :**
- Accès client aux factures via portail
- Consultation historique
- Téléchargement PDF
- Paiement en ligne (si intégré)

**Templates :**
- `account_portal_templates.xml` : Templates portail pour factures

**Recommandations pour Miyukini :**
- Portail client pour consultation factures
- Téléchargement PDF
- Historique accessible

---

## 3. Flux de Données Inter-Apps

### 3.1 Flux Principal Ventes

```
CRM Opportunity (crm)
    ↓
Sales Order (sale)
    ↓
Stock Picking (si stock)
    ↓
Account Invoice (account.move)
    ↓
Account Payment (account.payment)
```

### 3.2 Flux Principal Achats

```
Purchase Order (purchase)
    ↓
Stock Picking (si stock)
    ↓
Vendor Bill (account.move)
    ↓
Account Payment (account.payment)
```

### 3.3 Flux Projet

```
Project Task (project)
    ↓
Timesheet (project.task)
    ↓
Invoice Line (account.move.line)
    ↓
Account Invoice (account.move)
```

### 3.4 Flux Notes de Frais

```
Expense (expense)
    ↓
Expense Report (expense.sheet)
    ↓
Account Invoice (account.move)
```

### 3.5 Flux Données Partagées

**Données partagées :**
- **Partner** : Client/Fournisseur partagé entre Sales, Purchase, Accounting, CRM
- **Product** : Produits partagés entre Sales, Purchase, Stock, Accounting
- **Team** : Équipe commerciale partagée entre CRM, Sales, Accounting
- **Currency** : Devises partagées entre toutes les apps
- **Company** : Entreprises partagées entre toutes les apps
- **Journal** : Journaux comptables utilisés par Accounting uniquement
- **Account** : Comptes comptables utilisés par Accounting uniquement

---

## 4. Mécanismes d'Intégration

### 4.1 Hooks et Overrides

**Hooks utilisés :**
- `_prepare_invoice()` : Préparation facture depuis document source
- `_prepare_invoice_line()` : Préparation ligne facture depuis ligne source
- `_get_invoice_reference()` : Référence facture depuis document source
- `_action_post()` : Action après validation facture
- `_get_invoiceable_lines()` : Lignes facturables depuis document source

**Overrides :**
- `account.move` : Lien vers `sale_line_ids`, `purchase_line_ids`
- `account.move.line` : Lien vers `sale_line_id`, `purchase_line_id`, `stock_picking_id`
- `res.partner` : Champs comptables (credit, debit, balance)
- `product.product` : Comptes comptables par défaut

### 4.2 Événements et Signaux

**Événements :**
- Validation facture → Mise à jour statut commande (si Sales/Purchase)
- Paiement facture → Mise à jour statut paiement
- Réconciliation → Mise à jour statut facture
- Annulation facture → Mise à jour statut commande

**Signaux :**
- `onchange_partner_id` : Mise à jour comptes par défaut
- `onchange_product_id` : Mise à jour compte produit
- `onchange_invoice_date` : Mise à jour taux de change

### 4.3 APIs et Méthodes Publiques

**Méthodes principales :**
- `account.move._create_invoices()` : Création factures depuis document source
- `account.move._post()` : Validation facture
- `account.move._reverse()` : Contre-passation
- `account.move.line._reconcile()` : Réconciliation lignes
- `account.payment._register_payment()` : Enregistrement paiement

**APIs externes :**
- Pas d'API REST publique dans Accounting core
- APIs via `account_payment` pour paiements en ligne
- APIs via `portal` pour consultation client

---

## 5. Intégrations avec Services Externes

### 5.1 Paiements en Ligne

**Intégration :**
- `account_payment` : Module paiements en ligne
- Support Stripe, PayPal, Adyen, etc.
- Transactions : `payment.transaction` liées à `account.move`

**Flux :**
```
Invoice → Payment Transaction → Payment Provider → Confirmation
```

### 5.2 EDI (Electronic Data Interchange)

**Intégration :**
- Modules `l10n_*_edi` : EDI par pays
- Génération fichiers EDI depuis factures
- Envoi automatique aux autorités fiscales

**Exemples :**
- `l10n_fr_edi` : Factur-X (France)
- `l10n_es_edi` : Facturae (Espagne)
- `l10n_it_edi` : FatturaPA (Italie)

### 5.3 Banques

**Intégration :**
- `account_bank_statement_import` : Import relevés bancaires
- `account_bank_statement_import_ofx` : Format OFX
- `account_bank_statement_import_csv` : Format CSV
- Réconciliation automatique avec écritures

**Flux :**
```
Bank Statement → Import → Reconciliation → Account Move Line
```

---

## 6. Recommandations pour Miyukini

### 6.1 Intégrations Prioritaires

**Intégrations natives :**
1. **Miyukini Sales** : Génération factures depuis commandes
2. **MiyuInvoice** : Utilisation outils facturation (si séparé)
3. **MiyuContacts** : Clients et fournisseurs
4. **MiyuStore** : Produits et comptes par défaut
5. **MiyuPayment** : Paiements en ligne (si nécessaire)

### 6.2 Patterns d'Intégration

**Actions :**
- Lien bidirectionnel document source ↔ facture
- Synchronisation automatique des montants
- Génération automatique depuis documents sources
- Réconciliation automatique avec paiements

**Gouvernance COG :**
- StrongFather : Autorisation génération facture
- KindMother : Persistance via WriteIntent
- Master Butler : Permissions facturation
- WorrySentinel : Sécurité données financières

### 6.3 Architecture d'Intégration

**Opérateurs proposés :**
1. **AccountMoveOperator** : Gestion écritures comptables
2. **AccountMoveLineOperator** : Gestion lignes d'écriture
3. **AccountReconcileOperator** : Réconciliation
4. **AccountInvoiceOperator** : Génération factures depuis sources
5. **AccountPaymentOperator** : Gestion paiements
6. **AccountReportOperator** : Génération rapports
7. **AccountUI** : Interface utilisateur

**Intégrations via BondingBrother :**
- Traduction intentions depuis Sales/Purchase
- Traduction réponses vers sources
- Médiation sans autorité

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
