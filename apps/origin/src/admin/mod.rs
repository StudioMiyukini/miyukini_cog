//! @id     origin_admin_mod
//! @do     declarer_module_server
//! @role   orchestration
//! @layer  app
//! @human  origin — module « admin mod ».

//! MiyukiniAdmin Origin — Panneau d'administration.
//!
//! Interface d'administration pour Origin sur le port 8081.

pub mod acces;
pub mod server;

pub use server::AdminServer;
