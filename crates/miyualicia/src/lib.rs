//! # miyualicia
//!
//! Orchestrateur central d'Alicia Home Assistante.
//! Connecte le registre de dispositifs, les transports MQTT/HTTP,
//! le pipeline NLU (miou-llm-bridge + fallback regex), le wake word,
//! et expose un snapshot complet de l'etat de la maison.
//!
//! ## Loi d'Autonomie
//!
//! L'orchestrateur fonctionne en mode degrade sans broker MQTT,
//! sans bridge NLU, et sans connexion Internet. Seul le registre
//! de dispositifs locaux est requis pour demarrer.

#![forbid(unsafe_code)]

// @id: service.alicia.orchestrator
// @role: central_orchestrator
// @layer: 7
// @human: Orchestrateur central Alicia ; devices, MQTT, HTTP, NLU, commandes, snapshot.
// @do: orchestrate_alicia_home_assistant

pub mod admin_cell;
pub mod command;
pub mod config;
pub mod errors;
pub mod intent;
pub mod nlu_bridge;
pub mod nlu_fallback;
pub mod service;
pub mod snapshot;

pub use config::AliciaConfig;
pub use errors::AliciaError;
pub use intent::Intent;
pub use service::AliciaService;
pub use snapshot::AliciaSnapshot;
