//! Active skill definitions per class — the usable hotbar actives, with their
//! cast behaviour ([`SkillEffect`]) and visual flavour ([`SkillFx`]). Each class
//! ends with its summon skills (three for the Soldat, two for the others).

use mge_gameplay::skills::{SkillCost, SkillDef, MAX_SKILL_LEVEL};

use crate::components::player::ClassType;
use crate::components::skill::{ActiveSkill, AuraType, OrderType, PassiveType, SkillEffect, SkillKind};
use crate::data::summon;
use crate::ecs::SkillFx;

/// A named cluster of skill ids that share an unlock gate. Each class owns its
/// three branches for free; foreign branches can be bought at the cost of one
/// skill point and become investable like native ones.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    pub class: ClassType,
    pub index: u32,
    pub name: String,
    pub skill_ids: Vec<u32>,
}

/// The three branches a class organises its rich actives into. The order
/// matches the in-fiction grouping (e.g. Soldat = Combat / Invocations /
/// Ordres) and the indexing is stable across saves.
#[must_use]
pub fn branches_for_class(class: ClassType) -> Vec<Branch> {
    match class {
        ClassType::Soldat => vec![
            Branch { class, index: 0, name: "Combat".into(), skill_ids: vec![1001, 1002, 1003] },
            Branch { class, index: 1, name: "Invocations".into(), skill_ids: vec![1005, 1006, 1007] },
            Branch { class, index: 2, name: "Ordres".into(), skill_ids: vec![1008, 1009, 1010] },
        ],
        ClassType::Clerc => vec![
            Branch {
                class,
                index: 0,
                name: "Soutien".into(),
                skill_ids: vec![5001, 5002, 5003, 5004, 5005, 5006],
            },
            Branch {
                class,
                index: 1,
                name: "Béni".into(),
                skill_ids: vec![5007, 5008, 5009, 5010, 5011],
            },
            Branch {
                class,
                index: 2,
                name: "Divin".into(),
                skill_ids: vec![5012, 5013, 5014, 5015],
            },
        ],
        ClassType::MageDeGuerre => vec![
            Branch { class, index: 0, name: "Feu".into(), skill_ids: vec![3001, 3002, 3003] },
            Branch { class, index: 1, name: "Glace".into(), skill_ids: vec![3004, 3005, 3006] },
            Branch { class, index: 2, name: "Foudre".into(), skill_ids: vec![3007, 3008, 3009] },
        ],
        ClassType::Archer => vec![
            Branch { class, index: 0, name: "Tir".into(), skill_ids: vec![4001, 4002, 4003] },
            Branch { class, index: 1, name: "Piège".into(), skill_ids: vec![4004, 4005, 4006] },
            Branch { class, index: 2, name: "Agilité".into(), skill_ids: vec![4007, 4008, 4009] },
        ],
    }
}

/// Look up the `ActiveSkill` definition (and originating class) for a skill id.
#[must_use]
pub fn active_skill_by_id(skill_id: u32) -> Option<(ClassType, ActiveSkill)> {
    for class in ClassType::all() {
        if let Some(s) = skills_for(class).into_iter().find(|s| s.id == skill_id) {
            return Some((class, s));
        }
    }
    None
}

/// Locate the branch a given skill id belongs to (across every class).
#[must_use]
pub fn branch_of_skill(skill_id: u32) -> Option<Branch> {
    for class in ClassType::all() {
        for b in branches_for_class(class) {
            if b.skill_ids.contains(&skill_id) {
                return Some(b);
            }
        }
    }
    None
}

/// Id of the skill that must have at least 1 point invested before `skill_id`
/// becomes investable — the previous skill in the same branch. `None` for the
/// first skill of each branch (always available). `class` is accepted for
/// callsite clarity; branch ownership is global.
#[must_use]
pub fn prerequisite_for(skill_id: u32, _class: ClassType) -> Option<u32> {
    let branch = branch_of_skill(skill_id)?;
    let position = branch.skill_ids.iter().position(|&i| i == skill_id)?;
    if position == 0 {
        None
    } else {
        Some(branch.skill_ids[position - 1])
    }
}

