//! Player setup, input handling, and the player-side scheduling wiring.

use mge_items::affixes::ItemAffixes;
use mge_items::item::Item;
use mge_items::rarity::Rarity;

use crate::app::{App, PhaseId, Plugin};
use crate::components::equipment::{EquipmentSlots, Inventory};
use crate::components::item::ItemInstance;
use crate::components::player::{PlayerTag, Progression, Stats};
use crate::components::skill::{SkillSlot, SkillTree};
use crate::components::{Combat, Faction, Health, Mana, MoveTarget, Position2D};
use crate::data::classes::class_def;
use crate::data::items::base_item;
use crate::data::skills::skills_for;
use crate::ecs::{Entity, World};
use crate::systems::level_up::level_up_system;
use crate::systems::movement::{find_path, movement_system};
use crate::systems::stats::recompute_player_stats;
use crate::systems::waypoint::{ensure_starting_waypoints, waypoint_discovery_system};

/// Tiles per second the player walks.
const PLAYER_SPEED: f32 = 4.5;

/// A high-level command issued by the controller (UI, script, or AI).
#[derive(Debug, Clone)]
pub enum PlayerCommand {
    MoveTo(i32, i32),
    Attack(Entity),
    UseSkill { idx: usize, target: Entity },
    Pickup(Entity),
    SelectSkill(usize),
    /// Open the procedurally-placed chest at `Entity`. Range-gated to 1.5 tiles
    /// of the player, mirroring the [`PlayerCommand::Pickup`] reach.
    OpenChest(Entity),
}

/// Input resource consumed each tick.
#[derive(Debug, Default)]
pub struct PlayerInput {
    pub command: Option<PlayerCommand>,
    /// A skill use queued for the Skills phase.
    pub pending_skill: Option<Entity>,
    /// World-space cursor position, used to aim ground-targeted casts (the
    /// Soldat "Taille" wedge and the Mage's ground AoEs). `None` falls back to
    /// the caster's own position.
    pub cursor_world: Option<Position2D>,
}

/// Spawn the player from `world.chosen_class` at the map's player spawn.
pub fn spawn_player_system(world: &mut World) {
    let class = world.chosen_class;
    let e = world.spawn();
    world.player_entity = Some(e);

    let spawn = world.map.player_spawn;
    world.positions.insert(e, Position2D::from_tile(spawn.0, spawn.1));
    world.factions.insert(e, Faction::Player);
    world.player_tags.insert(e, PlayerTag);
    world.classes.insert(e, class);

    let stats = Stats::new(class, 1);
    let life = stats.inner.resolved.life_max;
    let mana = stats.inner.resolved.mana_max;
    world.health.insert(e, Health::full(life));
    world.mana.insert(e, Mana::full(mana));
    world.stats.insert(e, stats);
    world.progression.insert(e, Progression::new(class));
    world.combat.insert(e, Combat::new(1.8, 0.8));
    world.inventory.insert(e, Inventory::new());
    // Start with only the class's first basic skill on the hotbar; the rest are
    // learned in the skill tree (and then auto-placed / toggled onto the bar).
    let starter: Vec<_> = skills_for(class).into_iter().take(1).collect();
    world.skills.insert(e, SkillSlot::from_skills(starter));

    let mut tree = SkillTree::new(class.to_mge());
    tree.add_points(1);
    world.skill_trees.insert(e, tree);

    // Starting weapon.
    let mut eq = EquipmentSlots::new();
    let weapon_kind = class_def(class).starting_weapon;
    let base = base_item(weapon_kind, 1);
    let weapon_name = format!("Worn {}", base.name);
    let item = Item::new(e.0 * 1_000, base, Rarity::Normal, 1);
    let weapon = ItemInstance::new(item, ItemAffixes::new(), Vec::new(), weapon_name);
    let _ = eq.equip(weapon);
    world.equipment.insert(e, eq);

    // Fold the starting weapon's affixes (none for now, but a future
    // tutorial reroll could put some on) into the stat sheet so combat
    // sees the right damage range from frame one.
    recompute_player_stats(world, e);

    ensure_starting_waypoints(world, e);

    world.log_event(format!(
        "{} enters {} (area level {}).",
        class.display_name(),
        world.map.name,
        world.map.area_level
    ));
}

