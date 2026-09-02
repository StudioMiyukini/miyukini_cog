//! @id     games_allumina_components_item
//! @do     is_ranged
//! @role   ui
//! @layer  core
//! @human  games allumina — module « components item ».

//! Item components. Built on `mge-items` (base defs, rarity, affixes) plus a
//! thin `ItemInstance` that carries the rolled affixes and a display name.

use serde::{Deserialize, Serialize};

use mge_gameplay::stats::{Modifier, StatKind, StatModifier};
use mge_items::affixes::{AffixMod, AppliedAffix, ItemAffixes};
use mge_items::item::{EquipSlot, Item, ItemKind};
use mge_items::rarity::Rarity;

/// Re-exports aligned with the requested component names.
pub use mge_items::affixes::{AffixPool, ItemAffixes as Affixes};
pub use mge_items::rarity::Rarity as ItemRarity;

/// Whether `kind` is a ranged weapon (fires a projectile at distance).
/// Bows/crossbows are physical-ranged; staves/wands/scepters are magic-ranged.
#[must_use]
pub fn is_ranged(kind: ItemKind) -> bool {
    matches!(
        kind,
        ItemKind::Bow | ItemKind::Crossbow | ItemKind::Staff | ItemKind::Wand | ItemKind::Scepter
    )
}

/// Whether `kind` is a magic-flavoured ranged weapon (its projectile bypasses
/// physical defence in [`crate::systems::projectile`]).
#[must_use]
pub fn is_magic_ranged(kind: ItemKind) -> bool {
    matches!(kind, ItemKind::Staff | ItemKind::Wand | ItemKind::Scepter)
}

/// Whether `kind` is a one-handed melee weapon — the weapons light enough to
/// hold a second of in the off-hand for dual-wield parrying (see
/// [`crate::systems::stats`] block computation).
#[must_use]
pub fn is_one_handed_melee(kind: ItemKind) -> bool {
    matches!(kind, ItemKind::Sword | ItemKind::Axe | ItemKind::Mace | ItemKind::Dagger)
}

/// Auto-attack reach (tiles) for an equipped weapon `kind`. Melee weapons share
/// the 1.5-tile reach, bows/crossbows reach 8 tiles, magic foci 6 tiles.
#[must_use]
pub fn attack_range(kind: ItemKind) -> f32 {
    match kind {
        ItemKind::Sword | ItemKind::Axe | ItemKind::Mace | ItemKind::Dagger => 1.5,
        ItemKind::Bow | ItemKind::Crossbow => 8.0,
        ItemKind::Staff | ItemKind::Wand | ItemKind::Scepter => 6.0,
        _ => 1.5,
    }
}

/// The seven gem families. Each grants a different effect depending on whether
/// it sits in a weapon, body/helm armour, or a shield (see [`gem_mods`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GemKind {
    Diamant,
    Rubis,
    Emeraude,
    Saphir,
    Topaze,
    Crane,
    Zirconia,
}

impl GemKind {
    /// Every gem family, for random selection at drop time.
    pub const ALL: [GemKind; 7] = [
        GemKind::Diamant,
        GemKind::Rubis,
        GemKind::Emeraude,
        GemKind::Saphir,
        GemKind::Topaze,
        GemKind::Crane,
        GemKind::Zirconia,
    ];

    /// Base French name (without the quality word).
    #[must_use]
    pub fn base_name(self) -> &'static str {
        match self {
            GemKind::Diamant => "Diamant",
            GemKind::Rubis => "Rubis",
            GemKind::Emeraude => "Émeraude",
            GemKind::Saphir => "Saphir",
            GemKind::Topaze => "Topaze",
            GemKind::Crane => "Crâne",
            GemKind::Zirconia => "Zirconia",
        }
    }
}

/// Which contextual effect table a host item draws from for its gems.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GemHost {
    Weapon,
    Armor,
    Shield,
}

