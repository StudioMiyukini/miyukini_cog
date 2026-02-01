# Odoo Sales — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Sales dans Miyukini.

**Date :** 2026-02-01

---

## Portée / Scope

**Ce document couvre :**
- Architecture technique détaillée
- Spécifications des crates Rust
- Schémas de données
- API et contrats
- Plan de développement par phases
- Bornage fonctionnel (MVP → Complet)

---

## 1. Architecture Technique

### 1.1 Structure des Crates

```
crates/
├── miyukini-sales-order/        # SalesOrder Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── order.rs              # Modèle Order
│   │   ├── state.rs              # États et transitions
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-sales-line/          # SalesOrderLine Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── line.rs               # Modèle Line
│   │   ├── pricing.rs            # Calculs prix
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-sales-pricelist/      # SalesPricelist Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── pricelist.rs          # Modèle Pricelist
│   │   ├── rules.rs              # Règles de prix
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-sales-invoice/        # SalesInvoice Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── invoice.rs            # Génération factures
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-sales-payment/        # SalesPayment Opérateur
    ├── src/
    │   ├── lib.rs
    │   ├── payment.rs            # Gestion paiements
    │   ├── signature.rs          # Signatures
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores

**Kits existants :**
- `miyuinvoice` : Facturation
- `miyustore` : Produits et catalogues
- `miyucontacts` : Contacts/clients
- `miyuclock` : Dates

---

## 2. Schémas de Données

### 2.1 Modèle Order

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub name: String,
    pub state: OrderState,
    pub locked: bool,
    
    // Client
    pub partner_id: PartnerId,
    pub partner_invoice_id: PartnerId,
    pub partner_shipping_id: PartnerId,
    
    // Équipe
    pub team_id: Option<TeamId>,
    pub user_id: Option<UserId>,
    pub company_id: CompanyId,
    
    // Dates
    pub create_date: DateTime<Utc>,
    pub date_order: DateTime<Utc>,
    pub commitment_date: Option<DateTime<Utc>>,
    pub validity_date: Option<NaiveDate>,
    
    // Prix
    pub pricelist_id: PricelistId,
    pub currency_id: CurrencyId,
    pub amount_untaxed: Decimal,
    pub amount_tax: Decimal,
    pub amount_total: Decimal,
    
    // Facturation
    pub invoice_ids: Vec<InvoiceId>,
    pub invoice_status: InvoiceStatus,
    
    // Paiement
    pub require_signature: bool,
    pub require_payment: bool,
    pub prepayment_percent: f64,
    pub signature: Option<Image>,
    pub amount_paid: Decimal,
}
```

---

## 3. Plan de Développement

### 3.1 Phase 1 : MVP

**Durée estimée :** 4-6 semaines

**Fonctionnalités :**
- Création/modification de devis
- Ajout de lignes produits
- Calcul des prix basique
- Confirmation de commande
- Génération facture simple

**Crates :**
- `miyukini-sales-order` (MVP)
- `miyukini-sales-line` (MVP)

### 3.2 Phase 2 : Fonctionnalités Essentielles

**Durée estimée :** 6-8 semaines

**Fonctionnalités :**
- Pricelist complète
- Calcul taxes avancé
- Gestion quantités (livrées, facturées)
- Paiement en ligne
- Signature en ligne

**Crates :**
- `miyukini-sales-pricelist` (complet)
- `miyukini-sales-payment` (MVP)

### 3.3 Phase 3 : Fonctionnalités Avancées

**Durée estimée :** 6-8 semaines

**Fonctionnalités :**
- Groupement de factures
- Acomptes
- Remises globales
- Intégrations complètes

**Crates :**
- `miyukini-sales-invoice` (complet)
- `miyukini-sales-payment` (complet)

---

## 4. Bornage Fonctionnel

### 4.1 MVP (Phase 1)

**Inclus :**
- CRUD devis/commandes basique
- Lignes produits simples
- Calcul prix basique (sans pricelist complexe)
- Confirmation manuelle
- Génération facture simple

**Exclu :**
- Pricelist avancée
- Calcul taxes complexe
- Paiement en ligne
- Signature en ligne
- Acomptes
- Groupement factures

### 4.2 Complet (Phase 3)

**Inclus :**
- Toutes les fonctionnalités Odoo Sales identifiées
- Pricelist complète
- Calcul taxes avancé
- Paiement et signature en ligne
- Acomptes
- Groupement factures
- Intégrations complètes

---

## 5. Critères d'Acceptation

### 5.1 Fonctionnels

- [ ] Création devis fonctionnelle
- [ ] Ajout lignes produits fonctionnel
- [ ] Calcul prix correct
- [ ] Confirmation commande fonctionnelle
- [ ] Génération facture fonctionnelle
- [ ] Intégration CRM fonctionnelle
- [ ] Intégration Account fonctionnelle

### 5.2 Techniques

- [ ] Toutes les écritures passent par WriteIntent
- [ ] Toutes les décisions passent par StrongFather
- [ ] Permissions vérifiées via Master Butler
- [ ] Sécurité vérifiée via WorrySentinel
- [ ] Cycle de vie géré via Ever Buddy

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
