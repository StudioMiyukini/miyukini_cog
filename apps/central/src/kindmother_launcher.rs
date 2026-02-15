//! Gestion du lancement automatique de KindMother.
//!
//! Ordre au démarrage de Central :
//! 1. **Dépendances** : vérifier que les dépendances sont présentes ;
//!    - `kindmother-server` (exécutable) : si absent, construire `kindmother-service` ;
//!    - `miyuwebway_participant` (bibliothèque MWS) : si workspace dispo, construire le crate.
//! 2. **Présence KM** : vérifier que le service KindMother est accessible (déjà en cours) ;
//!    si non, le lancer puis attendre qu'il soit prêt.

use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use std::thread;

/// Adresse par défaut de KindMother.
const KINDMOTHER_DEFAULT_ADDR: &str = "127.0.0.1:50051";

/// Timeout pour la vérification de connexion (ms).
const CONNECTION_TIMEOUT_MS: u64 = 500;

/// Nombre maximum de tentatives de connexion après lancement.
const MAX_RETRY_ATTEMPTS: u32 = 10;

/// Délai entre les tentatives (ms).
const RETRY_DELAY_MS: u64 = 500;

/// Nom du package Cargo pour le serveur KindMother.
const KINDMOTHER_SERVICE_PACKAGE: &str = "kindmother-service";

/// Nom du package Cargo pour le client MWS (bibliothèque, utilisée par Central).
const MIYUWEBWAY_PARTICIPANT_PACKAGE: &str = "miyuwebway_participant";

/// Résultat du lancement de KindMother.
#[derive(Debug)]
pub enum LaunchResult {
    /// KindMother était déjà en cours d'exécution.
    AlreadyRunning,
    /// KindMother a été lancé avec succès.
    Launched(Child),
    /// Échec du lancement.
    Failed(String),
}

/// Vérifie si KindMother est accessible sur l'adresse donnée.
pub fn is_kindmother_running(addr: &str) -> bool {
    if TcpStream::connect_timeout(
        &addr.parse().expect("Invalid address"),
        Duration::from_millis(CONNECTION_TIMEOUT_MS),
    )
    .is_ok()
    {
        tracing::debug!("KindMother is reachable at {addr}");
        true
    } else {
        tracing::debug!("KindMother is not reachable at {addr}");
        false
    }
}

/// Indique si un répertoire est la racine du workspace (contient Cargo.toml avec [workspace]).
fn is_workspace_root(dir: &Path) -> bool {
    let cargo_toml = dir.join("Cargo.toml");
    if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
        return content.contains("[workspace]");
    }
    false
}

/// Trouve la racine du workspace (répertoire contenant Cargo.toml avec [workspace]).
fn find_workspace_root() -> Option<PathBuf> {
    // 1. Depuis le répertoire courant (cas "cargo run" depuis la racine)
    if let Ok(cwd) = std::env::current_dir() {
        let mut cur = cwd.as_path();
        for _ in 0..10 {
            if is_workspace_root(cur) {
                tracing::debug!("Workspace root (from cwd): {:?}", cur);
                return Some(cur.to_path_buf());
            }
            cur = match cur.parent() {
                Some(p) => p,
                None => break,
            };
        }
    }

    // 2. Depuis l'exécutable (ex: .../Miyukini_COG/target/debug/miyukini-central.exe)
    if let Ok(exe) = std::env::current_exe() {
        let mut cur = exe.parent();
        // Remonter target/debug ou target/release
        while let Some(p) = cur {
            if let Some(name) = p.file_name() {
                let name = name.to_string_lossy();
                if name == "debug" || name == "release" {
                    if let Some(target) = p.parent() {
                        if target.file_name().is_some_and(|n| n == "target") {
                            if let Some(ws) = target.parent() {
                                if is_workspace_root(ws) {
                                    tracing::debug!("Workspace root (from exe): {:?}", ws);
                                    return Some(ws.to_path_buf());
                                }
                            }
                        }
                    }
                }
            }
            cur = p.parent();
        }
    }

    None
}

