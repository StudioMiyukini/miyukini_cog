//! Crowd-control components: knockdown, pushback (recul), and the interruptible
//! cast marker. All three are pure data ticked by [`crate::systems::cc`]; the
//! [`Unstoppable`](crate::components::skill::Unstoppable) window makes an entity
//! immune to every one of them (see [`crate::systems::combat::is_unstoppable`]).

/// A renversement: the entity is on the ground and can do nothing — no movement,
/// no attack, no cast — for `remaining` seconds, and takes `dmg_amp`× damage
/// while down (1.2 = +20%).
#[derive(Debug, Clone, Copy)]
pub struct Knockdown {
    pub remaining: f32,
    pub dmg_amp: f64,
}

/// A recul: the entity is shoved along the unit vector `(dx, dy)` for `distance`
/// tiles at `speed` tiles/s, unable to act until it arrives. Hitting a wall stops
/// it early and deals `wall_bonus_dmg`; an optional `knockdown_after` chains a
/// renversement on arrival (or on impact).
#[derive(Debug, Clone, Copy)]
pub struct Pushback {
    pub dx: f32,
    pub dy: f32,
    pub distance: f32,
    pub traveled: f32,
    pub speed: f32,
    pub knockdown_after: Option<f32>,
    pub wall_bonus_dmg: f64,
}

/// Marks an entity mid-cast: a brief window after a spell during which incoming
/// damage, a stun, or a knockdown cancels it (refunding half the cooldown). Casts
/// resolve instantly in this engine, so this models the recovery/channel tail —
/// melee techniques skip it because they are [`Unstoppable`](crate::components::skill::Unstoppable).
#[derive(Debug, Clone, Copy)]
pub struct Interruptible {
    pub casting: bool,
    pub skill_id: u32,
    /// Cast progress in `[0,1]`; the window closes (and the entity is no longer
    /// interruptible) once it reaches 1.0.
    pub progress: f32,
}

impl Knockdown {
    pub fn new(duration: f32) -> Self {
        Self { remaining: duration, dmg_amp: 1.2 }
    }
}

impl Interruptible {
    pub fn cast(skill_id: u32) -> Self {
        Self { casting: true, skill_id, progress: 0.0 }
    }
}
