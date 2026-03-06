//! E9-01 / E10-01 -- Tests securite: path traversal WebDAV.
//!
//! Verifie que le validateur de chemins rejette toutes les formes
//! d'attaque par traversee de repertoire.

use miyucloud_dav::common::path_validator::validate_dav_path;

// ════════════════════════════════════════════════════════════════════════════
// Path traversal basique
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn rejects_dot_dot_slash() {
    assert!(validate_dav_path("/../etc/passwd").is_err());
    assert!(validate_dav_path("/foo/../../etc/shadow").is_err());
    assert!(validate_dav_path("/../../../").is_err());
}

#[test]
fn rejects_backslash_traversal() {
    assert!(validate_dav_path("/foo\\..\\bar").is_err());
    assert!(validate_dav_path("\\..\\windows\\system32").is_err());
}

// ════════════════════════════════════════════════════════════════════════════
// Encodages URL
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn rejects_url_encoded_dot_dot() {
    // %2e = .  =>  ..
    assert!(validate_dav_path("/%2e%2e/etc/passwd").is_err());
    assert!(validate_dav_path("/%2E%2E/etc/passwd").is_err());
}

// ════════════════════════════════════════════════════════════════════════════
// Null bytes
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn rejects_null_bytes() {
    assert!(validate_dav_path("/foo\0/bar").is_err());
}

// ════════════════════════════════════════════════════════════════════════════
// Chemins valides
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn accepts_valid_paths() {
    assert!(validate_dav_path("/").is_ok());
    assert!(validate_dav_path("/documents").is_ok());
    assert!(validate_dav_path("/documents/photos/vacation.jpg").is_ok());
    assert!(validate_dav_path("/mon dossier/fichier.txt").is_ok());
}

#[test]
fn accepts_paths_with_dots_in_names() {
    assert!(validate_dav_path("/file.txt").is_ok());
    assert!(validate_dav_path("/archive.tar.gz").is_ok());
    assert!(validate_dav_path("/.hidden").is_ok());
}

// ════════════════════════════════════════════════════════════════════════════
// Normalisation
// ════════════════════════════════════════════════════════════════════════════

#[test]
fn normalizes_trailing_slash() {
    let result = validate_dav_path("/documents/").unwrap();
    // Le chemin normalise doit etre coherent
    assert!(result.starts_with('/'));
}

#[test]
fn normalizes_double_slashes() {
    let result = validate_dav_path("/documents//photos///file.txt");
    // Double slashes should be handled gracefully
    assert!(result.is_ok());
}