/// Construit un ou plusieurs packages Cargo (depuis la racine du workspace).
fn build_packages(workspace_root: &Path, release: bool, packages: &[&str]) -> Result<(), String> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(workspace_root).arg("build");
    for p in packages {
        cmd.arg("-p").arg(p);
    }
    if release {
        cmd.arg("--release");
    }
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());

    let profile = if release { "release" } else { "debug" };
    tracing::info!(
        "Construction de {} (profile: {})...",
        packages.join(", "),
        profile
    );

    let output = cmd
        .output()
        .map_err(|e| format!("Impossible de lancer cargo: {e}"))?;

    if output.status.success() {
        tracing::info!("Packages construits avec succès.");
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "cargo build -p {} a échoué:\n{}",
            packages.join(" -p "),
            stderr.trim()
        ))
    }
}

/// Assure que les dépendances sont présentes (kindmother-server, miyuwebway_participant).
///
/// - **kindmother-server** (exécutable) : si absent, construction de `kindmother-service` et de
///   `miyuwebway_participant` (bibliothèque MWS utilisée par Central).
/// - **miyuwebway_participant** (bibliothèque) : construite en même temps que kindmother-service
///   lorsque l'exécutable est manquant, pour garder le workspace cohérent.
///
/// # Returns
/// - `Ok(())` si l'exécutable est présent ou a été construit
/// - `Err(msg)` si absent et construction impossible ou échouée
pub fn ensure_dependencies() -> Result<(), String> {
    if find_kindmother_executable().is_some() {
        tracing::info!("Dépendances (kindmother-server, miyuwebway_participant) déjà présentes.");
        return Ok(());
    }

    tracing::info!(
        "kindmother-server introuvable, construction de {} et {}...",
        MIYUWEBWAY_PARTICIPANT_PACKAGE,
        KINDMOTHER_SERVICE_PACKAGE
    );

    let workspace_root = find_workspace_root().ok_or_else(|| {
        "Racine du workspace introuvable (pas de Cargo.toml [workspace] depuis le répertoire courant ou le chemin de l'exécutable). \
         Lancez Central depuis la racine du dépôt ou placez kindmother-server dans le même dossier que l'exécutable."
            .to_string()
    })?;

    // Utiliser le même profile que Central (release si exe dans target/release)
    let release = std::env::current_exe()
        .ok()
        .is_some_and(|exe_path| {
            exe_path
                .parent()
                .and_then(std::path::Path::file_name)
                .is_some_and(|n| n == "release")
        });

    build_packages(
        &workspace_root,
        release,
        &[MIYUWEBWAY_PARTICIPANT_PACKAGE, KINDMOTHER_SERVICE_PACKAGE],
    )?;

    if find_kindmother_executable().is_some() {
        Ok(())
    } else {
        Err(format!(
            "{} a été construit mais l'exécutable kindmother-server est toujours introuvable. \
             Vérifiez target/{}.",
            KINDMOTHER_SERVICE_PACKAGE,
            if release { "release" } else { "debug" }
        ))
    }
}

/// Trouve le chemin vers l'exécutable kindmother-server.
fn find_kindmother_executable() -> Option<PathBuf> {
    // 1. Vérifier dans le même répertoire que l'exécutable Central
    if let Ok(exe_path) = std::env::current_exe() {
        let exe_dir = exe_path.parent()?;
        
        // Windows: kindmother-server.exe, Unix: kindmother-server
        let km_name = if cfg!(windows) {
            "kindmother-server.exe"
        } else {
            "kindmother-server"
        };
        
        let km_path = exe_dir.join(km_name);
        if km_path.exists() {
            tracing::info!("Found kindmother-server at {:?}", km_path);
            return Some(km_path);
        }
    }
    
    // 2. Vérifier dans target/debug ou target/release (développement)
    if let Ok(cwd) = std::env::current_dir() {
        for profile in &["debug", "release"] {
            let km_name = if cfg!(windows) {
                "kindmother-server.exe"
            } else {
                "kindmother-server"
            };
            
            let km_path = cwd.join("target").join(profile).join(km_name);
            if km_path.exists() {
                tracing::info!("Found kindmother-server at {:?}", km_path);
                return Some(km_path);
            }
        }
    }
    
    // 3. Vérifier dans le PATH
    if let Ok(output) = Command::new(if cfg!(windows) { "where" } else { "which" })
        .arg("kindmother-server")
        .output()
    {
        if output.status.success() {
            let path_str = String::from_utf8_lossy(&output.stdout);
            let path = PathBuf::from(path_str.trim());
            if path.exists() {
                tracing::info!("Found kindmother-server in PATH at {:?}", path);
                return Some(path);
            }
        }
    }
    
    None
}

