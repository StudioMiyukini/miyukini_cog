#[derive(Debug, Clone)]
pub enum MiyuantispamError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuantispamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuantispamError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuantispamError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuantispamError {}
