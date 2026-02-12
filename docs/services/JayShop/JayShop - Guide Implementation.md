# JayShop — Guide Implementation

## Contexte

Ce document est le **guide d'implementation** du service **JayShop**. Il detaille l'architecture technique, la structure des crates, le schema de persistance, les Operateurs et Kits d'Outils, le plan de livraison par phases et l'integration dans Miyukini Central.

**References** : [Document fondateur](./JayShop%20-%20Document%20Fondateur.md), [Analyse des besoins](./JayShop%20-%20Analyse%20des%20besoins.md), [Ecrans et UI](./JayShop%20-%20Ecrans%20et%20UI.md), [Interfaces Inter-Services](./JayShop%20-%20Interfaces%20Inter-Services.md), [Reference Loyverse](./reference/JayShop%20-%20Reference%20Loyverse%20Back%20Office.md).

## Portee / Scope

- **Perimetre** : Architecture crate, schema de persistance KindMother, Operateurs et Kits, plan d'implementation par phases, integration Central et portail, regles de travail.
- **Hors perimetre** : Specifications UI pixel-perfect, implementation du portail web client (Phase 2+).

---

## 1. Architecture technique

### 1.1 Vue d'ensemble

```
┌──────────────────────────────────────────────────────────────────┐
│                      MIYUKINI CENTRAL (app)                      │
│  ┌─────────────────┐  ┌─────────────────┐  ┌────────────────┐  │
│  │ JayShop UI      │  │ JayXpose UI     │  │ JayKonta UI    │  │
│  │ (Dioxus views)  │  │ (Dioxus views)  │  │ (Dioxus views) │  │
│  └────────┬────────┘  └────────┬────────┘  └───────┬────────┘  │
│           │                     │                    │           │
│  ┌────────▼──────────────────────────────────────────▼────────┐ │
│  │                  ServiceConnections                         │ │
│  │  jayshop: Arc<JayShopDb>   (via kindmother-client)          │ │
│  │  jayxpose: Arc<JayXposeDb> (via kindmother-client)          │ │
│  │  jaykonta: Arc<JayKontaDb> (via kindmother-client)          │ │
│  └──────────────────────┬──────────────────────────────────────┘ │
└─────────────────────────┼────────────────────────────────────────┘
                          │ JSON/TCP (localhost:50051)
┌─────────────────────────▼────────────────────────────────────────┐
│                 KINDMOTHER SERVICE (processus isole)              │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │  Arbitration Engine (acces + validation WriteIntents)    │    │
│  └──────────────────────┬──────────────────────────────────┘    │
│  ┌──────────────────────▼──────────────────────────────────┐    │
│  │  libSQL Engine (fork Turso)                              │    │
│  │  Chiffrement AES-256-CBC natif                           │    │
│  │  Cles derivees par Argon2id                              │    │
│  │                                                          │    │
│  │  jayshop.db  │  jayxpose.db  │  jaykonta.db  │  ...     │    │
│  └──────────────────────────────────────────────────────────┘    │
└──────────────────────────────────────────────────────────────────┘
```

**Principe** : Aucun operateur n'a d'acces direct aux fichiers DB. Toute lecture passe par `client.query()`, toute ecriture par `client.execute()` avec un WriteIntent trace et audite.

### 1.2 Flux de donnees principal

```
[Admin / PoS]
    ↓ saisie ticket
[JayShop UI] (apps/central/src/services/jayshop/)
    ↓ appel service
[JayShopDb] (crates/jayshop/src/data/kindmother_client_db.rs)
    ↓ JSON/TCP → KindMother Service → libSQL chiffre (jayshop.db)
    ↓ sync stock → [JayXposeDb] → KindMother → jayxpose.db
    ↓ transmission comptable → [JayKontaDb] → KindMother → jaykonta.db
```

---

## 2. Structure du crate `jayshop`

### 2.1 Arborescence

```
crates/jayshop/
├── Cargo.toml
└── src/
    ├── lib.rs                    # Racine du module + API publique
    ├── data/
    │   ├── mod.rs                # Exports publics (pub use types::*, kindmother_client_db::*)
    │   ├── types.rs              # Types de domaine (Ticket, TicketLine, Payment, etc.)
    │   ├── kindmother_client_db.rs  # JayShopDb — CRUD via KindMother Client (JSON/TCP)
    │   └── schema.sql            # Schema DDL (applique par KindMother Service sur jayshop.db)
    ├── domain/
    │   ├── mod.rs                # Exports publics
    │   ├── ticket.rs             # Logique metier ticket (creation, cloture, remboursement)
    │   ├── payment.rs            # Logique paiement (calcul rendu, paiement mixte)
    │   ├── cash_session.rs       # Sessions de caisse (ouverture, mouvements, cloture)
    │   ├── pos_config.rs         # Configuration PoS (onglets, boutons, representation)
    │   ├── discount.rs           # Remises pre-definies
    │   └── tax.rs                # Taxes multi-taux (incluse/ajoutee)
    ├── integrations/
    │   ├── mod.rs                # Exports publics
    │   ├── jayxpose_sync.rs      # Synchronisation catalogue et stocks avec JayXpose
    │   ├── jaykonta_pipeline.rs  # Transmission comptable vers JayKonta
    │   └── contracts.rs          # Payloads contractuels IFS-JSH-*
    └── services/
        ├── mod.rs                # Exports publics
        ├── ticket_service.rs     # Service metier : cycle de vie du ticket
        ├── payment_service.rs    # Service metier : paiement et rendu monnaie
        ├── cash_service.rs       # Service metier : sessions de caisse
        ├── report_service.rs     # Service metier : rapports et KPIs
        └── sync_service.rs       # Service metier : orchestration sync JayXpose/JayKonta
```

### 2.2 Cargo.toml

```toml
# @id: jayshop_crate_config
# @do: declare_jayshop_crate_and_dependencies
# @layer: infra
# Crate JayShop — Service officiel famille Jay (commerce, PoS, vente en ligne).
# Persistance via KindMother Client → KindMother Service → libSQL chiffre.

[package]
name = "jayshop"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
documentation.workspace = true
keywords.workspace = true
categories.workspace = true
description = "JayShop — Service commerce, point de vente et vente en ligne"

[dependencies]
kindmother = { path = "../kindmother" }
kindmother-client = { path = "../kindmother-client" }
jayxpose = { path = "../jayxpose" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"
tokio = { version = "1", features = ["rt-multi-thread"] }
thiserror = "2"

[dev-dependencies]

[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
```

**Note** : Pas de dependance a `libsql` ou `rusqlite`. L'acces DB passe exclusivement par `kindmother-client` (JSON/TCP). KindMother Service est le seul processus qui touche les fichiers `.db`.

### 2.3 lib.rs