/// Map a host item kind to the gem-effect context it provides. Off-hand shields
/// use the shield table; any weapon (1H melee or ranged) uses the weapon table;
/// everything else (body, helm, belt, jewellery…) uses the armour table.
#[must_use]
pub fn gem_host(kind: ItemKind) -> GemHost {
    match kind {
        ItemKind::Shield => GemHost::Shield,
        k if is_one_handed_melee(k) || is_ranged(k) => GemHost::Weapon,
        _ => GemHost::Armor,
    }
}

/// Quality word for a gem tier (0 = chipped … 4 = perfect). Tier 2 is the plain
/// gem with no qualifier.
#[must_use]
pub fn quality_word(tier: u8) -> &'static str {
    match tier {
        0 => "éclat",
        1 => "taillé",
        3 => "sans défaut",
        4 => "parfait",
        _ => "",
    }
}

/// The stat modifiers a gem of `kind`/`tier` grants in `host` (tier 0…4).
/// Effects that aren't plain stats (Zirconia's sunder / requirement cut) are
/// empty here and resolved via [`SocketedGem::sunder_pct`] /
/// [`SocketedGem::req_reduction_pct`].
#[must_use]
fn gem_mods(kind: GemKind, host: GemHost, tier: u8) -> Vec<AffixMod> {
    let t = tier.min(4) as usize;
    let flat = |stat, arr: [i32; 5]| vec![AffixMod { stat, modifier: Modifier::Flat(arr[t]) }];
    let all_resist = |arr: [i32; 5]| {
        [
            StatKind::FireResist,
            StatKind::ColdResist,
            StatKind::LightningResist,
            StatKind::PoisonResist,
        ]
        .into_iter()
        .map(|stat| AffixMod { stat, modifier: Modifier::Flat(arr[t]) })
        .collect()
    };
    match (kind, host) {
        (GemKind::Diamant, GemHost::Weapon) => flat(StatKind::AttackRating, [20, 35, 50, 75, 100]),
        (GemKind::Diamant, GemHost::Armor) => all_resist([3, 4, 6, 8, 11]),
        (GemKind::Diamant, GemHost::Shield) => all_resist([4, 6, 9, 12, 16]),

        (GemKind::Rubis, GemHost::Weapon) => flat(StatKind::DamageMax, [3, 5, 8, 12, 18]),
        (GemKind::Rubis, GemHost::Armor) => flat(StatKind::Life, [10, 17, 24, 31, 40]),
        (GemKind::Rubis, GemHost::Shield) => flat(StatKind::FireResist, [12, 16, 22, 28, 40]),

        (GemKind::Saphir, GemHost::Weapon) => flat(StatKind::DamageMax, [2, 4, 6, 9, 14]),
        (GemKind::Saphir, GemHost::Armor) => flat(StatKind::Mana, [10, 17, 24, 31, 40]),
        (GemKind::Saphir, GemHost::Shield) => flat(StatKind::ColdResist, [12, 16, 22, 28, 40]),

        (GemKind::Emeraude, GemHost::Weapon) => flat(StatKind::DamageMax, [3, 5, 7, 10, 15]),
        (GemKind::Emeraude, GemHost::Armor) => flat(StatKind::Dexterity, [3, 4, 6, 8, 10]),
        (GemKind::Emeraude, GemHost::Shield) => flat(StatKind::PoisonResist, [12, 16, 22, 28, 40]),

        (GemKind::Topaze, GemHost::Weapon) => flat(StatKind::DamageMax, [4, 9, 15, 22, 30]),
        (GemKind::Topaze, GemHost::Armor) => flat(StatKind::MagicFind, [9, 13, 16, 20, 24]),
        (GemKind::Topaze, GemHost::Shield) => flat(StatKind::LightningResist, [12, 16, 22, 28, 40]),

        (GemKind::Crane, GemHost::Weapon) => {
            vec![
                AffixMod { stat: StatKind::LifeOnHit, modifier: Modifier::Flat([2, 3, 4, 6, 8][t]) },
                AffixMod { stat: StatKind::ManaOnKill, modifier: Modifier::Flat([1, 2, 2, 3, 4][t]) },
            ]
        }
        (GemKind::Crane, GemHost::Armor) => {
            vec![
                AffixMod {
                    stat: StatKind::Stamina,
                    modifier: Modifier::Flat([10, 15, 20, 25, 30][t]),
                },
                AffixMod { stat: StatKind::Life, modifier: Modifier::Flat([3, 5, 7, 9, 12][t]) },
            ]
        }
        (GemKind::Crane, GemHost::Shield) => flat(StatKind::LifeOnHit, [1, 2, 3, 4, 5]),

        (GemKind::Zirconia, GemHost::Shield) => flat(StatKind::BlockChance, [1, 3, 5, 7, 10]),
        (GemKind::Zirconia, _) => Vec::new(),
    }
}

