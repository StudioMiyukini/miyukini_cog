//! Adaptateurs inter-services JayManga.
//!
//! Intégration MWS (présence, manifestes, découverte),
//! JayXpose (synchronisation catalogue → vitrine),
//! JayKonta (export transactions → comptabilité).

pub mod mws;
pub mod jayxpose;
pub mod jaykonta;
