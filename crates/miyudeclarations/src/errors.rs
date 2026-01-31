#[derive(Debug, Clone)]
pub enum MiyudeclarationsError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyudeclarationsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyudeclarationsError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyudeclarationsError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyudeclarationsError {}
