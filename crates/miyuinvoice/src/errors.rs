#[derive(Debug, Clone)]
pub enum MiyuinvoiceError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuinvoiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuinvoiceError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuinvoiceError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuinvoiceError {}
