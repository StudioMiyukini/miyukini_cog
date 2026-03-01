// @id: MGE-Rescale @do: image-rescale @role: back-end @layer: 6 @human: miyuk
//! Batch image rescaler -- nearest-neighbor and bilinear for pixel art.
#![deny(unsafe_code)]

/// Rescaling algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ScaleMode {
    /// Fast pixel-perfect rescaling (ideal for pixel art).
    NearestNeighbor,
    /// Smooth bilinear interpolation.
    Bilinear,
}

/// Error types for the rescaler.
#[derive(Debug, thiserror::Error)]
pub enum RescaleError {
    /// The input pixel slice is empty.
    #[error("Input pixel data is empty")]
    EmptyInput,
    /// Output width or height is zero.
    #[error("Output dimensions must be non-zero")]
    ZeroOutputDimension,
    /// Pixel data length does not match `width * height * 4`.
    #[error("Invalid pixel data: expected {expected} bytes, got {got}")]
    InvalidPixelData {
        /// Expected byte count.
        expected: usize,
        /// Actual byte count.
        got: usize,
    },
}

/// Image rescaler (stateless).
pub struct Rescaler {
    /// The scaling algorithm to use.
    pub mode: ScaleMode,
}

impl Rescaler {
    /// Create a new rescaler with the given algorithm.
    pub fn new(mode: ScaleMode) -> Self {
        Self { mode }
    }

    /// Rescale RGBA pixels from `src_w x src_h` to `dst_w x dst_h`.
    pub fn rescale(
        &self,
        pixels: &[u8],
        src_w: u32,
        src_h: u32,
        dst_w: u32,
        dst_h: u32,
    ) -> Result<Vec<u8>, RescaleError> {
        if pixels.is_empty() {
            return Err(RescaleError::EmptyInput);
        }
        if dst_w == 0 || dst_h == 0 {
            return Err(RescaleError::ZeroOutputDimension);
        }
        let expected = (src_w * src_h * 4) as usize;
        if pixels.len() != expected {
            return Err(RescaleError::InvalidPixelData {
                expected,
                got: pixels.len(),
            });
        }

        match self.mode {
            ScaleMode::NearestNeighbor => Ok(rescale_nearest(pixels, src_w, src_h, dst_w, dst_h)),
            ScaleMode::Bilinear => Ok(rescale_bilinear(pixels, src_w, src_h, dst_w, dst_h)),
        }
    }
}

fn sample_rgba(pixels: &[u8], w: u32, x: u32, y: u32) -> [u8; 4] {
    let base = ((y * w + x) * 4) as usize;
    [
        pixels[base],
        pixels[base + 1],
        pixels[base + 2],
        pixels[base + 3],
    ]
}

fn rescale_nearest(pixels: &[u8], src_w: u32, src_h: u32, dst_w: u32, dst_h: u32) -> Vec<u8> {
    let mut out = vec![0u8; (dst_w * dst_h * 4) as usize];
    for dy in 0..dst_h {
        for dx in 0..dst_w {
            let sx = (dx * src_w / dst_w).min(src_w - 1);
            let sy = (dy * src_h / dst_h).min(src_h - 1);
            let rgba = sample_rgba(pixels, src_w, sx, sy);
            let dst = ((dy * dst_w + dx) * 4) as usize;
            out[dst..dst + 4].copy_from_slice(&rgba);
        }
    }
    out
}

