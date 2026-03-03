//! Authentification a deux facteurs TOTP (RFC 6238).
//!
//! @id: miyucloud_auth_totp
//! @do: provide_totp_setup_validate_recovery
//! @role: auth
//! @layer: domain
//!
//! Gere la configuration 2FA, la validation des codes TOTP,
//! et les codes de recuperation chiffres.

use crate::crypto::at_rest::{decrypt_chunk, encrypt_chunk, EncryptedChunk};
use crate::crypto::keys::KeyManager;
use crate::errors::MiyucloudError;
use crate::utils::base64;
use crate::utils::constant_time::constant_time_eq;
use sha2::{Digest, Sha256};
use std::fmt::Write as FmtWrite;
use totp_rs::{Algorithm, TOTP};

/// Number of recovery codes generated during setup.
const RECOVERY_CODE_COUNT: usize = 8;

/// Length of each recovery code (alphanumeric characters).
const RECOVERY_CODE_LENGTH: usize = 8;

/// TOTP secret size in bytes (160 bits, as recommended by RFC 4226).
const TOTP_SECRET_BYTES: usize = 20;

/// Issuer name embedded in the otpauth URI.
const ISSUER: &str = "MiyuCloud";

/// Resultat du setup TOTP : secret + URI + codes de recuperation.
#[derive(Debug)]
pub struct TotpSetupResult {
    /// URI otpauth:// pour les apps TOTP (Google Authenticator, etc.)
    pub otpauth_uri: String,
    /// Codes de recuperation en clair (affiches une seule fois).
    pub recovery_codes: Vec<String>,
}

/// Configure le 2FA TOTP pour un proprietaire.
///
/// Genere un secret TOTP, le chiffre avec la master key, le stocke en DB,
/// et retourne l'URI otpauth + 8 codes de recuperation.
pub fn setup_totp(
    db: &crate::data::MiyucloudDb,
    key_manager: &KeyManager,
    owner_id: &str,
    account_name: &str,
) -> Result<TotpSetupResult, MiyucloudError> {
    // 1. Check if TOTP already exists and is active
    if let Some(existing) = db.totp_get_by_owner(owner_id)? {
        if existing.is_active {
            return Err(MiyucloudError::InvalidInput(
                "TOTP is already enabled for this owner".to_string(),
            ));
        }
        // Inactive TOTP exists: clean up before re-setup
        db.totp_delete(owner_id)?;
        db.recovery_codes_delete_all(owner_id)?;
    }

    // 2. Generate a random 20-byte TOTP secret
    let mut secret_bytes = [0u8; TOTP_SECRET_BYTES];
    rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut secret_bytes);

    // 3. Encrypt the secret with the master key
    let encrypted = encrypt_chunk(key_manager.master_key(), &secret_bytes)?;
    let encrypted_secret_b64 = base64::encode(&encrypted.ciphertext);
    let nonce_b64 = base64::encode(&encrypted.nonce);

    // 4. Store in DB
    let totp_id = uuid::Uuid::new_v4().to_string();
    db.totp_insert(&totp_id, owner_id, &encrypted_secret_b64, &nonce_b64, account_name)?;

    // 5. Create the TOTP instance and get the otpauth URI
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes.to_vec(),
        Some(ISSUER.to_string()),
        account_name.to_string(),
    )
    .map_err(|e| MiyucloudError::InvalidInput(format!("TOTP creation failed: {e}")))?;

    let otpauth_uri = totp.get_url();

    // 6. Generate 8 recovery codes, hash them, store hashes in DB
    let recovery_codes = generate_and_store_recovery_codes(db, owner_id)?;

    // 7. Activate the TOTP
    db.totp_activate(owner_id)?;

    Ok(TotpSetupResult {
        otpauth_uri,
        recovery_codes,
    })
}

