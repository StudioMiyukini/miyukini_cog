use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Skill Grid — 13×13 grid of scroll slots
// Scrolls are bought or dropped, then inserted into unlocked slots.
// Slots unlock from center outward in cardinal directions (1 per level).
// ---------------------------------------------------------------------------

pub const GRID_COLS: usize = 13;
pub const GRID_ROWS: usize = 13;
pub const GRID_CENTER_X: usize = 6;
pub const GRID_CENTER_Y: usize = 6;
pub const GRID_TOTAL: usize = GRID_COLS * GRID_ROWS; // 169

/// Flat index from (col, row).
#[inline]
pub const fn grid_index(col: usize, row: usize) -> usize {
    row * GRID_COLS + col
}

/// (col, row) from flat index.
#[inline]
pub const fn grid_pos(index: usize) -> (usize, usize) {
    (index % GRID_COLS, index / GRID_COLS)
}

pub const CENTER_INDEX: usize = GRID_CENTER_Y * GRID_COLS + GRID_CENTER_X;

// Backward compat aliases used by main.rs
pub const DISK_NODE_COUNT: usize = GRID_TOTAL;
pub const CENTER_NODE: usize = CENTER_INDEX;

// ---------------------------------------------------------------------------
// Slot types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SlotKind {
    Passive, // green — passive stat scrolls
    Active,  // pink  — active skill scrolls
    Special, // white — special/keystone scrolls
}

// ---------------------------------------------------------------------------
// Grid layout — which type each cell is
// ---------------------------------------------------------------------------

// P=Passive, A=Active, S=Special
// Center (6,6) is the origin diamond (not a regular slot).
// Symmetric across both axes.
use SlotKind::{Passive as P, Active as A, Special as S};
const SLOT_KINDS: [[SlotKind; GRID_COLS]; GRID_ROWS] = [
    //  0  1  2  3  4  5  6  7  8  9 10 11 12
    [S, P, P, A, P, P, A, P, P, A, P, P, S], // row 0
    [P, A, P, P, P, A, P, A, P, P, P, A, P], // row 1
    [P, P, P, S, P, P, P, P, P, S, P, P, P], // row 2
    [A, P, P, P, A, P, A, P, A, P, P, P, A], // row 3
    [P, P, P, A, P, P, P, P, P, A, P, P, P], // row 4
    [P, A, P, P, A, P, P, P, A, P, P, A, P], // row 5
    [A, P, P, A, P, P, P, P, P, A, P, P, A], // row 6 (center at col 6)
    [P, A, P, P, A, P, P, P, A, P, P, A, P], // row 7
    [P, P, P, A, P, P, P, P, P, A, P, P, P], // row 8
    [A, P, P, P, A, P, A, P, A, P, P, P, A], // row 9
    [P, P, P, S, P, P, P, P, P, S, P, P, P], // row 10
    [P, A, P, P, P, A, P, A, P, P, P, A, P], // row 11
    [S, P, P, A, P, P, A, P, P, A, P, P, S], // row 12
];

/// Returns the slot kind at (col, row). Returns None for the center origin.
pub fn slot_kind_at(col: usize, row: usize) -> Option<SlotKind> {
    if col == GRID_CENTER_X && row == GRID_CENTER_Y {
        return None; // center origin
    }
    if col >= GRID_COLS || row >= GRID_ROWS {
        return None;
    }
    Some(SLOT_KINDS[row][col])
}

/// Color for a slot kind (for rendering).
pub fn slot_color(kind: SlotKind) -> [f32; 4] {
    match kind {
        SlotKind::Passive => [0.55, 0.85, 0.55, 1.0], // green
        SlotKind::Active  => [0.90, 0.55, 0.55, 1.0], // pink/red
        SlotKind::Special => [0.85, 0.85, 0.85, 1.0], // white
    }
}

/// Slot kind short label.
pub fn slot_kind_label(kind: SlotKind) -> &'static [u8] {
    match kind {
        SlotKind::Passive => b"PAS",
        SlotKind::Active  => b"ACT",
        SlotKind::Special => b"SPE",
    }
}

// ---------------------------------------------------------------------------
// Canvas positions for rendering
// ---------------------------------------------------------------------------

pub const SLOT_SIZE: f32 = 32.0;
pub const SLOT_GAP: f32 = 6.0;
pub const SLOT_STRIDE: f32 = SLOT_SIZE + SLOT_GAP; // 38.0

/// Canvas position of slot (col, row). Center is at (0, 0).
#[allow(clippy::cast_precision_loss)]
pub fn slot_canvas_pos(col: usize, row: usize) -> [f32; 2] {
    let x = (col as f32 - GRID_CENTER_X as f32) * SLOT_STRIDE;
    let y = (row as f32 - GRID_CENTER_Y as f32) * SLOT_STRIDE;
    [x, y]
}

/// Total canvas size needed for the grid.
#[allow(clippy::cast_precision_loss)]
pub const GRID_CANVAS_W: f32 = GRID_COLS as f32 * 38.0;
pub const GRID_CANVAS_H: f32 = GRID_ROWS as f32 * 38.0;

// Backward compat
pub const DISK_CANVAS_SIZE: f32 = GRID_CANVAS_W;

