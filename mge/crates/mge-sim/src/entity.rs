use serde::{Deserialize, Serialize};

use crate::constants::*;

// ---------------------------------------------------------------------------
// Elemental resistances (fire / cold / lightning / poison)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Resistances {
    pub fire: f32,
    pub cold: f32,
    pub lightning: f32,
    pub poison: f32,
}

impl Resistances {
    pub fn apply(&self, base_dmg: f32, element: Element) -> f32 {
        let res = match element {
            Element::Physical => return base_dmg,
            Element::Fire => self.fire,
            Element::Cold => self.cold,
            Element::Lightning => self.lightning,
            Element::Poison => self.poison,
        };
        base_dmg * (1.0 - res.clamp(-1.0, 0.75))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Element {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
}

// ---------------------------------------------------------------------------
// Monster affixes (champion / unique packs)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterAffix {
    ExtraStrong,
    FireEnchanted,
    ColdEnchanted,
    LightningEnchanted,
    Cursed,
}

impl MonsterAffix {
    pub fn hp_mult(self) -> f32 {
        match self {
            Self::ExtraStrong => 1.5,
            _ => 1.2,
        }
    }

    pub fn damage_mult(self) -> f32 {
        match self {
            Self::ExtraStrong => 1.6,
            Self::Cursed => 1.0,
            _ => 1.2,
        }
    }

    pub fn element(self) -> Element {
        match self {
            Self::FireEnchanted => Element::Fire,
            Self::ColdEnchanted => Element::Cold,
            Self::LightningEnchanted => Element::Lightning,
            _ => Element::Physical,
        }
    }

    pub fn label(self) -> &'static [u8] {
        match self {
            Self::ExtraStrong => b"STRONG",
            Self::FireEnchanted => b"FIRE",
            Self::ColdEnchanted => b"COLD",
            Self::LightningEnchanted => b"LGHT",
            Self::Cursed => b"CURSED",
        }
    }

    pub fn color(self) -> [f32; 4] {
        match self {
            Self::ExtraStrong => [1.0, 0.85, 0.30, 1.0],
            Self::FireEnchanted => [1.0, 0.45, 0.10, 1.0],
            Self::ColdEnchanted => [0.40, 0.70, 1.0, 1.0],
            Self::LightningEnchanted => [0.60, 0.70, 1.0, 1.0],
            Self::Cursed => [0.70, 0.20, 0.20, 1.0],
        }
    }
}

// ---------------------------------------------------------------------------
// Difficulty levels
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Normal,
    Nightmare,
    Hell,
}

impl Difficulty {
    pub fn monster_hp_mult(self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Nightmare => 3.0,    // D2: ~3× HP
            Self::Hell => 5.0,         // D2: ~5× HP
        }
    }

    pub fn monster_dmg_mult(self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Nightmare => 2.0,    // D2: ~2× damage
            Self::Hell => 3.0,         // D2: ~3× damage
        }
    }

    pub fn monster_xp_mult(self) -> f32 {
        match self {
            Self::Normal => 1.0,
            Self::Nightmare => 3.0,    // D2: ~3× XP
            Self::Hell => 5.0,         // D2: ~5× XP
        }
    }

    pub fn resist_penalty(self) -> f32 {
        match self {
            Self::Normal => 0.0,
            Self::Nightmare => 0.40,   // D2: -40 all resist
            Self::Hell => 1.00,        // D2: -100 all resist
        }
    }

    pub fn label(self) -> &'static [u8] {
        match self {
            Self::Normal => b"NORMAL",
            Self::Nightmare => b"NIGHTMARE",
            Self::Hell => b"HELL",
        }
    }
}

// ---------------------------------------------------------------------------
// Zone identification — Act 1 open world (5×5 grid, camp at center)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneId {
    // Ring 0 — center
    RogueCamp       = 0,
    // Ring 1 — immediate surroundings (tier 1)
    BloodMoorN      = 1,
    BloodMoorE      = 2,
    BloodMoorS      = 3,
    BloodMoorW      = 4,
    // Ring 1 corners (tier 1-2)
    DenOfEvil       = 5,
    ColdPlainsNE    = 6,
    GraveyardSE     = 7,
    MeadowSW        = 8,
    // Ring 2 — outer ring (tier 2-4)
    ColdPlainsN     = 9,
    FrozenPath      = 10,
    StonyFieldE     = 11,
    DarkWoodE       = 12,
    BurialGrounds   = 13,
    BlackMarshS     = 14,
    TamoeFoothills  = 15,
    DarkWoodW       = 16,
    // Ring 2 corners (tier 3-5)
    DarkWoodNW      = 17,
    HighlandsNE     = 18,
    CatacombsEntry  = 19,
    TamoeHighlands  = 20,
    // Ring 2 outer edges (tier 4-5)
    RockyWaste      = 21,
    DryHills        = 22,
    FarOasis        = 23,
    OuterCloister   = 24,
}

impl ZoneId {
    pub const ALL: [ZoneId; 25] = [
        ZoneId::RogueCamp,
        ZoneId::BloodMoorN, ZoneId::BloodMoorE, ZoneId::BloodMoorS, ZoneId::BloodMoorW,
        ZoneId::DenOfEvil, ZoneId::ColdPlainsNE, ZoneId::GraveyardSE, ZoneId::MeadowSW,
        ZoneId::ColdPlainsN, ZoneId::FrozenPath, ZoneId::StonyFieldE, ZoneId::DarkWoodE,
        ZoneId::BurialGrounds, ZoneId::BlackMarshS, ZoneId::TamoeFoothills, ZoneId::DarkWoodW,
        ZoneId::DarkWoodNW, ZoneId::HighlandsNE, ZoneId::CatacombsEntry, ZoneId::TamoeHighlands,
        ZoneId::RockyWaste, ZoneId::DryHills, ZoneId::FarOasis, ZoneId::OuterCloister,
    ];

    #[must_use]
    pub fn from_index(i: usize) -> Self {
        Self::ALL[i]
    }

