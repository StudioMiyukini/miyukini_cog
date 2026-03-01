// @id: MGE-Packer @do: atlas-packing @role: back-end @layer: 6 @human: miyuk
//! Atlas packer: assembles individual PNG frames into a sprite sheet.
//!
//! Uses shelf-first bin packing (rows sorted by frame height).
#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Error returned by the atlas packer.
#[derive(Debug, thiserror::Error)]
pub enum PackerError {
    /// No frames were provided to pack.
    #[error("No frames provided")]
    NoFrames,
    /// The atlas maximum size is too small to fit all frames.
    #[error("Atlas too small: need at least {needed_width}x{needed_height} pixels")]
    AtlasTooSmall {
        /// Minimum width needed.
        needed_width: u32,
        /// Minimum height needed.
        needed_height: u32,
    },
    /// A frame has zero-size dimensions or empty pixel data.
    #[error("Frame '{id}' has zero-size pixels")]
    EmptyFrame {
        /// Identifier of the problematic frame.
        id: String,
    },
}

/// A single input frame to pack.
pub struct FrameInput {
    /// Unique frame identifier (e.g. "player_run_0").
    pub id: String,
    /// Raw RGBA pixels (width x height x 4 bytes).
    pub pixels: Vec<u8>,
    /// Frame width in pixels.
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
}

impl FrameInput {
    /// Create a new frame from raw RGBA pixels.
    pub fn new(id: impl Into<String>, pixels: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            id: id.into(),
            pixels,
            width,
            height,
        }
    }
}

/// A placed frame rect inside the assembled atlas.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PackedRect {
    /// Frame identifier matching the input.
    pub id: String,
    /// X position in the atlas.
    pub x: u32,
    /// Y position in the atlas.
    pub y: u32,
    /// Width of the placed frame.
    pub width: u32,
    /// Height of the placed frame.
    pub height: u32,
}

/// The assembled atlas: pixel data + frame placement map.
#[derive(Debug)]
pub struct PackedAtlas {
    /// Total atlas width in pixels.
    pub width: u32,
    /// Total atlas height in pixels.
    pub height: u32,
    /// Placement of each frame within the atlas.
    pub rects: Vec<PackedRect>,
    /// Raw RGBA pixels of the full atlas image.
    pub pixels: Vec<u8>,
}

impl PackedAtlas {
    /// Sample the RGBA colour at atlas position (x, y).
    pub fn pixel_at(&self, x: u32, y: u32) -> Option<[u8; 4]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let base = ((y * self.width + x) * 4) as usize;
        if base + 3 >= self.pixels.len() {
            return None;
        }
        Some([
            self.pixels[base],
            self.pixels[base + 1],
            self.pixels[base + 2],
            self.pixels[base + 3],
        ])
    }
}

/// Shelf-first atlas packer.
pub struct AtlasPacker {
    max_size: u32,
    margin: u32,
}

impl AtlasPacker {
    /// Create a packer.
    ///
    /// * `max_size` -- maximum atlas side length in pixels.
    /// * `margin` -- pixel gap between frames.
    pub fn new(max_size: u32, margin: u32) -> Self {
        Self { max_size, margin }
    }

