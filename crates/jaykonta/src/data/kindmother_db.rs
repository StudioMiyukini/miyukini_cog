//! Base locale SQLite JayKonta (KindMother Daughter).

use crate::data::types::{
    AuditRecord, InvoiceRecord, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord,
};
use chrono::Local;
use kindmother::{InstanceIdentity, InstanceType};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// Erreur de base de donnees JayKonta.
#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JayKonta DB: {}", self.0)
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        Self(e.to_string())
    }
}

/// Agregats KPI Account.
#[derive(Debug, Clone, Default)]
pub struct AccountStats {
    /// Total facture ce mois.
    pub ca_mensuel: f64,
    /// Montant impaye.
    pub impayes: f64,
    /// Nombre de factures.
    pub invoices_count: i64,
    /// Taux de paiement.
    pub payment_rate_pct: f64,
}

/// Agregats KPI Purse.
#[derive(Debug, Clone, Default)]
pub struct PurseStats {
    /// Depenses mensuelles.
    pub monthly_expenses: f64,
    /// Pourcentage budget utilise.
    pub budget_usage_pct: f64,
    /// Objectifs actifs.
    pub active_goals: i64,
    /// Mouvements categorises.
    pub categorized_pct: f64,
}

/// DB locale JayKonta.
pub struct JayKontaDb {
    conn: Mutex<Connection>,
    /// Identite KindMother (toujours Daughter pour service local).
    pub instance: InstanceIdentity,
}

