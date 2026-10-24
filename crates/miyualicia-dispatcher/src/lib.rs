//! # miyualicia-dispatcher
//!
//! Orchestrateur d'exécution des skills d'Alicia.
//!
//! Le dispatcher fait le pont entre :
//! - le **registre des skills** (`miyualicia-skills::SkillRegistry`)
//! - la **politique d'exécution** (`miyualicia-skills::AliciaPolicy`)
//! - la **gouvernance cores** (trait [`CoreGate`])
//! - les **services exécutants** (trait [`ServiceClient`])
//! - la **mémoire d'audit** (`miyualicia-memory::MemoryStore`)
//!
//! Pour P2, le dispatcher est l'intermédiaire **obligatoire** entre une
//! tool call reçue du LLM et l'exécution effective. Il applique :
//! 1. la résolution dans le registre
//! 2. la gate cores (sauf bypass explicite via `AliciaPolicy`)
//! 3. la confirmation utilisateur (déléguée au caller via [`Decision`])
//! 4. l'exécution via le `ServiceClient`
//! 5. l'enregistrement épisode dans `MemoryStore` (toujours, même bypass)
//!
//! Les implémentations réelles de `CoreGate` (vraie traversée BorderGuard /
//! StrongFather) et `ServiceClient` (clients HTTP des services COG) seront
//! branchées en **P3+**. P2 fournit des mocks et un `BypassCoreGate`.
//!
//! ## Lois d'autonomie
//!
//! - Aucune dépendance réseau imposée — les traits sont génériques
//! - Le bypass cores est explicite, jamais silencieux
//! - L'audit est posé même quand toutes les gates sont bypassées

#![doc(html_root_url = "https://docs.miyukini.com/miyualicia-dispatcher")]

// @id service.alicia.dispatcher.lib
// @role alicia_dispatcher_facade
// @layer 7
// @do expose_dispatcher
// @human Façade publique du dispatcher Alicia.

pub mod core_gate;
pub mod dispatcher;
pub mod errors;
pub mod service_client;

pub use core_gate::{BypassCoreGate, CoreGate, CoreGateOutcome};
pub use dispatcher::{Decision, Dispatcher, ExecutionResult, ResolvedLearnedCall};
pub use errors::DispatchError;
pub use service_client::{
    MockServiceClient, ServiceCallResult, ServiceClient, ServiceClientRouter,
};
