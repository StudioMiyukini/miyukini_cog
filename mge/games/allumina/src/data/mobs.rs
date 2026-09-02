//! @id     games_allumina_data_mobs
//! @do     definir_mob_def
//! @role   donnee
//! @layer  core
//! @human  games allumina — module « data mobs ».

//! Static monster definitions and their drop profiles.
//!
//! Forty-two normal mobs are grouped into fourteen level tiers (one tier per
//! five-level band, three mob types per tier) and five named bosses, one per
//! boss arena. Stats grow smoothly with level so a tier-1 zone can be cleared
//! by a fresh character while the end-game tiers stay threatening even with
//! late-game gear. Drop pools stay the same across all tiers — the loot
//! difficulty comes from item level, not the mob's family.

use serde::{Deserialize, Serialize};

use mge_items::item::ItemKind;

use crate::components::mob::AiBehavior;
use crate::components::skill::{Element, ElementResists};

/// Static definition of a monster type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobDef {
    pub id: u32,
    pub name: String,
    pub behavior: AiBehavior,
    pub level: u32,
    pub max_hp: f32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub attack_rating: i32,
    pub defense: i32,
    pub xp_reward: u64,
    /// Tiles at which an idle mob aggroes.
    pub aggro: f32,
    /// Tiles from spawn before it disengages.
    pub leash: f32,
    /// Attack reach in tiles.
    pub attack_range: f32,
    /// Seconds between attacks.
    pub attack_interval: f32,
    /// Percent chance (0..=100) to drop an item on death.
    pub drop_chance: u32,
    pub gold_min: u64,
    pub gold_max: u64,
    /// Item kinds this mob can drop.
    pub item_kinds: Vec<ItemKind>,
    pub is_boss: bool,
    /// Elemental resistances (`[fire, frost, lightning, poison]`).
    ///
    /// * `0.0` — neutral (no reduction, no vulnerability).
    /// * `1.0` — full immunity (zero damage taken from this element).
    /// * `< 0` — vulnerability (e.g. `-0.25` amplifies damage by 25%).
    ///
    /// Mobs below level 50 keep `[0.0; 4]`; mid-tier mobs (50-79) get partial
    /// resistances matching their theme; end-game mobs (80+) may get one full
    /// immunity. The combat pipeline reads the same indices the resolver uses
    /// (0 = fire, 1 = frost, 2 = lightning, 3 = poison).
    #[serde(default)]
    pub resistances: [f64; 4],
}

/// Indices into [`MobDef::resistances`] mirroring [`crate::components::skill::Element`].
pub const RES_FIRE: usize = 0;
pub const RES_FROST: usize = 1;
pub const RES_LIGHTNING: usize = 2;
pub const RES_POISON: usize = 3;

impl MobDef {
    /// Resistance for `element` as a clamped fraction in `[-0.5, 1.0]`.
    /// Physical attacks bypass the elemental pipeline entirely and return `0`.
    #[must_use]
    pub fn resistance(&self, element: Element) -> f32 {
        let raw = match element {
            Element::Fire => self.resistances[RES_FIRE],
            Element::Frost => self.resistances[RES_FROST],
            Element::Lightning => self.resistances[RES_LIGHTNING],
            Element::Poison => self.resistances[RES_POISON],
            Element::Physical => return 0.0,
        };
        (raw as f32).clamp(-0.5, 1.0)
    }

    /// `true` if this def fully nullifies `element` (a 100% resistance the
    /// end-game mobs carry).
    #[must_use]
    pub fn is_immune_to(&self, element: Element) -> bool {
        self.resistance(element) >= 1.0 - f32::EPSILON
    }

    /// Build the [`ElementResists`] component a fresh spawn should carry. The
    /// mob spawner copies this into the world so the combat pipeline finds it.
    #[must_use]
    pub fn elem_resists(&self) -> ElementResists {
        ElementResists {
            fire: self.resistance(Element::Fire),
            frost: self.resistance(Element::Frost),
            lightning: self.resistance(Element::Lightning),
            poison: self.resistance(Element::Poison),
        }
    }
}

fn common_kinds() -> Vec<ItemKind> {
    vec![
        ItemKind::Sword,
        ItemKind::Bow,
        ItemKind::Wand,
        ItemKind::Helm,
        ItemKind::BodyArmor,
        ItemKind::Boots,
        ItemKind::Gloves,
        ItemKind::Belt,
        ItemKind::Ring,
        ItemKind::Amulet,
        ItemKind::Shield,
    ]
}

