//! Integration tests for skill effects: each cast type is exercised against a
//! freshly-spawned player of the relevant class and a planted dummy mob.

use allumina::components::mob::{AggroRange, AiBehavior, MobTag, MobType};
use allumina::components::player::ClassType;
use allumina::components::skill::AuraType;
use allumina::components::{Combat, Health, Position2D};
use allumina::data::maps::act1_map;
use allumina::data::GameData;
use allumina::ecs::{Entity, World};
use allumina::plugins::player::spawn_player_system;
use allumina::systems::skill::skill_system;

/// Build a world with a spawned player of `class` and one dummy mob at
/// `offset` tiles from the player. Returns `(world, player, mob)`.
fn setup(class: ClassType, offset: (f32, f32)) -> (World, Entity, Entity) {
    let mut world = World::new(act1_map(), GameData::load(), 7);
    world.chosen_class = class;
    spawn_player_system(&mut world);
    let player = world.player_entity.expect("player");
    // The live game starts with one skill (the rest are tree-learned); these
    // per-skill effect tests cast by index, so give the full class hotbar.
    world.skills.insert(
        player,
        allumina::components::skill::SkillSlot::from_skills(allumina::data::skills::skills_for(class)),
    );
    let ppos = *world.positions.get(&player).expect("player pos");

    // Take mana out of the equation for single casts.
    if let Some(m) = world.mana.get_mut(&player) {
        m.max = 500.0;
        m.current = 500.0;
    }

    let mob = world.spawn();
    let mpos = Position2D::new(ppos.x + offset.0, ppos.y + offset.1);
    world.positions.insert(mob, mpos);
    world.health.insert(mob, Health::full(200.0));
    world.mob_tags.insert(mob, MobTag);
    world.mob_types.insert(mob, MobType { def_id: 1 });
    world.aggro.insert(mob, AggroRange::new(6.0, 12.0, AiBehavior::Melee, mpos));
    world.combat.insert(mob, Combat::new(1.4, 1.0));
    (world, player, mob)
}

/// Select skill `idx`, queue it against `target`, and run one skill tick.
fn cast(world: &mut World, idx: usize, target: Entity) {
    let player = world.player_entity.unwrap();
    world.skills.get_mut(&player).unwrap().select(idx);
    world.input.pending_skill = Some(target);
    world.dt = 0.1;
    skill_system(world);
}

// ── Soldat ──────────────────────────────────────────────────────────────────

#[test]
fn grand_coup_damages_single_target() {
    let (mut world, _p, mob) = setup(ClassType::Soldat, (1.0, 0.0));
    cast(&mut world, 0, mob); // Grand coup
    assert!(world.health.get(&mob).unwrap().current < 200.0);
}

#[test]
fn taille_sweeps_the_forward_arc() {
    let (mut world, player, mob) = setup(ClassType::Soldat, (2.0, 0.0));
    cast(&mut world, 1, player); // Taille (cursor-aimed wedge; defaults forward)
    assert!(world.health.get(&mob).unwrap().current < 200.0);
}

#[test]
fn estocs_lands_a_thrust_and_locks_the_combo() {
    let (mut world, player, mob) = setup(ClassType::Soldat, (1.0, 0.0));
    cast(&mut world, 2, mob); // Estocs (first thrust now, lock plays out the rest)
    assert!(world.health.get(&mob).unwrap().current < 200.0);
    assert!(world.combo_locks.contains_key(&player), "the combo should be locked");
}

#[test]
fn advance_order_buffs_the_army() {
    let (mut world, player, _mob) = setup(ClassType::Soldat, (3.0, 0.0));
    cast(&mut world, 6, player); // En avant ! (battle order, self + nearby allies)
    assert!(world.order_buffs.contains_key(&player), "an order should be active");
}

// ── Clerc ─────────────────────────────────────────────────────────────────────

#[test]
fn exorcism_damages_enemies_in_radius() {
    let (mut world, _p, mob) = setup(ClassType::Clerc, (4.0, 0.0));
    // Exorcisme is cursor-aimed: drop the centre on the mob.
    let mpos = *world.positions.get(&mob).unwrap();
    world.input.cursor_world = Some(mpos);
    cast(&mut world, 7, mob); // Exorcisme (BÉNI branch — damages foes in a small radius)
    assert!(world.health.get(&mob).unwrap().current < 200.0);
}

#[test]
fn healing_aura_toggles_on() {
    let (mut world, player, _mob) = setup(ClassType::Clerc, (3.0, 0.0));
    cast(&mut world, 0, player); // Aura de soin (SOUTIEN, idx 0)
    let active = world.active_auras.get(&player).expect("a healing aura should be active");
    assert_eq!(active.aura_type, AuraType::Heal);
}

