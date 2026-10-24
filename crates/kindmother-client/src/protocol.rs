//! Protocole IPC JSON pour KindMother Client.
//!
//! @id     kindmother_client_protocol
//! @do     define_client_protocol
//@human ! @layer  toolkit
//!
//! Séquence (K-04, protocole `kindmother-auth-v2`, authentification mutuelle) :
//! 1. le client envoie `ClientHello { protocol, client_nonce }` ;
//! 2. le serveur répond `Challenge { protocol, server_nonce, server_pub, server_sig }` ;
//!    le client vérifie `server_pub` contre la clé épinglée et `server_sig` ;
//! 3. le client envoie `Authenticate { operator_id, signature }` ;
//! 4. le serveur confirme `Authenticated` ou refuse et ferme.
//!
//! Toute autre version de protocole (dont `kindmother-auth-v1`) est refusée. Les champs
//! `operator_id` des requêtes suivantes doivent égaler l'identité authentifiée.

use serde::{Deserialize, Serialize};

/// Requête vers KindMother Service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[allow(missing_docs)]
pub enum Request {
    /// Ouverture de la poignée de main (première trame envoyée par le client).
    ///
    /// `client_nonce` : 32 octets aléatoires (hex).
    ClientHello {
        protocol: String,
        client_nonce: String,
    },

    /// Réponse au défi (seconde trame envoyée par le client).
    ///
    /// `signature` : signature Ed25519 (hex, 64 octets) de la transcription client
    /// `"kindmother-auth-v2\0" || "client\0" || client_nonce || server_nonce || server_pub || operator_id`.
    Authenticate {
        operator_id: String,
        signature: String,
    },

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
    /// Défi d'authentification signé par le serveur (réponse au `ClientHello`).
    ///
    /// `server_sig` : signature Ed25519 (hex) de la transcription serveur
    /// `"kindmother-auth-v2\0" || "server\0" || client_nonce || server_nonce || server_pub`.
    Challenge {
        protocol: String,
        server_nonce: String,
        server_pub: String,
        server_sig: String,
    },

    /// Authentification réussie : l'identité est liée à la connexion.
    Authenticated { operator_id: String },

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
