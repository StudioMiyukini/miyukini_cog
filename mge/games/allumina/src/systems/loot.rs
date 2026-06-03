//! Loot generation: rarity rolls, affix rolls, and ground-item spawning.
//!
//! Rarity distribution (per the design): Normal 70%, Magic 20%, Rare 8%,
//! Unique 2%.

use std::collections::HashSet;

use mge_items::affixes::{AffixKind, AffixMod, AppliedAffix, ItemAffixes};
use mge_items::item::{Item, ItemKind};
use mge_items::rarity::Rarity;

use crate::components::item::{GemKind, GroundItem, ItemInstance, SocketedGem};
use crate::components::world::ChestTier;
use crate::components::Position2D;
use crate::data::affixes::{AffixDef, AffixPool, AffixSlot, RARE_FIRST_WORDS, RARE_SECOND_WORDS};
use crate::data::items::base_item;
use crate::data::maps::zone_level_range;
use crate::ecs::{Entity, RenderEvent, World};
use crate::rng::Rng;

/// Maximum item level a drop can roll. Mirrors the level cap on gear.
const MAX_ITEM_LEVEL: u32 = 120;

/// Mob drops roll their item level around the mob's level with a ±2 variance,
/// clamped to `[1, MAX_ITEM_LEVEL]`. Bosses skip the variance and drop at
/// their exact level. The returned value is used both as the item's `ilvl`
/// (driving affix tiers and base scaling) and as its `level_req`.
#[must_use]
pub fn roll_drop_item_level(rng: &mut Rng, mob_level: u32, is_boss: bool) -> u32 {
    if is_boss {
        return mob_level.clamp(1, MAX_ITEM_LEVEL);
    }
    let delta = rng.range_u32(0, 4) as i32 - 2; // [-2, +2]
    let raw = mob_level as i32 + delta;
    raw.clamp(1, MAX_ITEM_LEVEL as i32) as u32
}

/// Rolled affixes plus the display words for each slot, in roll order.
struct RolledAffixes {
    affixes: ItemAffixes,
    prefix_words: Vec<String>,
    suffix_words: Vec<String>,
}

impl RolledAffixes {
    fn new() -> Self {
        Self { affixes: ItemAffixes::new(), prefix_words: Vec::new(), suffix_words: Vec::new() }
    }

    fn push(&mut self, slot: AffixSlot, applied: AppliedAffix, word: String) {
        match slot {
            AffixSlot::Prefix => {
                self.affixes.add_prefix(applied);
                self.prefix_words.push(word);
            }
            AffixSlot::Suffix => {
                self.affixes.add_suffix(applied);
                self.suffix_words.push(word);
            }
        }
    }

    fn len(&self) -> usize {
        self.affixes.prefixes.len() + self.affixes.suffixes.len()
    }
}

fn slot_to_kind(slot: AffixSlot) -> AffixKind {
    match slot {
        AffixSlot::Prefix => AffixKind::Prefix,
        AffixSlot::Suffix => AffixKind::Suffix,
    }
}

fn roll_rarity(world: &mut World) -> Rarity {
    match world.rng.next_u32() % 100 {
        0..=1 => Rarity::Unique,
        2..=9 => Rarity::Rare,
        10..=29 => Rarity::Magic,
        _ => Rarity::Normal,
    }
}

/// Rarity roll biased by a chest tier (Silver = +Magic, Gold = +Rare/Unique).
fn roll_rarity_for_chest(world: &mut World, tier: ChestTier) -> Rarity {
    let r = world.rng.next_u32() % 100;
    match tier {
        ChestTier::Normal => match r {
            0..=1 => Rarity::Unique,
            2..=9 => Rarity::Rare,
            10..=29 => Rarity::Magic,
            _ => Rarity::Normal,
        },
        // +20% Magic on top of baseline.
        ChestTier::Silver => match r {
            0..=2 => Rarity::Unique,
            3..=14 => Rarity::Rare,
            15..=64 => Rarity::Magic,
            _ => Rarity::Normal,
        },
        // +30% Rare + can roll a Unique.
        ChestTier::Gold => match r {
            0..=7 => Rarity::Unique,
            8..=47 => Rarity::Rare,
            48..=82 => Rarity::Magic,
            _ => Rarity::Normal,
        },
    }
}

