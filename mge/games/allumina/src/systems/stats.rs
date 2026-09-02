//! @id     games_allumina_systems_stats
//! @do     definir_equipment_source_max
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems stats ».

//! Player stat recomputation: sums the base sheet plus every equipped affix
//! and refreshes the cached pools that depend on it.
//!
//! Always go through [`recompute_player_stats`] after touching equipment,
//! attributes, or level. The combat/UI code reads `stats.inner.resolved` and
//! the `Health`/`Mana` components — they only stay in sync if this fires.
//!
//! `Combat.attack_interval` is *not* mutated here: it stays as the base value
//! the entity was spawned with. `combat_system` factors in
//! `resolved.attack_speed` at swing time, the same way it already factors in
//! auras and battle orders.
//!
//! `Combat.range`, on the other hand, *is* refreshed here from the main-hand
//! weapon kind: equipping a bow stretches the auto-attack reach to 8 tiles,
//! unequipping it (or swapping back to a sword) snaps it back to melee.
//!
//! On top of the modifier pipeline this module also applies the Allumina
//! *derived stat formulas*:
//!
//! | base attribute   | contribution                  |
//! | ---------------- | ----------------------------- |
//! | +1 Strength      | +1 min damage, +1 max damage  |
//! | +1 Dexterity     | +2 base defence               |
//! | +1 Vitality      | +3 max HP                     |
//! | +1 Energy / INT  | +3 max Mana                   |
//!
//! These layer on top of any flat `+DamageMin`, `+Defense`, etc. affixes
//! rolled on worn gear.

use mge_gameplay::stats::{Modifier, ResolvedStats, StatKind};
use mge_items::item::{EquipRequirements, EquipSlot, ItemKind};

use crate::components::item::{attack_range, is_one_handed_melee};
use crate::components::player::{Stats, ALLUMINA_BASE_ATTR};
use crate::ecs::{Entity, World};

// Block tuning lives in `systems::balance` (single source of truth).
use crate::systems::balance::{BLOCK_CAP, DUAL_WIELD_BLOCK_BASE, SHIELD_BLOCK_BASE};

/// `source_id` threshold separating worn-gear modifiers from persistent
/// non-gear modifiers (currently only allocated stat points — see
/// `progression::spend_stat_point`, which OR's `0x8000_0000` into its id).
/// `recompute_player_stats` rewrites everything below this threshold and
/// leaves the rest alone.
const EQUIPMENT_SOURCE_MAX: u64 = 0x8000_0000;

/// Re-derive `stats.resolved`, `Combat.range`, and `Health`/`Mana` max for
/// `entity` from base attributes plus equipment affixes. Safe to call on
/// entities without a `Stats` component (no-op).
pub fn recompute_player_stats(world: &mut World, entity: Entity) {
    let equipment_mods =
        world.equipment.get(&entity).map(|eq| eq.all_modifiers()).unwrap_or_default();
    // Read the equipment shape (shield / dual-wield) before the mutable borrow
    // below; it feeds the intrinsic block chance.
    let block_base = equipment_block_base(world, entity);
    // Base armour value of every worn piece (armour items carry no Defense in
    // their base def, so without this they'd grant 0 DEF unless they rolled a
    // +Defense affix). Summed before the mutable borrow.
    let armor_base = world.equipment.get(&entity).map_or(0, |eq| {
        eq.iter().map(|e| armor_base_defense(e.item.item.base.kind, e.item.ilvl() as i32)).sum()
    });

    if let Some(stats) = world.stats.get_mut(&entity) {
        // Drop every modifier in the equipment id range (i.e. previously
        // worn items, including any piece we just unequipped) and re-add
        // the current set. Non-gear sources — spent stat points, buffs —
        // live above the threshold and survive untouched.
        stats.inner.modifiers.retain(|m| m.source_id >= EQUIPMENT_SOURCE_MAX);
        for m in equipment_mods {
            stats.inner.modifiers.push(m);
        }
        // mge-gameplay's generic recompute handles resists, crit, speeds,
        // life-on-hit, etc. We then overwrite the lines whose formulas
        // Allumina overrides with primary-attribute scaling.
        stats.inner.recompute();
        apply_allumina_derived_formulas(stats);

        // Worn armour's intrinsic defence, on top of the DEX/affix defence the
        // derived formulas just set.
        stats.inner.resolved.defense += armor_base;

        // Block chance: a shield (or two one-handed weapons) grants a base
        // chance that Dexterity sharpens (+1% per 5 DEX), layered on top of any
        // `+Block` gear affixes the generic recompute already summed in. Without
        // a blocking off-hand there is no intrinsic block.
        let affix_block = stats.inner.resolved.block_chance;
        let dex = stats.inner.effective_attributes().dexterity.max(0);
        let intrinsic = block_chance_from_base(block_base, dex);
        stats.inner.resolved.block_chance = (affix_block + intrinsic).clamp(0, BLOCK_CAP);
    } else {
        return;
    }

    sync_pools(world, entity);
    sync_combat_range(world, entity);
}

