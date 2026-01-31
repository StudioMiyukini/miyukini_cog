#[derive(Debug, Clone)]
pub enum MiyutreasuryError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyutreasuryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyutreasuryError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyutreasuryError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyutreasuryError {}
