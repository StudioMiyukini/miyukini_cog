//! Twenty-nine linear zones (1 hub + 20 hunting grounds + 8 boss arenas)
//! covering levels 1–100.
//!
//! Normal hunting maps share a procedurally-generated 60×60 layout whose mob
//! pool is fixed by the *zone's* level range (the player's level only
//! influences the seed, never the difficulty). Boss arenas keep a hand-crafted
//! 40×40 layout with a single named boss and a handful of adds. The Village
//! is the 40×40 safe hub: no hostiles, all merchants, and a portal to the
//! first hunting zone.

use crate::components::world::{
    Chest, ChestTier, Decor, DecorKind, MapData, MobPack, NpcRole, NpcSpawn, ObstacleKind, Portal,
    SpawnPoint, TileType, Waypoint, ZoneId,
};
use crate::rng::Rng;

/// Stable waypoint ids. Reused across save/network so they cannot be renumbered
/// without a save migration. Boss zones deliberately have no waypoint, so the
/// id list mirrors the normal zones + the village hub.
pub const WP_VILLAGE: u32 = 1;
pub const WP_GREEN_PLAINS: u32 = 2;
pub const WP_ROCKY_HILLS: u32 = 3;
pub const WP_DARK_FOREST: u32 = 4;
pub const WP_PUTRID_SWAMP: u32 = 5;
pub const WP_ANCIENT_RUINS: u32 = 6;
pub const WP_FORGOTTEN_CRYPT: u32 = 7;
pub const WP_BURNING_DESERT: u32 = 8;
pub const WP_WIND_CANYON: u32 = 9;
pub const WP_FALLEN_FORTRESS: u32 = 10;
pub const WP_FROZEN_PEAKS: u32 = 11;
pub const WP_CRYSTAL_CAVES: u32 = 12;
pub const WP_VOLCANIC_ABYSS: u32 = 13;
pub const WP_CELESTIAL_TOWER: u32 = 14;
pub const WP_CORRUPTED_SANCTUARY: u32 = 15;
pub const WP_ASH_PLAINS: u32 = 16;
pub const WP_SHADOW_CITADEL: u32 = 17;
pub const WP_DESOLATE_LAND: u32 = 18;
pub const WP_DIMENSIONAL_NEXUS: u32 = 19;
pub const WP_TIME_FAULT: u32 = 20;
pub const WP_CHAOS_HEART: u32 = 21;

/// Legacy alias — the first Act I hunting ground used to be called "Plains".
pub const WP_PLAINS: u32 = WP_GREEN_PLAINS;

/// Build a `width`x`height` floor with a solid wall border.
fn bordered(width: i32, height: i32) -> Vec<TileType> {
    let mut tiles = vec![TileType::Floor; (width * height) as usize];
    let idx = |x: i32, y: i32| (y * width + x) as usize;
    for x in 0..width {
        tiles[idx(x, 0)] = TileType::Wall;
        tiles[idx(x, height - 1)] = TileType::Wall;
    }
    for y in 0..height {
        tiles[idx(0, y)] = TileType::Wall;
        tiles[idx(width - 1, y)] = TileType::Wall;
    }
    tiles
}

#[allow(clippy::cast_sign_loss)]
fn set(tiles: &mut [TileType], width: i32, x: i32, y: i32, t: TileType) {
    tiles[(y * width + x) as usize] = t;
}

/// Convenience constructor for an [`NpcSpawn`].
fn npc(kind: NpcRole, x: i32, y: i32, name: &str, dialogue: &str) -> NpcSpawn {
    NpcSpawn { kind, x, y, name: name.to_string(), dialogue: dialogue.to_string() }
}

/// Convenience constructor for a [`Decor`].
fn dec(kind: DecorKind, x: i32, y: i32) -> Decor {
    Decor { kind, x, y }
}

// ── Zone metadata ─────────────────────────────────────────────────────────

/// `(min, max)` mob level for `zone`. The Village returns `(0, 0)`.
#[must_use]
pub fn zone_level_range(zone: ZoneId) -> (u32, u32) {
    match zone {
        ZoneId::Village => (0, 0),
        ZoneId::GreenPlains => (1, 5),
        ZoneId::RockyHills => (6, 10),
        ZoneId::DarkForest | ZoneId::WerewolfDen => (11, 15),
        ZoneId::PutridSwamp => (16, 20),
        ZoneId::AncientRuins => (21, 25),
        ZoneId::ForgottenCrypt | ZoneId::LichHall => (26, 30),
        ZoneId::BurningDesert => (31, 35),
        ZoneId::WindCanyon => (36, 40),
        ZoneId::FallenFortress | ZoneId::TyrantThrone => (41, 45),
        ZoneId::FrozenPeaks => (46, 50),
        ZoneId::CrystalCaves => (51, 55),
        ZoneId::VolcanicAbyss | ZoneId::InfernalForge => (56, 60),
        ZoneId::CelestialTower => (61, 65),
        ZoneId::CorruptedSanctuary | ZoneId::FallenArchangelThrone => (66, 70),
        ZoneId::AshPlains => (71, 75),
        ZoneId::ShadowCitadel | ZoneId::SoulAbyss => (76, 80),
        ZoneId::DesolateLand => (81, 85),
        ZoneId::DimensionalNexus | ZoneId::VoidPalace => (86, 90),
        ZoneId::TimeFault => (91, 95),
        ZoneId::ChaosHeart | ZoneId::EternalThrone => (96, 100),
    }
}

/// True for zones that fight a named boss and therefore stay hand-crafted
/// (fixed layout, fixed spawns) regardless of the procedural seed.
#[must_use]
pub fn is_boss_zone(zone: ZoneId) -> bool {
    matches!(
        zone,
        ZoneId::WerewolfDen
            | ZoneId::LichHall
            | ZoneId::TyrantThrone
            | ZoneId::InfernalForge
            | ZoneId::FallenArchangelThrone
            | ZoneId::SoulAbyss
            | ZoneId::VoidPalace
            | ZoneId::EternalThrone
    )
}

