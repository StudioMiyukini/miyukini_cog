//! Adaptateur KindMother — Utilitaires communs pour les services.
//!
//! Ce crate fournit des fonctions utilitaires pures utilisables par tous les
//! services de l'écosystème Miyukini COG.
//!
//! Depuis la migration libSQL, l'accès base de données passe exclusivement
//! par `kindmother-client`. Ce crate ne conserve que les helpers purs :
//! - Conversions bool/int
//! - Timestamps (RFC3339, ISO local, clé mois)
//! - Génération UUID
//! - Hash SHA-256
//!
//! @id: kindmother_db_adapter
//! @do: provide_common_utility_functions
//! @layer: infra

// ═══════════════════════════════════════════════════════════════════════════
// DbError - Type d'erreur générique
// ═══════════════════════════════════════════════════════════════════════════

/// Erreur générique pour les services KindMother.
///
/// Encapsule les erreurs de communication, sérialisation, et internes.
#[derive(Debug)]
pub struct DbError {
    /// Nom du service (pour les messages d'erreur).
    pub service: &'static str,
    /// Message d'erreur détaillé.
    pub message: String,
}

impl DbError {
    /// Crée une nouvelle erreur avec le nom du service et le message.
    pub fn new(service: &'static str, message: impl Into<String>) -> Self {
        Self {
            service,
            message: message.into(),
        }
    }

    /// Crée une erreur à partir d'un échec de verrouillage du mutex.
    pub fn from_mutex<T>(service: &'static str, _e: std::sync::PoisonError<T>) -> Self {
        Self::new(service, "Mutex lock poisoned")
    }
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} DB: {}", self.service, self.message)
    }
}

impl std::error::Error for DbError {}

// ═══════════════════════════════════════════════════════════════════════════
// Macro pour définir un type DbError spécifique à un service
// ═══════════════════════════════════════════════════════════════════════════

/// Macro pour définir un type `DbError` spécifique à un service.
///
/// Génère une struct `DbError` avec les implémentations standards
/// (Display, Error, From<String>).
///
/// # Exemple
///
/// ```ignore
/// define_db_error!(JayFestival);
/// // Génère: pub struct DbError(pub String);
/// // avec impl Display affichant "JayFestival DB: {message}"
/// ```
#[macro_export]
macro_rules! define_db_error {
    ($service_name:ident) => {
        /// Erreur de la base de données.
        #[derive(Debug)]
        pub struct DbError(pub String);

        impl std::fmt::Display for DbError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($service_name), " DB: {}"), self.0)
            }
        }

        impl std::error::Error for DbError {}

        impl From<String> for DbError {
            fn from(s: String) -> Self {
                DbError(s)
            }
        }
    };
}

// ═══════════════════════════════════════════════════════════════════════════
// Helpers pour les conversions de types courants
// ═══════════════════════════════════════════════════════════════════════════

/// Convertit un `Option<i32>` en `Option<bool>`.
///
/// Convention: 0 = false, non-0 = true.
#[inline]
pub fn int_to_bool_opt(val: Option<i32>) -> Option<bool> {
    val.map(|x| x != 0)
}

/// Convertit un `i32` en `bool`.
///
/// Convention: 0 = false, non-0 = true.
#[inline]
pub fn int_to_bool(val: i32) -> bool {
    val != 0
}

/// Convertit un `bool` en `i32`.
///
/// Convention: false = 0, true = 1.
#[inline]
pub fn bool_to_int(val: bool) -> i32 {
    if val {
        1
    } else {
        0
    }
}

/// Convertit un `Option<bool>` en `Option<i32>`.
#[inline]
pub fn bool_opt_to_int_opt(val: Option<bool>) -> Option<i32> {
    val.map(|b| if b { 1 } else { 0 })
}

// ═══════════════════════════════════════════════════════════════════════════
// Helpers pour les timestamps
// ═══════════════════════════════════════════════════════════════════════════

/// Retourne le timestamp courant au format RFC3339 (UTC).
pub fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Retourne le timestamp courant au format local ISO.
pub fn now_local_iso() -> String {
    chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}

/// Retourne le mois courant au format "YYYY-MM".
pub fn current_month_key() -> String {
    chrono::Local::now().format("%Y-%m").to_string()
}

// ═══════════════════════════════════════════════════════════════════════════
// Helpers pour les identifiants
// ═══════════════════════════════════════════════════════════════════════════

/// Génère un nouvel UUID v4 sous forme de String.
pub fn new_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Génère un UUID si l'option est None, sinon retourne la valeur existante.
pub fn ensure_uuid(id: Option<String>) -> String {
    id.unwrap_or_else(new_uuid)
}

// ═══════════════════════════════════════════════════════════════════════════
// Helpers pour le hachage de mots de passe (Argon2id + compatibilité legacy)
// ═══════════════════════════════════════════════════════════════════════════

const ARGON2_PREFIX: &str = "argon2id$";
const ARGON2_SALT_LEN: usize = 16;
const ARGON2_OUTPUT_LEN: usize = 32;

/// Indique si le hash stocké est au format legacy (SHA-256, 64 caractères hex).
pub fn is_legacy_password_hash(stored: &str) -> bool {
    stored.len() == 64 && stored.chars().all(|c| c.is_ascii_hexdigit())
}

