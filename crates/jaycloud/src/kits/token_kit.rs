//! @id     jaycloud_kits_token_kit
//! @do     definir_token_prefix
//! @role   securite
//! @layer  core
//! @human  jaycloud — module « kits token kit ».

//! `token_kit` — jetons applicatifs WebDAV.
//!
//! Format :
//! ```text
//! mws-jc-<base32(16 random bytes)>-<base32(8 hmac bytes)>
//! ```
//!
//! - 16 bytes aléatoires = 128 bits d'entropie (≈ 26 caractères base32).
//! - 8 bytes HMAC-SHA256(secret, random) = checksum d'intégrité (≈ 13 chars).
//! - Le token brut n'est **jamais stocké** côté JayCloud — seul son
//!   `SHA-256(token)` l'est, comme pour les sessions.
//!
//! Conforme DT-06 de la Spec MSCM/MIP (jetons applicatifs scopés et
//! révocables, jamais en clair côté serveur).

use base32::Alphabet;
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;

use crate::kits::crypto_kit::Key32;

const TOKEN_PREFIX: &str = "mws-jc-";
const RANDOM_BYTES: usize = 16;
const CHECKSUM_BYTES: usize = 8;
const BASE32_ALPHABET: Alphabet = Alphabet::Rfc4648Lower { padding: false };

type HmacSha256 = Hmac<Sha256>;

/// Erreurs du Kit token.
#[derive(Debug, thiserror::Error)]
pub enum TokenKitError {
    /// Format de token invalide (préfixe / structure).
    #[error("format de token invalide : {0}")]
    InvalidFormat(String),
    /// Échec de décodage base32.
    #[error("décodage base32 : {0}")]
    Decode(String),
    /// Checksum HMAC invalide → token forgé ou tronqué.
    #[error("checksum invalide : token forgé ou modifié")]
    InvalidChecksum,
}

/// Génère un nouveau jeton applicatif. Renvoie le token brut (à afficher
/// **une seule fois** à l'utilisateur) et son hash SHA-256 (à stocker en DB).
///
/// `signing_secret` est une `Key32` propre à JayCloud (typiquement dérivée
/// via `crypto_kit::derive_key_hkdf` depuis la master KindMother avec le
/// contexte `b"jaycloud_token_kit_v1"`).
pub fn generate(signing_secret: &Key32) -> (String, String) {
    let mut random = [0u8; RANDOM_BYTES];
    rand::rngs::OsRng.fill_bytes(&mut random);

    let checksum = hmac8(signing_secret, &random);

    let random_part = base32::encode(BASE32_ALPHABET, &random);
    let checksum_part = base32::encode(BASE32_ALPHABET, &checksum);
    let raw_token = format!("{TOKEN_PREFIX}{random_part}-{checksum_part}");

    let hash = hash_token(&raw_token);
    (raw_token, hash)
}

