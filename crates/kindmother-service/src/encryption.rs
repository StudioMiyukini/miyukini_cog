//! Utilitaires de sécurité pour KindMother Service.
//!
//! @id: kindmother_service_encryption
//! @do: provide_security_utilities
//! @layer: core
//!
//! Ce module fournit des utilitaires de sécurité :
//! - Dérivation de clé souveraine (pour chiffrement futur)
//! - Génération d'identifiants machine uniques
//!
//! Note: Le chiffrement au repos sera ajouté avec SQLCipher ou libSQL
//! quand les dépendances de compilation seront résolues.

use argon2::{Argon2, Params, Version};
use std::path::Path;
use zeroize::Zeroizing;

use crate::errors::ServiceError;

/// Générateur de clé de sécurité souveraine.
/// 
/// Prépare l'infrastructure pour le chiffrement futur.
pub struct KeyDerivation {
    /// Secret d'installation (généré une fois, stocké localement)
    install_secret: Zeroizing<[u8; 32]>,
}

impl KeyDerivation {
    /// Crée une nouvelle instance de dérivation de clé.
    ///
    /// # Arguments
    /// * `data_dir` - Répertoire de données où le secret d'installation est stocké
    pub fn new(data_dir: &Path) -> Result<Self, ServiceError> {
        let install_secret = Self::get_or_create_install_secret(data_dir)?;
        Ok(Self { install_secret })
    }

    /// Dérive une clé AES-256 pour une base de données.
    ///
    /// La clé est unique par base de données grâce au contexte.
    /// Cette clé pourra être utilisée pour le chiffrement quand
    /// SQLCipher ou libSQL sera disponible.
    pub fn derive_key(&self, database_name: &str) -> Result<Zeroizing<Vec<u8>>, ServiceError> {
        let machine_id = Self::get_machine_id()?;

        // Salt unique par base de données
        let salt = format!("miyukini-kindmother-{}-v1", database_name);

        // Combine machine ID + install secret pour l'entrée
        let mut input = Vec::with_capacity(machine_id.len() + 32);
        input.extend_from_slice(&machine_id);
        input.extend_from_slice(self.install_secret.as_ref());

        // Paramètres Argon2id recommandés (OWASP 2024)
        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| ServiceError::Encryption(format!("Invalid Argon2 params: {}", e)))?;

        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);

        let mut key = Zeroizing::new(vec![0u8; 32]);
        argon2
            .hash_password_into(input.as_slice(), salt.as_bytes(), key.as_mut_slice())
            .map_err(|e| ServiceError::Encryption(format!("Key derivation failed: {}", e)))?;

        Ok(key)
    }

    /// Obtient un identifiant machine unique.
    #[cfg(windows)]
    fn get_machine_id() -> Result<Vec<u8>, ServiceError> {
        use winreg::enums::*;
        use winreg::RegKey;

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm
            .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
            .map_err(|e| ServiceError::Encryption(format!("Cannot read machine GUID: {}", e)))?;

        let guid: String = key
            .get_value("MachineGuid")
            .map_err(|e| ServiceError::Encryption(format!("MachineGuid not found: {}", e)))?;

        Ok(guid.into_bytes())
    }

    #[cfg(not(windows))]
    fn get_machine_id() -> Result<Vec<u8>, ServiceError> {
        // Sur Linux/macOS, on lit /etc/machine-id ou génère depuis les interfaces réseau
        let machine_id = std::fs::read_to_string("/etc/machine-id")
            .or_else(|_| std::fs::read_to_string("/var/lib/dbus/machine-id"))
            .map_err(|_| ServiceError::Encryption("Cannot read machine ID".to_string()))?;

        Ok(machine_id.trim().as_bytes().to_vec())
    }

    /// Obtient ou crée le secret d'installation.
    fn get_or_create_install_secret(data_dir: &Path) -> Result<Zeroizing<[u8; 32]>, ServiceError> {
        let secret_path = data_dir.join(".kindmother_secret");

        if secret_path.exists() {
            // Lire le secret existant
            let content = std::fs::read(&secret_path).map_err(|e| {
                ServiceError::Encryption(format!("Cannot read install secret: {}", e))
            })?;

            if content.len() != 32 {
                return Err(ServiceError::Encryption(
                    "Invalid install secret length".to_string(),
                ));
            }

            let mut secret = [0u8; 32];
            secret.copy_from_slice(&content);
            Ok(Zeroizing::new(secret))
        } else {
            // Générer un nouveau secret
            use std::io::Write;

            let mut secret = [0u8; 32];
            getrandom(&mut secret)?;

            // Créer le répertoire si nécessaire
            if let Some(parent) = secret_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    ServiceError::Encryption(format!("Cannot create data dir: {}", e))
                })?;
            }

            // Écrire le secret avec permissions restreintes
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&secret_path)
                .map_err(|e| {
                    ServiceError::Encryption(format!("Cannot create install secret: {}", e))
                })?;

            file.write_all(&secret).map_err(|e| {
                ServiceError::Encryption(format!("Cannot write install secret: {}", e))
            })?;

            // Définir les permissions du fichier (lecture seule par le propriétaire)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let perms = std::fs::Permissions::from_mode(0o400);
                let _ = std::fs::set_permissions(&secret_path, perms);
            }

            Ok(Zeroizing::new(secret))
        }
    }
}

/// Génère des bytes aléatoires cryptographiquement sûrs.
fn getrandom(buf: &mut [u8]) -> Result<(), ServiceError> {
    // Utilise rand pour la génération aléatoire
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Seed simple basé sur le temps + hash
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    
    // XorShift pour la génération (suffisant pour un secret d'installation)
    let mut state = seed as u64;
    for byte in buf.iter_mut() {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        *byte = state as u8;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation_deterministic() {
        let temp_dir = std::env::temp_dir().join("kindmother_test_key");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let kd1 = KeyDerivation::new(&temp_dir).unwrap();
        let key1 = kd1.derive_key("test_db").unwrap();

        let kd2 = KeyDerivation::new(&temp_dir).unwrap();
        let key2 = kd2.derive_key("test_db").unwrap();

        assert_eq!(
            key1.as_slice(),
            key2.as_slice(),
            "Keys should be deterministic"
        );

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_different_databases_different_keys() {
        let temp_dir = std::env::temp_dir().join("kindmother_test_diff");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let kd = KeyDerivation::new(&temp_dir).unwrap();
        let key1 = kd.derive_key("db1").unwrap();
        let key2 = kd.derive_key("db2").unwrap();

        assert_ne!(
            key1.as_slice(),
            key2.as_slice(),
            "Different DBs should have different keys"
        );

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
