//! Module de traduction de formats de BondingBrother

/// @id bondingbrother_translation
/// @role data
/// @layer core
/// @do represent_translation
//@human / @human Traduction d'un format vers un autre.
#[derive(Debug, Clone)]
pub struct Translation {
    /// @id bondingbrother_translation_source_format
    /// @role data
    /// @layer core
    /// @do store_source_format
    //@human / @human Format source.
    /// @depends bondingbrother_translation
    pub source_format: String,
    /// @id bondingbrother_translation_target_format
    /// @role data
    /// @layer core
    /// @do store_target_format
    //@human / @human Format cible.
    /// @depends bondingbrother_translation
    pub target_format: String,
}

/// @id bondingbrother_translator_trait
/// @role infrastructure
/// @layer core
/// @do define_translator_contract
//@human / @human Trait de traduction de formats.
pub trait Translator {
    /// @id bondingbrother_translator_translate
    /// @role infrastructure
    /// @layer core
    /// @do translate_data
    //@human / @human Traduit des données d'un format vers un autre.
    /// @depends bondingbrother_translator_trait
    fn translate(
        &self,
        data: &[u8],
        translation: &Translation,
    ) -> Result<Vec<u8>, TranslationError>;
}

/// @id bondingbrother_translation_error
/// @role error
/// @layer core
/// @do represent_translation_error
//@human / @human Erreur de traduction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslationError {
    /// @id bondingbrother_translation_error_unsupported
    /// @role error
    /// @layer core
    /// @do represent_unsupported_error
    //@human / @human Format non supporté.
    /// @depends bondingbrother_translation_error
    UnsupportedFormat(String),
}

impl std::fmt::Display for TranslationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranslationError::UnsupportedFormat(format) => {
                write!(f, "Unsupported format: {format}")
            }
        }
    }
}

impl std::error::Error for TranslationError {}

/// Implémentation par défaut : identité si même format, sinon erreur.
#[derive(Debug, Default)]
pub struct DefaultTranslator;

impl DefaultTranslator {
    /// Crée un traducteur.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Translator for DefaultTranslator {
    fn translate(
        &self,
        data: &[u8],
        translation: &Translation,
    ) -> Result<Vec<u8>, TranslationError> {
        if translation.source_format == translation.target_format {
            Ok(data.to_vec())
        } else {
            Err(TranslationError::UnsupportedFormat(format!(
                "{} -> {}",
                translation.source_format, translation.target_format
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id bondingbrother_translation_test_creation
    /// @role test
    /// @layer core
    /// @do verify_translation_creation
    //@human / @human Test de création d'une traduction.
    /// @depends bondingbrother_translation
    #[test]
    fn test_translation_creation() {
        let translation = Translation {
            source_format: "json".to_string(),
            target_format: "yaml".to_string(),
        };
        assert_eq!(translation.source_format, "json");
    }

    #[test]
    fn test_default_translator_identity() {
        use super::*;
        let tr = DefaultTranslator::new();
        let t = Translation {
            source_format: "json".to_string(),
            target_format: "json".to_string(),
        };
        let data = b"hello";
        let out = tr.translate(data, &t).unwrap();
        assert_eq!(out.as_slice(), data);
    }

    #[test]
    fn test_default_translator_unsupported() {
        use super::*;
        let tr = DefaultTranslator::new();
        let t = Translation {
            source_format: "json".to_string(),
            target_format: "yaml".to_string(),
        };
        assert!(tr.translate(b"x", &t).is_err());
    }
}
