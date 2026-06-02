//! Skill components. The skill *tree* (passive progression) reuses
//! `mge-gameplay`; the *hotbar* of usable actives is Allumina-specific.

use serde::{Deserialize, Serialize};

pub use mge_gameplay::skills::{SkillKind, SkillTree};

use crate::components::Position2D;
use crate::data::summon::SummonProfile;
use crate::ecs::{Entity, SkillFx};

/// What an active skill does when cast. Drives the execution branch in
/// [`crate::systems::skill`] and the gating (targeted vs self-cast).
#[derive(Debug, Clone, Copy)]
pub enum SkillEffect {
    /// Hit a single enemy.
    SingleTarget,
    /// Damage every enemy within `radius` of the caster.
    AoeSelf { radius: f32 },
    /// Damage every enemy within `radius` of the targeted enemy.
    AoeTarget { radius: f32 },
    /// Hit the target, then arc to `jumps` more enemies within `radius`.
    Chain { jumps: u32, radius: f32 },
    /// Grant the caster — and allies within `radius` (0 = self only) — an attack
    /// buff for `dur` seconds.
    Buff { atk_pct: f32, dur: f32, radius: f32 },
    /// Dash to the target, then strike.
    Dash,
    /// Teleport the caster up to `dist` tiles toward the target point.
    Blink { dist: f32 },
    /// Self-centred AoE that also slows hit enemies (slow `factor` for `dur`s).
    FrostNova { radius: f32, slow: f32, dur: f32 },
    /// Drop a trap at the caster's feet that detonates near an enemy. On
    /// detonation it slows survivors by `slow` for `dur` seconds (0 = no slow).
    Trap { radius: f32, slow: f32, dur: f32 },
    /// Brief invulnerability + a short backward hop.
    Dodge { dur: f32 },
    /// Summon allied creatures near the caster, sized and timed by `profile`.
    Summon { profile: &'static SummonProfile },
    /// Hit a single enemy and stun it for `stun` seconds.
    Smite { stun: f32 },
    /// Hit a single enemy and slow its movement by `slow` for `dur` seconds.
    Frostbolt { slow: f32, dur: f32 },
    /// Hit a single enemy, then apply a poison damage-over-time (`dps` for `dur`
    /// seconds) and a defence reduction of `def_pct` for the same window.
    PoisonShot { dps: f32, dur: f32, def_pct: f32 },
    /// AoE around the targeted enemy that lands `hits` separate strikes,
    /// optionally slowing survivors by `slow` for `dur` seconds.
    MultiAoe { radius: f32, hits: u32, slow: f32, dur: f32 },
    /// Heal the caster and nearby allies by `hp_pct` of their max life per second
    /// for `dur` seconds, within `radius` tiles.
    HealField { radius: f32, hp_pct: f32, dur: f32 },
    /// Grant the caster and allies within `radius` a damage-reduction blessing of
    /// `resist_pct` for `dur` seconds.
    Bless { resist_pct: f32, dur: f32, radius: f32 },
    /// Issue a battle order: an army-wide timed buff applied to the caster and
    /// allies within `radius`. The magnitude is derived from the Soldat's
    /// invested skill levels and their synergies (see [`crate::systems::skill`]).
    Order { order: OrderType, dur: f32, radius: f32 },
    /// "Grand coup" (Soldat): one uninterruptible strike on a single enemy. Its
    /// damage scales with the wielded weapon (one- vs two-handed) and the skill
    /// rank. The caster is marked [`Unstoppable`] for `duration_base` seconds,
    /// shortened by `duration_speed_reduction` per point of the player's
    /// attacks-per-second (faster attackers recover control sooner).
    GrandCoup { duration_base: f32, duration_speed_reduction: f32 },
    /// "Taille" (Soldat): a frontal `arc_deg`-wide sweep of `radius` tiles, aimed
    /// and centred on the cursor (not the caster), hitting every enemy in the
    /// wedge. Reduced base damage with rank and Grand-coup synergy. The caster is
    /// [`Unstoppable`] for the same speed-scaled window as [`Self::GrandCoup`].
    Taille { arc_deg: f32, radius: f32, duration_base: f32, duration_speed_reduction: f32 },
    /// "Estocs" (Soldat): `hits` guaranteed thrusts against one enemy in quick
    /// succession. The sequence is uninterruptible by mobs *or* the player — it
    /// installs a [`ComboLock`] that plays out the full animation.
    Estocs { hits: u32 },

    // ── Mage de guerre: three elemental branches ───────────────────────────
    // These deal flat, level-scaled *elemental* damage (not weapon multiples)
    // with cross-branch synergies resolved in [`crate::systems::skill`].
    /// FEU — "Boule de feu": single-target fire bolt.
    Fireball,
    /// FEU — "Mur de flammes": a ground rectangle that burns over time
    /// ([`FlameWall`]).
    FlameWall,
    /// FEU — "Pluie de météores": a circular barrage of random impacts
    /// ([`MeteorField`]).
    MeteorRain { radius: f32 },
    /// GLACE — "Lance de glace": single-target frost bolt that slows.
    IceLance,
    /// GLACE — "Armure de givre": a self defensive buff that retaliates against
    /// melee attackers ([`FrostArmor`]).
    FrostArmor,
    /// GLACE — "Blizzard": a circular chilling field with a growing slow
    /// ([`BlizzardField`]).
    Blizzard { radius: f32 },
    /// FOUDRE — "Éclair": instant single-target bolt that chains to nearby foes.
    ChainBolt { radius: f32 },
    /// FOUDRE — "Bouclier de foudre": a self buff that periodically zaps nearby
    /// enemies ([`LightningShield`]).
    LightningShield,
    /// FOUDRE — "Nova électrique": a self-centred burst that damages and stuns.
    ElectricNova { radius: f32 },

    // ── Archer: three thematic branches ────────────────────────────────────
    // The Archer's nine actives. Damage values and synergies are resolved at
    // execute time from the caster's invested ranks across all archer skills.
    /// TIR — "Tir rapide": fast single-target arrow with branch synergies.
    FastShot,
    /// TIR — "Tir perçant": straight-line projectile that drills through every
    /// enemy on its path, losing power per pierce. `half_width` is the
    /// perpendicular tolerance (tiles) for an enemy to count as on the line.
    PiercingShot { half_width: f32 },
    /// TIR — "Tir chargé": a one-second charge-up culminating in a heavy single
    /// hit with a guaranteed crit window. Installs a [`ChargingShot`] that
    /// resolves when its timer elapses, cancelled by stuns/knockdowns.
    ChargedShot,
    /// PIÈGE — "Piège explosif": a cursor-placed AoE trap that detonates on the
    /// first enemy in range, dealing flat damage and slowing survivors. Capped
    /// at `max_active` simultaneous instances per caster.
    ExplosiveTrap { radius: f32, max_active: u32, lifetime: f32 },
    /// PIÈGE — "Piège à mâchoires": a cursor-placed snap trap that immobilises
    /// every foe in its small radius. Capped at `max_active` per caster.
    JawTrap { radius: f32, max_active: u32, lifetime: f32 },
    /// AGILITÉ — "Évasion": a directional dodge roll granting brief
    /// invulnerability and travelling `dist` tiles toward the cursor (or the
    /// caster's facing if no cursor).
    DodgeRoll { dist: f32, invuln: f32 },
    /// AGILITÉ — "Pluie de flèches": a cursor-placed circular zone that rains
    /// arrows for several ticks ([`ArrowRainField`]).
    ArrowRain { radius: f32 },
    /// AGILITÉ — "Instinct du chasseur": a self-buff granting bonus crit chance
    /// and dodge for a window, plus a reveal radius for hidden foes (cosmetic
    /// today; the field is read by future stealth mechanics).
    HunterInstinct,

    // ── Clerc: three thematic branches ─────────────────────────────────────
    /// SOUTIEN — Toggle an aura on/off. Only one Clerc aura can be active at a
    /// time (a second toggle deactivates the previous one). The aura ticks every
    /// second on the caster, applying its effect to allies in the radius. The
    /// per-aura mana reservation reduces the caster's effective max mana while
    /// active.
    Aura { aura_type: AuraType },
    /// BÉNI — Immobilise enemies in a small AoE around the cursor for a brief
    /// stun window. Synergises with Exorcisme via shared zone radius.
    Prayer,
    /// BÉNI — Holy damage in an AoE around the cursor. Cross-branch synergies
    /// with Prière (radius) and Conversion / Pardon (damage).
    Exorcism,
    /// BÉNI — Charm a single enemy into a temporary ally that attacks other foes.
    /// Does not work on bosses.
    Conversion,
    /// BÉNI — Mark an enemy: any attacker striking it heals for a percentage of
    /// the mark's max HP per hit.
    Pardon,
    /// BÉNI — Begin a long cast that resurrects a dead ally (mercenary in solo).
    Resurrection,
    /// DIVIN — A permanent, always-on passive (Élu prophétique) that boosts
    /// every angelic summon's stats based on its rank.
    Passive { passive_type: PassiveType },
}

/// The six Clerc support auras (Branche SOUTIEN). Only one can be active at a
/// time and each reserves mana on the caster while toggled on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuraType {
    /// "Aura de soin" — Restore a fraction of each ally's max HP every second.
    Heal,
    /// "Aura de défense" — Stacking defence bonus for allies in radius.
    Defense,
    /// "Aura de résistance" — Elemental + physical resistance bonus.
    Resist,
    /// "Aura de guerre" — Damage bonus for allies in radius.
    War,
    /// "Aura du vent" — Movement and attack speed bonus.
    Wind,
    /// "Aura de vengeance" — Reflect a fraction of incoming damage.
    Vengeance,
}