/// Détecte si l'exécutable est dans un répertoire système (ex: Program Files) où l'utilisateur
/// ne peut pas écrire. Dans ce cas, on utilise %LOCALAPPDATA% pour les données.
fn is_installed_in_system_dir() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|parent| parent.to_string_lossy().into_owned()))
        .map_or(false, |path| {
            path.contains("Program Files") || path.contains("Program Files (x86)")
        })
}

/// Retourne le répertoire de données KindMother (toujours inscriptible par l'utilisateur).
fn kindmother_data_dir() -> PathBuf {
    if cfg!(windows) && is_installed_in_system_dir() {
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            let data_dir = PathBuf::from(local).join("Miyukini-COG").join("data").join("kindmother");
            tracing::info!("Installed app: using LOCALAPPDATA for KindMother data: {:?}", data_dir);
            return data_dir;
        }
    }

    std::env::current_dir()
        .map_or_else(|_| PathBuf::from("./data/kindmother"), |p| p.join("data").join("kindmother"))
}

/// Lance le service KindMother.
fn launch_kindmother_process(exe_path: &PathBuf) -> Result<Child, String> {
    tracing::info!("Launching KindMother from {:?}", exe_path);

    let data_dir = kindmother_data_dir();

    // Créer le répertoire de données si nécessaire
    if let Err(e) = std::fs::create_dir_all(&data_dir) {
        tracing::warn!("Could not create data directory: {}", e);
    }
    
    Command::new(exe_path)
        .env("KINDMOTHER_DATA_DIR", data_dir.to_string_lossy().to_string())
        .env("KINDMOTHER_LISTEN_ADDR", KINDMOTHER_DEFAULT_ADDR)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn kindmother-server: {e}"))
}

/// Attend que KindMother soit prêt à accepter des connexions.
fn wait_for_kindmother_ready(addr: &str, max_attempts: u32) -> bool {
    for attempt in 1..=max_attempts {
        tracing::debug!(
            "Waiting for KindMother to be ready (attempt {}/{})",
            attempt,
            max_attempts
        );
        
        if is_kindmother_running(addr) {
            tracing::info!("KindMother is now ready after {} attempts", attempt);
            return true;
        }
        
        thread::sleep(Duration::from_millis(RETRY_DELAY_MS));
    }
    
    false
}

/// Assure que KindMother est en cours d'exécution.
///
/// Vérifie d'abord si le service est accessible. Si non, tente de le lancer.
///
/// # Returns
/// - `LaunchResult::AlreadyRunning` si KM était déjà actif
/// - `LaunchResult::Launched(Child)` si KM a été lancé (le Child doit être gardé en vie)
/// - `LaunchResult::Failed(String)` si le lancement a échoué
pub fn ensure_kindmother_running() -> LaunchResult {
    let addr = std::env::var("KINDMOTHER_LISTEN_ADDR")
        .unwrap_or_else(|_| KINDMOTHER_DEFAULT_ADDR.to_string());
    
    tracing::info!("Checking if KindMother is running at {}", addr);
    
    // Vérifier si déjà en cours d'exécution
    if is_kindmother_running(&addr) {
        tracing::info!("KindMother is already running at {}", addr);
        return LaunchResult::AlreadyRunning;
    }
    
    // Trouver l'exécutable
    let Some(exe_path) = find_kindmother_executable() else {
        let msg = "Could not find kindmother-server executable. \
                   Please ensure it is built (cargo build -p kindmother-service) \
                   or available in PATH.";
        tracing::error!("{msg}");
        return LaunchResult::Failed(msg.to_string());
    };

    // Lancer le processus
    let child = match launch_kindmother_process(&exe_path) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to launch KindMother: {e}");
            return LaunchResult::Failed(e);
        }
    };
    
    // Attendre que le service soit prêt
    if wait_for_kindmother_ready(&addr, MAX_RETRY_ATTEMPTS) {
        tracing::info!("KindMother launched successfully");
        LaunchResult::Launched(child)
    } else {
        let msg = format!(
            "KindMother was launched but failed to become ready after {MAX_RETRY_ATTEMPTS} attempts"
        );
        tracing::error!("{}", msg);
        LaunchResult::Failed(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_kindmother_running_when_not_running() {
        // Sur un port probablement non utilisé
        assert!(!is_kindmother_running("127.0.0.1:59999"));
    }
}