/// Build a normal mob definition scaled by `level`.
#[allow(clippy::too_many_arguments)]
fn make_mob(id: u32, name: &str, behavior: AiBehavior, level: u32, dps_mult: f32, hp_mult: f32) -> MobDef {
    let lvl = level as f32;
    let max_hp = (24.0 + lvl * 8.0) * hp_mult;
    let dmg_base = (1.5 + lvl * 0.9) * dps_mult;
    let damage_min = dmg_base.round().max(1.0) as i32;
    let damage_max = (dmg_base * 2.1).round().max((damage_min + 1) as f32) as i32;
    let attack_rating = (25.0 + lvl * 5.0) as i32;
    let defense = (5.0 + lvl * 2.0) as i32;
    // Base XP scales loosely with mob level so end-game mobs still feel like
    // they're worth pulling. Élites are multiplied at award-time (death.rs).
    let xp_reward = (u64::from(level) / 3).max(1);
    // Intervals are the seconds between an enemy's attacks. They were halved in
    // rate (i.e. doubled in duration) to slow enemy attack speed by 50%.
    let (attack_range, attack_interval, aggro, leash) = match behavior {
        AiBehavior::Melee => (1.5, 2.2, 6.0, 12.0),
        AiBehavior::Ranged => (6.5, 2.8, 8.0, 14.0),
        AiBehavior::Caster => (7.5, 3.6, 9.0, 14.0),
    };
    // Per-mob chance to drop an item. Kept low so loot feels meaningful (big
    // packs still yield a few drops); gems roll on a separate track in
    // `loot_system`, so this doesn't gate them.
    let drop_chance = (14 + level.min(18)).min(32);
    let gold_min = 1 + u64::from(level) / 2;
    let gold_max = 20 + u64::from(level) * 4;
    MobDef {
        id,
        name: name.into(),
        behavior,
        level,
        max_hp,
        damage_min,
        damage_max,
        attack_rating,
        defense,
        xp_reward,
        aggro,
        leash,
        attack_range,
        attack_interval,
        drop_chance,
        gold_min,
        gold_max,
        item_kinds: common_kinds(),
        is_boss: false,
        resistances: [0.0; 4],
    }
}

/// Build a boss definition. Bosses share melee aggro but get a wide leash so
/// they keep chasing in the arena, and a much higher loot tier.
fn make_boss(id: u32, name: &str, level: u32, hp_mult: f32, dps_mult: f32) -> MobDef {
    let lvl = level as f32;
    let max_hp = (160.0 + lvl * 22.0) * hp_mult;
    let dmg_base = (4.0 + lvl * 1.3) * dps_mult;
    let damage_min = dmg_base.round().max(2.0) as i32;
    let damage_max = (dmg_base * 2.0).round().max((damage_min + 2) as f32) as i32;
    MobDef {
        id,
        name: name.into(),
        behavior: AiBehavior::Melee,
        level,
        max_hp,
        damage_min,
        damage_max,
        attack_rating: (120 + level * 6) as i32,
        defense: (40 + level * 3) as i32,
        // Bosses pay 10× the normal mob XP for their level.
        xp_reward: (u64::from(level) / 3).max(1) * 10,
        aggro: 14.0,
        leash: 40.0,
        attack_range: 2.0,
        // Halved attack rate (doubled interval) like normal mobs: 1.2 → 2.4.
        attack_interval: 2.4,
        drop_chance: 100,
        gold_min: 300 + u64::from(level) * 12,
        gold_max: 900 + u64::from(level) * 40,
        item_kinds: common_kinds(),
        is_boss: true,
        resistances: [0.0; 4],
    }
}

/// Attach elemental resistances to an existing [`MobDef`]. Wrapped around the
/// `make_mob` / `make_boss` calls so the table stays a flat list of rows.
fn with_resists(mut def: MobDef, r: [f64; 4]) -> MobDef {
    def.resistances = r;
    def
}

