// @id: MGE-AnimPack-AseExport @do: aseprite-export @role: back-end @layer: 6 @human: francois
//! Aseprite (.ase) file export from imported PNG frames.
//!
//! NOTE: The Aseprite binary format (spec at
//! <https://github.com/aseprite/aseprite/blob/main/docs/ase-file-specs.md>)
//! is complex. This module provides the public interface and data types,
//! but the actual binary writer is stubbed. Full implementation is planned
//! for a future sprint.

use std::path::Path;

use crate::errors::AnimPackError;

/// Configuration for exporting an Aseprite file.
#[derive(Debug, Clone)]
pub struct AsepriteExportConfig {
    /// Entity identifier used in the output filename.
    pub entity_id: String,
    /// Frames to write into the Aseprite file.
    pub frames: Vec<ExportFrame>,
    /// Animation tags (named frame ranges).
    pub tags: Vec<ExportTag>,
    /// Frame width in pixels.
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
}

/// A single frame to export.
#[derive(Debug, Clone)]
pub struct ExportFrame {
    /// Raw RGBA pixel data (width * height * 4 bytes).
    pub pixels: Vec<u8>,
    /// Frame duration in milliseconds.
    pub duration_ms: u32,
}

/// An animation tag for the exported file.
#[derive(Debug, Clone)]
pub struct ExportTag {
    /// Tag name (e.g. "idle", "walk").
    pub name: String,
    /// First frame index (inclusive).
    pub from_frame: u32,
    /// Last frame index (inclusive).
    pub to_frame: u32,
}

/// Export frames and tags to an Aseprite (.ase) binary file.
///
/// # Current Status
///
/// This function is **not yet implemented**. The Aseprite binary format
/// requires writing:
/// - File header (128 bytes)
/// - Frame headers with chunk counts
/// - Cel chunks with compressed pixel data (zlib deflate)
/// - Tag chunks for animation grouping
/// - Color profile and palette chunks
///
/// A full implementation requires careful byte-level writing and is
/// scheduled for a future sprint.
///
/// # Errors
///
/// Currently always returns `AnimPackError::ExportNotImplemented`.
pub fn export_aseprite(
    config: &AsepriteExportConfig,
    output: &Path,
) -> Result<(), AnimPackError> {
    // Validate config before reporting not-implemented.
    validate_config(config)?;

    // TODO: Implement Aseprite binary format writer.
    // See: https://github.com/aseprite/aseprite/blob/main/docs/ase-file-specs.md
    //
    // Implementation plan:
    // 1. Write file header: magic number 0xA5E0, frame count, width, height, etc.
    // 2. For each frame:
    //    a. Write frame header (16 bytes + chunks)
    //    b. Write cel chunk (type 0x2005) with zlib-compressed RGBA data
    //    c. Write frame duration
    // 3. Write tags chunk (type 0x2018)
    // 4. Calculate and write all size fields
    Err(AnimPackError::ExportNotImplemented(format!(
        "Aseprite export to '{}' with {} frames and {} tags",
        output.display(),
        config.frames.len(),
        config.tags.len(),
    )))
}

/// Validate that the export configuration is internally consistent.
fn validate_config(config: &AsepriteExportConfig) -> Result<(), AnimPackError> {
    if config.width == 0 || config.height == 0 {
        return Err(AnimPackError::InvalidPixelData {
            name: config.entity_id.clone(),
            expected: 1,
            got: 0,
        });
    }

    let expected_bytes = (config.width * config.height * 4) as usize;
    for (i, frame) in config.frames.iter().enumerate() {
        if frame.pixels.len() != expected_bytes {
            return Err(AnimPackError::InvalidPixelData {
                name: format!("{}:frame_{i}", config.entity_id),
                expected: expected_bytes,
                got: frame.pixels.len(),
            });
        }
    }

    for tag in &config.tags {
        if tag.to_frame < tag.from_frame {
            return Err(AnimPackError::InvalidFrameName(format!(
                "Tag '{}' has to_frame ({}) < from_frame ({})",
                tag.name, tag.to_frame, tag.from_frame
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pixels(w: u32, h: u32) -> Vec<u8> {
        vec![0u8; (w * h * 4) as usize]
    }

    #[test]
    fn aseprite_export_config_valid() {
        let config = AsepriteExportConfig {
            entity_id: "fallen".to_string(),
            frames: vec![
                ExportFrame {
                    pixels: make_pixels(16, 16),
                    duration_ms: 100,
                },
                ExportFrame {
                    pixels: make_pixels(16, 16),
                    duration_ms: 100,
                },
            ],
            tags: vec![ExportTag {
                name: "idle".to_string(),
                from_frame: 0,
                to_frame: 1,
            }],
            width: 16,
            height: 16,
        };
        assert_eq!(config.entity_id, "fallen");
        assert_eq!(config.frames.len(), 2);
        assert_eq!(config.tags.len(), 1);
        assert_eq!(config.width, 16);
        assert_eq!(config.height, 16);
    }

    #[test]
    fn aseprite_export_returns_not_implemented() {
        let config = AsepriteExportConfig {
            entity_id: "test".to_string(),
            frames: vec![ExportFrame {
                pixels: make_pixels(8, 8),
                duration_ms: 100,
            }],
            tags: vec![],
            width: 8,
            height: 8,
        };
        let result = export_aseprite(&config, Path::new("/tmp/test.ase"));
        assert!(result.is_err());
        match result {
            Err(AnimPackError::ExportNotImplemented(msg)) => {
                assert!(msg.contains("1 frames"), "Message: {msg}");
            }
            other => panic!("Expected ExportNotImplemented, got: {other:?}"),
        }
    }

    #[test]
    fn validate_config_invalid_pixel_size() {
        let config = AsepriteExportConfig {
            entity_id: "bad".to_string(),
            frames: vec![ExportFrame {
                pixels: vec![0; 10], // Wrong size for 8x8
                duration_ms: 100,
            }],
            tags: vec![],
            width: 8,
            height: 8,
        };
        let result = export_aseprite(&config, Path::new("/tmp/bad.ase"));
        assert!(result.is_err());
        assert!(matches!(result, Err(AnimPackError::InvalidPixelData { .. })));
    }

    #[test]
    fn validate_config_zero_dimensions() {
        let config = AsepriteExportConfig {
            entity_id: "zero".to_string(),
            frames: vec![],
            tags: vec![],
            width: 0,
            height: 0,
        };
        let result = export_aseprite(&config, Path::new("/tmp/zero.ase"));
        assert!(result.is_err());
    }

    #[test]
    fn validate_config_bad_tag_range() {
        let config = AsepriteExportConfig {
            entity_id: "tag_test".to_string(),
            frames: vec![],
            tags: vec![ExportTag {
                name: "bad".to_string(),
                from_frame: 5,
                to_frame: 2, // to < from
            }],
            width: 8,
            height: 8,
        };
        let result = export_aseprite(&config, Path::new("/tmp/tag.ase"));
        assert!(result.is_err());
    }

    #[test]
    fn export_frame_struct_fields() {
        let frame = ExportFrame {
            pixels: vec![255, 0, 0, 255],
            duration_ms: 150,
        };
        assert_eq!(frame.duration_ms, 150);
        assert_eq!(frame.pixels.len(), 4);
    }

    #[test]
    fn export_tag_struct_fields() {
        let tag = ExportTag {
            name: "walk".to_string(),
            from_frame: 0,
            to_frame: 7,
        };
        assert_eq!(tag.name, "walk");
        assert_eq!(tag.from_frame, 0);
        assert_eq!(tag.to_frame, 7);
    }
}
