#[derive(Debug, Clone)]
pub enum MiyusocialfeedError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyusocialfeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyusocialfeedError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyusocialfeedError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyusocialfeedError {}
