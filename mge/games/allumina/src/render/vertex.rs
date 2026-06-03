//! Vertex/colour primitives for the 2D sprite renderer.
//!
//! The GPU-side vertex layout and instanced quad pipeline live in `mge-render`
//! (a single tinted unit-quad per sprite). This module holds the CPU-side
//! [`Vertex`] used for geometry helpers/hit-testing and the Allumina colour
//! palette every drawing routine pulls from, so placeholder art stays
//! consistent across the tilemap, entities, VFX, and HUD.

/// RGBA colour, linear-ish `[0,1]` per channel, matching `SpriteInstance::tint`.
pub type Color = [f32; 4];

/// A logical 2D vertex: screen position, atlas UV, vertex colour.
///
/// Used by geometry helpers (e.g. building a quad's corners for picking); the
/// actual GPU upload happens through `mge_render::batch::SpriteInstance`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: Color,
}

impl Vertex {
    pub const fn new(position: [f32; 2], uv: [f32; 2], color: Color) -> Self {
        Self { position, uv, color }
    }
}

/// The six vertices (two triangles) covering an axis-aligned quad at
/// `(x, y)` with size `(w, h)` and a uniform colour. Handy for tests and any
/// future custom geometry; the live renderer uses instanced unit quads.
#[must_use]
pub fn quad(x: f32, y: f32, w: f32, h: f32, color: Color) -> [Vertex; 6] {
    let tl = Vertex::new([x, y], [0.0, 0.0], color);
    let tr = Vertex::new([x + w, y], [1.0, 0.0], color);
    let br = Vertex::new([x + w, y + h], [1.0, 1.0], color);
    let bl = Vertex::new([x, y + h], [0.0, 1.0], color);
    [tl, tr, br, tl, br, bl]
}

// ── Palette ─────────────────────────────────────────────────────────────────

pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];
pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];

/// Player placeholder — blue.
pub const PLAYER: Color = [0.30, 0.55, 1.0, 1.0];
/// Generic hostile mob — red.
pub const MOB: Color = [0.90, 0.25, 0.22, 1.0];
/// Ranged mob — orange-red.
pub const MOB_RANGED: Color = [0.95, 0.55, 0.20, 1.0];
/// Caster mob — magenta.
pub const MOB_CASTER: Color = [0.80, 0.35, 0.90, 1.0];
/// Boss — deep crimson.
pub const MOB_BOSS: Color = [0.65, 0.05, 0.10, 1.0];
/// Ground item — yellow.
pub const ITEM: Color = [0.95, 0.85, 0.25, 1.0];
/// Summoned ally — turquoise/cyan.
pub const SUMMON: Color = [0.20, 0.90, 0.85, 1.0];
/// Mercenary companion — light green.
pub const MERC: Color = [0.45, 0.92, 0.45, 1.0];

/// Warm humanoid skin tone (heads of player/mercs/NPCs).
pub const SKIN: Color = [0.85, 0.68, 0.52, 1.0];
/// Soldat body — burnished steel blue armour.
pub const CLASS_SOLDAT: Color = [0.36, 0.50, 0.66, 1.0];
/// Clerc body — ivory-and-gold holy robe.
pub const CLASS_CLERC: Color = [0.88, 0.82, 0.62, 1.0];
/// Mage de guerre body — deep arcane violet.
pub const CLASS_MAGE: Color = [0.52, 0.32, 0.80, 1.0];
/// Archer body — forest green leathers.
pub const CLASS_ARCHER: Color = [0.24, 0.52, 0.30, 1.0];
/// Skeletal mob/summon — bone off-white.
pub const SKEL: Color = [0.88, 0.86, 0.78, 1.0];
/// Zombie mob — dark sickly green.
pub const ZOMBIE: Color = [0.30, 0.44, 0.24, 1.0];
/// Golem mob/summon — stone grey.
pub const GOLEM: Color = [0.52, 0.52, 0.56, 1.0];
/// Caster mob robe — deep magenta.
pub const CASTER_ROBE: Color = [0.62, 0.30, 0.72, 1.0];

/// Walkable floor — muted desaturated olive-green (D2 Act-1 wilderness feel).
pub const FLOOR: Color = [0.26, 0.33, 0.20, 1.0];
/// Alternating floor checker — darker, drier olive.
pub const FLOOR_ALT: Color = [0.21, 0.28, 0.17, 1.0];
/// Wall / blocker — earthy brown.
pub const WALL: Color = [0.38, 0.27, 0.16, 1.0];
/// Exit / transition tile — teal.
pub const EXIT: Color = [0.25, 0.70, 0.70, 1.0];
/// Player-spawn marker tint blended into the floor.
pub const SPAWN: Color = [0.33, 0.40, 0.25, 1.0];

/// HUD panel background — translucent dark.
pub const PANEL_BG: Color = [0.05, 0.05, 0.07, 0.85];
/// HUD panel border.
pub const PANEL_BORDER: Color = [0.55, 0.45, 0.25, 1.0];
/// Life orb / bar.
pub const LIFE: Color = [0.80, 0.12, 0.12, 1.0];
/// Mana orb / bar.
pub const MANA: Color = [0.15, 0.35, 0.90, 1.0];
/// Experience bar.
pub const XP: Color = [0.85, 0.70, 0.20, 1.0];

/// Map a Diablo-style rarity colour name to an RGBA tint.
#[must_use]
pub fn rarity_color(name: &str) -> Color {
    match name {
        "blue" => [0.40, 0.55, 1.0, 1.0],
        "yellow" => [0.95, 0.90, 0.30, 1.0],
        "orange" => [0.95, 0.55, 0.15, 1.0],
        "green" => [0.30, 0.85, 0.35, 1.0],
        "gold" => [0.80, 0.65, 0.20, 1.0],
        _ => WHITE, // "white" / normal
    }
}

/// Linearly interpolate between two colours (used by hit/crit flashes).
#[must_use]
pub fn lerp(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
        a[3] + (b[3] - a[3]) * t,
    ]
}

/// Scale a colour's alpha (used by fades).
#[must_use]
pub fn with_alpha(c: Color, a: f32) -> Color {
    [c[0], c[1], c[2], a.clamp(0.0, 1.0)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_has_six_vertices_spanning_box() {
        let v = quad(10.0, 20.0, 4.0, 8.0, WHITE);
        assert_eq!(v.len(), 6);
        assert_eq!(v[0].position, [10.0, 20.0]);
        assert_eq!(v[2].position, [14.0, 28.0]);
    }

    #[test]
    fn lerp_endpoints() {
        assert_eq!(lerp(BLACK, WHITE, 0.0), BLACK);
        assert_eq!(lerp(BLACK, WHITE, 1.0), WHITE);
        let mid = lerp(BLACK, WHITE, 0.5);
        assert!((mid[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn rarity_colors_distinct() {
        assert_ne!(rarity_color("blue"), rarity_color("gold"));
        assert_eq!(rarity_color("unknown"), WHITE);
    }
}
