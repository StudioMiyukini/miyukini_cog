#[derive(Debug, Clone)]
pub enum MiyuwebError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuwebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuwebError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuwebError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuwebError {}
