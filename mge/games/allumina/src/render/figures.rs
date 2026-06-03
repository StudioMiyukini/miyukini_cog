//! Expressive procedural figures: directional humanoid silhouettes (one per
//! player class), stone golems, and the boss, assembled from the
//! [`shapes`](super::shapes) quad primitives.
//!
//! A [`FigureStyle`] describes the palette/build/weapon/silhouette, a
//! [`Direction`] orients the body across nine facings (N..NW + Idle), and an
//! [`AnimSample`] supplies the per-frame pose (bob, limb swing, attack arc,
//! hit/death colour transform). The silhouette and direction together decide
//! which limbs are visible, where the weapon sits, and which back items
//! (cape/quiver) show.

use serde::{Deserialize, Serialize};

use mge_items::item::ItemKind;
use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::Position2D;

use super::animation::AnimSample;
use super::camera::Camera2D;
use super::pixel_sprite;
use super::shapes;
use super::sprite::y_sort;
use super::sprites_data;
use super::vertex::{self, with_alpha, Color};

/// Nine facings for a figure. The eight compass directions are screen-relative
/// (N = up, E = right). `Idle` renders as a front-facing rest pose (= `S`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    Idle,
}

impl Default for Direction {
    fn default() -> Self {
        Self::S
    }
}

impl Direction {
    /// Map a world-space movement/attack vector to a screen-relative facing.
    /// Returns `None` if the vector is essentially zero (caller keeps the old
    /// facing). The 2:1 isometric squash is folded in so the facing matches
    /// what the player sees on screen.
    #[must_use]
    pub fn from_world_vector(dx: f32, dy: f32) -> Option<Self> {
        if dx.abs() < 1e-4 && dy.abs() < 1e-4 {
            return None;
        }
        // World → screen direction (TILE_W:TILE_H is 2:1, so y is halved).
        let sx = dx - dy;
        let sy = (dx + dy) * 0.5;
        if sx.abs() < 1e-5 && sy.abs() < 1e-5 {
            return None;
        }
        // Screen y grows downward; negate so "up" is +90°.
        let mut deg = (-sy).atan2(sx).to_degrees();
        if deg < 0.0 {
            deg += 360.0;
        }
        #[allow(clippy::cast_possible_truncation)]
        let sector = (((deg + 22.5) / 45.0).floor() as i32).rem_euclid(8);
        Some(match sector {
            0 => Self::E,
            1 => Self::NE,
            2 => Self::N,
            3 => Self::NW,
            4 => Self::W,
            5 => Self::SW,
            6 => Self::S,
            _ => Self::SE,
        })
    }

    /// Lateral facing in `[-1, 1]`: screen-left (W) is negative, right (E) is
    /// positive, pure N/S is centred.
    #[must_use]
    pub fn lateral(self) -> f32 {
        match self {
            Self::E => 1.0,
            Self::NE | Self::SE => 0.7,
            Self::W => -1.0,
            Self::NW | Self::SW => -0.7,
            Self::N | Self::S | Self::Idle => 0.0,
        }
    }

    /// Depth in `[-1, 1]`: `-1` faces fully away from the camera (N), `+1` faces
    /// the camera (S), `0` is a pure side profile (E/W).
    #[must_use]
    pub fn depth(self) -> f32 {
        match self {
            Self::N => -1.0,
            Self::NE | Self::NW => -0.5,
            Self::E | Self::W => 0.0,
            Self::SE | Self::SW => 0.5,
            Self::S | Self::Idle => 1.0,
        }
    }

    /// Whether the figure shows its back (no face, weapon hidden, cape/quiver up).
    #[must_use]
    pub fn faces_away(self) -> bool {
        matches!(self, Self::N | Self::NE | Self::NW)
    }
}

/// Weapon silhouette a figure carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weapon {
    None,
    Sword,
    /// A stubby blade — the militiaman's side-arm.
    ShortSword,
    Axe,
    Staff,
    Wand,
    Scepter,
    Orb,
    Bow,
    Crossbow,
    /// Bone club — skeletal melee.
    Bone,
    /// A wooden hayfork — the peasant's improvised weapon (a shaft with prongs).
    Pitchfork,
}

