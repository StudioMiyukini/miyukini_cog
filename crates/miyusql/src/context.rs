//! Contexte gouverné pour l'exécution MiyuSQL.
//!
//! Mandat d'exécution, niveau de sécurité, pas d'identité Opérateur (BOUND-5).

// @id: miyusql_governed_context
// @role: data
// @layer: toolkit
// @human: Contexte gouverné pour l'exécution MiyuSQL (mandat, niveau sécurité, pas d'identité Opérateur).
// @do: represent_governed_context
// Contexte fourni par la gouvernance ; MiyuSQL ne connaît pas l'Opérateur appelant.

/// @id: miyusql_governed_context_struct
/// @role: data
/// @layer: toolkit
/// @human: Contexte d'exécution gouverné.
/// @do: hold_governed_context
/// @depends: miyusql_governed_context
#[derive(Debug, Clone)]
pub struct GovernedContext {
    /// @id: miyusql_context_mandate_id
    /// @role: data
    /// @layer: toolkit
    /// @human: Identifiant du mandat d'exécution.
    /// @do: store_mandate_id
    /// @depends: miyusql_governed_context_struct
    pub mandate_id: String,

    /// @id: miyusql_context_security_level
    /// @role: data
    /// @layer: toolkit
    /// @human: Niveau de sécurité (0-4, WorrySentinel).
    /// @do: store_security_level
    /// @depends: miyusql_governed_context_struct
    pub security_level: u8,
}

impl GovernedContext {
    /// @id: miyusql_context_new
    /// @role: mutator
    /// @layer: toolkit
    /// @human: Construit un contexte gouverné.
    /// @do: build_governed_context
    /// @depends: miyusql_governed_context_struct
    #[must_use]
    pub fn new(mandate_id: String, security_level: u8) -> Self {
        Self {
            mandate_id,
            security_level,
        }
    }

    /// @id: miyusql_context_has_mandate
    /// @role: accessor
    /// @layer: toolkit
    /// @human: Indique si le contexte a un mandat valide.
    /// @do: check_mandate_valid
    /// @depends: miyusql_governed_context_struct
    #[must_use]
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }
}
