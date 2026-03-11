use crate::constants::*;
use crate::entity::*;

// Re-export for convenience
pub use crate::entity::{Difficulty, Resistances};

/// All server-authoritative player state. No rendering fields.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct PlayerState {
    pub pos: [f32; 2],
    pub hp: f32,
    pub mp: f32,
    pub xp: f32,
    pub level: u32,
    pub gold: u32,
    pub dead: bool,
    pub respawn_timer: u32,
    pub attack_cd: u32,
    pub running: bool,
    pub facing_right: bool,
    pub poison_timer: u32,

    // Stats
    pub stat_str: u32,
    pub stat_dex: u32,
    pub stat_vit: u32,
    pub stat_ene: u32,
    pub stat_points: u32,

    // Skills
    pub active_skill: SkillId,
    pub skill_levels: [u32; SKILL_COUNT],
    pub skill_points: u32,

    // Equipment & belt
    pub equipment: [Option<Equipment>; EQUIP_SLOT_COUNT],
    pub belt: [Option<PotionKind>; 4],

    // Quests
    pub quests: Vec<Quest>,

    // Town portal
    pub portal_active: bool,
    pub portal_pos: [f32; 2],
    pub portal_zone: ZoneId,

    // Zone
    pub current_zone: ZoneId,
    pub waypoints: [bool; ZONE_COUNT],

    // Mercenary
    pub mercenary: Option<Mercenary>,

    // Buffs
    pub buffs: Vec<ActiveBuff>,

    // Resistances
    pub resistances: Resistances,
    pub difficulty: Difficulty,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            pos: [12.0, 12.0],
            hp: PLAYER_MAX_HP,
            mp: PLAYER_MAX_MP,
            xp: 0.0,
            level: 1,
            gold: 0,
            dead: false,
            respawn_timer: 0,
            attack_cd: 0,
            running: false,
            facing_right: true,
            poison_timer: 0,
            stat_str: 0,
            stat_dex: 0,
            stat_vit: 0,
            stat_ene: 0,
            stat_points: 0,
            active_skill: SkillId::Melee,
            skill_levels: [1, 1, 1, 1, 1],
            skill_points: 0,
            equipment: [None; EQUIP_SLOT_COUNT],
            belt: [None; 4],
            quests: vec![
                Quest { name: "Den of Evil", desc: "Clear the Den of Evil", done: false },
                Quest { name: "Blood Raven", desc: "Defeat Blood Raven", done: false },
                Quest { name: "Gold Hoard", desc: "Collect 100 gold", done: false },
            ],
            portal_active: false,
            portal_pos: [0.0, 0.0],
            portal_zone: ZoneId::RogueCamp,
            current_zone: ZoneId::RogueCamp,
            waypoints: {
                let mut wp = [false; ZONE_COUNT];
                wp[ZoneId::RogueCamp as usize] = true;
                wp
            },
            mercenary: None,
            buffs: Vec::new(),
            resistances: Resistances::default(),
            difficulty: Difficulty::Normal,
        }
    }

    // -----------------------------------------------------------------------
    // Aggregate stat methods
    // -----------------------------------------------------------------------

    pub fn total_str(&self) -> u32 {
        self.stat_str + self.equip_bonus_str()
    }

    pub fn total_dex(&self) -> u32 {
        self.stat_dex + self.equip_bonus_dex()
    }

    pub fn total_vit(&self) -> u32 {
        self.stat_vit + self.equip_bonus_vit()
    }

    pub fn total_ene(&self) -> u32 {
        self.stat_ene + self.equip_bonus_ene()
    }

    fn equip_bonus_str(&self) -> u32 {
        self.equipment.iter().flatten().map(|e| e.bonuses().0).sum()
    }

    fn equip_bonus_dex(&self) -> u32 {
        self.equipment.iter().flatten().map(|e| e.bonuses().1).sum()
    }

    fn equip_bonus_vit(&self) -> u32 {
        self.equipment.iter().flatten().map(|e| e.bonuses().2).sum()
    }

    fn equip_bonus_ene(&self) -> u32 {
        self.equipment.iter().flatten().map(|e| e.bonuses().3).sum()
    }

    // -----------------------------------------------------------------------
    // Equipment aggregate effects
    // -----------------------------------------------------------------------

    pub fn equip_flat_damage(&self) -> f32 {
        self.equipment
            .iter()
            .flatten()
            .map(|e| e.flat_damage())
            .sum()
    }

    pub fn equip_flat_defense(&self) -> f32 {
        self.equipment
            .iter()
            .flatten()
            .map(|e| e.flat_defense())
            .sum()
    }

    pub fn equip_hp_percent(&self) -> f32 {
        self.equipment
            .iter()
            .flatten()
            .map(|e| e.hp_percent())
            .sum()
    }

    pub fn equip_move_speed(&self) -> f32 {
        self.equipment
            .iter()
            .flatten()
            .map(|e| e.move_speed_bonus())
            .sum()
    }

    pub fn equip_cd_reduction(&self) -> u32 {
        self.equipment
            .iter()
            .flatten()
            .map(|e| e.cooldown_reduction())
            .sum()
    }

    pub fn equip_spell_percent(&self) -> f32 {
        self.equipment
            .iter()
            .flatten()
            .map(|e| e.spell_percent())
            .sum()
    }

    pub fn equip_life_steal(&self) -> f32 {
        self.equipment
            .iter()
            .flatten()
            .map(|e| e.life_steal())
            .sum()
    }

    pub fn equip_mana_steal(&self) -> f32 {
        self.equipment
            .iter()
            .flatten()
            .map(|e| e.mana_steal())
            .sum()
    }

    // -----------------------------------------------------------------------
    // Derived combat stats
    // -----------------------------------------------------------------------

    #[allow(clippy::cast_precision_loss)]
    pub fn max_hp(&self) -> f32 {
        // D2: base HP + VIT scaling + per-level bonus
        let base = PLAYER_MAX_HP + self.total_vit() as f32 * 2.0
            + (self.level - 1) as f32 * 1.5; // D2: +1-2 HP/level
        base * (1.0 + self.equip_hp_percent())
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn max_mp(&self) -> f32 {
        PLAYER_MAX_MP + self.total_ene() as f32 * 1.5 // D2: +1-2 MP/ENE
            + (self.level - 1) as f32 * 1.0            // D2: +1 MP/level
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn melee_damage(&self) -> f32 {
        // D2: STR adds ~1% ED per point; simplified to flat +0.2/STR
        let str_bonus = self.total_str() as f32 * 0.2;
        let base = ATTACK_DAMAGE + str_bonus + self.equip_flat_damage();
        base * (1.0 + (self.skill_levels[0] - 1) as f32 * 0.10)
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn attack_cooldown(&self) -> u32 {
        let reduction = (self.total_dex() as f32 * 0.6) as u32 + self.equip_cd_reduction();
        ATTACK_COOLDOWN_BASE.saturating_sub(reduction).max(15)
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn spell_bonus(&self) -> f32 {
        1.0 + self.total_ene() as f32 * 0.02 + self.equip_spell_percent()
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn damage_reduction(&self) -> f32 {
        self.total_vit() as f32 * 0.08 + self.equip_flat_defense() // D2: defense is modest early
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn dodge_chance(&self) -> f32 {
        (self.total_dex() as f32 * 0.008).min(0.40) // D2: block/dodge from DEX, cap 40%
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn hp_regen(&self) -> f32 {
        HP_REGEN_BASE + self.total_vit() as f32 * 0.001 // D2: very slow natural regen
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn mp_regen(&self) -> f32 {
        MP_REGEN_BASE + self.total_ene() as f32 * 0.002 // D2: slow mana regen
    }

    pub fn walk_speed(&self) -> f32 {
        PLAYER_SPEED + self.equip_move_speed()
    }

    pub fn run_speed(&self) -> f32 {
        PLAYER_RUN_SPEED + self.equip_move_speed()
    }

    pub fn click_walk_speed(&self) -> f32 {
        CLICK_MOVE_SPEED + self.equip_move_speed()
    }

    pub fn click_run_speed(&self) -> f32 {
        CLICK_RUN_SPEED + self.equip_move_speed()
    }

    /// Effective resistances after difficulty penalty.
    pub fn effective_resistances(&self) -> Resistances {
        let penalty = self.difficulty.resist_penalty();
        Resistances {
            fire: (self.resistances.fire - penalty).clamp(-1.0, 0.75),
            cold: (self.resistances.cold - penalty).clamp(-1.0, 0.75),
            lightning: (self.resistances.lightning - penalty).clamp(-1.0, 0.75),
            poison: (self.resistances.poison - penalty).clamp(-1.0, 0.75),
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}