/// All mob definitions for the twenty-nine-zone campaign (levels 1–100).
pub fn mob_table() -> Vec<MobDef> {
    use AiBehavior::*;
    vec![
        // Tier 1 — Plaines verdoyantes (lvl 1-5)
        make_mob(1, "Loup affamé", Melee, 1, 0.95, 1.0),
        make_mob(2, "Bandit de grand chemin", Ranged, 2, 1.0, 0.9),
        make_mob(3, "Slime gluant", Melee, 3, 0.9, 1.15),
        // Tier 2 — Collines rocheuses (lvl 6-10)
        make_mob(4, "Gobelin pillard", Melee, 7, 1.0, 1.0),
        make_mob(5, "Squelette archer", Ranged, 8, 1.0, 0.95),
        make_mob(6, "Araignée venimeuse", Melee, 9, 1.0, 1.0),
        // Tier 3 — Forêt sombre / Antre du Loup-Garou (lvl 11-15)
        make_mob(7, "Zombie putride", Melee, 12, 1.0, 1.1),
        make_mob(8, "Golem de pierre", Melee, 13, 1.1, 1.3),
        make_mob(9, "Loup-garou", Melee, 14, 1.1, 1.0),
        // Tier 4 — Marais putrides (lvl 16-20)
        make_mob(10, "Grenouille géante", Melee, 17, 1.0, 1.15),
        make_mob(11, "Crocodile des marais", Melee, 18, 1.15, 1.2),
        make_mob(12, "Spectre des marais", Caster, 19, 1.1, 0.9),
        // Tier 5 — Ruines anciennes (lvl 21-25)
        make_mob(13, "Squelette guerrier", Melee, 22, 1.1, 1.05),
        make_mob(14, "Momie", Melee, 23, 1.0, 1.2),
        make_mob(15, "Gargoyle", Ranged, 24, 1.1, 1.0),
        // Tier 6 — Crypte oubliée / Salle du Liche (lvl 26-30)
        make_mob(16, "Nécromancien", Caster, 27, 1.15, 1.0),
        make_mob(17, "Revenant", Melee, 28, 1.1, 1.1),
        make_mob(18, "Vampire", Melee, 29, 1.2, 1.0),
        // Tier 7 — Désert brûlant (lvl 31-35)
        make_mob(19, "Scorpion géant", Melee, 32, 1.1, 1.15),
        make_mob(20, "Djinn", Caster, 33, 1.2, 0.95),
        make_mob(21, "Ver des sables", Melee, 34, 1.15, 1.25),
        // Tier 8 — Canyon des vents (lvl 36-40)
        make_mob(22, "Harpie", Ranged, 37, 1.15, 1.0),
        make_mob(23, "Élémentaire d'air", Caster, 38, 1.2, 1.0),
        make_mob(24, "Griffon", Melee, 39, 1.2, 1.1),
        // Tier 9 — Forteresse déchue / Trône du Tyran (lvl 41-45)
        make_mob(25, "Chevalier noir", Melee, 42, 1.2, 1.2),
        make_mob(26, "Orc d'élite", Melee, 43, 1.25, 1.1),
        make_mob(27, "Démon mineur", Melee, 44, 1.25, 1.05),
        // Tier 10 — Pics gelés (lvl 46-50). Yéti and ice elementals carry their
        // signature frost/fire profile from this tier onwards.
        make_mob(28, "Yéti", Melee, 47, 1.2, 1.3),
        make_mob(29, "Élémentaire de glace", Caster, 48, 1.3, 1.0),
        make_mob(30, "Wyverne", Melee, 49, 1.3, 1.15),
        // Tier 11 — Cavernes de cristal (lvl 51-55). Naga is the venom anchor.
        make_mob(31, "Golem de cristal", Melee, 52, 1.25, 1.4),
        make_mob(32, "Basilic", Ranged, 53, 1.3, 1.05),
        with_resists(make_mob(33, "Naga", Melee, 54, 1.3, 1.15), [0.0, 0.0, 0.0, 0.50]),
        // Tier 12 — Abîme volcanique / Forge infernale (lvl 56-60).
        with_resists(make_mob(34, "Élémentaire de feu", Caster, 57, 1.35, 1.0), [0.75, -0.25, 0.0, 0.0]),
        with_resists(make_mob(35, "Salamandre infernale", Melee, 58, 1.3, 1.2), [0.50, -0.25, 0.0, 0.0]),
        with_resists(make_mob(36, "Dragon mineur", Melee, 59, 1.4, 1.25), [0.50, 0.0, 0.0, 0.25]),
        // Tier 13 — Tour céleste (lvl 61-65).
        make_mob(37, "Ange déchu", Melee, 62, 1.4, 1.2),
        make_mob(38, "Chimère", Melee, 63, 1.4, 1.25),
        with_resists(make_mob(39, "Démon majeur", Caster, 64, 1.45, 1.05), [0.50, 0.0, 0.0, 0.0]),
        // Tier 14 — Sanctuaire corrompu / Trône de l'Archange déchu (lvl 66-70).
        with_resists(make_mob(40, "Archidémon", Melee, 67, 1.5, 1.3), [0.50, 0.0, 0.0, 0.0]),
        make_mob(41, "Séraphin corrompu", Caster, 68, 1.5, 1.15),
        make_mob(42, "Ombre ancienne", Melee, 69, 1.5, 1.2),

        // Tier 15 — Plaines de cendres (lvl 71-75). Ash/lava theme: high fire,
        // ice vulnerability. Still below the immunity tier (<80).
        with_resists(make_mob(43, "Titan de cendre", Melee, 72, 1.55, 1.4), [0.75, -0.25, 0.0, 0.0]),
        with_resists(make_mob(44, "Phoenix mort-vivant", Caster, 73, 1.55, 1.1), [0.75, -0.25, 0.0, 0.0]),
        with_resists(make_mob(45, "Golem de lave", Melee, 74, 1.55, 1.5), [0.75, -0.25, 0.0, 0.0]),
        // Tier 16 — Citadelle des ombres / Abîme des âmes (lvl 76-80). Spec
        // rule: immunity reserved for lvl 80+, so the lvl 77-79 mobs sit just
        // below 100% with thematic high resistance.
        with_resists(make_mob(46, "Spectre ancien", Caster, 77, 1.6, 1.0), [0.0, 0.0, 0.75, 0.0]),
        with_resists(make_mob(47, "Liche mineure", Caster, 78, 1.65, 1.05), [-0.25, 0.50, 0.0, 0.0]),
        with_resists(make_mob(48, "Ombre vivante", Melee, 79, 1.6, 1.2), [0.0, 0.0, 0.50, 0.50]),
        // Tier 17 — Terre désolée (lvl 81-85). Pure-physical bruisers; the
        // dragon line keeps its fire immunity.
        with_resists(make_mob(49, "Béhémoth", Melee, 82, 1.7, 1.5), [0.50, 0.0, 0.0, 0.50]),
        with_resists(make_mob(50, "Dragon adulte", Melee, 83, 1.75, 1.4), [1.0, 0.0, 0.50, 0.25]),
        with_resists(make_mob(51, "Colosse de pierre", Melee, 84, 1.7, 1.6), [0.50, 0.50, -0.25, 0.0]),
        // Tier 18 — Nexus dimensionnel / Palais du Néant (lvl 86-90).
        with_resists(make_mob(52, "Seigneur démon", Melee, 87, 1.85, 1.35), [1.0, 0.0, 0.0, 0.0]),
        with_resists(make_mob(53, "Entité du vide", Caster, 88, 1.85, 1.1), [0.50, 0.50, 0.50, 0.50]),
        with_resists(make_mob(54, "Avatar élémentaire", Caster, 89, 1.9, 1.15), [0.50, 0.50, 0.50, 0.0]),
        // Tier 19 — Faille temporelle (lvl 91-95).
        with_resists(make_mob(55, "Chronophage", Caster, 92, 1.95, 1.15), [0.50, 0.50, 1.0, 0.0]),
        with_resists(make_mob(56, "Gardien dimensionnel", Melee, 93, 2.0, 1.5), [0.50, 0.50, 0.50, 0.50]),
        with_resists(make_mob(57, "Dévoreur d'âmes", Melee, 94, 2.0, 1.4), [0.0, 0.0, 0.0, 1.0]),
        // Tier 20 — Cœur du Chaos / Trône de l'Éternel (lvl 96-100). Mid-game
        // bosses' adds tier: heavy resistances, one immunity each.
        with_resists(make_mob(58, "Archon déchu", Caster, 97, 2.1, 1.2), [1.0, 0.50, 0.50, 0.0]),
        with_resists(make_mob(59, "Titan primordial", Melee, 98, 2.15, 1.6), [0.75, 0.75, 0.0, 0.50]),
        with_resists(make_mob(60, "Incarnation du chaos", Caster, 99, 2.2, 1.3), [0.75, 0.75, 0.75, 0.75]),

        // Bosses — one per boss arena, ids match data::maps::boss_def_id.
        make_boss(100, "Alpha du Loup-Garou", 15, 1.2, 1.1),
        make_boss(101, "Liche Sombre", 30, 1.35, 1.25),
        make_boss(102, "Tyran de fer", 45, 1.5, 1.35),
        with_resists(make_boss(103, "Seigneur de la Forge", 60, 1.7, 1.5), [1.0, -0.25, 0.50, 0.0]),
        with_resists(make_boss(104, "Archange déchu", 70, 2.0, 1.75), [0.50, 0.50, 0.50, 0.50]),
        // ── Tier-16+ bosses: shadow / void / final. Each gets at least one
        //    immunity matching its theme.
        // Roi des Ombres — drains souls in AoE, two clones each phase shift.
        with_resists(make_boss(105, "Roi des Ombres", 80, 2.3, 1.85), [0.50, 0.50, 1.0, 0.75]),
        // Seigneur du Néant — teleports + vortex pull; mostly resistant to all.
        with_resists(make_boss(106, "Seigneur du Néant", 90, 2.6, 2.0), [0.75, 0.75, 0.75, 1.0]),
        // L'Éternel — final boss, five phases. Spec rule "max 1 immunity per
        // mob" applies even to the final boss: fire is locked at 100%, the
        // rest are high resistances (and a token frost weakness the player
        // can exploit).
        with_resists(make_boss(107, "L'Éternel", 100, 3.2, 2.5), [1.0, -0.10, 0.75, 0.75]),
    ]
}
