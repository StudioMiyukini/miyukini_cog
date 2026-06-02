//! Integration tests for the summon (invocation) and mercenary systems.

use allumina::build_app_in;
use allumina::components::mercenary::MercType;
use allumina::components::mob::{AggroRange, AiBehavior, MobTag, MobType};
use allumina::components::player::ClassType;
use allumina::components::skill::SkillEffect;
use allumina::components::summon::SummonType;
use allumina::components::world::ZoneId;
use allumina::components::{Combat, Health, Position2D};
use allumina::data::summon;
use allumina::plugins::player::PlayerCommand;
use allumina::systems::mercenary::{
    active_mercenary, mercenary_ai_system, mercenary_update_system, recruit_mercenary, RESPAWN_TIME,
};
use allumina::systems::summon::{host_size, spawn_summon, summon_system, MAX_SUMMONS};
use allumina::Entity;

const DT: f32 = 1.0 / 30.0;

/// Count the summons of a given kind currently on the field.
fn count_kind(app: &allumina::App, kind: SummonType) -> usize {
    app.world.summon_tags.values().filter(|t| t.summon_type == kind).count()
}

/// Give the player the full class hotbar. The live game now starts with only
/// one skill and learns the rest in the tree (which then places them on the
/// bar); these system tests need the skills present to exercise them.
fn equip_all_skills(app: &mut allumina::App, player: Entity) {
    let class = app.world.classes.get(&player).copied().unwrap();
    let bar = allumina::components::skill::SkillSlot::from_skills(allumina::data::skills::skills_for(class));
    app.world.skills.insert(player, bar);
}

/// Set the points invested in the Soldat's three summon skills (peasant /
/// militiaman / warrior) on the player's hotbar — the source the summon system
/// reads for host size and cross-synergy bonuses.
fn set_host_levels(app: &mut allumina::App, player: Entity, peasant: u32, militiaman: u32, warrior: u32) {
    equip_all_skills(app, player);
    let slot = app.world.skills.get_mut(&player).unwrap();
    for s in slot.bar.iter_mut() {
        if let SkillEffect::Summon { profile } = s.effect {
            s.level = match profile.kind {
                SummonType::Peasant => peasant,
                SummonType::Militiaman => militiaman,
                SummonType::EliteWarrior => warrior,
                _ => s.level,
            };
        }
    }
}

// ── Summons: the global cap (fixed-count hosts) ─────────────────────────────────

