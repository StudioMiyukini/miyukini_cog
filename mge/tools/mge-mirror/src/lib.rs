// @id: MGE-Mirror @do: image-mirror @role: back-end @layer: 6 @human: miyuk
//! Sprite mirroring -- horizontal and vertical axis flip for RGBA pixel data.
#![deny(unsafe_code)]

/// Mirror axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum MirrorAxis {
    /// Flip left-right (reverse pixel order in each row).
    Horizontal,
    /// Flip top-bottom (reverse row order).
    Vertical,
}

/// Error types for the mirrorer.
#[derive(Debug, thiserror::Error)]
pub enum MirrorError {
    /// The input pixel slice is empty.
    #[error("Input pixel data is empty")]
    EmptyInput,
    /// Pixel data length does not match `width * height * 4`.
    #[error("Invalid pixel data: expected {expected} bytes, got {got}")]
    InvalidPixelData {
        /// Expected byte count.
        expected: usize,
        /// Actual byte count.
        got: usize,
    },
}

/// Stateless image mirrorer.
pub struct Mirrorer;

impl Mirrorer {
    /// Mirror RGBA pixels along the given axis.
    pub fn mirror(
        pixels: &[u8],
        width: u32,
        height: u32,
        axis: MirrorAxis,
    ) -> Result<Vec<u8>, MirrorError> {
        if pixels.is_empty() {
            return Err(MirrorError::EmptyInput);
        }
        let expected = (width * height * 4) as usize;
        if pixels.len() != expected {
            return Err(MirrorError::InvalidPixelData {
                expected,
                got: pixels.len(),
            });
        }
        match axis {
            MirrorAxis::Horizontal => Ok(flip_h(pixels, width, height)),
            MirrorAxis::Vertical => Ok(flip_v(pixels, width, height)),
        }
    }

    /// Horizontal flip (left-right).
    pub fn flip_h(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, MirrorError> {
        Self::mirror(pixels, width, height, MirrorAxis::Horizontal)
    }

    /// Vertical flip (top-bottom).
    pub fn flip_v(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, MirrorError> {
        Self::mirror(pixels, width, height, MirrorAxis::Vertical)
    }
}

fn flip_h(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut out = vec![0u8; pixels.len()];
    for y in 0..height {
        for x in 0..width {
            let src = ((y * width + x) * 4) as usize;
            let dst_x = width - 1 - x;
            let dst = ((y * width + dst_x) * 4) as usize;
            out[dst..dst + 4].copy_from_slice(&pixels[src..src + 4]);
        }
    }
    out
}

fn flip_v(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut out = vec![0u8; pixels.len()];
    let row_bytes = (width * 4) as usize;
    for y in 0..height {
        let src_row = (y * width * 4) as usize;
        let dst_row = ((height - 1 - y) * width * 4) as usize;
        out[dst_row..dst_row + row_bytes].copy_from_slice(&pixels[src_row..src_row + row_bytes]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Horizontal flip 2x1: [R, G] -> [G, R].
    #[test]
    fn test_flip_h_2x1() {
        let pixels: Vec<u8> = vec![
            255, 0, 0, 255, // red
            0, 255, 0, 255, // green
        ];
        let out = Mirrorer::flip_h(&pixels, 2, 1).unwrap();
        assert_eq!(
            out,
            vec![
                0, 255, 0, 255, // green
                255, 0, 0, 255, // red
            ]
        );
    }

    /// Vertical flip 1x2: rows [R] [G] -> [G] [R].
    #[test]
    fn test_flip_v_1x2() {
        let pixels: Vec<u8> = vec![
            255, 0, 0, 255, // row 0: red
            0, 255, 0, 255, // row 1: green
        ];
        let out = Mirrorer::flip_v(&pixels, 1, 2).unwrap();
        assert_eq!(
            out,
            vec![
                0, 255, 0, 255, // row 0: green (was row 1)
                255, 0, 0, 255, // row 1: red   (was row 0)
            ]
        );
    }

    /// Horizontal flip 1x1: single pixel unchanged.
    #[test]
    fn test_flip_h_identity_1x1() {
        let pixels: Vec<u8> = vec![42, 84, 126, 255];
        let out = Mirrorer::flip_h(&pixels, 1, 1).unwrap();
        assert_eq!(out, pixels);
    }

    /// Empty input must return `EmptyInput`.
    #[test]
    fn test_flip_empty() {
        let err = Mirrorer::flip_h(&[], 0, 0).unwrap_err();
        assert!(matches!(err, MirrorError::EmptyInput));
    }

    /// Invalid pixel data size must return `InvalidPixelData`.
    #[test]
    fn test_flip_invalid_size() {
        let pixels: Vec<u8> = vec![255, 0, 0]; // 3 bytes, not 4
        let err = Mirrorer::flip_h(&pixels, 1, 1).unwrap_err();
        assert!(matches!(err, MirrorError::InvalidPixelData { .. }));
    }

    /// Vertical flip 2x2: verify row inversion.
    #[test]
    fn test_flip_v_2x2() {
        let pixels: Vec<u8> = vec![
            255, 0, 0, 255, 0, 255, 0, 255, // row 0: red, green
            0, 0, 255, 255, 255, 255, 0, 255, // row 1: blue, yellow
        ];
        let out = Mirrorer::flip_v(&pixels, 2, 2).unwrap();
        assert_eq!(
            out,
            vec![
                0, 0, 255, 255, 255, 255, 0, 255, // row 0: blue, yellow (was row 1)
                255, 0, 0, 255, 0, 255, 0, 255, // row 1: red, green   (was row 0)
            ]
        );
    }

    /// Double horizontal flip returns original.
    #[test]
    fn test_flip_h_double_is_identity() {
        let pixels: Vec<u8> = vec![
            255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 0, 255,
        ];
        let once = Mirrorer::flip_h(&pixels, 2, 2).unwrap();
        let twice = Mirrorer::flip_h(&once, 2, 2).unwrap();
        assert_eq!(twice, pixels);
    }
}
