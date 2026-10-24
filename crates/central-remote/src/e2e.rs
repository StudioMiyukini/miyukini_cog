//! @id     central_remote_e2e
//! @do     gerer_identite_et_sessions_e2e_cog
//! @role   security
//! @layer  toolkit
//! @human  central-remote — identité E2E longue durée du COG et sessions E2E v2.

//! Côté COG du chiffrement de bout en bout mobile ↔ COG (protocole v2 du
//! crate `miyukini-cog-bridge`, R-019 / R-023).
//!
//! - [`load_or_create_cog_identity`] : identité ed25519 longue durée du COG,
//!   générée une fois et stockée dans un fichier `0600` (Unix). Un fichier aux
//!   permissions trop larges, mal formé ou trop grand est refusé (C-12, C-13).
//! - [`E2eSessionStore`] : sessions établies par `/api/bridge/e2e-handshake`,
//!   indexées par identifiant public de session, bornées en nombre et en durée,
//!   une seule session active par appareil.
//!
//! Aucune primitive cryptographique n'est réimplémentée ici (C-24) : tout passe
//! par `miyukini_cog_bridge::e2e`.

use miyukini_cog_bridge::E2eIdentity;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

pub use miyukini_cog_bridge::E2eSession;

/// Nom par défaut du fichier d'identité E2E du COG (à côté de la base auth).
pub const DEFAULT_IDENTITY_FILE: &str = "central-remote-e2e-identity.key";

/// Taille maximale du fichier d'identité (64 hex + fin de ligne, C-13).
const MAX_IDENTITY_FILE_LEN: u64 = 128;

/// Nombre maximal de sessions E2E simultanées (C-13).
pub const MAX_E2E_SESSIONS: usize = 256;

/// Durée de vie maximale d'une session E2E (nouveau handshake ensuite).
pub const E2E_SESSION_TTL: Duration = Duration::from_secs(3600);

/// Erreurs du stockage de l'identité E2E du COG (une variante par cause, C-11).
#[derive(Debug, thiserror::Error)]
pub enum IdentityStoreError {
    #[error("identité E2E : lecture/écriture impossible ({0})")]
    Io(String),
    #[error("identité E2E : permissions trop larges (attendu 0600)")]
    InsecurePermissions,
    #[error("identité E2E : fichier mal formé")]
    Malformed,
    #[error("identité E2E : fichier trop grand")]
    TooLarge,
}

/// Charge l'identité E2E du COG depuis `path`, ou la crée (fichier `0600`)
/// si elle n'existe pas encore. Fail-closed : toute anomalie est une erreur.
pub fn load_or_create_cog_identity(path: &Path) -> Result<E2eIdentity, IdentityStoreError> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => load_identity(path),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => create_identity(path),
        Err(e) => Err(IdentityStoreError::Io(e.kind().to_string())),
    }
}

fn load_identity(path: &Path) -> Result<E2eIdentity, IdentityStoreError> {
    let meta =
        std::fs::symlink_metadata(path).map_err(|e| IdentityStoreError::Io(e.kind().to_string()))?;
    if !meta.file_type().is_file() {
        return Err(IdentityStoreError::Malformed);
    }
    if meta.len() > MAX_IDENTITY_FILE_LEN {
        return Err(IdentityStoreError::TooLarge);
    }
    check_permissions(&meta)?;

    let mut file =
        std::fs::File::open(path).map_err(|e| IdentityStoreError::Io(e.kind().to_string()))?;
    let mut content = Zeroizing::new(String::new());
    Read::take(&mut file, MAX_IDENTITY_FILE_LEN + 1)
        .read_to_string(&mut content)
        .map_err(|_| IdentityStoreError::Malformed)?;
    let hex_seed = content.trim();
    if hex_seed.len() != 64 {
        return Err(IdentityStoreError::Malformed);
    }
    let mut seed = Zeroizing::new([0u8; 32]);
    hex::decode_to_slice(hex_seed, seed.as_mut()).map_err(|_| IdentityStoreError::Malformed)?;
    Ok(E2eIdentity::from_secret_bytes(&seed))
}

