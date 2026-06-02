//! Central tuning knobs for the combat & character model. Gathering the scalar
//! constants here (instead of scattering them across `combat.rs` / `stats.rs`)
//! gives one place to read and adjust balance, and one place the balance tests
//! can pin. Pure data — no logic. Formulas that *use* these live in their
//! systems.

// ── Crit & primary-attribute scaling ─────────────────────────────────────────

/// Base critical-hit chance (%) before gear/skills.
pub const CRIT_BASE_PCT: i32 = 5;
/// Critical-hit damage multiplier.
pub const CRIT_MULT: f32 = 1.5;
/// Primary-attribute damage scaling: `1 + primary / DIVISOR`. At 150, 100 points
/// of the class's primary stat is +66% (vs the old +100% at 100), taming all-in
/// stat builds — especially Strength, which also feeds flat melee damage.
pub const PRIMARY_DMG_DIVISOR: f32 = 150.0;

// ── Avoidance & mitigation model (both contest the attacker's Attack Rating) ──
//
// Esquive (dodge):   ESQ / (ESQ + AR)              capped at `DODGE_CAP`
// Mitigation (DEF):  DEF / (DEF + DEF_VS_AR * AR)   capped at `DEF_REDUCTION_CAP`
//
// where AR is the *attacker's* Attack Rating, ESQ the defender's avoidance
// rating (dexterity-fed), and DEF the defender's defence. A more accurate enemy
// (higher AR) is both harder to dodge and punches further through armour, which
// is the whole point of folding AR into both rolls.

/// Defender avoidance points contributed per point of effective Dexterity.
pub const ESQ_PER_DEX: f32 = 0.4;
/// Hard ceiling on dodge probability so nothing is fully un-hittable.
pub const DODGE_CAP: f32 = 0.65;
/// How heavily the attacker's AR offsets the defender's DEF in the reduction
/// curve. Tuned so a fresh hero (DEF 40) vs a level-1 mob (AR 30) sheds ~40%.
pub const DEF_VS_AR: f32 = 2.0;
/// Hard ceiling on physical damage reduction from defence.
pub const DEF_REDUCTION_CAP: f32 = 0.85;

// ── Attack Rating ─────────────────────────────────────────────────────────────

/// Player Attack-Rating baseline (mobs carry their own AR on the `MobDef`).
pub const AR_BASE: f32 = 20.0;
/// Player Attack Rating gained per character level.
pub const AR_PER_LEVEL: f32 = 5.0;
/// Player Attack Rating gained per point of effective Dexterity.
pub const AR_PER_DEX: f32 = 3.0;
/// Fixed AR used by incidental retaliation (frost armour, vengeance): keeps the
/// old `attacker_level = 1` penetration (`DEF/(DEF+50)`) unchanged.
pub const REFLECT_AR: f32 = 25.0;

// ── Block & dual-wield ────────────────────────────────────────────────────────

/// Attack-speed bonus (as a fraction, +30%) granted by dual-wielding two
/// one-handed melee weapons — the trade-off for a shield's block + defence.
pub const DUAL_WIELD_IAS: f32 = 0.30;
/// Base block chance (%) granted just by carrying a shield in the off-hand.
pub const SHIELD_BLOCK_BASE: i32 = 30;
/// Base block chance (%) for dual-wielding two one-handed melee weapons.
pub const DUAL_WIELD_BLOCK_BASE: i32 = 15;
/// Hard ceiling on total block chance (%).
pub const BLOCK_CAP: i32 = 75;
