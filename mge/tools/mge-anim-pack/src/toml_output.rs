// @id: MGE-AnimPack-TomlOutput @do: toml-generation @role: back-end @layer: 6 @human: francois
//! TOML output generation for atlas descriptors and animation banks.
//!
//! Generates two kinds of TOML files:
//! 1. `atlas.toml` — frame positions within the atlas image.
//! 2. `animations.toml` — animation clips with frame sequences and timing.

use serde::{Deserialize, Serialize};

use crate::errors::AnimPackError;

// ---------------------------------------------------------------------------
// Atlas descriptor
// ---------------------------------------------------------------------------

/// Top-level atlas descriptor for TOML output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasOutput {
    /// Unique atlas identifier (e.g. "fallen_atlas").
    pub atlas_id: String,
    /// Atlas width in pixels.
    pub width: u32,
    /// Atlas height in pixels.
    pub height: u32,
    /// Frames placed in the atlas.
    pub frames: Vec<AtlasFrameOutput>,
}

/// A single frame within the atlas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasFrameOutput {
    /// Frame name.
    pub name: String,
    /// X offset in the atlas.
    pub x: u32,
    /// Y offset in the atlas.
    pub y: u32,
    /// Frame width.
    pub w: u32,
    /// Frame height.
    pub h: u32,
}

/// Generate an atlas TOML string from an `AtlasOutput`.
pub fn write_atlas_toml(output: &AtlasOutput) -> Result<String, AnimPackError> {
    toml::to_string_pretty(output).map_err(|e| AnimPackError::TomlSerialize(e.to_string()))
}

// ---------------------------------------------------------------------------
// Animation bank
// ---------------------------------------------------------------------------

/// Top-level animation bank for TOML output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimBankOutput {
    /// Entity identifier.
    pub entity_id: String,
    /// Atlas this bank references.
    pub atlas_id: String,
    /// Animation clips.
    pub clips: Vec<AnimClipOutput>,
}

/// A single animation clip (e.g. "idle_s").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimClipOutput {
    /// Clip name (e.g. "idle_s", "walk_sw").
    pub name: String,
    /// Frames per second.
    pub fps: u32,
    /// Whether the clip loops.
    pub looping: bool,
    /// Ordered list of frame names in this clip.
    pub frame_names: Vec<String>,
}

/// Generate an animations TOML string from clip data.
pub fn write_animations_toml(
    entity_id: &str,
    atlas_id: &str,
    clips: &[AnimClipOutput],
) -> Result<String, AnimPackError> {
    let bank = AnimBankOutput {
        entity_id: entity_id.to_string(),
        atlas_id: atlas_id.to_string(),
        clips: clips.to_vec(),
    };
    toml::to_string_pretty(&bank).map_err(|e| AnimPackError::TomlSerialize(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atlas_toml_valid() {
        let output = AtlasOutput {
            atlas_id: "test_atlas".to_string(),
            width: 256,
            height: 256,
            frames: vec![
                AtlasFrameOutput {
                    name: "frame_0".to_string(),
                    x: 0,
                    y: 0,
                    w: 64,
                    h: 64,
                },
                AtlasFrameOutput {
                    name: "frame_1".to_string(),
                    x: 64,
                    y: 0,
                    w: 64,
                    h: 64,
                },
            ],
        };

        let toml_str = write_atlas_toml(&output).unwrap();

        // Verify it is valid TOML by parsing it back.
        let parsed: AtlasOutput = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.atlas_id, "test_atlas");
        assert_eq!(parsed.width, 256);
        assert_eq!(parsed.height, 256);
        assert_eq!(parsed.frames.len(), 2);
        assert_eq!(parsed.frames[0].name, "frame_0");
        assert_eq!(parsed.frames[1].x, 64);
    }

    #[test]
    fn animations_toml_valid() {
        let clips = vec![
            AnimClipOutput {
                name: "idle_s".to_string(),
                fps: 12,
                looping: true,
                frame_names: vec![
                    "fallen_idle_s_000.png".to_string(),
                    "fallen_idle_s_001.png".to_string(),
                    "fallen_idle_s_002.png".to_string(),
                ],
            },
            AnimClipOutput {
                name: "walk_sw".to_string(),
                fps: 12,
                looping: true,
                frame_names: vec![
                    "fallen_walk_sw_000.png".to_string(),
                    "fallen_walk_sw_001.png".to_string(),
                ],
            },
        ];

        let toml_str = write_animations_toml("fallen", "fallen_atlas", &clips).unwrap();

        // Verify it is valid TOML by parsing it back.
        let parsed: AnimBankOutput = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.entity_id, "fallen");
        assert_eq!(parsed.atlas_id, "fallen_atlas");
        assert_eq!(parsed.clips.len(), 2);
        assert_eq!(parsed.clips[0].name, "idle_s");
        assert_eq!(parsed.clips[0].fps, 12);
        assert!(parsed.clips[0].looping);
        assert_eq!(parsed.clips[0].frame_names.len(), 3);
    }

    #[test]
    fn atlas_toml_empty_frames() {
        let output = AtlasOutput {
            atlas_id: "empty".to_string(),
            width: 64,
            height: 64,
            frames: vec![],
        };
        let toml_str = write_atlas_toml(&output).unwrap();
        let parsed: AtlasOutput = toml::from_str(&toml_str).unwrap();
        assert!(parsed.frames.is_empty());
    }

    #[test]
    fn animations_toml_roundtrip() {
        let clips = vec![AnimClipOutput {
            name: "attack_n".to_string(),
            fps: 8,
            looping: false,
            frame_names: vec!["skeleton_attack_n_000.png".to_string()],
        }];
        let toml_str = write_animations_toml("skeleton", "skeleton_atlas", &clips).unwrap();
        let parsed: AnimBankOutput = toml::from_str(&toml_str).unwrap();
        assert!(!parsed.clips[0].looping);
        assert_eq!(parsed.clips[0].fps, 8);
    }
}
