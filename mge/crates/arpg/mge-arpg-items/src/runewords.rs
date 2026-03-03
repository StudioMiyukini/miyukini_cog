// @id: MGE-ARPG-Runewords @do: runeword-system @role: back-end @layer: 3 @human: miyuk
//! Runeword definitions and matching logic for low-tier runewords.
//!
//! A runeword activates when the correct runes are inserted **in exact order**
//! into a compatible base item type.  Each [`RunewordDef`] declares the rune
//! sequence, allowed base types, and the stat bonuses granted on activation.
//!
//! # Matching rules
//!
//! 1. The rune names must match in **exact order** (position-sensitive).
//! 2. The item's base type must be listed in [`RunewordDef::base_types`].
//! 3. The first matching runeword wins (definitions are evaluated sequentially).

// ---------------------------------------------------------------------------
// Rune name <-> numeric ID mapping (El = 1 .. Zod = 33)
// ---------------------------------------------------------------------------

/// Map a rune name to its numeric id used in [`super::sockets::SocketFiller::Rune`].
///
/// Returns `None` for unrecognized rune names.
#[must_use]
pub fn rune_name_to_id(name: &str) -> Option<u8> {
    match name {
        "El" => Some(1),
        "Eld" => Some(2),
        "Tir" => Some(3),
        "Nef" => Some(4),
        "Eth" => Some(5),
        "Ith" => Some(6),
        "Tal" => Some(7),
        "Ral" => Some(8),
        "Ort" => Some(9),
        "Thul" => Some(10),
        "Amn" => Some(11),
        "Sol" => Some(12),
        "Shael" => Some(13),
        "Dol" => Some(14),
        "Hel" => Some(15),
        "Io" => Some(16),
        "Lum" => Some(17),
        "Ko" => Some(18),
        "Fal" => Some(19),
        "Lem" => Some(20),
        "Pul" => Some(21),
        "Um" => Some(22),
        "Mal" => Some(23),
        "Ist" => Some(24),
        "Gul" => Some(25),
        "Vex" => Some(26),
        "Ohm" => Some(27),
        "Lo" => Some(28),
        "Sur" => Some(29),
        "Ber" => Some(30),
        "Jah" => Some(31),
        "Cham" => Some(32),
        "Zod" => Some(33),
        _ => None,
    }
}

/// Map a numeric rune id back to its canonical name.
///
/// Returns `None` for ids outside the valid 1..=33 range.
#[must_use]
pub fn rune_id_to_name(id: u8) -> Option<&'static str> {
    match id {
        1 => Some("El"),
        2 => Some("Eld"),
        3 => Some("Tir"),
        4 => Some("Nef"),
        5 => Some("Eth"),
        6 => Some("Ith"),
        7 => Some("Tal"),
        8 => Some("Ral"),
        9 => Some("Ort"),
        10 => Some("Thul"),
        11 => Some("Amn"),
        12 => Some("Sol"),
        13 => Some("Shael"),
        14 => Some("Dol"),
        15 => Some("Hel"),
        16 => Some("Io"),
        17 => Some("Lum"),
        18 => Some("Ko"),
        19 => Some("Fal"),
        20 => Some("Lem"),
        21 => Some("Pul"),
        22 => Some("Um"),
        23 => Some("Mal"),
        24 => Some("Ist"),
        25 => Some("Gul"),
        26 => Some("Vex"),
        27 => Some("Ohm"),
        28 => Some("Lo"),
        29 => Some("Sur"),
        30 => Some("Ber"),
        31 => Some("Jah"),
        32 => Some("Cham"),
        33 => Some("Zod"),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// RunewordDef
// ---------------------------------------------------------------------------

/// Definition of a runeword: required rune sequence, valid base types, and
/// the stat bonuses granted when activated.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RunewordDef {
    /// Display name of the runeword (e.g. `"Steel"`).
    pub name: String,
    /// Rune names in insertion order (e.g. `["Tir", "El"]`).
    pub runes: Vec<String>,
    /// Base item types that can host this runeword (e.g. `["sword", "axe"]`).
    pub base_types: Vec<String>,
    /// Stat bonuses granted when the runeword is active.
    /// Each tuple is `(stat_name, value)` where value can be negative.
    pub granted_stats: Vec<(String, i32)>,
}

