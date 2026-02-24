# Allumina — Objets, Armes, Armures et Équipement

## Contexte

Ce document est la **spécification complète** du système d'objets d'Allumina : base de données des types d'items, armes, armures, consommables, matériaux, équipement, qualité, génération, prérequis, durabilité, composants ECS et tables de référence. Il constitue la source de vérité pour l'implémentation MGE et le contenu du jeu.

## Portée / Scope

- **Applicable à :** Implémentation ECS, data-driven JSON, game design contenu, équilibrage.
- **Audience :** Développement moteur, game design, level design.
- **Statut :** Spécification normative. Les valeurs numériques sont des valeurs de départ — elles seront affinées au tuning.

### Hors périmètre

- Génération procédurale des items magiques et rares (post-MVP v0.2 — affixes, propriétés enchantées).
- Socketing / enchantement (post-MVP v0.3).
- Items de set (post-MVP v0.3).
- Économie et prix des marchands (voir document économie dédié).

---

## 1. Architecture du système d'items

### 1.1 Hiérarchie des types

```
Item
├── Équipement (portés, améliorent les stats)
│   ├── Armes
│   │   ├── Mêlée (tranchant, contondant, perforant)
│   │   └── À distance (arc, arbalète, arquebuse)
│   ├── Armures
│   │   ├── Corps (tête, torse, jambes)
│   │   └── Bouclier (main gauche)
│   └── Accessoires (anneau, amulette, ceinture)
├── Consommables (usage unique)
│   ├── Potions (soin, endurance, mana)
│   └── Nourriture (soin lent, effets passifs)
├── Matériaux (craft, commerce)
│   ├── Bruts (minerai, bois, cuir, herbes)
│   └── Transformés (lingots, planches, cuir travaillé)
├── Outils (craft, récolte)
│   ├── Récolte (pioche, hache, faucille)
│   └── Craft (marteau de forge, alêne, ciseau)
└── Divers
    ├── Monnaie (or)
    ├── Clés et sceaux
    └── Livres et parchemins
```

### 1.2 Identifiants

Chaque type d'item a un identifiant textuel unique (`item_type_id`), utilisé dans les fichiers JSON de configuration et le code ECS. Convention : `snake_case`, descriptif, stable.

Exemples : `iron_sword`, `leather_helmet`, `health_potion_small`, `iron_ore`, `blacksmith_hammer`.

---

## 2. Composants ECS

### 2.1 Composant Item (universel)

```rust
/// Composant de base présent sur toute entité-item.
pub struct Item {
    /// Identifiant du type (référence dans item_types.json)
    pub type_id: ItemTypeId,
    /// Qualité de l'item
    pub quality: ItemQuality,
    /// Poids unitaire (unités arbitraires ; 10 = 1 kg de référence)
    pub weight: u32,
    /// Nombre empilé (1 pour les non-empilables)
    pub stack_count: u32,
    /// L'item peut-il s'empiler ?
    pub stackable: bool,
    /// L'item est-il identifié ? (false = stats masquées à l'acheteur)
    pub identified: bool,
}

pub enum ItemQuality {
    /// Item dégradé — stats réduites de 25 à 50%
    Degraded,
    /// Item standard
    Normal,
    /// Item de qualité supérieure — +10 à +15% stats, +25% durabilité
    Superior,
    /// Item magique — 1 à 2 propriétés enchantées (v0.2+)
    Magic,
    /// Item rare — 3 à 6 propriétés enchantées (v0.2+)
    Rare,
    /// Item artisan signé — créé par un joueur maître artisan (v0.2+)
    Crafted { crafter_name: String },
}
```

### 2.2 Composant EquipmentStats (items portés)

```rust
/// Présent sur toutes les pièces d'équipement portables.
pub struct EquipmentStats {
    /// Slot d'équipement cible
    pub slot: EquipmentSlot,
    /// Prérequis pour équiper
    pub requirements: EquipRequirements,
    /// Durabilité courante / maximum
    pub durability: (u32, u32),
    /// Bonus/malus de stats appliqués quand équipé
    pub stat_modifiers: Vec<StatModifier>,
}

pub enum EquipmentSlot {
    Head, Torso, Legs, Feet, Hands,
    MainHand, OffHand,
    Ring, Amulet, Belt,
}

pub struct EquipRequirements {
    /// Force minimale requise (0 = pas de prérequis)
    pub min_for: u8,
    /// Agilité minimale requise
    pub min_agi: u8,
    /// Dextérité minimale requise
    pub min_dex: u8,
    /// Niveau de compétence d'arme requis (0 = pas de prérequis)
    pub min_skill_level: u8,
    /// Compétence requise (None si aucune)
    pub required_skill: Option<SkillId>,
}

pub struct StatModifier {
    pub stat: StatCode,
    pub value: i32,
    /// Si true, valeur en % (ex: +10% atk) ; si false, valeur flat
    pub is_percent: bool,
}
```

### 2.3 Composant WeaponData

```rust
/// Présent sur toutes les armes (mêlée et distance).
pub struct WeaponData {
    /// Catégorie d'arme
    pub category: WeaponCategory,
    /// Type de dégâts principal
    pub damage_type: DamageType,
    /// Dégâts min / max (base, avant For/Dex/stats)
    pub damage_min: u32,
    pub damage_max: u32,
    /// Modificateur de vitesse d'attaque
    /// Valeur en ±centièmes de la base ; ex: -20 = rapide, +20 = lent
    pub speed_modifier: i8,
    /// Portée en tiles (mêlée : 1–2 ; distance : 4–12)
    pub range: f32,
    /// Bonus spécifiques à l'arme
    pub bonus_ar: i32,      // bonus à l'atk (toucher)
    pub bonus_par: i32,     // bonus à la parade (si arme de parade)
    /// Deux mains requises ?
    pub two_handed: bool,
}

pub enum WeaponCategory {
    // Mêlée
    Sword,       // Tranchant — Dex→toucher, For→dégâts
    Axe,         // Tranchant — For dominant
    Mace,        // Contondant — For dominant
    Hammer,      // Contondant — For dominant, lent
    Spear,       // Perforant — deux mains ou avec bouclier
    Dagger,      // Tranchant/Perforant — Agi+Dex
    Staff,       // Contondant — deux mains, bonus magie (v0.2)
    // Distance
    Bow,         // Perforant — deux mains, For+Per
    Crossbow,    // Perforant — deux mains, Fort+Per, lent/puissant
    Arquebus,    // Perforant — deux mains, For+Per, très lent (v0.2)
    Pistol,      // Perforant — une main, Agi+Per (v0.2)
    Thrown,      // Mixte — haches/couteaux lancés (v0.3)
}
```

### 2.4 Composant ArmorData

```rust
/// Présent sur toutes les pièces d'armure (corps) et boucliers.
pub struct ArmorData {
    /// Catégorie d'armure
    pub category: ArmorCategory,
    /// Réduction en % par type de dégâts
    pub ar_tranchant: u8,     // % réduction dégâts Tranc (0–75)
    pub ar_contondant: u8,    // % réduction dégâts Cont
    pub ar_perforant: u8,     // % réduction dégâts Perc
    /// Résistance (points absorbés avant que l'armure ne se dégrade)
    /// Décroît de 1 pt par point de dégâts subi
    pub resistance_max: u32,
    /// Pénalité de vitesse d'attaque imposée (en centièmes ; 0 = aucune)
    pub attack_speed_penalty: i8,
    /// Pénalité de vitesse de déplacement (en centièmes ; 0 = aucune)
    pub move_speed_penalty: i8,
}

pub enum ArmorCategory {
    // Corps
    Helmet,      // Tête
    Chestplate,  // Torse
    Leggings,    // Jambes
    Boots,       // Pieds
    Gloves,      // Mains
    // Protection
    Shield,      // Bouclier — slot OffHand
    // Qualité globale
    Light,       // Cuir, tissu — peu d'AR, pas de pénalité
    Medium,      // Mailles, brigandine — AR moyen, légère pénalité
    Heavy,       // Plate — AR élevé, forte pénalité
}
```