    #[must_use]
    pub fn zone_type(self) -> ZoneType {
        match self {
            ZoneId::RogueCamp => ZoneType::Town,
            ZoneId::BloodMoorN | ZoneId::BloodMoorE
                | ZoneId::BloodMoorS | ZoneId::BloodMoorW
                | ZoneId::MeadowSW => ZoneType::Wilderness,
            ZoneId::DenOfEvil | ZoneId::CatacombsEntry => ZoneType::Cave,
            ZoneId::ColdPlainsNE | ZoneId::ColdPlainsN | ZoneId::FrozenPath => ZoneType::Plains,
            ZoneId::GraveyardSE | ZoneId::BurialGrounds => ZoneType::Graveyard,
            ZoneId::StonyFieldE | ZoneId::RockyWaste | ZoneId::DryHills => ZoneType::Field,
            ZoneId::DarkWoodE | ZoneId::DarkWoodW | ZoneId::DarkWoodNW => ZoneType::Forest,
            ZoneId::BlackMarshS | ZoneId::FarOasis => ZoneType::Marsh,
            ZoneId::TamoeFoothills | ZoneId::TamoeHighlands
                | ZoneId::HighlandsNE | ZoneId::OuterCloister => ZoneType::Highlands,
        }
    }

    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            ZoneId::RogueCamp => "Rogue Encampment",
            ZoneId::BloodMoorN | ZoneId::BloodMoorE
                | ZoneId::BloodMoorS | ZoneId::BloodMoorW => "Blood Moor",
            ZoneId::DenOfEvil => "Den of Evil",
            ZoneId::ColdPlainsNE | ZoneId::ColdPlainsN => "Cold Plains",
            ZoneId::FrozenPath => "Frozen Path",
            ZoneId::GraveyardSE => "Graveyard",
            ZoneId::MeadowSW => "Meadow",
            ZoneId::StonyFieldE => "Stony Field",
            ZoneId::DarkWoodE | ZoneId::DarkWoodW | ZoneId::DarkWoodNW => "Dark Wood",
            ZoneId::BurialGrounds => "Burial Grounds",
            ZoneId::BlackMarshS => "Black Marsh",
            ZoneId::TamoeFoothills => "Tamoe Foothills",
            ZoneId::TamoeHighlands => "Tamoe Highlands",
            ZoneId::HighlandsNE => "Highlands Pass",
            ZoneId::CatacombsEntry => "Catacombs Entry",
            ZoneId::RockyWaste => "Rocky Waste",
            ZoneId::DryHills => "Dry Hills",
            ZoneId::FarOasis => "Far Oasis",
            ZoneId::OuterCloister => "Outer Cloister",
        }
    }

    /// Difficulty tier (0 = easiest, higher = harder enemies).
    #[must_use]
    pub fn tier(self) -> u8 {
        match self {
            ZoneId::RogueCamp => 0,
            ZoneId::BloodMoorN | ZoneId::BloodMoorE
                | ZoneId::BloodMoorS | ZoneId::BloodMoorW => 1,
            ZoneId::DenOfEvil | ZoneId::ColdPlainsNE
                | ZoneId::GraveyardSE | ZoneId::MeadowSW => 2,
            ZoneId::ColdPlainsN | ZoneId::FrozenPath
                | ZoneId::StonyFieldE | ZoneId::BurialGrounds => 3,
            ZoneId::DarkWoodE | ZoneId::DarkWoodW | ZoneId::DarkWoodNW
                | ZoneId::BlackMarshS | ZoneId::TamoeFoothills
                | ZoneId::HighlandsNE | ZoneId::RockyWaste => 4,
            ZoneId::TamoeHighlands | ZoneId::CatacombsEntry
                | ZoneId::DryHills | ZoneId::FarOasis | ZoneId::OuterCloister => 5,
        }
    }
}

/// Biome type — determines terrain generation, enemy pools, and ambient visuals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZoneType {
    Town,
    Wilderness,
    Cave,
    Plains,
    Graveyard,
    Field,
    Forest,
    Marsh,
    Highlands,
}

