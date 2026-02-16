#[derive(Debug, Clone)]
pub enum MiyuprofileError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyuprofileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuprofileError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuprofileError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyuprofileError::InvalidInput(ref msg) => write!(f, "{msg}"),
        }
    }
}
impl std::error::Error for MiyuprofileError {}
