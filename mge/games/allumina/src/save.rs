//! Persistent character saves.
//!
//! JSON files written to a `saves/` directory next to the binary. Each save is
//! tagged with a unique `player_id` (UUID-shaped) and a checksum of the
//! serialized payload + a fixed salt so a future authoritative server can
//! reject hand-edited files. The on-disk schema (`SavedItem`/`SavedAffix`) is a
//! deliberate, slightly lossy view: it's stable across small `mge-items`
//! refactors and easy to diff in plain text.

use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use mge_gameplay::skills::LearnedSkill;
use mge_gameplay::stats::{Modifier, StatKind};
use mge_items::affixes::{AffixKind, AffixMod, AppliedAffix, ItemAffixes};
use mge_items::item::{EquipSlot, Item, ItemKind};
use mge_items::rarity::Rarity;

use crate::components::item::ItemInstance;
use crate::components::player::ClassType;
use crate::components::world::ZoneId;
use crate::data::items::base_item;
use crate::ecs::World;

/// Current save format version.
pub const SAVE_VERSION: u32 = 1;
/// Fixed salt mixed into the checksum.
const CHECKSUM_SALT: &str = "allumina-v1-anti-tamper-salt";
/// How many slots a character may keep at once.
pub const MAX_SAVE_SLOTS: u32 = 10;
/// Seconds between idle auto-saves while the player is in Playing state.
pub const AUTOSAVE_INTERVAL_SECS: f32 = 60.0;
/// How long the "Sauvegarde..." disc indicator stays on screen.
pub const SAVE_INDICATOR_SECS: f32 = 2.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedStats {
    pub strength: i32,
    pub dexterity: i32,
    pub vitality: i32,
    pub energy: i32,
    /// Unspent stat points.
    pub unspent_points: u32,
}

/// One stat modifier inside a saved affix. `pct` records whether `value` is a
/// percentage (vs flat) modifier — without it, percentage affixes round-trip
/// back as flat ones, silently corrupting every magic/rare item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedMod {
    pub stat: String,
    pub value: i32,
    #[serde(default)]
    pub pct: bool,
}

/// A single rolled affix, preserved as one group so multi-stat affixes (e.g.
/// "of Warding" = four resists) keep their structure and `def_id` across a
/// save/load instead of being flattened into one synthetic bucket.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedAffix {
    /// Affix definition id (0 for legacy/synthetic affixes).
    #[serde(default)]
    pub def_id: u32,
    /// "prefix" or "suffix".
    pub slot: String,
    /// Every stat part this affix grants.
    #[serde(default)]
    pub mods: Vec<SavedMod>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SavedItem {
    pub base_kind: String,
    pub rarity: String,
    pub name: String,
    pub affixes: Vec<SavedAffix>,
    pub level_req: u32,
    /// `Some(unique_id)` for unique-rarity items. The fixed mods + flavour are
    /// rebuilt from the registry at load time so the on-disk file stays small
    /// and edits to the unique table take effect on the next session.
    #[serde(default)]
    pub unique_id: Option<u32>,
    /// Socket count rolled on the item (0 if none).
    #[serde(default)]
    pub sockets: u8,
    /// Gems currently sitting in the sockets, in order.
    #[serde(default)]
    pub socketed: Vec<crate::components::item::SocketedGem>,
    /// For a loose gem item, its family + quality identity.
    #[serde(default)]
    pub gem: Option<crate::components::item::SocketedGem>,
    /// Display words for the rolled prefixes (the blue tooltip lines).
    #[serde(default)]
    pub prefix_words: Vec<String>,
    /// Display words for the rolled suffixes.
    #[serde(default)]
    pub suffix_words: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub character_name: String,
    pub class: ClassType,
    pub level: u32,
    pub experience: u64,
    pub stats: SavedStats,
    pub health: f64,
    pub mana: f64,
    pub position: (f32, f32),
    pub current_zone: String,
    pub inventory: Vec<SavedItem>,
    pub equipment: HashMap<String, SavedItem>,
    pub skill_levels: HashMap<u32, u32>,
    /// Unspent skill points (the pool the player draws from to invest in the
    /// tree). Defaulted to 0 for saves written before the field existed.
    #[serde(default)]
    pub skill_points: u32,
    pub gold: u64,
    /// Crafting materials salvaged at the forge. Defaulted for older saves.
    #[serde(default)]
    pub materials: crate::systems::forge::Materials,
    pub play_time_secs: u64,
    pub quests_completed: Vec<String>,
    /// Activated waypoint ids for the player, used for fast-travel persistence.
    #[serde(default)]
    pub discovered_waypoints: Vec<u32>,
    /// Per-zone list of `SpawnPoint::stable_id()`s the player has already
    /// killed — these spawns are skipped on the next zone load so dead mobs
    /// stay dead within a run.
    #[serde(default)]
    pub killed_mobs: BTreeMap<String, Vec<u64>>,
    /// Foreign skill branches the player has bought, identified by
    /// `(originating class token, branch index)`. Native branches are not
    /// stored — they are implicitly granted by the player's own class.
    #[serde(default)]
    pub unlocked_branches: Vec<(String, u32)>,
    pub timestamp: String,
    pub player_id: String,
    pub checksum: String,
}

