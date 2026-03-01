// @id: MGE-Remap @do: palette-remap @role: back-end @layer: 6 @human: miyuk
//! Palette remapper -- exact-match color substitution for RGBA pixel data.
#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};

/// A 32-bit RGBA colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rgba {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
    /// Alpha channel.
    pub a: u8,
}

impl Rgba {
    /// Create a new RGBA colour.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Fully transparent black.
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
}

/// A single colour substitution rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorMapping {
    /// Source colour to match.
    pub from: Rgba,
    /// Replacement colour.
    pub to: Rgba,
}

impl ColorMapping {
    /// Create a new mapping from one colour to another.
    pub fn new(from: Rgba, to: Rgba) -> Self {
        Self { from, to }
    }
}

/// Error types for the palette remapper.
#[derive(Debug, thiserror::Error)]
pub enum RemapError {
    /// The input pixel slice is empty.
    #[error("Input pixel data is empty")]
    EmptyInput,
    /// Pixel data length is not divisible by 4.
    #[error("Invalid pixel data length: {len} (must be divisible by 4)")]
    InvalidPixelData {
        /// Actual byte count.
        len: usize,
    },
}

/// Applies palette colour substitutions to RGBA pixel data.
#[derive(Debug, Clone, Default)]
pub struct PaletteRemap {
    /// The list of colour substitution rules.
    pub mappings: Vec<ColorMapping>,
}

impl PaletteRemap {
    /// Create a new empty remapper.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a colour substitution rule.
    pub fn add_mapping(&mut self, from: Rgba, to: Rgba) {
        self.mappings.push(ColorMapping::new(from, to));
    }

    /// Apply all mappings to RGBA pixel data.
    /// Only exact colour matches are substituted.
    pub fn apply(&self, pixels: &[u8]) -> Result<Vec<u8>, RemapError> {
        if pixels.is_empty() {
            return Err(RemapError::EmptyInput);
        }
        if !pixels.len().is_multiple_of(4) {
            return Err(RemapError::InvalidPixelData { len: pixels.len() });
        }

        let mut out = pixels.to_vec();
        for chunk in out.chunks_exact_mut(4) {
            let pixel = Rgba::new(chunk[0], chunk[1], chunk[2], chunk[3]);
            if let Some(mapping) = self.mappings.iter().find(|m| m.from == pixel) {
                chunk[0] = mapping.to.r;
                chunk[1] = mapping.to.g;
                chunk[2] = mapping.to.b;
                chunk[3] = mapping.to.a;
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Single mapping: red -> blue.
    #[test]
    fn test_remap_single_color() {
        let mut remap = PaletteRemap::new();
        remap.add_mapping(
            Rgba::new(255, 0, 0, 255),
            Rgba::new(0, 0, 255, 255),
        );
        let pixels: Vec<u8> = vec![255, 0, 0, 255];
        let out = remap.apply(&pixels).unwrap();
        assert_eq!(out, vec![0, 0, 255, 255]);
    }

    /// Multiple mappings: red->blue AND green->yellow.
    #[test]
    fn test_remap_multiple_mappings() {
        let mut remap = PaletteRemap::new();
        remap.add_mapping(
            Rgba::new(255, 0, 0, 255),
            Rgba::new(0, 0, 255, 255),
        );
        remap.add_mapping(
            Rgba::new(0, 255, 0, 255),
            Rgba::new(255, 255, 0, 255),
        );
        let pixels: Vec<u8> = vec![
            255, 0, 0, 255, // red -> blue
            0, 255, 0, 255, // green -> yellow
        ];
        let out = remap.apply(&pixels).unwrap();
        assert_eq!(
            out,
            vec![
                0, 0, 255, 255, // blue
                255, 255, 0, 255, // yellow
            ]
        );
    }

    /// Pixel with no matching mapping stays unchanged (alpha preserved).
    #[test]
    fn test_remap_alpha_preserved() {
        let remap = PaletteRemap::new(); // no mappings
        let pixels: Vec<u8> = vec![128, 64, 32, 200];
        let out = remap.apply(&pixels).unwrap();
        assert_eq!(out, pixels);
    }

    /// Pixel that does not match any mapping passes through.
    #[test]
    fn test_remap_no_match_passthrough() {
        let mut remap = PaletteRemap::new();
        remap.add_mapping(
            Rgba::new(255, 0, 0, 255),
            Rgba::new(0, 0, 255, 255),
        );
        // white pixel -- does not match red
        let pixels: Vec<u8> = vec![255, 255, 255, 255];
        let out = remap.apply(&pixels).unwrap();
        assert_eq!(out, pixels);
    }

    /// Empty input must return `EmptyInput`.
    #[test]
    fn test_remap_empty_input() {
        let remap = PaletteRemap::new();
        let err = remap.apply(&[]).unwrap_err();
        assert!(matches!(err, RemapError::EmptyInput));
    }

    /// Pixel data not divisible by 4 must return `InvalidPixelData`.
    #[test]
    fn test_remap_invalid_length() {
        let remap = PaletteRemap::new();
        let pixels: Vec<u8> = vec![255, 0, 0]; // 3 bytes
        let err = remap.apply(&pixels).unwrap_err();
        assert!(matches!(err, RemapError::InvalidPixelData { .. }));
    }

    /// Mapping with alpha channel difference: only exact RGBA match triggers.
    #[test]
    fn test_remap_exact_rgba_match() {
        let mut remap = PaletteRemap::new();
        remap.add_mapping(
            Rgba::new(255, 0, 0, 255),
            Rgba::new(0, 0, 255, 255),
        );
        // Same RGB but different alpha -- must NOT match
        let pixels: Vec<u8> = vec![255, 0, 0, 128];
        let out = remap.apply(&pixels).unwrap();
        assert_eq!(out, pixels);
    }
}