fn create_identity(path: &Path) -> Result<E2eIdentity, IdentityStoreError> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent).map_err(|e| IdentityStoreError::Io(e.kind().to_string()))?;
    }
    let identity = E2eIdentity::generate();
    let seed = identity.secret_bytes();
    let mut content = Zeroizing::new(hex::encode(seed.as_ref()));
    content.push('\n');

    let mut options = std::fs::OpenOptions::new();
    // create_new : jamais d'écrasement d'une identité existante.
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|e| IdentityStoreError::Io(e.kind().to_string()))?;
    file.write_all(content.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|e| IdentityStoreError::Io(e.kind().to_string()))?;
    Ok(identity)
}

#[cfg(unix)]
fn check_permissions(meta: &std::fs::Metadata) -> Result<(), IdentityStoreError> {
    use std::os::unix::fs::PermissionsExt;
    if meta.permissions().mode() & 0o077 != 0 {
        return Err(IdentityStoreError::InsecurePermissions);
    }
    Ok(())
}

#[cfg(not(unix))]
fn check_permissions(_meta: &std::fs::Metadata) -> Result<(), IdentityStoreError> {
    // Hors Unix : le fichier vit dans le profil utilisateur (ACL héritées).
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// Sessions
// ═══════════════════════════════════════════════════════════════════════════

/// Session E2E établie côté COG, liée à l'appareil appairé qui l'a ouverte.
pub struct ServerE2eSession {
    /// Session de chiffrement (rôle COG).
    pub session: E2eSession,
    /// Identifiant de l'appareil appairé (whitelist) qui a ouvert la session.
    pub device_id: String,
    /// Clé d'identité E2E (hex) de l'appareil au moment du handshake.
    pub device_key_hex: String,
    created_at: Instant,
}

/// Registre borné des sessions E2E côté COG.
pub struct E2eSessionStore {
    sessions: Mutex<HashMap<[u8; 32], Arc<ServerE2eSession>>>,
    max: usize,
    ttl: Duration,
}

impl Default for E2eSessionStore {
    fn default() -> Self {
        Self::new(MAX_E2E_SESSIONS, E2E_SESSION_TTL)
    }
}

impl E2eSessionStore {
    /// Registre avec capacité et durée de vie données.
    pub fn new(max: usize, ttl: Duration) -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            max: max.max(1),
            ttl,
        }
    }

    /// Enregistre une session : remplace toute session précédente du même
    /// appareil, purge les sessions expirées, évince la plus ancienne si plein.
    /// Retourne false si le verrou est empoisonné (session non enregistrée).
    pub fn insert(&self, device_id: &str, device_key_hex: &str, session: E2eSession) -> bool {
        let Ok(mut map) = self.sessions.lock() else { return false };
        let now = Instant::now();
        let ttl = self.ttl;
        map.retain(|_, s| s.device_id != device_id && now.duration_since(s.created_at) < ttl);
        while map.len() >= self.max {
            let oldest = map
                .iter()
                .min_by_key(|(_, s)| s.created_at)
                .map(|(k, _)| *k);
            match oldest {
                Some(k) => {
                    map.remove(&k);
                }
                None => break,
            }
        }
        map.insert(
            session.session_id(),
            Arc::new(ServerE2eSession {
                session,
                device_id: device_id.to_string(),
                device_key_hex: device_key_hex.to_string(),
                created_at: now,
            }),
        );
        true
    }

    /// Retrouve une session active par son identifiant public (hex 32 octets).
    /// Une session expirée est supprimée et non retournée.
    pub fn get_hex(&self, session_id_hex: &str) -> Option<Arc<ServerE2eSession>> {
        if session_id_hex.len() != 64 {
            return None;
        }
        let mut id = [0u8; 32];
        hex::decode_to_slice(session_id_hex, &mut id).ok()?;
        let mut map = self.sessions.lock().ok()?;
        let expired = map
            .get(&id)
            .is_some_and(|s| s.created_at.elapsed() >= self.ttl);
        if expired {
            map.remove(&id);
            return None;
        }
        map.get(&id).cloned()
    }

    /// Supprime les sessions d'un appareil (révocation).
    pub fn remove_device(&self, device_id: &str) {
        if let Ok(mut map) = self.sessions.lock() {
            map.retain(|_, s| s.device_id != device_id);
        }
    }

    /// Nombre de sessions enregistrées.
    pub fn len(&self) -> usize {
        self.sessions.lock().map_or(0, |m| m.len())
    }

    /// Vrai si aucune session n'est enregistrée.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyukini_cog_bridge::{E2eRole, PendingHandshake};

    fn tmp_path(name: &str) -> std::path::PathBuf {
        std::env::temp_dir()
            .join(format!("central-remote-e2e-{}", uuid::Uuid::new_v4()))
            .join(name)
    }

    fn session_pair() -> E2eSession {
        let device = E2eIdentity::generate();
        let cog = E2eIdentity::generate();
        let (d, d_offer) =
            PendingHandshake::initiate(E2eRole::Device, &device, &cog.public_key_bytes()).unwrap();
        let (c, c_offer) =
            PendingHandshake::initiate(E2eRole::Cog, &cog, &device.public_key_bytes()).unwrap();
        let _ = d.complete(&c_offer).unwrap();
        c.complete(&d_offer).unwrap()
    }

    #[test]
    fn identity_created_once_and_reloaded() {
        let path = tmp_path("id.key");
        let a = load_or_create_cog_identity(&path).unwrap();
        let b = load_or_create_cog_identity(&path).unwrap();
        assert_eq!(a.public_key_bytes(), b.public_key_bytes());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
    }

    #[cfg(unix)]
    #[test]
    fn identity_with_loose_permissions_refused() {
        use std::os::unix::fs::PermissionsExt;
        let path = tmp_path("id.key");
        load_or_create_cog_identity(&path).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(matches!(
            load_or_create_cog_identity(&path),
            Err(IdentityStoreError::InsecurePermissions)
        ));
    }

    #[test]
    fn malformed_or_oversized_identity_refused() {
        let path = tmp_path("id.key");
        load_or_create_cog_identity(&path).unwrap();
        std::fs::write(&path, "pas-de-l-hex").unwrap();
        assert!(matches!(
            load_or_create_cog_identity(&path),
            Err(IdentityStoreError::Malformed)
        ));
        std::fs::write(&path, "a".repeat(200)).unwrap();
        assert!(matches!(
            load_or_create_cog_identity(&path),
            Err(IdentityStoreError::TooLarge)
        ));
    }

    #[test]
    fn store_one_session_per_device_and_bounded() {
        let store = E2eSessionStore::new(2, Duration::from_secs(60));
        let s1 = session_pair();
        let id1 = hex::encode(s1.session_id());
        assert!(store.insert("dev-a", "k", s1));
        let s2 = session_pair();
        let id2 = hex::encode(s2.session_id());
        assert!(store.insert("dev-a", "k", s2));
        // La session précédente du même appareil est remplacée.
        assert!(store.get_hex(&id1).is_none());
        assert!(store.get_hex(&id2).is_some());
        assert!(store.insert("dev-b", "k", session_pair()));
        assert!(store.insert("dev-c", "k", session_pair()));
        assert_eq!(store.len(), 2);
        assert!(store.get_hex("zz").is_none());
        store.remove_device("dev-c");
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn expired_session_not_returned() {
        let store = E2eSessionStore::new(4, Duration::from_millis(1));
        let s = session_pair();
        let id = hex::encode(s.session_id());
        store.insert("dev", "k", s);
        std::thread::sleep(Duration::from_millis(5));
        assert!(store.get_hex(&id).is_none());
        assert!(store.is_empty());
    }
}
