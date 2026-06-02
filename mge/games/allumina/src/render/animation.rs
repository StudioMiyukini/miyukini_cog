//! Animation state for entities. With no sprite sheets, "animation" is
//! procedural: each state modulates the placeholder quad's scale and tint
//! (idle bob, walk bounce, attack lunge, hit flash, crit white-out, death
//! fade). Phase 4 skills and Phase 2 combat events drive the state changes.

use serde::{Deserialize, Serialize};

use super::vertex::{self, Color};

/// High-level animation states an entity can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimState {
    Idle,
    Walk,
    Attack,
    /// Spell-casting: arm raised, a brief glow at the hand.
    Cast,
    Hit,
    Death,
}

/// Per-entity procedural animation driver.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationController {
    pub state: AnimState,
    /// Seconds spent in the current state.
    pub timer: f32,
    /// Free-running clock for idle/walk oscillation.
    clock: f32,
    /// Set when the last hit was a critical (drives a white flash).
    pub crit: bool,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self { state: AnimState::Idle, timer: 0.0, clock: 0.0, crit: false }
    }
}

/// How long transient states last before reverting to idle/walk.
const HIT_TIME: f32 = 0.22;
const ATTACK_TIME: f32 = 0.18;
const CAST_TIME: f32 = 0.34;
/// Death fade duration.
pub const DEATH_TIME: f32 = 0.6;

/// A sampled animation pose for one frame, consumed by the multi-quad figure
/// renderer. Unlike [`AnimationController::modulate`] (a single scale+tint for
/// the old square placeholder), this exposes the individual channels a jointed
/// silhouette needs: a vertical bob, a limb swing, an attack lunge, and a
/// colour transform (hit flash / death fade) applied per body part.
#[derive(Debug, Clone, Copy)]
pub struct AnimSample {
    /// Overall multiplicative size (death shrink, attack lunge, hit pulse).
    pub scale: f32,
    /// Vertical offset in unscaled figure px (negative = up): idle/walk bounce.
    pub bob: f32,
    /// Limb swing in `[-1, 1]` driving alternating legs/arms while walking.
    pub swing: f32,
    /// Attack progress in `[0, 1]` (a single swing arc), else `0`.
    pub attack: f32,
    /// Overall alpha multiplier (death fade, summon translucency).
    pub alpha: f32,
    /// Colour to blend each part toward (hit/crit flash).
    flash: Color,
    /// Blend amount toward `flash` in `[0, 1]`.
    flash_amt: f32,
    /// Darken amount toward black in `[0, 1]` (death).
    dark: f32,
}

impl AnimSample {
    /// A neutral standing pose for entities with no controller (NPCs, etc.).
    #[must_use]
    pub fn still() -> Self {
        Self { scale: 1.0, bob: 0.0, swing: 0.0, attack: 0.0, alpha: 1.0, flash: vertex::WHITE, flash_amt: 0.0, dark: 0.0 }
    }

    /// Apply this frame's colour transform to a body part's base colour.
    #[must_use]
    pub fn tint(&self, base: Color) -> Color {
        let c = vertex::lerp(base, self.flash, self.flash_amt);
        let c = vertex::lerp(c, vertex::BLACK, self.dark);
        vertex::with_alpha(c, (base[3] * self.alpha).clamp(0.0, 1.0))
    }
}

impl AnimationController {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enter a transient state (hit/attack), unless already dying.
    pub fn trigger(&mut self, state: AnimState, crit: bool) {
        if self.state == AnimState::Death {
            return;
        }
        self.state = state;
        self.timer = 0.0;
        self.crit = crit;
    }

    /// Set a persistent locomotion state (idle/walk) without disturbing a
    /// transient hit/attack/death in progress.
    pub fn set_locomotion(&mut self, walking: bool) {
        match self.state {
            AnimState::Idle | AnimState::Walk => {
                self.state = if walking { AnimState::Walk } else { AnimState::Idle };
            }
            _ => {}
        }
    }

    pub fn is_dead(&self) -> bool {
        self.state == AnimState::Death
    }

    /// `true` once the death fade has fully elapsed (safe to despawn visually).
    pub fn death_finished(&self) -> bool {
        self.state == AnimState::Death && self.timer >= DEATH_TIME
    }

    pub fn tick(&mut self, dt: f32) {
        self.timer += dt;
        self.clock += dt;
        match self.state {
            AnimState::Hit if self.timer >= HIT_TIME => self.state = AnimState::Idle,
            AnimState::Attack if self.timer >= ATTACK_TIME => self.state = AnimState::Idle,
            AnimState::Cast if self.timer >= CAST_TIME => self.state = AnimState::Idle,
            _ => {}
        }
    }