/// Hache un mot de passe avec Argon2id (recommandé pour la production).
///
/// Format stocké : `argon2id$base64(salt)$base64(hash)`.
pub fn hash_password(password: &str) -> String {
    use argon2::{Argon2, Params, Version};
    let mut salt = [0u8; ARGON2_SALT_LEN];
    getrandom::getrandom(&mut salt).expect("getrandom failed");
    let params = Params::new(65536, 3, 4, Some(ARGON2_OUTPUT_LEN)).expect("Argon2 params");
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);
    let mut out = [0u8; ARGON2_OUTPUT_LEN];
    argon2
        .hash_password_into(password.as_bytes(), &salt, &mut out)
        .expect("Argon2 hash");
    use base64::Engine;
    let salt_b64 = base64::engine::general_purpose::STANDARD.encode(&salt);
    let hash_b64 = base64::engine::general_purpose::STANDARD.encode(&out);
    format!("{}{}${}", ARGON2_PREFIX, salt_b64, hash_b64)
}

/// Vérifie un mot de passe contre un hash stocké (legacy SHA-256 ou Argon2id).
///
/// Utilise une comparaison en temps constant pour les hashes legacy.
/// Retourne `true` si le mot de passe est valide.
pub fn verify_password(stored: &str, password: &str) -> bool {
    if is_legacy_password_hash(stored) {
        #[allow(deprecated)]
        let computed = hash_password_sha256(password);
        subtle::ConstantTimeEq::ct_eq(computed.as_bytes(), stored.as_bytes()).into()
    } else if let Some(rest) = stored.strip_prefix(ARGON2_PREFIX) {
        verify_password_argon2(rest, password)
    } else {
        false
    }
}

fn verify_password_argon2(encoded: &str, password: &str) -> bool {
    use argon2::{Argon2, Params, Version};
    let parts: Vec<&str> = encoded.splitn(2, '$').collect();
    if parts.len() != 2 {
        return false;
    }
    use base64::Engine;
    let salt = match base64::engine::general_purpose::STANDARD.decode(parts[0]) {
        Ok(s) if s.len() == ARGON2_SALT_LEN => s,
        _ => return false,
    };
    let expected = match base64::engine::general_purpose::STANDARD.decode(parts[1]) {
        Ok(h) if h.len() == ARGON2_OUTPUT_LEN => h,
        _ => return false,
    };
    let params = Params::new(65536, 3, 4, Some(ARGON2_OUTPUT_LEN)).expect("Argon2 params");
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);
    let mut out = [0u8; ARGON2_OUTPUT_LEN];
    if argon2
        .hash_password_into(password.as_bytes(), &salt, &mut out)
        .is_err()
    {
        return false;
    }
    subtle::ConstantTimeEq::ct_eq(out.as_slice(), expected.as_slice()).into()
}

/// Hache un mot de passe avec SHA-256 (compatibilité legacy uniquement).
///
/// Utilisé uniquement pour la vérification des anciens hashes ; les nouveaux
/// comptes utilisent [`hash_password`] (Argon2id).
#[deprecated(
    since = "0.1.0",
    note = "Utiliser hash_password (Argon2id) pour les nouveaux comptes."
)]
pub fn hash_password_sha256(password: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    format!("{:x}", hasher.finalize())
}

// ═══════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_error_display() {
        let err = DbError::new("TestService", "something went wrong");
        assert_eq!(err.to_string(), "TestService DB: something went wrong");
    }

    #[test]
    fn test_int_to_bool() {
        assert!(!int_to_bool(0));
        assert!(int_to_bool(1));
        assert!(int_to_bool(42));
        assert!(int_to_bool(-1));
    }

    #[test]
    fn test_int_to_bool_opt() {
        assert_eq!(int_to_bool_opt(None), None);
        assert_eq!(int_to_bool_opt(Some(0)), Some(false));
        assert_eq!(int_to_bool_opt(Some(1)), Some(true));
    }

    #[test]
    fn test_bool_to_int() {
        assert_eq!(bool_to_int(false), 0);
        assert_eq!(bool_to_int(true), 1);
    }

    #[test]
    fn test_ensure_uuid() {
        let existing = "existing-id".to_string();
        assert_eq!(ensure_uuid(Some(existing.clone())), existing);

        let generated = ensure_uuid(None);
        assert!(!generated.is_empty());
        assert!(generated.contains('-')); // UUID format
    }

    #[test]
    fn test_now_timestamps() {
        let rfc = now_rfc3339();
        assert!(rfc.contains('T'));
        assert!(rfc.contains('+') || rfc.contains('Z'));

        let local = now_local_iso();
        assert!(local.contains('T'));

        let month = current_month_key();
        assert_eq!(month.len(), 7); // "YYYY-MM"
        assert!(month.contains('-'));
    }

    #[test]
    #[allow(deprecated)]
    fn test_hash_password_sha256() {
        let hash = hash_password_sha256("test123");
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
        assert_eq!(hash, hash_password_sha256("test123"));
    }

    #[test]
    fn test_is_legacy_hash() {
        assert!(is_legacy_password_hash("a".repeat(64).as_str()));
        assert!(!is_legacy_password_hash("argon2id$xxx$yyy"));
        assert!(!is_legacy_password_hash("short"));
    }

    #[test]
    fn test_hash_and_verify_argon2() {
        let pw = "SecureP4ss!";
        let stored = hash_password(pw);
        assert!(stored.starts_with(ARGON2_PREFIX));
        assert!(verify_password(&stored, pw));
        assert!(!verify_password(&stored, "wrong"));
    }

    #[test]
    #[allow(deprecated)]
    fn test_verify_legacy() {
        let pw = "test123";
        let legacy = hash_password_sha256(pw);
        assert!(verify_password(&legacy, pw));
        assert!(!verify_password(&legacy, "other"));
    }
}
