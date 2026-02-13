//! Miyukini Central — Application Native
//!
//! Hub de gestion des Services COG avec interface Steam-like.
//! Rendu natif via Dioxus Desktop (WGPU).

mod app;
mod audio;
mod components;
mod data;
mod kindmother_launcher;
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

    // === Phase 1: Assurer que KindMother est disponible ===
    tracing::info!("Vérification de KindMother...");
    
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
    let config = dioxus::desktop::Config::new()
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
        eprintln!("║ Impossible de démarrer KindMother.                           ║");
        eprintln!("║                                                              ║");
        eprintln!("║ {:<60} ║", err.chars().take(60).collect::<String>());
        eprintln!("║                                                              ║");
        eprintln!("║ Vérifiez que kindmother-server.exe est présent dans le même  ║");
        eprintln!("║ dossier que miyukini-central.exe ou dans target/debug/.      ║");
        eprintln!("╚══════════════════════════════════════════════════════════════╝\n");
    }
    
    #[cfg(not(windows))]
    {
        eprintln!("ERREUR CRITIQUE: Impossible de démarrer KindMother.");
        eprintln!("{}", err);
        eprintln!("Vérifiez que kindmother-server est présent dans le même dossier ou dans target/debug/.");
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
