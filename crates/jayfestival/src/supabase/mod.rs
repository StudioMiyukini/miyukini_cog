//! Module Supabase : client REST + Auth et types alignés Reference Base de Donnees.

pub mod client;
pub mod types;

pub use client::SupabaseClient;
pub use types::{Edition, EditionExposant, Exposant, Organisateur, Profile, UserType};