### 2.5 Composant ConsumableData

```rust
pub struct ConsumableData {
    pub effect: ConsumableEffect,
    /// Durée de l'effet en ticks (0 = instantané)
    pub duration_ticks: u32,
    /// Délai avant réutilisation en ticks (cooldown)
    pub cooldown_ticks: u32,
}

pub enum ConsumableEffect {
    HealHp { amount: u32 },
    HealHpOverTime { amount_per_tick: u32 },
    HealEnd { amount: u32 },
    RestoreMana { amount: u32 },                 // v0.2 (magie)
    StatBuff { stat: StatCode, value: i32, is_percent: bool },
    Antidote,
    Food { hp_per_tick: u32, duration_ticks: u32 },
}
```

### 2.6 Composant MaterialData

```rust
pub struct MaterialData {
    pub material_type: MaterialType,
    /// Catégorie pour le craft
    pub craft_category: CraftCategory,
}

pub enum MaterialType {
    Ore,         // Minerai brut
    Ingot,       // Lingot fondu
    Wood,        // Bois brut
    Plank,       // Planche travaillée
    RawLeather,  // Cuir brut
    Leather,     // Cuir travaillé
    Herb,        // Plante médicinale ou alchimique
    Extract,     // Extrait concentré (v0.3)
}
```

---

## 3. Table des armes

### 3.1 Légende

| Colonne | Signification |
|---------|--------------|
| `type_id` | Identifiant JSON |
| `Cat` | Catégorie (Sw=Sword, Ax=Axe, Ma=Mace, Ha=Hammer, Sp=Spear, Da=Dagger, Bo=Bow, Xb=Crossbow) |
| `Dmg` | Dégâts min–max base |
| `Type` | Tranc / Cont / Perc |
| `Spd` | Modificateur de vitesse (-20=rapide, 0=neutre, +20=lent) |
| `Portée` | En tiles |
| `2M` | Deux mains requises |
| `Prérequis` | For / Agi / Dex minimum |
| `Poids` | En unités (10 = 1 kg de référence) |
| `Rés.` | Résistance de durabilité max |

### 3.2 Armes de mêlée — Épées (Tranchant)

| type_id | Nom | Cat | Dmg | Type | Spd | Portée | 2M | For | Agi | Dex | Poids | Rés. |
|---------|-----|-----|-----|------|-----|--------|----|-----|-----|-----|-------|------|
| `short_sword` | Épée courte | Sw | 3–8 | Tranc | -10 | 1.5 | Non | 3 | 0 | 2 | 25 | 120 |
| `iron_sword` | Épée en fer | Sw | 5–12 | Tranc | 0 | 1.5 | Non | 4 | 0 | 3 | 35 | 160 |
| `broad_sword` | Épée large | Sw | 7–16 | Tranc | +5 | 1.5 | Non | 5 | 0 | 3 | 45 | 200 |
| `steel_sword` | Épée en acier | Sw | 10–22 | Tranc | 0 | 1.5 | Non | 6 | 0 | 4 | 40 | 240 |
| `longsword` | Longue épée | Sw | 8–18 | Tranc | +5 | 2.0 | Non | 6 | 0 | 4 | 50 | 220 |
| `steel_longsword` | Longue épée acier | Sw | 14–28 | Tranc | +5 | 2.0 | Non | 7 | 0 | 5 | 55 | 280 |
| `greatsword` | Grande épée | Sw | 15–32 | Tranc | +15 | 2.0 | Oui | 8 | 0 | 5 | 80 | 320 |
| `steel_greatsword` | Grande épée acier | Sw | 22–45 | Tranc | +15 | 2.0 | Oui | 9 | 0 | 6 | 90 | 400 |

**Note :** Les épées donnent un bonus de parade (`+par +5 flat`) quand elles sont en main principale, simulant l'utilisation défensive de la lame.

### 3.3 Armes de mêlée — Haches (Tranchant/Contondant)

| type_id | Nom | Cat | Dmg | Type | Spd | Portée | 2M | For | Agi | Dex | Poids | Rés. |
|---------|-----|-----|-----|------|-----|--------|----|-----|-----|-----|-------|------|
| `hand_axe` | Hachette | Ax | 4–9 | Tranc | -5 | 1.5 | Non | 4 | 0 | 0 | 30 | 130 |
| `iron_axe` | Hache en fer | Ax | 6–14 | Tranc | +5 | 1.5 | Non | 5 | 0 | 0 | 50 | 180 |
| `steel_axe` | Hache en acier | Ax | 10–22 | Tranc | +5 | 1.5 | Non | 6 | 0 | 0 | 55 | 220 |
| `battle_axe` | Hache de guerre | Ax | 12–26 | Tranc | +10 | 2.0 | Oui | 7 | 0 | 0 | 80 | 260 |
| `steel_battle_axe` | Hache de guerre acier | Ax | 18–38 | Tranc | +10 | 2.0 | Oui | 8 | 0 | 0 | 90 | 320 |

### 3.4 Armes de mêlée — Masses et marteaux (Contondant)

| type_id | Nom | Cat | Dmg | Type | Spd | Portée | 2M | For | Agi | Dex | Poids | Rés. |
|---------|-----|-----|-----|------|-----|--------|----|-----|-----|-----|-------|------|
| `club` | Gourdin | Ma | 3–7 | Cont | -5 | 1.5 | Non | 3 | 0 | 0 | 30 | 100 |
| `iron_mace` | Masse en fer | Ma | 5–12 | Cont | 0 | 1.5 | Non | 4 | 0 | 0 | 45 | 170 |
| `steel_mace` | Masse en acier | Ma | 9–20 | Cont | 0 | 1.5 | Non | 6 | 0 | 0 | 50 | 210 |
| `iron_hammer` | Marteau en fer | Ha | 8–18 | Cont | +15 | 1.5 | Non | 6 | 0 | 0 | 70 | 200 |
| `steel_hammer` | Marteau en acier | Ha | 14–30 | Cont | +15 | 1.5 | Non | 7 | 0 | 0 | 80 | 260 |
| `warhammer` | Marteau de guerre | Ha | 18–38 | Cont | +20 | 2.0 | Oui | 8 | 0 | 0 | 120 | 320 |
| `steel_warhammer` | Marteau de guerre acier | Ha | 26–52 | Cont | +20 | 2.0 | Oui | 9 | 0 | 0 | 130 | 400 |

### 3.5 Armes de mêlée — Lances et piques (Perforant)

| type_id | Nom | Cat | Dmg | Type | Spd | Portée | 2M | For | Agi | Dex | Poids | Rés. |
|---------|-----|-----|-----|------|-----|--------|----|-----|-----|-----|-------|------|
| `iron_spear` | Lance en fer | Sp | 6–13 | Perc | +5 | 2.0 | Oui | 4 | 0 | 2 | 60 | 160 |
| `steel_spear` | Lance en acier | Sp | 11–22 | Perc | +5 | 2.0 | Oui | 6 | 0 | 3 | 70 | 200 |
| `pike` | Pique en acier | Sp | 16–32 | Perc | +15 | 2.5 | Oui | 7 | 0 | 3 | 90 | 240 |
| `iron_dagger` | Dague en fer | Da | 2–6 | Perc | -20 | 1.0 | Non | 2 | 2 | 4 | 15 | 80 |
| `steel_dagger` | Dague en acier | Da | 4–10 | Perc | -20 | 1.0 | Non | 3 | 3 | 5 | 18 | 100 |

