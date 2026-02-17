#[derive(Debug, Clone)]
pub enum MiyucptaledgerError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyucptaledgerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyucptaledgerError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyucptaledgerError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyucptaledgerError::InvalidInput(ref m) => write!(f, "{m}"),
        }
    }
}
impl std::error::Error for MiyucptaledgerError {}
