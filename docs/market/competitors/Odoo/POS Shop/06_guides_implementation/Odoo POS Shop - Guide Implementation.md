# Odoo POS Shop — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent POS Shop dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée (crates Rust)
- Schémas de données (Session, Order, OrderLine, Payment)
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyupossales/                 # PosOrder + PosSession (cœur métier)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── session.rs            # Modèle Session, états
│   │   ├── order.rs               # Modèle Order, OrderLine
│   │   ├── state.rs               # États session et order
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyupospayment/               # PosPayment
│   ├── src/
│   │   ├── lib.rs
│   │   ├── payment.rs             # Paiements, rapprochement
│   │   ├── method.rs              # Méthodes de paiement
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyuposconfig/                # PosConfig (optionnel, peut être dans miyupossales)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── config.rs              # Configuration POS
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-central/             # Intégration PosUI (frontend)
    └── (écrans POS dans l'app centrale ou module dédié)
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores

**Kits existants :**
- `miyustore` : Produits, pricelist, code-barres
- `miyuinvoice` : Facturation
- `miyucontacts` : Clients
- `miyuclock` : Dates
- `miyutreasury` ou `miyubilling` : Relevés de caisse, journaux (selon périmètre)

---

## 2. Schémas de Données

### 2.1 Modèle Session

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosSession {
    pub id: SessionId,
    pub name: String,
    pub state: SessionState,
    pub config_id: PosConfigId,
    pub user_id: UserId,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub opening_balance: Decimal,
    pub closing_balance: Option<Decimal>,
    pub counted_balance: Option<Decimal>,
    pub difference: Option<Decimal>,
    pub order_count: u32,
    pub amount_total: Decimal,
    pub statement_id: Option<StatementId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionState {
    OpeningControl,
    Opened,
    ClosingControl,
    Closed,
}
```

### 2.2 Modèle Order

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosOrder {
    pub id: OrderId,
    pub name: String,
    pub pos_reference: String,
    pub session_id: SessionId,
    pub sequence_number: u32,
    pub state: OrderState,
    pub partner_id: Option<PartnerId>,
    pub config_id: PosConfigId,
    pub pricelist_id: PricelistId,
    pub fiscal_position_id: Option<FiscalPositionId>,
    pub lines: Vec<PosOrderLine>,
    pub amount_untaxed: Decimal,
    pub amount_tax: Decimal,
    pub amount_total: Decimal,
    pub amount_paid: Decimal,
    pub amount_return: Decimal,
    pub currency_id: CurrencyId,
    pub invoice_id: Option<InvoiceId>,
    pub is_invoiced: bool,
    pub created_at: DateTime<Utc>,
    pub employee_id: Option<UserId>,
    pub customer_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderState {
    Draft,
    Paid,
    Done,
    Invoiced,
    Cancel,
}
```

### 2.3 Modèle OrderLine

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosOrderLine {
    pub id: OrderLineId,
    pub order_id: OrderId,
    pub product_id: ProductId,
    pub product_uom_id: UomId,
    pub qty: f64,
    pub price_unit: Decimal,
    pub discount: f64,
    pub price_subtotal: Decimal,
    pub price_subtotal_incl: Decimal,
    pub tax_ids: Vec<TaxId>,
    pub customer_note: Option<String>,
}
```

### 2.4 Modèle Payment

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosPayment {
    pub id: PaymentId,
    pub order_id: OrderId,
    pub payment_method_id: PaymentMethodId,
    pub amount: Decimal,
    pub name: String,
    pub is_partial: bool,
    pub statement_line_id: Option<StatementLineId>,
}
```

### 2.5 Modèle PosConfig

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosConfig {
    pub id: PosConfigId,
    pub name: String,
    pub journal_id: JournalId,
    pub invoice_journal_id: Option<JournalId>,
    pub pricelist_id: PricelistId,
    pub payment_method_ids: Vec<PaymentMethodId>,
    pub company_id: CompanyId,
}
```

---

## 3. Plan de Développement

### 3.1 Phase 1 : MVP

**Durée estimée :** 6–8 semaines

**Fonctionnalités :**
- Ouverture / clôture de session (fond de caisse, comptage)
- Création de commandes (lignes produits, quantités, prix, remise)
- Calcul des totaux (HT, TTC, taxes)
- Paiement (une ou plusieurs méthodes)
- Validation de commande (done) et sortie de stock basique
- Nouvelle commande (New Order)
- Interface POS minimale (liste produits, panier, paiement)

**Crates :**
- `miyupossales` (session + order + config minimal)
- `miyupospayment` (paiements, rapprochement caisse)
- Intégration MiyuStore (produits, prix)
- Intégration stock (sorties à la vente)

### 3.2 Phase 2 : Fonctionnalités Essentielles

**Durée estimée :** 4–6 semaines

**Fonctionnalités :**
- Client sur la commande (MiyuContacts)
- Facturation depuis le ticket (MiyuInvoice)
- Retours / remboursements (commande avoir, entrée stock)
- Cash In / Cash Out
- Multi-méthodes de paiement
- Rapport de session (commandes, totaux par méthode)

**Crates :**
- `miyupossales` (refund, invoice link)
- `miyupospayment` (multi-méthodes, écarts)
- Intégration MiyuInvoice, MiyuContacts

### 3.3 Phase 3 : Fonctionnalités Avancées

**Durée estimée :** 4–6 semaines

**Fonctionnalités :**
- Multi-employés (employee login, attribution commande)
- Code-barres / scan (recherche produit)
- Cash rounding
- Position fiscale (taxes par client / zone)
- Intégration Sales (encaissement commandes Sales, création commandes depuis POS)
- Mode offline et synchronisation
- Imprimante ticket (si périmètre matériel)

**Crates :**
- Extensions `miyupossales` (fiscal position, loyalty si module)
- Intégration Miyukini Sales
- Module offline (file de commandes, sync)

---

## 4. Bornage Fonctionnel

### 4.1 MVP (Phase 1)

**Inclus :**
- Session : ouverture, clôture, contrôle d’ouverture et de clôture
- Commande : lignes, totaux, remise, paiement, validation
- Sortie de stock à la validation
- Interface : produits, panier, paiement, nouvelle commande

**Exclu :**
- Client et facturation
- Retours / remboursements
- Cash In / Cash Out
- Multi-employés, code-barres, offline

### 4.2 Complet (Phase 3)

**Inclus :**
- Toutes les fonctionnalités Odoo POS Shop identifiées
- Client, facturation, retours
- Cash In/Out, multi-méthodes, écarts de caisse
- Intégration Sales, mode offline, options matériel (imprimante, scan)

---

## 5. Critères d'Acceptation

### 5.1 Fonctionnels

- [ ] Ouverture de session fonctionnelle (fond de caisse)
- [ ] Création et validation de commande fonctionnelles
- [ ] Paiement (une ou plusieurs méthodes) fonctionnel
- [ ] Clôture de session avec contrôle des montants fonctionnelle
- [ ] Sortie de stock à la validation fonctionnelle
- [ ] Client et facturation (Phase 2) fonctionnels
- [ ] Retours / remboursements (Phase 2) fonctionnels

### 5.2 Techniques

- [ ] Toutes les écritures passent par WriteIntent
- [ ] Décisions (ouverture, clôture, validation, facturation) passent par StrongFather
- [ ] Permissions vérifiées via Master Butler (par POS et rôle)
- [ ] Sécurité caisse et paiements vérifiée via WorrySentinel
- [ ] Cycle de vie session et commande géré via Ever Buddy

---

## 6. Correspondance Miyukini

**Service Miyukini proposé :**
- **MiyuPosSales** (ou **Miyukini PosShop**) : Opérateurs PosSession, PosOrder, PosPayment, PosConfig
- **MiyuPosPayment** : Paiements POS et rapprochement caisse (ou intégré dans miyupossales / miyutreasury selon choix d’architecture)

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
