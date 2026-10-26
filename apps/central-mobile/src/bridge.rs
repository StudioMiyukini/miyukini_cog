//! @id     central_mobile_bridge
//! @do     fournir_bridge
//! @role   orchestration
//! @layer  app
//! @human  central-mobile — module « bridge ».

//! Gestion du bridge COG Host — lifecycle et intégration Dioxus.

// Façade du module, pas encore consommée par les écrans.
#[allow(unused_imports)]
pub use miyukini_cog_bridge::{BridgeStatus, CogBridge, ConnectionMode};