    /// Pack frames into an atlas.
    pub fn pack(&self, mut frames: Vec<FrameInput>) -> Result<PackedAtlas, PackerError> {
        if frames.is_empty() {
            return Err(PackerError::NoFrames);
        }
        for f in &frames {
            if f.width == 0 || f.height == 0 || f.pixels.is_empty() {
                return Err(PackerError::EmptyFrame { id: f.id.clone() });
            }
        }

        // Sort frames by height descending (shelf packing heuristic)
        frames.sort_by(|a, b| b.height.cmp(&a.height));

        // --- Shelf-first placement ---
        let mut rects: Vec<PackedRect> = Vec::with_capacity(frames.len());
        let mut cursor_x: u32 = self.margin;
        let mut cursor_y: u32 = self.margin;
        let mut shelf_height: u32 = 0;

        for frame in &frames {
            let needed_w = frame.width + self.margin;
            let needed_h = frame.height + self.margin;

            // Start a new shelf if frame doesn't fit horizontally
            if cursor_x + needed_w > self.max_size {
                cursor_x = self.margin;
                cursor_y += shelf_height + self.margin;
                shelf_height = 0;
            }

            // Check vertical overflow
            if cursor_y + needed_h > self.max_size {
                let min_needed = cursor_y + needed_h;
                return Err(PackerError::AtlasTooSmall {
                    needed_width: min_needed,
                    needed_height: min_needed,
                });
            }

            rects.push(PackedRect {
                id: frame.id.clone(),
                x: cursor_x,
                y: cursor_y,
                width: frame.width,
                height: frame.height,
            });

            cursor_x += needed_w;
            if frame.height > shelf_height {
                shelf_height = frame.height;
            }
        }

        // Calculate actual atlas dimensions
        let atlas_width = self.max_size.min(
            rects
                .iter()
                .map(|r| r.x + r.width + self.margin)
                .max()
                .unwrap_or(self.margin),
        );
        let atlas_height = rects
            .iter()
            .map(|r| r.y + r.height + self.margin)
            .max()
            .unwrap_or(self.margin);

        // Assemble pixels (RGBA, zeroed = transparent black)
        let mut pixels = vec![0u8; (atlas_width * atlas_height * 4) as usize];

        for (rect, frame) in rects.iter().zip(frames.iter()) {
            for row in 0..frame.height {
                let src_start = (row * frame.width * 4) as usize;
                let src_end = src_start + (frame.width * 4) as usize;
                let dst_row = rect.y + row;
                let dst_start = ((dst_row * atlas_width + rect.x) * 4) as usize;
                let dst_end = dst_start + (frame.width * 4) as usize;
                pixels[dst_start..dst_end]
                    .copy_from_slice(&frame.pixels[src_start..src_end]);
            }
        }

        Ok(PackedAtlas {
            width: atlas_width,
            height: atlas_height,
            rects,
            pixels,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: create a solid RGBA frame of the given colour.
    fn make_frame(id: &str, w: u32, h: u32, rgba: [u8; 4]) -> FrameInput {
        let pixel_count = (w * h) as usize;
        let mut pixels = Vec::with_capacity(pixel_count * 4);
        for _ in 0..pixel_count {
            pixels.extend_from_slice(&rgba);
        }
        FrameInput::new(id, pixels, w, h)
    }

    #[test]
    fn test_pack_empty_input() {
        let packer = AtlasPacker::new(256, 0);
        let result = packer.pack(vec![]);
        assert!(result.is_err());
        assert!(
            matches!(result.unwrap_err(), PackerError::NoFrames),
            "Expected PackerError::NoFrames"
        );
    }

    #[test]
    fn test_pack_single_frame() {
        let packer = AtlasPacker::new(256, 0);
        let frame = make_frame("single", 8, 8, [255, 0, 0, 255]);
        let atlas = packer.pack(vec![frame]).unwrap();
        assert_eq!(atlas.rects.len(), 1);
        assert!(!atlas.pixels.is_empty());
        assert_eq!(atlas.rects[0].id, "single");
    }

    #[test]
    fn test_pack_multiple_frames() {
        let packer = AtlasPacker::new(256, 1);
        let frames = vec![
            make_frame("a", 16, 16, [255, 0, 0, 255]),
            make_frame("b", 16, 16, [0, 255, 0, 255]),
            make_frame("c", 16, 16, [0, 0, 255, 255]),
        ];
        let atlas = packer.pack(frames).unwrap();
        assert_eq!(atlas.rects.len(), 3);

        // Verify no overlap between any pair of rects
        for i in 0..atlas.rects.len() {
            for j in (i + 1)..atlas.rects.len() {
                let a = &atlas.rects[i];
                let b = &atlas.rects[j];
                let no_overlap = a.x + a.width <= b.x
                    || b.x + b.width <= a.x
                    || a.y + a.height <= b.y
                    || b.y + b.height <= a.y;
                assert!(no_overlap, "Rects {i} and {j} overlap");
            }
        }
    }

    #[test]
    fn test_pack_empty_frame() {
        let packer = AtlasPacker::new(256, 0);
        let frame = FrameInput::new("bad", vec![], 0, 0);
        let result = packer.pack(vec![frame]);
        assert!(result.is_err());
        match result.unwrap_err() {
            PackerError::EmptyFrame { id } => assert_eq!(id, "bad"),
            other => panic!("Expected EmptyFrame, got {other:?}"),
        }
    }

    #[test]
    fn test_packed_rect_fields() {
        let packer = AtlasPacker::new(256, 0);
        let frame = make_frame("rect_check", 10, 20, [128, 128, 128, 255]);
        let atlas = packer.pack(vec![frame]).unwrap();
        let rect = &atlas.rects[0];
        assert_eq!(rect.id, "rect_check");
        assert_eq!(rect.x, 0);
        assert_eq!(rect.y, 0);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn test_atlas_pixel_at_valid() {
        // Create a tiny 2x2 atlas with known pixel data
        let atlas = PackedAtlas {
            width: 2,
            height: 2,
            rects: vec![],
            pixels: vec![
                255, 0, 0, 255, // (0,0) red
                0, 255, 0, 255, // (1,0) green
                0, 0, 255, 255, // (0,1) blue
                255, 255, 0, 255, // (1,1) yellow
            ],
        };
        assert_eq!(atlas.pixel_at(0, 0), Some([255, 0, 0, 255]));
        assert_eq!(atlas.pixel_at(1, 0), Some([0, 255, 0, 255]));
        assert_eq!(atlas.pixel_at(0, 1), Some([0, 0, 255, 255]));
        assert_eq!(atlas.pixel_at(1, 1), Some([255, 255, 0, 255]));
    }

    #[test]
    fn test_atlas_pixel_at_out_of_bounds() {
        let atlas = PackedAtlas {
            width: 2,
            height: 2,
            rects: vec![],
            pixels: vec![0; 16],
        };
        assert_eq!(atlas.pixel_at(2, 0), None);
        assert_eq!(atlas.pixel_at(0, 2), None);
        assert_eq!(atlas.pixel_at(100, 100), None);
    }

    #[test]
    fn test_pack_shelf_new_row() {
        // max_size = 20, margin = 0. Two 12-wide frames must go on separate shelves.
        let packer = AtlasPacker::new(20, 0);
        let frames = vec![
            make_frame("wide_a", 12, 8, [255, 0, 0, 255]),
            make_frame("wide_b", 12, 8, [0, 255, 0, 255]),
        ];
        let atlas = packer.pack(frames).unwrap();
        assert_eq!(atlas.rects.len(), 2);

        // Both frames have the same height, so sorting doesn't change order.
        // First frame should be at y=0, second at y=8 (new shelf).
        let r0 = &atlas.rects[0];
        let r1 = &atlas.rects[1];
        assert_eq!(r0.y, 0);
        assert_eq!(r1.y, 8, "Second frame should be on a new shelf row");
        assert_eq!(r1.x, 0, "Second frame should start at x=0 on new shelf");
    }
}