    /// Sample the full jointed pose for this frame (used by the figure renderer).
    #[must_use]
    pub fn sample(&self) -> AnimSample {
        let mut s = AnimSample::still();
        match self.state {
            AnimState::Idle => {
                s.bob = (self.clock * 2.6).sin() * 1.4;
                s.swing = (self.clock * 1.8).sin() * 0.12;
            }
            AnimState::Walk => {
                s.bob = -(self.clock * 11.0).sin().abs() * 3.2;
                s.swing = (self.clock * 11.0).sin();
            }
            AnimState::Attack => {
                let t = (self.timer / ATTACK_TIME).clamp(0.0, 1.0);
                s.attack = (t * std::f32::consts::PI).sin();
                s.scale = 1.0 + s.attack * 0.08;
            }
            AnimState::Cast => {
                // Hold the arm up through the cast, with a faint warm glow.
                let t = (self.timer / CAST_TIME).clamp(0.0, 1.0);
                s.attack = (t * std::f32::consts::PI).sin().max(0.35);
                s.bob = (self.clock * 3.0).sin() * 0.6;
                s.flash = [1.0, 0.95, 0.7, 1.0];
                s.flash_amt = (t * std::f32::consts::PI).sin() * 0.2;
            }
            AnimState::Hit => {
                let t = (self.timer / HIT_TIME).clamp(0.0, 1.0);
                s.flash = if self.crit { vertex::WHITE } else { [1.0, 0.25, 0.25, 1.0] };
                s.flash_amt = (1.0 - t) * 0.85;
                s.scale = 1.0 + (1.0 - t) * 0.12;
            }
            AnimState::Death => {
                let t = (self.timer / DEATH_TIME).clamp(0.0, 1.0);
                s.dark = t * 0.8;
                s.alpha = (1.0 - t).max(0.0);
                s.scale = (1.0 - t * 0.5).max(0.1);
            }
        }
        s
    }

    /// Returns `(scale_multiplier, tint)` to apply to the entity's base sprite.
    #[must_use]
    pub fn modulate(&self, base: Color) -> (f32, Color) {
        match self.state {
            AnimState::Idle => {
                let bob = 1.0 + (self.clock * 3.0).sin() * 0.03;
                (bob, base)
            }
            AnimState::Walk => {
                let bob = 1.0 + (self.clock * 12.0).sin().abs() * 0.08;
                (bob, base)
            }
            AnimState::Attack => {
                let t = (self.timer / ATTACK_TIME).clamp(0.0, 1.0);
                // Quick lunge: scale up then settle.
                let lunge = 1.0 + (t * std::f32::consts::PI).sin() * 0.22;
                (lunge, base)
            }
            AnimState::Cast => {
                let t = (self.timer / CAST_TIME).clamp(0.0, 1.0);
                let glow = vertex::lerp(base, [1.0, 0.95, 0.7, 1.0], (t * std::f32::consts::PI).sin() * 0.3);
                (1.0 + (t * std::f32::consts::PI).sin() * 0.06, glow)
            }
            AnimState::Hit => {
                let t = (self.timer / HIT_TIME).clamp(0.0, 1.0);
                let flash_to = if self.crit { vertex::WHITE } else { [1.0, 0.2, 0.2, 1.0] };
                let tint = vertex::lerp(flash_to, base, t);
                let pulse = 1.0 + (1.0 - t) * 0.18;
                (pulse, tint)
            }
            AnimState::Death => {
                let t = (self.timer / DEATH_TIME).clamp(0.0, 1.0);
                let tint = vertex::with_alpha(vertex::lerp(base, vertex::BLACK, t), 1.0 - t);
                let shrink = (1.0 - t * 0.6).max(0.05);
                (shrink, tint)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_reverts_to_idle_after_window() {
        let mut a = AnimationController::new();
        a.trigger(AnimState::Hit, false);
        a.tick(HIT_TIME + 0.01);
        assert_eq!(a.state, AnimState::Idle);
    }

    #[test]
    fn death_is_terminal() {
        let mut a = AnimationController::new();
        a.trigger(AnimState::Death, false);
        a.trigger(AnimState::Hit, false);
        assert_eq!(a.state, AnimState::Death);
    }

    #[test]
    fn death_finishes_after_fade() {
        let mut a = AnimationController::new();
        a.trigger(AnimState::Death, false);
        assert!(!a.death_finished());
        a.tick(DEATH_TIME + 0.01);
        assert!(a.death_finished());
    }

    #[test]
    fn death_fade_reduces_alpha() {
        let mut a = AnimationController::new();
        a.trigger(AnimState::Death, false);
        a.tick(DEATH_TIME * 0.5);
        let (_, tint) = a.modulate(vertex::MOB);
        assert!(tint[3] < 1.0);
    }
}
