use serde::{Deserialize, Serialize};

/// Global directional light for a biome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmbientLight {
    pub color: [f32; 3],
    pub intensity: f32,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self { color: [1.0, 0.95, 0.85], intensity: 0.6 }
    }
}

/// Point light source (torches, braziers, spell effects, portals).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointLight {
    pub world_pos: [f32; 2],
    pub color: [f32; 3],
    pub intensity: f32,
    pub radius: f32,
}

/// Fog/darkness zone descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FogZone {
    pub center: [f32; 2],
    pub radius: f32,
    pub density: f32,
    pub color: [f32; 3],
}

/// Color grading LUT reference per biome or narrative state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorGradingProfile {
    pub name: String,
    pub brightness: f32,
    pub contrast: f32,
    pub saturation: f32,
    pub tint: [f32; 3],
}

impl Default for ColorGradingProfile {
    fn default() -> Self {
        Self {
            name: "neutral".into(),
            brightness: 1.0,
            contrast: 1.0,
            saturation: 1.0,
            tint: [1.0, 1.0, 1.0],
        }
    }
}

/// Holds all lighting state for a single frame.
#[derive(Debug, Clone, Default)]
pub struct LightingState {
    pub ambient: AmbientLight,
    pub point_lights: Vec<PointLight>,
    pub fog_zones: Vec<FogZone>,
    pub color_grading: ColorGradingProfile,
}

impl LightingState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_point_light(&mut self, light: PointLight) {
        self.point_lights.push(light);
    }

    pub fn add_fog_zone(&mut self, fog: FogZone) {
        self.fog_zones.push(fog);
    }

    /// Compute effective light contribution at a world position.
    /// Returns RGB multiplier to apply to sprites.
    pub fn sample_at(&self, wx: f32, wy: f32) -> [f32; 3] {
        let mut r = self.ambient.color[0] * self.ambient.intensity;
        let mut g = self.ambient.color[1] * self.ambient.intensity;
        let mut b = self.ambient.color[2] * self.ambient.intensity;

        for light in &self.point_lights {
            let dx = wx - light.world_pos[0];
            let dy = wy - light.world_pos[1];
            let dist_sq = dx * dx + dy * dy;
            let radius_sq = light.radius * light.radius;
            if dist_sq < radius_sq {
                let falloff = 1.0 - (dist_sq / radius_sq);
                r += light.color[0] * light.intensity * falloff;
                g += light.color[1] * light.intensity * falloff;
                b += light.color[2] * light.intensity * falloff;
            }
        }

        // Apply fog darkening.
        for fog in &self.fog_zones {
            let dx = wx - fog.center[0];
            let dy = wy - fog.center[1];
            let dist_sq = dx * dx + dy * dy;
            let radius_sq = fog.radius * fog.radius;
            if dist_sq < radius_sq {
                let fog_factor = (1.0 - (dist_sq / radius_sq)) * fog.density;
                r = r * (1.0 - fog_factor) + fog.color[0] * fog_factor;
                g = g * (1.0 - fog_factor) + fog.color[1] * fog_factor;
                b = b * (1.0 - fog_factor) + fog.color[2] * fog_factor;
            }
        }

        [r.min(2.0), g.min(2.0), b.min(2.0)]
    }

    /// Apply color grading to an RGB value.
    pub fn apply_grading(&self, rgb: [f32; 3]) -> [f32; 3] {
        let cg = &self.color_grading;
        let mut out = rgb;
        // Brightness
        out[0] *= cg.brightness;
        out[1] *= cg.brightness;
        out[2] *= cg.brightness;
        // Contrast (pivot at 0.5)
        for c in &mut out {
            *c = ((*c - 0.5) * cg.contrast + 0.5).clamp(0.0, 2.0);
        }
        // Saturation
        let lum = out[0] * 0.299 + out[1] * 0.587 + out[2] * 0.114;
        for (i, c) in out.iter_mut().enumerate() {
            *c = lum + (*c - lum) * cg.saturation;
            *c *= cg.tint[i];
            *c = c.clamp(0.0, 2.0);
        }
        out
    }

    pub fn clear(&mut self) {
        self.point_lights.clear();
        self.fog_zones.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ambient_only_returns_base_color() {
        let state = LightingState::new();
        let [r, g, b] = state.sample_at(0.0, 0.0);
        assert!(r > 0.0);
        assert!(g > 0.0);
        assert!(b > 0.0);
    }

    #[test]
    fn point_light_adds_contribution() {
        let mut state = LightingState::new();
        state.ambient = AmbientLight { color: [0.0, 0.0, 0.0], intensity: 0.0 };
        state.add_point_light(PointLight {
            world_pos: [5.0, 5.0],
            color: [1.0, 0.5, 0.0],
            intensity: 1.0,
            radius: 10.0,
        });

        let at_light = state.sample_at(5.0, 5.0);
        assert!(at_light[0] > 0.9, "red={}", at_light[0]);
        assert!(at_light[1] > 0.4, "green={}", at_light[1]);

        let far_away = state.sample_at(100.0, 100.0);
        assert!(far_away[0] < 0.01, "should be dark far away");
    }

    #[test]
    fn fog_darkens_area() {
        let mut state = LightingState::new();
        state.ambient = AmbientLight { color: [1.0, 1.0, 1.0], intensity: 1.0 };
        state.add_fog_zone(FogZone {
            center: [0.0, 0.0],
            radius: 10.0,
            density: 0.9,
            color: [0.0, 0.0, 0.0],
        });

        let in_fog = state.sample_at(0.0, 0.0);
        let outside = state.sample_at(20.0, 20.0);
        assert!(in_fog[0] < outside[0], "in_fog should be darker");
    }

    #[test]
    fn neutral_grading_preserves_color() {
        let state = LightingState::new();
        let input = [0.5, 0.5, 0.5];
        let output = state.apply_grading(input);
        for i in 0..3 {
            assert!((output[i] - input[i]).abs() < 0.01, "channel {i}: {}", output[i]);
        }
    }

    #[test]
    fn high_contrast_grading_shifts_values() {
        let mut state = LightingState::new();
        state.color_grading.contrast = 2.0;
        let bright = state.apply_grading([0.8, 0.8, 0.8]);
        let dark = state.apply_grading([0.2, 0.2, 0.2]);
        assert!(bright[0] > 0.8);
        assert!(dark[0] < 0.2);
    }
}