/// Summary of a save file shown in the "Charger partie" list.
#[derive(Debug, Clone)]
pub struct SaveSummary {
    pub file_name: String,
    pub path: PathBuf,
    pub character_name: String,
    pub class: ClassType,
    pub level: u32,
    pub current_zone: String,
    pub play_time_secs: u64,
    pub timestamp: String,
}

// ── ID + checksum helpers ──────────────────────────────────────────────────

/// Synthesise a UUID-shaped identifier from the system clock + a process-local
/// counter. The actual `uuid` crate would be ideal but isn't in the workspace;
/// this is enough to be globally unique in practice for save tagging.
pub fn generate_player_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let c = COUNTER.fetch_add(1, Ordering::Relaxed);
    let mix1 = nanos.rotate_left(13) ^ c.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    let mix2 = (nanos as u128 ^ ((c as u128) << 64)) as u64 ^ 0xCAFE_F00D_DEAD_BEEF;
    let a = (mix1 >> 32) as u32;
    let b = (mix1 & 0xFFFF) as u16;
    // Force the version nibble to 4 to look like UUIDv4.
    let c2 = (((mix1 >> 16) & 0x0FFF) as u16) | 0x4000;
    let d = (((mix2 >> 48) & 0x3FFF) as u16) | 0x8000;
    let e = mix2 & 0xFFFF_FFFF_FFFF;
    format!("{a:08x}-{b:04x}-{c2:04x}-{d:04x}-{e:012x}")
}

/// Now-as-ISO-8601 (UTC, second resolution). Hand-formatted to avoid pulling
/// in `chrono` for a single timestamp.
fn iso8601_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Civil-from-days (Howard Hinnant). Works for any year >= 1970.
    let days = (secs / 86_400) as i64;
    let rem = (secs % 86_400) as u32;
    let h = rem / 3600;
    let m = (rem % 3600) / 60;
    let s = rem % 60;
    let z = days + 719_468;
    let era = if z >= 0 { z / 146_097 } else { (z - 146_096) / 146_097 };
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let yr = if mo <= 2 { y + 1 } else { y };
    format!("{yr:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

/// Mix the serialized save payload through `DefaultHasher` with a fixed salt.
/// Not SHA-256 (sha2 isn't in the workspace), but enough to catch the
/// hand-edited JSON files an "anti-triche basique" guard is meant to spot.
fn checksum_for(payload: &str) -> String {
    let mut hasher = DefaultHasher::new();
    CHECKSUM_SALT.hash(&mut hasher);
    payload.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

// ── ItemInstance <-> SavedItem ─────────────────────────────────────────────

fn stat_name(stat: StatKind) -> &'static str {
    match stat {
        StatKind::Strength => "Strength",
        StatKind::Dexterity => "Dexterity",
        StatKind::Vitality => "Vitality",
        StatKind::Energy => "Energy",
        StatKind::Life => "Life",
        StatKind::Mana => "Mana",
        StatKind::Stamina => "Stamina",
        StatKind::AttackRating => "AttackRating",
        StatKind::Defense => "Defense",
        StatKind::BlockChance => "BlockChance",
        StatKind::DamageMin => "DamageMin",
        StatKind::DamageMax => "DamageMax",
        StatKind::CritChance => "CritChance",
        StatKind::AttackSpeed => "AttackSpeed",
        StatKind::CastSpeed => "CastSpeed",
        StatKind::MovementSpeed => "MovementSpeed",
        StatKind::FireResist => "FireResist",
        StatKind::ColdResist => "ColdResist",
        StatKind::LightningResist => "LightningResist",
        StatKind::PoisonResist => "PoisonResist",
        StatKind::MagicFind => "MagicFind",
        StatKind::GoldFind => "GoldFind",
        StatKind::LifeOnHit => "LifeOnHit",
        StatKind::ManaOnKill => "ManaOnKill",
    }
}

fn stat_from_name(s: &str) -> StatKind {
    match s {
        "Strength" => StatKind::Strength,
        "Dexterity" => StatKind::Dexterity,
        "Vitality" => StatKind::Vitality,
        "Energy" => StatKind::Energy,
        "Life" => StatKind::Life,
        "Mana" => StatKind::Mana,
        "Stamina" => StatKind::Stamina,
        "AttackRating" => StatKind::AttackRating,
        "Defense" => StatKind::Defense,
        "BlockChance" => StatKind::BlockChance,
        "DamageMin" => StatKind::DamageMin,
        "DamageMax" => StatKind::DamageMax,
        "CritChance" => StatKind::CritChance,
        "AttackSpeed" => StatKind::AttackSpeed,
        "CastSpeed" => StatKind::CastSpeed,
        "MovementSpeed" => StatKind::MovementSpeed,
        "FireResist" => StatKind::FireResist,
        "ColdResist" => StatKind::ColdResist,
        "LightningResist" => StatKind::LightningResist,
        "PoisonResist" => StatKind::PoisonResist,
        "MagicFind" => StatKind::MagicFind,
        "GoldFind" => StatKind::GoldFind,
        "LifeOnHit" => StatKind::LifeOnHit,
        _ => StatKind::ManaOnKill,
    }
}

fn kind_name(k: ItemKind) -> &'static str {
    match k {
        ItemKind::Sword => "Sword",
        ItemKind::Axe => "Axe",
        ItemKind::Mace => "Mace",
        ItemKind::Dagger => "Dagger",
        ItemKind::Bow => "Bow",
        ItemKind::Crossbow => "Crossbow",
        ItemKind::Staff => "Staff",
        ItemKind::Wand => "Wand",
        ItemKind::Scepter => "Scepter",
        ItemKind::Helm => "Helm",
        ItemKind::BodyArmor => "BodyArmor",
        ItemKind::Belt => "Belt",
        ItemKind::Gloves => "Gloves",
        ItemKind::Boots => "Boots",
        ItemKind::Shield => "Shield",
        ItemKind::Ring => "Ring",
        ItemKind::Amulet => "Amulet",
        ItemKind::Gem => "Gem",
        ItemKind::Rune => "Rune",
        ItemKind::Jewel => "Jewel",
        ItemKind::SmallCharm => "SmallCharm",
        ItemKind::LargeCharm => "LargeCharm",
        ItemKind::GrandCharm => "GrandCharm",
        ItemKind::Gold => "Gold",
        ItemKind::Key => "Key",
        ItemKind::ScrollOfIdentify => "ScrollOfIdentify",
        ItemKind::TomeOfIdentify => "TomeOfIdentify",
        ItemKind::TownPortalScroll => "TownPortalScroll",
    }
}