/// A learnable passive (Branche DIVIN). Passives have no cast effect; their
/// rank is read by other systems (e.g. summon spawning) at the time their
/// bonuses apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassiveType {
    /// "Élu prophétique" — Boost every angelic summon's HP/DEF/resists/attack
    /// speed by +10% per rank.
    ProphetElect,
}

/// The Clerc's currently toggled aura: which skill is providing it, its rank
/// and type, the mana it has reserved (deducted from the caster's max) and a
/// per-second tick counter.
#[derive(Debug, Clone, Copy)]
pub struct ActiveAura {
    pub skill_id: u32,
    pub aura_type: AuraType,
    pub level: u32,
    pub reserved_mana: f32,
    pub radius: f32,
    /// Counts down from 1.0; the aura applies its effects when it reaches zero.
    pub tick_timer: f32,
}

/// Per-recipient aggregated aura bonuses (one entry per ally currently in the
/// effective radius of a Clerc aura). Refreshed every aura tick. Read by combat
/// and movement systems where they hook in; the structure carries enough fields
/// to back every aura without invalidating older code paths.
#[derive(Debug, Clone, Copy, Default)]
pub struct AuraBonus {
    pub def_pct: f32,
    pub resist_pct: f32,
    pub dmg_pct: f32,
    pub move_pct: f32,
    pub atk_speed_pct: f32,
    /// Fraction of incoming damage reflected back to the attacker.
    pub retaliate_pct: f32,
    pub remaining: f32,
}

