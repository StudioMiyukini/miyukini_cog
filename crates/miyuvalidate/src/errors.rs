#[derive(Debug, Clone)]
pub enum MiyuValidateError {
    NoMandate,
    Unimplemented,
    InvalidSchema(String),
    InvalidData(String),
}

impl std::fmt::Display for MiyuValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuValidateError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuValidateError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyuValidateError::InvalidSchema(m) => write!(f, "Invalid schema: {m}"),
            MiyuValidateError::InvalidData(m) => write!(f, "Invalid data: {m}"),
        }
    }
}
impl std::error::Error for MiyuValidateError {}
