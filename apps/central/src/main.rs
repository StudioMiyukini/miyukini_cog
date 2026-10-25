//! @id     central
//! @do     lancer_central
//! @role   orchestration
//! @layer  app
//! @human  Application Miyukini Central : hub de gestion des services COG (interface Steam-like), racine du projet.
//!
//! Miyukini Central — Standalone Launcher
//!
//! Hub de gestion des Services COG avec interface Steam-like.
//! Les services sont des binaires indépendants gérés par le ServiceManager.

mod app;
mod audio;
pub mod canal_officiel;
pub mod cle_service;
mod components;
mod data;
mod kindmother_launcher;
pub mod llm_client;
pub mod market_client;
mod miou;
/// Provenance signée des modèles GGUF : **source de maia-llm** (binaire sans
/// bibliothèque), compilée ici pour l'installateur de modèles délégué du canal
/// officiel ([`canal_officiel::modeles`]) ; analysée par les lints de maia-llm.
#[path = "../../maia-llm/src/provenance.rs"]
#[allow(dead_code, unused_imports, clippy::all, clippy::pedantic)]
mod provenance;
pub mod remote;
mod screens;
pub mod service_manager;
mod services;
mod state;
mod theme;

use std::process::Child;
use std::sync::{Mutex, OnceLock};

/// Handle vers le processus KindMother si lancé par Central.
/// Utilise OnceLock + Mutex pour un accès thread-safe sans unsafe.
static KINDMOTHER_PROCESS: OnceLock<Mutex<Option<Child>>> = OnceLock::new();

/// CA-03 : ramène les fichiers secrets existants (anciennes versions en 0644) en
/// 0600 et les dossiers applicatifs en 0700. Le répertoire courant n'est jamais
/// modifié (seuls les dossiers nommés `Miyukini-COG` sont durcis).
fn durcir_fichiers_au_demarrage() {
    use miyukini_central::chemins::{dossier_config, dossier_donnees, DOSSIER_APP, NOM_BASE_AUTH};
    use miyukini_central::fichiers_secrets::{creer_dossier_prive, durcir_fichier};

    let est_dossier_app = |d: &std::path::Path| d.file_name().is_some_and(|n| n == DOSSIER_APP);
    if let Ok(conf) = dossier_config() {
        if est_dossier_app(&conf) && conf.is_dir() {
            if let Err(e) = creer_dossier_prive(&conf) {
                tracing::warn!("Permissions de {} non corrigées : {e}", conf.display());
            }
        }
        for nom in ["mws-config.json", "miyucloud-config.json"] {
            if let Err(e) = durcir_fichier(&conf.join(nom)) {
                tracing::warn!("Permissions de {nom} non corrigées : {e}");
            }
        }
    }
    if let Ok(donnees) = dossier_donnees() {
        if est_dossier_app(&donnees) && donnees.is_dir() {
            if let Err(e) = creer_dossier_prive(&donnees) {
                tracing::warn!("Permissions de {} non corrigées : {e}", donnees.display());
            }
        }
        if let Err(e) = durcir_fichier(&donnees.join(NOM_BASE_AUTH)) {
            tracing::warn!("Permissions de {NOM_BASE_AUTH} non corrigées : {e}");
        }
    }
}

fn main() {
    // Initialiser le logging
    tracing_subscriber::fmt()
        .with_env_filter("miyukini_central=debug,kindmother_service=info,dioxus=info")
        .init();

    tracing::info!("Démarrage de Miyukini Central...");

    // CA-03 : secrets existants ramenés en 0600, dossiers applicatifs en 0700.
    durcir_fichiers_au_demarrage();

    // Lot 2 : clé de service de Central (appels signés à MAIA) — clé publique journalisée.
    cle_service::initialiser();

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
            tracing::info!(
                "KindMother a été lancé par Central (PID: {:?}).",
                child.id()
            );
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
        eprintln!(
            "Lancez Central depuis la racine du dépôt pour auto-construction de kindmother-server."
        );
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
