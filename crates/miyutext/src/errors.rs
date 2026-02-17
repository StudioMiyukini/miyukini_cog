//! Types d'erreur contractuels MiyuText (BOUND-*).

#[derive(Debug, Clone)]
pub enum MiyuTextError {
    NoMandate,
    Unimplemented,
    /// Regex non supporté en v0.1 (documenté).
    RegexNotSupportedV01,
}

impl std::fmt::Display for MiyuTextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuTextError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuTextError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyuTextError::RegexNotSupportedV01 => write!(f, "Regex replace not supported in v0.1; use Literal mode"),
        }
    }
}
impl std::error::Error for MiyuTextError {}
