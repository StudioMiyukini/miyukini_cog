//! Tokens CSRF pour les formulaires POST du COG Web Portal.
//!
//! @id: cog_web_portal_csrf @do: generate_validate_csrf_tokens
//! @role: security @layer: app
//! @human: CSRF Portal — génère et valide des tokens HMAC-SHA256 avec fenêtre de 10 minutes.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Clé secrète CSRF (injectée depuis l'environnement ou défaut de dev).
fn csrf_secret() -> Vec<u8> {
    std::env::var("PORTAL_CSRF_SECRET")
        .unwrap_or_else(|_| "dev-csrf-secret-changeme".to_string())
        .into_bytes()
}

/// Retourne le timestamp Unix en secondes.
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Génère un token CSRF valide pour le timestamp courant.
/// Format : `{timestamp_hex}.{hmac_hex}`
pub fn generate_csrf_token() -> String {
    let ts = now_secs();
    let ts_hex = format!("{ts:016x}");
    let mac = compute_mac(&ts_hex);
    format!("{ts_hex}.{mac}")
}

/// Valide un token CSRF. Retourne `true` si le token est valide et dans la fenêtre de 10 min.
pub fn validate_csrf_token(token: &str) -> bool {
    let parts: Vec<&str> = token.splitn(2, '.').collect();
    if parts.len() != 2 {
        return false;
    }
    let ts_hex = parts[0];
    let provided_mac = parts[1];

    // Vérification temporelle : fenêtre de 10 minutes.
    let ts = u64::from_str_radix(ts_hex, 16).unwrap_or(0);
    let now = now_secs();
    if now < ts || now - ts > 600 {
        return false;
    }

    // Vérification HMAC (constant-time via hmac::Mac::verify_slice).
    let expected_mac = compute_mac(ts_hex);
    constant_time_eq(provided_mac, &expected_mac)
}

fn compute_mac(data: &str) -> String {
    let mut mac =
        HmacSha256::new_from_slice(&csrf_secret()).expect("HMAC accepts any key length");
    mac.update(data.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let result = a
        .bytes()
        .zip(b.bytes())
        .fold(0u8, |acc, (x, y)| acc | (x ^ y));
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_roundtrip() {
        let token = generate_csrf_token();
        assert!(validate_csrf_token(&token), "token should be valid immediately");
    }

    #[test]
    fn tampered_token_rejected() {
        let token = generate_csrf_token();
        let tampered = token.replace('a', "b");
        // Only reject if the tamper actually changed something meaningful
        if tampered != token {
            assert!(
                !validate_csrf_token(&tampered),
                "tampered token should be rejected"
            );
        }
    }

    #[test]
    fn malformed_token_rejected() {
        assert!(!validate_csrf_token("notavalidtoken"));
        assert!(!validate_csrf_token(""));
        assert!(!validate_csrf_token("only.one"));
    }
}
