// @id: MGE-Slicer @do: atlas-slicing @role: back-end @layer: 6 @human: miyuk
//! Atlas slicer: extracts sub-images from a sprite sheet.
#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Slicer error types.
#[derive(Debug, thiserror::Error)]
pub enum SlicerError {
    /// The requested slice exceeds atlas boundaries.
    #[error("Slice '{id}' at ({x},{y}) + {w}x{h} exceeds atlas {atlas_w}x{atlas_h}")]
    OutOfBounds {
        /// Slice identifier.
        id: String,
        /// X origin of the slice.
        x: u32,
        /// Y origin of the slice.
        y: u32,
        /// Width of the slice.
        w: u32,
        /// Height of the slice.
        h: u32,
        /// Atlas width.
        atlas_w: u32,
        /// Atlas height.
        atlas_h: u32,
    },
    /// The atlas pixel buffer size does not match the declared dimensions.
    #[error("Atlas pixel data size is invalid (expected {expected}, got {got})")]
    InvalidAtlasSize {
        /// Expected byte count (width * height * 4).
        expected: usize,
        /// Actual byte count received.
        got: usize,
    },
    /// A slice definition has zero width or height.
    #[error("Slice has zero width or height")]
    EmptySlice,
}

/// A region in an atlas to extract as a single frame.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SliceDef {
    /// Unique slice identifier.
    pub id: String,
    /// X position in the atlas.
    pub x: u32,
    /// Y position in the atlas.
    pub y: u32,
    /// Width of the slice.
    pub width: u32,
    /// Height of the slice.
    pub height: u32,
}

impl SliceDef {
    /// Create a new slice definition.
    pub fn new(id: impl Into<String>, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            id: id.into(),
            x,
            y,
            width,
            height,
        }
    }
}

/// Stateless atlas slicer.
pub struct AtlasSlicer;

impl AtlasSlicer {
    /// Extract a single slice from atlas RGBA pixels.
    ///
    /// Returns the extracted frame as RGBA pixel bytes.
    pub fn extract(
        atlas_pixels: &[u8],
        atlas_width: u32,
        atlas_height: u32,
        slice: &SliceDef,
    ) -> Result<Vec<u8>, SlicerError> {
        if slice.width == 0 || slice.height == 0 {
            return Err(SlicerError::EmptySlice);
        }
        let expected = (atlas_width * atlas_height * 4) as usize;
        if atlas_pixels.len() != expected {
            return Err(SlicerError::InvalidAtlasSize {
                expected,
                got: atlas_pixels.len(),
            });
        }
        if slice.x + slice.width > atlas_width || slice.y + slice.height > atlas_height {
            return Err(SlicerError::OutOfBounds {
                id: slice.id.clone(),
                x: slice.x,
                y: slice.y,
                w: slice.width,
                h: slice.height,
                atlas_w: atlas_width,
                atlas_h: atlas_height,
            });
        }

        let mut out = vec![0u8; (slice.width * slice.height * 4) as usize];
        for row in 0..slice.height {
            let src_row = slice.y + row;
            let src_start = ((src_row * atlas_width + slice.x) * 4) as usize;
            let src_end = src_start + (slice.width * 4) as usize;
            let dst_start = (row * slice.width * 4) as usize;
            let dst_end = dst_start + (slice.width * 4) as usize;
            out[dst_start..dst_end].copy_from_slice(&atlas_pixels[src_start..src_end]);
        }
        Ok(out)
    }

    /// Extract all slices from an atlas.
    pub fn extract_all(
        atlas_pixels: &[u8],
        atlas_width: u32,
        atlas_height: u32,
        slices: &[SliceDef],
    ) -> Result<Vec<(String, Vec<u8>)>, SlicerError> {
        slices
            .iter()
            .map(|s| {
                Self::extract(atlas_pixels, atlas_width, atlas_height, s)
                    .map(|pixels| (s.id.clone(), pixels))
            })
            .collect()
    }