#[test]
fn prayer_stuns_enemies_in_radius() {
    let (mut world, _p, mob) = setup(ClassType::Clerc, (4.0, 0.0));
    // Prière is cursor-aimed: drop the centre on the mob.
    let mpos = *world.positions.get(&mob).unwrap();
    world.input.cursor_world = Some(mpos);
    cast(&mut world, 6, mob); // Prière (BÉNI branch — stuns foes in a small radius)
    assert!(world.stuns.contains_key(&mob), "prayer should stun its target");
}

// ── Mage de guerre ────────────────────────────────────────────────────────────

#[test]
fn fireball_damages_target_area() {
    let (mut world, _p, mob) = setup(ClassType::MageDeGuerre, (5.0, 0.0));
    cast(&mut world, 0, mob); // Boule de feu (single-target fire bolt)
    assert!(world.health.get(&mob).unwrap().current < 200.0);
}

#[test]
fn ice_lance_slows_and_damages_target() {
    let (mut world, _p, mob) = setup(ClassType::MageDeGuerre, (3.0, 0.0));
    cast(&mut world, 3, mob); // Lance de glace (frost bolt + slow)
    assert!(world.slows.contains_key(&mob), "enemy should be slowed");
    assert!(world.health.get(&mob).unwrap().current < 200.0);
}

#[test]
fn electric_nova_hits_around_caster() {
    let (mut world, player, mob) = setup(ClassType::MageDeGuerre, (2.0, 0.0));
    cast(&mut world, 8, player); // Nova électrique (self-centred burst)
    assert!(world.health.get(&mob).unwrap().current < 200.0);
    assert!(world.stuns.contains_key(&mob), "nova should stun nearby enemies");
}

#[test]
fn flame_wall_deploys_a_burning_zone() {
    let (mut world, player, _mob) = setup(ClassType::MageDeGuerre, (3.0, 0.0));
    let before = world.flame_walls.len();
    cast(&mut world, 1, player); // Mur de flammes (ground rectangle)
    assert_eq!(world.flame_walls.len(), before + 1, "a flame wall should be deployed");
}

#[test]
fn frost_armor_buffs_the_caster() {
    let (mut world, player, _mob) = setup(ClassType::MageDeGuerre, (3.0, 0.0));
    cast(&mut world, 4, player); // Armure de givre (self buff)
    assert!(world.frost_armor.contains_key(&player), "frost armour should be active");
}

#[test]
fn lightning_shield_buffs_the_caster() {
    let (mut world, player, _mob) = setup(ClassType::MageDeGuerre, (3.0, 0.0));
    cast(&mut world, 7, player); // Bouclier de foudre (self buff)
    assert!(world.lightning_shield.contains_key(&player), "lightning shield should be active");
}

// ── Archer ──────────────────────────────────────────────────────────────────

#[test]
fn quick_shot_damages_single_target() {
    let (mut world, _p, mob) = setup(ClassType::Archer, (5.0, 0.0));
    cast(&mut world, 0, mob); // Tir rapide
    assert!(world.health.get(&mob).unwrap().current < 200.0);
}

#[test]
fn poison_shot_applies_a_dot() {
    let (mut world, _p, mob) = setup(ClassType::Archer, (5.0, 0.0));
    cast(&mut world, 3, mob); // Tir empoisonné (idx 3 after the new TIR branch)
    assert!(world.dots.contains_key(&mob), "a poison should be applied");
    assert!(world.def_debuffs.contains_key(&mob), "poison should strip defence");
}

#[test]
fn explosive_trap_is_deployed() {
    let (mut world, player, _mob) = setup(ClassType::Archer, (3.0, 0.0));
    let before = world.traps.len();
    // The trap is cursor-placed; without a cursor it lands at the caster's
    // position. Self-cast is fine — the placement uses cursor_world.
    cast(&mut world, 4, player); // Piège explosif
    assert_eq!(world.traps.len(), before + 1, "a trap should be deployed");
}

#[test]
fn arrow_rain_hits_target_area() {
    let (mut world, _p, mob) = setup(ClassType::Archer, (4.0, 0.0));
    // Pluie de flèches drops a persistent zone; aim it at the mob.
    let mob_pos = *world.positions.get(&mob).unwrap();
    world.input.cursor_world = Some(mob_pos);
    let before = world.arrow_rains.len();
    cast(&mut world, 7, mob); // Pluie de flèches (idx 7)
    assert_eq!(world.arrow_rains.len(), before + 1, "an arrow-rain zone should be deployed");
}
