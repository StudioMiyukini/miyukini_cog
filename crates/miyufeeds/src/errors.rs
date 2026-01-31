#[derive(Debug, Clone)]
pub enum MiyufeedsError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyufeedsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyufeedsError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyufeedsError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyufeedsError {}
