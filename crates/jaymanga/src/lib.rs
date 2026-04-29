#![allow(missing_docs)]
//! # JayManga — service.media.jaymanga
//!
//! Service de lecture et vente de manga en ligne sous gouvernance COG.
//!
//! JayManga permet à tout propriétaire de COG de publier sa collection de manga,
//! de la proposer en lecture (gratuite ou payante) sur la surface web de son COG,
//! et de gérer les ventes depuis son environnement Host.
//!
//! ## Position architecturale
//!
//! - **Strate** : 7 (Opérateur)
//! - **Type** : Service de Type 2 (surface web externe) avec composante Type 1
//!   (gestion admin dans Central) et Type 3 (Portail Agrégé inter-COG).
//! - **Cores** : KindMother (persistance), StrongFather (gouvernance publication),
//!   MasterButler (permissions lecture/téléchargement), WorrySentinel (sécurité paiement),
//!   BorderGuard (frontières contenu).
//!
//! ## Feature flags
//!
//! - `legacy-sqlite` (défaut) : persistance via rusqlite locale.
//! - `kindmother-only` : persistance via KindMother Client (IPC/gRPC).
//! - `db-encryption` : chiffrement de la base via `kindmother-db-key`.
//!
//! @id service.media.jaymanga
//! @role media_distribution
//! @layer domain
//! @human Service de lecture et vente de manga en ligne
//! @do manage_manga_catalog_reading_sales

/// Couche persistance (types de domaine, base de données, feature flags).
pub mod data;

/// Authentification et permissions (vendeur, lecteur, visiteur).
pub mod auth;

/// Logique métier (catalogue, lecteur, optimisation, paiement, favoris,
/// gamification, agrégation).
pub mod domain;

/// Adaptateurs inter-services (MWS, JayKonta).
pub mod services;

/// Endpoints REST (surface web / Portail).
pub mod api;

/// Exports (CSV ventes, rapports PDF).
pub mod export;

/// Templates et assets pour le Portail web.
pub mod web;

// Réexport pour les consommateurs (Origin, Central).
pub use data::JayMangaDb;
