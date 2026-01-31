#[derive(Debug, Clone)]
pub enum MiyupospaymentError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyupospaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyupospaymentError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyupospaymentError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyupospaymentError {}
