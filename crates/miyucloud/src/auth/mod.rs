//! Module d'authentification et de permissions MiyuCloud.
//!
//! @id: miyucloud_auth_module
//! @do: expose_permission_verification
//! @role: auth
//! @layer: domain

pub mod permissions;
pub mod sessions;
pub mod totp;
pub mod web_tokens;

pub use permissions::verify_access;
pub use web_tokens::{generate_share_token, hash_password, validate_share_link, verify_password};
