#[derive(Debug, Clone)]
pub enum MiyuposkitchenError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuposkitchenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuposkitchenError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuposkitchenError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuposkitchenError {}
