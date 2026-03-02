// @id: MGE-AnimPack-Packer @do: atlas-packing @role: back-end @layer: 6 @human: francois
//! Shelf-first atlas packer for animation frames.
//!
//! Packs named rectangular frames into a power-of-two atlas using
//! a shelf-first algorithm (rows sorted by descending height).
//! This is the same algorithm used by `mge-packer`.

use crate::errors::AnimPackError;

/// A frame placed within the atlas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackedFrame {
    /// Frame name (e.g. "fallen_idle_s_000.png").
    pub name: String,
    /// X position in the atlas (pixels).
    pub x: u32,
    /// Y position in the atlas (pixels).
    pub y: u32,
    /// Frame width (pixels).
    pub width: u32,
    /// Frame height (pixels).
    pub height: u32,
}

/// Result of packing frames into an atlas.
#[derive(Debug, Clone)]
pub struct PackResult {
    /// Atlas width (pixels).
    pub atlas_width: u32,
    /// Atlas height (pixels).
    pub atlas_height: u32,
    /// Placed frames with their positions.
    pub frames: Vec<PackedFrame>,
}

/// Input frame for packing: (name, width, height).
pub type FrameSpec = (String, u32, u32);

/// Pack frames into an atlas using shelf-first bin packing.
///
/// - `frames`: list of (name, width, height) to pack.
/// - `max_size`: maximum atlas dimension (width and height).
/// - `margin`: pixel gap between frames.
///
/// Frames are sorted by descending height before placement.
/// Returns the placement map; does not produce pixel data.
pub fn pack_frames(
    frames: &[FrameSpec],
    max_size: u32,
    margin: u32,
) -> Result<PackResult, AnimPackError> {
    if frames.is_empty() {
        return Err(AnimPackError::NoFrames);
    }

    // Sort by descending height for better shelf utilisation.
    let mut sorted: Vec<(usize, &FrameSpec)> = frames.iter().enumerate().collect();
    sorted.sort_by(|a, b| b.1 .2.cmp(&a.1 .2));

    let mut placed: Vec<PackedFrame> = Vec::with_capacity(frames.len());
    let mut cursor_x: u32 = margin;
    let mut cursor_y: u32 = margin;
    let mut shelf_height: u32 = 0;

    for (_orig_idx, (name, w, h)) in &sorted {
        let needed_w = w + margin;
        let needed_h = h + margin;

        // Start a new shelf if the frame does not fit horizontally.
        if cursor_x + needed_w > max_size {
            cursor_x = margin;
            cursor_y += shelf_height + margin;
            shelf_height = 0;
        }

        // Check vertical overflow.
        if cursor_y + needed_h > max_size {
            return Err(AnimPackError::AtlasOverflow { max_size });
        }

        placed.push(PackedFrame {
            name: name.clone(),
            x: cursor_x,
            y: cursor_y,
            width: *w,
            height: *h,
        });

        cursor_x += needed_w;
        if *h > shelf_height {
            shelf_height = *h;
        }
    }

    // Calculate actual atlas dimensions (tight bounding box).
    let atlas_width = placed
        .iter()
        .map(|r| r.x + r.width + margin)
        .max()
        .unwrap_or(margin)
        .min(max_size);

    let atlas_height = placed
        .iter()
        .map(|r| r.y + r.height + margin)
        .max()
        .unwrap_or(margin);

    Ok(PackResult {
        atlas_width,
        atlas_height,
        frames: placed,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_single_frame() {
        let frames = vec![("frame_0".to_string(), 64, 64)];
        let result = pack_frames(&frames, 256, 1).unwrap();
        assert_eq!(result.frames.len(), 1);
        let f = &result.frames[0];
        assert_eq!(f.name, "frame_0");
        assert_eq!(f.x, 1); // margin
        assert_eq!(f.y, 1); // margin
        assert_eq!(f.width, 64);
        assert_eq!(f.height, 64);
        assert!(result.atlas_width <= 256);
        assert!(result.atlas_height <= 256);
    }

    #[test]
    fn pack_multiple_frames() {
        let frames = vec![
            ("a".to_string(), 64, 64),
            ("b".to_string(), 64, 64),
            ("c".to_string(), 64, 64),
            ("d".to_string(), 64, 64),
        ];
        let result = pack_frames(&frames, 512, 0).unwrap();
        assert_eq!(result.frames.len(), 4);

        // Verify no overlap between any pair.
        for i in 0..result.frames.len() {
            for j in (i + 1)..result.frames.len() {
                let a = &result.frames[i];
                let b = &result.frames[j];
                let no_overlap = a.x + a.width <= b.x
                    || b.x + b.width <= a.x
                    || a.y + a.height <= b.y
                    || b.y + b.height <= a.y;
                assert!(
                    no_overlap,
                    "Frames {} and {} overlap: ({},{} {}x{}) vs ({},{} {}x{})",
                    a.name, b.name, a.x, a.y, a.width, a.height, b.x, b.y, b.width, b.height
                );
            }
        }
    }

    #[test]
    fn pack_overflow() {
        // 4 frames of 200x200 cannot fit in a 256x256 atlas.
        let frames = vec![
            ("a".to_string(), 200, 200),
            ("b".to_string(), 200, 200),
            ("c".to_string(), 200, 200),
            ("d".to_string(), 200, 200),
        ];
        let result = pack_frames(&frames, 256, 0);
        assert!(result.is_err());
        match result {
            Err(AnimPackError::AtlasOverflow { max_size }) => {
                assert_eq!(max_size, 256);
            }
            other => panic!("Expected AtlasOverflow, got: {other:?}"),
        }
    }

    #[test]
    fn pack_no_frames() {
        let result = pack_frames(&[], 256, 0);
        assert!(result.is_err());
        assert!(matches!(result, Err(AnimPackError::NoFrames)));
    }

    #[test]
    fn pack_with_margin() {
        let frames = vec![
            ("a".to_string(), 32, 32),
            ("b".to_string(), 32, 32),
        ];
        let result = pack_frames(&frames, 256, 2).unwrap();
        assert_eq!(result.frames.len(), 2);

        // First frame starts at margin (2,2).
        let a = &result.frames[0];
        assert_eq!(a.x, 2);
        assert_eq!(a.y, 2);

        // Second frame should be offset by width + margin.
        let b = &result.frames[1];
        assert_eq!(b.x, 2 + 32 + 2); // margin + width + margin
        assert_eq!(b.y, 2);
    }

    #[test]
    fn pack_shelf_new_row() {
        // max_size = 80, margin = 0. Three 30-wide frames: first two fit on row,
        // third goes to next shelf.
        let frames = vec![
            ("a".to_string(), 30, 20),
            ("b".to_string(), 30, 20),
            ("c".to_string(), 30, 20),
        ];
        let result = pack_frames(&frames, 80, 0).unwrap();
        assert_eq!(result.frames.len(), 3);

        // All same height, so sorted order is stable.
        // First two on row 0, third on row 1.
        let rows: Vec<u32> = result.frames.iter().map(|f| f.y).collect();
        let on_row_0 = rows.iter().filter(|&&y| y == 0).count();
        let on_row_1 = rows.iter().filter(|&&y| y == 20).count();
        assert_eq!(on_row_0, 2);
        assert_eq!(on_row_1, 1);
    }
}