fn kind_from_name(s: &str) -> ItemKind {
    match s {
        "Sword" => ItemKind::Sword,
        "Axe" => ItemKind::Axe,
        "Mace" => ItemKind::Mace,
        "Dagger" => ItemKind::Dagger,
        "Bow" => ItemKind::Bow,
        "Crossbow" => ItemKind::Crossbow,
        "Staff" => ItemKind::Staff,
        "Wand" => ItemKind::Wand,
        "Scepter" => ItemKind::Scepter,
        "Helm" => ItemKind::Helm,
        "BodyArmor" => ItemKind::BodyArmor,
        "Belt" => ItemKind::Belt,
        "Gloves" => ItemKind::Gloves,
        "Boots" => ItemKind::Boots,
        "Shield" => ItemKind::Shield,
        "Ring" => ItemKind::Ring,
        "Amulet" => ItemKind::Amulet,
        _ => ItemKind::Sword,
    }
}

fn rarity_name(r: Rarity) -> &'static str {
    match r {
        Rarity::Normal => "Normal",
        Rarity::Magic => "Magic",
        Rarity::Rare => "Rare",
        Rarity::Set => "Set",
        Rarity::Unique => "Unique",
        Rarity::Crafted => "Crafted",
    }
}

fn rarity_from_name(s: &str) -> Rarity {
    match s {
        "Magic" => Rarity::Magic,
        "Rare" => Rarity::Rare,
        "Set" => Rarity::Set,
        "Unique" => Rarity::Unique,
        "Crafted" => Rarity::Crafted,
        _ => Rarity::Normal,
    }
}

fn slot_to_string(s: EquipSlot) -> &'static str {
    match s {
        EquipSlot::Head => "Head",
        EquipSlot::Chest => "Chest",
        EquipSlot::Belt => "Belt",
        EquipSlot::Gloves => "Gloves",
        EquipSlot::Boots => "Boots",
        EquipSlot::Ring1 => "Ring1",
        EquipSlot::Ring2 => "Ring2",
        EquipSlot::Amulet => "Amulet",
        EquipSlot::MainHand => "MainHand",
        EquipSlot::OffHand => "OffHand",
    }
}

fn slot_from_string(s: &str) -> Option<EquipSlot> {
    Some(match s {
        "Head" => EquipSlot::Head,
        "Chest" => EquipSlot::Chest,
        "Belt" => EquipSlot::Belt,
        "Gloves" => EquipSlot::Gloves,
        "Boots" => EquipSlot::Boots,
        "Ring1" => EquipSlot::Ring1,
        "Ring2" => EquipSlot::Ring2,
        "Amulet" => EquipSlot::Amulet,
        "MainHand" => EquipSlot::MainHand,
        "OffHand" => EquipSlot::OffHand,
        _ => return None,
    })
}

