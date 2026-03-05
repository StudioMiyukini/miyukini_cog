//! Protocole IPC JSON pour KindMother.
//!
//! @id: kindmother_service_protocol
//! @do: define_ipc_protocol
//! @layer: core
//!
//! Protocole simple basé sur JSON avec framing par longueur.
//! Format: [4 bytes length (big endian)][JSON payload]

use serde::{Deserialize, Serialize};

/// Requête vers KindMother Service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(missing_docs)]
pub enum Request {
    /// Requête SELECT.
    Query {
        operator_id: String,
        database: String,
        sql: String,
        params: Vec<String>,
    },

    /// Requête d'écriture (INSERT, UPDATE, DELETE).
    Execute {
        operator_id: String,
        database: String,
        sql: String,
        params: Vec<String>,
        intent: String,
    },

    /// Transaction avec plusieurs opérations.
    Transaction {
        operator_id: String,
        database: String,
        operations: Vec<Operation>,
        intent: String,
    },

    /// Vérification de santé.
    HealthCheck { database: Option<String> },

    /// Informations sur une base de données.
    DatabaseInfo {
        operator_id: String,
        database: String,
    },
}

/// Opération dans une transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct Operation {
    pub sql: String,
    pub params: Vec<String>,
}

/// Réponse de KindMother Service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(missing_docs)]
pub enum Response {
    /// Résultat d'une requête SELECT.
    QueryResult {
        success: bool,
        rows: Vec<serde_json::Value>,
        row_count: u64,
        error: Option<String>,
    },

    /// Résultat d'une requête d'écriture.
    ExecuteResult {
        success: bool,
        rows_affected: u64,
        last_insert_id: Option<i64>,
        error: Option<String>,
    },

    /// Résultat d'une transaction.
    TransactionResult {
        success: bool,
        results: Vec<u64>,
        error: Option<String>,
    },

    /// Résultat de vérification de santé.
    HealthCheckResult {
        healthy: bool,
        status: String,
        latency_ms: u64,
    },

    /// Informations sur une base de données.
    DatabaseInfoResult {
        success: bool,
        database: String,
        size_bytes: u64,
        table_count: u32,
        encrypted: bool,
        error: Option<String>,
    },

    /// Erreur générique.
    Error { code: String, message: String },
}

impl Response {
    /// Crée une réponse d'erreur.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Response::Error {
            code: code.into(),
            message: message.into(),
        }
    }
}
