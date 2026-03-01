//! Item quality determination using Magic Find (D2-style MF scaling).
//!
//! The [`QualityRoller`] converts item level and magic find percentage into a
//! [`DropQuality`] via weighted random rolls that mirror D2 approximate ratios.

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