```rust
//! Bibliotheque JayShop — Service commerce, point de vente et vente en ligne.
//!
//! @id: jayshop_lib
//! @do: expose_public_modules_jayshop
//! @layer: infra
//!
//! JayShop s'appuie sur JayXpose pour le catalogue et les stocks,
//! et transmet les donnees comptables a JayKonta.

/// Persistance KindMother (libSQL via KindMother Service).
pub mod data;
/// Types et logique de domaine JayShop.
pub mod domain;
/// Integrations inter-services (JayXpose, JayKonta).
pub mod integrations;
/// Services metier (ticket, paiement, caisse, rapports, sync).
pub mod services;
```

---

## 3. Schema de persistance KindMother (libSQL)

Le schema est applique par KindMother Service sur la base `jayshop.db` (libSQL chiffree AES-256-CBC).
Toutes les operations CRUD passent par `kindmother-client` via JSON/TCP. Aucun crate n'a d'acces direct au fichier.

### 3.1 schema.sql

```sql
-- =============================================
-- JayShop — Schema de persistance KindMother
-- Engine: libSQL (fork Turso) + AES-256-CBC
-- =============================================

-- Configuration boutique
CREATE TABLE IF NOT EXISTS shop_config (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    shop_name TEXT NOT NULL,
    slug TEXT,
    currency TEXT NOT NULL DEFAULT 'EUR',
    legal_cgv TEXT,
    legal_mentions TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Taux de taxe
CREATE TABLE IF NOT EXISTS tax_rates (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    name TEXT NOT NULL,
    rate REAL NOT NULL,                -- ex. 20.0 pour 20%
    tax_type TEXT NOT NULL DEFAULT 'included',  -- 'included' | 'added'
    apply_to_new_items INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Association taxe <-> produit
CREATE TABLE IF NOT EXISTS product_tax_rates (
    product_id TEXT NOT NULL,
    tax_rate_id TEXT NOT NULL REFERENCES tax_rates(id),
    PRIMARY KEY (product_id, tax_rate_id)
);

-- Remises pre-definies
CREATE TABLE IF NOT EXISTS discounts (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    name TEXT NOT NULL,
    discount_type TEXT NOT NULL,       -- 'percent' | 'amount'
    value REAL NOT NULL,               -- ex. 10.0 pour 10% ou 500 pour 5.00 EUR
    restricted_access INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Configuration PoS
CREATE TABLE IF NOT EXISTS pos_config (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL UNIQUE,
    tabs_json TEXT NOT NULL DEFAULT '[]',        -- JSON: [{name, sort_order, color}]
    buttons_json TEXT NOT NULL DEFAULT '[]',     -- JSON: [{product_id, tab_index, label, color, shape, image_url, size, position}]
    payment_methods_json TEXT NOT NULL DEFAULT '["cash"]',
    receipt_logo_url TEXT,
    receipt_header TEXT,
    receipt_footer TEXT,
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Modes de paiement configures
CREATE TABLE IF NOT EXISTS payment_methods (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    name TEXT NOT NULL,                -- ex. 'Cash', 'CB', 'Cheque'
    method_type TEXT NOT NULL,         -- 'cash' | 'card' | 'check' | 'transfer' | 'other'
    sort_order INTEGER NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Sessions de caisse
CREATE TABLE IF NOT EXISTS cash_sessions (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    opened_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    closed_at TEXT,
    opening_cash INTEGER NOT NULL DEFAULT 0,      -- en centimes
    closing_cash_expected INTEGER,
    closing_cash_counted INTEGER,
    cash_difference INTEGER,
    total_sales INTEGER,
    total_refunds INTEGER,
    note TEXT
);

-- Mouvements manuels de caisse
CREATE TABLE IF NOT EXISTS cash_movements (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    session_id TEXT NOT NULL REFERENCES cash_sessions(id),
    movement_type TEXT NOT NULL,       -- 'in' | 'out'
    amount INTEGER NOT NULL,           -- en centimes
    reason TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Tickets de vente
CREATE TABLE IF NOT EXISTS tickets (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    ticket_number TEXT NOT NULL UNIQUE,
    seller_id TEXT NOT NULL,
    session_id TEXT REFERENCES cash_sessions(id),
    event_id TEXT REFERENCES events(id),  -- FK optionnel vers evenement (vente sur evenement)
    source TEXT NOT NULL,              -- 'pos' | 'online'
    status TEXT NOT NULL DEFAULT 'draft',  -- 'draft' | 'open' | 'paid' | 'refunded' | 'cancelled'
    customer_id TEXT,                  -- FK optionnel vers fiche client
    subtotal INTEGER NOT NULL DEFAULT 0,   -- en centimes HT
    tax_total INTEGER NOT NULL DEFAULT 0,
    discount_total INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,      -- en centimes TTC
    currency TEXT NOT NULL DEFAULT 'EUR',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    closed_at TEXT,
    refund_of TEXT REFERENCES tickets(id)  -- si c'est un ticket de remboursement
);

-- Lignes de ticket
CREATE TABLE IF NOT EXISTS ticket_lines (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    ticket_id TEXT NOT NULL REFERENCES tickets(id),
    product_id TEXT NOT NULL,          -- reference JayXpose
    product_name TEXT NOT NULL,        -- snapshot au moment de la vente
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price INTEGER NOT NULL,       -- en centimes (snapshot)
    discount_id TEXT REFERENCES discounts(id),
    discount_amount INTEGER NOT NULL DEFAULT 0,
    tax_rate REAL NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    line_total INTEGER NOT NULL,       -- en centimes TTC
    sort_order INTEGER NOT NULL DEFAULT 0
);

-- Paiements
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    ticket_id TEXT NOT NULL REFERENCES tickets(id),
    method_id TEXT REFERENCES payment_methods(id),
    method_type TEXT NOT NULL,         -- 'cash' | 'card' | 'check' | 'transfer' | 'other'
    amount INTEGER NOT NULL,           -- en centimes
    given_amount INTEGER,              -- montant donne (especes) en centimes
    change_amount INTEGER,             -- rendu monnaie en centimes
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Clients (fichier client local, lie a MiyuContacts)
CREATE TABLE IF NOT EXISTS customers (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address TEXT,
    city TEXT,
    postal_code TEXT,
    country TEXT DEFAULT 'FR',
    notes TEXT,
    total_purchases INTEGER NOT NULL DEFAULT 0,  -- nombre de tickets
    total_spent INTEGER NOT NULL DEFAULT 0,      -- en centimes
    loyalty_points INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Journal de synchronisation inter-services
CREATE TABLE IF NOT EXISTS sync_logs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    sync_source TEXT NOT NULL,         -- 'jayshop' | 'jayxpose' | 'jaykonta'
    sync_type TEXT NOT NULL,           -- 'stock_push' | 'sale_closed' | 'refund' | etc.
    status TEXT NOT NULL,              -- 'ok' | 'partial' | 'error' | 'denied'
    payload_json TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Parametres de fonctionnalites activables (toggles)
CREATE TABLE IF NOT EXISTS feature_toggles (
    seller_id TEXT NOT NULL,
    feature_key TEXT NOT NULL,         -- 'shifts' | 'open_tickets' | 'low_stock_alerts' | 'events' | etc.
    is_enabled INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (seller_id, feature_key)
);

-- =============================================
-- GESTION DES EVENEMENTS / FESTIVALS
-- =============================================

-- Fiche evenement / participation a un evenement
CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    name TEXT NOT NULL,                -- ex. "Salon du Livre 2026"
    start_date TEXT NOT NULL,          -- DATE ISO
    end_date TEXT NOT NULL,            -- DATE ISO
    location TEXT,                     -- lieu de l'evenement
    stand_info TEXT,                   -- informations stand (numero, zone, taille)
    status TEXT NOT NULL DEFAULT 'draft',  -- 'draft' | 'confirmed' | 'ongoing' | 'closed' | 'cancelled'
    notes TEXT,                        -- notes internes
    -- Liaison optionnelle avec JayFestival
    jayfestival_edition_id TEXT,       -- FK vers edition JayFestival
    jayfestival_candidature_id TEXT,   -- FK vers candidature JayFestival
    jayfestival_sync_status TEXT,      -- 'synced' | 'pending' | 'error' | NULL si creation manuelle
    -- Metriques (calculees et mises a jour)
    total_revenue INTEGER DEFAULT 0,   -- CA total en centimes
    total_costs INTEGER DEFAULT 0,     -- Couts totaux en centimes
    gross_profit INTEGER DEFAULT 0,    -- Benefice brut (revenue - costs)
    -- Timestamps
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    closed_at TEXT                     -- date de cloture
);

-- Couts de participation a un evenement
CREATE TABLE IF NOT EXISTS event_costs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    event_id TEXT NOT NULL REFERENCES events(id),
    category TEXT NOT NULL,            -- 'stand' | 'transport' | 'lodging' | 'food' | 'other'
    label TEXT NOT NULL,               -- ex. "Billet train A/R", "Hotel 3 nuits"
    amount INTEGER NOT NULL,           -- montant en centimes
    currency TEXT NOT NULL DEFAULT 'EUR',
    cost_date TEXT,                    -- date du cout (pour ventilation)
    receipt_url TEXT,                  -- URL justificatif (optionnel)
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Stock temporaire alloue a un evenement
CREATE TABLE IF NOT EXISTS event_stock (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    event_id TEXT NOT NULL REFERENCES events(id),
    product_id TEXT NOT NULL,          -- FK vers produit JayXpose
    product_name TEXT NOT NULL,        -- snapshot du nom (pour affichage meme si produit modifie)
    allocated_qty INTEGER NOT NULL,    -- quantite allouee pour l'evenement
    sold_qty INTEGER NOT NULL DEFAULT 0,  -- quantite vendue
    returned_qty INTEGER NOT NULL DEFAULT 0, -- quantite reintegree au stock global
    unit_cost INTEGER,                 -- cout unitaire d'achat (pour calcul benefice net)
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(event_id, product_id)
);

-- Index
CREATE INDEX IF NOT EXISTS idx_tickets_seller ON tickets(seller_id);
CREATE INDEX IF NOT EXISTS idx_tickets_status ON tickets(status);
CREATE INDEX IF NOT EXISTS idx_tickets_created ON tickets(created_at);
CREATE INDEX IF NOT EXISTS idx_tickets_session ON tickets(session_id);
CREATE INDEX IF NOT EXISTS idx_tickets_event ON tickets(event_id);
CREATE INDEX IF NOT EXISTS idx_ticket_lines_ticket ON ticket_lines(ticket_id);
CREATE INDEX IF NOT EXISTS idx_payments_ticket ON payments(ticket_id);
CREATE INDEX IF NOT EXISTS idx_cash_sessions_seller ON cash_sessions(seller_id);
CREATE INDEX IF NOT EXISTS idx_customers_seller ON customers(seller_id);
CREATE INDEX IF NOT EXISTS idx_sync_logs_type ON sync_logs(sync_type);
CREATE INDEX IF NOT EXISTS idx_sync_logs_created ON sync_logs(created_at);
-- Index evenements
CREATE INDEX IF NOT EXISTS idx_events_seller ON events(seller_id);
CREATE INDEX IF NOT EXISTS idx_events_status ON events(status);
CREATE INDEX IF NOT EXISTS idx_events_dates ON events(start_date, end_date);
CREATE INDEX IF NOT EXISTS idx_events_jayfestival ON events(jayfestival_edition_id);
CREATE INDEX IF NOT EXISTS idx_event_costs_event ON event_costs(event_id);
CREATE INDEX IF NOT EXISTS idx_event_stock_event ON event_stock(event_id);
CREATE INDEX IF NOT EXISTS idx_event_stock_product ON event_stock(product_id);
```

