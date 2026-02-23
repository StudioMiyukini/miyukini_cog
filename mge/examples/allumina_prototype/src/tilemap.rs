//! @id allumina.prototype.tilemap
//! @role data
//! @layer application
//! @domain allumina
//! @do tilemap_data_and_json_loader
//!
//! Tilemap isométrique — Chargement JSON et itération des tiles visibles.
//!
//! Conforme au MVP Sandbox : TileMap, Tile, graphic_id, flags.

use serde::Deserialize;
use std::path::Path;

/// Flags de tuile (MVP Sandbox)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TileFlags(pub u16);

impl TileFlags {
    pub const WALKABLE: u16 = 0x01;
    pub const WATER: u16 = 0x02;
    pub const HARVESTABLE: u16 = 0x04;
    pub const BLOCKED: u16 = 0x08;

    #[inline]
    pub fn contains(&self, flag: u16) -> bool {
        self.0 & flag != 0
    }

    #[inline]
    pub fn is_walkable(&self) -> bool {
        self.contains(Self::WALKABLE) && !self.contains(Self::BLOCKED)
    }
}

/// Tuile individuelle (MVP Sandbox)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub graphic_id: u16,
    pub altitude: i8,
    pub flags: TileFlags,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            graphic_id: 0,
            altitude: 0,
            flags: TileFlags(TileFlags::WALKABLE),
        }
    }
}

impl Tile {
    pub fn new(graphic_id: u16, flags: TileFlags) -> Self {
        Self {
            graphic_id,
            altitude: 0,
            flags,
        }
    }
}

/// Carte de tuiles (MVP Sandbox)
#[derive(Debug, Clone)]
pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}

impl TileMap {
    /// Crée une TileMap vide
    pub fn new(width: u32, height: u32) -> Self {
        let len = (width as usize).saturating_mul(height as usize);
        Self {
            width,
            height,
            tiles: vec![Tile::default(); len],
        }
    }

    /// Index linéaire pour (x, y)
    #[inline]
    pub fn index(&self, x: u32, y: u32) -> usize {
        (y as usize).saturating_mul(self.width as usize).saturating_add(x as usize)
    }

    /// Retourne la tuile à (x, y), ou default si hors bornes
    #[inline]
    pub fn get(&self, x: u32, y: u32) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.tiles.get(self.index(x, y))
    }

    /// Retourne la tuile à (x, y) de manière mutable
    #[inline]
    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let i = self.index(x, y);
        self.tiles.get_mut(i)
    }

    /// Définit la tuile à (x, y)
    #[inline]
    pub fn set(&mut self, x: u32, y: u32, tile: Tile) {
        if let Some(t) = self.get_mut(x, y) {
            *t = tile;
        }
    }

    /// Retourne true si la cellule (x, y) est marchable (pour pathfinding/spawn).
    #[inline]
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }
        self.get(x as u32, y as u32)
            .map(|t| t.flags.is_walkable())
            .unwrap_or(false)
    }

    /// Itère sur les tuiles visibles dans le rectangle monde (en coordonnées tile)
    /// Bounds : (min_x, min_y, max_x, max_y) inclus
    pub fn visible_tiles(
        &self,
        min_tx: i32,
        min_ty: i32,
        max_tx: i32,
        max_ty: i32,
    ) -> impl Iterator<Item = (i32, i32, &Tile)> {
        let width = self.width as i32;
        let height = self.height as i32;
        let tiles = &self.tiles;

        (min_ty..=max_ty).flat_map(move |ty| {
            (min_tx..=max_tx).filter_map(move |tx| {
                if tx >= 0 && tx < width && ty >= 0 && ty < height {
                    let idx = (ty as usize) * (width as usize) + (tx as usize);
                    tiles.get(idx).map(|t| (tx, ty, t))
                } else {
                    None
                }
            })
        })
    }

    /// Charge depuis un fichier JSON
    pub fn load_from_json<P: AsRef<Path>>(path: P) -> Result<Self, TileMapLoadError> {
        let path = path.as_ref();
        let data = std::fs::read_to_string(path).map_err(TileMapLoadError::Io)?;
        let loaded: TileMapJson = serde_json::from_str(&data).map_err(TileMapLoadError::Json)?;
        let base_dir = path.parent().unwrap_or(Path::new("."));
        Ok(loaded.into_tilemap_with_chunks(base_dir))
    }
}

#[derive(Debug)]
pub enum TileMapLoadError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

/// Référence à un chunk préfabriqué (fichier + offset)
#[derive(Deserialize)]
struct ChunkRefJson {
    file: String,
    x: u32,
    y: u32,
}

/// Zone rectangulaire pour génération procédurale (Phase 4)
#[derive(Deserialize)]
struct ZoneJson {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    #[serde(rename = "g")]
    graphic_id: u16,
    #[serde(rename = "f", default)]
    flags: u16,
}

