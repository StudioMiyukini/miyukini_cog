use serde::{Deserialize, Serialize};

use crate::actions::EntityId;
use crate::stats::{Modifier, StatKind, StatModifier};

/// Duration type for effects.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Duration {
    Timed(f32),
    Permanent,
    UntilCancelled,
}

impl Duration {
    pub fn is_expired(&self, elapsed: f32) -> bool {
        if let Duration::Timed(max) = self {
            elapsed >= *max
        } else {
            false
        }
    }
}

/// Active buff or debuff on an entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveEffect {
    pub id: u64,
    pub name: String,
    pub is_debuff: bool,
    pub source: EntityId,
    pub elapsed: f32,
    pub duration: Duration,
    pub stacks: u8,
    pub max_stacks: u8,
    /// Stat modifiers applied while this effect is active.
    pub modifiers: Vec<StatModifier>,
}

impl ActiveEffect {
    pub fn is_expired(&self) -> bool {
        self.duration.is_expired(self.elapsed)
    }

    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
    }
}

/// An aura that projects stats in a radius around the caster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aura {
    pub skill_id: u32,
    pub caster: EntityId,
    pub radius: f32,
    pub active: bool,
    pub modifiers: Vec<(StatKind, Modifier)>,
}

impl Aura {
    pub fn new(skill_id: u32, caster: EntityId, radius: f32) -> Self {
        Self { skill_id, caster, radius, active: true, modifiers: Vec::new() }
    }

    pub fn add_modifier(&mut self, stat: StatKind, modifier: Modifier) {
        self.modifiers.push((stat, modifier));
    }
}

/// The effect manager for a single entity.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EffectStack {
    pub effects: Vec<ActiveEffect>,
}

impl EffectStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, mut effect: ActiveEffect) {
        // Stack if already present and below max_stacks.
        if let Some(existing) = self.effects.iter_mut().find(|e| e.name == effect.name) {
            if existing.stacks < existing.max_stacks {
                existing.stacks += 1;
                existing.elapsed = 0.0; // refresh duration
            }
            return;
        }
        effect.stacks = 1;
        self.effects.push(effect);
    }

    pub fn remove_by_id(&mut self, id: u64) {
        self.effects.retain(|e| e.id != id);
    }

    pub fn tick(&mut self, dt: f32) {
        for e in &mut self.effects {
            e.tick(dt);
        }
        self.effects.retain(|e| !e.is_expired());
    }

    pub fn has_debuff(&self) -> bool {
        self.effects.iter().any(|e| e.is_debuff)
    }

    pub fn count(&self) -> usize {
        self.effects.len()
    }
}

// ─── Summons / Traps (T11) ────────────────────────────────────────────────

/// Kind of summoned entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SummonKind {
    Skeleton,
    Revived,
    GolemClay,
    Totem,
    Trap,
    ShadowWarrior,
}

/// A single summoned entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summon {
    pub id: EntityId,
    pub owner: EntityId,
    pub kind: SummonKind,
    pub life: f32,
    pub max_life: f32,
    pub skill_id: u32,
    pub active: bool,
}

impl Summon {
    pub fn is_alive(&self) -> bool {
        self.active && self.life > 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.life = (self.life - amount).max(0.0);
        if self.life <= 0.0 {
            self.active = false;
        }
    }
}

/// Manager for all summons belonging to one owner.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SummonPool {
    pub summons: Vec<Summon>,
    pub max_skeletons: u8,
    pub max_totems: u8,
    pub max_traps: u8,
}

impl SummonPool {
    pub fn new() -> Self {
        Self { max_skeletons: 9, max_totems: 3, max_traps: 5, summons: Vec::new() }
    }

    pub fn add(&mut self, summon: Summon) -> bool {
        let count = self.summons.iter().filter(|s| s.kind == summon.kind && s.is_alive()).count();
        let cap = match summon.kind {
            SummonKind::Skeleton | SummonKind::Revived | SummonKind::ShadowWarrior => {
                usize::from(self.max_skeletons)
            }
            SummonKind::GolemClay => 1,
            SummonKind::Totem => usize::from(self.max_totems),
            SummonKind::Trap => usize::from(self.max_traps),
        };
        if count >= cap {
            // Dismiss oldest to make room (D2 behavior).
            if let Some(oldest) =
                self.summons.iter_mut().find(|s| s.kind == summon.kind && s.is_alive())
            {
                oldest.active = false;
            }
        }
        self.summons.push(summon);
        true
    }

    pub fn alive_count(&self, kind: SummonKind) -> usize {
        self.summons.iter().filter(|s| s.kind == kind && s.is_alive()).count()
    }

    /// Purge dead summons.
    pub fn prune(&mut self) {
        self.summons.retain(Summon::is_alive);
    }
}

// ─── Armor, block, breakpoints, crits (T12) ──────────────────────────────

/// Effective block rate capped at 75%.
pub fn effective_block(block_chance_pct: i32) -> i32 {
    block_chance_pct.clamp(0, 75)
}

