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

/// Soldat: flat steel helm with a visor band, broad armoured pauldrons, a
/// belted steel-blue cuirass, plate skirt, and leather boots.
const SOLDIER: &[&str] = &[
    "................",
    "....mmmmmmmm....",
    "...mmmmmmmmmm...",
    "...kkkkkkkkkk...",
    "....ssssssss....",
    "....ssessess....",
    "....ssssssss....",
    "..mmppppppppmm..",
    "..mppppppppppm..",
    "...pppppppppp...",
    "...plpppppplp...",
    "...pppppppppp...",
    "...aaaaaaaaaa...",
    "...kkkkkkkkkk...",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....www..www....",
    "...wwww..wwww...",
    "...wwww..wwww...",
];

/// Clerc: a rounded cowl framing the face, a long flared robe (no legs) with a
/// gold sash and a gilded hem.
const CLERIC: &[&str] = &[
    "................",
    ".....pppppp.....",
    "....pppppppp....",
    "....pssssssp....",
    "....psessesp....",
    "....pssssssp....",
    "....pppppppp....",
    "...pppppppppp...",
    "...pppppppppp...",
    "..pppppppppppp..",
    "..pppppppppppp..",
    "..ppppggggpppp..",
    "..pppppppppppp..",
    ".pppppppppppppp.",
    ".pppppppppppppp.",
    ".pgpppppppppgp..",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "pppppppppppppppp",
    "gggggggggggggggg",
    ".pppppppppppppp.",
];

/// Mage de guerre: a tall pointed hat with a wide brim, mid-length arcane robe,
/// and stubby boots peeking under the hem (the orb is drawn as a weapon).
const WARMAGE: &[&str] = &[
    ".......pp.......",
    "......pppp......",
    "......pppp......",
    ".....pppppp.....",
    ".....pppppp.....",
    "...pppppppppp...",
    "....ssssssss....",
    "....ssessess....",
    "....ssssssss....",
    "...pppppppppp...",
    "...pppppppppp...",
    "..pppppppppppp..",
    "..pppppppppppp..",
    "..pppppppppppp..",
    ".pppppppppppppp.",
    ".pppppppppppppp.",
    "..dddddddddd....",
    "....dd...dd.....",
    "....dd...dd.....",
    "....dd...dd.....",
    "....ww...ww.....",
    "....ww...ww.....",
];

/// Archer: a short hood, a slim leather-strapped torso, a belt, and lean legs
/// (the bow and quiver are layered on by the renderer).
const ARCHER: &[&str] = &[
    "................",
    ".....pppppp.....",
    "....pppppppp....",
    "....pssssssp....",
    "....psessesp....",
    "....pssssssp....",
    "....ssssssss....",
    "...lppppppppl...",
    "...wppppppppw...",
    "....pppppppp....",
    "....pppppppp....",
    "....wwwwwwww....",
    "....pppppppp....",
    ".....dd..dd.....",
    ".....dd..dd.....",
    ".....dd..dd.....",
    ".....dd..dd.....",
    ".....dd..dd.....",
    ".....ww..ww.....",
    ".....ww..ww.....",
];

/// Generic humanoid (mercs, NPCs, skeletons, zombies, robed casters): bare head
/// with hair, a primary-coloured torso with shoulders, arms, legs, and boots.
/// Skin/primary/eyes all come from the style, so the same grid reads as bone,
/// rotting green, or robed magenta depending on the entity.
const BRUTE: &[&str] = &[
    "................",
    "......hhhh......",
    "....hssssssh....",
    "....ssssssss....",
    "....ssessess....",
    ".....ssssss.....",
    "..sllpppppplls..",
    "..sppppppppps...",
    "..sppppppppps...",
    "...pppppppppp...",
    "...pppppppppp...",
    "...pppppppppp...",
    "...aaaaaaaaaa...",
    "...dddddddddd...",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....ddd..ddd....",
    "....www..www....",
    "....www..www....",
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
