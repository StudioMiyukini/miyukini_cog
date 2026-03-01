<!-- @id: SD-Tech-Data-Schemas @do: reference @role: tech-lead @layer: 3 @human: miyuk -->

# SD-Tech-Data-Schemas -- Schemas de donnees TOML Sodomight

**Auteur :** Denis (Chef Dev Senior, Miyukini AI Studio)
**Date :** 2026-02-28
**Statut :** Reference technique -- v1.0
**Projet :** Sodomight (clone fidele Diablo 2 LoD, assets maison)
**Moteur :** MGE (Miyukini Game Engine) -- data-driven TOML + serde Rust

---

## Table des matieres

1. [Principes de schemas data-driven](#1-principes-de-schemas-data-driven)
2. [Items -- Schemas TOML et structs Rust](#2-items--schemas-toml-et-structs-rust)
3. [Monstres -- Schemas TOML et structs Rust](#3-monstres--schemas-toml-et-structs-rust)
4. [Competences -- Schemas TOML et structs Rust](#4-competences--schemas-toml-et-structs-rust)
5. [Zones -- Schemas TOML et structs Rust](#5-zones--schemas-toml-et-structs-rust)
6. [Quetes -- Schemas TOML et structs Rust](#6-quetes--schemas-toml-et-structs-rust)
7. [Personnages -- Schemas TOML et structs Rust](#7-personnages--schemas-toml-et-structs-rust)
8. [Regles de validation](#8-regles-de-validation)

---

## 1. Principes de schemas data-driven

### 1.1 Conventions

- Tous les fichiers de donnees de jeu sont en TOML.
- Chaque fichier TOML correspond a une struct Rust `serde::Deserialize`.
- Les IDs sont des strings lisibles humainement (ex: `"long_sword"`, `"act1/blood_moor"`).
- Les references entre fichiers utilisent les IDs string (ex: le champ `monster_id` d'une zone reference un ID de monstre).
- Les valeurs numeriques sont i32 sauf indication contraire.
- Les pourcentages sont stockes comme entiers (ex: 75 pour 75%).
- Les durees en jeu sont en frames (1 frame = 40 ms a 25 Hz) sauf mention explicite "seconds".

### 1.2 Arborescence des donnees

```
games/sodomight/data/
  classes/
    amazon.toml
    necromancer.toml
    barbarian.toml
    sorceress.toml
    paladin.toml
    druid.toml
    assassin.toml
  skills/
    amazon/        # 30 fichiers (10 par arbre)
    necromancer/
    barbarian/
    sorceress/
    paladin/
    druid/
    assassin/
  items/
    bases/
      weapons.toml
      armor.toml
      shields.toml
      helms.toml
      gloves.toml
      boots.toml
      belts.toml
      rings.toml
      amulets.toml
    affixes/
      prefixes.toml
      suffixes.toml
    uniques/
      weapons.toml
      armor.toml
      shields.toml
      rings_amulets.toml
      charms.toml
      class_specific.toml
    sets/
      angelic_raiment.toml
      arcanna_tricks.toml
      ... (34 fichiers, 1 par set)
    runewords.toml
    runes.toml
    gems.toml
    charms_bases.toml
    cube_recipes.toml
  monsters/
    act1/
    act2/
    act3/
    act4/
    act5/
    affixes/
      champion_affixes.toml
    super_uniques/
  zones/
    act1/
    act2/
    act3/
    act4/
    act5/
  loot_tables/
    treasure_classes.toml
    bosses.toml
    rune_drops.toml
  quests/
    act1/ act2/ act3/ act4/ act5/
  config/
    difficulty.toml
    breakpoints.toml
    experience.toml
    shrines.toml
```

---

## 2. Items -- Schemas TOML et structs Rust

### 2.1 Item de base (commun a tous les types)

**Fichier :** `data/items/bases/weapons.toml` (exemple armes)

```toml
# Exemple : Long Sword (Normal), Battle Sword (Exceptional), Gothic Sword (Elite)

[[weapons]]
id = "long_sword"
name = "Long Sword"
category = "Weapon"
weapon_type = "Sword"
attack_type = "Melee"
tier = "Normal"
damage_min = 3
damage_max = 16
speed = 0                    # Weapon Speed Modifier (WSM)
range = 1                    # tiles
durability = 44
grid_size = [1, 3]           # colonnes x rangees dans l'inventaire
required_strength = 55
required_dexterity = 0
required_level = 1
quality_level = 20           # qlvl
max_sockets = 2
str_factor = 100             # bonus dmg% par STR
dex_factor = 0               # bonus dmg% par DEX
can_be_ethereal = true
throwable = false
class_restriction = ""       # "" = aucune restriction

[[weapons]]
id = "battle_sword"
name = "Battle Sword"
category = "Weapon"
weapon_type = "Sword"
attack_type = "Melee"
tier = "Exceptional"
damage_min = 6
damage_max = 21
speed = 0
range = 1
durability = 44
grid_size = [1, 3]
required_strength = 92
required_dexterity = 43
required_level = 25
quality_level = 40
max_sockets = 3
str_factor = 100
dex_factor = 0
can_be_ethereal = true
throwable = false
class_restriction = ""

[[weapons]]
id = "gothic_sword"
name = "Gothic Sword"
category = "Weapon"
weapon_type = "Sword"
attack_type = "Melee"
tier = "Elite"
damage_min = 14
damage_max = 45
speed = 0
range = 1
durability = 44
grid_size = [1, 3]
required_strength = 113
required_dexterity = 20
required_level = 48
quality_level = 62
max_sockets = 4
str_factor = 100
dex_factor = 0
can_be_ethereal = true
throwable = false
class_restriction = ""
```

**Struct Rust correspondante :**

```rust
// @id: sd-data-item-base @do: define @role: arpg @layer: 3 @human: miyuk
// Crate: mge-arpg-items

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemBaseDef {
    pub id: String,
    pub name: String,
    pub category: ItemCategory,
    pub weapon_type: Option<WeaponType>,
    pub attack_type: Option<AttackType>,
    pub tier: ItemTier,
    pub damage_min: Option<i32>,
    pub damage_max: Option<i32>,
    /// Defense min (pour armures).
    pub defense_min: Option<i32>,
    /// Defense max (pour armures).
    pub defense_max: Option<i32>,
    pub speed: Option<i32>,
    pub range: Option<f32>,
    pub durability: i32,
    pub grid_size: [u8; 2],
    pub required_strength: i32,
    pub required_dexterity: i32,
    pub required_level: u8,
    pub quality_level: u8,
    pub max_sockets: u8,
    pub str_factor: Option<i32>,
    pub dex_factor: Option<i32>,
    pub can_be_ethereal: bool,
    pub throwable: bool,
    /// Classe restreinte ("" = aucune).
    pub class_restriction: String,
    /// Block chance (boucliers).
    pub block_chance: Option<i32>,
    /// Nombre de rangees de potions (ceintures).
    pub potion_rows: Option<u8>,
    /// magic_lvl (circlets, wands, orbes).
    pub magic_level: Option<i32>,
}
```

### 2.2 Item Magique (1-2 affixes)

Les items magiques utilisent les memes bases que les normaux, avec 1 prefixe et/ou
1 suffixe tires des tables d'affixes.

**Fichier :** `data/items/affixes/prefixes.toml`

```toml
[[prefixes]]
id = "cruel"
name = "Cruel"
group = 114                  # famille d'affixe (pas deux du meme groupe sur un item)
level = 56                   # affix level requis (alvl)
frequency = 1                # poids de drop (rare)
spawn_on = ["Weapon"]        # categories autorisees
properties = [
    { stat = "enhanced_damage", min = 201, max = 300 },
]

[[prefixes]]
id = "masters"
name = "Master's"
group = 12
level = 43
frequency = 3
spawn_on = ["Weapon"]
properties = [
    { stat = "enhanced_damage", min = 151, max = 200 },
]

[[prefixes]]
id = "squeezing"
name = "Squeezing"
group = 47
level = 1
frequency = 5
spawn_on = ["Ring", "Amulet", "Gloves", "Belt"]
properties = [
    { stat = "mana_steal", min = 1, max = 3 },
]
```

**Fichier :** `data/items/affixes/suffixes.toml`

```toml
[[suffixes]]
id = "of_the_whale"
name = "of the Whale"
group = 1
level = 35
frequency = 2
spawn_on = ["Armor", "Helm", "Shield", "Belt", "Amulet"]
properties = [
    { stat = "life", min = 81, max = 100 },
]

[[suffixes]]
id = "of_speed"
name = "of Speed"
group = 76
level = 37
frequency = 2
spawn_on = ["Weapon", "Gloves"]
properties = [
    { stat = "increased_attack_speed", min = 20, max = 20 },
]

[[suffixes]]
id = "of_the_colossus"
name = "of the Colossus"
group = 3
level = 62
frequency = 1
spawn_on = ["Armor", "Helm", "Shield", "Belt", "Gloves", "Boots"]
properties = [
    { stat = "strength", min = 16, max = 20 },
]
```

**Struct Rust :**

```rust
// @id: sd-data-affix-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixDef {
    pub id: String,
    pub name: String,
    pub group: u32,
    pub level: u8,
    pub frequency: u32,
    pub spawn_on: Vec<String>,
    pub properties: Vec<AffixPropertyDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixPropertyDef {
    pub stat: String,
    pub min: i32,
    pub max: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffixTableFile {
    pub prefixes: Vec<AffixDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuffixTableFile {
    pub suffixes: Vec<AffixDef>,
}
```

### 2.3 Item Rare (3-6 affixes)

Les items Rare suivent le meme systeme d'affixes que les Magic, mais avec 2-3 prefixes
et 2-3 suffixes. Le nom est genere depuis une table de noms.

```toml
# data/items/affixes/rare_names.toml

[[rare_names_weapon]]
prefix = "Doom"
suffix = "Bringer"

[[rare_names_weapon]]
prefix = "Shadow"
suffix = "Fang"

[[rare_names_armor]]
prefix = "Storm"
suffix = "Shroud"

[[rare_names_armor]]
prefix = "Blood"
suffix = "Veil"
```

```rust
// @id: sd-data-rare-names @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareNameEntry {
    pub prefix: String,
    pub suffix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareNamesFile {
    pub rare_names_weapon: Vec<RareNameEntry>,
    pub rare_names_armor: Vec<RareNameEntry>,
    pub rare_names_shield: Vec<RareNameEntry>,
    pub rare_names_ring: Vec<RareNameEntry>,
    pub rare_names_amulet: Vec<RareNameEntry>,
}
```

### 2.4 Set Items

**Fichier :** `data/items/sets/tal_rasha.toml` (exemple)

```toml
[set]
id = "tal_rasha"
name = "Tal Rasha's Wrappings"
class_restriction = ""
num_items = 5

# Bonus partiels (nombre de pieces equipes)
[[set.partial_bonuses]]
count = 2
properties = [
    { stat = "magic_find", value = 65 },
]

[[set.partial_bonuses]]
count = 3
properties = [
    { stat = "fire_resist", value = 33 },
    { stat = "lightning_resist", value = 33 },
    { stat = "cold_resist", value = 33 },
]

[[set.partial_bonuses]]
count = 4
properties = [
    { stat = "life", value = 100 },
]

# Bonus complet (5 pieces)
[set.full_bonus]
properties = [
    { stat = "life", value = 150 },
    { stat = "all_resistances", value = 50 },
    { stat = "magic_find", value = 150 },
    { stat = "enhanced_damage", value = 200 },
]

# Pieces du set
[[set.items]]
id = "tal_rasha_lidless_eye"
name = "Tal Rasha's Lidless Eye"
base_type = "swirling_crystal"
required_level = 26
properties = [
    { stat = "faster_cast_rate", value = 20 },
    { stat = "lightning_damage_min", value = 1 },
    { stat = "lightning_damage_max", value = 77 },
    { stat = "fire_damage_min", value = 3 },
    { stat = "fire_damage_max", value = 56 },
    { stat = "cold_damage_min", value = 3 },
    { stat = "cold_damage_max", value = 14 },
    { stat = "mana", value = 57 },
    { stat = "energy", value = 10 },
]

[[set.items]]
id = "tal_rasha_guardianship"
name = "Tal Rasha's Guardianship"
base_type = "lacquered_plate"
required_level = 71
properties = [
    { stat = "magic_find", value = 88 },
    { stat = "fire_resist", value = 40 },
    { stat = "lightning_resist", value = 40 },
    { stat = "cold_resist", value = 40 },
    { stat = "defense", min = 400, max = 400 },
]

[[set.items]]
id = "tal_rasha_horadric_crest"
name = "Tal Rasha's Horadric Crest"
base_type = "death_mask"
required_level = 66
properties = [
    { stat = "life_steal", value = 10 },
    { stat = "mana_steal", value = 10 },
    { stat = "all_resistances", value = 15 },
    { stat = "life", value = 60 },
    { stat = "mana", value = 30 },
]

[[set.items]]
id = "tal_rasha_fine_spun_cloth"
name = "Tal Rasha's Fine-Spun Cloth"
base_type = "mesh_belt"
required_level = 53
properties = [
    { stat = "dexterity", value = 20 },
    { stat = "mana", value = 30 },
    { stat = "faster_cast_rate", value = 10 },
    { stat = "defense_per_level", value = 1.5 },
]

[[set.items]]
id = "tal_rasha_adjudication"
name = "Tal Rasha's Adjudication"
base_type = "amulet"
required_level = 67
properties = [
    { stat = "lightning_damage_min", value = 3 },
    { stat = "lightning_damage_max", value = 32 },
    { stat = "lightning_resist", value = 33 },
    { stat = "life", value = 50 },
    { stat = "mana", value = 42 },
]
```

**Struct Rust :**

```rust
// @id: sd-data-set-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDef {
    pub set: SetHeader,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetHeader {
    pub id: String,
    pub name: String,
    pub class_restriction: String,
    pub num_items: u8,
    pub partial_bonuses: Vec<PartialBonus>,
    pub full_bonus: FullBonus,
    pub items: Vec<SetItemDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialBonus {
    pub count: u8,
    pub properties: Vec<StatValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullBonus {
    pub properties: Vec<StatValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetItemDef {
    pub id: String,
    pub name: String,
    pub base_type: String,
    pub required_level: u8,
    pub properties: Vec<StatValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatValue {
    pub stat: String,
    #[serde(default)]
    pub value: Option<i32>,
    #[serde(default)]
    pub min: Option<i32>,
    #[serde(default)]
    pub max: Option<i32>,
}
```

### 2.5 Unique Items

**Fichier :** `data/items/uniques/weapons.toml` (extrait)

```toml
[[uniques]]
id = "shako"
name = "Harlequin Crest"
base_type = "shako"
quality_level = 69
required_level = 62
properties = [
    { stat = "all_skills", value = 2 },
    { stat = "life_per_level", value = 1.5 },
    { stat = "mana_per_level", value = 1.5 },
    { stat = "damage_reduction", value = 10 },
    { stat = "magic_find", value = 50 },
    { stat = "all_attributes", value = 2 },
]

[[uniques]]
id = "windforce"
name = "Windforce"
base_type = "hydra_bow"
quality_level = 73
required_level = 73
properties = [
    { stat = "enhanced_damage", min = 250, max = 250 },
    { stat = "increased_attack_speed", value = 20 },
    { stat = "mana_steal", value = 6 },
    { stat = "knockback", value = 1 },
    { stat = "dexterity", value = 10 },
    { stat = "strength", value = 5 },
]

[[uniques]]
id = "grief"
name = "Grief"
base_type = "phase_blade"
quality_level = 59
required_level = 59
properties = [
    { stat = "flat_damage", min = 340, max = 400 },
    { stat = "increased_attack_speed", min = 30, max = 40 },
    { stat = "ignore_target_defense", value = 1 },
    { stat = "all_resistances", min = -25, max = -25 },
    { stat = "deadly_strike", min = 20, max = 25 },
    { stat = "prevent_monster_heal", value = 1 },
]

[[uniques]]
id = "enigma_runeword"
name = "Enigma"
base_type = ""   # N/A -- c'est un runeword, voir runewords.toml
quality_level = 0
required_level = 65
properties = []
```

**Struct Rust :**

```rust
// @id: sd-data-unique-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueItemDef {
    pub id: String,
    pub name: String,
    pub base_type: String,
    pub quality_level: u8,
    pub required_level: u8,
    pub properties: Vec<StatValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueItemsFile {
    pub uniques: Vec<UniqueItemDef>,
}
```

### 2.6 Runewords

**Fichier :** `data/items/runewords.toml`

```toml
[[runewords]]
id = "spirit"
name = "Spirit"
rune_sequence = ["Tal", "Thul", "Ort", "Amn"]
allowed_types = ["Sword", "Shield"]
required_level = 25
properties = [
    { stat = "all_skills", value = 2 },
    { stat = "faster_cast_rate", value = 25, range_max = 35 },
    { stat = "faster_hit_recovery", value = 55 },
    { stat = "vitality", value = 22 },
    { stat = "mana", min = 89, max = 112 },
    { stat = "magic_absorb", min = 3, max = 8 },
]

[[runewords]]
id = "enigma"
name = "Enigma"
rune_sequence = ["Jah", "Ith", "Ber"]
allowed_types = ["BodyArmor"]
required_level = 65
properties = [
    { stat = "all_skills", value = 2 },
    { stat = "teleport_charges", value = 1 },
    { stat = "run_walk_speed", value = 45 },
    { stat = "strength_per_level", value = 0.75 },
    { stat = "life_after_kill", value = 5 },
    { stat = "damage_reduction", value = 8 },
    { stat = "defense", min = 750, max = 775 },
    { stat = "magic_find_per_level", value = 1 },
]

[[runewords]]
id = "infinity"
name = "Infinity"
rune_sequence = ["Ber", "Mal", "Ber", "Ist"]
allowed_types = ["Polearm"]
required_level = 63
properties = [
    { stat = "conviction_aura_level", value = 12 },
    { stat = "enhanced_damage", min = 255, max = 325 },
    { stat = "run_walk_speed", value = 35 },
    { stat = "vitality", value = 40 },
    { stat = "lightning_damage_min", value = 1 },
    { stat = "lightning_damage_max", value = 495 },
    { stat = "crushing_blow", value = 49 },
    { stat = "prevent_monster_heal", value = 1 },
]

[[runewords]]
id = "call_to_arms"
name = "Call to Arms"
rune_sequence = ["Amn", "Ral", "Mal", "Ist", "Ohm"]
allowed_types = ["Weapon"]
required_level = 57
properties = [
    { stat = "all_skills", value = 1 },
    { stat = "increased_attack_speed", value = 40 },
    { stat = "enhanced_damage", min = 240, max = 290 },
    { stat = "fire_damage_min", value = 5 },
    { stat = "fire_damage_max", value = 30 },
    { stat = "battle_command_level", min = 2, max = 6 },
    { stat = "battle_orders_level", min = 1, max = 6 },
    { stat = "battle_cry_level", min = 1, max = 4 },
    { stat = "prevent_monster_heal", value = 1 },
]
```

**Struct Rust :**

```rust
// @id: sd-data-runeword-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunewordDef {
    pub id: String,
    pub name: String,
    pub rune_sequence: Vec<String>,
    pub allowed_types: Vec<String>,
    pub required_level: u8,
    pub properties: Vec<StatValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunewordsFile {
    pub runewords: Vec<RunewordDef>,
}
```

### 2.7 Gemmes

**Fichier :** `data/items/gems.toml`

```toml
[[gems]]
id = "perfect_amethyst"
name = "Perfect Amethyst"
gem_type = "Amethyst"
quality = "Perfect"
required_level = 18
grid_size = [1, 1]
in_weapon = [
    { stat = "attack_rating", value = 150 },
]
in_armor = [
    { stat = "strength", value = 10 },
]
in_helm = [
    { stat = "strength", value = 10 },
]
in_shield = [
    { stat = "defense", value = 8 },
]

[[gems]]
id = "perfect_topaz"
name = "Perfect Topaz"
gem_type = "Topaz"
quality = "Perfect"
required_level = 18
grid_size = [1, 1]
in_weapon = [
    { stat = "lightning_damage_min", value = 1 },
    { stat = "lightning_damage_max", value = 40 },
]
in_armor = [
    { stat = "magic_find", value = 24 },
]
in_helm = [
    { stat = "magic_find", value = 24 },
]
in_shield = [
    { stat = "lightning_resist", value = 40 },
]

[[gems]]
id = "perfect_ruby"
name = "Perfect Ruby"
gem_type = "Ruby"
quality = "Perfect"
required_level = 18
grid_size = [1, 1]
in_weapon = [
    { stat = "fire_damage_min", value = 15 },
    { stat = "fire_damage_max", value = 20 },
]
in_armor = [
    { stat = "life", value = 38 },
]
in_helm = [
    { stat = "life", value = 38 },
]
in_shield = [
    { stat = "fire_resist", value = 40 },
]

[[gems]]
id = "perfect_sapphire"
name = "Perfect Sapphire"
gem_type = "Sapphire"
quality = "Perfect"
required_level = 18
grid_size = [1, 1]
in_weapon = [
    { stat = "cold_damage_min", value = 10 },
    { stat = "cold_damage_max", value = 14 },
    { stat = "cold_duration_frames", value = 75 },
]
in_armor = [
    { stat = "mana", value = 38 },
]
in_helm = [
    { stat = "mana", value = 38 },
]
in_shield = [
    { stat = "cold_resist", value = 40 },
]

[[gems]]
id = "perfect_emerald"
name = "Perfect Emerald"
gem_type = "Emerald"
quality = "Perfect"
required_level = 18
grid_size = [1, 1]
in_weapon = [
    { stat = "poison_damage", value = 100 },
    { stat = "poison_duration_frames", value = 150 },
]
in_armor = [
    { stat = "dexterity", value = 10 },
]
in_helm = [
    { stat = "dexterity", value = 10 },
]
in_shield = [
    { stat = "poison_resist", value = 40 },
]

[[gems]]
id = "perfect_diamond"
name = "Perfect Diamond"
gem_type = "Diamond"
quality = "Perfect"
required_level = 18
grid_size = [1, 1]
in_weapon = [
    { stat = "damage_to_undead", value = 100 },
]
in_armor = [
    { stat = "all_resistances", value = 19 },
]
in_helm = [
    { stat = "all_resistances", value = 19 },
]
in_shield = [
    { stat = "all_resistances", value = 19 },
]

[[gems]]
id = "perfect_skull"
name = "Perfect Skull"
gem_type = "Skull"
quality = "Perfect"
required_level = 18
grid_size = [1, 1]
in_weapon = [
    { stat = "life_steal", value = 4 },
    { stat = "mana_steal", value = 4 },
]
in_armor = [
    { stat = "replenish_life", value = 5 },
    { stat = "mana_regen", value = 19 },
]
in_helm = [
    { stat = "replenish_life", value = 5 },
    { stat = "mana_regen", value = 19 },
]
in_shield = [
    { stat = "attacker_takes_damage", value = 20 },
]
```

**Struct Rust :**

```rust
// @id: sd-data-gem-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GemQuality {
    Chipped,
    Flawed,
    Normal,
    Flawless,
    Perfect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemDef {
    pub id: String,
    pub name: String,
    pub gem_type: String,
    pub quality: GemQuality,
    pub required_level: u8,
    pub grid_size: [u8; 2],
    pub in_weapon: Vec<StatValue>,
    pub in_armor: Vec<StatValue>,
    pub in_helm: Vec<StatValue>,
    pub in_shield: Vec<StatValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemsFile {
    pub gems: Vec<GemDef>,
}
```

### 2.8 Runes (33 runes Lux a Aeon)

**Fichier :** `data/items/runes.toml`

```toml
[[runes]]
id = "lux"
name = "Lux"
number = 1
required_level = 11
grid_size = [1, 1]
in_weapon = [
    { stat = "light_radius", value = 1 },
]
in_armor = [
    { stat = "light_radius", value = 1 },
    { stat = "defense", value = 15 },
]
in_shield = [
    { stat = "light_radius", value = 1 },
    { stat = "defense", value = 15 },
]
in_helm = [
    { stat = "light_radius", value = 1 },
    { stat = "defense", value = 15 },
]
upgrade_recipe_to = "nef"
upgrade_count = 3

[[runes]]
id = "ber"
name = "Ber"
number = 30
required_level = 63
grid_size = [1, 1]
in_weapon = [
    { stat = "crushing_blow", value = 20 },
]
in_armor = [
    { stat = "damage_reduction", value = 8 },
]
in_shield = [
    { stat = "damage_reduction", value = 8 },
]
in_helm = [
    { stat = "damage_reduction", value = 8 },
]
upgrade_recipe_to = "jah"
upgrade_count = 2

[[runes]]
id = "jah"
name = "Jah"
number = 31
required_level = 65
grid_size = [1, 1]
in_weapon = [
    { stat = "ignore_target_defense", value = 1 },
]
in_armor = [
    { stat = "life_increase_pct", value = 5 },
]
in_shield = [
    { stat = "life_increase_pct", value = 5 },
]
in_helm = [
    { stat = "life_increase_pct", value = 5 },
]
upgrade_recipe_to = "cham"
upgrade_count = 2

[[runes]]
id = "aeon"
name = "Aeon"
number = 33
required_level = 69
grid_size = [1, 1]
in_weapon = [
    { stat = "indestructible", value = 1 },
]
in_armor = [
    { stat = "indestructible", value = 1 },
]
in_shield = [
    { stat = "indestructible", value = 1 },
]
in_helm = [
    { stat = "indestructible", value = 1 },
]
upgrade_recipe_to = ""
upgrade_count = 0
```

**Struct Rust :**

```rust
// @id: sd-data-rune-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuneDef {
    pub id: String,
    pub name: String,
    pub number: u8,
    pub required_level: u8,
    pub grid_size: [u8; 2],
    pub in_weapon: Vec<StatValue>,
    pub in_armor: Vec<StatValue>,
    pub in_shield: Vec<StatValue>,
    pub in_helm: Vec<StatValue>,
    /// Rune cible de la recette d'upgrade ("" = pas d'upgrade).
    pub upgrade_recipe_to: String,
    /// Nombre de runes necessaires pour upgrade.
    pub upgrade_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunesFile {
    pub runes: Vec<RuneDef>,
}
```

### 2.9 Charmes

```toml
# data/items/charms_bases.toml

[[charms]]
id = "small_charm"
name = "Small Charm"
grid_size = [1, 1]
max_affixes = 2

[[charms]]
id = "large_charm"
name = "Large Charm"
grid_size = [1, 2]
max_affixes = 2

[[charms]]
id = "grand_charm"
name = "Grand Charm"
grid_size = [1, 3]
max_affixes = 2
```

```rust
// @id: sd-data-charm-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharmBaseDef {
    pub id: String,
    pub name: String,
    pub grid_size: [u8; 2],
    pub max_affixes: u8,
}
```

### 2.10 Recettes du Cube Alchimique

**Fichier :** `data/items/cube_recipes.toml`

```toml
[[recipes]]
id = "socket_normal_weapon"
name = "Add Sockets to Normal Weapon"
inputs = [
    { type = "rune", id = "ral" },
    { type = "rune", id = "amn" },
    { type = "gem", id = "perfect_amethyst" },
    { type = "item", quality = "Normal", category = "Weapon" },
]
output = { type = "modify_last_input", modification = "add_sockets_random" }
description = "Ajoute des sockets aleatoires a une arme normale"

[[recipes]]
id = "upgrade_rune_low"
name = "Upgrade Rune (low tier)"
inputs = [
    { type = "rune_same", count = 3 },
]
output = { type = "rune_next" }
description = "3 runes identiques = rune superieure (tiers 1-19)"

[[recipes]]
id = "upgrade_rune_high"
name = "Upgrade Rune (high tier)"
inputs = [
    { type = "rune_same", count = 2 },
]
output = { type = "rune_next" }
description = "2 runes identiques = rune superieure (tiers 20-33)"

[[recipes]]
id = "upgrade_gem"
name = "Upgrade Gem"
inputs = [
    { type = "gem_same_type_quality", count = 3 },
]
output = { type = "gem_next_quality" }
description = "3 gemmes identiques = qualite superieure"

[[recipes]]
id = "reroll_rare"
name = "Reroll Rare"
inputs = [
    { type = "item", quality = "Rare" },
    { type = "gem", id = "perfect_skull" },
    { type = "scroll", id = "scroll_tp" },
]
output = { type = "reroll_affixes", quality = "Rare" }
description = "Reroll les affixes d'un item rare"

[[recipes]]
id = "upgrade_normal_to_exceptional"
name = "Upgrade Normal Unique to Exceptional"
inputs = [
    { type = "item", quality = "Unique", tier = "Normal" },
    { type = "rune", id = "ral" },
    { type = "rune", id = "sol" },
    { type = "gem", id = "perfect_emerald" },
]
output = { type = "upgrade_tier", target_tier = "Exceptional" }
description = "Upgrade un unique Normal en Exceptional"

[[recipes]]
id = "crafted_blood_ring"
name = "Crafted Blood Ring"
inputs = [
    { type = "item", quality = "Magic", base_type = "ring" },
    { type = "jewel" },
    { type = "gem", id = "perfect_ruby" },
    { type = "rune", id = "sol" },
]
output = { type = "crafted", guaranteed_properties = [
    { stat = "life_steal", min = 1, max = 3 },
    { stat = "life", min = 10, max = 20 },
] }
description = "Cree un anneau Blood Craft"
```

**Struct Rust :**

```rust
// @id: sd-data-cube-recipe @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeRecipeDef {
    pub id: String,
    pub name: String,
    pub inputs: Vec<CubeInput>,
    pub output: CubeOutput,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CubeInput {
    #[serde(rename = "rune")]
    Rune { id: String },
    #[serde(rename = "rune_same")]
    RuneSame { count: u8 },
    #[serde(rename = "gem")]
    Gem { id: String },
    #[serde(rename = "gem_same_type_quality")]
    GemSame { count: u8 },
    #[serde(rename = "item")]
    Item {
        #[serde(default)]
        quality: Option<String>,
        #[serde(default)]
        category: Option<String>,
        #[serde(default)]
        base_type: Option<String>,
        #[serde(default)]
        tier: Option<String>,
    },
    #[serde(rename = "jewel")]
    Jewel,
    #[serde(rename = "scroll")]
    Scroll { id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CubeOutput {
    #[serde(rename = "rune_next")]
    RuneNext,
    #[serde(rename = "gem_next_quality")]
    GemNextQuality,
    #[serde(rename = "modify_last_input")]
    ModifyLastInput { modification: String },
    #[serde(rename = "reroll_affixes")]
    RerollAffixes { quality: String },
    #[serde(rename = "upgrade_tier")]
    UpgradeTier { target_tier: String },
    #[serde(rename = "crafted")]
    Crafted {
        guaranteed_properties: Vec<StatValue>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeRecipesFile {
    pub recipes: Vec<CubeRecipeDef>,
}
```

---

## 3. Monstres -- Schemas TOML et structs Rust

### 3.1 Monstre de base

**Fichier :** `data/monsters/act1/fallen.toml`

```toml
[monster]
id = "fallen"
name = "Fallen"
type = "Demon"
ai_archetype = "MeleeSwarm"
base_sprite = "monsters/fallen"
grid_hitbox_radius = 0.4

[monster.stats.normal]
level = 2
life = 8
defense = 4
attack_rating = 15
damage_min = 1
damage_max = 3
experience = 20
fire_resist = 0
cold_resist = 0
lightning_resist = 0
poison_resist = 0
physical_resist = 0
immunities = []

[monster.stats.nightmare]
level = 36
life = 750
defense = 350
attack_rating = 800
damage_min = 8
damage_max = 16
experience = 3800
fire_resist = 33
cold_resist = 33
lightning_resist = 33
poison_resist = 33
physical_resist = 0
immunities = []

[monster.stats.hell]
level = 67
life = 4500
defense = 950
attack_rating = 2800
damage_min = 18
damage_max = 42
experience = 22000
fire_resist = 50
cold_resist = 50
lightning_resist = 50
poison_resist = 50
physical_resist = 10
immunities = []

[monster.ai_params]
aggro_radius = 10.0
leash_radius = 25.0
flee_hp_pct = 0.3
rally_on_flee = true
attack_cooldown = 15

[[monster.ai_params.skills]]
skill_id = "melee_attack"
priority = 1
condition = { type = "TargetInRange", range = 1.5 }
cooldown = 0

[monster.drops]
loot_table = "tc_act1_normal"
gold_min = 1
gold_max = 5
```

**Struct Rust :**

```rust
// @id: sd-data-monster-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterFile {
    pub monster: MonsterDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDef {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub monster_type_tag: String,
    pub ai_archetype: String,
    pub base_sprite: String,
    pub grid_hitbox_radius: f32,
    pub stats: MonsterStatsByDifficulty,
    pub ai_params: MonsterAiParams,
    pub drops: MonsterDrops,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterStatsByDifficulty {
    pub normal: MonsterStatsBlock,
    pub nightmare: MonsterStatsBlock,
    pub hell: MonsterStatsBlock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterStatsBlock {
    pub level: u8,
    pub life: i32,
    pub defense: i32,
    pub attack_rating: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub experience: u32,
    pub fire_resist: i32,
    pub cold_resist: i32,
    pub lightning_resist: i32,
    pub poison_resist: i32,
    pub physical_resist: i32,
    pub immunities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterAiParams {
    pub aggro_radius: f32,
    pub leash_radius: f32,
    pub flee_hp_pct: Option<f32>,
    pub rally_on_flee: bool,
    pub attack_cooldown: u32,
    pub skills: Vec<MonsterSkillEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterSkillEntry {
    pub skill_id: String,
    pub priority: u8,
    pub condition: MonsterSkillCondition,
    pub cooldown: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MonsterSkillCondition {
    Always,
    TargetInRange { range: f32 },
    HealthBelow { pct: f32 },
    CooldownReady,
    AlliesNearby { count: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterDrops {
    pub loot_table: String,
    pub gold_min: i32,
    pub gold_max: i32,
}
```

### 3.2 Super Unique

**Fichier :** `data/monsters/super_uniques/rakanishu.toml`

```toml
[super_unique]
id = "rakanishu"
name = "Rakanishu"
base_monster = "fallen"
zone = "act1/stony_field"
position = [45.0, 32.0]

fixed_affixes = ["ExtraFast", "Lightning Enchanted"]

[super_unique.multipliers]
life_mult = 3.0
damage_mult = 2.0
attack_rating_mult = 2.0
experience_mult = 5.0

[super_unique.extra_drops]
guaranteed_drops = []
extra_loot_table = "tc_super_unique_act1"
```

```rust
// @id: sd-data-super-unique-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperUniqueFile {
    pub super_unique: SuperUniqueDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperUniqueDef {
    pub id: String,
    pub name: String,
    pub base_monster: String,
    pub zone: String,
    pub position: [f32; 2],
    pub fixed_affixes: Vec<String>,
    pub multipliers: SuperUniqueMultipliers,
    pub extra_drops: SuperUniqueDrops,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperUniqueMultipliers {
    pub life_mult: f32,
    pub damage_mult: f32,
    pub attack_rating_mult: f32,
    pub experience_mult: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuperUniqueDrops {
    pub guaranteed_drops: Vec<String>,
    pub extra_loot_table: String,
}
```

### 3.3 Champion Affixes

**Fichier :** `data/monsters/affixes/champion_affixes.toml`

```toml
[[champion_affixes]]
id = "extra_strong"
name = "Extra Strong"
stat_modifiers = [
    { stat = "enhanced_damage", value = 100 },
]
visual = "red_glow"

[[champion_affixes]]
id = "extra_fast"
name = "Extra Fast"
stat_modifiers = [
    { stat = "speed_modifier", value = 50 },
    { stat = "attack_speed_modifier", value = 30 },
]
visual = "speed_trail"

[[champion_affixes]]
id = "lightning_enchanted"
name = "Lightning Enchanted"
stat_modifiers = [
    { stat = "lightning_damage_min", value = 1 },
    { stat = "lightning_damage_max", value = 120 },
    { stat = "lightning_resist", value = 75 },
]
on_death = "charged_bolts_explosion"
visual = "lightning_sparks"

[[champion_affixes]]
id = "fire_enchanted"
name = "Fire Enchanted"
stat_modifiers = [
    { stat = "fire_damage_min", value = 20 },
    { stat = "fire_damage_max", value = 80 },
    { stat = "fire_resist", value = 75 },
]
on_death = "fire_explosion"
visual = "fire_glow"

[[champion_affixes]]
id = "cold_enchanted"
name = "Cold Enchanted"
stat_modifiers = [
    { stat = "cold_damage_min", value = 15 },
    { stat = "cold_damage_max", value = 60 },
    { stat = "cold_resist", value = 75 },
]
on_death = "frost_nova"
visual = "frost_aura"

[[champion_affixes]]
id = "stone_skin"
name = "Stone Skin"
stat_modifiers = [
    { stat = "defense_mult", value = 200 },
    { stat = "physical_resist", value = 50 },
]
visual = "stone_skin"

[[champion_affixes]]
id = "aura_enchanted"
name = "Aura Enchanted"
aura = { type = "random", possible_auras = ["Might", "Fanaticism", "Conviction", "HolyFreeze"] }
visual = "aura_glow"

[[champion_affixes]]
id = "multishot"
name = "Multishot"
stat_modifiers = [
    { stat = "projectile_count", value = 3 },
]
visual = ""

[[champion_affixes]]
id = "cursed"
name = "Cursed"
on_hit = "amplify_damage_curse"
visual = "curse_cloud"

[[champion_affixes]]
id = "mana_burn"
name = "Mana Burn"
on_hit = "drain_mana"
stat_modifiers = [
    { stat = "magic_resist", value = 75 },
]
visual = "mana_drain_glow"
```

```rust
// @id: sd-data-champion-affix @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionAffixDef {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub stat_modifiers: Vec<StatValue>,
    #[serde(default)]
    pub on_death: Option<String>,
    #[serde(default)]
    pub on_hit: Option<String>,
    #[serde(default)]
    pub aura: Option<AuraEnchant>,
    pub visual: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraEnchant {
    #[serde(rename = "type")]
    pub aura_type: String,
    pub possible_auras: Vec<String>,
}
```

### 3.4 Boss d'acte

**Fichier :** `data/monsters/act1/andariel.toml`

```toml
[boss]
id = "andariel"
name = "Andariel"
act = 1
base_sprite = "bosses/andariel"
grid_hitbox_radius = 1.2

[boss.stats.normal]
level = 12
life = 1024
defense = 90
attack_rating = 220
damage_min = 8
damage_max = 28
experience = 12000
fire_resist = -50
cold_resist = 50
lightning_resist = 50
poison_resist = 75
physical_resist = 0

[boss.stats.nightmare]
level = 49
life = 20480
defense = 750
attack_rating = 1600
damage_min = 25
damage_max = 78
experience = 120000
fire_resist = -50
cold_resist = 75
lightning_resist = 75
poison_resist = 95
physical_resist = 20

[boss.stats.hell]
level = 75
life = 65536
defense = 1500
attack_rating = 4200
damage_min = 52
damage_max = 162
experience = 650000
fire_resist = -50
cold_resist = 75
lightning_resist = 110
poison_resist = 110
physical_resist = 33

[boss.ai_params]
aggro_radius = 15.0
leash_radius = 50.0
attack_cooldown = 10

[[boss.ai_params.skills]]
skill_id = "melee_attack"
priority = 1
condition = { type = "TargetInRange", range = 2.0 }
cooldown = 0

[[boss.ai_params.skills]]
skill_id = "poison_spray"
priority = 2
condition = { type = "TargetInRange", range = 6.0 }
cooldown = 75

[[boss.phases]]
phase = 1
hp_threshold = 1.0
behavior = "BossPhased"
description = "Normal combat"

[[boss.phases]]
phase = 2
hp_threshold = 0.5
behavior = "BossEnraged"
description = "Enraged at 50% HP, faster attacks"
modifiers = [
    { stat = "attack_speed_modifier", value = 50 },
    { stat = "speed_modifier", value = 30 },
]

[boss.drops]
loot_table = "tc_andariel"
guaranteed_quest_drop = "horadric_malus_quest"
```

```rust
// @id: sd-data-boss-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossFile {
    pub boss: BossDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossDef {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub base_sprite: String,
    pub grid_hitbox_radius: f32,
    pub stats: MonsterStatsByDifficulty,
    pub ai_params: MonsterAiParams,
    pub phases: Vec<BossPhase>,
    pub drops: BossDrops,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossPhase {
    pub phase: u8,
    pub hp_threshold: f32,
    pub behavior: String,
    pub description: String,
    #[serde(default)]
    pub modifiers: Vec<StatValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BossDrops {
    pub loot_table: String,
    #[serde(default)]
    pub guaranteed_quest_drop: Option<String>,
}
```

---

## 4. Competences -- Schemas TOML et structs Rust

### 4.1 Skill actif

**Fichier :** `data/skills/necromancer/bone_spear.toml`

```toml
[skill]
id = "bone_spear"
name = "Bone Spear"
class = "Mortecian"
tree = "Bone"
tree_position = [2, 3]          # colonne, rangee dans l'arbre
required_level = 18
max_level = 20
skill_type = "Active"
target_type = "Direction"
icon_sprite = "ui/skills/necro_bone_spear"

[skill.base]
mana_cost = 7
cooldown_frames = 0              # 0 = pas de cooldown
damage_type = "Magic"
projectile = true
projectile_speed = 12.0
projectile_sprite = "effects/bone_spear"
aoe_radius = 0.0                # 0 = impact simple
piercing = true
pierce_count = -1                # -1 = infini

[skill.per_level]
mana_cost_increment = 0.5
damage_min_base = 16
damage_min_per_level = 16
damage_max_base = 24
damage_max_per_level = 16

[[skill.synergies]]
skill_id = "teeth"
bonus_per_level = 7              # +7% damage par niveau de Teeth

[[skill.synergies]]
skill_id = "bone_wall"
bonus_per_level = 7

[[skill.synergies]]
skill_id = "bone_prison"
bonus_per_level = 7

[skill.prerequisites]
skills_required = ["teeth", "corpse_explosion"]
```

### 4.2 Skill passif

**Fichier :** `data/skills/necromancer/skeleton_mastery.toml`

```toml
[skill]
id = "skeleton_mastery"
name = "Skeleton Mastery"
class = "Mortecian"
tree = "Summoning"
tree_position = [0, 1]
required_level = 1
max_level = 20
skill_type = "Passive"
target_type = "None"
icon_sprite = "ui/skills/necro_skeleton_mastery"

[skill.per_level]
bonus_life_per_level = 8
bonus_damage_per_level = 2
bonus_ar_per_level = 10
bonus_defense_per_level = 5

[skill.prerequisites]
skills_required = ["raise_skeleton"]
```

### 4.3 Aura

**Fichier :** `data/skills/paladin/fanaticism.toml`

```toml
[skill]
id = "fanaticism"
name = "Fanaticism"
class = "CroiseSolaire"
tree = "Offensive"
tree_position = [2, 5]
required_level = 30
max_level = 20
skill_type = "Aura"
target_type = "Self"
icon_sprite = "ui/skills/paladin_fanaticism"

[skill.aura]
radius_base = 6.0
radius_per_level = 0.3
affects = "Party"

[skill.per_level]
enhanced_damage_base = 200
enhanced_damage_per_level = 21
attack_speed_base = 25
attack_speed_per_level = 1
attack_rating_pct_base = 200
attack_rating_pct_per_level = 40

[skill.prerequisites]
skills_required = ["might"]
```

**Struct Rust :**

```rust
// @id: sd-data-skill-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFile {
    pub skill: SkillDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    pub id: String,
    pub name: String,
    pub class: String,
    pub tree: String,
    pub tree_position: [u8; 2],
    pub required_level: u8,
    pub max_level: u8,
    pub skill_type: SkillType,
    pub target_type: SkillTargetType,
    pub icon_sprite: String,
    #[serde(default)]
    pub base: Option<SkillBase>,
    pub per_level: SkillPerLevel,
    #[serde(default)]
    pub synergies: Vec<SkillSynergy>,
    pub prerequisites: SkillPrereqs,
    #[serde(default)]
    pub aura: Option<AuraDef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillType {
    Active,
    Passive,
    Aura,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillTargetType {
    Entity,
    Position,
    Direction,
    Self_,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillBase {
    pub mana_cost: f32,
    pub cooldown_frames: u32,
    pub damage_type: String,
    #[serde(default)]
    pub projectile: bool,
    #[serde(default)]
    pub projectile_speed: f32,
    #[serde(default)]
    pub projectile_sprite: String,
    #[serde(default)]
    pub aoe_radius: f32,
    #[serde(default)]
    pub piercing: bool,
    #[serde(default)]
    pub pierce_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPerLevel {
    #[serde(default)]
    pub mana_cost_increment: Option<f32>,
    #[serde(default)]
    pub damage_min_base: Option<i32>,
    #[serde(default)]
    pub damage_min_per_level: Option<i32>,
    #[serde(default)]
    pub damage_max_base: Option<i32>,
    #[serde(default)]
    pub damage_max_per_level: Option<i32>,
    #[serde(default)]
    pub enhanced_damage_base: Option<i32>,
    #[serde(default)]
    pub enhanced_damage_per_level: Option<i32>,
    #[serde(default)]
    pub attack_speed_base: Option<i32>,
    #[serde(default)]
    pub attack_speed_per_level: Option<i32>,
    #[serde(default)]
    pub attack_rating_pct_base: Option<i32>,
    #[serde(default)]
    pub attack_rating_pct_per_level: Option<i32>,
    #[serde(default)]
    pub bonus_life_per_level: Option<i32>,
    #[serde(default)]
    pub bonus_damage_per_level: Option<i32>,
    #[serde(default)]
    pub bonus_ar_per_level: Option<i32>,
    #[serde(default)]
    pub bonus_defense_per_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSynergy {
    pub skill_id: String,
    pub bonus_per_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPrereqs {
    pub skills_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraDef {
    pub radius_base: f32,
    pub radius_per_level: f32,
    pub affects: String,
}
```

---

## 5. Zones -- Schemas TOML et structs Rust

**Fichier :** `data/zones/act1/blood_moor.toml`

```toml
[zone]
id = "act1/blood_moor"
name = "Blood Moor"
act = 1
is_town = false
has_waypoint = false
music_track = "music/act1_wilderness"
ambient_track = "ambient/wilderness_day"
tileset = "tiles/act1_wilderness"
map_file = "maps/act1/blood_moor.ldtk"

[zone.area_levels]
normal = 1
nightmare = 36
hell = 67

[zone.connections]
east = "act1/rogue_encampment"
west = "act1/cold_plains"
cave = "act1/den_of_evil"

[[zone.spawn_groups]]
monster_id = "fallen"
min_count = 4
max_count = 8
area_center = [20.0, 15.0]
area_radius = 8.0

[[zone.spawn_groups]]
monster_id = "quill_rat"
min_count = 3
max_count = 6
area_center = [35.0, 25.0]
area_radius = 6.0

[[zone.spawn_groups]]
monster_id = "zombie"
min_count = 2
max_count = 5
area_center = [50.0, 10.0]
area_radius = 10.0

[zone.super_uniques]

[zone.champion_density]
packs_per_area = 2
min_pack_size = 3
max_pack_size = 5
affixes_count = 1

[zone.unique_density]
packs_per_area = 1
minions_min = 3
minions_max = 5
affixes_count_min = 1
affixes_count_max = 3
```

**Struct Rust :**

```rust
// @id: sd-data-zone-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneFile {
    pub zone: ZoneDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDef {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub is_town: bool,
    pub has_waypoint: bool,
    pub music_track: String,
    pub ambient_track: String,
    pub tileset: String,
    pub map_file: String,
    pub area_levels: AreaLevels,
    pub connections: ZoneConnections,
    pub spawn_groups: Vec<SpawnGroup>,
    #[serde(default)]
    pub super_uniques: Vec<ZoneSuperUnique>,
    pub champion_density: ChampionDensity,
    pub unique_density: UniqueDensity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaLevels {
    pub normal: u8,
    pub nightmare: u8,
    pub hell: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConnections {
    #[serde(default)]
    pub north: Option<String>,
    #[serde(default)]
    pub south: Option<String>,
    #[serde(default)]
    pub east: Option<String>,
    #[serde(default)]
    pub west: Option<String>,
    #[serde(default)]
    pub cave: Option<String>,
    #[serde(default)]
    pub dungeon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnGroup {
    pub monster_id: String,
    pub min_count: u32,
    pub max_count: u32,
    pub area_center: [f32; 2],
    pub area_radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneSuperUnique {
    pub su_id: String,
    pub position: [f32; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionDensity {
    pub packs_per_area: u32,
    pub min_pack_size: u32,
    pub max_pack_size: u32,
    pub affixes_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueDensity {
    pub packs_per_area: u32,
    pub minions_min: u32,
    pub minions_max: u32,
    pub affixes_count_min: u8,
    pub affixes_count_max: u8,
}
```

---

## 6. Quetes -- Schemas TOML et structs Rust

**Fichier :** `data/quests/act1/den_of_evil.toml`

```toml
[quest]
id = "den_of_evil"
name = "Den of Evil"
act = 1
quest_giver = "akara"
description = "Clear the Den of Evil of all monsters"

[quest.trigger]
type = "NpcTalked"
npc_id = "akara"

[quest.objective]
type = "ClearZone"
zone_id = "act1/den_of_evil"

[quest.completion_trigger]
type = "AllMonstersKilled"
zone_id = "act1/den_of_evil"

[[quest.rewards]]
type = "SkillPoint"
value = 1

[[quest.rewards]]
type = "SkillReset"
value = 1
description = "Free skill/stat reset from Akara"

[quest.dialogue]
accept_script = "scripts/quests/act1/den_of_evil_accept.rhai"
progress_script = "scripts/quests/act1/den_of_evil_progress.rhai"
complete_script = "scripts/quests/act1/den_of_evil_complete.rhai"
```

**Struct Rust :**

```rust
// @id: sd-data-quest-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestFile {
    pub quest: QuestDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDef {
    pub id: String,
    pub name: String,
    pub act: u8,
    pub quest_giver: String,
    pub description: String,
    pub trigger: QuestTriggerDef,
    pub objective: QuestObjectiveDef,
    pub completion_trigger: QuestCompletionDef,
    pub rewards: Vec<QuestRewardDef>,
    pub dialogue: QuestDialogue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestTriggerDef {
    #[serde(rename = "type")]
    pub trigger_type: String,
    #[serde(default)]
    pub npc_id: Option<String>,
    #[serde(default)]
    pub zone_id: Option<String>,
    #[serde(default)]
    pub monster_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjectiveDef {
    #[serde(rename = "type")]
    pub objective_type: String,
    #[serde(default)]
    pub zone_id: Option<String>,
    #[serde(default)]
    pub monster_id: Option<String>,
    #[serde(default)]
    pub item_id: Option<String>,
    #[serde(default)]
    pub npc_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestCompletionDef {
    #[serde(rename = "type")]
    pub completion_type: String,
    #[serde(default)]
    pub zone_id: Option<String>,
    #[serde(default)]
    pub monster_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRewardDef {
    #[serde(rename = "type")]
    pub reward_type: String,
    pub value: i32,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestDialogue {
    pub accept_script: String,
    pub progress_script: String,
    pub complete_script: String,
}
```

---

## 7. Personnages -- Schemas TOML et structs Rust

### 7.1 Stats de base par classe

**Fichier :** `data/classes/necromancer.toml`

```toml
[class]
id = "mortecian"
name = "Mortecian"
d2_name = "Necromancer"

[class.base_stats]
strength = 15
dexterity = 25
vitality = 15
energy = 25
life = 45
mana = 25
stamina = 79

[class.per_level]
life_per_level = 1.5
mana_per_level = 2.0
stamina_per_level = 1.0
life_per_vitality = 2
mana_per_energy = 2.0
stamina_per_vitality = 1

[class.combat]
class_base_ar = 7
base_block_frames = 11

[class.skill_trees]
trees = ["Summoning", "PoisonBone", "Curses"]
skills_per_tree = 10
total_skills = 30

[class.sprite]
base_sprite = "characters/necromancer"
attack_sprite = "characters/necromancer_attack"
cast_sprite = "characters/necromancer_cast"
```

### 7.2 Breakpoints par classe

**Fichier :** `data/config/breakpoints.toml`

```toml
[fcr.mortecian]
breakpoints = [0, 9, 18, 30, 48, 75, 125]
frames =      [15, 14, 13, 12, 11, 10, 9]

[fcr.arcaniste]
breakpoints = [0, 9, 20, 37, 63, 105, 200]
frames =      [13, 12, 11, 10, 9, 8, 7]

[fcr.croise_solaire]
breakpoints = [0, 9, 18, 30, 48, 75]
frames =      [15, 14, 13, 12, 11, 10]

[fhr.mortecian]
breakpoints = [0, 5, 10, 16, 26, 39, 56, 86, 152]
frames =      [13, 12, 11, 10, 9, 8, 7, 6, 5]

[fhr.arcaniste]
breakpoints = [0, 5, 9, 14, 20, 30, 42, 60, 86, 142]
frames =      [15, 14, 13, 12, 11, 10, 9, 8, 7, 6]

[fhr.ravageur]
breakpoints = [0, 7, 15, 27, 48, 86, 200]
frames =      [9, 8, 7, 6, 5, 4, 3]

[fbr.mortecian]
breakpoints = [0, 6, 13, 20, 32, 52, 86, 174]
frames =      [11, 10, 9, 8, 7, 6, 5, 4]

[fbr.croise_solaire]
breakpoints = [0, 13, 32, 86]
frames =      [5, 4, 3, 2]

[ias]
# L'IAS est calcule differemment (depend de l'arme et du skill).
# EIAS = floor(120 * IAS / (120 + IAS))
# Frames = ceil(256 * AnimLength / (256 + EIAS + SIAS - WSM))
```

**Struct Rust :**

```rust
// @id: sd-data-class-def @do: define @role: arpg @layer: 3 @human: miyuk

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassFile {
    pub class: ClassDef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDef {
    pub id: String,
    pub name: String,
    pub d2_name: String,
    pub base_stats: ClassBaseStats,
    pub per_level: ClassPerLevel,
    pub combat: ClassCombat,
    pub skill_trees: ClassSkillTrees,
    pub sprite: ClassSprite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassBaseStats {
    pub strength: i32,
    pub dexterity: i32,
    pub vitality: i32,
    pub energy: i32,
    pub life: i32,
    pub mana: i32,
    pub stamina: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassPerLevel {
    pub life_per_level: f32,
    pub mana_per_level: f32,
    pub stamina_per_level: f32,
    pub life_per_vitality: i32,
    pub mana_per_energy: f32,
    pub stamina_per_vitality: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassCombat {
    pub class_base_ar: i32,
    pub base_block_frames: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSkillTrees {
    pub trees: Vec<String>,
    pub skills_per_tree: u8,
    pub total_skills: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSprite {
    pub base_sprite: String,
    pub attack_sprite: String,
    pub cast_sprite: String,
}

// Breakpoints file

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointsFile {
    pub fcr: HashMap<String, BreakpointTable>,
    pub fhr: HashMap<String, BreakpointTable>,
    pub fbr: HashMap<String, BreakpointTable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakpointTable {
    pub breakpoints: Vec<i32>,
    pub frames: Vec<u32>,
}
```

---

## 8. Regles de validation

### 8.1 Validation des items

| Champ | Regle | Message d'erreur |
|-------|-------|-----------------|
| `id` | Non vide, unique dans le fichier | "Item id must be non-empty and unique" |
| `name` | Non vide | "Item name required" |
| `tier` | "Normal", "Exceptional", ou "Elite" | "Invalid tier" |
| `damage_min` | >= 0, <= damage_max (si present) | "damage_min must be >= 0 and <= damage_max" |
| `damage_max` | >= damage_min (si present) | "damage_max must be >= damage_min" |
| `defense_min` | >= 0 (si present) | "defense_min must be >= 0" |
| `durability` | > 0 | "durability must be > 0" |
| `grid_size` | [1-4, 1-4] | "grid_size must be [1..4, 1..4]" |
| `required_level` | 1..99 | "required_level must be 1..99" |
| `quality_level` | 1..99 | "quality_level must be 1..99" |
| `max_sockets` | 0..6 | "max_sockets must be 0..6" |

### 8.2 Validation des affixes

| Champ | Regle | Message d'erreur |
|-------|-------|-----------------|
| `group` | > 0 | "Affix group must be > 0" |
| `level` | 1..99 | "Affix level must be 1..99" |
| `frequency` | > 0 | "Affix frequency must be > 0" |
| `spawn_on` | Non vide, valeurs valides | "spawn_on must contain valid categories" |
| `properties[].min` | <= max | "Affix property min must be <= max" |

### 8.3 Validation des monstres

| Champ | Regle | Message d'erreur |
|-------|-------|-----------------|
| `ai_archetype` | Valeur valide dans AiArchetype | "Invalid AI archetype" |
| `stats.*.level` | 1..99 | "Monster level must be 1..99" |
| `stats.*.life` | > 0 | "Monster life must be > 0" |
| `stats.*.damage_min` | >= 0, <= damage_max | "damage range invalid" |
| `stats.*.experience` | >= 0 | "experience must be >= 0" |
| `stats.*.*_resist` | -100..200 | "resistance must be -100..200" |
| `drops.loot_table` | Reference valide | "loot_table must reference existing TC" |
| `aggro_radius` | > 0 | "aggro_radius must be > 0" |
| `leash_radius` | >= aggro_radius | "leash_radius must be >= aggro_radius" |

### 8.4 Validation des skills

| Champ | Regle | Message d'erreur |
|-------|-------|-----------------|
| `class` | Valeur valide dans CharacterClass | "Invalid class" |
| `required_level` | 1..30 | "Skill required_level must be 1..30" |
| `max_level` | 1..40 | "max_level must be 1..40" |
| `base.mana_cost` | >= 0 | "mana_cost must be >= 0" |
| `synergies[].skill_id` | Reference valide | "Synergy references non-existent skill" |
| `prerequisites.skills_required` | References valides | "Prerequisite skill not found" |

### 8.5 Validation des zones

| Champ | Regle | Message d'erreur |
|-------|-------|-----------------|
| `act` | 1..5 | "act must be 1..5" |
| `area_levels.normal` | 1..85 | "area_level normal must be 1..85" |
| `area_levels.nightmare` | 36..85 | "area_level nightmare must be 36..85" |
| `area_levels.hell` | 67..85 | "area_level hell must be 67..85" |
| `connections.*` | References de zones valides | "Connection references non-existent zone" |
| `spawn_groups[].monster_id` | Reference de monstre valide | "Invalid monster_id" |
| `spawn_groups[].min_count` | > 0, <= max_count | "min_count invalid" |

### 8.6 Validation des runewords

| Champ | Regle | Message d'erreur |
|-------|-------|-----------------|
| `rune_sequence` | Non vide, toutes les runes existent | "Rune not found in runes.toml" |
| `allowed_types` | Non vide | "allowed_types must not be empty" |
| `required_level` | 1..99 | "required_level must be 1..99" |

### 8.7 Implementation de validation Rust

```rust
// @id: sd-data-validation @do: define @role: arpg @layer: 3 @human: miyuk

pub fn validate_all_data(
    items: &[ItemBaseDef],
    affixes: &AffixTableFile,
    suffixes: &SuffixTableFile,
    monsters: &[MonsterDef],
    skills: &[SkillDef],
    zones: &[ZoneDef],
    runewords: &[RunewordDef],
    runes: &[RuneDef],
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // Valider les items.
    for item in items {
        if item.id.is_empty() {
            errors.push(ValidationError::new("item", &item.id, "id is empty"));
        }
        if item.durability <= 0 {
            errors.push(ValidationError::new(
                "item",
                &item.id,
                "durability must be > 0",
            ));
        }
        if let (Some(min), Some(max)) = (item.damage_min, item.damage_max) {
            if min > max {
                errors.push(ValidationError::new(
                    "item",
                    &item.id,
                    "damage_min > damage_max",
                ));
            }
        }
    }

    // Valider les monstres.
    for monster in monsters {
        if monster.ai_params.leash_radius < monster.ai_params.aggro_radius {
            errors.push(ValidationError::new(
                "monster",
                &monster.id,
                "leash_radius < aggro_radius",
            ));
        }
    }

    // Valider les runewords.
    let rune_ids: HashSet<&str> = runes.iter().map(|r| r.id.as_str()).collect();
    for rw in runewords {
        for rune_id in &rw.rune_sequence {
            let lower = rune_id.to_lowercase();
            if !rune_ids.contains(lower.as_str()) {
                errors.push(ValidationError::new(
                    "runeword",
                    &rw.id,
                    &format!("rune '{}' not found", rune_id),
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub category: String,
    pub id: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(category: &str, id: &str, message: &str) -> Self {
        Self {
            category: category.to_string(),
            id: id.to_string(),
            message: message.to_string(),
        }
    }
}
```

---

*Document redige par Denis, Chef Dev Senior -- Miyukini AI Studio*
*Revision : 2026-02-28 v1.0*