impl Weapon {
    /// Compact wire code for replicating a weapon silhouette over the network.
    #[must_use]
    pub fn to_code(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Sword => 1,
            Self::ShortSword => 2,
            Self::Axe => 3,
            Self::Staff => 4,
            Self::Wand => 5,
            Self::Scepter => 6,
            Self::Orb => 7,
            Self::Bow => 8,
            Self::Crossbow => 9,
            Self::Bone => 10,
            Self::Pitchfork => 11,
        }
    }

    /// Inverse of [`to_code`](Self::to_code); unknown codes fall back to `None`.
    #[must_use]
    pub fn from_code(c: u8) -> Self {
        match c {
            1 => Self::Sword,
            2 => Self::ShortSword,
            3 => Self::Axe,
            4 => Self::Staff,
            5 => Self::Wand,
            6 => Self::Scepter,
            7 => Self::Orb,
            8 => Self::Bow,
            9 => Self::Crossbow,
            10 => Self::Bone,
            11 => Self::Pitchfork,
            _ => Self::None,
        }
    }

    /// The silhouette an equipped [`ItemKind`] presents in-hand.
    #[must_use]
    pub fn for_item_kind(kind: ItemKind) -> Self {
        match kind {
            ItemKind::Sword => Self::Sword,
            ItemKind::Dagger => Self::ShortSword,
            ItemKind::Axe | ItemKind::Mace => Self::Axe,
            ItemKind::Staff => Self::Staff,
            ItemKind::Wand => Self::Wand,
            ItemKind::Scepter => Self::Scepter,
            ItemKind::Bow => Self::Bow,
            ItemKind::Crossbow => Self::Crossbow,
            _ => Self::None,
        }
    }
}

/// Overall body silhouette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// Soldat: broad, square armoured shoulders, flat helm.
    Soldier,
    /// Clerc: long robe (no legs), hood, soft holy aura.
    Cleric,
    /// Mage de guerre: mid robe, pointed hat, cape, glowing orb.
    Warmage,
    /// Archer: slim, short hood, quiver on the back.
    Archer,
    /// Generic two-legged humanoid (mercs, NPCs, skeletons, zombies, casters).
    Brute,
    /// Hulking stone block.
    Golem,
}

/// Full description of how to draw one figure.
#[derive(Debug, Clone, Copy)]
pub struct FigureStyle {
    /// Torso / main colour.
    pub primary: Color,
    /// Head / skin colour.
    pub skin: Color,
    /// Width multiplier: <1 lean (skeleton), >1 stocky (zombie/golem).
    pub build: f32,
    pub weapon: Weapon,
    /// A second weapon held in the off-hand (dual-wield). Drawn mirrored on the
    /// opposite side from [`weapon`]; `None` for shields / single-wield.
    pub offhand: Option<Weapon>,
    pub shape: Shape,
    /// Base alpha (summons are translucent at ~0.75).
    pub alpha: f32,
    /// Glowing rim silhouette behind the body (summons cyan, boss gold).
    pub outline: Option<Color>,
    /// Pulsing aura behind the figure (boss, or the Clerc's holy glow).
    pub aura: Option<Color>,
    /// Glowing eyes (skeleton/golem/boss).
    pub eyes: Option<Color>,
    /// A shield carried in the off-hand (the militiaman), drawn in this colour.
    pub shield: Option<Color>,
}

impl FigureStyle {
    /// A plain (generic) humanoid with the given palette.
    #[must_use]
    pub fn humanoid(primary: Color, skin: Color, weapon: Weapon) -> Self {
        Self {
            primary,
            skin,
            build: 1.0,
            weapon,
            offhand: None,
            shape: Shape::Brute,
            alpha: 1.0,
            outline: None,
            aura: None,
            eyes: None,
            shield: None,
        }
    }
}

// Steel / wood / brass accents shared by weapons.
const STEEL: Color = [0.80, 0.83, 0.90, 1.0];
const WOOD: Color = [0.46, 0.31, 0.18, 1.0];
const BRASS: Color = [0.72, 0.56, 0.22, 1.0];
const BONE: Color = [0.90, 0.88, 0.80, 1.0];

