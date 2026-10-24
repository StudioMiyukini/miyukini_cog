//! Serveur TCP/JSON KindMother.
//!
//! @id     kindmother_service_server
//! @do     implement_tcp_json_server
//@human ! @layer  core
//!
//! Sécurité (K-04) :
//! - écoute sur la boucle locale uniquement, sauf `allow_non_loopback` explicite (C-21) ;
//! - chaque connexion commence par une authentification **mutuelle** (`kindmother-auth-v2`,
//!   voir `credentials`) ; tant que l'opérateur n'est pas authentifié, aucune requête
//!   n'est traitée ; toute autre version de protocole est refusée ;
//! - l'identité authentifiée est liée à la connexion ; le champ `operator_id` des
//!   requêtes doit lui être égal, sinon refus (C-20) ;
//! - trames bornées, délais de lecture (C-13).
//!
//! Résistance à l'épuisement (DoS) :
//! - les connexions non authentifiées occupent une table **séparée et petite**
//!   (`max_preauth`, `max_preauth_per_peer`) ; quand elle est pleine, la poignée de main
//!   en attente la plus ancienne (du même pair d'abord) est évincée : une inondation de
//!   connexions muettes ne bloque pas un client légitime, dont la poignée de main est
//!   brève ;
//! - un emplacement principal (`max_connections`) n'est pris **qu'après** une
//!   authentification réussie ;
//! - la poignée de main est bornée par `auth_timeout` ; une connexion authentifiée
//!   inactive est fermée après `idle_timeout`.

use std::collections::BTreeMap;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{oneshot, OwnedSemaphorePermit, Semaphore};
use tracing::{debug, error, info, instrument, warn};

use crate::arbitration::{AccessMode, ArbitrationEngine};
use crate::credentials::{
    decode_hex_fixed, encode_hex, new_nonce, CredentialStore, AUTH_PROTOCOL, NONCE_LEN,
};
use crate::database::EncryptedDatabase;
use crate::errors::ServiceError;
use crate::audit::{AuditKind, AuditRecord};
use crate::protocol::{Operation, Request, Response};
use kindmother::intent::{
    IntentState, SchemaMigration, TypedSelect, TypedWriteIntent, MAX_INTENTS_PER_BATCH,
};
use crate::sql_guard::SqlPolicy;

/// Bornes du serveur (C-13).
#[derive(Debug, Clone, Copy)]
pub struct ServerLimits {
    /// Connexions **authentifiées** simultanées maximales
    pub max_connections: usize,
    /// Poignées de main (connexions non authentifiées) simultanées maximales
    pub max_preauth: usize,
    /// Poignées de main simultanées maximales par adresse IP de pair
    pub max_preauth_per_peer: usize,
    /// Taille maximale d'une trame de requête (octets)
    pub max_request_frame: usize,
    /// Taille maximale d'une trame d'authentification (octets)
    pub max_auth_frame: usize,
    /// Délai pour terminer l'authentification
    pub auth_timeout: Duration,
    /// Délai d'inactivité d'une connexion authentifiée avant fermeture
    pub idle_timeout: Duration,
    /// Délai pour recevoir le corps d'une trame une fois l'en-tête lu
    pub frame_body_timeout: Duration,
}

impl Default for ServerLimits {
    fn default() -> Self {
        Self {
            max_connections: 64,
            max_preauth: 16,
            max_preauth_per_peer: 8,
            max_request_frame: 4 * 1024 * 1024,
            max_auth_frame: 4 * 1024,
            auth_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(5 * 60),
            frame_body_timeout: Duration::from_secs(30),
        }
    }
}

/// Poignée de main en attente : l'abandon de `_cancel` l'interrompt (éviction).
struct PendingHandshake {
    peer: IpAddr,
    _cancel: oneshot::Sender<()>,
}

/// Table des poignées de main en cours, bornée globalement et par pair.
///
/// Les identifiants sont croissants : l'ordre de la table est l'ordre d'arrivée.
#[derive(Default)]
struct PreAuthTable {
    next_id: u64,
    pending: BTreeMap<u64, PendingHandshake>,
}

