//! Authored pixel-art grids, one per [`Shape`]. Each grid is a column of rows,
//! one character per pixel, resolved against a
//! [`Palette`](super::pixel_sprite::Palette):
//!
//! - `.`/space transparent · `p` primary · `l` primary-light · `d` primary-dark
//! - `a` accent (belt/trim) · `s` skin · `x` skin-dark · `h` hair
//! - `e` eyes · `m` steel · `k` steel-dark · `g` gold · `b` bone · `w` wood/boots
//!
//! Grids are the front (S) pose; the renderer mirrors them for W-facings and
//! derives a faceless back view for N-facings. Weapons, auras, and the rim
//! outline are layered on top procedurally in [`figures`](super::figures).

use super::figures::Shape;

/// Soldat: domed steel helm with a visor band, armoured pauldrons over a belted
/// steel-blue cuirass, a short plate skirt, and greaved legs with boots. Taller,
/// leaner proportions (smaller head, longer legs) than the old squat sprite.
const SOLDIER: &[&str] = &[
    "................",
    "....mmmmmmmm....",
    "...mmmmmmmmmm...",
    "...kkkkkkkkkk...",
    "....ssssssss....",
    "....ssessess....",
    "....ssssssss....",
    "......ssss......",
    "..mmppppppppmm..",
    "..mppppppppppm..",
    "...pppppppppp...",
    "...plpppppplp...",
    "...pppppppppp...",
    "...aaaaaaaaaa...",
    "...kkkkkkkkkk...",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....ppp..ppp....",
    "....ppp..ppp....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....www..www....",
    "...wwww..wwww...",
    "................",
];

/// Clerc: a rounded cowl framing the face over a tall narrow neck, falling into
/// a long flared robe (no legs) with a gold sash and a gilded hem.
const CLERIC: &[&str] = &[
    "................",
    ".....pppppp.....",
    "....pppppppp....",
    "....pssssssp....",
    "....psessesp....",
    "....pssssssp....",
    "......ssss......",
    "....pppppppp....",
    "...pppppppppp...",
    "...pppppppppp...",
    "..pppppppppppp..",
    "..ppppggggpppp..",
    "..pppppppppppp..",
    "..pppppppppppp..",
    ".pppppppppppppp.",
    ".pppppppppppppp.",
    ".pppppppppppppp.",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "gggggggggggggggg",
    ".pppppppppppppp.",
    "................",
];

/// Mage de guerre: a tall pointed hat with a wide brim, a slim neck, a
/// mid-length arcane robe, and pointed boots peeking under the hem (the orb is
/// drawn as a weapon by the renderer).
const WARMAGE: &[&str] = &[
    "................",
    ".......pp.......",
    "......pppp......",
    "......pppp......",
    ".....pppppp.....",
    "...pppppppppp...",
    "....ssssssss....",
    "....ssessess....",
    "....ssssssss....",
    "......ssss......",
    "...pppppppppp...",
    "...pppppppppp...",
    "..pppppppppppp..",
    "..pppppppppppp..",
    ".pppppppppppppp.",
    ".pppppppppppppp.",
    "...dddddddddd...",
    "...dddddddddd...",
    ".....ww..ww.....",
    ".....ww..ww.....",
    ".....ww..ww.....",
    "................",
];

/// Archer: a short hood framing the face, a slim leather-strapped torso, a belt,
/// and lean two-pixel legs (the bow and quiver are layered on by the renderer).
const ARCHER: &[&str] = &[
    "................",
    ".....pppppp.....",
    "....pppppppp....",
    "....pssssssp....",
    "....psessesp....",
    "....pssssssp....",
    "......ssss......",
    "....pppppppp....",
    "...spppppppps...",
    "....pppppppp....",
    "....wwwwwwww....",
    "....pppppppp....",
    ".....pp..pp.....",
    ".....dd..dd.....",
    ".....pp..pp.....",
    ".....pp..pp.....",
    ".....dd..dd.....",
    ".....dd..dd.....",
    ".....ww..ww.....",
    "................",
];

/// Generic humanoid (mercs, NPCs, skeletons, zombies, robed casters): bare head
/// with hair, a neck, a primary torso with shaded shoulder/arm columns and skin
/// hands, then legs and boots. Skin/primary/eyes all come from the style, so the
/// same grid reads as bone, rotting green, or robed magenta. Re-proportioned for
/// a less chibi, more human silhouette (smaller head, neck, longer legs).
const BRUTE: &[&str] = &[
    "................",
    ".....hhhhhh.....",
    "....hhhhhhhh....",
    "....hssssssh....",
    "....ssessess....",
    "....ssssssss....",
    "......ssss......",
    "...dppppppppd...",
    "...dppppppppd...",
    "...spppppppps...",
    "....pppppppp....",
    "....pppppppp....",
    "....aaaaaaaa....",
    "....dddddddd....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....ppp..ppp....",
    "....ppp..ppp....",
    "....ppp..ppp....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....www..www....",
    "...wwww..wwww...",
    "................",
];

/// Golem: a hulking stone block, full-width torso with boulder fists, glowing
/// eyes, and short stubby legs.
const GOLEM: &[&str] = &[
    "................",
    "................",
    ".....pppp.......",
    ".....peep.......",
    ".....pppp.......",
    "...pppppppppp...",
    "..pppppppppppp..",
    ".pppppppppppppp.",
    ".pppppppppppppp.",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "lppppppppppppppl",
    "lppppppppppppppl",
    ".pppppppppppppp.",
    ".pppppppppppppp.",
    "..pppp..pppp....",
    "..pppp..pppp....",
    "..pppp..pppp....",
    "..dddd..dddd....",
    "..dddd..dddd....",
];

/// The authored grid for a shape (front / S pose).
#[must_use]
pub fn grid(shape: Shape) -> &'static [&'static str] {
    match shape {
        Shape::Soldier => SOLDIER,
        Shape::Cleric => CLERIC,
        Shape::Warmage => WARMAGE,
        Shape::Archer => ARCHER,
        Shape::Brute => BRUTE,
        Shape::Golem => GOLEM,
    }
}
