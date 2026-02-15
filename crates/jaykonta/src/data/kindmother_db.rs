//! Base locale SQLite JayKonta (KindMother Daughter).
//!
//! Schema complet : 14 tables, triggers, vues.
//! Voir `data/schema.sql` pour le DDL de reference.

use crate::data::types::{
    AuditRecord, InvoiceRecord, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord,
};
use crate::domain::purse::{
    BudgetRef, BudgetStatus, Category, CategoryBreakdown, CategoryRef, GoalStatus, GoalType,
    Movement, MovementFilters, MovementType, OccasionalBudget, Goal,
    RecurringDirection, RecurringFrequency, RecurringTransaction,
};
// Types Account importes au besoin pour les methodes futures
// use crate::domain::account::{Counterparty, InvoiceStatus, PaymentMethod, QuoteStatus};
use chrono::Local;
use kindmother::{InstanceIdentity, InstanceType};
#[cfg(feature = "db-encryption")]
use kindmother_db_key::KeyDerivation;
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
    /// Ouvre ou cree la DB locale chiffree et initialise le schema.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let path = path.as_ref();
        let conn = Connection::open(path)?;

        #[cfg(feature = "db-encryption")]
        {
            let data_dir = path.parent().unwrap_or(path);
            let db_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("jaykonta");
            let kd = KeyDerivation::new(data_dir).map_err(|e| DbError(e.0))?;
            let pragma_key = kd.pragma_key_hex(db_name).map_err(|e| DbError(e.0))?;
            conn.pragma_update(None, "key", &pragma_key)
                .map_err(|e| DbError(e.to_string()))?;
        }

        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let db = Self {
            conn: Mutex::new(conn),
            instance: InstanceIdentity::new(InstanceType::Daughter),
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Version courante du schema JayKonta.
    /// Incrementer a chaque evolution incompatible du DDL.
    const SCHEMA_VERSION: i32 = 2;

    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;

        // Lire la version courante de la DB
        let current_version: i32 = conn
            .query_row("PRAGMA user_version", [], |row| row.get(0))
            .unwrap_or(0);

        if current_version < Self::SCHEMA_VERSION {
            // Migration : supprimer les anciennes tables/vues/triggers pour recreer proprement.
            // En Phase 1 (developpement), on accepte la perte des donnees legacy.
            conn.execute_batch(
                "
                -- Supprimer vues
                DROP VIEW   IF EXISTS v_movements_by_category;
                DROP VIEW   IF EXISTS v_invoice_summary;
                DROP VIEW   IF EXISTS v_quote_summary;

                -- Supprimer triggers
                DROP TRIGGER IF EXISTS trg_movements_updated;
                DROP TRIGGER IF EXISTS trg_budgets_updated;
                DROP TRIGGER IF EXISTS trg_goals_updated;
                DROP TRIGGER IF EXISTS trg_quotes_updated;
                DROP TRIGGER IF EXISTS trg_invoices_updated;
                DROP TRIGGER IF EXISTS trg_budget_spent_insert;
                DROP TRIGGER IF EXISTS trg_invoice_paid_insert;
                DROP TRIGGER IF EXISTS trg_recurring_updated;

                -- Supprimer tables (ordre inverse des dependances)
                DROP TABLE IF EXISTS recurring_transactions;
                DROP TABLE IF EXISTS audit;
                DROP TABLE IF EXISTS reminders;
                DROP TABLE IF EXISTS payments;
                DROP TABLE IF EXISTS invoice_lines;
                DROP TABLE IF EXISTS invoices;
                DROP TABLE IF EXISTS quote_lines;
                DROP TABLE IF EXISTS quotes;
                DROP TABLE IF EXISTS counterparties;
                DROP TABLE IF EXISTS alerts;
                DROP TABLE IF EXISTS goals;
                DROP TABLE IF EXISTS budgets;
                DROP TABLE IF EXISTS movements;
                DROP TABLE IF EXISTS categories;
                DROP TABLE IF EXISTS accounts;

                -- Supprimer ancienne table legacy si existante
                DROP TABLE IF EXISTS audit_log;
                ",
            )?;
        }

        // Creer (ou re-creer) le schema complet
        conn.execute_batch(include_str!("schema.sql"))?;

        // Mettre a jour la version
        if current_version < Self::SCHEMA_VERSION {
            conn.execute_batch(&format!(
                "PRAGMA user_version = {};",
                Self::SCHEMA_VERSION
            ))?;
        }

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════
    // Legacy compatibility (existing integration pipeline)
    // ═══════════════════════════════════════════════════════════

    /// Indique si la DB est encore vide de flux d'integration.
    pub fn is_bootstrap_needed(&self) -> Result<bool, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        // Verifie les deux tables d'audit (legacy + nouvelle)
        let legacy_exists: bool = conn.query_row(
            "SELECT COUNT(*) > 0 FROM sqlite_master WHERE type='table' AND name='audit_log'",
            [],
            |row| row.get(0),
        ).unwrap_or(false);

        if legacy_exists {
            let count: i64 = conn.query_row("SELECT COUNT(*) FROM audit_log", [], |row| row.get(0))?;
            if count > 0 { return Ok(false); }
        }

        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM audit",
            [],
            |row| row.get(0),
        ).unwrap_or(0);
        Ok(count == 0)
    }

    /// Insere un mouvement (API legacy pour pipeline d'integration).
    pub fn insert_movement(&self, m: &MovementRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        // Assure qu'un compte existe pour ce scope
        let account_id = self.ensure_account_exists_inner(&conn, &m.scope)?;
        conn.execute(
            "INSERT OR REPLACE INTO movements
             (id, account_id, scope, amount, currency, description, movement_date, context_ref, source_service, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, datetime('now'))",
            params![
                m.id,
                account_id,
                m.scope,
                m.amount,
                m.currency,
                m.category, // legacy : category comme description
                m.movement_date,
                m.context_ref,
                m.source_service
            ],
        )?;
        Ok(())
    }

    /// Insere un devis (API legacy pour pipeline d'integration).
    pub fn insert_quote(&self, q: &QuoteRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let account_id = self.ensure_account_exists_inner(&conn, &q.scope)?;
        // Assure contrepartie
        let cp_id = self.ensure_counterparty_inner(&conn, &account_id, &q.counterparty_ref)?;
        let number = format!("DEV-{}", &q.id[..8.min(q.id.len())]);
        conn.execute(
            "INSERT OR REPLACE INTO quotes
             (id, account_id, number, counterparty_id, context_ref, total_ht, total_tva, total_ttc,
              currency, status, source_service, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0.0, ?6, ?7, ?8, ?9, ?10)",
            params![
                q.id,
                account_id,
                number,
                cp_id,
                q.context_ref,
                q.total,
                q.currency,
                q.status,
                "integration",
                q.created_at
            ],
        )?;
        Ok(())
    }

    /// Insere une facture (API legacy pour pipeline d'integration).
    pub fn insert_invoice(&self, i: &InvoiceRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let account_id = self.ensure_account_exists_inner(&conn, &i.scope)?;
        let cp_id = self.ensure_counterparty_inner(&conn, &account_id, &i.counterparty_ref)?;
        let number = format!("FAC-{}", &i.id[..8.min(i.id.len())]);
        conn.execute(
            "INSERT OR REPLACE INTO invoices
             (id, account_id, number, counterparty_id, quote_id, context_ref,
              total_ht, total_tva, total_ttc, paid_amount, currency, status, issued_at, due_at, source_service)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0.0, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                i.id,
                account_id,
                number,
                cp_id,
                i.quote_id,
                i.context_ref,
                i.total,
                i.paid_amount,
                i.currency,
                i.status,
                i.issued_at,
                i.due_at,
                "integration"
            ],
        )?;
        Ok(())
    }

    /// Insere un paiement et met a jour la facture associee (API legacy).
    pub fn insert_payment_and_update_invoice(&self, p: &PaymentRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        // Retrouver account_id depuis la facture
        let account_id: String = conn.query_row(
            "SELECT account_id FROM invoices WHERE id = ?1",
            params![p.invoice_id],
            |row| row.get(0),
        ).unwrap_or_else(|_| "default-account".to_string());
        conn.execute(
            "INSERT OR REPLACE INTO payments
             (id, account_id, invoice_id, amount, currency, method, reference_opaque, paid_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                p.id,
                account_id,
                p.invoice_id,
                p.amount,
                p.currency,
                p.method,
                p.reference_opaque,
                p.paid_at
            ],
        )?;
        // Le trigger trg_invoice_paid_insert met a jour paid_amount et status
        Ok(())
    }

    /// Insere un rappel (API legacy pour pipeline d'integration).
    pub fn insert_reminder(&self, r: &ReminderRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        // Utiliser le premier account disponible ou en creer un
        let account_id = self.ensure_account_exists_inner(&conn, "purse")?;
        conn.execute(
            "INSERT OR REPLACE INTO reminders
             (id, account_id, deadline_ref, due_at, label, context_ref, source_service)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                r.id,
                account_id,
                r.deadline_ref,
                r.due_at,
                r.label,
                r.context_ref,
                r.source_service
            ],
        )?;
        Ok(())
    }

    /// Insere une entree d'audit (API legacy).
    pub fn insert_audit(&self, a: &AuditRecord) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let account_id = self.first_account_id_inner(&conn)?;
        conn.execute(
            "INSERT INTO audit
             (id, account_id, contract_id, actor_ref, operation, scope, object_ref, result, payload, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                a.id,
                account_id,
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

    // ═══════════════════════════════════════════════════════════
    // Legacy stats (compatible avec backend/service.rs)
    // ═══════════════════════════════════════════════════════════

    /// Retourne les statistiques Account.
    pub fn account_stats(&self) -> Result<AccountStats, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let month_key = Local::now().format("%Y-%m").to_string();

        let ca_mensuel: f64 = conn.query_row(
            "SELECT COALESCE(SUM(total_ttc), 0) FROM invoices
             WHERE substr(issued_at, 1, 7)=?1",
            params![month_key],
            |row| row.get(0),
        ).unwrap_or(0.0);

        let impayes: f64 = conn.query_row(
            "SELECT COALESCE(SUM(total_ttc - paid_amount), 0) FROM invoices
             WHERE status IN ('issued','sent','partial','overdue')",
            [],
            |row| row.get(0),
        ).unwrap_or(0.0);

        let invoices_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM invoices",
            [],
            |row| row.get(0),
        ).unwrap_or(0);

        let paid_sum: f64 = conn.query_row(
            "SELECT COALESCE(SUM(paid_amount), 0) FROM invoices",
            [],
            |row| row.get(0),
        ).unwrap_or(0.0);
        let total_sum: f64 = conn.query_row(
            "SELECT COALESCE(SUM(total_ttc), 0) FROM invoices",
            [],
            |row| row.get(0),
        ).unwrap_or(0.0);
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
            "SELECT COALESCE(SUM(ABS(amount)), 0) FROM movements
             WHERE scope='purse' AND amount < 0 AND substr(movement_date, 1, 7)=?1",
            params![month_key],
            |row| row.get(0),
        ).unwrap_or(0.0);

        let budget_in: f64 = conn.query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM movements
             WHERE scope='purse' AND amount > 0",
            [],
            |row| row.get(0),
        ).unwrap_or(0.0);
        let budget_out: f64 = conn.query_row(
            "SELECT COALESCE(SUM(ABS(amount)), 0) FROM movements
             WHERE scope='purse' AND amount < 0",
            [],
            |row| row.get(0),
        ).unwrap_or(0.0);
        let budget_usage_pct = if budget_in > 0.0 {
            (budget_out / budget_in * 100.0).min(999.0)
        } else {
            0.0
        };

        let active_goals: i64 = conn.query_row(
            "SELECT COUNT(*) FROM goals WHERE status='active'",
            [],
            |row| row.get(0),
        ).unwrap_or(0);

        let total_moves: i64 = conn.query_row(
            "SELECT COUNT(*) FROM movements WHERE scope='purse'",
            [],
            |row| row.get(0),
        ).unwrap_or(0);
        let categorized_moves: i64 = conn.query_row(
            "SELECT COUNT(*) FROM movements
             WHERE scope='purse' AND category_id IS NOT NULL",
            [],
            |row| row.get(0),
        ).unwrap_or(0);
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

    /// Solde net du Purse (somme algebrique des mouvements scope=purse).
    pub fn solde_purse(&self) -> Result<f64, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let solde: f64 = conn.query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM movements WHERE scope='purse'",
            [],
            |row| row.get(0),
        )?;
        Ok(solde)
    }

    /// Liste les mouvements recents (tout scope, ordre decroissant).
    pub fn movements_recent(&self, limit: i64) -> Result<Vec<MovementRecord>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT m.id, m.scope, COALESCE(m.context_ref, ''), COALESCE(m.description, ''),
                    m.amount, m.currency, m.movement_date, COALESCE(m.source_service, 'manual')
             FROM movements m ORDER BY m.movement_date DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(MovementRecord {
                id: row.get(0)?,
                scope: row.get(1)?,
                context_ref: row.get(2)?,
                category: row.get(3)?,
                amount: row.get(4)?,
                currency: row.get(5)?,
                movement_date: row.get(6)?,
                source_service: row.get(7)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Liste les factures recentes (tout scope, ordre decroissant).
    pub fn invoices_recent(&self, limit: i64) -> Result<Vec<InvoiceRecord>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT i.id, 'account', COALESCE(i.context_ref, ''),
                    COALESCE((SELECT c.name FROM counterparties c WHERE c.id = i.counterparty_id), ''),
                    i.quote_id, i.total_ttc, i.paid_amount, i.currency, i.status, i.issued_at, i.due_at
             FROM invoices i ORDER BY i.issued_at DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(InvoiceRecord {
                id: row.get(0)?,
                scope: row.get(1)?,
                context_ref: row.get(2)?,
                counterparty_ref: row.get(3)?,
                quote_id: row.get(4)?,
                total: row.get(5)?,
                paid_amount: row.get(6)?,
                currency: row.get(7)?,
                status: row.get(8)?,
                issued_at: row.get(9)?,
                due_at: row.get(10)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Nombre d'ecritures d'audit.
    pub fn audit_count(&self) -> Result<i64, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count = conn.query_row("SELECT COUNT(*) FROM audit", [], |row| row.get(0))?;
        Ok(count)
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Mouvements
    // ═══════════════════════════════════════════════════════════

    /// Cree un mouvement Purse.
    pub fn create_purse_movement(
        &self,
        account_id: &str,
        id: &str,
        amount: f64,
        category_id: Option<&str>,
        description: Option<&str>,
        movement_date: &str,
        budget_id: Option<&str>,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO movements
             (id, account_id, scope, category_id, amount, currency, description,
              movement_date, source_service, budget_id)
             VALUES (?1, ?2, 'purse', ?3, ?4, 'EUR', ?5, ?6, 'manual', ?7)",
            params![id, account_id, category_id, amount, description, movement_date, budget_id],
        )?;
        Ok(())
    }

    /// Liste les mouvements Purse filtres et pagines.
    pub fn list_purse_movements(
        &self,
        account_id: &str,
        filters: &MovementFilters,
    ) -> Result<(Vec<Movement>, u32), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;

        // Construire la clause WHERE
        let mut where_clauses = vec!["m.scope = 'purse'".to_string(), "m.account_id = ?1".to_string()];
        let mut param_idx = 2u32;

        if filters.month.is_some() {
            where_clauses.push(format!("substr(m.movement_date, 1, 7) = ?{param_idx}"));
            param_idx += 1;
        }
        if filters.category_id.is_some() {
            where_clauses.push(format!("m.category_id = ?{param_idx}"));
            param_idx += 1;
        }
        if let Some(mt) = &filters.movement_type {
            match mt {
                MovementType::Expense => where_clauses.push("m.amount < 0".to_string()),
                MovementType::Income => where_clauses.push("m.amount > 0".to_string()),
            }
        }
        if filters.budget_id.is_some() {
            where_clauses.push(format!("m.budget_id = ?{param_idx}"));
            // param_idx += 1; // (dernier, pas besoin d'incrementer)
        }

        let where_sql = where_clauses.join(" AND ");
        let per_page = if filters.per_page == 0 { 20 } else { filters.per_page };
        let offset = filters.page * per_page;

        // Compter le total
        let count_sql = format!("SELECT COUNT(*) FROM movements m WHERE {where_sql}");
        let list_sql = format!(
            "SELECT m.id, m.amount, m.currency, m.category_id, c.name AS cat_name,
                    m.description, m.movement_date, m.budget_id, b.name AS budget_name,
                    m.source_service, m.context_ref, m.created_at
             FROM movements m
             LEFT JOIN categories c ON m.category_id = c.id
             LEFT JOIN budgets b ON m.budget_id = b.id
             WHERE {where_sql}
             ORDER BY m.movement_date DESC
             LIMIT {per_page} OFFSET {offset}"
        );

        // On doit passer les parametres dynamiquement
        // Approche simplifiee : utiliser des queries separees par filtre
        let total_count: u32 = {
            let mut stmt = conn.prepare(&count_sql)?;
            let mut p_idx = 1;
            stmt.raw_bind_parameter(p_idx, account_id)?; p_idx += 1;
            if let Some(m) = &filters.month { stmt.raw_bind_parameter(p_idx, m.as_str())?; p_idx += 1; }
            if let Some(c) = &filters.category_id { stmt.raw_bind_parameter(p_idx, c.as_str())?; p_idx += 1; }
            if let Some(b) = &filters.budget_id { stmt.raw_bind_parameter(p_idx, b.as_str())?; }
            let mut rows = stmt.raw_query();
            if let Some(row) = rows.next()? {
                row.get::<_, i64>(0)? as u32
            } else {
                0
            }
        };

        let movements = {
            let mut stmt = conn.prepare(&list_sql)?;
            let mut p_idx = 1;
            stmt.raw_bind_parameter(p_idx, account_id)?; p_idx += 1;
            if let Some(m) = &filters.month { stmt.raw_bind_parameter(p_idx, m.as_str())?; p_idx += 1; }
            if let Some(c) = &filters.category_id { stmt.raw_bind_parameter(p_idx, c.as_str())?; p_idx += 1; }
            if let Some(b) = &filters.budget_id { stmt.raw_bind_parameter(p_idx, b.as_str())?; }
            let mut rows = stmt.raw_query();
            let mut items = Vec::new();
            while let Some(row) = rows.next()? {
                let cat_id: Option<String> = row.get(3)?;
                let cat_name: Option<String> = row.get(4)?;
                let budget_id: Option<String> = row.get(7)?;
                let budget_name: Option<String> = row.get(8)?;
                items.push(Movement {
                    id: row.get(0)?,
                    amount: row.get(1)?,
                    currency: row.get(2)?,
                    category: cat_id.zip(cat_name).map(|(id, name)| CategoryRef { id, name }),
                    description: row.get(5)?,
                    movement_date: row.get(6)?,
                    budget: budget_id.zip(budget_name).map(|(id, name)| BudgetRef { id, name }),
                    source_service: row.get(9)?,
                    context_ref: row.get(10)?,
                    created_at: row.get::<_, String>(11).unwrap_or_default(),
                });
            }
            items
        };

        Ok((movements, total_count))
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Categories
    // ═══════════════════════════════════════════════════════════

    /// Liste les categories d'un compte.
    pub fn list_categories(&self, account_id: &str) -> Result<Vec<Category>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, icon, color, parent_id, is_income, sort_order
             FROM categories WHERE account_id = ?1 ORDER BY sort_order, name",
        )?;
        let rows = stmt.query_map(params![account_id], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                icon: row.get(2)?,
                color: row.get(3)?,
                parent_id: row.get(4)?,
                is_income: row.get::<_, i32>(5)? != 0,
                sort_order: row.get(6)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Cree une categorie.
    pub fn create_category(
        &self,
        id: &str,
        account_id: &str,
        name: &str,
        icon: Option<&str>,
        color: Option<&str>,
        is_income: bool,
        sort_order: i32,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT OR IGNORE INTO categories (id, account_id, name, icon, color, is_income, sort_order)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, account_id, name, icon, color, is_income as i32, sort_order],
        )?;
        Ok(())
    }

    /// Initialise les categories par defaut Purse si aucune n'existe.
    pub fn ensure_default_categories(&self, account_id: &str) -> Result<(), DbError> {
        use crate::domain::purse::DEFAULT_PURSE_CATEGORIES;
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM categories WHERE account_id = ?1",
            params![account_id],
            |row| row.get(0),
        )?;
        if count > 0 {
            return Ok(());
        }
        for (i, (name, icon, is_income)) in DEFAULT_PURSE_CATEGORIES.iter().enumerate() {
            let cat_id = format!("cat-default-{}", name.to_lowercase().replace(' ', "-"));
            conn.execute(
                "INSERT OR IGNORE INTO categories (id, account_id, name, icon, is_income, sort_order)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![cat_id, account_id, name, icon, *is_income as i32, i as i32],
            )?;
        }
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Budgets occasionnels
    // ═══════════════════════════════════════════════════════════

    /// Liste les budgets occasionnels d'un compte.
    pub fn list_budgets(&self, account_id: &str) -> Result<Vec<OccasionalBudget>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT b.id, b.name, b.target, b.spent, b.currency, b.start_date, b.end_date, b.status,
                    (SELECT COUNT(*) FROM movements m WHERE m.budget_id = b.id) AS moves_count
             FROM budgets b WHERE b.account_id = ?1 ORDER BY b.created_at DESC",
        )?;
        let rows = stmt.query_map(params![account_id], |row| {
            let target: f64 = row.get(2)?;
            let spent: f64 = row.get(3)?;
            let remaining = (target - spent).max(0.0);
            let progress_pct = if target > 0.0 { (spent / target * 100.0).min(100.0) } else { 0.0 };
            Ok(OccasionalBudget {
                id: row.get(0)?,
                name: row.get(1)?,
                target,
                spent,
                remaining,
                progress_pct,
                currency: row.get(4)?,
                start_date: row.get(5)?,
                end_date: row.get(6)?,
                status: BudgetStatus::from_str(&row.get::<_, String>(7)?),
                movements_count: row.get::<_, i64>(8)? as u32,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Cree un budget occasionnel.
    pub fn create_budget(
        &self,
        id: &str,
        account_id: &str,
        name: &str,
        target: f64,
        start_date: &str,
        end_date: Option<&str>,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO budgets (id, account_id, name, target, start_date, end_date)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, account_id, name, target, start_date, end_date],
        )?;
        Ok(())
    }

    /// Cloture un budget occasionnel.
    pub fn close_budget(&self, budget_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE budgets SET status = 'closed' WHERE id = ?1",
            params![budget_id],
        )?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Objectifs
    // ═══════════════════════════════════════════════════════════

    /// Liste les objectifs d'un compte.
    pub fn list_goals(&self, account_id: &str) -> Result<Vec<Goal>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, target, current, currency, goal_type, deadline, status
             FROM goals WHERE account_id = ?1 ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map(params![account_id], |row| {
            let target: f64 = row.get(2)?;
            let current: f64 = row.get(3)?;
            let progress_pct = if target > 0.0 { (current / target * 100.0).min(100.0) } else { 0.0 };
            let goal_type = GoalType::from_str(&row.get::<_, String>(5)?);
            let deadline: Option<String> = row.get(6)?;
            // Calcul montant mensuel requis si deadline
            let monthly_required = deadline.as_ref().and_then(|d| {
                let remaining = target - current;
                if remaining <= 0.0 { return None; }
                // Estimation simplifiee : mois restants
                let now = Local::now().format("%Y-%m").to_string();
                let dl_month = &d[..7.min(d.len())];
                if dl_month <= now.as_str() { return Some(remaining); }
                // Approximation nombre de mois
                Some(remaining / 6.0) // placeholder
            });
            let is_on_track = monthly_required.map_or(true, |mr| mr < target * 0.3);
            Ok(Goal {
                id: row.get(0)?,
                name: row.get(1)?,
                target,
                current,
                progress_pct,
                currency: row.get(4)?,
                goal_type,
                deadline,
                status: GoalStatus::from_str(&row.get::<_, String>(7)?),
                monthly_required,
                is_on_track,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Cree un objectif.
    pub fn create_goal(
        &self,
        id: &str,
        account_id: &str,
        name: &str,
        target: f64,
        goal_type: &str,
        deadline: Option<&str>,
    ) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO goals (id, account_id, name, target, goal_type, deadline)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, account_id, name, target, goal_type, deadline],
        )?;
        Ok(())
    }

    /// Met a jour la progression d'un objectif.
    pub fn update_goal_progress(&self, goal_id: &str, amount: f64) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE goals SET current = current + ?1 WHERE id = ?2",
            params![amount, goal_id],
        )?;
        // Auto-mark reached si cible atteinte
        conn.execute(
            "UPDATE goals SET status = 'reached'
             WHERE id = ?1 AND current >= target AND status = 'active'",
            params![goal_id],
        )?;
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Dashboard
    // ═══════════════════════════════════════════════════════════

    /// Repartition par categorie pour le mois courant.
    pub fn categories_breakdown_purse(
        &self,
        account_id: &str,
        month: &str,
    ) -> Result<Vec<CategoryBreakdown>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let total_expenses: f64 = conn.query_row(
            "SELECT COALESCE(SUM(ABS(amount)), 0) FROM movements
             WHERE scope='purse' AND account_id=?1 AND amount < 0
             AND substr(movement_date, 1, 7)=?2",
            params![account_id, month],
            |row| row.get(0),
        ).unwrap_or(0.0);

        let mut stmt = conn.prepare(
            "SELECT COALESCE(c.name, 'Sans categorie'), SUM(ABS(m.amount))
             FROM movements m
             LEFT JOIN categories c ON m.category_id = c.id
             WHERE m.scope='purse' AND m.account_id=?1 AND m.amount < 0
             AND substr(m.movement_date, 1, 7)=?2
             GROUP BY c.name ORDER BY SUM(ABS(m.amount)) DESC",
        )?;
        let rows = stmt.query_map(params![account_id, month], |row| {
            let amount: f64 = row.get(1)?;
            let percentage = if total_expenses > 0.0 { amount / total_expenses * 100.0 } else { 0.0 };
            Ok(CategoryBreakdown {
                category_name: row.get(0)?,
                amount,
                percentage,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Revenus et depenses du mois pour Purse.
    pub fn purse_month_totals(&self, account_id: &str, month: &str) -> Result<(f64, f64), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let income: f64 = conn.query_row(
            "SELECT COALESCE(SUM(amount), 0) FROM movements
             WHERE scope='purse' AND account_id=?1 AND amount > 0
             AND substr(movement_date, 1, 7)=?2",
            params![account_id, month],
            |row| row.get(0),
        ).unwrap_or(0.0);
        let expense: f64 = conn.query_row(
            "SELECT COALESCE(SUM(ABS(amount)), 0) FROM movements
             WHERE scope='purse' AND account_id=?1 AND amount < 0
             AND substr(movement_date, 1, 7)=?2",
            params![account_id, month],
            |row| row.get(0),
        ).unwrap_or(0.0);
        Ok((income, expense))
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Transactions recurrentes
    // ═══════════════════════════════════════════════════════════

    /// Liste les transactions recurrentes d'un compte (scope purse).
    pub fn list_recurring(
        &self,
        account_id: &str,
        active_only: bool,
    ) -> Result<Vec<RecurringTransaction>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let sql = if active_only {
            "SELECT r.id, r.direction, r.amount, r.currency, r.category_id, c.name AS cat_name,
                    r.description, r.frequency, r.day_of_month, r.day_of_week,
                    r.start_date, r.end_date, r.next_due_date, r.is_active, r.auto_create,
                    r.notes, r.created_at
             FROM recurring_transactions r
             LEFT JOIN categories c ON r.category_id = c.id
             WHERE r.account_id = ?1 AND r.scope = 'purse' AND r.is_active = 1
             ORDER BY r.next_due_date ASC"
        } else {
            "SELECT r.id, r.direction, r.amount, r.currency, r.category_id, c.name AS cat_name,
                    r.description, r.frequency, r.day_of_month, r.day_of_week,
                    r.start_date, r.end_date, r.next_due_date, r.is_active, r.auto_create,
                    r.notes, r.created_at
             FROM recurring_transactions r
             LEFT JOIN categories c ON r.category_id = c.id
             WHERE r.account_id = ?1 AND r.scope = 'purse'
             ORDER BY r.is_active DESC, r.next_due_date ASC"
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![account_id], |row| {
            let cat_id: Option<String> = row.get(4)?;
            let cat_name: Option<String> = row.get(5)?;
            Ok(RecurringTransaction {
                id: row.get(0)?,
                direction: RecurringDirection::from_str(&row.get::<_, String>(1)?),
                amount: row.get(2)?,
                currency: row.get(3)?,
                category: cat_id.zip(cat_name).map(|(id, name)| CategoryRef { id, name }),
                description: row.get(6)?,
                frequency: RecurringFrequency::from_str(&row.get::<_, String>(7)?),
                day_of_month: row.get(8)?,
                day_of_week: row.get(9)?,
                start_date: row.get(10)?,
                end_date: row.get(11)?,
                next_due_date: row.get(12)?,
                is_active: row.get::<_, i32>(13)? != 0,
                auto_create: row.get::<_, i32>(14)? != 0,
                notes: row.get(15)?,
                created_at: row.get(16)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Cree une transaction recurrente.
    pub fn create_recurring(
        &self,
        id: &str,
        account_id: &str,
        direction: &str,
        amount: f64,
        category_id: Option<&str>,
        description: &str,
        frequency: &str,
        day_of_month: Option<i32>,
        day_of_week: Option<i32>,
        start_date: &str,
        end_date: Option<&str>,
        next_due_date: &str,
        auto_create: bool,
        notes: Option<&str>,
    ) -> Result<(), DbError> {
        if account_id.is_empty() {
            return Err(DbError("account_id manquant (compte Purse non initialisé)".to_string()));
        }
        // Chaîne vide pour category_id viole la FK : on insère NULL si absent ou vide.
        let category_id_null = category_id
            .and_then(|s| if s.trim().is_empty() { None } else { Some(s) });
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO recurring_transactions
             (id, account_id, scope, direction, amount, currency, category_id, description,
              frequency, day_of_month, day_of_week, start_date, end_date, next_due_date,
              is_active, auto_create, notes)
             VALUES (?1, ?2, 'purse', ?3, ?4, 'EUR', ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, 1, ?13, ?14)",
            params![
                id, account_id, direction, amount, category_id_null, description,
                frequency, day_of_month, day_of_week, start_date, end_date, next_due_date,
                auto_create as i32, notes
            ],
        )?;
        Ok(())
    }

    /// Desactive (suspend) une transaction recurrente.
    pub fn toggle_recurring(&self, recurring_id: &str, active: bool) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE recurring_transactions SET is_active = ?1 WHERE id = ?2",
            params![active as i32, recurring_id],
        )?;
        Ok(())
    }

    /// Supprime une transaction recurrente.
    pub fn delete_recurring(&self, recurring_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "DELETE FROM recurring_transactions WHERE id = ?1",
            params![recurring_id],
        )?;
        Ok(())
    }

    /// Totaux mensuels estimes des recurrents actifs d'un compte.
    /// Retourne (monthly_income, monthly_expense).
    pub fn recurring_monthly_totals(&self, account_id: &str) -> Result<(f64, f64), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT direction, amount, frequency FROM recurring_transactions
             WHERE account_id = ?1 AND scope = 'purse' AND is_active = 1",
        )?;
        let mut income = 0.0;
        let mut expense = 0.0;
        let mut rows = stmt.query(params![account_id])?;
        while let Some(row) = rows.next()? {
            let dir: String = row.get(0)?;
            let amount: f64 = row.get(1)?;
            let freq: String = row.get(2)?;
            // Normaliser en montant mensuel
            let monthly = match freq.as_str() {
                "weekly" => amount * 52.0 / 12.0,
                "biweekly" => amount * 26.0 / 12.0,
                "monthly" => amount,
                "quarterly" => amount / 3.0,
                "yearly" => amount / 12.0,
                _ => amount,
            };
            if dir == "income" {
                income += monthly;
            } else {
                expense += monthly;
            }
        }
        Ok((income, expense))
    }

    // ═══════════════════════════════════════════════════════════
    // Comptes — gestion
    // ═══════════════════════════════════════════════════════════

    /// Retourne ou cree un account pour un scope donne.
    pub fn ensure_account(&self, scope: &str, user_id: &str, label: &str) -> Result<String, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        if let Ok(id) = conn.query_row(
            "SELECT id FROM accounts WHERE scope=?1 AND user_id=?2",
            params![scope, user_id],
            |row| row.get::<_, String>(0),
        ) {
            return Ok(id);
        }
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO accounts (id, scope, user_id, label) VALUES (?1, ?2, ?3, ?4)",
            params![id, scope, user_id, label],
        )?;
        Ok(id)
    }

    /// Retourne l'account_id pour un scope (interne, avec connexion deja lockee).
    fn ensure_account_exists_inner(&self, conn: &Connection, scope: &str) -> Result<String, DbError> {
        if let Ok(id) = conn.query_row(
            "SELECT id FROM accounts WHERE scope=?1",
            params![scope],
            |row| row.get::<_, String>(0),
        ) {
            return Ok(id);
        }
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO accounts (id, scope, user_id, label) VALUES (?1, ?2, 'default', ?3)",
            params![id, scope, format!("JayKonta {}", scope)],
        )?;
        Ok(id)
    }

    /// Retourne le premier account_id existant ou en cree un.
    fn first_account_id_inner(&self, conn: &Connection) -> Result<String, DbError> {
        if let Ok(id) = conn.query_row(
            "SELECT id FROM accounts LIMIT 1",
            [],
            |row| row.get::<_, String>(0),
        ) {
            return Ok(id);
        }
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO accounts (id, scope, user_id, label) VALUES (?1, 'account', 'default', 'JayKonta')",
            params![id],
        )?;
        Ok(id)
    }

    /// Assure qu'une contrepartie existe pour une reference (interne, avec connexion deja lockee).
    fn ensure_counterparty_inner(&self, conn: &Connection, account_id: &str, cp_ref: &str) -> Result<String, DbError> {
        if let Ok(id) = conn.query_row(
            "SELECT id FROM counterparties WHERE account_id=?1 AND name=?2",
            params![account_id, cp_ref],
            |row| row.get::<_, String>(0),
        ) {
            return Ok(id);
        }
        let id = uuid::Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO counterparties (id, account_id, name) VALUES (?1, ?2, ?3)",
            params![id, account_id, cp_ref],
        )?;
        Ok(id)
    }
}
