//! Base de données JayShop via KindMother Client.
//!
//! Cette implémentation utilise exclusivement `kindmother-client` pour accéder aux données.
//! KindMother est le seul interlocuteur autorisé de la base de données.
//!
//! L'API publique est **synchrone** pour maintenir la compatibilité avec le code existant.
//! En interne, les appels async sont exécutés via le runtime Tokio.
//!
//! @id: jayshop_kindmother_client_db
//! @do: persist_jayshop_data_via_kindmother_client
//! @layer: infra

use crate::data::types::{Event, EventCost, PaymentMethod, ShopConfig, Ticket};
use kindmother_client::{ClientError, KindMotherClient};
use std::sync::Arc;
use std::sync::OnceLock;
use thiserror::Error;
use tokio::sync::OnceCell;

/// Erreur de la base JayShop (KindMother Client).
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

    /// Entité non trouvée.
    #[error("Not found: {0}")]
    NotFound(String),
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

/// Base de données JayShop via KindMother Client.
///
/// Fournit une API synchrone pour la compatibilité avec le code existant.
///
/// @id: jayshop_db_client_struct
/// @do: hold_kindmother_client_connection
/// @layer: infra
#[derive(Clone)]
pub struct JayShopDb {
    client: Arc<KindMotherClient>,
}