fn zone_to_string(z: ZoneId) -> &'static str {
    z.key()
}

#[allow(dead_code)]
fn zone_from_string(s: &str) -> ZoneId {
    ZoneId::from_key(s)
}

fn affix_to_saved(a: &AppliedAffix, kind: AffixKind) -> SavedAffix {
    let slot = match kind {
        AffixKind::Prefix => "prefix",
        AffixKind::Suffix => "suffix",
    };
    SavedAffix {
        def_id: a.def_id,
        slot: slot.to_string(),
        mods: a
            .mods
            .iter()
            .map(|m| {
                let (value, pct) = match m.modifier {
                    Modifier::Flat(v) => (v, false),
                    Modifier::Percentage(v) => (v, true),
                };
                SavedMod { stat: stat_name(m.stat).to_string(), value, pct }
            })
            .collect(),
    }
}

fn instance_to_saved(inst: &ItemInstance) -> SavedItem {
    let mut affixes = Vec::new();
    for p in &inst.affixes.prefixes {
        affixes.push(affix_to_saved(p, AffixKind::Prefix));
    }
    for s in &inst.affixes.suffixes {
        affixes.push(affix_to_saved(s, AffixKind::Suffix));
    }
    SavedItem {
        base_kind: kind_name(inst.item.base.kind).to_string(),
        rarity: rarity_name(inst.item.rarity).to_string(),
        name: inst.name.clone(),
        affixes,
        level_req: inst.item.base.requirements.min_level,
        unique_id: inst.unique_id,
        sockets: inst.sockets,
        socketed: inst.socketed.clone(),
        gem: inst.gem,
        prefix_words: inst.prefix_words.clone(),
        suffix_words: inst.suffix_words.clone(),
    }
}

#[cfg(test)]
fn instance_from_saved(s: &SavedItem, uid: u64) -> Option<ItemInstance> {
    instance_from_saved_with_data(s, uid, None)
}

fn instance_from_saved_with_data(
    s: &SavedItem,
    uid: u64,
    data: Option<&crate::data::GameData>,
) -> Option<ItemInstance> {
    let kind = kind_from_name(&s.base_kind);
    let rarity = rarity_from_name(&s.rarity);
    let level = s.level_req.max(1);
    let base = base_item(kind, level);
    let mut item = Item::new(uid, base, rarity, level);
    item.identified = true;
    // Rebuild each rolled affix as its own group, preserving its def_id and the
    // flat/percentage nature of every mod.
    let mut affixes = ItemAffixes::new();
    for sa in &s.affixes {
        let mods: Vec<AffixMod> = sa
            .mods
            .iter()
            .map(|m| AffixMod {
                stat: stat_from_name(&m.stat),
                modifier: if m.pct {
                    Modifier::Percentage(m.value)
                } else {
                    Modifier::Flat(m.value)
                },
            })
            .collect();
        if mods.is_empty() {
            continue;
        }
        let is_suffix = sa.slot == "suffix";
        let applied = AppliedAffix {
            def_id: sa.def_id,
            kind: if is_suffix { AffixKind::Suffix } else { AffixKind::Prefix },
            mods,
        };
        if is_suffix {
            let _ = affixes.add_suffix(applied);
        } else {
            let _ = affixes.add_prefix(applied);
        }
    }
    // For unique-rarity items, ignore the saved affixes and rebuild the
    // fixed mods + flavour from the registry by id. Falls back to the
    // saved name when the id isn't found (e.g. the unique was removed).
    let mut extra_mods: Vec<AffixMod> = Vec::new();
    let mut flavour: Option<String> = None;
    if let (Some(id), Some(gd)) = (s.unique_id, data) {
        if let Some(def) = gd.unique_by_id(id) {
            extra_mods = def
                .mods
                .iter()
                .map(|m| AffixMod { stat: m.stat, modifier: m.modifier })
                .collect();
            flavour = (!def.flavour.is_empty()).then(|| def.flavour.clone());
        }
    }

    let mut inst = ItemInstance::new(item, affixes, extra_mods, s.name.clone());
    inst.unique_id = s.unique_id;
    inst.flavour = flavour;
    inst.sockets = s.sockets;
    inst.socketed = s.socketed.clone();
    // Guard the `socketed.len() <= sockets` invariant against hand-edited saves.
    inst.socketed.truncate(s.sockets as usize);
    inst.gem = s.gem;
    inst.prefix_words = s.prefix_words.clone();
    inst.suffix_words = s.suffix_words.clone();
    Some(inst)
}

