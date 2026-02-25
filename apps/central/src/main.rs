//! Miyukini Central — Application Native
//!
//! Hub de gestion des Services COG avec interface Steam-like.
//! Rendu natif via Dioxus Desktop (WGPU).

mod app;
mod audio;
mod components;
mod data;
mod kindmother_launcher;
mod market_client;
mod miou;
mod screens;
mod services;
mod state;
mod theme;

use std::process::Child;
use std::sync::{Mutex, OnceLock};

/// Handle vers le processus KindMother si lancé par Central.
/// Utilise OnceLock + Mutex pour un accès thread-safe sans unsafe.
static KINDMOTHER_PROCESS: OnceLock<Mutex<Option<Child>>> = OnceLock::new();

fn main() {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter("miyukini_central=debug,kindmother_service=info,dioxus=info")
        .init();

    tracing::info!("Démarrage de Miyukini Central...");

    // Initialiser le holder du processus KindMother
    KINDMOTHER_PROCESS.get_or_init(|| Mutex::new(None));

    // === Phase 0: Vérifier les dépendances (kindmother-server) ===
    tracing::info!("Vérification des dépendances...");
    if let Err(e) = kindmother_launcher::ensure_dependencies() {
        tracing::error!("Dépendances manquantes: {}", e);
        show_error_dialog(&e);
        std::process::exit(1);
    }

    // === Phase 1: Vérifier la présence de KindMother (lancer si besoin) ===
    tracing::info!("Vérification de la présence de KindMother...");
    
    match kindmother_launcher::ensure_kindmother_running() {
        kindmother_launcher::LaunchResult::AlreadyRunning => {
            tracing::info!("KindMother était déjà en cours d'exécution.");
        }
        kindmother_launcher::LaunchResult::Launched(child) => {
            tracing::info!("KindMother a été lancé par Central (PID: {:?}).", child.id());
            // Stocker le handle pour garder le processus en vie
            if let Some(lock) = KINDMOTHER_PROCESS.get() {
                if let Ok(mut guard) = lock.lock() {
                    *guard = Some(child);
                }
            }
        }
        kindmother_launcher::LaunchResult::Failed(err) => {
            tracing::error!("Échec du démarrage de KindMother: {}", err);
            show_error_dialog(&err);
            std::process::exit(1);
        }
    }

    // === Phase 2: Lancer l'interface Dioxus ===
    tracing::info!("KindMother OK, démarrage de l'interface...");

    // Configuration de la fenêtre
    let mut config = dioxus::desktop::Config::new();

    // Sur Windows : rediriger WebView2 vers LOCALAPPDATA pour éviter "Accès refusé"
    // quand l'app est installée dans Program Files (dossier non accessible en écriture).
    #[cfg(windows)]
    {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            let webview2_dir = std::path::PathBuf::from(local)
                .join("Miyukini-COG")
                .join("webview2");
            if let Err(e) = std::fs::create_dir_all(&webview2_dir) {
                tracing::warn!("Impossible de créer le dossier WebView2: {}", e);
            } else {
                tracing::debug!("WebView2 user data: {:?}", webview2_dir);
                config = config.with_data_directory(webview2_dir);
            }
        }
    }

    config = config
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("Miyukini Central")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1400.0, 900.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(1024.0, 700.0))
                .with_decorations(true)
                .with_resizable(true),
        )
        .with_disable_context_menu(true);

    // Lancer l'application
    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(app::App);
    
    // === Phase 3: Nettoyage à la fermeture ===
    cleanup_kindmother();
}

/// Affiche une boîte de dialogue d'erreur.
fn show_error_dialog(err: &str) {
    #[cfg(windows)]
    {
        // Utiliser l'API Windows native via windows-sys ou simple message console
        // Pour éviter les problèmes de dépendances, on utilise juste la console
        eprintln!("\n╔══════════════════════════════════════════════════════════════╗");
        eprintln!("║              MIYUKINI CENTRAL - ERREUR CRITIQUE              ║");
        eprintln!("╠══════════════════════════════════════════════════════════════╣");
        eprintln!("║ Dépendances ou KindMother manquants.                         ║");
        eprintln!("║                                                              ║");
        eprintln!("║ {:<60} ║", err.chars().take(60).collect::<String>());
        eprintln!("║                                                              ║");
        eprintln!("║ Central construit kindmother-server si absent (cargo build). ║");
        eprintln!("║ Lancez depuis la racine du dépôt ou placez kindmother-server ║");
        eprintln!("║ dans le même dossier que miyukini-central.exe.                ║");
        eprintln!("╚══════════════════════════════════════════════════════════════╝\n");
    }
    
    #[cfg(not(windows))]
    {
        eprintln!("ERREUR CRITIQUE: Dépendances ou KindMother manquants.");
        eprintln!("{}", err);
        eprintln!("Lancez Central depuis la racine du dépôt pour auto-construction de kindmother-server.");
    }
}

/// Arrête proprement le processus KindMother s'il a été lancé par Central.
fn cleanup_kindmother() {
    if let Some(lock) = KINDMOTHER_PROCESS.get() {
        if let Ok(mut guard) = lock.lock() {
            if let Some(mut child) = guard.take() {
                tracing::info!("Arrêt de KindMother...");
                let _ = child.kill();
                let _ = child.wait();
                tracing::info!("KindMother arrêté.");
            }
        }
    }
}
