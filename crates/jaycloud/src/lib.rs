//! # jaycloud
//!
//! **Service de sauvegarde cloud souverain du COG.**
//!
//! Périmètre : backup chiffré de fichiers, snapshots complets +
//! incrémentaux, restore sélectif, liens publics signés, exposition
//! WebDAV (RFC 4918) pour outils tiers (rclone, restic, Duplicati,
//! Cyberduck).
//!
//! **JayCloud ne consomme aucun autre service Jay** (DT-04 de la
//! Spec MSCM/MIP). Il opère uniquement sur des fichiers et blobs qu'on
//! lui confie. Cohérent avec DS-03/DS-08 du Document Fondateur.
//!
//! Ce crate est en P2 (skeleton) : les modules existent mais leur
//! implémentation viendra aux PRs suivantes selon le plan de la Spec.

#![doc(html_root_url = "https://docs.miyukini.com/jaycloud")]
#![allow(dead_code)]

// @id: service.jaycloud
// @role: backup_storage_service
// @layer: 7
// @human: Service Miyukini de sauvegarde cloud souverain.
// @do: provide_cloud_backup_service

pub mod adapters;
pub mod config;
pub mod errors;
pub mod files;
pub mod kits;
pub mod operators;

pub use config::JayCloudConfig;
pub use errors::JayCloudError;
