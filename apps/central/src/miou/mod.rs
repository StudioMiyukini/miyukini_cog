//! @id     central_miou_mod
//! @do     declarer_module_bubble
//! @role   orchestration
//! @layer  app
//! @human  central — module « miou mod ».

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

pub mod bubble;
pub mod context;
pub mod engine;
pub mod state;
pub mod templates;

pub use bubble::{ActionType, BulleAction, BulleOutput, MiouBubbleOverlay};
pub use context::BotContext;
pub use engine::decide;
pub use templates::select_variante;