/// Multiply a colour's RGB by `f` (darken `<1`, lighten `>1`), keeping alpha.
fn shade(c: Color, f: f32) -> Color {
    [(c[0] * f).clamp(0.0, 1.0), (c[1] * f).clamp(0.0, 1.0), (c[2] * f).clamp(0.0, 1.0), c[3]]
}

/// Point at polar `(angle, radius)` from origin `o` (screen space, +y down).
fn polar(o: [f32; 2], angle: f32, r: f32) -> [f32; 2] {
    [o[0] + angle.cos() * r, o[1] + angle.sin() * r]
}

/// Linear interpolation between two scalars.
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}

// Draw-order parts within a single figure (combined with the entity id).
const P_AURA: u32 = 0;
const P_OUTLINE: u32 = 1;
const P_BACK: u32 = 2;
const P_TORSO: u32 = 3;
const P_WEAPON: u32 = 7;

/// Draw a complete figure, feet anchored at the projected tile centre of `pos`.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub fn draw(
    batch: &mut SpriteBatch,
    cam: &Camera2D,
    pos: Position2D,
    base: [f32; 2],
    style: &FigureStyle,
    sample: &AnimSample,
    dir: Direction,
    id: u32,
    time: f32,
) {
    let s = cam.world_to_screen(pos);
    let z = cam.zoom;
    let w = base[0] * z * sample.scale;
    let h = base[1] * z * sample.scale;
    let cx = s[0];
    let feet_y = s[1];
    let bob = sample.bob * z;
    let ys = y_sort(pos);

    // Compose part sub-orders so a figure's quads stay grouped and ordered even
    // when two entities share a depth row.
    let base_sub = (id & 0x07FF_FFFF).wrapping_mul(16);
    let psub = |part: u32| base_sub.wrapping_add(part);

    // Fold the style alpha into every part, then let the sample apply the
    // hit-flash / death-fade colour transform.
    let pa = style.alpha;
    let tint = |rgb: Color| sample.tint([rgb[0], rgb[1], rgb[2], rgb[3] * pa]);

    // Ground shadow (under everything, on the decal layer).
    let shadow_a = 0.34 * sample.alpha;
    shapes::ellipse(
        batch,
        cx,
        feet_y,
        w * 0.42,
        w * 0.17,
        RenderLayer::Decals,
        ys,
        psub(P_AURA),
        [0.0, 0.0, 0.0, shadow_a],
    );

    // Pulsing aura (boss menace, or a Clerc's soft holy glow).
    if let Some(aura) = style.aura {
        let pulse = 0.18 + 0.14 * (time * 3.0).sin().abs();
        shapes::ellipse(
            batch,
            cx,
            feet_y - h * 0.45 + bob,
            w * 0.62,
            h * 0.46,
            RenderLayer::Entities,
            ys,
            psub(P_AURA),
            with_alpha(aura, pulse * sample.alpha),
        );
    }

    draw_pixel_body(batch, cx, feet_y, bob, w, h, ys, &psub, style, sample, dir, &tint);
}