/// Format JSON pour le chargement (flat array : [graphic_id, flags] ou [g, f, g, f, ...])
#[derive(Deserialize)]
struct TileMapJson {
    width: u32,
    height: u32,
    /// Données des tiles : tableau plat [graphic_id, flags] par tile, ou "tiles" avec objets
    #[serde(default)]
    tiles: Option<Vec<TileJson>>,
    /// Alternatif : tableau plat de graphic_ids (flags = WALKABLE par défaut)
    #[serde(default)]
    data: Option<Vec<u16>>,
    /// Zones rectangulaires (Phase 4) — village, forêt, mines, donjon
    #[serde(default)]
    zones: Option<Vec<ZoneJson>>,
    /// graphic_id par défaut si zones utilisées (0 = herbe)
    #[serde(rename = "default_g", default)]
    default_graphic: u16,
    /// Chunks préfabriqués à fusionner (ex. chunks grass 100x100)
    #[serde(default)]
    chunks: Option<Vec<ChunkRefJson>>,
}

#[derive(Deserialize)]
struct TileJson {
    #[serde(default)]
    g: u16,
    #[serde(default)]
    f: u16,
    #[serde(default)]
    a: i8,
}

#[derive(Deserialize)]
struct ChunkJson {
    width: u32,
    height: u32,
    data: Vec<u16>,
}

impl TileMapJson {
    fn into_tilemap_with_chunks(self, base_dir: &Path) -> TileMap {
        let len = (self.width as usize).saturating_mul(self.height as usize);
        let default_flags = TileFlags::WALKABLE;
        // 1) Base : default ou data/tiles. 2) Chunks grass. 3) Zones (village, eau, etc.)
        let mut tiles = if let Some(ref data) = self.data {
            data.iter()
                .take(len)
                .map(|&g| Tile::new(g, TileFlags(default_flags)))
                .collect()
        } else if let Some(ref arr) = self.tiles {
            arr.iter()
                .take(len)
                .map(|t| Tile {
                    graphic_id: t.g,
                    altitude: t.a,
                    flags: TileFlags(t.f),
                })
                .collect()
        } else if let Some(ref zones) = self.zones {
            let mut t: Vec<Tile> = (0..len)
                .map(|_| Tile::new(self.default_graphic, TileFlags(default_flags)))
                .collect();
            let w = self.width as usize;

            // Fusion des chunks AVANT les zones (chunks = grass varié, zones = village/eau/etc.)
            if let Some(ref chunks) = self.chunks {
                for c in chunks {
                    let chunk_path = base_dir.join(&c.file);
                    if let Ok(data) = std::fs::read_to_string(&chunk_path) {
                        if let Ok(chunk) = serde_json::from_str::<ChunkJson>(&data) {
                            let cw = chunk.width as usize;
                            let ch = chunk.height as usize;
                            for dy in 0..ch {
                                for dx in 0..cw {
                                    let mx = c.x as usize + dx;
                                    let my = c.y as usize + dy;
                                    if mx < w {
                                        let idx = my * w + mx;
                                        if idx < t.len() {
                                            let src_idx = dy * cw + dx;
                                            if src_idx < chunk.data.len() {
                                                let g = chunk.data[src_idx] % 8;
                                                t[idx] = Tile::new(g, TileFlags(default_flags));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for z in zones {
                let x1 = z.x as usize;
                let y1 = z.y as usize;
                let x2 = (z.x + z.w).min(self.width) as usize;
                let y2 = (z.y + z.h).min(self.height) as usize;
                for ty in y1..y2 {
                    for tx in x1..x2 {
                        let idx = ty * w + tx;
                        if idx < t.len() {
                            let f = if z.flags != 0 {
                                TileFlags(z.flags)
                            } else {
                                TileFlags(default_flags)
                            };
                            t[idx] = Tile::new(z.graphic_id, f);
                        }
                    }
                }
            }
            t
        } else {
            let mut t: Vec<Tile> = (0..len)
                .map(|_| Tile::new(self.default_graphic, TileFlags(default_flags)))
                .collect();
            let w = self.width as usize;
            if let Some(ref chunks) = self.chunks {
                for c in chunks {
                    let chunk_path = base_dir.join(&c.file);
                    if let Ok(data) = std::fs::read_to_string(&chunk_path) {
                        if let Ok(chunk) = serde_json::from_str::<ChunkJson>(&data) {
                            let cw = chunk.width as usize;
                            let ch = chunk.height as usize;
                            for dy in 0..ch {
                                for dx in 0..cw {
                                    let mx = c.x as usize + dx;
                                    let my = c.y as usize + dy;
                                    if mx < w {
                                        let idx = my * w + mx;
                                        if idx < t.len() {
                                            let src_idx = dy * cw + dx;
                                            if src_idx < chunk.data.len() {
                                                let g = chunk.data[src_idx] % 8;
                                                t[idx] = Tile::new(g, TileFlags(default_flags));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            t
        };

        // Pad si nécessaire
        while tiles.len() < len {
            tiles.push(Tile::default());
        }
        tiles.truncate(len);

        TileMap {
            width: self.width,
            height: self.height,
            tiles,
        }
    }
}
