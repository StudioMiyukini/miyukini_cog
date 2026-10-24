//! @id     central_remote_paired_store
//! @do     persister_liste_blanche_appareils
//! @role   security
//! @layer  toolkit
//! @human  central-remote — persistance de la liste blanche des appareils appairés (R-024).

//! Persistance de la liste blanche des appareils mobiles appairés (R-024).
//!
//! - Fichier JSON `0600` (Unix) dans le même dossier que l'identité E2E du COG
//!   ([`DEFAULT_PAIRED_DEVICES_FILE`]).
//! - Écriture atomique : fichier temporaire `0600` créé à côté (`create_new`),
//!   `fsync`, puis `rename` ; un lecteur ne voit jamais un fichier partiel.
//! - Chargé au démarrage. **Fail-closed** (C-12) : fichier corrompu, trop
//!   grand, aux permissions trop larges, d'une version inconnue ou contenant
//!   une entrée invalide (clé faible, doublon…) ⇒ erreur, et le serveur remote
//!   **refuse de démarrer** plutôt que de repartir d'une liste vide en silence
//!   (une liste vide ferait disparaître des révocations implicites ; une liste
//!   altérée pourrait autoriser une clé étrangère). L'administrateur corrige ou
//!   supprime le fichier explicitement (supprimer = tous les appareils doivent
//!   être ré-appairés).
//! - Fichier absent ⇒ liste vide (premier démarrage).

use crate::pairing::{PairedDevice, MAX_DEVICE_ID_LEN, MAX_DEVICE_NAME_CHARS};
use miyukini_cog_bridge::parse_identity_key_hex;
use rand::rngs::OsRng;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Nom par défaut du fichier de liste blanche (à côté de l'identité E2E).
pub const DEFAULT_PAIRED_DEVICES_FILE: &str = "central-remote-paired-devices.json";

/// Version du format du fichier.
pub const PAIRED_STORE_FORMAT_VERSION: u32 = 1;

/// Taille maximale du fichier (C-13).
const MAX_STORE_FILE_LEN: u64 = 1024 * 1024;

/// Nombre maximal d'appareils persistés (C-13).
pub const MAX_PAIRED_DEVICES: usize = 256;

/// Erreurs du stockage de la liste blanche (une variante par cause, C-11).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PairedStoreError {
    #[error("liste blanche : lecture/écriture impossible ({0})")]
    Io(String),
    #[error("liste blanche : permissions trop larges (attendu 0600)")]
    InsecurePermissions,
    #[error("liste blanche : fichier corrompu ({0})")]
    Malformed(&'static str),
    #[error("liste blanche : fichier trop grand")]
    TooLarge,
    #[error("liste blanche : version de format non supportée ({0})")]
    UnsupportedVersion(u32),
}

/// Entrée persistée (aucun secret : identifiant, nom, clé publique, date).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoredDevice {
    device_id: String,
    name: String,
    /// Clé publique d'identité E2E ed25519 (hex 32 octets, minuscules).
    e2e_identity_key: String,
    /// Date d'approbation (secondes UNIX).
    paired_at: u64,
    mode: String,
    /// Profil Central titulaire (absent dans les fichiers antérieurs).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoreFile {
    format_version: u32,
    devices: Vec<StoredDevice>,
}

/// Charge la liste blanche depuis `path`. Fichier absent ⇒ liste vide ;
/// toute autre anomalie ⇒ erreur (fail-closed).
pub fn load_paired_devices(path: &Path) -> Result<Vec<PairedDevice>, PairedStoreError> {
    let meta = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(PairedStoreError::Io(e.kind().to_string())),
    };
    if !meta.file_type().is_file() {
        return Err(PairedStoreError::Malformed("pas un fichier régulier"));
    }
    if meta.len() > MAX_STORE_FILE_LEN {
        return Err(PairedStoreError::TooLarge);
    }
    check_permissions(&meta)?;

    let mut file =
        std::fs::File::open(path).map_err(|e| PairedStoreError::Io(e.kind().to_string()))?;
    let mut content = Vec::new();
    Read::take(&mut file, MAX_STORE_FILE_LEN + 1)
        .read_to_end(&mut content)
        .map_err(|e| PairedStoreError::Io(e.kind().to_string()))?;
    if content.len() as u64 > MAX_STORE_FILE_LEN {
        return Err(PairedStoreError::TooLarge);
    }
    let parsed: StoreFile =
        serde_json::from_slice(&content).map_err(|_| PairedStoreError::Malformed("JSON"))?;
    if parsed.format_version != PAIRED_STORE_FORMAT_VERSION {
        return Err(PairedStoreError::UnsupportedVersion(parsed.format_version));
    }
    if parsed.devices.len() > MAX_PAIRED_DEVICES {
        return Err(PairedStoreError::TooLarge);
    }

    let mut ids = HashSet::new();
    let mut keys = HashSet::new();
    let mut out = Vec::with_capacity(parsed.devices.len());
    for d in parsed.devices {
        validate_entry(&d)?;
        if !ids.insert(d.device_id.clone()) {
            return Err(PairedStoreError::Malformed("device_id en double"));
        }
        if !keys.insert(d.e2e_identity_key.clone()) {
            return Err(PairedStoreError::Malformed("clé d'identité en double"));
        }
        out.push(PairedDevice {
            device_id: d.device_id,
            name: d.name,
            paired_at: d.paired_at,
            last_seen: d.paired_at,
            mode: d.mode,
            e2e_identity_key: d.e2e_identity_key,
            profile_id: d.profile_id,
        });
    }
    Ok(out)
}