/// Linear progression to the next zone in the chain. `None` after the final
/// boss.
#[must_use]
pub fn next_zone(zone: ZoneId) -> Option<ZoneId> {
    Some(match zone {
        ZoneId::Village => ZoneId::GreenPlains,
        ZoneId::GreenPlains => ZoneId::RockyHills,
        ZoneId::RockyHills => ZoneId::DarkForest,
        ZoneId::DarkForest => ZoneId::WerewolfDen,
        ZoneId::WerewolfDen => ZoneId::PutridSwamp,
        ZoneId::PutridSwamp => ZoneId::AncientRuins,
        ZoneId::AncientRuins => ZoneId::ForgottenCrypt,
        ZoneId::ForgottenCrypt => ZoneId::LichHall,
        ZoneId::LichHall => ZoneId::BurningDesert,
        ZoneId::BurningDesert => ZoneId::WindCanyon,
        ZoneId::WindCanyon => ZoneId::FallenFortress,
        ZoneId::FallenFortress => ZoneId::TyrantThrone,
        ZoneId::TyrantThrone => ZoneId::FrozenPeaks,
        ZoneId::FrozenPeaks => ZoneId::CrystalCaves,
        ZoneId::CrystalCaves => ZoneId::VolcanicAbyss,
        ZoneId::VolcanicAbyss => ZoneId::InfernalForge,
        ZoneId::InfernalForge => ZoneId::CelestialTower,
        ZoneId::CelestialTower => ZoneId::CorruptedSanctuary,
        ZoneId::CorruptedSanctuary => ZoneId::FallenArchangelThrone,
        ZoneId::FallenArchangelThrone => ZoneId::AshPlains,
        ZoneId::AshPlains => ZoneId::ShadowCitadel,
        ZoneId::ShadowCitadel => ZoneId::SoulAbyss,
        ZoneId::SoulAbyss => ZoneId::DesolateLand,
        ZoneId::DesolateLand => ZoneId::DimensionalNexus,
        ZoneId::DimensionalNexus => ZoneId::VoidPalace,
        ZoneId::VoidPalace => ZoneId::TimeFault,
        ZoneId::TimeFault => ZoneId::ChaosHeart,
        ZoneId::ChaosHeart => ZoneId::EternalThrone,
        ZoneId::EternalThrone => return None,
    })
}

/// French zone name (alias of [`ZoneId::name`]).
#[must_use]
pub fn zone_name(zone: ZoneId) -> &'static str {
    zone.name()
}

/// Tilemap footprint: `(width, height)` in tiles. Boss arenas are 40×40,
/// hunting grounds are 60×60, the Village is 40×40.
#[must_use]
pub fn zone_tilemap_size(zone: ZoneId) -> (u32, u32) {
    if matches!(zone, ZoneId::Village) {
        (40, 40)
    } else if is_boss_zone(zone) {
        (40, 40)
    } else {
        (60, 60)
    }
}

// ── Village ───────────────────────────────────────────────────────────────

/// Safe hub at 40x40: central plaza with fountain + waypoint, four trade
/// quarters (market north, barracks east, temple west, inn south), eight
/// named NPCs and a portal to the first hunting zone in the south-east corner.
pub fn village_map() -> MapData {
    const W: i32 = 40;
    const H: i32 = 40;
    let mut tiles = bordered(W, H);

    let player_spawn = (4, 20);
    set(&mut tiles, W, player_spawn.0, player_spawn.1, TileType::PlayerSpawn);
    // Portal to the first hunting zone in the south-east.
    let plains_portal = (38, 35);
    set(&mut tiles, W, plains_portal.0, plains_portal.1, TileType::Exit);

    let npc_spawns = vec![
        npc(NpcRole::PotionMerchant, 15, 8, "Lyra l'Apothicaire",
            "Potions de vie 5 or, potions de mana 8 or."),
        npc(NpcRole::WeaponMerchant, 20, 8, "Elena la Marchande",
            "Lames, arcs, batons, sceptres — du niveau de la zone."),
        npc(NpcRole::ArmorMerchant, 25, 8, "Marcus l'Armurier",
            "Plastrons, casques, bottes, gants, boucliers."),
        npc(NpcRole::Healer, 8, 20, "Pere Aldric",
            "Approchez, je restaurerai vos forces."),
        npc(NpcRole::Blacksmith, 28, 20, "Gareth le Forgeron",
            "Je peux renforcer vos armes... bientot."),
        npc(NpcRole::Recruiter, 30, 20, "Hans le Capitaine",
            "Vous cherchez de l'aide ? Mes hommes sont a louer."),
        npc(NpcRole::QuestGiver, 20, 32, "Dame Isolde",
            "Vingt terres a purger, voyageur. L'Archange dechu vous attend au bout."),
    ];

    let merchant_spawn = Some((20, 8));

    let mut decors = Vec::new();
    decors.push(dec(DecorKind::Fountain, 20, 20));
    for (x, y) in [(13, 13), (27, 13), (13, 27), (27, 27)] {
        decors.push(dec(DecorKind::Torch, x, y));
    }
    for (x, y) in [(18, 19), (22, 19), (18, 21), (22, 21)] {
        decors.push(dec(DecorKind::Bench, x, y));
    }
    for (x, y) in [(13, 6), (18, 6), (23, 6), (27, 6)] {
        decors.push(dec(DecorKind::MarketStall, x, y));
    }
    for (x, y) in [(16, 10), (17, 10), (23, 10), (24, 10)] {
        decors.push(dec(DecorKind::Barrel, x, y));
    }
    for (x, y) in [(12, 8), (28, 8), (12, 11), (28, 11)] {
        decors.push(dec(DecorKind::Crate, x, y));
    }
    for (x, y) in [(32, 18), (32, 22)] {
        decors.push(dec(DecorKind::Dummy, x, y));
    }
    for (x, y) in [(34, 19), (34, 21)] {
        decors.push(dec(DecorKind::WeaponRack, x, y));
    }
    decors.push(dec(DecorKind::Flag, 35, 17));
    decors.push(dec(DecorKind::Flag, 35, 23));
    decors.push(dec(DecorKind::Altar, 6, 20));
    for (x, y) in [(5, 17), (5, 23), (7, 17), (7, 23), (6, 16), (6, 24)] {
        decors.push(dec(DecorKind::Candle, x, y));
    }
    for (x, y) in [(16, 30), (24, 30), (20, 33)] {
        decors.push(dec(DecorKind::Table, x, y));
    }
    decors.push(dec(DecorKind::Fireplace, 28, 32));
    decors.push(dec(DecorKind::Sign, 20, 27));
    for (x, y) in [
        (3, 3), (5, 3), (10, 3), (30, 3), (35, 3), (37, 3),
        (3, 36), (8, 36), (15, 36), (25, 36), (32, 36), (37, 36),
        (3, 10), (3, 25), (37, 10), (37, 25),
    ] {
        decors.push(dec(DecorKind::Tree, x, y));
    }
    for (x, y) in [
        (4, 5), (36, 5), (4, 34), (36, 34),
        (10, 12), (29, 12), (10, 28), (29, 28),
        (13, 35), (27, 35),
    ] {
        decors.push(dec(DecorKind::Bush, x, y));
    }
    for (x, y) in [
        (15, 15), (25, 15), (15, 25), (25, 25),
        (20, 13), (13, 20), (27, 20), (20, 27),
        (5, 7), (35, 7), (5, 33), (35, 33),
    ] {
        decors.push(dec(DecorKind::Flower, x, y));
    }

    MapData {
        width: W,
        height: H,
        tiles,
        player_spawn,
        mob_spawns: Vec::new(),
        area_level: 1,
        name: ZoneId::Village.name().to_string(),
        zone: ZoneId::Village,
        portals: vec![Portal { at: plains_portal, to: ZoneId::GreenPlains, spawn: (3, 30) }],
        merchant_spawn,
        npc_spawns,
        decors,
        obstacles: Vec::new(),
        chests: Vec::new(),
        packs: Vec::new(),
        seed: 0,
    }
}

