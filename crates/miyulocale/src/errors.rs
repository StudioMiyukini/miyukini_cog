#[derive(Debug, Clone)]
pub enum MiyuLocaleError {
    NoMandate,
    InvalidTimestamp,
}

impl std::fmt::Display for MiyuLocaleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuLocaleError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuLocaleError::InvalidTimestamp => write!(f, "Invalid timestamp"),
        }
    }
}
impl std::error::Error for MiyuLocaleError {}