/// Hit recovery frames (simplified, breakpoint tiers).
pub fn hit_recovery_frames(faster_hit_recovery_pct: i32) -> u32 {
    match faster_hit_recovery_pct {
        0..=6 => 13,
        7..=14 => 11,
        15..=26 => 9,
        27..=39 => 8,
        40..=59 => 7,
        60..=85 => 6,
        86..=119 => 5,
        120..=199 => 4,
        _ => 3,
    }
}

/// Faster cast rate frames (simplified Sorceress breakpoints).
pub fn cast_rate_frames(faster_cast_rate_pct: i32) -> u32 {
    match faster_cast_rate_pct {
        0..=8 => 13,
        9..=19 => 12,
        20..=34 => 11,
        35..=54 => 10,
        55..=79 => 9,
        80..=109 => 8,
        110..=142 => 7,
        143..=199 => 6,
        _ => 5,
    }
}

// ─── Death, corpse, town return (T13) ────────────────────────────────────

/// State of a character body (for corpse mechanics).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorpseState {
    Fresh,
    Raised,
    Exploded,
    Expired,
}

/// A corpse in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Corpse {
    pub entity_id: EntityId,
    pub state: CorpseState,
    pub lifetime_remaining: f32,
}

impl Corpse {
    pub fn new(entity_id: EntityId, lifetime: f32) -> Self {
        Self { entity_id, state: CorpseState::Fresh, lifetime_remaining: lifetime }
    }

    pub fn tick(&mut self, dt: f32) {
        if self.state == CorpseState::Fresh {
            self.lifetime_remaining -= dt;
            if self.lifetime_remaining <= 0.0 {
                self.state = CorpseState::Expired;
            }
        }
    }

    pub fn is_usable(&self) -> bool {
        self.state == CorpseState::Fresh
    }
}

/// Death event for a player character.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerDeath {
    pub entity_id: EntityId,
    pub gold_penalty_pct: u8,
    pub experience_penalty_pct: u8,
}

impl PlayerDeath {
    pub fn new(entity_id: EntityId) -> Self {
        // D2-like: lose 10% gold and 5% XP on death in non-hardcore.
        Self { entity_id, gold_penalty_pct: 10, experience_penalty_pct: 5 }
    }

    pub fn gold_lost(&self, current_gold: u64) -> u64 {
        current_gold * u64::from(self.gold_penalty_pct) / 100
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn xp_lost(&self, current_xp: u64) -> u64 {
        (current_xp as f64 * f64::from(self.experience_penalty_pct) / 100.0) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_stack_applies_and_ticks() {
        let mut stack = EffectStack::new();
        stack.apply(ActiveEffect {
            id: 1,
            name: "Poison".into(),
            is_debuff: true,
            source: 0,
            elapsed: 0.0,
            duration: Duration::Timed(2.0),
            stacks: 0,
            max_stacks: 3,
            modifiers: vec![],
        });
        assert_eq!(stack.count(), 1);
        assert!(stack.has_debuff());
        stack.tick(1.0);
        assert_eq!(stack.count(), 1);
        stack.tick(1.5);
        assert_eq!(stack.count(), 0);
    }

    #[test]
    fn effect_stacks_up_to_max() {
        let mut stack = EffectStack::new();
        let make_eff = || ActiveEffect {
            id: 1,
            name: "Bleed".into(),
            is_debuff: true,
            source: 0,
            elapsed: 0.0,
            duration: Duration::Timed(5.0),
            stacks: 0,
            max_stacks: 2,
            modifiers: vec![],
        };
        stack.apply(make_eff());
        stack.apply(make_eff());
        stack.apply(make_eff()); // exceeds max → refresh, not add
        assert_eq!(stack.count(), 1);
        assert_eq!(stack.effects[0].stacks, 2);
    }

    #[test]
    fn summon_pool_caps_and_dismisses() {
        let mut pool = SummonPool::new();
        pool.max_skeletons = 2;
        for i in 0..2 {
            pool.add(Summon {
                id: i,
                owner: 0,
                kind: SummonKind::Skeleton,
                life: 100.0,
                max_life: 100.0,
                skill_id: 400,
                active: true,
            });
        }
        assert_eq!(pool.alive_count(SummonKind::Skeleton), 2);
        pool.add(Summon {
            id: 3,
            owner: 0,
            kind: SummonKind::Skeleton,
            life: 100.0,
            max_life: 100.0,
            skill_id: 400,
            active: true,
        });
        pool.prune();
        assert_eq!(pool.alive_count(SummonKind::Skeleton), 2);
    }

    #[test]
    fn hit_recovery_breakpoints() {
        assert_eq!(hit_recovery_frames(0), 13);
        assert_eq!(hit_recovery_frames(27), 8);
        assert_eq!(hit_recovery_frames(200), 3);
    }

    #[test]
    fn corpse_expires() {
        let mut corpse = Corpse::new(1, 30.0);
        assert!(corpse.is_usable());
        corpse.tick(31.0);
        assert_eq!(corpse.state, CorpseState::Expired);
    }

    #[test]
    fn death_penalty_calculations() {
        let death = PlayerDeath::new(1);
        assert_eq!(death.gold_lost(1000), 100);
        assert_eq!(death.xp_lost(1000), 50);
    }

    #[test]
    fn block_capped_at_75() {
        assert_eq!(effective_block(100), 75);
        assert_eq!(effective_block(50), 50);
    }
}
