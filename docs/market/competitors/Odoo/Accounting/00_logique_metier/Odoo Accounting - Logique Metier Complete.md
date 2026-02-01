# Odoo Accounting — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Accounting** d'Odoo (version 19.0), extraite du code source GitHub. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes de gouvernance pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** `https://github.com/odoo/odoo/tree/19.0/addons/account`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (Account, Journal, Move, MoveLine)
- Règles métier et contraintes
- Workflows et transitions d'état
- Calculs comptables (balance, réconciliation, taxes)
- Mécanismes de séquencement et numérotation
- Gestion multi-devises
- Système de réconciliation

**Hors scope :**
- Implémentation technique détaillée (sera dans le guide d'implémentation)
- Spécifications UI/UX (document dédié)
- Parcours utilisateur (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `account.account` (Plan Comptable)

**Rôle :** Représente un compte comptable dans le plan comptable.

**Champs clés :**
- `code` : Code du compte (ex: "411000", "701000")
- `name` : Libellé du compte
- `account_type` : Type de compte (asset_receivable, liability_payable, income, expense, etc.)
- `reconcile` : Boolean indiquant si le compte permet la réconciliation
- `currency_id` : Devise du compte (optionnel, pour comptes multi-devises)
- `company_ids` : Many2many vers `res.company` (multi-company)
- `tag_ids` : Tags analytiques pour les rapports

**Règles métier :**
- Les comptes de type `asset_receivable` ou `liability_payable` **doivent** avoir `reconcile=True`
- Un compte ne peut pas être supprimé s'il contient des lignes d'écriture (`account.move.line`)
- Le code de compte doit être unique par entreprise (avec gestion hiérarchique parent/enfant)
- Les comptes peuvent être partagés entre plusieurs entreprises (multi-company)

**Types de comptes (account_type) :**
```
- asset_receivable : Créances clients
- asset_cash : Banque et caisse
- asset_current : Actifs courants
- asset_non_current : Actifs non courants
- asset_prepayments : Acomptes
- asset_fixed : Immobilisations
- liability_payable : Dettes fournisseurs
- liability_credit_card : Cartes de crédit
- liability_current : Passifs courants
- liability_non_current : Passifs non courants
- equity : Capitaux propres
- equity_unaffected : Résultat de l'exercice
- income : Produits
- income_other : Autres produits
- expense : Charges
- expense_other : Autres charges
- expense_depreciation : Amortissements
- expense_direct_cost : Coût des ventes
- off_balance : Hors bilan
```

### 1.2 Modèle `account.journal` (Journaux Comptables)

**Rôle :** Représente un journal comptable (ventes, achats, banque, caisse, divers).

**Champs clés :**
- `name` : Nom du journal
- `code` : Code du journal (ex: "VEN", "ACH", "BAN")
- `type` : Type de journal (sale, purchase, bank, cash, general)
- `default_account_id` : Compte par défaut pour les écritures
- `currency_id` : Devise du journal (optionnel)
- `sequence_id` : Séquence pour la numérotation automatique
- `company_id` : Entreprise propriétaire

**Types de journaux :**
- `sale` : Journal des ventes
- `purchase` : Journal des achats
- `bank` : Journal bancaire
- `cash` : Journal de caisse
- `general` : Journal divers

**Règles métier :**
- Chaque journal a une séquence de numérotation unique
- Les journaux de type `sale`/`purchase` génèrent automatiquement des factures
- Les journaux `bank`/`cash` sont utilisés pour les paiements

### 1.3 Modèle `account.move` (Écriture Comptable / Facture)

**Rôle :** Représente une écriture comptable ou une facture (client/fournisseur).

**Champs clés :**
- `name` : Numéro de l'écriture (séquence automatique)
- `date` : Date comptable
- `journal_id` : Journal utilisé
- `move_type` : Type d'écriture (entry, out_invoice, in_invoice, out_refund, in_refund, etc.)
- `state` : État (draft, posted, cancel)
- `partner_id` : Partenaire (client/fournisseur)
- `invoice_date` : Date de facture (si facture)
- `invoice_date_due` : Date d'échéance
- `amount_total` : Montant total TTC
- `amount_untaxed` : Montant HT
- `amount_tax` : Montant des taxes
- `payment_state` : État de paiement (not_paid, partial, paid, reversed)
- `line_ids` : Lignes d'écriture (One2many vers `account.move.line`)

**Types d'écritures (move_type) :**
- `entry` : Écriture manuelle
- `out_invoice` : Facture client
- `out_refund` : Avoir client
- `in_invoice` : Facture fournisseur
- `in_refund` : Avoir fournisseur
- `out_receipt` : Reçu de vente
- `in_receipt` : Reçu d'achat

**États (state) :**
- `draft` : Brouillon (modifiable)
- `posted` : Validé (non modifiable sauf exception)
- `cancel` : Annulé

**Règles métier critiques :**
1. **Équilibre comptable :** La somme des débits doit égaler la somme des crédits
2. **Séquencement :** Le numéro (`name`) est généré automatiquement selon la séquence du journal
3. **Verrouillage fiscal :** Une fois validée, une écriture ne peut pas être modifiée si elle dépasse la date de verrouillage fiscale
4. **Hash d'inaltérabilité :** Les écritures validées peuvent être sécurisées avec un hash SHA-256 pour conformité légale
5. **Multi-devises :** Gestion des taux de change avec conversion automatique

### 1.4 Modèle `account.move.line` (Ligne d'Écriture)

**Rôle :** Représente une ligne d'écriture comptable (débit ou crédit).

**Champs clés :**
- `move_id` : Écriture parente
- `account_id` : Compte comptable
- `partner_id` : Partenaire (hérité de move)
- `name` : Libellé
- `debit` : Montant débit
- `credit` : Montant crédit
- `balance` : Balance (débit - crédit)
- `amount_currency` : Montant en devise étrangère
- `currency_id` : Devise
- `date_maturity` : Date d'échéance (pour créances/dettes)
- `reconciled` : Boolean indiquant si la ligne est réconciliée
- `full_reconcile_id` : Réconciliation complète
- `matched_debit_ids` / `matched_credit_ids` : Réconciliations partielles

**Types d'affichage (display_type) :**
- `product` : Ligne produit/service
- `tax` : Ligne de taxe
- `payment_term` : Ligne d'échéance
- `rounding` : Arrondi
- `line_section` : Section
- `line_note` : Note

**Règles métier :**
- Une ligne ne peut avoir qu'un débit **OU** un crédit (pas les deux)
- Les lignes de type `payment_term` sont générées automatiquement selon les conditions de paiement
- Les lignes de taxe sont générées automatiquement selon les taxes appliquées
- La réconciliation se fait entre lignes de comptes réconciliables (`reconcile=True`)

---

## 2. Workflows et Transitions d'État

### 2.1 Workflow d'Écriture Comptable

```
[Draft] → [Posted] → [Cancel] (si annulation)
         ↓
    [Locked] (si hash d'inaltérabilité)
```

**Transitions :**

1. **Draft → Posted :**
   - Vérification de l'équilibre comptable
   - Génération du numéro de séquence (si non défini)
   - Création des lignes analytiques (si analytique activé)
   - Calcul des taxes et répartition
   - Vérification des dates de verrouillage fiscale
   - Génération des lignes d'échéance selon conditions de paiement

2. **Posted → Cancel :**
   - Vérification que l'écriture n'est pas réconciliée
   - Vérification des dates de verrouillage
   - Annulation des lignes analytiques
   - Inversion des montants (si nécessaire)

3. **Posted → Draft (Reset) :**
   - Possible uniquement si pas de hash d'inaltérabilité
   - Possible uniquement si pas de verrouillage fiscal
   - Suppression des lignes analytiques
   - Réinitialisation des montants résiduels

### 2.2 Workflow de Facture

```
[Draft] → [Posted] → [Sent] → [Paid]
         ↓
    [Cancel]
```

**États de paiement (payment_state) :**
- `not_paid` : Non payé
- `partial` : Partiellement payé
- `paid` : Payé
- `in_payment` : En cours de paiement
- `reversed` : Inversé (avoir)
- `blocked` : Bloqué

**Calcul automatique :**
- Le `payment_state` est calculé automatiquement selon les réconciliations
- Une facture est `paid` si toutes les lignes créances/dettes sont réconciliées

---

## 3. Calculs Comptables

### 3.1 Calcul de Balance

**Formule :**
```
balance = debit - credit
```

**Règles :**
- Pour les comptes d'actif : balance positive = débit, balance négative = crédit
- Pour les comptes de passif : balance positive = crédit, balance négative = débit
- Pour les comptes de charges : balance positive = débit
- Pour les comptes de produits : balance positive = crédit

### 3.2 Calcul des Montants de Facture

**Montants calculés :**
- `amount_untaxed` : Somme des lignes produits HT
- `amount_tax` : Somme des lignes taxes
- `amount_total` : `amount_untaxed + amount_tax`
- `amount_residual` : Montant restant à payer (non réconcilié)

**Calcul des lignes produits :**
```
price_subtotal = quantity * price_unit * (1 - discount/100)
price_total = price_subtotal * (1 + tax_rate/100)
```

### 3.3 Gestion Multi-Devises

**Taux de change :**
- `invoice_currency_rate` : Taux de change au moment de la facture
- Conversion automatique : `balance = amount_currency / currency_rate`

**Règles :**
- Le taux de change est figé à la validation de la facture
- Les différences de change sont enregistrées dans des comptes d'écart de change
- Les réconciliations multi-devises utilisent le taux de change de la ligne

### 3.4 Calcul des Taxes

**Mécanisme :**
1. Pour chaque ligne produit, application des taxes configurées
2. Calcul de la base taxable (HT)
3. Calcul du montant de taxe : `tax_amount = base * tax_rate / 100`
4. Création automatique de lignes de taxe (`display_type='tax'`)
5. Gestion des taxes incluses/exclues selon configuration entreprise

**Types de taxes :**
- `percent` : Pourcentage
- `fixed` : Montant fixe
- `group_of_taxes` : Groupe de taxes

**Répartition des taxes :**
- `base` : Base taxable
- `tax` : Montant de taxe
- Les taxes peuvent être réparties sur plusieurs comptes (compte de charge/produit, compte de taxe)

---

## 4. Système de Réconciliation

### 4.1 Principe

La réconciliation permet de faire correspondre des créances (débits) avec des paiements (crédits) pour indiquer qu'une facture est payée.

**Types de réconciliation :**
- **Réconciliation complète :** Toutes les lignes sont réconciliées
- **Réconciliation partielle :** Seule une partie est réconciliée

### 4.2 Modèle `account.partial.reconcile`

**Rôle :** Enregistre une réconciliation partielle entre deux lignes.

**Champs :**
- `debit_move_id` : Ligne débit
- `credit_move_id` : Ligne crédit
- `amount` : Montant réconcilié (en devise entreprise)
- `debit_amount_currency` / `credit_amount_currency` : Montants en devises étrangères

**Règles :**
- Une réconciliation partielle peut concerner plusieurs lignes
- Les montants résiduels sont recalculés automatiquement
- Les réconciliations partielles peuvent être combinées en réconciliation complète

### 4.3 Modèle `account.full.reconcile`

**Rôle :** Enregistre une réconciliation complète (toutes les lignes sont réconciliées).

**Règles :**
- Une réconciliation complète ne peut être créée que si toutes les lignes sont à zéro résiduel
- Le `payment_state` de la facture passe à `paid`

---

## 5. Séquencement et Numérotation

### 5.1 Système de Séquence

**Principe :**
- Chaque journal a une séquence (`ir.sequence`)
- Le numéro est généré automatiquement lors de la validation
- Format : `{prefix}{year}{month}{seq}` (ex: "VEN/2026/01/0001")

**Types de séquences :**
- `monthly` : Réinitialisation mensuelle
- `yearly` : Réinitialisation annuelle
- `fixed` : Séquence continue

**Règles :**
- Le numéro est unique par journal et par période
- Les numéros ne peuvent pas avoir de trous (sauf configuration spéciale)
- Les écritures annulées gardent leur numéro

### 5.2 Hash d'Inaltérabilité

**Principe :**
- Pour conformité légale (France, etc.), les écritures validées peuvent être sécurisées avec un hash SHA-256
- Le hash inclut : numéro, date, montants, comptes, partenaires
- Une fois hashée, l'écriture ne peut plus être modifiée

**Calcul du hash :**
```python
hash_input = {
    'name': move.name,
    'date': move.date,
    'journal_id': move.journal_id.id,
    'line_ids': [(line.account_id.code, line.debit, line.credit) for line in move.line_ids]
}
hash = sha256(json.dumps(hash_input, sort_keys=True)).hexdigest()
```

---

## 6. Conditions de Paiement et Échéances

### 6.1 Modèle `account.payment.term`

**Rôle :** Définit les conditions de paiement (ex: "30 jours", "50% à la commande, 50% à la livraison").

**Champs :**
- `name` : Nom (ex: "30 jours")
- `line_ids` : Lignes de paiement (One2many vers `account.payment.term.line`)

**Lignes de paiement :**
- `value` : Type (percent, fixed, balance)
- `value_amount` : Montant ou pourcentage
- `nb_days` : Nombre de jours jusqu'à l'échéance
- `discount_percentage` : Pourcentage de remise pour paiement anticipé
- `discount_days` : Nombre de jours pour bénéficier de la remise

### 6.2 Génération Automatique des Lignes d'Échéance

**Workflow :**
1. Lors de la création/modification d'une facture avec conditions de paiement
2. Calcul automatique des échéances selon `invoice_payment_term_id`
3. Création de lignes `account.move.line` avec `display_type='payment_term'`
4. Chaque ligne a une `date_maturity` calculée

**Exemple :**
- Facture de 1000€ avec conditions "30 jours"
- Création d'une ligne `payment_term` de 1000€ avec `date_maturity = invoice_date + 30 jours`

---

## 7. Gestion Analytique

### 7.1 Modèle `account.analytic.account`

**Rôle :** Représente un compte analytique (centre de coût, projet, etc.).

**Intégration :**
- Les lignes d'écriture peuvent avoir une `analytic_distribution` (JSON)
- Format : `{"account_id_1": percentage_1, "account_id_2": percentage_2}`
- Les pourcentages doivent totaliser 100%

**Création automatique :**
- Lors de la validation d'une écriture avec distribution analytique
- Création de lignes `account.analytic.line` automatiques

---

## 8. Règles de Validation et Contraintes

### 8.1 Contraintes d'Équilibre

**Règle fondamentale :**
```
SUM(debit) = SUM(credit)
```

**Vérification :**
- À chaque création/modification d'écriture
- Blocage de la validation si non équilibrée

### 8.2 Contraintes de Dates

**Verrouillage fiscal :**
- `fiscalyear_lock_date` : Date de verrouillage de l'exercice
- `period_lock_date` : Date de verrouillage de période
- `tax_lock_date` : Date de verrouillage des taxes

**Règles :**
- Une écriture ne peut pas être modifiée si sa date est antérieure à la date de verrouillage
- Les écritures de taxes sont protégées par `tax_lock_date`

### 8.3 Contraintes de Réconciliation

**Règles :**
- Une ligne réconciliée ne peut pas être modifiée
- Pour modifier, il faut d'abord annuler la réconciliation
- Les réconciliations partielles peuvent être annulées individuellement

---

## 9. Points d'Attention pour Miyukini

### 9.1 Différences Architecturales

| Aspect | Odoo | Miyukini |
|--------|------|----------|
| **Architecture** | Monolithique Python | COG distribué Rust |
| **Gouvernance** | ORM Odoo | Cores (StrongFather, KindMother) |
| **Données** | Base unique | WriteIntent vers KindMother |
| **Permissions** | Groups/Users | Mandats de Permission |
| **Sécurité** | Groups | WorrySentinel + Master Butler |

### 9.2 Équivalences Conceptuelles

| Concept Odoo | Équivalent Miyukini |
|--------------|---------------------|
| `account.move` | Écriture comptable (KindMother) |
| `account.move.line` | Ligne d'écriture (KindMother) |
| `account.account` | Plan comptable (KindMother) |
| `account.journal` | Journal comptable (KindMother) |
| Validation (`action_post`) | WriteIntent vers KindMother + décision StrongFather |
| Réconciliation | Opération gouvernée (StrongFather décide, KindMother persiste) |
| Taxes | Calcul via Tools (MiyuComptaLedger) |
| Conditions de paiement | Règles métier (StrongFather) |

### 9.3 Défis d'Implémentation

1. **Équilibre comptable :** Vérification avant WriteIntent KindMother
2. **Séquencement :** Gestion par Ever Buddy (cycle de vie)
3. **Réconciliation :** Workflow gouverné (StrongFather valide, KindMother persiste)
4. **Multi-devises :** Taux de change via service externe ou KindMother
5. **Hash d'inaltérabilité :** Intégrité via WorrySentinel + Kernel

---

## 10. Références Techniques

### 10.1 Fichiers Clés Analysés

- `addons/account/models/account_move.py` : Modèle principal des écritures
- `addons/account/models/account_move_line.py` : Modèle des lignes d'écriture
- `addons/account/models/account_account.py` : Modèle du plan comptable
- `addons/account/models/account_journal.py` : Modèle des journaux
- `addons/account/models/account_payment.py` : Modèle des paiements
- `addons/account/models/account_partial_reconcile.py` : Réconciliations partielles

### 10.2 Méthodes Métier Critiques

**account.move :**
- `action_post()` : Validation de l'écriture
- `_compute_amount()` : Calcul des montants
- `_check_balanced()` : Vérification de l'équilibre
- `_get_invoice_computed_reference()` : Génération référence paiement

**account.move.line :**
- `_compute_amount_residual()` : Calcul du montant résiduel
- `_reconcile_lines()` : Réconciliation de lignes
- `_compute_totals()` : Calcul des totaux HT/TTC

---

## 11. Conclusion

L'application Accounting d'Odoo présente une **logique métier riche et complexe** avec :

- **Modèles de données solides** : Plan comptable, journaux, écritures, lignes
- **Workflows gouvernés** : Validation, annulation, réconciliation
- **Calculs automatiques** : Taxes, échéances, multi-devises
- **Conformité légale** : Hash d'inaltérabilité, verrouillages fiscaux

Pour Miyukini, l'implémentation devra :
1. **Respecter l'architecture COG** : WriteIntent, gouvernance par Cores
2. **Séparer décision et exécution** : StrongFather décide, KindMother persiste
3. **Gérer la sécurité** : WorrySentinel pour les niveaux, Master Butler pour les permissions
4. **Maintenir la cohérence** : Équilibre comptable, séquencement, réconciliation

**Prochaines étapes :** Voir [Guide d'Implémentation](./06_guides_implementation/Odoo%20Accounting%20-%20Guide%20Implementation.md) pour les spécifications techniques détaillées.

---

**Document** : Odoo Accounting — Analyse Logique Métier Complète  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Analyse complète — référence pour implémentation Miyukini
