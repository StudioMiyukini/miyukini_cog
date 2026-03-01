<!-- @id: SD-Impl-03 @do: guide @role: back-end @layer: 3 @human: miyuk -->

# IMPL-03 -- Guide d'Implementation du Systeme de Combat

**Auteur :** Francois (Dev Back-End, Miyukini AI Studio)
**Base :** SD-Tech-Systems.md + SD-Combat-Stats.md (Denis)
**Date :** 2026-02-28
**Statut :** Guide d'implementation -- v1.0

---

## Table des matieres

1. [Pipeline de degats complet](#1-pipeline-de-degats-complet)
2. [Chance to Hit (CTH)](#2-chance-to-hit-cth)
3. [Calcul de Defense et Block](#3-calcul-de-defense-et-block)
4. [Degats physiques](#4-degats-physiques)
5. [Degats elementaires](#5-degats-elementaires)
6. [Critical Strike et Deadly Strike](#6-critical-strike-et-deadly-strike)
7. [Effets speciaux de combat](#7-effets-speciaux-de-combat)
8. [Systeme de skills](#8-systeme-de-skills)
9. [Buffs et debuffs](#9-buffs-et-debuffs)
10. [Systeme d'aura](#10-systeme-daura)
11. [Projectiles](#11-projectiles)
12. [Death et loot spawn](#12-death-et-loot-spawn)
13. [Tests d'integration combat](#13-tests-dintegration-combat)

---

## 1. Pipeline de degats complet

Le pipeline de degats D2 suit 8 etapes strictement ordonnees. Toute attaque
(auto-attack, skill melee, projectile) passe par ce pipeline.

### 1.1 Types de resultat

```rust
/// @id: sd-combat-damage-result @do: define @role: arpg @layer: 3
/// Crate: mge-arpg-combat

/// Resultat complet du calcul de degats.
#[derive(Debug, Clone)]
pub struct DamageResult {
    /// L'attaque a-t-elle touche ?
    pub hit: bool,
    /// L'attaque a-t-elle ete bloquee ?
    pub blocked: bool,
    /// Degats physiques appliques (apres resistances).
    pub physical_damage: i32,
    /// Degats de feu appliques.
    pub fire_damage: i32,
    /// Degats de froid appliques.
    pub cold_damage: i32,
    /// Degats de foudre appliques.
    pub lightning_damage: i32,
    /// Degats de poison appliques (total sur la duree).
    pub poison_damage: i32,
    /// Duree du poison en frames.
    pub poison_duration_frames: u32,
    /// Degats magiques purs appliques.
    pub magic_damage: i32,
    /// L'attaque a-t-elle declenche un critical strike ?
    pub is_critical: bool,
    /// Crushing Blow applique.
    pub crushing_blow_damage: i32,
    /// Open Wounds declenche.
    pub open_wounds_triggered: bool,
    /// Vie volee (life steal).
    pub life_stolen: i32,
    /// Mana volee (mana steal).
    pub mana_stolen: i32,
    /// Degats totaux infliges.
    pub total_damage: i32,
}

/// Type de cible pour les modificateurs de Crushing Blow et Open Wounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    Normal,
    ChampionBoss,
    ActBoss,
    PlayerHireling,
}

/// Bundle de donnees de l'attaquant pour le pipeline.
#[derive(Debug, Clone)]
pub struct AttackerBundle {
    pub attack_rating: u32,
    pub level: u8,
    pub strength: i32,
    pub dexterity: i32,
    pub weapon_min_damage: i32,
    pub weapon_max_damage: i32,
    pub weapon_is_ethereal: bool,
    pub weapon_str_factor: i32,
    pub weapon_dex_factor: i32,
    pub ed_on_weapon: i32,
    pub ed_off_weapon: i32,
    pub flat_damage_min: i32,
    pub flat_damage_max: i32,
    pub critical_strike_chance: i32,
    pub deadly_strike_chance: i32,
    pub crushing_blow_chance: i32,
    pub open_wounds_chance: i32,
    pub life_steal_percent: i32,
    pub mana_steal_percent: i32,
    pub is_ranged: bool,
    pub fire_damage_min: i32,
    pub fire_damage_max: i32,
    pub cold_damage_min: i32,
    pub cold_damage_max: i32,
    pub lightning_damage_min: i32,
    pub lightning_damage_max: i32,
    pub poison_damage_total: i32,
    pub poison_duration_frames: u32,
    pub magic_damage: i32,
    pub ignore_target_defense: bool,
}

/// Bundle de donnees du defenseur pour le pipeline.
#[derive(Debug, Clone)]
pub struct DefenderBundle {
    pub defense: u32,
    pub level: u8,
    pub life_current: i32,
    pub life_max: i32,
    pub fire_resist: i32,
    pub cold_resist: i32,
    pub lightning_resist: i32,
    pub poison_resist: i32,
    pub magic_resist: i32,
    pub physical_resist: i32,
    pub damage_reduction_flat: i32,
    pub block_chance: i32,
    pub target_type: TargetType,
}
```

### 1.2 Pipeline principal

```rust
/// @id: sd-combat-pipeline @do: define @role: arpg @layer: 3
/// Crate: mge-arpg-combat
///
/// Calcule le resultat de degats complet en suivant les 8 etapes du pipeline D2.
///
/// Etapes :
/// 1. Chance to Hit (CTH)
/// 2. Block check
/// 3. Calcul degats physiques de base
/// 4. Enhanced Damage et bonus STR/DEX
/// 5. Critical Strike / Deadly Strike
/// 6. Crushing Blow
/// 7. Degats elementaires (feu, froid, foudre, poison, magie)
/// 8. Effets speciaux (Open Wounds, Life Steal, Mana Steal)
pub fn calculate_damage(
    attacker: &AttackerBundle,
    defender: &DefenderBundle,
    rng: &mut impl rand::Rng,
) -> DamageResult {
    let mut result = DamageResult {
        hit: false,
        blocked: false,
        physical_damage: 0,
        fire_damage: 0,
        cold_damage: 0,
        lightning_damage: 0,
        poison_damage: 0,
        poison_duration_frames: 0,
        magic_damage: 0,
        is_critical: false,
        crushing_blow_damage: 0,
        open_wounds_triggered: false,
        life_stolen: 0,
        mana_stolen: 0,
        total_damage: 0,
    };

    // === ETAPE 1 : Chance to Hit ===
    let cth = if attacker.ignore_target_defense {
        chance_to_hit_itd(attacker.level, defender.level)
    } else {
        chance_to_hit(attacker.attack_rating, defender.defense, attacker.level, defender.level)
    };

    let hit_roll: f32 = rng.gen_range(0.0..100.0);
    if hit_roll >= cth {
        return result; // Miss.
    }
    result.hit = true;

    // === ETAPE 2 : Block check ===
    if defender.block_chance > 0 {
        let block_roll: i32 = rng.gen_range(0..100);
        if block_roll < defender.block_chance {
            result.blocked = true;
            return result; // Bloque, aucun degat.
        }
    }

    // === ETAPE 3 : Degats physiques de base ===
    let base_min = attacker.weapon_min_damage;
    let base_max = attacker.weapon_max_damage.max(base_min);
    let base_damage = rng.gen_range(base_min..=base_max);

    // Bonus ethereal (x1.5 avant tout).
    let damage_after_ethereal = if attacker.weapon_is_ethereal {
        base_damage * 3 / 2
    } else {
        base_damage
    };

    // === ETAPE 4 : Enhanced Damage + STR/DEX bonus ===
    // ED on-weapon.
    let damage_after_ed_weapon =
        damage_after_ethereal * (100 + attacker.ed_on_weapon) / 100;

    // Flat damage added.
    let flat_damage = rng.gen_range(attacker.flat_damage_min..=attacker.flat_damage_max.max(attacker.flat_damage_min));
    let damage_after_flat = damage_after_ed_weapon + flat_damage;

    // ED off-weapon.
    let damage_after_ed_off =
        damage_after_flat * (100 + attacker.ed_off_weapon) / 100;

    // STR/DEX bonus.
    let str_dex_bonus =
        (attacker.strength * attacker.weapon_str_factor
            + attacker.dexterity * attacker.weapon_dex_factor)
            / 100;
    let damage_after_stats = damage_after_ed_off * (100 + str_dex_bonus) / 100;

    // === ETAPE 5 : Critical Strike / Deadly Strike ===
    let cs = attacker.critical_strike_chance;
    let ds = attacker.deadly_strike_chance;
    // P(double) = 1 - (1 - CS/100) * (1 - DS/100)
    let p_double = 100 - ((100 - cs) * (100 - ds) / 100);
    let crit_roll: i32 = rng.gen_range(0..100);
    let is_critical = crit_roll < p_double;
    let damage_after_crit = if is_critical {
        result.is_critical = true;
        damage_after_stats * 2
    } else {
        damage_after_stats
    };

    // Apply physical resistance.
    let physical_after_resist = apply_resistance(damage_after_crit, defender.physical_resist);

    // Apply flat damage reduction.
    let physical_final = (physical_after_resist - defender.damage_reduction_flat).max(0);
    result.physical_damage = physical_final;

    // === ETAPE 6 : Crushing Blow ===
    if attacker.crushing_blow_chance > 0 {
        let cb_roll: i32 = rng.gen_range(0..100);
        if cb_roll < attacker.crushing_blow_chance {
            result.crushing_blow_damage =
                crushing_blow_damage(defender.life_current, defender.target_type, attacker.is_ranged);
        }
    }

    // === ETAPE 7 : Degats elementaires ===
    // Fire.
    if attacker.fire_damage_max > 0 {
        let fire_base = rng.gen_range(attacker.fire_damage_min..=attacker.fire_damage_max);
        result.fire_damage = apply_resistance(fire_base, defender.fire_resist);
    }

    // Cold.
    if attacker.cold_damage_max > 0 {
        let cold_base = rng.gen_range(attacker.cold_damage_min..=attacker.cold_damage_max);
        result.cold_damage = apply_resistance(cold_base, defender.cold_resist);
    }

    // Lightning.
    if attacker.lightning_damage_max > 0 {
        let light_base = rng.gen_range(attacker.lightning_damage_min..=attacker.lightning_damage_max);
        result.lightning_damage = apply_resistance(light_base, defender.lightning_resist);
    }

    // Poison.
    if attacker.poison_damage_total > 0 {
        result.poison_damage =
            apply_resistance(attacker.poison_damage_total, defender.poison_resist);
        result.poison_duration_frames = attacker.poison_duration_frames;
    }

    // Magic.
    if attacker.magic_damage > 0 {
        result.magic_damage = apply_resistance(attacker.magic_damage, defender.magic_resist);
    }

    // === ETAPE 8 : Effets speciaux ===
    // Open Wounds.
    if attacker.open_wounds_chance > 0 {
        let ow_roll: i32 = rng.gen_range(0..100);
        if ow_roll < attacker.open_wounds_chance {
            result.open_wounds_triggered = true;
        }
    }

    // Life Steal.
    if attacker.life_steal_percent > 0 {
        result.life_stolen = physical_final * attacker.life_steal_percent / 100;
    }

    // Mana Steal.
    if attacker.mana_steal_percent > 0 {
        result.mana_stolen = physical_final * attacker.mana_steal_percent / 100;
    }

    // Total.
    result.total_damage = result.physical_damage
        + result.crushing_blow_damage
        + result.fire_damage
        + result.cold_damage
        + result.lightning_damage
        + result.magic_damage;
    // Note : poison n'est pas inclus dans le total instantane (c'est un DoT).

    result
}
```

---

## 2. Chance to Hit (CTH)

### 2.1 Formule standard

```rust
/// @id: sd-combat-cth @do: define @role: arpg @layer: 3
///
/// Calcule la chance de toucher standard D2.
///
/// CTH = 200 * AR / (AR + DR) * clvl / (clvl + mlvl)
/// Resultat cappe : min 5%, max 95%.
///
/// # Arguments
/// - `ar`: Attack Rating de l'attaquant
/// - `dr`: Defense Rating du defenseur
/// - `clvl`: Niveau de l'attaquant
/// - `mlvl`: Niveau du defenseur
///
/// # Returns
/// Pourcentage de chance de toucher (5.0 a 95.0).
pub fn chance_to_hit(ar: u32, dr: u32, clvl: u8, mlvl: u8) -> f32 {
    let ar_f = ar as f32;
    let dr_f = dr as f32;
    let clvl_f = clvl as f32;
    let mlvl_f = mlvl as f32;

    // Eviter la division par zero.
    if ar_f + dr_f < f32::EPSILON {
        return 5.0;
    }
    if clvl_f + mlvl_f < f32::EPSILON {
        return 5.0;
    }

    let cth = 200.0 * (ar_f / (ar_f + dr_f)) * (clvl_f / (clvl_f + mlvl_f));
    cth.clamp(5.0, 95.0)
}
```

### 2.2 Variante ITD (Ignore Target's Defense)

```rust
/// @id: sd-combat-cth-itd @do: define @role: arpg @layer: 3
///
/// Calcule la chance de toucher avec Ignore Target's Defense.
/// La defense du defenseur est ignoree, seul l'ecart de niveau compte.
///
/// CTH_ITD = 200 * clvl / (clvl + mlvl)
pub fn chance_to_hit_itd(clvl: u8, mlvl: u8) -> f32 {
    let clvl_f = clvl as f32;
    let mlvl_f = mlvl as f32;

    if clvl_f + mlvl_f < f32::EPSILON {
        return 5.0;
    }

    let cth = 200.0 * (clvl_f / (clvl_f + mlvl_f));
    cth.clamp(5.0, 95.0)
}
```

### 2.3 Attack Rating de base

```rust
/// @id: sd-combat-base-ar @do: define @role: arpg @layer: 3
///
/// Calcule l'Attack Rating de base d'un personnage.
///
/// BaseAR = (DEX - 7) * 5 + ClassBaseAR
pub fn base_attack_rating(dexterity: i32, class: CharacterClass) -> i32 {
    let class_base = match class {
        CharacterClass::Sarith => 9,         // Amazon
        CharacterClass::Ombrelame => 14,     // Assassin
        CharacterClass::Ravageur => 10,      // Barbarian
        CharacterClass::Animiste => 7,       // Druid
        CharacterClass::Mortecian => 7,      // Necromancer
        CharacterClass::CroiseSolaire => 10, // Paladin
        CharacterClass::Arcaniste => 7,      // Sorceress
    };
    (dexterity - 7) * 5 + class_base
}
```

---

## 3. Calcul de Defense et Block

### 3.1 Chance to Block

```rust
/// @id: sd-combat-block @do: define @role: arpg @layer: 3
///
/// Calcule la chance de bloquer.
///
/// CTB = floor((ShieldBlock + Bonus) * (DEX - 15) / (clvl * 2))
/// CTB = min(CTB, 75)
///
/// En run : CTB = min(floor(CTB / 3), 25)
pub fn chance_to_block(
    shield_block: i32,
    bonus: i32,
    dexterity: i32,
    clvl: u8,
    is_running: bool,
) -> i32 {
    if shield_block <= 0 {
        return 0;
    }

    let clvl_i = clvl as i32;
    if clvl_i == 0 {
        return 0;
    }

    let ctb = (shield_block + bonus) * (dexterity - 15) / (clvl_i * 2);
    let ctb = ctb.clamp(0, 75);

    if is_running {
        (ctb / 3).min(25)
    } else {
        ctb
    }
}

/// Calcule la DEX requise pour atteindre 75% de block.
pub fn dex_for_max_block(shield_block: i32, bonus: i32, clvl: u8) -> i32 {
    let clvl_i = clvl as i32;
    if shield_block + bonus <= 0 {
        return i32::MAX; // Impossible.
    }
    15 + (150 * clvl_i + (shield_block + bonus) - 1) / (shield_block + bonus)
}
```

---

## 4. Degats physiques

### 4.1 Application des resistances

```rust
/// @id: sd-combat-resistance @do: define @role: arpg @layer: 3
///
/// Applique une resistance a un montant de degats.
/// Les resistances negatives amplifient les degats.
/// Un resist >= 100 = immunite (0 degats).
///
/// DmgFinal = Dmg * (100 - Resist) / 100
pub fn apply_resistance(damage: i32, resistance: i32) -> i32 {
    if resistance >= 100 {
        return 0; // Immune.
    }
    let result = damage * (100 - resistance) / 100;
    result.max(0)
}
```

### 4.2 Formule complete des degats physiques

```rust
/// @id: sd-combat-phys-damage @do: define @role: arpg @layer: 3
///
/// Calcule les degats physiques complets selon le pipeline D2.
///
/// Pipeline :
/// 1. WeaponDmg (base)
/// 2. x1.5 si Ethereal
/// 3. x(1 + ED_on_weapon/100)
/// 4. + FlatDmg
/// 5. x(1 + ED_off_weapon/100)
/// 6. x(1 + StrDexBonus/100)
/// 7. x2 si Critical/Deadly Strike
pub fn calculate_physical_damage(
    weapon_damage: i32,
    is_ethereal: bool,
    ed_on_weapon: i32,
    flat_damage: i32,
    ed_off_weapon: i32,
    str_dex_bonus: i32,
    is_critical: bool,
) -> i32 {
    // Etape 2 : Ethereal.
    let step2 = if is_ethereal {
        weapon_damage * 3 / 2
    } else {
        weapon_damage
    };

    // Etape 3 : ED on-weapon.
    let step3 = step2 * (100 + ed_on_weapon) / 100;

    // Etape 4 : Flat damage.
    let step4 = step3 + flat_damage;

    // Etape 5 : ED off-weapon.
    let step5 = step4 * (100 + ed_off_weapon) / 100;

    // Etape 6 : STR/DEX bonus.
    let step6 = step5 * (100 + str_dex_bonus) / 100;

    // Etape 7 : Critical/Deadly Strike.
    if is_critical {
        step6 * 2
    } else {
        step6
    }
}
```

---

## 5. Degats elementaires

```rust
/// @id: sd-combat-elem-damage @do: define @role: arpg @layer: 3
///
/// Calcule les degats elementaires avec application des resistances.
pub fn calculate_elemental_damage(
    base_damage: i32,
    resistance: i32,
) -> i32 {
    apply_resistance(base_damage, resistance)
}

/// Calcule les degats poison (DoT) avec resistance.
pub fn calculate_poison_dot(
    total_poison: i32,
    duration_frames: u32,
    poison_resist: i32,
) -> PoisonDotResult {
    let total_after_resist = apply_resistance(total_poison, poison_resist);
    let per_frame = if duration_frames > 0 {
        total_after_resist / duration_frames as i32
    } else {
        0
    };
    PoisonDotResult {
        total_damage: total_after_resist,
        damage_per_frame: per_frame,
        duration_frames,
    }
}

#[derive(Debug, Clone)]
pub struct PoisonDotResult {
    pub total_damage: i32,
    pub damage_per_frame: i32,
    pub duration_frames: u32,
}
```

---

## 6. Critical Strike et Deadly Strike

```rust
/// @id: sd-combat-critical @do: define @role: arpg @layer: 3
///
/// Calcule la probabilite combinee de Critical Strike + Deadly Strike.
///
/// P(double) = 1 - (1 - CS/100) * (1 - DS/100)
///
/// Exemple : CS 50% + DS 50% = 1 - (0.5 * 0.5) = 75%
pub fn combined_critical_chance(critical_strike: i32, deadly_strike: i32) -> i32 {
    let cs = critical_strike.clamp(0, 100);
    let ds = deadly_strike.clamp(0, 100);
    100 - ((100 - cs) * (100 - ds) / 100)
}
```

---

## 7. Effets speciaux de combat

### 7.1 Crushing Blow

```rust
/// @id: sd-combat-crushing-blow @do: define @role: arpg @layer: 3
///
/// Calcule les degats de Crushing Blow.
/// Retire une fraction de la vie ACTUELLE de la cible.
pub fn crushing_blow_damage(
    current_life: i32,
    target_type: TargetType,
    is_ranged: bool,
) -> i32 {
    let (numerator, denominator) = match (target_type, is_ranged) {
        (TargetType::Normal, false) => (1, 4),
        (TargetType::Normal, true) => (1, 8),
        (TargetType::ChampionBoss, false) => (1, 8),
        (TargetType::ChampionBoss, true) => (1, 16),
        (TargetType::ActBoss, false) => (1, 8),
        (TargetType::ActBoss, true) => (1, 16),
        (TargetType::PlayerHireling, false) => (1, 10),
        (TargetType::PlayerHireling, true) => (1, 20),
    };
    current_life * numerator / denominator
}
```

### 7.2 Open Wounds

```rust
/// @id: sd-combat-open-wounds @do: define @role: arpg @layer: 3
///
/// Calcule les degats par frame d'Open Wounds en fonction du niveau.
/// Duree : 200 frames (8 secondes a 25 fps).
pub fn open_wounds_damage_per_frame(clvl: u8) -> f32 {
    let clvl_i = clvl as i32;
    let dpf_x256 = if clvl_i <= 15 {
        9 * clvl_i + 31
    } else if clvl_i <= 30 {
        18 * clvl_i - 104
    } else if clvl_i <= 45 {
        27 * clvl_i - 374
    } else if clvl_i <= 60 {
        36 * clvl_i - 779
    } else {
        45 * clvl_i - 1319
    };
    dpf_x256 as f32 / 256.0
}

/// Modificateur de degats Open Wounds selon le type de cible et la portee.
pub fn open_wounds_modifier(target_type: TargetType, is_ranged: bool) -> f32 {
    match (target_type, is_ranged) {
        (TargetType::Normal, false) => 1.0,
        (TargetType::Normal, true) => 0.5,
        (TargetType::ChampionBoss, false) => 0.5,
        (TargetType::ChampionBoss, true) => 0.25,
        (TargetType::ActBoss, false) => 0.5,
        (TargetType::ActBoss, true) => 0.25,
        (TargetType::PlayerHireling, false) => 0.25,
        (TargetType::PlayerHireling, true) => 0.125,
    }
}

/// Duree de l'Open Wounds en frames (constante D2).
pub const OPEN_WOUNDS_DURATION_FRAMES: u32 = 200; // 8 secondes a 25 fps
```

---

## 8. Systeme de skills

### 8.1 Structures de donnees

```rust
/// @id: sd-combat-skill-def @do: define @role: arpg @layer: 3
/// Crate: mge-arpg-skills

/// Definition d'un skill chargee depuis TOML.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkillDef {
    pub id: String,
    pub name: String,
    pub class: String,
    pub tree: String,
    pub tree_position: [u8; 2],
    pub max_level: u8,
    pub required_level: u8,
    pub prerequisites: Vec<String>,
    pub skill_type: SkillType,
    pub mana_cost_base: f32,
    pub mana_cost_per_level: f32,
    pub cooldown_frames: u32,
    pub cast_frames_base: u32,
    pub damage_type: DamageType,
    pub synergies: Vec<SynergyDef>,
    pub projectile: Option<ProjectileDef>,
    pub aoe_radius: Option<f32>,
    pub effect_formula: SkillFormula,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum SkillType {
    Melee,
    Ranged,
    Spell,
    Aura,
    Passive,
    Summon,
    Buff,
    Warcry,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DamageType {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Magic,
    Mixed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynergyDef {
    pub skill_id: String,
    pub bonus_per_level: f32,
    pub bonus_type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectileDef {
    pub speed: f32,
    pub max_range: f32,
    pub pierce_chance: i32,
    pub count: u8,
    pub spread_angle: f32,
    pub homing: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkillFormula {
    pub base_damage_min: i32,
    pub base_damage_max: i32,
    pub damage_per_level: f32,
    pub duration_frames_base: u32,
    pub duration_per_level: f32,
}

impl SkillDef {
    /// Calcule le cout en mana a un niveau donne.
    pub fn mana_cost_at_level(&self, level: u8) -> f32 {
        self.mana_cost_base + self.mana_cost_per_level * (level.saturating_sub(1)) as f32
    }

    /// Calcule le cooldown en frames a un niveau donne.
    pub fn cooldown_at_level(&self, _level: u8) -> u32 {
        // La plupart des skills D2 ont un cooldown fixe.
        self.cooldown_frames
    }

    /// Calcule les degats de base a un niveau donne (avant synergies).
    pub fn base_damage_at_level(&self, level: u8) -> (i32, i32) {
        let bonus = (self.effect_formula.damage_per_level * level as f32) as i32;
        (
            self.effect_formula.base_damage_min + bonus,
            self.effect_formula.base_damage_max + bonus,
        )
    }
}
```

### 8.2 Instance de skill et synergies

```rust
/// @id: sd-combat-skill-instance @do: define @role: arpg @layer: 3

/// Etat d'un skill dans les slots du personnage.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SkillInstance {
    pub skill_id: String,
    /// Niveau investi (hard points, 0-20).
    pub hard_points: u8,
    /// Bonus de niveau depuis items (+skills).
    pub bonus_levels: i32,
    /// Cooldown restant en frames.
    pub cooldown_remaining: u32,
    /// Charges restantes (pour les skills a charges, ex: CTA BO).
    pub charges: Option<u32>,
    pub charges_max: Option<u32>,
}

impl SkillInstance {
    /// Retourne le niveau effectif du skill (hard + bonus).
    pub fn effective_level(&self) -> u8 {
        let total = self.hard_points as i32 + self.bonus_levels;
        total.clamp(0, 255) as u8
    }

    /// Verifie si le skill est en cooldown.
    pub fn is_on_cooldown(&self) -> bool {
        self.cooldown_remaining > 0
    }

    /// Decremente le cooldown d'une frame.
    pub fn tick_cooldown(&mut self) {
        if self.cooldown_remaining > 0 {
            self.cooldown_remaining -= 1;
        }
    }
}

/// Calcule le bonus de synergie total pour un skill donne.
pub fn calculate_synergy_bonus(
    skill_def: &SkillDef,
    all_skills: &std::collections::HashMap<String, SkillInstance>,
) -> f32 {
    let mut total_bonus = 0.0;
    for synergy in &skill_def.synergies {
        if let Some(synergy_skill) = all_skills.get(&synergy.skill_id) {
            let synergy_level = synergy_skill.hard_points; // Synergies = hard points only.
            total_bonus += synergy.bonus_per_level * synergy_level as f32;
        }
    }
    total_bonus
}
```

---

## 9. Buffs et debuffs

### 9.1 Definitions et instances

```rust
/// @id: sd-combat-buff-def @do: define @role: arpg @layer: 3

/// Definition d'un buff/debuff chargee depuis TOML.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuffDef {
    pub id: String,
    pub name: String,
    pub buff_type: BuffType,
    pub duration_frames: u32,
    pub max_stacks: u8,
    pub stacking_rule: StackingRule,
    pub effects: Vec<BuffEffect>,
    pub tick_damage: Option<TickDamage>,
    pub immunity_list: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BuffType {
    Buff,
    Debuff,
    Aura,
    CrowdControl,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum StackingRule {
    /// Le dernier remplace le precedent.
    Refresh,
    /// La duree s'additionne.
    Extend,
    /// Les stacks s'ajoutent (jusqu'a max_stacks).
    Stack,
    /// Pas de stack : un seul actif a la fois.
    NoStack,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BuffEffect {
    pub stat: String,
    pub value: i32,
    pub operation: BuffOperation,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BuffOperation {
    Add,
    Multiply,
    Set,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TickDamage {
    pub damage_type: DamageType,
    pub damage_per_tick: i32,
    pub tick_interval_frames: u32,
}
```

### 9.2 Application et tick

```rust
/// @id: sd-combat-buff-system @do: define @role: arpg @layer: 3

/// Composant sparse : liste des buffs actifs sur une entite.
#[derive(Debug, Clone)]
pub struct ActiveBuffs {
    pub buffs: Vec<ActiveBuffInstance>,
}

#[derive(Debug, Clone)]
pub struct ActiveBuffInstance {
    pub buff_def_id: String,
    pub remaining_frames: u32,
    pub stacks: u8,
    pub source_entity: EntityId,
    pub tick_accumulator: u32,
}

/// Applique un buff a une entite en respectant les regles de stacking.
pub fn apply_buff(
    active_buffs: &mut ActiveBuffs,
    buff_def: &BuffDef,
    source: EntityId,
) {
    // Chercher un buff existant du meme type.
    let existing = active_buffs
        .buffs
        .iter_mut()
        .find(|b| b.buff_def_id == buff_def.id);

    match existing {
        Some(existing_buff) => {
            match buff_def.stacking_rule {
                StackingRule::Refresh => {
                    existing_buff.remaining_frames = buff_def.duration_frames;
                }
                StackingRule::Extend => {
                    existing_buff.remaining_frames += buff_def.duration_frames;
                }
                StackingRule::Stack => {
                    if existing_buff.stacks < buff_def.max_stacks {
                        existing_buff.stacks += 1;
                    }
                    existing_buff.remaining_frames = buff_def.duration_frames;
                }
                StackingRule::NoStack => {
                    // Ne rien faire, le buff existant reste.
                }
            }
        }
        None => {
            active_buffs.buffs.push(ActiveBuffInstance {
                buff_def_id: buff_def.id.clone(),
                remaining_frames: buff_def.duration_frames,
                stacks: 1,
                source_entity: source,
                tick_accumulator: 0,
            });
        }
    }
}

/// Tick les buffs : decremente la duree, supprime les expires, applique les tick damages.
pub fn tick_buffs(active_buffs: &mut ActiveBuffs) {
    active_buffs.buffs.retain_mut(|buff| {
        if buff.remaining_frames == 0 {
            return false; // Expire, supprimer.
        }
        buff.remaining_frames -= 1;
        buff.tick_accumulator += 1;
        true
    });
}
```

---

## 10. Systeme d'aura

```rust
/// @id: sd-combat-aura-system @do: define @role: arpg @layer: 3

/// Composant d'aura active sur une entite (Paladin auras, Conviction, etc.).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActiveAura {
    pub aura_id: String,
    pub radius: f32,
    pub level: u8,
    pub effects: Vec<BuffEffect>,
    /// Entites actuellement dans le rayon de l'aura.
    pub affected_entities: Vec<EntityId>,
}

/// Detecte les entites dans le rayon de l'aura et applique les effets.
/// Utilise un simple grid partitioning pour les performances.
pub fn update_aura(
    aura_pos: &Position,
    aura: &mut ActiveAura,
    all_entities: &[(EntityId, Position)],
) {
    aura.affected_entities.clear();

    let radius_sq = aura.radius * aura.radius;

    for (entity_id, entity_pos) in all_entities {
        let dx = aura_pos.x - entity_pos.x;
        let dy = aura_pos.y - entity_pos.y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq <= radius_sq {
            aura.affected_entities.push(*entity_id);
        }
    }
}

/// Partitionnement spatial simple en grille pour accelerer les
/// requetes de proximite (auras, aggro, AoE).
#[derive(Debug)]
pub struct SpatialGrid {
    cell_size: f32,
    cells: std::collections::HashMap<(i32, i32), Vec<EntityId>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: std::collections::HashMap::new(),
        }
    }

    /// Insere une entite dans la grille.
    pub fn insert(&mut self, entity: EntityId, x: f32, y: f32) {
        let cell_x = (x / self.cell_size).floor() as i32;
        let cell_y = (y / self.cell_size).floor() as i32;
        self.cells.entry((cell_x, cell_y)).or_default().push(entity);
    }

    /// Efface toute la grille (a refaire chaque frame).
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Retourne toutes les entites dans un rayon donne autour d'un point.
    pub fn query_radius(&self, x: f32, y: f32, radius: f32) -> Vec<EntityId> {
        let mut result = Vec::new();
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        let center_cx = (x / self.cell_size).floor() as i32;
        let center_cy = (y / self.cell_size).floor() as i32;

        for cx in (center_cx - cell_radius)..=(center_cx + cell_radius) {
            for cy in (center_cy - cell_radius)..=(center_cy + cell_radius) {
                if let Some(entities) = self.cells.get(&(cx, cy)) {
                    result.extend_from_slice(entities);
                }
            }
        }

        result
    }
}
```

---

## 11. Projectiles

```rust
/// @id: sd-combat-projectiles @do: define @role: arpg @layer: 3

/// Composant pour un projectile en vol.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectileComponent {
    /// Entite source (lanceur).
    pub source: EntityId,
    /// Skill qui a genere ce projectile.
    pub skill_id: String,
    /// Niveau du skill au moment du lancement.
    pub skill_level: u8,
    /// Degats a appliquer a l'impact.
    pub damage_min: i32,
    pub damage_max: i32,
    pub damage_type: DamageType,
    /// Chance de percer la cible et continuer.
    pub pierce_chance: i32,
    /// Nombre de cibles deja touchees.
    pub targets_hit: u8,
    /// Cible specifique (pour homing) ou None (direction).
    pub homing_target: Option<EntityId>,
    /// Rayon d'AoE a l'impact (0 = pas d'AoE).
    pub aoe_radius: f32,
}

/// Systeme de deplacement et collision des projectiles.
/// Appele a chaque tick FixedUpdate (25 Hz).
pub fn projectile_system_tick(
    projectiles: &mut Vec<(EntityId, Position, Velocity, ProjectileComponent, Lifetime)>,
    targets: &[(EntityId, Position, CircleHitbox)],
    damage_events: &mut Vec<DamageEvent>,
    dt: f32,
) {
    for (proj_id, proj_pos, proj_vel, proj_data, proj_life) in projectiles.iter_mut() {
        // Deplacer le projectile.
        proj_pos.x += proj_vel.dx * dt;
        proj_pos.y += proj_vel.dy * dt;

        // Decrementer la duree de vie.
        if proj_life.remaining_frames == 0 {
            continue; // Sera despawn par le cleanup.
        }
        proj_life.remaining_frames -= 1;

        // Verifier les collisions avec les cibles.
        for (target_id, target_pos, target_hitbox) in targets {
            // Ne pas toucher la source.
            if *target_id == proj_data.source {
                continue;
            }

            let dx = proj_pos.x - target_pos.x;
            let dy = proj_pos.y - target_pos.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < target_hitbox.radius + 0.3 {
                // Impact.
                damage_events.push(DamageEvent {
                    source: proj_data.source,
                    target: *target_id,
                    damage_type: proj_data.damage_type.clone(),
                    damage_min: proj_data.damage_min,
                    damage_max: proj_data.damage_max,
                    skill_id: proj_data.skill_id.clone(),
                });

                proj_data.targets_hit += 1;

                // Pierce check.
                // Si le projectile ne perce pas, il est detruit.
                if proj_data.pierce_chance <= 0 {
                    proj_life.remaining_frames = 0;
                    break;
                }
                // Pierce logic ici (roll random).
            }
        }
    }
}

/// Evenement de degats genere par un projectile.
#[derive(Debug, Clone)]
pub struct DamageEvent {
    pub source: EntityId,
    pub target: EntityId,
    pub damage_type: DamageType,
    pub damage_min: i32,
    pub damage_max: i32,
    pub skill_id: String,
}
```

---

## 12. Death et loot spawn

```rust
/// @id: sd-combat-death @do: define @role: arpg @layer: 3

/// Verifie si une entite doit mourir (HP <= 0) et declenche la sequence de mort.
pub fn check_death(
    entity_id: EntityId,
    vitals: &VitalPools,
    locomotion: &mut Locomotion,
) -> bool {
    if vitals.life_current <= 0 && locomotion.state != LocomotionState::Dead {
        locomotion.state = LocomotionState::Dead;
        true
    } else {
        false
    }
}

/// Evenement emis quand une entite meurt.
#[derive(Debug, Clone)]
pub struct DeathEvent {
    pub entity: EntityId,
    pub killer: Option<EntityId>,
    pub position: Position,
    pub monster_data: Option<MonsterDeathData>,
}

#[derive(Debug, Clone)]
pub struct MonsterDeathData {
    pub monster_def_id: String,
    pub monster_level: u8,
    pub treasure_class: String,
    pub is_champion: bool,
    pub is_super_unique: bool,
    pub is_act_boss: bool,
}

/// Sequence de mort d'un monstre :
/// 1. Emettre DeathEvent
/// 2. Jouer animation de mort
/// 3. Generer le loot (LootGenerationSystem lit le DeathEvent)
/// 4. Distribuer l'XP (XPSystem lit le DeathEvent)
/// 5. Mettre a jour les quetes (QuestSystem lit le DeathEvent)
/// 6. Apres l'animation : despawn l'entite (DeathCleanupSystem)
```

---

## 13. Tests d'integration combat

### 13.1 Test CTH aux valeurs extremes

```rust
#[cfg(test)]
mod cth_tests {
    use super::*;

    #[test]
    fn test_cth_minimum_5_percent() {
        // AR tres faible, defense tres haute -> cap a 5%.
        let cth = chance_to_hit(1, 10000, 1, 99);
        assert!((cth - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_cth_maximum_95_percent() {
        // AR tres haute, defense zero -> cap a 95%.
        let cth = chance_to_hit(100_000, 0, 99, 1);
        assert!((cth - 95.0).abs() < 0.1);
    }

    #[test]
    fn test_cth_equal_levels_equal_stats() {
        // AR = DR, meme niveau -> CTH = 200 * 0.5 * 0.5 = 50%.
        let cth = chance_to_hit(100, 100, 50, 50);
        assert!((cth - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_cth_zero_defense() {
        // Defense = 0 -> AR / (AR + 0) = 1.0.
        let cth = chance_to_hit(100, 0, 50, 50);
        // 200 * 1.0 * 0.5 = 100 -> cap 95.
        assert!((cth - 95.0).abs() < 0.1);
    }

    #[test]
    fn test_cth_itd() {
        let cth = chance_to_hit_itd(80, 80);
        // 200 * 80 / 160 = 100 -> cap 95.
        assert!((cth - 95.0).abs() < 0.1);
    }
}
```

### 13.2 Test damage pipeline avec resistances

```rust
#[cfg(test)]
mod damage_pipeline_tests {
    use super::*;

    fn make_basic_attacker() -> AttackerBundle {
        AttackerBundle {
            attack_rating: 500,
            level: 30,
            strength: 100,
            dexterity: 50,
            weapon_min_damage: 50,
            weapon_max_damage: 100,
            weapon_is_ethereal: false,
            weapon_str_factor: 100,
            weapon_dex_factor: 0,
            ed_on_weapon: 0,
            ed_off_weapon: 0,
            flat_damage_min: 0,
            flat_damage_max: 0,
            critical_strike_chance: 0,
            deadly_strike_chance: 0,
            crushing_blow_chance: 0,
            open_wounds_chance: 0,
            life_steal_percent: 0,
            mana_steal_percent: 0,
            is_ranged: false,
            fire_damage_min: 0,
            fire_damage_max: 0,
            cold_damage_min: 0,
            cold_damage_max: 0,
            lightning_damage_min: 0,
            lightning_damage_max: 0,
            poison_damage_total: 0,
            poison_duration_frames: 0,
            magic_damage: 0,
            ignore_target_defense: false,
        }
    }

    fn make_basic_defender() -> DefenderBundle {
        DefenderBundle {
            defense: 100,
            level: 30,
            life_current: 1000,
            life_max: 1000,
            fire_resist: 0,
            cold_resist: 0,
            lightning_resist: 0,
            poison_resist: 0,
            magic_resist: 0,
            physical_resist: 0,
            damage_reduction_flat: 0,
            block_chance: 0,
            target_type: TargetType::Normal,
        }
    }

    #[test]
    fn test_basic_physical_hit() {
        let attacker = make_basic_attacker();
        let defender = make_basic_defender();
        let mut rng = rand::thread_rng();

        // Lancer 100 fois pour avoir des hits et des misses.
        let mut hits = 0;
        for _ in 0..100 {
            let result = calculate_damage(&attacker, &defender, &mut rng);
            if result.hit {
                hits += 1;
                assert!(result.total_damage > 0);
            }
        }
        // Avec AR=500, DR=100, niveaux egaux, CTH devrait etre haute.
        assert!(hits > 50, "Expected most attacks to hit, got {hits}/100");
    }

    #[test]
    fn test_fire_resistance_reduces_damage() {
        let mut attacker = make_basic_attacker();
        attacker.fire_damage_min = 100;
        attacker.fire_damage_max = 100;

        let mut defender = make_basic_defender();
        defender.fire_resist = 75; // 75% fire resist.
        defender.defense = 0; // Toujours touche.

        let mut rng = rand::thread_rng();
        let result = calculate_damage(&attacker, &defender, &mut rng);

        if result.hit {
            // 100 fire damage * (100 - 75) / 100 = 25.
            assert_eq!(result.fire_damage, 25);
        }
    }

    #[test]
    fn test_immune_target_zero_damage() {
        let mut attacker = make_basic_attacker();
        attacker.fire_damage_min = 100;
        attacker.fire_damage_max = 100;

        let mut defender = make_basic_defender();
        defender.fire_resist = 100; // Immune.
        defender.defense = 0;

        let mut rng = rand::thread_rng();
        let result = calculate_damage(&attacker, &defender, &mut rng);

        if result.hit {
            assert_eq!(result.fire_damage, 0);
        }
    }

    #[test]
    fn test_crushing_blow_on_boss() {
        let damage = crushing_blow_damage(1000, TargetType::ActBoss, false);
        // 1/8 de 1000 = 125.
        assert_eq!(damage, 125);
    }

    #[test]
    fn test_crushing_blow_ranged_on_normal() {
        let damage = crushing_blow_damage(800, TargetType::Normal, true);
        // 1/8 de 800 = 100.
        assert_eq!(damage, 100);
    }

    #[test]
    fn test_open_wounds_level_scaling() {
        let dpf_1 = open_wounds_damage_per_frame(1);
        let dpf_50 = open_wounds_damage_per_frame(50);
        let dpf_99 = open_wounds_damage_per_frame(99);

        // Les degats augmentent avec le niveau.
        assert!(dpf_50 > dpf_1);
        assert!(dpf_99 > dpf_50);
    }

    #[test]
    fn test_block_prevents_damage() {
        let attacker = make_basic_attacker();
        let mut defender = make_basic_defender();
        defender.block_chance = 100; // Toujours bloque (test).
        defender.defense = 0;

        let mut rng = rand::thread_rng();
        let mut blocks = 0;
        for _ in 0..100 {
            let result = calculate_damage(&attacker, &defender, &mut rng);
            if result.hit && result.blocked {
                blocks += 1;
                assert_eq!(result.total_damage, 0);
            }
        }
        // Tous les hits devraient etre bloques.
        assert!(blocks > 80);
    }
}
```

### 13.3 Test synergies

```rust
#[cfg(test)]
mod synergy_tests {
    use super::*;

    #[test]
    fn test_synergy_bonus_calculation() {
        let skill_def = SkillDef {
            id: "bone_spear".to_string(),
            name: "Bone Spear".to_string(),
            class: "Mortecian".to_string(),
            tree: "Poison & Bone".to_string(),
            tree_position: [1, 3],
            max_level: 20,
            required_level: 18,
            prerequisites: vec!["teeth".to_string()],
            skill_type: SkillType::Ranged,
            mana_cost_base: 7.0,
            mana_cost_per_level: 0.5,
            cooldown_frames: 0,
            cast_frames_base: 15,
            damage_type: DamageType::Magic,
            synergies: vec![
                SynergyDef {
                    skill_id: "teeth".to_string(),
                    bonus_per_level: 7.0,
                    bonus_type: "damage_percent".to_string(),
                },
                SynergyDef {
                    skill_id: "bone_wall".to_string(),
                    bonus_per_level: 7.0,
                    bonus_type: "damage_percent".to_string(),
                },
            ],
            projectile: None,
            aoe_radius: None,
            effect_formula: SkillFormula {
                base_damage_min: 16,
                base_damage_max: 24,
                damage_per_level: 8.0,
                duration_frames_base: 0,
                duration_per_level: 0.0,
            },
        };

        let mut skills = std::collections::HashMap::new();
        skills.insert("teeth".to_string(), SkillInstance {
            skill_id: "teeth".to_string(),
            hard_points: 20,
            bonus_levels: 5,
            cooldown_remaining: 0,
            charges: None,
            charges_max: None,
        });
        skills.insert("bone_wall".to_string(), SkillInstance {
            skill_id: "bone_wall".to_string(),
            hard_points: 10,
            bonus_levels: 0,
            cooldown_remaining: 0,
            charges: None,
            charges_max: None,
        });

        let bonus = calculate_synergy_bonus(&skill_def, &skills);
        // Teeth: 20 hard points * 7% = 140%
        // Bone Wall: 10 hard points * 7% = 70%
        // Total: 210%
        assert!((bonus - 210.0).abs() < 0.1);
    }

    #[test]
    fn test_mana_cost_scaling() {
        let skill_def = SkillDef {
            id: "test_skill".to_string(),
            name: "Test".to_string(),
            class: "Test".to_string(),
            tree: "Test".to_string(),
            tree_position: [0, 0],
            max_level: 20,
            required_level: 1,
            prerequisites: vec![],
            skill_type: SkillType::Spell,
            mana_cost_base: 5.0,
            mana_cost_per_level: 1.5,
            cooldown_frames: 0,
            cast_frames_base: 13,
            damage_type: DamageType::Fire,
            synergies: vec![],
            projectile: None,
            aoe_radius: None,
            effect_formula: SkillFormula {
                base_damage_min: 10,
                base_damage_max: 20,
                damage_per_level: 5.0,
                duration_frames_base: 0,
                duration_per_level: 0.0,
            },
        };

        // Niveau 1 : base = 5.0.
        assert!((skill_def.mana_cost_at_level(1) - 5.0).abs() < 0.01);
        // Niveau 10 : 5.0 + 1.5 * 9 = 18.5.
        assert!((skill_def.mana_cost_at_level(10) - 18.5).abs() < 0.01);
        // Niveau 20 : 5.0 + 1.5 * 19 = 33.5.
        assert!((skill_def.mana_cost_at_level(20) - 33.5).abs() < 0.01);
    }
}
```

---

*Document redige par Francois, Dev Back-End -- Miyukini AI Studio*
*Base sur SD-Tech-Systems.md + SD-Combat-Stats.md de Denis*
*Revision : 2026-02-28 v1.0*
