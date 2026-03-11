use crate::constants::*;
use crate::entity::*;

// ---------------------------------------------------------------------------
// Chunk coordinate — each zone occupies MAP_W × MAP_H in the world grid.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkCoord {
    pub cx: i32,
    pub cy: i32,
}

impl ChunkCoord {
    pub const fn new(cx: i32, cy: i32) -> Self {
        Self { cx, cy }
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn world_origin(self) -> [f32; 2] {
        [self.cx as f32 * MAP_W as f32, self.cy as f32 * MAP_H as f32]
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn to_local(self, world_pos: [f32; 2]) -> [f32; 2] {
        let o = self.world_origin();
        [world_pos[0] - o[0], world_pos[1] - o[1]]
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn to_world(self, local: [f32; 2]) -> [f32; 2] {
        let o = self.world_origin();
        [local[0] + o[0], local[1] + o[1]]
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn contains(self, world_pos: [f32; 2]) -> bool {
        let local = self.to_local(world_pos);
        local[0] >= 0.0
            && local[0] < MAP_W as f32
            && local[1] >= 0.0
            && local[1] < MAP_H as f32
    }
}

// ---------------------------------------------------------------------------
// Zone placements — 5×5 grid centered on (0,0) = Rogue Camp
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct ZonePlacement {
    pub zone_id: ZoneId,
    pub coord: ChunkCoord,
}

/// Act 1 open world layout (each chunk = 100×100 tiles = 100×100m):
///
/// ```text
///        x=-2       x=-1       x=0        x=1        x=2
/// y=-2  DkWoodNW   FrozenPath ColdPlnN   HilandsNE  RockyWaste
/// y=-1  DkWoodW    DenOfEvil  BloodMoorN ColdPlnNE  StonyFldE
/// y= 0  TamoeFooth BloodMoorW RogueCamp  BloodMoorE DarkWoodE
/// y= 1  TamoeHigh  MeadowSW   BloodMoorS GraveyardSE BurialGnd
/// y= 2  OutCloistr DryHills   BlackMarsh CataEntry  FarOasis
/// ```
///
/// Total: 25 zones, world spans 500×500 tiles (500m×500m).
pub const ZONE_PLACEMENTS: &[ZonePlacement] = &[
    // Center — Rogue Camp
    ZonePlacement { zone_id: ZoneId::RogueCamp,       coord: ChunkCoord::new(0, 0) },
    // Ring 1 — cardinal neighbors
    ZonePlacement { zone_id: ZoneId::BloodMoorN,      coord: ChunkCoord::new(0, -1) },
    ZonePlacement { zone_id: ZoneId::BloodMoorE,      coord: ChunkCoord::new(1, 0) },
    ZonePlacement { zone_id: ZoneId::BloodMoorS,      coord: ChunkCoord::new(0, 1) },
    ZonePlacement { zone_id: ZoneId::BloodMoorW,      coord: ChunkCoord::new(-1, 0) },
    // Ring 1 — diagonal neighbors
    ZonePlacement { zone_id: ZoneId::DenOfEvil,       coord: ChunkCoord::new(-1, -1) },
    ZonePlacement { zone_id: ZoneId::ColdPlainsNE,    coord: ChunkCoord::new(1, -1) },
    ZonePlacement { zone_id: ZoneId::GraveyardSE,     coord: ChunkCoord::new(1, 1) },
    ZonePlacement { zone_id: ZoneId::MeadowSW,        coord: ChunkCoord::new(-1, 1) },
    // Ring 2 — north row
    ZonePlacement { zone_id: ZoneId::ColdPlainsN,     coord: ChunkCoord::new(0, -2) },
    ZonePlacement { zone_id: ZoneId::FrozenPath,      coord: ChunkCoord::new(-1, -2) },
    // Ring 2 — east column
    ZonePlacement { zone_id: ZoneId::StonyFieldE,     coord: ChunkCoord::new(2, -1) },
    ZonePlacement { zone_id: ZoneId::DarkWoodE,       coord: ChunkCoord::new(2, 0) },
    // Ring 2 — south row
    ZonePlacement { zone_id: ZoneId::BurialGrounds,   coord: ChunkCoord::new(2, 1) },
    ZonePlacement { zone_id: ZoneId::BlackMarshS,     coord: ChunkCoord::new(0, 2) },
    // Ring 2 — west column
    ZonePlacement { zone_id: ZoneId::TamoeFoothills,  coord: ChunkCoord::new(-2, 0) },
    ZonePlacement { zone_id: ZoneId::DarkWoodW,       coord: ChunkCoord::new(-2, -1) },
    // Ring 2 — corners
    ZonePlacement { zone_id: ZoneId::DarkWoodNW,      coord: ChunkCoord::new(-2, -2) },
    ZonePlacement { zone_id: ZoneId::HighlandsNE,     coord: ChunkCoord::new(1, -2) },
    ZonePlacement { zone_id: ZoneId::CatacombsEntry,  coord: ChunkCoord::new(1, 2) },
    ZonePlacement { zone_id: ZoneId::TamoeHighlands,  coord: ChunkCoord::new(-2, 1) },
    // Ring 2 — outer edges
    ZonePlacement { zone_id: ZoneId::RockyWaste,      coord: ChunkCoord::new(2, -2) },
    ZonePlacement { zone_id: ZoneId::DryHills,        coord: ChunkCoord::new(-1, 2) },
    ZonePlacement { zone_id: ZoneId::FarOasis,        coord: ChunkCoord::new(2, 2) },
    ZonePlacement { zone_id: ZoneId::OuterCloister,   coord: ChunkCoord::new(-2, 2) },
];

/// Find the chunk coordinate for a given zone.
pub fn zone_coord(zone: ZoneId) -> ChunkCoord {
    ZONE_PLACEMENTS
        .iter()
        .find(|p| p.zone_id == zone)
        .expect("zone not in ZONE_PLACEMENTS")
        .coord
}

/// Find which zone a world-space position belongs to, if any.
pub fn zone_at_world(world_pos: [f32; 2]) -> Option<ZoneId> {
    ZONE_PLACEMENTS
        .iter()
        .find(|p| p.coord.contains(world_pos))
        .map(|p| p.zone_id)
}

// ---------------------------------------------------------------------------
// Visibility rect
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct WorldRect {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl WorldRect {
    pub fn around(center: [f32; 2], half_w: f32, half_h: f32) -> Self {
        Self {
            min_x: center[0] - half_w,
            min_y: center[1] - half_h,
            max_x: center[0] + half_w,
            max_y: center[1] + half_h,
        }
    }

    #[must_use]
    pub fn expand(self, margin: f32) -> Self {
        Self {
            min_x: self.min_x - margin,
            min_y: self.min_y - margin,
            max_x: self.max_x + margin,
            max_y: self.max_y + margin,
        }
    }

    pub fn contains(self, pos: [f32; 2]) -> bool {
        pos[0] >= self.min_x && pos[0] <= self.max_x
            && pos[1] >= self.min_y && pos[1] <= self.max_y
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn overlaps_chunk(self, coord: ChunkCoord) -> bool {
        let o = coord.world_origin();
        let cw = MAP_W as f32;
        let ch = MAP_H as f32;
        self.min_x < o[0] + cw && self.max_x > o[0]
            && self.min_y < o[1] + ch && self.max_y > o[1]
    }
}

// ---------------------------------------------------------------------------
// Loaded chunk
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct LoadedChunk {
    pub zone_id: ZoneId,
    pub coord: ChunkCoord,
    pub enemy_start: usize,
    pub enemy_count: usize,
}

// ---------------------------------------------------------------------------
// Simulation radius (scaled for 100×100 chunks)
// ---------------------------------------------------------------------------

pub const SIM_RADIUS: f32 = 14.0;
pub const DORMANT_RADIUS: f32 = 20.0;

pub fn in_sim_radius(player_world: [f32; 2], entity_world: [f32; 2]) -> bool {
    let dx = entity_world[0] - player_world[0];
    let dy = entity_world[1] - player_world[1];
    dx * dx + dy * dy < SIM_RADIUS * SIM_RADIUS
}

pub fn in_dormant_radius(player_world: [f32; 2], entity_world: [f32; 2]) -> bool {
    let dx = entity_world[0] - player_world[0];
    let dy = entity_world[1] - player_world[1];
    dx * dx + dy * dy < DORMANT_RADIUS * DORMANT_RADIUS
}

// ---------------------------------------------------------------------------
// World-space enemy wrapper
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct WorldEnemy {
    pub enemy: Enemy,
    pub chunk: ChunkCoord,
}

impl WorldEnemy {
    pub fn world_pos(&self) -> [f32; 2] {
        self.chunk.to_world(self.enemy.pos)
    }

    pub fn world_spawn(&self) -> [f32; 2] {
        self.chunk.to_world(self.enemy.spawn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_coord_conversions() {
        let c = ChunkCoord::new(1, 0);
        let origin = c.world_origin();
        assert!((origin[0] - MAP_W as f32).abs() < 0.001);
        assert!(origin[1].abs() < 0.001);

        let world = [MAP_W as f32 + 5.0, 3.0];
        let local = c.to_local(world);
        assert!((local[0] - 5.0).abs() < 0.001);
        assert!((local[1] - 3.0).abs() < 0.001);

        let back = c.to_world(local);
        assert!((back[0] - world[0]).abs() < 0.001);
        assert!((back[1] - world[1]).abs() < 0.001);
    }

    #[test]
    fn chunk_contains() {
        let camp = ChunkCoord::new(0, 0);
        assert!(camp.contains([50.0, 50.0]));
        assert!(!camp.contains([MAP_W as f32 + 1.0, 50.0]));
        assert!(!camp.contains([-1.0, 50.0]));
    }

    #[test]
    fn zone_at_world_lookup() {
        // Camp at (0,0)
        assert_eq!(zone_at_world([50.0, 50.0]), Some(ZoneId::RogueCamp));
        // BloodMoorE at (1,0)
        assert_eq!(zone_at_world([MAP_W as f32 + 5.0, 5.0]), Some(ZoneId::BloodMoorE));
        // BloodMoorN at (0,-1)
        assert_eq!(zone_at_world([5.0, -(MAP_H as f32) + 5.0]), Some(ZoneId::BloodMoorN));
        // DenOfEvil at (-1,-1)
        assert_eq!(zone_at_world([-(MAP_W as f32) + 5.0, -(MAP_H as f32) + 5.0]), Some(ZoneId::DenOfEvil));
        // Void
        assert_eq!(zone_at_world([500.0, 500.0]), None);
    }

    #[test]
    fn world_rect_overlaps_chunk() {
        let rect = WorldRect::around([95.0, 50.0], 10.0, 10.0);
        let camp = ChunkCoord::new(0, 0);
        let blood_e = ChunkCoord::new(1, 0);
        assert!(rect.overlaps_chunk(camp));
        assert!(rect.overlaps_chunk(blood_e)); // crosses boundary
        let far = ChunkCoord::new(2, 0);
        assert!(!rect.overlaps_chunk(far));
    }

    #[test]
    fn sim_radius_check() {
        let player = [50.0, 50.0];
        assert!(in_sim_radius(player, [60.0, 50.0]));   // 10 tiles
        assert!(!in_sim_radius(player, [100.0, 50.0])); // 50 tiles
    }

    #[test]
    fn all_zones_placed() {
        assert_eq!(ZONE_PLACEMENTS.len(), ZONE_COUNT);
        for &z in &ZoneId::ALL {
            assert!(ZONE_PLACEMENTS.iter().any(|p| p.zone_id == z),
                "Zone {:?} missing from ZONE_PLACEMENTS", z);
        }
    }
}
