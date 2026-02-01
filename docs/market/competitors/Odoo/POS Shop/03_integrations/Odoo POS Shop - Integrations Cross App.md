# Odoo POS Shop — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l'application Point of Sale (POS) Shop d'Odoo, identifiant les dépendances, flux de données, mécanismes d'intégration et APIs utilisées.

**Source d'analyse :** Documentation Odoo 18/19, module `point_of_sale`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec autres apps Odoo
- Flux de données inter-apps
- Mécanismes d'intégration (Stock, Accounting, Sales, Product, Contacts)
- Recommandations pour Miyukini

---

## 1. Dépendances Principales

### 1.1 Modules Requis

**Dépendances explicites (typiques du module POS) :**
- `account` : Comptabilité (journal de caisse, factures, écritures)
- `product` : Produits, catalogues, variantes, UOM
- `stock` : Mouvements de stock (sorties à la vente, entrées aux retours)
- `sale` : Commandes Sales (import devis/commandes dans le POS, création de commandes Sales depuis le POS)
- `mail` : Messagerie et activités (optionnel)
- `utm` : Tracking marketing (optionnel)

### 1.2 Modules Optionnels

**Dépendances optionnelles :**
- `pos_loyalty` : Programmes fidélité (points, récompenses)
- `pos_restaurant` : Mode restaurant (tables, services)
- `pos_hr` : Multi-employés (employee login)
- `pos_iot` : Imprimantes, balances, scanners (IoT Box)
- `website_sale` : Lien avec la boutique en ligne (si applicable)

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Stock (Inventory)

**Flux :**
```
pos.order (state → done) → stock.move (sortie)
pos.order (refund) → stock.move (entrée)
```

**Mécanismes :**
- À la validation de la commande POS (state = done), création de mouvements de stock (sortie) pour chaque ligne.
- Entrepôt de destination configuré sur le POS (ex. magasin).
- Retours : commande de remboursement → mouvements d'entrée (retour au stock).
- Lots / numéros de série : gérés sur les lignes POS si activé (`pos_lot` / serial).

**Champs liés :**
- `pos.config` : Entrepôt ou emplacement de sortie
- `pos.order.line` : `product_id`, `qty` → `stock.move`

**Recommandations pour Miyukini :**
- Intégration native avec gestion de stock (sortie à la vente, entrée aux retours).
- Traçabilité lots/séries si nécessaire (MiyuStore ou module dédié).

### 2.2 Intégration avec Comptabilité (Accounting)

**Flux :**
```
pos.payment → account.bank.statement.line
pos.session (closed) → account.bank.statement (validation)
pos.order (to_invoice) → account.move (facture client)
```

**Mécanismes :**
- **Journal de caisse :** Chaque paiement POS alimente une ligne de relevé bancaire (`account.bank.statement.line`) du journal configuré sur le POS.
- **Clôture de session :** Le relevé de caisse (statement) est validé ; les écritures comptables sont générées (encaissements, écarts éventuels).
- **Facturation :** Si la commande est marquée "To invoice" ou si le client demande une facture, création d'un `account.move` (facture client) ; lien bidirectionnel `pos.order.account_move`.

**Champs liés :**
- `pos.config.journal_id` : Journal de caisse
- `pos.config.invoice_journal_id` : Journal pour factures POS
- `pos.payment.statement_line_id` : Ligne de relevé
- `pos.order.account_move` : Facture générée

**Recommandations pour Miyukini :**
- Intégration native avec MiyuInvoice (facturation depuis le ticket).
- Rapprochement caisse via trésorerie / comptabilité (MiyuTreasury, MiyuBilling ou équivalent).

### 2.3 Intégration avec Sales

**Flux :**
```
sale.order → POS (encaissement, paiement)
POS → sale.order (création commande Sales payée via POS)
```

**Mécanismes :**
- **Import dans le POS :** Depuis le POS, accès aux devis/commandes Sales ("Quotations/Orders") ; application d’un acompte ou règlement total via le POS.
- **Création depuis le POS :** Possibilité de créer une commande Sales depuis le POS et de la régler au POS.
- Données partagées : client (`res.partner`), produits (`product.product`), pricelist (`product.pricelist`).

**Champs liés :**
- `pos.order.partner_id` : Client (partagé avec Sales)
- `pos.order` : Lien optionnel vers `sale.order` (selon implémentation)
- Pricelist : `pos.config.pricelist_id` ou `partner_id.property_product_pricelist`

**Recommandations pour Miyukini :**
- Lien bidirectionnel POS ↔ Sales (Miyukini Sales) : encaissement de commandes Sales au POS, création de commandes Sales depuis le POS si besoin.

### 2.4 Intégration avec Product

**Flux :**
```
product.product → pos.order.line
product.pricelist → prix POS
product.template (barcode) → recherche / scan POS
```

