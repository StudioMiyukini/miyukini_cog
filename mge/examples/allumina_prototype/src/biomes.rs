//! @id allumina.prototype.biomes
//! @role data
//! @layer application
//! @domain allumina
//! @do biome_tile_registry_and_season_mapping
//!
//! Registre des biomes saisonniers — mapping graphic_id vers saison et sous-type.
//!
//! Atlas source : `seasons_atlas_strip.png` (96 tiles, 128×64 chacun, strip horizontal).
//! Chaque saison contient 24 tiles organisés en 6 sous-types de 4 variantes.

/// Nombre total de tiles dans l'atlas saisonnier.
pub const SEASON_TILE_COUNT: u16 = 96;

/// Nombre de tiles par saison.
pub const TILES_PER_SEASON: u16 = 24;

/// Nombre de variantes par sous-type.
pub const VARIANTS_PER_SUBTYPE: u16 = 4;

// ── Saisons ─────────────────────────────────────────────────────────────────

/// Saison / biome du terrain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    /// Premier graphic_id de la saison dans l'atlas.
    #[inline]
    pub const fn base_id(self) -> u16 {
        match self {
            Self::Spring => 0,
            Self::Summer => 24,
            Self::Autumn => 48,
            Self::Winter => 72,
        }
    }

    /// Dernier graphic_id (inclus) de la saison.
    #[inline]
    pub const fn max_id(self) -> u16 {
        self.base_id() + TILES_PER_SEASON - 1
    }

    /// Détermine la saison à partir d'un graphic_id.
    #[inline]
    pub fn from_graphic_id(gid: u16) -> Option<Self> {
        match gid {
            0..=23 => Some(Self::Spring),
            24..=47 => Some(Self::Summer),
            48..=71 => Some(Self::Autumn),
            72..=95 => Some(Self::Winter),
            _ => None,
        }
    }

    /// Toutes les saisons dans l'ordre de l'atlas.
    pub const ALL: [Season; 4] = [Self::Spring, Self::Summer, Self::Autumn, Self::Winter];

    /// Nom lisible (pour debug/UI).
    pub const fn label(self) -> &'static str {
        match self {
            Self::Spring => "Printemps",
            Self::Summer => "Été",
            Self::Autumn => "Automne",
            Self::Winter => "Hiver",
        }
    }
}

// ── Sous-types de terrain ───────────────────────────────────────────────────

/// Sous-type de terrain au sein d'une saison.
/// Chaque sous-type a 4 variantes (indices 0..3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileSubtype {
    /// Sol plat, texture de base.
    Flat,
    /// Sol avec détails légers (herbe haute, terre visible).
    Detail,
    /// Sol avec éléments notables (cailloux, touffes, mousse).
    Feature,
    /// Sol très marqué (rochers, souches, débris).
    Heavy,
    /// Transition / bord de biome.
    Edge,
    /// Coin / transition avancée.
    Corner,
}

impl TileSubtype {
    /// Offset local (0, 4, 8, 12, 16, 20) au sein d'une saison.
    #[inline]
    pub const fn offset(self) -> u16 {
        match self {
            Self::Flat => 0,
            Self::Detail => 4,
            Self::Feature => 8,
            Self::Heavy => 12,
            Self::Edge => 16,
            Self::Corner => 20,
        }
    }

    /// Détermine le sous-type à partir de l'offset local (0..23).
    pub fn from_local_offset(local: u16) -> Option<Self> {
        match local {
            0..=3 => Some(Self::Flat),
            4..=7 => Some(Self::Detail),
            8..=11 => Some(Self::Feature),
            12..=15 => Some(Self::Heavy),
            16..=19 => Some(Self::Edge),
            20..=23 => Some(Self::Corner),
            _ => None,
        }
    }

    /// Tous les sous-types dans l'ordre.
    pub const ALL: [TileSubtype; 6] = [
        Self::Flat, Self::Detail, Self::Feature,
        Self::Heavy, Self::Edge, Self::Corner,
    ];
}

// ── API de composition ──────────────────────────────────────────────────────