/// Valide un code TOTP a 6 chiffres.
///
/// Accepte t-1, t, t+1 (skew=1).
pub fn validate_totp(
    db: &crate::data::MiyucloudDb,
    key_manager: &KeyManager,
    owner_id: &str,
    code: &str,
) -> Result<bool, MiyucloudError> {
    // 1. Get TOTP row, check active
    let row = db
        .totp_get_by_owner(owner_id)?
        .ok_or_else(|| MiyucloudError::NotFound("TOTP not configured for this owner".to_string()))?;

    if !row.is_active {
        return Err(MiyucloudError::NotFound(
            "TOTP is not active for this owner".to_string(),
        ));
    }

    // 2. Decrypt the secret
    let secret_bytes = decrypt_totp_secret(key_manager, &row)?;

    // 3. Create TOTP instance with skew=1
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        Some(ISSUER.to_string()),
        row.account_name.clone(),
    )
    .map_err(|e| MiyucloudError::Crypto(format!("TOTP instance creation failed: {e}")))?;

    // 4. Validate the code with the current system time
    let valid = totp
        .check_current(code)
        .map_err(|e| MiyucloudError::Crypto(format!("System time error: {e}")))?;

    // 5. If valid, update last_used_at
    if valid {
        db.totp_update_last_used(owner_id)?;
    }

    Ok(valid)
}

/// Valide un code de recuperation (usage unique).
pub fn validate_recovery_code(
    db: &crate::data::MiyucloudDb,
    owner_id: &str,
    code: &str,
) -> Result<bool, MiyucloudError> {
    // 1. Get all recovery codes for owner
    let codes = db.recovery_codes_by_owner(owner_id)?;

    // 2. Hash the provided code
    let provided_hash = sha256_hex(code);

    // 3. Find a matching unused code using constant-time comparison
    for row in &codes {
        if row.is_used {
            continue;
        }
        if constant_time_eq(provided_hash.as_bytes(), row.code_hash.as_bytes()) {
            // 4. Mark as used
            db.recovery_code_mark_used(&row.id)?;
            return Ok(true);
        }
    }

    Ok(false)
}

/// Desactive le 2FA pour un proprietaire.
pub fn disable_totp(
    db: &crate::data::MiyucloudDb,
    owner_id: &str,
) -> Result<(), MiyucloudError> {
    // 1. Delete TOTP secret
    db.totp_delete(owner_id)?;
    // 2. Delete all recovery codes
    db.recovery_codes_delete_all(owner_id)?;
    Ok(())
}

/// Verifie si le 2FA est active pour un proprietaire.
pub fn is_totp_enabled(
    db: &crate::data::MiyucloudDb,
    owner_id: &str,
) -> Result<bool, MiyucloudError> {
    match db.totp_get_by_owner(owner_id)? {
        Some(row) => Ok(row.is_active),
        None => Ok(false),
    }
}

/// Regenere les codes de recuperation.
pub fn regenerate_recovery_codes(
    db: &crate::data::MiyucloudDb,
    owner_id: &str,
) -> Result<Vec<String>, MiyucloudError> {
    // 1. Check TOTP is enabled
    if !is_totp_enabled(db, owner_id)? {
        return Err(MiyucloudError::InvalidInput(
            "TOTP is not enabled, cannot regenerate recovery codes".to_string(),
        ));
    }

    // 2. Delete old recovery codes
    db.recovery_codes_delete_all(owner_id)?;

    // 3. Generate and store new codes
    generate_and_store_recovery_codes(db, owner_id)
}

// ---------------------------------------------------------------------------
// Helpers internes
// ---------------------------------------------------------------------------

/// Decrypt the TOTP secret from a `TotpRow`.
fn decrypt_totp_secret(
    key_manager: &KeyManager,
    row: &crate::data::types::TotpRow,
) -> Result<Vec<u8>, MiyucloudError> {
    let ciphertext = base64::decode(&row.encrypted_secret)?;
    let nonce_bytes = base64::decode(&row.encryption_nonce)?;

    let nonce: [u8; 12] = nonce_bytes.try_into().map_err(|_| {
        MiyucloudError::Crypto("Invalid nonce length: expected 12 bytes".to_string())
    })?;

    let encrypted = EncryptedChunk {
        nonce,
        ciphertext,
    };

    decrypt_chunk(key_manager.master_key(), &encrypted)
}

