//! @id     jaymanga_services_mws_mod
//! @do     declarer_module_discovery
//! @role   orchestration
//! @layer  core
//! @human  jaymanga — module « services mws mod ».

//! Intégration MWS (Miyukini Webway System) pour JayManga.
//!
//! Requêtes de présence, publication de manifestes,
//! et découverte de COGs JayManga via les trackers.

pub mod discovery;
pub mod manifests;
pub mod presence;
