# Odoo Invoicing — Guide d'Implémentation

## Contexte

Ce document fournit un **guide d'implémentation technique** pour développer l'équivalent **Invoicing** dans Miyukini : architecture technique, schémas de données, API, plan de développement par phases et bornage fonctionnel.

**Références :**
- [Logique Métier](../00_logique_metier/Odoo%20Invoicing%20-%20Logique%20Metier%20Complete.md)
- [Spécifications Opérateurs Miyukini](../04_specifications_miyukini/Odoo%20Invoicing%20-%20Specifications%20Operateurs%20Miyukini.md)
- [Guide Intégration COG](../05_integration_cog/Odoo%20Invoicing%20-%20Guide%20Integration%20COG.md)

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique (crates Rust, périmètre Invoicing)
- Schémas de données (facture, ligne, paiement, conditions de paiement)
- API et contrats (InvoiceLedger, InvoicePayment, InvoiceSend, InvoiceTerms)
- Plan de développement par phases (MVP → Complet)
- Bornage fonctionnel et critères d'acceptation

**Hors scope :**
- Implémentation complète du code (sera dans les crates)
- Grand livre et rapprochement bancaire (voir Accounting)

---

## 1. Architecture Technique

### 1.1 Structure des Crates (Périmètre Invoicing)

