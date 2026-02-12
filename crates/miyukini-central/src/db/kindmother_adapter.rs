//! Adaptateur KindMother - Utilitaires communs pour les services.
//!
//! Ce module fournit des fonctions utilitaires pures utilisables par tous les
//! services de l'écosystème Miyukini COG.
//!
//! Depuis la migration libSQL, l'accès base de données passe exclusivement
//! par `kindmother-client`. Ce module ne conserve que les helpers purs.
//!
//! @id: kindmother_adapter
//! @do: provide_common_utility_functions
//! @layer: infra

// ═══════════════════════════════════════════════════════════════════════════
// Helpers pour les conversions de types courants
// ═══════════════════════════════════════════════════════════════════════════

/// Convertit un `Option<i32>` en `Option<bool>`.
///
/// Convention: 0 = false, non-0 = true.
pub fn int_to_bool_opt(val: Option<i32>) -> Option<bool> {
    val.map(|x| x != 0)
}

/// Convertit un `i32` en `bool`.
///
/// Convention: 0 = false, non-0 = true.
pub fn int_to_bool(val: i32) -> bool {
    val != 0
}

/// Convertit un `bool` en `i32`.
///
/// Convention: false = 0, true = 1.
pub fn bool_to_int(val: bool) -> i32 {
    if val { 1 } else { 0 }
}

/// Convertit un `Option<bool>` en `Option<i32>`.
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
// Helpers pour le hachage de mots de passe
// ═══════════════════════════════════════════════════════════════════════════

/// Hache un mot de passe avec SHA-256 (pour compatibilité legacy).
///
/// Note: Pour la production, préférer bcrypt ou argon2.
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
    fn test_hash_password_sha256() {
        let hash = hash_password_sha256("test123");
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
        // Same input should produce same hash
        assert_eq!(hash, hash_password_sha256("test123"));
    }
}
