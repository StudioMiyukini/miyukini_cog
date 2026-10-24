//! @id     miyauth_errors
//! @do     definir_miyauth_error
//! @role   securite
//! @layer  core
//! @human  miyauth — module « errors ».

#[derive(Debug, Clone)]
pub enum MiyauthError {
    NoMandate,
    Unimplemented,
    /// Fonction volontairement non implémentée (fail-closed, lot 2).
    NotImplemented(&'static str),
    /// Titre d'identité refusé (passeport, visa, révocation, possession).
    Refus(crate::passeport::RefusIdentite),
}

impl std::fmt::Display for MiyauthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyauthError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyauthError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyauthError::NotImplemented(what) => write!(f, "Not implemented: {what}"),
            MiyauthError::Refus(r) => write!(f, "Identity refused: {r}"),
        }
    }
}
impl std::error::Error for MiyauthError {}