/// Weighted-pick one affix of `slot` at `level`, skipping any used family, then
/// roll its value within the tier unlocked at `level`. Returns the applied
/// affix, its display word, and its family (for anti-duplication tracking).
fn pick_affix(
    pool: &AffixPool,
    rng: &mut Rng,
    slot: AffixSlot,
    level: u32,
    used: &HashSet<&'static str>,
) -> Option<(AppliedAffix, String, &'static str)> {
    let eligible: Vec<&AffixDef> =
        pool.eligible(slot, level).into_iter().filter(|d| !used.contains(d.family)).collect();
    let total: u32 = eligible.iter().map(|d| d.weight).sum();
    if total == 0 {
        return None;
    }
    let mut roll = rng.next_u32() % total;
    let mut chosen = eligible[0];
    for d in &eligible {
        if roll < d.weight {
            chosen = d;
            break;
        }
        roll -= d.weight;
    }
    let tier = chosen.tier_for(level)?;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let value = rng.range_u32(tier.min_value.round() as u32, tier.max_value.round() as u32) as i32;
    let applied =
        AppliedAffix { def_id: chosen.id, kind: slot_to_kind(slot), mods: chosen.stat.to_mods(value) };
    Some((applied, chosen.name.to_string(), chosen.family))
}

/// Magic item: 1-2 affixes (a prefix and/or a suffix).
fn roll_magic(pool: &AffixPool, rng: &mut Rng, level: u32) -> RolledAffixes {
    let mut rolled = RolledAffixes::new();
    let mut used: HashSet<&'static str> = HashSet::new();
    let both = rng.next_u32() % 2 == 0;
    let want_prefix = both || rng.next_u32() % 2 == 0;
    let want_suffix = both || !want_prefix;

    if want_prefix {
        if let Some((a, word, fam)) = pick_affix(pool, rng, AffixSlot::Prefix, level, &used) {
            used.insert(fam);
            rolled.push(AffixSlot::Prefix, a, word);
        }
    }
    if want_suffix {
        if let Some((a, word, fam)) = pick_affix(pool, rng, AffixSlot::Suffix, level, &used) {
            used.insert(fam);
            rolled.push(AffixSlot::Suffix, a, word);
        }
    }
    rolled
}

/// Rare item: 3-6 affixes (max 3 prefixes, max 3 suffixes), no family twice.
fn roll_rare(pool: &AffixPool, rng: &mut Rng, level: u32) -> RolledAffixes {
    let mut rolled = RolledAffixes::new();
    let mut used: HashSet<&'static str> = HashSet::new();
    let target = rng.range_u32(3, 6) as usize;

    let mut guard = 0;
    while rolled.len() < target && guard < 64 {
        guard += 1;
        let can_prefix = rolled.affixes.prefixes.len() < 3;
        let can_suffix = rolled.affixes.suffixes.len() < 3;
        let slot = match (can_prefix, can_suffix) {
            (true, true) => {
                if rng.next_u32() % 2 == 0 {
                    AffixSlot::Prefix
                } else {
                    AffixSlot::Suffix
                }
            }
            (true, false) => AffixSlot::Prefix,
            (false, true) => AffixSlot::Suffix,
            (false, false) => break,
        };
        if let Some((a, word, fam)) = pick_affix(pool, rng, slot, level, &used) {
            used.insert(fam);
            rolled.push(slot, a, word);
        }
    }
    rolled
}

fn rare_name(rng: &mut Rng, base: &str) -> String {
    let w1 = RARE_FIRST_WORDS[rng.pick_index(RARE_FIRST_WORDS.len()).unwrap_or(0)];
    let w2 = RARE_SECOND_WORDS[rng.pick_index(RARE_SECOND_WORDS.len()).unwrap_or(0)];
    format!("{w1} {w2} {base}")
}

/// Compose a magic name: "[prefix] Base [suffix]".
fn magic_name(rolled: &RolledAffixes, base: &str) -> String {
    let mut name = base.to_string();
    if let Some(p) = rolled.prefix_words.first() {
        name = format!("{p} {name}");
    }
    if let Some(s) = rolled.suffix_words.first() {
        name = format!("{name} {s}");
    }
    name
}

/// Generate one item instance of the given kind at the given item level.
pub fn generate_item(world: &mut World, kind: ItemKind, ilvl: u32, uid: u64) -> ItemInstance {
    let rarity = roll_rarity(world);
    generate_item_with_rarity(world, kind, ilvl, uid, rarity)
}

