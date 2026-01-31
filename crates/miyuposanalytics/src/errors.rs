#[derive(Debug, Clone)]
pub enum MiyuposanalyticsError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuposanalyticsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuposanalyticsError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuposanalyticsError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuposanalyticsError {}
