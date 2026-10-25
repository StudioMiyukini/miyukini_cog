#![allow(missing_docs)]
//! # MiyuAntiSpam — toolkit.security.antispam
//!
//! Kit d'outils antispam (captcha, flood, rate_limit). Décision bloquer = StrongFather.
//! Alignement MIP : domaine `security`, layer tool/toolkit.

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

// @id toolkit.security.miyuantispam
/// @role infrastructure
/// @layer toolkit
/// @do expose_miyuantispam_toolkit
//@human / @human Point d'entrée du toolkit MiyuAntiSpam ; expose les modules tools.
pub mod admin_cell;
pub mod captcha;
pub mod context;
pub mod errors;
pub mod flood;
pub mod rate_limit;

pub use admin_cell::{
    miyuantispam_admin_cell, MiyuantispamAdminCell, MiyuantispamIdentification,
    MiyuantispamIntegrity, MiyuantispamTestManifest, TOOLKIT_ID,
};
pub use captcha::{generate as captcha_generate, verify as captcha_verify};
pub use context::GovernedContext;
pub use errors::MiyuantispamError;
pub use flood::{check as flood_check, FloodCheckResult};
pub use rate_limit::{
    check as rate_limit_check, RateLimitCheckResult, RateLimitPolicy, RateLimiter, MAX_KEY_BYTES,
};