fn validate_entry(d: &StoredDevice) -> Result<(), PairedStoreError> {
    if d.device_id.is_empty()
        || d.device_id.len() > MAX_DEVICE_ID_LEN
        || d.device_id.chars().any(char::is_control)
    {
        return Err(PairedStoreError::Malformed("device_id"));
    }
    if d.name.chars().count() > MAX_DEVICE_NAME_CHARS || d.name.chars().any(char::is_control) {
        return Err(PairedStoreError::Malformed("name"));
    }
    if d.mode.len() > 32 || d.mode.chars().any(char::is_control) {
        return Err(PairedStoreError::Malformed("mode"));
    }
    if d
        .profile_id
        .as_ref()
        .is_some_and(|p| p.is_empty() || p.len() > MAX_DEVICE_ID_LEN || p.chars().any(char::is_control))
    {
        return Err(PairedStoreError::Malformed("profile_id"));
    }
    // Clé valide (point ed25519 non faible) et forme canonique (minuscules).
    let key = parse_identity_key_hex(&d.e2e_identity_key)
        .map_err(|_| PairedStoreError::Malformed("e2e_identity_key"))?;
    if hex::encode(key) != d.e2e_identity_key {
        return Err(PairedStoreError::Malformed("e2e_identity_key non canonique"));
    }
    Ok(())
}

/// Écrit la liste blanche de façon atomique (temporaire `0600` + `rename`).
pub fn save_paired_devices(path: &Path, devices: &[PairedDevice]) -> Result<(), PairedStoreError> {
    if devices.len() > MAX_PAIRED_DEVICES {
        return Err(PairedStoreError::TooLarge);
    }
    let mut stored: Vec<StoredDevice> = devices
        .iter()
        .map(|d| StoredDevice {
            device_id: d.device_id.clone(),
            name: d.name.clone(),
            e2e_identity_key: d.e2e_identity_key.clone(),
            paired_at: d.paired_at,
            mode: d.mode.clone(),
            profile_id: d.profile_id.clone(),
        })
        .collect();
    // Ordre stable (diffs lisibles, contenu déterministe).
    stored.sort_by(|a, b| a.device_id.cmp(&b.device_id));
    let file = StoreFile {
        format_version: PAIRED_STORE_FORMAT_VERSION,
        devices: stored,
    };
    let mut bytes = serde_json::to_vec_pretty(&file)
        .map_err(|_| PairedStoreError::Malformed("sérialisation"))?;
    bytes.push(b'\n');

    let dir = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
    std::fs::create_dir_all(&dir).map_err(|e| PairedStoreError::Io(e.kind().to_string()))?;
    let file_name = path
        .file_name()
        .ok_or(PairedStoreError::Malformed("chemin sans nom de fichier"))?
        .to_string_lossy()
        .into_owned();
    let mut suffix = [0u8; 8];
    OsRng.fill_bytes(&mut suffix);
    let tmp = dir.join(format!(".{file_name}.tmp-{}", hex::encode(suffix)));

    let result = write_new_private_file(&tmp, &bytes)
        .and_then(|()| std::fs::rename(&tmp, path).map_err(|e| PairedStoreError::Io(e.kind().to_string())));
    if result.is_err() {
        let _ = std::fs::remove_file(&tmp);
        return result;
    }
    sync_dir(&dir);
    Ok(())
}

fn write_new_private_file(path: &Path, bytes: &[u8]) -> Result<(), PairedStoreError> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|e| PairedStoreError::Io(e.kind().to_string()))?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|e| PairedStoreError::Io(e.kind().to_string()))
}

/// `fsync` du dossier après `rename` (durabilité ; sans effet hors Unix).
fn sync_dir(dir: &Path) {
    #[cfg(unix)]
    if let Ok(d) = std::fs::File::open(dir) {
        let _ = d.sync_all();
    }
    #[cfg(not(unix))]
    let _ = dir;
}

