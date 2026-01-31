#[derive(Debug, Clone)]
pub enum MiyuwebwayParticipantError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuwebwayParticipantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuwebwayParticipantError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuwebwayParticipantError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuwebwayParticipantError {}