### 3.2 Types de domaine (data/types.rs)

```rust
use serde::{Deserialize, Serialize};

// -- Enums --

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketSource { Pos, Online }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TicketStatus { Draft, Open, Paid, Refunded, Cancelled }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscountType { Percent, Amount }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaxType { Included, Added }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MovementType { In, Out }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventStatus { Draft, Confirmed, Ongoing, Closed, Cancelled }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventCostCategory { Stand, Transport, Lodging, Food, Other }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JayFestivalSyncStatus { Synced, Pending, Error }

// -- Types principaux --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub ticket_number: String,
    pub seller_id: String,
    pub session_id: Option<String>,
    pub source: TicketSource,
    pub status: TicketStatus,
    pub customer_id: Option<String>,
    pub subtotal: i64,               // centimes HT
    pub tax_total: i64,
    pub discount_total: i64,
    pub total: i64,                  // centimes TTC
    pub currency: String,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub refund_of: Option<String>,
}

pub struct TicketLine { /* ... */ }
pub struct Payment { /* ... */ }
pub struct CashSession { /* ... */ }
pub struct CashMovement { /* ... */ }
pub struct Discount { /* ... */ }
pub struct TaxRate { /* ... */ }
pub struct PosConfig { /* ... */ }
pub struct PaymentMethod { /* ... */ }
pub struct Customer { /* ... */ }
pub struct ShopConfig { /* ... */ }
pub struct SyncLogEntry { /* ... */ }

// -- Types evenements --

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub seller_id: String,
    pub name: String,
    pub start_date: String,           // DATE ISO
    pub end_date: String,             // DATE ISO
    pub location: Option<String>,
    pub stand_info: Option<String>,
    pub status: EventStatus,
    pub notes: Option<String>,
    // Liaison JayFestival
    pub jayfestival_edition_id: Option<String>,
    pub jayfestival_candidature_id: Option<String>,
    pub jayfestival_sync_status: Option<JayFestivalSyncStatus>,
    // Metriques
    pub total_revenue: i64,           // centimes
    pub total_costs: i64,             // centimes
    pub gross_profit: i64,            // centimes
    // Timestamps
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCost {
    pub id: String,
    pub event_id: String,
    pub category: EventCostCategory,
    pub label: String,
    pub amount: i64,                  // centimes
    pub currency: String,
    pub cost_date: Option<String>,
    pub receipt_url: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStock {
    pub id: String,
    pub event_id: String,
    pub product_id: String,
    pub product_name: String,
    pub allocated_qty: i32,
    pub sold_qty: i32,
    pub returned_qty: i32,
    pub unit_cost: Option<i64>,       // cout unitaire d'achat (centimes)
    pub created_at: String,
    pub updated_at: String,
}
```

