//! @id     jaykonta_kindmother_client_db
//! @do     persister_jaykonta_via_kindmother
//! @role   donnee
//! @layer  infra
//! @human  jaykonta — persistance via le service KindMother (seul écrivain, K-13).

//! Base de données JayKonta via KindMother Client (chemin par défaut, lot C, K-13).
//!
//! KindMother est le **seul écrivain** de la base `jaykonta` : ce module ne contient
//! aucun texte SQL d'écriture. Règles :
//!
//! - **Schéma** : identique au chemin hérité (`schema.sql`), appliqué par l'API de
//!   migrations versionnées du service ([`schema_migrations`]). Toute évolution future
//!   est une **nouvelle** `SchemaMigration` ajoutée en fin de liste (v2, v3…) : ne
//!   jamais modifier la v1, ne jamais supprimer de table de données (mêmes règles que
//!   `migrations.rs`, EX-08, C-41 ; le service refuse toute disparition de table, de
//!   colonne ou de ligne).
//! - **Écritures** : intentions typées seulement (`TypedWriteIntent`), lots atomiques
//!   (`submit_batch`) pour les écritures en plusieurs étapes. Les déclencheurs de
//!   `schema.sql` (cumul budget, montant réglé d'une facture, `updated_at`) s'exécutent
//!   côté service, comme sur le chemin hérité.
//! - **Lectures** : lecture typée (`TypedSelect`) pour les égalités simples ; les
//!   agrégats, jointures et intervalles passent encore par la lecture SQL brute
//!   transitoire (lecture seule, sous autorisateur SQLite), marquée à chaque usage.
//!
//! L'API publique est **synchrone** (parité avec `kindmother_db.rs`), avec des
//! variantes `*_async`. Tous les appels réseau s'exécutent sur le runtime global du
//! module (voir [`run_blocking`]).
//!
//! Erreurs : toute méthode retourne [`DbError`] (refus ou échec KindMother,
//! transport, client non initialisé) ; convention commune, non répétée par méthode.
#![allow(clippy::missing_errors_doc)]

use crate::data::types::{
    AuditRecord, InvoiceRecord, MovementRecord, PaymentRecord, QuoteRecord, ReminderRecord,
};
use crate::domain::purse::{
    BudgetRef, BudgetStatus, Category, CategoryBreakdown, CategoryRef, Goal, GoalStatus, GoalType,
    Movement, MovementFilters, MovementType, OccasionalBudget, RecurringDirection,
    RecurringFrequency, RecurringTransaction, DEFAULT_PURSE_CATEGORIES,
};
use chrono::Local;
use kindmother::{InstanceIdentity, InstanceType};
use kindmother_client::{
    ClientError, KindMotherClient, OperatorCredential, TypedSelect,
    TypedWriteIntent,
};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, OnceLock};
use thiserror::Error;
use tokio::sync::OnceCell;

/// Ligne renvoyée par KindMother (colonne → valeur JSON).
type Row = HashMap<String, Value>;

/// Erreur de base de données JayKonta (KindMother Client).
#[derive(Error, Debug)]
pub enum DbError {
    /// Erreur du client KindMother (refus, échec, transport).
    #[error("KindMother client error: {0}")]
    Client(#[from] ClientError),

    /// Erreur de sérialisation.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Erreur interne.
    #[error("Internal error: {0}")]
    Internal(String),

    /// Client global non initialisé.
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

/// Variable d'environnement qui surcharge l'adresse du service.
const ENV_KINDMOTHER_ADDR: &str = "KINDMOTHER_ADDR";

/// Identité de l'opérateur (et nom de la base) côté KindMother.
const OPERATOR_ID: &str = "jaykonta";

use super::schema_kindmother::schema_migrations;

/// Client global partagé (utilisé par [`JayKontaDb::open`]).
static CLIENT: OnceCell<Arc<KindMotherClient>> = OnceCell::const_new();

/// Runtime Tokio global : toutes les entrées/sorties KindMother y sont exécutées.
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn get_runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("jaykonta-kindmother")
            .enable_all()
            .build()
            .expect("JayKonta: création du runtime Tokio impossible")
    })
}

/// Exécute un future de manière bloquante sur le runtime global du module.
///
/// - Hors runtime : `block_on` direct sur le runtime global.
/// - Dans un runtime (Dioxus, test `#[tokio::test]`) : `block_on` d'un autre runtime
///   y est interdit ; le future est donc exécuté sur un fil dédié (portée bornée)
///   qui appelle le runtime global. Dans un runtime multi-fils, l'attente est
///   enveloppée dans `block_in_place` pour libérer l'ordonnanceur ; dans un runtime
///   `current_thread`, `block_in_place` paniquerait : on attend simplement le fil
///   (le runtime appelant est bloqué pendant l'appel, comme toute API synchrone).
fn run_blocking<F>(f: F) -> F::Output
where
    F: Future + Send,
    F::Output: Send,
{
    use tokio::runtime::{Handle, RuntimeFlavor};

    fn on_dedicated_thread<F>(f: F) -> F::Output
    where
        F: Future + Send,
        F::Output: Send,
    {
        std::thread::scope(|scope| {
            scope
                .spawn(|| get_runtime().block_on(f))
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
        })
    }
    match Handle::try_current() {
        Err(_) => get_runtime().block_on(f),
        Ok(handle) if handle.runtime_flavor() == RuntimeFlavor::MultiThread => {
            tokio::task::block_in_place(|| on_dedicated_thread(f))
        }
        Ok(_) => on_dedicated_thread(f),
    }
}

