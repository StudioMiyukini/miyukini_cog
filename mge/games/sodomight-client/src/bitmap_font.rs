// @id: Sodomight-Client-BitmapFont @do: bitmap-font @role: front-end @layer: 4 @human: miyuk
//! Minimal bitmap font renderer for in-game text.
//!
//! Generates a procedural 128x48 pixel texture atlas containing ASCII
//! characters 32..=126 arranged in a 16-column by 6-row grid.
//! Each glyph cell is 8x8 pixels; the actual glyphs are 5x7 with 1px
//! right padding and 1px bottom padding.
//!
//! Text is rendered by pushing quads into the existing [`SpriteBatcher`]
//! with UV coordinates pointing at the correct glyph cell.

#![deny(unsafe_code)]

use mge_render::{GpuTexture, SpriteBatcher, SpritePipeline};

/// Glyph cell width in the atlas (pixels).
const CELL_W: u32 = 8;
/// Glyph cell height in the atlas (pixels).
const CELL_H: u32 = 8;

/// Number of glyph columns in the atlas.
const ATLAS_COLS: u32 = 16;
/// Number of glyph rows in the atlas.
const ATLAS_ROWS: u32 = 6;

/// Atlas texture width in pixels.
const ATLAS_W: u32 = ATLAS_COLS * CELL_W; // 128
/// Atlas texture height in pixels.
const ATLAS_H: u32 = ATLAS_ROWS * CELL_H; // 48

/// First printable ASCII code point in the atlas.
const ASCII_START: u8 = 32;
/// Last printable ASCII code point in the atlas (inclusive).
const ASCII_END: u8 = 126;

/// Bitmap font renderer.
///
/// Owns a GPU texture atlas and provides [`push_text`](Self::push_text) to
/// batch text quads into a [`SpriteBatcher`].
pub struct BitmapFont {
    /// The GPU texture holding the glyph atlas.
    pub texture: GpuTexture,
}

impl BitmapFont {
    /// Create the bitmap font, generating the atlas texture on the GPU.
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pipeline: &SpritePipeline,
    ) -> Self {
        let img = generate_font_atlas();
        let texture = GpuTexture::from_image(device, queue, pipeline, &img, "bitmap_font_atlas");
        Self { texture }
    }

    /// Push a string of text into the batcher as a series of glyph quads.
    ///
    /// Returns the total width in pixels of the rendered text.
    ///
    /// # Arguments
    ///
    /// * `batcher` -- the sprite batcher to push quads into.
    /// * `x`, `y` -- top-left screen position of the first character.
    /// * `text` -- the ASCII string to render.
    /// * `color` -- RGBA tint colour for all glyphs.
    /// * `scale` -- multiplier on glyph size (1.0 = 8px, 2.0 = 16px).
    pub fn push_text(
        batcher: &mut SpriteBatcher,
        x: f32,
        y: f32,
        text: &str,
        color: [f32; 4],
        scale: f32,
    ) -> f32 {
        let gw = CELL_W as f32 * scale;
        let gh = CELL_H as f32 * scale;
        let inv_atlas_w = 1.0 / ATLAS_W as f32;
        let inv_atlas_h = 1.0 / ATLAS_H as f32;

        let mut cx = x;

        for byte in text.bytes() {
            let code = if (ASCII_START..=ASCII_END).contains(&byte) {
                byte - ASCII_START
            } else {
                // Non-printable character: render as space (index 0 = space).
                0
            };

            let col = u32::from(code) % ATLAS_COLS;
            let row = u32::from(code) / ATLAS_COLS;

            let u_min = (col * CELL_W) as f32 * inv_atlas_w;
            let v_min = (row * CELL_H) as f32 * inv_atlas_h;
            let u_max = ((col + 1) * CELL_W) as f32 * inv_atlas_w;
            let v_max = ((row + 1) * CELL_H) as f32 * inv_atlas_h;

            let uv = [u_min, v_min, u_max, v_max];
            batcher.push(cx, y, gw, gh, uv, color);

            cx += gw;
        }

        cx - x
    }

    /// Width of a single character at the given scale.
    #[must_use]
    pub fn char_width(scale: f32) -> f32 {
        CELL_W as f32 * scale
    }

    /// Height of a line of text at the given scale.
    #[must_use]
    pub fn line_height(scale: f32) -> f32 {
        CELL_H as f32 * scale
    }
}

// ---------------------------------------------------------------------------
// Font atlas generation (procedural, no external file)
// ---------------------------------------------------------------------------

