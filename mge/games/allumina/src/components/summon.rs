//! Summon (invocation) components. A summon is a short-lived or permanent ally
//! spawned by a player skill; it fights mobs on the player's side and inherits a
//! fraction of the player's power. Behaviour lives in [`crate::systems::summon`].

use crate::data::summon::SummonProfile;
use crate::ecs::Entity;

/// The flavour of a summoned ally. The Soldat fields three synergistic kinds
/// (peasant swarm, militia tanks, elite-warrior DPS); the other classes summon
/// two thematic kinds each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SummonType {
    /// Generic raised undead (reserved fallback).
    Skeleton,
    /// Soldat footman: a melee trooper summoned as a squad.
    Footman,
    /// Soldat peasant: a cheap, frail villager that swarms a single target.
    Peasant,
    /// Soldat militiaman: a shielded light soldier that tanks and holds the line.
    Militiaman,
    /// Soldat elite warrior: a heavy knight that focuses high-HP targets and
    /// periodically cleaves the foes around it.
    EliteWarrior,
    /// Clerc guardian angel: a sturdy flying tank.
    GuardianAngel,
    /// Clerc cherubin: a small support invocation that heals nearby allies.
    Cherubin,
    /// Clerc archange: a powerful, temporary celestial that scales with the
    /// other angelic skills.
    Archangel,
    /// Mage de guerre elemental: a ranged elemental damage dealer.
    Elemental,
    /// Archer hunting wolf: a fast melee beast, summoned as a pack.
    Wolf,
    /// Stationary fixture (sacred circle, hydra, or jaw trap).
    Totem,
}

impl SummonType {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Skeleton => "Squelette",
            Self::Footman => "Fantassin",
            Self::Peasant => "Paysan",
            Self::Militiaman => "Milicien",
            Self::EliteWarrior => "Guerrier",
            Self::GuardianAngel => "Ange gardien",
            Self::Cherubin => "Chérubin",
            Self::Archangel => "Archange",
            Self::Elemental => "Élémentaire",
            Self::Wolf => "Loup de chasse",
            Self::Totem => "Totem",
        }
    }
}

/// Marks an entity as a summon owned by `owner`.
#[derive(Debug, Clone, Copy)]
pub struct SummonTag {
    /// The player (or other entity) that summoned this ally.
    pub owner: Entity,
    pub summon_type: SummonType,
    /// Seconds of life remaining. `0.0` means permanent (lives until killed or
    /// the owner dies). A timed summon is created with a positive value and is
    /// despawned the moment it ticks down past zero, so it never lingers at 0.0.
    pub duration: f32,
    /// The world tick at which this summon was created (used for FIFO eviction
    /// when the per-owner cap is exceeded).
    pub created_tick: u64,
    /// Seconds until this summon's next special action (the elite warrior's
    /// cleave). `0.0` for kinds without a special; counts down each tick.
    pub special_cd: f32,
    /// Effective movement speed (tiles/sec), including any cross-synergy bonus
    /// baked in at spawn. Defaults to the profile's base speed. The AI reads
    /// this rather than the profile so synergy bonuses take effect.
    pub move_speed: f32,
    /// Effective healing aura fraction (per second), baked in at spawn so that
    /// rank- or passive-scaled values (e.g. the Cherubin's +0.5%/lvl bonus and
    /// the Élu prophétique passive) can override the profile's static base.
    /// Defaults to [`SummonProfile::aura_heal_pct`].
    pub aura_heal_pct: f32,
    /// Static behaviour tuning (mobility, attacks/aura). Kept on the tag so the
    /// AI can recover behaviour even when two kinds share a [`SummonType`]
    /// (e.g. the hydra and the jaw trap are both totems).
    pub profile: &'static SummonProfile,
}

impl SummonTag {
    pub fn new(
        owner: Entity,
        duration: f32,
        created_tick: u64,
        profile: &'static SummonProfile,
    ) -> Self {
        Self {
            owner,
            summon_type: profile.kind,
            duration,
            created_tick,
            special_cd: profile.cleave_interval,
            move_speed: profile.move_speed,
            aura_heal_pct: profile.aura_heal_pct,
            profile,
        }
    }

    /// Permanent summons carry a non-positive duration.
    pub fn is_permanent(&self) -> bool {
        self.duration <= 0.0
    }
}
