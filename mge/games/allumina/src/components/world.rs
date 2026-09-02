//! @id     games_allumina_components_world
//! @do     definir_tile_type
//! @role   ui
//! @layer  core
//! @human  games allumina — module « components world ».

//! Tilemap data components.

use serde::{Deserialize, Serialize};

/// What occupies a tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Floor,
    Wall,
    Void,
    /// Walkable floor flagged as the player start.
    PlayerSpawn,
    /// Walkable floor flagged as a mob spawn.
    MobSpawn,
    /// Walkable exit/transition tile.
    Exit,
}

impl TileType {
    pub fn walkable(self) -> bool {
        !matches!(self, Self::Wall | Self::Void)
    }

    /// ASCII glyph for the debug renderer.
    pub fn glyph(self) -> char {
        match self {
            Self::Floor => '.',
            Self::Wall => '#',
            Self::Void => ' ',
            Self::PlayerSpawn => '@',
            Self::MobSpawn => 'm',
            Self::Exit => '>',
        }
    }
}

/// A single tile (kept as a component-style struct for clarity; the map stores
/// a flat grid of `TileType` for cache efficiency).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub kind: TileType,
}

/// A mob spawn anchor declared by the map.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpawnPoint {
    pub x: i32,
    pub y: i32,
    pub mob_def_id: u32,
    /// Elite spawn: stats +50%, XP +100%, better loot. Defaults to `false` for
    /// hand-crafted maps; procedural generation sets this on pack rolls.
    #[serde(default)]
    pub elite: bool,
}

impl SpawnPoint {
    /// Deterministic per-(zone, position, def) identifier used by the
    /// killed-mob bookkeeping so a slain mob stays dead across zone re-entries.
    /// Map authoring keeps positions unique, so (x, y, mob_def_id) is sufficient.
    #[must_use]
    pub fn stable_id(&self) -> u64 {
        #[allow(clippy::cast_sign_loss)]
        let xb = self.x as i64 as u64;
        #[allow(clippy::cast_sign_loss)]
        let yb = self.y as i64 as u64;
        xb.wrapping_mul(0x9E37_79B9_7F4A_7C15)
            ^ yb.wrapping_mul(0xC6BC_279E_69EE_3D6B).rotate_left(17)
            ^ u64::from(self.mob_def_id)
            ^ if self.elite { 0xE1175E5E_E1175E5E } else { 0 }
    }
}

/// A procedurally-rolled pack of identical mobs centred on `(center_x, center_y)`
/// and scattered within `spread` tiles. Packs are flattened into [`SpawnPoint`]s
/// when the [`MapData`] is built so the existing spawn machinery can place them
/// without any extra plumbing; the struct is kept on `MapData` for debugging
/// and tests.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MobPack {
    pub center_x: f32,
    pub center_y: f32,
    pub mob_def_id: u32,
    pub count: u32,
    pub spread: f32,
    pub elite: bool,
}

/// A non-walkable scenery prop placed by procedural generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObstacleKind {
    /// 1x1 grey rock.
    Rock,
    /// 2x2 cluster of rocks.
    BoulderCluster,
    /// 1x1 dead tree (brown/black).
    DeadTree,
    /// 2x2 small water pond.
    Pond,
    /// 2x1 broken ruins.
    Ruins,
    /// 1x1 thorn bush (deep green).
    Thornbush,
}

impl ObstacleKind {
    /// Footprint of the obstacle in tiles, used by the layout system to avoid
    /// overlap with portals/waypoints/walls.
    #[must_use]
    pub fn footprint(self) -> (i32, i32) {
        match self {
            Self::Rock | Self::DeadTree | Self::Thornbush => (1, 1),
            Self::BoulderCluster | Self::Pond => (2, 2),
            Self::Ruins => (2, 1),
        }
    }
}

/// Quality tier of a procedurally-placed chest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChestTier {
    /// 1-2 items, baseline rarities.
    Normal,
    /// 2-3 items, +20% Magic chance.
    Silver,
    /// 3-5 items, +30% Rare chance, may roll a Unique.
    Gold,
}

impl ChestTier {
    /// `(min_items, max_items)` produced when the chest is opened.
    #[must_use]
    pub fn item_count(self) -> (u32, u32) {
        match self {
            Self::Normal => (1, 2),
            Self::Silver => (2, 3),
            Self::Gold => (3, 5),
        }
    }
}

/// A procedurally-placed reward container.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Chest {
    pub x: f32,
    pub y: f32,
    pub tier: ChestTier,
    pub opened: bool,
}

/// Twenty-nine linear zones: a safe hub (Village), twenty procedurally
/// generated hunting grounds, and eight hand-crafted boss arenas (including
/// the final EternalThrone). The player walks the chain forward via portals;
/// waypoints let them teleport back to any normal zone they have already
/// visited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneId {
    Village,
    GreenPlains,
    RockyHills,
    DarkForest,
    WerewolfDen,
    PutridSwamp,
    AncientRuins,
    ForgottenCrypt,
    LichHall,
    BurningDesert,
    WindCanyon,
    FallenFortress,
    TyrantThrone,
    FrozenPeaks,
    CrystalCaves,
    VolcanicAbyss,
    InfernalForge,
    CelestialTower,
    CorruptedSanctuary,
    FallenArchangelThrone,
    AshPlains,
    ShadowCitadel,
    SoulAbyss,
    DesolateLand,
    DimensionalNexus,
    VoidPalace,
    TimeFault,
    ChaosHeart,
    EternalThrone,
}