/// A gem identity — its family and quality tier (0 = éclat … 4 = parfait).
/// Stored both on a loose gem item ([`ItemInstance::gem`]) and inside each
/// filled socket of a host item, so the whole gem can be removed in one piece.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SocketedGem {
    pub kind: GemKind,
    pub tier: u8,
}

impl SocketedGem {
    /// Display name like "Diamant parfait" or plain "Rubis" at the neutral tier.
    #[must_use]
    pub fn display_name(&self) -> String {
        let q = quality_word(self.tier);
        if q.is_empty() {
            self.kind.base_name().to_string()
        } else {
            format!("{} {q}", self.kind.base_name())
        }
    }

    /// Stat modifiers this gem grants while sitting in `host`.
    #[must_use]
    pub fn stat_mods(&self, host: GemHost) -> Vec<AffixMod> {
        gem_mods(self.kind, host, self.tier)
    }

    /// Weapon-only: fraction of the target's Defense this gem strips (Zirconia).
    #[must_use]
    pub fn sunder_pct(&self, host: GemHost) -> f32 {
        match (self.kind, host) {
            (GemKind::Zirconia, GemHost::Weapon) => {
                [0.01, 0.02, 0.03, 0.04, 0.05][self.tier.min(4) as usize]
            }
            _ => 0.0,
        }
    }

    /// Armour/helm-only: percent cut to the host item's own equip requirements
    /// (Zirconia).
    #[must_use]
    pub fn req_reduction_pct(&self, host: GemHost) -> u32 {
        match (self.kind, host) {
            (GemKind::Zirconia, GemHost::Armor) => [10, 20, 30, 40, 50][self.tier.min(4) as usize],
            _ => 0,
        }
    }

    /// One-line human description of the gem's effect in `host`.
    #[must_use]
    pub fn effect_desc(&self, host: GemHost) -> String {
        if self.kind == GemKind::Zirconia {
            match host {
                GemHost::Weapon => {
                    return format!("-{}% Défense de l'ennemi", (self.sunder_pct(host) * 100.0) as i32);
                }
                GemHost::Armor => {
                    return format!("-{}% Conditions requises", self.req_reduction_pct(host));
                }
                GemHost::Shield => {}
            }
        }
        self.stat_mods(host)
            .iter()
            .map(|m| format_mod(m.stat, m.modifier))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// A single item instance, ready to drop, carry, or equip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemInstance {
    pub item: Item,
    /// Random prefixes/suffixes (magic/rare).
    pub affixes: ItemAffixes,
    /// Fixed mods for unique/set items.
    pub extra_mods: Vec<AffixMod>,
    /// Full display name, including affix words or the unique title.
    pub name: String,
    /// Display words for the rolled prefixes, in roll order (e.g. "Jagged").
    pub prefix_words: Vec<String>,
    /// Display words for the rolled suffixes, in roll order (e.g. "of Fire").
    pub suffix_words: Vec<String>,
    /// `Some(unique_id)` for unique-rarity items. Used by save/load to look the
    /// def back up so fixed stats are reconstituted from the registry instead
    /// of being trusted from disk.
    #[serde(default)]
    pub unique_id: Option<u32>,
    /// Flavour text for uniques (italic grey line at tooltip bottom).
    #[serde(default)]
    pub flavour: Option<String>,
    /// Number of socket slots rolled on this item at drop time (0 if none).
    #[serde(default)]
    pub sockets: u8,
    /// Gems inserted into the sockets (one entry per filled socket;
    /// `socketed.len() <= sockets`). Folded into [`stat_modifiers`].
    #[serde(default)]
    pub socketed: Vec<SocketedGem>,
    /// For a *loose* gem item (kind Gem/Rune/Jewel), its family + quality. This
    /// identity is what gets copied into a host's socket. `None` for gear.
    #[serde(default)]
    pub gem: Option<SocketedGem>,
}

impl ItemInstance {
    pub fn new(item: Item, affixes: ItemAffixes, extra_mods: Vec<AffixMod>, name: String) -> Self {
        Self {
            item,
            affixes,
            extra_mods,
            name,
            prefix_words: Vec::new(),
            suffix_words: Vec::new(),
            unique_id: None,
            flavour: None,
            sockets: 0,
            socketed: Vec::new(),
            gem: None,
        }
    }

