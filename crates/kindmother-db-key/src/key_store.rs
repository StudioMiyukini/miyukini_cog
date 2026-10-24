//! @id     kindmother_db_key_store
//! @do     deriver_cle_par_base_depuis_secret_propre
//! @role   securite
//! @layer  core
//! @human  kindmother-db-key — une clé SQLCipher par base, issue d'un secret aléatoire propre à la base (K-07).

//! Magasin de clés par base (K-07, EX-21 « une clé de DB par service », C-40).
//!
//! Schéma `v1` ([`crate::KeyDerivation`]) : clé = Argon2id(machine-id ‖ secret
//! d'installation, sel = nom de la base). Le nom est public : quiconque lit le
//! secret d'installation et le machine-id dérive **toutes** les clés.
//!
//! Schéma `v2` (ce module) : chaque base reçoit à sa création un secret aléatoire
//! de 32 octets, stocké dans `<data_dir>/keys/<base>.secret` (répertoire 0700,
//! fichier 0400), lu par `kindmother-service` seul. Clé = Argon2id(machine-id ‖
//! secret d'installation ‖ secret de la base, sel = « miyukini-kindmother-v2-<base> »).
//! La fuite du secret d'une base ne donne pas les clés des autres. Les services ne
//! manipulent plus aucune clé : leur persistance passe par KindMother (K-13).
//!
//! Compatibilité : une base existante sans secret `v2` est ouverte avec la clé `v1`
//! (signalé par [`KeyScheme::V1Legacy`]) ; le passage en `v2` d'une base existante
//! (`PRAGMA rekey`) reste à faire (voir le rapport du lot C).

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use argon2::{Argon2, Params, Version};
use zeroize::Zeroizing;

use crate::key_derivation::{DbKeyError, KeyDerivation};

/// Sous-répertoire des secrets par base.
pub const KEYS_DIR: &str = "keys";

/// Schéma de clé utilisé pour une base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyScheme {
    /// Secret propre à la base (K-07).
    V2PerDatabase,
    /// Base existante créée avec la dérivation `v1` (secret d'installation partagé).
    V1Legacy,
}

/// Clé `PRAGMA key` en cache, et son schéma.
type CachedKey = (Zeroizing<String>, KeyScheme);

/// Magasin de clés par base, détenu par `kindmother-service`.
pub struct DatabaseKeyStore {
    keys_dir: PathBuf,
    install_secret: Zeroizing<[u8; 32]>,
    legacy: KeyDerivation,
    cache: Mutex<HashMap<String, CachedKey>>,
}

impl DatabaseKeyStore {
    /// Ouvre (ou crée) le magasin dans `data_dir`.
    pub fn open(data_dir: &Path) -> Result<Self, DbKeyError> {
        let install_secret = KeyDerivation::get_or_create_install_secret(data_dir)?;
        let legacy = KeyDerivation::new(data_dir)?;
        let keys_dir = data_dir.join(KEYS_DIR);
        std::fs::create_dir_all(&keys_dir)
            .map_err(|e| DbKeyError(format!("Cannot create keys dir: {e}")))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&keys_dir, std::fs::Permissions::from_mode(0o700))
                .map_err(|e| DbKeyError(format!("Cannot restrict keys dir: {e}")))?;
        }
        Ok(Self {
            keys_dir,
            install_secret,
            legacy,
            cache: Mutex::new(HashMap::new()),
        })
    }

    /// Valeur `PRAGMA key` (format `x'…'`) pour `database`.
    ///
    /// `existing_file` : la base existe déjà sur disque. Une base existante sans
    /// secret `v2` est ouverte en `v1` ; une base nouvelle reçoit un secret `v2`.
    pub fn pragma_key(
        &self,
        database: &str,
        existing_file: bool,
    ) -> Result<(Zeroizing<String>, KeyScheme), DbKeyError> {
        if !is_valid_name(database) {
            return Err(DbKeyError(format!("Invalid database name: {database}")));
        }
        {
            let cache = self
                .cache
                .lock()
                .map_err(|_| DbKeyError("key cache poisoned".to_string()))?;
            if let Some((key, scheme)) = cache.get(database) {
                return Ok((key.clone(), *scheme));
            }
        }

        let secret_path = self.secret_path(database);
        let (key, scheme) = if secret_path.exists() {
            let secret = read_secret(&secret_path)?;
            (self.derive_v2(database, &secret)?, KeyScheme::V2PerDatabase)
        } else if existing_file {
            (
                Zeroizing::new(self.legacy.pragma_key_hex(database)?),
                KeyScheme::V1Legacy,
            )
        } else {
            let secret = create_secret(&secret_path)?;
            (self.derive_v2(database, &secret)?, KeyScheme::V2PerDatabase)
        };

        let mut cache = self
            .cache
            .lock()
            .map_err(|_| DbKeyError("key cache poisoned".to_string()))?;
        cache.insert(database.to_string(), (key.clone(), scheme));
        Ok((key, scheme))
    }

    /// Chemin du secret propre à `database`.
    pub fn secret_path(&self, database: &str) -> PathBuf {
        self.keys_dir.join(format!("{database}.secret"))
    }

    fn derive_v2(&self, database: &str, db_secret: &[u8; 32]) -> Result<Zeroizing<String>, DbKeyError> {
        let machine_id = KeyDerivation::get_machine_id()?;
        let mut input = Zeroizing::new(Vec::with_capacity(machine_id.len() + 64));
        input.extend_from_slice(&machine_id);
        input.extend_from_slice(self.install_secret.as_ref());
        input.extend_from_slice(db_secret);
        let salt = format!("miyukini-kindmother-v2-{database}");

        let params = Params::new(65536, 3, 4, Some(32))
            .map_err(|e| DbKeyError(format!("Invalid Argon2 params: {e}")))?;
        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);
        let mut key = Zeroizing::new([0u8; 32]);
        argon2
            .hash_password_into(input.as_slice(), salt.as_bytes(), key.as_mut_slice())
            .map_err(|e| DbKeyError(format!("Key derivation failed: {e}")))?;
        let hex: String = key.iter().map(|b| format!("{b:02x}")).collect();
        Ok(Zeroizing::new(format!("x'{hex}'")))
    }
}

