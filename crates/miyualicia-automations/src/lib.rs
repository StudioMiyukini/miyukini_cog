//! # miyualicia-automations
//!
//! Moteur d'automatisations domotiques pour Alicia Home Assistante.
//! Gere le cycle complet : declenchement -> evaluation des conditions -> execution des actions.
//!
//! ## Declencheurs supportes
//!
//! - `Cron` : expression cron standard (ex: "0 22 * * *" = tous les soirs 22h00)
//! - `SensorChange` : changement d'etat d'un capteur au-dela d'un seuil
//! - `VoiceCommand` : routine vocale nommee ("bonne nuit", "je pars", etc.)
//! - `ApiEvent` : evenement declenche via `POST /automations/{id}/trigger`
//!
//! ## Loi d'Autonomie
//!
//! Ce crate fonctionne 100 % en local. Le scheduler tourne dans le runtime Tokio
//! de l'application sans connexion externe. Les automatisations sont stockees
//! dans KindMother (SQLite local).

#![forbid(unsafe_code)]

// @id: toolkit.alicia.automations
// @role: automation_engine
// @layer: 6
// @human: Moteur de regles et routines Alicia ; triggers, conditions, actions, scheduler.
// @do: schedule_and_execute_home_automations

pub mod admin_cell;
pub mod dispatcher;
pub mod engine;
pub mod errors;
pub mod evaluator;
pub mod executor;
pub mod parser;
pub mod types;

// Re-exports publics
pub use dispatcher::{AliciaCommandDispatcher, HomeSnapshot};
pub use engine::AutomationEngine;
pub use errors::AutomationError;
pub use evaluator::evaluate_conditions;
pub use parser::{parse_automation_json, parse_automation_toml};
pub use types::{Action, Automation, Condition, ConditionOp, TriggerType};
