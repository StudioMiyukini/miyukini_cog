// @id: MGE-AnimPack-Aseprite @do: aseprite-loader @role: back-end @layer: 6 @human: francois
//! Native Aseprite (.ase) file loader using the `asefile` crate.
//!
//! Loads an Aseprite file and extracts frame pixel data (RGBA),
//! frame durations, and animation tags.

use std::path::Path;

use crate::errors::AnimPackError;

/// Extracted data from an Aseprite file.
#[derive(Debug, Clone)]
pub struct AsepriteData {
    /// Sprite width in pixels.
    pub width: u32,
    /// Sprite height in pixels.
    pub height: u32,
    /// All frames with their pixel data.
    pub frames: Vec<AseFrame>,
    /// Animation tags (named frame ranges).
    pub tags: Vec<AseTag>,
}

/// A single extracted frame.
#[derive(Debug, Clone)]
pub struct AseFrame {
    /// Frame index (0-based).
    pub index: u32,
    /// Frame duration in milliseconds.
    pub duration_ms: u32,
    /// Raw RGBA pixel data (width * height * 4 bytes).
    pub pixels: Vec<u8>,
}

/// An animation tag (named frame range).
#[derive(Debug, Clone)]
pub struct AseTag {
    /// Tag name (e.g. "idle", "walk", "attack").
    pub name: String,
    /// First frame index (inclusive).
    pub from_frame: u32,
    /// Last frame index (inclusive).
    pub to_frame: u32,
}

/// Load an Aseprite file and extract its data.
///
/// Returns all frames as RGBA pixel data, plus animation tags.
pub fn load_aseprite(path: &Path) -> Result<AsepriteData, AnimPackError> {
    let ase = asefile::AsepriteFile::read_file(path)
        .map_err(|e| AnimPackError::AsepriteParse(format!("{e}")))?;

    let width = ase.width() as u32;
    let height = ase.height() as u32;

    // Extract frames.
    let mut frames = Vec::with_capacity(ase.num_frames() as usize);
    for i in 0..ase.num_frames() {
        let frame = ase.frame(i);
        let duration_ms = frame.duration();
        // frame.image() returns an image::RgbaImage (image 0.24).
        // .into_raw() gives us Vec<u8> of RGBA data.
        let rgba_image = frame.image();
        let pixels = rgba_image.into_raw();

        frames.push(AseFrame {
            index: i,
            duration_ms,
            pixels,
        });
    }

    // Extract tags.
    let mut tags = Vec::with_capacity(ase.num_tags() as usize);
    for i in 0..ase.num_tags() {
        let tag = ase.tag(i);
        tags.push(AseTag {
            name: tag.name().to_string(),
            from_frame: tag.from_frame(),
            to_frame: tag.to_frame(),
        });
    }

    Ok(AsepriteData {
        width,
        height,
        frames,
        tags,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn aseprite_load_error_missing_file() {
        let path = PathBuf::from("/nonexistent/path/to/file.ase");
        let result = load_aseprite(&path);
        assert!(result.is_err());
        match result {
            Err(AnimPackError::AsepriteParse(msg)) => {
                assert!(!msg.is_empty(), "Error message should not be empty");
            }
            other => panic!("Expected AsepriteParse error, got: {other:?}"),
        }
    }

    #[test]
    fn ase_frame_struct_fields() {
        let frame = AseFrame {
            index: 0,
            duration_ms: 100,
            pixels: vec![255, 0, 0, 255],
        };
        assert_eq!(frame.index, 0);
        assert_eq!(frame.duration_ms, 100);
        assert_eq!(frame.pixels.len(), 4);
    }

    #[test]
    fn ase_tag_struct_fields() {
        let tag = AseTag {
            name: "idle".to_string(),
            from_frame: 0,
            to_frame: 3,
        };
        assert_eq!(tag.name, "idle");
        assert_eq!(tag.from_frame, 0);
        assert_eq!(tag.to_frame, 3);
    }

    #[test]
    fn aseprite_data_struct_fields() {
        let data = AsepriteData {
            width: 64,
            height: 64,
            frames: vec![],
            tags: vec![],
        };
        assert_eq!(data.width, 64);
        assert_eq!(data.height, 64);
        assert!(data.frames.is_empty());
        assert!(data.tags.is_empty());
    }
}