impl ZoneId {
    pub const ALL: [Self; 29] = [
        Self::Village,
        Self::GreenPlains,
        Self::RockyHills,
        Self::DarkForest,
        Self::WerewolfDen,
        Self::PutridSwamp,
        Self::AncientRuins,
        Self::ForgottenCrypt,
        Self::LichHall,
        Self::BurningDesert,
        Self::WindCanyon,
        Self::FallenFortress,
        Self::TyrantThrone,
        Self::FrozenPeaks,
        Self::CrystalCaves,
        Self::VolcanicAbyss,
        Self::InfernalForge,
        Self::CelestialTower,
        Self::CorruptedSanctuary,
        Self::FallenArchangelThrone,
        Self::AshPlains,
        Self::ShadowCitadel,
        Self::SoulAbyss,
        Self::DesolateLand,
        Self::DimensionalNexus,
        Self::VoidPalace,
        Self::TimeFault,
        Self::ChaosHeart,
        Self::EternalThrone,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Village => "Village",
            Self::GreenPlains => "Plaines verdoyantes",
            Self::RockyHills => "Collines rocheuses",
            Self::DarkForest => "Forêt sombre",
            Self::WerewolfDen => "Antre du Loup-Garou",
            Self::PutridSwamp => "Marais putrides",
            Self::AncientRuins => "Ruines anciennes",
            Self::ForgottenCrypt => "Crypte oubliée",
            Self::LichHall => "Salle du Liche",
            Self::BurningDesert => "Désert brûlant",
            Self::WindCanyon => "Canyon des vents",
            Self::FallenFortress => "Forteresse déchue",
            Self::TyrantThrone => "Trône du Tyran",
            Self::FrozenPeaks => "Pics gelés",
            Self::CrystalCaves => "Cavernes de cristal",
            Self::VolcanicAbyss => "Abîme volcanique",
            Self::InfernalForge => "Forge infernale",
            Self::CelestialTower => "Tour céleste",
            Self::CorruptedSanctuary => "Sanctuaire corrompu",
            Self::FallenArchangelThrone => "Trône de l'Archange déchu",
            Self::AshPlains => "Plaines de cendres",
            Self::ShadowCitadel => "Citadelle des ombres",
            Self::SoulAbyss => "Abîme des âmes",
            Self::DesolateLand => "Terre désolée",
            Self::DimensionalNexus => "Nexus dimensionnel",
            Self::VoidPalace => "Palais du Néant",
            Self::TimeFault => "Faille temporelle",
            Self::ChaosHeart => "Cœur du Chaos",
            Self::EternalThrone => "Trône de l'Éternel",
        }
    }

    pub fn is_safe(self) -> bool {
        matches!(self, Self::Village)
    }

    /// Stable short token used as a map key in saves (killed-mob tracking,
    /// `current_zone`, …). Mirrored by [`Self::from_key`].
    #[must_use]
    pub fn key(self) -> &'static str {
        match self {
            Self::Village => "village",
            Self::GreenPlains => "green_plains",
            Self::RockyHills => "rocky_hills",
            Self::DarkForest => "dark_forest",
            Self::WerewolfDen => "werewolf_den",
            Self::PutridSwamp => "putrid_swamp",
            Self::AncientRuins => "ancient_ruins",
            Self::ForgottenCrypt => "forgotten_crypt",
            Self::LichHall => "lich_hall",
            Self::BurningDesert => "burning_desert",
            Self::WindCanyon => "wind_canyon",
            Self::FallenFortress => "fallen_fortress",
            Self::TyrantThrone => "tyrant_throne",
            Self::FrozenPeaks => "frozen_peaks",
            Self::CrystalCaves => "crystal_caves",
            Self::VolcanicAbyss => "volcanic_abyss",
            Self::InfernalForge => "infernal_forge",
            Self::CelestialTower => "celestial_tower",
            Self::CorruptedSanctuary => "corrupted_sanctuary",
            Self::FallenArchangelThrone => "fallen_archangel_throne",
            Self::AshPlains => "ash_plains",
            Self::ShadowCitadel => "shadow_citadel",
            Self::SoulAbyss => "soul_abyss",
            Self::DesolateLand => "desolate_land",
            Self::DimensionalNexus => "dimensional_nexus",
            Self::VoidPalace => "void_palace",
            Self::TimeFault => "time_fault",
            Self::ChaosHeart => "chaos_heart",
            Self::EternalThrone => "eternal_throne",
        }
    }

    #[must_use]
    pub fn from_key(s: &str) -> Self {
        for z in Self::ALL {
            if z.key() == s {
                return z;
            }
        }
        Self::Village
    }
}

