//! # miyualicia-api
//!
//! Serveur API REST pour Alicia Home Assistante.
//! Expose l'etat de la maison et les commandes domotiques via HTTP/JSON,
//! securise par JWT HS256 (cle locale, pas de cloud).
//!
//! ## Port par defaut
//!
//! Port 7890. Configurable dans `alicia.toml` section `[api]`.
//!
//! ## Authentication
//!
//! JWT HS256. Le token est obtenu via `POST /api/v1/alicia/auth/token`.
//! Duree de vie par defaut : 3600 secondes (1 heure).
//!
//! ## Loi d'Autonomie
//!
//! L'API est locale. Elle peut etre appelee depuis un serveur MWS distant
//! mais ne depend d'aucun service externe pour fonctionner.

#![forbid(unsafe_code)]

// @id: service.alicia.rest-api
// @role: http_api_gateway
// @layer: 7
// @human: Serveur REST JWT Alicia ; routes, auth, DTOs, handlers, rate limit.
// @do: expose_alicia_home_as_rest_api

pub mod admin_cell;
pub mod auth;
pub mod config;
pub mod dto;
pub mod errors;
pub mod handlers;
pub mod router;
pub mod server;

pub use config::ApiConfig;
pub use errors::ApiError;
pub use server::AliciaApiServer;