// ── Hand-crafted skeleton for hunting grounds ──────────────────────────────

/// Anchor for normal (procedurally-rolled) zones: a 60×60 walled rectangle with
/// fixed west/east portals to the previous / next zone in the chain. The
/// procedural pass fills the interior with mobs, obstacles and chests.
fn hunting_skeleton(zone: ZoneId) -> MapData {
    const W: i32 = 60;
    const H: i32 = 60;
    let mut tiles = bordered(W, H);

    let player_spawn = (3, 30);
    set(&mut tiles, W, player_spawn.0, player_spawn.1, TileType::PlayerSpawn);

    // West portal → previous zone, east portal → next zone.
    set(&mut tiles, W, 1, 30, TileType::Exit);
    set(&mut tiles, W, W - 2, 30, TileType::Exit);

    let (lo, _hi) = zone_level_range(zone);
    let portals = hunting_portals(zone, W);

    MapData {
        width: W,
        height: H,
        tiles,
        player_spawn,
        mob_spawns: Vec::new(),
        area_level: lo,
        name: zone.name().to_string(),
        zone,
        portals,
        merchant_spawn: None,
        npc_spawns: Vec::new(),
        decors: Vec::new(),
        obstacles: Vec::new(),
        chests: Vec::new(),
        packs: Vec::new(),
        seed: 0,
    }
}

/// Portals for a hunting (non-boss, non-village) zone: west tile returns to the
/// previous zone in the chain (Village for `GreenPlains`), east tile leads to
/// the next zone in the chain. Coordinates mirror the spawn tiles used by
/// neighbouring zones so the player lands on a walkable square.
fn hunting_portals(zone: ZoneId, w: i32) -> Vec<Portal> {
    let mut out = Vec::new();
    if let Some(prev) = previous_zone(zone) {
        // West portal sends the player back to the previous zone — they should
        // land on its east-side arrival tile.
        let spawn = arrival_tile_from(zone, prev);
        out.push(Portal { at: (1, 30), to: prev, spawn });
    }
    if let Some(next) = next_zone(zone) {
        // East portal sends the player forward to the next zone — they should
        // land on its west-side arrival tile.
        let spawn = arrival_tile_from(zone, next);
        out.push(Portal { at: (w - 2, 30), to: next, spawn });
    }
    out
}

/// The reverse of [`next_zone`].
#[must_use]
pub fn previous_zone(zone: ZoneId) -> Option<ZoneId> {
    ZoneId::ALL.iter().copied().find(|z| next_zone(*z) == Some(zone))
}

/// Tile the player should land on when entering `dest` from `src`. We arrive
/// on the west side when stepping forward in the chain, and on the east side
/// when stepping backward.
fn arrival_tile_from(src: ZoneId, dest: ZoneId) -> (i32, i32) {
    if matches!(dest, ZoneId::Village) {
        return (37, 35);
    }
    let (w, _h) = zone_tilemap_size(dest);
    let w = w as i32;
    // Forward step: previous → current, arrive on the west spawn.
    if next_zone(src) == Some(dest) {
        return (3, 30);
    }
    // Backward step: next → current, arrive on the east tile.
    if next_zone(dest) == Some(src) {
        return (w - 3, 30);
    }
    // Fallback (e.g. waypoint teleports) — west spawn.
    (3, 30)
}

// ── Boss arenas ────────────────────────────────────────────────────────────