**Note Dague :** les dagues n'ont pas de bonus de parade. En contrepartie, leur vitesse est la plus élevée du jeu et elles permettent le port d'un second accessoire en OffHand si on les combine avec la compétence **Ambidextrie** (v0.3).

### 3.6 Armes à distance — Arcs (Perforant)

| type_id | Nom | Cat | Dmg | Type | Spd | Portée | 2M | For | Agi | Per | Poids | Rés. |
|---------|-----|-----|-----|------|-----|--------|----|-----|-----|-----|-------|------|
| `simple_bow` | Arc simple | Bo | 4–10 | Perc | -5 | 7 | Oui | 3 | 2 | 2 | 30 | 80 |
| `hunting_bow` | Arc de chasse | Bo | 6–14 | Perc | 0 | 8 | Oui | 4 | 3 | 3 | 35 | 100 |
| `composite_bow` | Arc composite | Bo | 9–19 | Perc | 0 | 9 | Oui | 5 | 3 | 4 | 40 | 120 |
| `longbow` | Arc long | Bo | 12–24 | Perc | +10 | 10 | Oui | 6 | 2 | 4 | 50 | 140 |
| `elven_bow` | Arc elfique | Bo | 14–28 | Perc | -5 | 10 | Oui | 5 | 4 | 5 | 35 | 160 |

**Note :** les arcs consomment des **flèches** (matériau empilable). 1 flèche par tir. Les flèches sont consommables mais leur poids total s'additionne au port.

### 3.7 Armes à distance — Arbalètes (Perforant)

| type_id | Nom | Cat | Dmg | Type | Spd | Portée | 2M | For | Agi | Per | Poids | Rés. |
|---------|-----|-----|-----|------|-----|--------|----|-----|-----|-----|-------|------|
| `light_crossbow` | Arbalète légère | Xb | 10–20 | Perc | +15 | 8 | Oui | 4 | 0 | 3 | 60 | 120 |
| `crossbow` | Arbalète | Xb | 15–30 | Perc | +20 | 9 | Oui | 5 | 0 | 3 | 80 | 150 |
| `heavy_crossbow` | Arbalète lourde | Xb | 22–42 | Perc | +25 | 10 | Oui | 7 | 0 | 4 | 100 | 180 |