// ---------------------------------------------------------------------------
// Scroll stat types — what passive scrolls can grant
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScrollStat {
    // --- Percent bonuses (+x%) ---
    AttackSpeedPct,
    CastSpeedPct,
    HpPct,
    ManaPct,
    PhysDmgPct,
    FireDmgPct,
    ColdDmgPct,
    LightningDmgPct,
    PoisonDmgPct,
    ArmorPct,
    FireResistPct,
    ColdResistPct,
    LightningResistPct,
    PoisonResistPct,
    AllResistPct,
    EndurancePct,
    MagicFindPct,
    BlockChancePct,
    DodgeChancePct,
    AttackRangePct,
    SpellRangePct,
    SpellDurationPct,
    AoeSizePct,
    SpellCostReductionPct,
    BuffDurationPct,
    DotDurationPct,
    // Civil skill speeds
    MiningSpeedPct,
    WoodcuttingSpeedPct,
    AgricultureSpeedPct,
    ConstructionSpeedPct,
    ForgeSpeedPct,
    CarpentrySpeedPct,
    AlchemySpeedPct,
    // General
    MoveSpeedPct,
    PushbackPct,
    XpBonusPct,
    GoldBonusPct,
    CritChancePct,
    CritDamagePct,
    LifeStealPct,
    ManaStealPct,
    HpRegenPct,
    MpRegenPct,
    KnockbackPct,
    PotionEfficiencyPct,

    // --- Flat bonuses (+x) ---
    StrFlat,
    DexFlat,
    VitFlat,
    EneFlat,
    HpFlat,
    ManaFlat,
    PhysDmgFlat,
    FireDmgFlat,
    ColdDmgFlat,
    LightningDmgFlat,
    PoisonDmgFlat,
    ArmorFlat,
    FireResistFlat,
    ColdResistFlat,
    LightningResistFlat,
    PoisonResistFlat,
    AllResistFlat,
    EnduranceFlat,
    SummonCountFlat,
    CompanionCountFlat,
    AoeSizeFlat,
    DotDurationFlat,
    HpRegenFlat,
    MpRegenFlat,
}

impl ScrollStat {
    /// Short display name for HUD (max ~8 chars).
    pub fn short_name(self) -> &'static [u8] {
        match self {
            Self::AttackSpeedPct => b"ASPD%",
            Self::CastSpeedPct => b"CSPD%",
            Self::HpPct => b"HP%",
            Self::ManaPct => b"MP%",
            Self::PhysDmgPct => b"PDMG%",
            Self::FireDmgPct => b"FIRE%",
            Self::ColdDmgPct => b"COLD%",
            Self::LightningDmgPct => b"LTNG%",
            Self::PoisonDmgPct => b"POIS%",
            Self::ArmorPct => b"ARM%",
            Self::FireResistPct => b"FRES%",
            Self::ColdResistPct => b"CRES%",
            Self::LightningResistPct => b"LRES%",
            Self::PoisonResistPct => b"PRES%",
            Self::AllResistPct => b"ARES%",
            Self::EndurancePct => b"END%",
            Self::MagicFindPct => b"MF%",
            Self::BlockChancePct => b"BLK%",
            Self::DodgeChancePct => b"DDG%",
            Self::AttackRangePct => b"ARNG%",
            Self::SpellRangePct => b"SRNG%",
            Self::SpellDurationPct => b"SDUR%",
            Self::AoeSizePct => b"AOE%",
            Self::SpellCostReductionPct => b"SCRD%",
            Self::BuffDurationPct => b"BDUR%",
            Self::DotDurationPct => b"DOT%",
            Self::MiningSpeedPct => b"MINE%",
            Self::WoodcuttingSpeedPct => b"WOOD%",
            Self::AgricultureSpeedPct => b"AGRI%",
            Self::ConstructionSpeedPct => b"CNST%",
            Self::ForgeSpeedPct => b"FRGE%",
            Self::CarpentrySpeedPct => b"CARP%",
            Self::AlchemySpeedPct => b"ALCH%",
            Self::MoveSpeedPct => b"MSPD%",
            Self::PushbackPct => b"PUSH%",
            Self::XpBonusPct => b"XP%",
            Self::GoldBonusPct => b"GOLD%",
            Self::CritChancePct => b"CRIT%",
            Self::CritDamagePct => b"CRTD%",
            Self::LifeStealPct => b"LSTL%",
            Self::ManaStealPct => b"MSTL%",
            Self::HpRegenPct => b"HREG%",
            Self::MpRegenPct => b"MREG%",
            Self::KnockbackPct => b"KB%",
            Self::PotionEfficiencyPct => b"POT%",
            Self::StrFlat => b"+STR",
            Self::DexFlat => b"+DEX",
            Self::VitFlat => b"+VIT",
            Self::EneFlat => b"+ENE",
            Self::HpFlat => b"+HP",
            Self::ManaFlat => b"+MP",
            Self::PhysDmgFlat => b"+PDMG",
            Self::FireDmgFlat => b"+FIRE",
            Self::ColdDmgFlat => b"+COLD",
            Self::LightningDmgFlat => b"+LTNG",
            Self::PoisonDmgFlat => b"+POIS",
            Self::ArmorFlat => b"+ARM",
            Self::FireResistFlat => b"+FRES",
            Self::ColdResistFlat => b"+CRES",
            Self::LightningResistFlat => b"+LRES",
            Self::PoisonResistFlat => b"+PRES",
            Self::AllResistFlat => b"+ARES",
            Self::EnduranceFlat => b"+END",
            Self::SummonCountFlat => b"+SUMM",
            Self::CompanionCountFlat => b"+COMP",
            Self::AoeSizeFlat => b"+AOE",
            Self::DotDurationFlat => b"+DOT",
            Self::HpRegenFlat => b"+HREG",
            Self::MpRegenFlat => b"+MREG",
        }
    }
}

