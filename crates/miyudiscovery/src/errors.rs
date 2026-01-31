#[derive(Debug, Clone)]
pub enum MiyudiscoveryError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyudiscoveryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyudiscoveryError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyudiscoveryError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyudiscoveryError {}