/// A mob that has been "Converted" by the Clerc: it temporarily fights for the
/// player and its original faction is restored when the timer expires.
#[derive(Debug, Clone, Copy)]
pub struct Converted {
    pub original_faction: crate::components::Faction,
    pub remaining: f32,
}

/// A mob that has been "Pardonné": any attacker hitting it heals for
/// `leech_pct` of the mob's max HP per attack, until the timer expires.
#[derive(Debug, Clone, Copy)]
pub struct Pardoned {
    pub leech_pct: f32,
    pub remaining: f32,
}

/// An in-progress Resurrection cast: when `remaining` reaches zero the dead
/// ally is restored. Interruptible.
#[derive(Debug, Clone, Copy)]
pub struct Resurrecting {
    pub remaining: f32,
    pub mana_cost: f32,
}

/// What kind of trap a [`Trap`] is. Drives the per-kind active cap (one
/// caster can field up to N explosives and M jaw traps separately) and the
/// detonation flavour. Older callers that built a generic trap default to
/// [`TrapKind::Explosive`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrapKind {
    Explosive,
    Jaw,
}

/// The three Soldat "Ordres" — commands that buff the whole army at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    /// "En avant !" — move-speed surge.
    Advance,
    /// "Tenez bon !" — defensive bracing (DEF + HP).
    HoldFast,
    /// "Wargh !" — berserk offence (attack speed + damage) at the cost of defence.
    Wargh,
}