// ── Build a SaveData snapshot from the live World ──────────────────────────

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn snapshot(world: &World, character_name: &str, player_id: &str, play_time_secs: u64) -> Option<SaveData> {
    let p = world.player_entity?;
    let class = *world.classes.get(&p)?;
    let prog = world.progression.get(&p)?;
    let stats = world.stats.get(&p)?;
    let pos = world.positions.get(&p).copied()?;
    let health = world.health.get(&p).map(|h| h.current).unwrap_or(0.0);
    let mana = world.mana.get(&p).map(|m| m.current).unwrap_or(0.0);
    let gold = world.inventory.get(&p).map(|i| i.gold).unwrap_or(0);
    let materials = world.inventory.get(&p).map(|i| i.materials.clone()).unwrap_or_default();

    let inventory: Vec<SavedItem> = world
        .inventory
        .get(&p)
        .map(|inv| inv.slots.iter().filter_map(|s| s.as_ref().map(instance_to_saved)).collect())
        .unwrap_or_default();

    let mut equipment: HashMap<String, SavedItem> = HashMap::new();
    if let Some(eq) = world.equipment.get(&p) {
        for (slot, item) in &eq.slots {
            equipment.insert(slot_to_string(*slot).to_string(), instance_to_saved(item));
        }
    }

    let quests_completed = world
        .quests
        .quests
        .iter()
        .filter(|q| q.done)
        .map(|q| q.title.clone())
        .collect();

    // Persist invested skill ranks + the unspent point pool from the tree.
    let (skill_levels, skill_points) = world
        .skill_trees
        .get(&p)
        .map(|tree| {
            let levels: HashMap<u32, u32> = tree.learned.iter().map(|s| (s.id, s.level)).collect();
            (levels, tree.available_points)
        })
        .unwrap_or_default();

    let discovered_waypoints = world
        .discovered_wps
        .get(&p)
        .map(|d| d.activated.clone())
        .unwrap_or_default();

    let killed_mobs = world.killed_mobs.clone();

    let unlocked_branches: Vec<(String, u32)> = world
        .unlocked_branches
        .get(&p)
        .map(|entries| entries.iter().map(|(c, i)| (class_token(*c).to_string(), *i)).collect())
        .unwrap_or_default();

    let mut data = SaveData {
        version: SAVE_VERSION,
        character_name: character_name.to_string(),
        class,
        level: prog.level(),
        experience: prog.experience(),
        stats: SavedStats {
            strength: stats.inner.attributes.strength,
            dexterity: stats.inner.attributes.dexterity,
            vitality: stats.inner.attributes.vitality,
            energy: stats.inner.attributes.energy,
            unspent_points: prog.inner.stat_points,
        },
        health: health as f64,
        mana: mana as f64,
        position: (pos.x, pos.y),
        current_zone: zone_to_string(world.zone).to_string(),
        inventory,
        equipment,
        skill_levels,
        skill_points,
        gold,
        materials,
        play_time_secs,
        quests_completed,
        discovered_waypoints,
        killed_mobs,
        unlocked_branches,
        timestamp: iso8601_now(),
        player_id: player_id.to_string(),
        checksum: String::new(),
    };
    // Compute checksum over the JSON payload with an empty checksum field.
    let body = serde_json::to_string(&data).ok()?;
    data.checksum = checksum_for(&body);
    Some(data)
}

// ── Disk helpers ───────────────────────────────────────────────────────────

/// Resolve the saves directory next to the running binary; fall back to CWD.
pub fn saves_dir() -> PathBuf {
    let exe = std::env::current_exe().ok();
    let base = exe
        .as_ref()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
    base.join("saves")
}

fn class_token(c: ClassType) -> &'static str {
    match c {
        ClassType::Soldat => "soldat",
        ClassType::Clerc => "clerc",
        ClassType::MageDeGuerre => "mage",
        ClassType::Archer => "archer",
    }
}

