//! MiyukiniBB — Service forum (phpBB adapté pour Miyukini COG).
//!
//! Le **compte forum** est le **profil Central** : auth unifiée via l’API Origin.
//! Une copie des informations nécessaires (id, email, password_hash, pseudonyme)
//! est conservée côté serveur (Origin) pour la validation des connexions forum.
//!
//! Ce crate fournit le **client** pour synchroniser les profils Central vers
//! Origin. Le forum lui-même est hébergé sur Origin (phpBB + extension
//! StudioMiyukini Central Auth), accessible par exemple à `forum.miyukini.com`.

pub mod client;
pub mod errors;
pub mod types;

pub use client::MiyukiniBbClient;
pub use errors::MiyukiniBbError;
pub use types::ForumProfileSync;