// ---------------------------------------------------------------------------
// Special scroll effects (go in Special slots)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpecialEffect {
    /// Cast a skill at level X every N frames.
    AutoCast { skill_id: u8, level: u8, interval: u32 },
    /// Reflect X% damage to attacker.
    DamageReflect { pct: f32 },
    /// Add X extra projectiles to projectile skills.
    ExtraProjectiles { count: u8 },
    /// Summons explode on death for X% of their HP as AoE.
    SummonExplodeOnDeath { hp_pct: f32 },
    /// Cannot be stunned, but cannot dodge.
    UnstoppableNoEvade,
    /// Potions are X% more effective.
    PotionEfficiency { pct: f32 },
    /// Mana added to HP, mana set to 0; all mana costs become HP costs.
    BloodMagic,
    /// Double max mana, halve max HP.
    DoubleManaHalveHp,
    /// X% of damage taken reduces mana instead of HP.
    ManaShield { pct: f32 },
    /// Companions cost X% less to resurrect.
    CompanionResurrectDiscount { pct: f32 },
    /// +X% to equipment piece effects.
    EquipmentEffectBonus { pct: f32 },
    /// Life steal doubled, no HP regen (like old VaalPact).
    VaalPact,
    /// Crits deal 3× instead of 2× (like old LethalStrike).
    LethalStrike,
}

impl SpecialEffect {
    pub fn name(self) -> &'static [u8] {
        match self {
            Self::AutoCast { .. } => b"AUTO-CAST",
            Self::DamageReflect { .. } => b"DMG REFLECT",
            Self::ExtraProjectiles { .. } => b"EXTRA PROJ",
            Self::SummonExplodeOnDeath { .. } => b"EXPLODE",
            Self::UnstoppableNoEvade => b"UNSTOPPABLE",
            Self::PotionEfficiency { .. } => b"POTION+",
            Self::BloodMagic => b"BLOOD MAGIC",
            Self::DoubleManaHalveHp => b"2x MANA",
            Self::ManaShield { .. } => b"MANA SHIELD",
            Self::CompanionResurrectDiscount { .. } => b"COMP DISC",
            Self::EquipmentEffectBonus { .. } => b"EQUIP+",
            Self::VaalPact => b"VAAL PACT",
            Self::LethalStrike => b"LETHAL",
        }
    }
}

// ---------------------------------------------------------------------------
// Slot content — what's inserted in a slot
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SlotContent {
    /// Passive scroll: one stat bonus with a value.
    Passive { stat: ScrollStat, value: f32 },
    /// Active skill scroll: grants an active skill (placeholder — skills TBD).
    Active { skill_id: u8, level: u8 },
    /// Special scroll: unique gameplay modifier.
    Special(SpecialEffect),
}

impl SlotContent {
    /// Short display text for the slot icon.
    pub fn short_label(&self) -> &'static [u8] {
        match self {
            Self::Passive { stat, .. } => stat.short_name(),
            Self::Active { .. } => b"SKILL",
            Self::Special(eff) => eff.name(),
        }
    }

    /// Which slot kind this content fits in.
    pub fn required_slot_kind(&self) -> SlotKind {
        match self {
            Self::Passive { .. } => SlotKind::Passive,
            Self::Active { .. } => SlotKind::Active,
            Self::Special(_) => SlotKind::Special,
        }
    }
}

// ---------------------------------------------------------------------------
// Scroll item — what the player carries in inventory
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScrollItem {
    pub content: SlotContent,
}

// ---------------------------------------------------------------------------
// Skill Grid state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SkillGrid {
    /// Which slots are unlocked. Center is always unlocked.
    pub unlocked: Vec<bool>,
    /// What scroll is inserted in each slot (None = empty/locked).
    pub slots: Vec<Option<SlotContent>>,
}

impl Default for SkillGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillGrid {
    pub fn new() -> Self {
        let mut unlocked = vec![false; GRID_TOTAL];
        unlocked[CENTER_INDEX] = true;
        Self {
            unlocked,
            slots: vec![None; GRID_TOTAL],
        }
    }

    /// Manhattan distance from center.
    pub fn distance_from_center(col: usize, row: usize) -> usize {
        let dx = if col >= GRID_CENTER_X { col - GRID_CENTER_X } else { GRID_CENTER_X - col };
        let dy = if row >= GRID_CENTER_Y { row - GRID_CENTER_Y } else { GRID_CENTER_Y - row };
        dx + dy
    }

    /// Can this slot be unlocked? Must be cardinally adjacent to an unlocked slot.
    pub fn can_unlock(&self, col: usize, row: usize) -> bool {
        if col >= GRID_COLS || row >= GRID_ROWS { return false; }
        let idx = grid_index(col, row);
        if self.unlocked[idx] { return false; }
        // Must be adjacent (cardinal) to at least one unlocked slot
        self.has_unlocked_neighbor(col, row)
    }