fn class_from_token(s: &str) -> Option<ClassType> {
    Some(match s {
        "soldat" => ClassType::Soldat,
        "clerc" => ClassType::Clerc,
        "mage" => ClassType::MageDeGuerre,
        "archer" => ClassType::Archer,
        _ => return None,
    })
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

pub fn save_filename(character_name: &str, class: ClassType, slot: u32) -> String {
    format!("{}_{}_{slot}.json", sanitize(character_name), class_token(class))
}

/// Write `data` to its conventional slot file under `saves_dir()`.
pub fn save_to_slot(data: &SaveData, slot: u32) -> anyhow::Result<PathBuf> {
    let dir = saves_dir();
    fs::create_dir_all(&dir)?;
    let path = dir.join(save_filename(&data.character_name, data.class, slot));
    let body = serde_json::to_string_pretty(data)?;
    fs::write(&path, body)?;
    Ok(path)
}

/// Read a save file. Returns `Err` if the checksum mismatches.
pub fn load_from_path(path: &Path) -> anyhow::Result<SaveData> {
    let body = fs::read_to_string(path)?;
    let mut data: SaveData = serde_json::from_str(&body)?;
    let claimed = std::mem::take(&mut data.checksum);
    let recomputed = checksum_for(&serde_json::to_string(&data)?);
    if claimed != recomputed {
        anyhow::bail!("checksum mismatch in {}", path.display());
    }
    data.checksum = claimed;
    Ok(data)
}

/// List every parseable save in `saves_dir()`, newest first.
pub fn list_saves() -> Vec<SaveSummary> {
    let dir = saves_dir();
    let mut out: Vec<SaveSummary> = Vec::new();
    let Ok(entries) = fs::read_dir(&dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Ok(data) = load_from_path(&path) else { continue };
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        out.push(SaveSummary {
            file_name,
            path,
            character_name: data.character_name,
            class: data.class,
            level: data.level,
            current_zone: data.current_zone,
            play_time_secs: data.play_time_secs,
            timestamp: data.timestamp,
        });
    }
    out.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    out
}

// ── Applying a loaded SaveData back onto a freshly-built World ─────────────

pub fn apply_to_world(world: &mut World, data: &SaveData) {
    let Some(p) = world.player_entity else { return };

    if let Some(prog) = world.progression.get_mut(&p) {
        prog.inner.level = data.level;
        prog.inner.experience = data.experience;
        prog.inner.stat_points = data.stats.unspent_points;
    }

    if let Some(stats) = world.stats.get_mut(&p) {
        stats.inner.attributes.strength = data.stats.strength;
        stats.inner.attributes.dexterity = data.stats.dexterity;
        stats.inner.attributes.vitality = data.stats.vitality;
        stats.inner.attributes.energy = data.stats.energy;
        stats.inner.level = data.level;
    }

    if let Some(h) = world.health.get_mut(&p) {
        h.current = (data.health as f32).min(h.max).max(0.0);
    }
    if let Some(m) = world.mana.get_mut(&p) {
        m.current = (data.mana as f32).min(m.max).max(0.0);
    }

    if let Some(pos) = world.positions.get_mut(&p) {
        pos.x = data.position.0;
        pos.y = data.position.1;
    }
    world.camera.center = crate::components::Position2D::new(data.position.0, data.position.1);

    // Inventory: clear + refill, preserve gold.
    // Materialise the rebuilt instances first (this borrows `world.data`
    // immutably) so we can then take a mutable borrow on `world.inventory`.
    let inv_slot_cap = world.inventory.get(&p).map(|i| i.slots.len()).unwrap_or(0);
    if data.inventory.len() > inv_slot_cap {
        world.log_event(format!(
            "Sauvegarde : {} objet(s) au-delà de la capacité ({}) ignoré(s).",
            data.inventory.len() - inv_slot_cap,
            inv_slot_cap
        ));
    }
    let rebuilt: Vec<Option<ItemInstance>> = data
        .inventory
        .iter()
        .take(inv_slot_cap)
        .enumerate()
        .map(|(i, saved)| {
            let uid = p.0.wrapping_mul(1_000_000).wrapping_add(i as u64 + 1);
            instance_from_saved_with_data(saved, uid, Some(&world.data))
        })
        .collect();
    if let Some(inv) = world.inventory.get_mut(&p) {
        for s in inv.slots.iter_mut() {
            *s = None;
        }
        inv.gold = data.gold;
        inv.materials = data.materials.clone();
        for (i, slot) in rebuilt.into_iter().enumerate() {
            if i >= inv.slots.len() {
                break;
            }
            inv.slots[i] = slot;
        }
    }

    // Equipment: clear current modifiers, refill.
    if let Some(eq) = world.equipment.get_mut(&p) {
        eq.slots.clear();
    }
    if let Some(stats) = world.stats.get_mut(&p) {
        stats.inner.modifiers.clear();
    }
    let mut uid = p.0.wrapping_mul(2_000_000) + 1;
    let saved_eq: Vec<(EquipSlot, ItemInstance)> = data
        .equipment
        .iter()
        .filter_map(|(k, v)| {
            slot_from_string(k).and_then(|slot| {
                instance_from_saved_with_data(
                    v,
                    {
                        uid += 1;
                        uid
                    },
                    Some(&world.data),
                )
                .map(|i| (slot, i))
            })
        })
        .collect();
    for (slot, item) in saved_eq {
        if let Some(eq) = world.equipment.get_mut(&p) {
            eq.slots.insert(slot, item);
        }
    }
    // Fold every just-restored affix into the stat sheet and resync pools.
    crate::systems::stats::recompute_player_stats(world, p);

    // Restore the skill tree: invested ranks + the unspent point pool, then
    // mirror each rank onto the hotbar so the existing cast-time formulas
    // (which read `ActiveSkill.level`) pick up the loaded progression.
    if let Some(tree) = world.skill_trees.get_mut(&p) {
        tree.learned.clear();
        for (&id, &lvl) in &data.skill_levels {
            if lvl > 0 {
                tree.learned.push(LearnedSkill { id, level: lvl });
            }
        }
        tree.available_points = data.skill_points;
    }
    if let Some(slot) = world.skills.get_mut(&p) {
        for active in slot.bar.iter_mut() {
            if let Some(&lvl) = data.skill_levels.get(&active.id) {
                if lvl > 0 {
                    active.level = lvl;
                }
            }
        }
    }

    // Restore the killed-mob ledger: replaces whatever the fresh World started
    // with so previously slain spawns stay skipped on the next zone enter.
    world.killed_mobs = data.killed_mobs.clone();

    // Restore unlocked foreign branches.
    let mut unlocked: Vec<(ClassType, u32)> = Vec::new();
    for (token, idx) in &data.unlocked_branches {
        if let Some(c) = class_from_token(token) {
            unlocked.push((c, *idx));
        }
    }
    world.unlocked_branches.insert(p, unlocked);

    // Restore activated waypoints (always include the village starter beacon).
    let entry = world.discovered_wps.entry(p).or_default();
    entry.activated.clear();
    for id in &data.discovered_waypoints {
        entry.activate(*id);
    }
    entry.activate(crate::data::maps::WP_VILLAGE);

    world.log_event(format!("Save loaded ({} lvl {}).", data.character_name, data.level));
}

/// Pretty-print a duration as `HhMmSs` (e.g. "2h13m04s") for the load list.
pub fn fmt_play_time(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    if h > 0 {
        format!("{h}h{m:02}m{s:02}s")
    } else if m > 0 {
        format!("{m}m{s:02}s")
    } else {
        format!("{s}s")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_changes_with_payload() {
        assert_ne!(checksum_for("a"), checksum_for("b"));
        assert_eq!(checksum_for("hello"), checksum_for("hello"));
    }

    #[test]
    fn player_id_is_uuid_shaped() {
        let id = generate_player_id();
        let parts: Vec<&str> = id.split('-').collect();
        assert_eq!(parts.len(), 5);
        assert_eq!(parts[0].len(), 8);
        assert_eq!(parts[1].len(), 4);
        assert_eq!(parts[2].len(), 4);
        assert_eq!(parts[3].len(), 4);
        assert_eq!(parts[4].len(), 12);
    }

    #[test]
    fn fmt_play_time_compact() {
        assert_eq!(fmt_play_time(45), "45s");
        assert_eq!(fmt_play_time(125), "2m05s");
        assert_eq!(fmt_play_time(3661), "1h01m01s");
    }

    #[test]
    fn discovered_waypoints_round_trip_via_save() {
        use crate::components::player::ClassType;
        use crate::data::maps::{village_map, WP_PLAINS, WP_VILLAGE};
        use crate::data::GameData;
        use crate::ecs::World;
        use crate::plugins::player::spawn_player_system;

        let mut world = World::new(village_map(), GameData::load(), 7);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        let p = world.player_entity.unwrap();
        // Pretend the player has discovered both beacons.
        world.discovered_wps.entry(p).or_default().activate(WP_VILLAGE);
        world.discovered_wps.entry(p).or_default().activate(WP_PLAINS);

        let snap = snapshot(&world, "Hero", "pid", 0).expect("snapshot");
        assert!(snap.discovered_waypoints.contains(&WP_VILLAGE));
        assert!(snap.discovered_waypoints.contains(&WP_PLAINS));

        // Rebuild a fresh world and apply the save.
        let mut world2 = World::new(village_map(), GameData::load(), 7);
        world2.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world2);
        apply_to_world(&mut world2, &snap);
        let p2 = world2.player_entity.unwrap();
        let known = &world2.discovered_wps[&p2].activated;
        assert!(known.contains(&WP_VILLAGE));
        assert!(known.contains(&WP_PLAINS));
    }

    #[test]
    fn item_roundtrip_via_saved_form() {
        use crate::data::items::base_item;
        let base = base_item(ItemKind::Sword, 1);
        let item = Item::new(7, base, Rarity::Normal, 1);
        let inst = ItemInstance::new(item, ItemAffixes::new(), Vec::new(), "Sword of Test".into());
        let saved = instance_to_saved(&inst);
        let back = instance_from_saved(&saved, 8).expect("rebuild");
        assert_eq!(back.name, "Sword of Test");
        assert_eq!(back.item.base.kind, ItemKind::Sword);
    }

    #[test]
    fn mixed_flat_pct_and_multistat_affixes_survive_roundtrip() {
        use crate::data::items::base_item;
        // A rare boot: a flat prefix, a percentage suffix, and a multi-stat
        // prefix — the exact shapes the old flat-only schema corrupted.
        let item = Item::new(7, base_item(ItemKind::Boots, 12), Rarity::Rare, 12);
        let mut affixes = ItemAffixes::new();
        affixes.add_prefix(AppliedAffix {
            def_id: 101,
            kind: AffixKind::Prefix,
            mods: vec![AffixMod { stat: StatKind::Strength, modifier: Modifier::Flat(7) }],
        });
        affixes.add_prefix(AppliedAffix {
            def_id: 102,
            kind: AffixKind::Prefix,
            mods: vec![
                AffixMod { stat: StatKind::Strength, modifier: Modifier::Flat(3) },
                AffixMod { stat: StatKind::Vitality, modifier: Modifier::Flat(5) },
            ],
        });
        affixes.add_suffix(AppliedAffix {
            def_id: 203,
            kind: AffixKind::Suffix,
            mods: vec![AffixMod { stat: StatKind::MovementSpeed, modifier: Modifier::Percentage(30) }],
        });
        let mut inst = ItemInstance::new(item, affixes, Vec::new(), "Bottes Fleet du Géant".into());
        inst.prefix_words = vec!["Giant's".into(), "Fleet".into()];
        inst.suffix_words = vec!["of Speed".into()];

        let back = instance_from_saved(&instance_to_saved(&inst), 8).expect("rebuild");

        // Per-affix structure is preserved (2 prefixes, 1 suffix), not merged.
        assert_eq!(back.affixes.prefixes.len(), 2, "two distinct prefixes survive");
        assert_eq!(back.affixes.suffixes.len(), 1, "the suffix survives");
        assert_eq!(back.affixes.prefixes[0].def_id, 101, "def_id is preserved");
        // The multi-stat prefix keeps both of its parts.
        assert_eq!(back.affixes.prefixes[1].mods.len(), 2, "multi-stat affix keeps both mods");
        // The percentage suffix stays a percentage (the C1 corruption case).
        assert_eq!(
            back.affixes.suffixes[0].mods[0].modifier,
            Modifier::Percentage(30),
            "a %% affix must NOT round-trip back as flat",
        );
        // Affix display words survive.
        assert_eq!(back.prefix_words, vec!["Giant's", "Fleet"]);
        assert_eq!(back.suffix_words, vec!["of Speed"]);
    }

    #[test]
    fn oversized_socketed_list_is_truncated_on_load() {
        use crate::components::item::{GemKind, SocketedGem};
        use crate::data::items::base_item;
        // A hand-edited save claiming 1 socket but 3 gems must clamp to 1.
        let item = Item::new(7, base_item(ItemKind::Sword, 5), Rarity::Magic, 5);
        let mut inst = ItemInstance::new(item, ItemAffixes::new(), Vec::new(), "Triche".into());
        inst.sockets = 1;
        inst.socketed = vec![
            SocketedGem { kind: GemKind::Rubis, tier: 4 },
            SocketedGem { kind: GemKind::Rubis, tier: 4 },
            SocketedGem { kind: GemKind::Rubis, tier: 4 },
        ];
        let back = instance_from_saved(&instance_to_saved(&inst), 8).expect("rebuild");
        assert_eq!(back.socketed.len(), 1, "socketed is clamped to the socket count");
    }

    #[test]
    fn sockets_and_gems_survive_the_saved_form() {
        use crate::components::item::{GemKind, SocketedGem};
        use crate::data::items::base_item;
        // A two-socket sword with a perfect Zirconia inserted.
        let item = Item::new(7, base_item(ItemKind::Sword, 5), Rarity::Magic, 5);
        let mut inst = ItemInstance::new(item, ItemAffixes::new(), Vec::new(), "Lame sertie".into());
        inst.sockets = 2;
        inst.socketed.push(SocketedGem { kind: GemKind::Zirconia, tier: 4 });

        let back = instance_from_saved(&instance_to_saved(&inst), 8).expect("rebuild");
        assert_eq!(back.sockets, 2, "socket count persists");
        assert_eq!(back.socketed.len(), 1, "the inserted gem persists");
        assert_eq!(back.socketed[0].kind, GemKind::Zirconia);
        assert_eq!(back.socketed[0].tier, 4);

        // A loose gem keeps its identity too.
        let gem_item = Item::new(9, base_item(ItemKind::Gem, 1), Rarity::Magic, 1);
        let mut loose = ItemInstance::new(gem_item, ItemAffixes::new(), Vec::new(), "Rubis".into());
        loose.gem = Some(SocketedGem { kind: GemKind::Rubis, tier: 2 });
        let back = instance_from_saved(&instance_to_saved(&loose), 10).expect("rebuild");
        assert_eq!(back.gem, Some(SocketedGem { kind: GemKind::Rubis, tier: 2 }));
    }
}
