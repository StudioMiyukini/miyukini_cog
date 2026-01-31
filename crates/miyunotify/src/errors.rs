#[derive(Debug, Clone)]
pub enum MiyunotifyError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyunotifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyunotifyError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyunotifyError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyunotifyError {}
