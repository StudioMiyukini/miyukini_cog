//! Module Miou — Avatar et mascotte du COG.
//!
//! Miou est le canal privilégié par lequel le COG s'adresse à l'utilisateur :
//! accueil, suggestions, rappels, félicitations, notifications.
//!
//! Architecture :
//! - `context` : Construction du BotContext depuis les sources de données
//! - `engine` : Moteur de décision (quelle bulle afficher)
//! - `templates` : Banque de templates et injection de variables
//! - `state` : État Miou (préférences, session)
//! - `bubble` : Composant UI et file d'attente

pub mod context;
pub mod engine;
pub mod templates;
pub mod state;
pub mod bubble;

pub use context::BotContext;
pub use engine::decide;
pub use templates::select_variante;
pub use bubble::{BulleOutput, BulleAction, ActionType, MiouBubbleOverlay};
