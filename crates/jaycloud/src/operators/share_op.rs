//! `share_op` — gestion des liens publics signés.
//!
//! Implémentation en PR-3 (P3.b) puis PR-6 (P5) pour les redirections legacy.
//! Responsabilités :
//! - `create_share(snapshot_id, file_path?, expires_in?, password?) -> ShareLink`
//! - `resolve_share(token) -> Option<SharedResource>`
//! - `resolve_legacy_miyucloud(legacy_token) -> Option<308Redirect>` (PR-6)
//! - Signature et vérification via KindMother
