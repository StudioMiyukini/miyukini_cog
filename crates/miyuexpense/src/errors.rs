#[derive(Debug, Clone)]
pub enum MiyuexpenseError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuexpenseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuexpenseError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuexpenseError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuexpenseError {}
