//! Dérivation de clé pour SQLCipher.
//!
//! @id: kindmother_db_key_derivation
//! @do: derive_aes256_key_per_database
//! @layer: core

use argon2::{Argon2, Params, Version};
use std::path::Path;
use zeroize::Zeroizing;

/// Erreur de dérivation de clé.
#[derive(Debug)]
pub struct DbKeyError(pub String);

impl std::fmt::Display for DbKeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DB key error: {}", self.0)
    }
}

impl std::error::Error for DbKeyError {}

/// Générateur de clé souveraine pour bases de données.
///
/// Dérive une clé AES-256 unique par base à partir :
/// - D'un secret d'installation (stocké dans `data_dir/.kindmother_secret`)
/// - De l'identifiant machine (MachineGuid Windows, /etc/machine-id Linux/macOS)
/// - Du nom de la base (salt)
pub struct KeyDerivation {
    install_secret: Zeroizing<[u8; 32]>,
}

impl KeyDerivation {
    /// Crée une nouvelle instance de dérivation de clé.
    ///
    /// # Arguments
    /// * `data_dir` - Répertoire de données où le secret d'installation est stocké
    pub fn new(data_dir: &Path) -> Result<Self, DbKeyError> {
        let install_secret = Self::get_or_create_install_secret(data_dir)?;
        Ok(Self { install_secret })
    }

    /// Dérive une clé AES-256 pour une base de données.
    ///
    /// La clé est unique par base grâce au salt.
    pub fn derive_key(&self, database_name: &str) -> Result<Zeroizing<Vec<u8>>, DbKeyError> {
        let machine_id = Self::get_machine_id()?;

        let salt = format!("miyukini-kindmother-{}-v1", database_name);

        let mut input = Vec::with_capacity(machine_id.len() + 32);
        input.extend_from_slice(&machine_id);
        input.extend_from_slice(self.install_secret.as_ref());

        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| DbKeyError(format!("Invalid Argon2 params: {}", e)))?;

        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);

        let mut key = Zeroizing::new(vec![0u8; 32]);
        argon2
            .hash_password_into(input.as_slice(), salt.as_bytes(), key.as_mut_slice())
            .map_err(|e| DbKeyError(format!("Key derivation failed: {}", e)))?;

        Ok(key)
    }

    /// Retourne la valeur à passer à `PRAGMA key` pour SQLCipher (format hex raw bytes).
    pub fn pragma_key_hex(&self, database_name: &str) -> Result<String, DbKeyError> {
        let key = self.derive_key(database_name)?;
        let hex_str: String = key.iter().map(|b| format!("{:02x}", b)).collect();
        Ok(format!("x'{}'", hex_str))
    }

    #[cfg(windows)]
    fn get_machine_id() -> Result<Vec<u8>, DbKeyError> {
        use winreg::enums::*;
        use winreg::RegKey;

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm
            .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
            .map_err(|e| DbKeyError(format!("Cannot read machine GUID: {}", e)))?;

        let guid: String = key
            .get_value("MachineGuid")
            .map_err(|e| DbKeyError(format!("MachineGuid not found: {}", e)))?;

        Ok(guid.into_bytes())
    }

    #[cfg(not(windows))]
    fn get_machine_id() -> Result<Vec<u8>, DbKeyError> {
        let machine_id = std::fs::read_to_string("/etc/machine-id")
            .or_else(|_| std::fs::read_to_string("/var/lib/dbus/machine-id"))
            .map_err(|_| DbKeyError("Cannot read machine ID".to_string()))?;

        Ok(machine_id.trim().as_bytes().to_vec())
    }

    fn get_or_create_install_secret(data_dir: &Path) -> Result<Zeroizing<[u8; 32]>, DbKeyError> {
        let secret_path = data_dir.join(".kindmother_secret");

        if secret_path.exists() {
            let content = std::fs::read(&secret_path)
                .map_err(|e| DbKeyError(format!("Cannot read install secret: {}", e)))?;

            if content.len() != 32 {
                return Err(DbKeyError("Invalid install secret length".to_string()));
            }

            let mut secret = [0u8; 32];
            secret.copy_from_slice(&content);
            Ok(Zeroizing::new(secret))
        } else {
            let mut secret = [0u8; 32];
            getrandom::getrandom(&mut secret)
                .map_err(|e| DbKeyError(format!("Cannot generate random bytes: {}", e)))?;

            if let Some(parent) = secret_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| DbKeyError(format!("Cannot create data dir: {}", e)))?;
            }

            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&secret_path)
                .map_err(|e| DbKeyError(format!("Cannot create install secret: {}", e)))?;

            use std::io::Write;
            file.write_all(&secret)
                .map_err(|e| DbKeyError(format!("Cannot write install secret: {}", e)))?;

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

    #[test]
    fn test_pragma_key_format() {
        let temp_dir = std::env::temp_dir().join("kindmother_test_pragma");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(&temp_dir).unwrap();

        let kd = KeyDerivation::new(&temp_dir).unwrap();
        let pragma = kd.pragma_key_hex("test").unwrap();

        assert!(pragma.starts_with("x'"));
        assert!(pragma.ends_with('\''));
        assert_eq!(pragma.len(), 2 + 64 + 1); // x' + 32 bytes hex + '

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