    pub fn rarity(&self) -> Rarity {
        self.item.rarity
    }

    /// Whether this item is a socketable gem (the thing you insert, not the
    /// gear you insert it into).
    #[must_use]
    pub fn is_gem(&self) -> bool {
        matches!(self.item.base.kind, ItemKind::Gem | ItemKind::Rune | ItemKind::Jewel)
    }

    /// Empty socket slots still available for a gem.
    #[must_use]
    pub fn open_sockets(&self) -> u8 {
        self.sockets.saturating_sub(self.socketed.len() as u8)
    }

    /// Insert `gem` (an item of gem kind) into one of this item's open sockets.
    pub fn socket(&mut self, gem: &ItemInstance) {
        if let Some(spec) = gem.gem {
            self.socketed.push(spec);
        }
    }

    /// Pop the most-recently-inserted gem out of its socket, if any.
    pub fn unsocket(&mut self) -> Option<SocketedGem> {
        self.socketed.pop()
    }

    /// Total fraction of a target's Defense this weapon's gems strip (Zirconia),
    /// capped to keep mitigation meaningful.
    #[must_use]
    pub fn sunder_pct(&self) -> f32 {
        let host = gem_host(self.item.base.kind);
        self.socketed.iter().map(|g| g.sunder_pct(host)).sum::<f32>().min(0.75)
    }

    /// Total percent cut to this item's own equip requirements from its gems.
    #[must_use]
    pub fn req_reduction_pct(&self) -> u32 {
        let host = gem_host(self.item.base.kind);
        self.socketed.iter().map(|g| g.req_reduction_pct(host)).sum::<u32>().min(90)
    }

    pub fn ilvl(&self) -> u32 {
        self.item.ilvl
    }

    pub fn slot(&self) -> Option<EquipSlot> {
        self.item.base.kind.equip_slot()
    }

    /// Sum of this item's flat `+DamageMin`/`+DamageMax` across affixes, fixed
    /// mods and socketed gems, without building the full [`stat_modifiers`]
    /// list. Called per swing in the dual-wield hot path, so it avoids the
    /// `Vec<StatModifier>` allocation that `stat_modifiers()` incurs.
    #[must_use]
    pub fn flat_damage_bonus(&self) -> (i32, i32) {
        let (mut mn, mut mx) = (0, 0);
        let mut add = |stat: StatKind, modifier: Modifier| {
            if let Modifier::Flat(v) = modifier {
                match stat {
                    StatKind::DamageMin => mn += v,
                    StatKind::DamageMax => mx += v,
                    _ => {}
                }
            }
        };
        for affix in self.affixes.prefixes.iter().chain(self.affixes.suffixes.iter()) {
            for m in &affix.mods {
                add(m.stat, m.modifier);
            }
        }
        for m in &self.extra_mods {
            add(m.stat, m.modifier);
        }
        let host = gem_host(self.item.base.kind);
        for gem in &self.socketed {
            for m in gem.stat_mods(host) {
                add(m.stat, m.modifier);
            }
        }
        (mn, mx)
    }

