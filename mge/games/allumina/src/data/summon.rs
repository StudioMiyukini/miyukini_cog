//! Static tuning for each summon kind. The summon system reads a profile to
//! size the new ally relative to the owner's power (a percentage of the owner's
//! life, damage, and defence) and to drive its behaviour (melee/ranged, mobile
//! or stationary, attacker or pure totem with one or more auras).

use crate::components::summon::SummonType;

/// Per-kind summon tuning. Stat percentages are applied to the owner's resolved
/// numbers at spawn time, so summons scale with the player automatically.
#[derive(Debug, Clone, Copy)]
pub struct SummonProfile {
    pub kind: SummonType,
    pub name: &'static str,
    /// How many copies this profile spawns (e.g. a wolf pack of two).
    pub count: u32,
    /// Lifetime in seconds; `0.0` means permanent.
    pub duration: f32,
    /// Fraction of the owner's max life.
    pub hp_pct: f32,
    /// Fraction of the owner's effective per-swing damage.
    pub dmg_pct: f32,
    /// Fraction of the owner's defence.
    pub def_pct: f32,
    /// Attack reach in tiles.
    pub attack_range: f32,
    /// Seconds between swings.
    pub attack_interval: f32,
    /// Movement speed in tiles/sec.
    pub move_speed: f32,
    /// Stationary summons (totems) never path toward foes.
    pub stationary: bool,
    /// Whether this summon engages enemies in melee/ranged combat.
    pub attacks: bool,
    /// If `> 0.0`, the active cap (and the per-cast count) of this kind scales
    /// with the casting skill's level: `cap = 1 + floor((level - 1) * this)`.
    /// `1.0` adds one per level (peasants); `0.5` adds one every two levels
    /// (militia, warriors). A scaled kind uses a per-type cap and is exempt from
    /// the global per-owner cap, so a Soldat can field a large mixed host. `0.0`
    /// means a fixed [`count`](Self::count) and the shared global cap.
    pub max_per_level: f32,
    /// If `> 0.0`, this summon cleaves the foes around it this often (seconds) —
    /// the elite warrior's "Frappe de taille".
    pub cleave_interval: f32,
    /// Damage multiplier of a cleave swing (fraction of the summon's base hit).
    pub cleave_mult: f32,
    /// Radius (tiles) of a cleave swing.
    pub cleave_radius: f32,
    /// If `> 0.0`, projects an attack buff of this fraction onto allies within
    /// `aura_radius`.
    pub aura_atk_pct: f32,
    /// If `> 0.0`, projects a damage-reduction (defensive) buff onto allies.
    pub aura_def_pct: f32,
    /// If `> 0.0`, restores this fraction of each ally's max life per second.
    pub aura_heal_pct: f32,
    /// If `> 0.0`, slows enemies within `aura_radius` by this factor (a value
    /// near 1.0 effectively immobilises them).
    pub enemy_slow: f32,
    pub aura_radius: f32,
}

impl SummonProfile {
    /// Construct a base profile with every aura field zeroed; callers override
    /// the handful of fields they care about.
    const fn base(kind: SummonType, name: &'static str) -> Self {
        Self {
            kind,
            name,
            count: 1,
            duration: 0.0,
            hp_pct: 0.4,
            dmg_pct: 0.4,
            def_pct: 0.4,
            attack_range: 1.6,
            attack_interval: 1.0,
            move_speed: 3.5,
            stationary: false,
            attacks: true,
            max_per_level: 0.0,
            cleave_interval: 0.0,
            cleave_mult: 0.0,
            cleave_radius: 0.0,
            aura_atk_pct: 0.0,
            aura_def_pct: 0.0,
            aura_heal_pct: 0.0,
            enemy_slow: 0.0,
            aura_radius: 0.0,
        }
    }
}

// ── Soldat ────────────────────────────────────────────────────────────────
//
// The Soldat fields three synergistic hosts. Their active cap (and per-cast
// count) scales with the casting skill's level via `max_per_level`, and they
// are exempt from the global per-owner cap, so the three can stack into a small
// army. When at least one of each is present the summon system grants every one
// of the Soldat's summons a synergy attack bonus (see `systems::summon`).

