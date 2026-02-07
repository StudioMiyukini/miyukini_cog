//! Contexte gouverné pour l'exécution MiyuCalc.
//!
//! Mandat d'exécution, niveau de sécurité, pas d'identité Opérateur (BOUND-5).

/// @id: miyucalc_governed_context
/// @role: data
/// @layer: toolkit
/// @human: Contexte gouverné pour l'exécution MiyuCalc (mandat, niveau sécurité, pas d'identité Opérateur).
/// @do: represent_governed_context

/// @id: miyucalc_governed_context_struct
/// @role: data
/// @layer: toolkit
/// @human: Contexte d'exécution gouverné.
/// @do: hold_governed_context
#[derive(Debug, Clone)]
pub struct GovernedContext {
    /// Identifiant du mandat d'exécution.
    pub mandate_id: String,
    /// Niveau de sécurité (0-4, WorrySentinel).
    pub security_level: u8,
}

impl GovernedContext {
    /// Construit un contexte gouverné.
    #[must_use] 
    pub fn new(mandate_id: String, security_level: u8) -> Self {
        Self {
            mandate_id,
            security_level,
        }
    }

    /// Indique si le contexte a un mandat valide.
    #[must_use] 
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }
}
