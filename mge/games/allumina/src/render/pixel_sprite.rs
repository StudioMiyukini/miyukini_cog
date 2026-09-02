//! @id     games_allumina_render_pixel_sprite
//! @do     shade
//! @role   ui
//! @layer  core
//! @human  games allumina — module « render pixel sprite ».

//! Procedural pixel-art sprites: an `NxM` grid of coloured cells, each rendered
//! as one small quad so the body reads as hand-placed pixels instead of smooth
//! geometric shapes.
//!
//! A sprite is authored as a slice of equal-length rows (see
//! [`sprites_data`](super::sprites_data)) using single-character palette codes,
//! resolved against a [`Palette`] derived from the entity's [`FigureStyle`] so
//! every class/mob keeps its own colours. Drawing supports a horizontal mirror
//! (W-facing), a vertical bob, and a cheap two-leg "scissor" walk offset.

use mge_render::batch::{RenderLayer, SpriteBatch};

use super::figures::FigureStyle;
use super::shapes;
use super::vertex::Color;

/// Multiply a colour's RGB by `f`, keeping alpha (darken `<1`, lighten `>1`).
fn shade(c: Color, f: f32) -> Color {
    [(c[0] * f).clamp(0.0, 1.0), (c[1] * f).clamp(0.0, 1.0), (c[2] * f).clamp(0.0, 1.0), c[3]]
}

/// Resolved colours for the single-character palette codes used in the grids.
///
/// Class/mob-specific hues come from the style (`primary`, `skin`, `eyes`);
/// shared materials (steel, gold, wood, bone) are fixed so weapons and trim
/// stay consistent across figures.
pub struct Palette {
    primary: Color,
    primary_light: Color,
    primary_dark: Color,
    accent: Color,
    skin: Color,
    skin_dark: Color,
    hair: Color,
    eyes: Color,
}

impl Palette {
    /// Derive a palette from a figure's style.
    #[must_use]
    pub fn from_style(style: &FigureStyle) -> Self {
        Self {
            primary: style.primary,
            primary_light: shade(style.primary, 1.25),
            primary_dark: shade(style.primary, 0.62),
            accent: shade(style.primary, 0.45),
            skin: style.skin,
            skin_dark: shade(style.skin, 0.7),
            // Back-of-head / hair: a dark tone derived from the skin.
            hair: shade(style.skin, 0.42),
            // Players have no glowing eyes → dark sockets; mobs override.
            eyes: style.eyes.unwrap_or([0.10, 0.10, 0.13, 1.0]),
        }
    }

    /// Resolve a palette code to a colour, or `None` for transparent (`.`).
    fn get(&self, c: char) -> Option<Color> {
        Some(match c {
            '.' | ' ' => return None,
            'p' => self.primary,
            'l' => self.primary_light,
            'd' => self.primary_dark,
            'a' => self.accent,
            's' => self.skin,
            'x' => self.skin_dark,
            'h' => self.hair,
            'e' => self.eyes,
            'm' => [0.80, 0.83, 0.90, 1.0], // steel
            'k' => [0.46, 0.49, 0.56, 1.0], // steel dark
            'g' => [0.85, 0.70, 0.20, 1.0], // gold / brass
            'b' => [0.90, 0.88, 0.80, 1.0], // bone
            'w' => [0.46, 0.31, 0.18, 1.0], // wood / leather (boots)
            'o' => [0.10, 0.10, 0.13, 1.0], // dark outline
            _ => self.primary,
        })
    }
}

/// A grid of optional colours; `None` cells are transparent.
pub struct PixelSprite {
    width: u32,
    height: u32,
    pixels: Vec<Option<Color>>,
}

impl PixelSprite {
    /// Build a sprite from authored rows, resolving each character via `pal`.
    ///
    /// Rows may be ragged; the width is the longest row and short rows pad with
    /// transparent on the right. When `back` is set, the head region's skin and
    /// eyes are swapped for hair so a figure seen from behind shows no face.
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn from_grid(rows: &[&str], pal: &Palette, back: bool) -> Self {
        let height = rows.len() as u32;
        let width = rows.iter().map(|r| r.chars().count()).max().unwrap_or(0) as u32;
        // Anything in the top ~30% of the sprite counts as "head" for the back view.
        let head_rows = ((height as f32) * 0.30).ceil() as u32;
        let mut pixels = vec![None; (width * height) as usize];
        for (y, row) in rows.iter().enumerate() {
            for (x, mut ch) in row.chars().enumerate() {
                if back && (y as u32) < head_rows {
                    ch = match ch {
                        's' | 'x' | 'e' => 'h',
                        other => other,
                    };
                }
                pixels[y * width as usize + x] = pal.get(ch);
            }
        }
        Self { width, height, pixels }
    }

