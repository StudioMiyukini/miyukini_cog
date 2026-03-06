use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowProfile {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorRgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorRgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderProfile {
    pub clear_rgba: ColorRgba,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioProfile {
    pub listener_bus: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneBootstrap {
    pub id: String,
    pub biome: String,
    pub ambient_rgb: ColorRgb,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBootstrap {
    pub game_id: String,
    pub build_label: String,
    pub window: WindowProfile,
    pub render: RenderProfile,
    pub audio: AudioProfile,
    pub startup_scene: SceneBootstrap,
}

pub fn load_bootstrap(path: &Path) -> Result<GameBootstrap> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("failed to read bootstrap file {}", path.display()))?;
    ron::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bootstrap_from_ron() {
        let raw = r#"
            (
                game_id: "sodomight",
                build_label: "dev",
                window: (title: "Sodomight", width: 1280, height: 720),
                render: (clear_rgba: (r: 0.08, g: 0.04, b: 0.02, a: 1.0)),
                audio: (listener_bus: "master"),
                startup_scene: (
                    id: "act1_rogue_camp",
                    biome: "rogue_camp",
                    ambient_rgb: (r: 0.2, g: 0.15, b: 0.1),
                ),
            )
        "#;
        let config: GameBootstrap = ron::from_str(raw).expect("valid bootstrap");
        assert_eq!(config.game_id, "sodomight");
        assert_eq!(config.window.width, 1280);
    }
}
