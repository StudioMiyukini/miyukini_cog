# Odoo Sales — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application Sales d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

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
- `account` : Comptabilité (facturation)
- `crm` : CRM (équipes commerciales, opportunities)
- `product` : Produits et catalogues
- `portal` : Portail client (signature, paiement)
- `payment` : Paiements en ligne
- `utm` : Tracking marketing (campaigns, medium, source)
- `mail` : Messagerie et activités
- `analytic` : Comptabilité analytique

### 1.2 Modules Optionnels

**Dépendances optionnelles :**
- `sale_stock` : Intégration stock (livraisons, picking)
- `sale_timesheet` : Intégration timesheet (quantités livrées)
- `sale_expense` : Intégration notes de frais
- `website_sale` : Boutique en ligne
- `sale_margin` : Marges commerciales

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Accounting

**Flux :**
```
Sales Order → Invoice (account.move)
```

**Mécanismes :**
- Génération factures depuis commandes : `_create_invoices()`
- Lien bidirectionnel : `invoice_ids` ↔ `sale_line_ids`
- Synchronisation montants : `amount_invoiced`, `amount_to_invoice`
- Statut facturation : `invoice_status` (invoiced, to invoice, upselling, no)

**Champs liés :**
- `journal_id` : Journal de facturation
- `fiscal_position_id` : Position fiscale
- `payment_term_id` : Conditions de paiement
- `preferred_payment_method_line_id` : Méthode de paiement

**Recommandations pour Miyukini :**
- Intégration native avec MiyuInvoice
- Lien bidirectionnel commande ↔ facture
- Synchronisation automatique des montants

### 2.2 Intégration avec CRM

**Flux :**
```
CRM Opportunity → Sales Quotation → Sales Order
```

**Mécanismes :**
- Conversion Opportunity → Quotation (wizard `crm.lead2opportunity.partner`)
- Lien : `opportunity_id` sur `sale.order`
- Synchronisation équipe : `team_id`, `user_id` depuis CRM
- Tags CRM : `tag_ids` (Many2many vers `crm.tag`)

**Champs liés :**
- `team_id` : Équipe commerciale (depuis CRM)
- `user_id` : Commercial (depuis CRM ou Opportunity)
- `campaign_id`, `medium_id`, `source_id` : Tracking marketing (UTM)

**Recommandations pour Miyukini :**
- Conversion native Opportunity → Quotation
- Synchronisation équipe commerciale
- Tracking marketing intégré

### 2.3 Intégration avec Product

**Flux :**
```
Product → Sale Order Line
```

**Mécanismes :**
- Sélection produits depuis catalogue
- Calcul prix depuis pricelist
- Gestion variantes et attributs produits
- Support produits configurables (combo, options)

**Champs liés :**
- `product_id` : Produit sélectionné
- `product_template_id` : Template produit
- `pricelist_id` : Liste de prix
- `product_uom_id` : Unité de mesure

**Recommandations pour Miyukini :**
- Intégration native avec MiyuStore
- Calcul prix automatique depuis pricelist
- Support variantes et attributs

### 2.4 Intégration avec Portal

**Flux :**
```
Sales Order → Portal (signature, paiement) → Confirmation
```

**Mécanismes :**
- Accès client au devis/commande via portail
- Signature en ligne : `signature`, `signed_by`, `signed_on`
- Paiement en ligne : `transaction_ids`, `amount_paid`
- Confirmation automatique après signature + paiement

**Champs liés :**
- `require_signature` : Signature requise
- `require_payment` : Paiement requis
- `prepayment_percent` : Pourcentage acompte
- `signature` : Image signature
- `transaction_ids` : Transactions de paiement

**Recommandations pour Miyukini :**
- Portail client pour consultation devis/commandes
- Signature et paiement en ligne
- Confirmation automatique après paiement

### 2.5 Intégration avec Payment

**Flux :**
```
Sales Order → Payment Transaction → Confirmation
```

**Mécanismes :**
- Transactions de paiement liées : `transaction_ids`
- Capture/void des transactions : `payment_action_capture()`, `payment_action_void()`
- Montant payé : `amount_paid` (somme transactions autorisées/terminées)
- Vérification confirmation : `_is_confirmation_amount_reached()`

**Recommandations pour Miyukini :**
- Intégration avec système de paiement
- Gestion transactions (capture, void)
- Suivi montants payés

### 2.6 Intégration avec Stock (sale_stock)

**Flux :**
```
Sales Order → Stock Picking → Delivery
```

**Mécanismes :**
- Génération picking lors de confirmation
- Calcul `qty_delivered` depuis picking
- Calcul `expected_date` depuis délais produits
- Gestion multi-adresses (facturation vs livraison)

**Recommandations pour Miyukini :**
- Intégration avec gestion stock (si nécessaire)
- Calcul quantités livrées depuis picking
- Gestion dates de livraison

### 2.7 Intégration avec Timesheet (sale_timesheet)

**Flux :**
```
Sales Order Line → Timesheet → qty_delivered
```

**Mécanismes :**
- Calcul `qty_delivered` depuis heures timesheet
- Lien avec projets et tâches
- Facturation basée sur heures

**Recommandations pour Miyukini :**
- Intégration avec timesheet (si nécessaire)
- Calcul quantités livrées depuis heures

---

## 3. Flux de Données Inter-Apps

### 3.1 Flux Complet Devis → Facture

```
CRM Opportunity
    ↓
Sales Quotation (draft)
    ↓
Sales Quotation (sent) → Portal (signature + paiement)
    ↓
Sales Order (sale)
    ↓
Stock Picking (si stock)
    ↓
Account Invoice (depuis commande)
    ↓
Account Payment
```

### 3.2 Flux Données Partagées

**Données partagées :**
- **Partner** : Client partagé entre CRM, Sales, Accounting
- **Product** : Produits partagés entre Sales, Stock, Accounting
- **Team** : Équipe commerciale partagée entre CRM et Sales
- **Invoice** : Factures partagées entre Sales et Accounting

---

## 4. Mécanismes d'Intégration

### 4.1 Hooks et Overrides

**Hooks utilisés :**
- `_action_confirm()` : Hook pour génération documents liés (picking, invoice)
- `_prepare_invoice()` : Préparation facture depuis commande
- `_prepare_invoice_line()` : Préparation ligne facture depuis ligne commande

**Overrides :**
- `account.move` : Lien vers `sale_line_ids`
- `crm.lead` : Conversion vers `sale.order`
- `product.product` : Calcul prix depuis pricelist

### 4.2 Événements et Signaux

**Événements :**
- Confirmation commande → Génération picking (si stock)
- Confirmation commande → Génération facture (si auto)
- Paiement → Confirmation automatique (si requis)

---

## 5. Recommandations pour Miyukini

### 5.1 Intégrations Prioritaires

**Intégrations natives :**
1. **Miyukini CRM** : Conversion Opportunity → Quotation
2. **MiyuInvoice** : Génération factures depuis commandes
3. **MiyuStore** : Produits et pricelist
4. **MiyuContacts** : Clients et adresses
5. **MiyuPayment** : Paiements en ligne (si nécessaire)

### 5.2 Patterns d'Intégration

**Actions :**
- Lien bidirectionnel commande ↔ facture
- Synchronisation automatique des montants
- Conversion native Opportunity → Quotation
- Calcul prix automatique depuis pricelist

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
