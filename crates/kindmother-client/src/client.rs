//! Client KindMother - délégation vers le service de persistance.
//!
//! @id     kindmother_client_impl
//! @do     implement_delegation_client
//@human ! @layer  toolkit

use std::collections::HashMap;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tracing::{debug, instrument};

use rand_core::{OsRng, RngCore};

use crate::credential::{
    decode_hex_fixed, encode_hex, OperatorCredential, AUTH_PROTOCOL, NONCE_LEN, PUBLIC_KEY_LEN,
};
use crate::errors::ClientError;
use crate::protocol::{Operation, Request, Response};
use kindmother::etiquette::{EtatEtiquette, IntentionEtiquette, RefDonnee};
use kindmother::intent::{
    IntentOutcome, IntentState, SchemaMigration, TypedSelect, TypedWriteIntent,
};

/// Taille maximale d'une trame de réponse acceptée (C-13).
const MAX_RESPONSE_FRAME: usize = 64 * 1024 * 1024;

/// Taille maximale d'une trame d'authentification.
const MAX_AUTH_FRAME: usize = 4 * 1024;

/// Délai de connexion et d'authentification.
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

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
    /// Crée une connexion authentifiée vers le service KindMother.
    ///
    /// Authentification mutuelle (K-04, `kindmother-auth-v2`) : le serveur doit prouver
    /// la possession de la clé épinglée dans le justificatif (`server.pub`), puis
    /// l'opérateur signe une transcription liée aux deux nonces et à la clé du serveur.
    /// Sans pin, la connexion est refusée avant tout échange.
    ///
    /// # Arguments
    /// * `addr` - Adresse du service (ex: "127.0.0.1:50051")
    /// * `credential` - Justificatif de l'opérateur avec pin du serveur
    ///   (voir [`OperatorCredential::load`])
    /// * `database` - Nom de la base de données par défaut
    pub async fn connect(
        addr: &str,
        credential: &OperatorCredential,
        database: &str,
    ) -> Result<Self, ClientError> {
        if credential.server_pin().is_none() {
            return Err(ClientError::Authentication(
                "No pinned server key (server.pub missing): refusing to connect".to_string(),
            ));
        }

        let mut stream = tokio::time::timeout(HANDSHAKE_TIMEOUT, TcpStream::connect(addr))
            .await
            .map_err(|_| ClientError::Timeout("Connection timeout".to_string()))?
            .map_err(|e| ClientError::Connection(e.to_string()))?;

        tokio::time::timeout(
            HANDSHAKE_TIMEOUT,
            Self::authenticate(&mut stream, credential),
        )
        .await
        .map_err(|_| ClientError::Timeout("Authentication timeout".to_string()))??;

        debug!(
            "Connected to KindMother service at {} as operator '{}'",
            addr,
            credential.operator_id()
        );

        Ok(Self {
            stream: Mutex::new(stream),
            operator_id: credential.operator_id().to_string(),
            default_database: database.to_string(),
        })
    }

    /// Poignée de main `kindmother-auth-v2` (authentification mutuelle).
    async fn authenticate(
        stream: &mut TcpStream,
        credential: &OperatorCredential,
    ) -> Result<(), ClientError> {
        // 1. ClientHello
        let mut client_nonce = [0u8; NONCE_LEN];
        OsRng.fill_bytes(&mut client_nonce);
        let hello = Request::ClientHello {
            protocol: AUTH_PROTOCOL.to_string(),
            client_nonce: encode_hex(&client_nonce),
        };
        write_frame(stream, &serde_json::to_vec(&hello)?).await?;

        // 2. Challenge : version exacte, clé épinglée, signature du serveur.
        let frame = read_frame(stream, MAX_AUTH_FRAME).await?;
        let (server_nonce, server_pub, server_sig) = parse_challenge(&frame)?;
        credential.verify_server(&client_nonce, &server_nonce, &server_pub, &server_sig)?;

        // 3. Authenticate : signature de la transcription client.
        let request = Request::Authenticate {
            operator_id: credential.operator_id().to_string(),
            signature: credential.sign_transcript(&client_nonce, &server_nonce, &server_pub),
        };
        write_frame(stream, &serde_json::to_vec(&request)?).await?;

        // 4. Confirmation
        let reply = read_frame(stream, MAX_AUTH_FRAME).await?;
        match serde_json::from_slice::<Response>(&reply)? {
            Response::Authenticated { operator_id } if operator_id == credential.operator_id() => {
                Ok(())
            }
            Response::Error { message, .. } => Err(ClientError::Authentication(message)),
            _ => Err(ClientError::Authentication(
                "Unexpected authentication reply".to_string(),
            )),
        }
    }

    /// Envoie une requête et reçoit la réponse.
    async fn send_request(&self, request: Request) -> Result<Response, ClientError> {
        let payload = serde_json::to_vec(&request)?;

        let mut stream = self.stream.lock().await;
        write_frame(&mut stream, &payload).await?;
        let response_buf = read_frame(&mut stream, MAX_RESPONSE_FRAME).await?;

        let response: Response = serde_json::from_slice(&response_buf)?;

        // Vérifier les erreurs
        if let Response::Error { code, message } = &response {
            return Err(match code.as_str() {
                "PERMISSION_DENIED" => ClientError::PermissionDenied(message.clone()),
                "NOT_FOUND" => ClientError::DatabaseNotFound(message.clone()),
                "QUOTA_EXCEEDED" => ClientError::QuotaExceeded(message.clone()),
                "AUTH_FAILED" => ClientError::Authentication(message.clone()),
                "INVALID_REQUEST" => ClientError::InvalidRequest(message.clone()),
                _ => ClientError::Service(message.clone()),
            });
        }

        Ok(response)
    }

    /// Soumet une intention d'écriture typée (lot C, K-13) ; retourne son issue.
    ///
    /// `Err` si l'intention n'est pas appliquée (refusée ou échouée) ; le motif est
    /// dans le message. L'identifiant d'intention est tiré au hasard s'il est vide.
    pub async fn submit(&self, intent: TypedWriteIntent) -> Result<IntentOutcome, ClientError> {
        let mut outcomes = self.submit_batch(vec![intent]).await?;
        outcomes
            .pop()
            .ok_or_else(|| ClientError::Service("empty intent outcome".to_string()))
    }

    /// Soumet un lot d'intentions typées, appliqué atomiquement (même base).
    ///
    /// Les champs `operator_id` (identité authentifiée), `database` (base par défaut si
    /// vide) et `intent_id` (aléatoire si vide) sont complétés. `Err` si une intention
    /// n'est pas appliquée : aucune ne l'est alors.
    pub async fn submit_batch(
        &self,
        intents: Vec<TypedWriteIntent>,
    ) -> Result<Vec<IntentOutcome>, ClientError> {
        let outcomes = self.submit_batch_outcomes(intents).await?;
        if let Some(bad) = outcomes.iter().find(|o| o.state != IntentState::Appliquee) {
            let reason = bad.reason.clone().unwrap_or_default();
            return Err(match bad.state {
                IntentState::Refusee => ClientError::PermissionDenied(format!(
                    "intent {} refused: {reason}",
                    bad.intent_id
                )),
                _ => ClientError::Service(format!("intent {} failed: {reason}", bad.intent_id)),
            });
        }
        Ok(outcomes)
    }

    /// Comme [`Self::submit_batch`], mais retourne les issues sans les convertir en erreur.
    pub async fn submit_batch_outcomes(
        &self,
        intents: Vec<TypedWriteIntent>,
    ) -> Result<Vec<IntentOutcome>, ClientError> {
        let mut intents = intents;
        let database = intents
            .iter()
            .find(|i| !i.database.is_empty())
            .map_or_else(|| self.default_database.clone(), |i| i.database.clone());
        for intent in &mut intents {
            intent.operator_id.clone_from(&self.operator_id);
            if intent.database.is_empty() {
                intent.database.clone_from(&database);
            }
            if intent.intent_id.is_empty() {
                intent.intent_id = uuid::Uuid::new_v4().to_string();
            }
        }
        let request = Request::SubmitIntents {
            operator_id: self.operator_id.clone(),
            database,
            intents,
        };
        match self.send_request(request).await? {
            Response::IntentsResult { outcomes, .. } => Ok(outcomes),
            _ => Err(ClientError::Service("Unexpected response type".to_string())),
        }
    }

    /// Relaie une intention d'étiquette `publiable` **signée par l'utilisateur** (EX-47).
    ///
    /// Le client ne signe rien : l'intention vient signée de l'appareil de
    /// l'utilisateur ; le service la refuse si la signature n'est pas celle d'une
    /// clé d'utilisateur épinglée, si la donnée est `nationale`, ou si l'empreinte
    /// ne correspond pas au contenu courant. Retourne l'état après application.
    pub async fn etiqueter(&self, intention: IntentionEtiquette) -> Result<EtatEtiquette, ClientError> {
        let request = Request::Etiqueter {
            operator_id: self.operator_id.clone(),
            intention,
        };
        etat_of(self.send_request(request).await?)
    }

    /// Lit l'état de l'étiquette `publiable` d'une donnée exacte (EX-47).
    pub async fn lire_etiquette(&self, donnee: RefDonnee) -> Result<EtatEtiquette, ClientError> {
        let request = Request::LireEtiquette {
            operator_id: self.operator_id.clone(),
            donnee,
        };
        etat_of(self.send_request(request).await?)
    }

    /// Lecture typée sur la base par défaut.
    pub async fn select(
        &self,
        query: TypedSelect,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>, ClientError> {
        self.select_db(&self.default_database, query).await
    }

    /// Lecture typée sur une base donnée.
    pub async fn select_db(
        &self,
        database: &str,
        query: TypedSelect,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>, ClientError> {
        let request = Request::Select {
            operator_id: self.operator_id.clone(),
            database: database.to_string(),
            query,
        };
        rows_of(self.send_request(request).await?)
    }

    /// Applique les migrations de schéma versionnées (`1..=n`) de la base par défaut.
    ///
    /// Retourne `(version avant, version après)`. Non destructif côté service.
    pub async fn apply_migrations(
        &self,
        migrations: Vec<SchemaMigration>,
    ) -> Result<(u32, u32), ClientError> {
        let request = Request::ApplyMigrations {
            operator_id: self.operator_id.clone(),
            database: self.default_database.clone(),
            migrations,
        };
        match self.send_request(request).await? {
            Response::MigrationResult {
                success: true,
                version_before,
                version_after,
                ..
            } => Ok((version_before, version_after)),
            Response::MigrationResult { error, .. } => Err(ClientError::Service(
                error.unwrap_or_else(|| "Migration failed".to_string()),
            )),
            _ => Err(ClientError::Service("Unexpected response type".to_string())),
        }
    }

    /// Exécute une requête SELECT et retourne les lignes.
    ///
    /// **TRANSITOIRE (lot C)** : SQL brut ; préférer [`Self::select`].
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
                            .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                            .ok_or_else(|| {
                                ClientError::Serialization("Invalid row format".to_string())
                            })
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
    ///
    /// **TRANSITOIRE (lot C)** : SQL brut ; préférer [`Self::submit`].
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
    ///
    /// **TRANSITOIRE (lot C)** : SQL brut ; préférer [`Self::submit_batch`].
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

/// Lignes d'une réponse de lecture.
fn rows_of(response: Response) -> Result<Vec<HashMap<String, serde_json::Value>>, ClientError> {
    match response {
        Response::QueryResult {
            success: true,
            rows,
            ..
        } => rows
            .into_iter()
            .map(|r| serde_json::from_value(r).map_err(ClientError::from))
            .collect(),
        Response::QueryResult { error, .. } => Err(ClientError::Service(
            error.unwrap_or_else(|| "Query failed".to_string()),
        )),
        _ => Err(ClientError::Service("Unexpected response type".to_string())),
    }
}

/// Décode le défi du serveur ; refuse explicitement toute autre version de protocole.
fn parse_challenge(
    frame: &[u8],
) -> Result<([u8; NONCE_LEN], [u8; PUBLIC_KEY_LEN], String), ClientError> {
    let value: serde_json::Value = serde_json::from_slice(frame)?;
    if value.get("type").and_then(serde_json::Value::as_str) == Some("Error") {
        let message = value
            .get("message")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("authentication refused");
        return Err(ClientError::Authentication(message.to_string()));
    }
    let protocol = value.get("protocol").and_then(serde_json::Value::as_str);
    if protocol != Some(AUTH_PROTOCOL) {
        return Err(ClientError::Authentication(format!(
            "Unsupported authentication protocol {protocol:?} (expected {AUTH_PROTOCOL})"
        )));
    }
    let Response::Challenge {
        server_nonce,
        server_pub,
        server_sig,
        ..
    } = serde_json::from_value::<Response>(value)?
    else {
        return Err(ClientError::Authentication(
            "Unexpected challenge".to_string(),
        ));
    };
    let malformed = || ClientError::Authentication("Malformed challenge".to_string());
    let server_nonce = decode_hex_fixed::<NONCE_LEN>(&server_nonce).ok_or_else(malformed)?;
    let server_pub = decode_hex_fixed::<PUBLIC_KEY_LEN>(&server_pub).ok_or_else(malformed)?;
    Ok((server_nonce, server_pub, server_sig))
}

/// Lit une trame `[u32 BE longueur][payload]` bornée à `max` octets.
async fn read_frame(stream: &mut TcpStream, max: usize) -> Result<Vec<u8>, ClientError> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).await?;
    let len = usize::try_from(u32::from_be_bytes(len_buf))
        .map_err(|_| ClientError::Transport("Frame too large".to_string()))?;
    if len > max {
        return Err(ClientError::Transport(format!(
            "Frame too large ({len} > {max} bytes)"
        )));
    }
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    Ok(buf)
}

/// Écrit une trame `[u32 BE longueur][payload]`.
async fn write_frame(stream: &mut TcpStream, payload: &[u8]) -> Result<(), ClientError> {
    let len = u32::try_from(payload.len())
        .map_err(|_| ClientError::InvalidRequest("Request too large".to_string()))?;
    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(payload).await?;
    Ok(())
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

/// État d'étiquette d'une réponse.
fn etat_of(response: Response) -> Result<EtatEtiquette, ClientError> {
    match response {
        Response::EtiquetteResult { success: true, etat: Some(etat), .. } => Ok(*etat),
        Response::EtiquetteResult { error, .. } => Err(ClientError::Service(error.unwrap_or_default())),
        _ => Err(ClientError::Service("Unexpected response type".to_string())),
    }
}