    fn has_unlocked_neighbor(&self, col: usize, row: usize) -> bool {
        if col > 0 && self.unlocked[grid_index(col - 1, row)] { return true; }
        if col + 1 < GRID_COLS && self.unlocked[grid_index(col + 1, row)] { return true; }
        if row > 0 && self.unlocked[grid_index(col, row - 1)] { return true; }
        if row + 1 < GRID_ROWS && self.unlocked[grid_index(col, row + 1)] { return true; }
        false
    }

    /// Unlock a slot. Returns true if successful.
    pub fn unlock(&mut self, col: usize, row: usize) -> bool {
        if !self.can_unlock(col, row) { return false; }
        self.unlocked[grid_index(col, row)] = true;
        true
    }

    /// Insert a scroll into an unlocked empty slot. Returns false if invalid.
    pub fn insert(&mut self, col: usize, row: usize, content: SlotContent) -> bool {
        if col >= GRID_COLS || row >= GRID_ROWS { return false; }
        let idx = grid_index(col, row);
        if !self.unlocked[idx] { return false; }
        if self.slots[idx].is_some() { return false; }
        // Check slot kind compatibility (center can't hold scrolls)
        let kind = match slot_kind_at(col, row) {
            Some(k) => k,
            None => return false, // center
        };
        if content.required_slot_kind() != kind { return false; }
        self.slots[idx] = Some(content);
        true
    }

    /// Remove a scroll from a slot. Returns the scroll if any.
    pub fn remove(&mut self, col: usize, row: usize) -> Option<SlotContent> {
        if col >= GRID_COLS || row >= GRID_ROWS { return None; }
        self.slots[grid_index(col, row)].take()
    }

    /// Count unlocked slots (including center).
    pub fn count_unlocked(&self) -> usize {
        self.unlocked.iter().filter(|&&u| u).count()
    }

