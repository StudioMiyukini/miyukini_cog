//! Types d'erreur contractuels MiyuText (BOUND-*).

#[derive(Debug, Clone)]
pub enum MiyuTextError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuTextError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuTextError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuTextError {}