/// Soldat: a cheap peasant swarm — very frail (20% of the owner's power) but
/// numerous (one extra per skill level) and fast, mobbing a single foe.
pub const PEASANTS: SummonProfile = SummonProfile {
    duration: 20.0,
    hp_pct: 0.20,
    dmg_pct: 0.20,
    def_pct: 0.20,
    attack_range: 1.4,
    attack_interval: 0.8,
    move_speed: 4.6,
    max_per_level: 1.0,
    ..SummonProfile::base(SummonType::Peasant, "Paysan")
};

/// Soldat: shielded militiamen — average offence (40%) but high defence and
/// solid life, made to tank and hold the line (one extra every two levels).
pub const MILITIAMEN: SummonProfile = SummonProfile {
    duration: 30.0,
    hp_pct: 0.55,
    dmg_pct: 0.40,
    def_pct: 1.20,
    attack_range: 1.6,
    attack_interval: 1.1,
    move_speed: 3.4,
    max_per_level: 0.5,
    ..SummonProfile::base(SummonType::Militiaman, "Milicien")
};

/// Soldat: elite heavy-knight warriors — strong (60%) with very high life, that
/// periodically cleave the foes around them (one extra every two levels).
pub const ELITE_WARRIORS: SummonProfile = SummonProfile {
    duration: 40.0,
    hp_pct: 0.80,
    dmg_pct: 0.60,
    def_pct: 0.70,
    attack_range: 1.8,
    attack_interval: 1.0,
    move_speed: 3.6,
    max_per_level: 0.5,
    cleave_interval: 5.0,
    cleave_mult: 1.5,
    cleave_radius: 2.0,
    ..SummonProfile::base(SummonType::EliteWarrior, "Guerrier")
};

// ── Clerc ───────────────────────────────────────────────────────────────────
//
// The Clerc fields three angelic invocations whose stat fractions are *base*
// numbers — the actual spawned stats are scaled at spawn time by the casting
// skill's level and by the "Élu prophétique" passive (see
// [`crate::systems::summon::spawn_summon`]).

/// Clerc Chérubin: a small flying healer. Stationary-ish support invocation
/// whose only contribution is a healing aura on nearby allies. Lifetime ticks
/// down and the heal fraction is amplified at spawn-time by its skill rank.
pub const CHERUBIN: SummonProfile = SummonProfile {
    duration: 30.0,
    hp_pct: 0.20,
    dmg_pct: 0.0,
    def_pct: 0.30,
    attack_range: 0.0,
    attack_interval: 0.0,
    move_speed: 2.5,
    stationary: false,
    attacks: false,
    aura_heal_pct: 0.01,
    aura_radius: 5.0,
    ..SummonProfile::base(SummonType::Cherubin, "Chérubin")
};

/// Clerc Ange gardien: a permanent flying melee tank. The 30%/50% base scales
/// per rank and via the prophet passive at spawn-time.
pub const GUARDIAN_ANGEL: SummonProfile = SummonProfile {
    duration: 0.0,
    hp_pct: 0.50,
    dmg_pct: 0.30,
    def_pct: 0.30,
    attack_range: 1.7,
    attack_interval: 1.2,
    move_speed: 3.6,
    ..SummonProfile::base(SummonType::GuardianAngel, "Ange gardien")
};

/// Clerc Archange: a powerful, temporary celestial. Its 200% base scales up
/// per rank of the Chérubin and Ange-gardien skills (and the prophet passive).
/// The duration grows with the Archange rank too.
pub const ARCHANGEL: SummonProfile = SummonProfile {
    duration: 15.0,
    hp_pct: 2.00,
    dmg_pct: 2.00,
    def_pct: 2.00,
    attack_range: 2.0,
    attack_interval: 1.0,
    move_speed: 4.0,
    ..SummonProfile::base(SummonType::Archangel, "Archange")
};

// ── Mage de guerre ────────────────────────────────────────────────────────