/// Base block chance (%) from the off-hand configuration: a shield blocks best,
/// dual-wielding two one-handed melee weapons parries for less, and an empty or
/// ranged-occupied off-hand grants none.
#[must_use]
fn equipment_block_base(world: &World, entity: Entity) -> i32 {
    let Some(eq) = world.equipment.get(&entity) else { return 0 };
    let off = eq.get(EquipSlot::OffHand).map(|i| i.item.base.kind);
    let main = eq.get(EquipSlot::MainHand).map(|i| i.item.base.kind);
    block_base_from_kinds(main, off)
}

/// Intrinsic defence an armour piece grants from its kind + ilvl (base item
/// defs carry no defence, so this is what makes armour actually protect). Scales
/// gently with item level. Non-armour kinds grant none.
#[must_use]
pub fn armor_base_defense(kind: ItemKind, ilvl: i32) -> i32 {
    let base = match kind {
        ItemKind::BodyArmor => 18,
        ItemKind::Shield => 12,
        ItemKind::Helm => 8,
        ItemKind::Belt => 6,
        ItemKind::Gloves => 5,
        ItemKind::Boots => 5,
        _ => 0,
    };
    if base == 0 { 0 } else { base + base * ilvl.max(0) / 4 }
}

/// Base block (%) for a given main-/off-hand item-kind pair, before Dexterity.
#[must_use]
pub fn block_base_from_kinds(main: Option<ItemKind>, off: Option<ItemKind>) -> i32 {
    if off == Some(ItemKind::Shield) {
        SHIELD_BLOCK_BASE
    } else if matches!(off, Some(k) if is_one_handed_melee(k))
        && matches!(main, Some(k) if is_one_handed_melee(k))
    {
        DUAL_WIELD_BLOCK_BASE
    } else {
        0
    }
}

/// Final intrinsic block chance (%) from a base and Dexterity (+1% per 5 DEX),
/// capped. `0` base (no blocking off-hand) yields `0`. Shared by the player and
/// mercenary recompute paths.
#[must_use]
pub fn block_chance_from_base(base: i32, dex: i32) -> i32 {
    if base > 0 { (base + dex.max(0) / 5).clamp(0, BLOCK_CAP) } else { 0 }
}

/// Override the four "primary-attribute-fed" resolved stats with Allumina's
/// derived formulas, layered with any rolled flat/percentage affixes.
fn apply_allumina_derived_formulas(stats: &mut Stats) {
    let eff = stats.inner.effective_attributes();
    let level = stats.inner.level;

    let mods_for = |stat: StatKind| -> Vec<Modifier> {
        stats.inner.modifiers.iter().filter(|m| m.stat == stat).map(|m| m.modifier).collect()
    };

    // Strength feeds flat melee damage at a *third* of a point each (≈+1 per 3
    // STR), not 1:1 — combined with the gentler primary multiplier in
    // `systems::combat`, an all-STR build no longer trivialises the game.
    stats.inner.resolved.damage_min =
        ResolvedStats::apply_modifiers(1 + eff.strength / 3, &mods_for(StatKind::DamageMin));
    stats.inner.resolved.damage_max =
        ResolvedStats::apply_modifiers(4 + eff.strength / 3, &mods_for(StatKind::DamageMax));

    // +1 Dexterity → +2 base defence.
    stats.inner.resolved.defense =
        ResolvedStats::apply_modifiers(2 * eff.dexterity, &mods_for(StatKind::Defense));

    // +1 Vitality (Constitution) → +1 max HP, on top of the flat 45 floor and
    // the +2/level rolling bonus (the "bonus" the player keeps besides the 1:1
    // Constitution contribution).
    let base_life = 45.0 + eff.vitality as f32 + (level as f32 - 1.0) * 2.0;
    stats.inner.resolved.life_max =
        ResolvedStats::apply_modifiers_f32(base_life, &mods_for(StatKind::Life));

    // +1 Energy (Intelligence) → +3 max Mana on top of the flat 10 floor.
    let base_mana = 10.0 + 3.0 * eff.energy as f32;
    stats.inner.resolved.mana_max =
        ResolvedStats::apply_modifiers_f32(base_mana, &mods_for(StatKind::Mana));
}

