//! Base de données JayKonta via KindMother Client.
//!
//! Cette implémentation utilise exclusivement `kindmother-client` pour accéder aux données.
//! KindMother est le seul interlocuteur autorisé de la base de données.
//!
//! L'API publique est **synchrone** pour maintenir la compatibilité avec le code existant.
//! En interne, les appels async sont exécutés via le runtime Tokio.
//!
//! @id: jaykonta_kindmother_client_db
//! @do: persist_jaykonta_data_via_kindmother_client
//! @layer: infra

use crate::data::types::{
    AuditRecord, InvoiceRecord, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord,
};
use chrono::Local;
use kindmother_client::{ClientError, KindMotherClient};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::OnceCell;
use std::sync::OnceLock;

/// Erreur de base de données JayKonta (KindMother Client).
#[derive(Error, Debug)]
pub enum DbError {
    /// Erreur du client KindMother.
    #[error("KindMother client error: {0}")]
    Client(#[from] ClientError),

    /// Erreur de sérialisation.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Erreur interne.
    #[error("Internal error: {0}")]
    Internal(String),

    /// Client non initialisé.
    #[error("Client not initialized")]
    NotInitialized,
}

/// Agrégats KPI Account.
#[derive(Debug, Clone, Default)]
pub struct AccountStats {
    /// Total facturé ce mois.
    pub ca_mensuel: f64,
    /// Montant impayé.
    pub impayes: f64,
    /// Nombre de factures.
    pub invoices_count: i64,
    /// Taux de paiement.
    pub payment_rate_pct: f64,
}

/// Agrégats KPI Purse.
#[derive(Debug, Clone, Default)]
pub struct PurseStats {
    /// Dépenses mensuelles.
    pub monthly_expenses: f64,
    /// Pourcentage budget utilisé.
    pub budget_usage_pct: f64,
    /// Objectifs actifs.
    pub active_goals: i64,
    /// Mouvements catégorisés.
    pub categorized_pct: f64,
}

/// Adresse par défaut du service KindMother.
const DEFAULT_KINDMOTHER_ADDR: &str = "127.0.0.1:50051";

/// Client global partagé (singleton).
static CLIENT: OnceCell<Arc<KindMotherClient>> = OnceCell::const_new();

/// Runtime Tokio global pour exécuter les appels async.
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn get_runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
}

/// Exécute un future de manière bloquante, compatible avec un runtime tokio existant (Dioxus).
fn run_blocking<F: std::future::Future>(f: F) -> F::Output {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(f)),
        Err(_) => run_blocking(f),
    }
}

/// Base de données JayKonta via KindMother Client.
/// 
/// Fournit une API synchrone pour la compatibilité avec le code existant.
#[derive(Clone)]
pub struct JayKontaDb {
    client: Arc<KindMotherClient>,
}