**Note :** les arbalètes consomment des **carreaux** (empilables). Plus puissantes mais plus lentes que les arcs ; rechargement visible (animation + délai fixe de 1.5 s après chaque tir, non réduit par l'`atk speed`).

---

## 4. Table des armures

### 4.1 Légend

| Colonne | Signification |
|---------|--------------|
| `ARt` | Réduction % dégâts Tranchant |
| `ARc` | Réduction % dégâts Contondant |
| `ARp` | Réduction % dégâts Perforant |
| `Rés.` | Résistance max de la pièce (points absorbés avant dégradation) |
| `Pén. Spd` | Pénalité vitesse d'attaque (en centièmes de la base) |
| `Pén. Mv` | Pénalité vitesse de déplacement (en centièmes) |
| `For min` | Force minimale pour équiper |
| `Poids` | Poids en unités |

### 4.2 Casques

| type_id | Nom | ARt | ARc | ARp | Rés. | Pén.Spd | Pén.Mv | For | Poids |
|---------|-----|-----|-----|-----|------|---------|--------|-----|-------|
| `leather_cap` | Bonnet de cuir | 2 | 1 | 2 | 60 | 0 | 0 | 1 | 15 |
| `iron_helmet` | Casque en fer | 5 | 6 | 3 | 120 | 0 | 0 | 3 | 35 |
| `chainmail_coif` | Camail en mailles | 4 | 5 | 5 | 110 | 0 | 0 | 3 | 30 |
| `steel_helmet` | Casque en acier | 8 | 9 | 5 | 180 | 0 | 0 | 5 | 45 |
| `great_helm` | Heaume intégral | 10 | 10 | 8 | 220 | -5 | 0 | 6 | 60 |
| `steel_great_helm` | Heaume acier | 14 | 14 | 11 | 300 | -5 | 0 | 7 | 70 |

**Note :** le `great_helm` impose une pénalité de -5 à `atk speed` (vision réduite → réactivité diminuée) mais pas de pénalité de déplacement.

### 4.3 Torses

| type_id | Nom | Classe | ARt | ARc | ARp | Rés. | Pén.Spd | Pén.Mv | For | Poids |
|---------|-----|--------|-----|-----|-----|------|---------|--------|-----|-------|
| `cloth_robe` | Robe de tissu | Light | 1 | 0 | 1 | 40 | 0 | 0 | 1 | 10 |
| `leather_armor` | Armure de cuir | Light | 5 | 2 | 5 | 100 | 0 | 0 | 2 | 40 |
| `studded_leather` | Cuir clouté | Light | 7 | 3 | 6 | 130 | 0 | 0 | 3 | 55 |
| `chainmail` | Cotte de mailles | Medium | 6 | 10 | 8 | 180 | -5 | -3 | 4 | 80 |
| `brigandine` | Brigandine | Medium | 8 | 12 | 9 | 210 | -5 | -3 | 5 | 95 |
| `iron_plate` | Plastron en fer | Heavy | 10 | 16 | 7 | 240 | -10 | -8 | 6 | 130 |
| `steel_plate` | Plastron en acier | Heavy | 15 | 22 | 10 | 320 | -10 | -8 | 7 | 150 |
| `full_plate` | Armure complète | Heavy | 18 | 25 | 13 | 380 | -15 | -12 | 8 | 180 |
| `steel_full_plate` | Armure complète acier | Heavy | 24 | 32 | 17 | 480 | -15 | -12 | 9 | 200 |

### 4.4 Jambières

| type_id | Nom | ARt | ARc | ARp | Rés. | Pén.Spd | Pén.Mv | For | Poids |
|---------|-----|-----|-----|-----|------|---------|--------|-----|-------|
| `leather_leggings` | Jambières de cuir | 3 | 1 | 3 | 70 | 0 | 0 | 2 | 25 |
| `chainmail_leggings` | Jambières en mailles | 4 | 7 | 5 | 120 | 0 | -2 | 3 | 60 |
| `iron_greaves` | Grèves en fer | 7 | 11 | 5 | 160 | 0 | -5 | 5 | 90 |
| `steel_greaves` | Grèves en acier | 10 | 15 | 7 | 210 | 0 | -5 | 6 | 105 |

### 4.5 Bottes

| type_id | Nom | ARt | ARc | ARp | Rés. | Pén.Mv | For | Poids | Bonus |
|---------|-----|-----|-----|-----|------|--------|-----|-------|-------|
| `leather_boots` | Bottes de cuir | 1 | 1 | 1 | 50 | 0 | 1 | 15 | — |
| `iron_boots` | Bottes en fer | 3 | 4 | 2 | 90 | -3 | 3 | 45 | — |
| `steel_boots` | Bottes en acier | 5 | 6 | 3 | 120 | -3 | 4 | 55 | — |
| `hiking_boots` | Bottes de randonnée | 1 | 2 | 1 | 60 | +5 | 1 | 20 | +5 Athlétisme |

**Note :** les bottes de randonnée donnent une **pénalité de déplacement positive** (bonus), compensant légèrement la fatigue naturelle du terrain.

### 4.6 Gants

| type_id | Nom | ARt | ARc | ARp | Rés. | Pén.Spd | For | Poids | Bonus |
|---------|-----|-----|-----|-----|------|---------|-----|-------|-------|
| `leather_gloves` | Gants de cuir | 1 | 1 | 1 | 40 | 0 | 1 | 8 | — |
| `iron_gauntlets` | Gantelets en fer | 3 | 4 | 2 | 80 | -3 | 3 | 30 | — |
| `steel_gauntlets` | Gantelets en acier | 5 | 6 | 3 | 110 | -3 | 4 | 38 | — |
| `archer_gloves` | Gants d'archer | 1 | 1 | 2 | 45 | +3 | 1 | 10 | +5 tirC |
| `thief_gloves` | Gants de voleur | 1 | 1 | 1 | 35 | +5 | 1 | 8 | +5 Larcin |

**Note :** les gants de mêlée lourde imposent une légère pénalité à `atk speed` (rigidité des doigts). Les gants spécialisés (archer, voleur) sacrifient la protection contre des bonus de compétence.

### 4.7 Boucliers

| type_id | Nom | ARt | ARc | ARp | Rés. | Bonus Par | For | Poids |
|---------|-----|-----|-----|-----|------|-----------|-----|-------|
| `wooden_shield` | Bouclier en bois | 4 | 6 | 2 | 80 | +8 | 2 | 40 |
| `iron_shield` | Bouclier en fer | 6 | 8 | 4 | 130 | +12 | 4 | 65 |
| `steel_shield` | Bouclier en acier | 9 | 11 | 6 | 180 | +16 | 5 | 80 |
| `kite_shield` | Écu | 8 | 10 | 7 | 160 | +15 | 5 | 75 |
| `tower_shield` | Écu-tour | 12 | 15 | 10 | 220 | +20 | 7 | 110 |
| `steel_tower_shield` | Écu-tour acier | 16 | 20 | 14 | 300 | +25 | 8 | 130 |
| `buckler` | Bocle | 3 | 4 | 3 | 60 | +6 | 2 | 25 |

**Note :** le **Bonus Par** des boucliers s'additionne à la valeur de parade du personnage (`par`) uniquement quand le bouclier est équipé en OffHand. Il représente la surface de blocage additionnelle. La résistance du bouclier diminue à chaque parade réussie (voir section 9).

### 4.8 Accessoires

Les accessoires (anneaux, amulettes, ceintures) n'offrent pas de réduction de dégâts mais des **bonus de stats** flat ou en pourcentage. Ils ne se dégradent pas (pas de système de durabilité).

| type_id | Nom | Slot | Effet de base | Prérequis |
|---------|-----|------|---------------|-----------|
| `iron_ring` | Anneau en fer | Ring | +1 For | — |
| `signet_ring` | Anneau à chevalière | Ring | +1 Cha | — |
| `leather_belt` | Ceinture de cuir | Belt | +1 slot potion | — |
| `warrior_belt` | Ceinture de guerrier | Belt | +2 slots potion, +1 Con | 3 For |
| `bone_amulet` | Amulette en os | Amulet | +1 Con | — |
| `jade_amulet` | Amulette de jade | Amulet | +1 Per | — |

**Note :** les accessoires de qualité supérieure (Magic, Rare — v0.2+) seront les principaux vecteurs de propriétés enchantées. À ce stade (MVP), ils sont intentionnellement limités pour ne pas complexifier le système.

---

## 5. Formules de stats d'équipement

### 5.1 Calcul des dégâts d'arme

```
// Dégâts bruts au combat — séquence complète
dmg_roll = random(weapon.damage_min, weapon.damage_max)

// Bonus de caractéristique selon le type d'arme :
//   Mêlée standard (Sword, Axe, Mace, Hammer, Spear) : For dominant
//   Dague : For/2 + Dex/2
//   Arc : For/2 + Per/2
//   Arbalète : For + Per/2 (puissance mécanique)
char_bonus = compute_char_bonus(weapon.category, char_stats)

// Bonus flat de qualité
quality_bonus = match item.quality {
    Superior => dmg_roll * 0.12,  // +12%
    _        => 0.0,
}

dmg_total = (dmg_roll + char_bonus + quality_bonus).floor()
```

**Formules de bonus de caractéristique par catégorie d'arme :**

| Catégorie | Formule bonus dégâts |
|-----------|---------------------|
| Sword, Axe, Mace, Hammer, Spear | `(For - 1) × 0.8` |
| Dagger | `(For - 1) × 0.4 + (Dex - 1) × 0.4` |
| Staff (v0.2) | `(For - 1) × 0.3 + (Int - 1) × 0.6` |
| Bow | `(For - 1) × 0.4 + (Per - 1) × 0.4` |
| Crossbow | `(For - 1) × 0.6 + (Per - 1) × 0.3` |
| Arquebus (v0.2) | `(For - 1) × 0.5 + (Per - 1) × 0.5` |

*Exemples : un personnage For 5 avec une épée en fer (5–12) : bonus = (5-1) × 0.8 = 3.2 → dégâts 8.2–15.2.*

### 5.2 Calcul de la vitesse d'attaque effective

```
// Vitesse d'attaque en aptitude (atk speed, 1–100)
base_atk_speed = Agi × 10

// Modificateur de l'arme (speed_modifier : négatif = plus rapide)
// Signe inversé : une arme speed_modifier = +20 RÉDUIT la vitesse
weapon_penalty = weapon.speed_modifier   // déjà en centièmes

// Pénalité armure (torse + casque + gants)
armor_penalty = sum(armor_piece.attack_speed_penalty for equipped armors)

// Vitesse d'attaque effective
atk_speed_effective = base_atk_speed - weapon_penalty - armor_penalty
atk_speed_effective = clamp(atk_speed_effective, 5, 100)

// Cooldown entre coups (secondes)
// atk_speed 50 = 1 attaque/sec ; 100 = 2 attaques/sec ; 5 = 0.1 attaque/sec
attack_cooldown = 1.0 / (atk_speed_effective / 50.0)
```

*Exemples :*
- *Agi 5, épée neutre (spd 0), cuir (pén 0) : atk_speed 50 → 1 att/s*
- *Agi 5, épée courte (spd -10) : atk_speed 60 → 1.2 att/s*
- *Agi 5, marteau de guerre (spd +20), plate (pén -10) : atk_speed 20 → 0.4 att/s*
- *Agi 8, dague (spd -20) : atk_speed 100 → 2 att/s*

### 5.3 Calcul de la vitesse de déplacement effective

```
base_move_speed = 3.0 + (Agi - 5) × 0.2    // tiles/sec (voir MVP Sandbox §2)

armor_mv_penalty = sum(armor_piece.move_speed_penalty for equipped armors)
// Pénalité est négative (ex : -8 pour iron_plate) → exprimée en centièmes de tile/s
armor_mv_factor = 1.0 + (armor_mv_penalty / 100.0)

move_speed_effective = base_move_speed × armor_mv_factor
move_speed_effective = max(move_speed_effective, 0.5)    // plancher à 0.5 tile/s
```

*Exemple : Agi 5, full plate (pén -12) : 3.0 × (1 - 0.12) = 2.64 tiles/s.*

### 5.4 Calcul de la réduction de dégâts par l'armure

L'armure réduit les dégâts en % selon le type, avec une règle de dégradation de la résistance.

```
// Réduction appliquée sur les dégâts bruts finaux
ar_pct = get_armor_reduction(damage_type)   // ARt, ARc ou ARp, en %
dmg_after_ar = dmg_total × (1.0 - ar_pct / 100.0)

// Note : les réductions de plusieurs pièces s'additionnent, plafonnées à 75%
total_ar = sum(piece.ar_type for all equipped pieces)
total_ar = min(total_ar, 75)
```

**Cumul des pièces — exemple avec un personnage en armure d'acier complète (torse+casque+greaves+gauntlets) face à des dégâts Tranchant :**

| Pièce | ARt |
|-------|-----|
| steel_plate (torse) | 15 |
| steel_helmet (casque) | 8 |
| steel_greaves (jambes) | 10 |
| steel_gauntlets (gants) | 5 |
| **Total ARt** | **38%** (plafonné à 75%) |

### 5.5 Prérequis et conséquences du non-respect

Si un personnage équipe une pièce dont il ne remplit pas les prérequis de caractéristique :

```
// L'item peut être équipé physiquement (inventaire → slot)
// MAIS les bonus de stat sont désactivés
// ET les pénalités (poids, vitesse) s'appliquent quand même
penalty_mode = true   // uniquement les malus, aucun bénéfice
```

Ce comportement est délibéré : il punit le port prématuré d'équipement lourd sans supprimer le gameplay (un joueur peut ramasser et transporter un item pour le vendre, même s'il ne peut pas l'utiliser efficacement).

### 5.6 Dégradation de la durabilité

```
// À chaque attaque reçue (mêlée ou distance)
// La pièce d'armure qui "absorbe" le coup perd 1 pt de résistance par point de dégâts bruts
// (avant réduction ARt/ARc/ARp)
affected_piece = determine_hit_location(attack_type)
affected_piece.durability.current -= floor(dmg_total_before_ar × 0.1)
// Taux d'usure = 10% des dégâts bruts, arrondi à l'inférieur

// Quand durability.current atteint 0 : la pièce est BRISÉE
// → ARt/ARc/ARp de la pièce passent à 0
// → Le bonus Par du bouclier passe à 0
// → La pièce ne peut plus être équipée jusqu'à réparation
broken = durability.current <= 0
```

**Détermination de la pièce touchée :**

| Situation | Pièce principalement touchée |
|-----------|------------------------------|
| Attaque mêlée standard | Torse (70%) / Casque (15%) / Jambes (15%) |
| Attaque à distance | Torse (50%) / Jambes (30%) / Casque (20%) |
| Parade réussie | Bouclier (si équipé) ou arme de parade |
| Attaque sur les mains | Gants (si ciblage spécifique — v0.3) |

*La randomisation est pondérée. En MVP, une pièce aléatoire est choisie avec le poids ci-dessus.*

### 5.7 Réparation

```
// La réparation se fait auprès d'un artisan PNJ (forge, atelier)
// ou d'un joueur Maître Artisan (v0.2)

// Coût de réparation = (manque de durabilité) × coût_par_point
repair_needed = durability.max - durability.current
gold_cost = repair_needed × item_base.repair_cost_per_point
material_cost = repair_needed × item_base.repair_material_per_point   // optionnel

// Pièces de qualité Supérieure : coût × 1.3
// Pièces Brisées : coût × 2.0 (réparation d'urgence)

// Restauration : durability.current = durability.max
```

---

## 6. Table des consommables

### 6.1 Potions

| type_id | Nom | Effet | Valeur | Durée | Cooldown | Poids | Empilable |
|---------|-----|-------|--------|-------|----------|-------|-----------|
| `health_potion_small` | Petite potion de soin | HealHp | 30 PV | Instantané | 5s | 3 | Oui (×20) |
| `health_potion_medium` | Potion de soin | HealHp | 80 PV | Instantané | 5s | 4 | Oui (×20) |
| `health_potion_large` | Grande potion de soin | HealHp | 180 PV | Instantané | 5s | 5 | Oui (×10) |
| `rejuv_potion` | Potion de vigueur | HealHpOverTime | 8/tick | 20 ticks (~7s) | 10s | 4 | Oui (×10) |
| `endurance_potion` | Potion d'endurance | HealEnd | 100 End | Instantané | 8s | 3 | Oui (×20) |
| `antidote` | Antidote | Antidote | — | Instantané | 0 | 2 | Oui (×20) |

**Cooldown potions :** le cooldown s'applique par catégorie. Boire une petite potion de soin déclenche un cooldown de 5 s sur toutes les potions de soin, mais pas sur les potions d'endurance.

### 6.2 Nourriture

La nourriture fournit un soin lent mais sans cooldown de potion, ce qui la rend précieuse hors combat.

| type_id | Nom | PV/tick | Durée | Conditions | Poids | Empilable |
|---------|-----|---------|-------|-----------|-------|-----------|
| `bread` | Pain | 2 | 30 ticks (10s) | Pas en combat | 5 | Oui (×10) |
| `roasted_meat` | Viande rôtie | 5 | 30 ticks | Pas en combat | 8 | Oui (×10) |
| `dried_herbs` | Herbes séchées | 1 | 60 ticks | Partout | 2 | Oui (×20) |
| `field_rations` | Rations de campagne | 3 | 45 ticks | Partout | 6 | Oui (×10) |

**Condition "Pas en combat"** : si le personnage reçoit un coup ou attaque dans les 3 s, l'effet de nourriture est interrompu.

---

## 7. Table des matériaux

### 7.1 Matériaux bruts (récolte)

| type_id | Nom | Source | Skill requis | Outil requis | Poids | Empilable |
|---------|-----|--------|--------------|-------------|-------|-----------|
| `iron_ore` | Minerai de fer | Veines de fer | Minage 0 | Pioche | 8 | Oui (×30) |
| `steel_ore` | Minerai d'acier | Veines d'acier | Minage 40 | Pioche | 10 | Oui (×30) |
| `oak_wood` | Bois de chêne | Chênes | Bûcheronnage 0 | Hache | 6 | Oui (×30) |
| `hardwood` | Bois dur | Ormes durs | Bûcheronnage 40 | Hache | 8 | Oui (×30) |
| `raw_leather` | Cuir brut | Monstres (loups, cerfs) | — | — | 5 | Oui (×20) |
| `healing_herb` | Herbe médicinale | Plantes sauvages | Herboristerie 0 | — | 1 | Oui (×40) |
| `alchemic_herb` | Herbe alchimique | Plantes rares | Herboristerie 30 | — | 1 | Oui (×40) |
| `wolfpelt` | Peau de loup | Loups | — | — | 12 | Oui (×10) |

### 7.2 Matériaux transformés (craft intermédiaire — v0.2)

En MVP, les recettes utilisent directement les matériaux bruts. En v0.2, les chaînes de production intermédiaires seront ajoutées.

| type_id | Nom | Produit de | Poids | Usage |
|---------|-----|------------|-------|-------|
| `iron_ingot` | Lingot de fer | 3× minerai fer | 6 | Craft armes/armures fer |
| `steel_ingot` | Lingot d'acier | 3× minerai acier | 7 | Craft armes/armures acier |
| `oak_plank` | Planche de chêne | 2× bois chêne | 4 | Craft arcs, manche |
| `hardwood_plank` | Planche de bois dur | 2× bois dur | 5 | Craft arcs, structures |
| `leather` | Cuir travaillé | 2× cuir brut | 4 | Craft armures légères |
| `herb_extract` | Extrait médicinal | 3× herbe médicinale | 1 | Potions améliorées (v0.3) |

### 7.3 Munitions

| type_id | Nom | Pour | Dégâts bonus | Poids/unité | Empilable |
|---------|-----|------|-------------|------------|-----------|
| `arrow` | Flèche | Arc | 0 | 0.5 | Oui (×100) |
| `iron_arrow` | Flèche en fer | Arc | +1 Perc | 0.7 | Oui (×80) |
| `bodkin_arrow` | Flèche bodkin | Arc | +2 Perc, -1 Tranc | 0.8 | Oui (×60) |
| `bolt` | Carreau | Arbalète | 0 | 0.8 | Oui (×80) |
| `iron_bolt` | Carreau en fer | Arbalète | +2 Perc | 1.0 | Oui (×60) |

**Note :** les munitions en rupture de stock désactivent l'arme à distance. Un message d'alerte apparaît à 10 munitions restantes. Sans munitions, le personnage ne peut pas utiliser l'arme à distance mais peut continuer à se battre au corps à corps si une arme de mêlée est en OffHand (fonctionnalité post-MVP v0.2).

---

## 8. Table des outils

Les outils ne sont pas équipés dans les slots d'équipement : ils sont utilisés depuis l'inventaire par interaction avec un nœud ou une station. Ils s'usent à chaque usage (durabilité propre).

| type_id | Nom | Usage | Skill associé | Durabilité | Poids |
|---------|-----|-------|--------------|-----------|-------|
| `pickaxe` | Pioche | Minage | Minage | 200 | 35 |
| `iron_pickaxe` | Pioche en fer | Minage | Minage | 350 | 45 |
| `hatchet` | Hachette de bûcheron | Bûcheronnage | Bûcheronnage | 200 | 30 |
| `iron_hatchet` | Hachette en fer | Bûcheronnage | Bûcheronnage | 350 | 40 |
| `blacksmith_hammer` | Marteau de forge | Forge (Mécanique) | Mécanique | 500 | 50 |
| `iron_hammer_craft` | Marteau de forge en fer | Forge (Mécanique) | Mécanique | 800 | 60 |
| `woodworking_plane` | Rabot de menuisier | Atelier (Mécanique) | Mécanique | 400 | 25 |
| `alchemist_mortar` | Mortier d'alchimiste | Atelier (Herboristerie) | Herboristerie | 600 | 20 |
| `sewing_kit` | Nécessaire de couture | Atelier cuir | Mécanique | 300 | 10 |

**Dégradation des outils :**
```
// À chaque utilisation (récolte ou craft), l'outil perd 1 point de durabilité
tool.durability.current -= 1
if tool.durability.current <= 0:
    tool.broken = true
    // L'outil ne peut plus être utilisé jusqu'à réparation ou remplacement
```

Les outils peuvent être réparés chez l'artisan ou par un joueur Maître Artisan (v0.2), mais leur durabilité maximale diminue légèrement à chaque réparation (max -10% par réparation, plancher à 50% de la valeur d'origine).