/// A walkable transition tile linking one zone to another.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Portal {
    /// Tile that triggers the transition.
    pub at: (i32, i32),
    /// Destination zone.
    pub to: ZoneId,
    /// Tile the player arrives on in the destination.
    pub spawn: (i32, i32),
}

/// Purely-cosmetic prop placed by map authoring. Rendered by the tilemap
/// renderer between the ground and the entities; they have no collision or
/// interaction and exist only to make zones feel inhabited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DecorKind {
    Fountain,
    Torch,
    Bench,
    MarketStall,
    Barrel,
    Crate,
    Dummy,
    WeaponRack,
    Flag,
    Altar,
    Candle,
    Table,
    Fireplace,
    Sign,
    Tree,
    Bush,
    Flower,
}

/// A single decor prop at a (sub-)tile position.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Decor {
    pub kind: DecorKind,
    /// Tile-space coordinates; the renderer adds a 0.5 centre offset.
    pub x: i32,
    pub y: i32,
}

/// A village NPC declared by the map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcSpawn {
    pub kind: NpcRole,
    pub name: String,
    pub dialogue: String,
    pub x: i32,
    pub y: i32,
}

/// What an NPC actually does in the village. (Behavioural — the NPC entity
/// also receives a [`crate::components::npc::NpcKind`] mirror so legacy code
/// keeps working.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcRole {
    WeaponMerchant,
    ArmorMerchant,
    PotionMerchant,
    Blacksmith,
    Healer,
    QuestGiver,
    Recruiter,
}

/// A fixed fast-travel beacon on a zone map. Players activate one by stepping
/// onto it; once activated it can be selected from the waypoint menu to
/// instantly travel between any pair of activated beacons.
///
/// `is_boss_zone` is kept on the struct for completeness but our authoring
/// rule is that boss zones have *no* waypoint, so the live list only ever
/// contains non-boss beacons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Waypoint {
    pub id: u32,
    pub name: String,
    pub zone: ZoneId,
    pub x: f32,
    pub y: f32,
    pub is_boss_zone: bool,
}

/// Per-player list of activated waypoint ids, serialised with the save.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DiscoveredWaypoints {
    pub activated: Vec<u32>,
}

impl DiscoveredWaypoints {
    pub fn new() -> Self {
        Self { activated: Vec::new() }
    }

    pub fn contains(&self, id: u32) -> bool {
        self.activated.contains(&id)
    }

    /// Add `id` if absent. Returns `true` when newly discovered.
    pub fn activate(&mut self, id: u32) -> bool {
        if self.activated.contains(&id) {
            false
        } else {
            self.activated.push(id);
            true
        }
    }
}

/// In-progress waypoint teleport (1s cast, interruptible by damage).
#[derive(Debug, Clone, Copy)]
pub struct WaypointTeleportCast {
    pub target_wp: u32,
    pub remaining: f32,
}

impl WaypointTeleportCast {
    pub const CAST_TIME: f32 = 1.0;

    pub fn new(target_wp: u32) -> Self {
        Self { target_wp, remaining: Self::CAST_TIME }
    }
}

/// A loaded tilemap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<TileType>,
    pub player_spawn: (i32, i32),
    pub mob_spawns: Vec<SpawnPoint>,
    /// Area level — drives loot ilvl and mob scaling.
    pub area_level: u32,
    pub name: String,
    /// Which zone this map represents.
    pub zone: ZoneId,
    /// Transition tiles to other zones.
    pub portals: Vec<Portal>,
    /// Merchant NPC anchor, if this zone has one. Kept for back-compat: the
    /// village now places its NPCs via [`Self::npc_spawns`], and this field
    /// still points at the primary weapon-merchant tile so existing tests and
    /// the legacy "merchant_spawn" save format keep working.
    pub merchant_spawn: Option<(i32, i32)>,
    /// All non-combat NPCs declared by the map (village hub).
    #[serde(default)]
    pub npc_spawns: Vec<NpcSpawn>,
    /// Purely-visual props rendered with the tiles (fountains, torches, etc.).
    #[serde(default)]
    pub decors: Vec<Decor>,
    /// Non-walkable scenery: rocks, dead trees, ponds, etc. Empty for
    /// hand-crafted maps; filled by the procedural generator for normal zones.
    #[serde(default)]
    pub obstacles: Vec<(f32, f32, ObstacleKind)>,
    /// Lootable containers placed by the generator.
    #[serde(default)]
    pub chests: Vec<Chest>,
    /// Source packs for the procedural mob spawns (purely informational, the
    /// already-flattened spawns live in [`Self::mob_spawns`]).
    #[serde(default)]
    pub packs: Vec<MobPack>,
    /// Seed used to build this map. Equals zero for hand-crafted maps. Two
    /// `MapData`s built from the same seed must be byte-identical.
    #[serde(default)]
    pub seed: u64,
}

impl MapData {
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    pub fn get(&self, x: i32, y: i32) -> TileType {
        if self.in_bounds(x, y) {
            #[allow(clippy::cast_sign_loss)]
            self.tiles[(y * self.width + x) as usize]
        } else {
            TileType::Void
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.get(x, y).walkable()
    }
}
