//! Gestion du lancement automatique de KindMother.
//!
//! Ce module vérifie si le service KindMother est accessible.
//! Si non, il tente de le lancer automatiquement avant que Central ne démarre.

use std::net::TcpStream;
use std::path::PathBuf;
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
    match TcpStream::connect_timeout(
        &addr.parse().expect("Invalid address"),
        Duration::from_millis(CONNECTION_TIMEOUT_MS),
    ) {
        Ok(_) => {
            tracing::debug!("KindMother is reachable at {}", addr);
            true
        }
        Err(_) => {
            tracing::debug!("KindMother is not reachable at {}", addr);
            false
        }
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

/// Lance le service KindMother.
fn launch_kindmother_process(exe_path: &PathBuf) -> Result<Child, String> {
    tracing::info!("Launching KindMother from {:?}", exe_path);
    
    // Configurer les variables d'environnement pour KindMother
    let data_dir = std::env::current_dir()
        .map(|p| p.join("data").join("kindmother"))
        .unwrap_or_else(|_| PathBuf::from("./data/kindmother"));
    
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
        .map_err(|e| format!("Failed to spawn kindmother-server: {}", e))
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
    let exe_path = match find_kindmother_executable() {
        Some(path) => path,
        None => {
            let msg = "Could not find kindmother-server executable. \
                       Please ensure it is built (cargo build -p kindmother-service) \
                       or available in PATH.";
            tracing::error!("{}", msg);
            return LaunchResult::Failed(msg.to_string());
        }
    };
    
    // Lancer le processus
    let child = match launch_kindmother_process(&exe_path) {
        Ok(child) => child,
        Err(e) => {
            tracing::error!("Failed to launch KindMother: {}", e);
            return LaunchResult::Failed(e);
        }
    };
    
    // Attendre que le service soit prêt
    if wait_for_kindmother_ready(&addr, MAX_RETRY_ATTEMPTS) {
        tracing::info!("KindMother launched successfully");
        LaunchResult::Launched(child)
    } else {
        let msg = format!(
            "KindMother was launched but failed to become ready after {} attempts",
            MAX_RETRY_ATTEMPTS
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