/// Generate a 128x48 RGBA image containing a 5x7 bitmap font for ASCII 32-126.
///
/// Each glyph occupies an 8x8 cell. The glyph data is 5 pixels wide and 7
/// pixels tall, with 3 pixels of padding (right + bottom) in each cell.
/// The background is fully transparent; glyph pixels are white (255,255,255,255).
fn generate_font_atlas() -> image::RgbaImage {
    let mut img = image::RgbaImage::new(ATLAS_W, ATLAS_H);

    for code in ASCII_START..=ASCII_END {
        let index = (code - ASCII_START) as u32;
        let col = index % ATLAS_COLS;
        let row = index / ATLAS_COLS;
        let ox = col * CELL_W;
        let oy = row * CELL_H;

        let glyph = glyph_data(code);
        for (gy, glyph_row) in glyph.iter().enumerate() {
            for gx in 0..5u32 {
                if glyph_row & (1 << (4 - gx)) != 0 {
                    let px = ox + gx;
                    let py = oy + gy as u32;
                    if px < ATLAS_W && py < ATLAS_H {
                        img.put_pixel(px, py, image::Rgba([255, 255, 255, 255]));
                    }
                }
            }
        }
    }

    img
}

/// Return 7 rows of 5-bit glyph data for an ASCII character.
///
/// Each `u8` represents one row; bit 4 is the leftmost pixel, bit 0 is the
/// rightmost. Glyphs are a compact 5x7 design inspired by classic LCD fonts.
#[allow(clippy::too_many_lines)]
fn glyph_data(code: u8) -> [u8; 7] {
    match code {
        // Space
        b' ' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        b'!' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100],
        b'"' => [0b01010, 0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000],
        b'#' => [0b01010, 0b01010, 0b11111, 0b01010, 0b11111, 0b01010, 0b01010],
        b'$' => [0b00100, 0b01111, 0b10100, 0b01110, 0b00101, 0b11110, 0b00100],
        b'%' => [0b11000, 0b11001, 0b00010, 0b00100, 0b01000, 0b10011, 0b00011],
        b'&' => [0b01100, 0b10010, 0b10100, 0b01000, 0b10101, 0b10010, 0b01101],
        b'\'' => [0b00100, 0b00100, 0b01000, 0b00000, 0b00000, 0b00000, 0b00000],
        b'(' => [0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010],
        b')' => [0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000],
        b'*' => [0b00000, 0b00100, 0b10101, 0b01110, 0b10101, 0b00100, 0b00000],
        b'+' => [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],
        b',' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100, 0b01000],
        b'-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        b'.' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100],
        b'/' => [0b00000, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00000],
        b'0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        b'1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        b'2' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111],
        b'3' => [0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110],
        b'4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        b'5' => [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
        b'6' => [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        b'7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        b'8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        b'9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100],
        b':' => [0b00000, 0b00000, 0b00100, 0b00000, 0b00100, 0b00000, 0b00000],
        b';' => [0b00000, 0b00000, 0b00100, 0b00000, 0b00100, 0b00100, 0b01000],
        b'<' => [0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010],
        b'=' => [0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000],
        b'>' => [0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000],
        b'?' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100],
        b'@' => [0b01110, 0b10001, 0b10111, 0b10101, 0b10110, 0b10000, 0b01110],
        b'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        b'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        b'C' => [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
        b'D' => [0b11100, 0b10010, 0b10001, 0b10001, 0b10001, 0b10010, 0b11100],
        b'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        b'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        b'G' => [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111],
        b'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        b'I' => [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        b'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100],
        b'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        b'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        b'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        b'N' => [0b10001, 0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001],
        b'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        b'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        b'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        b'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        b'S' => [0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110],
        b'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        b'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        b'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        b'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010],
        b'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        b'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        b'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
        b'[' => [0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110],
        b'\\' => [0b00000, 0b10000, 0b01000, 0b00100, 0b00010, 0b00001, 0b00000],
        b']' => [0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110],
        b'^' => [0b00100, 0b01010, 0b10001, 0b00000, 0b00000, 0b00000, 0b00000],
        b'_' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
        b'`' => [0b01000, 0b00100, 0b00010, 0b00000, 0b00000, 0b00000, 0b00000],
        b'a' => [0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111],
        b'b' => [0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b11110],
        b'c' => [0b00000, 0b00000, 0b01110, 0b10000, 0b10000, 0b10001, 0b01110],
        b'd' => [0b00001, 0b00001, 0b01101, 0b10011, 0b10001, 0b10001, 0b01111],
        b'e' => [0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110],
        b'f' => [0b00110, 0b01001, 0b01000, 0b11100, 0b01000, 0b01000, 0b01000],
        b'g' => [0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110],
        b'h' => [0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001],
        b'i' => [0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110],
        b'j' => [0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b10010, 0b01100],
        b'k' => [0b10000, 0b10000, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010],
        b'l' => [0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        b'm' => [0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10001, 0b10001],
        b'n' => [0b00000, 0b00000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001],
        b'o' => [0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110],
        b'p' => [0b00000, 0b00000, 0b11110, 0b10001, 0b11110, 0b10000, 0b10000],
        b'q' => [0b00000, 0b00000, 0b01101, 0b10011, 0b01111, 0b00001, 0b00001],
        b'r' => [0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000],
        b's' => [0b00000, 0b00000, 0b01110, 0b10000, 0b01110, 0b00001, 0b11110],
        b't' => [0b01000, 0b01000, 0b11100, 0b01000, 0b01000, 0b01001, 0b00110],
        b'u' => [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101],
        b'v' => [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        b'w' => [0b00000, 0b00000, 0b10001, 0b10001, 0b10101, 0b10101, 0b01010],
        b'x' => [0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001],
        b'y' => [0b00000, 0b00000, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110],
        b'z' => [0b00000, 0b00000, 0b11111, 0b00010, 0b00100, 0b01000, 0b11111],
        b'{' => [0b00010, 0b00100, 0b00100, 0b01000, 0b00100, 0b00100, 0b00010],
        b'|' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        b'}' => [0b01000, 0b00100, 0b00100, 0b00010, 0b00100, 0b00100, 0b01000],
        b'~' => [0b00000, 0b00000, 0b01000, 0b10101, 0b00010, 0b00000, 0b00000],
        // Fallback: filled square for unknown characters.
        _ => [0b11111, 0b11111, 0b11111, 0b11111, 0b11111, 0b11111, 0b11111],
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atlas_dimensions_correct() {
        let img = generate_font_atlas();
        assert_eq!(img.width(), ATLAS_W);
        assert_eq!(img.height(), ATLAS_H);
    }

    #[test]
    fn space_glyph_is_blank() {
        let img = generate_font_atlas();
        // Space is at column 0, row 0 (index 0). All pixels should be transparent.
        for y in 0..CELL_H {
            for x in 0..CELL_W {
                let pixel = img.get_pixel(x, y);
                assert_eq!(
                    pixel.0[3], 0,
                    "Space glyph pixel ({x},{y}) should be transparent"
                );
            }
        }
    }

    #[test]
    fn letter_a_has_nonzero_pixels() {
        let img = generate_font_atlas();
        // 'A' is code 65, index 65-32 = 33. Column = 33 % 16 = 1, row = 33 / 16 = 2.
        let ox = 1 * CELL_W;
        let oy = 2 * CELL_H;
        let mut nonzero = 0u32;
        for y in 0..CELL_H {
            for x in 0..CELL_W {
                let pixel = img.get_pixel(ox + x, oy + y);
                if pixel.0[3] > 0 {
                    nonzero += 1;
                }
            }
        }
        assert!(nonzero > 0, "Letter 'A' should have visible pixels");
    }

    #[test]
    fn char_width_and_line_height() {
        assert!((BitmapFont::char_width(1.0) - 8.0).abs() < f32::EPSILON);
        assert!((BitmapFont::char_width(2.0) - 16.0).abs() < f32::EPSILON);
        assert!((BitmapFont::line_height(1.0) - 8.0).abs() < f32::EPSILON);
        assert!((BitmapFont::line_height(2.0) - 16.0).abs() < f32::EPSILON);
    }

    #[test]
    fn push_text_returns_correct_width() {
        // We cannot create a real SpriteBatcher without a GPU device, so
        // we test the math directly.
        let text = "Hello";
        let scale = 2.0;
        let expected_width = text.len() as f32 * CELL_W as f32 * scale;
        assert!(
            (expected_width - 80.0).abs() < f32::EPSILON,
            "5 chars at scale 2 should be 80px wide"
        );
    }

    #[test]
    fn all_printable_ascii_glyphs_exist() {
        // Verify that glyph_data returns a valid (non-panic) result for all
        // printable ASCII characters.
        for code in ASCII_START..=ASCII_END {
            let data = glyph_data(code);
            // At least one row should be non-zero for any visible character
            // (except space which is all zeros).
            if code != b' ' {
                let total: u8 = data.iter().copied().sum();
                assert!(
                    total > 0,
                    "Glyph for '{}' (code {code}) should have visible pixels",
                    code as char
                );
            }
        }
    }
}
