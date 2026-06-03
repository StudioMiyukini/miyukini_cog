//! Combat resolution using the Allumina (Diablo-like) damage formulas:
//!
//! * `ATK   = (base_dmg + weapon_dmg) * (1 + primary_stat / 100)`
//! * `DEF reduction = DEF / (DEF + 50 * attacker_level)`
//! * `Dodge        = ESQ / (ESQ + 100)`
//! * Crit: 5% base chance, 150% damage.

use mge_items::item::{EquipSlot, ItemKind};

use crate::components::cc::{Knockdown, Pushback};
use crate::components::item::{is_magic_ranged, is_one_handed_melee, is_ranged};
use crate::components::item::ItemInstance;
use crate::components::mercenary::MercType;
use crate::components::mob::AiBehavior;
use crate::components::skill::Element;
use crate::components::Position2D;
use crate::data::classes::{class_def, PrimaryStat};
use crate::ecs::{Entity, RenderEvent, World};
use crate::render::figures::Direction;
use crate::systems::projectile::spawn_projectile;

/// Classification of an entity's ranged-attack mode at attack time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackerKind {
    /// Standard melee swing (resolved instantly).
    Melee,
    /// Physical projectile (bow/crossbow, or `AiBehavior::Ranged` mob).
    Ranged,
    /// Magic projectile (staff/wand/scepter, or `AiBehavior::Caster` mob).
    MagicRanged,
}

/// Decide whether `e` swings in melee or fires a projectile.
///
/// * Players follow their equipped main-hand weapon's [`ItemKind`].
/// * Mobs follow their [`AiBehavior`]: `Ranged` → physical, `Caster` → magic.
/// * Anything else defaults to melee.
#[must_use]
pub fn attacker_kind(world: &World, e: Entity) -> AttackerKind {
    if let Some(weapon) = world
        .equipment
        .get(&e)
        .and_then(|eq| eq.get(EquipSlot::MainHand))
    {
        let kind: ItemKind = weapon.item.base.kind;
        if is_magic_ranged(kind) {
            return AttackerKind::MagicRanged;
        }
        if is_ranged(kind) {
            return AttackerKind::Ranged;
        }
        return AttackerKind::Melee;
    }
    if let Some(a) = world.aggro.get(&e) {
        return match a.behavior {
            AiBehavior::Ranged => AttackerKind::Ranged,
            AiBehavior::Caster => AttackerKind::MagicRanged,
            AiBehavior::Melee => AttackerKind::Melee,
        };
    }
    if let Some(tag) = world.mercenary_tags.get(&e) {
        return match tag.merc_type {
            MercType::Archer => AttackerKind::Ranged,
            MercType::Mage => AttackerKind::MagicRanged,
            MercType::Guardian | MercType::Healer => AttackerKind::Melee,
        };
    }
    AttackerKind::Melee
}

// Combat tuning constants live in `systems::balance` (single source of truth).
use crate::systems::balance::{
    AR_BASE, AR_PER_DEX, AR_PER_LEVEL, CRIT_BASE_PCT, CRIT_MULT, DEF_REDUCTION_CAP, DEF_VS_AR,
    DODGE_CAP, DUAL_WIELD_IAS, ESQ_PER_DEX, PRIMARY_DMG_DIVISOR, REFLECT_AR,
};

/// Flattened attacker numbers, copied out of the world before rolling.
#[derive(Debug, Clone, Copy)]
struct AttackerStats {
    dmg_min: f32,
    dmg_max: f32,
    weapon_dmg: f32,
    primary: f32,
    crit_pct: u32,
    /// Attack Rating — drives both the defender's dodge and DEF mitigation.
    attack_rating: f32,
    /// Fraction of the target's Defense stripped before mitigation (Zirconia
    /// gems socketed in the attacker's weapon(s)). 0 for unarmed/mobs.
    target_def_pct: f32,
}

/// Flattened defender numbers.
#[derive(Debug, Clone, Copy)]
struct DefenderStats {
    defense: f32,
    dodge: f32,
    /// Block chance in `[0, 1]` (a shield or dual-wield parry negating the hit).
    block: f32,
}

/// The result of one resolved swing.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttackOutcome {
    Dodged,
    /// A shield / dual-wield parry negated the hit entirely.
    Blocked,
    Hit { damage: f32, crit: bool },
}

/// Swing-rate multiplier for `e`: folds worn gear's "% Attack Speed" affixes,
/// "Wargh !"/order IAS, the Clerc's Aura du vent, and the dual-wield bonus.
/// `interval / swing_speed` gives the seconds between swings.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn swing_speed(world: &World, e: Entity) -> f32 {
    let gear = world.stats.get(&e).map_or(0.0, |s| s.inner.resolved.attack_speed as f32 / 100.0);
    let aura = world.aura_bonuses.get(&e).map_or(0.0, |a| a.atk_speed_pct);
    // Dual-wielding two one-handed weapons swings faster (the off-hand fills
    // the gaps between main-hand strikes).
    let dual = if is_dual_wield_melee(world, e) { DUAL_WIELD_IAS } else { 0.0 };
    (1.0 + world.order_bonuses(e).atk_speed_pct + aura + gear + dual).max(0.1)
}

/// The item worn in `slot`, sourced from either the player's `equipment` or a
/// mercenary's `merc_inventory`, so combat reads both alike.
#[must_use]
fn equipped_weapon(world: &World, e: Entity, slot: EquipSlot) -> Option<&ItemInstance> {
    world
        .equipment
        .get(&e)
        .and_then(|eq| eq.get(slot))
        .or_else(|| world.merc_inventory.get(&e).and_then(|inv| inv.get(slot)))
}

/// The item *kind* worn in `slot` (from the player's `equipment` or a merc's
/// `merc_inventory`). Used by the network layer to replicate appearance.
#[must_use]
pub fn equipped_kind(world: &World, e: Entity, slot: EquipSlot) -> Option<ItemKind> {
    equipped_weapon(world, e, slot).map(|i| i.item.base.kind)
}

/// Whether `e` wields a one-handed melee weapon in *both* hands (the off-hand
/// slot holding a weapon rather than a shield). Works for the player and for
/// dual-wield mercenaries (e.g. the Guardian).
#[must_use]
pub fn is_dual_wield_melee(world: &World, e: Entity) -> bool {
    let one_h = |slot| equipped_weapon(world, e, slot).is_some_and(|i| is_one_handed_melee(i.item.base.kind));
    one_h(EquipSlot::MainHand) && one_h(EquipSlot::OffHand)
}

/// The ilvl-based base damage a single weapon contributes (mirrors the player
/// formula `4 + ilvl*1.5`).
fn weapon_base_dmg(w: &ItemInstance) -> f32 {
    4.0 + w.ilvl() as f32 * 1.5
}

