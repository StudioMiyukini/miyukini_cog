#[derive(Debug, Clone)]
pub enum MiyuwebwayTrackerError {
    NoMandate,
    Unimplemented,
    /// Tracker (Origin) indisponible ; délégation réseau non implémentée ou échec connexion.
    TrackerUnavailable(String),
}

impl std::fmt::Display for MiyuwebwayTrackerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuwebwayTrackerError::NoMandate => {
                write!(f, "Execution refused: no governed mandate")
            }
            MiyuwebwayTrackerError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyuwebwayTrackerError::TrackerUnavailable(ref msg) => {
                write!(f, "Tracker unavailable: {msg}")
            }
        }
    }
}
impl std::error::Error for MiyuwebwayTrackerError {}
