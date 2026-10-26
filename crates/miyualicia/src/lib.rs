//! # miyualicia
//!
//! Orchestrateur central d'Alicia Home Assistante.
//! Connecte le registre de dispositifs, les transports MQTT/HTTP,
//! le pipeline NLU (moteur local maia-llm à sortie contrainte, repli regex), le wake word,
//! et expose un snapshot complet de l'etat de la maison.
//!
//! ## Loi d'Autonomie
//!
//! L'orchestrateur fonctionne en mode degrade sans broker MQTT,
//! sans bridge NLU, et sans connexion Internet. Seul le registre
//! de dispositifs locaux est requis pour demarrer.

#![forbid(unsafe_code)]

// C-10 : aucun unwrap/expect/panic hors tests dans le code exposé au réseau.
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

// @id service.alicia.orchestrator
// @role central_orchestrator
// @layer 7
// @do orchestrate_alicia_home_assistant
// @human Orchestrateur central Alicia ; devices, MQTT, HTTP, NLU, commandes, snapshot.

pub mod admin_cell;
pub mod capabilities_runtime;
pub mod command;
pub mod config;
pub mod errors;
pub mod intent;
pub mod nlu_bridge;
pub mod nlu_fallback;
pub mod nlu_local;
pub mod service;
pub mod snapshot;

pub use capabilities_runtime::{CapabilitiesRuntime, CapabilitiesRuntimeConfig, PendingExpiryConfig};
pub use config::AliciaConfig;
pub use errors::AliciaError;
pub use intent::Intent;
pub use nlu_local::{PropositionIa, MENTION_IA_ALICIA, PRESENTATION_ALICIA};
pub use service::AliciaService;
pub use snapshot::AliciaSnapshot;
