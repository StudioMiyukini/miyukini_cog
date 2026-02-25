//! Optimisation et compression des images de pages JayManga.
//!
//! Génère des variantes optimisées (HD, SD, mobile, thumb) en WebP
//! à partir des images originales. Sélectionne la meilleure variante
//! selon le viewport et les capacités du client.

use crate::data::{ImageVariant, OptimizationConfig};

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module d'optimisation.
#[derive(Debug, thiserror::Error)]
pub enum OptimizerError {
    #[error("Image processing error: {0}")]
    Processing(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("No original image found for page {0}")]
    NoOriginal(String),
}

// ---------------------------------------------------------------------------
// Job d'optimisation
// ---------------------------------------------------------------------------

/// Job d'optimisation pour une page.
#[derive(Debug, Clone)]
pub struct OptimizationJob {
    pub page_id: String,
    pub original_path: String,
    pub config: OptimizationConfig,
}

/// Profils de résolution disponibles.
pub const PROFILE_HD: (&str, i32) = ("hd", 1920);
pub const PROFILE_SD: (&str, i32) = ("sd", 1280);
pub const PROFILE_MOBILE: (&str, i32) = ("mobile", 800);
pub const PROFILE_THUMB: (&str, i32) = ("thumb", 300);

/// Tous les profils par défaut.
pub const ALL_PROFILES: [(&str, i32); 4] = [PROFILE_HD, PROFILE_SD, PROFILE_MOBILE, PROFILE_THUMB];

// ---------------------------------------------------------------------------
// Sélection de variante
// ---------------------------------------------------------------------------

/// Sélectionne la meilleure variante pour le viewport et les capacités du client.
///
/// Algorithme : choisir la variante dont la largeur est la plus proche
/// (en excès) de `viewport_width * pixel_ratio`, en préférant WebP si supporté.
#[must_use]
pub fn optimizer_select_variant<'a>(
    variants: &'a [ImageVariant],
    viewport_width: i32,
    pixel_ratio: f64,
    supports_webp: bool,
    _supports_avif: bool,
) -> Option<&'a ImageVariant> {
    if variants.is_empty() {
        return None;
    }

    let target_width = (f64::from(viewport_width) * pixel_ratio) as i32;

    // Filtrer par format supporté
    let candidates: Vec<&ImageVariant> = variants
        .iter()
        .filter(|v| {
            if v.format == "webp" && !supports_webp {
                return false;
            }
            true
        })
        .collect();

    if candidates.is_empty() {
        return variants.first();
    }

    // Trouver la variante la plus proche en excès
    let mut best: Option<&ImageVariant> = None;
    let mut best_diff = i32::MAX;

    for v in &candidates {
        let diff = v.width - target_width;
        // Préférer en excès (diff >= 0), sinon le plus grand disponible
        if diff >= 0 && diff < best_diff {
            best = Some(v);
            best_diff = diff;
        }
    }

    // Si aucune variante en excès, prendre la plus grande
    if best.is_none() {
        best = candidates.iter().max_by_key(|v| v.width).copied();
    }

    best
}

/// Calcule la résolution cible pour un profil donné,
/// en préservant le ratio d'aspect.
#[must_use]
pub fn optimizer_target_dimensions(
    original_width: i32,
    original_height: i32,
    target_width: i32,
) -> (i32, i32) {
    if original_width <= target_width {
        return (original_width, original_height);
    }
    let ratio = f64::from(target_width) / f64::from(original_width);
    let target_height = (f64::from(original_height) * ratio) as i32;
    (target_width, target_height)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_variant(profile: &str, width: i32, format: &str) -> ImageVariant {
        ImageVariant {
            profile: profile.to_string(),
            format: format.to_string(),
            path: format!("/path/{profile}.{format}"),
            width,
            height: (width as f64 * 1.5) as i32,
            file_size: (width as i64) * 500,
        }
    }

    #[test]
    fn test_select_variant_mobile_viewport() {
        let variants = vec![
            make_variant("hd", 1920, "webp"),
            make_variant("sd", 1280, "webp"),
            make_variant("mobile", 800, "webp"),
            make_variant("thumb", 300, "webp"),
        ];
        let selected = optimizer_select_variant(&variants, 800, 1.0, true, false);
        assert_eq!(selected.unwrap().profile, "mobile");
    }

    #[test]
    fn test_select_variant_hd_retina() {
        let variants = vec![
            make_variant("hd", 1920, "webp"),
            make_variant("sd", 1280, "webp"),
            make_variant("mobile", 800, "webp"),
        ];
        let selected = optimizer_select_variant(&variants, 1920, 2.0, true, false);
        // 1920*2=3840, aucune variante >= 3840, donc la plus grande (hd=1920)
        assert_eq!(selected.unwrap().profile, "hd");
    }

    #[test]
    fn test_select_variant_no_webp_support() {
        let variants = vec![
            make_variant("hd", 1920, "webp"),
            make_variant("hd", 1920, "jpeg"),
            make_variant("sd", 1280, "jpeg"),
        ];
        let selected = optimizer_select_variant(&variants, 1920, 1.0, false, false);
        assert_eq!(selected.unwrap().format, "jpeg");
    }

    #[test]
    fn test_select_variant_empty() {
        let variants: Vec<ImageVariant> = Vec::new();
        assert!(optimizer_select_variant(&variants, 800, 1.0, true, false).is_none());
    }

    #[test]
    fn test_target_dimensions_downscale() {
        let (w, h) = optimizer_target_dimensions(4000, 6000, 1920);
        assert_eq!(w, 1920);
        assert_eq!(h, 2880);
    }

    #[test]
    fn test_target_dimensions_no_upscale() {
        let (w, h) = optimizer_target_dimensions(800, 1200, 1920);
        assert_eq!(w, 800);
        assert_eq!(h, 1200);
    }
}