    /// All stat modifiers this item grants while equipped, keyed by the item uid.
    pub fn stat_modifiers(&self) -> Vec<StatModifier> {
        let mut out = Vec::new();
        let source_id = self.item.uid;
        let mut push = |m: &AffixMod| {
            out.push(StatModifier { source_id, stat: m.stat, modifier: m.modifier });
        };
        for affix in &self.affixes.prefixes {
            affix.mods.iter().for_each(&mut push);
        }
        for affix in &self.affixes.suffixes {
            affix.mods.iter().for_each(&mut push);
        }
        self.extra_mods.iter().for_each(&mut push);
        // Inserted gems contribute context-dependent mods while the gear is worn
        // (weapon / armour+helm / shield tables).
        let host = gem_host(self.item.base.kind);
        for gem in &self.socketed {
            gem.stat_mods(host).iter().for_each(&mut push);
        }
        out
    }

    /// Lines for a Diablo-style tooltip.
    pub fn tooltip_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("{} [{}]", self.name, self.rarity().color_name()),
            format!("{:?}  (ilvl {})", self.item.base.kind, self.ilvl()),
        ];
        for word in &self.prefix_words {
            lines.push(format!("  {word}"));
        }
        // A loose gem lists what it grants in each kind of host item.
        if let Some(gem) = self.gem {
            lines.push(format!("  Arme: {}", gem.effect_desc(GemHost::Weapon)));
            lines.push(format!("  Armure/Casque: {}", gem.effect_desc(GemHost::Armor)));
            lines.push(format!("  Bouclier: {}", gem.effect_desc(GemHost::Shield)));
            return lines;
        }
        let req = &self.item.base.requirements;
        if req.min_level > 0
            || req.min_strength > 0
            || req.min_dexterity > 0
            || req.min_intelligence > 0
        {
            lines.push(format!(
                "Requires: lvl {}  str {}  dex {}  int {}",
                req.min_level, req.min_strength, req.min_dexterity, req.min_intelligence
            ));
        }
        let mods: Vec<&AppliedAffix> =
            self.affixes.prefixes.iter().chain(self.affixes.suffixes.iter()).collect();
        for affix in mods {
            for m in &affix.mods {
                lines.push(format!("  {}", format_mod(m.stat, m.modifier)));
            }
        }
        for m in &self.extra_mods {
            lines.push(format!("  {}", format_mod(m.stat, m.modifier)));
        }
        for word in &self.suffix_words {
            lines.push(format!("  {word}"));
        }
        lines
    }
}

/// An item lying on the ground (paired with a `Position2D` on the same entity).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundItem {
    pub instance: ItemInstance,
    /// Frames remaining before the drop despawns.
    pub life: u32,
}

/// Human-readable label for a stat.
pub fn stat_label(stat: StatKind) -> &'static str {
    match stat {
        StatKind::Strength => "Strength",
        StatKind::Dexterity => "Dexterity",
        StatKind::Vitality => "Vitality",
        StatKind::Energy => "Energy",
        StatKind::Life => "Life",
        StatKind::Mana => "Mana",
        StatKind::Stamina => "Stamina",
        StatKind::AttackRating => "Attack Rating",
        StatKind::Defense => "Defense",
        StatKind::BlockChance => "Block",
        StatKind::DamageMin => "Min Damage",
        StatKind::DamageMax => "Max Damage",
        StatKind::CritChance => "Crit Chance",
        StatKind::AttackSpeed => "Attack Speed",
        StatKind::CastSpeed => "Cast Speed",
        StatKind::MovementSpeed => "Movement Speed",
        StatKind::FireResist => "Fire Resist",
        StatKind::ColdResist => "Cold Resist",
        StatKind::LightningResist => "Lightning Resist",
        StatKind::PoisonResist => "Poison Resist",
        StatKind::MagicFind => "Magic Find",
        StatKind::GoldFind => "Gold Find",
        StatKind::LifeOnHit => "Life on Hit",
        StatKind::ManaOnKill => "Mana on Kill",
    }
}

fn format_mod(stat: StatKind, modifier: Modifier) -> String {
    match modifier {
        Modifier::Flat(v) => format!("{:+} {}", v, stat_label(stat)),
        Modifier::Percentage(v) => format!("{:+}% {}", v, stat_label(stat)),
    }
}