/// Render a figure as a procedural pixel-art sprite (body) plus the layered
/// weapon, off-hand shield, and an optional glowing rim (summon/boss trim).
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn draw_pixel_body(
    batch: &mut SpriteBatch,
    cx: f32,
    feet_y: f32,
    bob: f32,
    w: f32,
    h: f32,
    ys: i32,
    psub: &dyn Fn(u32) -> u32,
    style: &FigureStyle,
    sample: &AnimSample,
    dir: Direction,
    tint: &dyn Fn(Color) -> Color,
) {
    let away = dir.faces_away();
    let mirror = dir.lateral() < -0.05;
    let face = if mirror { -1.0 } else { 1.0 };
    let dep = dir.depth();
    let torso_top = feet_y - h * 0.80 + bob;

    // Glowing rim silhouette (summon / boss trim): an inflated torso+head.
    if let Some(rim) = style.outline {
        let tw = w * 0.52 * style.build + 4.0;
        shapes::rounded_rect(
            batch,
            cx - tw * 0.5,
            torso_top - 2.0,
            tw,
            h * 0.42 + 4.0,
            tw * 0.28,
            RenderLayer::Entities,
            ys,
            psub(P_OUTLINE),
            with_alpha(rim, 0.55 * sample.alpha),
        );
        shapes::circle(
            batch,
            cx,
            feet_y - h * 0.86 + bob,
            w * 0.22 + 3.0,
            RenderLayer::Entities,
            ys,
            psub(P_OUTLINE),
            with_alpha(rim, 0.55 * sample.alpha),
        );
    }

    // Weapon hangs behind the body when the figure faces away.
    if away {
        draw_weapon(batch, weapon_hand(cx, torso_top, w, h, face, dep), w, h, ys, psub, style, sample, style.weapon, face, tint, true);
        if let Some(off) = style.offhand {
            draw_weapon(batch, weapon_hand(cx, torso_top, w, h, -face, dep), w, h, ys, psub, style, sample, off, -face, tint, true);
        }
    }

    // ── Pixel body ───────────────────────────────────────────────────────────
    let pal = pixel_sprite::Palette::from_style(style);
    let sprite = pixel_sprite::PixelSprite::from_grid(sprites_data::grid(style.shape), &pal, away);
    let px = h / sprite.height() as f32;
    // Horizontal pixel size derives from the figure's *base width* (not its
    // height), so a high-resolution grid still renders a slim figure instead of
    // a near-square blob. Stocky builds widen it, lean ones narrow it.
    let px_x = (w / sprite.width() as f32) * (0.82 + 0.22 * style.build).clamp(0.6, 1.5);
    // A small two-leg scissor offset drives the walk; idle bob is folded in via `bob`.
    let leg_swing = sample.swing * px;
    sprite.draw(batch, cx, feet_y, px, px_x, RenderLayer::Entities, ys, psub(P_TORSO), mirror, leg_swing, bob, tint);

    // ── Off-hand shield (militiaman), opposite the weapon hand ──────────────
    if !away {
        if let Some(shield_c) = style.shield {
            let sx = cx - face * w * 0.30;
            let sy = torso_top + h * 0.14;
            let sw = w * 0.36 * style.build;
            let sh = h * 0.28;
            shapes::rounded_rect(batch, sx - sw * 0.5, sy, sw, sh, sw * 0.32, RenderLayer::Entities, ys, psub(P_WEAPON), tint(shield_c));
            shapes::circle(batch, sx, sy + sh * 0.5, sw * 0.2, RenderLayer::Entities, ys, psub(P_WEAPON), tint(shade(shield_c, 1.3)));
        }
    }

    // ── Weapon(s) in front (when not facing away) ───────────────────────────
    if !away {
        let hand = weapon_hand(cx, torso_top, w, h, face, dep);
        draw_weapon(batch, hand, w, h, ys, psub, style, sample, style.weapon, face, tint, false);
        // Off-hand weapon (dual-wield), mirrored to the opposite hand.
        if let Some(off) = style.offhand {
            let ohand = weapon_hand(cx, torso_top, w, h, -face, dep);
            draw_weapon(batch, ohand, w, h, ys, psub, style, sample, off, -face, tint, false);
        }
    }
}

/// Resolve the weapon hand position for the current facing.
fn weapon_hand(cx: f32, torso_top: f32, w: f32, h: f32, face: f32, dep: f32) -> [f32; 2] {
    // Forward when facing the camera, tucked closer when turned away.
    let reach = lerp(0.16, 0.32, (dep + 1.0) * 0.5);
    [cx + face * w * reach, torso_top + h * 0.20]
}