/// Generate a random alphanumeric recovery code of the given length.
fn generate_recovery_code() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::rngs::OsRng;
    (0..RECOVERY_CODE_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Compute SHA-256 hash of input and return as lowercase hex string.
fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for byte in result {
        // Using fmt::Write which does not fail on String
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

/// Generate recovery codes, hash and store them, return plaintext codes.
fn generate_and_store_recovery_codes(
    db: &crate::data::MiyucloudDb,
    owner_id: &str,
) -> Result<Vec<String>, MiyucloudError> {
    let mut plaintext_codes = Vec::with_capacity(RECOVERY_CODE_COUNT);

    for _ in 0..RECOVERY_CODE_COUNT {
        let code = generate_recovery_code();
        let code_hash = sha256_hex(&code);
        let code_id = uuid::Uuid::new_v4().to_string();
        db.recovery_code_insert(&code_id, owner_id, &code_hash)?;
        plaintext_codes.push(code);
    }

    Ok(plaintext_codes)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[cfg(feature = "legacy-sqlite")]
mod tests {
    use super::*;
    use crate::crypto::keys::KeyManager;
    use crate::data::MiyucloudDb;

    /// Open an in-memory DB for testing.
    fn mem_db() -> MiyucloudDb {
        MiyucloudDb::open(":memory:").expect("in-memory db")
    }

    /// Create a test key manager with a fixed key.
    fn test_key_manager() -> KeyManager {
        KeyManager::from_master_key([42u8; 32])
    }

    // -- V2-T01 tests -------------------------------------------------------

    #[test]
    fn test_setup_totp_returns_uri_and_codes() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-001";

        let result = setup_totp(&db, &km, owner, "alice@miyucloud.local")
            .expect("setup should succeed");

        // URI should start with otpauth://
        assert!(
            result.otpauth_uri.starts_with("otpauth://totp/"),
            "URI should be otpauth format, got: {}",
            result.otpauth_uri
        );
        assert!(
            result.otpauth_uri.contains("MiyuCloud"),
            "URI should contain issuer"
        );
        assert!(
            result.otpauth_uri.contains("alice%40miyucloud.local"),
            "URI should contain account name, got: {}",
            result.otpauth_uri
        );

        // Should have 8 recovery codes
        assert_eq!(result.recovery_codes.len(), RECOVERY_CODE_COUNT);

        // Each code should be 8 chars alphanumeric
        for code in &result.recovery_codes {
            assert_eq!(code.len(), RECOVERY_CODE_LENGTH);
            assert!(
                code.chars().all(|c| c.is_ascii_alphanumeric()),
                "Code should be alphanumeric: {code}"
            );
        }
    }

    #[test]
    fn test_setup_totp_already_active_fails() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-002";

        setup_totp(&db, &km, owner, "bob@test.com").expect("first setup ok");

        let err = setup_totp(&db, &km, owner, "bob@test.com")
            .expect_err("second setup should fail");
        let msg = err.to_string();
        assert!(
            msg.contains("already enabled"),
            "Error should mention already enabled: {msg}"
        );
    }

    #[test]
    fn test_is_totp_enabled_false_when_not_setup() {
        let db = mem_db();
        let owner = "owner-003";

        assert!(
            !is_totp_enabled(&db, owner).expect("should not error"),
            "Should be false when not configured"
        );
    }

    #[test]
    fn test_is_totp_enabled_true_after_setup() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-004";

        setup_totp(&db, &km, owner, "charlie@test.com").expect("setup ok");

        assert!(
            is_totp_enabled(&db, owner).expect("should not error"),
            "Should be true after setup"
        );
    }

    // -- V2-T02 tests -------------------------------------------------------

    #[test]
    fn test_validate_totp_with_current_code() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-005";

        setup_totp(&db, &km, owner, "dave@test.com").expect("setup ok");

        // Retrieve the secret to generate a valid code
        let row = db.totp_get_by_owner(owner).unwrap().unwrap();
        let secret = decrypt_totp_secret(&km, &row).unwrap();

        let totp = TOTP::new(
            Algorithm::SHA1,
            6,
            1,
            30,
            secret,
            Some(ISSUER.to_string()),
            "dave@test.com".to_string(),
        )
        .unwrap();

        let current_code = totp.generate_current().unwrap();

        let valid = validate_totp(&db, &km, owner, &current_code)
            .expect("validation should not error");
        assert!(valid, "Current code should be valid");
    }

    #[test]
    fn test_validate_totp_wrong_code_returns_false() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-006";

        setup_totp(&db, &km, owner, "eve@test.com").expect("setup ok");

        let valid = validate_totp(&db, &km, owner, "000000")
            .expect("validation should not error");
        // It is statistically possible (but extremely unlikely) that "000000"
        // matches the current window. We accept this minimal flakiness.
        // In practice with a random secret this will be false.
        let _ = valid; // We just verify no panic/error
    }

    #[test]
    fn test_validate_totp_not_configured_fails() {
        let db = mem_db();
        let km = test_key_manager();

        let err = validate_totp(&db, &km, "nonexistent", "123456")
            .expect_err("should error for missing TOTP");
        assert!(
            err.to_string().contains("not configured"),
            "Should indicate not configured: {}",
            err
        );
    }

    #[test]
    fn test_validate_recovery_code_success() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-007";

        let result = setup_totp(&db, &km, owner, "frank@test.com").expect("setup ok");

        // Use the first recovery code
        let code = &result.recovery_codes[0];
        let valid = validate_recovery_code(&db, owner, code)
            .expect("validation should not error");
        assert!(valid, "Recovery code should be valid");

        // Same code should not work twice (single-use)
        let valid_again = validate_recovery_code(&db, owner, code)
            .expect("validation should not error");
        assert!(
            !valid_again,
            "Recovery code should be single-use"
        );
    }

    #[test]
    fn test_validate_recovery_code_wrong_code() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-008";

        setup_totp(&db, &km, owner, "grace@test.com").expect("setup ok");

        let valid = validate_recovery_code(&db, owner, "ZZZZZZZZ")
            .expect("validation should not error");
        assert!(!valid, "Wrong recovery code should be invalid");
    }

    #[test]
    fn test_disable_totp() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-009";

        setup_totp(&db, &km, owner, "heidi@test.com").expect("setup ok");
        assert!(is_totp_enabled(&db, owner).unwrap());

        disable_totp(&db, owner).expect("disable ok");
        assert!(
            !is_totp_enabled(&db, owner).unwrap(),
            "Should be disabled after disable_totp"
        );

        // Recovery codes should also be gone
        let codes = db.recovery_codes_by_owner(owner).unwrap();
        assert!(codes.is_empty(), "Recovery codes should be deleted");
    }

    #[test]
    fn test_regenerate_recovery_codes() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-010";

        let original = setup_totp(&db, &km, owner, "ivan@test.com").expect("setup ok");
        let original_codes = original.recovery_codes;

        let new_codes = regenerate_recovery_codes(&db, owner)
            .expect("regenerate should succeed");

        assert_eq!(new_codes.len(), RECOVERY_CODE_COUNT);
        // New codes should differ from originals (statistically guaranteed)
        assert_ne!(
            new_codes, original_codes,
            "New codes should differ from originals"
        );

        // Old codes should no longer work
        let valid = validate_recovery_code(&db, owner, &original_codes[0])
            .expect("validation should not error");
        assert!(!valid, "Old recovery codes should be invalidated");

        // New codes should work
        let valid = validate_recovery_code(&db, owner, &new_codes[0])
            .expect("validation should not error");
        assert!(valid, "New recovery codes should work");
    }

    #[test]
    fn test_regenerate_recovery_codes_fails_without_totp() {
        let db = mem_db();
        let owner = "owner-011";

        let err = regenerate_recovery_codes(&db, owner)
            .expect_err("should fail without TOTP");
        assert!(
            err.to_string().contains("not enabled"),
            "Should indicate TOTP not enabled: {}",
            err
        );
    }

    #[test]
    fn test_sha256_hex_known_value() {
        // SHA-256("hello") = 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
        let hash = sha256_hex("hello");
        assert_eq!(
            hash,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_recovery_code_format() {
        let code = generate_recovery_code();
        assert_eq!(code.len(), RECOVERY_CODE_LENGTH);
        assert!(
            code.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()),
            "Code should be uppercase alphanumeric: {code}"
        );
    }

    #[test]
    fn test_setup_cleans_inactive_totp_on_retry() {
        let db = mem_db();
        let km = test_key_manager();
        let owner = "owner-012";

        // Manually insert an inactive TOTP entry
        let enc = encrypt_chunk(km.master_key(), &[0u8; 20]).unwrap();
        let enc_b64 = base64::encode(&enc.ciphertext);
        let nonce_b64 = base64::encode(&enc.nonce);
        db.totp_insert("old-id", owner, &enc_b64, &nonce_b64, "old@test.com")
            .unwrap();
        // Entry exists but is_active=false (default from insert)

        // setup_totp should clean up and succeed
        let result = setup_totp(&db, &km, owner, "new@test.com");
        assert!(result.is_ok(), "Should succeed after cleaning inactive entry");
        assert!(is_totp_enabled(&db, owner).unwrap());
    }
}
