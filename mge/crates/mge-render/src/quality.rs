use serde::{Deserialize, Serialize};

/// Quality tier that controls rendering fidelity.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QualityTier {
    Low,
    #[default]
    Standard,
    High,
}

/// Budget limits per quality tier.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QualityBudget {
    pub max_point_lights: u16,
    pub max_vfx_particles: u32,
    pub max_dynamic_batches: u16,
    pub enable_normal_maps: bool,
    pub enable_color_grading_lut: bool,
    pub shadow_mode: ShadowMode,
    pub vfx_density_scale: f32,
}

/// Shadow rendering mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShadowMode {
    None,
    BlobOnly,
    ContactShadow,
}

impl QualityBudget {
    /// Return the budget for a given tier.
    pub fn for_tier(tier: QualityTier) -> Self {
        match tier {
            QualityTier::Low => Self {
                max_point_lights: 8,
                max_vfx_particles: 500,
                max_dynamic_batches: 64,
                enable_normal_maps: false,
                enable_color_grading_lut: false,
                shadow_mode: ShadowMode::None,
                vfx_density_scale: 0.5,
            },
            QualityTier::Standard => Self {
                max_point_lights: 24,
                max_vfx_particles: 2000,
                max_dynamic_batches: 256,
                enable_normal_maps: false,
                enable_color_grading_lut: true,
                shadow_mode: ShadowMode::BlobOnly,
                vfx_density_scale: 1.0,
            },
            QualityTier::High => Self {
                max_point_lights: 64,
                max_vfx_particles: 5000,
                max_dynamic_batches: 512,
                enable_normal_maps: true,
                enable_color_grading_lut: true,
                shadow_mode: ShadowMode::ContactShadow,
                vfx_density_scale: 1.5,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tier_is_standard() {
        assert_eq!(QualityTier::default(), QualityTier::Standard);
    }

    #[test]
    fn low_has_fewer_resources_than_standard() {
        let low = QualityBudget::for_tier(QualityTier::Low);
        let std = QualityBudget::for_tier(QualityTier::Standard);
        assert!(low.max_point_lights < std.max_point_lights);
        assert!(low.max_vfx_particles < std.max_vfx_particles);
        assert!(!low.enable_color_grading_lut);
    }

    #[test]
    fn high_has_more_resources_than_standard() {
        let std = QualityBudget::for_tier(QualityTier::Standard);
        let high = QualityBudget::for_tier(QualityTier::High);
        assert!(high.max_point_lights > std.max_point_lights);
        assert!(high.enable_normal_maps);
        assert_eq!(high.shadow_mode, ShadowMode::ContactShadow);
    }

    #[test]
    fn vfx_density_scales_by_tier() {
        let low = QualityBudget::for_tier(QualityTier::Low);
        let high = QualityBudget::for_tier(QualityTier::High);
        assert!(low.vfx_density_scale < 1.0);
        assert!(high.vfx_density_scale > 1.0);
    }
}