/// Like [`generate_item`] but with a caller-chosen rarity. Used by chests and
/// elite-pack drops, both of which override the default rarity distribution.
pub fn generate_item_with_rarity(
    world: &mut World,
    kind: ItemKind,
    ilvl: u32,
    uid: u64,
    rarity: Rarity,
) -> ItemInstance {
    let base = base_item(kind, ilvl);
    let mut item = Item::new(uid, base.clone(), rarity, ilvl);
    item.identified = true;

    let mut rolled = RolledAffixes::new();
    let mut extra_mods: Vec<AffixMod> = Vec::new();
    let mut name = base.name.clone();
    let mut unique_id: Option<u32> = None;
    let mut flavour: Option<String> = None;

    match rarity {
        Rarity::Normal => {}
        Rarity::Magic => {
            rolled = roll_magic(&world.data.affix_pool, &mut world.rng, ilvl);
            name = magic_name(&rolled, &base.name);
        }
        Rarity::Rare | Rarity::Crafted | Rarity::Set => {
            rolled = roll_rare(&world.data.affix_pool, &mut world.rng, ilvl);
            name = rare_name(&mut world.rng, &base.name);
        }
        Rarity::Unique => {
            // Pick a named unique whose level_req sits in [ilvl-5, ilvl+5].
            let pick = world
                .data
                .pick_unique(kind, ilvl, &mut world.rng)
                .map(|u| (u.id, u.name.clone(), u.mods.clone(), u.flavour.clone()));
            if let Some((uid, uname, mods, uflavour)) = pick {
                extra_mods = mods
                    .iter()
                    .map(|m| AffixMod { stat: m.stat, modifier: m.modifier })
                    .collect();
                name = uname;
                unique_id = Some(uid);
                flavour = (!uflavour.is_empty()).then_some(uflavour);
            } else {
                // No named unique in the band: downgrade to a Rare.
                item.rarity = Rarity::Rare;
                rolled = roll_rare(&world.data.affix_pool, &mut world.rng, ilvl);
                name = rare_name(&mut world.rng, &base.name);
            }
        }
    }

    let mut instance = ItemInstance::new(item, rolled.affixes, extra_mods, name);
    instance.prefix_words = rolled.prefix_words;
    instance.suffix_words = rolled.suffix_words;
    instance.unique_id = unique_id;
    instance.flavour = flavour;
    // Roll random socket slots on socketable gear (weighted toward fewer: the
    // smaller of two rolls in `0..=max_sockets`).
    if instance.item.can_be_socketed() {
        let max = u32::from(instance.item.base.max_sockets);
        let a = world.rng.range_u32(0, max);
        let b = world.rng.range_u32(0, max);
        instance.sockets = a.min(b) as u8;
    }
    instance
}

/// Build a droppable gem: a random family (Diamant, Rubis, … Zirconia) whose
/// quality tier (éclat … parfait) climbs with the slain mob's level, with luck.
/// The gem's effect is contextual (weapon / armour / shield), resolved when it
/// is socketed — see [`crate::components::item::gem_mods`].
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn gem_drop(rng: &mut Rng, mob_level: u32, uid: u64) -> ItemInstance {
    // Quality: deeper zones + a lucky roll push toward the perfect tier.
    let q = rng.range_u32(0, 99) + mob_level.min(80);
    let tier = match q {
        0..=39 => 0u8,
        40..=79 => 1,
        80..=119 => 2,
        120..=149 => 3,
        _ => 4,
    };
    let kind = GemKind::ALL[rng.pick_index(GemKind::ALL.len()).unwrap_or(0)];
    let spec = SocketedGem { kind, tier };

    let ilvl = mob_level.max(1);
    let item = Item::new(uid, base_item(ItemKind::Gem, ilvl), Rarity::Magic, ilvl);
    let name = spec.display_name();
    let mut instance = ItemInstance::new(item, ItemAffixes::new(), Vec::new(), name);
    instance.gem = Some(spec);
    instance
}

