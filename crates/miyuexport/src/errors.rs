#[derive(Debug, Clone)]
pub enum MiyuExportError {
    NoMandate,
    Unimplemented,
    /// Erreur de chargement de police ou de rendu (PDF/XLSX).
    Io(String),
}

impl std::fmt::Display for MiyuExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MiyuExportError::NoMandate => write!(f, "Execution refused: no governed mandate"),
            MiyuExportError::Unimplemented => write!(f, "Tool not yet implemented"),
            MiyuExportError::Io(ref msg) => write!(f, "{}", msg),
        }
    }
}
impl std::error::Error for MiyuExportError {}