// ---------------------------------------------------------------------------
// Low-tier runeword factory functions
// ---------------------------------------------------------------------------

/// **Steel** -- Tir + El -- swords, axes, maces.
///
/// +20% enhanced damage, +3 minimum damage.
#[must_use]
pub fn steel() -> RunewordDef {
    RunewordDef {
        name: "Steel".to_owned(),
        runes: vec!["Tir".to_owned(), "El".to_owned()],
        base_types: vec![
            "sword".to_owned(),
            "axe".to_owned(),
            "mace".to_owned(),
        ],
        granted_stats: vec![
            ("enhanced_damage_pct".to_owned(), 20),
            ("min_damage".to_owned(), 3),
        ],
    }
}

/// **Malice** -- Ith + El + Eth -- melee weapons.
///
/// +33% enhanced damage, -100 defense.
#[must_use]
pub fn malice() -> RunewordDef {
    RunewordDef {
        name: "Malice".to_owned(),
        runes: vec![
            "Ith".to_owned(),
            "El".to_owned(),
            "Eth".to_owned(),
        ],
        base_types: vec![
            "sword".to_owned(),
            "axe".to_owned(),
            "mace".to_owned(),
            "polearm".to_owned(),
        ],
        granted_stats: vec![
            ("enhanced_damage_pct".to_owned(), 33),
            ("defense".to_owned(), -100),
        ],
    }
}

/// **Stealth** -- Tal + Eth -- body armor.
///
/// +25% faster run/walk, +25% faster cast rate.
#[must_use]
pub fn stealth() -> RunewordDef {
    RunewordDef {
        name: "Stealth".to_owned(),
        runes: vec!["Tal".to_owned(), "Eth".to_owned()],
        base_types: vec!["armor".to_owned()],
        granted_stats: vec![
            ("faster_run_walk_pct".to_owned(), 25),
            ("faster_cast_rate_pct".to_owned(), 25),
        ],
    }
}

/// **Nadir** -- Nef + Tir -- helms.
///
/// +50% gold find, -33% requirements.
#[must_use]
pub fn nadir() -> RunewordDef {
    RunewordDef {
        name: "Nadir".to_owned(),
        runes: vec!["Nef".to_owned(), "Tir".to_owned()],
        base_types: vec!["helm".to_owned()],
        granted_stats: vec![
            ("gold_find_pct".to_owned(), 50),
            ("requirements_pct".to_owned(), -33),
        ],
    }
}

/// **Leaf** -- Tir + Ral -- staves.
///
/// +3 fire skills, +33% cold resist.
#[must_use]
pub fn leaf() -> RunewordDef {
    RunewordDef {
        name: "Leaf".to_owned(),
        runes: vec!["Tir".to_owned(), "Ral".to_owned()],
        base_types: vec!["staff".to_owned()],
        granted_stats: vec![
            ("fire_skills".to_owned(), 3),
            ("cold_resist_pct".to_owned(), 33),
        ],
    }
}

/// **Zephyr** -- Ort + Eth -- bows.
///
/// +33% increased attack speed, +25% faster run/walk.
#[must_use]
pub fn zephyr() -> RunewordDef {
    RunewordDef {
        name: "Zephyr".to_owned(),
        runes: vec!["Ort".to_owned(), "Eth".to_owned()],
        base_types: vec!["bow".to_owned()],
        granted_stats: vec![
            ("increased_attack_speed_pct".to_owned(), 33),
            ("faster_run_walk_pct".to_owned(), 25),
        ],
    }
}

/// **Edge** -- Tir + Tal + Amn -- bows.
///
/// +35% increased attack speed, -15% vendor prices.
#[must_use]
pub fn edge() -> RunewordDef {
    RunewordDef {
        name: "Edge".to_owned(),
        runes: vec![
            "Tir".to_owned(),
            "Tal".to_owned(),
            "Amn".to_owned(),
        ],
        base_types: vec!["bow".to_owned()],
        granted_stats: vec![
            ("increased_attack_speed_pct".to_owned(), 35),
            ("vendor_prices_pct".to_owned(), -15),
        ],
    }
}