/// Resize `Health.max`/`Mana.max` to match the newly resolved pools. The
/// fraction is preserved (via `Pool::set_max`) so a +Life ring doesn't
/// full-heal — current HP scales with max.
fn sync_pools(world: &mut World, entity: Entity) {
    let Some((life_max, mana_max)) = world
        .stats
        .get(&entity)
        .map(|s| (s.inner.resolved.life_max, s.inner.resolved.mana_max))
    else {
        return;
    };
    if let Some(h) = world.health.get_mut(&entity) {
        h.set_max(life_max.max(1.0));
    }
    if let Some(m) = world.mana.get_mut(&entity) {
        m.set_max(mana_max.max(0.0));
    }
}

/// Sync `Combat.range` with the equipped main-hand weapon (bow → 8 tiles,
/// staff/wand/scepter → 6, melee → 1.5). Unarmed (no main-hand) snaps back to
/// melee.
fn sync_combat_range(world: &mut World, entity: Entity) {
    let new_range = world
        .equipment
        .get(&entity)
        .and_then(|eq| eq.get(EquipSlot::MainHand))
        .map_or(1.5, |w| attack_range(w.item.base.kind));
    if let Some(c) = world.combat.get_mut(&entity) {
        c.range = new_range;
    }
}

/// Whether `entity` meets every minimum-attribute requirement on a piece of
/// gear. Reads *effective* attributes (base + Flat modifiers) so a +5 STR
/// amulet can let you wear a STR-25 axe.
#[must_use]
pub fn meets_requirements(world: &World, entity: Entity, req: &EquipRequirements) -> bool {
    let Some(stats) = world.stats.get(&entity) else { return false };
    let eff = stats.inner.effective_attributes();
    let level = world.progression.get(&entity).map_or(stats.inner.level, |p| p.level());
    eff.strength.max(0) as u32 >= req.min_strength
        && eff.dexterity.max(0) as u32 >= req.min_dexterity
        && eff.energy.max(0) as u32 >= req.min_intelligence
        && level >= req.min_level
}

/// One of the four primary attributes a player can spend stat points on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryAttr {
    Strength,
    Dexterity,
    /// Constitution in Allumina's vocabulary; Vitality in `mge-gameplay`.
    Vitality,
    /// Intelligence in Allumina's vocabulary; Energy in `mge-gameplay`.
    Energy,
}

fn bump_attr(stats: &mut Stats, attr: PrimaryAttr, delta: i32) {
    let a = &mut stats.inner.attributes;
    match attr {
        PrimaryAttr::Strength => a.strength += delta,
        PrimaryAttr::Dexterity => a.dexterity += delta,
        PrimaryAttr::Vitality => a.vitality += delta,
        PrimaryAttr::Energy => a.energy += delta,
    }
}

fn attr_value(stats: &Stats, attr: PrimaryAttr) -> i32 {
    let a = &stats.inner.attributes;
    match attr {
        PrimaryAttr::Strength => a.strength,
        PrimaryAttr::Dexterity => a.dexterity,
        PrimaryAttr::Vitality => a.vitality,
        PrimaryAttr::Energy => a.energy,
    }
}

/// Spend one stat point on `attr`. Bumps the base attribute by 1 and triggers
/// a full recompute so derived stats follow immediately. Returns `false` if
/// no points are left.
pub fn spend_stat_point(world: &mut World, entity: Entity, attr: PrimaryAttr) -> bool {
    let Some(prog) = world.progression.get_mut(&entity) else { return false };
    if prog.inner.stat_points == 0 {
        return false;
    }
    prog.inner.stat_points -= 1;
    if let Some(stats) = world.stats.get_mut(&entity) {
        bump_attr(stats, attr, 1);
    }
    recompute_player_stats(world, entity);
    true
}