/// Draw `weapon` held at `hand`, swinging through `sample.attack`, facing the
/// `face` side (+1 right, -1 left). `behind` routes it behind the body (lower
/// sub-order, dimmed) when the figure faces away. Passing an explicit weapon +
/// face lets the off-hand be drawn as a mirrored second weapon.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn draw_weapon(
    batch: &mut SpriteBatch,
    hand: [f32; 2],
    w: f32,
    h: f32,
    ys: i32,
    psub: &dyn Fn(u32) -> u32,
    style: &FigureStyle,
    sample: &AnimSample,
    weapon: Weapon,
    face: f32,
    tint: &dyn Fn(Color) -> Color,
    behind: bool,
) {
    let atk = sample.attack;
    let sub = if behind { psub(P_BACK) } else { psub(P_WEAPON) };
    // Dim weapons drawn behind the body.
    let dim = |c: Color| if behind { shade(c, 0.7) } else { c };
    match weapon {
        Weapon::None => {}
        Weapon::Pitchfork => {
            // A wooden shaft with two short steel prongs fanning out at the tip.
            let len = h * 0.60;
            let ang = face * lerp(-1.05, 0.5, atk);
            let tip = polar(hand, ang, len);
            shapes::line(batch, hand, tip, (w * 0.06).max(2.0), RenderLayer::Entities, ys, sub, tint(dim(WOOD)));
            let prong = len * 0.24;
            for spread in [-0.30f32, 0.30] {
                let p = polar(tip, ang + spread, prong);
                shapes::line(batch, tip, p, (w * 0.05).max(1.0), RenderLayer::Entities, ys, sub, tint(dim(STEEL)));
            }
        }
        Weapon::Sword | Weapon::Axe | Weapon::Bone | Weapon::ShortSword => {
            let (blade, len) = match weapon {
                Weapon::Bone => (tint(dim(BONE)), h * 0.40),
                Weapon::Axe => (tint(dim(STEEL)), h * 0.46),
                Weapon::ShortSword => (tint(dim(STEEL)), h * 0.34),
                _ => (tint(dim(STEEL)), h * 0.52),
            };
            let ang = face * lerp(-1.15, 0.55, atk);
            let tip = polar(hand, ang, len);
            let guard_a = ang + std::f32::consts::FRAC_PI_2;
            let g0 = polar(hand, guard_a, w * 0.12);
            let g1 = polar(hand, guard_a, -w * 0.12);
            shapes::line(batch, g0, g1, (w * 0.07).max(2.0), RenderLayer::Entities, ys, sub, tint(dim(BRASS)));
            shapes::line(batch, hand, tip, (w * 0.07).max(2.0), RenderLayer::Entities, ys, sub, blade);
            if weapon == Weapon::Axe {
                // A broad head near the tip.
                let head = polar(hand, ang, len * 0.78);
                shapes::circle(batch, head[0], head[1], w * 0.16, RenderLayer::Entities, ys, sub, blade);
            }
        }
        Weapon::Staff | Weapon::Wand | Weapon::Scepter => {
            let len = match weapon {
                Weapon::Wand => h * 0.34,
                Weapon::Scepter => h * 0.5,
                _ => h * 0.64,
            };
            // Held mostly upright; tilts toward the cast on attack.
            let ang = -std::f32::consts::FRAC_PI_2 + face * lerp(0.0, 0.5, atk);
            let tip = polar(hand, ang, len);
            shapes::line(batch, hand, tip, (w * 0.06).max(2.0), RenderLayer::Entities, ys, sub, tint(dim(WOOD)));
            // Glowing head, brightening with the cast.
            let glow = shade(style.primary, 1.4);
            let r = w * (0.14 + 0.06 * atk);
            shapes::circle(batch, tip[0], tip[1], r * 1.5, RenderLayer::Entities, ys, sub, tint(dim(with_alpha(glow, 0.5))));
            shapes::circle(batch, tip[0], tip[1], r, RenderLayer::Entities, ys, sub, tint(dim(glow)));
            // Cast flash at the hand.
            if !behind && atk > 0.05 {
                shapes::circle(batch, hand[0], hand[1], w * 0.12 * atk, RenderLayer::Entities, ys, psub(P_WEAPON), tint(with_alpha(glow, 0.6 * atk)));
            }
        }
        Weapon::Orb => {
            // A floating glowing sphere just past the hand.
            let glow = shade(style.primary, 1.5);
            let ox = hand[0] + face * w * (0.06 + 0.1 * atk);
            let oy = hand[1] - h * 0.02;
            let r = w * (0.16 + 0.05 * atk);
            shapes::circle(batch, ox, oy, r * 1.6, RenderLayer::Entities, ys, sub, tint(dim(with_alpha(glow, 0.45))));
            shapes::circle(batch, ox, oy, r, RenderLayer::Entities, ys, sub, tint(dim(glow)));
            shapes::circle(batch, ox, oy, r * 0.5, RenderLayer::Entities, ys, sub, tint(dim(vertex::lerp(glow, vertex::WHITE, 0.6))));
        }
        Weapon::Bow | Weapon::Crossbow => {
            let r = h * 0.34;
            let a0 = face * -1.25;
            let a1 = face * 1.25;
            let lo = a0.min(a1);
            let hi = a0.max(a1);
            shapes::arc(batch, hand[0], hand[1], r, lo, hi, 10, (w * 0.05).max(2.0), RenderLayer::Entities, ys, sub, tint(dim(WOOD)));
            let e0 = polar(hand, lo, r);
            let e1 = polar(hand, hi, r);
            if weapon == Weapon::Crossbow {
                // A stock crossing the bow.
                shapes::line(batch, [hand[0] - face * w * 0.1, hand[1]], [hand[0] + face * (r + w * 0.25), hand[1]], (w * 0.08).max(2.0), RenderLayer::Entities, ys, sub, tint(dim(shade(WOOD, 0.8))));
            }
            shapes::line(batch, e0, e1, (w * 0.03).max(1.0), RenderLayer::Entities, ys, sub, tint(dim([0.85, 0.85, 0.8, 0.8])));
            // Nocked arrow thrusts forward on attack.
            let pull = lerp(0.0, w * 0.5, atk);
            shapes::line(batch, [hand[0] - face * pull, hand[1]], [hand[0] + face * (r + w * 0.2), hand[1]], (w * 0.04).max(1.0), RenderLayer::Entities, ys, sub, tint(dim(STEEL)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shade_clamps() {
        let c = shade([0.5, 0.5, 0.5, 1.0], 4.0);
        assert!(c[0] <= 1.0);
        let d = shade([0.5, 0.5, 0.5, 1.0], 0.5);
        assert!((d[0] - 0.25).abs() < 1e-6);
    }

    #[test]
    fn direction_from_vector_screen_relative() {
        // World +x,+y projects straight down the screen → South.
        assert_eq!(Direction::from_world_vector(1.0, 1.0), Some(Direction::S));
        // World -x,-y → up the screen → North.
        assert_eq!(Direction::from_world_vector(-1.0, -1.0), Some(Direction::N));
        // World +x,-y → screen right → East.
        assert_eq!(Direction::from_world_vector(1.0, -1.0), Some(Direction::E));
        // World -x,+y → screen left → West.
        assert_eq!(Direction::from_world_vector(-1.0, 1.0), Some(Direction::W));
        // Zero vector keeps the old facing.
        assert_eq!(Direction::from_world_vector(0.0, 0.0), None);
    }

    #[test]
    fn facing_away_only_for_northerly() {
        assert!(Direction::N.faces_away());
        assert!(Direction::NE.faces_away());
        assert!(!Direction::S.faces_away());
        assert!(!Direction::E.faces_away());
    }

    #[test]
    fn humanoid_emits_many_quads() {
        let mut b = SpriteBatch::new();
        let cam = Camera2D::new(Position2D::new(0.0, 0.0), 1.0, [800.0, 600.0]);
        let style = FigureStyle::humanoid(vertex::PLAYER, [0.85, 0.68, 0.52, 1.0], Weapon::Sword);
        draw(&mut b, &cam, Position2D::new(0.0, 0.0), [26.0, 40.0], &style, &AnimSample::still(), Direction::S, 1, 0.0);
        assert!(b.len() > 6, "figure should be many parts, got {}", b.len());
    }

    #[test]
    fn dual_wield_draws_a_second_weapon() {
        let cam = Camera2D::new(Position2D::new(0.0, 0.0), 1.0, [800.0, 600.0]);
        let pos = Position2D::new(0.0, 0.0);
        let base = [26.0, 40.0];
        let mut single = FigureStyle::humanoid(vertex::PLAYER, [0.85, 0.68, 0.52, 1.0], Weapon::Sword);
        single.shape = Shape::Soldier;
        let mut both = single;
        both.offhand = Some(Weapon::Axe);

        let mut bs = SpriteBatch::new();
        draw(&mut bs, &cam, pos, base, &single, &AnimSample::still(), Direction::S, 1, 0.0);
        let mut bd = SpriteBatch::new();
        draw(&mut bd, &cam, pos, base, &both, &AnimSample::still(), Direction::S, 1, 0.0);
        assert!(bd.len() > bs.len(), "off-hand weapon should add quads ({} vs {})", bd.len(), bs.len());
    }
}