    /// Count filled slots.
    pub fn count_filled(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// Compute aggregate bonuses from all inserted scrolls.
    pub fn compute_bonuses(&self) -> DiskBonuses {
        let mut b = DiskBonuses::default();
        for slot in &self.slots {
            let Some(content) = slot else { continue };
            match content {
                SlotContent::Passive { stat, value } => apply_stat(&mut b, *stat, *value),
                SlotContent::Active { .. } => {
                    // Active skills — will be implemented later
                }
                SlotContent::Special(eff) => apply_special(&mut b, *eff),
            }
        }
        b
    }
}

fn apply_stat(b: &mut DiskBonuses, stat: ScrollStat, v: f32) {
    match stat {
        ScrollStat::AttackSpeedPct => b.attack_speed_pct += v,
        ScrollStat::CastSpeedPct => b.cast_speed_pct += v,
        ScrollStat::HpPct => b.max_hp_pct += v,
        ScrollStat::ManaPct => b.max_mp_pct += v,
        ScrollStat::PhysDmgPct => b.melee_dmg_pct += v,
        ScrollStat::FireDmgPct => b.fire_dmg_pct += v,
        ScrollStat::ColdDmgPct => b.cold_dmg_pct += v,
        ScrollStat::LightningDmgPct => b.lightning_dmg_pct += v,
        ScrollStat::PoisonDmgPct => b.poison_dmg_pct += v,
        ScrollStat::ArmorPct => b.armor_pct += v,
        ScrollStat::FireResistPct => b.fire_resist += v,
        ScrollStat::ColdResistPct => b.cold_resist += v,
        ScrollStat::LightningResistPct => b.lightning_resist += v,
        ScrollStat::PoisonResistPct => b.poison_resist += v,
        ScrollStat::AllResistPct => b.all_resist += v,
        ScrollStat::EndurancePct => b.endurance_pct += v,
        ScrollStat::MagicFindPct => b.magic_find_pct += v,
        ScrollStat::BlockChancePct => b.block_chance_pct += v,
        ScrollStat::DodgeChancePct => b.dodge_pct += v,
        ScrollStat::AttackRangePct => b.attack_range_pct += v,
        ScrollStat::SpellRangePct => b.spell_range_pct += v,
        ScrollStat::SpellDurationPct => b.spell_duration_pct += v,
        ScrollStat::AoeSizePct => b.aoe_size_pct += v,
        ScrollStat::SpellCostReductionPct => b.spell_cost_reduction_pct += v,
        ScrollStat::BuffDurationPct => b.buff_duration_pct += v,
        ScrollStat::DotDurationPct => b.dot_duration_pct += v,
        ScrollStat::MiningSpeedPct => b.mining_speed_pct += v,
        ScrollStat::WoodcuttingSpeedPct => b.woodcutting_speed_pct += v,
        ScrollStat::AgricultureSpeedPct => b.agriculture_speed_pct += v,
        ScrollStat::ConstructionSpeedPct => b.construction_speed_pct += v,
        ScrollStat::ForgeSpeedPct => b.forge_speed_pct += v,
        ScrollStat::CarpentrySpeedPct => b.carpentry_speed_pct += v,
        ScrollStat::AlchemySpeedPct => b.alchemy_speed_pct += v,
        ScrollStat::MoveSpeedPct => b.move_speed_pct += v,
        ScrollStat::PushbackPct => b.knockback_pct += v,
        ScrollStat::XpBonusPct => b.xp_bonus_pct += v,
        ScrollStat::GoldBonusPct => b.gold_bonus_pct += v,
        ScrollStat::CritChancePct => b.crit_chance += v,
        ScrollStat::CritDamagePct => b.crit_damage += v,
        ScrollStat::LifeStealPct => b.life_steal_pct += v,
        ScrollStat::ManaStealPct => b.mana_steal_pct += v,
        ScrollStat::HpRegenPct => b.hp_regen += v,
        ScrollStat::MpRegenPct => b.mp_regen += v,
        ScrollStat::KnockbackPct => b.knockback_pct += v,
        ScrollStat::PotionEfficiencyPct => b.potion_efficiency_pct += v,
        ScrollStat::StrFlat => b.bonus_str += v,
        ScrollStat::DexFlat => b.bonus_dex += v,
        ScrollStat::VitFlat => b.bonus_vit += v,
        ScrollStat::EneFlat => b.bonus_ene += v,
        ScrollStat::HpFlat => b.max_hp_flat += v,
        ScrollStat::ManaFlat => b.max_mp_flat += v,
        ScrollStat::PhysDmgFlat => b.melee_dmg_flat += v,
        ScrollStat::FireDmgFlat => b.fire_dmg_flat += v,
        ScrollStat::ColdDmgFlat => b.cold_dmg_flat += v,
        ScrollStat::LightningDmgFlat => b.lightning_dmg_flat += v,
        ScrollStat::PoisonDmgFlat => b.poison_dmg_flat += v,
        ScrollStat::ArmorFlat => b.armor_flat += v,
        ScrollStat::FireResistFlat => b.fire_resist += v,
        ScrollStat::ColdResistFlat => b.cold_resist += v,
        ScrollStat::LightningResistFlat => b.lightning_resist += v,
        ScrollStat::PoisonResistFlat => b.poison_resist += v,
        ScrollStat::AllResistFlat => b.all_resist += v,
        ScrollStat::EnduranceFlat => b.endurance_flat += v,
        ScrollStat::SummonCountFlat => b.summon_count_bonus += v as i32,
        ScrollStat::CompanionCountFlat => b.companion_count_bonus += v as i32,
        ScrollStat::AoeSizeFlat => b.aoe_size_flat += v,
        ScrollStat::DotDurationFlat => b.dot_duration_flat += v,
        ScrollStat::HpRegenFlat => b.hp_regen += v,
        ScrollStat::MpRegenFlat => b.mp_regen += v,
    }
}

fn apply_special(b: &mut DiskBonuses, eff: SpecialEffect) {
    match eff {
        SpecialEffect::DamageReflect { pct } => b.damage_reflect_pct += pct,
        SpecialEffect::ExtraProjectiles { count } => b.extra_projectiles += count,
        SpecialEffect::SummonExplodeOnDeath { hp_pct } => b.summon_explode_hp_pct = hp_pct,
        SpecialEffect::UnstoppableNoEvade => b.unstoppable_no_evade = true,
        SpecialEffect::PotionEfficiency { pct } => b.potion_efficiency_pct += pct,
        SpecialEffect::BloodMagic => b.blood_magic = true,
        SpecialEffect::DoubleManaHalveHp => b.double_mana_halve_hp = true,
        SpecialEffect::ManaShield { pct } => b.mana_shield_pct += pct,
        SpecialEffect::CompanionResurrectDiscount { pct } => b.companion_resurrect_discount += pct,
        SpecialEffect::EquipmentEffectBonus { pct } => b.equipment_effect_bonus += pct,
        SpecialEffect::VaalPact => b.vaal_pact = true,
        SpecialEffect::LethalStrike => b.lethal_strike = true,
        SpecialEffect::AutoCast { skill_id, level, interval } => {
            if b.auto_cast_count < 4 {
                let i = b.auto_cast_count as usize;
                b.auto_casts[i] = (skill_id, level, interval);
                b.auto_cast_count += 1;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Aggregated bonuses from the skill grid
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Default)]
pub struct DiskBonuses {
    // --- Flat stats (from old system, kept for combat compat) ---
    pub bonus_str: f32,
    pub bonus_dex: f32,
    pub bonus_vit: f32,
    pub bonus_ene: f32,
    pub melee_dmg_pct: f32,
    pub spell_dmg_pct: f32,
    pub fire_dmg_pct: f32,
    pub cold_dmg_pct: f32,
    pub lightning_dmg_pct: f32,
    pub poison_dmg_pct: f32,
    pub crit_chance: f32,
    pub crit_damage: f32,
    pub attack_speed_pct: f32,
    pub cast_speed_pct: f32,
    pub max_hp_flat: f32,
    pub max_hp_pct: f32,
    pub max_mp_flat: f32,
    pub max_mp_pct: f32,
    pub armor_flat: f32,
    pub armor_pct: f32,
    pub dodge_pct: f32,
    pub all_resist: f32,
    pub fire_resist: f32,
    pub cold_resist: f32,
    pub lightning_resist: f32,
    pub poison_resist: f32,
    pub hp_regen: f32,
    pub mp_regen: f32,
    pub life_steal_pct: f32,
    pub mana_steal_pct: f32,
    pub move_speed_pct: f32,
    pub spell_cost_reduction_pct: f32,
    pub knockback_pct: f32,
    pub potion_efficiency_pct: f32,

    // --- New flat bonuses ---
    pub melee_dmg_flat: f32,
    pub fire_dmg_flat: f32,
    pub cold_dmg_flat: f32,
    pub lightning_dmg_flat: f32,
    pub poison_dmg_flat: f32,
    pub endurance_flat: f32,
    pub endurance_pct: f32,
    pub aoe_size_flat: f32,
    pub aoe_size_pct: f32,
    pub dot_duration_flat: f32,
    pub dot_duration_pct: f32,

    // --- New percent bonuses ---
    pub magic_find_pct: f32,
    pub block_chance_pct: f32,
    pub attack_range_pct: f32,
    pub spell_range_pct: f32,
    pub spell_duration_pct: f32,
    pub buff_duration_pct: f32,
    pub xp_bonus_pct: f32,
    pub gold_bonus_pct: f32,

    // --- Civil skill speed bonuses ---
    pub mining_speed_pct: f32,
    pub woodcutting_speed_pct: f32,
    pub agriculture_speed_pct: f32,
    pub construction_speed_pct: f32,
    pub forge_speed_pct: f32,
    pub carpentry_speed_pct: f32,
    pub alchemy_speed_pct: f32,

    // --- Summon/companion ---
    pub summon_count_bonus: i32,
    pub companion_count_bonus: i32,

    // --- Special effect flags ---
    pub damage_reflect_pct: f32,
    pub extra_projectiles: u8,
    pub summon_explode_hp_pct: f32,
    pub unstoppable_no_evade: bool,
    pub blood_magic: bool,
    pub double_mana_halve_hp: bool,
    pub mana_shield_pct: f32,
    pub companion_resurrect_discount: f32,
    pub equipment_effect_bonus: f32,
    pub vaal_pact: bool,
    pub lethal_strike: bool,
    pub auto_casts: [(u8, u8, u32); 4],
    pub auto_cast_count: u8,
}

impl DiskBonuses {
    /// Effective fire damage multiplier.
    pub fn fire_mult(&self) -> f32 {
        1.0 + self.fire_dmg_pct + self.spell_dmg_pct
    }
    /// Effective cold damage multiplier.
    pub fn cold_mult(&self) -> f32 {
        1.0 + self.cold_dmg_pct + self.spell_dmg_pct
    }
    /// Effective lightning damage multiplier.
    pub fn lightning_mult(&self) -> f32 {
        1.0 + self.lightning_dmg_pct + self.spell_dmg_pct
    }
}

// ---------------------------------------------------------------------------
// Adjacency helpers (for backward compat with main.rs rendering)
// ---------------------------------------------------------------------------

/// Returns true if two grid slots (by flat index) are cardinally adjacent.
pub fn nodes_adjacent(a: usize, b: usize) -> bool {
    if a == b || a >= GRID_TOTAL || b >= GRID_TOTAL { return false; }
    let (ac, ar) = grid_pos(a);
    let (bc, br) = grid_pos(b);
    let dc = if ac > bc { ac - bc } else { bc - ac };
    let dr = if ar > br { ar - br } else { br - ar };
    (dc == 1 && dr == 0) || (dc == 0 && dr == 1)
}

/// Backward compat: check if a slot can be allocated (= unlocked).
pub fn can_allocate(node: usize, allocated: &[bool]) -> bool {
    if node >= GRID_TOTAL { return false; }
    if allocated.get(node).copied().unwrap_or(false) { return false; }
    let (col, row) = grid_pos(node);
    // Check cardinal neighbors
    if col > 0 && allocated.get(grid_index(col - 1, row)).copied().unwrap_or(false) { return true; }
    if col + 1 < GRID_COLS && allocated.get(grid_index(col + 1, row)).copied().unwrap_or(false) { return true; }
    if row > 0 && allocated.get(grid_index(col, row - 1)).copied().unwrap_or(false) { return true; }
    if row + 1 < GRID_ROWS && allocated.get(grid_index(col, row + 1)).copied().unwrap_or(false) { return true; }
    false
}

/// Backward compat: canvas position by flat index.
pub fn node_canvas_pos(node: usize) -> [f32; 2] {
    let (col, row) = grid_pos(node);
    slot_canvas_pos(col, row)
}

/// Backward compat: color for a slot by flat index.
pub fn node_color(node: usize) -> [f32; 4] {
    if node == CENTER_INDEX {
        return [1.0, 1.0, 1.0, 1.0];
    }
    let (col, row) = grid_pos(node);
    match slot_kind_at(col, row) {
        Some(kind) => slot_color(kind),
        None => [1.0, 1.0, 1.0, 1.0],
    }
}

/// Count unlocked slots.
pub fn count_allocated(allocated: &[bool]) -> usize {
    allocated.iter().filter(|&&a| a).count()
}

// ---------------------------------------------------------------------------
// Random scroll generation (for drops and shops)
// ---------------------------------------------------------------------------

/// Common passive stats that can appear on scroll drops (early-game friendly subset).
const COMMON_PASSIVE_STATS: [ScrollStat; 20] = [
    ScrollStat::HpFlat, ScrollStat::ManaFlat, ScrollStat::StrFlat, ScrollStat::DexFlat,
    ScrollStat::VitFlat, ScrollStat::EneFlat, ScrollStat::PhysDmgPct, ScrollStat::FireDmgPct,
    ScrollStat::ColdDmgPct, ScrollStat::LightningDmgPct, ScrollStat::ArmorPct,
    ScrollStat::FireResistPct, ScrollStat::ColdResistPct, ScrollStat::AllResistPct,
    ScrollStat::AttackSpeedPct, ScrollStat::CritChancePct, ScrollStat::MoveSpeedPct,
    ScrollStat::HpRegenFlat, ScrollStat::MpRegenFlat, ScrollStat::LifeStealPct,
];

/// Generate a random passive scroll based on a hash seed.
/// Value scales with `ilvl` (item level / player level).
#[allow(clippy::cast_precision_loss)]
pub fn random_passive_scroll(seed: u32, ilvl: u32) -> SlotContent {
    let stat_idx = (seed as usize) % COMMON_PASSIVE_STATS.len();
    let stat = COMMON_PASSIVE_STATS[stat_idx];
    let base = match stat {
        // Flat stats: 1-5 base + ilvl scaling
        ScrollStat::HpFlat => 3.0 + (seed / 100 % 5) as f32 + ilvl as f32 * 0.5,
        ScrollStat::ManaFlat => 2.0 + (seed / 100 % 4) as f32 + ilvl as f32 * 0.3,
        ScrollStat::StrFlat | ScrollStat::DexFlat | ScrollStat::VitFlat | ScrollStat::EneFlat =>
            1.0 + (seed / 100 % 3) as f32 + ilvl as f32 * 0.2,
        ScrollStat::HpRegenFlat => 0.005 + (seed / 100 % 3) as f32 * 0.003 + ilvl as f32 * 0.001,
        ScrollStat::MpRegenFlat => 0.003 + (seed / 100 % 3) as f32 * 0.002 + ilvl as f32 * 0.001,
        // Percent stats: 2-8% base + ilvl scaling
        _ => 0.02 + (seed / 100 % 6) as f32 * 0.01 + ilvl as f32 * 0.005,
    };
    SlotContent::Passive { stat, value: base }
}

/// Generate a random special scroll based on a hash seed.
#[allow(clippy::cast_precision_loss)]
pub fn random_special_scroll(seed: u32) -> SlotContent {
    let variant = seed % 10;
    let eff = match variant {
        0 => SpecialEffect::DamageReflect { pct: 0.05 + (seed / 10 % 5) as f32 * 0.02 },
        1 => SpecialEffect::ExtraProjectiles { count: 1 + (seed / 10 % 2) as u8 },
        2 => SpecialEffect::PotionEfficiency { pct: 0.20 + (seed / 10 % 4) as f32 * 0.10 },
        3 => SpecialEffect::BloodMagic,
        4 => SpecialEffect::ManaShield { pct: 0.15 + (seed / 10 % 4) as f32 * 0.05 },
        5 => SpecialEffect::VaalPact,
        6 => SpecialEffect::LethalStrike,
        7 => SpecialEffect::UnstoppableNoEvade,
        8 => SpecialEffect::DoubleManaHalveHp,
        _ => SpecialEffect::EquipmentEffectBonus { pct: 0.10 + (seed / 10 % 5) as f32 * 0.05 },
    };
    SlotContent::Special(eff)
}

/// Generate a random scroll drop. `scroll_seed` determines type and stats.
/// Passive scrolls are most common, Special scrolls are rare.
pub fn random_scroll_drop(seed: u32, ilvl: u32) -> SlotContent {
    if seed % 20 == 0 {
        // 5% chance: special scroll
        random_special_scroll(seed.wrapping_mul(7_919))
    } else {
        // 95% chance: passive scroll
        random_passive_scroll(seed.wrapping_mul(2_654_435_761), ilvl)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_is_unlocked_by_default() {
        let grid = SkillGrid::new();
        assert!(grid.unlocked[CENTER_INDEX]);
    }

    #[test]
    fn center_has_no_slot_kind() {
        assert!(slot_kind_at(GRID_CENTER_X, GRID_CENTER_Y).is_none());
    }

    #[test]
    fn grid_is_symmetric_lr() {
        for row in 0..GRID_ROWS {
            for col in 0..GRID_COLS {
                let mirror = GRID_COLS - 1 - col;
                assert_eq!(SLOT_KINDS[row][col], SLOT_KINDS[row][mirror],
                    "row {row} not symmetric at col {col} vs {mirror}");
            }
        }
    }

    #[test]
    fn grid_is_symmetric_tb() {
        for row in 0..GRID_ROWS {
            let mirror = GRID_ROWS - 1 - row;
            for col in 0..GRID_COLS {
                assert_eq!(SLOT_KINDS[row][col], SLOT_KINDS[mirror][col],
                    "col {col} not symmetric at row {row} vs {mirror}");
            }
        }
    }

    #[test]
    fn can_unlock_adjacent_to_center() {
        let grid = SkillGrid::new();
        // Cardinal neighbors of center (6,6)
        assert!(grid.can_unlock(5, 6)); // left
        assert!(grid.can_unlock(7, 6)); // right
        assert!(grid.can_unlock(6, 5)); // up
        assert!(grid.can_unlock(6, 7)); // down
        // Diagonal — NOT unlockable
        assert!(!grid.can_unlock(5, 5));
        assert!(!grid.can_unlock(7, 7));
    }

    #[test]
    fn cannot_unlock_already_unlocked() {
        let grid = SkillGrid::new();
        assert!(!grid.can_unlock(GRID_CENTER_X, GRID_CENTER_Y));
    }

    #[test]
    fn unlock_then_insert() {
        let mut grid = SkillGrid::new();
        assert!(grid.unlock(5, 6)); // left of center
        let kind = slot_kind_at(5, 6).unwrap();
        let scroll = match kind {
            SlotKind::Passive => SlotContent::Passive { stat: ScrollStat::StrFlat, value: 5.0 },
            SlotKind::Active => SlotContent::Active { skill_id: 0, level: 1 },
            SlotKind::Special => SlotContent::Special(SpecialEffect::BloodMagic),
        };
        assert!(grid.insert(5, 6, scroll));
        // Can't insert again (already filled)
        assert!(!grid.insert(5, 6, scroll));
    }

    #[test]
    fn insert_wrong_kind_fails() {
        let mut grid = SkillGrid::new();
        grid.unlock(5, 6);
        // (5,6) is Passive in the grid
        let special = SlotContent::Special(SpecialEffect::BloodMagic);
        assert!(!grid.insert(5, 6, special)); // should fail — wrong kind
    }

    #[test]
    fn cannot_insert_at_center() {
        let mut grid = SkillGrid::new();
        let scroll = SlotContent::Passive { stat: ScrollStat::HpFlat, value: 10.0 };
        assert!(!grid.insert(GRID_CENTER_X, GRID_CENTER_Y, scroll));
    }

    #[test]
    fn remove_returns_scroll() {
        let mut grid = SkillGrid::new();
        grid.unlock(5, 6);
        let scroll = SlotContent::Passive { stat: ScrollStat::StrFlat, value: 3.0 };
        grid.insert(5, 6, scroll);
        let removed = grid.remove(5, 6);
        assert_eq!(removed, Some(scroll));
        assert!(grid.slots[grid_index(5, 6)].is_none());
    }

    #[test]
    fn compute_bonuses_adds_stats() {
        let mut grid = SkillGrid::new();
        // Unlock and fill two passive slots
        grid.unlock(5, 6);
        grid.insert(5, 6, SlotContent::Passive { stat: ScrollStat::StrFlat, value: 5.0 });
        grid.unlock(7, 6);
        grid.insert(7, 6, SlotContent::Passive { stat: ScrollStat::StrFlat, value: 3.0 });
        let b = grid.compute_bonuses();
        assert!((b.bonus_str - 8.0).abs() < 0.001);
    }

    #[test]
    fn compute_bonuses_special() {
        let mut grid = SkillGrid::new();
        // Find a Special slot near center and unlock path to it
        // (0,0) is Special — unlock path: (5,6) → (4,6) → (3,6) → (2,6) → (1,6) → (0,6)
        grid.unlock(5, 6);
        grid.unlock(4, 6);
        grid.unlock(3, 6);
        grid.unlock(2, 6);
        grid.unlock(1, 6);
        grid.unlock(0, 6);
        // (0,6) is Active slot. Let's check (0,0) — need longer path.
        // Instead, just verify specials work by inserting in a Special slot.
        // Row 2 col 3 is Special. Path: center → (6,5) → (5,5) → (4,5) → (3,5) → (3,4) → (3,3) → (3,2)
        grid.unlock(6, 5);
        grid.unlock(5, 5);
        grid.unlock(4, 5);
        grid.unlock(3, 5);
        grid.unlock(3, 4);
        grid.unlock(3, 3);
        grid.unlock(3, 2);
        // (3,2) = row 2, col 3 = Special
        assert_eq!(slot_kind_at(3, 2), Some(SlotKind::Special));
        let eff = SlotContent::Special(SpecialEffect::VaalPact);
        assert!(grid.insert(3, 2, eff));
        let b = grid.compute_bonuses();
        assert!(b.vaal_pact);
    }

    #[test]
    fn distance_from_center_correct() {
        assert_eq!(SkillGrid::distance_from_center(6, 6), 0);
        assert_eq!(SkillGrid::distance_from_center(5, 6), 1);
        assert_eq!(SkillGrid::distance_from_center(0, 0), 12);
    }

    #[test]
    fn slot_count() {
        // Count each type
        let mut passive = 0;
        let mut active = 0;
        let mut special = 0;
        for row in 0..GRID_ROWS {
            for col in 0..GRID_COLS {
                match slot_kind_at(col, row) {
                    Some(SlotKind::Passive) => passive += 1,
                    Some(SlotKind::Active) => active += 1,
                    Some(SlotKind::Special) => special += 1,
                    None => {} // center
                }
            }
        }
        // Verify we have a reasonable distribution
        assert!(passive > 100, "should have >100 passive slots, got {passive}");
        assert!(active > 30, "should have >30 active slots, got {active}");
        assert!(special >= 8, "should have >=8 special slots, got {special}");
        assert_eq!(passive + active + special + 1, GRID_TOTAL); // +1 for center
    }
}
