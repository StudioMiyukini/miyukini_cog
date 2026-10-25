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

use kindmother::etiquette::{EtatEtiquette, IntentionEtiquette, RefDonnee};
use kindmother::intent::{IntentOutcome, SchemaMigration, TypedSelect, TypedWriteIntent};
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

    /// Lot d'intentions d'écriture **typées** (lot C, K-13) : aucune instruction SQL.
    ///
    /// Toutes les intentions visent `database` et portent l'opérateur authentifié ;
    /// le lot est appliqué atomiquement (une refusée ou échouée : aucune appliquée).
    SubmitIntents {
        operator_id: String,
        database: String,
        intents: Vec<TypedWriteIntent>,
    },

    /// Lecture **typée** (table, colonnes, égalités, tri, limite).
    Select {
        operator_id: String,
        database: String,
        query: TypedSelect,
    },

    /// Migrations de schéma versionnées (Admin seulement, non destructives).
    ApplyMigrations {
        operator_id: String,
        database: String,
        migrations: Vec<SchemaMigration>,
    },

    /// Pose ou retire l'étiquette `publiable` (EX-47) : intention **signée par une clé
    /// de l'utilisateur** (jamais par l'opérateur qui la relaie), refusée sur une donnée
    /// `nationale`, journalisée dans l'audit chaîné.
    Etiqueter {
        operator_id: String,
        intention: IntentionEtiquette,
    },

    /// Lecture typée de l'étiquette d'une donnée exacte (EX-47).
    LireEtiquette {
        operator_id: String,
        donnee: RefDonnee,
    },

    /// Requête SELECT en SQL brut.
    ///
    /// **TRANSITOIRE (lot C)** : chemin déprécié, gardé par l'autorisateur SQLite ; à
    /// retirer quand tous les services utiliseront `Select`.
    Query {
        operator_id: String,
        database: String,
        sql: String,
        params: Vec<String>,
    },

    /// Requête d'écriture en SQL brut (INSERT, UPDATE, DELETE).
    ///
    /// **TRANSITOIRE (lot C)** : chemin déprécié, gardé par l'autorisateur SQLite ; à
    /// retirer quand tous les services utiliseront `SubmitIntents`.
    Execute {
        operator_id: String,
        database: String,
        sql: String,
        params: Vec<String>,
        intent: String,
    },

    /// Transaction en SQL brut avec plusieurs opérations.
    ///
    /// **TRANSITOIRE (lot C)** : à retirer au profit de `SubmitIntents` (lot atomique).
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

    /// Issue d'un lot d'intentions typées (une issue par intention, dans l'ordre).
    IntentsResult {
        success: bool,
        outcomes: Vec<IntentOutcome>,
        error: Option<String>,
    },

    /// État de l'étiquette `publiable` d'une donnée (après pose / retrait, ou lecture).
    EtiquetteResult {
        success: bool,
        etat: Option<Box<EtatEtiquette>>,
        error: Option<String>,
    },

    /// Résultat de migrations de schéma.
    MigrationResult {
        success: bool,
        version_before: u32,
        version_after: u32,
        backup: Option<String>,
        error: Option<String>,
    },

    /// Résultat d'une requête SELECT (SQL brut ou lecture typée).
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