impl JayKontaDb {
    /// Ouvre ou cree la DB locale et initialise le schema.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
            instance: InstanceIdentity::new(InstanceType::Daughter),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS ledger_movements (
                id TEXT PRIMARY KEY,
                scope TEXT NOT NULL,
                context_ref TEXT NOT NULL,
                category TEXT NOT NULL,
                amount REAL NOT NULL,
                currency TEXT NOT NULL,
                movement_date TEXT NOT NULL,
                source_service TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS quotes (
                id TEXT PRIMARY KEY,
                scope TEXT NOT NULL,
                context_ref TEXT NOT NULL,
                counterparty_ref TEXT NOT NULL,
                total REAL NOT NULL,
                currency TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS invoices (
                id TEXT PRIMARY KEY,
                scope TEXT NOT NULL,
                context_ref TEXT NOT NULL,
                counterparty_ref TEXT NOT NULL,
                quote_id TEXT,
                total REAL NOT NULL,
                paid_amount REAL NOT NULL DEFAULT 0,
                currency TEXT NOT NULL,
                status TEXT NOT NULL,
                issued_at TEXT NOT NULL,
                due_at TEXT
            );

            CREATE TABLE IF NOT EXISTS payments (
                id TEXT PRIMARY KEY,
                invoice_id TEXT NOT NULL,
                amount REAL NOT NULL,
                currency TEXT NOT NULL,
                paid_at TEXT NOT NULL,
                method TEXT NOT NULL,
                reference_opaque TEXT NOT NULL,
                FOREIGN KEY (invoice_id) REFERENCES invoices(id)
            );

            CREATE TABLE IF NOT EXISTS reminders (
                id TEXT PRIMARY KEY,
                deadline_ref TEXT NOT NULL,
                due_at TEXT NOT NULL,
                label TEXT NOT NULL,
                context_ref TEXT NOT NULL,
                source_service TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS audit_log (
                id TEXT PRIMARY KEY,
                contract_id TEXT NOT NULL,
                actor_ref TEXT NOT NULL,
                operation TEXT NOT NULL,
                scope TEXT NOT NULL,
                object_ref TEXT NOT NULL,
                result TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_movements_scope_date ON ledger_movements(scope, movement_date);
            CREATE INDEX IF NOT EXISTS idx_invoices_scope_status ON invoices(scope, status);
            CREATE INDEX IF NOT EXISTS idx_payments_invoice ON payments(invoice_id);
            CREATE INDEX IF NOT EXISTS idx_audit_created_at ON audit_log(created_at);
            "#,
        )?;
        Ok(())
    }

    /// Indique si la DB est encore vide de flux d'integration.
    pub fn is_bootstrap_needed(&self) -> Result<bool, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))?;
        Ok(count == 0)
    }

    /// Insere un mouvement.
    pub fn insert_movement(&self, m: &MovementRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO ledger_movements
             (id, scope, context_ref, category, amount, currency, movement_date, source_service)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                m.id,
                m.scope,
                m.context_ref,
                m.category,
                m.amount,
                m.currency,
                m.movement_date,
                m.source_service
            ],
        )?;
        Ok(())
    }

    /// Insere un devis.
    pub fn insert_quote(&self, q: &QuoteRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO quotes
             (id, scope, context_ref, counterparty_ref, total, currency, status, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                q.id,
                q.scope,
                q.context_ref,
                q.counterparty_ref,
                q.total,
                q.currency,
                q.status,
                q.created_at
            ],
        )?;
        Ok(())
    }

    /// Insere une facture.
    pub fn insert_invoice(&self, i: &InvoiceRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO invoices
             (id, scope, context_ref, counterparty_ref, quote_id, total, paid_amount, currency, status, issued_at, due_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                i.id,
                i.scope,
                i.context_ref,
                i.counterparty_ref,
                i.quote_id,
                i.total,
                i.paid_amount,
                i.currency,
                i.status,
                i.issued_at,
                i.due_at
            ],
        )?;
        Ok(())
    }

    /// Insere un paiement et met a jour la facture associee.
    pub fn insert_payment_and_update_invoice(&self, p: &PaymentRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO payments
             (id, invoice_id, amount, currency, paid_at, method, reference_opaque)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                p.id,
                p.invoice_id,
                p.amount,
                p.currency,
                p.paid_at,
                p.method,
                p.reference_opaque
            ],
        )?;

        conn.execute(
            "UPDATE invoices
             SET paid_amount = COALESCE(paid_amount, 0) + ?1,
                 status = CASE
                    WHEN COALESCE(paid_amount, 0) + ?1 >= total THEN 'paid'
                    WHEN COALESCE(paid_amount, 0) + ?1 > 0 THEN 'partial'
                    ELSE status
                 END
             WHERE id = ?2",
            params![p.amount, p.invoice_id],
        )?;
        Ok(())
    }

    /// Insere un rappel.
    pub fn insert_reminder(&self, r: &ReminderRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT OR REPLACE INTO reminders
             (id, deadline_ref, due_at, label, context_ref, source_service)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                r.id,
                r.deadline_ref,
                r.due_at,
                r.label,
                r.context_ref,
                r.source_service
            ],
        )?;
        Ok(())
    }

    /// Insere une entree d'audit.
    pub fn insert_audit(&self, a: &AuditRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO audit_log
             (id, contract_id, actor_ref, operation, scope, object_ref, result, payload_json, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                a.id,
                a.contract_id,
                a.actor_ref,
                a.operation,
                a.scope,
                a.object_ref,
                a.result,
                a.payload_json,
                a.created_at
            ],
        )?;
        Ok(())
    }

    /// Retourne les statistiques Account.
    pub fn account_stats(&self) -> Result<AccountStats, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let month_key = Local::now().format("%Y-%m").to_string();

        let ca_mensuel: f64 = conn.query_row(
            "SELECT COALESCE(SUM(total), 0) FROM invoices
             WHERE scope='account' AND substr(issued_at, 1, 7)=?1",
            params![month_key],
            |row| row.get(0),
        )?;

        let impayes: f64 = conn.query_row(
            "SELECT COALESCE(SUM(MAX(total - paid_amount, 0)), 0) FROM invoices
             WHERE scope='account' AND status IN ('issued','partial','overdue')",
            [],
            |row| row.get(0),
        )?;

        let invoices_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM invoices WHERE scope='account'",
            [],
            |row| row.get(0),
        )?;

        let paid_sum: f64 = conn.query_row(
            "SELECT COALESCE(SUM(paid_amount), 0) FROM invoices WHERE scope='account'",
            [],
            |row| row.get(0),
        )?;
        let total_sum: f64 = conn.query_row(
            "SELECT COALESCE(SUM(total), 0) FROM invoices WHERE scope='account'",
            [],
            |row| row.get(0),
        )?;
        let payment_rate_pct = if total_sum > 0.0 {
            (paid_sum / total_sum) * 100.0
        } else {
            0.0
        };

        Ok(AccountStats {
            ca_mensuel,
            impayes,
            invoices_count,
            payment_rate_pct,
        })
    }

    /// Retourne les statistiques Purse.
    pub fn purse_stats(&self) -> Result<PurseStats, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let month_key = Local::now().format("%Y-%m").to_string();

        let monthly_expenses: f64 = conn.query_row(
            "SELECT COALESCE(SUM(ABS(amount)), 0) FROM ledger_movements
             WHERE scope='purse' AND amount < 0 AND substr(movement_date, 1, 7)=?1",
            params![month_key],
            |row| row.get(0),
        )?;

        let budget_in: f64 = conn.query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM ledger_movements
             WHERE scope='purse' AND category='budget_in'",
            [],
            |row| row.get(0),
        )?;
        let budget_out: f64 = conn.query_row(
            "SELECT COALESCE(SUM(ABS(amount)), 0) FROM ledger_movements
             WHERE scope='purse' AND amount < 0",
            [],
            |row| row.get(0),
        )?;
        let budget_usage_pct = if budget_in > 0.0 {
            (budget_out / budget_in * 100.0).min(999.0)
        } else {
            0.0
        };

        let active_goals: i64 = conn.query_row(
            "SELECT COUNT(*) FROM reminders WHERE source_service='jaykoa'",
            [],
            |row| row.get(0),
        )?;

        let total_moves: i64 = conn.query_row(
            "SELECT COUNT(*) FROM ledger_movements WHERE scope='purse'",
            [],
            |row| row.get(0),
        )?;
        let categorized_moves: i64 = conn.query_row(
            "SELECT COUNT(*) FROM ledger_movements
             WHERE scope='purse' AND category IS NOT NULL AND category != ''",
            [],
            |row| row.get(0),
        )?;
        let categorized_pct = if total_moves > 0 {
            categorized_moves as f64 * 100.0 / total_moves as f64
        } else {
            0.0
        };

        Ok(PurseStats {
            monthly_expenses,
            budget_usage_pct,
            active_goals,
            categorized_pct,
        })
    }

    /// Nombre d'ecritures d'audit.
    pub fn audit_count(&self) -> Result<i64, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count = conn.query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))?;
        Ok(count)
    }
}