/// The stat deltas an [`OrderType`] grants, all as additive fractions (e.g.
/// `0.05` = +5%). `def_pct` is signed: positive bracing, negative the Wargh
/// defence sacrifice. Several active orders sum their bonuses (see
/// [`crate::ecs::World::order_bonuses`]).
#[derive(Debug, Clone, Copy, Default)]
pub struct OrderBonuses {
    pub move_speed_pct: f32,
    pub def_pct: f32,
    pub hp_pct: f32,
    pub atk_speed_pct: f32,
    pub dmg_pct: f32,
}

/// One active battle order on an entity, ticked down each frame.
#[derive(Debug, Clone, Copy)]
pub struct OrderBuff {
    pub order_type: OrderType,
    pub remaining_secs: f32,
    pub bonuses: OrderBonuses,
    /// Absolute max-life added when this order was applied, so the exact amount
    /// can be removed again when it expires.
    pub hp_added: f32,
}

impl SkillEffect {
    /// Whether casting requires a valid enemy target in range.
    pub fn needs_target(self) -> bool {
        matches!(
            self,
            Self::SingleTarget
                | Self::AoeTarget { .. }
                | Self::Chain { .. }
                | Self::Dash
                | Self::Blink { .. }
                | Self::Smite { .. }
                | Self::Frostbolt { .. }
                | Self::PoisonShot { .. }
                | Self::MultiAoe { .. }
                | Self::GrandCoup { .. }
                | Self::Estocs { .. }
                | Self::Fireball
                | Self::IceLance
                | Self::ChainBolt { .. }
                | Self::FastShot
                | Self::PiercingShot { .. }
                | Self::ChargedShot
                | Self::Conversion
                | Self::Pardon
        )
    }
}

/// Damage element. Physical attacks ignore elemental resistance; the Mage de
/// guerre's three branches deal Fire/Frost/Lightning and are mitigated by the
/// matching [`ElementResists`] entry on the defender. Poison is carried by
/// late-game mob families (Naga, swamp, Soul Devourer) and behaves like the
/// three classical elements at resolution time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Physical,
    Fire,
    Frost,
    Lightning,
    Poison,
}

/// Per-element damage resistance on a defender. Values clamp to
/// `[-0.5, 1.0]`: a negative value amplifies that element's damage
/// (vulnerability), `1.0` is full immunity (the hit deals zero).
#[derive(Debug, Clone, Copy, Default)]
pub struct ElementResists {
    pub fire: f32,
    pub frost: f32,
    pub lightning: f32,
    pub poison: f32,
}

impl ElementResists {
    /// Per-channel value for `element`. Physical returns `0.0` since the
    /// combat pipeline applies physical mitigation via the defence stat.
    #[must_use]
    pub fn value(self, element: Element) -> f32 {
        match element {
            Element::Fire => self.fire,
            Element::Frost => self.frost,
            Element::Lightning => self.lightning,
            Element::Poison => self.poison,
            Element::Physical => 0.0,
        }
    }

    /// `true` if `element` deals zero damage against this defender.
    #[must_use]
    pub fn is_immune_to(self, element: Element) -> bool {
        self.value(element) >= 1.0 - f32::EPSILON
    }

    /// Clamp every channel to the legal `[-0.5, 1.0]` band.
    #[must_use]
    pub fn clamped(self) -> Self {
        Self {
            fire: self.fire.clamp(-0.5, 1.0),
            frost: self.frost.clamp(-0.5, 1.0),
            lightning: self.lightning.clamp(-0.5, 1.0),
            poison: self.poison.clamp(-0.5, 1.0),
        }
    }
}

/// A persistent ground rectangle of fire (Mage "Mur de flammes"), oriented along
/// `dir` (a unit vector) and centred on `center`. Burns every enemy inside the
/// `2*half_len` × `2*half_wid` footprint each `tick_interval` seconds.
#[derive(Debug, Clone, Copy)]
pub struct FlameWall {
    pub owner: Entity,
    pub center: Position2D,
    pub dir: (f32, f32),
    pub half_len: f32,
    pub half_wid: f32,
    pub dmg_per_tick: f32,
    pub tick_interval: f32,
    pub tick_timer: f32,
    pub remaining: f32,
}

