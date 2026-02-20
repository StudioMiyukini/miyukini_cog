//! Trait Plugin — enregistrement composants et systèmes.
//!
//! @id mge.core.plugin
//! @role simulation
//! @layer core
//! @human Contrat d'extension par plugin
//! @do define_plugin_trait

use crate::engine::Engine;

/// Un plugin enregistre ses composants et systèmes lors du build.
pub trait Plugin {
    fn name(&self) -> &str;
    fn build(&self, engine: &mut Engine);
    fn dependencies(&self) -> &[&str] {
        &[]
    }
}
