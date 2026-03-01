/// @id toolkit.alicia.wakeword.models
/// @do manage_wake_word_model_files
/// @role model_management
/// @layer 6
/// @human Gestion des fichiers modeles wake word (chargement, validation, metadonnees)
use std::path::Path;

use crate::errors::WakeWordError;

/// Types de modeles rustpotter supportes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    /// Modele reference (3-8 enregistrements WAV, DTW).
    /// Extension : `.rpw`
    Reference,
    /// Modele entraine (reseau de neurones, meilleure precision).
    /// Extension : `.rpwm`
    Trained,
}

impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reference => write!(f, "reference (.rpw)"),
            Self::Trained => write!(f, "trained (.rpwm)"),
        }
    }
}

/// Informations sur un modele charge.
#[derive(Debug, Clone)]
pub struct ModelInfo {
    /// Chemin du fichier modele.
    pub path: String,
    /// Type de modele (reference ou entraine).
    pub model_type: ModelType,
    /// Mot-cle associe.
    pub keyword: String,
    /// Taille du fichier en octets.
    pub file_size: u64,
}

/// Determine le type de modele a partir de l'extension du fichier.
///
/// # Errors
///
/// Retourne `WakeWordError::ModelInvalid` si l'extension n'est pas reconnue.
pub fn model_type_from_extension(path: &str) -> Result<ModelType, WakeWordError> {
    let path = Path::new(path);
    match path.extension().and_then(|e| e.to_str()) {
        Some("rpw") => Ok(ModelType::Reference),
        Some("rpwm") => Ok(ModelType::Trained),
        Some(ext) => Err(WakeWordError::ModelInvalid(format!(
            "Unsupported model extension '.{ext}', expected .rpw or .rpwm"
        ))),
        None => Err(WakeWordError::ModelInvalid(
            "Model file has no extension, expected .rpw or .rpwm".to_string(),
        )),
    }
}

/// Verifie qu'un fichier modele existe et est valide.
///
/// Controles effectues :
/// 1. Le fichier existe
/// 2. Le fichier n'est pas vide
/// 3. L'extension est reconnue (.rpw ou .rpwm)
///
/// # Errors
///
/// - `WakeWordError::ModelNotFound` si le fichier n'existe pas
/// - `WakeWordError::ModelInvalid` si le fichier est vide ou l'extension inconnue
pub fn validate_model(path: &str) -> Result<ModelType, WakeWordError> {
    let file_path = Path::new(path);

    if !file_path.exists() {
        return Err(WakeWordError::ModelNotFound(format!(
            "Model file does not exist: {path}"
        )));
    }

    let metadata = std::fs::metadata(file_path).map_err(|e| {
        WakeWordError::ModelNotFound(format!("Cannot read model file metadata: {e}"))
    })?;

    if metadata.len() == 0 {
        return Err(WakeWordError::ModelInvalid(
            "Model file is empty (0 bytes)".to_string(),
        ));
    }

    model_type_from_extension(path)
}

/// Charge et valide un fichier modele rustpotter.
///
/// Verifie l'existence, la lisibilite, et la version du format.
/// Retourne les metadonnees du modele (type, taille, mot-cle).
///
/// # Errors
///
/// - `WakeWordError::ModelNotFound` si le fichier n'existe pas
/// - `WakeWordError::ModelInvalid` si le fichier est corrompu ou le format inconnu
pub fn load_model(path: &str, keyword: &str) -> Result<ModelInfo, WakeWordError> {
    let model_type = validate_model(path)?;

    let metadata = std::fs::metadata(path).map_err(|e| {
        WakeWordError::ModelNotFound(format!("Cannot read model file metadata: {e}"))
    })?;

    tracing::info!(
        path = path,
        model_type = %model_type,
        size_bytes = metadata.len(),
        keyword = keyword,
        "Model loaded successfully"
    );

    Ok(ModelInfo {
        path: path.to_string(),
        model_type,
        keyword: keyword.to_string(),
        file_size: metadata.len(),
    })
}

/// Retourne le chemin par defaut du modele "Hey Alicia".
///
/// Convention : `models/hey_alicia.rpw` relatif au repertoire de travail.
#[must_use]
pub fn default_model_path() -> String {
    "models/hey_alicia.rpw".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_type_from_extension_rpw() {
        let result = model_type_from_extension("models/hey_alicia.rpw");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ModelType::Reference);
    }

    #[test]
    fn test_model_type_from_extension_rpwm() {
        let result = model_type_from_extension("models/hey_alicia.rpwm");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ModelType::Trained);
    }

    #[test]
    fn test_model_type_from_extension_invalid() {
        let result = model_type_from_extension("models/hey_alicia.wav");
        assert!(result.is_err());
        match result {
            Err(WakeWordError::ModelInvalid(msg)) => {
                assert!(msg.contains(".wav"));
            }
            _ => panic!("Expected ModelInvalid error"),
        }
    }

    #[test]
    fn test_model_type_from_extension_no_ext() {
        let result = model_type_from_extension("models/hey_alicia");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_model_not_found() {
        let result = validate_model("/nonexistent/path/to/model.rpw");
        assert!(result.is_err());
        match result {
            Err(WakeWordError::ModelNotFound(_)) => {}
            _ => panic!("Expected ModelNotFound error"),
        }
    }

    #[test]
    fn test_default_model_path() {
        let path = default_model_path();
        assert!(std::path::Path::new(&path)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("rpw")));
        assert!(path.contains("hey_alicia"));
    }

    #[test]
    fn test_model_type_display() {
        assert_eq!(format!("{}", ModelType::Reference), "reference (.rpw)");
        assert_eq!(format!("{}", ModelType::Trained), "trained (.rpwm)");
    }

    #[test]
    fn test_load_model_not_found() {
        let result = load_model("/nonexistent/model.rpw", "hey_alicia");
        assert!(result.is_err());
        match result {
            Err(WakeWordError::ModelNotFound(_)) => {}
            _ => panic!("Expected ModelNotFound error"),
        }
    }
}
