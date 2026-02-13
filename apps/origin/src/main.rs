//! Miyukini Webway System Origin
//!
//! Point central de vérité du MWS : relay + tracker + registre + site web.
//!
//! # Usage
//!
//! ```bash
//! miyukini-origin --config /etc/miyukini/origin.toml
//! ```

#![allow(missing_docs)]

mod admin;
mod config;
mod protocol;
mod relay;
mod tracker;
mod web;

use std::path::PathBuf;
use std::sync::Arc;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use admin::AdminServer;
use config::OriginConfig;
use relay::RelayServer;
use tracker::TrackerServer;
use web::WebServer;

/// Arguments de la ligne de commande.
struct Args {
    /// Chemin du fichier de configuration.
    config_path: PathBuf,
}

impl Args {
    fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        let mut config_path = PathBuf::from("/etc/miyukini/origin.toml");

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--config" | "-c" => {
                    i += 1;
                    if i < args.len() {
                        config_path = PathBuf::from(&args[i]);
                    }
                }
                "--help" | "-h" => {
                    eprintln!("Usage: miyukini-origin [OPTIONS]");
                    eprintln!();
                    eprintln!("Options:");
                    eprintln!("  -c, --config <PATH>  Chemin du fichier de configuration");
                    eprintln!("                       (défaut: /etc/miyukini/origin.toml)");
                    eprintln!("  -h, --help           Affiche cette aide");
                    eprintln!("  -V, --version        Affiche la version");
                    std::process::exit(0);
                }
                "--version" | "-V" => {
                    eprintln!("miyukini-origin {}", env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("Argument inconnu: {}", args[i]);
                    eprintln!("Utilisez --help pour l'aide");
                    std::process::exit(1);
                }
            }
            i += 1;
        }

        Self { config_path }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parser les arguments
    let args = Args::parse();

    // Initialiser le logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Banner
    info!("═══════════════════════════════════════════════════════════");
    info!("  Miyukini Webway System — Origin");
    info!("  Version: {}", env!("CARGO_PKG_VERSION"));
    info!("═══════════════════════════════════════════════════════════");

    // Charger la configuration
    info!("Loading configuration from: {}", args.config_path.display());

    let config = if args.config_path.exists() {
        match OriginConfig::load(&args.config_path) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to load config: {}", e);
                error!("Using default configuration");
                OriginConfig::default()
            }
        }
    } else {
        info!("Config file not found, using defaults");
        OriginConfig::default()
    };

    let config = Arc::new(config);

    info!("Configuration loaded:");
    info!("  Role: {}", config.identity.role);
    info!("  IP: {}", config.identity.ip);
    info!("  Relay port: {}", config.relay.port);
    info!("  Tracker port: {}", config.tracker.port);
    info!("  Admin port: {}", config.admin.port);

    // Créer les serveurs
    info!("Starting servers...");

    // Créer le tracker (pour partager le pool_manager)
    let tracker = TrackerServer::new(Arc::clone(&config));
    let pool_manager = tracker.pool_manager();

    // Créer le serveur web
    let web_server = WebServer::new(Arc::clone(&config), Arc::clone(&pool_manager));

    // Créer le serveur admin
    let admin = AdminServer::new(Arc::clone(&config))
        .with_pools(Arc::clone(&pool_manager));

    // Créer le serveur relay
    let relay_result = RelayServer::new(Arc::clone(&config)).await;

    // Démarrer les serveurs en parallèle
    let tracker_handle = {
        let tracker = tracker;
        tokio::spawn(async move {
            if let Err(e) = tracker.run().await {
                error!("Tracker server error: {}", e);
            }
        })
    };

    let web_handle = {
        let web = web_server;
        tokio::spawn(async move {
            if let Err(e) = web.run().await {
                error!("Web server error: {}", e);
            }
        })
    };

    let admin_handle = {
        let admin = admin;
        tokio::spawn(async move {
            if let Err(e) = admin.run().await {
                error!("Admin server error: {}", e);
            }
        })
    };

    let relay_handle = tokio::spawn(async move {
        match relay_result {
            Ok(relay) => {
                if let Err(e) = relay.run().await {
                    error!("Relay server error: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to create relay server: {}", e);
                error!("Relay server disabled (TLS configuration issue?)");
                // Continuer sans le relay - le tracker et admin fonctionnent
            }
        }
        // Keep the task alive if relay failed to start
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        }
    });

    info!("═══════════════════════════════════════════════════════════");
    info!("  Origin is running!");
    info!("  Relay:   {}:{}", config.relay.host, config.relay.port);
    info!("  Tracker: {}:{}", config.tracker.host, config.tracker.port);
    info!("  Website: http://{}:{}", config.tracker.host, config.tracker.web_port);
    info!("  Admin:   {}:{}", config.admin.host, config.admin.port);
    info!("═══════════════════════════════════════════════════════════");

    // Attendre la fin (Ctrl+C)
    tokio::select! {
        _ = tracker_handle => {
            error!("Tracker stopped unexpectedly");
        }
        _ = web_handle => {
            error!("Web server stopped unexpectedly");
        }
        _ = admin_handle => {
            error!("Admin stopped unexpectedly");
        }
        _ = relay_handle => {
            error!("Relay stopped unexpectedly");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down...");
        }
    }

    info!("Origin shutdown complete");
    Ok(())
}