### 3.3 Pattern JayShopDb (data/kindmother_client_db.rs)

Le pattern suit exactement celui de `JayXposeDb` et `JayKontaDb` : un `KindMotherClient` global initialise au demarrage, enveloppe dans un `Arc`.

```rust
use std::sync::{Arc, OnceLock};
use kindmother_client::KindMotherClient;
use crate::data::types::*;

const DEFAULT_KINDMOTHER_ADDR: &str = "127.0.0.1:50051";
static CLIENT: OnceLock<Arc<KindMotherClient>> = OnceLock::new();

/// Erreurs de la couche data JayShop.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("KindMother client error: {0}")]
    Client(#[from] kindmother_client::ClientError),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

/// Base de donnees JayShop via KindMother Client.
///
/// @id: jayshop_db_client_struct
/// @do: hold_kindmother_client_connection
/// @layer: infra
#[derive(Clone)]
pub struct JayShopDb {
    client: Arc<KindMotherClient>,
}

impl JayShopDb {
    /// Initialise la connexion globale au service KindMother.
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), DbError> {
        let addr = addr.unwrap_or(DEFAULT_KINDMOTHER_ADDR);
        let client = KindMotherClient::connect(addr, "jayshop", "jayshop").await?;
        CLIENT
            .set(Arc::new(client))
            .map_err(|_| DbError::Internal("Client already initialized".to_string()))?;
        Ok(())
    }

    /// Recupere l'instance globale.
    pub fn global() -> Result<Self, DbError> {
        let client = CLIENT
            .get()
            .ok_or_else(|| DbError::Internal("Client not initialized".to_string()))?
            .clone();
        Ok(Self { client })
    }

    // -- Exemple : creer un ticket --
    pub async fn create_ticket(&self, ticket: &Ticket) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT INTO tickets (id, ticket_number, seller_id, session_id, source, status,
                    customer_id, subtotal, tax_total, discount_total, total, currency, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                vec![
                    ticket.id.clone(),
                    ticket.ticket_number.clone(),
                    ticket.seller_id.clone(),
                    ticket.session_id.clone().unwrap_or_default(),
                    // ... autres params
                ],
                "ticket_create",  // WriteIntent
            )
            .await?;
        Ok(())
    }

    // -- Exemple : lire les tickets d'un vendeur --
    pub async fn list_tickets(&self, seller_id: &str) -> Result<Vec<Ticket>, DbError> {
        let rows = self.client
            .query(
                "SELECT id, ticket_number, seller_id, session_id, source, status,
                        customer_id, subtotal, tax_total, discount_total, total, currency,
                        created_at, closed_at, refund_of
                 FROM tickets WHERE seller_id = ?1
                 ORDER BY created_at DESC",
                vec![seller_id.to_string()],
            )
            .await?;
        // Mapper les rows en Vec<Ticket>
        // ...
        Ok(vec![])
    }
}
```

**Points cles du pattern** :
- `OnceLock<Arc<KindMotherClient>>` : client global, initialise une seule fois
- `KindMotherClient::connect(addr, "jayshop", "jayshop")` : operateur = `jayshop`, base = `jayshop`
- Lectures via `client.query(sql, params)` — pas de WriteIntent
- Ecritures via `client.execute(sql, params, intent)` — WriteIntent trace et audite
- Transactions atomiques via `client.transaction(operations, intent)`

### 3.4 Pattern data/mod.rs

```rust
mod types;
mod kindmother_client_db;

pub use types::*;
pub use kindmother_client_db::{JayShopDb, DbError};
```

---

## 4. Operateurs JayShop

### 4.1 Operateur « JayShop Vente » (cycle de vie du ticket)

| Attribut | Description |
|----------|-------------|
| **Role** | Exposer la **creation, modification, cloture et remboursement des tickets de vente**. |
| **Public servi** | Admin (PoS et gestion) ; Client (boutique en ligne, Phase 2). |
| **Gouvernance** | Mandat de Permission (StrongFather) ; persistance (KindMother) ; securite (WorrySentinel). |
| **Capacites exposees** | Creer un ticket (draft) ; ajouter/modifier/supprimer des lignes ; appliquer une remise ; calculer les totaux (HT, taxes, TTC) ; cloturer le ticket (apres paiement) ; rembourser (total ou partiel) ; lister les tickets ouverts. |
| **Ne fait pas** | Paiement (JayShop Paiement) ; gestion catalogue (JayXpose) ; comptabilite (JayKonta). |

### 4.2 Operateur « JayShop Paiement » (encaissement)

| Attribut | Description |
|----------|-------------|
| **Role** | Exposer l'**encaissement d'un ticket** : selection mode de paiement, saisie montant, calcul rendu monnaie, paiement mixte, validation. |
| **Public servi** | Admin (PoS) ; Client (boutique en ligne, Phase 2). |
| **Gouvernance** | Mandat de Permission ; securite **Critical (3)** pour les donnees de paiement. |
| **Capacites exposees** | Enregistrer un paiement (montant, mode, montant donne) ; calculer le rendu monnaie ; calculer le reste a payer (paiement mixte) ; valider le paiement (quand reste = 0) ; declencher la cloture ticket et la transmission comptable. |
| **Ne fait pas** | Gestion du ticket (JayShop Vente) ; comptabilite (JayKonta). |

### 4.3 Operateur « JayShop Caisse » (sessions de caisse)

| Attribut | Description |
|----------|-------------|
| **Role** | Exposer la **gestion des sessions de caisse** : ouverture, mouvements manuels, cloture, synthese. |
| **Public servi** | Admin (PoS). |
| **Gouvernance** | Mandat ; securite Sensitive (2) a Critical (3). |
| **Capacites exposees** | Ouvrir une session (fond de caisse) ; enregistrer des mouvements manuels (entree/sortie + motif) ; cloturer une session (saisie especes comptees, calcul ecart) ; generer la synthese (ventilation par mode, especes attendues vs comptees). |
| **Ne fait pas** | Vente (JayShop Vente) ; paiement (JayShop Paiement). |