/// Total target-Defense reduction from Zirconia gems in `e`'s weapon(s).
/// Both hands contribute (a dual-wielder sunders with whichever it swings, but
/// summing keeps the model simple and rewards socketing both). Capped at 75%.
#[must_use]
fn weapon_sunder_pct(world: &World, e: Entity) -> f32 {
    let hand = |slot| equipped_weapon(world, e, slot).map_or(0.0, ItemInstance::sunder_pct);
    (hand(EquipSlot::MainHand) + hand(EquipSlot::OffHand)).min(0.75)
}

/// Sum of a weapon's flat `+DamageMin/Max` affixes — used to peel the *resting*
/// weapon's rolls off the resolved range so two weapons don't pool their affixes.
#[allow(clippy::cast_precision_loss)]
fn flat_damage_affixes(w: &ItemInstance) -> (f32, f32) {
    let (mn, mx) = w.flat_damage_bonus();
    (mn as f32, mx as f32)
}

#[allow(clippy::cast_precision_loss)]
fn gather_attacker(world: &World, e: Entity) -> AttackerStats {
    if let Some(stats) = world.stats.get(&e) {
        let resolved = &stats.inner.resolved;
        // Effective attributes already fold in equipment Flat modifiers via
        // `CharacterStats::recompute`; never sum gear mods on top of resolved
        // fields here or we'd double-count.
        let eff = stats.inner.effective_attributes();

        let primary = match world.classes.get(&e).map(|c| class_def(*c).primary) {
            Some(PrimaryStat::Strength) => eff.strength,
            Some(PrimaryStat::Dexterity) => eff.dexterity,
            Some(PrimaryStat::Energy) => eff.energy,
            None => 0,
        } as f32;

        // Weapon damage. The resolved range is the entity's full damage *without*
        // any weapon base (the player's STR/affix range; a merc's profile/affix
        // range — neither bakes the weapon term in). `weapon_dmg` adds the
        // swinging weapon's ilvl term. Dual-wielders alternate hands and peel the
        // *resting* weapon's flat `+Damage` affixes off the range, so each weapon
        // strikes for its own numbers while shared gear bonuses still apply.
        let (dmg_min, dmg_max, weapon_dmg) = if is_dual_wield_melee(world, e) {
            let off_hand = world.dual_hand.get(&e).copied().unwrap_or(false);
            let (swing, rest) = if off_hand {
                (EquipSlot::OffHand, EquipSlot::MainHand)
            } else {
                (EquipSlot::MainHand, EquipSlot::OffHand)
            };
            let (rest_min, rest_max) =
                equipped_weapon(world, e, rest).map_or((0.0, 0.0), flat_damage_affixes);
            let mn = (resolved.damage_min as f32 - rest_min).max(1.0);
            let mx = (resolved.damage_max as f32 - rest_max).max(mn);
            let wd = equipped_weapon(world, e, swing).map_or(0.0, weapon_base_dmg);
            (mn, mx, wd)
        } else {
            let wd = equipped_weapon(world, e, EquipSlot::MainHand).map_or(0.0, weapon_base_dmg);
            (resolved.damage_min as f32, resolved.damage_max as f32, wd)
        };

        let crit_pct = (CRIT_BASE_PCT + resolved.crit_chance).max(0) as u32
            + world.hunter_instinct.get(&e).map_or(0, |b| b.crit_pct);

        AttackerStats {
            dmg_min,
            dmg_max,
            weapon_dmg,
            primary,
            crit_pct,
            attack_rating: attack_rating(world, e),
            target_def_pct: weapon_sunder_pct(world, e),
        }
    } else if let Some(def) =
        world.mob_types.get(&e).and_then(|m| world.data.mob(m.def_id))
    {
        AttackerStats {
            dmg_min: def.damage_min as f32,
            dmg_max: def.damage_max as f32,
            weapon_dmg: 0.0,
            primary: 0.0,
            crit_pct: CRIT_BASE_PCT.max(0) as u32,
            attack_rating: def.attack_rating.max(0) as f32,
            target_def_pct: 0.0,
        }
    } else {
        let crit_pct = world.hunter_instinct.get(&e).map_or(0, |b| b.crit_pct);
        AttackerStats {
            dmg_min: 1.0,
            dmg_max: 2.0,
            weapon_dmg: 0.0,
            primary: 0.0,
            crit_pct,
            attack_rating: AR_BASE,
            target_def_pct: 0.0,
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn gather_defender(world: &World, e: Entity) -> DefenderStats {
    let mut stats = if let Some(stats) = world.stats.get(&e) {
        // Resolved already includes equipment after `recompute()`; reading
        // `all_modifiers()` here would double-count gear.
        let defense = stats.inner.resolved.defense as f32;
        // Avoidance rating from Dexterity (mobs carry none and so never dodge,
        // keeping player swings reliable). Contested against the attacker's AR.
        let dodge = stats.inner.effective_attributes().dexterity.max(0) as f32 * ESQ_PER_DEX;
        // Block chance was folded into `resolved.block_chance` by the stats
        // recompute (shield / dual-wield + DEX + gear affixes).
        let block = (stats.inner.resolved.block_chance.max(0) as f32 / 100.0).min(1.0);
        DefenderStats { defense, dodge, block }
    } else if let Some(def) = world.mob_types.get(&e).and_then(|m| world.data.mob(m.def_id)) {
        DefenderStats { defense: def.defense as f32, dodge: 0.0, block: 0.0 }
    } else {
        DefenderStats { defense: 0.0, dodge: 0.0, block: 0.0 }
    };
    // A defence-reduction debuff (e.g. a poisoned arrow) strips a fraction of DEF.
    if let Some(d) = world.def_debuffs.get(&e) {
        stats.defense *= (1.0 - d.pct).clamp(0.0, 1.0);
    }
    // Battle orders shift defence: "Tenez bon !" braces it, "Wargh !" sacrifices it.
    stats.defense *= (1.0 + world.order_bonuses(e).def_pct).max(0.0);
    // The Mage's "Armure de givre" raises defence while active.
    if let Some(fa) = world.frost_armor.get(&e) {
        stats.defense *= 1.0 + fa.def_pct;
    }
    // The Clerc's defensive aura raises defence while in range.
    if let Some(ab) = world.aura_bonuses.get(&e) {
        stats.defense *= 1.0 + ab.def_pct;
    }
    // The Archer's "Instinct du chasseur" adds flat dodge points.
    if let Some(hi) = world.hunter_instinct.get(&e) {
        stats.dodge += hi.dodge_pts;
    }
    stats
}

/// A defender's resistance fraction against `element`. Physical attacks are
/// never resisted here (they use defence reduction instead). The returned
/// value is the raw resistance — `1.0` means immunity and `< 0` means
/// vulnerability — and is *not* clamped here so the caller can decide how to
/// interpret the special cases.
#[must_use]
fn elem_resist_for(world: &World, e: Entity, element: Element) -> f32 {
    world
        .elem_resists
        .get(&e)
        .map_or(0.0, |r| match element {
            Element::Fire => r.fire,
            Element::Frost => r.frost,
            Element::Lightning => r.lightning,
            Element::Poison => r.poison,
            Element::Physical => 0.0,
        })
}

/// Apply the mitigation pipeline to a raw damage `amount`: attacker-level-scaled
/// defence reduction, then a damage-reduction blessing, then elemental
/// resistance. Floored at 1 so any landed hit registers.
///
/// `elem_resist == 1.0` is treated as full immunity: the call returns `0.0`,
/// bypassing the usual floor. Vulnerabilities (negative `elem_resist`) amplify
/// damage (e.g. `-0.25` → ×1.25).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn mitigate(amount: f32, defense: f32, attack_rating: f32, resist: f32, elem_resist: f32) -> f32 {
    if elem_resist >= 1.0 {
        return 0.0;
    }
    let reduction = defense_reduction(defense, attack_rating);
    let elem_factor = (1.0 - elem_resist).max(0.0);
    (amount * (1.0 - reduction) * (1.0 - resist.clamp(0.0, 0.9)) * elem_factor).max(1.0)
}

/// Physical damage-reduction fraction granted by `defense` against an attacker
/// of the given `attack_rating`: `DEF / (DEF + DEF_VS_AR*AR)`, capped. A higher
/// AR shrinks the reduction (the strike lands more squarely).
#[must_use]
pub fn defense_reduction(defense: f32, attack_rating: f32) -> f32 {
    let ar = attack_rating.max(1.0);
    let denom = defense + DEF_VS_AR * ar;
    if denom > 0.0 { (defense / denom).clamp(0.0, DEF_REDUCTION_CAP) } else { 0.0 }
}

/// Dodge probability for a defender with avoidance `esq` against an attacker of
/// `attack_rating`: `ESQ / (ESQ + AR)`, capped at [`DODGE_CAP`].
#[must_use]
pub fn dodge_from(esq: f32, attack_rating: f32) -> f32 {
    let e = esq.max(0.0);
    let ar = attack_rating.max(1.0);
    if e + ar <= 0.0 { 0.0 } else { (e / (e + ar)).clamp(0.0, DODGE_CAP) }
}

/// Resolve a flat *elemental* spell hit (Mage de guerre branches). Unlike
/// [`resolve_attack`] this applies a flat `base` amount rather than a weapon
/// multiple, cannot be dodged, but still rolls crit and runs the full
/// [`mitigate`] pipeline (defence + blessing + elemental resistance).
#[allow(clippy::cast_precision_loss)]
pub fn resolve_spell(
    world: &mut World,
    attacker: Entity,
    defender: Entity,
    base: f32,
    element: Element,
) -> AttackOutcome {
    record_last_enemy(world, attacker, defender);
    let target_pos = world.positions.get(&defender).copied().unwrap_or(Position2D::new(0.0, 0.0));

    // Invulnerable defenders (e.g. mid-dodge) take nothing.
    if world.invuln.get(&defender).is_some_and(|&t| t > 0.0) {
        world.render_events.push(RenderEvent::Damage {
            attacker,
            target: defender,
            pos: target_pos,
            amount: 0.0,
            crit: false,
            dodged: true,
        });
        return AttackOutcome::Dodged;
    }

    let att = gather_attacker(world, attacker);
    let def = gather_defender(world, defender);

    let crit = world.rng.chance_pct(att.crit_pct);
    let aura_dmg = 1.0 + world.aura_bonuses.get(&attacker).map_or(0.0, |a| a.dmg_pct);
    let scaled = base * aura_dmg;
    let after_crit = if crit { scaled * CRIT_MULT } else { scaled };
    let blessing = world.resist.get(&defender).map_or(0.0, |r| r.pct);
    let aura_resist = world.aura_bonuses.get(&defender).map_or(0.0, |a| a.resist_pct);
    let resist = (blessing + aura_resist).clamp(0.0, 0.9);
    let elem = elem_resist_for(world, defender, element);
    // A knocked-down defender takes amplified damage (+20%).
    let knock_amp = world.knockdowns.get(&defender).map_or(1.0, |k| k.dmg_amp as f32);
    // Zirconia weapon gems strip part of the target's Defense before mitigation.
    let pierced_def = def.defense * (1.0 - att.target_def_pct);
    let damage = mitigate(after_crit, pierced_def, att.attack_rating, resist, elem) * knock_amp;

    if let Some(h) = world.health.get_mut(&defender) {
        h.drain(damage);
    }
    world.render_events.push(RenderEvent::Damage {
        attacker,
        target: defender,
        pos: target_pos,
        amount: damage,
        crit,
        dodged: false,
    });
    // Any landed damage interrupts an in-progress cast (unless Unstoppable).
    try_interrupt(world, defender);
    // Vengeance and Pardon respond to spell damage too.
    reflect_vengeance(world, attacker, defender, damage);
    pardon_leech(world, attacker, defender);
    AttackOutcome::Hit { damage, crit }
}

/// If `defender` is bathed in the Clerc's Aura de vengeance, reflect a fraction
/// of the damage taken back to `attacker`. Mitigated by the attacker's
/// defence; bypasses crit/dodge so it never amplifies into oblivion.
fn reflect_vengeance(world: &mut World, attacker: Entity, defender: Entity, damage: f32) {
    if attacker == defender {
        return;
    }
    let pct = world.aura_bonuses.get(&defender).map_or(0.0, |a| a.retaliate_pct);
    if pct <= 0.0 || damage <= 0.0 {
        return;
    }
    if world.health.get(&attacker).map_or(true, |h| h.is_empty()) {
        return;
    }
    let raw = damage * pct;
    let amount = mitigate(raw, effective_defense(world, attacker), REFLECT_AR, 0.0, 0.0);
    if let Some(h) = world.health.get_mut(&attacker) {
        h.drain(amount);
    }
    let apos = world.positions.get(&attacker).copied().unwrap_or(Position2D::new(0.0, 0.0));
    world.render_events.push(RenderEvent::Damage {
        attacker: defender,
        target: attacker,
        pos: apos,
        amount,
        crit: false,
        dodged: false,
    });
}

/// If `defender` carries the Clerc's "Pardon" mark, `attacker` heals for a
/// fraction of the mark's max HP. No-op on self-hits.
fn pardon_leech(world: &mut World, attacker: Entity, defender: Entity) {
    if attacker == defender {
        return;
    }
    let Some(p) = world.pardoned.get(&defender).copied() else { return };
    let max = world.health.get(&defender).map_or(0.0, |h| h.max);
    if max <= 0.0 {
        return;
    }
    let amount = max * p.leech_pct;
    if let Some(h) = world.health.get_mut(&attacker) {
        h.restore(amount);
    }
}

/// If `defender` wears the Mage's "Armure de givre" and `attacker` struck from
/// melee range, retaliate with flat frost damage. Bypasses crit/dodge; mitigated
/// only by the attacker's defence and any frost resistance.
fn reflect_frost(world: &mut World, attacker: Entity, defender: Entity) {
    let Some(fa) = world.frost_armor.get(&defender).copied() else { return };
    if fa.reflect <= 0.0 || attacker == defender {
        return;
    }
    let (Some(&apos), Some(&dpos)) = (world.positions.get(&attacker), world.positions.get(&defender))
    else {
        return;
    };
    if apos.distance(dpos) > 2.5 {
        return;
    }
    if world.health.get(&attacker).map_or(true, |h| h.is_empty()) {
        return;
    }
    let amount = mitigate(
        fa.reflect,
        effective_defense(world, attacker),
        REFLECT_AR,
        0.0,
        elem_resist_for(world, attacker, Element::Frost),
    );
    if let Some(h) = world.health.get_mut(&attacker) {
        h.drain(amount);
    }
    world.render_events.push(RenderEvent::Damage {
        attacker: defender,
        target: attacker,
        pos: apos,
        amount,
        crit: false,
        dodged: false,
    });
}

/// Effective per-swing damage range for `e` (base + weapon, scaled by the
/// primary attribute). Summons inherit a fraction of this to scale with their
/// owner's offence.
#[must_use]
pub fn effective_damage(world: &World, e: Entity) -> (f32, f32) {
    let a = gather_attacker(world, e);
    let factor = 1.0 + a.primary / PRIMARY_DMG_DIVISOR;
    ((a.dmg_min + a.weapon_dmg) * factor, (a.dmg_max + a.weapon_dmg) * factor)
}

/// Effective defence for `e` (summons inherit a fraction of this).
#[must_use]
pub fn effective_defense(world: &World, e: Entity) -> f32 {
    gather_defender(world, e).defense
}

/// Effective Attack Rating for `e`. Players (and stat-bearing summons/mercs)
/// derive a baseline from level + Dexterity plus any `+AttackRating` gear; mobs
/// carry their AR on the [`MobDef`]; anything else falls back to [`AR_BASE`].
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn attack_rating(world: &World, e: Entity) -> f32 {
    if let Some(stats) = world.stats.get(&e) {
        let eff = stats.inner.effective_attributes();
        let level = world.progression.get(&e).map_or(stats.inner.level, |p| p.level());
        let gear = stats.inner.resolved.attack_rating.max(0) as f32;
        AR_BASE + level as f32 * AR_PER_LEVEL + eff.dexterity.max(0) as f32 * AR_PER_DEX + gear
    } else if let Some(def) = world.mob_types.get(&e).and_then(|m| world.data.mob(m.def_id)) {
        def.attack_rating.max(0) as f32
    } else {
        AR_BASE
    }
}

/// Probability in `[0, DODGE_CAP)` that `e` dodges a dodgeable attack from an
/// attacker whose Attack Rating is `attacker_ar`, using the exact same pipeline
/// as [`resolve_attack`]. Read by the character sheet's "chance to be hit".
#[must_use]
pub fn dodge_chance_vs(world: &World, e: Entity, attacker_ar: f32) -> f32 {
    dodge_from(gather_defender(world, e).dodge, attacker_ar)
}

/// Block probability in `[0, 1]` for `e` (shield / dual-wield parry). Used by
/// the projectile resolver to let shields knock arrows aside.
#[must_use]
pub fn block_chance(world: &World, e: Entity) -> f32 {
    gather_defender(world, e).block
}

/// Snapshot the mob involved in a player fight into `world.last_enemy`, so the
/// character sheet can keep showing its threat after it dies. No-op when the
/// exchange doesn't involve the player and a mob.
fn record_last_enemy(world: &mut World, attacker: Entity, defender: Entity) {
    let Some(player) = world.player_entity else { return };
    let enemy = if attacker == player {
        defender
    } else if defender == player {
        attacker
    } else {
        return;
    };
    // Snapshot primitives first so the `world.data` borrow ends before we take
    // a fresh immutable borrow for the name and then the mutable write.
    let Some((level, dmin, dmax, ar)) = world
        .mob_types
        .get(&enemy)
        .and_then(|m| world.data.mob(m.def_id))
        .map(|def| (def.level, def.damage_min as f32, def.damage_max as f32, def.attack_rating.max(0) as f32))
    else {
        return;
    };
    let name = entity_name(world, enemy);
    world.last_enemy =
        Some(crate::ecs::LastEnemy { name, level, damage_min: dmin, damage_max: dmax, attack_rating: ar });
}

/// Whether `e` is in an uninterruptible window (a Soldat melee technique): such
/// entities ignore stuns and cannot have their action interrupted.
#[must_use]
pub fn is_unstoppable(world: &World, e: Entity) -> bool {
    world.unstoppable.get(&e).is_some_and(|u| u.remaining > 0.0)
}

/// Whether `e` is a designated boss (per its `MobDef::is_boss` flag). Bosses
/// have poise — they are immune to *recul* (pushback) but can still be
/// stunned or knocked down.
#[must_use]
pub fn is_boss(world: &World, e: Entity) -> bool {
    world
        .mob_types
        .get(&e)
        .and_then(|m| world.data.mob(m.def_id))
        .is_some_and(|d| d.is_boss)
}

/// Whether `e` is locked out of acting by crowd control (stun, knockdown, or an
/// active recul). Unstoppable entities never report locked.
#[must_use]
pub fn is_cc_locked(world: &World, e: Entity) -> bool {
    if is_unstoppable(world, e) {
        return false;
    }
    world.stuns.get(&e).is_some_and(|&t| t > 0.0)
        || world.knockdowns.contains_key(&e)
        || world.pushbacks.contains_key(&e)
}

/// Knock `target` down for `duration` seconds. No-op on Unstoppable entities. The
/// knockdown freezes the target, drops its combat target, interrupts any cast,
/// and never shortens a longer existing knockdown.
pub fn apply_knockdown(world: &mut World, target: Entity, duration: f32) {
    if duration <= 0.0 || is_unstoppable(world, target) {
        return;
    }
    let cur = world.knockdowns.get(&target).map_or(0.0, |k| k.remaining);
    world.knockdowns.insert(target, Knockdown { remaining: cur.max(duration), dmg_amp: 1.2 });
    world.move_targets.remove(&target);
    if let Some(c) = world.combat.get_mut(&target) {
        c.target = None;
    }
    try_interrupt(world, target);
}

/// Shove `target` `distance` tiles along `(dx, dy)` at `speed` tiles/s. Unstoppable
/// entities are immune; the direction is normalised here.
pub fn apply_pushback(world: &mut World, target: Entity, dx: f32, dy: f32, distance: f32, speed: f32) {
    push_internal(world, target, dx, dy, distance, speed, None, 0.0);
}

/// As [`apply_pushback`], but chains a `knockdown` (seconds) when the recul ends
/// and deals `wall_bonus` flat damage if it slams into a wall.
pub fn apply_pushback_then_knockdown(
    world: &mut World,
    target: Entity,
    dx: f32,
    dy: f32,
    distance: f32,
    speed: f32,
    knockdown: f32,
    wall_bonus: f64,
) {
    let kd = if knockdown > 0.0 { Some(knockdown) } else { None };
    push_internal(world, target, dx, dy, distance, speed, kd, wall_bonus);
}

#[allow(clippy::too_many_arguments)]
fn push_internal(
    world: &mut World,
    target: Entity,
    dx: f32,
    dy: f32,
    distance: f32,
    speed: f32,
    knockdown_after: Option<f32>,
    wall_bonus_dmg: f64,
) {
    if distance <= 0.0 || speed <= 0.0 || is_unstoppable(world, target) || is_boss(world, target) {
        return;
    }
    let len = (dx * dx + dy * dy).sqrt();
    let (nx, ny) = if len <= 1e-4 { (1.0, 0.0) } else { (dx / len, dy / len) };
    world.pushbacks.insert(
        target,
        Pushback { dx: nx, dy: ny, distance, traveled: 0.0, speed, knockdown_after, wall_bonus_dmg },
    );
    world.move_targets.remove(&target);
    try_interrupt(world, target);
}

/// Cancel `target`'s in-progress cast, if any: refund half the skill's cooldown
/// and emit the interrupt feedback. Unstoppable or non-casting entities are
/// unaffected. Returns whether a cast was actually interrupted.
pub fn try_interrupt(world: &mut World, target: Entity) -> bool {
    if is_unstoppable(world, target) {
        return false;
    }
    // Waypoint teleport casts are always interruptible by damage.
    let waypoint_interrupted = crate::systems::waypoint::interrupt_waypoint_cast(world, target);
    if !world.interruptibles.get(&target).is_some_and(|i| i.casting) {
        return waypoint_interrupted;
    }
    let skill_id = world.interruptibles.get(&target).map_or(0, |i| i.skill_id);
    world.interruptibles.remove(&target);
    // Reset the matching skill's cooldown to 50% of its full value.
    if let Some(slot) = world.skills.get_mut(&target) {
        if let Some(idx) = slot.bar.iter().position(|s| s.id == skill_id) {
            let full = slot.bar[idx].cooldown;
            if let Some(cd) = slot.cooldowns.get_mut(idx) {
                cd.remaining = full * 0.5;
            }
        }
    }
    let pos = world.positions.get(&target).copied().unwrap_or(Position2D::new(0.0, 0.0));
    world.render_events.push(RenderEvent::Interrupt { target, pos });
    let name = entity_name(world, target);
    world.log_event(format!("{name} is interrupted!"));
    true
}

/// Resolve a single attack from `attacker` to `defender`, applying damage.
/// `damage_mult` lets skills scale the swing.
#[allow(clippy::cast_precision_loss)]
pub fn resolve_attack(
    world: &mut World,
    attacker: Entity,
    defender: Entity,
    damage_mult: f32,
) -> AttackOutcome {
    record_last_enemy(world, attacker, defender);
    let att = gather_attacker(world, attacker);
    let def = gather_defender(world, defender);
    let target_pos = world.positions.get(&defender).copied().unwrap_or(Position2D::new(0.0, 0.0));

    // Dual-wielders alternate hands: `att` above already used the current hand,
    // so flip the parity now for the next swing (every swing counts, even a
    // dodged or blocked one).
    if is_dual_wield_melee(world, attacker) {
        let next = !world.dual_hand.get(&attacker).copied().unwrap_or(false);
        world.dual_hand.insert(attacker, next);
    }

    // Invulnerable defenders (e.g. mid-dodge) take nothing.
    if world.invuln.get(&defender).is_some_and(|&t| t > 0.0) {
        world.render_events.push(RenderEvent::Damage {
            attacker,
            target: defender,
            pos: target_pos,
            amount: 0.0,
            crit: false,
            dodged: true,
        });
        return AttackOutcome::Dodged;
    }

    // Dodge — the defender's avoidance contested against the attacker's AR.
    let dodge_chance = dodge_from(def.dodge, att.attack_rating);
    if world.rng.next_f32() < dodge_chance {
        world.render_events.push(RenderEvent::Damage {
            attacker,
            target: defender,
            pos: target_pos,
            amount: 0.0,
            crit: false,
            dodged: true,
        });
        return AttackOutcome::Dodged;
    }

    // Block — a shield or dual-wield parry negates the hit outright. Only rolled
    // when the defender actually has block, so it costs no RNG otherwise.
    if def.block > 0.0 && world.rng.next_f32() < def.block {
        world.render_events.push(RenderEvent::Block { pos: target_pos });
        return AttackOutcome::Blocked;
    }

    // Base damage roll → ATK, scaled by any active attack buff on the attacker.
    let roll = world.rng.next_f32();
    let base = att.dmg_min + (att.dmg_max - att.dmg_min).max(0.0) * roll;
    let buff_mult = 1.0 + world.buffs.get(&attacker).map_or(0.0, |b| b.atk_pct);
    // "Wargh !" pours extra damage into every swing.
    let order_dmg = 1.0 + world.order_bonuses(attacker).dmg_pct;
    // The Clerc's war aura adds damage to allies within range.
    let aura_dmg = 1.0 + world.aura_bonuses.get(&attacker).map_or(0.0, |a| a.dmg_pct);
    let raw = (base + att.weapon_dmg)
        * (1.0 + att.primary / PRIMARY_DMG_DIVISOR)
        * damage_mult
        * buff_mult
        * order_dmg
        * aura_dmg;

    // Crit.
    let crit = world.rng.chance_pct(att.crit_pct);
    let after_crit = if crit { raw * CRIT_MULT } else { raw };

    // Defense reduction (scaled by the attacker's Attack Rating). Zirconia
    // weapon gems strip part of the target's Defense first.
    let pierced_def = def.defense * (1.0 - att.target_def_pct);
    let reduction = defense_reduction(pierced_def, att.attack_rating);
    // A blessing (e.g. the Clerc's Bénédiction) further reduces incoming damage.
    let blessing = world.resist.get(&defender).map_or(0.0, |r| r.pct);
    let aura_resist = world.aura_bonuses.get(&defender).map_or(0.0, |a| a.resist_pct);
    let resist = (blessing + aura_resist).clamp(0.0, 0.9);
    // A knocked-down defender takes amplified damage (+20%).
    let knock_amp = world.knockdowns.get(&defender).map_or(1.0, |k| k.dmg_amp as f32);
    let damage = (after_crit * (1.0 - reduction) * (1.0 - resist)).max(1.0) * knock_amp;

    if let Some(h) = world.health.get_mut(&defender) {
        h.drain(damage);
    }
    world.render_events.push(RenderEvent::Damage {
        attacker,
        target: defender,
        pos: target_pos,
        amount: damage,
        crit,
        dodged: false,
    });
    // Any landed damage interrupts an in-progress cast (unless Unstoppable).
    try_interrupt(world, defender);
    // A summon or mercenary that strikes a mob pulls its aggro onto itself, so
    // the mob retaliates against the ally instead of beelining for the player.
    let by_ally =
        world.summon_tags.contains_key(&attacker) || world.mercenary_tags.contains_key(&attacker);
    if by_ally && world.mob_tags.contains_key(&defender) {
        world.aggressor.insert(defender, attacker);
    }
    // The Mage's frost armour retaliates against melee attackers.
    reflect_frost(world, attacker, defender);
    // The Clerc's Aura de vengeance reflects a fraction of damage back.
    reflect_vengeance(world, attacker, defender, damage);
    // A "Pardoned" mob bleeds healing into every attacker.
    pardon_leech(world, attacker, defender);
    AttackOutcome::Hit { damage, crit }
}

/// A readable label for an entity in the combat log.
pub fn entity_name(world: &World, e: Entity) -> String {
    if world.player_entity == Some(e) {
        return "You".to_owned();
    }
    world
        .mob_types
        .get(&e)
        .and_then(|m| world.data.mob(m.def_id))
        .map_or_else(|| format!("Entity#{}", e.0), |d| d.name.clone())
}

/// Process every combatant: tick cooldowns and resolve swings in range.
pub fn combat_system(world: &mut World) {
    let dt = world.dt;
    for e in world.combat.keys().copied().collect::<Vec<_>>() {
        if let Some(c) = world.combat.get_mut(&e) {
            c.tick(dt);
        }
    }

    for e in world.combat.keys().copied().collect::<Vec<_>>() {
        let Some((target, range, ready, interval)) = world
            .combat
            .get(&e)
            .map(|c| (c.target, c.range, c.ready(), c.attack_interval))
        else {
            continue;
        };
        let Some(target) = target else { continue };

        // Attacker must be alive and not stunned.
        if world.health.get(&e).is_some_and(|h| h.is_empty()) {
            continue;
        }
        if is_cc_locked(world, e) {
            continue;
        }
        // Drop dead/missing targets.
        let target_dead =
            world.health.get(&target).map_or(true, |h| h.is_empty());
        if target_dead || !world.positions.contains_key(&target) {
            if let Some(c) = world.combat.get_mut(&e) {
                c.target = None;
            }
            continue;
        }
        if !ready {
            continue;
        }
        let (Some(&epos), Some(&tpos)) =
            (world.positions.get(&e), world.positions.get(&target))
        else {
            continue;
        };
        if epos.distance(tpos) > range {
            continue;
        }
        // Face the target while engaging it.
        if let Some(dir) = Direction::from_world_vector(tpos.x - epos.x, tpos.y - epos.y) {
            world.directions.insert(e, dir);
        }

        let kind = attacker_kind(world, e);
        match kind {
            AttackerKind::Melee => {
                let outcome = resolve_attack(world, e, target, 1.0);
                let att = entity_name(world, e);
                let def = entity_name(world, target);
                match outcome {
                    AttackOutcome::Dodged => {
                        world.log_event(format!("{att} attacks {def} — dodged!"));
                    }
                    AttackOutcome::Blocked => {
                        world.log_event(format!("{att} attacks {def} — blocked!"));
                    }
                    AttackOutcome::Hit { damage, crit } => {
                        let tag = if crit { " (CRIT)" } else { "" };
                        world.log_event(format!("{att} hits {def} for {damage:.0}{tag}"));
                    }
                }
            }
            AttackerKind::Ranged | AttackerKind::MagicRanged => {
                let is_magic = kind == AttackerKind::MagicRanged;
                let _ = spawn_projectile(world, e, target, is_magic);
                let att = entity_name(world, e);
                let def = entity_name(world, target);
                let verb = if is_magic { "casts at" } else { "shoots" };
                world.log_event(format!("{att} {verb} {def}"));
            }
        }
        let atk_speed = swing_speed(world, e);
        if let Some(c) = world.combat.get_mut(&e) {
            c.cooldown = interval / atk_speed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::item::ItemInstance;
    use crate::components::mercenary::MercType;
    use crate::components::mob::{MobTag, MobType};
    use crate::components::player::ClassType;
    use crate::components::Health;
    use crate::data::items::base_item;
    use crate::systems::mercenary::{recompute_merc_stats, spawn_mercenary};
    use crate::systems::stats::recompute_player_stats;
    use mge_items::affixes::ItemAffixes;
    use mge_items::item::{EquipSlot, Item, ItemKind};
    use mge_items::rarity::Rarity;

    fn weapon_ilvl(uid: u64, kind: ItemKind, ilvl: u32) -> ItemInstance {
        let base = base_item(kind, ilvl.max(1));
        ItemInstance::new(Item::new(uid, base, Rarity::Normal, ilvl), ItemAffixes::new(), Vec::new(), format!("{kind:?}"))
    }

    /// The public `dodge_chance_vs` must match the exact `ESQ/(ESQ+AR)` roll
    /// used inside `resolve_attack`, so the character sheet never drifts.
    #[test]
    fn dodge_chance_matches_resolve_formula() {
        let app = crate::build_app(ClassType::Soldat, 1);
        let player = app.world.player_entity.unwrap();
        let esq = gather_defender(&app.world, player).dodge;
        let ar = 60.0_f32;
        let expected = (esq / (esq + ar)).clamp(0.0, DODGE_CAP);
        assert!((dodge_chance_vs(&app.world, player, ar) - expected).abs() < 1e-6);
    }

    /// A more accurate attacker (higher AR) is harder to dodge and shreds more
    /// armour — the core of the new model.
    #[test]
    fn higher_attack_rating_lowers_avoidance_and_mitigation() {
        let app = crate::build_app(ClassType::Soldat, 1);
        let player = app.world.player_entity.unwrap();
        let lo = dodge_chance_vs(&app.world, player, 30.0);
        let hi = dodge_chance_vs(&app.world, player, 300.0);
        assert!(hi < lo, "dodge must drop vs a higher-AR foe ({hi} !< {lo})");
        assert!(defense_reduction(100.0, 300.0) < defense_reduction(100.0, 30.0));
    }

    /// Balance guard-rails: lock the avoidance/mitigation curve into healthy
    /// bands so future tuning can't silently make a fresh hero un-hittable or
    /// let defence collapse at high level the way the old `50*level` model did.
    #[test]
    fn balance_curve_stays_in_healthy_bands() {
        // Fresh hero: ESQ from base Dexterity 20, DEF 40 (= 2*dex).
        let esq0 = 20.0 * ESQ_PER_DEX;

        // vs a level-1 mob (AR 30): a modest, not dominant, defence.
        let dodge0 = dodge_from(esq0, 30.0);
        assert!((0.15..=0.30).contains(&dodge0), "fresh dodge out of band: {dodge0}");
        let red0 = defense_reduction(40.0, 30.0);
        assert!((0.30..=0.50).contains(&red0), "fresh reduction out of band: {red0}");

        // A far more accurate foe (boss AR 126) punches through both layers.
        assert!(dodge_from(esq0, 126.0) < dodge0);
        assert!(defense_reduction(40.0, 126.0) < red0);

        // Both layers respect their caps no matter how lopsided the inputs.
        assert!(dodge_from(10_000.0, 1.0) <= DODGE_CAP + 1e-6);
        assert!(defense_reduction(10_000.0, 1.0) <= DEF_REDUCTION_CAP + 1e-6);

        // Defence stays meaningful deep into the curve: DEF 500 vs a level-40
        // mob (AR 225) still sheds ~50% (the old `50*level` model gave ~20%).
        assert!(defense_reduction(500.0, 225.0) > 0.45);
    }

    /// A shielded hero turns a meaningful share of incoming melee swings into
    /// blocks (zero-damage outcomes), and an un-shielded one never does.
    #[test]
    fn a_shield_blocks_incoming_attacks() {
        let mut app = crate::build_app(ClassType::Soldat, 1);
        let player = app.world.player_entity.unwrap();
        let ppos = *app.world.positions.get(&player).unwrap();

        // Strap on a shield so block_chance > 0.
        let base = base_item(ItemKind::Shield, 1);
        let shield = ItemInstance::new(Item::new(1, base, Rarity::Normal, 1), ItemAffixes::new(), Vec::new(), "Buckler".into());
        app.world.equipment.get_mut(&player).unwrap().equip_to(EquipSlot::OffHand, shield);
        recompute_player_stats(&mut app.world, player);
        assert!(block_chance(&app.world, player) > 0.0, "shield must grant block");

        // A level-1 mob hammers the player; over many swings some are blocked.
        let mob_id = app.world.data.mobs[0].id;
        let mob = app.world.spawn();
        app.world.mob_tags.insert(mob, MobTag);
        app.world.mob_types.insert(mob, MobType { def_id: mob_id });
        app.world.positions.insert(mob, ppos);
        app.world.health.insert(mob, Health::full(1_000_000.0));

        let mut blocked = 0;
        for _ in 0..400 {
            // Keep the player alive so the loop measures outcomes, not deaths.
            app.world.health.get_mut(&player).unwrap().restore(9999.0);
            if resolve_attack(&mut app.world, mob, player, 1.0) == AttackOutcome::Blocked {
                blocked += 1;
            }
        }
        assert!(blocked > 0, "a shielded hero should block some swings (got {blocked})");
    }

    /// Dual-wielding swaps which weapon's damage is used each swing, so two
    /// weapons of different power produce different per-hand damage.
    #[test]
    fn dual_wield_alternates_per_hand_damage() {
        let mut app = crate::build_app(ClassType::Soldat, 1);
        let player = app.world.player_entity.unwrap();
        {
            let eq = app.world.equipment.get_mut(&player).unwrap();
            eq.equip_to(EquipSlot::MainHand, weapon_ilvl(1, ItemKind::Sword, 40)); // strong
            eq.equip_to(EquipSlot::OffHand, weapon_ilvl(2, ItemKind::Dagger, 1)); // weak
        }
        recompute_player_stats(&mut app.world, player);
        assert!(is_dual_wield_melee(&app.world, player), "two one-handers = dual-wield");

        // Off-hand parity false → main-hand damage; true → off-hand damage.
        app.world.dual_hand.insert(player, false);
        let (_, main_max) = effective_damage(&app.world, player);
        app.world.dual_hand.insert(player, true);
        let (_, off_max) = effective_damage(&app.world, player);
        assert!(
            main_max > off_max,
            "the ilvl-40 main-hand should out-damage the ilvl-1 off-hand ({main_max} vs {off_max})",
        );

        // A swing flips the parity for the next one.
        let before = app.world.dual_hand.get(&player).copied().unwrap_or(false);
        let mob = app.world.spawn();
        app.world.mob_tags.insert(mob, MobTag);
        app.world.mob_types.insert(mob, MobType { def_id: app.world.data.mobs[0].id });
        app.world.positions.insert(mob, *app.world.positions.get(&player).unwrap());
        app.world.health.insert(mob, Health::full(1000.0));
        let _ = resolve_attack(&mut app.world, player, mob, 1.0);
        assert_ne!(before, app.world.dual_hand.get(&player).copied().unwrap_or(false), "swing flips the hand");
    }

    /// Dual-wielding swings faster than a single weapon (the +30% IAS bonus).
    #[test]
    fn dual_wield_swings_faster() {
        let mut app = crate::build_app(ClassType::Soldat, 1);
        let player = app.world.player_entity.unwrap();
        let single = swing_speed(&app.world, player);
        {
            let eq = app.world.equipment.get_mut(&player).unwrap();
            eq.equip_to(EquipSlot::MainHand, weapon_ilvl(1, ItemKind::Sword, 5));
            eq.equip_to(EquipSlot::OffHand, weapon_ilvl(2, ItemKind::Axe, 5));
        }
        recompute_player_stats(&mut app.world, player);
        let dual = swing_speed(&app.world, player);
        assert!((dual - single - DUAL_WIELD_IAS).abs() < 1e-6, "dual-wield should add the IAS bonus ({single} -> {dual})");
    }

    /// A mercenary with a blocking off-hand gains block chance, like the player.
    #[test]
    fn mercenary_with_shield_gains_block() {
        let mut app = crate::build_app(ClassType::Soldat, 1);
        let owner = app.world.player_entity.unwrap();
        // A Healer wields a single Wand → no blocking off-hand → no block.
        let healer = spawn_mercenary(&mut app.world, owner, MercType::Healer, 5).expect("merc spawns");
        recompute_merc_stats(&mut app.world, healer);
        assert_eq!(block_chance(&app.world, healer), 0.0, "no block without a shield");

        // Hand it a shield (routes to the off-hand) → block.
        let base = base_item(ItemKind::Shield, 1);
        let shield = ItemInstance::new(Item::new(9, base, Rarity::Normal, 1), ItemAffixes::new(), Vec::new(), "Buckler".into());
        let _ = app.world.merc_inventory.get_mut(&healer).unwrap().equip(shield);
        recompute_merc_stats(&mut app.world, healer);
        assert!(block_chance(&app.world, healer) > 0.0, "a shielded merc should block");
    }

    /// Gifting gear to a merc equips it: a one-handed weapon routes to the
    /// off-hand (dual-wield), a shield takes the off-hand for block.
    #[test]
    fn gifting_gear_to_a_merc_equips_it() {
        let mut app = crate::build_app(ClassType::Soldat, 1);
        let player = app.world.player_entity.unwrap();
        let guard = spawn_mercenary(&mut app.world, player, MercType::Guardian, 5).expect("merc spawns");

        // A one-handed weapon → off-hand (the Guardian already wields a Sword).
        let mace = weapon_ilvl(50, ItemKind::Mace, 5);
        let idx = app.world.inventory.get_mut(&player).unwrap().add(mace).expect("inv slot");
        assert!(crate::plugins::inventory::equip_inventory_item_to_merc(&mut app.world, player, idx, guard));
        let off = app.world.merc_inventory.get(&guard).unwrap().get(EquipSlot::OffHand).map(|w| w.item.base.kind);
        assert_eq!(off, Some(ItemKind::Mace), "one-hander routes to the merc off-hand");
        assert!(is_dual_wield_melee(&app.world, guard));

        // A shield → off-hand for block (no longer dual-wield).
        let shield = weapon_ilvl(51, ItemKind::Shield, 5);
        let sidx = app.world.inventory.get_mut(&player).unwrap().add(shield).expect("inv slot");
        assert!(crate::plugins::inventory::equip_inventory_item_to_merc(&mut app.world, player, sidx, guard));
        let off2 = app.world.merc_inventory.get(&guard).unwrap().get(EquipSlot::OffHand).map(|w| w.item.base.kind);
        assert_eq!(off2, Some(ItemKind::Shield));
        assert!(block_chance(&app.world, guard) > 0.0, "shielded merc blocks");
    }

    /// A fresh hero starts with a single hotbar skill — the rest are learned in
    /// the skill tree (which then places them on the bar).
    #[test]
    fn fresh_hero_starts_with_one_hotbar_skill() {
        let app = crate::build_app(ClassType::Soldat, 1);
        let p = app.world.player_entity.unwrap();
        assert_eq!(app.world.skills.get(&p).map(|s| s.bar.len()), Some(1));
    }

    /// The party holds several distinct companions up to the cap; duplicates of
    /// an archetype and over-cap hires are refused.
    #[test]
    fn party_holds_multiple_distinct_companions() {
        use crate::systems::mercenary::{companions, recruit_mercenary, MAX_COMPANIONS};
        let mut app = crate::build_app(ClassType::Soldat, 1); // spawns in the safe Village
        let player = app.world.player_entity.unwrap();
        app.world.inventory.get_mut(&player).unwrap().gold = 1_000_000;

        assert!(recruit_mercenary(&mut app.world, player, MercType::Guardian));
        assert!(recruit_mercenary(&mut app.world, player, MercType::Archer));
        assert_eq!(companions(&app.world, player).len(), 2, "both companions stay in the party");
        assert!(!recruit_mercenary(&mut app.world, player, MercType::Guardian), "no duplicate archetype");
        assert!(recruit_mercenary(&mut app.world, player, MercType::Healer));
        assert_eq!(companions(&app.world, player).len(), MAX_COMPANIONS);
        assert!(!recruit_mercenary(&mut app.world, player, MercType::Mage), "party is full");
    }

    /// The merc panel's unequip path takes a worn item back into the backpack
    /// and updates the merc's resolved stats.
    #[test]
    fn unequipping_a_merc_slot_returns_the_item() {
        let mut app = crate::build_app(ClassType::Soldat, 1);
        let player = app.world.player_entity.unwrap();
        let guard = spawn_mercenary(&mut app.world, player, MercType::Guardian, 5).expect("merc spawns");
        assert_eq!(crate::ui::merc_ui::active_merc(&app.world, player), Some(guard));

        // Guardian starts dual-wield (Sword + Axe). Take the off-hand Axe back.
        assert!(is_dual_wield_melee(&app.world, guard));
        let before = app.world.inventory.get(&player).map_or(0, |i| i.count());
        assert!(crate::plugins::inventory::unequip_merc_slot(&mut app.world, player, guard, EquipSlot::OffHand));
        assert!(app.world.merc_inventory.get(&guard).unwrap().get(EquipSlot::OffHand).is_none());
        assert!(!is_dual_wield_melee(&app.world, guard), "no longer dual-wields after removing the off-hand");
        let after = app.world.inventory.get(&player).map_or(0, |i| i.count());
        assert_eq!(after, before + 1, "the removed weapon returns to the backpack");
    }

    /// The Guardian dual-wields out of the box: it reads as dual-wield, swings
    /// faster, and blocks — all sourced from its `merc_inventory`.
    #[test]
    fn guardian_mercenary_dual_wields() {
        let mut app = crate::build_app(ClassType::Soldat, 1);
        let owner = app.world.player_entity.unwrap();
        let guard = spawn_mercenary(&mut app.world, owner, MercType::Guardian, 5).expect("merc spawns");
        assert!(is_dual_wield_melee(&app.world, guard), "Guardian wields two one-handers");
        assert!(block_chance(&app.world, guard) > 0.0, "dual-wield grants parry block");
        // The dual-wield IAS bonus applies to the merc's swing rate too.
        let base = base_item(ItemKind::Sword, 5);
        let lone = ItemInstance::new(Item::new(77, base, Rarity::Normal, 5), ItemAffixes::new(), Vec::new(), "S".into());
        let speed_dual = swing_speed(&app.world, guard);
        app.world.merc_inventory.get_mut(&guard).unwrap().unequip(EquipSlot::OffHand);
        let _ = app.world.merc_inventory.get_mut(&guard).unwrap().equip(lone); // back to single Sword
        let speed_single = swing_speed(&app.world, guard);
        assert!(speed_dual > speed_single, "dual-wield should swing faster ({speed_dual} vs {speed_single})");
    }

    /// A player↔mob exchange snapshots the mob into `world.last_enemy`, even when
    /// the swing is dodged (it is recorded before the dodge roll).
    #[test]
    fn resolve_attack_records_the_last_enemy() {
        let mut app = crate::build_app(ClassType::Soldat, 1);
        let player = app.world.player_entity.unwrap();
        let ppos = *app.world.positions.get(&player).unwrap();
        let mob_id = app.world.data.mobs[0].id;

        let mob = app.world.spawn();
        app.world.mob_tags.insert(mob, MobTag);
        app.world.mob_types.insert(mob, MobType { def_id: mob_id });
        app.world.positions.insert(mob, ppos);
        app.world.health.insert(mob, Health::full(100.0));

        assert!(app.world.last_enemy.is_none());
        let _ = resolve_attack(&mut app.world, player, mob, 1.0);

        let def = app.world.data.mob(mob_id).unwrap();
        let (level, dmax) = (def.level, def.damage_max as f32);
        let le = app.world.last_enemy.as_ref().expect("last enemy recorded");
        assert_eq!(le.level, level);
        assert_eq!(le.damage_max, dmax);
    }
}
