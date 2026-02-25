//! @id allumina.prototype.constants
//! @role constants
//! @layer application
//! @domain allumina
//! @do shared_game_constants

/// Taille de la carte en tiles
pub const MAP_SIZE: u32 = 96;
pub const MAP_SIZE_F: f32 = 96.0;
/// Limite basse (tiles depuis le bord)
pub const MAP_MIN_F: f32 = 0.5;
/// Limite haute
pub const MAP_MAX_F: f32 = 95.5;

// ── Camp de départ (style D2 Rogue Encampment) ──────────────────────────────

/// Centre du camp (et de la safe zone)
pub const SAFE_ZONE_X: f32 = 48.0;
pub const SAFE_ZONE_Y: f32 = 48.0;
/// Rayon de la safe zone (tiles) — les mobs n'agressent pas dans cette zone
pub const SAFE_ZONE_RADIUS: f32 = 20.0;

/// Limites du camp en tiles (rectangle avec bordure d'eau)
/// Le camp fait ~20×16 tiles, centré à (48,48)
pub const CAMP_X_MIN: u32 = 38; // 48 - 10
pub const CAMP_X_MAX: u32 = 58; // 48 + 10
pub const CAMP_Y_MIN: u32 = 40; // 48 - 8
pub const CAMP_Y_MAX: u32 = 56; // 48 + 8

/// Passage SE : ouverture dans la bordure d'eau (3 tiles de large)
/// Position : coin sud-est du camp, centré sur Y=CAMP_Y_MAX, X=CAMP_X_MAX
pub const PASSAGE_X_MIN: u32 = 56; // X début du passage
pub const PASSAGE_X_MAX: u32 = 59; // X fin du passage (exclusif)
pub const PASSAGE_Y_MIN: u32 = 54; // Y début du passage
pub const PASSAGE_Y_MAX: u32 = 57; // Y fin du passage (exclusif)

/// Teste si une position monde est dans la safe zone.
#[inline]
pub fn in_safe_zone(wx: f32, wy: f32) -> bool {
    let dx = wx - SAFE_ZONE_X;
    let dy = wy - SAFE_ZONE_Y;
    dx * dx + dy * dy <= SAFE_ZONE_RADIUS * SAFE_ZONE_RADIUS
}

/// Teste si une tile (tx,ty) fait partie de l'intérieur du camp (walkable).
#[inline]
pub fn is_camp_interior(tx: u32, ty: u32) -> bool {
    tx > CAMP_X_MIN && tx < CAMP_X_MAX && ty > CAMP_Y_MIN && ty < CAMP_Y_MAX
}

/// Teste si une tile (tx,ty) est la bordure d'eau du camp.
#[inline]
pub fn is_camp_water(tx: u32, ty: u32) -> bool {
    // La bordure = les tiles qui sont sur le rectangle du camp mais pas à l'intérieur
    let on_border = (tx >= CAMP_X_MIN && tx <= CAMP_X_MAX && ty >= CAMP_Y_MIN && ty <= CAMP_Y_MAX)
        && !is_camp_interior(tx, ty);

    if !on_border {
        return false;
    }

    // Exception : le passage SE reste ouvert (pas d'eau)
    if tx >= PASSAGE_X_MIN && tx < PASSAGE_X_MAX && ty >= PASSAGE_Y_MIN && ty < PASSAGE_Y_MAX {
        return false;
    }

    true
}

/// Teste si une position monde est dans les limites du camp (intérieur + bordure).
/// Utilisé pour empêcher le spawn de mobs dans le camp.
#[inline]
pub fn in_camp_zone(wx: f32, wy: f32) -> bool {
    let tx = wx as u32;
    let ty = wy as u32;
    tx >= CAMP_X_MIN && tx <= CAMP_X_MAX && ty >= CAMP_Y_MIN && ty <= CAMP_Y_MAX
}