### 4.4 Operateur « JayShop Config » (configuration)

| Attribut | Description |
|----------|-------------|
| **Role** | Exposer la **configuration de la boutique, du PoS et des parametres** : modes de paiement, taxes, remises, recus, fonctionnalites activables. |
| **Public servi** | Admin. |
| **Gouvernance** | Mandat ; securite Standard (1). |
| **Capacites exposees** | CRUD configuration boutique (nom, slug, devise, CGV) ; CRUD modes de paiement ; CRUD taux de taxe ; CRUD remises pre-definies ; configuration PoS (onglets, boutons, representation) ; parametres de recu ; toggles fonctionnalites. |
| **Ne fait pas** | Vente ; paiement ; comptabilite. |

### 4.5 Operateur « JayShop Rapports » (tableau de bord et rapports)

| Attribut | Description |
|----------|-------------|
| **Role** | Exposer les **rapports de vente et le tableau de bord** : KPIs, sous-rapports, export. |
| **Public servi** | Admin. |
| **Gouvernance** | Lecture seule ; securite Standard (1) a Sensitive (2). |
| **Capacites exposees** | KPIs (ventes brutes, remboursements, remises, ventes nettes, profit brut) ; rapports par produit, categorie, mode de paiement, taxes, remises ; export CSV/PDF. |
| **Ne fait pas** | Vente ; paiement ; comptabilite. |

### 4.6 Operateur « JayShop Sync » (integrations inter-services)

| Attribut | Description |
|----------|-------------|
| **Role** | Orchestrer la **synchronisation avec JayXpose et JayKonta** : lecture/ecriture catalogue, sync stocks, transmission comptable. |
| **Public servi** | Systeme (arriere-plan). |
| **Gouvernance** | Mandat ; securite Sensitive (2) a Critical (3) ; audit complet. |
| **Capacites exposees** | Lire le catalogue JayXpose ; ecrire produit/categorie vers JayXpose ; sync stock push/pull ; transmettre vente/remboursement/cloture vers JayKonta ; journaliser dans sync_logs. |
| **Ne fait pas** | Vente ; paiement ; UI. |

### 4.7 Operateur « JayShop Evenement » (gestion des participations evenements/festivals)

| Attribut | Description |
|----------|-------------|
| **Role** | Exposer la **gestion des participations a des evenements/festivals** : fiche evenement, couts, stock temporaire, suivi des benefices, cloture. |
| **Public servi** | Admin. |
| **Gouvernance** | Mandat ; securite Standard (1) a Sensitive (2) pour les donnees financieres. |
| **Capacites exposees** | CRUD fiche evenement (creation manuelle ou automatique depuis JayFestival) ; CRUD couts de participation (stand, transport, logement, nourriture, autres) ; allouer un stock temporaire (deduit du stock global JayXpose) ; associer un ticket a un evenement ; calculer les metriques (CA, couts, benefice) ; cloturer l'evenement (reintegration stock, synthese comptable) ; dashboard evenement. |
| **Composants sous-jacents** | KindMother Client (tables `events`, `event_costs`, `event_stock`) ; JayXpose (lecture/decrementation stock) ; JayKonta (transmission comptable par evenement) ; JayFestival (synchronisation automatique). |
| **Ne fait pas** | Vente (JayShop Vente) ; paiement (JayShop Paiement) ; gestion candidatures exposant (JayFestival). |

### 4.8 Synthese des Operateurs

| Operateur | Besoins couverts |
|-----------|------------------|
| **JayShop Vente** | JSH-30 a JSH-36, JSH-40, JSH-48, JSH-50 a JSH-52, JSH-117 (association ticket/evenement) |
| **JayShop Paiement** | JSH-41 a JSH-47 |
| **JayShop Caisse** | JSH-60 a JSH-63 |
| **JayShop Config** | JSH-01 a JSH-06, JSH-90 a JSH-97 |
| **JayShop Rapports** | JSH-53 a JSH-59, JSH-54, JSH-118 (dashboard evenement) |
| **JayShop Sync** | JSH-70 a JSH-76, JSH-80 a JSH-83, JSH-130 a JSH-134 (integration JayFestival) |
| **JayShop Evenement** | JSH-110 a JSH-120 (gestion evenements, couts, stock temporaire) |

---

## 5. Kits d'Outils JayShop

### 5.1 Kit « Ticket de vente »

| Attribut | Description |
|----------|-------------|
| **Role** | Orchestrer le **cycle de vie complet du ticket** (creation, lignes, remises, taxes, cloture, remboursement). |
| **Outils agreges** | `ticket.create`, `ticket.line.add`, `ticket.line.update`, `ticket.line.remove`, `ticket.discount.apply`, `ticket.totals.compute`, `ticket.close`, `ticket.refund`, `ticket.list`, `ticket.get`, `ticket.open_list` (tickets ouverts). |
| **Consomme par** | JayShop Vente ; JayShop Paiement (cloture) ; JayShop Rapports (lecture). |
| **Composants sous-jacents** | KindMother Client → KindMother Service → libSQL `jayshop.db` (tables `tickets`, `ticket_lines`) ; JayXpose (catalogue pour resolution produit). |

### 5.2 Kit « Paiement »

| Attribut | Description |
|----------|-------------|
| **Role** | Orchestrer l'**encaissement** : saisie, calcul, validation. |
| **Outils agreges** | `payment.record` (enregistrer un paiement), `payment.change.compute` (calculer rendu), `payment.remaining.compute` (calculer reste a payer), `payment.validate` (valider quand total atteint), `payment.list_by_ticket`. |
| **Consomme par** | JayShop Paiement ; JayShop Caisse (pour synthese) ; JayShop Rapports. |
| **Composants sous-jacents** | KindMother (table `payments`). |

### 5.3 Kit « Session de caisse »

| Attribut | Description |
|----------|-------------|
| **Role** | Orchestrer les **sessions de caisse** : ouverture, mouvements, cloture, synthese. |
| **Outils agreges** | `cash_session.open`, `cash_session.close`, `cash_session.movement.add`, `cash_session.summary.compute`, `cash_session.list`, `cash_session.get`. |
| **Consomme par** | JayShop Caisse ; JayShop Rapports ; JayShop Sync (cloture → JayKonta). |
| **Composants sous-jacents** | KindMother (tables `cash_sessions`, `cash_movements`). |

### 5.4 Kit « Configuration »

