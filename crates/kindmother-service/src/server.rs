//! Serveur TCP/JSON KindMother.
//!
//! @id: kindmother_service_server
//! @do: implement_tcp_json_server
//! @layer: core

use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn, error, debug, instrument};

use crate::arbitration::ArbitrationEngine;
use crate::database::EncryptedDatabase;
use crate::errors::ServiceError;
use crate::protocol::{Request, Response, Operation};

/// Serveur KindMother - point d'entrée unique pour la persistance.
pub struct KindMotherServer {
    /// Gestionnaire de bases de données
    database: Arc<EncryptedDatabase>,
    /// Moteur d'arbitrage
    arbitration: Arc<ArbitrationEngine>,
}

impl KindMotherServer {
    /// Crée un nouveau serveur KindMother.
    ///
    /// # Arguments
    /// * `data_dir` - Répertoire de stockage des bases de données
    pub async fn new(data_dir: impl AsRef<Path>) -> Result<Self, ServiceError> {
        let database = Arc::new(EncryptedDatabase::new(data_dir)?);
        let arbitration = Arc::new(ArbitrationEngine::new());

        // Enregistrer les opérateurs par défaut
        arbitration.register_default_operators().await;

        info!("KindMother server initialized");

        Ok(Self {
            database,
            arbitration,
        })
    }

    /// Démarre le serveur sur l'adresse spécifiée.
    ///
    /// # Arguments
    /// * `addr` - Adresse d'écoute (ex: "127.0.0.1:50051")
    pub async fn serve(self, addr: &str) -> Result<(), ServiceError> {
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| ServiceError::Internal(format!("Cannot bind: {}", e)))?;

        info!("KindMother server listening on {}", addr);

        let server = Arc::new(self);

        loop {
            match listener.accept().await {
                Ok((stream, peer)) => {
                    debug!("New connection from {}", peer);
                    let server = server.clone();
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream).await {
                            warn!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    }

    /// Gère une connexion client.
    async fn handle_connection(&self, mut stream: TcpStream) -> Result<(), ServiceError> {
        loop {
            // Lire la longueur du message (4 bytes big endian)
            let mut len_buf = [0u8; 4];
            match stream.read_exact(&mut len_buf).await {
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    debug!("Client disconnected");
                    return Ok(());
                }
                Err(e) => {
                    return Err(ServiceError::Transport(format!("Read error: {}", e)));
                }
            }

            let len = u32::from_be_bytes(len_buf) as usize;
            if len > 10 * 1024 * 1024 {
                // Max 10 MB
                return Err(ServiceError::Validation("Message too large".to_string()));
            }

            // Lire le payload JSON
            let mut payload = vec![0u8; len];
            stream
                .read_exact(&mut payload)
                .await
                .map_err(|e| ServiceError::Transport(format!("Read error: {}", e)))?;

            // Parser la requête
            let request: Request = serde_json::from_slice(&payload)
                .map_err(|e| ServiceError::Validation(format!("Invalid JSON: {}", e)))?;

            // Traiter la requête
            let response = self.handle_request(request).await;

            // Sérialiser la réponse
            let response_json = serde_json::to_vec(&response)
                .map_err(|e| ServiceError::Internal(format!("Serialize error: {}", e)))?;

            // Écrire la longueur puis le payload
            let len_bytes = (response_json.len() as u32).to_be_bytes();
            stream
                .write_all(&len_bytes)
                .await
                .map_err(|e| ServiceError::Transport(format!("Write error: {}", e)))?;
            stream
                .write_all(&response_json)
                .await
                .map_err(|e| ServiceError::Transport(format!("Write error: {}", e)))?;
        }
    }

    /// Traite une requête et retourne une réponse.
    #[instrument(skip(self))]
    async fn handle_request(&self, request: Request) -> Response {
        match request {
            Request::Query {
                operator_id,
                database,
                sql,
                params,
            } => self.handle_query(&operator_id, &database, &sql, params).await,

            Request::Execute {
                operator_id,
                database,
                sql,
                params,
                intent,
            } => {
                self.handle_execute(&operator_id, &database, &sql, params, &intent)
                    .await
            }

            Request::Transaction {
                operator_id,
                database,
                operations,
                intent,
            } => {
                self.handle_transaction(&operator_id, &database, operations, &intent)
                    .await
            }

            Request::HealthCheck { database } => self.handle_health_check(database.as_deref()).await,

            Request::DatabaseInfo {
                operator_id,
                database,
            } => self.handle_database_info(&operator_id, &database).await,
        }
    }

    async fn handle_query(
        &self,
        operator_id: &str,
        database: &str,
        sql: &str,
        params: Vec<String>,
    ) -> Response {
        // Arbitrage
        if let Err(e) = self
            .arbitration
            .authorize(operator_id, database, sql, false)
            .await
        {
            return Response::error("PERMISSION_DENIED", e.to_string());
        }

        // Exécution
        match self.database.query(database, sql, params).await {
            Ok(rows) => {
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

    async fn handle_execute(
        &self,
        operator_id: &str,
        database: &str,
        sql: &str,
        params: Vec<String>,
        intent: &str,
    ) -> Response {
        // Arbitrage
        if let Err(e) = self
            .arbitration
            .authorize(operator_id, database, sql, true)
            .await
        {
            return Response::error("PERMISSION_DENIED", e.to_string());
        }

        // Exécution
        match self
            .database
            .execute(database, operator_id, sql, params, intent)
            .await
        {
            Ok((rows_affected, last_insert_id)) => Response::ExecuteResult {
                success: true,
                rows_affected,
                last_insert_id,
                error: None,
            },
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

    async fn handle_transaction(
        &self,
        operator_id: &str,
        database: &str,
        operations: Vec<Operation>,
        intent: &str,
    ) -> Response {
        // Arbitrage pour chaque opération
        for op in &operations {
            if let Err(e) = self
                .arbitration
                .authorize(operator_id, database, &op.sql, true)
                .await
            {
                return Response::error("PERMISSION_DENIED", e.to_string());
            }
        }

        let ops: Vec<(String, Vec<String>)> = operations
            .into_iter()
            .map(|op| (op.sql, op.params))
            .collect();

        // Exécution
        match self
            .database
            .transaction(database, operator_id, ops, intent)
            .await
        {
            Ok(results) => Response::TransactionResult {
                success: true,
                results,
                error: None,
            },
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

    async fn handle_health_check(&self, database: Option<&str>) -> Response {
        let start = std::time::Instant::now();

        if let Some(db) = database {
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
            .authorize(operator_id, database, "SELECT 1", false)
            .await
        {
            return Response::error("PERMISSION_DENIED", e.to_string());
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
