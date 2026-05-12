//! `auth_op` — sessions web et jetons applicatifs.
//!
//! Implémentation en PR-2 (P3.a) minimal, puis PR-5 (P4) complet.
//! Responsabilités :
//! - `verify_login(credentials) -> Session` (délégation KindMother)
//! - `create_app_password(name, scopes) -> (id, raw_token)`
//! - `verify_app_password(token) -> Option<AppPasswordContext>`
//! - `revoke_app_password(id)` (effet immédiat)
//! - `list_app_passwords(user_id) -> Vec<AppPasswordSummary>`