impl JayKontaDb {
    /// Initialise la connexion globale au service KindMother (synchrone).
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), DbError> {
        run_blocking(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale au service KindMother (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), DbError> {
        let addr = addr.unwrap_or(DEFAULT_KINDMOTHER_ADDR);
        let client = KindMotherClient::connect(addr, "jaykonta", "jaykonta").await?;

        CLIENT
            .set(Arc::new(client))
            .map_err(|_| DbError::Internal("Client already initialized".to_string()))?;

        Ok(())
    }

    /// Crée une nouvelle instance utilisant le client global.
    pub fn new() -> Result<Self, DbError> {
        let client = CLIENT.get().ok_or(DbError::NotInitialized)?.clone();
        Ok(Self { client })
    }

    /// Ouvre une connexion à la base de données (API compatible avec l'ancienne implémentation).
    /// Cette méthode initialise le client global si nécessaire.
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self, DbError> {
        // Initialiser le client global si pas déjà fait
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        // Initialiser le schéma
        db.init_schema_sync()?;
        Ok(db)
    }

    /// Crée une nouvelle instance avec une connexion personnalisée (async).
    pub async fn connect_async(addr: &str) -> Result<Self, DbError> {
        let client = KindMotherClient::connect(addr, "jaykonta", "jaykonta").await?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Initialise le schéma de la base de données (synchrone).
    pub fn init_schema_sync(&self) -> Result<(), DbError> {
        run_blocking(self.init_schema_async())
    }

    /// Initialise le schéma de la base de données (async).
    pub async fn init_schema_async(&self) -> Result<(), DbError> {
        let schema = r#"
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
        "#;

        let statements: Vec<&str> = schema
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for sql in statements {
            self.client
                .execute(sql, Vec::<String>::new(), "init_schema")
                .await?;
        }

        Ok(())
    }

    /// Indique si la DB est encore vide de flux d'intégration (synchrone).
    pub fn is_bootstrap_needed(&self) -> Result<bool, DbError> {
        run_blocking(self.is_bootstrap_needed_async())
    }

    /// Indique si la DB est encore vide de flux d'intégration (async).
    pub async fn is_bootstrap_needed_async(&self) -> Result<bool, DbError> {
        let rows = self
            .client
            .query("SELECT COUNT(*) as cnt FROM audit_log", Vec::<String>::new())
            .await?;

        let count = rows
            .first()
            .and_then(|r| Self::get_opt_i64(r, "cnt"))
            .unwrap_or(0);

        Ok(count == 0)
    }

    /// Insère un mouvement (synchrone).
    pub fn insert_movement(&self, m: &MovementRecord) -> Result<(), DbError> {
        run_blocking(self.insert_movement_async(m))
    }

    /// Insère un mouvement (async).
    pub async fn insert_movement_async(&self, m: &MovementRecord) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT OR REPLACE INTO ledger_movements
                 (id, scope, context_ref, category, amount, currency, movement_date, source_service)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                vec![
                    m.id.clone(),
                    m.scope.clone(),
                    m.context_ref.clone(),
                    m.category.clone(),
                    m.amount.to_string(),
                    m.currency.clone(),
                    m.movement_date.clone(),
                    m.source_service.clone(),
                ],
                "insert_movement",
            )
            .await?;
        Ok(())
    }

    /// Insère un devis (synchrone).
    pub fn insert_quote(&self, q: &QuoteRecord) -> Result<(), DbError> {
        run_blocking(self.insert_quote_async(q))
    }

    /// Insère un devis (async).
    pub async fn insert_quote_async(&self, q: &QuoteRecord) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT OR REPLACE INTO quotes
                 (id, scope, context_ref, counterparty_ref, total, currency, status, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                vec![
                    q.id.clone(),
                    q.scope.clone(),
                    q.context_ref.clone(),
                    q.counterparty_ref.clone(),
                    q.total.to_string(),
                    q.currency.clone(),
                    q.status.clone(),
                    q.created_at.clone(),
                ],
                "insert_quote",
            )
            .await?;
        Ok(())
    }

    /// Insère une facture (synchrone).
    pub fn insert_invoice(&self, i: &InvoiceRecord) -> Result<(), DbError> {
        run_blocking(self.insert_invoice_async(i))
    }

    /// Insère une facture (async).
    pub async fn insert_invoice_async(&self, i: &InvoiceRecord) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT OR REPLACE INTO invoices
                 (id, scope, context_ref, counterparty_ref, quote_id, total, paid_amount, currency, status, issued_at, due_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                vec![
                    i.id.clone(),
                    i.scope.clone(),
                    i.context_ref.clone(),
                    i.counterparty_ref.clone(),
                    i.quote_id.clone().unwrap_or_default(),
                    i.total.to_string(),
                    i.paid_amount.to_string(),
                    i.currency.clone(),
                    i.status.clone(),
                    i.issued_at.clone(),
                    i.due_at.clone().unwrap_or_default(),
                ],
                "insert_invoice",
            )
            .await?;
        Ok(())
    }

    /// Insère un paiement et met à jour la facture associée (synchrone).
    pub fn insert_payment_and_update_invoice(&self, p: &PaymentRecord) -> Result<(), DbError> {
        run_blocking(self.insert_payment_and_update_invoice_async(p))
    }

    /// Insère un paiement et met à jour la facture associée (async).
    pub async fn insert_payment_and_update_invoice_async(&self, p: &PaymentRecord) -> Result<(), DbError> {
        // Insert payment
        self.client
            .execute(
                "INSERT OR REPLACE INTO payments
                 (id, invoice_id, amount, currency, paid_at, method, reference_opaque)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                vec![
                    p.id.clone(),
                    p.invoice_id.clone(),
                    p.amount.to_string(),
                    p.currency.clone(),
                    p.paid_at.clone(),
                    p.method.clone(),
                    p.reference_opaque.clone(),
                ],
                "insert_payment",
            )
            .await?;

        // Update invoice
        self.client
            .execute(
                "UPDATE invoices
                 SET paid_amount = COALESCE(paid_amount, 0) + ?1,
                     status = CASE
                        WHEN COALESCE(paid_amount, 0) + ?1 >= total THEN 'paid'
                        WHEN COALESCE(paid_amount, 0) + ?1 > 0 THEN 'partial'
                        ELSE status
                     END
                 WHERE id = ?2",
                vec![p.amount.to_string(), p.invoice_id.clone()],
                "update_invoice_payment",
            )
            .await?;

        Ok(())
    }

    /// Insère un rappel (synchrone).
    pub fn insert_reminder(&self, r: &ReminderRecord) -> Result<(), DbError> {
        run_blocking(self.insert_reminder_async(r))
    }

    /// Insère un rappel (async).
    pub async fn insert_reminder_async(&self, r: &ReminderRecord) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT OR REPLACE INTO reminders
                 (id, deadline_ref, due_at, label, context_ref, source_service)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                vec![
                    r.id.clone(),
                    r.deadline_ref.clone(),
                    r.due_at.clone(),
                    r.label.clone(),
                    r.context_ref.clone(),
                    r.source_service.clone(),
                ],
                "insert_reminder",
            )
            .await?;
        Ok(())
    }

    /// Insère une entrée d'audit (synchrone).
    pub fn insert_audit(&self, a: &AuditRecord) -> Result<(), DbError> {
        run_blocking(self.insert_audit_async(a))
    }

    /// Insère une entrée d'audit (async).
    pub async fn insert_audit_async(&self, a: &AuditRecord) -> Result<(), DbError> {
        self.client
            .execute(
                "INSERT INTO audit_log
                 (id, contract_id, actor_ref, operation, scope, object_ref, result, payload_json, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                vec![
                    a.id.clone(),
                    a.contract_id.clone(),
                    a.actor_ref.clone(),
                    a.operation.clone(),
                    a.scope.clone(),
                    a.object_ref.clone(),
                    a.result.clone(),
                    a.payload_json.clone(),
                    a.created_at.clone(),
                ],
                "insert_audit",
            )
            .await?;
        Ok(())
    }

    /// Retourne les statistiques Account (synchrone).
    pub fn account_stats(&self) -> Result<AccountStats, DbError> {
        run_blocking(self.account_stats_async())
    }

    /// Retourne les statistiques Account (async).
    pub async fn account_stats_async(&self) -> Result<AccountStats, DbError> {
        let month_key = Local::now().format("%Y-%m").to_string();

        // CA mensuel
        let rows = self
            .client
            .query(
                "SELECT COALESCE(SUM(total), 0) as val FROM invoices
                 WHERE scope='account' AND substr(issued_at, 1, 7)=?1",
                vec![&month_key],
            )
            .await?;
        let ca_mensuel = rows
            .first()
            .and_then(|r| Self::get_opt_f64(r, "val"))
            .unwrap_or(0.0);

        // Impayés
        let rows = self
            .client
            .query(
                "SELECT COALESCE(SUM(MAX(total - paid_amount, 0)), 0) as val FROM invoices
                 WHERE scope='account' AND status IN ('issued','partial','overdue')",
                Vec::<String>::new(),
            )
            .await?;
        let impayes = rows
            .first()
            .and_then(|r| Self::get_opt_f64(r, "val"))
            .unwrap_or(0.0);

        // Nombre de factures
        let rows = self
            .client
            .query(
                "SELECT COUNT(*) as val FROM invoices WHERE scope='account'",
                Vec::<String>::new(),
            )
            .await?;
        let invoices_count = rows
            .first()
            .and_then(|r| Self::get_opt_i64(r, "val"))
            .unwrap_or(0);

        // Taux de paiement
        let rows = self
            .client
            .query(
                "SELECT COALESCE(SUM(paid_amount), 0) as paid, COALESCE(SUM(total), 0) as total FROM invoices WHERE scope='account'",
                Vec::<String>::new(),
            )
            .await?;
        let (paid_sum, total_sum) = rows
            .first()
            .map(|r| {
                (
                    Self::get_opt_f64(r, "paid").unwrap_or(0.0),
                    Self::get_opt_f64(r, "total").unwrap_or(0.0),
                )
            })
            .unwrap_or((0.0, 0.0));

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

    /// Retourne les statistiques Purse (synchrone).
    pub fn purse_stats(&self) -> Result<PurseStats, DbError> {
        run_blocking(self.purse_stats_async())
    }

    /// Retourne les statistiques Purse (async).
    pub async fn purse_stats_async(&self) -> Result<PurseStats, DbError> {
        let month_key = Local::now().format("%Y-%m").to_string();

        // Dépenses mensuelles
        let rows = self
            .client
            .query(
                "SELECT COALESCE(SUM(ABS(amount)), 0) as val FROM ledger_movements
                 WHERE scope='purse' AND amount < 0 AND substr(movement_date, 1, 7)=?1",
                vec![&month_key],
            )
            .await?;
        let monthly_expenses = rows
            .first()
            .and_then(|r| Self::get_opt_f64(r, "val"))
            .unwrap_or(0.0);

        // Budget usage
        let rows = self
            .client
            .query(
                "SELECT COALESCE(SUM(amount), 0) as val FROM ledger_movements
                 WHERE scope='purse' AND category='budget_in'",
                Vec::<String>::new(),
            )
            .await?;
        let budget_in = rows
            .first()
            .and_then(|r| Self::get_opt_f64(r, "val"))
            .unwrap_or(0.0);

        let rows = self
            .client
            .query(
                "SELECT COALESCE(SUM(ABS(amount)), 0) as val FROM ledger_movements
                 WHERE scope='purse' AND amount < 0",
                Vec::<String>::new(),
            )
            .await?;
        let budget_out = rows
            .first()
            .and_then(|r| Self::get_opt_f64(r, "val"))
            .unwrap_or(0.0);

        let budget_usage_pct = if budget_in > 0.0 {
            (budget_out / budget_in * 100.0).min(999.0)
        } else {
            0.0
        };

        // Objectifs actifs
        let rows = self
            .client
            .query(
                "SELECT COUNT(*) as val FROM reminders WHERE source_service='jaykoa'",
                Vec::<String>::new(),
            )
            .await?;
        let active_goals = rows
            .first()
            .and_then(|r| Self::get_opt_i64(r, "val"))
            .unwrap_or(0);

        // Mouvements catégorisés
        let rows = self
            .client
            .query(
                "SELECT COUNT(*) as total,
                        SUM(CASE WHEN category IS NOT NULL AND category != '' THEN 1 ELSE 0 END) as categorized
                 FROM ledger_movements WHERE scope='purse'",
                Vec::<String>::new(),
            )
            .await?;
        let (total_moves, categorized_moves) = rows
            .first()
            .map(|r| {
                (
                    Self::get_opt_i64(r, "total").unwrap_or(0),
                    Self::get_opt_i64(r, "categorized").unwrap_or(0),
                )
            })
            .unwrap_or((0, 0));

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

    /// Solde net du Purse (synchrone).
    pub fn solde_purse(&self) -> Result<f64, DbError> {
        run_blocking(self.solde_purse_async())
    }

    /// Solde net du Purse (async).
    pub async fn solde_purse_async(&self) -> Result<f64, DbError> {
        let rows = self
            .client
            .query(
                "SELECT COALESCE(SUM(amount), 0) as val FROM ledger_movements WHERE scope='purse'",
                Vec::<String>::new(),
            )
            .await?;

        Ok(rows
            .first()
            .and_then(|r| Self::get_opt_f64(r, "val"))
            .unwrap_or(0.0))
    }

    /// Liste les mouvements récents (synchrone).
    pub fn movements_recent(&self, limit: i64) -> Result<Vec<MovementRecord>, DbError> {
        run_blocking(self.movements_recent_async(limit))
    }

    /// Liste les mouvements récents (async).
    pub async fn movements_recent_async(&self, limit: i64) -> Result<Vec<MovementRecord>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, scope, context_ref, category, amount, currency, movement_date, source_service
                 FROM ledger_movements ORDER BY movement_date DESC LIMIT ?1",
                vec![limit.to_string()],
            )
            .await?;

        rows.iter().map(|r| Self::row_to_movement(r)).collect()
    }

    /// Liste les factures récentes (synchrone).
    pub fn invoices_recent(&self, limit: i64) -> Result<Vec<InvoiceRecord>, DbError> {
        run_blocking(self.invoices_recent_async(limit))
    }

    /// Liste les factures récentes (async).
    pub async fn invoices_recent_async(&self, limit: i64) -> Result<Vec<InvoiceRecord>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, scope, context_ref, counterparty_ref, quote_id, total, paid_amount, currency, status, issued_at, due_at
                 FROM invoices ORDER BY issued_at DESC LIMIT ?1",
                vec![limit.to_string()],
            )
            .await?;

        rows.iter().map(|r| Self::row_to_invoice(r)).collect()
    }

    /// Nombre d'écritures d'audit (synchrone).
    pub fn audit_count(&self) -> Result<i64, DbError> {
        run_blocking(self.audit_count_async())
    }

    /// Nombre d'écritures d'audit (async).
    pub async fn audit_count_async(&self) -> Result<i64, DbError> {
        let rows = self
            .client
            .query(
                "SELECT COUNT(*) as val FROM audit_log",
                Vec::<String>::new(),
            )
            .await?;

        Ok(rows
            .first()
            .and_then(|r| Self::get_opt_i64(r, "val"))
            .unwrap_or(0))
    }

    /// Vérification de la santé de la connexion (synchrone).
    pub fn health_check(&self) -> Result<bool, DbError> {
        run_blocking(self.health_check_async())
    }

    /// Vérification de la santé de la connexion (async).
    pub async fn health_check_async(&self) -> Result<bool, DbError> {
        self.client.health_check().await.map_err(DbError::from)
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn row_to_movement(row: &HashMap<String, serde_json::Value>) -> Result<MovementRecord, DbError> {
        Ok(MovementRecord {
            id: Self::get_string(row, "id"),
            scope: Self::get_string(row, "scope"),
            context_ref: Self::get_string(row, "context_ref"),
            category: Self::get_string(row, "category"),
            amount: Self::get_opt_f64(row, "amount").unwrap_or(0.0),
            currency: Self::get_string(row, "currency"),
            movement_date: Self::get_string(row, "movement_date"),
            source_service: Self::get_string(row, "source_service"),
        })
    }

    fn row_to_invoice(row: &HashMap<String, serde_json::Value>) -> Result<InvoiceRecord, DbError> {
        Ok(InvoiceRecord {
            id: Self::get_string(row, "id"),
            scope: Self::get_string(row, "scope"),
            context_ref: Self::get_string(row, "context_ref"),
            counterparty_ref: Self::get_string(row, "counterparty_ref"),
            quote_id: Self::get_opt_string(row, "quote_id"),
            total: Self::get_opt_f64(row, "total").unwrap_or(0.0),
            paid_amount: Self::get_opt_f64(row, "paid_amount").unwrap_or(0.0),
            currency: Self::get_string(row, "currency"),
            status: Self::get_string(row, "status"),
            issued_at: Self::get_string(row, "issued_at"),
            due_at: Self::get_opt_string(row, "due_at"),
        })
    }

    fn get_string(row: &HashMap<String, serde_json::Value>, key: &str) -> String {
        row.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    }

    fn get_opt_string(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
        row.get(key)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    }

    fn get_opt_i64(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<i64> {
        row.get(key).and_then(|v| v.as_i64())
    }

    fn get_opt_f64(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<f64> {
        row.get(key).and_then(|v| v.as_f64())
    }
}