/// A persistent chilling field (Mage "Blizzard"): continuous frost damage and a
/// slow that deepens with `elapsed` time, within `radius` of `center`.
#[derive(Debug, Clone, Copy)]
pub struct BlizzardField {
    pub owner: Entity,
    pub center: Position2D,
    pub radius: f32,
    pub dmg_per_sec: f32,
    pub elapsed: f32,
    pub remaining: f32,
}

/// A timed meteor barrage (Mage "Pluie de météores"): `impacts_left` strikes
/// land at random points within `radius` of `center`, one every
/// `impact_interval` seconds.
#[derive(Debug, Clone, Copy)]
pub struct MeteorField {
    pub owner: Entity,
    pub center: Position2D,
    pub radius: f32,
    pub dmg_per_impact: f32,
    pub impacts_left: u32,
    pub impact_interval: f32,
    pub impact_timer: f32,
}

/// Self-buff (Mage "Armure de givre"): raises the owner's defence by `def_pct`
/// and reflects `reflect` frost damage to any melee attacker, for `remaining`
/// seconds.
#[derive(Debug, Clone, Copy)]
pub struct FrostArmor {
    pub owner: Entity,
    pub def_pct: f32,
    pub reflect: f32,
    pub remaining: f32,
}

/// Self-buff (Mage "Bouclier de foudre"): every `interval` seconds, zaps every
/// enemy within `radius` of the owner for `dmg` lightning damage, for
/// `remaining` seconds.
#[derive(Debug, Clone, Copy)]
pub struct LightningShield {
    pub owner: Entity,
    pub dmg: f32,
    pub radius: f32,
    pub interval: f32,
    pub timer: f32,
    pub remaining: f32,
}

/// A usable active skill definition (lives on the hotbar).
#[derive(Debug, Clone)]
pub struct ActiveSkill {
    pub id: u32,
    pub name: String,
    /// Skill rank, starting at 1. Summon skills scale the size of the host they
    /// raise with this (see [`crate::systems::summon`]).
    pub level: u32,
    pub kind: SkillKind,
    pub mana_cost: f32,
    /// Cooldown in seconds.
    pub cooldown: f32,
    /// Damage multiplier applied to the attacker's weapon damage.
    pub damage_mult: f32,
    /// Effective reach in tiles.
    pub range: f32,
    /// Behaviour when cast.
    pub effect: SkillEffect,
    /// Visual flavour emitted on cast.
    pub fx: SkillFx,
}

/// A temporary attack buff (e.g. War Cry).
#[derive(Debug, Clone, Copy)]
pub struct Buff {
    pub atk_pct: f32,
    pub remaining: f32,
}

/// A movement slow applied by a chilling effect.
#[derive(Debug, Clone, Copy)]
pub struct Slow {
    /// Speed multiplier in `(0,1]`.
    pub factor: f32,
    pub remaining: f32,
}

/// A poison-style damage-over-time, attributed to `owner` for the damage feed.
#[derive(Debug, Clone, Copy)]
pub struct Dot {
    pub owner: Entity,
    /// Damage applied per second.
    pub dps: f32,
    pub remaining: f32,
}

/// A defence-reduction debuff (e.g. from a poisoned arrow).
#[derive(Debug, Clone, Copy)]
pub struct DefDebuff {
    /// Fraction of defence stripped, in `[0,1]`.
    pub pct: f32,
    pub remaining: f32,
}

/// A damage-reduction blessing on an ally (e.g. the Clerc's Bénédiction).
#[derive(Debug, Clone, Copy)]
pub struct Resist {
    /// Fraction of incoming damage prevented, in `[0,1]`.
    pub pct: f32,
    pub remaining: f32,
}

/// A healing field centred on its owner that mends nearby allies over time.
#[derive(Debug, Clone, Copy)]
pub struct HealField {
    pub radius: f32,
    /// Fraction of each ally's max life restored per second.
    pub hp_pct: f32,
    pub remaining: f32,
}