---

## 9. Système de durabilité — résumé des règles

### 9.1 Armures et boucliers

| Événement | Impact durabilité |
|-----------|------------------|
| Coup mêlée reçu | -floor(dmg_brut × 0.10) sur la pièce touchée |
| Coup distance reçu | -floor(dmg_brut × 0.08) sur la pièce touchée |
| Parade réussie (bouclier/arme) | -floor(dmg_brut × 1.0) sur l'objet qui pare |
| Chute importante (v0.3) | -1 à -5 sur bottes |
| Réparation (forgeron) | Restaure à max, coût or |

### 9.2 Armes

| Événement | Impact durabilité |
|-----------|------------------|
| 50 attaques portées | -1 sur l'arme |
| Utilisation comme outil de fortune (v0.3) | -5 par usage |
| Réparation (forgeron) | Restaure à max |

**Note :** les armes se dégradent lentement à l'usage normal. La durabilité d'arme est intentionnellement plus lente à chuter que celle d'armure, car les armes sont représentées en tant que pièces maîtresses de l'équipement.

### 9.3 États de durabilité

```rust
pub enum DurabilityState {
    /// 75–100% — aucun effet
    Good,
    /// 50–74% — -5% aux stats de la pièce
    Worn,
    /// 25–49% — -15% aux stats, icône d'alerte en UI
    Damaged,
    /// 1–24% — -30% aux stats, notification urgente
    Critical,
    /// 0 — pièce inactive, aucune stat, doit être réparée
    Broken,
}

fn durability_state(current: u32, max: u32) -> DurabilityState {
    let pct = (current * 100) / max;
    match pct {
        75..=100 => DurabilityState::Good,
        50..=74  => DurabilityState::Worn,
        25..=49  => DurabilityState::Damaged,
        1..=24   => DurabilityState::Critical,
        0        => DurabilityState::Broken,
    }
}
```