/// Translate the queued `PlayerCommand` into component changes.
pub fn input_system(world: &mut World) {
    let Some(cmd) = world.input.command.take() else { return };
    let Some(player) = world.player_entity else { return };

    // A combo lock (the Soldat "Estocs") freezes the player: while it plays out
    // its owner can neither move nor attack (see [`ComboLock`]).
    if world.combo_locks.contains_key(&player)
        && matches!(cmd, PlayerCommand::MoveTo(..) | PlayerCommand::Attack(_))
    {
        return;
    }

    match cmd {
        PlayerCommand::MoveTo(tx, ty) => {
            if let Some(&ppos) = world.positions.get(&player) {
                if let Some(path) = find_path(&world.map, ppos.tile(), (tx, ty)) {
                    if !path.is_empty() {
                        world.move_targets.insert(
                            player,
                            MoveTarget::new(Position2D::from_tile(tx, ty), path, PLAYER_SPEED),
                        );
                    }
                }
            }
            if let Some(c) = world.combat.get_mut(&player) {
                c.target = None;
            }
        }
        PlayerCommand::Attack(target) => {
            if let Some(c) = world.combat.get_mut(&player) {
                c.target = Some(target);
            }
            let range = world.combat.get(&player).map_or(1.8, |c| c.range);
            if let (Some(&ppos), Some(&tpos)) =
                (world.positions.get(&player), world.positions.get(&target))
            {
                if ppos.distance(tpos) > range {
                    if let Some(path) = find_path(&world.map, ppos.tile(), tpos.tile()) {
                        if !path.is_empty() {
                            world
                                .move_targets
                                .insert(player, MoveTarget::new(tpos, path, PLAYER_SPEED));
                        }
                    }
                }
            }
        }
        PlayerCommand::UseSkill { idx, target } => {
            if let Some(s) = world.skills.get_mut(&player) {
                s.select(idx);
            }
            world.input.pending_skill = Some(target);
        }
        PlayerCommand::Pickup(item_e) => {
            let near = match (world.positions.get(&player), world.positions.get(&item_e)) {
                (Some(p), Some(i)) => p.distance(*i) <= 1.8,
                _ => false,
            };
            if !near {
                return;
            }
            let Some(instance) = world.ground_items.get(&item_e).map(|g| g.instance.clone()) else {
                return;
            };
            let label = format!("{} [{}]", instance.name, instance.rarity().color_name());
            let added = world
                .inventory
                .get_mut(&player)
                .map_or(false, |inv| inv.add(instance).is_ok());
            if added {
                world.despawn(item_e);
                world.quests.record_item();
                world.log_event(format!("Picked up {label}"));
            } else {
                world.log_event("Inventory full!");
            }
        }
        PlayerCommand::SelectSkill(idx) => {
            if let Some(s) = world.skills.get_mut(&player) {
                s.select(idx);
            }
        }
        PlayerCommand::OpenChest(chest_e) => {
            let near = match (world.positions.get(&player), world.positions.get(&chest_e)) {
                (Some(p), Some(c)) => p.distance(*c) <= 1.5,
                _ => false,
            };
            if !near {
                world.log_event("Trop loin du coffre.");
                return;
            }
            if !world.chests.contains_key(&chest_e) {
                return;
            }
            let opened = crate::systems::loot::open_chest(world, chest_e);
            if !opened {
                world.log_event("Le coffre est vide.");
            }
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system);
        app.add_system(PhaseId::Input, input_system);
        app.add_system(PhaseId::Movement, movement_system);
        app.add_system(PhaseId::Movement, waypoint_discovery_system);
        app.add_system(PhaseId::Progression, level_up_system);
    }
}
