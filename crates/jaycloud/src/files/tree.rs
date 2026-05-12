//! Arborescence virtuelle des fichiers — chemins canoniques + anti path-traversal.
//!
//! Implémentation en PR-2 (P3.a). API attendue :
//! - `canonical_path(input) -> Result<CanonicalPath, JayCloudError>`
//!   (rejette `..`, normalise séparateurs, valide caractères autorisés)
//! - `join(base, sub) -> CanonicalPath`
//! - `parent(path) -> Option<CanonicalPath>`