    /// Number of pixel rows (used by callers to size one pixel against a target
    /// on-screen body height).
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Number of pixel columns (so callers can size horizontal pixels against a
    /// target body *width*, keeping the figure slim independently of its height).
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    fn at(&self, x: u32, y: u32) -> Option<Color> {
        self.pixels[(y * self.width + x) as usize]
    }

    /// Draw the sprite with its feet centred on `(cx, feet_y)`.
    ///
    /// `px`/`px_x` are the per-pixel vertical/horizontal sizes (so a stocky
    /// build widens without stretching height). `mirror` flips it left/right,
    /// `bob` shifts the whole body vertically, and `leg_swing_px` offsets the
    /// two leg halves in opposite directions for a walk cycle. Consecutive
    /// same-colour cells in a row are merged into one quad.
    #[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
    pub fn draw(
        &self,
        batch: &mut SpriteBatch,
        cx: f32,
        feet_y: f32,
        px: f32,
        px_x: f32,
        layer: RenderLayer,
        ys: i32,
        sub: u32,
        mirror: bool,
        leg_swing_px: f32,
        bob: f32,
        tint: &dyn Fn(Color) -> Color,
    ) {
        let ox = cx - self.width as f32 * px_x * 0.5;
        let oy = feet_y - self.height as f32 * px + bob;
        // Rows below this get the scissor-walk leg offset.
        let leg_y0 = ((self.height as f32) * 0.62) as u32;
        let half = self.width as f32 * 0.5;
        for y in 0..self.height {
            let row_y = oy + y as f32 * px;
            let is_leg = y >= leg_y0;
            let mut x = 0u32;
            while x < self.width {
                // `x` is the on-screen column; `src` is which cell supplies the colour.
                let src = if mirror { self.width - 1 - x } else { x };
                let Some(color) = self.at(src, y) else {
                    x += 1;
                    continue;
                };
                let side = if is_leg {
                    if (x as f32) < half { -1.0 } else { 1.0 }
                } else {
                    0.0
                };
                // Extend the run while colour and leg-side stay the same.
                let mut run = 1u32;
                while x + run < self.width {
                    let src2 = if mirror { self.width - 1 - (x + run) } else { x + run };
                    let side2 = if is_leg {
                        if ((x + run) as f32) < half { -1.0 } else { 1.0 }
                    } else {
                        0.0
                    };
                    if self.at(src2, y) != Some(color) || side2 != side {
                        break;
                    }
                    run += 1;
                }
                let fx = ox + x as f32 * px_x + side * leg_swing_px;
                shapes::rect(
                    batch,
                    fx,
                    row_y,
                    px_x * run as f32 + 0.5,
                    px + 0.5,
                    layer,
                    ys,
                    sub,
                    tint(color),
                );
                x += run;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{figures::Weapon, vertex};
    use super::*;

    fn pal() -> Palette {
        let style = FigureStyle::humanoid(vertex::PLAYER, vertex::SKIN, Weapon::Sword);
        Palette::from_style(&style)
    }

    #[test]
    fn ragged_rows_pad_to_max_width() {
        let s = PixelSprite::from_grid(&["pp", "pppp", "p"], &pal(), false);
        assert_eq!(s.width, 4);
        assert_eq!(s.height, 3);
        assert!(s.at(3, 0).is_none(), "short row pads transparent");
        assert!(s.at(0, 2).is_some());
    }

    #[test]
    fn back_view_hides_face() {
        // Skin + eyes in the head region become hair (still opaque, but the
        // hair tone, not skin/eyes).
        let front = PixelSprite::from_grid(&["se", "pp", "pp", "pp"], &pal(), false);
        let back = PixelSprite::from_grid(&["se", "pp", "pp", "pp"], &pal(), true);
        assert_ne!(front.at(0, 0), back.at(0, 0));
        assert_eq!(back.at(0, 0), back.at(1, 0), "skin and eye both map to hair");
    }

    #[test]
    fn draw_emits_quads() {
        let s = PixelSprite::from_grid(&["pp", "pp"], &pal(), false);
        let mut b = SpriteBatch::new();
        s.draw(&mut b, 0.0, 0.0, 2.0, 2.0, RenderLayer::Entities, 0, 0, false, 0.0, 0.0, &|c| c);
        assert!(b.len() >= 2, "two rows should emit quads, got {}", b.len());
    }
}