/// Horodatage équivalent à `datetime('now')` de SQLite (UTC, `AAAA-MM-JJ HH:MM:SS`).
fn now_sql() -> String {
    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Base de données JayKonta via KindMother Client.
#[derive(Clone)]
pub struct JayKontaDb {
    client: Arc<KindMotherClient>,
    /// Identité KindMother (toujours Daughter pour le service local).
    pub instance: InstanceIdentity,
}

impl JayKontaDb {
    /// Version courante du schéma JayKonta côté KindMother (nombre de migrations).
    ///
    /// Distincte de `migrations::VERSION_SCHEMA` (chemin hérité, `PRAGMA user_version`).
    pub const SCHEMA_VERSION: i32 = 1;

    // ═══════════════════════════════════════════════════════════
    // Connexion
    // ═══════════════════════════════════════════════════════════

    /// Initialise la connexion globale (synchrone). `addr` absent : `KINDMOTHER_ADDR`
    /// ou `127.0.0.1:50051` ; justificatif : `OperatorCredential::load("jaykonta")`.
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), DbError> {
        run_blocking(Self::init_global_async(addr)).map(|_| ())
    }

    /// Initialise la connexion globale (async) ; idempotent.
    pub async fn init_global_async(addr: Option<&str>) -> Result<Arc<KindMotherClient>, DbError> {
        let addr = addr.map_or_else(
            || std::env::var(ENV_KINDMOTHER_ADDR).unwrap_or_else(|_| DEFAULT_KINDMOTHER_ADDR.to_string()),
            str::to_string,
        );
        CLIENT
            .get_or_try_init(|| async {
                let credential = OperatorCredential::load(OPERATOR_ID)?;
                let client = KindMotherClient::connect(&addr, &credential, OPERATOR_ID).await?;
                Ok::<_, DbError>(Arc::new(client))
            })
            .await
            .cloned()
    }

    /// Instance sur le client global (déjà initialisé), sans appliquer les migrations.
    pub fn new() -> Result<Self, DbError> {
        let client = CLIENT.get().ok_or(DbError::NotInitialized)?.clone();
        Ok(Self::from_client(client))
    }

    fn from_client(client: Arc<KindMotherClient>) -> Self {
        Self {
            client,
            instance: InstanceIdentity::new(InstanceType::Daughter),
        }
    }

    /// Ouvre la base JayKonta (API compatible avec le chemin hérité).
    ///
    /// `_path` est ignoré : la base est `jaykonta` côté service KindMother. Initialise le
    /// client global au besoin puis applique les migrations versionnées.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, DbError> {
        kindmother_client::heritage::refuser_si_base_heritee(path.as_ref())
            .map_err(DbError::Internal)?;
        run_blocking(async {
            let client = Self::init_global_async(None).await?;
            let db = Self::from_client(client);
            db.init_schema_async().await?;
            Ok(db)
        })
    }

    /// Connexion injectée (tests, outils) : client dédié, hors singleton global ;
    /// applique les migrations.
    pub fn connect_with(addr: &str, credential: &OperatorCredential) -> Result<Self, DbError> {
        run_blocking(Self::connect_with_async(addr, credential))
    }

    /// Variante async de [`Self::connect_with`].
    pub async fn connect_with_async(
        addr: &str,
        credential: &OperatorCredential,
    ) -> Result<Self, DbError> {
        let client = KindMotherClient::connect(addr, credential, OPERATOR_ID).await?;
        let db = Self::from_client(Arc::new(client));
        db.init_schema_async().await?;
        Ok(db)
    }

    /// Applique les migrations versionnées (synchrone) ; retourne `(avant, après)`.
    pub fn init_schema_sync(&self) -> Result<(u32, u32), DbError> {
        run_blocking(self.init_schema_async())
    }

    /// Applique les migrations versionnées (async, idempotent).
    pub async fn init_schema_async(&self) -> Result<(u32, u32), DbError> {
        let (before, after) = self.client.apply_migrations(schema_migrations()).await?;
        if before != after {
            eprintln!("[jaykonta] schéma KindMother migré v{before} -> v{after}");
        }
        Ok((before, after))
    }

    /// Vérification de la santé de la connexion.
    pub fn health_check(&self) -> Result<bool, DbError> {
        run_blocking(self.health_check_async())
    }

    /// Variante async de [`Self::health_check`].
    pub async fn health_check_async(&self) -> Result<bool, DbError> {
        Ok(self.client.health_check().await?)
    }

    // ═══════════════════════════════════════════════════════════
    // Primitives d'accès
    // ═══════════════════════════════════════════════════════════

    async fn submit(&self, intent: TypedWriteIntent) -> Result<(), DbError> {
        self.client.submit(intent).await?;
        Ok(())
    }

    async fn submit_batch(&self, intents: Vec<TypedWriteIntent>) -> Result<(), DbError> {
        self.client.submit_batch(intents).await?;
        Ok(())
    }

    async fn select(&self, query: TypedSelect) -> Result<Vec<Row>, DbError> {
        Ok(self.client.select(query).await?)
    }

    /// Lecture SQL brute transitoire (lot C) : paramètres liés en TEXT.
    async fn raw(&self, sql: &str, params: Vec<String>) -> Result<Vec<Row>, DbError> {
        Ok(self.client.query(sql, params).await?)
    }

    // ═══════════════════════════════════════════════════════════
    // Legacy compatibility (pipeline d'intégration)
    // ═══════════════════════════════════════════════════════════

    /// Indique si la DB est encore vide de flux d'intégration.
    pub fn is_bootstrap_needed(&self) -> Result<bool, DbError> {
        run_blocking(self.is_bootstrap_needed_async())
    }

    /// Variante async de [`Self::is_bootstrap_needed`].
    ///
    /// Écart : le chemin hérité consultait aussi une ancienne table `audit_log` ;
    /// elle n'existe pas dans le schéma KindMother (et `sqlite_master` y est protégé).
    pub async fn is_bootstrap_needed_async(&self) -> Result<bool, DbError> {
        let rows = self
            .select(TypedSelect::from("audit").columns(["id"]).limit(1))
            .await?;
        Ok(rows.is_empty())
    }

    /// Insère (ou remplace) un mouvement (API legacy pour pipeline d'intégration).
    pub fn insert_movement(&self, m: &MovementRecord) -> Result<(), DbError> {
        run_blocking(self.insert_movement_async(m))
    }

    /// Variante async de [`Self::insert_movement`].
    ///
    /// Écart : `INSERT OR REPLACE` devient un upsert sur `id` ; en cas d'id existant,
    /// la ligne est mise à jour (les colonnes non fournies, ex. `category_id`, sont
    /// conservées au lieu d'être remises à NULL) et le déclencheur d'insertion n'est
    /// pas rejoué.
    pub async fn insert_movement_async(&self, m: &MovementRecord) -> Result<(), DbError> {
        let account_id = self.ensure_account_exists_inner(&m.scope).await?;
        self.submit(
            TypedWriteIntent::upsert("movements")
                .key("id", m.id.as_str())
                .value("account_id", account_id)
                .value("scope", m.scope.as_str())
                .value("amount", m.amount)
                .value("currency", m.currency.as_str())
                // legacy : category comme description
                .value("description", m.category.as_str())
                .value("movement_date", m.movement_date.as_str())
                .value("context_ref", m.context_ref.as_str())
                .value("source_service", m.source_service.as_str())
                .value("created_at", now_sql())
                .purpose("jaykonta.integration.movement"),
        )
        .await
    }

    /// Insère (ou remplace) un devis (API legacy pour pipeline d'intégration).
    pub fn insert_quote(&self, q: &QuoteRecord) -> Result<(), DbError> {
        run_blocking(self.insert_quote_async(q))
    }

    /// Variante async de [`Self::insert_quote`] (upsert sur `id`, cf. `insert_movement`).
    pub async fn insert_quote_async(&self, q: &QuoteRecord) -> Result<(), DbError> {
        let account_id = self.ensure_account_exists_inner(&q.scope).await?;
        let cp_id = self
            .ensure_counterparty_inner(&account_id, &q.counterparty_ref)
            .await?;
        let number = format!("DEV-{}", &q.id[..8.min(q.id.len())]);
        self.submit(
            TypedWriteIntent::upsert("quotes")
                .key("id", q.id.as_str())
                .value("account_id", account_id)
                .value("number", number)
                .value("counterparty_id", cp_id)
                .value("context_ref", q.context_ref.as_str())
                .value("total_ht", q.total)
                .value("total_tva", 0.0)
                .value("total_ttc", q.total)
                .value("currency", q.currency.as_str())
                .value("status", q.status.as_str())
                .value("source_service", "integration")
                .value("created_at", q.created_at.as_str())
                .purpose("jaykonta.integration.quote"),
        )
        .await
    }

    /// Insère (ou remplace) une facture (API legacy pour pipeline d'intégration).
    pub fn insert_invoice(&self, i: &InvoiceRecord) -> Result<(), DbError> {
        run_blocking(self.insert_invoice_async(i))
    }

    /// Variante async de [`Self::insert_invoice`] (upsert sur `id`).
    pub async fn insert_invoice_async(&self, i: &InvoiceRecord) -> Result<(), DbError> {
        let account_id = self.ensure_account_exists_inner(&i.scope).await?;
        let cp_id = self
            .ensure_counterparty_inner(&account_id, &i.counterparty_ref)
            .await?;
        let number = format!("FAC-{}", &i.id[..8.min(i.id.len())]);
        self.submit(
            TypedWriteIntent::upsert("invoices")
                .key("id", i.id.as_str())
                .value("account_id", account_id)
                .value("number", number)
                .value("counterparty_id", cp_id)
                .value("quote_id", i.quote_id.as_deref())
                .value("context_ref", i.context_ref.as_str())
                .value("total_ht", i.total)
                .value("total_tva", 0.0)
                .value("total_ttc", i.total)
                .value("paid_amount", i.paid_amount)
                .value("currency", i.currency.as_str())
                .value("status", i.status.as_str())
                .value("issued_at", i.issued_at.as_str())
                .value("due_at", i.due_at.as_deref())
                .value("source_service", "integration")
                .purpose("jaykonta.integration.invoice"),
        )
        .await
    }

    /// Insère un paiement et met à jour la facture associée (API legacy).
    pub fn insert_payment_and_update_invoice(&self, p: &PaymentRecord) -> Result<(), DbError> {
        run_blocking(self.insert_payment_and_update_invoice_async(p))
    }

    /// Variante async de [`Self::insert_payment_and_update_invoice`].
    ///
    /// Nouveau paiement : insertion seule, le déclencheur `trg_invoice_paid_insert`
    /// recalcule `paid_amount`/`status` dans la même instruction (atomique).
    /// Paiement rejoué (id existant, `INSERT OR REPLACE` hérité) : l'upsert ne rejoue
    /// pas le déclencheur d'insertion ; le cumul est donc recalculé côté client et
    /// appliqué dans un **lot atomique** (paiement + facture). Écart : ce recalcul est
    /// une lecture-puis-écriture (un paiement concurrent sur la même facture entre la
    /// lecture et le lot peut être omis du cumul jusqu'au paiement suivant).
    pub async fn insert_payment_and_update_invoice_async(
        &self,
        p: &PaymentRecord,
    ) -> Result<(), DbError> {
        let invoice = self
            .select(
                TypedSelect::from("invoices")
                    .columns(["account_id", "total_ttc"])
                    .filter("id", p.invoice_id.as_str())
                    .limit(1),
            )
            .await?;
        let account_id = invoice
            .first()
            .and_then(|r| opt_str(r, "account_id"))
            .unwrap_or_else(|| "default-account".to_string());
        let payment = TypedWriteIntent::upsert("payments")
            .key("id", p.id.as_str())
            .value("account_id", account_id)
            .value("invoice_id", p.invoice_id.as_str())
            .value("amount", p.amount)
            .value("currency", p.currency.as_str())
            .value("method", p.method.as_str())
            .value("reference_opaque", p.reference_opaque.as_str())
            .value("paid_at", p.paid_at.as_str())
            .purpose("jaykonta.integration.payment");

        let existing = self
            .select(
                TypedSelect::from("payments")
                    .columns(["id"])
                    .filter("id", p.id.as_str())
                    .limit(1),
            )
            .await?;
        let Some(invoice) = invoice.first().filter(|_| !existing.is_empty()) else {
            return self.submit(payment).await;
        };

        let others: f64 = self
            .select(
                TypedSelect::from("payments")
                    .columns(["id", "amount"])
                    .filter("invoice_id", p.invoice_id.as_str()),
            )
            .await?
            .iter()
            .filter(|r| str_of(r, "id") != p.id)
            .map(|r| f64_of(r, "amount"))
            .sum();
        let paid = others + p.amount;
        let status = if paid >= f64_of(invoice, "total_ttc") {
            "paid"
        } else {
            "partial"
        };
        self.submit_batch(vec![
            payment,
            TypedWriteIntent::update("invoices")
                .key("id", p.invoice_id.as_str())
                .value("paid_amount", paid)
                .value("status", status)
                .purpose("jaykonta.integration.payment.invoice_paid"),
        ])
        .await
    }

    /// Insère (ou remplace) un rappel (API legacy pour pipeline d'intégration).
    pub fn insert_reminder(&self, r: &ReminderRecord) -> Result<(), DbError> {
        run_blocking(self.insert_reminder_async(r))
    }

    /// Variante async de [`Self::insert_reminder`] (upsert sur `id`).
    pub async fn insert_reminder_async(&self, r: &ReminderRecord) -> Result<(), DbError> {
        let account_id = self.ensure_account_exists_inner("purse").await?;
        self.submit(
            TypedWriteIntent::upsert("reminders")
                .key("id", r.id.as_str())
                .value("account_id", account_id)
                .value("deadline_ref", r.deadline_ref.as_str())
                .value("due_at", r.due_at.as_str())
                .value("label", r.label.as_str())
                .value("context_ref", r.context_ref.as_str())
                .value("source_service", r.source_service.as_str())
                .purpose("jaykonta.integration.reminder"),
        )
        .await
    }

    /// Insère une entrée d'audit (API legacy).
    pub fn insert_audit(&self, a: &AuditRecord) -> Result<(), DbError> {
        run_blocking(self.insert_audit_async(a))
    }

    /// Variante async de [`Self::insert_audit`].
    pub async fn insert_audit_async(&self, a: &AuditRecord) -> Result<(), DbError> {
        let account_id = self.first_account_id_inner().await?;
        self.submit(
            TypedWriteIntent::insert("audit")
                .value("id", a.id.as_str())
                .value("account_id", account_id)
                .value("contract_id", a.contract_id.as_str())
                .value("actor_ref", a.actor_ref.as_str())
                .value("operation", a.operation.as_str())
                .value("scope", a.scope.as_str())
                .value("object_ref", a.object_ref.as_str())
                .value("result", a.result.as_str())
                .value("payload", a.payload_json.as_str())
                .value("created_at", a.created_at.as_str())
                .purpose("jaykonta.audit"),
        )
        .await
    }

    // ═══════════════════════════════════════════════════════════
    // Statistiques (backend/service.rs)
    // ═══════════════════════════════════════════════════════════

    /// Retourne les statistiques Account.
    pub fn account_stats(&self) -> Result<AccountStats, DbError> {
        run_blocking(self.account_stats_async())
    }

    /// Variante async de [`Self::account_stats`].
    pub async fn account_stats_async(&self) -> Result<AccountStats, DbError> {
        let month_key = Local::now().format("%Y-%m").to_string();
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                "SELECT
                    (SELECT COALESCE(SUM(total_ttc), 0) FROM invoices
                      WHERE substr(issued_at, 1, 7) = ?1) AS ca_mensuel,
                    (SELECT COALESCE(SUM(total_ttc - paid_amount), 0) FROM invoices
                      WHERE status IN ('issued','sent','partial','overdue')) AS impayes,
                    (SELECT COUNT(*) FROM invoices) AS invoices_count,
                    (SELECT COALESCE(SUM(paid_amount), 0) FROM invoices) AS paid_sum,
                    (SELECT COALESCE(SUM(total_ttc), 0) FROM invoices) AS total_sum",
                vec![month_key],
            )
            .await?;
        let Some(r) = rows.first() else {
            return Ok(AccountStats::default());
        };
        let paid_sum = f64_of(r, "paid_sum");
        let total_sum = f64_of(r, "total_sum");
        Ok(AccountStats {
            ca_mensuel: f64_of(r, "ca_mensuel"),
            impayes: f64_of(r, "impayes"),
            invoices_count: i64_of(r, "invoices_count"),
            payment_rate_pct: if total_sum > 0.0 {
                (paid_sum / total_sum) * 100.0
            } else {
                0.0
            },
        })
    }

    /// Retourne les statistiques Purse.
    pub fn purse_stats(&self) -> Result<PurseStats, DbError> {
        run_blocking(self.purse_stats_async())
    }

    /// Variante async de [`Self::purse_stats`].
    #[allow(clippy::cast_precision_loss)]
    pub async fn purse_stats_async(&self) -> Result<PurseStats, DbError> {
        let month_key = Local::now().format("%Y-%m").to_string();
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                "SELECT
                    (SELECT COALESCE(SUM(ABS(amount)), 0) FROM movements
                      WHERE scope='purse' AND amount < 0
                        AND substr(movement_date, 1, 7) = ?1) AS monthly_expenses,
                    (SELECT COALESCE(SUM(amount), 0) FROM movements
                      WHERE scope='purse' AND amount > 0) AS budget_in,
                    (SELECT COALESCE(SUM(ABS(amount)), 0) FROM movements
                      WHERE scope='purse' AND amount < 0) AS budget_out,
                    (SELECT COUNT(*) FROM goals WHERE status='active') AS active_goals,
                    (SELECT COUNT(*) FROM movements WHERE scope='purse') AS total_moves,
                    (SELECT COUNT(*) FROM movements
                      WHERE scope='purse' AND category_id IS NOT NULL) AS categorized_moves",
                vec![month_key],
            )
            .await?;
        let Some(r) = rows.first() else {
            return Ok(PurseStats::default());
        };
        let budget_in = f64_of(r, "budget_in");
        let budget_out = f64_of(r, "budget_out");
        let total_moves = i64_of(r, "total_moves");
        let categorized_moves = i64_of(r, "categorized_moves");
        Ok(PurseStats {
            monthly_expenses: f64_of(r, "monthly_expenses"),
            budget_usage_pct: if budget_in > 0.0 {
                (budget_out / budget_in * 100.0).min(999.0)
            } else {
                0.0
            },
            active_goals: i64_of(r, "active_goals"),
            categorized_pct: if total_moves > 0 {
                categorized_moves as f64 * 100.0 / total_moves as f64
            } else {
                0.0
            },
        })
    }

    /// Solde net du Purse (somme algébrique des mouvements scope=purse).
    pub fn solde_purse(&self) -> Result<f64, DbError> {
        run_blocking(self.solde_purse_async())
    }

    /// Variante async de [`Self::solde_purse`].
    pub async fn solde_purse_async(&self) -> Result<f64, DbError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                "SELECT COALESCE(SUM(amount), 0) AS solde FROM movements WHERE scope='purse'",
                Vec::new(),
            )
            .await?;
        Ok(rows.first().map_or(0.0, |r| f64_of(r, "solde")))
    }

    /// Liste les mouvements récents (tout scope, ordre décroissant).
    pub fn movements_recent(&self, limit: i64) -> Result<Vec<MovementRecord>, DbError> {
        run_blocking(self.movements_recent_async(limit))
    }

    /// Variante async de [`Self::movements_recent`].
    pub async fn movements_recent_async(&self, limit: i64) -> Result<Vec<MovementRecord>, DbError> {
        let limit = u32::try_from(limit.max(0)).unwrap_or(u32::MAX);
        let rows = self
            .select(
                TypedSelect::from("movements")
                    .columns([
                        "id",
                        "scope",
                        "context_ref",
                        "description",
                        "amount",
                        "currency",
                        "movement_date",
                        "source_service",
                    ])
                    .order_by("movement_date", true)
                    .limit(limit),
            )
            .await?;
        Ok(rows
            .iter()
            .map(|r| MovementRecord {
                id: str_of(r, "id"),
                scope: str_of(r, "scope"),
                context_ref: str_of(r, "context_ref"),
                category: str_of(r, "description"),
                amount: f64_of(r, "amount"),
                currency: str_of(r, "currency"),
                movement_date: str_of(r, "movement_date"),
                source_service: opt_str(r, "source_service")
                    .unwrap_or_else(|| "manual".to_string()),
            })
            .collect())
    }

    /// Liste les factures récentes (tout scope, ordre décroissant).
    pub fn invoices_recent(&self, limit: i64) -> Result<Vec<InvoiceRecord>, DbError> {
        run_blocking(self.invoices_recent_async(limit))
    }

    /// Variante async de [`Self::invoices_recent`].
    pub async fn invoices_recent_async(&self, limit: i64) -> Result<Vec<InvoiceRecord>, DbError> {
        let limit = limit.max(0);
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                &format!(
                    "SELECT i.id, COALESCE(i.context_ref, '') AS context_ref,
                        COALESCE((SELECT c.name FROM counterparties c
                                   WHERE c.id = i.counterparty_id), '') AS counterparty_ref,
                        i.quote_id, i.total_ttc, i.paid_amount, i.currency, i.status,
                        i.issued_at, i.due_at
                     FROM invoices i ORDER BY i.issued_at DESC LIMIT {limit}"
                ),
                Vec::new(),
            )
            .await?;
        Ok(rows
            .iter()
            .map(|r| InvoiceRecord {
                id: str_of(r, "id"),
                scope: "account".to_string(),
                context_ref: str_of(r, "context_ref"),
                counterparty_ref: str_of(r, "counterparty_ref"),
                quote_id: opt_str(r, "quote_id"),
                total: f64_of(r, "total_ttc"),
                paid_amount: f64_of(r, "paid_amount"),
                currency: str_of(r, "currency"),
                status: str_of(r, "status"),
                issued_at: str_of(r, "issued_at"),
                due_at: opt_str(r, "due_at"),
            })
            .collect())
    }

    /// Nombre d'écritures d'audit.
    pub fn audit_count(&self) -> Result<i64, DbError> {
        run_blocking(self.audit_count_async())
    }

    /// Variante async de [`Self::audit_count`].
    pub async fn audit_count_async(&self) -> Result<i64, DbError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw("SELECT COUNT(*) AS n FROM audit", Vec::new())
            .await?;
        Ok(rows.first().map_or(0, |r| i64_of(r, "n")))
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Mouvements
    // ═══════════════════════════════════════════════════════════

    /// Crée un mouvement Purse.
    #[allow(clippy::too_many_arguments)]
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
        run_blocking(self.create_purse_movement_async(
            account_id,
            id,
            amount,
            category_id,
            description,
            movement_date,
            budget_id,
        ))
    }

    /// Variante async de [`Self::create_purse_movement`] (le déclencheur
    /// `trg_budget_spent_insert` met à jour le cumul du budget côté service).
    #[allow(clippy::too_many_arguments)]
    pub async fn create_purse_movement_async(
        &self,
        account_id: &str,
        id: &str,
        amount: f64,
        category_id: Option<&str>,
        description: Option<&str>,
        movement_date: &str,
        budget_id: Option<&str>,
    ) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::insert("movements")
                .value("id", id)
                .value("account_id", account_id)
                .value("scope", "purse")
                .value("category_id", category_id)
                .value("amount", amount)
                .value("currency", "EUR")
                .value("description", description)
                .value("movement_date", movement_date)
                .value("source_service", "manual")
                .value("budget_id", budget_id)
                .purpose("jaykonta.purse.movement.create"),
        )
        .await
    }

    /// Liste les mouvements Purse filtrés et paginés.
    pub fn list_purse_movements(
        &self,
        account_id: &str,
        filters: &MovementFilters,
    ) -> Result<(Vec<Movement>, u32), DbError> {
        run_blocking(self.list_purse_movements_async(account_id, filters))
    }

    /// Variante async de [`Self::list_purse_movements`].
    pub async fn list_purse_movements_async(
        &self,
        account_id: &str,
        filters: &MovementFilters,
    ) -> Result<(Vec<Movement>, u32), DbError> {
        let mut where_clauses = vec!["m.scope = 'purse'".to_string(), "m.account_id = ?1".to_string()];
        let mut params = vec![account_id.to_string()];
        if let Some(month) = &filters.month {
            params.push(month.clone());
            where_clauses.push(format!("substr(m.movement_date, 1, 7) = ?{}", params.len()));
        }
        if let Some(category_id) = &filters.category_id {
            params.push(category_id.clone());
            where_clauses.push(format!("m.category_id = ?{}", params.len()));
        }
        if let Some(mt) = &filters.movement_type {
            where_clauses.push(match mt {
                MovementType::Expense => "m.amount < 0".to_string(),
                MovementType::Income => "m.amount > 0".to_string(),
            });
        }
        if let Some(budget_id) = &filters.budget_id {
            params.push(budget_id.clone());
            where_clauses.push(format!("m.budget_id = ?{}", params.len()));
        }
        let where_sql = where_clauses.join(" AND ");
        let per_page = if filters.per_page == 0 {
            20
        } else {
            filters.per_page
        };
        let offset = u64::from(filters.page) * u64::from(per_page);

        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let count_rows = self
            .raw(
                &format!("SELECT COUNT(*) AS n FROM movements m WHERE {where_sql}"),
                params.clone(),
            )
            .await?;
        let total_count = count_rows
            .first()
            .map_or(0, |r| u32::try_from(i64_of(r, "n")).unwrap_or(u32::MAX));

        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                &format!(
                    "SELECT m.id, m.amount, m.currency, m.category_id, c.name AS cat_name,
                        m.description, m.movement_date, m.budget_id, b.name AS budget_name,
                        m.source_service, m.context_ref, m.created_at
                     FROM movements m
                     LEFT JOIN categories c ON m.category_id = c.id
                     LEFT JOIN budgets b ON m.budget_id = b.id
                     WHERE {where_sql}
                     ORDER BY m.movement_date DESC
                     LIMIT {per_page} OFFSET {offset}"
                ),
                params,
            )
            .await?;
        let movements = rows
            .iter()
            .map(|r| Movement {
                id: str_of(r, "id"),
                amount: f64_of(r, "amount"),
                currency: str_of(r, "currency"),
                category: opt_str(r, "category_id")
                    .zip(opt_str(r, "cat_name"))
                    .map(|(id, name)| CategoryRef { id, name }),
                description: opt_str(r, "description"),
                movement_date: str_of(r, "movement_date"),
                budget: opt_str(r, "budget_id")
                    .zip(opt_str(r, "budget_name"))
                    .map(|(id, name)| BudgetRef { id, name }),
                source_service: opt_str(r, "source_service"),
                context_ref: opt_str(r, "context_ref"),
                created_at: str_of(r, "created_at"),
            })
            .collect();
        Ok((movements, total_count))
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Catégories
    // ═══════════════════════════════════════════════════════════

    /// Liste les catégories d'un compte.
    pub fn list_categories(&self, account_id: &str) -> Result<Vec<Category>, DbError> {
        run_blocking(self.list_categories_async(account_id))
    }

    /// Variante async de [`Self::list_categories`].
    pub async fn list_categories_async(&self, account_id: &str) -> Result<Vec<Category>, DbError> {
        let rows = self
            .select(
                TypedSelect::from("categories")
                    .columns(["id", "name", "icon", "color", "parent_id", "is_income", "sort_order"])
                    .filter("account_id", account_id)
                    .order_by("sort_order", false)
                    .order_by("name", false),
            )
            .await?;
        Ok(rows
            .iter()
            .map(|r| Category {
                id: str_of(r, "id"),
                name: str_of(r, "name"),
                icon: opt_str(r, "icon"),
                color: opt_str(r, "color"),
                parent_id: opt_str(r, "parent_id"),
                is_income: i64_of(r, "is_income") != 0,
                sort_order: i32::try_from(i64_of(r, "sort_order")).unwrap_or_default(),
            })
            .collect())
    }

    /// Crée une catégorie (ignorée si l'id ou le couple compte/nom existe déjà).
    #[allow(clippy::too_many_arguments)]
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
        run_blocking(self.create_category_async(
            id, account_id, name, icon, color, is_income, sort_order,
        ))
    }

    /// Variante async de [`Self::create_category`].
    ///
    /// `INSERT OR IGNORE` hérité → lecture-puis-insertion : si l'insertion échoue,
    /// l'existence est revérifiée (doublon concurrent = ignoré, autre cause = erreur).
    #[allow(clippy::too_many_arguments)]
    pub async fn create_category_async(
        &self,
        id: &str,
        account_id: &str,
        name: &str,
        icon: Option<&str>,
        color: Option<&str>,
        is_income: bool,
        sort_order: i32,
    ) -> Result<(), DbError> {
        if self.category_exists(id, account_id, name).await? {
            return Ok(());
        }
        let inserted = self
            .submit(
                TypedWriteIntent::insert("categories")
                    .value("id", id)
                    .value("account_id", account_id)
                    .value("name", name)
                    .value("icon", icon)
                    .value("color", color)
                    .value("is_income", is_income)
                    .value("sort_order", sort_order)
                    .purpose("jaykonta.purse.category.create"),
            )
            .await;
        match inserted {
            Err(_) if self.category_exists(id, account_id, name).await? => Ok(()),
            other => other,
        }
    }

    async fn category_exists(&self, id: &str, account_id: &str, name: &str) -> Result<bool, DbError> {
        let by_id = self
            .select(TypedSelect::from("categories").columns(["id"]).filter("id", id).limit(1))
            .await?;
        if !by_id.is_empty() {
            return Ok(true);
        }
        let by_name = self
            .select(
                TypedSelect::from("categories")
                    .columns(["id"])
                    .filter("account_id", account_id)
                    .filter("name", name)
                    .limit(1),
            )
            .await?;
        Ok(!by_name.is_empty())
    }

    /// Initialise les catégories par défaut Purse si aucune n'existe.
    pub fn ensure_default_categories(&self, account_id: &str) -> Result<(), DbError> {
        run_blocking(self.ensure_default_categories_async(account_id))
    }

    /// Variante async de [`Self::ensure_default_categories`] : les catégories absentes
    /// (par id, comme `INSERT OR IGNORE`) sont insérées en un lot atomique.
    pub async fn ensure_default_categories_async(&self, account_id: &str) -> Result<(), DbError> {
        let existing = self
            .select(
                TypedSelect::from("categories")
                    .columns(["id"])
                    .filter("account_id", account_id)
                    .limit(1),
            )
            .await?;
        if !existing.is_empty() {
            return Ok(());
        }
        let mut intents = Vec::new();
        for (i, (name, icon, is_income)) in DEFAULT_PURSE_CATEGORIES.iter().enumerate() {
            let cat_id = format!("cat-default-{}", name.to_lowercase().replace(' ', "-"));
            let taken = self
                .select(
                    TypedSelect::from("categories")
                        .columns(["id"])
                        .filter("id", cat_id.as_str())
                        .limit(1),
                )
                .await?;
            if !taken.is_empty() {
                continue;
            }
            intents.push(
                TypedWriteIntent::insert("categories")
                    .value("id", cat_id)
                    .value("account_id", account_id)
                    .value("name", *name)
                    .value("icon", *icon)
                    .value("is_income", *is_income)
                    .value("sort_order", i64::try_from(i).unwrap_or(i64::MAX))
                    .purpose("jaykonta.purse.category.defaults"),
            );
        }
        if intents.is_empty() {
            return Ok(());
        }
        self.submit_batch(intents).await
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Budgets occasionnels
    // ═══════════════════════════════════════════════════════════

    /// Liste les budgets occasionnels d'un compte.
    pub fn list_budgets(&self, account_id: &str) -> Result<Vec<OccasionalBudget>, DbError> {
        run_blocking(self.list_budgets_async(account_id))
    }

    /// Variante async de [`Self::list_budgets`].
    pub async fn list_budgets_async(&self, account_id: &str) -> Result<Vec<OccasionalBudget>, DbError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                "SELECT b.id, b.name, b.target, b.spent, b.currency, b.start_date, b.end_date,
                        b.status,
                        (SELECT COUNT(*) FROM movements m WHERE m.budget_id = b.id) AS moves_count
                 FROM budgets b WHERE b.account_id = ?1 ORDER BY b.created_at DESC",
                vec![account_id.to_string()],
            )
            .await?;
        Ok(rows
            .iter()
            .map(|r| {
                let target = f64_of(r, "target");
                let spent = f64_of(r, "spent");
                OccasionalBudget {
                    id: str_of(r, "id"),
                    name: str_of(r, "name"),
                    target,
                    spent,
                    remaining: (target - spent).max(0.0),
                    progress_pct: if target > 0.0 {
                        (spent / target * 100.0).min(100.0)
                    } else {
                        0.0
                    },
                    currency: str_of(r, "currency"),
                    start_date: str_of(r, "start_date"),
                    end_date: opt_str(r, "end_date"),
                    status: BudgetStatus::from_str(&str_of(r, "status")),
                    movements_count: u32::try_from(i64_of(r, "moves_count")).unwrap_or(0),
                }
            })
            .collect())
    }

    /// Crée un budget occasionnel.
    pub fn create_budget(
        &self,
        id: &str,
        account_id: &str,
        name: &str,
        target: f64,
        start_date: &str,
        end_date: Option<&str>,
    ) -> Result<(), DbError> {
        run_blocking(self.create_budget_async(id, account_id, name, target, start_date, end_date))
    }

    /// Variante async de [`Self::create_budget`].
    pub async fn create_budget_async(
        &self,
        id: &str,
        account_id: &str,
        name: &str,
        target: f64,
        start_date: &str,
        end_date: Option<&str>,
    ) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::insert("budgets")
                .value("id", id)
                .value("account_id", account_id)
                .value("name", name)
                .value("target", target)
                .value("start_date", start_date)
                .value("end_date", end_date)
                .purpose("jaykonta.purse.budget.create"),
        )
        .await
    }

    /// Clôture un budget occasionnel.
    pub fn close_budget(&self, budget_id: &str) -> Result<(), DbError> {
        run_blocking(self.close_budget_async(budget_id))
    }

    /// Variante async de [`Self::close_budget`].
    pub async fn close_budget_async(&self, budget_id: &str) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::update("budgets")
                .key("id", budget_id)
                .value("status", "closed")
                .purpose("jaykonta.purse.budget.close"),
        )
        .await
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Objectifs
    // ═══════════════════════════════════════════════════════════

    /// Liste les objectifs d'un compte.
    pub fn list_goals(&self, account_id: &str) -> Result<Vec<Goal>, DbError> {
        run_blocking(self.list_goals_async(account_id))
    }

    /// Variante async de [`Self::list_goals`].
    pub async fn list_goals_async(&self, account_id: &str) -> Result<Vec<Goal>, DbError> {
        let rows = self
            .select(
                TypedSelect::from("goals")
                    .columns([
                        "id", "name", "target", "current", "currency", "goal_type", "deadline",
                        "status",
                    ])
                    .filter("account_id", account_id)
                    .order_by("created_at", true),
            )
            .await?;
        Ok(rows.iter().map(goal_of).collect())
    }

    /// Crée un objectif.
    pub fn create_goal(
        &self,
        id: &str,
        account_id: &str,
        name: &str,
        target: f64,
        goal_type: &str,
        deadline: Option<&str>,
    ) -> Result<(), DbError> {
        run_blocking(self.create_goal_async(id, account_id, name, target, goal_type, deadline))
    }

    /// Variante async de [`Self::create_goal`].
    pub async fn create_goal_async(
        &self,
        id: &str,
        account_id: &str,
        name: &str,
        target: f64,
        goal_type: &str,
        deadline: Option<&str>,
    ) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::insert("goals")
                .value("id", id)
                .value("account_id", account_id)
                .value("name", name)
                .value("target", target)
                .value("goal_type", goal_type)
                .value("deadline", deadline)
                .purpose("jaykonta.purse.goal.create"),
        )
        .await
    }

    /// Met à jour la progression d'un objectif (passe à `reached` si cible atteinte).
    pub fn update_goal_progress(&self, goal_id: &str, amount: f64) -> Result<(), DbError> {
        run_blocking(self.update_goal_progress_async(goal_id, amount))
    }

    /// Variante async de [`Self::update_goal_progress`].
    ///
    /// L'incrément est atomique côté service (`current = current + ?`) ; le passage à
    /// `reached` (conditionnel en SQL hérité) est décidé côté client à partir de la
    /// valeur lue juste avant, dans la même intention. Objectif absent : sans effet.
    pub async fn update_goal_progress_async(&self, goal_id: &str, amount: f64) -> Result<(), DbError> {
        let rows = self
            .select(
                TypedSelect::from("goals")
                    .columns(["current", "target", "status"])
                    .filter("id", goal_id)
                    .limit(1),
            )
            .await?;
        let Some(goal) = rows.first() else {
            return Ok(());
        };
        let mut intent = TypedWriteIntent::update("goals")
            .key("id", goal_id)
            .increment("current", amount)
            .purpose("jaykonta.purse.goal.progress");
        if str_of(goal, "status") == "active" && f64_of(goal, "current") + amount >= f64_of(goal, "target") {
            intent = intent.value("status", "reached");
        }
        self.submit(intent).await
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Tableau de bord
    // ═══════════════════════════════════════════════════════════

    /// Répartition des dépenses par catégorie pour un mois (`AAAA-MM`).
    pub fn categories_breakdown_purse(
        &self,
        account_id: &str,
        month: &str,
    ) -> Result<Vec<CategoryBreakdown>, DbError> {
        run_blocking(self.categories_breakdown_purse_async(account_id, month))
    }

    /// Variante async de [`Self::categories_breakdown_purse`].
    pub async fn categories_breakdown_purse_async(
        &self,
        account_id: &str,
        month: &str,
    ) -> Result<Vec<CategoryBreakdown>, DbError> {
        let (_, total_expenses) = self.purse_month_totals_async(account_id, month).await?;
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                "SELECT COALESCE(c.name, 'Sans categorie') AS category_name,
                        SUM(ABS(m.amount)) AS amount
                 FROM movements m
                 LEFT JOIN categories c ON m.category_id = c.id
                 WHERE m.scope='purse' AND m.account_id=?1 AND m.amount < 0
                   AND substr(m.movement_date, 1, 7)=?2
                 GROUP BY c.name ORDER BY SUM(ABS(m.amount)) DESC",
                vec![account_id.to_string(), month.to_string()],
            )
            .await?;
        Ok(rows
            .iter()
            .map(|r| {
                let amount = f64_of(r, "amount");
                CategoryBreakdown {
                    category_name: str_of(r, "category_name"),
                    amount,
                    percentage: if total_expenses > 0.0 {
                        amount / total_expenses * 100.0
                    } else {
                        0.0
                    },
                }
            })
            .collect())
    }

    /// Revenus et dépenses du mois pour Purse : `(income, expense)`.
    pub fn purse_month_totals(&self, account_id: &str, month: &str) -> Result<(f64, f64), DbError> {
        run_blocking(self.purse_month_totals_async(account_id, month))
    }

    /// Variante async de [`Self::purse_month_totals`].
    pub async fn purse_month_totals_async(
        &self,
        account_id: &str,
        month: &str,
    ) -> Result<(f64, f64), DbError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                "SELECT
                    COALESCE(SUM(CASE WHEN amount > 0 THEN amount ELSE 0 END), 0) AS income,
                    COALESCE(SUM(CASE WHEN amount < 0 THEN ABS(amount) ELSE 0 END), 0) AS expense
                 FROM movements
                 WHERE scope='purse' AND account_id=?1 AND substr(movement_date, 1, 7)=?2",
                vec![account_id.to_string(), month.to_string()],
            )
            .await?;
        Ok(rows
            .first()
            .map_or((0.0, 0.0), |r| (f64_of(r, "income"), f64_of(r, "expense"))))
    }

    // ═══════════════════════════════════════════════════════════
    // CRUD Purse — Transactions récurrentes
    // ═══════════════════════════════════════════════════════════

    /// Liste les transactions récurrentes d'un compte (scope purse).
    pub fn list_recurring(
        &self,
        account_id: &str,
        active_only: bool,
    ) -> Result<Vec<RecurringTransaction>, DbError> {
        run_blocking(self.list_recurring_async(account_id, active_only))
    }

    /// Variante async de [`Self::list_recurring`].
    pub async fn list_recurring_async(
        &self,
        account_id: &str,
        active_only: bool,
    ) -> Result<Vec<RecurringTransaction>, DbError> {
        let (filter, order) = if active_only {
            ("AND r.is_active = 1", "r.next_due_date ASC")
        } else {
            ("", "r.is_active DESC, r.next_due_date ASC")
        };
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self
            .raw(
                &format!(
                    "SELECT r.id, r.direction, r.amount, r.currency, r.category_id,
                        c.name AS cat_name, r.description, r.frequency, r.day_of_month,
                        r.day_of_week, r.start_date, r.end_date, r.next_due_date, r.is_active,
                        r.auto_create, r.notes, r.created_at
                     FROM recurring_transactions r
                     LEFT JOIN categories c ON r.category_id = c.id
                     WHERE r.account_id = ?1 AND r.scope = 'purse' {filter}
                     ORDER BY {order}"
                ),
                vec![account_id.to_string()],
            )
            .await?;
        Ok(rows
            .iter()
            .map(|r| RecurringTransaction {
                id: str_of(r, "id"),
                direction: RecurringDirection::from_str(&str_of(r, "direction")),
                amount: f64_of(r, "amount"),
                currency: str_of(r, "currency"),
                category: opt_str(r, "category_id")
                    .zip(opt_str(r, "cat_name"))
                    .map(|(id, name)| CategoryRef { id, name }),
                description: str_of(r, "description"),
                frequency: RecurringFrequency::from_str(&str_of(r, "frequency")),
                day_of_month: opt_i32(r, "day_of_month"),
                day_of_week: opt_i32(r, "day_of_week"),
                start_date: str_of(r, "start_date"),
                end_date: opt_str(r, "end_date"),
                next_due_date: str_of(r, "next_due_date"),
                is_active: i64_of(r, "is_active") != 0,
                auto_create: i64_of(r, "auto_create") != 0,
                notes: opt_str(r, "notes"),
                created_at: str_of(r, "created_at"),
            })
            .collect())
    }

    /// Crée une transaction récurrente.
    #[allow(clippy::too_many_arguments)]
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
        run_blocking(self.create_recurring_async(
            id,
            account_id,
            direction,
            amount,
            category_id,
            description,
            frequency,
            day_of_month,
            day_of_week,
            start_date,
            end_date,
            next_due_date,
            auto_create,
            notes,
        ))
    }

    /// Variante async de [`Self::create_recurring`].
    #[allow(clippy::too_many_arguments)]
    pub async fn create_recurring_async(
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
            return Err(DbError::Internal(
                "account_id manquant (compte Purse non initialisé)".to_string(),
            ));
        }
        // Chaîne vide pour category_id viole la FK : NULL si absent ou vide.
        let category_id = category_id.filter(|s| !s.trim().is_empty());
        self.submit(
            TypedWriteIntent::insert("recurring_transactions")
                .value("id", id)
                .value("account_id", account_id)
                .value("scope", "purse")
                .value("direction", direction)
                .value("amount", amount)
                .value("currency", "EUR")
                .value("category_id", category_id)
                .value("description", description)
                .value("frequency", frequency)
                .value("day_of_month", day_of_month)
                .value("day_of_week", day_of_week)
                .value("start_date", start_date)
                .value("end_date", end_date)
                .value("next_due_date", next_due_date)
                .value("is_active", true)
                .value("auto_create", auto_create)
                .value("notes", notes)
                .purpose("jaykonta.purse.recurring.create"),
        )
        .await
    }

    /// Active ou suspend une transaction récurrente.
    pub fn toggle_recurring(&self, recurring_id: &str, active: bool) -> Result<(), DbError> {
        run_blocking(self.toggle_recurring_async(recurring_id, active))
    }

    /// Variante async de [`Self::toggle_recurring`].
    pub async fn toggle_recurring_async(&self, recurring_id: &str, active: bool) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::update("recurring_transactions")
                .key("id", recurring_id)
                .value("is_active", active)
                .purpose("jaykonta.purse.recurring.toggle"),
        )
        .await
    }

    /// Supprime une transaction récurrente.
    pub fn delete_recurring(&self, recurring_id: &str) -> Result<(), DbError> {
        run_blocking(self.delete_recurring_async(recurring_id))
    }

    /// Variante async de [`Self::delete_recurring`].
    pub async fn delete_recurring_async(&self, recurring_id: &str) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::delete("recurring_transactions")
                .key("id", recurring_id)
                .purpose("jaykonta.purse.recurring.delete"),
        )
        .await
    }

    /// Totaux mensuels estimés des récurrents actifs : `(monthly_income, monthly_expense)`.
    pub fn recurring_monthly_totals(&self, account_id: &str) -> Result<(f64, f64), DbError> {
        run_blocking(self.recurring_monthly_totals_async(account_id))
    }

    /// Variante async de [`Self::recurring_monthly_totals`].
    pub async fn recurring_monthly_totals_async(&self, account_id: &str) -> Result<(f64, f64), DbError> {
        let rows = self
            .select(
                TypedSelect::from("recurring_transactions")
                    .columns(["direction", "amount", "frequency"])
                    .filter("account_id", account_id)
                    .filter("scope", "purse")
                    .filter("is_active", 1_i64),
            )
            .await?;
        let mut income = 0.0;
        let mut expense = 0.0;
        for r in &rows {
            let amount = f64_of(r, "amount");
            let monthly = match str_of(r, "frequency").as_str() {
                "weekly" => amount * 52.0 / 12.0,
                "biweekly" => amount * 26.0 / 12.0,
                "quarterly" => amount / 3.0,
                "yearly" => amount / 12.0,
                _ => amount,
            };
            if str_of(r, "direction") == "income" {
                income += monthly;
            } else {
                expense += monthly;
            }
        }
        Ok((income, expense))
    }

    // ═══════════════════════════════════════════════════════════
    // Comptes
    // ═══════════════════════════════════════════════════════════

    /// Retourne ou crée le compte d'un utilisateur pour un scope.
    pub fn ensure_account(&self, scope: &str, user_id: &str, label: &str) -> Result<String, DbError> {
        run_blocking(self.ensure_account_async(scope, user_id, label))
    }

    /// Variante async de [`Self::ensure_account`] (lecture-puis-insertion ; un doublon
    /// concurrent est absorbé par la contrainte `UNIQUE(user_id, scope)` puis relu).
    pub async fn ensure_account_async(
        &self,
        scope: &str,
        user_id: &str,
        label: &str,
    ) -> Result<String, DbError> {
        let lookup = || {
            TypedSelect::from("accounts")
                .columns(["id"])
                .filter("scope", scope)
                .filter("user_id", user_id)
                .limit(1)
        };
        self.find_or_create_account(lookup, scope, user_id, label).await
    }

    /// Compte d'un scope (tout utilisateur), créé pour `default` au besoin.
    async fn ensure_account_exists_inner(&self, scope: &str) -> Result<String, DbError> {
        let lookup = || {
            TypedSelect::from("accounts")
                .columns(["id"])
                .filter("scope", scope)
                .order_by("created_at", false)
                .limit(1)
        };
        let label = format!("JayKonta {scope}");
        self.find_or_create_account(lookup, scope, "default", &label).await
    }

    /// Premier compte existant, ou compte `account` par défaut.
    async fn first_account_id_inner(&self) -> Result<String, DbError> {
        let lookup = || {
            TypedSelect::from("accounts")
                .columns(["id"])
                .order_by("created_at", false)
                .limit(1)
        };
        self.find_or_create_account(lookup, "account", "default", "JayKonta").await
    }

    async fn find_or_create_account(
        &self,
        lookup: impl Fn() -> TypedSelect,
        scope: &str,
        user_id: &str,
        label: &str,
    ) -> Result<String, DbError> {
        if let Some(id) = self.select(lookup()).await?.first().and_then(|r| opt_str(r, "id")) {
            return Ok(id);
        }
        let id = uuid::Uuid::new_v4().to_string();
        let inserted = self
            .submit(
                TypedWriteIntent::insert("accounts")
                    .value("id", id.as_str())
                    .value("scope", scope)
                    .value("user_id", user_id)
                    .value("label", label)
                    .purpose("jaykonta.account.ensure"),
            )
            .await;
        match inserted {
            Ok(()) => Ok(id),
            Err(e) => self
                .select(lookup())
                .await?
                .first()
                .and_then(|r| opt_str(r, "id"))
                .ok_or(e),
        }
    }

    /// Contrepartie d'un compte par nom (créée au besoin).
    async fn ensure_counterparty_inner(&self, account_id: &str, cp_ref: &str) -> Result<String, DbError> {
        let rows = self
            .select(
                TypedSelect::from("counterparties")
                    .columns(["id"])
                    .filter("account_id", account_id)
                    .filter("name", cp_ref)
                    .limit(1),
            )
            .await?;
        if let Some(id) = rows.first().and_then(|r| opt_str(r, "id")) {
            return Ok(id);
        }
        let id = uuid::Uuid::new_v4().to_string();
        self.submit(
            TypedWriteIntent::insert("counterparties")
                .value("id", id.as_str())
                .value("account_id", account_id)
                .value("name", cp_ref)
                .purpose("jaykonta.counterparty.ensure"),
        )
        .await?;
        Ok(id)
    }
}