    /// Generate uniform grid slice definitions (e.g., for sprite sheets).
    pub fn uniform_grid(
        frame_width: u32,
        frame_height: u32,
        cols: u32,
        rows: u32,
    ) -> Vec<SliceDef> {
        let mut defs = Vec::with_capacity((cols * rows) as usize);
        for row in 0..rows {
            for col in 0..cols {
                let idx = row * cols + col;
                defs.push(SliceDef {
                    id: format!("frame_{idx}"),
                    x: col * frame_width,
                    y: row * frame_height,
                    width: frame_width,
                    height: frame_height,
                });
            }
        }
        defs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a solid RGBA atlas of the given dimensions and colour.
    fn make_atlas(w: u32, h: u32, rgba: [u8; 4]) -> Vec<u8> {
        let count = (w * h) as usize;
        let mut pixels = Vec::with_capacity(count * 4);
        for _ in 0..count {
            pixels.extend_from_slice(&rgba);
        }
        pixels
    }

    /// Helper: create a gradient atlas where each pixel encodes its (x, y) position.
    fn make_position_atlas(w: u32, h: u32) -> Vec<u8> {
        let mut pixels = Vec::with_capacity((w * h * 4) as usize);
        for y in 0..h {
            for x in 0..w {
                pixels.push(x as u8);
                pixels.push(y as u8);
                pixels.push(0);
                pixels.push(255);
            }
        }
        pixels
    }

    #[test]
    fn test_extract_single_pixel() {
        let atlas = make_atlas(1, 1, [42, 84, 126, 255]);
        let slice = SliceDef::new("px", 0, 0, 1, 1);
        let result = AtlasSlicer::extract(&atlas, 1, 1, &slice).unwrap();
        assert_eq!(result, vec![42, 84, 126, 255]);
    }

    #[test]
    fn test_extract_region() {
        // 4x4 atlas with position-encoded pixels; extract 2x2 from (1,1)
        let atlas = make_position_atlas(4, 4);
        let slice = SliceDef::new("sub", 1, 1, 2, 2);
        let result = AtlasSlicer::extract(&atlas, 4, 4, &slice).unwrap();

        // Expected pixels: (1,1), (2,1), (1,2), (2,2)
        let expected: Vec<u8> = vec![
            1, 1, 0, 255, // (1,1)
            2, 1, 0, 255, // (2,1)
            1, 2, 0, 255, // (1,2)
            2, 2, 0, 255, // (2,2)
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_out_of_bounds() {
        let atlas = make_atlas(4, 4, [0, 0, 0, 255]);
        let slice = SliceDef::new("oob", 3, 3, 2, 2);
        let result = AtlasSlicer::extract(&atlas, 4, 4, &slice);
        assert!(result.is_err());
        match result.unwrap_err() {
            SlicerError::OutOfBounds { id, .. } => assert_eq!(id, "oob"),
            other => panic!("Expected OutOfBounds, got {other:?}"),
        }
    }

    #[test]
    fn test_extract_empty_slice() {
        let atlas = make_atlas(4, 4, [0, 0, 0, 255]);
        let slice = SliceDef::new("empty", 0, 0, 0, 0);
        let result = AtlasSlicer::extract(&atlas, 4, 4, &slice);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), SlicerError::EmptySlice));
    }

    #[test]
    fn test_extract_invalid_atlas_size() {
        // Atlas claims 4x4 but only has 8 bytes (not 64)
        let bad_atlas = vec![0u8; 8];
        let slice = SliceDef::new("inv", 0, 0, 1, 1);
        let result = AtlasSlicer::extract(&bad_atlas, 4, 4, &slice);
        assert!(result.is_err());
        match result.unwrap_err() {
            SlicerError::InvalidAtlasSize { expected, got } => {
                assert_eq!(expected, 64);
                assert_eq!(got, 8);
            }
            other => panic!("Expected InvalidAtlasSize, got {other:?}"),
        }
    }

    #[test]
    fn test_extract_all() {
        // 4x2 atlas, extract two 2x2 slices side by side
        let atlas = make_position_atlas(4, 2);
        let slices = vec![
            SliceDef::new("left", 0, 0, 2, 2),
            SliceDef::new("right", 2, 0, 2, 2),
        ];
        let results = AtlasSlicer::extract_all(&atlas, 4, 2, &slices).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "left");
        assert_eq!(results[1].0, "right");

        // Verify left slice contains (0,0), (1,0), (0,1), (1,1)
        let left_expected: Vec<u8> = vec![
            0, 0, 0, 255, // (0,0)
            1, 0, 0, 255, // (1,0)
            0, 1, 0, 255, // (0,1)
            1, 1, 0, 255, // (1,1)
        ];
        assert_eq!(results[0].1, left_expected);

        // Verify right slice contains (2,0), (3,0), (2,1), (3,1)
        let right_expected: Vec<u8> = vec![
            2, 0, 0, 255, // (2,0)
            3, 0, 0, 255, // (3,0)
            2, 1, 0, 255, // (2,1)
            3, 1, 0, 255, // (3,1)
        ];
        assert_eq!(results[1].1, right_expected);
    }

    #[test]
    fn test_uniform_grid_2x2() {
        let defs = AtlasSlicer::uniform_grid(16, 16, 2, 2);
        assert_eq!(defs.len(), 4);

        assert_eq!(defs[0].x, 0);
        assert_eq!(defs[0].y, 0);
        assert_eq!(defs[1].x, 16);
        assert_eq!(defs[1].y, 0);
        assert_eq!(defs[2].x, 0);
        assert_eq!(defs[2].y, 16);
        assert_eq!(defs[3].x, 16);
        assert_eq!(defs[3].y, 16);

        for def in &defs {
            assert_eq!(def.width, 16);
            assert_eq!(def.height, 16);
        }
    }

    #[test]
    fn test_uniform_grid_id_format() {
        let defs = AtlasSlicer::uniform_grid(8, 8, 2, 2);
        assert_eq!(defs[0].id, "frame_0");
        assert_eq!(defs[1].id, "frame_1");
        assert_eq!(defs[2].id, "frame_2");
        assert_eq!(defs[3].id, "frame_3");
    }
}