---

## 10. Génération des items (loot et craft)

### 10.1 Qualité au loot

En MVP, la qualité est déterminée par un jet simple pondéré selon le niveau du monstre tué et le niveau de `Chance (Luk)` du personnage.

```rust
fn roll_item_quality(monster_level: u32, killer_luk: u8) -> ItemQuality {
    // Bonus Luk : +0.5% de chance Supérieur par point de Luk
    let luk_bonus = killer_luk as f32 * 0.5;

    let roll = random_d100() as f32;

    // Seuils de qualité (les % s'appliquent dans l'ordre)
    // Les items magiques/rares sont exclus du MVP — réservés v0.2
    let chance_degraded = match monster_level {
        1..=4  => 20.0,
        5..=9  => 12.0,
        _      => 6.0,
    };
    let chance_superior = match monster_level {
        1..=4  => 5.0 + luk_bonus,
        5..=9  => 8.0 + luk_bonus,
        _      => 12.0 + luk_bonus,
    };

    if roll < chance_degraded {
        ItemQuality::Degraded
    } else if roll < chance_degraded + chance_superior {
        ItemQuality::Superior
    } else {
        ItemQuality::Normal
    }
}
```

### 10.2 Qualité au craft

La qualité au craft est déterminée par le résultat du jet de **Mécanique** (ou Herboristerie selon la recette).

```rust
fn craft_quality_from_skill_check(skill_result: f32, difficulty: f32) -> ItemQuality {
    // skill_result est le résultat du jet opposé (0–100)
    // difficulty est la difficulté de la recette (0–100)
    let margin = skill_result - difficulty;   // marge de réussite

    match margin {
        m if m >= 25.0  => ItemQuality::Superior,    // large réussite
        m if m >= 0.0   => ItemQuality::Normal,       // réussite standard
        m if m >= -15.0 => ItemQuality::Degraded,     // quasi-échec
        _               => {
            // Échec total : composants perdus, aucun item produit
            return Err(CraftError::FailedAndLost);
        }
    }
}
```

### 10.3 Stats des items Supérieur et Dégradé

```
// Item Supérieur : stats de base × 1.12, durabilité × 1.25
Superior.damage_min = floor(base.damage_min × 1.12)
Superior.damage_max = floor(base.damage_max × 1.12)
Superior.ar_type    = floor(base.ar_type × 1.12)
Superior.resistance = floor(base.resistance × 1.25)

// Item Dégradé : stats de base × 0.65, durabilité × 0.60
Degraded.damage_min = floor(base.damage_min × 0.65)
Degraded.damage_max = floor(base.damage_max × 0.65)
Degraded.ar_type    = floor(base.ar_type × 0.65)
Degraded.resistance = floor(base.resistance × 0.60)
```

---

## 11. Tables de loot par monstre (MVP)

Les tables de loot définissent ce que chaque type de monstre peut lâcher. Chaque entrée a une probabilité indépendante (plusieurs items peuvent tomber simultanément).

### 11.1 Format de la table

```json
{
  "loot_table_id": "wolf_loot",
  "gold_min": 3,
  "gold_max": 12,
  "entries": [
    { "item_type": "raw_leather", "probability": 0.70, "count": [1, 2] },
    { "item_type": "wolfpelt",    "probability": 0.30, "count": [1, 1] },
    { "item_type": "healing_herb","probability": 0.15, "count": [1, 3] }
  ]
}
```

