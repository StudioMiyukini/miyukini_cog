# Odoo Invoicing — Intégrations Cross-App

## Contexte

Ce document analyse les **intégrations cross-app** de l’application **Invoicing** d’Odoo : dépendances, flux de données, mécanismes d’intégration et APIs, pour alimenter l’équivalent Miyukini.

**Source d'analyse :** Code source Odoo 19.0 (module account et modules optionnels).

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Dépendances avec les autres apps Odoo
- Flux de données facturation ↔ Sales, Purchase, CRM, etc.
- Mécanismes d’intégration (hooks, champs liés, wizards)
- Recommandations pour Miyukini

**Hors scope :**
- Détail des modèles internes (voir Logique Métier)
- Implémentation technique Miyukini (voir Guide Implémentation)

---

## 1. Dépendances Principales

### 1.1 Modules requis (Invoicing / account)

**Dépendances explicites (`__manifest__.py`) :**
- `base` : Partenaires, sociétés, devises, séquences
- `mail` : Activités, suivi, notifications
- `portal` : Portail client (consultation factures, paiement en ligne si activé)
- `web` : Framework web
- `utm` : Campagnes / source (optionnel pour facturation)

### 1.2 Modules optionnels (facturation)

**Dépendances optionnelles :**
- `sale` : Factures depuis commandes clients
- `purchase` : Factures fournisseurs depuis commandes d’achat
- `stock` : Facturation à la livraison (quantités livrées)
- `project` : Facturation projets / timesheet
- `expense` : Remboursements et factures liées aux notes de frais
- `crm` : Factures depuis opportunités / commandes CRM
- `website` : Portail public et paiement en ligne
- `l10n_*` : Localisations (plans comptables, taxes par pays)

---

## 2. Intégrations Détaillées

### 2.1 Intégration avec Sales

**Flux :**
```
Commande client (sale.order) → Facture (account.move)
```

**Mécanismes :**
- Création de factures depuis la commande : `sale.order._create_invoices()`
- Lien bidirectionnel : `invoice_ids` sur `sale.order` ↔ `sale_line_ids` sur `account.move.line`
- Synchronisation des montants : `amount_invoiced`, `amount_to_invoice` sur la commande
- Statut facturation commande : `invoice_status` (invoiced, to invoice, upselling, no)

**Champs liés :**
- `invoice_origin` : Référence à la commande
- `invoice_payment_term_id` : Repris depuis la commande
- Lignes de facture liées aux lignes de commande (quantité à facturer, prix, taxes)

**Hooks / méthodes :**
- `sale.order._prepare_invoice()` : Préparation de l’en-tête facture
- `sale.order._prepare_invoice_line()` : Préparation des lignes
- `account.move._get_invoice_reference()` : Référence de paiement (commande)

**Recommandations Miyukini :**
- Intégration native **Miyukini Sales ↔ MiyuInvoice** : création de factures depuis les commandes confirmées
- Lien bidirectionnel commande ↔ facture et mise à jour des montants facturés / à facturer

### 2.2 Intégration avec Purchase

**Flux :**
```
Commande fournisseur (purchase.order) → Facture fournisseur (account.move)
```

**Mécanismes :**
- Création de factures fournisseur depuis la commande : `purchase.order._create_invoices()`
- Lien : `invoice_ids` sur `purchase.order` ↔ `purchase_line_ids` sur `account.move.line`
- Montants : `amount_invoiced`, `amount_to_invoice` sur la commande
- Statut : `invoice_status` (invoiced, to invoice, no)

**Champs liés :**
- `invoice_origin` : Référence commande fournisseur
- Conditions de paiement et partenaire repris de la commande

**Recommandations Miyukini :**
- Si module Purchase existe : création de factures fournisseur depuis commandes d’achat
- Même pattern que Sales : origine, lignes, montants synchronisés

### 2.3 Intégration avec Stock (Livraison)

**Flux :**
```
Livraison (stock.picking) → Quantités livrées → Facturation (sale + account)
```

**Mécanismes :**
- Avec `sale_stock` : `qty_delivered` mis à jour depuis les livraisons
- Facturation sur livraison : lignes de facture basées sur les quantités livrées
- Lien optionnel : `stock_move_id` / `stock_picking_id` sur lignes pour traçabilité

**Recommandations Miyukini :**
- Intégration MiyuShipping / logistique avec MiyuInvoice : quantités livrées pour facturation à la livraison

