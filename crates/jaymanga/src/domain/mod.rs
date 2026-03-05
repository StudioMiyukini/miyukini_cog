//! Logique métier JayManga.
//!
//! Modules de domaine pour le catalogue, la liseuse, l'optimisation,
//! le paiement, les promotions, le téléchargement, les favoris,
//! la gamification et l'agrégation.

pub mod aggregator;
pub mod catalog;
pub mod download;
pub mod favorites;
pub mod gamification;
pub mod optimizer;
pub mod payment;
pub mod promotion;
pub mod reader;