/// Drain the death-drop queue and spawn loot.
#[allow(clippy::cast_possible_truncation)]
pub fn loot_system(world: &mut World) {
    let drops = std::mem::take(&mut world.loot_queue);
    if drops.is_empty() {
        return;
    }

    for drop in drops {
        let Some(def) = world.data.mob(drop.mob_def_id) else { continue };
        let is_boss = def.is_boss;
        let drop_chance = def.drop_chance;
        let gold_min = def.gold_min as u32;
        let gold_max = def.gold_max as u32;
        let kinds = def.item_kinds.clone();
        let mob_name = def.name.clone();
        let mob_level = drop.mob_level.max(1);

        // Gold.
        let gold = u64::from(world.rng.range_u32(gold_min, gold_max));
        if gold > 0 {
            if let Some(player) = world.player_entity {
                if let Some(inv) = world.inventory.get_mut(&player) {
                    inv.add_gold(gold);
                }
            }
            world.log_event(format!("{mob_name} dropped {gold} gold"));
        }

        // Items. Bosses always drop three; elites get a guaranteed item plus
        // a coin-flip second one on top of the usual roll.
        let n_items = if is_boss {
            3
        } else if drop.elite {
            1 + u32::from(world.rng.chance_pct(drop_chance.max(50)))
        } else {
            u32::from(world.rng.chance_pct(drop_chance))
        };
        for _ in 0..n_items {
            let Some(idx) = world.rng.pick_index(kinds.len()) else { break };
            let kind = kinds[idx];
            let entity = world.spawn();
            let item_level = roll_drop_item_level(&mut world.rng, mob_level, is_boss);
            let mut instance = generate_item(world, kind, item_level, entity.0);
            instance.item.base.requirements.min_level = item_level;

            let rarity = instance.rarity().color_name();
            let label = format!("{} [{}]", instance.name, rarity);
            let ox = (world.rng.next_f32() - 0.5) * 1.5;
            let oy = (world.rng.next_f32() - 0.5) * 1.5;
            let pos = Position2D::new(drop.pos.x + ox, drop.pos.y + oy);
            world.positions.insert(entity, pos);
            world.ground_items.insert(entity, GroundItem { instance, life: 6000 });
            world.render_events.push(RenderEvent::Loot { pos, rarity });
            world.log_event(format!("{mob_name} dropped {label}"));
        }

        // Gems: a chance to drop a socketable gem (better odds for elites/bosses).
        let gem_chance = if is_boss { 60 } else if drop.elite { 35 } else { 12 };
        if world.rng.chance_pct(gem_chance) {
            let entity = world.spawn();
            let gem = gem_drop(&mut world.rng, mob_level, entity.0);
            let label = gem.name.clone();
            let ox = (world.rng.next_f32() - 0.5) * 1.5;
            let oy = (world.rng.next_f32() - 0.5) * 1.5;
            let pos = Position2D::new(drop.pos.x + ox, drop.pos.y + oy);
            world.positions.insert(entity, pos);
            world.ground_items.insert(entity, GroundItem { instance: gem, life: 6000 });
            world.render_events.push(RenderEvent::Loot { pos, rarity: "blue" });
            world.log_event(format!("{mob_name} dropped {label}"));
        }
    }
}