fn rescale_bilinear(pixels: &[u8], src_w: u32, src_h: u32, dst_w: u32, dst_h: u32) -> Vec<u8> {
    let mut out = vec![0u8; (dst_w * dst_h * 4) as usize];
    for dy in 0..dst_h {
        for dx in 0..dst_w {
            let u = (f64::from(dx) + 0.5) / f64::from(dst_w);
            let v = (f64::from(dy) + 0.5) / f64::from(dst_h);
            let sx = (u * f64::from(src_w) - 0.5).max(0.0);
            let sy = (v * f64::from(src_h) - 0.5).max(0.0);
            #[allow(clippy::cast_possible_truncation)]
            let x0 = (sx as u32).min(src_w - 1);
            #[allow(clippy::cast_possible_truncation)]
            let y0 = (sy as u32).min(src_h - 1);
            let x1 = (x0 + 1).min(src_w - 1);
            let y1 = (y0 + 1).min(src_h - 1);
            let fx = sx.fract();
            let fy = sy.fract();

            let c00 = sample_rgba(pixels, src_w, x0, y0);
            let c10 = sample_rgba(pixels, src_w, x1, y0);
            let c01 = sample_rgba(pixels, src_w, x0, y1);
            let c11 = sample_rgba(pixels, src_w, x1, y1);

            let mut rgba = [0u8; 4];
            for i in 0..4 {
                let top = f64::from(c00[i]) * (1.0 - fx) + f64::from(c10[i]) * fx;
                let bot = f64::from(c01[i]) * (1.0 - fx) + f64::from(c11[i]) * fx;
                let val = top * (1.0 - fy) + bot * fy;
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                {
                    rgba[i] = val.round() as u8;
                }
            }
            let dst = ((dy * dst_w + dx) * 4) as usize;
            out[dst..dst + 4].copy_from_slice(&rgba);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 2x2 identity: rescale 2x2 -> 2x2, output must equal input.
    #[test]
    fn test_rescale_identity() {
        let pixels: Vec<u8> = vec![
            255, 0, 0, 255, // red
            0, 255, 0, 255, // green
            0, 0, 255, 255, // blue
            255, 255, 0, 255, // yellow
        ];
        let r = Rescaler::new(ScaleMode::NearestNeighbor);
        let out = r.rescale(&pixels, 2, 2, 2, 2).unwrap();
        assert_eq!(out, pixels);
    }

    /// Upscale 1x1 red pixel -> 2x2, all pixels must be red.
    #[test]
    fn test_rescale_upscale_2x() {
        let pixels: Vec<u8> = vec![255, 0, 0, 255];
        let r = Rescaler::new(ScaleMode::NearestNeighbor);
        let out = r.rescale(&pixels, 1, 1, 2, 2).unwrap();
        assert_eq!(out.len(), 16);
        for chunk in out.chunks_exact(4) {
            assert_eq!(chunk, &[255, 0, 0, 255]);
        }
    }

    /// Downscale 4x4 -> 2x2: output length must be 16 bytes.
    #[test]
    fn test_rescale_downscale() {
        let pixels: Vec<u8> = [128, 128, 128, 255].repeat(16);
        let r = Rescaler::new(ScaleMode::NearestNeighbor);
        let out = r.rescale(&pixels, 4, 4, 2, 2).unwrap();
        assert_eq!(out.len(), 16);
    }

    /// Empty input must return `EmptyInput`.
    #[test]
    fn test_rescale_empty_input() {
        let r = Rescaler::new(ScaleMode::NearestNeighbor);
        let err = r.rescale(&[], 0, 0, 2, 2).unwrap_err();
        assert!(matches!(err, RescaleError::EmptyInput));
    }

    /// Zero output dimension must return `ZeroOutputDimension`.
    #[test]
    fn test_rescale_zero_output() {
        let pixels: Vec<u8> = vec![255, 0, 0, 255];
        let r = Rescaler::new(ScaleMode::NearestNeighbor);
        let err = r.rescale(&pixels, 1, 1, 0, 2).unwrap_err();
        assert!(matches!(err, RescaleError::ZeroOutputDimension));
    }

    /// Bilinear 2x2 -> 4x4: output length must be 64 bytes.
    #[test]
    fn test_rescale_bilinear() {
        let pixels: Vec<u8> = vec![
            255, 0, 0, 255, // red
            0, 255, 0, 255, // green
            0, 0, 255, 255, // blue
            255, 255, 0, 255, // yellow
        ];
        let r = Rescaler::new(ScaleMode::Bilinear);
        let out = r.rescale(&pixels, 2, 2, 4, 4).unwrap();
        assert_eq!(out.len(), 64);
    }

    /// Invalid pixel data length must return `InvalidPixelData`.
    #[test]
    fn test_rescale_invalid_pixel_data() {
        let pixels: Vec<u8> = vec![255, 0, 0]; // 3 bytes, not 4
        let r = Rescaler::new(ScaleMode::NearestNeighbor);
        let err = r.rescale(&pixels, 1, 1, 2, 2).unwrap_err();
        assert!(matches!(err, RescaleError::InvalidPixelData { .. }));
    }
}
