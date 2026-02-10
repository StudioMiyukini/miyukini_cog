//! Client KindMother - délégation vers le service de persistance.
//!
//! @id: kindmother_client_impl
//! @do: implement_delegation_client
//! @layer: toolkit

use std::collections::HashMap;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, instrument};

use crate::errors::ClientError;
use crate::protocol::{Operation, Request, Response};

/// Client de délégation vers KindMother.
///
/// Ce client encapsule toute la communication avec le service KindMother.
/// Les opérateurs n'ont jamais accès direct à la base de données.
pub struct KindMotherClient {
    /// Stream TCP
    stream: Mutex<TcpStream>,
    /// Identifiant de l'opérateur
    operator_id: String,
    /// Base de données par défaut
    default_database: String,
}

impl KindMotherClient {
    /// Crée une nouvelle connexion vers le service KindMother.
    ///
    /// # Arguments
    /// * `addr` - Adresse du service (ex: "127.0.0.1:50051")
    /// * `operator_id` - Identifiant de l'opérateur (ex: "jayxpose")
    /// * `database` - Nom de la base de données par défaut
    pub async fn connect(addr: &str, operator_id: &str, database: &str) -> Result<Self, ClientError> {
        let stream = tokio::time::timeout(Duration::from_secs(10), TcpStream::connect(addr))
            .await
            .map_err(|_| ClientError::Timeout("Connection timeout".to_string()))?
            .map_err(|e| ClientError::Connection(e.to_string()))?;

        debug!(
            "Connected to KindMother service at {} as operator '{}'",
            addr, operator_id
        );

        Ok(Self {
            stream: Mutex::new(stream),
            operator_id: operator_id.to_string(),
            default_database: database.to_string(),
        })
    }

    /// Envoie une requête et reçoit la réponse.
    async fn send_request(&self, request: Request) -> Result<Response, ClientError> {
        let payload = serde_json::to_vec(&request)?;

        let mut stream = self.stream.lock().await;

        // Écrire la longueur puis le payload
        let len_bytes = (payload.len() as u32).to_be_bytes();
        stream.write_all(&len_bytes).await?;
        stream.write_all(&payload).await?;

        // Lire la longueur de la réponse
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;

        // Lire la réponse
        let mut response_buf = vec![0u8; len];
        stream.read_exact(&mut response_buf).await?;

        let response: Response = serde_json::from_slice(&response_buf)?;

        // Vérifier les erreurs
        if let Response::Error { code, message } = &response {
            return Err(match code.as_str() {
                "PERMISSION_DENIED" => ClientError::PermissionDenied(message.clone()),
                "NOT_FOUND" => ClientError::DatabaseNotFound(message.clone()),
                "QUOTA_EXCEEDED" => ClientError::QuotaExceeded(message.clone()),
                _ => ClientError::Service(message.clone()),
            });
        }

        Ok(response)
    }

