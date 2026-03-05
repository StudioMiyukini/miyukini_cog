#[derive(Debug, Clone)]
pub enum MiyucomptareportsError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyucomptareportsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyucomptareportsError::NoMandate => {
                write!(f, "Execution refused: no governed mandate")
            }
            MiyucomptareportsError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyucomptareportsError {}