**Mécanismes :**
- Catalogue produits partagé : sélection et scan (code-barres) dans le POS.
- Prix : depuis la pricelist du POS ou du client ; modification manuelle possible.
- Variantes, UOM, taxes : héritées du produit.
- Produits limités en chargement : bouton "Search more" pour charger davantage.

**Champs liés :**
- `pos.order.line.product_id`, `product_uom_id`, `price_unit`, `tax_ids`
- `pos.config.pricelist_id`
- `product.product.barcode` (recherche / scan)

**Recommandations pour Miyukini :**
- Intégration native avec MiyuStore (produits, prix, code-barres, pricelist).

### 2.5 Intégration avec Contacts (res.partner)

**Flux :**
```
res.partner → pos.order.partner_id (client sur le ticket)
res.partner → pricelist, adresse facturation (facture)
res.partner → fidélité (si pos_loyalty)
```

**Mécanismes :**
- Client optionnel sur la commande POS : recherche et sélection ; création rapide depuis le POS (Customer → Create).
- Si client renseigné : application de sa pricelist, adresse de facturation pour la facture, programme fidélité si activé.
- Données partagées : `res.partner` (Contacts).

**Champs liés :**
- `pos.order.partner_id`
- `partner_id.property_product_pricelist`, adresses

**Recommandations pour Miyukini :**
- Intégration native avec MiyuContacts (recherche client, création rapide, pricelist, adresses).

### 2.6 Intégration avec Paiements (Payment)

**Flux :**
```
pos.payment.method → terminaux (Adyen, Stripe, etc. si modules)
pos.payment → account.bank.statement.line
```

**Mécanismes :**
- Méthodes de paiement configurées sur le POS (espèces, carte, QR, etc.).
- Terminaux physiques : modules dédiés (Adyen, Ingenico, Stripe, etc.) pour paiement carte.
- Chaque paiement POS → une ligne de relevé (caisse ou bancaire) selon la méthode.

**Recommandations pour Miyukini :**
- Intégration avec moyens de paiement (MiyuTreasury, MiyuBilling ou module paiement) pour rapprochement et terminaux si besoin.

---

## 3. Flux de Données Inter-Apps

### 3.1 Flux Complet Vente POS

```
Product (catalogue)
    ↓
pos.session (opened)
    ↓
pos.order (draft → paid → done)
    ↓
pos.payment → account.bank.statement.line
    ↓
Stock move (sortie)
    ↓
[Option] account.move (facture client)
    ↓
pos.session (closed) → validation statement
```

### 3.2 Flux Retour / Remboursement

```
pos.order (original) → Refund
    ↓
pos.order (refund, lignes négatives)
    ↓
pos.payment (refund method)
    ↓
Stock move (entrée)
    ↓
account.move (avoir) si facturé
```

### 3.3 Données Partagées

**Données partagées :**
- **Partner** : Client partagé entre POS, Sales, Accounting, Contacts
- **Product** : Produits partagés entre POS, Stock, Sales, Accounting
- **Pricelist** : Listes de prix partagées (Product, Sales, POS)
- **Journal / Statement** : Relevés de caisse partagés entre POS et Accounting

---

## 4. Mécanismes d'Intégration

### 4.1 Hooks et Overrides

**Hooks typiques (conceptuels) :**
- Validation de commande POS → création des `stock.move`
- Paiement POS → création de `account.bank.statement.line`
- Clôture de session → validation du statement, écritures comptables
- Facturation POS → création de `account.move` (facture), lien avec `pos.order`

**Overrides :**
- `product.product` : Prix et taxes selon pricelist et position fiscale POS
- `res.partner` : Création rapide depuis le POS (champs minimaux)

### 4.2 Événements et Signaux

**Événements :**
- Commande POS validée (done) → Génération mouvements de stock
- Paiement enregistré → Mise à jour du relevé de caisse
- Session clôturée → Validation du relevé, consolidation comptable
- Commande facturée → Création facture, mise à jour statut

---

## 5. Recommandations pour Miyukini

### 5.1 Intégrations Prioritaires

**Intégrations natives :**
1. **MiyuStore** : Produits, pricelist, code-barres, taxes
2. **MiyuInvoice** : Facturation depuis le ticket POS
3. **MiyuContacts** : Clients (recherche, création rapide, pricelist)
4. **Gestion Stock** : Sorties à la vente, entrées aux retours (MiyuStore ou module stock)
5. **Trésorerie / Paiements** : Rapprochement caisse (MiyuTreasury, MiyuBilling ou équivalent)

### 5.2 Patterns d'Intégration

**Actions :**
- Lien bidirectionnel POS ↔ Sales pour encaissement de commandes et création de commandes depuis le POS
- Synchronisation automatique stock à la validation de la commande POS
- Génération facture depuis le ticket avec lien bidirectionnel
- Rapprochement caisse par méthode de paiement à la clôture de session

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