/// Open a chest entity and drop its loot on the floor next to it. Idempotent:
/// once a chest is flagged `opened` it can't be re-rolled. The chest entity
/// stays around (so the renderer can keep showing the lid open) but its
/// position is preserved.
pub fn open_chest(world: &mut World, chest_e: Entity) -> bool {
    let Some(chest) = world.chests.get(&chest_e).copied() else {
        return false;
    };
    if chest.opened {
        return false;
    }
    let tier = chest.tier;
    // Chests roll at the zone's midpoint level so they're neither trivially
    // under-leveled (zone floor) nor punishingly over-leveled (zone ceiling).
    let (lo, hi) = zone_level_range(world.zone);
    let zone_mid = ((lo + hi) / 2).max(1);
    let (min_n, max_n) = tier.item_count();
    let count = world.rng.range_u32(min_n, max_n);

    // Item kinds available for chest loot — same pool as common mob drops.
    let kinds = [
        ItemKind::Sword,
        ItemKind::Bow,
        ItemKind::Wand,
        ItemKind::Helm,
        ItemKind::BodyArmor,
        ItemKind::Boots,
        ItemKind::Gloves,
        ItemKind::Belt,
        ItemKind::Ring,
        ItemKind::Amulet,
        ItemKind::Shield,
    ];
    let pos = Position2D::new(chest.x, chest.y);
    for _ in 0..count {
        let Some(idx) = world.rng.pick_index(kinds.len()) else { break };
        let kind = kinds[idx];
        let rarity = roll_rarity_for_chest(world, tier);
        let item_e = world.spawn();
        let mut instance = generate_item_with_rarity(world, kind, zone_mid, item_e.0, rarity);
        instance.item.base.requirements.min_level = zone_mid;
        let rarity_name = instance.rarity().color_name();
        let label = format!("{} [{}]", instance.name, rarity_name);
        let ox = (world.rng.next_f32() - 0.5) * 1.5;
        let oy = (world.rng.next_f32() - 0.5) * 1.5;
        let drop_pos = Position2D::new(pos.x + ox, pos.y + oy);
        world.positions.insert(item_e, drop_pos);
        world.ground_items.insert(item_e, GroundItem { instance, life: 6000 });
        world.render_events.push(RenderEvent::Loot { pos: drop_pos, rarity: rarity_name });
        world.log_event(format!("Chest dropped {label}"));
    }
    if let Some(c) = world.chests.get_mut(&chest_e) {
        c.opened = true;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::affixes::affix_pool;

    fn family_of(pool: &AffixPool, def_id: u32) -> &'static str {
        pool.defs().iter().find(|d| d.id == def_id).unwrap().family
    }

    #[test]
    fn magic_rolls_one_or_two_affixes() {
        let pool = affix_pool();
        for seed in 0..300u64 {
            let mut rng = Rng::new(seed);
            let rolled = roll_magic(&pool, &mut rng, 25);
            let n = rolled.len();
            assert!((1..=2).contains(&n), "magic count {n} (seed {seed})");
            assert!(rolled.affixes.prefixes.len() <= 1);
            assert!(rolled.affixes.suffixes.len() <= 1);
            assert_eq!(rolled.prefix_words.len(), rolled.affixes.prefixes.len());
            assert_eq!(rolled.suffix_words.len(), rolled.affixes.suffixes.len());
        }
    }

    #[test]
    fn rare_rolls_three_to_six_affixes() {
        let pool = affix_pool();
        for seed in 0..300u64 {
            let mut rng = Rng::new(seed);
            let rolled = roll_rare(&pool, &mut rng, 40);
            let n = rolled.len();
            assert!((3..=6).contains(&n), "rare count {n} (seed {seed})");
            assert!(rolled.affixes.prefixes.len() <= 3, "too many prefixes (seed {seed})");
            assert!(rolled.affixes.suffixes.len() <= 3, "too many suffixes (seed {seed})");
        }
    }

    #[test]
    fn rare_never_repeats_a_family() {
        let pool = affix_pool();
        for seed in 0..300u64 {
            let mut rng = Rng::new(seed);
            let rolled = roll_rare(&pool, &mut rng, 40);
            let mut fams: Vec<&str> = rolled
                .affixes
                .prefixes
                .iter()
                .chain(rolled.affixes.suffixes.iter())
                .map(|a| family_of(&pool, a.def_id))
                .collect();
            let count = fams.len();
            fams.sort_unstable();
            fams.dedup();
            assert_eq!(fams.len(), count, "duplicate family (seed {seed})");
        }
    }

    #[test]
    fn magic_name_brackets_the_base() {
        let pool = affix_pool();
        // Seed 0 yields both a prefix and a suffix; verify the "[pre] Base [suf]" shape.
        let mut rng = Rng::new(0);
        let rolled = roll_magic(&pool, &mut rng, 25);
        let name = magic_name(&rolled, "Long Sword");
        assert!(name.contains("Long Sword"));
        if let Some(p) = rolled.prefix_words.first() {
            assert!(name.starts_with(p), "prefix not leading: {name}");
        }
        if let Some(s) = rolled.suffix_words.first() {
            assert!(name.ends_with(s), "suffix not trailing: {name}");
        }
    }

    #[test]
    fn rare_name_has_two_words_then_base() {
        let mut rng = Rng::new(3);
        let name = rare_name(&mut rng, "Long Sword");
        assert!(name.ends_with("Long Sword"));
        let head = name.strip_suffix(" Long Sword").unwrap();
        assert_eq!(head.split(' ').count(), 2, "rare prefix should be two words: {name}");
    }

    #[test]
    fn mob_drop_level_stays_within_two_of_mob_level() {
        for seed in 0..200u64 {
            let mut rng = Rng::new(seed);
            let lvl = roll_drop_item_level(&mut rng, 20, false);
            assert!((18..=22).contains(&lvl), "mob lvl 20 produced item lvl {lvl} (seed {seed})");
        }
    }

    #[test]
    fn boss_drop_level_equals_mob_level_exactly() {
        for seed in 0..50u64 {
            let mut rng = Rng::new(seed);
            assert_eq!(roll_drop_item_level(&mut rng, 30, true), 30);
        }
    }

    #[test]
    fn low_level_mob_drop_clamped_to_one() {
        for seed in 0..200u64 {
            let mut rng = Rng::new(seed);
            let lvl = roll_drop_item_level(&mut rng, 1, false);
            assert!(lvl >= 1, "mob lvl 1 produced item lvl {lvl} (seed {seed})");
            // Variance is ±2, so the ceiling is mob_level + 2 = 3.
            assert!(lvl <= 3, "mob lvl 1 produced unexpectedly high item lvl {lvl} (seed {seed})");
        }
    }

    #[test]
    fn drop_level_clamped_to_max_item_level() {
        for seed in 0..50u64 {
            let mut rng = Rng::new(seed);
            let lvl = roll_drop_item_level(&mut rng, 121, false);
            assert!(lvl <= MAX_ITEM_LEVEL, "item lvl {lvl} > cap (seed {seed})");
        }
    }

    #[test]
    fn dropped_item_level_req_matches_mob_level() {
        use crate::components::mob::AiBehavior;
        use crate::data::maps::plains_map;
        use crate::data::mobs::MobDef;
        use crate::data::GameData;
        use crate::ecs::{DeathDrop, World};
        use mge_items::item::ItemKind;

        // Synthetic mob def: level 20, guaranteed drop, single kind.
        let def = MobDef {
            id: 9_999,
            name: "Test Mob".into(),
            behavior: AiBehavior::Melee,
            level: 20,
            max_hp: 1.0,
            damage_min: 0,
            damage_max: 0,
            attack_rating: 0,
            defense: 0,
            xp_reward: 0,
            aggro: 0.0,
            leash: 0.0,
            attack_range: 1.0,
            attack_interval: 1.0,
            drop_chance: 100,
            gold_min: 0,
            gold_max: 0,
            item_kinds: vec![ItemKind::Sword],
            is_boss: false,
            resistances: [0.0; 4],
        };

        // Try several seeds; with 100% drop_chance most will produce items.
        let mut checked = 0;
        for seed in 0..16u64 {
            let mut world = World::new(plains_map(), GameData::load(), seed);
            world.data.mobs.push(def.clone());
            world.loot_queue.push(DeathDrop {
                pos: Position2D::new(5.0, 5.0),
                mob_def_id: 9_999,
                mob_level: 20,
                elite: false,
            });
            loot_system(&mut world);

            for (_, g) in world.ground_items.iter() {
                // Gems aren't gear with attribute/level requirements.
                if g.instance.is_gem() {
                    continue;
                }
                let req = g.instance.item.base.requirements.min_level;
                assert!(
                    (18..=22).contains(&req),
                    "dropped item level_req {req} not within mob lvl 20 ±2 (seed {seed})"
                );
                assert_eq!(g.instance.item.ilvl, req, "ilvl mismatches level_req (seed {seed})");
                checked += 1;
            }
        }
        assert!(checked > 0, "no items dropped across all seeds");
    }

    #[test]
    fn socketable_gear_frequently_rolls_sockets() {
        use crate::data::maps::plains_map;
        use crate::data::GameData;
        use crate::ecs::World;
        let mut world = World::new(plains_map(), GameData::load(), 7);
        let (mut socketed, total) = (0u32, 200u64);
        for i in 0..total {
            let it = generate_item_with_rarity(&mut world, ItemKind::Sword, 20, 1000 + i, Rarity::Normal);
            assert!(it.item.can_be_socketed(), "a sword must be socketable");
            if it.sockets > 0 {
                socketed += 1;
            }
        }
        // Min-of-two over 0..=3 ⇒ roughly half of socketable gear gets a socket.
        assert!(socketed as u64 > total / 3, "too few socketed swords: {socketed}/{total}");
    }

    #[test]
    fn open_chest_drops_loot_then_locks() {
        use crate::components::player::ClassType;
        use crate::components::world::{Chest, ChestTier};
        use crate::data::maps::plains_map;
        use crate::data::GameData;
        use crate::ecs::World;
        use crate::plugins::player::spawn_player_system;

        let mut world = World::new(plains_map(), GameData::load(), 99);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);

        let chest_e = world.spawn();
        let chest = Chest { x: 12.5, y: 12.5, tier: ChestTier::Silver, opened: false };
        world.positions.insert(chest_e, Position2D::new(chest.x, chest.y));
        world.chests.insert(chest_e, chest);

        let before = world.ground_items.len();
        assert!(open_chest(&mut world, chest_e));
        assert!(world.chests[&chest_e].opened);
        assert!(world.ground_items.len() > before, "chest should drop ground items");

        // Re-opening is a no-op.
        let after_first = world.ground_items.len();
        assert!(!open_chest(&mut world, chest_e));
        assert_eq!(world.ground_items.len(), after_first);
    }
}