/// Refund one previously-spent stat point. Refuses to drop a base attribute
/// below the [`ALLUMINA_BASE_ATTR`] starting floor. Returns whether anything
/// was refunded.
pub fn refund_stat_point(world: &mut World, entity: Entity, attr: PrimaryAttr) -> bool {
    let cur = world.stats.get(&entity).map_or(0, |s| attr_value(s, attr));
    if cur <= ALLUMINA_BASE_ATTR {
        return false;
    }
    if let Some(stats) = world.stats.get_mut(&entity) {
        bump_attr(stats, attr, -1);
    }
    if let Some(prog) = world.progression.get_mut(&entity) {
        prog.inner.stat_points += 1;
    }
    recompute_player_stats(world, entity);
    true
}

#[cfg(test)]
mod tests {
    use mge_gameplay::stats::{Modifier, StatKind};
    use mge_items::affixes::{AffixKind, AffixMod, AppliedAffix, ItemAffixes};
    use mge_items::item::{EquipRequirements, EquipSlot, Item, ItemKind};
    use mge_items::rarity::Rarity;

    use super::{
        meets_requirements, recompute_player_stats, refund_stat_point, spend_stat_point,
        PrimaryAttr,
    };
    use crate::components::equipment::{EquipmentSlots, Inventory};
    use crate::components::item::{GemKind, ItemInstance, SocketedGem};
    use crate::components::player::{ClassType, PlayerTag, Progression, Stats};
    use crate::components::{Combat, Faction, Health, Mana, Position2D};
    use crate::data::items::base_item;
    use crate::data::maps::village_map;
    use crate::data::GameData;
    use crate::ecs::{Entity, World};
    use crate::systems::combat::{effective_damage, effective_defense};

    fn spawn_test_player(world: &mut World, class: ClassType) -> Entity {
        let e = world.spawn();
        world.player_entity = Some(e);
        world.positions.insert(e, Position2D::new(0.0, 0.0));
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
        world.equipment.insert(e, EquipmentSlots::new());
        world.inventory.insert(e, Inventory::new());
        world.combat.insert(e, Combat::new(1.8, 0.8));
        // Apply Allumina derived formulas to the fresh sheet.
        recompute_player_stats(world, e);
        e
    }

    fn defense_chest(uid: u64, defense: i32) -> ItemInstance {
        // Body armour → Chest slot, with a single flat-Defense suffix.
        // Strip the auto-generated requirements so this test never trips on
        // the gating logic — we're measuring +DEF, not requirement checks.
        let mut base = base_item(ItemKind::BodyArmor, 1);
        base.requirements = EquipRequirements::default();
        let item = Item::new(uid, base, Rarity::Magic, 1);
        let mut affixes = ItemAffixes::new();
        affixes.suffixes.push(AppliedAffix {
            def_id: 9001,
            kind: AffixKind::Suffix,
            mods: vec![AffixMod { stat: StatKind::Defense, modifier: Modifier::Flat(defense) }],
        });
        ItemInstance::new(item, affixes, Vec::new(), format!("Test Chest of +{defense} Def"))
    }