### 11.2 Tables des monstres MVP

**Rat (`rat_loot`)** — Niveau 1

| Item | Prob | Qté |
|------|------|-----|
| Or | 1–4 | — |
| `raw_leather` | 0.40 | 1 |
| `healing_herb` | 0.10 | 1 |

**Loup (`wolf_loot`)** — Niveau 3

| Item | Prob | Qté |
|------|------|-----|
| Or | 3–12 | — |
| `raw_leather` | 0.70 | 1–2 |
| `wolfpelt` | 0.30 | 1 |
| `healing_herb` | 0.15 | 1–3 |

**Gobelin (`goblin_loot`)** — Niveau 5

| Item | Prob | Qté |
|------|------|-----|
| Or | 8–25 | — |
| `iron_ore` | 0.40 | 1–3 |
| `short_sword` | 0.10 | 1 |
| `iron_helmet` | 0.06 | 1 |
| `health_potion_small` | 0.25 | 1 |

**Bandit (`bandit_loot`)** — Niveau 7

| Item | Prob | Qté |
|------|------|-----|
| Or | 15–50 | — |
| `leather_armor` | 0.12 | 1 |
| `iron_sword` | 0.10 | 1 |
| `iron_mace` | 0.08 | 1 |
| `leather_boots` | 0.15 | 1 |
| `health_potion_small` | 0.35 | 1–2 |
| `iron_ore` | 0.20 | 1–4 |

**Troll (`troll_loot`)** — Niveau 12

| Item | Prob | Qté |
|------|------|-----|
| Or | 40–120 | — |
| `steel_ore` | 0.30 | 2–5 |
| `iron_axe` | 0.15 | 1 |
| `chainmail` | 0.08 | 1 |
| `iron_greaves` | 0.10 | 1 |
| `health_potion_medium` | 0.40 | 1–2 |
| `endurance_potion` | 0.20 | 1 |

**Chef Troll (`troll_boss_loot`)** — Niveau 20

| Item | Prob | Qté |
|------|------|-----|
| Or | 150–400 | — |
| `steel_ore` | 0.80 | 4–10 |
| `steel_sword` | 0.20 | 1 |
| `steel_helmet` | 0.15 | 1 |
| `steel_plate` | 0.10 | 1 |
| `health_potion_large` | 0.60 | 1–3 |
| `rejuv_potion` | 0.30 | 1–2 |

---

## 12. Recettes de craft (MVP — normatives)

La table suivante est la référence normative pour l'implémentation. Elle complète et remplace la table indicative du document MVP Sandbox.

| `recipe_id` | Nom | Inputs | Output | Skill | Min | Station | Qualité |
|-------------|-----|--------|--------|-------|-----|---------|---------|
| `r_short_sword` | Épée courte | 3× iron_ore | short_sword | Mécanique | 0 | Forge | Jet craft |
| `r_iron_sword` | Épée en fer | 5× iron_ore | iron_sword | Mécanique | 0 | Forge | Jet craft |
| `r_iron_mace` | Masse en fer | 4× iron_ore | iron_mace | Mécanique | 0 | Forge | Jet craft |
| `r_iron_hammer` | Marteau en fer | 6× iron_ore | iron_hammer | Mécanique | 10 | Forge | Jet craft |
| `r_iron_spear` | Lance en fer | 3× iron_ore + 2× oak_wood | iron_spear | Mécanique | 10 | Forge | Jet craft |
| `r_iron_dagger` | Dague en fer | 2× iron_ore | iron_dagger | Mécanique | 5 | Forge | Jet craft |
| `r_iron_axe` | Hache en fer | 4× iron_ore + 1× oak_wood | iron_axe | Mécanique | 5 | Forge | Jet craft |
| `r_simple_bow` | Arc simple | 3× oak_wood | simple_bow | Mécanique | 0 | Atelier | Jet craft |
| `r_hunting_bow` | Arc de chasse | 4× oak_wood + 1× raw_leather | hunting_bow | Mécanique | 20 | Atelier | Jet craft |
| `r_light_crossbow` | Arbalète légère | 4× oak_wood + 2× iron_ore | light_crossbow | Mécanique | 30 | Forge | Jet craft |
| `r_leather_cap` | Bonnet de cuir | 2× raw_leather | leather_cap | Mécanique | 0 | Atelier | Jet craft |
| `r_leather_armor` | Armure de cuir | 5× raw_leather | leather_armor | Mécanique | 0 | Atelier | Jet craft |
| `r_leather_leggings` | Jambières de cuir | 3× raw_leather | leather_leggings | Mécanique | 5 | Atelier | Jet craft |
| `r_leather_boots` | Bottes de cuir | 2× raw_leather | leather_boots | Mécanique | 0 | Atelier | Jet craft |
| `r_leather_gloves` | Gants de cuir | 2× raw_leather | leather_gloves | Mécanique | 0 | Atelier | Jet craft |
| `r_wooden_shield` | Bouclier en bois | 4× oak_wood + 1× raw_leather | wooden_shield | Mécanique | 0 | Atelier | Jet craft |
| `r_iron_helmet` | Casque en fer | 3× iron_ore | iron_helmet | Mécanique | 15 | Forge | Jet craft |
| `r_chainmail` | Cotte de mailles | 8× iron_ore | chainmail | Mécanique | 30 | Forge | Jet craft |
| `r_iron_shield` | Bouclier en fer | 5× iron_ore | iron_shield | Mécanique | 20 | Forge | Jet craft |
| `r_iron_plate` | Plastron en fer | 10× iron_ore | iron_plate | Mécanique | 40 | Forge | Jet craft |
| `r_iron_greaves` | Grèves en fer | 6× iron_ore | iron_greaves | Mécanique | 30 | Forge | Jet craft |
| `r_iron_gauntlets` | Gantelets en fer | 3× iron_ore | iron_gauntlets | Mécanique | 20 | Forge | Jet craft |
| `r_iron_boots_craft` | Bottes en fer | 4× iron_ore + 1× raw_leather | iron_boots | Mécanique | 20 | Forge | Jet craft |
| `r_pickaxe` | Pioche | 3× iron_ore + 2× oak_wood | pickaxe | Mécanique | 0 | Forge | — |
| `r_hatchet` | Hachette | 2× iron_ore + 1× oak_wood | hatchet | Mécanique | 0 | Forge | — |
| `r_arrow_bunch` | Flèches (×20) | 1× oak_wood | arrow ×20 | Mécanique | 0 | Atelier | — |
| `r_iron_arrow_bunch` | Flèches en fer (×20) | 1× oak_wood + 1× iron_ore | iron_arrow ×20 | Mécanique | 10 | Forge | — |
| `r_bolt_bunch` | Carreaux (×20) | 1× oak_wood + 1× iron_ore | bolt ×20 | Mécanique | 5 | Forge | — |
| `r_health_potion_small` | Petite potion de soin | 2× healing_herb | health_potion_small ×3 | Herboristerie | 0 | Atelier | Jet craft |
| `r_health_potion_medium` | Potion de soin | 4× healing_herb | health_potion_medium ×2 | Herboristerie | 20 | Atelier | Jet craft |
| `r_rejuv_potion` | Potion de vigueur | 3× healing_herb + 1× alchemic_herb | rejuv_potion ×2 | Herboristerie | 35 | Atelier | Jet craft |
| `r_endurance_potion` | Potion d'endurance | 3× alchemic_herb | endurance_potion ×2 | Herboristerie | 25 | Atelier | Jet craft |
| `r_antidote` | Antidote | 2× alchemic_herb | antidote ×3 | Herboristerie | 15 | Atelier | — |
| `r_steel_sword` | Épée en acier | 5× steel_ore | steel_sword | Mécanique | 50 | Forge | Jet craft |
| `r_steel_mace` | Masse en acier | 4× steel_ore | steel_mace | Mécanique | 50 | Forge | Jet craft |
| `r_steel_axe` | Hache en acier | 4× steel_ore + 1× hardwood | steel_axe | Mécanique | 55 | Forge | Jet craft |
| `r_steel_spear` | Lance en acier | 3× steel_ore + 2× hardwood | steel_spear | Mécanique | 55 | Forge | Jet craft |
| `r_steel_plate` | Plastron en acier | 10× steel_ore | steel_plate | Mécanique | 65 | Forge | Jet craft |
| `r_steel_helmet` | Casque en acier | 3× steel_ore | steel_helmet | Mécanique | 60 | Forge | Jet craft |
| `r_steel_greaves` | Grèves en acier | 6× steel_ore | steel_greaves | Mécanique | 60 | Forge | Jet craft |
| `r_steel_shield` | Bouclier en acier | 5× steel_ore | steel_shield | Mécanique | 60 | Forge | Jet craft |

