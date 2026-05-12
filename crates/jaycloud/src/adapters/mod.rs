//! Couche 1 — Adaptateurs (protocoles exposés).
//!
//! - `webdav` : adaptateur WebDAV (RFC 4918) via `dav-server` (PR-4).
//! - `api`    : REST JSON interne consommé par Central et l'UI web (PR-3).
//! - `ui`     : pages HTML minimalistes askama + HTMX (PR-5).
//! - `auth_routes` : login portail + gestion app-passwords (PR-2 minimal, PR-5 complet).

pub mod api;
pub mod auth_routes;
pub mod ui;
pub mod webdav;
