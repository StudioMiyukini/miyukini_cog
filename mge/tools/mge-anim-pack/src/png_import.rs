// @id: MGE-AnimPack-PngImport @do: png-import @role: back-end @layer: 6 @human: francois
//! Import individual PNG frames with size presets.
//!
//! Scans a directory for PNG files, applies size class presets, and
//! organises frames into the standard naming convention.

use std::path::Path;

use crate::errors::AnimPackError;

/// Size presets for different entity classes.
///
/// Each preset defines (width, height) in pixels for a single frame.
/// These correspond to common sprite sizes in isometric ARPGs.
const SIZE_PRESETS: &[(&str, u32, u32)] = &[
    ("small", 64, 192),     // Small objects/critters: 1 tile wide, 3 tiles tall
    ("medium", 96, 288),    // Medium monsters: 1.5 tiles wide, 4.5 tiles tall
    ("humanoid", 128, 384), // Humanoid characters: 2 tiles wide, 6 tiles tall
    ("large", 192, 576),    // Large monsters: 3 tiles wide, 9 tiles tall
    ("boss", 256, 768),     // Boss monsters: 4 tiles wide, 12 tiles tall
];

/// A scanned PNG frame from disk.
#[derive(Debug, Clone)]
pub struct PngFrame {
    /// Original filename.
    pub filename: String,
    /// Full file path.
    pub path: std::path::PathBuf,
}

/// Configuration for the PNG import pipeline.
#[derive(Debug, Clone)]
pub struct ImportConfig {
    /// Input directory containing PNG files.
    pub input_dir: std::path::PathBuf,
    /// Output directory.
    pub output_dir: std::path::PathBuf,
    /// Entity identifier.
    pub entity: String,
    /// Size class preset name.
    pub size_class: String,
    /// Animation action/state.
    pub action: String,
    /// Number of directions (4 or 8).
    pub directions: u32,
    /// Frames per second.
    pub fps: u32,
    /// Whether to also generate an Aseprite file.
    pub generate_aseprite: bool,
}

/// Result of the import pipeline.
#[derive(Debug)]
pub struct ImportResult {
    /// Number of frames processed.
    pub frame_count: usize,
    /// Width of each frame (from preset).
    pub frame_width: u32,
    /// Height of each frame (from preset).
    pub frame_height: u32,
    /// Renamed frame files.
    pub renamed_frames: Vec<String>,
}

/// Return the (width, height) for a given size class preset.
pub fn apply_size_preset(size_class: &str) -> Result<(u32, u32), AnimPackError> {
    SIZE_PRESETS
        .iter()
        .find(|(name, _, _)| *name == size_class)
        .map(|(_, w, h)| (*w, *h))
        .ok_or_else(|| AnimPackError::UnknownSizeClass(size_class.to_string()))
}

/// Scan a directory for PNG files, sorted alphabetically.
pub fn scan_png_dir(dir: &Path) -> Result<Vec<PngFrame>, AnimPackError> {
    if !dir.exists() {
        return Err(AnimPackError::Io {
            path: dir.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Directory does not exist",
            ),
        });
    }

    let mut frames: Vec<PngFrame> = std::fs::read_dir(dir)
        .map_err(|e| AnimPackError::io(dir, e))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "png") {
                let filename = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                Some(PngFrame { filename, path })
            } else {
                None
            }
        })
        .collect();

    frames.sort_by(|a, b| a.filename.cmp(&b.filename));
    Ok(frames)
}