**Note outils :** la qualité "— " signifie que les outils et munitions n'ont pas de jet de qualité. Ils sont toujours produits en qualité Normal.

---

## 13. Inventaire et stockage

### 13.1 Slots d'inventaire

```
Inventaire du personnage : 20 slots (MVP)
Équipement : 10 slots dédiés (Head, Torso, Legs, Feet, Hands, MainHand, OffHand, Ring, Amulet, Belt)
Ceinture : 2 slots de potion rapides par défaut (portés à 4 avec warrior_belt)
```

### 13.2 Poids et surcharge

Le poids total porté = somme des poids de tous les items en inventaire + équipés.

```rust
fn total_weight(character: &Character) -> u32 {
    let equip_weight: u32 = character.equipment.iter()
        .filter_map(|s| s.as_ref())
        .map(|item| item.weight)
        .sum();

    let inv_weight: u32 = character.inventory.slots.iter()
        .filter_map(|s| s.as_ref())
        .map(|item| item.weight * item.stack_count)
        .sum();

    equip_weight + inv_weight
}

fn is_overloaded(character: &Character) -> bool {
    let pds_max = (character.stats.for_ + character.stats.con) * 5;
    total_weight(character) > pds_max
}
```

Effets de la surcharge : consommation d'Endurance continue = `(poids_total - pds_max)` End/tick (voir document Caractéristiques §9).

### 13.3 Empilement

Les items empilables partagent un slot pour plusieurs unités. L'empilage suit les règles :
- Un slot = un type d'item (`type_id` identique, `quality` identique).
- Le nombre max de la pile est défini dans la config de l'item (`stack_max`).
- Les items équipés ne sont jamais empilés (toujours 1 unité par slot d'équipement).

---

## 14. Configuration JSON — structure des fichiers

### 14.1 item_types.json (extrait)

```json
{
  "item_types": [
    {
      "type_id": "iron_sword",
      "name": "Épée en fer",
      "category": "weapon",
      "subcategory": "sword",
      "weight": 35,
      "stackable": false,
      "stack_max": 1,
      "identified_by_default": true,
      "repair_cost_per_point": 2,
      "weapon_data": {
        "category": "Sword",
        "damage_type": "Tranchant",
        "damage_min": 5,
        "damage_max": 12,
        "speed_modifier": 0,
        "range": 1.5,
        "two_handed": false,
        "bonus_ar": 0,
        "bonus_par": 5
      },
      "equipment_stats": {
        "slot": "MainHand",
        "requirements": { "min_for": 4, "min_agi": 0, "min_dex": 3 },
        "durability_max": 160
      }
    },
    {
      "type_id": "leather_armor",
      "name": "Armure de cuir",
      "category": "armor",
      "subcategory": "torso_light",
      "weight": 40,
      "stackable": false,
      "repair_cost_per_point": 1,
      "armor_data": {
        "category": "Light",
        "ar_tranchant": 5,
        "ar_contondant": 2,
        "ar_perforant": 5,
        "attack_speed_penalty": 0,
        "move_speed_penalty": 0
      },
      "equipment_stats": {
        "slot": "Torso",
        "requirements": { "min_for": 2 },
        "durability_max": 100
      }
    },
    {
      "type_id": "health_potion_small",
      "name": "Petite potion de soin",
      "category": "consumable",
      "weight": 3,
      "stackable": true,
      "stack_max": 20,
      "consumable_data": {
        "effect": "HealHp",
        "value": 30,
        "duration_ticks": 0,
        "cooldown_ticks": 150
      }
    }
  ]
}
```

### 14.2 loot_tables.json (extrait)

```json
{
  "loot_tables": [
    {
      "id": "wolf_loot",
      "gold_min": 3,
      "gold_max": 12,
      "entries": [
        { "item_type": "raw_leather", "probability": 0.70, "count_min": 1, "count_max": 2 },
        { "item_type": "wolfpelt",    "probability": 0.30, "count_min": 1, "count_max": 1 },
        { "item_type": "healing_herb","probability": 0.15, "count_min": 1, "count_max": 3 }
      ]
    }
  ]
}
```

### 14.3 recipes.json (extrait)

```json
{
  "recipes": [
    {
      "id": "r_iron_sword",
      "name": "Épée en fer",
      "inputs": [
        { "type": "iron_ore", "qty": 5 }
      ],
      "output": "iron_sword",
      "output_qty": 1,
      "skill": "mecanique",
      "skill_min": 0,
      "skill_difficulty": 20,
      "station": "forge",
      "quality_by_jet": true
    }
  ]
}
```

---

## 15. Roadmap post-MVP

| Version | Contenu items |
|---------|--------------|
| **v0.2** | Chaînes de production intermédiaires (lingots, planches, cuir travaillé). Items magiques (1–2 propriétés enchantées). Génération d'affixes (pool de préfixes/suffixes par type d'arme). Signature artisan sur les items craftés. Armes à deux mains utilisables avec bouclier via compétence Prise en main. |
| **v0.3** | Items rares (3–6 propriétés). Items de set (bonus de collection). Socketing (encoches + gemmes). Poisons et armes empoisonnées. Armes de jet (haches, couteaux). Dressage : équipement pour animaux (v0.3 si dressage actif). |
| **v0.4** | Items légendaires (uniques nommés). Enchantements runiques. Armures de plate complètes avec mouvements spéciaux. Armes de siège (catapulte, baliste — war engine). |

---

## 16. Références

| Document | Rôle |
|----------|------|
| [Allumina - Caractéristiques, Aptitudes et Compétences](./Concept/Allumina%20-%20Caracteristiques%20Aptitudes%20et%20Competences.md) | Formules atk, par, esq, Pds max, prérequis |
| [Allumina - Combat et Troupes](./Concept/Allumina%20-%20Combat%20et%20Troupes.md) | Séquence de combat, résistance bouclier/parade |
| [Allumina - MVP Sandbox](./Allumina%20-%20MVP%20Sandbox.md) | Système de craft, économie, recettes initiales |
| [Allumina - Extraction Systèmes D2 pour MGE](./Allumina%20-%20Extraction%20Systemes%20D2%20OpenDiablo2%20pour%20MGE.md) | Référence technique items D2 (qualité, affixes, loot) |
| [Allumina - Compétences et Enseignement](./Concept/Allumina%20-%20Competences%20et%20Enseignement.md) | Mécanique, Herboristerie, plafonds |

---

**Document** : Allumina — Objets, Armes, Armures et Équipement
**Version** : 1.0
**Date** : 2026-02-24
**Statut** : Spécification normative game design