/// Build a `SkillDef` that the player's `SkillTree` will accept. The `class`
/// field is rewritten to `owner_class.to_mge()` so `SkillTree::can_learn`'s
/// "wrong class" check passes for foreign-branch skills the player has
/// unlocked — the tree only ever holds one `Class`, not a set.
#[must_use]
pub fn skill_def_for(skill_id: u32, owner_class: ClassType) -> Option<SkillDef> {
    let (_origin, active) = active_skill_by_id(skill_id)?;
    let branch = branch_of_skill(skill_id)?;
    let position = branch.skill_ids.iter().position(|&i| i == skill_id)?;
    let prerequisites =
        if position == 0 { Vec::new() } else { vec![branch.skill_ids[position - 1]] };
    Some(SkillDef {
        id: skill_id,
        name: active.name.clone(),
        kind: active.kind,
        class: owner_class.to_mge(),
        min_level: 1,
        prerequisites,
        cost: SkillCost { mana_base: 0.0, mana_per_level: 0.0 },
        max_level: MAX_SKILL_LEVEL,
    })
}

/// The starting hotbar of usable actives for a class.
pub fn skills_for(class: ClassType) -> Vec<ActiveSkill> {
    match class {
        ClassType::Soldat => vec![
            ActiveSkill {
                id: 1001,
                name: "Grand coup".into(),
                level: 1,
                kind: SkillKind::Melee,
                // Damage is resolved per-hand in `systems::skill`; the static
                // `damage_mult` is unused for this effect. Mana scales with rank.
                mana_cost: 3.0,
                cooldown: 2.5,
                damage_mult: 1.0,
                range: 2.0,
                effect: SkillEffect::GrandCoup { duration_base: 0.8, duration_speed_reduction: 0.20 },
                fx: SkillFx::Slash,
            },
            ActiveSkill {
                id: 1002,
                name: "Taille".into(),
                level: 1,
                kind: SkillKind::Melee,
                mana_cost: 4.0,
                cooldown: 4.0,
                damage_mult: 1.0,
                range: 4.0,
                effect: SkillEffect::Taille {
                    arc_deg: 120.0,
                    radius: 4.0,
                    duration_base: 1.1,
                    duration_speed_reduction: 0.30,
                },
                fx: SkillFx::Whirlwind,
            },
            ActiveSkill {
                id: 1003,
                name: "Estocs".into(),
                level: 1,
                kind: SkillKind::Melee,
                mana_cost: 4.0,
                cooldown: 5.0,
                damage_mult: 1.0,
                range: 2.0,
                effect: SkillEffect::Estocs { hits: 3 },
                fx: SkillFx::Slash,
            },
            ActiveSkill {
                id: 1005,
                name: "Paysans".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 20.0,
                cooldown: 10.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::PEASANTS },
                fx: SkillFx::Summon,
            },
            ActiveSkill {
                id: 1006,
                name: "Miliciens".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 40.0,
                cooldown: 20.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::MILITIAMEN },
                fx: SkillFx::Summon,
            },
            ActiveSkill {
                id: 1007,
                name: "Guerriers".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 65.0,
                cooldown: 30.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::ELITE_WARRIORS },
                fx: SkillFx::Summon,
            },
            ActiveSkill {
                id: 1008,
                name: "En avant !".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 30.0,
                cooldown: 25.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Order { order: OrderType::Advance, dur: 20.0, radius: 30.0 },
                fx: SkillFx::Buff,
            },
            ActiveSkill {
                id: 1009,
                name: "Tenez bon !".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 40.0,
                cooldown: 25.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Order { order: OrderType::HoldFast, dur: 20.0, radius: 30.0 },
                fx: SkillFx::Buff,
            },
            ActiveSkill {
                id: 1010,
                name: "Wargh !".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 50.0,
                cooldown: 30.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Order { order: OrderType::Wargh, dur: 30.0, radius: 50.0 },
                fx: SkillFx::Buff,
            },
        ],
        ClassType::Clerc => vec![
            // ── Branche SOUTIEN — six toggleable auras (one active at a time) ──
            ActiveSkill {
                id: 5001,
                name: "Aura de soin".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 15.0,
                cooldown: 0.5,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Aura { aura_type: AuraType::Heal },
                fx: SkillFx::Heal,
            },
            ActiveSkill {
                id: 5002,
                name: "Aura de défense".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 12.0,
                cooldown: 0.5,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Aura { aura_type: AuraType::Defense },
                fx: SkillFx::Buff,
            },
            ActiveSkill {
                id: 5003,
                name: "Aura de résistance".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 12.0,
                cooldown: 0.5,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Aura { aura_type: AuraType::Resist },
                fx: SkillFx::Buff,
            },
            ActiveSkill {
                id: 5004,
                name: "Aura de guerre".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 18.0,
                cooldown: 0.5,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Aura { aura_type: AuraType::War },
                fx: SkillFx::Buff,
            },
            ActiveSkill {
                id: 5005,
                name: "Aura du vent".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 14.0,
                cooldown: 0.5,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Aura { aura_type: AuraType::Wind },
                fx: SkillFx::Buff,
            },
            ActiveSkill {
                id: 5006,
                name: "Aura de vengeance".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 20.0,
                cooldown: 0.5,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Aura { aura_type: AuraType::Vengeance },
                fx: SkillFx::Buff,
            },
            // ── Branche BÉNI — five active utility prayers ─────────────────────
            ActiveSkill {
                id: 5007,
                name: "Prière".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 12.0,
                cooldown: 6.0,
                damage_mult: 0.0,
                range: 7.0,
                effect: SkillEffect::Prayer,
                fx: SkillFx::Holy,
            },
            ActiveSkill {
                id: 5008,
                name: "Exorcisme".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 15.0,
                cooldown: 5.0,
                damage_mult: 1.0,
                range: 7.0,
                effect: SkillEffect::Exorcism,
                fx: SkillFx::Holy,
            },
            ActiveSkill {
                id: 5009,
                name: "Conversion".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 25.0,
                cooldown: 15.0,
                damage_mult: 0.0,
                range: 6.0,
                effect: SkillEffect::Conversion,
                fx: SkillFx::Holy,
            },
            ActiveSkill {
                id: 5010,
                name: "Pardon".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 18.0,
                cooldown: 8.0,
                damage_mult: 0.0,
                range: 7.0,
                effect: SkillEffect::Pardon,
                fx: SkillFx::Holy,
            },
            ActiveSkill {
                id: 5011,
                name: "Résurrection".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 50.0,
                cooldown: 30.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Resurrection,
                fx: SkillFx::Heal,
            },
            // ── Branche DIVIN — Élu prophétique + three angelic invocations ───
            ActiveSkill {
                id: 5012,
                name: "Élu prophétique".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 0.0,
                cooldown: 0.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Passive { passive_type: PassiveType::ProphetElect },
                fx: SkillFx::Buff,
            },
            ActiveSkill {
                id: 5013,
                name: "Chérubin".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 30.0,
                cooldown: 20.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::CHERUBIN },
                fx: SkillFx::Summon,
            },
            ActiveSkill {
                id: 5014,
                name: "Ange gardien".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 60.0,
                cooldown: 30.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::GUARDIAN_ANGEL },
                fx: SkillFx::Summon,
            },
            ActiveSkill {
                id: 5015,
                name: "Archange".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 100.0,
                cooldown: 60.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::ARCHANGEL },
                fx: SkillFx::Summon,
            },
        ],
        ClassType::MageDeGuerre => vec![
            // ── Branche FEU ───────────────────────────────────────────────
            ActiveSkill {
                id: 3001,
                name: "Boule de feu".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 5.0,
                cooldown: 1.5,
                damage_mult: 1.0,
                range: 8.0,
                effect: SkillEffect::Fireball,
                fx: SkillFx::Fire,
            },
            ActiveSkill {
                id: 3002,
                name: "Mur de flammes".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 12.0,
                cooldown: 6.0,
                damage_mult: 1.0,
                range: 7.0,
                effect: SkillEffect::FlameWall,
                fx: SkillFx::Fire,
            },
            ActiveSkill {
                id: 3003,
                name: "Pluie de météores".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 25.0,
                cooldown: 12.0,
                damage_mult: 1.0,
                range: 9.0,
                effect: SkillEffect::MeteorRain { radius: 5.0 },
                fx: SkillFx::Meteor,
            },
            // ── Branche GLACE ─────────────────────────────────────────────
            ActiveSkill {
                id: 3004,
                name: "Lance de glace".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 6.0,
                cooldown: 2.0,
                damage_mult: 1.0,
                range: 8.0,
                effect: SkillEffect::IceLance,
                fx: SkillFx::Frost,
            },
            ActiveSkill {
                id: 3005,
                name: "Armure de givre".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 15.0,
                cooldown: 20.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::FrostArmor,
                fx: SkillFx::Frost,
            },
            ActiveSkill {
                id: 3006,
                name: "Blizzard".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 20.0,
                cooldown: 10.0,
                damage_mult: 1.0,
                range: 9.0,
                effect: SkillEffect::Blizzard { radius: 6.0 },
                fx: SkillFx::Frost,
            },
            // ── Branche FOUDRE ────────────────────────────────────────────
            ActiveSkill {
                id: 3007,
                name: "Éclair".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 7.0,
                cooldown: 1.0,
                damage_mult: 1.0,
                range: 8.0,
                effect: SkillEffect::ChainBolt { radius: 4.0 },
                fx: SkillFx::Lightning,
            },
            ActiveSkill {
                id: 3008,
                name: "Bouclier de foudre".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 10.0,
                cooldown: 25.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::LightningShield,
                fx: SkillFx::Lightning,
            },
            ActiveSkill {
                id: 3009,
                name: "Nova électrique".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 18.0,
                cooldown: 8.0,
                damage_mult: 1.0,
                range: 0.0,
                effect: SkillEffect::ElectricNova { radius: 5.0 },
                fx: SkillFx::Lightning,
            },
            // ── Invocations ───────────────────────────────────────────────
            ActiveSkill {
                id: 3010,
                name: "Élémentaire".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 30.0,
                cooldown: 30.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::ELEMENTAL },
                fx: SkillFx::Summon,
            },
            ActiveSkill {
                id: 3011,
                name: "Hydre".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 20.0,
                cooldown: 15.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::HYDRA },
                fx: SkillFx::Summon,
            },
        ],
        ClassType::Archer => vec![
            // ── Branche TIR ───────────────────────────────────────────────
            ActiveSkill {
                id: 4001,
                name: "Tir rapide".into(),
                level: 1,
                kind: SkillKind::Ranged,
                mana_cost: 2.0,
                cooldown: 0.5,
                damage_mult: 1.0,
                range: 8.0,
                effect: SkillEffect::FastShot,
                fx: SkillFx::Arrow,
            },
            ActiveSkill {
                id: 4002,
                name: "Tir perçant".into(),
                level: 1,
                kind: SkillKind::Ranged,
                mana_cost: 5.0,
                cooldown: 1.5,
                damage_mult: 1.0,
                range: 9.0,
                effect: SkillEffect::PiercingShot { half_width: 0.8 },
                fx: SkillFx::Arrow,
            },
            ActiveSkill {
                id: 4003,
                name: "Tir chargé".into(),
                level: 1,
                kind: SkillKind::Ranged,
                mana_cost: 8.0,
                cooldown: 4.0,
                damage_mult: 1.0,
                range: 10.0,
                effect: SkillEffect::ChargedShot,
                fx: SkillFx::Arrow,
            },
            // ── Branche PIÈGE / DEBUFF ────────────────────────────────────
            ActiveSkill {
                id: 4004,
                name: "Tir empoisonné".into(),
                level: 1,
                kind: SkillKind::Ranged,
                mana_cost: 4.0,
                cooldown: 2.0,
                damage_mult: 0.6,
                range: 8.0,
                effect: SkillEffect::PoisonShot { dps: 8.0, dur: 5.0, def_pct: 0.20 },
                fx: SkillFx::Poison,
            },
            ActiveSkill {
                id: 4005,
                name: "Piège explosif".into(),
                level: 1,
                kind: SkillKind::Trap,
                mana_cost: 6.0,
                cooldown: 3.0,
                damage_mult: 0.0,
                range: 6.0,
                effect: SkillEffect::ExplosiveTrap { radius: 3.0, max_active: 3, lifetime: 15.0 },
                fx: SkillFx::Trap,
            },
            ActiveSkill {
                id: 4006,
                name: "Piège à mâchoires".into(),
                level: 1,
                kind: SkillKind::Trap,
                mana_cost: 5.0,
                cooldown: 5.0,
                damage_mult: 0.0,
                range: 6.0,
                effect: SkillEffect::JawTrap { radius: 1.5, max_active: 2, lifetime: 20.0 },
                fx: SkillFx::Trap,
            },
            // ── Branche AGILITÉ / SURVIE ──────────────────────────────────
            ActiveSkill {
                id: 4007,
                name: "Évasion".into(),
                level: 1,
                kind: SkillKind::Spell,
                mana_cost: 5.0,
                cooldown: 6.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::DodgeRoll { dist: 4.0, invuln: 0.4 },
                fx: SkillFx::Dodge,
            },
            ActiveSkill {
                id: 4008,
                name: "Pluie de flèches".into(),
                level: 1,
                kind: SkillKind::Ranged,
                mana_cost: 15.0,
                cooldown: 10.0,
                damage_mult: 0.0,
                range: 9.0,
                effect: SkillEffect::ArrowRain { radius: 5.0 },
                fx: SkillFx::ArrowRain,
            },
            ActiveSkill {
                id: 4009,
                name: "Instinct du chasseur".into(),
                level: 1,
                kind: SkillKind::Aura,
                mana_cost: 10.0,
                cooldown: 20.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::HunterInstinct,
                fx: SkillFx::Buff,
            },
            // ── Invocations ───────────────────────────────────────────────
            ActiveSkill {
                id: 4010,
                name: "Loup de chasse".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 20.0,
                cooldown: 15.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::HUNT_WOLF },
                fx: SkillFx::Summon,
            },
            ActiveSkill {
                id: 4011,
                name: "Piège sentinelle".into(),
                level: 1,
                kind: SkillKind::Summon,
                mana_cost: 15.0,
                cooldown: 20.0,
                damage_mult: 0.0,
                range: 0.0,
                effect: SkillEffect::Summon { profile: &summon::JAW_TRAP },
                fx: SkillFx::Summon,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expected hotbar size per class (combat actives + summon skills).
    fn expected_len(class: ClassType) -> usize {
        match class {
            ClassType::Soldat => 9,        // 3 combat + 3 summons + 3 orders
            ClassType::MageDeGuerre => 11, // 9 spells (3 fire/3 ice/3 lightning) + 2 summons
            ClassType::Archer => 11,       // 9 actives (3 tir/3 piège/3 agilité) + 2 summons
            ClassType::Clerc => 15,        // 6 auras + 5 bénis + 1 passive + 3 angelic summons
        }
    }

    /// Expected number of summon skills per class.
    fn expected_summons(class: ClassType) -> usize {
        match class {
            ClassType::Soldat => 3,
            ClassType::Clerc => 3,
            _ => 2,
        }
    }

    #[test]
    fn each_class_has_expected_skill_count() {
        for class in ClassType::all() {
            assert_eq!(skills_for(class).len(), expected_len(class), "{class:?}");
        }
    }

    #[test]
    fn each_class_has_its_summon_skills() {
        for class in ClassType::all() {
            let summons = skills_for(class)
                .into_iter()
                .filter(|s| matches!(s.effect, SkillEffect::Summon { .. }))
                .count();
            assert_eq!(summons, expected_summons(class), "{class:?}");
        }
    }

    #[test]
    fn soldat_fields_the_three_synergistic_hosts() {
        let names: Vec<String> = skills_for(ClassType::Soldat)
            .into_iter()
            .filter(|s| matches!(s.effect, SkillEffect::Summon { .. }))
            .map(|s| s.name)
            .collect();
        assert_eq!(names, ["Paysans", "Miliciens", "Guerriers"]);
    }

    #[test]
    fn skill_ids_unique() {
        let mut ids: Vec<u32> = ClassType::all().into_iter().flat_map(skills_for).map(|s| s.id).collect();
        let count = ids.len();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), count, "duplicate skill ids");
    }

    #[test]
    fn every_class_lists_three_branches() {
        for class in ClassType::all() {
            assert_eq!(branches_for_class(class).len(), 3, "{class:?}");
        }
    }

    #[test]
    fn every_branch_skill_id_resolves_to_a_real_active() {
        for class in ClassType::all() {
            let class_skills: Vec<u32> = skills_for(class).into_iter().map(|s| s.id).collect();
            for b in branches_for_class(class) {
                for id in &b.skill_ids {
                    assert!(class_skills.contains(id), "{class:?}/{} → {id} missing", b.name);
                }
            }
        }
    }

    #[test]
    fn skill_def_for_uses_owner_class_so_invest_passes() {
        // A Soldat player unlocking the Mage "Feu" branch should be able to
        // invest in Boule de feu (3001): the SkillDef must carry the player's
        // mge Class (Warrior), not the originating one (Sorceress).
        let def = skill_def_for(3001, ClassType::Soldat).expect("3001 exists");
        assert_eq!(def.class, ClassType::Soldat.to_mge());
        assert_eq!(def.id, 3001);
        assert!(def.prerequisites.is_empty(), "first skill of a branch has no prereq");
    }

    #[test]
    fn skill_def_for_second_in_branch_requires_first() {
        // Taille (1002) is the second Combat skill — Grand Coup (1001) gates it.
        let def = skill_def_for(1002, ClassType::Soldat).expect("1002 exists");
        assert_eq!(def.prerequisites, vec![1001]);
    }

    #[test]
    fn prerequisite_for_first_skill_of_every_branch_is_none() {
        for class in ClassType::all() {
            for b in branches_for_class(class) {
                let first = b.skill_ids[0];
                assert_eq!(prerequisite_for(first, class), None, "{class:?}/{} first", b.name);
            }
        }
    }

    #[test]
    fn prerequisite_for_chains_branch_order() {
        // Soldat Combat: 1001 -> 1002 -> 1003
        assert_eq!(prerequisite_for(1002, ClassType::Soldat), Some(1001));
        assert_eq!(prerequisite_for(1003, ClassType::Soldat), Some(1002));
        // Mage Feu: 3001 -> 3002 -> 3003
        assert_eq!(prerequisite_for(3003, ClassType::MageDeGuerre), Some(3002));
        // Clerc Soutien: 5001 -> 5002 -> ... -> 5006
        assert_eq!(prerequisite_for(5006, ClassType::Clerc), Some(5005));
        assert_eq!(prerequisite_for(5002, ClassType::Clerc), Some(5001));
    }
}
