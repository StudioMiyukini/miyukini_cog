# Odoo POS Shop — Analyse Logique Métier Complète

## Contexte

Ce document analyse en profondeur la **logique métier** de l'application **Point of Sale (POS) Shop** d'Odoo, basée sur la documentation officielle et les conventions du module. Il identifie les modèles de données, règles métier, workflows, calculs et mécanismes pour servir de référence à l'implémentation d'un équivalent dans l'écosystème Miyukini.

**Source d'analyse :** Documentation Odoo 18/19, module `point_of_sale`

**Date d'analyse :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Modèles de données principaux (pos.session, pos.order, pos.order.line)
- Règles métier et contraintes (session, caisse, paiements)
- Workflows (ouverture session → ventes → clôture session)
- Calculs (totaux, taxes, remises, arrondi caisse)
- Gestion des retours et avoirs
- Intégration Stock, Comptabilité, Sales

**Hors scope :**
- POS Restaurant (commandes table/service)
- Implémentation technique détaillée (guide d'implémentation)
- Spécifications UI/UX (document dédié)

---

## 1. Architecture des Modèles de Données

### 1.1 Modèle `pos.session` (Session de Caisse)

**Rôle :** Représente une session de travail au point de vente. Une session est ouverte au début du poste et clôturée en fin de poste.

**États (state) :**
- `opening_control` : Contrôle d'ouverture (saisie fonds de caisse)
- `opened` : Session ouverte, ventes possibles
- `closing_control` : Contrôle de clôture (comptage caisse)
- `closed` : Session clôturée

**Champs clés :**

#### Identification
- `name` : Libellé de la session (séquence, ex. "Session 00042")
- `state` : État de la session
- `config_id` : Many2one vers `pos.config` (configuration du point de vente)

#### Caisse et trésorerie
- `cash_register_id` : Many2one vers `account.bank.statement` (journal de caisse)
- `cash_register_balance_start` : Solde d'ouverture
- `cash_register_balance_end` : Solde de clôture réel
- `cash_register_total_entry_encoding` : Total des entrées (ventes encaissées)
- `cash_real_balance` : Montant compté (saisi à la clôture)
- `cash_real_difference` : Écart (compté - attendu)

#### Utilisateur et point de vente
- `user_id` : Many2one vers `res.users` (utilisateur ayant ouvert la session)
- `start_at` : Datetime (ouverture)
- `stop_at` : Datetime (clôture)

#### Commandes et totaux
- `order_ids` : One2many vers `pos.order` (commandes de la session)
- `order_count` : Nombre de commandes
- `amount_total` : Total des ventes de la session
- `payment_method_ids` : Méthodes de paiement disponibles

#### Résumé paiements
- Résumé par méthode (espèces, carte, etc.) pour contrôle de clôture

**Règles métier :**

1. **Une session ouverte par navigateur :** Une seule session ouverte par point de vente et navigateur.
2. **Ouverture :** Saisie du fonds de caisse initial obligatoire (ou zéro).
3. **Clôture :** Comptage du cash, saisie des billets/pièces, validation des écarts.
4. **Multi-utilisateurs :** Plusieurs employés peuvent être connectés à la même session (employee login).

### 1.2 Modèle `pos.order` (Commande POS)

**Rôle :** Représente une commande (ticket) enregistrée au point de vente.

**États :**
- `draft` : Brouillon (en cours de composition)
- `paid` : Payée
- `done` : Finalisée (ticket imprimé, stock mis à jour)
- `invoiced` : Facturée
- `cancel` : Annulée

**Champs clés :**

#### Identification
- `name` : Référence commande (séquence POS)
- `pos_reference` : Référence unique (session + sequence)
- `session_id` : Many2one vers `pos.session` (session de caisse)
- `sequence_number` : Numéro dans la session

#### Client et point de vente
- `partner_id` : Many2one vers `res.partner` (client, optionnel)
- `config_id` : Many2one vers `pos.config` (point de vente)
- `pricelist_id` : Many2one vers `product.pricelist` (liste de prix)
- `fiscal_position_id` : Many2one vers `account.fiscal.position` (position fiscale)

#### Lignes et totaux
- `lines` : One2many vers `pos.order.line` (lignes du ticket)
- `amount_untaxed` : Total HT
- `amount_tax` : Total taxes
- `amount_total` : Total TTC
- `amount_paid` : Montant déjà payé
- `amount_return` : Montant à rendre
- `currency_id` : Devise

#### Paiements
- `payment_ids` : One2many vers `pos.payment` (paiements du ticket)
- `statement_ids` : Lignes de relevé bancaire liées

#### Facturation
- `account_move` : Many2one vers `account.move` (facture si générée)
- `is_invoiced` : Boolean (commande facturée)

#### Retours et avoirs
- `refunded_order_ids` : Commandes d'avoir liées
- `refund_order_id` : Commande d'origine si c'est un avoir

#### Métadonnées
- `creation_date` : Datetime
- `employee_id` : Employé ayant enregistré la commande (si multi-employee)
- `customer_note` : Note client
- `to_invoice` : Boolean (à facturer)

**Règles métier :**

1. **Une commande = un ticket** : Lignes produits + paiements = commande payée.
2. **Paiement total requis :** La commande ne peut être validée que si `amount_paid >= amount_total`.
3. **Stock :** Lors de la validation (done), les mouvements de stock sont créés (sortie).
4. **Facturation optionnelle :** Le client peut demander une facture ; création d'un `account.move`.
5. **Retours :** Création d'une commande de type avoir (quantités négatives ou flux Refund).

### 1.3 Modèle `pos.order.line` (Ligne de Commande POS)

**Rôle :** Une ligne de ticket (produit, quantité, prix, remise, taxes).

**Champs clés :**

- `order_id` : Many2one vers `pos.order`
- `product_id` : Many2one vers `product.product`
- `product_uom_id` : Many2one vers `uom.uom`
- `qty` : Float (quantité)
- `price_unit` : Float (prix unitaire)
- `price_subtotal` : Float (sous-total HT)
- `price_subtotal_incl` : Float (sous-total TTC)
- `discount` : Float (remise %)
- `tax_ids` : Many2many vers `account.tax`
- `tax_ids_after_fiscal_position` : Taxes après position fiscale
- `customer_note` : Note client sur la ligne
- `pack_lot_ids` : Lots / numéros de série (si gérés)
- `refunded_qty` : Quantité remboursée

**Règles métier :**

1. **Prix :** Issu de la pricelist du POS ou du client, modifiable manuellement.
2. **Remise :** En % ou montant, appliquée avant taxes.
3. **Taxes :** Selon position fiscale et produit.
4. **Arrondi caisse :** Possibilité d'arrondi global (cash rounding) sur la commande.

### 1.4 Modèle `pos.payment` (Paiement POS)

**Rôle :** Un paiement (montant + méthode) sur une commande POS.

**Champs clés :**

- `pos_order_id` : Many2one vers `pos.order`
- `payment_method_id` : Many2one vers `pos.payment.method`
- `amount` : Montant
- `name` : Libellé (ex. date/heure)
- `is_partial` : Boolean (acompte)
- `statement_line_id` : Many2one vers `account.bank.statement.line` (écriture caisse/carte)

**Règles métier :**

1. **Plusieurs paiements par commande :** Ex. partie espèces + partie carte.
2. **Rapprochement :** Chaque paiement alimente un journal (caisse ou bancaire).
3. **Clôture session :** Les lignes de relevé sont regroupées par méthode pour contrôle.

### 1.5 Modèle `pos.config` (Configuration Point de Vente)

**Rôle :** Définit un point de vente (nom, journal de caisse, méthodes de paiement, pricelist, etc.).

**Champs clés :**

- `name` : Nom du POS
- `journal_id` : Many2one vers `account.journal` (journal de caisse)
- `invoice_journal_id` : Journal pour factures POS
- `pricelist_id` : Many2one vers `product.pricelist`
- `payment_method_ids` : Many2many vers `pos.payment.method`
- `company_id` : Société
- `use_pricelist` : Boolean
- `fiscal_position_ids` : Positions fiscales disponibles
- `tax_regime_selection` : Sélection régime fiscal
- Optionnels : imprimante ticket, balance, scan code-barres, programme fidélité, etc.

---

## 2. Workflows et Transitions d'État

### 2.1 Cycle de Vie Session

```
[POS Dashboard]
    ↓ New Session
[opening_control] → Saisie fonds de caisse → Open Session
    ↓
[opened] → Ventes (pos.order draft → paid → done)
    ↓ Menu → Close Session
[closing_control] → Comptage cash, validation écarts
    ↓ Close Session
[closed]
```

**Étapes détaillées :**

1. **Nouvelle session :** Clic "New Session" depuis le tableau de bord POS.
2. **Contrôle d'ouverture :** Saisie du montant d'ouverture de caisse (ou 0), clic "Open Session".
3. **Session ouverte :** Les commandes sont créées dans cette session.
4. **Clôture :** Menu → "Close Session" → écran de contrôle (nombre de commandes, totaux par moyen de paiement, montant espèces attendu).
5. **Comptage :** Saisie du cash compté (billets/pièces), calcul de l'écart. Possibilité de clôturer malgré écart (selon paramétrage).
6. **Clôture effective :** "Close Session" → state = closed, génération des écritures comptables et stock.

### 2.2 Cycle de Vie Commande POS

```
[Panier vide]
    ↓ Ajout produits
[Draft] (lignes, remises, client optionnel)
    ↓ Payment
[Paid] (paiements >= total)
    ↓ Validate / Print
[Done] (stock mis à jour, ticket imprimé)
    ↓ Option : Create Invoice
[Invoiced] (facture générée)
```

**Retours :**

- Actions → Refund → Sélection de la commande d'origine → Sélection des lignes / quantités à rembourser → Paiement (méthode remboursement) → Validation → Avoir + crédit stock si applicable.

---

## 3. Calculs et Algorithmes

### 3.1 Calcul des Totaux Commande

- **price_subtotal (ligne) :** `qty * price_unit * (1 - discount/100)` (HT).
- **price_subtotal_incl :** Idem avec taxes incluses (selon régime).
- **amount_untaxed :** Somme des `price_subtotal` des lignes.
- **amount_tax :** Somme des taxes calculées par ligne.
- **amount_total :** `amount_untaxed + amount_tax` (ou arrondi caisse si activé).

### 3.2 Cash Rounding

- Option "Cash rounding" : arrondir le total à la pièce légale (ex. 5 centimes).
- Méthode : arrondi au plus proche selon règle configurée ; une ligne d'ajustement (taxe ou produit fictif) peut être ajoutée.

### 3.3 Pricelist et Position Fiscale

- **Prix :** Depuis `pos.config.pricelist_id` ou `partner_id.property_product_pricelist` si client renseigné.
- **Taxes :** `product_id.taxes_id` mappé par `fiscal_position_id` (pos.order ou config).

### 3.4 Stock

- À la validation de la commande (state → done), création de mouvements de stock (sortie) pour chaque ligne, vers l'entrepôt configuré pour le POS.

---

## 4. Règles Métier Spécifiques

### 4.1 Caisse

- **Cash In / Cash Out :** En session ouverte, l'utilisateur peut enregistrer des entrées ou sorties de caisse (avec motif), sans lien avec une vente.
- **Clôture :** Les totaux par méthode de paiement sont comparés aux lignes de relevé ; pour les espèces, comparaison avec le montant compté.

### 4.2 Facturation

- Si "To invoice" coché ou client demandant une facture : création d'un `account.move` (facture client), lien `pos.order.account_move`.
- La facture reprend les lignes du ticket (produits, quantités, prix, taxes).

### 4.3 Multi-Employés

- Plusieurs employés peuvent être connectés à la même session (employee login).
- Chaque commande peut être associée à un employé (`employee_id`) pour rapports et commissions.

### 4.4 Offline

- Le POS peut fonctionner en mode déconnecté ; les données sont synchronisées au retour de la connexion (commandes, paiements, session).

---

## 5. Intégrations avec Autres Modules

### 5.1 Stock (Inventory)

- Sortie de stock à la validation de la commande (pos.order → done).
- Retours : entrée de stock via commande de remboursement.

### 5.2 Comptabilité (Accounting)

- Journal de caisse : chaque paiement génère une ligne de relevé bancaire (`account.bank.statement.line`).
- Facture : génération d'un `account.move` (facture) depuis la commande POS.
- Clôture de session : validation du relevé de caisse, écritures comptables associées.

### 5.3 Sales

- Création de devis/commandes Sales depuis le POS (Quotations/Orders) : commande Sales payée via POS.
- Import de commandes Sales dans le POS pour encaissement et facturation.

### 5.4 Product

- Catalogue produits, variantes, UOM, taxes, code-barres (recherche par scan).
- Pricelist et règles de prix.

### 5.5 Contacts

- Client (partner) optionnel sur la commande : pricelist, adresse facturation, programme fidélité (si module loyalty).

---

## 6. Considérations pour Miyukini COG

### 6.1 Architecture Opérateurs

**Opérateurs proposés :**
1. **PosSession** : Gestion des sessions de caisse (ouverture, clôture, contrôle).
2. **PosOrder** : Gestion des commandes (tickets) et lignes.
3. **PosPayment** : Gestion des paiements et rapprochement caisse.
4. **PosConfig** : Configuration des points de vente.
5. **PosUI** : Interface utilisateur POS (écran de vente, clavier, paiement).

### 6.2 Gouvernance COG

- **StrongFather :** Autorisation d'ouverture/clôture de session, validation des écarts de caisse, facturation.
- **KindMother :** Persistance des sessions, commandes, paiements (WriteIntent).
- **Master Butler :** Permissions par point de vente et par rôle (caissier, responsable).
- **WorrySentinel :** Niveau de sécurité 2–3 (données caisse et paiements).
- **Ever Buddy :** Cycle de vie session et commande (états, transitions).

### 6.3 Intégrations Miyukini

- **MiyuStore :** Produits, pricelist, taxes.
- **MiyuInvoice :** Génération de factures depuis commandes POS.
- **MiyuContacts :** Clients.
- **MiyuClock :** Horodatage session et commandes.
- **MiyuTreasury / MiyuBilling :** Paiements et rapprochement (selon périmètre).

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