/// Hand-crafted 40×40 boss arena: one boss + a handful of adds. The boss's
/// `mob_def_id` is read from the zone.
fn boss_arena(zone: ZoneId) -> MapData {
    const W: i32 = 40;
    const H: i32 = 40;
    let mut tiles = bordered(W, H);

    // Two pillar rows carving the room.
    for y in 8..=18 {
        if !(12..=14).contains(&y) {
            set(&mut tiles, W, 12, y, TileType::Wall);
        }
    }
    for y in 22..=32 {
        if !(26..=28).contains(&y) {
            set(&mut tiles, W, 26, y, TileType::Wall);
        }
    }

    let player_spawn = (3, 20);
    set(&mut tiles, W, player_spawn.0, player_spawn.1, TileType::PlayerSpawn);

    let (lo, _hi) = zone_level_range(zone);
    let add_id = pack_pool_for_range(lo)[0];
    let boss_id = boss_def_id(zone);

    let mob_spawns = vec![
        SpawnPoint { x: 30, y: 20, mob_def_id: boss_id, elite: false },
        SpawnPoint { x: 8, y: 8, mob_def_id: add_id, elite: false },
        SpawnPoint { x: 8, y: 32, mob_def_id: add_id, elite: false },
        SpawnPoint { x: 20, y: 8, mob_def_id: add_id, elite: false },
        SpawnPoint { x: 20, y: 32, mob_def_id: add_id, elite: false },
    ];
    for sp in &mob_spawns {
        set(&mut tiles, W, sp.x, sp.y, TileType::MobSpawn);
    }

    // West portal → previous zone, east portal → next (if any).
    let mut portals = Vec::new();
    if let Some(prev) = previous_zone(zone) {
        set(&mut tiles, W, 1, 20, TileType::Exit);
        portals.push(Portal { at: (1, 20), to: prev, spawn: arrival_tile_from(zone, prev) });
    }
    if let Some(next) = next_zone(zone) {
        set(&mut tiles, W, W - 2, 20, TileType::Exit);
        portals.push(Portal { at: (W - 2, 20), to: next, spawn: arrival_tile_from(zone, next) });
    }

    MapData {
        width: W,
        height: H,
        tiles,
        player_spawn,
        mob_spawns,
        area_level: lo,
        name: zone.name().to_string(),
        zone,
        portals,
        merchant_spawn: None,
        npc_spawns: Vec::new(),
        decors: Vec::new(),
        obstacles: Vec::new(),
        chests: Vec::new(),
        packs: Vec::new(),
        seed: 0,
    }
}

/// The boss mob def id for a boss zone.
#[must_use]
pub fn boss_def_id(zone: ZoneId) -> u32 {
    match zone {
        ZoneId::WerewolfDen => 100,
        ZoneId::LichHall => 101,
        ZoneId::TyrantThrone => 102,
        ZoneId::InfernalForge => 103,
        ZoneId::FallenArchangelThrone => 104,
        ZoneId::SoulAbyss => 105,
        ZoneId::VoidPalace => 106,
        ZoneId::EternalThrone => 107,
        _ => 0,
    }
}

// ── Public map builders ────────────────────────────────────────────────────

/// Legacy hand-crafted shorthand for the very first hunting zone. The actual
/// game uses [`map_for_run`] which threads the layout through procedural gen.
pub fn green_plains_map() -> MapData {
    let mut m = hunting_skeleton(ZoneId::GreenPlains);
    // Drop a fixed set of spawns so unit tests that call this directly still
    // see a populated map (the production code path goes through
    // [`map_for_run`] and ignores these). Three melee mob ids plus one ranged.
    let pool = pack_pool_for_range(zone_level_range(ZoneId::GreenPlains).0);
    let melee_a = pool[0]; // Loup (melee)
    let ranged = pool[1];  // Bandit (ranged)
    let melee_b = pool[2]; // Slime (melee)
    let spawns = vec![
        SpawnPoint { x: 10, y: 12, mob_def_id: melee_a, elite: false },
        SpawnPoint { x: 12, y: 18, mob_def_id: melee_a, elite: false },
        SpawnPoint { x: 14, y: 25, mob_def_id: ranged, elite: false },
        SpawnPoint { x: 16, y: 40, mob_def_id: melee_a, elite: false },
        SpawnPoint { x: 18, y: 47, mob_def_id: melee_b, elite: false },
        SpawnPoint { x: 27, y: 30, mob_def_id: ranged, elite: false },
        SpawnPoint { x: 30, y: 35, mob_def_id: melee_b, elite: false },
        SpawnPoint { x: 36, y: 12, mob_def_id: melee_a, elite: false },
        SpawnPoint { x: 40, y: 25, mob_def_id: melee_b, elite: false },
        SpawnPoint { x: 45, y: 33, mob_def_id: ranged, elite: false },
        SpawnPoint { x: 48, y: 48, mob_def_id: melee_b, elite: false },
        SpawnPoint { x: 52, y: 22, mob_def_id: melee_b, elite: false },
        SpawnPoint { x: 55, y: 38, mob_def_id: melee_a, elite: false },
    ];
    let w = m.width;
    for sp in &spawns {
        set(&mut m.tiles, w, sp.x, sp.y, TileType::MobSpawn);
    }
    m.mob_spawns = spawns;
    m
}

/// Back-compat alias for the original `plains_map` symbol. The hunting
/// skeleton in `GreenPlains` is the modern Plains.
pub fn plains_map() -> MapData {
    green_plains_map()
}

/// Back-compat alias for the original boss arena symbol — the Werewolf Den is
/// the first boss arena in the new chain.
pub fn dungeon_map() -> MapData {
    boss_arena(ZoneId::WerewolfDen)
}

/// Back-compat shorthand used by integration tests.
pub fn act1_map() -> MapData {
    green_plains_map()
}

/// The map for a given zone (hand-crafted version). Procedural generation runs
/// in [`map_for_run`] which knows the player's level + game seed; this fallback
/// is what tests and the boot path use before the player has progression.
pub fn map_for(zone: ZoneId) -> MapData {
    match zone {
        ZoneId::Village => village_map(),
        ZoneId::GreenPlains => green_plains_map(),
        _ if is_boss_zone(zone) => boss_arena(zone),
        _ => hunting_skeleton(zone),
    }
}

/// Build the live map for `zone`. Boss zones and the village hub keep their
/// hand-crafted layout; every other zone goes through
/// [`generate_zone_procedural`]: the same `(player_level, zone, difficulty,
/// game_seed)` always yields the same map, but changing the player's level
/// reshuffles packs and chests on every visit.
#[must_use]
pub fn map_for_run(zone: ZoneId, player_level: u32, difficulty: u32, game_seed: u64) -> MapData {
    if matches!(zone, ZoneId::Village) || is_boss_zone(zone) {
        return map_for(zone);
    }
    generate_zone_procedural(zone, player_level, difficulty, game_seed)
}

// ── Waypoints ──────────────────────────────────────────────────────────────

