-- ═══════════════════════════════════════════════════════════════
-- JayKonta — Schema complet KindMother (libSQL)
-- Purse + Account — 14 tables, triggers, vues
-- Version: 1.0 — Phase 1 Fondations
-- ═══════════════════════════════════════════════════════════════

-- ─── Comptes ─────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS accounts (
    id          TEXT PRIMARY KEY,
    scope       TEXT NOT NULL CHECK(scope IN ('purse', 'account')),
    user_id     TEXT NOT NULL,
    label       TEXT NOT NULL,
    currency    TEXT NOT NULL DEFAULT 'EUR',
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(user_id, scope)
);

-- ─── Categories ──────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS categories (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    icon        TEXT,
    color       TEXT,
    parent_id   TEXT REFERENCES categories(id),
    is_income   INTEGER NOT NULL DEFAULT 0,
    sort_order  INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(account_id, name)
);

-- ─── Mouvements (Purse + Account) ───────────────────────────

CREATE TABLE IF NOT EXISTS movements (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    scope           TEXT NOT NULL CHECK(scope IN ('purse', 'account')),
    category_id     TEXT REFERENCES categories(id),
    amount          REAL NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'EUR',
    description     TEXT,
    movement_date   TEXT NOT NULL,
    context_ref     TEXT,
    source_service  TEXT,
    budget_id       TEXT REFERENCES budgets(id),
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_movements_account ON movements(account_id);
CREATE INDEX IF NOT EXISTS idx_movements_date ON movements(movement_date);
CREATE INDEX IF NOT EXISTS idx_movements_category ON movements(category_id);
CREATE INDEX IF NOT EXISTS idx_movements_budget ON movements(budget_id);
CREATE INDEX IF NOT EXISTS idx_movements_scope ON movements(scope);

-- ─── Budgets occasionnels (Purse) ───────────────────────────

CREATE TABLE IF NOT EXISTS budgets (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    target      REAL NOT NULL,
    spent       REAL NOT NULL DEFAULT 0.0,
    currency    TEXT NOT NULL DEFAULT 'EUR',
    start_date  TEXT NOT NULL,
    end_date    TEXT,
    status      TEXT NOT NULL DEFAULT 'active'
                CHECK(status IN ('active', 'closed', 'cancelled')),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_budgets_account ON budgets(account_id);

-- ─── Objectifs (Purse) ──────────────────────────────────────

CREATE TABLE IF NOT EXISTS goals (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    target      REAL NOT NULL,
    current     REAL NOT NULL DEFAULT 0.0,
    currency    TEXT NOT NULL DEFAULT 'EUR',
    goal_type   TEXT NOT NULL DEFAULT 'savings'
                CHECK(goal_type IN ('savings', 'spending_limit')),
    deadline    TEXT,
    status      TEXT NOT NULL DEFAULT 'active'
                CHECK(status IN ('active', 'reached', 'failed', 'cancelled')),
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_goals_account ON goals(account_id);

-- ─── Alertes (Purse) ────────────────────────────────────────

CREATE TABLE IF NOT EXISTS alerts (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    alert_type      TEXT NOT NULL
                    CHECK(alert_type IN ('category_threshold', 'goal_warning', 'budget_warning')),
    category_id     TEXT REFERENCES categories(id),
    goal_id         TEXT REFERENCES goals(id),
    budget_id       TEXT REFERENCES budgets(id),
    threshold       REAL,
    frequency       TEXT NOT NULL DEFAULT 'weekly'
                    CHECK(frequency IN ('daily', 'weekly', 'monthly')),
    is_active       INTEGER NOT NULL DEFAULT 1,
    last_triggered  TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ─── Contreparties (Account) ────────────────────────────────

CREATE TABLE IF NOT EXISTS counterparties (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    email           TEXT,
    address         TEXT,
    siret           TEXT,
    tva_number      TEXT,
    phone           TEXT,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_counterparties_account ON counterparties(account_id);

-- ─── Devis (Account) ────────────────────────────────────────

CREATE TABLE IF NOT EXISTS quotes (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    number          TEXT NOT NULL,
    counterparty_id TEXT NOT NULL REFERENCES counterparties(id),
    context_ref     TEXT,
    total_ht        REAL NOT NULL,
    total_tva       REAL NOT NULL DEFAULT 0.0,
    total_ttc       REAL NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'EUR',
    status          TEXT NOT NULL DEFAULT 'draft'
                    CHECK(status IN ('draft', 'sent', 'accepted', 'rejected', 'converted')),
    validity_days   INTEGER NOT NULL DEFAULT 30,
    notes           TEXT,
    source_service  TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(account_id, number)
);

CREATE INDEX IF NOT EXISTS idx_quotes_account ON quotes(account_id);
CREATE INDEX IF NOT EXISTS idx_quotes_status ON quotes(status);

-- ─── Lignes de devis ────────────────────────────────────────

CREATE TABLE IF NOT EXISTS quote_lines (
    id          TEXT PRIMARY KEY,
    quote_id    TEXT NOT NULL REFERENCES quotes(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity    REAL NOT NULL DEFAULT 1.0,
    unit_price  REAL NOT NULL,
    tva_rate    REAL NOT NULL DEFAULT 20.0,
    total_ht    REAL NOT NULL,
    sort_order  INTEGER NOT NULL DEFAULT 0
);

-- ─── Factures (Account) ─────────────────────────────────────

CREATE TABLE IF NOT EXISTS invoices (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    number          TEXT NOT NULL,
    counterparty_id TEXT NOT NULL REFERENCES counterparties(id),
    quote_id        TEXT REFERENCES quotes(id),
    context_ref     TEXT,
    total_ht        REAL NOT NULL,
    total_tva       REAL NOT NULL DEFAULT 0.0,
    total_ttc       REAL NOT NULL,
    paid_amount     REAL NOT NULL DEFAULT 0.0,
    currency        TEXT NOT NULL DEFAULT 'EUR',
    status          TEXT NOT NULL DEFAULT 'issued'
                    CHECK(status IN ('issued', 'sent', 'partial', 'paid', 'overdue', 'cancelled')),
    issued_at       TEXT NOT NULL DEFAULT (datetime('now')),
    due_at          TEXT,
    source_service  TEXT,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now')),

    UNIQUE(account_id, number)
);

CREATE INDEX IF NOT EXISTS idx_invoices_account ON invoices(account_id);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
CREATE INDEX IF NOT EXISTS idx_invoices_due ON invoices(due_at);

-- ─── Lignes de facture ──────────────────────────────────────

CREATE TABLE IF NOT EXISTS invoice_lines (
    id          TEXT PRIMARY KEY,
    invoice_id  TEXT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity    REAL NOT NULL DEFAULT 1.0,
    unit_price  REAL NOT NULL,
    tva_rate    REAL NOT NULL DEFAULT 20.0,
    total_ht    REAL NOT NULL,
    sort_order  INTEGER NOT NULL DEFAULT 0
);

-- ─── Paiements (Account) ────────────────────────────────────

CREATE TABLE IF NOT EXISTS payments (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    invoice_id      TEXT NOT NULL REFERENCES invoices(id),
    amount          REAL NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'EUR',
    method          TEXT NOT NULL
                    CHECK(method IN ('virement', 'cb', 'cheque', 'especes', 'autre')),
    reference_opaque TEXT,
    paid_at         TEXT NOT NULL,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_payments_invoice ON payments(invoice_id);

-- ─── Transactions recurrentes (Purse + Account) ────────────

CREATE TABLE IF NOT EXISTS recurring_transactions (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    scope           TEXT NOT NULL CHECK(scope IN ('purse', 'account')),
    direction       TEXT NOT NULL CHECK(direction IN ('expense', 'income')),
    amount          REAL NOT NULL,
    currency        TEXT NOT NULL DEFAULT 'EUR',
    category_id     TEXT REFERENCES categories(id),
    description     TEXT NOT NULL,
    frequency       TEXT NOT NULL
                    CHECK(frequency IN ('weekly', 'biweekly', 'monthly', 'quarterly', 'yearly')),
    day_of_month    INTEGER,
    day_of_week     INTEGER,
    start_date      TEXT NOT NULL,
    end_date        TEXT,
    next_due_date   TEXT NOT NULL,
    is_active       INTEGER NOT NULL DEFAULT 1,
    auto_create     INTEGER NOT NULL DEFAULT 0,
    notes           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_recurring_account ON recurring_transactions(account_id);
CREATE INDEX IF NOT EXISTS idx_recurring_active ON recurring_transactions(is_active);
CREATE INDEX IF NOT EXISTS idx_recurring_next_due ON recurring_transactions(next_due_date);

-- ─── Rappels JayKoa (optionnel) ─────────────────────────────

CREATE TABLE IF NOT EXISTS reminders (
    id              TEXT PRIMARY KEY,
    account_id      TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    deadline_ref    TEXT NOT NULL,
    due_at          TEXT NOT NULL,
    label           TEXT NOT NULL,
    context_ref     TEXT,
    source_service  TEXT,
    published       INTEGER NOT NULL DEFAULT 0,
    published_at    TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ─── Audit ───────────────────────────────────────────────────

CREATE TABLE IF NOT EXISTS audit (
    id          TEXT PRIMARY KEY,
    account_id  TEXT NOT NULL REFERENCES accounts(id),
    contract_id TEXT NOT NULL,
    actor_ref   TEXT NOT NULL,
    operation   TEXT NOT NULL,
    scope       TEXT NOT NULL,
    object_ref  TEXT NOT NULL,
    result      TEXT NOT NULL CHECK(result IN ('ok', 'error', 'denied')),
    payload     TEXT,
    created_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_audit_account ON audit(account_id);
CREATE INDEX IF NOT EXISTS idx_audit_contract ON audit(contract_id);
CREATE INDEX IF NOT EXISTS idx_audit_date ON audit(created_at);

-- ─── Triggers ────────────────────────────────────────────────

-- Auto-update updated_at sur mouvements
CREATE TRIGGER IF NOT EXISTS trg_movements_updated
AFTER UPDATE ON movements FOR EACH ROW
BEGIN
    UPDATE movements SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Auto-update updated_at sur budgets
CREATE TRIGGER IF NOT EXISTS trg_budgets_updated
AFTER UPDATE ON budgets FOR EACH ROW
BEGIN
    UPDATE budgets SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Auto-update updated_at sur objectifs
CREATE TRIGGER IF NOT EXISTS trg_goals_updated
AFTER UPDATE ON goals FOR EACH ROW
BEGIN
    UPDATE goals SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Auto-update updated_at sur devis
CREATE TRIGGER IF NOT EXISTS trg_quotes_updated
AFTER UPDATE ON quotes FOR EACH ROW
BEGIN
    UPDATE quotes SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Auto-update updated_at sur factures
CREATE TRIGGER IF NOT EXISTS trg_invoices_updated
AFTER UPDATE ON invoices FOR EACH ROW
BEGIN
    UPDATE invoices SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Auto-update updated_at sur transactions recurrentes
CREATE TRIGGER IF NOT EXISTS trg_recurring_updated
AFTER UPDATE ON recurring_transactions FOR EACH ROW
BEGIN
    UPDATE recurring_transactions SET updated_at = datetime('now') WHERE id = NEW.id;
END;

-- Mise a jour cumul budget quand mouvement affecte
CREATE TRIGGER IF NOT EXISTS trg_budget_spent_insert
AFTER INSERT ON movements
WHEN NEW.budget_id IS NOT NULL
BEGIN
    UPDATE budgets
    SET spent = (
        SELECT COALESCE(SUM(ABS(amount)), 0)
        FROM movements
        WHERE budget_id = NEW.budget_id
    )
    WHERE id = NEW.budget_id;
END;

-- Mise a jour paid_amount sur facture quand paiement enregistre
CREATE TRIGGER IF NOT EXISTS trg_invoice_paid_insert
AFTER INSERT ON payments
BEGIN
    UPDATE invoices
    SET paid_amount = (
            SELECT COALESCE(SUM(amount), 0)
            FROM payments
            WHERE invoice_id = NEW.invoice_id
        ),
        status = CASE
            WHEN (SELECT COALESCE(SUM(amount), 0) FROM payments WHERE invoice_id = NEW.invoice_id) >= total_ttc
            THEN 'paid'
            ELSE 'partial'
        END
    WHERE id = NEW.invoice_id;
END;

-- ─── Vues ────────────────────────────────────────────────────

-- Synthese mouvements par categorie et mois
CREATE VIEW IF NOT EXISTS v_movements_by_category AS
SELECT
    m.account_id,
    m.scope,
    c.name AS category_name,
    strftime('%Y-%m', m.movement_date) AS month,
    SUM(CASE WHEN m.amount > 0 THEN m.amount ELSE 0 END) AS income,
    SUM(CASE WHEN m.amount < 0 THEN ABS(m.amount) ELSE 0 END) AS expense,
    COUNT(*) AS count
FROM movements m
LEFT JOIN categories c ON m.category_id = c.id
GROUP BY m.account_id, m.scope, c.name, strftime('%Y-%m', m.movement_date);

-- Synthese factures par statut
CREATE VIEW IF NOT EXISTS v_invoice_summary AS
SELECT
    account_id,
    status,
    COUNT(*) AS count,
    SUM(total_ttc) AS total,
    SUM(paid_amount) AS paid,
    SUM(total_ttc - paid_amount) AS remaining
FROM invoices
GROUP BY account_id, status;

-- Synthese devis par statut
CREATE VIEW IF NOT EXISTS v_quote_summary AS
SELECT
    account_id,
    status,
    COUNT(*) AS count,
    SUM(total_ttc) AS total
FROM quotes
GROUP BY account_id, status;
