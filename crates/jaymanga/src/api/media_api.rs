//! Endpoints REST media JayManga.
//!
//! Service des images de pages (variantes optimisées) et des couvertures.
//! Sélection automatique de variante selon Accept, viewport et pixel ratio.

use serde::Deserialize;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Paramètres de requête pour la sélection de variante.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MediaQuery {
    pub viewport: Option<i32>,
    pub dpr: Option<f64>, // Device pixel ratio
}

/// Headers pertinents pour la sélection de variante.
#[derive(Debug, Clone, Default)]
pub struct MediaHeaders {
    pub accept_webp: bool,
    pub accept_avif: bool,
    pub viewport_width: Option<i32>,
}

impl MediaHeaders {
    /// Parse les headers Accept pour déterminer les formats supportés.
    #[must_use]
    pub fn from_accept(accept: &str) -> Self {
        Self {
            accept_webp: accept.contains("image/webp"),
            accept_avif: accept.contains("image/avif"),
            viewport_width: None,
        }
    }
}

/// Réponse media avec métadonnées de cache.
#[derive(Debug, Clone)]
pub struct MediaResponse {
    pub data: Vec<u8>,
    pub content_type: String,
    pub cache_control: String,
    pub etag: Option<String>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Détermine le content-type à partir de l'extension du fichier.
#[must_use]
pub fn media_content_type(format: &str) -> &'static str {
    match format {
        "webp" => "image/webp",
        "avif" => "image/avif",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        _ => "application/octet-stream",
    }
}

/// Génère les headers de cache pour une image.
#[must_use]
pub fn media_cache_headers(is_optimized: bool) -> &'static str {
    if is_optimized {
        "public, max-age=86400, immutable" // 24h, immutable (contenu hashé)
    } else {
        "public, max-age=3600" // 1h pour les originaux
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type() {
        assert_eq!(media_content_type("webp"), "image/webp");
        assert_eq!(media_content_type("jpeg"), "image/jpeg");
        assert_eq!(media_content_type("unknown"), "application/octet-stream");
    }

    #[test]
    fn test_cache_headers() {
        assert!(media_cache_headers(true).contains("immutable"));
        assert!(!media_cache_headers(false).contains("immutable"));
    }

    #[test]
    fn test_media_headers_from_accept() {
        let headers = MediaHeaders::from_accept("image/webp, image/avif, image/jpeg");
        assert!(headers.accept_webp);
        assert!(headers.accept_avif);

        let headers2 = MediaHeaders::from_accept("image/jpeg");
        assert!(!headers2.accept_webp);
    }
}
