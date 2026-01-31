#[derive(Debug, Clone)]
pub enum MiyuLocaleError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuLocaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuLocaleError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuLocaleError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuLocaleError {}
