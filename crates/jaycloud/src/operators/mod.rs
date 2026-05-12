//! Couche 2 — Opérateurs (logique métier).
//!
//! - `files_op`     : storage chiffré, héritier MiyuCloud (PR-2).
//! - `snapshots_op` : création + politique de rétention + listing (PR-3).
//! - `restore_op`   : récupération sélective (PR-3).
//! - `share_op`     : liens publics signés + redirections legacy (PR-3 + PR-6).
//! - `auth_op`      : sessions + jetons applicatifs (PR-2 minimal).

pub mod auth_op;
pub mod files_op;
pub mod restore_op;
pub mod share_op;
pub mod snapshots_op;