| Attribut | Description |
|----------|-------------|
| **Role** | Orchestrer la **configuration** de la boutique, du PoS, des taxes, remises et modes de paiement. |
| **Outils agreges** | `config.shop.get`, `config.shop.update`, `config.pos.get`, `config.pos.update`, `config.payment_methods.crud`, `config.tax_rates.crud`, `config.discounts.crud`, `config.features.toggle`, `config.receipt.update`. |
| **Consomme par** | JayShop Config ; JayShop Vente (lecture taxes, remises) ; JayShop Paiement (lecture modes). |
| **Composants sous-jacents** | KindMother (tables `shop_config`, `pos_config`, `payment_methods`, `tax_rates`, `discounts`, `feature_toggles`). |

### 5.5 Kit « Rapports »

| Attribut | Description |
|----------|-------------|
| **Role** | Orchestrer les **requetes de rapport** : KPIs, ventilations, export. |
| **Outils agreges** | `report.kpi.summary` (ventes brutes, remboursements, remises, net, profit), `report.by_product`, `report.by_category`, `report.by_payment_type`, `report.by_tax`, `report.by_discount`, `report.export_csv`, `report.export_pdf`. |
| **Consomme par** | JayShop Rapports. |
| **Composants sous-jacents** | KindMother (requetes agregees sur `tickets`, `ticket_lines`, `payments`). |

### 5.6 Kit « Sync Inter-Services »

| Attribut | Description |
|----------|-------------|
| **Role** | Orchestrer la **synchronisation catalogue/stocks avec JayXpose**, la **transmission comptable vers JayKonta**, et l'**integration avec JayFestival**. |
| **Outils agreges** | `sync.jayxpose.catalogue.read`, `sync.jayxpose.product.write`, `sync.jayxpose.stock.push`, `sync.jayxpose.stock.pull`, `sync.jaykonta.sale.transmit`, `sync.jaykonta.refund.transmit`, `sync.jaykonta.session.transmit`, `sync.jayfestival.event.create` (creation auto depuis validation candidature), `sync.jayfestival.event.update` (sync infos edition), `sync.jayfestival.event.cancel`, `sync.log.record`. |
| **Consomme par** | JayShop Sync ; JayShop Evenement (pour stock temporaire). |
| **Composants sous-jacents** | JayXpose (crate `jayxpose::data`), JayKonta (crate futur), JayFestival (crate `jayfestival::data`), KindMother (table `sync_logs`). |

### 5.7 Kit « Evenement »

| Attribut | Description |
|----------|-------------|
| **Role** | Orchestrer la **gestion des participations a des evenements/festivals** : fiches, couts, stock temporaire, suivi benefices, cloture. |
| **Outils agreges** | `event.create` (creation manuelle), `event.update`, `event.get`, `event.list`, `event.delete`, `event.cost.add`, `event.cost.update`, `event.cost.delete`, `event.cost.list`, `event.cost.total`, `event.stock.allocate` (allouer stock temporaire depuis stock global), `event.stock.update`, `event.stock.list`, `event.stock.remaining`, `event.stock.reintegrate` (reintegrer au stock global), `event.metrics.compute` (CA, couts, benefice), `event.close` (cloture + reintegration + synthese comptable), `event.dashboard`. |
| **Consomme par** | JayShop Evenement ; JayShop Rapports (dashboard evenement) ; JayShop Sync (integration JayFestival). |
| **Composants sous-jacents** | KindMother (tables `events`, `event_costs`, `event_stock`) ; JayXpose (stock global) ; JayKonta (transmission comptable par evenement). |

---

## 6. Integration dans Miyukini Central

### 6.1 Enregistrement du service

Dans `apps/central/src/services/mod.rs`, ajouter :

```rust
pub mod jayshop;
pub mod jayshop_view;

// Dans le ServiceRouter :
Some("jayshop") => rsx! { JayShopView {} },
```

### 6.2 ServiceConnections

Dans `apps/central/src/data.rs`, ajouter :

```rust
use jayshop::data::JayShopDb;

pub struct ServiceConnections {
    // ... existants (jayxpose, jaykonta, jayfestival, jaykoa) ...
    pub jayshop: Arc<JayShopDb>,
}

impl ServiceConnections {
    pub async fn open() -> Result<Self, ...> {
        // ... existants ...
        // JayShop se connecte a KindMother Service via kindmother-client
        JayShopDb::init_global_async(None).await?;
        let jayshop = Arc::new(JayShopDb::global()?);
        // ...
    }
}
```

**Note** : Pas de chemin fichier a passer. `JayShopDb` se connecte a KindMother Service via JSON/TCP. KindMother Service gere la creation et le chiffrement de `jayshop.db` de facon isolee.

### 6.3 Cargo.toml (apps/central)

```toml
[dependencies]
# ... existants ...
jayshop = { path = "../../crates/jayshop" }
```

### 6.4 Structure UI (apps/central/src/services/jayshop/)

```
apps/central/src/services/jayshop/
├── mod.rs              # Module racine, JayShopSection enum, state
├── sidebar.rs          # Sidebar navigation JayShop
├── dashboard.rs        # JSH-A01 Tableau de bord
├── pos_main.rs         # JSH-A05 Ecran principal PoS
├── pos_payment.rs      # JSH-A06 Ecran de paiement
├── pos_config.rs       # JSH-A07 Configuration PoS
├── pos_open_close.rs   # JSH-A08/A09 Ouverture/Cloture caisse
├── products.rs         # JSH-A02 Liste produits
├── product_form.rs     # JSH-A03 Formulaire produit
├── categories.rs       # JSH-A04 Categories
├── discounts.rs        # JSH-A16 Remises
├── taxes.rs            # JSH-A18 Taxes
├── customers.rs        # JSH-A17 Fichier client
├── tickets.rs          # JSH-A10 Historique tickets
├── ticket_detail.rs    # JSH-A11 Detail ticket
├── orders.rs           # JSH-A12 Commandes en ligne
├── reports.rs          # JSH-A19 Rapports detailles
├── settings.rs         # JSH-A13 Parametres
└── components.rs       # Composants partages JayShop
```

---

## 7. Plan d'implementation par phases

### Regles de travail

- 1 tache = 1 fichier principal livre
- 1 bloc fonctionnel = 1 id de tracabilite
- Tests obligatoires quand logique metier modifiee
- Aucune ecriture sensible hors mandat et audit
- `unsafe_code = "forbid"` dans tous les crates

### Phase 0 — Preparation

**Objectif** : Verrouiller perimetre, dependances, nomenclature.

| Id | Tache | Livrable |
|----|-------|----------|
| [001] | Valider le document fondateur et l'analyse des besoins | Documents valides |
| [002] | Valider les contrats inter-services IFS-JSH-* | Payloads figes |
| [003] | Creer le crate `jayshop` avec squelette | `crates/jayshop/` avec Cargo.toml, lib.rs, schema.sql |

