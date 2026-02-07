//! Module de traduction de formats de BondingBrother

/// @id: bondingbrother_translation
/// @role: data
/// @layer: core
/// @human: Traduction d'un format vers un autre.
/// @do: represent_translation
#[derive(Debug, Clone)]
pub struct Translation {
    /// @id: bondingbrother_translation_source_format
    /// @role: data
    /// @layer: core
    /// @human: Format source.
    /// @do: store_source_format
    /// @depends: bondingbrother_translation
    pub source_format: String,
    /// @id: bondingbrother_translation_target_format
    /// @role: data
    /// @layer: core
    /// @human: Format cible.
    /// @do: store_target_format
    /// @depends: bondingbrother_translation
    pub target_format: String,
}

/// @id: bondingbrother_translator_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de traduction de formats.
/// @do: define_translator_contract
pub trait Translator {
    /// @id: bondingbrother_translator_translate
    /// @role: infrastructure
    /// @layer: core
    /// @human: Traduit des données d'un format vers un autre.
    /// @do: translate_data
    /// @depends: bondingbrother_translator_trait
    fn translate(&self, data: &[u8], translation: &Translation) -> Result<Vec<u8>, TranslationError>;
}

/// @id: bondingbrother_translation_error
/// @role: error
/// @layer: core
/// @human: Erreur de traduction.
/// @do: represent_translation_error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslationError {
    /// @id: bondingbrother_translation_error_unsupported
    /// @role: error
    /// @layer: core
    /// @human: Format non supporté.
    /// @do: represent_unsupported_error
    /// @depends: bondingbrother_translation_error
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

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: bondingbrother_translation_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une traduction.
    /// @do: verify_translation_creation
    /// @depends: bondingbrother_translation
    #[test]
    fn test_translation_creation() {
        let translation = Translation {
            source_format: "json".to_string(),
            target_format: "yaml".to_string(),
        };
        assert_eq!(translation.source_format, "json");
    }
}