### 2.4 Intégration avec CRM

**Flux :**
```
Opportunité / Commande CRM → Facture (account.move)
```

**Mécanismes :**
- Création de commande depuis l’opportunité, puis facture depuis la commande (voir Sales)
- Ou création directe de facture depuis une opportunité gagnée (selon flux métier)
- Champs UTM (campagne, source) sur la facture pour analyse

**Recommandations Miyukini :**
- Si CRM Miyukini : flux Opportunité → Commande → Facture, avec traçabilité origine

### 2.5 Intégration avec Project / Timesheet

**Flux :**
```
Tâches / Timesheet → Facturation au temps ou forfait (account.move)
```

**Mécanismes :**
- Lignes de facture créées depuis les lignes de timesheet (quantité = heures, prix = tarif)
- Lien projet / tâche → facture pour reporting
- Facturation forfait ou livrable : tâches livrables → lignes de facture

**Recommandations Miyukini :**
- Intégration MiyuProject / timesheet avec MiyuInvoice : facturation temps ou livrables

### 2.6 Intégration avec Expense (Notes de frais)

**Flux :**
```
Note de frais (hr.expense) → Remboursement / Facture fournisseur (account.move)
```

**Mécanismes :**
- Validation des notes de frais → création de facture fournisseur (remboursement) ou écriture de paiement
- Lien `expense_id` sur lignes de facture ou paiement

**Recommandations Miyukini :**
- Si module Notes de frais : flux validation → facture/remboursement avec lien traçable

### 2.7 Portail et Website

**Flux :**
```
Facture → Email avec lien portail → Client consulte / paie en ligne
```

**Mécanismes :**
- `portal` : Accès sécurisé du partenaire à ses factures et documents
- `website` + paiement en ligne : règlement par carte / SEPA depuis le portail
- Partage de document : token dans l’URL pour accès sans compte

**Recommandations Miyukini :**
- Façade Publique Gouvernée + Mandat Public d’Accès pour le portail client
- Pas d’accès aux Cores ; exposition contrôlée des factures et statuts de paiement

---

## 3. Mécanismes d’Intégration Communs

### 3.1 Création de facture depuis une origine

- **Pattern :** Bouton "Créer une facture" sur l’enregistrement source (commande, livraison, projet, etc.)
- **Données :** En-tête (partenaire, dates, conditions de paiement) et lignes (produits, quantités, prix) pré-remplies
- **Lien :** `invoice_origin` + champs relationnels (sale_line_ids, purchase_line_ids, etc.) pour synchronisation et reporting

### 3.2 Synchronisation des montants

- **Montants facturés / à facturer** sur l’objet source (commande, projet)
- Recalcul à chaque création/modification/annulation de facture ou de ligne
- **Montant résiduel** et **payment_state** sur la facture pour relances et tableaux de bord

### 3.3 Événements et notifications

- **Mail** : Envoi de la facture par email, activités de suivi
- **Notifications** : Facture validée, facture payée (pour le vendeur ou le responsable)
- **Activités** : Rappels "Relancer facture", "Vérifier paiement"

---

## 4. Synthèse des Flux (Schéma)

```
Sales (Commande)     →  Invoicing (Facture client)  →  Paiement / Réconciliation
Purchase (Commande)  →  Invoicing (Facture fourn.)  →  Paiement / Réconciliation
Project / Timesheet →  Invoicing (Facture temps)    →  Paiement
Expense             →  Invoicing (Remboursement)   →  Paiement
CRM (Opportunité)  →  Sales → Invoicing            →  Paiement
Portal / Website    ←  Invoicing (consultation / paiement client)
```

---

## 5. Recommandations Miyukini (Résumé)

1. **MiyuInvoice** comme socle facturation ; intégrations en amont (Sales, Purchase, Project, Expense) via Contrats d’Équipe et Mandats.
2. **Lien bidirectionnel** origine (commande, projet, etc.) ↔ facture avec montants synchronisés.
3. **Portail client** : Façade Publique Gouvernée, pas d’accès aux Cores ; exposition limitée aux factures et paiements du partenaire.
4. **Événements** : Notifications et activités gouvernées (Master Butler, WorrySentinel) pour envoi et suivi.
5. **Localisation** : Taxes et conditions de paiement par pays/régime (réutiliser règles métier Miyukini existantes).

---

**Document** : Odoo Invoicing — Intégrations Cross-App  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Référence pour implémentation Miyukini
