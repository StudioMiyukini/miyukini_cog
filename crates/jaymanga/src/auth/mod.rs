//! Authentification et permissions JayManga.
//!
//! Gère les rôles (Admin/Vendeur, Lecteur authentifié, Visiteur),
//! la vérification des licences d'achat et le contrôle d'accès
//! aux pages payantes.

pub mod license_verify;
pub mod permissions;

pub use license_verify::*;
pub use permissions::*;
