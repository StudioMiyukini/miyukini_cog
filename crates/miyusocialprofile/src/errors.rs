#[derive(Debug, Clone)]
pub enum MiyusocialprofileError {
    NoMandate,
    Unimplemented,
    InvalidInput(String),
}

impl std::fmt::Display for MiyusocialprofileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyusocialprofileError::NoMandate => {
                write!(f, "Execution refused: no governed mandate")
            }
            MiyusocialprofileError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyusocialprofileError::InvalidInput(ref m) => write!(f, "{m}"),
        }
    }
}
impl std::error::Error for MiyusocialprofileError {}