### Phase 1 — Fondations service

**Objectif** : Etablir l'ossature technique du service.

| Id | Tache | Livrable |
|----|-------|----------|
| [101] | Modeles de domaine (types.rs) | `Ticket`, `TicketLine`, `Payment`, `CashSession`, `Discount`, `TaxRate`, `PosConfig`, `Customer`, enums |
| [102] | Schema libSQL (schema.sql) | Tables creees via KindMother Service, index, migrations |
| [103] | JayShopDb — requetes CRUD de base | `kindmother_client_db.rs` avec toutes les operations CRUD |
| [104] | Integration dans ServiceConnections | `jayshop.db` cree, `Arc<JayShopDb>` disponible |
| [105] | Module UI vide dans Central | `apps/central/src/services/jayshop/mod.rs` + sidebar + routing |

**Criteres** : Crate compile, DB creee, service visible dans Central (ecran vide).

### Phase 2 — Configuration et catalogue

**Objectif** : Permettre a l'admin de configurer sa boutique et ses produits.

| Id | Tache | Livrable |
|----|-------|----------|
| [201] | Ecran parametres boutique (JSH-A13) | Nom, slug, devise, CGV, fonctionnalites toggles |
| [202] | CRUD modes de paiement | Ecran + logique |
| [203] | CRUD taux de taxe (JSH-A18) | Multi-taux, incluse/ajoutee, application par defaut |
| [204] | CRUD remises pre-definies (JSH-A16) | Entites remises, type %/montant |
| [205] | Lecture catalogue JayXpose (IFS-JSH-01) | Sync produits, categories depuis JayXpose |
| [206] | Ecran liste produits (JSH-A02) | Affichage catalogue, filtres, alertes stock |
| [207] | Formulaire produit CRUD (JSH-A03) | Creation/edition relayee vers JayXpose (IFS-JSH-02) |
| [208] | Gestion categories (JSH-A04) | Avec couleur par categorie |

**Criteres** : Admin peut configurer boutique, voir produits, creer/modifier produits, definir taxes et remises.

### Phase 3 — Point de vente (PoS) coeur

**Objectif** : Livrer le PoS fonctionnel bout en bout.

| Id | Tache | Livrable |
|----|-------|----------|
| [301] | Configuration PoS (JSH-A07) | Onglets, boutons, couleur+forme+image, modes de paiement |
| [302] | Ecran principal PoS (JSH-A05) | Zone ticket + grille boutons, ajout produit, modification ligne |
| [303] | Logique ticket (ticket_service.rs) | Creation, ajout ligne, calcul totaux, remises, taxes |
| [304] | Ecran de paiement (JSH-A06) | Recap, saisie montant, calcul rendu, boutons mode, pave numerique |
| [305] | Logique paiement (payment_service.rs) | Enregistrement, rendu monnaie, paiement mixte, validation |
| [306] | Decrementation stock (IFS-JSH-03) | Sync stock push vers JayXpose a chaque cloture |
| [307] | Generation recu | Numero, date, lignes, total, paiements, rendu |

**Criteres** : Flux complet en PoS : ouvrir ticket → ajouter produits → payer → cloturer → stock decremente → recu genere.

### Phase 4 — Gestion de caisse et historique

**Objectif** : Sessions de caisse et suivi des ventes.

| Id | Tache | Livrable |
|----|-------|----------|
| [401] | Ouverture de caisse (JSH-A08) | Saisie fond de caisse, creation session |
| [402] | Mouvements manuels (JSH-A14) | Entrees/sorties avec motif |
| [403] | Cloture de caisse (JSH-A09) | Saisie comptage, calcul ecart, synthese |
| [404] | Historique des tickets (JSH-A10) | Liste, filtres, compteurs (ventes/remboursements) |
| [405] | Detail ticket (JSH-A11) | Lignes, paiements, actions (reimprimer, rembourser) |
| [406] | Remboursement | Ticket de remboursement lie, reajustement stock |
| [407] | Tickets ouverts (JSH-36) | Sauvegarder/reprendre des tickets en cours |

**Criteres** : Session de caisse complete. Historique consultable. Remboursement fonctionnel.

### Phase 5 — Rapports et tableau de bord

**Objectif** : Tableau de bord et rapports de vente.

| Id | Tache | Livrable |
|----|-------|----------|
| [501] | Tableau de bord KPIs (JSH-A01) | 5 KPIs en bande, graphique CA, selecteur periode |
| [502] | Rapport ventes par produit (JSH-A19) | Sous-rapport |
| [503] | Rapport ventes par categorie | Sous-rapport |
| [504] | Rapport ventes par mode de paiement | Sous-rapport |
| [505] | Rapport taxes et remises | Sous-rapports |
| [506] | Export CSV / PDF | Export avec filtres |

**Criteres** : Tableau de bord operationnel. Rapports consultables et exportables.

### Phase 6 — Integrations inter-services

**Objectif** : Brancher JayXpose et JayKonta sur les contrats.

| Id | Tache | Livrable |
|----|-------|----------|
| [601] | Transmission comptable vente → JayKonta (IFS-JSH-04) | Payload `sale_closed` |
| [602] | Transmission remboursement → JayKonta | Payload `refund` |
| [603] | Transmission cloture caisse → JayKonta | Payload `cash_session_closed` |
| [604] | Sync stock bidirectionnelle JayXpose (IFS-JSH-03) | Push/pull, resolution conflits |
| [605] | CRUD catalogue relaye (IFS-JSH-02) | Ecriture produit/categorie vers JayXpose |
| [606] | Journalisation sync_logs | Audit complet des echanges |

**Criteres** : JayKonta recoit les ventes. JayXpose recoit les mouvements de stock. sync_logs trace tout.

### Phase 7 — Fichier client et fonctionnalites avancees

**Objectif** : Enrichissements post-MVP.

| Id | Tache | Livrable |
|----|-------|----------|
| [701] | Fichier client (JSH-A17) | CRUD client, association ticket ↔ client |
| [702] | Alertes stock bas (JSH-75) | Notifications quand stock < seuil |
| [703] | Alertes stock negatif (JSH-76) | Avertissement en caisse |
| [704] | Import CSV produits (JSH-17) | Import en masse |
| [705] | Ajustement stock simplifie (JSH-A15) | Reception, perte, casse → relaye JayXpose |

### Phase 8 — Gestion des evenements et integration JayFestival

**Objectif** : Permettre le suivi des participations a des evenements/festivals avec couts, stock temporaire et benefices.

