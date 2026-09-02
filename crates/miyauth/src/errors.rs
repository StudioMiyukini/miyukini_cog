//! @id     miyauth_errors
//! @do     definir_miyauth_error
//! @role   securite
//! @layer  core
//! @human  miyauth — module « errors ».

#[derive(Debug, Clone)]
pub enum MiyauthError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyauthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyauthError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyauthError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyauthError {}
