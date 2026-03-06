//! E9-04 (partiel) — Tests E2E WOPI: token round-trip et validation.
//!
//! Teste le cycle complet de generation/validation de tokens WOPI
//! et verifie les cas d'erreur.

use miyucloud_dav::wopi::auth::WopiTokenValidator;

#[test]
fn token_round_trip_multiple_files() {
    let validator = WopiTokenValidator::new(b"integration-test-secret");

    // Generer des tokens pour differents fichiers
    let token1 = validator.generate_token("file-001", "owner-alice");
    let token2 = validator.generate_token("file-002", "owner-bob");
    let token3 = validator.generate_token("file-003", "owner-alice");

    // Valider chaque token
    let (fid1, oid1) = validator.validate_token(&token1).unwrap();
    assert_eq!(fid1, "file-001");
    assert_eq!(oid1, "owner-alice");

    let (fid2, oid2) = validator.validate_token(&token2).unwrap();
    assert_eq!(fid2, "file-002");
    assert_eq!(oid2, "owner-bob");

    let (fid3, oid3) = validator.validate_token(&token3).unwrap();
    assert_eq!(fid3, "file-003");
    assert_eq!(oid3, "owner-alice");
}

#[test]
fn different_secrets_reject_each_other() {
    let v1 = WopiTokenValidator::new(b"secret-A");
    let v2 = WopiTokenValidator::new(b"secret-B");

    let token = v1.generate_token("file-x", "owner-y");
    assert!(v2.validate_token(&token).is_err());
}

#[test]
fn rejects_empty_and_garbage_tokens() {
    let validator = WopiTokenValidator::new(b"test-secret");

    assert!(validator.validate_token("").is_err());
    assert!(validator.validate_token("not:a:valid:token").is_err());
    assert!(validator.validate_token("a:b:0:invalid_sig").is_err());
    assert!(validator.validate_token(":::").is_err());
}

#[test]
fn rejects_modified_file_id() {
    let validator = WopiTokenValidator::new(b"test-secret");
    let token = validator.generate_token("file-real", "owner-1");

    // Remplacer file_id dans le token
    let tampered = token.replacen("file-real", "file-evil", 1);
    assert!(validator.validate_token(&tampered).is_err());
}

#[test]
fn rejects_modified_owner_id() {
    let validator = WopiTokenValidator::new(b"test-secret");
    let token = validator.generate_token("file-1", "owner-legit");

    // Remplacer owner_id
    let tampered = token.replacen("owner-legit", "owner-evil", 1);
    assert!(validator.validate_token(&tampered).is_err());
}