/// A deployed trap that detonates when an enemy steps near.
#[derive(Debug, Clone, Copy)]
pub struct Trap {
    pub owner: Entity,
    pub kind: TrapKind,
    pub radius: f32,
    /// Flat damage on detonation (replaces the weapon multiple for
    /// archer-placed traps).
    pub damage: f32,
    /// Seconds before the trap expires on its own.
    pub life: f32,
    /// Movement slow applied to survivors on detonation (0 = none, immobile).
    pub slow: f32,
    /// Duration of the detonation slow.
    pub slow_dur: f32,
}

/// A queued heavy shot in mid-charge ("Tir chargé"). Ticked every frame; when
/// `remaining` reaches zero it resolves a single high-damage hit with bonus
/// crit. Cancelled by stuns/knockdowns/pushbacks on the owner or if the
/// target dies before release.
#[derive(Debug, Clone, Copy)]
pub struct ChargingShot {
    pub target: Entity,
    pub remaining: f32,
    pub dmg_mult: f32,
    /// Additive crit-chance bonus in `[0,100]` percent for the resolved shot.
    pub crit_bonus_pct: u32,
}

/// A self-buff granted by "Instinct du chasseur": raises crit chance and dodge
/// for `remaining` seconds. Read by combat resolution.
#[derive(Debug, Clone, Copy)]
pub struct HunterInstinctBuff {
    pub remaining: f32,
    /// Additive crit-chance percentage (0–100).
    pub crit_pct: u32,
    /// Additive dodge stat (folded into the dexterity-style dodge formula).
    pub dodge_pts: f32,
    /// Cosmetic reveal radius (tiles); reserved for future stealth mechanics.
    pub reveal_radius: f32,
}

/// A persistent arrow-rain zone (Archer "Pluie de flèches"): a circular field
/// at `center` that lands one volley of arrows every `tick_interval` seconds
/// for `remaining` seconds, hitting every foe inside the disc.
#[derive(Debug, Clone, Copy)]
pub struct ArrowRainField {
    pub owner: Entity,
    pub center: Position2D,
    pub radius: f32,
    pub dmg_per_tick: f32,
    pub tick_interval: f32,
    pub tick_timer: f32,
    pub remaining: f32,
}

/// An uninterruptible-animation window: while present the entity ignores stuns
/// and cannot be interrupted (set by the Soldat melee techniques). Ticked down
/// each frame and removed at zero.
#[derive(Debug, Clone, Copy)]
pub struct Unstoppable {
    pub remaining: f32,
}

/// A locked melee combo (the Soldat "Estocs"): `hits_remaining` guaranteed
/// strikes against `target`, one every `interval` seconds (`timer` counts down
/// to the next). While the lock is present its owner can neither move, attack,
/// nor be interrupted.
#[derive(Debug, Clone, Copy)]
pub struct ComboLock {
    pub hits_remaining: u32,
    pub interval: f32,
    pub timer: f32,
    pub target: Entity,
    pub dmg_per_hit: f64,
}

/// A cooldown timer.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cooldown {
    pub remaining: f32,
}

impl Cooldown {
    pub fn ready() -> Self {
        Self { remaining: 0.0 }
    }

    pub fn is_ready(self) -> bool {
        self.remaining <= 0.0
    }

    pub fn trigger(&mut self, seconds: f32) {
        self.remaining = seconds;
    }

    pub fn tick(&mut self, dt: f32) {
        if self.remaining > 0.0 {
            self.remaining -= dt;
        }
    }
}

/// The player's action bar: parallel vectors of skills and their cooldowns.
#[derive(Debug, Clone, Default)]
pub struct SkillSlot {
    pub bar: Vec<ActiveSkill>,
    pub cooldowns: Vec<Cooldown>,
    pub selected: usize,
}

impl SkillSlot {
    pub fn from_skills(skills: Vec<ActiveSkill>) -> Self {
        let cooldowns = vec![Cooldown::ready(); skills.len()];
        Self { bar: skills, cooldowns, selected: 0 }
    }

    pub fn selected_skill(&self) -> Option<&ActiveSkill> {
        self.bar.get(self.selected)
    }

    pub fn select(&mut self, idx: usize) {
        if idx < self.bar.len() {
            self.selected = idx;
        }
    }

    pub fn tick(&mut self, dt: f32) {
        for cd in &mut self.cooldowns {
            cd.tick(dt);
        }
    }
}
