//! @id     jaymanga_services_mod
//! @do     declarer_module_jaykonta
//! @role   orchestration
//! @layer  core
//! @human  jaymanga — module « services mod ».

//! Adaptateurs inter-services JayManga.
//!
//! Intégration MWS (présence, manifestes, découverte),
//! JayKonta (export transactions → comptabilité).

pub mod jaykonta;
pub mod mws;
