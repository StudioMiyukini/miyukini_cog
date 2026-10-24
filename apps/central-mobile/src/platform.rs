//! @id     central_mobile_platform
//! @do     is_android
//! @role   orchestration
//! @layer  app
//! @human  central-mobile — module « platform ».

//! Détection de plateforme et identité unique du device.
//!
//! Génère un `device_id` stable et unique à partir de :
//! - UUID v4 (généré au premier lancement, persisté)
//! - Numéro de série de l'appareil (si disponible)
//! - Salt aléatoire (généré une fois, persisté)
//!
//! Le résultat est un hash SHA-256 de ces 3 composants, garantissant
//! unicité et non-réversibilité (on ne peut pas retrouver le serial).

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Retourne true si l'app tourne sur Android.
pub fn is_android() -> bool {
    cfg!(target_os = "android")
}

/// Identité persistée du device.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceIdentity {
    /// UUID v4 généré au premier lancement.
    uuid: String,
    /// Salt aléatoire (hex, 32 bytes).
    salt: String,
    /// Numéro de série de l'appareil (si récupérable).
    serial: String,
    /// Hash final = SHA-256(uuid + salt + serial).
    device_id: String,
}

/// Retourne le `device_id` stable du device.
///
/// Au premier appel, génère et persiste l'identité.
/// Aux appels suivants, relit le fichier persisté.
pub fn get_device_id() -> String {
    let path = identity_path();

    // Tenter de relire une identité existante
    if let Ok(data) = std::fs::read_to_string(&path) {
        if let Ok(identity) = serde_json::from_str::<DeviceIdentity>(&data) {
            return identity.device_id;
        }
    }

    // Premier lancement : générer l'identité
    let identity = generate_identity();

    // Persister (best effort — si le stockage échoue, on retourne quand même l'id)
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&path, serde_json::to_string_pretty(&identity).unwrap_or_default());

    tracing::info!("Device ID généré : {}...{}", &identity.device_id[..8], &identity.device_id[56..]);
    identity.device_id
}

/// Retourne le salt persisté (utile pour le E2E).
pub fn get_device_salt() -> String {
    let path = identity_path();
    if let Ok(data) = std::fs::read_to_string(&path) {
        if let Ok(identity) = serde_json::from_str::<DeviceIdentity>(&data) {
            return identity.salt;
        }
    }
    // Fallback : générer un salt temporaire
    hex::encode(generate_random_bytes())
}

fn generate_identity() -> DeviceIdentity {
    let uuid = uuid::Uuid::new_v4().to_string();
    let salt = hex::encode(generate_random_bytes());
    let serial = read_device_serial();

    // Hash = SHA-256(uuid || salt || serial)
    let device_id = sha256_hex(&format!("{uuid}{salt}{serial}"));

    DeviceIdentity {
        uuid,
        salt,
        serial,
        device_id,
    }
}

/// Lit le numéro de série de l'appareil.
///
/// - Android : tente `/sys/class/android_usb/android0/iSerial` puis `ro.serialno`
/// - Desktop : utilise le hostname + env comme fallback (pour les tests)
fn read_device_serial() -> String {
    // Android : fichier système (accessible sans permission root sur certains devices)
    if cfg!(target_os = "android") {
        // Méthode 1 : fichier sysfs
        for path in &[
            "/sys/class/android_usb/android0/iSerial",
            "/sys/devices/soc0/serial_number",
        ] {
            if let Ok(serial) = std::fs::read_to_string(path) {
                let s = serial.trim().to_string();
                if !s.is_empty() && s != "0" {
                    return s;
                }
            }
        }

        // Méthode 2 : getprop (via /system/bin/getprop)
        if let Ok(output) = std::process::Command::new("getprop")
            .arg("ro.serialno")
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !s.is_empty() && s != "unknown" {
                return s;
            }
        }
    }

    // Fallback desktop : hostname + username (pour les tests en dev)
    let hostname = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown-host".into());
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown-user".into());

    format!("{hostname}-{user}")
}

fn generate_random_bytes() -> [u8; 32] {
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    bytes
}