| Id | Tache | Livrable |
|----|-------|----------|
| [801] | Modeles de domaine evenement (types.rs) | `Event`, `EventCost`, `EventStock`, enums |
| [802] | Schema libSQL evenements | Tables `events`, `event_costs`, `event_stock` + index |
| [803] | JayShopDb — CRUD evenements | Requetes CRUD pour les 3 tables evenement |
| [804] | Ecran liste evenements (JSH-A20) | Liste avec statut, dates, CA, benefice |
| [805] | Formulaire creation/edition evenement (JSH-A21) | Nom, dates, lieu, stand, notes |
| [806] | Ecran couts de participation (JSH-A22) | CRUD couts (stand, transport, logement, nourriture, autres), total |
| [807] | Ecran stock temporaire (JSH-A23) | Selection produits, quantites, allocation depuis stock global |
| [808] | Allocation stock temporaire | Logique de decrementation stock global JayXpose, allocation vers event_stock |
| [809] | Association ticket ↔ evenement en PoS | Selecteur d'evenement actif, decrementation stock temporaire |
| [810] | Dashboard evenement (JSH-A24) | CA, couts, stock restant, benefice brut, benefice net |
| [811] | Cloture d'evenement | Reintegration stock non vendu vers stock global, synthese finale |
| [812] | Transmission comptable par evenement | Synthese vers JayKonta avec ventilation par evenement |
| [813] | Integration JayFestival — creation auto (IFS-JSH-06) | Reception notification validation candidature, creation fiche evenement |
| [814] | Integration JayFestival — synchronisation | Mise a jour infos edition (dates, lieu, stand), annulation |
| [815] | Notification creation evenement | Notification admin quand fiche creee automatiquement |

**Criteres** : Admin peut creer une fiche evenement, saisir les couts, allouer un stock temporaire, vendre sur evenement, voir le dashboard benefices, cloturer et reintegrer le stock. Integration JayFestival fonctionnelle.

### Phase 9 — Durcissement et verification

**Objectif** : Securite, audit, tests.

| Id | Tache | Livrable |
|----|-------|----------|
| [901] | Enforcement securite niveau 3 sur paiements | Chiffrement, mandats, audit |
| [902] | Tests unitaires domaine | ticket, payment, cash_session, tax, discount, event |
| [903] | Tests integration JayXpose sync | Fixtures JSON, scenarios push/pull, stock temporaire |
| [904] | Tests integration JayKonta transmission | Fixtures JSON, scenarios vente/remboursement/evenement |
| [905] | Tests integration JayFestival sync | Fixtures JSON, scenarios creation auto/sync/annulation |
| [906] | Hors-ligne et sync reconnexion | Queue locale, replay a la reconnexion |
| [907] | Rapport de verification final | Couverture, conformite, regressions |

---

## 9. Backlog Phase 2+

| Fonctionnalite | Phase |
|----------------|-------|
| Boutique en ligne (portail client) | Phase 2 |
| Programme de fidelite | Phase 2 |
| Variantes de produit (taille, couleur) | Phase 2 |
| Articles composites / kits | Phase 2 |
| Vente au poids/volume | Phase 2 |
| Integration terminaux de paiement (SumUp, Zettle) | Phase 2 |
| Multi-employes et pointage | Phase 2 |
| Affichage client (second ecran) | Phase 2 |
| Impressions cuisine (JayFaim) | Phase 2 |
| ~~PoS evenementiel JayFestival~~ | ~~Phase 2~~ → **Phase 8 (P1)** |
| Inventaire physique (scan) | Phase 3 |
| Bons de commande fournisseur | Phase 3 |

> **Note** : La gestion des evenements/festivals (fiches, couts, stock temporaire, integration JayFestival) a ete remontee en **Phase 8 (P1)** suite a l'analyse des besoins.

---

## 10. Checklist de livraison

- [ ] Crate `jayshop` compile (Cargo check)
- [ ] Schema libSQL applique via KindMother Service, DB jayshop.db creee
- [ ] Service visible dans Central (sidebar + routing)
- [ ] Configuration boutique operationnelle
- [ ] PoS fonctionnel (ticket → paiement → cloture)
- [ ] Sessions de caisse operationnelles
- [ ] Historique des tickets consultable
- [ ] Tableau de bord et KPIs
- [ ] Sync stock JayXpose active
- [ ] Transmission comptable JayKonta active
- [ ] sync_logs trace tous les echanges
- [ ] **Gestion des evenements operationnelle (fiche, couts, stock temporaire)**
- [ ] **Integration JayFestival active (creation auto, sync, annulation)**
- [ ] Tests unitaires passes
- [ ] Tests integration passes (JayXpose, JayKonta, JayFestival)
- [ ] Securite niveau 3 sur paiements
- [ ] `unsafe_code = "forbid"` verifie
- [ ] Lints clippy pedantic passes

---

## 11. Dependances (composants Miyukini)

| Besoin | Composant | Role |
|--------|-----------|------|
| Persistance | KindMother Service, kindmother-client | libSQL chiffre (JSON/TCP), CRUD via WriteIntents. |
| Catalogue produits | JayXpose (crate) | Source de verite produits, categories, stocks. |
| Comptabilite | JayKonta (crate) | Reception des ecritures comptables. |
| **Evenements/Festivals** | **JayFestival (crate)** | **Synchronisation automatique des participations evenements.** |
| Authentification | Miyauth, Master Butler | Compte admin, mandats, permissions. |
| Contacts client | MiyuContacts | Fiche client. |
| Securite, audit | WorrySentinel | Niveaux de securite, audit. |
| UI | Miyukini Central (Dioxus) | Shell UI, sidebar, routing. |

---

## 12. References

| Document | Role |
|----------|------|
| [JayShop - Document Fondateur](./JayShop%20-%20Document%20Fondateur.md) | Vision, fonctionnalites, principes. |
| [JayShop - Analyse des besoins](./JayShop%20-%20Analyse%20des%20besoins.md) | Besoins fonctionnels et non-fonctionnels. |
| [JayShop - Ecrans et UI](./JayShop%20-%20Ecrans%20et%20UI.md) | Cartographie ecrans et wireframes. |
| [JayShop - Parcours Utilisateur](./JayShop%20-%20Parcours%20Utilisateur.md) | Parcours admin et client. |
| [JayShop - Interfaces Inter-Services](./JayShop%20-%20Interfaces%20Inter-Services.md) | Contrats IFS-JSH-01 a 05. |
| [JayShop - Reference Loyverse](./reference/JayShop%20-%20Reference%20Loyverse%20Back%20Office.md) | Analyse concurrentielle Loyverse. |
| [JayXpose - Catalogue Produits](../JayXpose/JayXpose%20-%20Catalogue%20Produits.md) | Modele catalogue consomme. |
| [JayKonta - Plan Implementation](../JayKonta/JayKonta%20-%20Plan%20Implementation.md) | Pattern plan implementation reference. |

---

**Document** : JayShop — Guide Implementation
**Version** : 1.0
**Date** : 2026-02-11
**Statut** : Plan d'implementation reference