    #[test]
    fn equipping_a_defense_chest_adds_affix_plus_base_armor() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);

        let baseline = effective_defense(&world, player);

        let chest = defense_chest(42, 10);
        world.inventory.get_mut(&player).unwrap().add(chest).expect("inv slot");
        let equipped = crate::plugins::inventory::equip_from_inventory(&mut world, player, 0);
        assert!(equipped, "equip_from_inventory should succeed");

        // Rise = the +10 Defense affix plus the body armour's intrinsic base.
        let expected = 10.0 + super::armor_base_defense(ItemKind::BodyArmor, 1) as f32;
        let after = effective_defense(&world, player);
        assert!(
            (after - baseline - expected).abs() < 0.001,
            "expected effective_defense to rise by {expected} (baseline {baseline}, after {after})",
        );

        let unequipped =
            crate::plugins::inventory::unequip_to_inventory(&mut world, player, EquipSlot::Chest);
        assert!(unequipped, "unequip_to_inventory should succeed");
        let restored = effective_defense(&world, player);
        assert!(
            (restored - baseline).abs() < 0.001,
            "expected effective_defense to return to baseline (baseline {baseline}, restored {restored})",
        );
    }

    #[test]
    fn recompute_is_idempotent() {
        // Calling recompute twice with the same gear must not double the
        // contribution — this guards against the original bug where the
        // workaround in combat.rs added gear DEF on top of an already-folded
        // resolved.defense.
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);

        let chest = defense_chest(99, 25);
        world.equipment.get_mut(&player).unwrap().slots.insert(EquipSlot::Chest, chest);

        recompute_player_stats(&mut world, player);
        let once = effective_defense(&world, player);
        recompute_player_stats(&mut world, player);
        let twice = effective_defense(&world, player);

        assert!(
            (once - twice).abs() < 0.001,
            "recompute must be idempotent (once {once}, twice {twice})",
        );
    }

    #[test]
    fn starts_with_twenty_in_every_attribute() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::MageDeGuerre);
        let stats = world.stats.get(&player).unwrap();
        let a = &stats.inner.attributes;
        assert_eq!((a.strength, a.dexterity, a.vitality, a.energy), (20, 20, 20, 20));
    }

    #[test]
    fn allumina_derived_formulas_apply_on_fresh_character() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        let stats = world.stats.get(&player).unwrap();

        // Base 20s with the Allumina formulas:
        //   defence    = 2 * 20            = 40 (no armour worn)
        //   damage_min = 1 + 20/3          = 7
        //   damage_max = 4 + 20/3          = 10
        //   life_max   = 45 + 1*20 + 0     = 65
        //   mana_max   = 10 + 3*20         = 70
        assert_eq!(stats.inner.resolved.defense, 40);
        assert_eq!(stats.inner.resolved.damage_min, 7);
        assert_eq!(stats.inner.resolved.damage_max, 10);
        assert!((stats.inner.resolved.life_max - 65.0).abs() < 0.001);
        assert!((stats.inner.resolved.mana_max - 70.0).abs() < 0.001);

        // Health.max must follow the resolved pool, not its construction-time snapshot.
        assert!((world.health.get(&player).unwrap().max - 65.0).abs() < 0.001);
    }

    #[test]
    fn spending_a_stat_point_in_strength_raises_damage() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        world.progression.get_mut(&player).unwrap().inner.stat_points = 1;

        let (dmg_min_before, dmg_max_before) = effective_damage(&world, player);
        assert!(spend_stat_point(&mut world, player, PrimaryAttr::Strength));
        let (dmg_min_after, dmg_max_after) = effective_damage(&world, player);

        assert!(
            dmg_min_after > dmg_min_before,
            "damage_min should rise after a STR point (before {dmg_min_before}, after {dmg_min_after})",
        );
        assert!(
            dmg_max_after > dmg_max_before,
            "damage_max should rise after a STR point (before {dmg_max_before}, after {dmg_max_after})",
        );
        assert_eq!(world.progression.get(&player).unwrap().inner.stat_points, 0);
    }

    #[test]
    fn spending_in_vitality_raises_life_max_by_one() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        world.progression.get_mut(&player).unwrap().inner.stat_points = 1;

        let life_before = world.stats.get(&player).unwrap().inner.resolved.life_max;
        let max_before = world.health.get(&player).unwrap().max;

        assert!(spend_stat_point(&mut world, player, PrimaryAttr::Vitality));

        let life_after = world.stats.get(&player).unwrap().inner.resolved.life_max;
        let max_after = world.health.get(&player).unwrap().max;
        assert!((life_after - life_before - 1.0).abs() < 0.001);
        assert!((max_after - max_before - 1.0).abs() < 0.001);
    }

    #[test]
    fn refund_floors_at_base_twenty() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        let refunded = refund_stat_point(&mut world, player, PrimaryAttr::Strength);
        assert!(!refunded, "should refuse to refund below the 20-point floor");
        assert_eq!(world.stats.get(&player).unwrap().inner.attributes.strength, 20);
    }

    fn weapon(uid: u64, kind: ItemKind) -> ItemInstance {
        let mut base = base_item(kind, 1);
        base.requirements = EquipRequirements::default();
        let item = Item::new(uid, base, Rarity::Normal, 1);
        ItemInstance::new(item, ItemAffixes::new(), Vec::new(), format!("Test {kind:?}"))
    }

    #[test]
    fn shield_grants_block_chance() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        assert_eq!(world.stats.get(&player).unwrap().inner.resolved.block_chance, 0, "no block bare-handed");

        let shield = weapon(1, ItemKind::Shield);
        world.equipment.get_mut(&player).unwrap().equip(shield).unwrap();
        recompute_player_stats(&mut world, player);
        // SHIELD_BLOCK_BASE 30 + DEX 20/5 = 34.
        assert_eq!(world.stats.get(&player).unwrap().inner.resolved.block_chance, 34);
    }

    #[test]
    fn dual_wielding_two_one_handers_grants_block() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        let eq = world.equipment.get_mut(&player).unwrap();
        eq.equip(weapon(1, ItemKind::Sword)).unwrap(); // MainHand
        eq.equip_to(EquipSlot::OffHand, weapon(2, ItemKind::Axe)); // off-hand weapon
        recompute_player_stats(&mut world, player);
        // DUAL_WIELD_BLOCK_BASE 15 + DEX 20/5 = 19.
        assert_eq!(world.stats.get(&player).unwrap().inner.resolved.block_chance, 19);
    }

    /// The player's effective Dexterity after stat recompute (gems + gear).
    fn eff_dex(world: &World, player: Entity) -> i32 {
        world.stats.get(&player).unwrap().inner.effective_attributes().dexterity
    }

    #[test]
    fn socketing_a_gem_boosts_the_worn_item() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);

        // A worn chest with one open socket.
        let mut chest = weapon(30, ItemKind::BodyArmor);
        chest.sockets = 1;
        world.equipment.get_mut(&player).unwrap().equip_to(EquipSlot::Chest, chest);
        recompute_player_stats(&mut world, player);
        let dex_before = eff_dex(&world, player);

        // A perfect Émeraude (+10 Dexterity in armour) socketed via the plugin.
        let gem_item = Item::new(31, base_item(ItemKind::Gem, 1), Rarity::Magic, 1);
        let mut gem = ItemInstance::new(gem_item, ItemAffixes::new(), Vec::new(), "Émeraude parfait".into());
        gem.gem = Some(SocketedGem { kind: GemKind::Emeraude, tier: 4 });
        world.inventory.get_mut(&player).unwrap().add(gem).expect("slot");
        assert!(crate::plugins::inventory::socket_gem_from_inventory(&mut world, player, 0));

        assert_eq!(eff_dex(&world, player), dex_before + 10, "the armour gem adds +10 Dex");
        assert_eq!(world.inventory.get(&player).unwrap().count(), 0, "the gem is consumed");
        assert_eq!(
            world.equipment.get(&player).unwrap().get(EquipSlot::Chest).unwrap().open_sockets(),
            0,
            "the socket is now filled",
        );
    }

    #[test]
    fn the_same_gem_grants_different_effects_by_host() {
        // A perfect Rubis: +Damage in a weapon, +Life in armour, Fire Resist in
        // a shield. The host item decides which table applies.
        let weapon_gem = SocketedGem { kind: GemKind::Rubis, tier: 4 };
        let mut sword = weapon(20, ItemKind::Sword);
        sword.sockets = 1;
        sword.socketed.push(weapon_gem);
        let mut body = weapon(20, ItemKind::BodyArmor);
        body.sockets = 1;
        body.socketed.push(weapon_gem);
        let mut shield = weapon(20, ItemKind::Shield);
        shield.sockets = 1;
        shield.socketed.push(weapon_gem);

        let has = |it: &ItemInstance, stat: StatKind| {
            it.stat_modifiers().iter().any(|m| m.stat == stat)
        };
        assert!(has(&sword, StatKind::DamageMax), "Rubis in a weapon adds damage");
        assert!(has(&body, StatKind::Life), "Rubis in armour adds life");
        assert!(has(&shield, StatKind::FireResist), "Rubis in a shield adds fire resist");
    }

    #[test]
    fn zirconia_in_armour_eases_its_requirements() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);

        // Without a gem, an over-STR chest can't be worn (slot 0).
        let mut heavy = weapon(30, ItemKind::BodyArmor);
        heavy.item.base.requirements.min_strength = 1_000;
        world.inventory.get_mut(&player).unwrap().add(heavy).expect("slot");
        assert!(
            !crate::plugins::inventory::equip_from_inventory(&mut world, player, 0),
            "the chest is too heavy without the gem",
        );

        // A perfect Zirconia (−50%) drops a 35-STR chest to 17 — now wearable.
        let mut eased = weapon(30, ItemKind::BodyArmor);
        eased.item.base.requirements.min_strength = 35;
        eased.sockets = 1;
        eased.socketed.push(SocketedGem { kind: GemKind::Zirconia, tier: 4 });
        world.inventory.get_mut(&player).unwrap().add(eased).expect("slot");
        assert!(
            crate::plugins::inventory::equip_from_inventory(&mut world, player, 1),
            "the Zirconia drops the requirement low enough to wear",
        );
    }

    #[test]
    fn unsocketing_refunds_the_gem_and_charges_gold() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);

        let mut chest = weapon(30, ItemKind::BodyArmor);
        chest.sockets = 1;
        world.equipment.get_mut(&player).unwrap().equip_to(EquipSlot::Chest, chest);

        // Socket a perfect Émeraude (+10 Dex in armour), then fund the extraction.
        let gem_item = Item::new(31, base_item(ItemKind::Gem, 1), Rarity::Magic, 1);
        let mut gem = ItemInstance::new(gem_item, ItemAffixes::new(), Vec::new(), "Émeraude parfait".into());
        gem.gem = Some(SocketedGem { kind: GemKind::Emeraude, tier: 4 });
        world.inventory.get_mut(&player).unwrap().add(gem).expect("slot");
        assert!(crate::plugins::inventory::socket_gem_from_inventory(&mut world, player, 0));
        recompute_player_stats(&mut world, player);
        let dex_socketed = eff_dex(&world, player);
        world.inventory.get_mut(&player).unwrap().gold = 100_000;

        // Extracting the gem returns it to the bag, charges gold and drops the bonus.
        assert!(crate::plugins::inventory::unsocket_equipped(&mut world, player, EquipSlot::Chest));
        recompute_player_stats(&mut world, player);
        assert!(world.inventory.get(&player).unwrap().gold < 100_000, "the fee is paid");
        assert_eq!(world.inventory.get(&player).unwrap().count(), 1, "the gem is returned");
        assert_eq!(eff_dex(&world, player), dex_socketed - 10, "removing the gem drops its Dex");
        assert_eq!(
            world.equipment.get(&player).unwrap().get(EquipSlot::Chest).unwrap().open_sockets(),
            1,
            "the socket is open again",
        );
    }

    #[test]
    fn plain_armor_grants_base_defense() {
        // A Normal (affix-less) body armour used to give 0 DEF; now it grants
        // its intrinsic base armour value.
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        let before = effective_defense(&world, player);
        world.inventory.get_mut(&player).unwrap().add(weapon(20, ItemKind::BodyArmor)).expect("slot");
        crate::plugins::inventory::equip_from_inventory(&mut world, player, 0);
        let after = effective_defense(&world, player);
        assert!(
            (after - before - super::armor_base_defense(ItemKind::BodyArmor, 1) as f32).abs() < 0.001,
            "plain armour should grant its base defence (before {before}, after {after})",
        );
    }

    #[test]
    fn two_rings_fill_both_fingers() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        world.inventory.get_mut(&player).unwrap().add(weapon(10, ItemKind::Ring)).expect("slot 0");
        world.inventory.get_mut(&player).unwrap().add(weapon(11, ItemKind::Ring)).expect("slot 1");
        // First ring → Ring1, second ring → Ring2 (no longer overwrites).
        crate::plugins::inventory::equip_from_inventory(&mut world, player, 0);
        crate::plugins::inventory::equip_from_inventory(&mut world, player, 1);
        let eq = world.equipment.get(&player).unwrap();
        assert!(eq.get(EquipSlot::Ring1).is_some(), "first ring on finger 1");
        assert!(eq.get(EquipSlot::Ring2).is_some(), "second ring on finger 2");
    }

    #[test]
    fn a_lone_weapon_or_ranged_offhand_grants_no_block() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);
        world.equipment.get_mut(&player).unwrap().equip(weapon(1, ItemKind::Sword)).unwrap();
        recompute_player_stats(&mut world, player);
        assert_eq!(world.stats.get(&player).unwrap().inner.resolved.block_chance, 0);
    }

    #[test]
    fn meets_requirements_uses_effective_attributes() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        let player = spawn_test_player(&mut world, ClassType::Soldat);

        let low = EquipRequirements { min_strength: 20, ..EquipRequirements::default() };
        let high = EquipRequirements { min_strength: 30, ..EquipRequirements::default() };
        assert!(meets_requirements(&world, player, &low));
        assert!(!meets_requirements(&world, player, &high));

        // Spending 10 stat points into STR (→ 30) lets the high-STR check pass.
        world.progression.get_mut(&player).unwrap().inner.stat_points = 10;
        for _ in 0..10 {
            spend_stat_point(&mut world, player, PrimaryAttr::Strength);
        }
        assert!(meets_requirements(&world, player, &high));
    }
}
