//! Allumina — an isometric ARPG prototype built on the Miyukini Game Engine.
//!
//! Architecture: a small deterministic ECS ([`ecs::World`]) driven by an
//! [`app::App`] scheduler. [`plugins`] register [`systems`] against ordered
//! [`app::PhaseId`] phases. [`components`] are pure data; the heavy Diablo-like
//! domain logic (stats, combat math, items, affixes, loot) is reused from the
//! `mge-gameplay` and `mge-items` crates.

pub mod app;
pub mod components;
pub mod data;
pub mod ecs;
#[cfg(feature = "netcode")]
pub mod network;
pub mod plugins;
pub mod render;
pub mod rng;
pub mod save;
pub mod systems;
pub mod ui;

pub use app::{App, GameState, PhaseId, Plugin};
pub use ecs::{Entity, World};

use components::player::ClassType;
use components::world::ZoneId;

/// Build a fully-wired simulation [`App`] starting in `zone`, and run its
/// startup systems (spawns the player + zone contents). Shared by the
/// graphical client, the headless driver, and the authoritative server.
pub fn build_app_in(class: ClassType, seed: u64, zone: ZoneId) -> App {
    use crate::data::maps::{map_for, map_for_run};
    use crate::data::GameData;
    use crate::plugins::camera::CameraPlugin;
    use crate::plugins::cc::CcPlugin;
    use crate::plugins::combat::CombatPlugin;
    use crate::plugins::content::ContentPlugin;
    use crate::plugins::inventory::InventoryPlugin;
    use crate::plugins::isometric::IsometricPlugin;
    use crate::plugins::loot::LootPlugin;
    use crate::plugins::mercenary::MercenaryPlugin;
    use crate::plugins::mob_ai::MobAiPlugin;
    use crate::plugins::network::NetworkPlugin;
    use crate::plugins::player::PlayerPlugin;
    use crate::plugins::skills::SkillsPlugin;
    use crate::plugins::summon::SummonPlugin;
    use crate::plugins::tilemap::TilemapPlugin;

    // GreenPlains keeps its hand-crafted spawn layout for the boot path so the
    // headless driver / integration tests have a predictable population. Other
    // hunting zones are populated procedurally; boss zones and the village stay
    // hand-crafted by `map_for`.
    let initial_map = if matches!(zone, ZoneId::Village | ZoneId::GreenPlains)
        || crate::data::maps::is_boss_zone(zone)
    {
        map_for(zone)
    } else {
        map_for_run(zone, 1, 0, seed)
    };
    let mut world = World::new(initial_map, GameData::load(), seed);
    world.chosen_class = class;

    let mut app = App::new(world);
    app.add_plugin(IsometricPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MobAiPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(CcPlugin)
        .add_plugin(LootPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(SkillsPlugin)
        .add_plugin(ContentPlugin)
        // Allies are added after combat/content so their lifecycle systems run
        // last within their shared phases.
        .add_plugin(SummonPlugin)
        .add_plugin(MercenaryPlugin)
        .add_plugin(NetworkPlugin);
    app.run_startup();
    app
}

/// Build the default app: the player starts in the Village hub.
pub fn build_app(class: ClassType, seed: u64) -> App {
    build_app_in(class, seed, ZoneId::Village)
}
