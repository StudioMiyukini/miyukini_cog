#[derive(Debug, Clone)]
pub enum MiyubookmarksError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyubookmarksError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyubookmarksError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyubookmarksError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyubookmarksError {}
