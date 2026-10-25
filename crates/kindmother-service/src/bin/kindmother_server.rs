//! @id     kindmother_service_bin_kindmother_server
//! @do     definir_config
//! @role   orchestration
//! @layer  core
//! @human  kindmother-service — module « bin kindmother server ».

//! Point d'entrée du serveur KindMother.
//!
//! Lance le service KindMother qui est le seul interlocuteur autorisé
//! pour accéder aux bases de données.

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

use kindmother_service::{KindMotherServer, ServiceError};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Configuration du serveur.
struct Config {
    /// Répertoire de données
    data_dir: PathBuf,
    /// Adresse d'écoute
    listen_addr: String,
    /// Écoute hors boucle locale autorisée explicitement (`KINDMOTHER_ALLOW_NON_LOOPBACK=1`)
    allow_non_loopback: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./data/kindmother"),
            listen_addr: "127.0.0.1:50051".to_string(),
            allow_non_loopback: false,
        }
    }
}

impl Config {
    /// Charge la configuration depuis les variables d'environnement.
    fn from_env() -> Self {
        Self {
            data_dir: std::env::var("KINDMOTHER_DATA_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("./data/kindmother")),
            listen_addr: std::env::var("KINDMOTHER_LISTEN_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:50051".to_string()),
            allow_non_loopback: std::env::var("KINDMOTHER_ALLOW_NON_LOOPBACK")
                .is_ok_and(|v| v == "1"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    // Initialiser le logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "kindmother_service=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Charger la configuration
    let config = Config::from_env();

    tracing::info!(
        "Starting KindMother server with data_dir={:?}, listen_addr={}",
        config.data_dir,
        config.listen_addr
    );

    // Créer le répertoire de données si nécessaire
    std::fs::create_dir_all(&config.data_dir)
        .map_err(|e| ServiceError::Internal(format!("Cannot create data dir: {}", e)))?;

    // Créer et lancer le serveur
    let server = KindMotherServer::new(&config.data_dir)
        .await?
        .with_non_loopback_allowed(config.allow_non_loopback);

    // Chiffrement (K-06) : sans db-encryption, `KindMotherServer::new` a déjà refusé de
    // démarrer, sauf build de debug avec KINDMOTHER_DEV_PLAINTEXT=1.
    if server.database().is_encrypted() {
        tracing::info!("Storage: SQLCipher, one key per database (K-06, K-07)");
    } else {
        tracing::error!(
            "Storage: PLAINTEXT (development mode, {}=1) — never use in production",
            kindmother_service::ENV_DEV_PLAINTEXT
        );
    }

    tracing::info!(
        "KindMother server ready (operator credentials in {:?}), serving requests...",
        server.credentials_dir()
    );

    server.serve(&config.listen_addr).await?;

    Ok(())
}