    /// Exécute une requête SELECT et retourne les lignes.
    #[instrument(skip(self, params))]
    pub async fn query<S: AsRef<str>>(
        &self,
        sql: &str,
        params: Vec<S>,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>, ClientError> {
        self.query_db(&self.default_database, sql, params).await
    }

    /// Exécute une requête SELECT sur une base spécifique.
    #[instrument(skip(self, params))]
    pub async fn query_db<S: AsRef<str>>(
        &self,
        database: &str,
        sql: &str,
        params: Vec<S>,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>, ClientError> {
        let request = Request::Query {
            operator_id: self.operator_id.clone(),
            database: database.to_string(),
            sql: sql.to_string(),
            params: params.iter().map(|p| p.as_ref().to_string()).collect(),
        };

        let response = self.send_request(request).await?;

        match response {
            Response::QueryResult {
                success,
                rows,
                error,
                ..
            } => {
                if !success {
                    return Err(ClientError::Service(
                        error.unwrap_or_else(|| "Unknown error".to_string()),
                    ));
                }

                // Convertir les rows en HashMap
                rows.into_iter()
                    .map(|row| {
                        row.as_object()
                            .map(|obj| {
                                obj.iter()
                                    .map(|(k, v)| (k.clone(), v.clone()))
                                    .collect()
                            })
                            .ok_or_else(|| ClientError::Serialization("Invalid row format".to_string()))
                    })
                    .collect()
            }
            _ => Err(ClientError::Service("Unexpected response type".to_string())),
        }
    }

    /// Exécute une requête SELECT et retourne une seule ligne optionnelle.
    pub async fn query_one<S: AsRef<str>>(
        &self,
        sql: &str,
        params: Vec<S>,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, ClientError> {
        let rows = self.query(sql, params).await?;
        Ok(rows.into_iter().next())
    }

    /// Exécute une requête d'écriture (INSERT, UPDATE, DELETE).
    #[instrument(skip(self, params))]
    pub async fn execute<S: AsRef<str>>(
        &self,
        sql: &str,
        params: Vec<S>,
        intent: &str,
    ) -> Result<(u64, Option<i64>), ClientError> {
        self.execute_db(&self.default_database, sql, params, intent)
            .await
    }

    /// Exécute une requête d'écriture sur une base spécifique.
    #[instrument(skip(self, params))]
    pub async fn execute_db<S: AsRef<str>>(
        &self,
        database: &str,
        sql: &str,
        params: Vec<S>,
        intent: &str,
    ) -> Result<(u64, Option<i64>), ClientError> {
        let request = Request::Execute {
            operator_id: self.operator_id.clone(),
            database: database.to_string(),
            sql: sql.to_string(),
            params: params.iter().map(|p| p.as_ref().to_string()).collect(),
            intent: intent.to_string(),
        };

        let response = self.send_request(request).await?;

        match response {
            Response::ExecuteResult {
                success,
                rows_affected,
                last_insert_id,
                error,
            } => {
                if !success {
                    return Err(ClientError::Service(
                        error.unwrap_or_else(|| "Unknown error".to_string()),
                    ));
                }
                Ok((rows_affected, last_insert_id))
            }
            _ => Err(ClientError::Service("Unexpected response type".to_string())),
        }
    }

    /// Exécute une transaction avec plusieurs opérations.
    #[instrument(skip(self, operations))]
    pub async fn transaction<S: AsRef<str>>(
        &self,
        operations: Vec<(&str, Vec<S>)>,
        intent: &str,
    ) -> Result<Vec<u64>, ClientError> {
        self.transaction_db(&self.default_database, operations, intent)
            .await
    }

    /// Exécute une transaction sur une base spécifique.
    #[instrument(skip(self, operations))]
    pub async fn transaction_db<S: AsRef<str>>(
        &self,
        database: &str,
        operations: Vec<(&str, Vec<S>)>,
        intent: &str,
    ) -> Result<Vec<u64>, ClientError> {
        let ops: Vec<Operation> = operations
            .into_iter()
            .map(|(sql, params)| Operation {
                sql: sql.to_string(),
                params: params.iter().map(|p| p.as_ref().to_string()).collect(),
            })
            .collect();

        let request = Request::Transaction {
            operator_id: self.operator_id.clone(),
            database: database.to_string(),
            operations: ops,
            intent: intent.to_string(),
        };

        let response = self.send_request(request).await?;

        match response {
            Response::TransactionResult {
                success,
                results,
                error,
            } => {
                if !success {
                    return Err(ClientError::Service(
                        error.unwrap_or_else(|| "Transaction failed".to_string()),
                    ));
                }
                Ok(results)
            }
            _ => Err(ClientError::Service("Unexpected response type".to_string())),
        }
    }

    /// Vérifie la santé du service.
    pub async fn health_check(&self) -> Result<bool, ClientError> {
        let request = Request::HealthCheck {
            database: Some(self.default_database.clone()),
        };

        let response = self.send_request(request).await?;

        match response {
            Response::HealthCheckResult { healthy, .. } => Ok(healthy),
            _ => Err(ClientError::Service("Unexpected response type".to_string())),
        }
    }

    /// Obtient les informations sur la base de données.
    pub async fn database_info(&self) -> Result<DatabaseInfo, ClientError> {
        let request = Request::DatabaseInfo {
            operator_id: self.operator_id.clone(),
            database: self.default_database.clone(),
        };

        let response = self.send_request(request).await?;

        match response {
            Response::DatabaseInfoResult {
                success,
                database,
                size_bytes,
                table_count,
                encrypted,
                error,
            } => {
                if !success {
                    return Err(ClientError::Service(
                        error.unwrap_or_else(|| "Unknown error".to_string()),
                    ));
                }
                Ok(DatabaseInfo {
                    database,
                    size_bytes,
                    table_count,
                    encrypted,
                })
            }
            _ => Err(ClientError::Service("Unexpected response type".to_string())),
        }
    }

    /// Retourne l'identifiant de l'opérateur.
    pub fn operator_id(&self) -> &str {
        &self.operator_id
    }

    /// Retourne le nom de la base de données par défaut.
    pub fn database(&self) -> &str {
        &self.default_database
    }
}

/// Informations sur une base de données.
#[derive(Debug, Clone)]
pub struct DatabaseInfo {
    /// Nom de la base
    pub database: String,
    /// Taille en bytes
    pub size_bytes: u64,
    /// Nombre de tables
    pub table_count: u32,
    /// Base chiffrée
    pub encrypted: bool,
}