/// Hash SHA-256 hex d'un token brut (lowercase, 64 chars).
///
/// Utilisé pour stocker en DB sans révéler le token.
#[must_use]
pub fn hash_token(raw_token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(raw_token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Vérifie qu'un token correspond à un hash attendu (constant-time).
#[must_use]
pub fn verify_hash(raw_token: &str, expected_hash: &str) -> bool {
    let actual = hash_token(raw_token);
    // Comparaison constant-time pour éviter timing attacks.
    actual.as_bytes().ct_eq(expected_hash.as_bytes()).into()
}

/// Vérifie l'intégrité interne d'un token (préfixe + checksum HMAC).
///
/// Cette vérification permet de **rejeter immédiatement** les tokens
/// malformés ou forgés sans même consulter la DB.
pub fn verify_integrity(raw_token: &str, signing_secret: &Key32) -> Result<(), TokenKitError> {
    let body = raw_token
        .strip_prefix(TOKEN_PREFIX)
        .ok_or_else(|| TokenKitError::InvalidFormat(format!("préfixe attendu : '{TOKEN_PREFIX}'")))?;

    let (random_part, checksum_part) = body.split_once('-').ok_or_else(|| {
        TokenKitError::InvalidFormat("séparateur '-' manquant entre random et checksum".into())
    })?;

    let random = base32::decode(BASE32_ALPHABET, random_part)
        .ok_or_else(|| TokenKitError::Decode("partie random invalide".into()))?;
    if random.len() != RANDOM_BYTES {
        return Err(TokenKitError::InvalidFormat(format!(
            "partie random : {} octets attendus, reçu {}",
            RANDOM_BYTES,
            random.len()
        )));
    }

    let checksum_given = base32::decode(BASE32_ALPHABET, checksum_part)
        .ok_or_else(|| TokenKitError::Decode("partie checksum invalide".into()))?;
    if checksum_given.len() != CHECKSUM_BYTES {
        return Err(TokenKitError::InvalidFormat(format!(
            "checksum : {} octets attendus, reçu {}",
            CHECKSUM_BYTES,
            checksum_given.len()
        )));
    }

    // Vérification constant-time des 8 premiers octets du tag (crate `hmac`).
    hmac_sha256(signing_secret, &random)
        .verify_truncated_left(&checksum_given)
        .map_err(|_| TokenKitError::InvalidChecksum)
}

/// HMAC-SHA256 (crate `hmac`, RFC 2104) tronqué à 8 octets, utilisé comme checksum.
///
/// Sortie identique bit à bit à l'ancienne implémentation manuelle (clé de
/// 32 octets < bloc de 64 octets) : les jetons déjà émis restent valides.
fn hmac8(secret: &Key32, message: &[u8]) -> [u8; CHECKSUM_BYTES] {
    let tag = hmac_sha256(secret, message).finalize().into_bytes();
    let mut out = [0u8; CHECKSUM_BYTES];
    out.copy_from_slice(&tag[..CHECKSUM_BYTES]);
    out
}

/// Instancie un HMAC-SHA256 initialisé sur `message`.
fn hmac_sha256(secret: &Key32, message: &[u8]) -> HmacSha256 {
    // RFC 2104 : une clé plus courte que le bloc (64 octets) est complétée
    // par des zéros. Construire le bloc nous-mêmes donne le même tag que
    // `new_from_slice` tout en restant infaillible (C-10, aucun `expect`).
    let mut bloc = [0u8; 64];
    for (dst, src) in bloc.iter_mut().zip(secret.0.iter()) {
        *dst = *src;
    }
    let mut mac = <HmacSha256 as Mac>::new(&bloc.into());
    bloc.fill(0);
    mac.update(message);
    mac
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_format() {
        let secret = Key32::generate();
        let (token, _) = generate(&secret);
        assert!(token.starts_with(TOKEN_PREFIX));
        let body = token.strip_prefix(TOKEN_PREFIX).unwrap();
        assert!(body.contains('-'));
    }

    #[test]
    fn hash_is_64_hex_chars() {
        let secret = Key32::generate();
        let (token, hash) = generate(&secret);
        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
        // Le hash recalculé est identique.
        assert_eq!(hash_token(&token), hash);
    }

    #[test]
    fn two_generations_differ() {
        let secret = Key32::generate();
        let (t1, h1) = generate(&secret);
        let (t2, h2) = generate(&secret);
        assert_ne!(t1, t2);
        assert_ne!(h1, h2);
    }

    #[test]
    fn verify_hash_succeeds() {
        let secret = Key32::generate();
        let (token, hash) = generate(&secret);
        assert!(verify_hash(&token, &hash));
    }

    #[test]
    fn verify_hash_fails_on_modified_token() {
        let secret = Key32::generate();
        let (token, hash) = generate(&secret);
        let modified = format!("{token}x");
        assert!(!verify_hash(&modified, &hash));
    }

    #[test]
    fn integrity_valid_token_passes() {
        let secret = Key32::generate();
        let (token, _) = generate(&secret);
        assert!(verify_integrity(&token, &secret).is_ok());
    }

    #[test]
    fn integrity_wrong_prefix_fails() {
        let secret = Key32::generate();
        let r = verify_integrity("nope-abc-def", &secret);
        assert!(matches!(r, Err(TokenKitError::InvalidFormat(_))));
    }

    #[test]
    fn integrity_missing_separator_fails() {
        let secret = Key32::generate();
        let r = verify_integrity("mws-jc-abcdef", &secret);
        assert!(matches!(r, Err(TokenKitError::InvalidFormat(_))));
    }

    #[test]
    fn integrity_wrong_secret_fails() {
        let s1 = Key32::generate();
        let s2 = Key32::generate();
        let (token, _) = generate(&s1);
        let r = verify_integrity(&token, &s2);
        assert!(matches!(r, Err(TokenKitError::InvalidChecksum)));
    }

    #[test]
    fn integrity_tampered_random_fails() {
        let secret = Key32::generate();
        let (token, _) = generate(&secret);
        // Modifie un caractère de la partie random.
        let body = token.strip_prefix(TOKEN_PREFIX).unwrap();
        let (random, checksum) = body.split_once('-').unwrap();
        let mut chars: Vec<char> = random.chars().collect();
        chars[0] = if chars[0] == 'a' { 'b' } else { 'a' };
        let tampered: String = chars.into_iter().collect();
        let bad_token = format!("{TOKEN_PREFIX}{tampered}-{checksum}");
        let r = verify_integrity(&bad_token, &secret);
        assert!(matches!(r, Err(TokenKitError::InvalidChecksum)));
    }

    /// Référence : l'ancienne implémentation manuelle (ipad/opad), conservée
    /// uniquement en test pour prouver la compatibilité des jetons émis.
    fn legacy_hmac8(secret: &Key32, message: &[u8]) -> [u8; CHECKSUM_BYTES] {
        const BLOCK_SIZE: usize = 64;
        let mut key_padded = [0u8; BLOCK_SIZE];
        key_padded[..secret.0.len()].copy_from_slice(&secret.0);
        let ipad: Vec<u8> = key_padded.iter().map(|b| b ^ 0x36).collect();
        let opad: Vec<u8> = key_padded.iter().map(|b| b ^ 0x5c).collect();
        let mut inner = Sha256::new();
        inner.update(&ipad);
        inner.update(message);
        let mut outer = Sha256::new();
        outer.update(&opad);
        outer.update(inner.finalize());
        let mut out = [0u8; CHECKSUM_BYTES];
        out.copy_from_slice(&outer.finalize()[..CHECKSUM_BYTES]);
        out
    }

    #[test]
    fn hmac_crate_matches_legacy_checksum() {
        for _ in 0..16 {
            let secret = Key32::generate();
            let mut msg = [0u8; RANDOM_BYTES];
            rand::rngs::OsRng.fill_bytes(&mut msg);
            assert_eq!(hmac8(&secret, &msg), legacy_hmac8(&secret, &msg));
        }
    }

    #[test]
    fn legacy_token_still_verifies() {
        let secret = Key32::generate();
        let mut random = [0u8; RANDOM_BYTES];
        rand::rngs::OsRng.fill_bytes(&mut random);
        let token = format!(
            "{TOKEN_PREFIX}{}-{}",
            base32::encode(BASE32_ALPHABET, &random),
            base32::encode(BASE32_ALPHABET, &legacy_hmac8(&secret, &random))
        );
        assert!(verify_integrity(&token, &secret).is_ok());
    }

    #[test]
    fn hmac_rfc4231_case2_vector() {
        // RFC 4231 cas 2 : clé "Jefe", tag tronqué aux 8 premiers octets.
        let mut mac = <HmacSha256 as Mac>::new_from_slice(b"Jefe").unwrap();
        mac.update(b"what do ya want for nothing?");
        let tag = mac.finalize().into_bytes();
        assert_eq!(hex::encode(&tag[..8]), "5bdcc146bf60754e");
    }

    #[test]
    fn integrity_tampered_checksum_fails() {
        let secret = Key32::generate();
        let (token, _) = generate(&secret);
        let (head, checksum) = token.rsplit_once('-').unwrap();
        let mut bytes = base32::decode(BASE32_ALPHABET, checksum).unwrap();
        bytes[0] ^= 0x01;
        let bad = format!("{head}-{}", base32::encode(BASE32_ALPHABET, &bytes));
        assert!(matches!(
            verify_integrity(&bad, &secret),
            Err(TokenKitError::InvalidChecksum)
        ));
    }
}
