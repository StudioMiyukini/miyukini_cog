//! @id     games_allumina_systems_skill_archer
//! @do     definir_archer_levels
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems skill archer ».

//! Archer: the three thematic branches (Tir / Piège / Agilité). Pure
//! per-skill level curves; shared helpers (`lf`) are reached via `super`.

use super::*;

// ── Archer: three thematic branches ─────────────────────────────────────────

/// Rank invested in each of the Archer's nine active skills, read off the
/// caster's hotbar by id (0 for any not slotted). Drives the cross-branch
/// synergies (Tir/Piège/Agilité).
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct ArcherLevels {
    pub(super) fast: u32,      // "Tir rapide"           (4001)
    pub(super) piercing: u32,  // "Tir perçant"          (4002)
    pub(super) charged: u32,   // "Tir chargé"           (4003)
    pub(super) poison: u32,    // "Tir empoisonné"       (4004)
    pub(super) explosive: u32, // "Piège explosif"       (4005)
    pub(super) jaw: u32,       // "Piège à mâchoires"    (4006)
    pub(super) evasion: u32,   // "Évasion"              (4007)
    pub(super) rain: u32,      // "Pluie de flèches"     (4008)
    pub(super) instinct: u32,  // "Instinct du chasseur" (4009)
}

pub(super) fn read_archer_levels(world: &World, caster: Entity) -> ArcherLevels {
    ArcherLevels {
        fast: skill_level_by_id(world, caster, 4001),
        piercing: skill_level_by_id(world, caster, 4002),
        charged: skill_level_by_id(world, caster, 4003),
        poison: skill_level_by_id(world, caster, 4004),
        explosive: skill_level_by_id(world, caster, 4005),
        jaw: skill_level_by_id(world, caster, 4006),
        evasion: skill_level_by_id(world, caster, 4007),
        rain: skill_level_by_id(world, caster, 4008),
        instinct: skill_level_by_id(world, caster, 4009),
    }
}

// TIR ───────────────────────────────────────────────────────────────────────

/// "Tir rapide" multiplier: 0.8 + 0.05/lvl, +2%/lvl from Tir empoisonné.
pub(super) fn fast_shot_mult(a: ArcherLevels) -> f32 {
    (0.8 + 0.05 * lf(a.fast)) * (1.0 + 0.02 * lf(a.poison))
}

/// "Tir perçant" base multiplier: 1.0 + 0.08/lvl, +5%/lvl from Tir empoisonné.
pub(super) fn piercing_shot_mult(a: ArcherLevels) -> f32 {
    (1.0 + 0.08 * lf(a.piercing)) * (1.0 + 0.05 * lf(a.poison))
}

/// "Tir perçant" pierce count: 1 +1 every 3 Tir-rapide levels.
pub(super) fn piercing_pierces(a: ArcherLevels) -> u32 {
    1 + a.fast / 3
}

/// "Tir chargé" multiplier: 2.0 + 0.15/lvl, +5%/lvl from Tir perçant and
/// +3%/lvl from Tir rapide.
pub(super) fn charged_shot_mult(a: ArcherLevels) -> f32 {
    (2.0 + 0.15 * lf(a.charged))
        * (1.0 + 0.05 * lf(a.piercing) + 0.03 * lf(a.fast))
}

/// "Tir chargé" charge time: 1s, modestly faster with higher ranks (-0.02/lvl
/// floor 0.3s) so investment feels rewarding.
pub(super) fn charged_shot_charge_time(a: ArcherLevels) -> f32 {
    (1.0 - 0.02 * lf(a.charged)).max(0.3)
}

// PIÈGE ─────────────────────────────────────────────────────────────────────

/// Explosive-trap flat damage: 20 +12/lvl, +5%/lvl from Tir empoisonné.
pub(super) fn explosive_trap_dmg(a: ArcherLevels) -> f32 {
    (20.0 + 12.0 * lf(a.explosive)) * (1.0 + 0.05 * lf(a.poison))
}

/// Explosive-trap slow factor: 0.7 (30% slow) — fixed by design.
pub(super) fn explosive_trap_slow_factor(_a: ArcherLevels) -> f32 {
    0.7
}

/// Jaw-trap flat damage: 10 +5/lvl.
pub(super) fn jaw_trap_dmg(a: ArcherLevels) -> f32 {
    10.0 + 5.0 * lf(a.jaw)
}

/// Jaw-trap immobilise duration: 2s +0.3/lvl, +0.5s every 3 Piège-explosif
/// levels, +5%/lvl from Tir empoisonné.
pub(super) fn jaw_trap_immobilize_dur(a: ArcherLevels) -> f32 {
    let extra = lf(a.explosive / 3) * 0.5;
    (2.0 + 0.3 * lf(a.jaw) + extra) * (1.0 + 0.05 * lf(a.poison))
}

// AGILITÉ ───────────────────────────────────────────────────────────────────

/// Extra dodge-roll distance: +0.5 tile every 3 Pluie-de-flèches levels.
pub(super) fn dodge_roll_extra_dist(a: ArcherLevels) -> f32 {
    lf(a.rain / 3) * 0.5
}

/// Extra dodge-roll invuln: +0.1s every 5 Piège-explosif levels.
pub(super) fn dodge_roll_extra_invuln(a: ArcherLevels) -> f32 {
    lf(a.explosive / 5) * 0.1
}

/// Arrow-rain duration: 3s base.
pub(super) fn arrow_rain_duration(_a: ArcherLevels) -> f32 {
    3.0
}

/// Arrow-rain damage per 0.5s tick: 4 +2.5/lvl, +5%/lvl from Tir perçant.
pub(super) fn arrow_rain_tick_dmg(a: ArcherLevels) -> f32 {
    (4.0 + 2.5 * lf(a.rain)) * (1.0 + 0.05 * lf(a.piercing))
}

/// "Instinct du chasseur" additive crit chance (%): 10/lvl, +2%/lvl from Tir
/// chargé. Clamped at 80% to keep the ceiling reasonable.
pub(super) fn hunter_instinct_crit_pct(a: ArcherLevels) -> u32 {
    let v = (10.0 * lf(a.instinct)) * (1.0 + 0.02 * lf(a.charged));
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    {
        v.min(80.0) as u32
    }
}

/// "Instinct du chasseur" dodge stat boost: 15 +2/lvl, +2%/lvl from Évasion.
pub(super) fn hunter_instinct_dodge_pts(a: ArcherLevels) -> f32 {
    (15.0 + 2.0 * lf(a.instinct)) * (1.0 + 0.02 * lf(a.evasion))
}