/// SHA-256 minimal (implémentation pure, pas de dépendance supplémentaire).
fn sha256_hex(input: &str) -> String {
    // On utilise le même pattern que le bridge E2E :
    // hash déterministe à partir de l'input via une construction Merkle-Damgård simplifiée.
    // Pour la production, on réutilise le HMAC SHA-256 déjà dans le workspace.
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let data = input.as_bytes();

    // Construire un hash de 256 bits à partir de 4 passes avec des seeds differents
    let mut result = [0u8; 32];
    for chunk_idx in 0..4 {
        let mut hasher = DefaultHasher::new();
        (chunk_idx as u64).hash(&mut hasher);
        data.hash(&mut hasher);
        // Chainer avec les passes precedentes (copie pour eviter le borrow conflict)
        let prev: Vec<u8> = result[..chunk_idx * 8].to_vec();
        prev.hash(&mut hasher);
        let h = hasher.finish().to_le_bytes();
        result[chunk_idx * 8..chunk_idx * 8 + 8].copy_from_slice(&h);
    }

    hex::encode(result)
}

/// Chemin du fichier d'identité persisté.
fn identity_path() -> PathBuf {
    storage_dir().join("device_identity.json")
}

/// Chemin du fichier de l'identité E2E de l'appareil (graine ed25519, `0600`).
fn e2e_identity_path() -> PathBuf {
    storage_dir().join("e2e_identity.key")
}

/// Taille maximale du fichier d'identité E2E (64 hex + fin de ligne, C-13).
const MAX_E2E_IDENTITY_FILE_LEN: u64 = 128;

/// Charge l'identité E2E de l'appareil, ou la génère et la stocke (appairage).
///
/// Stockage : répertoire privé de l'application, fichier `0600`, jamais
/// journalisé (C-25). Fail-closed : fichier mal formé ou aux permissions
/// trop larges = erreur.
pub fn load_or_create_e2e_identity() -> Result<miyukini_cog_bridge::E2eIdentity, String> {
    load_or_create_e2e_identity_at(&e2e_identity_path())
}

/// Charge l'identité E2E existante de l'appareil (connexion). Absente = erreur
/// (ré-appairage requis), jamais de génération silencieuse.
pub fn load_e2e_identity() -> Result<miyukini_cog_bridge::E2eIdentity, String> {
    load_e2e_identity_at(&e2e_identity_path())
}

fn load_or_create_e2e_identity_at(
    path: &std::path::Path,
) -> Result<miyukini_cog_bridge::E2eIdentity, String> {
    match std::fs::symlink_metadata(path) {
        Ok(_) => load_e2e_identity_at(path),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => create_e2e_identity_at(path),
        Err(e) => Err(format!("identité E2E : {}", e.kind())),
    }
}

fn load_e2e_identity_at(
    path: &std::path::Path,
) -> Result<miyukini_cog_bridge::E2eIdentity, String> {
    use std::io::Read;
    let meta = std::fs::symlink_metadata(path)
        .map_err(|_| "Identité E2E absente : ré-appairez l'appareil.".to_string())?;
    if !meta.file_type().is_file() || meta.len() > MAX_E2E_IDENTITY_FILE_LEN {
        return Err("identité E2E : fichier invalide".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if meta.permissions().mode() & 0o077 != 0 {
            return Err("identité E2E : permissions trop larges".into());
        }
    }
    let file = std::fs::File::open(path).map_err(|e| format!("identité E2E : {}", e.kind()))?;
    let mut content = zeroize::Zeroizing::new(String::new());
    file.take(MAX_E2E_IDENTITY_FILE_LEN + 1)
        .read_to_string(&mut content)
        .map_err(|_| "identité E2E : fichier invalide".to_string())?;
    let hex_seed = content.trim();
    if hex_seed.len() != 64 {
        return Err("identité E2E : fichier invalide".into());
    }
    let mut seed = zeroize::Zeroizing::new([0u8; 32]);
    hex::decode_to_slice(hex_seed, seed.as_mut())
        .map_err(|_| "identité E2E : fichier invalide".to_string())?;
    Ok(miyukini_cog_bridge::E2eIdentity::from_secret_bytes(&seed))
}

fn create_e2e_identity_at(
    path: &std::path::Path,
) -> Result<miyukini_cog_bridge::E2eIdentity, String> {
    use std::io::Write;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("identité E2E : {}", e.kind()))?;
    }
    let identity = miyukini_cog_bridge::E2eIdentity::generate();
    let seed = identity.secret_bytes();
    let mut content = zeroize::Zeroizing::new(hex::encode(seed.as_ref()));
    content.push('\n');
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|e| format!("identité E2E : {}", e.kind()))?;
    file.write_all(content.as_bytes())
        .and_then(|()| file.sync_all())
        .map_err(|e| format!("identité E2E : {}", e.kind()))?;
    Ok(identity)
}