/// Construit un graphic_id à partir d'une saison, un sous-type, et une variante (0..3).
#[inline]
pub const fn tile_id(season: Season, subtype: TileSubtype, variant: u16) -> u16 {
    season.base_id() + subtype.offset() + (variant % VARIANTS_PER_SUBTYPE)
}

/// Décompose un graphic_id en (Season, TileSubtype, variante).
pub fn decompose(gid: u16) -> Option<(Season, TileSubtype, u16)> {
    let season = Season::from_graphic_id(gid)?;
    let local = gid - season.base_id();
    let subtype = TileSubtype::from_local_offset(local)?;
    let variant = local - subtype.offset();
    Some((season, subtype, variant))
}

/// Retourne un graphic_id aléatoire de sol plat pour une saison donnée.
/// Utilise un simple hash du (x, y) pour du pseudo-aléatoire déterministe.
#[inline]
pub fn flat_tile_for(season: Season, x: u32, y: u32) -> u16 {
    let hash = x.wrapping_mul(2654435761).wrapping_add(y.wrapping_mul(340573321));
    let variant = (hash >> 16) as u16 % VARIANTS_PER_SUBTYPE;
    tile_id(season, TileSubtype::Flat, variant)
}

/// Retourne un graphic_id aléatoire (tous sous-types confondus) pour une saison.
#[inline]
pub fn random_tile_for(season: Season, x: u32, y: u32) -> u16 {
    let hash = x.wrapping_mul(2654435761).wrapping_add(y.wrapping_mul(340573321));
    let offset = (hash >> 12) as u16 % TILES_PER_SEASON;
    season.base_id() + offset
}

/// Convertit un graphic_id d'une saison vers une autre (même sous-type + variante).
pub fn convert_season(gid: u16, target: Season) -> Option<u16> {
    let (_, subtype, variant) = decompose(gid)?;
    Some(tile_id(target, subtype, variant))
}

// ── Configuration walkability ───────────────────────────────────────────────

/// Retourne true si ce sous-type de tile est marchable par défaut.
/// Edge et Corner sont considérés comme potentiellement bloquants.
#[inline]
pub const fn is_default_walkable(subtype: TileSubtype) -> bool {
    match subtype {
        TileSubtype::Flat | TileSubtype::Detail | TileSubtype::Feature => true,
        TileSubtype::Heavy => true,  // marchable mais ralenti (futur)
        TileSubtype::Edge | TileSubtype::Corner => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_id_roundtrip() {
        for season in Season::ALL {
            for subtype in TileSubtype::ALL {
                for v in 0..4 {
                    let gid = tile_id(season, subtype, v);
                    let (s2, st2, v2) = decompose(gid).unwrap();
                    assert_eq!(s2, season);
                    assert_eq!(st2, subtype);
                    assert_eq!(v2, v);
                }
            }
        }
    }

    #[test]
    fn test_season_ranges() {
        assert_eq!(Season::from_graphic_id(0), Some(Season::Spring));
        assert_eq!(Season::from_graphic_id(23), Some(Season::Spring));
        assert_eq!(Season::from_graphic_id(24), Some(Season::Summer));
        assert_eq!(Season::from_graphic_id(48), Some(Season::Autumn));
        assert_eq!(Season::from_graphic_id(72), Some(Season::Winter));
        assert_eq!(Season::from_graphic_id(95), Some(Season::Winter));
        assert_eq!(Season::from_graphic_id(96), None);
    }

    #[test]
    fn test_convert_season() {
        let spring_flat_2 = tile_id(Season::Spring, TileSubtype::Flat, 2);
        let winter_flat_2 = convert_season(spring_flat_2, Season::Winter).unwrap();
        assert_eq!(winter_flat_2, tile_id(Season::Winter, TileSubtype::Flat, 2));
    }

    #[test]
    fn test_flat_tile_deterministic() {
        let a = flat_tile_for(Season::Spring, 10, 20);
        let b = flat_tile_for(Season::Spring, 10, 20);
        assert_eq!(a, b);
        assert!(a < 4); // Spring flat = 0..3
    }
}