// ---------------------------------------------------------------------------
// Enemy types
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnemyKind {
    Zombie,
    Skeleton,
    Fallen,
    QuillRat,
    FallenShaman,
    Boss,
    BloodRaven,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enemy {
    pub pos: [f32; 2],
    pub kind: EnemyKind,
    pub hp: f32,
    pub max_hp: f32,
    pub attack_cd: u32,
    pub spawn: [f32; 2],
    pub alive: bool,
    pub respawn_timer: u32,
    pub fade_timer: u32,
    pub freeze_timer: u32,
    pub slow_timer: u32,
    pub affix: Option<MonsterAffix>,
    pub resistances: Resistances,
}

impl Enemy {
    pub fn new(kind: EnemyKind, x: f32, y: f32) -> Self {
        // HP from D2 Normal Act 1 monstersheet (player base melee = 4 dmg):
        // Fallen: 1-2 hits, QuillRat: 2 hits, Skeleton: 2-3 hits,
        // Zombie: 4-5 hits, Shaman: 3-4 hits, BloodRaven: ~28 hits, Boss: ~63 hits
        let max_hp = match kind {
            EnemyKind::Fallen => 5.0,         // D2: 4-5 HP
            EnemyKind::QuillRat => 7.0,       // D2: 5-9 HP
            EnemyKind::Skeleton => 10.0,      // D2: 8-12 HP
            EnemyKind::FallenShaman => 14.0,  // D2: 12-18 HP
            EnemyKind::Zombie => 18.0,        // D2: 15-24 HP
            EnemyKind::BloodRaven => 110.0,   // D2: 105 HP (super unique)
            EnemyKind::Boss => BOSS_MAX_HP,   // D2 Andariel ~1024, scaled
        };
        let resistances = match kind {
            EnemyKind::Zombie => Resistances { poison: 0.50, ..Default::default() },
            EnemyKind::Skeleton => Resistances { cold: 0.20, poison: 0.50, ..Default::default() },
            EnemyKind::FallenShaman => Resistances { fire: 0.30, ..Default::default() },
            EnemyKind::Boss => Resistances { fire: 0.25, cold: 0.25, lightning: 0.25, poison: 0.25 },
            EnemyKind::BloodRaven => Resistances { cold: 0.33, lightning: 0.20, ..Default::default() },
            _ => Resistances::default(),
        };
        Self {
            pos: [x, y],
            kind,
            hp: max_hp,
            max_hp,
            attack_cd: 0,
            spawn: [x, y],
            alive: true,
            respawn_timer: 0,
            fade_timer: 0,
            freeze_timer: 0,
            slow_timer: 0,
            affix: None,
            resistances,
        }
    }

    /// Apply affix and difficulty scaling to this enemy.
    pub fn apply_modifiers(&mut self, difficulty: Difficulty) {
        let hp_mult = difficulty.monster_hp_mult()
            * self.affix.map_or(1.0, MonsterAffix::hp_mult);
        self.max_hp *= hp_mult;
        self.hp = self.max_hp;
    }

    pub fn effective_speed(&self) -> f32 {
        if self.freeze_timer > 0 {
            0.0
        } else if self.slow_timer > 0 {
            ENEMY_SPEED * FROST_SLOW_FACTOR
        } else {
            ENEMY_SPEED
        }
    }

    pub fn is_boss(&self) -> bool {
        matches!(self.kind, EnemyKind::Boss | EnemyKind::BloodRaven)
    }

    pub fn is_ranged(&self) -> bool {
        matches!(
            self.kind,
            EnemyKind::QuillRat | EnemyKind::FallenShaman | EnemyKind::BloodRaven
        )
    }

    /// Per-hit damage — deliberate pacing.
    /// Player 65 HP, pack of 3 Fallen (~3 DPS) → ~20s to die without potions.
    pub fn base_damage(&self) -> f32 {
        match self.kind {
            EnemyKind::Fallen => ENEMY_DAMAGE * 0.5,       // 2.0 — weakest
            EnemyKind::QuillRat => ENEMY_DAMAGE * 0.6,     // 2.4 — ranged harass
            EnemyKind::FallenShaman => ENEMY_DAMAGE * 0.55, // 2.2 — caster
            EnemyKind::Skeleton => ENEMY_DAMAGE * 0.8,     // 3.2 — solid melee
            EnemyKind::Zombie => ENEMY_DAMAGE * 1.0,       // 4.0 — slow but hard hitter
            EnemyKind::BloodRaven => ENEMY_DAMAGE * 2.5,   // 10.0 — mini-boss
            EnemyKind::Boss => ENEMY_DAMAGE * 5.0,         // 20.0 — boss
        }
    }

    pub fn attack_damage(&self) -> f32 {
        self.base_damage() * self.affix.map_or(1.0, MonsterAffix::damage_mult)
    }

    pub fn attack_element(&self) -> Element {
        self.affix.map_or(Element::Physical, MonsterAffix::element)
    }

    /// XP values aligned to D2 Normal Act 1 monstersheet.
    /// Level 2 requires 500 XP (~20 Fallen kills).
    pub fn xp_value(&self) -> f32 {
        let base = match self.kind {
            EnemyKind::Fallen => XP_PER_KILL * 1.0,       // 25  — D2: 23
            EnemyKind::QuillRat => XP_PER_KILL * 1.2,     // 30  — D2: 28
            EnemyKind::Skeleton => XP_PER_KILL * 1.8,     // 45  — D2: 44
            EnemyKind::Zombie => XP_PER_KILL * 2.0,       // 50  — D2: 51
            EnemyKind::FallenShaman => XP_PER_KILL * 2.5,  // 62  — D2: 60
            EnemyKind::BloodRaven => XP_PER_KILL * 42.0,   // 1050 — D2: 1055
            EnemyKind::Boss => XP_PER_KILL * 210.0,        // 5250 — D2 Andariel: 5230
        };
        base * if self.affix.is_some() { 1.5 } else { 1.0 }
    }
}

// ---------------------------------------------------------------------------
// Projectile
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projectile {
    pub pos: [f32; 2],
    pub dir: [f32; 2],
    pub dist_left: f32,
    pub damage: f32,
    pub is_frost: bool,
    pub friendly: bool,
}

// ---------------------------------------------------------------------------
// Lightning bolt (visual segment for chain lightning)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningBolt {
    pub from: [f32; 2],
    pub to: [f32; 2],
    pub life: u32,
}

// ---------------------------------------------------------------------------
// Mercenary / hireling — D2-style typed hirelings
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MercType {
    Rogue,      // Act 1 — ranged bow, fast attacks
    Guard,      // Act 2 — melee tank, carries an aura
    WolfRider,  // Act 3 — fast melee, high damage
    IronWolf,   // Act 4 — spell caster, elemental damage
}