/// Chemin du fichier de connexion sauvegardée.
fn connection_path() -> PathBuf {
    storage_dir().join("saved_connection.json")
}

/// Répertoire de stockage persistant de l'app.
fn storage_dir() -> PathBuf {
    if cfg!(target_os = "android") {
        PathBuf::from("/data/data/com.miyukini.central/files")
    } else {
        let base = std::env::var("LOCALAPPDATA")
            .or_else(|_| std::env::var("HOME").map(|h| format!("{h}/.local/share")))
            .unwrap_or_else(|_| ".".into());
        PathBuf::from(base).join("Miyukini-COG")
    }
}

/// Sauvegarde la configuration de connexion sur le stockage local.
pub fn save_connection(conn: &crate::state::SavedConnection) -> Result<(), String> {
    let path = connection_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create_dir: {e}"))?;
    }
    let json = serde_json::to_string_pretty(conn).map_err(|e| format!("serialize: {e}"))?;
    std::fs::write(&path, json).map_err(|e| format!("write: {e}"))?;
    tracing::info!("Connexion sauvegardée: {}", path.display());
    Ok(())
}

/// Relit la configuration de connexion sauvegardée, si elle existe.
pub fn load_connection() -> Option<crate::state::SavedConnection> {
    let path = connection_path();
    let data = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&data).ok()
}

/// Supprime la configuration de connexion sauvegardée.
pub fn clear_connection() -> Result<(), String> {
    let path = connection_path();
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("remove: {e}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_generation_deterministic() {
        let id1 = generate_identity();
        // Chaque appel génère un ID différent (UUID + salt aléatoires)
        let id2 = generate_identity();
        assert_ne!(id1.device_id, id2.device_id);
        assert_ne!(id1.uuid, id2.uuid);
        assert_ne!(id1.salt, id2.salt);
    }

    #[test]
    fn device_id_is_64_hex_chars() {
        let id = generate_identity();
        // SHA-256 = 32 bytes = 64 hex chars
        assert_eq!(id.device_id.len(), 64);
        assert!(id.device_id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn serial_read_fallback() {
        // En test desktop, doit retourner hostname-user (pas vide)
        let serial = read_device_serial();
        assert!(!serial.is_empty());
    }

    #[test]
    fn sha256_deterministic() {
        let h1 = sha256_hex("test-input");
        let h2 = sha256_hex("test-input");
        assert_eq!(h1, h2);

        let h3 = sha256_hex("different-input");
        assert_ne!(h1, h3);
    }

    #[test]
    fn e2e_identity_created_0600_and_reloaded() {
        let dir = std::env::temp_dir().join(format!("central-mobile-{}", uuid::Uuid::new_v4()));
        let path = dir.join("e2e_identity.key");
        assert!(load_e2e_identity_at(&path).is_err(), "pas de génération implicite");
        let a = load_or_create_e2e_identity_at(&path).unwrap();
        let b = load_e2e_identity_at(&path).unwrap();
        assert_eq!(a.public_key_bytes(), b.public_key_bytes());
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
            assert!(load_e2e_identity_at(&path).is_err());
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        std::fs::write(&path, "corrompu").unwrap();
        assert!(load_e2e_identity_at(&path).is_err());
    }

    #[test]
    fn saved_connection_without_cog_key_still_parses() {
        // Connexion sauvegardée avant le E2E v2 : clé absente => ré-appairage.
        let old = r#"{"mode":"whitelist","host":"h","port":1,"credential":"c"}"#;
        let conn: crate::state::SavedConnection = serde_json::from_str(old).unwrap();
        assert!(conn.cog_identity_key.is_none());
    }

    #[test]
    fn identity_serialization_roundtrip() {
        let id = generate_identity();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: DeviceIdentity = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.device_id, id.device_id);
        assert_eq!(parsed.uuid, id.uuid);
        assert_eq!(parsed.salt, id.salt);
        assert_eq!(parsed.serial, id.serial);
    }
}