impl JayShopDb {
    /// Initialise la connexion globale au service KindMother (synchrone).
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), DbError> {
        run_blocking(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale au service KindMother (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), DbError> {
        let addr = addr.unwrap_or(DEFAULT_KINDMOTHER_ADDR);
        let token_opt = std::env::var("KINDMOTHER_OPERATOR_TOKEN").ok();
        let client =
            KindMotherClient::connect(addr, "jayshop", "jayshop", token_opt.as_deref()).await?;

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

    /// Ouvre une connexion (API compatible avec l'ancienne implémentation).
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self, DbError> {
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        db.init_schema()?;
        Ok(db)
    }

    /// Crée une nouvelle instance avec une connexion personnalisée (async).
    pub async fn connect(addr: &str) -> Result<Self, DbError> {
        let token_opt = std::env::var("KINDMOTHER_OPERATOR_TOKEN").ok();
        let client =
            KindMotherClient::connect(addr, "jayshop", "jayshop", token_opt.as_deref()).await?;
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Initialise le schéma de la base de données (synchrone).
    pub fn init_schema(&self) -> Result<(), DbError> {
        run_blocking(self.init_schema_async())
    }

    /// Retire les commentaires SQL (-- et /* */) pour envoi à KindMother.
    fn strip_sql_comments(schema: &str) -> String {
        let mut out = String::with_capacity(schema.len());
        let mut chars = schema.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '-' && chars.peek() == Some(&'-') {
                chars.next();
                while let Some(n) = chars.next() {
                    if n == '\n' {
                        break;
                    }
                }
                if !out.ends_with('\n') && !out.is_empty() {
                    out.push('\n');
                }
            } else if c == '/' && chars.peek() == Some(&'*') {
                chars.next();
                let mut depth = 1u32;
                while depth > 0 {
                    match (chars.next(), chars.peek()) {
                        (Some('*'), Some(&'/')) => {
                            chars.next();
                            depth -= 1;
                        }
                        (Some('/'), Some(&'*')) => {
                            chars.next();
                            depth += 1;
                        }
                        (Some(_), _) => {}
                        (None, _) => break,
                    }
                }
                if !out.ends_with('\n') && !out.is_empty() {
                    out.push('\n');
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    /// Initialise le schéma de la base de données (async).
    pub async fn init_schema_async(&self) -> Result<(), DbError> {
        let schema_sql = include_str!("schema.sql");
        let schema_sql = Self::strip_sql_comments(schema_sql);

        let statements: Vec<String> = schema_sql
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();

        for sql in statements {
            self.client
                .execute(&sql, Vec::<String>::new(), "init_schema")
                .await
                .map_err(DbError::from)?;
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Tickets
    // -----------------------------------------------------------------------

    /// Insère un nouveau ticket (synchrone).
    pub fn ticket_insert(&self, t: &Ticket) -> Result<(), DbError> {
        run_blocking(self.ticket_insert_async(t))
    }

    async fn ticket_insert_async(&self, t: &Ticket) -> Result<(), DbError> {
        let id = t
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();

        self.client
            .execute(
                "INSERT INTO tickets (id, ticket_number, seller_id, session_id, event_id, source,
                    status, customer_id, subtotal, tax_total, discount_total, total, currency, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                vec![
                    id,
                    t.ticket_number.clone().unwrap_or_default(),
                    t.seller_id.clone().unwrap_or_default(),
                    t.session_id.clone().unwrap_or_default(),
                    t.event_id.clone().unwrap_or_default(),
                    t.source.clone().unwrap_or_else(|| "pos".to_string()),
                    t.status.clone().unwrap_or_else(|| "draft".to_string()),
                    t.customer_id.clone().unwrap_or_default(),
                    t.subtotal.unwrap_or(0).to_string(),
                    t.tax_total.unwrap_or(0).to_string(),
                    t.discount_total.unwrap_or(0).to_string(),
                    t.total.unwrap_or(0).to_string(),
                    t.currency
                        .clone()
                        .unwrap_or_else(|| "EUR".to_string()),
                    now,
                ],
                "ticket_insert",
            )
            .await?;
        Ok(())
    }

    /// Liste les tickets d'un vendeur (synchrone).
    pub fn tickets_by_seller(&self, seller_id: &str) -> Result<Vec<Ticket>, DbError> {
        run_blocking(self.tickets_by_seller_async(seller_id))
    }

    async fn tickets_by_seller_async(&self, seller_id: &str) -> Result<Vec<Ticket>, DbError> {
        let _rows = self
            .client
            .query(
                "SELECT id, ticket_number, seller_id, session_id, event_id, source, status,
                        customer_id, subtotal, tax_total, discount_total, total, currency,
                        created_at, closed_at, refund_of
                 FROM tickets WHERE seller_id = ?1
                 ORDER BY created_at DESC",
                vec![seller_id.to_string()],
            )
            .await?;
        // TODO: mapper les rows en Vec<Ticket>
        Ok(vec![])
    }

    // -----------------------------------------------------------------------
    // Events
    // -----------------------------------------------------------------------

    /// Insère un nouvel événement (synchrone).
    pub fn event_insert(&self, e: &Event) -> Result<(), DbError> {
        run_blocking(self.event_insert_async(e))
    }

    async fn event_insert_async(&self, e: &Event) -> Result<(), DbError> {
        let id = e
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();

        self.client
            .execute(
                "INSERT INTO events (id, seller_id, name, start_date, end_date, location,
                    stand_info, status, notes, jayfestival_edition_id, jayfestival_candidature_id,
                    jayfestival_sync_status, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                vec![
                    id,
                    e.seller_id.clone().unwrap_or_default(),
                    e.name.clone().unwrap_or_default(),
                    e.start_date.clone().unwrap_or_default(),
                    e.end_date.clone().unwrap_or_default(),
                    e.location.clone().unwrap_or_default(),
                    e.stand_info.clone().unwrap_or_default(),
                    e.status
                        .clone()
                        .unwrap_or_else(|| "draft".to_string()),
                    e.notes.clone().unwrap_or_default(),
                    e.jayfestival_edition_id.clone().unwrap_or_default(),
                    e.jayfestival_candidature_id.clone().unwrap_or_default(),
                    e.jayfestival_sync_status.clone().unwrap_or_default(),
                    now.clone(),
                    now,
                ],
                "event_insert",
            )
            .await?;
        Ok(())
    }

    /// Liste les événements d'un vendeur (synchrone).
    pub fn events_by_seller(&self, seller_id: &str) -> Result<Vec<Event>, DbError> {
        run_blocking(self.events_by_seller_async(seller_id))
    }

    async fn events_by_seller_async(&self, seller_id: &str) -> Result<Vec<Event>, DbError> {
        let _rows = self
            .client
            .query(
                "SELECT id, seller_id, name, start_date, end_date, location, stand_info,
                        status, notes, jayfestival_edition_id, jayfestival_candidature_id,
                        jayfestival_sync_status, total_revenue, total_costs, gross_profit,
                        created_at, updated_at, closed_at
                 FROM events WHERE seller_id = ?1
                 ORDER BY start_date DESC",
                vec![seller_id.to_string()],
            )
            .await?;
        // TODO: mapper les rows en Vec<Event>
        Ok(vec![])
    }

    // -----------------------------------------------------------------------
    // Event Costs
    // -----------------------------------------------------------------------

    /// Insère un coût événement (synchrone).
    pub fn event_cost_insert(&self, c: &EventCost) -> Result<(), DbError> {
        run_blocking(self.event_cost_insert_async(c))
    }

    async fn event_cost_insert_async(&self, c: &EventCost) -> Result<(), DbError> {
        let id = c
            .id
            .clone()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let now = chrono::Utc::now().to_rfc3339();

        self.client
            .execute(
                "INSERT INTO event_costs (id, event_id, category, label, amount, currency, cost_date, receipt_url, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                vec![
                    id,
                    c.event_id.clone().unwrap_or_default(),
                    c.category.clone().unwrap_or_else(|| "other".to_string()),
                    c.label.clone().unwrap_or_default(),
                    c.amount.unwrap_or(0).to_string(),
                    c.currency.clone().unwrap_or_else(|| "EUR".to_string()),
                    c.cost_date.clone().unwrap_or_default(),
                    c.receipt_url.clone().unwrap_or_default(),
                    now,
                ],
                "event_cost_insert",
            )
            .await?;
        Ok(())
    }

    /// Liste les coûts d'un événement (synchrone).
    pub fn event_costs_by_event(&self, event_id: &str) -> Result<Vec<EventCost>, DbError> {
        run_blocking(self.event_costs_by_event_async(event_id))
    }

    async fn event_costs_by_event_async(&self, event_id: &str) -> Result<Vec<EventCost>, DbError> {
        let _rows = self
            .client
            .query(
                "SELECT id, event_id, category, label, amount, currency, cost_date, receipt_url, created_at
                 FROM event_costs WHERE event_id = ?1
                 ORDER BY created_at DESC",
                vec![event_id.to_string()],
            )
            .await?;
        // TODO: mapper les rows en Vec<EventCost>
        Ok(vec![])
    }

    // -----------------------------------------------------------------------
    // Shop Config
    // -----------------------------------------------------------------------

    /// Récupère la configuration boutique d'un vendeur (synchrone).
    pub fn shop_config_get(&self, seller_id: &str) -> Result<Option<ShopConfig>, DbError> {
        run_blocking(self.shop_config_get_async(seller_id))
    }

    async fn shop_config_get_async(
        &self,
        seller_id: &str,
    ) -> Result<Option<ShopConfig>, DbError> {
        let _rows = self
            .client
            .query(
                "SELECT id, seller_id, shop_name, slug, currency, legal_cgv, legal_mentions,
                        created_at, updated_at
                 FROM shop_config WHERE seller_id = ?1 LIMIT 1",
                vec![seller_id.to_string()],
            )
            .await?;
        // TODO: mapper
        Ok(None)
    }

    // -----------------------------------------------------------------------
    // Payment Methods
    // -----------------------------------------------------------------------

    /// Liste les modes de paiement d'un vendeur (synchrone).
    pub fn payment_methods_by_seller(
        &self,
        seller_id: &str,
    ) -> Result<Vec<PaymentMethod>, DbError> {
        run_blocking(self.payment_methods_by_seller_async(seller_id))
    }

    async fn payment_methods_by_seller_async(
        &self,
        seller_id: &str,
    ) -> Result<Vec<PaymentMethod>, DbError> {
        let _rows = self
            .client
            .query(
                "SELECT id, seller_id, name, method_type, sort_order, is_active, created_at
                 FROM payment_methods WHERE seller_id = ?1
                 ORDER BY sort_order ASC",
                vec![seller_id.to_string()],
            )
            .await?;
        // TODO: mapper
        Ok(vec![])
    }
}