impl MercType {
    pub fn name(self) -> &'static [u8] {
        match self {
            Self::Rogue     => b"ROGUE",
            Self::Guard     => b"GUARD",
            Self::WolfRider => b"WOLF RIDER",
            Self::IronWolf  => b"IRON WOLF",
        }
    }

    pub fn base_hp(self) -> f32 {
        match self {
            Self::Rogue     => MERC_MAX_HP,           // 85
            Self::Guard     => MERC_GUARD_HP,         // 130
            Self::WolfRider => MERC_WOLF_RIDER_HP,    // 70
            Self::IronWolf  => MERC_MAGE_HP,          // 55
        }
    }

    pub fn base_damage(self) -> f32 {
        match self {
            Self::Rogue     => MERC_DAMAGE,              // 4
            Self::Guard     => MERC_GUARD_DAMAGE,        // 6
            Self::WolfRider => MERC_WOLF_RIDER_DAMAGE,   // 7
            Self::IronWolf  => MERC_MAGE_DAMAGE,         // 12
        }
    }

    pub fn base_speed(self) -> f32 {
        match self {
            Self::Rogue     => MERC_SPEED,              // 0.055
            Self::Guard     => MERC_SPEED * 0.85,       // 0.047
            Self::WolfRider => MERC_WOLF_RIDER_SPEED,   // 0.08
            Self::IronWolf  => MERC_SPEED * 0.90,       // 0.050
        }
    }

    pub fn attack_range(self) -> f32 {
        match self {
            Self::Rogue     => MERC_ROGUE_RANGE,        // 6.0 ranged
            Self::Guard     => MERC_GUARD_RANGE,        // 1.2 melee
            Self::WolfRider => MERC_WOLF_RIDER_RANGE,   // 1.5 melee
            Self::IronWolf  => MERC_MAGE_RANGE,         // 7.0 ranged
        }
    }

    pub fn is_ranged(self) -> bool {
        matches!(self, Self::Rogue | Self::IronWolf)
    }

    pub fn hire_cost(self) -> u32 {
        match self {
            Self::Rogue     => MERC_HIRE_COST,          // 80
            Self::Guard     => MERC_HIRE_COST * 2,      // 160
            Self::WolfRider => MERC_HIRE_COST * 3,      // 240
            Self::IronWolf  => MERC_HIRE_COST * 4,      // 320
        }
    }

    pub fn color(self) -> [f32; 4] {
        match self {
            Self::Rogue     => [0.4, 0.8, 0.4, 1.0],   // green
            Self::Guard     => [0.7, 0.6, 0.3, 1.0],   // gold
            Self::WolfRider => [0.6, 0.4, 0.2, 1.0],   // brown
            Self::IronWolf  => [0.4, 0.5, 0.9, 1.0],   // blue
        }
    }

    pub fn aura(self) -> Option<AuraKind> {
        match self {
            Self::Guard => Some(AuraKind::Defiance),    // tank aura
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mercenary {
    pub pos: [f32; 2],
    pub hp: f32,
    pub max_hp: f32,
    pub attack_cd: u32,
    pub alive: bool,
    pub respawn_timer: u32,
    pub merc_type: MercType,
    pub level: u32,
    pub xp: f32,
    pub kills: u32,
    pub equipment: Option<Equipment>,   // can equip one item
}

impl Mercenary {
    pub fn new(pos: [f32; 2], merc_type: MercType) -> Self {
        let base_hp = merc_type.base_hp();
        Self {
            pos,
            hp: base_hp,
            max_hp: base_hp,
            attack_cd: 0,
            alive: true,
            respawn_timer: 0,
            merc_type,
            level: 1,
            xp: 0.0,
            kills: 0,
            equipment: None,
        }
    }

    /// Effective max HP (base + level scaling + equipment).
    #[allow(clippy::cast_precision_loss)]
    pub fn effective_max_hp(&self) -> f32 {
        let base = self.merc_type.base_hp();
        let level_bonus = (self.level as f32 - 1.0) * 3.0;
        let equip_bonus = self.equipment.map_or(0.0, |e| {
            let (_, _, v, _) = e.bonuses();
            v as f32 * 2.0
        });
        base + level_bonus + equip_bonus
    }

    /// Effective damage.
    #[allow(clippy::cast_precision_loss)]
    pub fn effective_damage(&self) -> f32 {
        let base = self.merc_type.base_damage();
        let level_bonus = (self.level as f32 - 1.0) * 0.5;
        let equip_bonus = self.equipment.map_or(0.0, |e| e.flat_damage());
        base + level_bonus + equip_bonus
    }

    /// Add XP and level up. Returns levels gained.
    pub fn add_xp(&mut self, amount: f32) -> u32 {
        self.xp += amount;
        let mut gained = 0u32;
        loop {
            let need = 100.0 + self.level as f32 * 50.0;
            if self.xp < need { break; }
            self.xp -= need;
            self.level += 1;
            self.max_hp = self.effective_max_hp();
            self.hp = self.hp.min(self.max_hp); // don't exceed new max
            gained += 1;
        }
        gained
    }
}

// ---------------------------------------------------------------------------
// Auras — passive area buffs from mercs / skills
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuraKind {
    Might,       // +% damage to allies in radius
    Defiance,    // +flat defense to allies in radius
    Meditation,  // +mp regen to allies in radius
    Thorns,      // % damage reflected when allies take damage
}

impl AuraKind {
    pub fn name(self) -> &'static [u8] {
        match self {
            Self::Might     => b"MIGHT",
            Self::Defiance  => b"DEFIANCE",
            Self::Meditation=> b"MEDITATION",
            Self::Thorns    => b"THORNS",
        }
    }

    pub fn color(self) -> [f32; 4] {
        match self {
            Self::Might     => [0.9, 0.5, 0.2, 0.3],
            Self::Defiance  => [0.5, 0.7, 0.9, 0.3],
            Self::Meditation=> [0.3, 0.4, 0.9, 0.3],
            Self::Thorns    => [0.7, 0.3, 0.3, 0.3],
        }
    }
}

// ---------------------------------------------------------------------------
// Summons — player-owned minions
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummonKind {
    Skeleton,
    Golem,
    Wolf,
}

impl SummonKind {
    pub fn name(self) -> &'static [u8] {
        match self {
            Self::Skeleton => b"SKELETON",
            Self::Golem    => b"GOLEM",
            Self::Wolf     => b"WOLF",
        }
    }

    pub fn base_hp(self) -> f32 {
        match self {
            Self::Skeleton => SKELETON_HP,
            Self::Golem    => GOLEM_HP,
            Self::Wolf     => WOLF_HP,
        }
    }

    pub fn base_damage(self) -> f32 {
        match self {
            Self::Skeleton => SKELETON_DAMAGE,
            Self::Golem    => GOLEM_DAMAGE,
            Self::Wolf     => WOLF_DAMAGE,
        }
    }

    pub fn speed(self) -> f32 {
        match self {
            Self::Skeleton => SKELETON_SPEED,
            Self::Golem    => GOLEM_SPEED,
            Self::Wolf     => WOLF_SPEED,
        }
    }

    pub fn attack_range(self) -> f32 {
        match self {
            Self::Skeleton => SKELETON_ATTACK_RANGE,
            Self::Golem    => GOLEM_ATTACK_RANGE,
            Self::Wolf     => WOLF_ATTACK_RANGE,
        }
    }

    pub fn attack_cd(self) -> u32 {
        match self {
            Self::Skeleton => SKELETON_ATTACK_CD,
            Self::Golem    => GOLEM_ATTACK_CD,
            Self::Wolf     => WOLF_ATTACK_CD,
        }
    }

    pub fn color(self) -> [f32; 4] {
        match self {
            Self::Skeleton => [0.7, 0.7, 0.65, 1.0],
            Self::Golem    => [0.5, 0.4, 0.3, 1.0],
            Self::Wolf     => [0.6, 0.5, 0.35, 1.0],
        }
    }

    /// Mana cost to summon.
    pub fn mana_cost(self) -> f32 {
        match self {
            Self::Skeleton => 8.0,
            Self::Golem    => 25.0,
            Self::Wolf     => 15.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summon {
    pub pos: [f32; 2],
    pub kind: SummonKind,
    pub hp: f32,
    pub max_hp: f32,
    pub damage: f32,
    pub attack_cd: u32,
    pub alive: bool,
    pub target: Option<usize>,  // enemy index
    pub owner_boost: f32,       // multiplier from player's spell_dmg_pct etc.
}

impl Summon {
    pub fn new(kind: SummonKind, pos: [f32; 2], owner_spell_pct: f32) -> Self {
        let hp = kind.base_hp() * (1.0 + owner_spell_pct * 0.5);
        let damage = kind.base_damage() * (1.0 + owner_spell_pct);
        Self {
            pos,
            kind,
            hp,
            max_hp: hp,
            damage,
            attack_cd: 0,
            alive: true,
            target: None,
            owner_boost: owner_spell_pct,
        }
    }

    pub fn speed(&self) -> f32 { self.kind.speed() }
    pub fn attack_range(&self) -> f32 { self.kind.attack_range() }
}

// ---------------------------------------------------------------------------
// Shrines — temporary buffs
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShrineKind {
    Experience,
    Health,
    Damage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shrine {
    pub pos: [f32; 2],
    pub kind: ShrineKind,
    pub used: bool,
}

// ---------------------------------------------------------------------------
// Treasure chests
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chest {
    pub pos: [f32; 2],
    pub opened: bool,
}

// ---------------------------------------------------------------------------
// Active buff from shrine
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveBuff {
    pub kind: ShrineKind,
    pub remaining: u32,
}

// ---------------------------------------------------------------------------
// Breakable objects
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakable {
    pub pos: [f32; 2],
    pub hp: f32,
    pub alive: bool,
}

// ---------------------------------------------------------------------------
// Item drops on ground
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PotionKind {
    Health,
    Mana,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DropKind {
    HealthPotion,
    ManaPotion,
    Gold,
    Equipment(EquipSlot, ItemRarity),
    Scroll(crate::skill_disk::SlotContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDrop {
    pub pos: [f32; 2],
    pub kind: DropKind,
    pub gold_amount: u32,
    pub life: u32,
}

// ---------------------------------------------------------------------------
// Damage float (visual)
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageFloat {
    pub wx: f32,
    pub wy: f32,
    pub dy: f32,
    pub value: f32,
    pub is_heal: bool,
    pub life: u32,
}

// ---------------------------------------------------------------------------
// Equipment / item rarity
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemRarity {
    Normal,
    Magic,
    Rare,
    Unique,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipSlot {
    Helm,
    Armor,
    Weapon,
    Boots,
    Gloves,
    Ring,
}

impl ItemRarity {
    pub fn stat_mult(self) -> u32 {
        match self {
            Self::Normal => 1,
            Self::Magic => 2,
            Self::Rare => 3,
            Self::Unique => 5,
        }
    }

    pub fn name(self) -> &'static [u8] {
        match self {
            Self::Normal => b"",
            Self::Magic => b"MAGIC ",
            Self::Rare => b"RARE ",
            Self::Unique => b"UNIQUE ",
        }
    }

    pub fn color(self) -> [f32; 4] {
        match self {
            Self::Normal => [0.82, 0.82, 0.82, 1.0],
            Self::Magic => [0.40, 0.50, 1.0, 1.0],
            Self::Rare => [1.0, 1.0, 0.30, 1.0],
            Self::Unique => [0.75, 0.55, 0.15, 1.0],
        }
    }

    /// Base sell value (gold) per rarity tier.
    pub fn base_sell_value(self) -> u32 {
        match self {
            Self::Normal => 5,
            Self::Magic => 15,
            Self::Rare => 40,
            Self::Unique => 100,
        }
    }
}

impl EquipSlot {
    pub fn name(self) -> &'static [u8] {
        match self {
            Self::Helm => b"HELM",
            Self::Armor => b"ARMOR",
            Self::Weapon => b"SWORD",
            Self::Boots => b"BOOTS",
            Self::Gloves => b"GLOVES",
            Self::Ring => b"RING",
        }
    }

    pub fn base_bonus(self) -> (u32, u32, u32, u32) {
        match self {
            Self::Helm => (0, 0, 2, 1),
            Self::Armor => (1, 0, 3, 0),
            Self::Weapon => (3, 1, 0, 0),
            Self::Boots => (0, 2, 1, 0),
            Self::Gloves => (1, 2, 0, 0),
            Self::Ring => (0, 0, 0, 3),
        }
    }

    pub fn effect_label(self) -> &'static [u8] {
        match self {
            Self::Helm => b"+HP%",
            Self::Armor => b"+DEF",
            Self::Weapon => b"+DMG",
            Self::Boots => b"+SPD",
            Self::Gloves => b"-CD",
            Self::Ring => b"+SPELL%",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    pub slot: EquipSlot,
    pub rarity: ItemRarity,
    pub ilvl: u32,
}

impl Equipment {
    pub fn bonuses(self) -> (u32, u32, u32, u32) {
        let (bs, bd, bv, be) = self.slot.base_bonus();
        let m = self.rarity.stat_mult();
        let level_bonus = self.ilvl / 3;
        (
            bs * m + level_bonus,
            bd * m + level_bonus,
            bv * m + level_bonus,
            be * m + level_bonus,
        )
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn flat_damage(self) -> f32 {
        if self.slot != EquipSlot::Weapon {
            return 0.0;
        }
        let base = 2.0 * self.rarity.stat_mult() as f32;
        base + self.ilvl as f32 * 0.5
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn flat_defense(self) -> f32 {
        if self.slot != EquipSlot::Armor {
            return 0.0;
        }
        let base = 1.5 * self.rarity.stat_mult() as f32;
        base + self.ilvl as f32 * 0.3
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn hp_percent(self) -> f32 {
        if self.slot != EquipSlot::Helm {
            return 0.0;
        }
        self.rarity.stat_mult() as f32 * 0.03 + self.ilvl as f32 * 0.005
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn move_speed_bonus(self) -> f32 {
        if self.slot != EquipSlot::Boots {
            return 0.0;
        }
        self.rarity.stat_mult() as f32 * 0.005 + self.ilvl as f32 * 0.001
    }

    pub fn cooldown_reduction(self) -> u32 {
        if self.slot != EquipSlot::Gloves {
            return 0;
        }
        self.rarity.stat_mult() + self.ilvl / 4
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn spell_percent(self) -> f32 {
        if self.slot != EquipSlot::Ring {
            return 0.0;
        }
        self.rarity.stat_mult() as f32 * 0.03 + self.ilvl as f32 * 0.005
    }

    pub fn life_steal(self) -> f32 {
        if self.slot != EquipSlot::Weapon {
            return 0.0;
        }
        match self.rarity {
            ItemRarity::Rare => 0.03,
            ItemRarity::Unique => 0.06,
            _ => 0.0,
        }
    }

    pub fn mana_steal(self) -> f32 {
        if self.slot != EquipSlot::Ring {
            return 0.0;
        }
        match self.rarity {
            ItemRarity::Rare => 0.02,
            ItemRarity::Unique => 0.05,
            _ => 0.0,
        }
    }

    pub fn sell_value(self) -> u32 {
        self.rarity.stat_mult() * 5 + self.ilvl * 2
    }
}

// ---------------------------------------------------------------------------
// Skill system
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillId {
    Melee,
    Fireball,
    FrostBolt,
    ChainLightning,
    Teleport,
}

pub fn skill_name(id: SkillId) -> &'static [u8] {
    match id {
        SkillId::Melee => b"MELEE",
        SkillId::Fireball => b"FIREBALL",
        SkillId::FrostBolt => b"FROST BOLT",
        SkillId::ChainLightning => b"CHAIN LTN",
        SkillId::Teleport => b"TELEPORT",
    }
}

// ---------------------------------------------------------------------------
// Quests
// ---------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub struct Quest {
    pub name: &'static str,
    pub desc: &'static str,
    pub done: bool,
}

// ---------------------------------------------------------------------------
// Combat log entry
// ---------------------------------------------------------------------------
pub struct LogEntry {
    pub text: [u8; 40],
    pub len: usize,
    pub color: [f32; 4],
    pub life: u32,
}

impl LogEntry {
    pub fn new(msg: &[u8], color: [f32; 4]) -> Self {
        let mut text = [0u8; 40];
        let len = msg.len().min(40);
        text[..len].copy_from_slice(&msg[..len]);
        Self {
            text,
            len,
            color,
            life: LOG_LIFE,
        }
    }
}

// ---------------------------------------------------------------------------
// NPC info (static data)
// ---------------------------------------------------------------------------
pub struct NpcInfo {
    pub pos: [f32; 2],
    pub name: &'static [u8],
}

pub const NPCS: &[NpcInfo] = &[
    NpcInfo {
        pos: [42.0, 42.0],
        name: b"AKARA",
    },
    NpcInfo {
        pos: [58.0, 42.0],
        name: b"GHEED",
    },
    NpcInfo {
        pos: [42.0, 58.0],
        name: b"CHARSI",
    },
    NpcInfo {
        pos: [58.0, 58.0],
        name: b"KASHYA",
    },
];

// ---------------------------------------------------------------------------
// Zone exits — deprecated, kept for compatibility. Transitions are seamless.
// ---------------------------------------------------------------------------
pub struct ZoneExit {
    pub from: ZoneId,
    pub dir: u8,
    pub target: ZoneId,
    pub spawn: [f32; 2],
}

pub const ZONE_EXITS: &[ZoneExit] = &[];

// ---------------------------------------------------------------------------
// Combat — combo system
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComboTracker {
    pub stack: u32,         // current combo count (0..COMBO_MAX_STACK)
    pub timer: u32,         // frames remaining in combo window
    pub total_combos: u32,  // lifetime total (stats)
}

impl ComboTracker {
    /// Register a hit. Returns the current damage multiplier.
    pub fn hit(&mut self) -> f32 {
        if self.timer > 0 && self.stack < COMBO_MAX_STACK {
            self.stack += 1;
        } else if self.timer == 0 {
            self.stack = 1;
        }
        self.timer = COMBO_WINDOW;
        if self.stack >= COMBO_MAX_STACK {
            self.total_combos += 1;
        }
        1.0 + self.stack as f32 * COMBO_DMG_PER_STACK
    }

    /// Call every frame. Resets combo if window expires.
    pub fn tick(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
            if self.timer == 0 {
                self.stack = 0;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Critical hit helper
// ---------------------------------------------------------------------------
pub fn calc_crit(base_chance: f32, bonus_chance: f32, bonus_mult: f32, lethal_strike: bool) -> (bool, f32) {
    let chance = (CRIT_BASE_CHANCE + base_chance + bonus_chance).min(0.75);
    // Deterministic pseudo-crit based on hash — real game would use RNG
    let mult = if lethal_strike { 3.0 } else { CRIT_BASE_MULT + bonus_mult };
    (chance > 0.0, mult)
}

// ---------------------------------------------------------------------------
// MMO — party system
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartyMember {
    pub name: [u8; 20],
    pub name_len: usize,
    pub level: u32,
    pub hp_pct: f32,        // 0.0..1.0 for party frame display
    pub mp_pct: f32,
    pub pos: [f32; 2],
    pub zone: u8,           // ZoneId as u8
    pub alive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    pub members: Vec<PartyMember>,
    pub leader_idx: usize,
    pub loot_mode: LootMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LootMode {
    FreeForAll,
    RoundRobin,
    NeedGreed,
}

impl Party {
    pub fn new_solo(name: &[u8], level: u32) -> Self {
        let mut n = [0u8; 20];
        let len = name.len().min(20);
        n[..len].copy_from_slice(&name[..len]);
        Self {
            members: vec![PartyMember {
                name: n,
                name_len: len,
                level,
                hp_pct: 1.0,
                mp_pct: 1.0,
                pos: [50.0, 50.0],
                zone: 0,
                alive: true,
            }],
            leader_idx: 0,
            loot_mode: LootMode::FreeForAll,
        }
    }

    pub fn size(&self) -> usize { self.members.len() }

    pub fn xp_multiplier(&self) -> f32 {
        1.0 + (self.members.len().saturating_sub(1)) as f32 * PARTY_XP_BONUS_PER_MEMBER
    }

    pub fn can_invite(&self) -> bool { self.members.len() < PARTY_MAX_SIZE }
}

// ---------------------------------------------------------------------------
// MMO — chat system
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatChannel {
    Local,      // nearby players only
    Party,      // party members
    Trade,      // trade channel
    Global,     // server-wide
}

impl ChatChannel {
    pub fn prefix(self) -> &'static [u8] {
        match self {
            Self::Local  => b"[L] ",
            Self::Party  => b"[P] ",
            Self::Trade  => b"[T] ",
            Self::Global => b"[G] ",
        }
    }

    pub fn color(self) -> [f32; 4] {
        match self {
            Self::Local  => [0.85, 0.85, 0.85, 1.0],
            Self::Party  => [0.4, 0.7, 1.0, 1.0],
            Self::Trade  => [0.9, 0.8, 0.3, 1.0],
            Self::Global => [0.6, 0.9, 0.6, 1.0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub channel: ChatChannel,
    pub sender: [u8; 20],
    pub sender_len: usize,
    pub text: [u8; 80],
    pub text_len: usize,
    pub timestamp: u32,     // game frame
}

#[derive(Debug, Clone, Default)]
pub struct ChatLog {
    pub messages: Vec<ChatMessage>,
    pub max_display: usize,
}

impl ChatLog {
    pub fn new() -> Self {
        Self { messages: Vec::new(), max_display: 10 }
    }

    pub fn push(&mut self, msg: ChatMessage) {
        self.messages.push(msg);
        // Keep last 100 messages
        if self.messages.len() > 100 {
            self.messages.remove(0);
        }
    }

    pub fn recent(&self) -> &[ChatMessage] {
        let start = self.messages.len().saturating_sub(self.max_display);
        &self.messages[start..]
    }
}

// ---------------------------------------------------------------------------
// MMO — emotes
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Emote {
    Wave,
    Bow,
    Laugh,
    Taunt,
    Dance,
    Salute,
    Point,
    Sit,
}

impl Emote {
    pub fn text(self) -> &'static [u8] {
        match self {
            Self::Wave   => b"waves",
            Self::Bow    => b"bows",
            Self::Laugh  => b"laughs",
            Self::Taunt  => b"taunts",
            Self::Dance  => b"dances",
            Self::Salute => b"salutes",
            Self::Point  => b"points",
            Self::Sit    => b"sits down",
        }
    }

    pub fn duration(self) -> u32 {
        match self {
            Self::Dance => 180,
            Self::Sit   => 600,
            _ => 90,
        }
    }
}

// ---------------------------------------------------------------------------
// World boss event
// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldBoss {
    pub pos: [f32; 2],
    pub hp: f32,
    pub max_hp: f32,
    pub attack_cd: u32,
    pub alive: bool,
    pub phase: u8,              // 0=idle, 1=enraged (below 50% HP), 2=desperate (below 20%)
    pub spawn_timer: u32,       // frames until next spawn
    pub participants: u32,      // number of players that dealt damage
    pub total_damage_dealt: f32,
    pub resistances: Resistances,
}

impl WorldBoss {
    pub fn new(pos: [f32; 2]) -> Self {
        Self {
            pos,
            hp: WORLD_BOSS_HP,
            max_hp: WORLD_BOSS_HP,
            attack_cd: 0,
            alive: false,
            phase: 0,
            spawn_timer: WORLD_BOSS_SPAWN_INTERVAL,
            participants: 0,
            total_damage_dealt: 0.0,
            resistances: Resistances { fire: 0.30, cold: 0.30, lightning: 0.30, poison: 0.50 },
        }
    }

    pub fn phase_damage_mult(&self) -> f32 {
        match self.phase {
            0 => 1.0,
            1 => 1.5,     // enraged
            _ => 2.0,     // desperate
        }
    }

    pub fn update_phase(&mut self) {
        let ratio = self.hp / self.max_hp;
        self.phase = if ratio < 0.20 { 2 }
            else if ratio < 0.50 { 1 }
            else { 0 };
    }

    pub fn tick_spawn(&mut self) {
        if !self.alive {
            if self.spawn_timer > 0 {
                self.spawn_timer -= 1;
            }
            if self.spawn_timer == 0 {
                self.alive = true;
                self.hp = self.max_hp;
                self.phase = 0;
                self.participants = 0;
                self.total_damage_dealt = 0.0;
                self.spawn_timer = WORLD_BOSS_SPAWN_INTERVAL;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------
pub fn dist(a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    (dx * dx + dy * dy).sqrt()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // --- Mercenary ---
    #[test]
    fn test_merc_types_stats() {
        let rogue = Mercenary::new([0.0, 0.0], MercType::Rogue);
        let guard = Mercenary::new([0.0, 0.0], MercType::Guard);
        let wolf = Mercenary::new([0.0, 0.0], MercType::WolfRider);
        let mage = Mercenary::new([0.0, 0.0], MercType::IronWolf);
        assert!(guard.max_hp > rogue.max_hp);
        assert!(wolf.merc_type.base_speed() > guard.merc_type.base_speed());
        assert!(mage.merc_type.base_damage() > guard.merc_type.base_damage());
        assert!(rogue.merc_type.is_ranged());
        assert!(!guard.merc_type.is_ranged());
    }

    #[test]
    fn test_merc_leveling() {
        let mut merc = Mercenary::new([0.0, 0.0], MercType::Rogue);
        assert_eq!(merc.level, 1);
        let gained = merc.add_xp(500.0);
        assert!(gained >= 1);
        assert!(merc.level > 1);
        assert!(merc.effective_damage() > MercType::Rogue.base_damage());
        assert!(merc.effective_max_hp() > MercType::Rogue.base_hp());
    }

    #[test]
    fn test_merc_equipment() {
        let mut merc = Mercenary::new([0.0, 0.0], MercType::Guard);
        let eq = Equipment { slot: EquipSlot::Weapon, rarity: ItemRarity::Rare, ilvl: 10 };
        merc.equipment = Some(eq);
        assert!(merc.effective_damage() > MercType::Guard.base_damage());
    }

    #[test]
    fn test_guard_has_aura() {
        assert_eq!(MercType::Guard.aura(), Some(AuraKind::Defiance));
        assert_eq!(MercType::Rogue.aura(), None);
    }

    // --- Summons ---
    #[test]
    fn test_summon_creation() {
        let skel = Summon::new(SummonKind::Skeleton, [5.0, 5.0], 0.0);
        assert_eq!(skel.hp, SKELETON_HP);
        assert_eq!(skel.damage, SKELETON_DAMAGE);
        assert!(skel.alive);
    }

    #[test]
    fn test_summon_scales_with_spell() {
        let base = Summon::new(SummonKind::Golem, [0.0, 0.0], 0.0);
        let boosted = Summon::new(SummonKind::Golem, [0.0, 0.0], 0.5);
        assert!(boosted.hp > base.hp);
        assert!(boosted.damage > base.damage);
    }

    #[test]
    fn test_all_summon_kinds() {
        for kind in [SummonKind::Skeleton, SummonKind::Golem, SummonKind::Wolf] {
            assert!(kind.base_hp() > 0.0);
            assert!(kind.base_damage() > 0.0);
            assert!(kind.speed() > 0.0);
            assert!(kind.attack_range() > 0.0);
            assert!(kind.attack_cd() > 0);
            assert!(kind.mana_cost() > 0.0);
        }
    }

    // --- Combo ---
    #[test]
    fn test_combo_stacking() {
        let mut combo = ComboTracker::default();
        let m1 = combo.hit();
        assert!((m1 - 1.08).abs() < 0.01); // stack=1 → +8%
        let m2 = combo.hit();
        assert!((m2 - 1.16).abs() < 0.01); // stack=2 → +16%
    }

    #[test]
    fn test_combo_expires() {
        let mut combo = ComboTracker::default();
        combo.hit();
        assert!(combo.stack > 0);
        for _ in 0..COMBO_WINDOW + 1 {
            combo.tick();
        }
        assert_eq!(combo.stack, 0);
    }

    #[test]
    fn test_combo_max_stack() {
        let mut combo = ComboTracker::default();
        for _ in 0..COMBO_MAX_STACK + 3 {
            combo.hit();
        }
        assert_eq!(combo.stack, COMBO_MAX_STACK);
    }

    // --- Party ---
    #[test]
    fn test_party_solo() {
        let p = Party::new_solo(b"TEST", 1);
        assert_eq!(p.size(), 1);
        assert!((p.xp_multiplier() - 1.0).abs() < 0.001);
        assert!(p.can_invite());
    }

    #[test]
    fn test_party_xp_bonus() {
        let mut p = Party::new_solo(b"A", 1);
        let m = PartyMember {
            name: [0; 20], name_len: 1, level: 1,
            hp_pct: 1.0, mp_pct: 1.0, pos: [0.0, 0.0], zone: 0, alive: true,
        };
        p.members.push(m);
        assert!((p.xp_multiplier() - 1.10).abs() < 0.001); // +10%
    }

    // --- Chat ---
    #[test]
    fn test_chat_log() {
        let mut log = ChatLog::new();
        assert!(log.recent().is_empty());
        let msg = ChatMessage {
            channel: ChatChannel::Local,
            sender: [0; 20], sender_len: 4,
            text: [0; 80], text_len: 5,
            timestamp: 0,
        };
        log.push(msg);
        assert_eq!(log.recent().len(), 1);
    }

    // --- World Boss ---
    #[test]
    fn test_world_boss_phases() {
        let mut wb = WorldBoss::new([50.0, 50.0]);
        wb.alive = true;
        wb.hp = wb.max_hp * 0.80;
        wb.update_phase();
        assert_eq!(wb.phase, 0);
        wb.hp = wb.max_hp * 0.40;
        wb.update_phase();
        assert_eq!(wb.phase, 1); // enraged
        wb.hp = wb.max_hp * 0.15;
        wb.update_phase();
        assert_eq!(wb.phase, 2); // desperate
    }

    #[test]
    fn test_world_boss_spawn_timer() {
        let mut wb = WorldBoss::new([50.0, 50.0]);
        assert!(!wb.alive);
        wb.spawn_timer = 3;
        wb.tick_spawn();
        wb.tick_spawn();
        wb.tick_spawn();
        assert!(wb.alive);
    }

    // --- Emotes ---
    #[test]
    fn test_emote_durations() {
        assert!(Emote::Dance.duration() > Emote::Wave.duration());
        assert!(Emote::Sit.duration() > Emote::Dance.duration());
    }

    // --- Enemy basics ---
    #[test]
    fn test_enemy_hp_progression() {
        let fallen = Enemy::new(EnemyKind::Fallen, 0.0, 0.0);
        let zombie = Enemy::new(EnemyKind::Zombie, 0.0, 0.0);
        let boss = Enemy::new(EnemyKind::Boss, 0.0, 0.0);
        assert!(fallen.max_hp < zombie.max_hp);
        assert!(zombie.max_hp < boss.max_hp);
    }

    #[test]
    fn test_dist() {
        assert!((dist([0.0, 0.0], [3.0, 4.0]) - 5.0).abs() < 0.001);
    }
}
