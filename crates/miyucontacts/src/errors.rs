#[derive(Debug, Clone)]
pub enum MiyucontactsError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyucontactsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyucontactsError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyucontactsError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyucontactsError {}