#[cfg(unix)]
fn check_permissions(meta: &std::fs::Metadata) -> Result<(), PairedStoreError> {
    use std::os::unix::fs::PermissionsExt;
    if meta.permissions().mode() & 0o077 != 0 {
        return Err(PairedStoreError::InsecurePermissions);
    }
    Ok(())
}

#[cfg(not(unix))]
fn check_permissions(_meta: &std::fs::Metadata) -> Result<(), PairedStoreError> {
    // Hors Unix : le fichier vit dans le profil utilisateur (ACL héritées).
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyukini_cog_bridge::E2eIdentity;

    fn tmp_dir() -> PathBuf {
        let d = std::env::temp_dir().join(format!("paired-store-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn device(id: &str) -> PairedDevice {
        PairedDevice {
            device_id: id.into(),
            name: "Pixel".into(),
            paired_at: 1_700_000_000,
            last_seen: 1_700_000_500,
            mode: "whitelist".into(),
            e2e_identity_key: hex::encode(E2eIdentity::generate().public_key_bytes()),
            profile_id: None,
        }
    }

    #[test]
    fn missing_file_is_empty() {
        let p = tmp_dir().join(DEFAULT_PAIRED_DEVICES_FILE);
        assert!(load_paired_devices(&p).unwrap().is_empty());
    }

    #[test]
    fn roundtrip_atomic_and_0600() {
        let dir = tmp_dir();
        let p = dir.join(DEFAULT_PAIRED_DEVICES_FILE);
        let devices = vec![device("dev-b"), device("dev-a")];
        save_paired_devices(&p, &devices).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&p).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
        // Aucun fichier temporaire laissé derrière.
        let names: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec![DEFAULT_PAIRED_DEVICES_FILE.to_string()]);
        let loaded = load_paired_devices(&p).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].device_id, "dev-a");
        let b = loaded.iter().find(|d| d.device_id == "dev-b").unwrap();
        assert_eq!(b.e2e_identity_key, devices[0].e2e_identity_key);
        assert_eq!(b.paired_at, 1_700_000_000);
        // Réécriture (remplacement atomique).
        save_paired_devices(&p, &devices[..1]).unwrap();
        assert_eq!(load_paired_devices(&p).unwrap().len(), 1);
    }

    #[test]
    fn corrupted_file_refused() {
        let dir = tmp_dir();
        let p = dir.join(DEFAULT_PAIRED_DEVICES_FILE);
        let write = |content: &str| {
            let _ = std::fs::remove_file(&p);
            write_new_private_file(&p, content.as_bytes()).unwrap();
        };
        write("{ pas du json");
        assert!(matches!(load_paired_devices(&p), Err(PairedStoreError::Malformed(_))));
        write(r#"{"format_version":9,"devices":[]}"#);
        assert_eq!(load_paired_devices(&p).unwrap_err(), PairedStoreError::UnsupportedVersion(9));
        // Clé faible.
        let mut weak = [0u8; 32];
        weak[0] = 1;
        write(&format!(
            r#"{{"format_version":1,"devices":[{{"device_id":"d1","name":"x","e2e_identity_key":"{}","paired_at":1,"mode":"whitelist"}}]}}"#,
            hex::encode(weak)
        ));
        assert!(matches!(load_paired_devices(&p), Err(PairedStoreError::Malformed(_))));
        // Doublon de clé.
        let k = hex::encode(E2eIdentity::generate().public_key_bytes());
        write(&format!(
            r#"{{"format_version":1,"devices":[{{"device_id":"d1","name":"x","e2e_identity_key":"{k}","paired_at":1,"mode":"w"}},{{"device_id":"d2","name":"x","e2e_identity_key":"{k}","paired_at":1,"mode":"w"}}]}}"#
        ));
        assert!(matches!(load_paired_devices(&p), Err(PairedStoreError::Malformed(_))));
        // Champ inconnu.
        write(r#"{"format_version":1,"devices":[],"extra":1}"#);
        assert!(matches!(load_paired_devices(&p), Err(PairedStoreError::Malformed(_))));
    }

    #[cfg(unix)]
    #[test]
    fn loose_permissions_refused() {
        use std::os::unix::fs::PermissionsExt;
        let p = tmp_dir().join(DEFAULT_PAIRED_DEVICES_FILE);
        save_paired_devices(&p, &[device("dev-a")]).unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert_eq!(load_paired_devices(&p).unwrap_err(), PairedStoreError::InsecurePermissions);
    }

    #[test]
    fn oversized_file_refused() {
        let p = tmp_dir().join(DEFAULT_PAIRED_DEVICES_FILE);
        write_new_private_file(&p, &vec![b' '; (MAX_STORE_FILE_LEN + 1) as usize]).unwrap();
        assert_eq!(load_paired_devices(&p).unwrap_err(), PairedStoreError::TooLarge);
    }
}