```
crates/
├── miyuinvoice/                      # Existant — Kits facturation
│   ├── src/
│   │   ├── lib.rs
│   │   ├── lines.rs                  # Calcul lignes, taxes
│   │   ├── totals.rs                 # HT, TTC, résiduel
│   │   └── pdf.rs                     # Génération PDF
│   └── Cargo.toml
│
├── miyukini-invoice-ledger/           # InvoiceLedger Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── invoice.rs                 # Modèle Invoice
│   │   ├── line.rs                    # Modèle InvoiceLine
│   │   ├── validation.rs              # Équilibre, validation
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-invoice-payment/         # InvoicePayment Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── payment.rs                 # Modèle Payment
│   │   ├── reconciliation.rs         # Réconciliation facture/paiement
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-invoice-send/             # InvoiceSend Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── send.rs                    # Envoi email, PDF
│   │   ├── portal.rs                  # URL portail (optionnel)
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-invoice-terms/            # InvoiceTerms Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── payment_term.rs            # Modèle PaymentTerm
│   │   ├── due_dates.rs               # Calcul échéances
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-invoice-ui/               # InvoiceUI (Opérateur Interface)
    ├── src/
    │   ├── lib.rs
    │   ├── views.rs                   # Listes, formulaires
    │   ├── wizards.rs                 # Wizard paiement
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Id, Logger, Clock
- `miyukini-central` : StrongFather, KindMother, Master Butler, WorrySentinel, Ever Buddy

**Kits existants :**
- `miyuinvoice` : Calculs facturation, PDF
- Optionnel : `miyucptaledger` si réconciliation avancée

**Externes :**
- `serde`, `chrono`, `rust_decimal`, `uuid`
- Optionnel : librairie email (envoi), génération PDF

---

## 2. Schémas de Données

### 2.1 Modèle Invoice (Facture)

```rust
// miyukini-invoice-ledger/src/invoice.rs

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: InvoiceId,
    pub name: Option<String>,              // Numéro (séquence, après validation)
    pub move_type: InvoiceType,
    pub state: InvoiceState,
    pub partner_id: PartnerId,
    pub invoice_date: NaiveDate,
    pub invoice_date_due: NaiveDate,
    pub payment_term_id: Option<PaymentTermId>,
    pub journal_id: JournalId,
    pub line_ids: Vec<LineId>,
    pub amount_untaxed: Decimal,
    pub amount_tax: Decimal,
    pub amount_total: Decimal,
    pub amount_residual: Decimal,
    pub payment_state: PaymentState,
    pub currency_id: CurrencyId,
    pub company_id: CompanyId,
    pub sent: bool,                        // Marquer envoyé
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceType {
    OutInvoice,    // Facture client
    InInvoice,     // Facture fournisseur
    OutRefund,     // Avoir client
    InRefund,      // Avoir fournisseur
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceState {
    Draft,
    Posted,
    Cancel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentState {
    NotPaid,
    InPayment,
    Paid,
    Partial,
    Reversed,
    Blocked,
}

impl Invoice {
    pub fn is_balanced(&self) -> bool {
        // SUM(debit) = SUM(credit) sur les lignes (lecture KindMother)
        true
    }
}
```

### 2.2 Modèle InvoiceLine (Ligne de Facture)

```rust
// miyukini-invoice-ledger/src/line.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub id: LineId,
    pub invoice_id: InvoiceId,
    pub display_type: LineDisplayType,
    pub product_id: Option<ProductId>,
    pub name: String,
    pub quantity: Decimal,
    pub price_unit: Decimal,
    pub discount: Decimal,
    pub price_subtotal: Decimal,
    pub price_total: Decimal,
    pub account_id: AccountId,
    pub tax_ids: Vec<TaxId>,
    pub debit: Decimal,
    pub credit: Decimal,
    pub amount_residual: Decimal,
    pub date_maturity: Option<NaiveDate>,
    pub reconciled: bool,
    pub sequence: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineDisplayType {
    Product,
    Tax,
    PaymentTerm,
    LineSection,
    LineNote,
}
```

### 2.3 Modèle Payment (Paiement)

```rust
// miyukini-invoice-payment/src/payment.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: PaymentId,
    pub payment_type: PaymentType,        // Inbound (client), Outbound (fournisseur)
    pub partner_type: PartnerType,       // Customer, Supplier
    pub amount: Decimal,
    pub currency_id: CurrencyId,
    pub payment_date: NaiveDate,
    pub journal_id: JournalId,
    pub state: PaymentState,
    pub reconciled_invoice_ids: Vec<InvoiceId>,
    pub company_id: CompanyId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### 2.4 Modèle PaymentTerm (Conditions de Paiement)

```rust
// miyukini-invoice-terms/src/payment_term.rs

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTerm {
    pub id: PaymentTermId,
    pub name: String,
    pub line_ids: Vec<PaymentTermLineId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTermLine {
    pub id: PaymentTermLineId,
    pub value: PaymentTermValue,    // Percent, Fixed, Balance
    pub value_amount: Decimal,
    pub nb_days: u32,
    pub discount_percentage: Option<Decimal>,
    pub discount_days: Option<u32>,
}
```

---

## 3. API et Contrats

### 3.1 InvoiceLedger API

```rust
pub async fn create_invoice(ctx: &OperatorContext, draft: InvoiceDraft) -> Result<InvoiceId, InvoiceError>;
pub async fn update_invoice(ctx: &OperatorContext, invoice_id: InvoiceId, draft: InvoiceDraft) -> Result<(), InvoiceError>;
pub async fn validate_invoice(ctx: &OperatorContext, invoice_id: InvoiceId) -> Result<ValidatedInvoice, InvoiceError>;
pub async fn cancel_invoice(ctx: &OperatorContext, invoice_id: InvoiceId) -> Result<(), InvoiceError>;
pub async fn create_refund(ctx: &OperatorContext, invoice_id: InvoiceId, options: RefundOptions) -> Result<InvoiceId, InvoiceError>;
pub async fn get_invoice(ctx: &OperatorContext, invoice_id: InvoiceId) -> Result<Invoice, InvoiceError>;
pub async fn list_invoices(ctx: &OperatorContext, filters: InvoiceFilters) -> Result<Vec<Invoice>, InvoiceError>;
pub async fn add_line(ctx: &OperatorContext, invoice_id: InvoiceId, line: LineDraft) -> Result<LineId, InvoiceError>;
pub async fn remove_line(ctx: &OperatorContext, invoice_id: InvoiceId, line_id: LineId) -> Result<(), InvoiceError>;
```

### 3.2 InvoicePayment API

```rust
pub async fn record_payment(
    ctx: &OperatorContext,
    payment: PaymentDraft,
    invoice_ids: Vec<InvoiceId>,
    allocation: Option<Vec<Decimal>>,
) -> Result<PaymentId, InvoiceError>;
pub async fn get_payment(ctx: &OperatorContext, payment_id: PaymentId) -> Result<Payment, InvoiceError>;
pub async fn list_payments(ctx: &OperatorContext, filters: PaymentFilters) -> Result<Vec<Payment>, InvoiceError>;
pub async fn reconcile(ctx: &OperatorContext, payment_id: PaymentId, invoice_ids: Vec<InvoiceId>) -> Result<(), InvoiceError>;
```

### 3.3 InvoiceSend API

```rust
pub async fn send_invoice(ctx: &OperatorContext, invoice_id: InvoiceId, options: SendOptions) -> Result<(), InvoiceError>;
pub async fn render_pdf(ctx: &OperatorContext, invoice_id: InvoiceId) -> Result<Vec<u8>, InvoiceError>;
pub async fn mark_sent(ctx: &OperatorContext, invoice_id: InvoiceId) -> Result<(), InvoiceError>;
pub async fn portal_url(ctx: &OperatorContext, invoice_id: InvoiceId) -> Result<String, InvoiceError>;
```

### 3.4 InvoiceTerms API

```rust
pub async fn get(ctx: &OperatorContext, term_id: PaymentTermId) -> Result<PaymentTerm, InvoiceError>;
pub async fn list(ctx: &OperatorContext, filters: PaymentTermFilters) -> Result<Vec<PaymentTerm>, InvoiceError>;
pub async fn compute_due_dates(
    ctx: &OperatorContext,
    term_id: PaymentTermId,
    invoice_date: NaiveDate,
    amount_total: Decimal,
) -> Result<Vec<(NaiveDate, Decimal)>, InvoiceError>;
pub async fn create(ctx: &OperatorContext, draft: PaymentTermDraft) -> Result<PaymentTermId, InvoiceError>;
pub async fn update(ctx: &OperatorContext, term_id: PaymentTermId, draft: PaymentTermDraft) -> Result<(), InvoiceError>;
```

---

## 4. Plan de Développement par Phases

### 4.1 Phase 1 : MVP (Minimum Viable Product)

**Objectif :** Facturation de base (création, validation, envoi, paiement simple).

**Fonctionnalités incluses :**
- [ ] Création de factures clients (brouillon)
- [ ] Lignes produit, taxes, totaux (via MiyuInvoice)
- [ ] Validation de facture (équilibre, séquence numéro)
- [ ] Génération PDF et envoi email
- [ ] Enregistrement d’un paiement sur une facture
- [ ] Réconciliation facture/paiement (montant résiduel, payment_state)
- [ ] Conditions de paiement simples (30 j, 45 j)
- [ ] Liste et fiche facture (InvoiceUI basique)

**Fonctionnalités exclues :**
- ❌ Factures fournisseurs (Phase 2)
- ❌ Avoirs (Phase 2)
- ❌ Réconciliation partielle multi-factures (Phase 2)
- ❌ Portail client (Phase 3)
- ❌ Intégration Sales/Purchase (Phase 2–3)

**Durée estimée :** 6–8 semaines

**Critères d'acceptation :**
- Création facture client avec lignes et taxes
- Validation → numéro de facture généré
- PDF généré et envoyé par email
- Paiement enregistré → facture marquée Payée
- Filtres liste : Brouillon / Validées / Payées

### 4.2 Phase 2 : Fonctionnalités Essentielles

**Objectif :** Factures fournisseurs, avoirs, réconciliation avancée, intégration Sales.

**Fonctionnalités incluses :**
- [ ] Factures fournisseurs (création, validation, paiement)
- [ ] Avoirs (création depuis facture, liaison)
- [ ] Réconciliation partielle (un paiement sur plusieurs factures, une facture avec plusieurs paiements)
- [ ] Intégration Miyukini Sales : factures depuis commandes confirmées
- [ ] Wizard paiement (sélection factures, répartition montant)
- [ ] Conditions de paiement avancées (échéancier %, acomptes)
- [ ] Filtres "À relancer" (échéance dépassée, amount_residual > 0)

**Durée estimée :** 4–6 semaines

**Critères d'acceptation :**
- Facture fournisseur créée et payée
- Avoir créé et lié à la facture d’origine
- Paiement réparti sur 2+ factures
- Facture créée depuis une commande Sales (lien bidirectionnel)

### 4.3 Phase 3 : Complet et Portail

**Objectif :** Portail client, intégration Purchase/Project, rapports, multi-devises (optionnel).

**Fonctionnalités incluses :**
- [ ] Portail client : consultation factures, téléchargement PDF, paiement en ligne (si activé)
- [ ] Intégration Purchase : factures fournisseur depuis commandes d’achat
- [ ] Intégration Project / Timesheet : facturation temps ou livrables
- [ ] Rapports : factures par période, encaissements, à relancer
- [ ] Export CSV/Excel des factures et paiements
- [ ] Multi-devises (taux de change, affichage) — optionnel

**Durée estimée :** 4–6 semaines

**Critères d'acceptation :**
- Client consulte ses factures sur le portail (Façade Publique Gouvernée)
- Facture fournisseur créée depuis commande Purchase
- Rapport "Factures du mois" et "À relancer" fonctionnels

---

## 5. Bornage Fonctionnel

### 5.1 Inclus (Périmètre Invoicing)

- Factures clients et fournisseurs (création, validation, annulation)
- Avoirs (création, liaison à la facture d’origine)
- Conditions de paiement et échéanciers
- Envoi email + PDF, marquer envoyé
- Enregistrement des paiements et réconciliation facture/paiement
- Liste et fiche facture, wizard paiement
- Intégration Sales (factures depuis commandes)
- Intégration Purchase (factures fournisseur depuis commandes)
- Portail client (consultation factures, PDF, paiement en ligne si activé)
- Rapports facturation (liste, à relancer, encaissements)

### 5.2 Exclu (Hors Périmètre Invoicing)

- Grand livre complet (voir Accounting)
- Rapprochement bancaire (voir Accounting)
- Plan comptable avancé et journaux multiples (configuration minimale Invoicing)
- Rapports comptables (bilan, compte de résultat, balance) — voir Accounting
- Gestion d’actifs et budgets — voir Accounting

---

## 6. Critères d'Acceptation Globaux

- Toute facture validée a un numéro unique (séquence Ever Buddy)
- Équilibre comptable vérifié avant validation (KindMother)
- Paiement enregistré met à jour amount_residual et payment_state des factures réconciliées
- Envoi email + PDF et marquer envoyé (WriteIntent KindMother)
- Mandats de Permission respectés (Master Butler, StrongFather)
- Niveaux de sécurité WorrySentinel respectés (données facturation et paiement)
- Réutilisation de MiyuInvoice pour calculs et PDF sans duplication de logique

---

## 7. Risques et Mitigation

| Risque | Mitigation |
|--------|------------|
| Chevauchement Invoicing / Accounting | Bornage clair : Invoicing = facturation ; Accounting = grand livre + rapprochement. Réutilisation des mêmes modèles facture (KindMother) si les deux sont présents. |
| Complexité réconciliation multi-factures | Algorithme de répartition explicite (allocation par facture), tests de non-régression. |
| Envoi email (délai, échec) | File d’attente ou retry ; marquer "envoyé" uniquement après succès ou après accusé selon politique. |
| Portail et sécurité | Façade Publique Gouvernée, Mandat Public d’Accès, pas d’accès aux Cores. |

---

## 8. Conclusion

Le **guide d’implémentation** Invoicing définit l’architecture des crates (InvoiceLedger, InvoicePayment, InvoiceSend, InvoiceTerms, InvoiceUI), les schémas de données (Invoice, InvoiceLine, Payment, PaymentTerm), les API et le **plan de développement en 3 phases** (MVP → Essentiel → Complet/Portail), avec un **bornage clair** par rapport à Accounting. L’implémentation s’appuie sur **MiyuInvoice** et les **Cores** pour la gouvernance et la persistance.

**Prochaines étapes :** Revue des spécifications avec l’équipe technique, mise en place des crates et développement Phase 1 (MVP).

---

**Document** : Odoo Invoicing — Guide d'Implémentation  
**Version** : 1.0  
**Date** : 2026-02-01  
**Statut** : Référence pour implémentation Miyukini
