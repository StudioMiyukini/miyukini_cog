#[derive(Debug, Clone)]
pub enum MiyuposinventoryError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuposinventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuposinventoryError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuposinventoryError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuposinventoryError {}