// ═══════════════════════════════════════════════════════════
// Conversion des lignes JSON
// ═══════════════════════════════════════════════════════════

/// Objectif depuis une ligne `goals` (même calcul que le chemin hérité).
fn goal_of(r: &Row) -> Goal {
    let target = f64_of(r, "target");
    let current = f64_of(r, "current");
    let progress_pct = if target > 0.0 {
        (current / target * 100.0).min(100.0)
    } else {
        0.0
    };
    let deadline = opt_str(r, "deadline");
    let monthly_required = deadline.as_ref().and_then(|d| {
        let remaining = target - current;
        if remaining <= 0.0 {
            return None;
        }
        let now = Local::now().format("%Y-%m").to_string();
        let dl_month = &d[..7.min(d.len())];
        if dl_month <= now.as_str() {
            return Some(remaining);
        }
        Some(remaining / 6.0) // placeholder hérité
    });
    Goal {
        id: str_of(r, "id"),
        name: str_of(r, "name"),
        target,
        current,
        progress_pct,
        currency: str_of(r, "currency"),
        goal_type: GoalType::from_str(&str_of(r, "goal_type")),
        deadline,
        status: GoalStatus::from_str(&str_of(r, "status")),
        monthly_required,
        is_on_track: monthly_required.is_none_or(|mr| mr < target * 0.3),
    }
}

fn opt_str(r: &Row, key: &str) -> Option<String> {
    match r.get(key)? {
        Value::Null => None,
        Value::String(s) => Some(s.clone()),
        other => Some(other.to_string()),
    }
}

fn str_of(r: &Row, key: &str) -> String {
    opt_str(r, key).unwrap_or_default()
}

fn f64_of(r: &Row, key: &str) -> f64 {
    match r.get(key) {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(Value::String(s)) => s.parse().unwrap_or(0.0),
        _ => 0.0,
    }
}

#[allow(clippy::cast_possible_truncation)]
fn opt_i64(r: &Row, key: &str) -> Option<i64> {
    match r.get(key)? {
        Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)),
        Value::String(s) => s.parse().ok(),
        Value::Bool(b) => Some(i64::from(*b)),
        _ => None,
    }
}

fn i64_of(r: &Row, key: &str) -> i64 {
    opt_i64(r, key).unwrap_or(0)
}

fn opt_i32(r: &Row, key: &str) -> Option<i32> {
    opt_i64(r, key).and_then(|v| i32::try_from(v).ok())
}