impl PreAuthTable {
    /// Admet une nouvelle poignée de main, en évinçant au besoin les plus anciennes
    /// (celles du même pair d'abord). Retourne l'identifiant et le signal d'éviction.
    fn admit(
        &mut self,
        peer: IpAddr,
        max_total: usize,
        max_per_peer: usize,
    ) -> (u64, oneshot::Receiver<()>) {
        let max_total = max_total.max(1);
        let max_per_peer = max_per_peer.max(1);

        loop {
            let same_peer: Vec<u64> = self
                .pending
                .iter()
                .filter(|(_, p)| p.peer == peer)
                .map(|(id, _)| *id)
                .collect();
            let victim = if same_peer.len() >= max_per_peer {
                same_peer.first().copied()
            } else if self.pending.len() >= max_total {
                self.pending.keys().next().copied()
            } else {
                None
            };
            match victim {
                Some(id) => {
                    debug!("Pre-auth table full, evicting pending handshake #{id}");
                    self.pending.remove(&id);
                }
                None => break,
            }
        }

        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        let (tx, rx) = oneshot::channel();
        self.pending
            .insert(id, PendingHandshake { peer, _cancel: tx });
        (id, rx)
    }

    fn release(&mut self, id: u64) {
        self.pending.remove(&id);
    }
}

/// Libère l'entrée de la table à la fin de la poignée de main (quelle qu'en soit l'issue).
struct PreAuthGuard {
    table: Arc<Mutex<PreAuthTable>>,
    id: u64,
}

impl Drop for PreAuthGuard {
    fn drop(&mut self) {
        lock_table(&self.table).release(self.id);
    }
}

/// Verrouille la table ; un verrou empoisonné reste utilisable (état toujours cohérent).
fn lock_table(table: &Mutex<PreAuthTable>) -> std::sync::MutexGuard<'_, PreAuthTable> {
    table
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// Serveur KindMother - point d'entrée unique pour la persistance.
pub struct KindMotherServer {
    /// Gestionnaire de bases de données
    database: Arc<EncryptedDatabase>,
    /// Moteur d'arbitrage
    arbitration: Arc<ArbitrationEngine>,
    /// Clés publiques des opérateurs et identité du serveur
    credentials: Arc<CredentialStore>,
    /// Bornes
    limits: ServerLimits,
    /// Autorise une écoute hors boucle locale (configuration explicite uniquement)
    allow_non_loopback: bool,
    /// Opérateurs déjà avertis de l'usage du chemin SQL brut transitoire
    raw_path_warned: Mutex<std::collections::HashSet<String>>,
}

impl KindMotherServer {
    /// Crée un nouveau serveur KindMother.
    ///
    /// L'identité du serveur (`server.key`, `server.pub` à épingler par les clients) et
    /// les justificatifs des opérateurs par défaut sont approvisionnés dans
    /// `<data_dir>/credentials` (voir `credentials`).
    ///
    /// # Arguments
    /// * `data_dir` - Répertoire de stockage des bases de données
    pub async fn new(data_dir: impl AsRef<Path>) -> Result<Self, ServiceError> {
        let data_dir = data_dir.as_ref();
        let database = Arc::new(EncryptedDatabase::new(data_dir)?);
        let arbitration = Arc::new(ArbitrationEngine::new());

        // Enregistrer les opérateurs par défaut
        arbitration.register_default_operators().await;

        let operator_ids = arbitration.operator_ids().await;
        let credentials = Arc::new(CredentialStore::provision(
            data_dir.join("credentials"),
            &operator_ids,
        )?);

        info!(
            "KindMother server initialized ({} operators, credentials in {:?}, server key {})",
            operator_ids.len(),
            credentials.dir(),
            encode_hex(credentials.server().public_key())
        );

        Ok(Self {
            database,
            arbitration,
            credentials,
            limits: ServerLimits::default(),
            allow_non_loopback: false,
            raw_path_warned: Mutex::new(std::collections::HashSet::new()),
        })
    }

    /// Autorise explicitement l'écoute hors boucle locale (déconseillé, C-21).
    pub fn with_non_loopback_allowed(mut self, allow: bool) -> Self {
        self.allow_non_loopback = allow;
        self
    }

    /// Remplace les bornes par défaut.
    pub fn with_limits(mut self, limits: ServerLimits) -> Self {
        self.limits = limits;
        self
    }

    /// Gestionnaire des bases (journal d'audit, chiffrement) — inspection et tests.
    pub fn database(&self) -> &EncryptedDatabase {
        &self.database
    }

    /// Répertoire des justificatifs des opérateurs (et du pin `server.pub`).
    pub fn credentials_dir(&self) -> &Path {
        self.credentials.dir()
    }

    /// Clé publique du serveur, à épingler par les clients.
    pub fn server_public_key(&self) -> [u8; 32] {
        *self.credentials.server().public_key()
    }