fn is_valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        && !name.starts_with('-')
}

fn read_secret(path: &Path) -> Result<Zeroizing<[u8; 32]>, DbKeyError> {
    let content = Zeroizing::new(
        std::fs::read(path).map_err(|e| DbKeyError(format!("Cannot read database secret: {e}")))?,
    );
    if content.len() != 32 {
        return Err(DbKeyError("Invalid database secret length".to_string()));
    }
    let mut secret = Zeroizing::new([0u8; 32]);
    secret.copy_from_slice(&content);
    Ok(secret)
}

fn create_secret(path: &Path) -> Result<Zeroizing<[u8; 32]>, DbKeyError> {
    use std::io::Write;
    let mut secret = Zeroizing::new([0u8; 32]);
    getrandom::getrandom(secret.as_mut_slice())
        .map_err(|e| DbKeyError(format!("Cannot generate random bytes: {e}")))?;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = match options.open(path) {
        Ok(f) => f,
        // Créé entre-temps par un autre appel : on relit.
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => return read_secret(path),
        Err(e) => return Err(DbKeyError(format!("Cannot create database secret: {e}"))),
    };
    file.write_all(secret.as_ref())
        .and_then(|()| file.sync_all())
        .map_err(|e| DbKeyError(format!("Cannot write database secret: {e}")))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o400));
    }
    Ok(secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("km-keystore-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).expect("dir");
        d
    }

    #[test]
    fn cle_v2_stable_et_propre_a_chaque_base() {
        let d = dir("v2");
        let store = DatabaseKeyStore::open(&d).expect("store");
        let (a1, s) = store.pragma_key("alpha", false).expect("alpha");
        assert_eq!(s, KeyScheme::V2PerDatabase);
        let (b, _) = store.pragma_key("beta", false).expect("beta");
        assert_ne!(*a1, *b);
        assert!(store.secret_path("alpha").exists());

        // Réouverture : même clé (secret relu, pas recréé).
        let store2 = DatabaseKeyStore::open(&d).expect("store2");
        let (a2, s2) = store2.pragma_key("alpha", true).expect("alpha2");
        assert_eq!(*a1, *a2);
        assert_eq!(s2, KeyScheme::V2PerDatabase);
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn v2_differe_de_v1_et_secret_requis() {
        let d = dir("v1v2");
        let store = DatabaseKeyStore::open(&d).expect("store");
        let legacy = KeyDerivation::new(&d).expect("legacy");
        let v1 = legacy.pragma_key_hex("gamma").expect("v1");
        let (v2, _) = store.pragma_key("gamma", false).expect("v2");
        assert_ne!(v1, *v2, "le secret d'installation seul ne suffit plus");

        // Base existante sans secret v2 : clé v1 (compatibilité).
        let (k, s) = store.pragma_key("delta", true).expect("delta");
        assert_eq!(s, KeyScheme::V1Legacy);
        assert_eq!(*k, legacy.pragma_key_hex("delta").expect("v1"));
        assert!(!store.secret_path("delta").exists());
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn nom_invalide_refuse() {
        let d = dir("nom");
        let store = DatabaseKeyStore::open(&d).expect("store");
        assert!(store.pragma_key("../x", false).is_err());
        assert!(store.pragma_key("", false).is_err());
        let _ = std::fs::remove_dir_all(d);
    }
}