#[test]
fn fixed_summons_respect_the_global_cap() {
    let mut app = build_app_in(ClassType::Archer, 7, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();

    let wolves = spawn_summon(&mut app.world, player, &summon::HUNT_WOLF);
    assert_eq!(wolves.len(), 2, "the wolf pack is two");
    assert_eq!(app.world.summon_entities().len(), 2);

    // A jaw trap brings the total to exactly the cap.
    spawn_summon(&mut app.world, player, &summon::JAW_TRAP);
    assert_eq!(app.world.summon_entities().len(), MAX_SUMMONS);

    // Another pack would exceed the cap; oldest summons are evicted to fit.
    spawn_summon(&mut app.world, player, &summon::HUNT_WOLF);
    assert!(app.world.summon_entities().len() <= MAX_SUMMONS);
}

#[test]
fn timed_summon_expires_but_permanent_persists() {
    let mut app = build_app_in(ClassType::Clerc, 1, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();

    let cherubin = spawn_summon(&mut app.world, player, &summon::CHERUBIN)[0]; // 30s
    let angel = spawn_summon(&mut app.world, player, &summon::GUARDIAN_ANGEL)[0]; // permanent

    // Advance well past the timed summon's lifetime.
    app.world.dt = 35.0;
    summon_system(&mut app.world);

    assert!(!app.world.summon_tags.contains_key(&cherubin), "timed summon should expire");
    assert!(app.world.summon_tags.contains_key(&angel), "permanent summon should persist");
}

#[test]
fn player_death_dismisses_summons() {
    let mut app = build_app_in(ClassType::Clerc, 2, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    spawn_summon(&mut app.world, player, &summon::GUARDIAN_ANGEL);
    assert!(!app.world.summon_entities().is_empty());

    app.world.health.get_mut(&player).unwrap().current = 0.0;
    app.world.dt = DT;
    summon_system(&mut app.world);

    assert!(app.world.summon_entities().is_empty(), "summons vanish when the player dies");
}

#[test]
fn summon_inherits_a_fraction_of_owner_life() {
    let mut app = build_app_in(ClassType::MageDeGuerre, 3, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    let php = app.world.health.get(&player).unwrap().max;

    let elemental = spawn_summon(&mut app.world, player, &summon::ELEMENTAL)[0];
    let shp = app.world.health.get(&elemental).unwrap().max;

    let expected = php * summon::ELEMENTAL.hp_pct;
    assert!((shp - expected).abs() < 0.5, "summon life should scale with the owner");
}

#[test]
fn summon_damages_nearby_enemies() {
    let mut app = build_app_in(ClassType::Clerc, 5, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();

    let mob = app.world.mob_entities()[0];
    let mpos = *app.world.positions.get(&mob).unwrap();
    app.world.positions.insert(player, Position2D::new(mpos.x + 1.0, mpos.y));
    let before = app.world.health.get(&mob).unwrap().current;

    let angel = spawn_summon(&mut app.world, player, &summon::GUARDIAN_ANGEL)[0];
    app.world.positions.insert(angel, Position2D::new(mpos.x + 0.6, mpos.y));

    for _ in 0..180 {
        app.tick(DT);
        app.world.render_events.clear();
    }

    let after = app.world.health.get(&mob).map_or(0.0, |h| h.current);
    assert!(after < before, "the guardian angel should hurt the adjacent mob");
}

#[test]
fn casting_a_summon_skill_spawns_via_the_skill_system() {
    let mut app = build_app_in(ClassType::Archer, 4, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    equip_all_skills(&mut app, player);

    let idx = app
        .world
        .skills
        .get(&player)
        .unwrap()
        .bar
        .iter()
        .position(|s| s.name == "Loup de chasse")
        .expect("archer has the hunting-wolf skill");

    // Make sure mana is not the limiting factor.
    {
        let m = app.world.mana.get_mut(&player).unwrap();
        m.max = 200.0;
        m.current = 200.0;
    }
    app.world.input.command = Some(PlayerCommand::UseSkill { idx, target: player });
    app.tick(DT);

    assert_eq!(app.world.summon_entities().len(), 2, "casting summons the wolf pack");
}

// ── Summons: the Soldat's three synergistic hosts ──────────────────────────────

#[test]
fn host_size_scales_with_skill_level() {
    // Peasants: one extra per level.
    assert_eq!(host_size(&summon::PEASANTS, 1), 1);
    assert_eq!(host_size(&summon::PEASANTS, 5), 5);
    // Militia and warriors: one extra every two levels.
    assert_eq!(host_size(&summon::MILITIAMEN, 1), 1);
    assert_eq!(host_size(&summon::MILITIAMEN, 2), 1);
    assert_eq!(host_size(&summon::MILITIAMEN, 3), 2);
    assert_eq!(host_size(&summon::MILITIAMEN, 5), 3);
    assert_eq!(host_size(&summon::ELITE_WARRIORS, 5), 3);
}

#[test]
fn peasant_host_grows_and_refreshes_with_skill_level() {
    let mut app = build_app_in(ClassType::Soldat, 8, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();

    set_host_levels(&mut app, player, 1, 0, 0);
    assert_eq!(spawn_summon(&mut app.world, player, &summon::PEASANTS).len(), 1);
    assert_eq!(count_kind(&app, SummonType::Peasant), 1);

    // Re-casting at a higher level refreshes the swarm to its full new size.
    set_host_levels(&mut app, player, 5, 0, 0);
    assert_eq!(spawn_summon(&mut app.world, player, &summon::PEASANTS).len(), 5);
    assert_eq!(count_kind(&app, SummonType::Peasant), 5, "the host caps at the per-type cap");
}

#[test]
fn soldat_army_exceeds_the_global_cap() {
    let mut app = build_app_in(ClassType::Soldat, 9, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();

    set_host_levels(&mut app, player, 5, 5, 5);
    spawn_summon(&mut app.world, player, &summon::PEASANTS); // 5
    spawn_summon(&mut app.world, player, &summon::MILITIAMEN); // 3
    spawn_summon(&mut app.world, player, &summon::ELITE_WARRIORS); // 3

    // The Soldat's level-scaled hosts ignore the global cap of three.
    assert!(app.world.summon_entities().len() > MAX_SUMMONS);
    assert_eq!(app.world.summon_entities().len(), 11);
}

#[test]
fn cross_synergy_scales_hp_speed_and_attack_rate() {
    let mut app = build_app_in(ClassType::Soldat, 10, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    let php = app.world.health.get(&player).unwrap().max;

    // 5 pts Paysans, 3 pts Miliciens, 2 pts Guerriers.
    set_host_levels(&mut app, player, 5, 3, 2);

    // Peasant: +30% HP (3 militia pts × 10%).
    let peasant = spawn_summon(&mut app.world, player, &summon::PEASANTS)[0];
    let p_hp = app.world.health.get(&peasant).unwrap().max;
    assert!((p_hp - php * summon::PEASANTS.hp_pct * 1.30).abs() < 0.5, "peasant +30% HP");

    // Militiaman: +20% HP (2 warrior pts), +50% move speed (5 peasant pts).
    let mil = spawn_summon(&mut app.world, player, &summon::MILITIAMEN)[0];
    let m_hp = app.world.health.get(&mil).unwrap().max;
    assert!((m_hp - php * summon::MILITIAMEN.hp_pct * 1.20).abs() < 0.5, "militia +20% HP");
    let m_spd = app.world.summon_tags.get(&mil).unwrap().move_speed;
    assert!((m_spd - summon::MILITIAMEN.move_speed * 1.50).abs() < 1e-3, "militia +50% move speed");

    // Warrior: +30% HP (3 militia pts), +50% attack speed (5 peasant pts).
    let war = spawn_summon(&mut app.world, player, &summon::ELITE_WARRIORS)[0];
    let w_hp = app.world.health.get(&war).unwrap().max;
    assert!((w_hp - php * summon::ELITE_WARRIORS.hp_pct * 1.30).abs() < 0.5, "warrior +30% HP");
    let w_int = app.world.combat.get(&war).unwrap().attack_interval;
    assert!(
        (w_int - summon::ELITE_WARRIORS.attack_interval / 1.50).abs() < 1e-3,
        "warrior +50% attack speed (shorter interval)"
    );
}

#[test]
fn cross_synergy_raises_peasant_damage_with_warrior_points() {
    let mut app = build_app_in(ClassType::Soldat, 13, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    // Give the owner meaningful base damage so the bonus survives rounding.
    {
        let s = app.world.stats.get_mut(&player).unwrap();
        s.inner.resolved.damage_min = 100;
        s.inner.resolved.damage_max = 100;
    }

    // Baseline: no warrior points → no damage bonus.
    set_host_levels(&mut app, player, 1, 0, 0);
    let base = spawn_summon(&mut app.world, player, &summon::PEASANTS)[0];
    let base_dmg = app.world.stats.get(&base).unwrap().inner.resolved.damage_max;

    // Five warrior points → +50% damage (a re-cast evicts the baseline first).
    set_host_levels(&mut app, player, 1, 0, 5);
    let boosted = spawn_summon(&mut app.world, player, &summon::PEASANTS)[0];
    let boosted_dmg = app.world.stats.get(&boosted).unwrap().inner.resolved.damage_max;

    assert!(boosted_dmg > base_dmg, "warrior points should raise peasant damage");
}

#[test]
fn test_cross_synergy() {
    let mut app = build_app_in(ClassType::Soldat, 10, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    let php = app.world.health.get(&player).unwrap().max;
    // Large, clean base damage so the +20% bonus survives integer rounding.
    {
        let s = app.world.stats.get_mut(&player).unwrap();
        s.inner.resolved.damage_min = 1000;
        s.inner.resolved.damage_max = 1000;
    }

    // Baseline peasant: 5 peasant / 3 militia / 0 warrior → no damage synergy yet.
    set_host_levels(&mut app, player, 5, 3, 0);
    let base = spawn_summon(&mut app.world, player, &summon::PEASANTS)[0];
    let base_dmg = app.world.stats.get(&base).unwrap().inner.resolved.damage_max;

    // 5 peasant / 3 militia / 2 warrior: the peasant gains +30% HP (3 militia
    // points) and +20% damage (2 warrior points). The re-cast evicts the baseline.
    set_host_levels(&mut app, player, 5, 3, 2);
    let peasant = spawn_summon(&mut app.world, player, &summon::PEASANTS)[0];

    let hp = app.world.health.get(&peasant).unwrap().max;
    assert!((hp - php * summon::PEASANTS.hp_pct * 1.30).abs() < 0.5, "peasant +30% HP");

    let dmg = app.world.stats.get(&peasant).unwrap().inner.resolved.damage_max;
    assert!((dmg as f32 - base_dmg as f32 * 1.20).abs() <= 1.0, "peasant +20% damage");
}

#[test]
fn non_soldat_summons_get_no_cross_synergy() {
    let mut app = build_app_in(ClassType::MageDeGuerre, 14, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    let php = app.world.health.get(&player).unwrap().max;

    let elemental = spawn_summon(&mut app.world, player, &summon::ELEMENTAL)[0];
    let hp = app.world.health.get(&elemental).unwrap().max;
    let speed = app.world.summon_tags.get(&elemental).unwrap().move_speed;

    assert!((hp - php * summon::ELEMENTAL.hp_pct).abs() < 0.5, "no HP synergy off-class");
    assert!((speed - summon::ELEMENTAL.move_speed).abs() < 1e-3, "no speed synergy off-class");
}

#[test]
fn elite_warrior_cleaves_nearby_enemies() {
    let mut app = build_app_in(ClassType::Soldat, 12, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    let ppos = *app.world.positions.get(&player).unwrap();

    let warrior = spawn_summon(&mut app.world, player, &summon::ELITE_WARRIORS)[0];
    let wpos = *app.world.positions.get(&warrior).unwrap();

    // A dummy mob standing within the cleave radius (2.0 tiles).
    let mob = app.world.spawn();
    app.world.positions.insert(mob, Position2D::new(wpos.x + 1.0, wpos.y));
    app.world.health.insert(mob, Health::full(500.0));
    app.world.mob_tags.insert(mob, MobTag);
    app.world.mob_types.insert(mob, MobType { def_id: 1 });
    app.world
        .aggro
        .insert(mob, AggroRange::new(6.0, 12.0, AiBehavior::Melee, ppos));
    app.world.combat.insert(mob, Combat::new(1.4, 1.0));
    let before = app.world.health.get(&mob).unwrap().current;

    // Run only the summon lifecycle (no normal combat) past the 5s cleave timer,
    // so the only damage observed is the warrior's "Frappe de taille".
    app.world.dt = 6.0;
    summon_system(&mut app.world);

    let after = app.world.health.get(&mob).unwrap().current;
    assert!(after < before, "the elite warrior should cleave the adjacent mob");
}

// ── Mercenaries ───────────────────────────────────────────────────────────────

fn village_with_gold(seed: u64, gold: u64) -> (allumina::App, Entity) {
    let mut app = build_app_in(ClassType::Soldat, seed, ZoneId::Village);
    let player = app.world.player_entity.unwrap();
    app.world.inventory.get_mut(&player).unwrap().add_gold(gold);
    (app, player)
}

#[test]
fn recruit_mercenary_at_village_costs_gold() {
    let (mut app, player) = village_with_gold(1, 2000);
    let before = app.world.inventory.get(&player).unwrap().gold;

    assert!(recruit_mercenary(&mut app.world, player, MercType::Guardian));
    assert!(active_mercenary(&app.world, player).is_some());

    let after = app.world.inventory.get(&player).unwrap().gold;
    assert_eq!(before - after, 500, "guardian costs 500 gold");
}

#[test]
fn recruit_fails_without_gold() {
    let mut app = build_app_in(ClassType::Soldat, 1, ZoneId::Village);
    let player = app.world.player_entity.unwrap();
    assert!(!recruit_mercenary(&mut app.world, player, MercType::Mage));
    assert!(active_mercenary(&app.world, player).is_none());
}

#[test]
fn recruit_requires_a_safe_zone() {
    let mut app = build_app_in(ClassType::Soldat, 1, ZoneId::GreenPlains);
    let player = app.world.player_entity.unwrap();
    app.world.inventory.get_mut(&player).unwrap().add_gold(2000);
    assert!(!recruit_mercenary(&mut app.world, player, MercType::Guardian));
    assert!(active_mercenary(&app.world, player).is_none());
}

#[test]
fn hiring_builds_a_multi_companion_party() {
    let (mut app, player) = village_with_gold(2, 5000);

    // Distinct archetypes accumulate into the party (no longer replace).
    assert!(recruit_mercenary(&mut app.world, player, MercType::Guardian));
    assert!(recruit_mercenary(&mut app.world, player, MercType::Archer));
    assert_eq!(app.world.mercenary_entities().len(), 2, "both companions serve at once");

    // A second of the same archetype is refused.
    assert!(!recruit_mercenary(&mut app.world, player, MercType::Guardian));
    assert_eq!(app.world.mercenary_entities().len(), 2);
}

#[test]
fn mercenary_levels_up_from_player_xp() {
    let (mut app, player) = village_with_gold(3, 2000);
    assert!(recruit_mercenary(&mut app.world, player, MercType::Guardian));
    let merc = active_mercenary(&app.world, player).unwrap();
    assert_eq!(app.world.merc_states[&merc].level, 1);

    app.world.progression.get_mut(&player).unwrap().inner.grant_xp(100_000);
    app.world.dt = DT;
    mercenary_update_system(&mut app.world);

    assert!(app.world.merc_states[&merc].level > 1, "merc should level from the player's XP");
}

#[test]
fn mercenary_dies_then_respawns() {
    let (mut app, player) = village_with_gold(4, 2000);
    assert!(recruit_mercenary(&mut app.world, player, MercType::Guardian));
    let merc = active_mercenary(&app.world, player).unwrap();

    // Kill the mercenary.
    app.world.health.get_mut(&merc).unwrap().current = 0.0;
    app.world.dt = DT;
    mercenary_update_system(&mut app.world);
    assert!(app.world.merc_states[&merc].is_dead(), "merc enters the respawn state");

    // Wait out the respawn timer.
    app.world.dt = RESPAWN_TIME + 1.0;
    mercenary_update_system(&mut app.world);
    assert!(!app.world.merc_states[&merc].is_dead(), "merc should return");
    assert!(app.world.health.get(&merc).unwrap().current > 0.0, "respawn restores health");
}

#[test]
fn healer_mends_a_wounded_player() {
    let (mut app, player) = village_with_gold(5, 2000);
    assert!(recruit_mercenary(&mut app.world, player, MercType::Healer));

    let max = app.world.health.get(&player).unwrap().max;
    app.world.health.get_mut(&player).unwrap().current = max * 0.3;
    let before = app.world.health.get(&player).unwrap().current;

    app.world.dt = DT;
    mercenary_ai_system(&mut app.world);

    let after = app.world.health.get(&player).unwrap().current;
    assert!(after > before, "the healer should heal the player below half health");
}
