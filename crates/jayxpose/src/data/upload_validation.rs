//! Validation des uploads de documents JayXpose.
//!
//! Vérifie le type MIME et le nom de fichier avant insertion en base.
//!
//! @id: jayxpose_upload_validation @do: validate_document_upload
//! @role: logic @layer: service
//! @human: Validation uploads JayXpose — MIME autorisés, absence de path traversal, taille max.

use crate::data::types::DocumentProfessionnel;

/// Erreur de validation d'upload.
#[derive(Debug, Clone, PartialEq)]
pub enum UploadValidationError {
    /// Type MIME non autorisé.
    MimeTypeNotAllowed(String),
    /// Nom de fichier invalide (path traversal, caractères interdits).
    InvalidFileName(String),
    /// Taille de fichier dépassée (octets).
    FileTooLarge {
        /// Taille du fichier fourni (octets).
        size: i64,
        /// Taille maximale autorisée (octets).
        max: i64,
    },
    /// Champ obligatoire manquant.
    MissingField(&'static str),
}

impl std::fmt::Display for UploadValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MimeTypeNotAllowed(m) => write!(f, "Type MIME non autorisé: {m}"),
            Self::InvalidFileName(n) => write!(f, "Nom de fichier invalide: {n}"),
            Self::FileTooLarge { size, max } => {
                write!(f, "Fichier trop volumineux: {size} octets (max {max})")
            }
            Self::MissingField(field) => write!(f, "Champ obligatoire manquant: {field}"),
        }
    }
}

impl std::error::Error for UploadValidationError {}

/// Taille maximale d'un document (20 Mo).
pub const DOCUMENT_MAX_BYTES: i64 = 20 * 1024 * 1024;

/// Types MIME autorisés pour les documents professionnels JayXpose.
pub const ALLOWED_MIME_TYPES: &[&str] = &[
    "application/pdf",
    "image/jpeg",
    "image/png",
    "image/webp",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
];

/// Valide un `DocumentProfessionnel` avant insertion en base.
///
/// Vérifie :
/// - `mime_type` dans `ALLOWED_MIME_TYPES`
/// - `file_name` sans path traversal (`..`, `/`, `\`, NUL)
/// - `file_size` ≤ `DOCUMENT_MAX_BYTES`
/// - champs `exposant_id` et `doc_type` présents
pub fn validate_document_upload(
    doc: &DocumentProfessionnel,
) -> Result<(), UploadValidationError> {
    // Champs obligatoires
    if doc.exposant_id.as_deref().unwrap_or("").is_empty() {
        return Err(UploadValidationError::MissingField("exposant_id"));
    }
    if doc.doc_type.as_deref().unwrap_or("").is_empty() {
        return Err(UploadValidationError::MissingField("doc_type"));
    }

    // Validation MIME
    if let Some(mime) = &doc.mime_type {
        if !ALLOWED_MIME_TYPES.contains(&mime.as_str()) {
            return Err(UploadValidationError::MimeTypeNotAllowed(mime.clone()));
        }
    }

    // Validation nom de fichier (path traversal)
    if let Some(name) = &doc.file_name {
        if name.contains("..") || name.contains('/') || name.contains('\\') || name.contains('\0')
        {
            return Err(UploadValidationError::InvalidFileName(name.clone()));
        }
    }

    // Validation taille
    if let Some(size) = doc.file_size {
        if size > DOCUMENT_MAX_BYTES {
            return Err(UploadValidationError::FileTooLarge {
                size,
                max: DOCUMENT_MAX_BYTES,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::DocumentProfessionnel;

    fn base_doc() -> DocumentProfessionnel {
        DocumentProfessionnel {
            exposant_id: Some("exp-1".to_string()),
            doc_type: Some("rib".to_string()),
            mime_type: Some("application/pdf".to_string()),
            file_name: Some("rib.pdf".to_string()),
            file_size: Some(1024),
            ..DocumentProfessionnel::default()
        }
    }

    #[test]
    fn valid_document_passes() {
        assert!(validate_document_upload(&base_doc()).is_ok());
    }

    #[test]
    fn invalid_mime_rejected() {
        let mut d = base_doc();
        d.mime_type = Some("application/x-sh".to_string());
        assert!(matches!(
            validate_document_upload(&d),
            Err(UploadValidationError::MimeTypeNotAllowed(_))
        ));
    }

    #[test]
    fn path_traversal_rejected() {
        let mut d = base_doc();
        d.file_name = Some("../../etc/passwd".to_string());
        assert!(matches!(
            validate_document_upload(&d),
            Err(UploadValidationError::InvalidFileName(_))
        ));
    }

    #[test]
    fn file_too_large_rejected() {
        let mut d = base_doc();
        d.file_size = Some(DOCUMENT_MAX_BYTES + 1);
        assert!(matches!(
            validate_document_upload(&d),
            Err(UploadValidationError::FileTooLarge { .. })
        ));
    }

    #[test]
    fn missing_exposant_id_rejected() {
        let mut d = base_doc();
        d.exposant_id = None;
        assert!(matches!(
            validate_document_upload(&d),
            Err(UploadValidationError::MissingField("exposant_id"))
        ));
    }
}
