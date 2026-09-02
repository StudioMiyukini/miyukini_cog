//! @id     games_allumina_plugins_tilemap
//! @do     tilemap_load_system
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « plugins tilemap ».

//! Tilemap loading and an ASCII debug renderer (stands in for tile sprites).

use crate::app::{App, Plugin};
use crate::components::mob::AiBehavior;
use crate::ecs::World;

/// Log a one-line summary of the loaded map at startup.
pub fn tilemap_load_system(world: &mut World) {
    let m = &world.map;
    world.log_event(format!(
        "Map '{}' loaded: {}x{}, {} mob spawns.",
        m.name,
        m.width,
        m.height,
        m.mob_spawns.len()
    ));
}

/// Render the map as ASCII with live entities overlaid.
#[allow(clippy::cast_sign_loss)]
pub fn render_ascii(world: &World) -> String {
    let map = &world.map;
    let mut grid: Vec<Vec<char>> = (0..map.height)
        .map(|y| (0..map.width).map(|x| map.get(x, y).glyph()).collect())
        .collect();

    let mut place = |x: i32, y: i32, ch: char| {
        if map.in_bounds(x, y) {
            grid[y as usize][x as usize] = ch;
        }
    };

    for e in world.ground_item_entities() {
        if let Some(p) = world.positions.get(&e) {
            let (x, y) = p.tile();
            place(x, y, '*');
        }
    }
    for e in world.mob_entities() {
        if let (Some(p), Some(a)) = (world.positions.get(&e), world.aggro.get(&e)) {
            let (x, y) = p.tile();
            let ch = match a.behavior {
                AiBehavior::Melee => 'M',
                AiBehavior::Ranged => 'R',
                AiBehavior::Caster => 'C',
            };
            place(x, y, ch);
        }
    }
    if let Some(player) = world.player_entity {
        if let Some(p) = world.positions.get(&player) {
            let (x, y) = p.tile();
            place(x, y, '@');
        }
    }

    grid.into_iter().map(|row| row.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")
}

pub struct TilemapPlugin;

impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(tilemap_load_system);
    }
}
