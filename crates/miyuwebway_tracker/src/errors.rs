#[derive(Debug, Clone)]
pub enum MiyuwebwayTrackerError {
    NoMandate,
    Unimplemented,
}

impl std::fmt::Display for MiyuwebwayTrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuwebwayTrackerError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuwebwayTrackerError::Unimplemented => write!(f, "Tool not yet implemented"),
        }
    }
}
impl std::error::Error for MiyuwebwayTrackerError {}