    /// Démarre le serveur sur l'adresse spécifiée.
    ///
    /// # Arguments
    /// * `addr` - Adresse d'écoute (ex: "127.0.0.1:50051")
    pub async fn serve(self, addr: &str) -> Result<(), ServiceError> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| ServiceError::Internal(format!("Cannot bind: {}", e)))?;
        self.serve_listener(listener).await
    }

    /// Démarre le serveur sur un écouteur déjà lié.
    pub async fn serve_listener(self, listener: TcpListener) -> Result<(), ServiceError> {
        let local = listener
            .local_addr()
            .map_err(|e| ServiceError::Internal(format!("Cannot read local addr: {e}")))?;
        check_bind_address(local, self.allow_non_loopback)?;

        info!("KindMother server listening on {}", local);

        let slots = Arc::new(Semaphore::new(self.limits.max_connections));
        let preauth = Arc::new(Mutex::new(PreAuthTable::default()));
        let server = Arc::new(self);

        loop {
            match listener.accept().await {
                Ok((stream, peer)) => {
                    debug!("New connection from {}", peer);
                    let (id, cancel) = lock_table(&preauth).admit(
                        peer.ip(),
                        server.limits.max_preauth,
                        server.limits.max_preauth_per_peer,
                    );
                    let guard = PreAuthGuard {
                        table: preauth.clone(),
                        id,
                    };
                    let server = server.clone();
                    let slots = slots.clone();
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream, guard, cancel, slots).await
                        {
                            warn!("Connection from {} closed: {}", peer, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    }

    /// Gère une connexion client : authentification mutuelle puis boucle de requêtes.
    async fn handle_connection(
        &self,
        mut stream: TcpStream,
        guard: PreAuthGuard,
        cancel: oneshot::Receiver<()>,
        slots: Arc<Semaphore>,
    ) -> Result<(), ServiceError> {
        let outcome = tokio::select! {
            biased;
            _ = cancel => Err(ServiceError::Authentication("handshake evicted".to_string())),
            r = tokio::time::timeout(self.limits.auth_timeout, self.authenticate(&mut stream, &slots)) => {
                r.unwrap_or_else(|_| Err(ServiceError::Authentication("authentication timeout".to_string())))
            }
        };
        // Fin de la poignée de main : libère l'entrée de pré-authentification.
        drop(guard);

        let (operator_id, _permit) = match outcome {
            Ok(v) => v,
            Err(e) => {
                let _ = tokio::time::timeout(
                    Duration::from_secs(1),
                    write_response(&mut stream, &handshake_error(&e)),
                )
                .await;
                return Err(e);
            }
        };

        debug!("Operator '{}' authenticated", operator_id);

        loop {
            let Some(payload) = read_frame(
                &mut stream,
                self.limits.max_request_frame,
                Some(self.limits.idle_timeout),
                self.limits.frame_body_timeout,
            )
            .await?
            else {
                debug!("Client disconnected");
                return Ok(());
            };

            let response = match serde_json::from_slice::<Request>(&payload) {
                Ok(request) => self.handle_request(&operator_id, request).await,
                Err(_) => Response::error("INVALID_REQUEST", "Invalid JSON request"),
            };

            write_response(&mut stream, &response).await?;
        }
    }

    /// Poignée de main `kindmother-auth-v2` ; retourne l'identité authentifiée et
    /// l'emplacement principal pris après authentification.
    async fn authenticate(
        &self,
        stream: &mut TcpStream,
        slots: &Arc<Semaphore>,
    ) -> Result<(String, OwnedSemaphorePermit), ServiceError> {
        // 1. ClientHello
        let request = read_auth_request(stream, &self.limits).await?;
        let Request::ClientHello {
            protocol,
            client_nonce,
        } = request
        else {
            return Err(ServiceError::Authentication(
                "authentication required".to_string(),
            ));
        };
        if protocol != AUTH_PROTOCOL {
            return Err(ServiceError::Validation(format!(
                "unsupported authentication protocol (expected {AUTH_PROTOCOL})"
            )));
        }
        let client_nonce = decode_hex_fixed::<NONCE_LEN>(&client_nonce)
            .ok_or_else(|| ServiceError::Authentication("malformed client nonce".to_string()))?;

        // 2. Challenge signé par le serveur
        let server = self.credentials.server();
        let server_nonce = new_nonce();
        let server_sig = server.sign_challenge(&client_nonce, &server_nonce);
        write_response(
            stream,
            &Response::Challenge {
                protocol: AUTH_PROTOCOL.to_string(),
                server_nonce: encode_hex(&server_nonce),
                server_pub: encode_hex(server.public_key()),
                server_sig: encode_hex(&server_sig.to_bytes()),
            },
        )
        .await?;

        // 3. Authenticate
        let request = read_auth_request(stream, &self.limits).await?;
        let Request::Authenticate {
            operator_id,
            signature,
        } = request
        else {
            return Err(ServiceError::Authentication(
                "authentication required".to_string(),
            ));
        };
        self.credentials
            .verify(&operator_id, &client_nonce, &server_nonce, &signature)?;

        // 4. Emplacement principal pris seulement après authentification.
        let permit = slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| ServiceError::QuotaExceeded("connection limit reached".to_string()))?;

        write_response(
            stream,
            &Response::Authenticated {
                operator_id: operator_id.clone(),
            },
        )
        .await?;

        Ok((operator_id, permit))
    }

    /// Traite une requête d'un opérateur authentifié.
    #[instrument(skip(self, request))]
    async fn handle_request(&self, authenticated: &str, request: Request) -> Response {
        match request {
            Request::ClientHello { .. } | Request::Authenticate { .. } => {
                Response::error("INVALID_REQUEST", "Connection already authenticated")
            }

            Request::SubmitIntents {
                operator_id,
                database,
                intents,
            } => match check_identity(authenticated, &operator_id) {
                Err(r) => r,
                Ok(()) => self.handle_intents(authenticated, &database, intents).await,
            },

            Request::Select {
                operator_id,
                database,
                query,
            } => match check_identity(authenticated, &operator_id) {
                Err(r) => r,
                Ok(()) => self.handle_select(authenticated, &database, &query).await,
            },

            Request::ApplyMigrations {
                operator_id,
                database,
                migrations,
            } => match check_identity(authenticated, &operator_id) {
                Err(r) => r,
                Ok(()) => {
                    self.handle_migrations(authenticated, &database, &migrations)
                        .await
                }
            },

            Request::Query {
                operator_id,
                database,
                sql,
                params,
            } => match check_identity(authenticated, &operator_id) {
                Err(r) => r,
                Ok(()) => {
                    self.handle_query(authenticated, &database, &sql, params)
                        .await
                }
            },

            Request::Execute {
                operator_id,
                database,
                sql,
                params,
                intent,
            } => match check_identity(authenticated, &operator_id) {
                Err(r) => r,
                Ok(()) => {
                    self.handle_execute(authenticated, &database, &sql, params, &intent)
                        .await
                }
            },

            Request::Transaction {
                operator_id,
                database,
                operations,
                intent,
            } => match check_identity(authenticated, &operator_id) {
                Err(r) => r,
                Ok(()) => {
                    self.handle_transaction(authenticated, &database, operations, &intent)
                        .await
                }
            },

            Request::HealthCheck { database } => {
                self.handle_health_check(authenticated, database.as_deref())
                    .await
            }

            Request::DatabaseInfo {
                operator_id,
                database,
            } => match check_identity(authenticated, &operator_id) {
                Err(r) => r,
                Ok(()) => self.handle_database_info(authenticated, &database).await,
            },
        }
    }

    /// Journalise un refus d'arbitrage (avant tout accès à la base).
    fn audit_refusal(
        &self,
        kind: AuditKind,
        operator_id: &str,
        database: &str,
        operation: &str,
        table: Option<&str>,
        err: &ServiceError,
    ) {
        let record = AuditRecord {
            kind,
            intent_id: None,
            operator_id: operator_id.to_string(),
            database: database.to_string(),
            operation: operation.to_string(),
            table: table.map(str::to_string),
            verdict: "refusee".to_string(),
            decision_digest: None,
            params_hash: String::new(),
            detail: err.to_string(),
        };
        if let Err(e) = self.database.journal().append(&record) {
            error!("AUDIT FAILURE while recording a refusal: {e}");
        }
    }

    /// Signale (une fois par opérateur) l'usage du chemin SQL brut transitoire.
    fn note_raw_path(&self, operator_id: &str) {
        let mut warned = self
            .raw_path_warned
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if warned.insert(operator_id.to_string()) {
            warn!(
                "Operator '{operator_id}' uses the TRANSITIONAL raw SQL path \
                 (Query/Execute/Transaction); migrate to typed intents (lot C, K-13)"
            );
        }
    }

    /// Lot d'intentions typées : contrôle d'identité et de base par intention,
    /// arbitrage par table, puis validation/application/archivage par la base.
    async fn handle_intents(
        &self,
        operator_id: &str,
        database: &str,
        intents: Vec<TypedWriteIntent>,
    ) -> Response {
        if intents.is_empty() || intents.len() > MAX_INTENTS_PER_BATCH {
            return Response::error(
                "INVALID_REQUEST",
                format!("a batch holds 1 to {MAX_INTENTS_PER_BATCH} intents"),
            );
        }
        let mut refusals = Vec::with_capacity(intents.len());
        for intent in &intents {
            let reason = if intent.intent_id.is_empty() || intent.intent_id.len() > 128 {
                Some("intent_id required (1 to 128 characters)".to_string())
            } else if intent.operator_id != operator_id {
                Some("intent operator_id does not match authenticated operator".to_string())
            } else if intent.database != database {
                Some("intent database does not match the batch database".to_string())
            } else {
                self.arbitration
                    .authorize_table(operator_id, database, &intent.table, AccessMode::Write)
                    .await
                    .err()
                    .map(|e| e.to_string())
            };
            refusals.push(reason);
        }
        let outcomes = self
            .database
            .submit_intents(database, operator_id, intents, refusals)
            .await;
        let success = outcomes.iter().all(|o| o.state == IntentState::Appliquee);
        let error = outcomes.iter().find_map(|o| o.reason.clone());
        if !success {
            warn!("Intents from '{operator_id}' on '{database}' not applied: {error:?}");
        }
        Response::IntentsResult {
            success,
            outcomes,
            error,
        }
    }

    async fn handle_select(
        &self,
        operator_id: &str,
        database: &str,
        query: &TypedSelect,
    ) -> Response {
        if let Err(e) = self
            .arbitration
            .authorize(operator_id, database, AccessMode::Read)
            .await
        {
            self.audit_refusal(AuditKind::Select, operator_id, database, "select", Some(&query.table), &e);
            return error_response(&e);
        }
        match self.database.select(database, operator_id, query).await {
            Ok(rows) => rows_response(rows),
            Err(e) => {
                warn!("Select refused or failed for '{}': {}", operator_id, e);
                error_response(&e)
            }
        }
    }

    async fn handle_migrations(
        &self,
        operator_id: &str,
        database: &str,
        migrations: &[SchemaMigration],
    ) -> Response {
        let permission = self
            .arbitration
            .authorize(operator_id, database, AccessMode::Write)
            .await
            .and_then(|p| {
                if p == crate::arbitration::Permission::Admin {
                    Ok(p)
                } else {
                    Err(ServiceError::PermissionDenied(
                        "schema migrations require Admin permission".to_string(),
                    ))
                }
            });
        if let Err(e) = permission {
            self.audit_refusal(AuditKind::Migration, operator_id, database, "migrate", None, &e);
            return error_response(&e);
        }
        match self
            .database
            .apply_migrations(database, operator_id, migrations)
            .await
        {
            Ok(report) => Response::MigrationResult {
                success: true,
                version_before: report.version_before,
                version_after: report.version_after,
                backup: report.backup,
                error: None,
            },
            Err(e @ (ServiceError::PermissionDenied(_) | ServiceError::Validation(_))) => {
                warn!("Migration refused for '{}': {}", operator_id, e);
                error_response(&e)
            }
            Err(e) => {
                error!("Migration failed on '{}': {}", database, e);
                Response::MigrationResult {
                    success: false,
                    version_before: 0,
                    version_after: 0,
                    backup: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// **TRANSITOIRE (lot C)** : SQL brut en lecture, à retirer (voir `Select`).
    async fn handle_query(
        &self,
        operator_id: &str,
        database: &str,
        sql: &str,
        params: Vec<String>,
    ) -> Response {
        self.note_raw_path(operator_id);
        // Arbitrage (identité authentifiée)
        if let Err(e) = self
            .arbitration
            .authorize(operator_id, database, AccessMode::Read)
            .await
        {
            self.audit_refusal(AuditKind::RawRead, operator_id, database, "query", None, &e);
            return error_response(&e);
        }

        // Exécution (connexion lecture seule + autorisateur lecture seule)
        match self.database.query(database, operator_id, sql, params).await {
            Ok(rows) => rows_response(rows),
            Err(e @ (ServiceError::PermissionDenied(_) | ServiceError::Validation(_))) => {
                warn!("Query refused for '{}': {}", operator_id, e);
                error_response(&e)
            }
            Err(e) => {
                warn!("Query failed: {}", e);
                Response::QueryResult {
                    success: false,
                    rows: vec![],
                    row_count: 0,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// **TRANSITOIRE (lot C)** : SQL brut en écriture, à retirer (voir `SubmitIntents`).
    async fn handle_execute(
        &self,
        operator_id: &str,
        database: &str,
        sql: &str,
        params: Vec<String>,
        intent: &str,
    ) -> Response {
        self.note_raw_path(operator_id);
        // Arbitrage
        let permission = match self
            .arbitration
            .authorize(operator_id, database, AccessMode::Write)
            .await
        {
            Ok(p) => p,
            Err(e) => {
                self.audit_refusal(AuditKind::RawWrite, operator_id, database, "execute", None, &e);
                return error_response(&e);
            }
        };
        let policy = SqlPolicy::for_permission(&permission, AccessMode::Write);

        // Exécution
        match self
            .database
            .execute(database, operator_id, policy, sql, params, intent)
            .await
        {
            Ok((rows_affected, last_insert_id)) => Response::ExecuteResult {
                success: true,
                rows_affected,
                last_insert_id,
                error: None,
            },
            Err(e @ (ServiceError::PermissionDenied(_) | ServiceError::Validation(_))) => {
                warn!("Execute refused for '{}': {}", operator_id, e);
                error_response(&e)
            }
            Err(e) => {
                error!("Execute failed: {}", e);
                Response::ExecuteResult {
                    success: false,
                    rows_affected: 0,
                    last_insert_id: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    /// **TRANSITOIRE (lot C)** : transaction SQL brut, à retirer (voir `SubmitIntents`).
    async fn handle_transaction(
        &self,
        operator_id: &str,
        database: &str,
        operations: Vec<Operation>,
        intent: &str,
    ) -> Response {
        self.note_raw_path(operator_id);
        // Arbitrage
        let permission = match self
            .arbitration
            .authorize(operator_id, database, AccessMode::Write)
            .await
        {
            Ok(p) => p,
            Err(e) => {
                self.audit_refusal(AuditKind::RawWrite, operator_id, database, "transaction", None, &e);
                return error_response(&e);
            }
        };
        let policy = SqlPolicy::for_permission(&permission, AccessMode::Write);

        let ops: Vec<(String, Vec<String>)> = operations
            .into_iter()
            .map(|op| (op.sql, op.params))
            .collect();

        // Exécution
        match self
            .database
            .transaction(database, operator_id, policy, ops, intent)
            .await
        {
            Ok(results) => Response::TransactionResult {
                success: true,
                results,
                error: None,
            },
            Err(e @ (ServiceError::PermissionDenied(_) | ServiceError::Validation(_))) => {
                warn!("Transaction refused for '{}': {}", operator_id, e);
                error_response(&e)
            }
            Err(e) => {
                error!("Transaction failed: {}", e);
                Response::TransactionResult {
                    success: false,
                    results: vec![],
                    error: Some(e.to_string()),
                }
            }
        }
    }

    async fn handle_health_check(&self, operator_id: &str, database: Option<&str>) -> Response {
        let start = std::time::Instant::now();

        if let Some(db) = database {
            if let Err(e) = self
                .arbitration
                .authorize(operator_id, db, AccessMode::Read)
                .await
            {
                return error_response(&e);
            }
            match self.database.get_info(db).await {
                Ok(_) => Response::HealthCheckResult {
                    healthy: true,
                    status: format!("Database '{}' is healthy", db),
                    latency_ms: start.elapsed().as_millis() as u64,
                },
                Err(e) => Response::HealthCheckResult {
                    healthy: false,
                    status: format!("Database '{}' error: {}", db, e),
                    latency_ms: start.elapsed().as_millis() as u64,
                },
            }
        } else {
            Response::HealthCheckResult {
                healthy: true,
                status: "KindMother service is healthy".to_string(),
                latency_ms: start.elapsed().as_millis() as u64,
            }
        }
    }

    async fn handle_database_info(&self, operator_id: &str, database: &str) -> Response {
        // Vérifier que l'opérateur a accès à cette base
        if let Err(e) = self
            .arbitration
            .authorize(operator_id, database, AccessMode::Read)
            .await
        {
            return error_response(&e);
        }

        match self.database.get_info(database).await {
            Ok(info) => Response::DatabaseInfoResult {
                success: true,
                database: info.database,
                size_bytes: info.size_bytes,
                table_count: info.table_count,
                encrypted: info.encrypted,
                error: None,
            },
            Err(e) => Response::DatabaseInfoResult {
                success: false,
                database: database.to_string(),
                size_bytes: 0,
                table_count: 0,
                encrypted: false,
                error: Some(e.to_string()),
            },
        }
    }
}

/// Démarre un serveur éphémère sur `127.0.0.1:0` (tests d'intégration des services).
///
/// Retourne l'adresse d'écoute et le répertoire des justificatifs (clés des opérateurs
/// et pin `server.pub`, à charger avec `OperatorCredential::load_from_dir`).
pub async fn spawn_ephemeral(
    data_dir: impl AsRef<Path>,
) -> Result<(SocketAddr, std::path::PathBuf), ServiceError> {
    let server = KindMotherServer::new(data_dir).await?;
    let credentials = server.credentials_dir().to_path_buf();
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| ServiceError::Internal(format!("Cannot bind: {e}")))?;
    let addr = listener
        .local_addr()
        .map_err(|e| ServiceError::Internal(format!("Cannot read local addr: {e}")))?;
    tokio::spawn(server.serve_listener(listener));
    Ok((addr, credentials))
}

/// Comme [`spawn_ephemeral`], sur un fil dédié avec son propre runtime : utilisable
/// depuis un test synchrone (`#[test]`) qui appelle l'API bloquante d'un service.
pub fn spawn_ephemeral_blocking(
    data_dir: impl AsRef<Path>,
) -> Result<(SocketAddr, std::path::PathBuf), ServiceError> {
    let data_dir = data_dir.as_ref().to_path_buf();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("kindmother-ephemeral".to_string())
        .spawn(move || {
            let runtime = match tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = tx.send(Err(ServiceError::Internal(format!("runtime: {e}"))));
                    return;
                }
            };
            runtime.block_on(async move {
                let started = spawn_ephemeral(&data_dir).await;
                let ok = started.is_ok();
                let _ = tx.send(started);
                if ok {
                    std::future::pending::<()>().await;
                }
            });
        })
        .map_err(|e| ServiceError::Internal(format!("thread: {e}")))?;
    rx.recv()
        .map_err(|e| ServiceError::Internal(format!("ephemeral server: {e}")))?
}

/// Réponse de lecture (SQL brut transitoire ou lecture typée).
fn rows_response(rows: crate::database::Rows) -> Response {
    let row_count = rows.len() as u64;
    let rows: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| serde_json::to_value(r).unwrap_or(serde_json::Value::Null))
        .collect();
    Response::QueryResult {
        success: true,
        rows,
        row_count,
        error: None,
    }
}

/// Refuse une écoute hors boucle locale sauf autorisation explicite (C-21).
fn check_bind_address(addr: SocketAddr, allow_non_loopback: bool) -> Result<(), ServiceError> {
    if addr.ip().is_loopback() {
        return Ok(());
    }
    if allow_non_loopback {
        warn!(
            "KindMother listening on non-loopback address {} (explicit configuration)",
            addr
        );
        return Ok(());
    }
    Err(ServiceError::Validation(format!(
        "Refusing non-loopback bind address {addr} (explicit configuration required)"
    )))
}

/// Vérifie que l'`operator_id` déclaré égale l'identité authentifiée.
fn check_identity(authenticated: &str, declared: &str) -> Result<(), Response> {
    if declared == authenticated {
        Ok(())
    } else {
        warn!(
            "Operator '{}' declared foreign identity '{}', refused",
            authenticated, declared
        );
        Err(Response::error(
            "PERMISSION_DENIED",
            "operator_id does not match authenticated operator",
        ))
    }
}

/// Lit et décode une trame de la poignée de main (taille et délai bornés).
async fn read_auth_request(
    stream: &mut TcpStream,
    limits: &ServerLimits,
) -> Result<Request, ServiceError> {
    let payload = read_frame(
        stream,
        limits.max_auth_frame,
        Some(limits.auth_timeout),
        limits.auth_timeout,
    )
    .await?
    .ok_or_else(|| ServiceError::Authentication("connection closed".to_string()))?;
    serde_json::from_slice(&payload)
        .map_err(|_| ServiceError::Authentication("malformed authentication".to_string()))
}

/// Réponse envoyée lors d'un échec de poignée de main (aucun détail sur l'opérateur).
fn handshake_error(err: &ServiceError) -> Response {
    match err {
        ServiceError::QuotaExceeded(_) => Response::error("BUSY", "connection limit reached"),
        ServiceError::Validation(msg) => Response::error("AUTH_FAILED", msg.clone()),
        _ => Response::error("AUTH_FAILED", "authentication failed"),
    }
}

/// Convertit une erreur de contrôle en réponse d'erreur typée.
fn error_response(err: &ServiceError) -> Response {
    let code = match err {
        ServiceError::PermissionDenied(_) | ServiceError::UnknownOperator(_) => "PERMISSION_DENIED",
        ServiceError::QuotaExceeded(_) => "QUOTA_EXCEEDED",
        ServiceError::DatabaseNotFound(_) => "NOT_FOUND",
        ServiceError::Validation(_) => "INVALID_REQUEST",
        ServiceError::Authentication(_) => "AUTH_FAILED",
        _ => "INTERNAL",
    };
    Response::error(code, err.to_string())
}

/// Lit une trame `[u32 BE longueur][payload]` bornée à `max` octets.
///
/// Retourne `None` sur fermeture propre avant l'en-tête. `header_timeout` borne
/// l'attente de l'en-tête (aucune borne si `None` : connexion inactive tolérée).
async fn read_frame(
    stream: &mut TcpStream,
    max: usize,
    header_timeout: Option<Duration>,
    body_timeout: Duration,
) -> Result<Option<Vec<u8>>, ServiceError> {
    let mut len_buf = [0u8; 4];
    let header = stream.read_exact(&mut len_buf);
    let header_result = match header_timeout {
        Some(t) => tokio::time::timeout(t, header)
            .await
            .map_err(|_| ServiceError::Transport("Read timeout".to_string()))?,
        None => header.await,
    };
    match header_result {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(ServiceError::Transport(format!("Read error: {}", e))),
    }

    let len = usize::try_from(u32::from_be_bytes(len_buf))
        .map_err(|_| ServiceError::Validation("Message too large".to_string()))?;
    if len > max {
        return Err(ServiceError::Validation("Message too large".to_string()));
    }

    let mut payload = vec![0u8; len];
    tokio::time::timeout(body_timeout, stream.read_exact(&mut payload))
        .await
        .map_err(|_| ServiceError::Transport("Read timeout".to_string()))?
        .map_err(|e| ServiceError::Transport(format!("Read error: {}", e)))?;
    Ok(Some(payload))
}

/// Écrit une réponse en trame `[u32 BE longueur][JSON]`.
async fn write_response(stream: &mut TcpStream, response: &Response) -> Result<(), ServiceError> {
    let json = serde_json::to_vec(response)
        .map_err(|e| ServiceError::Internal(format!("Serialize error: {}", e)))?;
    let len = u32::try_from(json.len())
        .map_err(|_| ServiceError::Internal("Response too large".to_string()))?;
    stream
        .write_all(&len.to_be_bytes())
        .await
        .map_err(|e| ServiceError::Transport(format!("Write error: {}", e)))?;
    stream
        .write_all(&json)
        .await
        .map_err(|e| ServiceError::Transport(format!("Write error: {}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_hors_boucle_locale_refuse_par_defaut() {
        let lo: SocketAddr = "127.0.0.1:50051".parse().expect("addr");
        let lo6: SocketAddr = "[::1]:50051".parse().expect("addr");
        let any: SocketAddr = "0.0.0.0:50051".parse().expect("addr");
        let lan: SocketAddr = "192.168.1.10:50051".parse().expect("addr");
        assert!(check_bind_address(lo, false).is_ok());
        assert!(check_bind_address(lo6, false).is_ok());
        assert!(check_bind_address(any, false).is_err());
        assert!(check_bind_address(lan, false).is_err());
        assert!(check_bind_address(lan, true).is_ok());
    }

    #[tokio::test]
    async fn intention_portant_une_autre_identite_refusee() {
        let dir = std::env::temp_dir().join(format!("km-srv-{}", uuid::Uuid::new_v4()));
        let Ok(server) = KindMotherServer::new(&dir).await else {
            // Build sans chiffrement et sans KINDMOTHER_DEV_PLAINTEXT : démarrage refusé (attendu).
            return;
        };
        let mut intent = TypedWriteIntent::insert("t").value("a", 1_i64);
        intent.intent_id = "i-1".to_string();
        intent.operator_id = "jaykoa".to_string();
        intent.database = "jaykonta".to_string();
        let resp = server
            .handle_request(
                "jaykonta",
                Request::SubmitIntents {
                    operator_id: "jaykonta".to_string(),
                    database: "jaykonta".to_string(),
                    intents: vec![intent.clone()],
                },
            )
            .await;
        let Response::IntentsResult { success, outcomes, .. } = resp else {
            panic!("IntentsResult attendu");
        };
        assert!(!success);
        assert_eq!(outcomes[0].state, IntentState::Refusee);
        assert!(outcomes[0].archived);
        // Base différente de celle du lot : refus également.
        intent.operator_id = "jaykonta".to_string();
        intent.database = "central".to_string();
        intent.intent_id = "i-2".to_string();
        let resp = server
            .handle_request(
                "jaykonta",
                Request::SubmitIntents {
                    operator_id: "jaykonta".to_string(),
                    database: "jaykonta".to_string(),
                    intents: vec![intent],
                },
            )
            .await;
        assert!(matches!(resp, Response::IntentsResult { success: false, .. }));
        // Aucune base n'a été créée pour des refus.
        assert!(!dir.join("jaykonta.db").exists());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn identite_declaree_differente_refusee() {
        assert!(check_identity("jaykonta", "jaykonta").is_ok());
        assert!(check_identity("jaykonta", "miyukini-central").is_err());
        assert!(check_identity("jaykonta", "").is_err());
    }
}
