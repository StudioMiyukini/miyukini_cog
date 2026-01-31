#[derive(Debug, Clone)]
pub enum MiyushippingError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyushippingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyushippingError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyushippingError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyushippingError {}