/// **Strength** -- Amn + Tir -- melee weapons.
///
/// +35% enhanced damage, +25 strength.
#[must_use]
pub fn strength() -> RunewordDef {
    RunewordDef {
        name: "Strength".to_owned(),
        runes: vec!["Amn".to_owned(), "Tir".to_owned()],
        base_types: vec![
            "sword".to_owned(),
            "axe".to_owned(),
            "mace".to_owned(),
            "polearm".to_owned(),
        ],
        granted_stats: vec![
            ("enhanced_damage_pct".to_owned(), 35),
            ("strength".to_owned(), 25),
        ],
    }
}

// ---------------------------------------------------------------------------
// Collection and matching
// ---------------------------------------------------------------------------

/// Return all low-tier runeword definitions.
#[must_use]
pub fn all_runewords() -> Vec<RunewordDef> {
    vec![
        steel(),
        malice(),
        stealth(),
        nadir(),
        leaf(),
        zephyr(),
        edge(),
        strength(),
    ]
}

/// Check whether the given rune sequence and base type form a valid runeword.
///
/// Rune names must match in **exact order** and the base type must appear in
/// the runeword's [`RunewordDef::base_types`] list.
///
/// Returns the first matching [`RunewordDef`], or `None` if no runeword matches.
#[must_use]
pub fn check_runeword(runes: &[String], base_type: &str) -> Option<RunewordDef> {
    all_runewords().into_iter().find(|rw| {
        rw.runes == runes && rw.base_types.iter().any(|bt| bt == base_type)
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Steel: ["Tir","El"] in "sword" should match.
    #[test]
    fn runeword_steel_match() {
        let runes = vec!["Tir".to_owned(), "El".to_owned()];
        let result = check_runeword(&runes, "sword");
        assert!(result.is_some(), "Steel should match Tir+El in sword");
        assert_eq!(result.unwrap().name, "Steel");
    }

    /// Wrong order: ["El","Tir"] in "sword" should NOT match any runeword.
    #[test]
    fn runeword_wrong_order() {
        let runes = vec!["El".to_owned(), "Tir".to_owned()];
        let result = check_runeword(&runes, "sword");
        assert!(result.is_none(), "Reversed rune order must not match");
    }

    /// Wrong base: ["Tir","El"] in "armor" should NOT match Steel.
    #[test]
    fn runeword_wrong_base() {
        let runes = vec!["Tir".to_owned(), "El".to_owned()];
        let result = check_runeword(&runes, "armor");
        assert!(result.is_none(), "Steel does not apply to armor");
    }

    /// All 8 low-tier runewords are returned by all_runewords().
    #[test]
    fn all_runewords_count() {
        let all = all_runewords();
        assert_eq!(all.len(), 8, "Expected 8 low-tier runewords");
    }

    /// Stealth matches in armor base type.
    #[test]
    fn runeword_stealth_match() {
        let runes = vec!["Tal".to_owned(), "Eth".to_owned()];
        let result = check_runeword(&runes, "armor");
        assert!(result.is_some(), "Stealth should match Tal+Eth in armor");
        assert_eq!(result.unwrap().name, "Stealth");
    }

    /// Malice requires 3 runes; partial match must not activate.
    #[test]
    fn runeword_partial_runes_no_match() {
        let runes = vec!["Ith".to_owned(), "El".to_owned()];
        let result = check_runeword(&runes, "sword");
        assert!(result.is_none(), "Partial rune sequence must not match Malice");
    }

    /// Rune name to id round-trip.
    #[test]
    fn rune_name_id_roundtrip() {
        for id in 1..=33u8 {
            let name = rune_id_to_name(id).expect("valid id should have a name");
            let back = rune_name_to_id(name).expect("valid name should have an id");
            assert_eq!(back, id, "round-trip failed for rune id {id}");
        }
    }

    /// Unknown rune name returns None.
    #[test]
    fn rune_name_unknown() {
        assert!(rune_name_to_id("FakeRune").is_none());
    }

    /// Out-of-range rune id returns None.
    #[test]
    fn rune_id_out_of_range() {
        assert!(rune_id_to_name(0).is_none());
        assert!(rune_id_to_name(34).is_none());
    }
}
