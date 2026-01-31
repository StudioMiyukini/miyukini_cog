#[derive(Debug, Clone)]
pub enum MiyupmError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyupmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyupmError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyupmError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyupmError {}
