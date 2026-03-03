//! Contexte gouverne MiyuCloud.
//!
//! @id: miyucloud_context
//! @do: provide_governed_context_for_miyucloud
//! @role: governance
//! @layer: infra
//!
//! Le `GovernedContext` centralise les references partagees entre les couches
//! du service : identifiant proprietaire, chemin de stockage, etc.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Contexte gouverne pour une session MiyuCloud.
///
/// Transporte les informations de contexte necessaires aux operations domaine
/// sans coupler les couches entre elles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernedContext {
    /// Identifiant du proprietaire courant (profil local).
    pub owner_id: String,
    /// Racine du stockage physique des fichiers chiffres.
    #[serde(default)]
    pub storage_root: PathBuf,
    /// Identifiant COG local.
    #[serde(default)]
    pub cog_id: String,
}

impl GovernedContext {
    /// Cree un nouveau contexte gouverne.
    pub fn new(owner_id: String, storage_root: PathBuf, cog_id: String) -> Self {
        Self {
            owner_id,
            storage_root,
            cog_id,
        }
    }
}

impl Default for GovernedContext {
    fn default() -> Self {
        Self {
            owner_id: String::new(),
            storage_root: PathBuf::new(),
            cog_id: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn governed_context_default() {
        let ctx = GovernedContext::default();
        assert!(ctx.owner_id.is_empty());
        assert!(ctx.storage_root.as_os_str().is_empty());
        assert!(ctx.cog_id.is_empty());
    }

    #[test]
    fn governed_context_new() {
        let ctx = GovernedContext::new(
            "owner-123".into(),
            PathBuf::from("/data/miyucloud"),
            "cog-abc".into(),
        );
        assert_eq!(ctx.owner_id, "owner-123");
        assert_eq!(ctx.storage_root, PathBuf::from("/data/miyucloud"));
        assert_eq!(ctx.cog_id, "cog-abc");
    }
}
