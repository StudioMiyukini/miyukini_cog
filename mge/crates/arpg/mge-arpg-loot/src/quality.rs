//! Item quality determination using Magic Find (D2-style MF scaling).
//!
//! The [`QualityRoller`] converts item level and magic find percentage into a
//! [`DropQuality`] via weighted random rolls that mirror D2 approximate ratios.
//!
//! The standalone [`effective_mf`] and [`roll_quality`] functions extend this
//! system with per-tier diminishing returns keyed on [`ItemQuality`] from
//! `mge-arpg-items`, following exact D2 formulas.
//!
// @id: MGE-ARPG-LootQuality @do: mf-quality @role: back-end @layer: 3 @human: miyuk

use mge_arpg_items::ItemQuality;
use rand::Rng;

/// The quality tier assigned to a dropped item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DropQuality {
    /// White / common item.
    Normal,
    /// Blue / magic item.
    Magic,
    /// Yellow / rare item.
    Rare,
    /// Gold / unique item.
    Unique,
    /// Green / set item.
    Set,
}

/// Stateless roller that determines [`DropQuality`] from item level and Magic
/// Find.
pub struct QualityRoller;

impl QualityRoller {
    /// Base denominators (before MF adjustment) for each quality tier.
    /// Lower denominator = higher base chance.
    ///
    /// These approximate D2 ratios for a generic "droppable" item.
    const UNIQUE_BASE: u32 = 400;
    const SET_BASE: u32 = 500;
    const RARE_BASE: u32 = 600;
    const MAGIC_BASE: u32 = 1024;

    /// Rolls a [`DropQuality`] for the given item level and magic find
    /// percentage.
    ///
    /// `magic_find` is expressed as an integer percentage (e.g. `150` means
    /// +150% MF). The effective chance for each tier is:
    ///
    /// ```text
    /// chance = base / max(1, base - (base * effective_mf) / (effective_mf + base))
    /// ```
    ///
    /// The roll order (highest first) is: Unique -> Set -> Rare -> Magic ->
    /// Normal.
    pub fn roll(ilvl: u32, magic_find: u32, rng: &mut impl Rng) -> DropQuality {
        // Item level gives a small bonus: each 10 ilvls = +1% equivalent MF.
        let ilvl_bonus = ilvl / 10;
        let effective_mf = magic_find.saturating_add(ilvl_bonus);

        if Self::check_tier(Self::UNIQUE_BASE, effective_mf, rng) {
            return DropQuality::Unique;
        }
        if Self::check_tier(Self::SET_BASE, effective_mf, rng) {
            return DropQuality::Set;
        }
        if Self::check_tier(Self::RARE_BASE, effective_mf, rng) {
            return DropQuality::Rare;
        }
        if Self::check_tier(Self::MAGIC_BASE, effective_mf, rng) {
            return DropQuality::Magic;
        }

        DropQuality::Normal
    }

    /// Returns `true` when the roll succeeds for the given base denominator
    /// and effective magic find.
    ///
    /// Formula (D2 diminishing-returns style):
    ///   effective_chance_denom = base * base / (base + effective_mf)
    ///   success if rng(0..base) < base - effective_chance_denom
    fn check_tier(base: u32, effective_mf: u32, rng: &mut impl Rng) -> bool {
        // effective_denom decreases as MF rises, making the check easier.
        let denom = (u64::from(base) * u64::from(base))
            / (u64::from(base) + u64::from(effective_mf));
        let denom = u32::try_from(denom).unwrap_or(base);

        // Roll in [0, base). Success when roll >= denom.
        let roll = rng.gen_range(0..base);
        roll >= denom
    }
}

// ============================================================================
// MF diminishing returns — per-tier formulas (D2 exact model)
// ============================================================================

/// Computes the effective Magic Find after applying per-tier diminishing
/// returns, following the D2 formula for each [`ItemQuality`] tier.
///
/// | Tier    | Formula                          | DR cap |
/// |---------|----------------------------------|--------|
/// | Unique  | `raw * 250 / (raw + 250)`        | ~250   |
/// | Set     | `raw * 500 / (raw + 500)`        | ~500   |
/// | Rare    | `raw * 600 / (raw + 600)`        | ~600   |
/// | Magic   | `raw` (no diminishing returns)   | ∞      |
/// | Normal  | `raw` (no diminishing returns)   | ∞      |
/// | Crafted | `raw` (no diminishing returns)   | ∞      |
///
/// `raw_mf` is the raw integer percentage (e.g. `1000` = +1000 % MF).
/// Returns the effective MF value after applying the cap for the given tier.
/// A negative `raw_mf` is treated as zero.
pub fn effective_mf(raw_mf: i32, quality: ItemQuality) -> i32 {
    if raw_mf <= 0 {
        return 0;
    }
    match quality {
        ItemQuality::Unique => raw_mf * 250 / (raw_mf + 250),
        ItemQuality::Set => raw_mf * 500 / (raw_mf + 500),
        ItemQuality::Rare => raw_mf * 600 / (raw_mf + 600),
        // Magic, Normal, Crafted: no diminishing returns.
        ItemQuality::Magic | ItemQuality::Normal | ItemQuality::Crafted => raw_mf,
    }
}

/// Rolls an [`ItemQuality`] tier for a dropped item, applying MF diminishing
/// returns per tier.
///
/// # Algorithm
///
/// Base chances (1-in-N) before MF:
/// - Unique : 1/400
/// - Set    : 1/160
/// - Rare   : 1/16
/// - Magic  : 1/4
/// - Normal : fallback
///
/// The adjusted chance denominator for each tier is:
/// ```text
/// adjusted_denom = base / (1 + effective_mf(raw_mf, tier) / 100)
/// ```
/// (integer arithmetic — denominator floored, minimum 1.)
///
/// Cascade order: Unique → Set → Rare → Magic → Normal.
///
/// `mlvl` is reserved for future area-level gating; it is currently unused
/// beyond accepting the parameter to match the intended API.
pub fn roll_quality(mlvl: u32, mf: i32, rng: &mut impl Rng) -> ItemQuality {
    // mlvl reserved for future tier-gating (e.g. "ilvl too low to roll Unique").
    let _ = mlvl;

    // For each tier: compute the effective denominator after MF bonus,
    // then roll a uniform integer in [0, base_denom). Success if roll == 0,
    // which gives a 1-in-adjusted_denom chance.
    //
    // adjusted_denom = base / (1 + eff_mf / 100)
    //                = base * 100 / (100 + eff_mf)
    // We cap at 1 to guarantee at least a 1-in-base chance even with huge MF.

    let try_tier = |base: i32, quality: ItemQuality, rng: &mut dyn rand::RngCore| -> bool {
        let eff = effective_mf(mf, quality);
        // adjusted_denom = base * 100 / (100 + eff_mf), minimum 1.
        let adjusted = (base * 100) / (100 + eff).max(1);
        let adjusted = adjusted.max(1);
        rng.gen_range(0..adjusted) == 0
    };

    if try_tier(400, ItemQuality::Unique, rng) {
        return ItemQuality::Unique;
    }
    if try_tier(160, ItemQuality::Set, rng) {
        return ItemQuality::Set;
    }
    if try_tier(16, ItemQuality::Rare, rng) {
        return ItemQuality::Rare;
    }
    if try_tier(4, ItemQuality::Magic, rng) {
        return ItemQuality::Magic;
    }

    ItemQuality::Normal
}
