# Odoo Invoicing — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Invoicing** d'Odoo (version 19.0). Invoicing est une application standalone centrée sur la facturation : création et envoi de factures clients, factures fournisseurs, avoirs, suivi des paiements et conditions de paiement. Elle partage le socle technique du module `account` avec Accounting mais avec un périmètre fonctionnel réduit à la facturation.

**Source d'analyse :** Code source Odoo 19.0 (module `account` — périmètre Invoicing)

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données facturation (account.move factures, lignes, paiements)
- Règles métier factures clients/fournisseurs et avoirs
- Workflows création → validation → envoi → paiement
- Calculs (montants HT/TTC, taxes, échéances)
- Conditions de paiement et échéanciers
- États de paiement et réconciliation facture/paiement

**Hors scope :**
- Grand livre complet, rapprochement bancaire (document Accounting)
- Implémentation technique détaillée (guide d'implémentation)
- Spécifications UI/UX (document dédié)

---

## 1. Architecture des Modèles de Données (Périmètre Invoicing)

### 1.1 Modèle `account.move` (Facture / Écriture)

**Rôle :** Représente une facture client, facture fournisseur, avoir ou reçu. Dans le périmètre Invoicing, seuls les types liés à la facturation sont utilisés.

**Champs clés (facturation) :**
- `name` : Numéro de facture (séquence automatique)
- `move_type` : out_invoice, in_invoice, out_refund, in_refund, out_receipt, in_receipt
- `state` : draft, posted, cancel
- `partner_id` : Client ou fournisseur
- `invoice_date` : Date de facture
- `invoice_date_due` : Date d'échéance (ou calculée depuis payment_term_id)
- `invoice_payment_term_id` : Conditions de paiement
- `amount_untaxed`, `amount_tax`, `amount_total` : Montants calculés
- `amount_residual` : Restant à payer
- `payment_state` : not_paid, partial, paid, in_payment, reversed, blocked
- `line_ids` : Lignes de facture (produits, taxes, échéances)

**Types facturation (move_type) :**
- `out_invoice` : Facture client
- `in_invoice` : Facture fournisseur
- `out_refund` : Avoir client
- `in_refund` : Avoir fournisseur
- `out_receipt` : Reçu de vente
- `in_receipt` : Reçu d'achat

**Règles métier (facturation) :**
- Équilibre comptable : somme débits = somme crédits avant validation
- Numéro de facture généré à la validation (séquence du journal)
- payment_state dérivé des réconciliations des lignes créances/dettes
- Une facture validée ne peut être modifiée (sauf remise en brouillon si non verrouillée)

### 1.2 Modèle `account.move.line` (Ligne de Facture)

**Rôle :** Ligne de facture : produit/service, taxe, ou échéance de paiement.

**Champs clés :**
- `move_id` : Facture parente
- `display_type` : product, tax, payment_term, line_section, line_note
- `product_id`, `quantity`, `price_unit`, `discount` : Ligne produit
- `account_id` : Compte comptable
- `debit`, `credit`, `balance` : Montants
- `amount_residual` : Restant à payer (lignes créances/dettes)
- `date_maturity` : Échéance (lignes payment_term)
- `tax_ids` : Taxes appliquées
- `reconciled` : Ligne réconciliée ou non

**Règles métier :**
- Lignes `payment_term` générées automatiquement selon `invoice_payment_term_id`
- Lignes `tax` générées automatiquement à partir des lignes produit et des `tax_ids`
- Montants résiduels recalculés après chaque réconciliation

### 1.3 Modèle `account.payment.term` (Conditions de Paiement)

**Rôle :** Définit les échéances (ex. 30 jours, 50 % à la commande / 50 % à la livraison).

**Champs clés :**
- `name` : Libellé (ex. "30 jours")
- `line_ids` : Lignes d'échéance (percent, fixed, balance)
- `line_ids.value_amount`, `line_ids.nb_days`, `line_ids.discount_percentage`

**Règles métier :**
- Les lignes d'échéance déterminent le nombre de lignes `payment_term` sur la facture
- Chaque ligne a une `date_maturity` = invoice_date + nb_days (ou règle métier équivalente)

### 1.4 Modèle `account.payment` (Paiement)

**Rôle :** Enregistrement d'un paiement (client ou fournisseur) et liaison aux factures.

**Champs clés :**
- `payment_type` : inbound (client), outbound (fournisseur)
- `partner_type` : customer, supplier
- `amount` : Montant
- `reconciled_invoice_ids` : Factures réconciliées avec ce paiement
- `state` : draft, posted, sent, reconciled, cancel

**Règles métier :**
- Un paiement peut régler une ou plusieurs factures (réconciliation partielle ou totale)
- La réconciliation met à jour `payment_state` des factures concernées

---

## 2. Workflows Facturation

### 2.1 Workflow Facture Client

```
[Draft] → [Posted] → [Sent] → [Paid]
   ↓          ↓
[Cancel]   [Partial]
```

**Transitions :**
1. **Draft → Posted :** Validation (équilibre, séquence, calcul taxes/échéances)
2. **Posted → Sent :** Envoi (email, impression PDF)
3. **Posted → Paid / Partial :** Enregistrement de paiements et réconciliation
4. **Posted → Cancel :** Annulation (avoir ou contre-passation selon politique)

### 2.2 Workflow Facture Fournisseur

```
[Draft] → [Posted] → [Paid]
   ↓          ↓
[Cancel]   [Partial]
```

Même logique que facture client, sans état "Sent" métier obligatoire.

### 2.3 États de Paiement (payment_state)

- `not_paid` : Aucun paiement
- `in_payment` : Paiement(s) enregistré(s), pas encore réconcilié(s)
- `paid` : Intégralement réglé
- `partial` : Partiellement réglé
- `reversed` : Avoir / contre-passation
- `blocked` : Bloqué (litige, vérification)

---

## 3. Calculs Facturation

### 3.1 Montants de Ligne (HT)

- `price_subtotal = quantity * price_unit * (1 - discount/100)`
- `price_total` (TTC) selon taxes incluses/exclues et taux

### 3.2 Montants de Facture

- `amount_untaxed` : Somme des price_subtotal des lignes produit
- `amount_tax` : Somme des lignes de taxe
- `amount_total` : amount_untaxed + amount_tax (ou somme des price_total si TTC)
- `amount_residual` : Somme des montants résiduels des lignes créances/dettes (non réconciliés)

### 3.3 Échéancier (Conditions de Paiement)

- Pour chaque ligne de `account.payment.term.line` : calcul du montant (percent, fixed, balance) et de la date d’échéance (invoice_date + nb_days)
- Création des lignes `account.move.line` avec `display_type='payment_term'` et `date_maturity` renseignée

### 3.4 Taxes

- Application des `tax_ids` sur chaque ligne produit
- Création des lignes `display_type='tax'` avec compte de taxe et montant
- Gestion TVA incluse/exclue selon paramétrage entreprise

---

## 4. Envoi et Suivi

### 4.1 Envoi de Facture

- Génération PDF (template facture)
- Envoi par email (partenaire, pièce jointe PDF)
- Marqueur "sent" (champ ou statut) pour suivi
- Portail client : consultation et paiement en ligne (si module portal/website)

### 4.2 Relances

- Calcul des impayés (amount_residual, date_maturity)
- Workflows de relance (email, rapports "à relancer") souvent dans Accounting ou module dédié ; en Invoicing, le suivi se limite aux états et montants résiduels

---

## 5. Réconciliation Facture / Paiement

### 5.1 Principe

- Les lignes de facture sur comptes créances (clients) ou dettes (fournisseurs) sont réconciliables
- Les lignes de paiement (journal banque/caisse) sont réconciliées avec ces lignes
- Réconciliation partielle : un paiement réparti sur plusieurs factures, ou une facture réglée par plusieurs paiements
- Réconciliation complète : amount_residual = 0 → payment_state = paid

### 5.2 Modèles Associés

- `account.partial.reconcile` : lien entre lignes débit/crédit et montant réconcilié
- `account.full.reconcile` : regroupement de réconciliations partielles pour une réconciliation complète
- Recalcul automatique de `amount_residual` et `payment_state` après chaque réconciliation

---

## 6. Règles de Validation et Contraintes

### 6.1 Avant Validation (Post)

- Équilibre comptable (somme débits = somme crédits)
- Journal et type de mouvement cohérents (ex. journal ventes pour out_invoice)
- Partenaire obligatoire pour factures/avoirs
- Au moins une ligne de type produit ou service (selon règles métier Odoo)

### 6.2 Verrouillage et Annulation

- Verrouillage fiscal (dates) : pas de modification si date antérieure au verrouillage
- Annulation : possibilité de "Reset to Draft" si pas de verrouillage et pas de hash d’inaltérabilité
- Avoir : création d’une facture out_refund/in_refund liée ou contre-passation selon politique

---

## 7. Points d'Attention pour Miyukini

### 7.1 Équivalences Conceptuelles

| Concept Odoo Invoicing | Équivalent Miyukini |
|------------------------|----------------------|
| account.move (facture) | MiyuInvoice + WriteIntent KindMother |
| account.move.line (ligne) | Ligne facture (KindMother) |
| account.payment.term | Règles métier (StrongFather) / MiyuInvoice |
| account.payment | Paiement (KindMother) + décision StrongFather |
| Validation / Envoi | WriteIntent + StrongFather + outils envoi |
| Réconciliation | Opération gouvernée (StrongFather, KindMother) |

### 7.2 Périmètre Invoicing vs Accounting

| Fonctionnalité | Invoicing | Accounting |
|----------------|-----------|------------|
| Factures clients/fournisseurs | Oui | Oui |
| Avoirs | Oui | Oui |
| Conditions de paiement | Oui | Oui |
| Paiements et réconciliation | Oui | Oui |
| Grand livre / Journaux détaillés | Limité | Complet |
| Rapprochement bancaire | Non | Oui |
| Plan comptable avancé | Limité | Complet |
| Rapports (balance, bilan, etc.) | Limité | Complet |

### 7.3 Défis d'Implémentation

1. **Périmètre** : Réutiliser MiyuInvoice et éventuellement MiyuComptaLedger sans dupliquer la logique Accounting.
2. **Workflow** : Décision StrongFather pour validation et envoi ; KindMother pour persistance (WriteIntent).
3. **Séquencement** : Numéros de facture via Ever Buddy.
4. **Envoi** : Intégration email/PDF et portail (Mandats, WorrySentinel pour données personnelles).

---

## 8. Références Techniques

### 8.1 Fichiers Clés (module account)

- `account/models/account_move.py` : Factures, validation, calculs
- `account/models/account_move_line.py` : Lignes, résiduels, réconciliation
- `account/models/account_payment.py` : Paiements
- `account/models/account_payment_term.py` : Conditions de paiement

### 8.2 Méthodes Métier Critiques (facturation)

**account.move :**
- `action_post()` : Validation
- `_compute_amount()` : Montants HT/TTC/résiduel
- `_get_invoice_computed_reference()` : Référence paiement
- `action_invoice_sent()` : Marquer envoyé / envoi email

**account.move.line :**
- `_compute_amount_residual()` : Montant résiduel
- `_reconcile_lines()` : Réconciliation

---

## 9. Conclusion

L’application **Invoicing** d’Odoo couvre la **facturation de bout en bout** : création, validation, envoi, paiements et réconciliation, avec des modèles et calculs partagés avec Accounting. Pour Miyukini, l’implémentation devra s’appuyer sur **MiyuInvoice** et la gouvernance COG (StrongFather, KindMother, Ever Buddy, WorrySentinel) pour validation, persistance, numérotation et sécurité, sans dupliquer la logique complète du grand livre.

**Prochaines étapes :** Voir [Guide d'Implémentation](../06_guides_implementation/Odoo%20Invoicing%20-%20Guide%20Implementation.md) pour les spécifications techniques.

---

**Document** : Odoo Invoicing — Analyse Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Référence pour implémentation Miyukini
