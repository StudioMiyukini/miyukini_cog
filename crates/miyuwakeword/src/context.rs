//! @id toolkit.alicia.wakeword.context
//! @do define_governed_context_for_wake_word
//! @role governance_context
//! @layer 6
//! @human Contexte de gouvernance standard pour le toolkit alicia-wakeword

/// Contexte de gouvernance pour les operations alicia-wakeword.
///
/// Chaque appel metier requiert un mandat valide (non vide)
/// et un niveau de securite minimum.
#[derive(Debug, Clone)]
pub struct GovernedContext {
    /// Identifiant du mandat de gouvernance actif.
    pub mandate_id: String,
    /// Niveau de securite requis (0 = aucun, 1+ = gouverne).
    pub security_level: u8,
}

impl GovernedContext {
    /// Cree un nouveau contexte de gouvernance.
    #[must_use]
    pub fn new(mandate_id: String, security_level: u8) -> Self {
        Self {
            mandate_id,
            security_level,
        }
    }

    /// Verifie si un mandat valide est present.
    #[must_use]
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }
}