/// Every waypoint in the chain. Boss zones deliberately have no waypoint so
/// the player must walk in. The Village waypoint is the default starting
/// beacon and is auto-activated for fresh characters.
pub fn all_waypoints() -> Vec<Waypoint> {
    let mut out = Vec::new();
    out.push(waypoint(WP_VILLAGE, ZoneId::Village, 20.5, 20.5));
    out.push(waypoint(WP_GREEN_PLAINS, ZoneId::GreenPlains, 3.5, 30.5));
    out.push(waypoint(WP_ROCKY_HILLS, ZoneId::RockyHills, 3.5, 30.5));
    out.push(waypoint(WP_DARK_FOREST, ZoneId::DarkForest, 3.5, 30.5));
    out.push(waypoint(WP_PUTRID_SWAMP, ZoneId::PutridSwamp, 3.5, 30.5));
    out.push(waypoint(WP_ANCIENT_RUINS, ZoneId::AncientRuins, 3.5, 30.5));
    out.push(waypoint(WP_FORGOTTEN_CRYPT, ZoneId::ForgottenCrypt, 3.5, 30.5));
    out.push(waypoint(WP_BURNING_DESERT, ZoneId::BurningDesert, 3.5, 30.5));
    out.push(waypoint(WP_WIND_CANYON, ZoneId::WindCanyon, 3.5, 30.5));
    out.push(waypoint(WP_FALLEN_FORTRESS, ZoneId::FallenFortress, 3.5, 30.5));
    out.push(waypoint(WP_FROZEN_PEAKS, ZoneId::FrozenPeaks, 3.5, 30.5));
    out.push(waypoint(WP_CRYSTAL_CAVES, ZoneId::CrystalCaves, 3.5, 30.5));
    out.push(waypoint(WP_VOLCANIC_ABYSS, ZoneId::VolcanicAbyss, 3.5, 30.5));
    out.push(waypoint(WP_CELESTIAL_TOWER, ZoneId::CelestialTower, 3.5, 30.5));
    out.push(waypoint(WP_CORRUPTED_SANCTUARY, ZoneId::CorruptedSanctuary, 3.5, 30.5));
    out.push(waypoint(WP_ASH_PLAINS, ZoneId::AshPlains, 3.5, 30.5));
    out.push(waypoint(WP_SHADOW_CITADEL, ZoneId::ShadowCitadel, 3.5, 30.5));
    out.push(waypoint(WP_DESOLATE_LAND, ZoneId::DesolateLand, 3.5, 30.5));
    out.push(waypoint(WP_DIMENSIONAL_NEXUS, ZoneId::DimensionalNexus, 3.5, 30.5));
    out.push(waypoint(WP_TIME_FAULT, ZoneId::TimeFault, 3.5, 30.5));
    out.push(waypoint(WP_CHAOS_HEART, ZoneId::ChaosHeart, 3.5, 30.5));
    out
}

fn waypoint(id: u32, zone: ZoneId, x: f32, y: f32) -> Waypoint {
    Waypoint { id, name: zone.name().to_string(), zone, x, y, is_boss_zone: false }
}

/// The single waypoint id placed on a zone, or `None` for boss zones.
pub fn waypoint_in(zone: ZoneId) -> Option<u32> {
    all_waypoints().into_iter().find(|w| w.zone == zone).map(|w| w.id)
}

/// Look up a waypoint by id.
pub fn waypoint_by_id(id: u32) -> Option<Waypoint> {
    all_waypoints().into_iter().find(|w| w.id == id)
}

// ── Procedural generation ──────────────────────────────────────────────────

