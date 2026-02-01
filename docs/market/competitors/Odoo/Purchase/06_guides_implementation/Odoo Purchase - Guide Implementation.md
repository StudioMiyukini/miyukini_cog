# Odoo Purchase — Guide d'Implémentation avec Bornage

## Contexte

Ce document fournit un **guide d'implémentation technique complet** pour développer l'équivalent Purchase dans Miyukini.

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
├── miyukini-purchase-order/        # PurchaseOrder Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── order.rs                # Modèle Order
│   │   ├── state.rs                # États et transitions
│   │   ├── approval.rs             # Gestion approbations
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-purchase-line/         # PurchaseOrderLine Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── line.rs                 # Modèle Line
│   │   ├── pricing.rs              # Calculs prix
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-purchase-approval/     # PurchaseApproval Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── approval.rs             # Gestion approbations
│   │   ├── rules.rs                # Règles d'approbation
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
├── miyukini-purchase-invoice/      # PurchaseInvoice Opérateur
│   ├── src/
│   │   ├── lib.rs
│   │   ├── invoice.rs              # Génération factures
│   │   ├── matching.rs             # Bill Matching
│   │   └── admin_cell.rs
│   └── Cargo.toml
│
└── miyukini-purchase-reception/    # PurchaseReception Opérateur (si Inventory)
    ├── src/
    │   ├── lib.rs
    │   ├── reception.rs            # Gestion réceptions
    │   └── admin_cell.rs
    └── Cargo.toml
```

### 1.2 Dépendances Principales

**Cores Miyukini :**
- `miyukini-kernel` : Kernel
- `miyukini-central` : Cores

**Kits existants :**
- `miyuinvoice` : Facturation fournisseur
- `miyustore` : Produits et sellers
- `miyucontacts` : Contacts/fournisseurs
- `miyuclock` : Dates
- `miyunotify` : Notifications email
- `miyuinventory` : Gestion stock (si développé)

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
    pub priority: Priority,
    
    // Fournisseur
    pub partner_id: PartnerId,
    pub partner_ref: Option<String>,
    pub dest_address_id: Option<PartnerId>,
    
    // Dates
    pub date_order: DateTime,
    pub date_approve: Option<DateTime>,
    pub date_planned: Option<DateTime>,
    
    // Acheteur et entreprise
    pub user_id: UserId,
    pub company_id: CompanyId,
    pub company_currency_id: CurrencyId,
    
    // Devise et montants
    pub currency_id: CurrencyId,
    pub currency_rate: f64,
    pub amount_untaxed: Decimal,
    pub amount_tax: Decimal,
    pub amount_total: Decimal,
    pub amount_total_cc: Decimal,
    
    // Lignes
    pub lines: Vec<LineId>,
    
    // Facturation
    pub invoice_ids: Vec<InvoiceId>,
    pub invoice_count: u32,
    pub invoice_status: InvoiceStatus,
    
    // Conditions
    pub payment_term_id: Option<PaymentTermId>,
    pub fiscal_position_id: Option<FiscalPositionId>,
    pub incoterm_id: Option<IncotermId>,
    
    // Autres
    pub origin: Option<String>,
    pub acknowledged: bool,
    pub note: Option<String>,
    
    // Métadonnées
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.2 Modèle Line

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub id: LineId,
    pub order_id: OrderId,
    pub sequence: u32,
    pub display_type: Option<DisplayType>,
    
    // Produit
    pub product_id: Option<ProductId>,
    pub name: String,
    pub product_uom_id: UomId,
    pub product_qty: Decimal,
    pub product_uom_qty: Decimal,
    
    // Prix
    pub price_unit: Decimal,
    pub price_unit_product_uom: Decimal,
    pub price_unit_discounted: Decimal,
    pub discount: Decimal,
    pub price_subtotal: Decimal,
    pub price_total: Decimal,
    pub price_tax: Decimal,
    
    // Taxes
    pub tax_ids: Vec<TaxId>,
    
    // Dates
    pub date_planned: DateTime,
    
    // Quantités
    pub qty_received_method: QtyReceivedMethod,
    pub qty_received: Decimal,
    pub qty_received_manual: Decimal,
    pub qty_invoiced: Decimal,
    pub qty_to_invoice: Decimal,
    
    // Fournisseur
    pub selected_seller_id: Option<SellerId>,
    
    // Analytique
    pub analytic_distribution: Option<AnalyticDistribution>,
    
    // Facturation
    pub invoice_lines: Vec<InvoiceLineId>,
    
    // Métadonnées
    pub created_at: DateTime,
    pub updated_at: DateTime,
}
```

### 2.3 États et Transitions

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    Draft,
    Sent,
    ToApprove,
    Purchase,
    Cancel,
}

impl OrderState {
    pub fn can_transition_to(&self, target: OrderState) -> bool {
        match (self, target) {
            (OrderState::Draft, OrderState::Sent) => true,
            (OrderState::Draft, OrderState::ToApprove) => true,
            (OrderState::Draft, OrderState::Purchase) => true,
            (OrderState::Draft, OrderState::Cancel) => true,
            (OrderState::Sent, OrderState::ToApprove) => true,
            (OrderState::Sent, OrderState::Purchase) => true,
            (OrderState::Sent, OrderState::Cancel) => true,
            (OrderState::ToApprove, OrderState::Purchase) => true,
            (OrderState::ToApprove, OrderState::Cancel) => true,
            (OrderState::Purchase, OrderState::Cancel) => true,
            _ => false,
        }
    }
}
```

---

## 3. API et Contrats

### 3.1 API PurchaseOrder

```rust
pub trait PurchaseOrderOperator {
    // Création
    async fn create_rfq(
        &self,
        ctx: &OperatorContext,
        rfq: RFQDraft,
    ) -> Result<OrderId, PurchaseError>;
    
