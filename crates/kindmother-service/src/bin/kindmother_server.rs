//! Point d'entrée du serveur KindMother.
//!
//! Lance le service KindMother qui est le seul interlocuteur autorisé
//! pour accéder aux bases de données.

use kindmother_service::{KindMotherServer, ServiceError};
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Configuration du serveur.
struct Config {
    /// Répertoire de données
    data_dir: PathBuf,
    /// Adresse d'écoute
    listen_addr: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./data/kindmother"),
            listen_addr: "127.0.0.1:50051".to_string(),
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
    let server = KindMotherServer::new(&config.data_dir).await?;

    tracing::info!("KindMother server ready, serving requests...");

    server.serve(&config.listen_addr).await?;

    Ok(())
}