/// Mix the four run inputs into a single 64-bit seed. Designed so that any
/// change to a component reshuffles the layout (player levelling, picking a
/// new difficulty, restarting the run) but the result is stable as long as
/// the inputs hold.
#[must_use]
pub fn zone_seed(player_level: u32, zone: ZoneId, difficulty: u32, game_seed: u64) -> u64 {
    let zone_tag: u64 = 0xA0 + (zone_index(zone) as u64);
    let lvl = u64::from(player_level).wrapping_add(1);
    let diff = u64::from(difficulty).wrapping_add(1);
    let mut s = game_seed
        ^ lvl.wrapping_mul(0x9E37_79B9_7F4A_7C15)
        ^ zone_tag.wrapping_mul(0xC6BC_279E_69EE_3D6B).rotate_left(17)
        ^ diff.wrapping_mul(0xD1B5_4A32_D192_ED03).rotate_left(7);
    // SplitMix finaliser to avalanche the bits.
    s = (s ^ (s >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    s = (s ^ (s >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    s ^ (s >> 31)
}

fn zone_index(zone: ZoneId) -> usize {
    ZoneId::ALL.iter().position(|z| *z == zone).unwrap_or(0)
}

/// Build a hash of `SaveData::player_id` (or any other run-tagging string),
/// suitable for use as the `game_seed` argument to [`map_for_run`].
#[must_use]
pub fn game_seed_from_player_id(player_id: &str) -> u64 {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    for b in player_id.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100_0000_01B3);
    }
    h
}

/// Per-zone-level scaling tier for procedural packs. The pack density goes up
/// with the zone's level so end-game maps feel busier than the starting ones.
struct PackProfile {
    /// `(min, max)` mobs per pack.
    count: (u32, u32),
    /// Probability a grid cell hosts a pack (`0..=100`).
    cell_chance: u32,
    /// Probability that a chosen pack is elite (`0..=100`).
    elite_chance: u32,
    /// Eligible mob def ids at this tier.
    mob_pool: &'static [u32],
}

const POOL_T1: &[u32] = &[1, 2, 3];        // 1-5   Loups, Bandits, Slimes
const POOL_T2: &[u32] = &[4, 5, 6];        // 6-10  Gobelins, Squelettes, Araignées
const POOL_T3: &[u32] = &[7, 8, 9];        // 11-15 Zombies, Golems pierre, Loups-garous
const POOL_T4: &[u32] = &[10, 11, 12];     // 16-20 Grenouilles, Crocodiles, Spectres
const POOL_T5: &[u32] = &[13, 14, 15];     // 21-25 Squelettes guerriers, Momies, Gargoyles
const POOL_T6: &[u32] = &[16, 17, 18];     // 26-30 Nécromanciens, Revenants, Vampires
const POOL_T7: &[u32] = &[19, 20, 21];     // 31-35 Scorpions, Djinns, Vers
const POOL_T8: &[u32] = &[22, 23, 24];     // 36-40 Harpies, Élém. air, Griffons
const POOL_T9: &[u32] = &[25, 26, 27];     // 41-45 Chevaliers noirs, Orcs élite, Démons mineurs
const POOL_T10: &[u32] = &[28, 29, 30];    // 46-50 Yétis, Élém. glace, Wyvernes
const POOL_T11: &[u32] = &[31, 32, 33];    // 51-55 Golems cristal, Basilics, Naga
const POOL_T12: &[u32] = &[34, 35, 36];    // 56-60 Élém. feu, Salamandres, Dragons mineurs
const POOL_T13: &[u32] = &[37, 38, 39];    // 61-65 Anges déchus, Chimères, Démons majeurs
const POOL_T14: &[u32] = &[40, 41, 42];    // 66-70 Archidémons, Séraphins corrompus, Ombres anciennes
const POOL_T15: &[u32] = &[43, 44, 45];    // 71-75 Titan de cendre, Phoenix mort-vivant, Golem de lave
const POOL_T16: &[u32] = &[46, 47, 48];    // 76-80 Spectre ancien, Liche mineure, Ombre vivante
const POOL_T17: &[u32] = &[49, 50, 51];    // 81-85 Behemoth, Dragon adulte, Colosse de pierre
const POOL_T18: &[u32] = &[52, 53, 54];    // 86-90 Seigneur démon, Entité du vide, Avatar élémentaire
const POOL_T19: &[u32] = &[55, 56, 57];    // 91-95 Chronophage, Gardien dimensionnel, Dévoreur d'âmes
const POOL_T20: &[u32] = &[58, 59, 60];    // 96-100 Archon déchu, Titan primordial, Incarnation du chaos

/// Mob pool covering the given zone level (the low end of the range).
#[must_use]
pub fn pack_pool_for_range(level_lo: u32) -> &'static [u32] {
    match level_lo {
        0..=5 => POOL_T1,
        6..=10 => POOL_T2,
        11..=15 => POOL_T3,
        16..=20 => POOL_T4,
        21..=25 => POOL_T5,
        26..=30 => POOL_T6,
        31..=35 => POOL_T7,
        36..=40 => POOL_T8,
        41..=45 => POOL_T9,
        46..=50 => POOL_T10,
        51..=55 => POOL_T11,
        56..=60 => POOL_T12,
        61..=65 => POOL_T13,
        66..=70 => POOL_T14,
        71..=75 => POOL_T15,
        76..=80 => POOL_T16,
        81..=85 => POOL_T17,
        86..=90 => POOL_T18,
        91..=95 => POOL_T19,
        _ => POOL_T20,
    }
}

fn profile_for_zone(zone: ZoneId) -> PackProfile {
    let (lo, _hi) = zone_level_range(zone);
    let pool = pack_pool_for_range(lo);
    // Density tracks the zone's level so the starting zones stay clearable by
    // a fresh character while the end-game ones feel busy.
    let (count, cell_chance, elite_chance) = match lo {
        0..=5 => ((3, 6), 13, 5),
        6..=10 => ((4, 7), 19, 8),
        11..=25 => ((5, 9), 25, 12),
        26..=45 => ((6, 11), 28, 18),
        _ => ((8, 14), 31, 25),
    };
    PackProfile { count, cell_chance, elite_chance, mob_pool: pool }
}

/// Pick a count weighted toward the *high* end (bigger packs are the norm):
/// the larger of two rolls.
fn weighted_pack_count(rng: &mut Rng, min: u32, max: u32) -> u32 {
    let r = rng.range_u32(min, max);
    let r2 = rng.range_u32(min, max);
    r.max(r2)
}

/// `true` if `(x, y)` is a free floor tile (no wall, no portal, no waypoint,
/// no NPC). Used to validate every procedural placement before committing.
fn tile_is_free(map: &MapData, x: i32, y: i32) -> bool {
    if !map.is_walkable(x, y) {
        return false;
    }
    if (x, y) == map.player_spawn {
        return false;
    }
    for p in &map.portals {
        if p.at == (x, y) {
            return false;
        }
    }
    for n in &map.npc_spawns {
        if n.x == x && n.y == y {
            return false;
        }
    }
    for wp in all_waypoints() {
        if wp.zone == map.zone {
            let (wx, wy) = (wp.x.floor() as i32, wp.y.floor() as i32);
            let (dx, dy) = (x - wx, y - wy);
            if dx * dx + dy * dy <= 1 {
                return false;
            }
        }
    }
    true
}

fn footprint_free(map: &MapData, used: &[(i32, i32)], x: i32, y: i32, fw: i32, fh: i32) -> bool {
    for oy in 0..fh {
        for ox in 0..fw {
            let (tx, ty) = (x + ox, y + oy);
            if !tile_is_free(map, tx, ty) {
                return false;
            }
            if used.contains(&(tx, ty)) {
                return false;
            }
        }
    }
    true
}

/// Generate a hunting zone procedurally. Boss zones and the village must never
/// reach here — they're filtered upstream by [`map_for_run`].
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
#[must_use]
pub fn generate_zone_procedural(
    zone: ZoneId,
    player_level: u32,
    difficulty: u32,
    game_seed: u64,
) -> MapData {
    if matches!(zone, ZoneId::Village) || is_boss_zone(zone) {
        return map_for(zone);
    }

    let mut map = hunting_skeleton(zone);
    map.mob_spawns.clear();
    let seed = zone_seed(player_level, zone, difficulty, game_seed);
    map.seed = seed;
    let mut rng = Rng::new(seed);
    let profile = profile_for_zone(zone);

    // 1. Mob packs on an 8x8 grid.
    const CELL: i32 = 8;
    let mut packs: Vec<MobPack> = Vec::new();
    let mut spawns: Vec<SpawnPoint> = Vec::new();
    let mut used: Vec<(i32, i32)> = Vec::new();
    let cells_x = map.width / CELL;
    let cells_y = map.height / CELL;
    for cy in 0..cells_y {
        for cx in 0..cells_x {
            if !rng.chance_pct(profile.cell_chance) {
                continue;
            }
            let pool = profile.mob_pool;
            let mob_def_id = pool[rng.pick_index(pool.len()).unwrap_or(0)];
            let count = weighted_pack_count(&mut rng, profile.count.0, profile.count.1);
            let elite = rng.chance_pct(profile.elite_chance);
            // Spread grows with pack size (plus jitter) so larger packs scatter
            // rather than pile onto one tile.
            let spread = 2.0 + count as f32 * 0.4 + rng.next_f32() * 1.5;
            // Jitter the pack centre anywhere inside the cell (not its exact
            // middle) so placements no longer read as a regular grid.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let center_tx = cx * CELL + rng.range_u32(1, (CELL - 2) as u32) as i32;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let center_ty = cy * CELL + rng.range_u32(1, (CELL - 2) as u32) as i32;
            if !tile_is_free(&map, center_tx, center_ty) {
                continue;
            }
            packs.push(MobPack {
                center_x: center_tx as f32 + 0.5,
                center_y: center_ty as f32 + 0.5,
                mob_def_id,
                count,
                spread,
                elite,
            });
            for _ in 0..count {
                let mut placed = false;
                for _ in 0..16 {
                    let dx = (rng.next_f32() * 2.0 - 1.0) * spread;
                    let dy = (rng.next_f32() * 2.0 - 1.0) * spread;
                    let sx = center_tx + dx.round() as i32;
                    let sy = center_ty + dy.round() as i32;
                    if tile_is_free(&map, sx, sy) && !used.contains(&(sx, sy)) {
                        spawns.push(SpawnPoint { x: sx, y: sy, mob_def_id, elite });
                        used.push((sx, sy));
                        placed = true;
                        break;
                    }
                }
                if !placed {
                    spawns.push(SpawnPoint { x: center_tx, y: center_ty, mob_def_id, elite });
                    used.push((center_tx, center_ty));
                }
            }
        }
    }
    map.mob_spawns = spawns;
    map.packs = packs;

    // 2. Obstacles — 15-30 scattered props, vary the kind weighted-uniformly.
    let target_obstacles = rng.range_u32(15, 30);
    let obstacle_kinds = [
        ObstacleKind::Rock,
        ObstacleKind::Rock,
        ObstacleKind::DeadTree,
        ObstacleKind::DeadTree,
        ObstacleKind::Thornbush,
        ObstacleKind::BoulderCluster,
        ObstacleKind::Pond,
        ObstacleKind::Ruins,
    ];
    let mut obstacles: Vec<(f32, f32, ObstacleKind)> = Vec::new();
    let mut guard = 0;
    while obstacles.len() < target_obstacles as usize && guard < 400 {
        guard += 1;
        let kind = obstacle_kinds[rng.pick_index(obstacle_kinds.len()).unwrap_or(0)];
        let (fw, fh) = kind.footprint();
        let x = rng.range_u32(1, (map.width - fw - 1) as u32) as i32;
        let y = rng.range_u32(1, (map.height - fh - 1) as u32) as i32;
        if !footprint_free(&map, &used, x, y, fw, fh) {
            continue;
        }
        for oy in 0..fh {
            for ox in 0..fw {
                used.push((x + ox, y + oy));
            }
        }
        obstacles.push((x as f32 + (fw as f32) * 0.5, y as f32 + (fh as f32) * 0.5, kind));
    }
    map.obstacles = obstacles;

    // 3. Chests — 3-5 Normal, 1-2 Silver, 0-1 Gold (20% the Gold appears).
    let mut chests: Vec<Chest> = Vec::new();
    let normal_count = rng.range_u32(3, 5);
    let silver_count = rng.range_u32(1, 2);
    let gold_count = if rng.chance_pct(20) { 1 } else { 0 };
    let tier_pool = [
        (ChestTier::Normal, normal_count),
        (ChestTier::Silver, silver_count),
        (ChestTier::Gold, gold_count),
    ];
    for (tier, n) in tier_pool {
        for _ in 0..n {
            for _ in 0..32 {
                let x = rng.range_u32(1, (map.width - 2) as u32) as i32;
                let y = rng.range_u32(1, (map.height - 2) as u32) as i32;
                if footprint_free(&map, &used, x, y, 1, 1) {
                    used.push((x, y));
                    chests.push(Chest { x: x as f32 + 0.5, y: y as f32 + 0.5, tier, opened: false });
                    break;
                }
            }
        }
    }
    map.chests = chests;

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn village_is_safe_and_has_merchant() {
        let v = village_map();
        assert!(v.zone.is_safe());
        assert!(v.mob_spawns.is_empty());
        assert!(v.merchant_spawn.is_some());
        assert!(!v.portals.is_empty());
        assert!(!v.npc_spawns.is_empty(), "the hub should declare NPCs");
        assert!(!v.decors.is_empty(), "the hub should declare decor props");
    }

    #[test]
    fn portals_chain_is_walkable_in_both_directions() {
        for zone in ZoneId::ALL {
            let m = map_for(zone);
            for p in &m.portals {
                let dest = map_for(p.to);
                assert!(
                    dest.is_walkable(p.spawn.0, p.spawn.1),
                    "{:?} portal spawn {:?} not walkable in {:?}",
                    zone,
                    p.spawn,
                    p.to
                );
            }
        }
    }

    #[test]
    fn next_zone_chains_all_twentynine() {
        // Walking forward from the Village touches every zone exactly once and
        // ends at the final boss arena (EternalThrone).
        let mut seen = Vec::new();
        let mut z = ZoneId::Village;
        seen.push(z);
        while let Some(n) = next_zone(z) {
            seen.push(n);
            z = n;
            assert!(seen.len() <= 29, "chain must not loop");
        }
        assert_eq!(seen.len(), 29);
        assert_eq!(*seen.last().unwrap(), ZoneId::EternalThrone);
    }

    #[test]
    fn boss_zones_are_the_expected_eight() {
        let bosses: Vec<ZoneId> = ZoneId::ALL.iter().copied().filter(|z| is_boss_zone(*z)).collect();
        assert_eq!(
            bosses,
            vec![
                ZoneId::WerewolfDen,
                ZoneId::LichHall,
                ZoneId::TyrantThrone,
                ZoneId::InfernalForge,
                ZoneId::FallenArchangelThrone,
                ZoneId::SoulAbyss,
                ZoneId::VoidPalace,
                ZoneId::EternalThrone,
            ],
        );
    }

    #[test]
    fn boss_zones_have_no_waypoint() {
        for zone in ZoneId::ALL {
            if is_boss_zone(zone) {
                assert!(waypoint_in(zone).is_none(), "{:?} should have no waypoint", zone);
            }
        }
        for wp in all_waypoints() {
            assert!(!wp.is_boss_zone);
            assert!(!is_boss_zone(wp.zone));
        }
    }

    #[test]
    fn every_non_boss_zone_has_a_waypoint() {
        for zone in ZoneId::ALL {
            if !is_boss_zone(zone) {
                assert!(waypoint_in(zone).is_some(), "{:?} should have a waypoint", zone);
            }
        }
    }

    #[test]
    fn waypoints_land_on_walkable_tiles() {
        for wp in all_waypoints() {
            let m = map_for(wp.zone);
            let (tx, ty) = (wp.x.floor() as i32, wp.y.floor() as i32);
            assert!(m.is_walkable(tx, ty), "waypoint {} not on walkable tile", wp.name);
        }
    }

    #[test]
    fn boss_arenas_carry_their_named_boss() {
        for zone in [
            ZoneId::WerewolfDen,
            ZoneId::LichHall,
            ZoneId::TyrantThrone,
            ZoneId::InfernalForge,
            ZoneId::FallenArchangelThrone,
            ZoneId::SoulAbyss,
            ZoneId::VoidPalace,
            ZoneId::EternalThrone,
        ] {
            let m = map_for(zone);
            let expected = boss_def_id(zone);
            assert!(
                m.mob_spawns.iter().any(|s| s.mob_def_id == expected),
                "{:?} should spawn boss def {}",
                zone,
                expected,
            );
        }
    }

    #[test]
    fn maps_use_expected_dimensions() {
        assert_eq!((village_map().width, village_map().height), (40, 40));
        assert_eq!(zone_tilemap_size(ZoneId::GreenPlains), (60, 60));
        assert_eq!(zone_tilemap_size(ZoneId::WerewolfDen), (40, 40));
        assert_eq!(zone_tilemap_size(ZoneId::Village), (40, 40));
    }

    #[test]
    fn procedural_layout_is_deterministic() {
        let a = generate_zone_procedural(ZoneId::GreenPlains, 5, 1, 0xABCD);
        let b = generate_zone_procedural(ZoneId::GreenPlains, 5, 1, 0xABCD);
        assert_eq!(a.seed, b.seed);
        assert_eq!(a.mob_spawns.len(), b.mob_spawns.len());
        for (lhs, rhs) in a.mob_spawns.iter().zip(b.mob_spawns.iter()) {
            assert_eq!((lhs.x, lhs.y, lhs.mob_def_id, lhs.elite),
                       (rhs.x, rhs.y, rhs.mob_def_id, rhs.elite));
        }
        assert_eq!(a.chests.len(), b.chests.len());
        assert_eq!(a.obstacles.len(), b.obstacles.len());
        assert_eq!(a.packs.len(), b.packs.len());
    }

    #[test]
    fn mob_pool_follows_the_zone_level_not_the_player() {
        // Even at high player level, an early zone keeps tier-1 mob def ids.
        let m = generate_zone_procedural(ZoneId::GreenPlains, 70, 1, 0xABCD);
        for sp in &m.mob_spawns {
            assert!(POOL_T1.contains(&sp.mob_def_id), "tier-1 zone leaked id {}", sp.mob_def_id);
        }
        // The end-game zone uses tier-14 ids no matter the player's level.
        let m = generate_zone_procedural(ZoneId::CorruptedSanctuary, 1, 1, 0xABCD);
        for sp in &m.mob_spawns {
            assert!(POOL_T14.contains(&sp.mob_def_id), "tier-14 zone leaked id {}", sp.mob_def_id);
        }
    }

    #[test]
    fn boss_zones_skip_procedural_generation() {
        let crafted = boss_arena(ZoneId::WerewolfDen);
        let live = map_for_run(ZoneId::WerewolfDen, 25, 1, 0xDEAD);
        assert_eq!(crafted.mob_spawns.len(), live.mob_spawns.len());
        assert!(live.mob_spawns.iter().any(|s| s.mob_def_id == boss_def_id(ZoneId::WerewolfDen)));
        assert!(live.obstacles.is_empty());
        assert!(live.chests.is_empty());
        assert!(live.packs.is_empty());
        assert_eq!(live.seed, 0);
    }

    #[test]
    fn procedural_zone_has_packs_obstacles_and_chests() {
        let m = generate_zone_procedural(ZoneId::GreenPlains, 10, 1, 0x1234);
        assert!(!m.packs.is_empty(), "should roll at least one pack");
        assert!(!m.mob_spawns.is_empty(), "packs must yield spawn points");
        // Packs are now sizeable: every low-tier pack holds at least 3 mobs, and
        // the zone fields a real horde overall.
        assert!(m.packs.iter().all(|p| p.count >= 3), "low-tier packs hold >= 3 mobs");
        let total: u32 = m.packs.iter().map(|p| p.count).sum();
        assert!(total >= 10, "a procedural zone should field a sizeable horde (got {total})");
        assert!(!m.obstacles.is_empty(), "should place obstacles");
        assert!(!m.chests.is_empty(), "should place at least one chest");
        for sp in &m.mob_spawns {
            assert!(m.is_walkable(sp.x, sp.y), "mob spawn on a non-walkable tile");
        }
        for c in &m.chests {
            let (cx, cy) = (c.x.floor() as i32, c.y.floor() as i32);
            assert!(m.is_walkable(cx, cy), "chest on a non-walkable tile");
        }
    }
}
