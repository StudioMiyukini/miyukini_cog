#[derive(Debug, Clone)]
pub enum MiyuwidgetsError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuwidgetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuwidgetsError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuwidgetsError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuwidgetsError {}
