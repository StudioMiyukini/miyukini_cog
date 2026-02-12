-- =============================================
-- JayShop — Schéma de persistance KindMother
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
    rate REAL NOT NULL,
    tax_type TEXT NOT NULL DEFAULT 'included',
    apply_to_new_items INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Association taxe <-> produit
CREATE TABLE IF NOT EXISTS product_tax_rates (
    product_id TEXT NOT NULL,
    tax_rate_id TEXT NOT NULL REFERENCES tax_rates(id),
    PRIMARY KEY (product_id, tax_rate_id)
);

-- Remises pré-définies
CREATE TABLE IF NOT EXISTS discounts (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    name TEXT NOT NULL,
    discount_type TEXT NOT NULL,
    value REAL NOT NULL,
    restricted_access INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Configuration PoS
CREATE TABLE IF NOT EXISTS pos_config (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL UNIQUE,
    tabs_json TEXT NOT NULL DEFAULT '[]',
    buttons_json TEXT NOT NULL DEFAULT '[]',
    payment_methods_json TEXT NOT NULL DEFAULT '["cash"]',
    receipt_logo_url TEXT,
    receipt_header TEXT,
    receipt_footer TEXT,
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Modes de paiement configurés
CREATE TABLE IF NOT EXISTS payment_methods (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    name TEXT NOT NULL,
    method_type TEXT NOT NULL,
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
    opening_cash INTEGER NOT NULL DEFAULT 0,
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
    movement_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    reason TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Fiches événement / participation
CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    seller_id TEXT NOT NULL,
    name TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    location TEXT,
    stand_info TEXT,
    status TEXT NOT NULL DEFAULT 'draft',
    notes TEXT,
    jayfestival_edition_id TEXT,
    jayfestival_candidature_id TEXT,
    jayfestival_sync_status TEXT,
    total_revenue INTEGER DEFAULT 0,
    total_costs INTEGER DEFAULT 0,
    gross_profit INTEGER DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    closed_at TEXT
);

-- Coûts de participation à un événement
CREATE TABLE IF NOT EXISTS event_costs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    event_id TEXT NOT NULL REFERENCES events(id),
    category TEXT NOT NULL,
    label TEXT NOT NULL,
    amount INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'EUR',
    cost_date TEXT,
    receipt_url TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Stock temporaire alloué à un événement
CREATE TABLE IF NOT EXISTS event_stock (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    event_id TEXT NOT NULL REFERENCES events(id),
    product_id TEXT NOT NULL,
    product_name TEXT NOT NULL,
    allocated_qty INTEGER NOT NULL,
    sold_qty INTEGER NOT NULL DEFAULT 0,
    returned_qty INTEGER NOT NULL DEFAULT 0,
    unit_cost INTEGER,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    UNIQUE(event_id, product_id)
);

-- Tickets de vente
CREATE TABLE IF NOT EXISTS tickets (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    ticket_number TEXT NOT NULL UNIQUE,
    seller_id TEXT NOT NULL,
    session_id TEXT REFERENCES cash_sessions(id),
    event_id TEXT REFERENCES events(id),
    source TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    customer_id TEXT,
    subtotal INTEGER NOT NULL DEFAULT 0,
    tax_total INTEGER NOT NULL DEFAULT 0,
    discount_total INTEGER NOT NULL DEFAULT 0,
    total INTEGER NOT NULL DEFAULT 0,
    currency TEXT NOT NULL DEFAULT 'EUR',
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    closed_at TEXT,
    refund_of TEXT REFERENCES tickets(id)
);

-- Lignes de ticket
CREATE TABLE IF NOT EXISTS ticket_lines (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    ticket_id TEXT NOT NULL REFERENCES tickets(id),
    product_id TEXT NOT NULL,
    product_name TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price INTEGER NOT NULL,
    discount_id TEXT REFERENCES discounts(id),
    discount_amount INTEGER NOT NULL DEFAULT 0,
    tax_rate REAL NOT NULL DEFAULT 0,
    tax_amount INTEGER NOT NULL DEFAULT 0,
    line_total INTEGER NOT NULL,
    sort_order INTEGER NOT NULL DEFAULT 0
);

-- Paiements
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    ticket_id TEXT NOT NULL REFERENCES tickets(id),
    method_id TEXT REFERENCES payment_methods(id),
    method_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    given_amount INTEGER,
    change_amount INTEGER,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Clients (fichier client local)
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
    total_purchases INTEGER NOT NULL DEFAULT 0,
    total_spent INTEGER NOT NULL DEFAULT 0,
    loyalty_points INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Journal de synchronisation inter-services
CREATE TABLE IF NOT EXISTS sync_logs (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    sync_source TEXT NOT NULL,
    sync_type TEXT NOT NULL,
    status TEXT NOT NULL,
    payload_json TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- Paramètres de fonctionnalités activables (toggles)
CREATE TABLE IF NOT EXISTS feature_toggles (
    seller_id TEXT NOT NULL,
    feature_key TEXT NOT NULL,
    is_enabled INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (seller_id, feature_key)
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
CREATE INDEX IF NOT EXISTS idx_events_seller ON events(seller_id);
CREATE INDEX IF NOT EXISTS idx_events_status ON events(status);
CREATE INDEX IF NOT EXISTS idx_events_dates ON events(start_date, end_date);
CREATE INDEX IF NOT EXISTS idx_events_jayfestival ON events(jayfestival_edition_id);
CREATE INDEX IF NOT EXISTS idx_event_costs_event ON event_costs(event_id);
CREATE INDEX IF NOT EXISTS idx_event_stock_event ON event_stock(event_id);
CREATE INDEX IF NOT EXISTS idx_event_stock_product ON event_stock(product_id)