/// Mage de guerre: a ranged elemental damage dealer.
pub const ELEMENTAL: SummonProfile = SummonProfile {
    duration: 45.0,
    hp_pct: 0.35,
    dmg_pct: 0.70,
    def_pct: 0.40,
    attack_range: 6.0,
    attack_interval: 1.2,
    move_speed: 3.0,
    ..SummonProfile::base(SummonType::Elemental, "Élémentaire")
};

/// Mage de guerre: a fixed hydra that fires a rapid triple barrage in place.
pub const HYDRA: SummonProfile = SummonProfile {
    duration: 25.0,
    hp_pct: 0.40,
    dmg_pct: 0.45,
    def_pct: 0.60,
    attack_range: 6.0,
    attack_interval: 0.6,
    move_speed: 0.0,
    stationary: true,
    attacks: true,
    ..SummonProfile::base(SummonType::Totem, "Hydre")
};

// ── Archer ────────────────────────────────────────────────────────────────

/// Archer: a fast melee hunting wolf, summoned as a pack of two.
pub const HUNT_WOLF: SummonProfile = SummonProfile {
    count: 2,
    duration: 60.0,
    hp_pct: 0.30,
    dmg_pct: 0.45,
    def_pct: 0.40,
    attack_range: 1.5,
    attack_interval: 0.7,
    move_speed: 4.4,
    ..SummonProfile::base(SummonType::Wolf, "Loup de chasse")
};

/// Archer: a jaw trap that immobilises (heavily slows) nearby enemies.
pub const JAW_TRAP: SummonProfile = SummonProfile {
    duration: 20.0,
    hp_pct: 0.20,
    def_pct: 0.50,
    attack_range: 0.0,
    attack_interval: 0.0,
    move_speed: 0.0,
    stationary: true,
    attacks: false,
    enemy_slow: 0.90,
    aura_radius: 3.0,
    ..SummonProfile::base(SummonType::Totem, "Piège à mâchoires")
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permanent_summons_carry_zero_duration() {
        assert_eq!(GUARDIAN_ANGEL.duration, 0.0);
    }

    #[test]
    fn wolves_come_in_a_pack() {
        assert_eq!(HUNT_WOLF.count, 2);
    }

    #[test]
    fn soldat_hosts_scale_per_skill_level() {
        // Peasants add one per level; militia and warriors one every two levels.
        assert_eq!(PEASANTS.max_per_level, 1.0);
        assert_eq!(MILITIAMEN.max_per_level, 0.5);
        assert_eq!(ELITE_WARRIORS.max_per_level, 0.5);
    }

    #[test]
    fn host_power_climbs_peasant_to_warrior() {
        // Peasants are the frailest, warriors the sturdiest and strongest.
        assert!(PEASANTS.hp_pct < MILITIAMEN.hp_pct);
        assert!(MILITIAMEN.hp_pct < ELITE_WARRIORS.hp_pct);
        assert!(PEASANTS.dmg_pct < ELITE_WARRIORS.dmg_pct);
        // Militia are the dedicated tanks (highest defence fraction).
        assert!(MILITIAMEN.def_pct > ELITE_WARRIORS.def_pct);
    }

    #[test]
    fn only_warriors_cleave() {
        assert!(ELITE_WARRIORS.cleave_interval > 0.0);
        assert!(ELITE_WARRIORS.cleave_mult > 1.0);
        assert_eq!(PEASANTS.cleave_interval, 0.0);
        assert_eq!(MILITIAMEN.cleave_interval, 0.0);
    }

    #[test]
    fn cherubin_heals_without_fighting() {
        assert!(CHERUBIN.aura_heal_pct > 0.0);
        assert!(!CHERUBIN.attacks);
    }

    #[test]
    fn archangel_is_the_strongest_angelic_summon() {
        assert!(ARCHANGEL.hp_pct >= GUARDIAN_ANGEL.hp_pct);
        assert!(ARCHANGEL.dmg_pct >= GUARDIAN_ANGEL.dmg_pct);
        assert!(ARCHANGEL.duration > 0.0);
    }

    #[test]
    fn jaw_trap_immobilises() {
        assert!(JAW_TRAP.enemy_slow > 0.0);
        assert!(JAW_TRAP.stationary);
    }
}