/// Run the full PNG import pipeline.
///
/// Scans the input directory, applies the size preset, and generates
/// renamed frames following the naming convention.
pub fn import_pngs(config: &ImportConfig) -> Result<ImportResult, AnimPackError> {
    let (frame_width, frame_height) = apply_size_preset(&config.size_class)?;
    let pngs = scan_png_dir(&config.input_dir)?;

    let directions: &[&str] = if config.directions == 8 {
        &["s", "sw", "w", "nw", "n", "ne", "e", "se"]
    } else {
        &["s", "sw", "w", "nw"]
    };

    // Calculate frames per direction.
    let total_pngs = pngs.len();
    let dir_count = directions.len();
    if dir_count == 0 {
        return Err(AnimPackError::NoFrames);
    }
    let frames_per_dir = total_pngs / dir_count;

    let mut renamed_frames = Vec::with_capacity(total_pngs);

    for (i, png) in pngs.iter().enumerate() {
        let dir_idx = i / frames_per_dir.max(1);
        let frame_idx = i % frames_per_dir.max(1);
        let dir = directions.get(dir_idx).unwrap_or(&"s");

        let new_name = crate::naming::format_frame_name(
            &config.entity,
            &config.action,
            dir,
            frame_idx as u32,
        );
        renamed_frames.push(new_name);

        // Log the mapping (actual file copy would happen here in production).
        tracing::debug!(
            "  {} -> {} (dir={dir}, frame={frame_idx})",
            png.filename,
            renamed_frames.last().unwrap_or(&String::new()),
        );
    }

    Ok(ImportResult {
        frame_count: total_pngs,
        frame_width,
        frame_height,
        renamed_frames,
    })
}

/// List available size presets.
pub fn list_presets() -> Vec<(&'static str, u32, u32)> {
    SIZE_PRESETS.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_size_preset_small() {
        let (w, h) = apply_size_preset("small").unwrap();
        assert_eq!(w, 64);
        assert_eq!(h, 192);
    }

    #[test]
    fn apply_size_preset_humanoid() {
        let (w, h) = apply_size_preset("humanoid").unwrap();
        assert_eq!(w, 128);
        assert_eq!(h, 384);
    }

    #[test]
    fn apply_size_preset_boss() {
        let (w, h) = apply_size_preset("boss").unwrap();
        assert_eq!(w, 256);
        assert_eq!(h, 768);
    }

    #[test]
    fn apply_size_preset_invalid() {
        let result = apply_size_preset("unknown");
        assert!(result.is_err());
        match result {
            Err(AnimPackError::UnknownSizeClass(name)) => {
                assert_eq!(name, "unknown");
            }
            other => panic!("Expected UnknownSizeClass, got: {other:?}"),
        }
    }

    #[test]
    fn scan_png_dir_nonexistent() {
        let result = scan_png_dir(Path::new("/nonexistent/path/to/dir"));
        assert!(result.is_err());
    }

    #[test]
    fn scan_png_dir_empty() {
        // Create a temporary empty directory.
        let tmp = std::env::temp_dir().join("mge_anim_pack_test_empty");
        let _ = std::fs::create_dir_all(&tmp);
        let result = scan_png_dir(&tmp).unwrap();
        assert_eq!(result.len(), 0, "Expected 0 frames in empty directory");
        let _ = std::fs::remove_dir(&tmp);
    }

    #[test]
    fn list_presets_has_all() {
        let presets = list_presets();
        assert_eq!(presets.len(), 5);
        let names: Vec<&str> = presets.iter().map(|(n, _, _)| *n).collect();
        assert!(names.contains(&"small"));
        assert!(names.contains(&"medium"));
        assert!(names.contains(&"humanoid"));
        assert!(names.contains(&"large"));
        assert!(names.contains(&"boss"));
    }

    #[test]
    fn png_frame_struct_fields() {
        let frame = PngFrame {
            filename: "test.png".to_string(),
            path: std::path::PathBuf::from("/tmp/test.png"),
        };
        assert_eq!(frame.filename, "test.png");
    }

    #[test]
    fn import_config_struct_fields() {
        let config = ImportConfig {
            input_dir: std::path::PathBuf::from("/tmp/input"),
            output_dir: std::path::PathBuf::from("/tmp/output"),
            entity: "fallen".to_string(),
            size_class: "humanoid".to_string(),
            action: "idle".to_string(),
            directions: 4,
            fps: 12,
            generate_aseprite: false,
        };
        assert_eq!(config.entity, "fallen");
        assert_eq!(config.directions, 4);
    }
}
