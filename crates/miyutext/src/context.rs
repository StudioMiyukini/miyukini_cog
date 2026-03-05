//! Contexte gouverné pour l'exécution MiyuText (BOUND-5).

#[derive(Debug, Clone)]
pub struct GovernedContext {
    pub mandate_id: String,
    pub security_level: u8,
}

impl GovernedContext {
    #[must_use]
    pub fn new(mandate_id: String, security_level: u8) -> Self {
        Self {
            mandate_id,
            security_level,
        }
    }
    #[must_use]
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }
}