    // Modification
    async fn update_order(
        &self,
        ctx: &OperatorContext,
        order_id: OrderId,
        updates: OrderUpdates,
    ) -> Result<(), PurchaseError>;
    
    // Actions
    async fn send_rfq(
        &self,
        ctx: &OperatorContext,
        order_id: OrderId,
    ) -> Result<(), PurchaseError>;
    
    async fn confirm_order(
        &self,
        ctx: &OperatorContext,
        order_id: OrderId,
    ) -> Result<(), PurchaseError>;
    
    async fn cancel_order(
        &self,
        ctx: &OperatorContext,
        order_id: OrderId,
    ) -> Result<(), PurchaseError>;
    
    // Lecture
    async fn read_order(
        &self,
        ctx: &OperatorContext,
        order_id: OrderId,
    ) -> Result<Order, PurchaseError>;
    
    async fn list_orders(
        &self,
        ctx: &OperatorContext,
        filters: OrderFilters,
    ) -> Result<Vec<Order>, PurchaseError>;
}
```

### 3.2 API PurchaseOrderLine

```rust
pub trait PurchaseOrderLineOperator {
    // Création
    async fn create_line(
        &self,
        ctx: &OperatorContext,
        order_id: OrderId,
        line: LineDraft,
    ) -> Result<LineId, PurchaseError>;
    
    // Modification
    async fn update_line(
        &self,
        ctx: &OperatorContext,
        line_id: LineId,
        updates: LineUpdates,
    ) -> Result<(), PurchaseError>;
    
    // Calculs
    async fn compute_price(
        &self,
        ctx: &OperatorContext,
        line: &Line,
    ) -> Result<PriceDetails, PurchaseError>;
    
    async fn compute_amounts(
        &self,
        ctx: &OperatorContext,
        line: &Line,
    ) -> Result<Amounts, PurchaseError>;
}
```

---

## 4. Plan de Développement par Phases

### Phase 1 : MVP (Minimum Viable Product)

**Objectif :** Création et gestion basique des RFQ/commandes

**Fonctionnalités :**
- ✅ Création RFQ
- ✅ Ajout lignes produits
- ✅ Calcul montants (HT, TTC, taxes)
- ✅ Envoi RFQ (email)
- ✅ Confirmation commande
- ✅ Annulation commande

**Opérateurs :**
- PurchaseOrder (basique)
- PurchaseOrderLine (basique)
- PurchaseUI (basique)

**Intégrations :**
- MiyuContacts (fournisseurs)
- MiyuStore (produits)
- MiyuNotify (emails)

**Durée estimée :** 4-6 semaines

### Phase 2 : Approbations

**Objectif :** Système d'approbation des commandes

**Fonctionnalités :**
- ✅ Double validation
- ✅ Règles d'approbation
- ✅ Notifications approbation
- ✅ Historique approbations

**Opérateurs :**
- PurchaseApproval

**Durée estimée :** 2-3 semaines

### Phase 3 : Facturation

**Objectif :** Génération factures fournisseur depuis commandes

**Fonctionnalités :**
- ✅ Génération factures depuis commandes
- ✅ Lien bidirectionnel commande ↔ facture
- ✅ Synchronisation montants
- ✅ Bill Matching

**Opérateurs :**
- PurchaseInvoice

**Intégrations :**
- MiyuInvoice

**Durée estimée :** 3-4 semaines

### Phase 4 : Réceptions (si Inventory)

**Objectif :** Gestion des réceptions de produits

**Fonctionnalités :**
- ✅ Création réceptions depuis commandes
- ✅ Validation quantités reçues
- ✅ Synchronisation avec Inventory

**Opérateurs :**
- PurchaseReception

**Intégrations :**
- MiyuInventory

**Durée estimée :** 2-3 semaines

### Phase 5 : Fonctionnalités Avancées

**Objectif :** Fonctionnalités avancées et optimisations

**Fonctionnalités :**
- ✅ Fusion RFQ
- ✅ Comparaison prix
- ✅ Rappels réception
- ✅ Portail fournisseur
- ✅ Analytics et rapports
- ✅ Optimisations performance

**Durée estimée :** 4-6 semaines

---

## 5. Bornage Fonctionnel

### 5.1 MVP (Phase 1)

**Inclus :**
- Création/modification RFQ
- Ajout lignes produits
- Calcul montants
- Envoi RFQ
- Confirmation commande
- Annulation commande

**Exclu :**
- Approbations
- Facturation
- Réceptions
- Portail fournisseur
- Analytics

### 5.2 Version Complète (Phases 1-5)

**Inclus :**
- Toutes les fonctionnalités MVP
- Approbations
- Facturation
- Réceptions (si Inventory)
- Portail fournisseur
- Analytics et rapports
- Optimisations

---

## 6. Considérations Techniques

### 6.1 Performance

**Optimisations :**
- Cache des sellers produits
- Lazy loading des lignes
- Pagination intelligente
- Index base de données

### 6.2 Sécurité

**Mesures :**
- Validation données côté serveur
- Chiffrement données sensibles
- Audit trail complet
- Isolation cross-équipe

### 6.3 Scalabilité

**Architecture :**
- Opérateurs indépendants
- Communication asynchrone
- Cache distribué
- Base de données partitionnée

---

**Document créé le :** 2026-02-01  
**Dernière mise à jour :** 2026-02-01
