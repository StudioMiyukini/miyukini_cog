//! @id     games_allumina_render_sprites_data
//! @do     definir_soldier
//! @role   donnee
//! @layer  core
//! @human  games allumina — module « render sprites data ».

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

/// Soldat: domed steel helm with a visor band, plumed crest, armoured pauldrons
/// over a riveted steel-blue cuirass, a studded belt, a plate skirt, and greaved
/// legs with boots. 24-wide grid for crisper armour detail.
const SOLDIER: &[&str] = &[
    "........................",
    "...........gg...........",
    "..........gggg..........",
    "........mmmmmmmm........",
    ".......mmmmmmmmmm.......",
    "......mmmmmmmmmmmm......",
    "......mkkkkkkkkkkm......",
    "......mssssssssssm......",
    ".......sseesseess.......",
    ".......ssssssssss.......",
    "...........ss...........",
    ".....mmppppppppppmm.....",
    ".....mppppppppppppm.....",
    "......pppppppppppp......",
    "......ppllppppllpp......",
    ".......pppppppppp.......",
    ".......aaaaggaaaa.......",
    "........kkkkkkkk........",
    "........ddd..ddd........",
    "........ppp..ppp........",
    "........ddd..ddd........",
    "........ppp..ppp........",
    "........ddd..ddd........",
    "........ppp..ppp........",
    "........www..www........",
    ".......wwww..wwww.......",
    "........................",
];

/// Clerc: a rounded cowl framing the face over a narrow neck, falling into a
/// long flared robe (no legs) with a gold sash, panelled folds, and a gilded hem.
const CLERIC: &[&str] = &[
    "........................",
    "........pppppppp........",
    ".......pppppppppp.......",
    "......ppppppppppppp.....",
    "......ppssssssssspp.....",
    "......ppsseesseespp.....",
    "......ppssssssssspp.....",
    ".......ppssssssspp......",
    "........pppssspppp......",
    ".......pppppppppppp.....",
    "......pppppppppppppp....",
    "......ppppppggppppppp...",
    "......ppppggggggpppp....",
    "......pppppppppppppp....",
    ".....ppppppppppppppppp..",
    ".....ppplppppppplpppp...",
    "....pppppppppppppppppp..",
    "....pppplppppppplppppp..",
    "...pppppppppppppppppppp.",
    "...pppppppppppppppppppp.",
    "..pppppplppppppplpppppp.",
    "..pppppppppppppppppppppp",
    "..pppppppppppppppppppppp",
    "..gggggggggggggggggggggg",
    "...pppppppppppppppppppp.",
    "........................",
];

/// Mage de guerre: a tall pointed hat with a wide brim and a band, a slim neck,
/// a flowing arcane robe with folds, and pointed boots peeking under the hem
/// (the orb is drawn as a weapon by the renderer).
const WARMAGE: &[&str] = &[
    "........................",
    "...........pp...........",
    "..........pppp..........",
    "..........pppp..........",
    ".........pppppp.........",
    ".........pddddp.........",
    ".........pppppp.........",
    "......pppppppppppp......",
    ".......sssssssssss......",
    ".......sseesseess.......",
    ".......ssssssssss.......",
    "........ssssssss........",
    "..........ssss..........",
    "......pppppppppppp......",
    "......pplppppppldp......",
    ".....pppppppppppppp.....",
    ".....ppplpppppppdpp.....",
    "....pppppppppppppppp....",
    "....pppplppppppdpppp....",
    "...pppppppppppppppppp...",
    "...ppppppddddpppppppp...",
    "....dddddddddddddddd....",
    "......ww........ww......",
    ".....www........www.....",
    "....wwww........wwww....",
    "........................",
];

/// Archer: a short pointed hood framing the face, a slim leather-strapped torso
/// with a quiver strap, a belt, lean legs, and soft boots (the bow and quiver
/// are layered on by the renderer).
const ARCHER: &[&str] = &[
    "........................",
    ".........pppppp.........",
    "........pppppppp........",
    ".......pppppppppp.......",
    "......ppsssssssspp......",
    ".......sseesseess.......",
    ".......ssssxxssss.......",
    "........ssssssss........",
    "...........ss...........",
    ".....ddppppppppppdd.....",
    ".....dppppppppppppd.....",
    "......dppppppppppd......",
    "......wwwwwwwwwwww......",
    ".......pppppppppp.......",
    ".......aaaaaaaaaa.......",
    "........dddddddd........",
    ".........pp..pp.........",
    ".........pp..pp.........",
    ".........dd..dd.........",
    ".........pp..pp.........",
    ".........pp..pp.........",
    ".........dd..dd.........",
    ".........ww..ww.........",
    "........www..www........",
    "........................",
];

/// Generic humanoid (mercs, NPCs, skeletons, zombies, robed casters): bare head
/// with hair, a face with two eyes and a mouth, a neck, a shaded torso with
/// shoulders/arms and skin hands, a belt, then legs and boots. Skin/primary/eyes
/// all come from the style, so the same grid reads as bone, rotting green, or
/// robed magenta. 24-wide for finer features.
const BRUTE: &[&str] = &[
    "........................",
    ".........hhhhhh.........",
    "........hhhhhhhh........",
    ".......hhhhhhhhhh.......",
    "......hhsssssssshh......",
    "......hssssssssssh......",
    ".......sseesseess.......",
    ".......ssssxxssss.......",
    "........ssssssss........",
    "...........ss...........",
    ".....ddppppppppppdd.....",
    ".....dppppppppppppd.....",
    "......dppppppppppd......",
    "......sppppllpppps......",
    ".......pppppppppp.......",
    ".......aaaaaaaaaa.......",
    "........dddddddd........",
    "........ppp..ppp........",
    "........ppp..ppp........",
    "........ppp..ppp........",
    "........ddd..ddd........",
    "........ppp..ppp........",
    "........ppp..ppp........",
    "........ddd..ddd........",
    "........www..www........",
    ".......wwww..wwww.......",
    "........................",
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
