<!-- @id: SD-Tech-ECS-Components @do: reference @role: tech-lead @layer: 3 @human: miyuk -->

# SD-Tech-ECS-Components -- Composants ECS Sodomight

**Auteur :** Denis (Chef Dev Senior, Miyukini AI Studio)
**Date :** 2026-02-28
**Statut :** Reference technique -- v1.0
**Projet :** Sodomight (clone fidele Diablo 2 LoD, assets maison)
**Moteur :** MGE (Miyukini Game Engine) -- ECS archetype maison

---

## Table des matieres

1. [Principes ECS MGE](#1-principes-ecs-mge)
2. [Composants de base (mge-ecs)](#2-composants-de-base-mge-ecs)
3. [Composants de mouvement et position](#3-composants-de-mouvement-et-position)
4. [Composants de rendu et animation](#4-composants-de-rendu-et-animation)
5. [Composants de stats et attributs](#5-composants-de-stats-et-attributs)
6. [Composants de combat](#6-composants-de-combat)
7. [Composants d'items et inventaire](#7-composants-ditems-et-inventaire)
8. [Composants d'IA](#8-composants-dia)
9. [Composants de reseau](#9-composants-de-reseau)
10. [Composants d'interface](#10-composants-dinterface)
11. [Composants audio](#11-composants-audio)
12. [Composants de monde et zones](#12-composants-de-monde-et-zones)
13. [Composants de quetes et scripts](#13-composants-de-quetes-et-scripts)
14. [Sparse overlay -- etats ephemeres](#14-sparse-overlay--etats-ephemeres)
15. [Tags (zero-sized components)](#15-tags-zero-sized-components)
16. [Ressources globales](#16-ressources-globales)
17. [Archetypes principaux](#17-archetypes-principaux)
18. [Invariants et regles](#18-invariants-et-regles)

---

## 1. Principes ECS MGE

### 1.1 Architecture memoire

L'ECS MGE utilise le modele **archetype** : les entites partageant le meme ensemble de composants sont stockees contiguement en memoire (SoA -- Struct of Arrays). Cette organisation maximise la coherence de cache lors des iterations en masse.

```rust
// @id: mge-ecs-archetype @do: define @role: kernel @layer: 1

/// Identifiant unique d'un archetype, derive du hash des TypeId tries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArchetypeId(u64);

/// Stockage en colonnes pour un archetype donne.
pub struct Archetype {
    pub id: ArchetypeId,
    pub component_types: Vec<std::any::TypeId>,  // tries par TypeId
    pub columns: HashMap<std::any::TypeId, ComponentColumn>,
    pub entity_count: usize,
    pub entity_ids: Vec<EntityId>,
}

/// Colonne de donnees brutes pour un type de composant.
pub struct ComponentColumn {
    pub data: Vec<u8>,
    pub layout: std::alloc::Layout,
    pub drop_fn: Option<fn(*mut u8)>,
}
```

### 1.2 Sparse overlay

Pour les etats ephemeres (buffs, debuffs, statuts), un **sparse overlay** separe evite les migrations d'archetype couteuses. Les composants ephemeres ne modifient pas la structure de l'archetype de l'entite.

### 1.3 Conventions de nommage

- Tous les composants derivent `Clone, Debug`
- Les composants serializables derivent `serde::Serialize, serde::Deserialize`
- Les identifiants utilisent `uuid::Uuid` (v4)
- Les timestamps utilisent `i64` (millisecondes epoch) ou `String` ISO 8601
- Les composants sont marques `#[repr(C)]` uniquement si necessaire pour l'alignement GPU
- Aucun `unwrap()` dans les implementations de composants

---

## 2. Composants de base (mge-ecs)

### 2.1 Identification

```rust
// @id: comp-entity-id @do: define @role: kernel @layer: 1

/// Identifiant unique d'une entite dans le monde ECS.
/// Generation permet de detecter les entites recyclees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId {
    /// Index dans le tableau d'entites.
    pub index: u32,
    /// Generation pour invalider les references perimees.
    pub generation: u32,
}

/// Nom lisible d'une entite (debug, UI, chat).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub value: String,
}

/// Identifiant de definition TOML (reference vers le contenu data-driven).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DefId {
    /// Exemple: "monsters/act1/fallen", "items/uniques/shako"
    pub path: String,
}
```

### 2.2 Hierarchie parent-enfant

```rust
// @id: comp-hierarchy @do: define @role: kernel @layer: 1

/// Lien parent d'une entite (pour les projectiles, effets, invocations).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Parent {
    pub entity: EntityId,
}

/// Liste des enfants d'une entite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Children {
    pub entities: Vec<EntityId>,
}
```

### 2.3 Cycle de vie

```rust
// @id: comp-lifetime @do: define @role: kernel @layer: 1

/// Duree de vie restante d'une entite (projectiles, effets visuels).
/// Unite : frames (25 fps = 1 frame = 40ms).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Lifetime {
    pub remaining_frames: u32,
}

/// Age d'un item au sol depuis sa creation (pour timer de loot priority).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DropAge {
    /// Frame a laquelle l'item a ete drop.
    pub dropped_at_frame: u64,
    /// EntityId du joueur ayant droit de priorite (30 secondes = 750 frames).
    pub priority_owner: Option<EntityId>,
}
```

---

## 3. Composants de mouvement et position

### 3.1 Position et transform

```rust
// @id: comp-position @do: define @role: engine @layer: 2
// Crate: mge-ecs (kernel)

/// Position en coordonnees monde (tiles flottants).
/// (0.0, 0.0) = coin haut-gauche de la map.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Position {
    /// Coordonnee X en tiles (1 tile = 64px base).
    pub x: f32,
    /// Coordonnee Y en tiles (1 tile = 32px base, dimetric 2:1).
    pub y: f32,
}

/// Vitesse courante de l'entite (tiles par seconde).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

/// Direction de deplacement et d'orientation (8 directions D2 standard).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

/// Composant de direction courante pour une entite orientee.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Facing {
    pub direction: Direction,
}
```

### 3.2 Mouvement et pathfinding

```rust
// @id: comp-movement @do: define @role: arpg @layer: 3
// Crate: mge-arpg-entity

/// Vitesses de deplacement d'une entite mobile.
/// Toutes les vitesses en tiles par seconde.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MovementSpeed {
    /// Vitesse de marche (walk).
    pub walk_speed: f32,
    /// Vitesse de course (run). Consomme la stamina.
    pub run_speed: f32,
    /// Bonus/malus de vitesse (items, skills, effets). Multiplicateur.
    pub speed_modifier: f32,
}

/// Etat de locomotion courant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocomotionState {
    Idle,
    Walking,
    Running,
    /// En stagger (hit recovery), l'entite ne peut pas se deplacer.
    HitRecovery,
    /// En block recovery, l'entite ne peut pas se deplacer.
    BlockRecovery,
    /// En cast, l'entite ne peut pas se deplacer.
    Casting,
    /// En attaque, l'entite ne peut pas se deplacer (sauf skills mobiles).
    Attacking,
    /// Mort, aucune action possible.
    Dead,
}

/// Composant de locomotion courante.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Locomotion {
    pub state: LocomotionState,
    /// Frame restante dans l'animation courante (stagger, block, cast...).
    pub animation_frames_remaining: u32,
}

/// Chemin calcule par le pathfinding A*.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathPlan {
    /// Liste des tiles a traverser.
    pub waypoints: Vec<(i32, i32)>,
    /// Index courant dans le chemin.
    pub current_index: usize,
    /// Destination finale.
    pub target: (i32, i32),
}

/// Cible de deplacement (clic souris, ordre IA).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MoveTarget {
    pub x: f32,
    pub y: f32,
}
```

---

## 4. Composants de rendu et animation

### 4.1 Sprite et animation

```rust
// @id: comp-render @do: define @role: engine @layer: 2
// Crate: mge-render

/// Identifiant symbolique d'un sprite dans l'asset registry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpriteId(pub String);

/// Composant de rendu sprite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprite {
    /// ID dans le registry (ex: "necro_walk_north").
    pub sprite_id: SpriteId,
    /// Atlas auquel appartient ce sprite.
    pub atlas_id: String,
    /// Region UV dans l'atlas (pixels).
    pub uv_rect: UvRect,
    /// Offset de rendu par rapport a la position (pixels).
    pub offset_x: f32,
    pub offset_y: f32,
    /// Tint color (RGBA, 1.0 = pas de tint).
    pub tint: [f32; 4],
    /// Flip horizontal (pour directions miroir).
    pub flip_h: bool,
    /// Visible ou non.
    pub visible: bool,
    /// Couche de rendu (z-order additionnel dans la meme profondeur iso).
    pub render_layer: RenderLayer,
}

/// Rectangle UV dans un atlas.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UvRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Couches de rendu ordonnees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RenderLayer {
    /// Sol, tiles de base.
    Floor = 0,
    /// Ombres des entites.
    Shadow = 1,
    /// Items au sol.
    GroundItem = 2,
    /// Entites (joueurs, monstres, PNJ).
    Entity = 3,
    /// Effets au-dessus des entites (auras, buffs visuels).
    OverlayEffect = 4,
    /// Projectiles.
    Projectile = 5,
    /// Murs, obstacles au premier plan.
    Foreground = 6,
    /// Meteo (pluie, neige).
    Weather = 7,
    /// HUD, interface.
    Ui = 8,
}
```

### 4.2 Etat d'animation

```rust
// @id: comp-anim-state @do: define @role: engine @layer: 2

/// Identifiant d'une animation (combinaison action + direction).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimAction {
    Idle,
    Walk,
    Run,
    Attack1,
    Attack2,
    Cast,
    GetHit,
    Block,
    Death,
    /// Skill specifique (ex: "whirlwind", "bone_spear_cast").
    Skill(String),
    /// Resurrection, town portal, etc.
    Special(String),
}

/// Composant d'etat d'animation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimState {
    /// Action courante.
    pub action: AnimAction,
    /// Direction courante (pour choisir la spritesheet).
    pub direction: Direction,
    /// Frame courante dans l'animation.
    pub current_frame: u32,
    /// Nombre total de frames de cette animation.
    pub total_frames: u32,
    /// Frames par seconde de l'animation.
    pub fps: f32,
    /// Temps accumule depuis la derniere frame (secondes).
    pub elapsed: f32,
    /// L'animation boucle-t-elle ?
    pub looping: bool,
    /// L'animation est-elle terminee (pour les one-shot) ?
    pub finished: bool,
}
```

### 4.3 Effets visuels

```rust
// @id: comp-vfx @do: define @role: engine @layer: 2

/// Composant d'effet visuel temporaire (explosion, aura, impact).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEffect {
    /// Type d'effet.
    pub effect_type: EffectType,
    /// Duree totale en frames.
    pub duration_frames: u32,
    /// Frame courante.
    pub current_frame: u32,
    /// Echelle de l'effet (1.0 = taille normale).
    pub scale: f32,
    /// Opacite (0.0 = invisible, 1.0 = opaque).
    pub alpha: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    Explosion,
    AuraGlow,
    Impact,
    Heal,
    LevelUp,
    CurseOverlay,
    PoisonCloud,
    FrostNova,
    FireWall,
    LightningBolt,
    /// Effet personnalise defini dans les TOML.
    Custom(String),
}
```

---

## 5. Composants de stats et attributs

### 5.1 Attributs de base

```rust
// @id: comp-stats-base @do: define @role: arpg @layer: 3
// Crate: mge-arpg-stats

/// Classe du personnage (7 classes D2 avec noms Sodomight).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CharacterClass {
    Sarith,        // Amazon
    Mortecian,     // Necromancer
    Ravageur,      // Barbarian
    Arcaniste,     // Sorceress
    CroiseSolaire, // Paladin
    Animiste,      // Druid
    Ombrelame,     // Assassin
}

/// Attributs primaires du personnage.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BaseAttributes {
    pub strength: i32,
    pub dexterity: i32,
    pub vitality: i32,
    pub energy: i32,
}

/// Points non distribues.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct UnspentPoints {
    pub stat_points: i32,
    pub skill_points: i32,
}

/// Stats de base du personnage (classe, niveau).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInfo {
    pub class: CharacterClass,
    pub level: u8,
    pub experience: u64,
    /// Points d'experience requis pour le prochain niveau.
    pub experience_next_level: u64,
    /// Nom du personnage.
    pub name: String,
    /// Mode hardcore.
    pub hardcore: bool,
}
```

### 5.2 Stats derivees

```rust
// @id: comp-stats-derived @do: define @role: arpg @layer: 3

/// Pool de vie, mana, stamina.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VitalPools {
    pub life_current: i32,
    pub life_max: i32,
    pub mana_current: i32,
    pub mana_max: i32,
    pub stamina_current: i32,
    pub stamina_max: i32,
}

/// Vitesse de regeneration des pools (par frame, a 25 fps).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Regeneration {
    /// Mana regeneree par frame.
    /// Formule: MaxMana * (100 + RegenMana%) / (120 * 25 * 100)
    pub mana_per_frame: f32,
    /// Life regeneree par frame (items "Replenish Life").
    pub life_per_frame: f32,
    /// Stamina regeneree par frame (en idle/walk).
    pub stamina_per_frame: f32,
}

/// Defense Rating (affecte la chance de toucher de l'attaquant).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Defense {
    /// Defense de base (armure + bonus).
    pub base_defense: i32,
    /// Defense totale avec tous les modificateurs.
    pub total_defense: i32,
}

/// Attack Rating (chance de toucher du personnage).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AttackRating {
    /// AR de base: (DEX - 7) * 5 + ClassBaseAR
    pub base_ar: i32,
    /// AR total avec bonus d'items et skills.
    pub total_ar: i32,
}

/// Resistances elementaires et physiques.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Resistances {
    /// Resistance au feu (-100 a +75 en normal, -100 a +95 avec items).
    pub fire: i32,
    /// Resistance au froid.
    pub cold: i32,
    /// Resistance a la foudre.
    pub lightning: i32,
    /// Resistance au poison.
    pub poison: i32,
    /// Resistance magique (tres rare).
    pub magic: i32,
    /// Resistance physique (Damage Reduction %).
    pub physical_pct: i32,
    /// Reduction de dommage fixe (Damage Reduction flat).
    pub physical_flat: i32,
}

/// Caps de resistance par difficulte.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResistanceCaps {
    /// Cap de base (75 par defaut, +5 par source specifique).
    pub fire_cap: i32,
    pub cold_cap: i32,
    pub lightning_cap: i32,
    pub poison_cap: i32,
}

/// Penalites de resistance par difficulte.
/// Normal: 0, Nightmare: -40, Hell: -100
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ResistancePenalty {
    pub fire_penalty: i32,
    pub cold_penalty: i32,
    pub lightning_penalty: i32,
    pub poison_penalty: i32,
}
```

### 5.3 Breakpoints

```rust
// @id: comp-breakpoints @do: define @role: arpg @layer: 3

/// Breakpoints courants du personnage (frames d'animation resolues).
/// Precalcules a chaque changement d'equipement ou de buff.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Breakpoints {
    /// Faster Cast Rate total (%).
    pub fcr_total: i32,
    /// Frames pour lancer un sort (resolve via table).
    pub cast_frames: u32,

    /// Faster Hit Recovery total (%).
    pub fhr_total: i32,
    /// Frames de stagger (resolve via table).
    pub hit_recovery_frames: u32,

    /// Faster Block Rate total (%).
    pub fbr_total: i32,
    /// Frames de blocage (resolve via table).
    pub block_frames: u32,

    /// Increased Attack Speed total (items, %).
    pub ias_total: i32,
    /// EIAS effectif: floor(120 * IAS / (120 + IAS))
    pub eias: i32,
    /// Skill IAS (Fanaticism, Burst of Speed, etc.).
    pub sias: i32,
    /// Weapon Speed Modifier de l'arme equipe.
    pub wsm: i32,
    /// Frames d'attaque resolues.
    pub attack_frames: u32,
}

/// Chance to Block calculee.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockChance {
    /// CTB en stand: min(floor((ShieldBlock + Bonus) * (DEX-15) / (clvl*2)), 75)
    pub standing: i32,
    /// CTB en run: min(floor(CTB / 3), 25)
    pub running: i32,
}
```

### 5.4 Stats de combat avancees

```rust
// @id: comp-combat-stats @do: define @role: arpg @layer: 3

/// Stats de combat speciales issues des items et skills.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CombatBonuses {
    /// Enhanced Damage on-weapon (%).
    pub ed_on_weapon: i32,
    /// Enhanced Damage off-weapon total (%).
    pub ed_off_weapon: i32,
    /// Flat damage bonus min.
    pub flat_damage_min: i32,
    /// Flat damage bonus max.
    pub flat_damage_max: i32,

    /// Critical Strike chance (skill, %).
    pub critical_strike_pct: i32,
    /// Deadly Strike chance (item, %).
    pub deadly_strike_pct: i32,
    /// Crushing Blow chance (%).
    pub crushing_blow_pct: i32,
    /// Open Wounds chance (%).
    pub open_wounds_pct: i32,

    /// Chance de toucher magique (Ignore Target's Defense).
    pub ignore_target_defense: bool,
    /// Prevent Monster Heal.
    pub prevent_monster_heal: bool,
    /// Cannot Be Frozen.
    pub cannot_be_frozen: bool,
    /// Half Freeze Duration.
    pub half_freeze_duration: bool,

    /// Life Steal (%).
    pub life_steal_pct: i32,
    /// Mana Steal (%).
    pub mana_steal_pct: i32,

    /// Magic Find total (%).
    pub magic_find: i32,
    /// Gold Find total (%).
    pub gold_find: i32,

    /// Damage to Undead (%).
    pub damage_to_undead: i32,
    /// Damage to Demons (%).
    pub damage_to_demons: i32,
    /// Attack Rating vs Undead.
    pub ar_vs_undead: i32,
    /// Attack Rating vs Demons.
    pub ar_vs_demons: i32,
}

/// Dommages elementaires bonus (depuis items, skills, auras).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ElementalDamage {
    pub fire_min: i32,
    pub fire_max: i32,
    pub cold_min: i32,
    pub cold_max: i32,
    /// Duree du slow Cold en frames.
    pub cold_duration_frames: u32,
    pub lightning_min: i32,
    pub lightning_max: i32,
    pub poison_total: i32,
    /// Duree du poison en frames.
    pub poison_duration_frames: u32,
    pub magic_min: i32,
    pub magic_max: i32,
}
```

---

## 6. Composants de combat

### 6.1 Composants d'attaque

```rust
// @id: comp-combat @do: define @role: arpg @layer: 3
// Crate: mge-arpg-combat

/// Arme equipee courante (pour les calculs de dommage).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquippedWeapon {
    /// ID de la definition d'arme.
    pub item_def_id: String,
    /// Type d'arme (pour STR/DEX factors).
    pub weapon_type: WeaponType,
    /// Dommage de base min.
    pub base_damage_min: i32,
    /// Dommage de base max.
    pub base_damage_max: i32,
    /// Arme ethereal (+50% damage).
    pub ethereal: bool,
    /// Weapon Speed Modifier.
    pub wsm: i32,
    /// Range de l'arme (tiles).
    pub range: f32,
    /// Type de dommage physique.
    pub attack_type: AttackType,
}

/// Type d'arme (pour les facteurs STR/DEX du bonus de dommage).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,        // 100% STR
    Axe,          // 100% STR
    Mace,         // 110% STR
    Staff,        // 100% STR
    Spear,        // 100% STR
    Polearm,      // 100% STR
    Dagger,       // 50% STR + 50% DEX
    Throwing,     // 50% STR + 50% DEX
    Bow,          // 100% DEX
    Crossbow,     // 100% DEX
    Javelin,      // 50% STR + 50% DEX
    Claw,         // 75% STR + 75% DEX
    Wand,         // 100% STR
    Orb,          // 100% STR
}

/// Type d'attaque.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackType {
    Melee,
    Ranged,
    Thrown,
}

/// Hitbox circulaire pour la detection de collision combat.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CircleHitbox {
    pub radius: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

/// Attaque en cours d'execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveAttack {
    /// Skill utilise (None = auto-attack).
    pub skill_id: Option<String>,
    /// Cible de l'attaque.
    pub target: AttackTarget,
    /// Frame courante de l'animation d'attaque.
    pub current_frame: u32,
    /// Frame a laquelle le dommage est applique (hit frame).
    pub hit_frame: u32,
    /// Frame totale de l'animation.
    pub total_frames: u32,
    /// Le hit a-t-il deja ete applique ?
    pub hit_applied: bool,
}

/// Cible d'une attaque.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AttackTarget {
    /// Attaque une entite specifique.
    Entity(EntityId),
    /// Attaque une position (AoE, projectile).
    Position { x: f32, y: f32 },
    /// Attaque en direction (auto-target le plus proche).
    Direction(Direction),
}
```

### 6.2 Projectiles

```rust
// @id: comp-projectile @do: define @role: arpg @layer: 3

/// Donnees d'un projectile en vol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectileData {
    /// Entite source (le lanceur).
    pub source: EntityId,
    /// Skill source.
    pub skill_id: String,
    /// Vitesse du projectile (tiles/sec).
    pub speed: f32,
    /// Dommage physique (min, max).
    pub phys_damage: (i32, i32),
    /// Dommages elementaires.
    pub elem_damage: ElementalDamage,
    /// Rayon de l'AoE a l'impact (0 = impact simple).
    pub aoe_radius: f32,
    /// Peut traverser des ennemis (pierce) ?
    pub piercing: bool,
    /// Nombre de cibles pouvant etre traversees (-1 = infini).
    pub pierce_count: i32,
    /// Nombre de cibles deja touchees.
    pub targets_hit: i32,
    /// Entites deja touchees (pour eviter le multi-hit).
    pub hit_list: Vec<EntityId>,
}

/// Missile guide (suivi de cible).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Homing {
    pub target: EntityId,
    /// Vitesse angulaire de rotation (radians/sec).
    pub turn_rate: f32,
}
```

### 6.3 Area of Effect (AoE)

```rust
// @id: comp-aoe @do: define @role: arpg @layer: 3

/// Zone d'effet active au sol (Fire Wall, Blizzard, Poison Nova...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaOfEffect {
    /// Position centre de la zone.
    pub center_x: f32,
    pub center_y: f32,
    /// Rayon en tiles.
    pub radius: f32,
    /// Dommage par frame.
    pub damage_per_frame: i32,
    /// Type de dommage.
    pub damage_type: DamageElement,
    /// Duree restante en frames.
    pub remaining_frames: u32,
    /// Frequence de tick de dommage (toutes les N frames).
    pub tick_interval: u32,
    /// Frame du dernier tick.
    pub last_tick_frame: u32,
    /// Source (pour attribution de kill).
    pub source: EntityId,
}

/// Elements de dommage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageElement {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Magic,
}
```

---

## 7. Composants d'items et inventaire

### 7.1 Donnees d'item

```rust
// @id: comp-items @do: define @role: arpg @layer: 3
// Crate: mge-arpg-items

/// Qualite d'un item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemQuality {
    Normal,
    Superior,
    Magic,
    Rare,
    Set,
    Unique,
    Crafted,
    Runeword,
}

/// Tier de l'item base.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemTier {
    Normal,
    Exceptional,
    Elite,
}

/// Donnees completes d'un item instancie.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    /// UUID unique de cette instance d'item.
    pub instance_id: uuid::Uuid,
    /// Reference vers la definition TOML.
    pub def_id: String,
    /// Nom genere (pour Rare, le nom est aleatoire).
    pub display_name: String,
    /// Type de base (ex: "long_sword", "gothic_plate").
    pub base_type: String,
    /// Categorie (arme, armure, potion, etc.).
    pub category: ItemCategory,
    /// Qualite.
    pub quality: ItemQuality,
    /// Tier.
    pub tier: ItemTier,
    /// Niveau de l'item (ilvl) -- determine les affixes possibles.
    pub item_level: u8,
    /// Prerequis de force.
    pub required_strength: i32,
    /// Prerequis de dexterite.
    pub required_dexterity: i32,
    /// Niveau requis.
    pub required_level: u8,
    /// Ethereal (bonus +50% damage/defense, non reparable).
    pub ethereal: bool,
    /// Identifie ?
    pub identified: bool,
    /// Taille dans l'inventaire (colonnes, rangees).
    pub grid_size: (u8, u8),
}

/// Categorie d'item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemCategory {
    Weapon,
    Armor,
    Shield,
    Helm,
    Gloves,
    Boots,
    Belt,
    Ring,
    Amulet,
    Charm,
    Potion,
    Scroll,
    Tome,
    Key,
    Gem,
    Rune,
    Jewel,
    Quest,
    Gold,
    Misc,
}

/// Durabilite d'un item.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Durability {
    pub current: i32,
    pub max: i32,
    /// Indestructible ?
    pub indestructible: bool,
}
```

### 7.2 Affixes et proprietes

```rust
// @id: comp-affixes @do: define @role: arpg @layer: 3

/// Affixe magique sur un item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affix {
    /// ID du prefix ou suffix (ref TOML).
    pub affix_id: String,
    /// Nom de l'affixe (ex: "of the Whale", "Cruel").
    pub name: String,
    /// Prefix ou suffix.
    pub affix_type: AffixType,
    /// Proprietes conferees par cet affixe.
    pub properties: Vec<ItemProperty>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AffixType {
    Prefix,
    Suffix,
}

/// Propriete unitaire d'un item (stat + valeur).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemProperty {
    /// Identifiant de stat (ex: "strength", "fire_resist", "faster_cast_rate").
    pub stat: String,
    /// Valeur de la propriete.
    pub value: PropertyValue,
}

/// Valeur d'une propriete (fixe, range, par niveau...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    /// Valeur fixe.
    Flat(i32),
    /// Range (valeur roulable entre min et max).
    Range { min: i32, max: i32, rolled: i32 },
    /// Pourcentage.
    Percent(i32),
    /// Par niveau du personnage.
    PerLevel { value_per_level: f32 },
}

/// Affixes complets d'un item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAffixes {
    /// Prefixes (max 3 pour Rare, 1 pour Magic).
    pub prefixes: Vec<Affix>,
    /// Suffixes (max 3 pour Rare, 1 pour Magic).
    pub suffixes: Vec<Affix>,
    /// Proprietes intrinseques (Unique, Set, Runeword).
    pub intrinsic_properties: Vec<ItemProperty>,
}
```

### 7.3 Sockets et runes

```rust
// @id: comp-sockets @do: define @role: arpg @layer: 3

/// Sockets d'un item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sockets {
    /// Nombre total de sockets.
    pub count: u8,
    /// Items socketes (runes, gems, jewels). None = socket vide.
    pub filled: Vec<Option<SocketedItem>>,
}

/// Item insere dans un socket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketedItem {
    /// Type d'item sockete.
    pub socket_type: SocketItemType,
    /// ID de la rune ou du gem (ex: "rune_ber", "gem_perfect_amethyst").
    pub item_id: String,
    /// Proprietes conferees par cet item sockete.
    pub properties: Vec<ItemProperty>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocketItemType {
    Rune,
    Gem,
    Jewel,
}

/// Runeword active sur un item (quand la combinaison de runes est correcte).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRuneword {
    /// ID du runeword (ex: "spirit_shield", "enigma").
    pub runeword_id: String,
    /// Nom du runeword.
    pub name: String,
    /// Proprietes conferees par le runeword (en plus des proprietes individuelles des runes).
    pub properties: Vec<ItemProperty>,
}
```

### 7.4 Inventaire et equipement

```rust
// @id: comp-inventory @do: define @role: arpg @layer: 3

/// Inventaire grille 10x4 du personnage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    /// Grille 10 colonnes x 4 rangees.
    /// Chaque cellule contient None ou l'UUID de l'item qui occupe cette case.
    pub grid: [[Option<uuid::Uuid>; 10]; 4],
    /// Gold en inventaire (cap: 10000 * clvl).
    pub gold: i64,
}

/// Equipement (paperdoll).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub helm: Option<uuid::Uuid>,
    pub amulet: Option<uuid::Uuid>,
    pub armor: Option<uuid::Uuid>,
    pub main_hand: Option<uuid::Uuid>,
    pub off_hand: Option<uuid::Uuid>,
    pub gloves: Option<uuid::Uuid>,
    pub belt_item: Option<uuid::Uuid>,
    pub boots: Option<uuid::Uuid>,
    pub ring_left: Option<uuid::Uuid>,
    pub ring_right: Option<uuid::Uuid>,
    /// Weapon swap (Set 2).
    pub swap_main_hand: Option<uuid::Uuid>,
    pub swap_off_hand: Option<uuid::Uuid>,
}

/// Ceinture de potions (belt).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PotionBelt {
    /// Nombre de rangees (2, 3 ou 4 selon le type de ceinture equipe).
    pub rows: u8,
    /// 4 colonnes x N rangees. Chaque slot contient l'UUID d'une potion ou None.
    pub slots: Vec<[Option<uuid::Uuid>; 4]>,
}

/// Stash (coffre personnel et partage).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stash {
    /// Onglet courant.
    pub current_tab: u8,
    /// Grilles par onglet (10x10 = 100 slots chacun).
    pub tabs: Vec<StashTab>,
    /// Gold dans le stash (cap: 2 500 000).
    pub gold: i64,
}

/// Onglet de stash.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StashTab {
    pub name: String,
    /// true = onglet partage entre personnages du compte.
    pub shared: bool,
    /// Grille 10x10.
    pub grid: [[Option<uuid::Uuid>; 10]; 10],
}

/// Cube Alchimique (Horadric Cube).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemicalCube {
    /// Grille interne 3x4 (12 slots).
    pub grid: [[Option<uuid::Uuid>; 3]; 4],
}
```

---

## 8. Composants d'IA

### 8.1 IA de monstres

```rust
// @id: comp-ai @do: define @role: arpg @layer: 3
// Crate: mge-arpg-ai

/// Archetype d'IA (comportement de base).
/// 43 archetypes identifies dans SD-Monsters-AI.md.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiArchetype {
    MeleeFollower,
    MeleeCharger,
    MeleeHitAndRun,
    MeleeStationary,
    MeleeBerserker,
    MeleeCowardLeader,
    MeleeSwarm,
    MeleePack,
    MeleeAmbush,
    RangedSkirmisher,
    RangedStationary,
    RangedBomber,
    RangedSniper,
    RangedMortar,
    CasterOffensive,
    CasterSupport,
    CasterSummoner,
    CasterCurser,
    CasterAreaDenial,
    HybridMeleeCast,
    HybridRangedMelee,
    HybridPhaseShifter,
    SpecialBurrower,
    SpecialFlyer,
    SpecialTeleporter,
    SpecialStealth,
    SpecialMultiPhase,
    SpecialMiniBoss,
    BossPhased,
    BossEnraged,
    BossSummoner,
    BossAreaControl,
    BossMultiForm,
    PackLeader,
    PackFollower,
    Scavenger,
    Coward,
    Patrol,
    Guard,
    Idle,
    Fleeing,
    Returning,
    Custom(u32),
}

/// Composant d'IA attache a une entite monstre.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiState {
    /// Archetype d'IA de base.
    pub archetype: AiArchetype,
    /// Etat courant de l'IA.
    pub current_state: AiBehaviorState,
    /// Cible courante (entite agressee).
    pub target: Option<EntityId>,
    /// Timer avant de changer d'etat (frames).
    pub state_timer: u32,
    /// Position de spawn (pour le leashing).
    pub home_position: (f32, f32),
    /// Rayon de leash (distance max depuis le spawn avant de retourner).
    pub leash_radius: f32,
    /// Rayon d'aggro (distance de detection des joueurs).
    pub aggro_radius: f32,
}

/// Etats de comportement IA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiBehaviorState {
    Idle,
    Patrol,
    Aggro,
    Chase,
    Attack,
    Cast,
    Flee,
    ReturnHome,
    Staggered,
    Dead,
    /// Phase de boss specifique.
    BossPhase(u8),
}

/// Donnees de monstre specifiques.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonsterData {
    /// Type de monstre.
    pub monster_type: MonsterType,
    /// Niveau du monstre.
    pub monster_level: u8,
    /// Acte d'origine.
    pub act: u8,
    /// Experience octroyee au kill.
    pub experience: u32,
    /// Table de loot (reference TOML).
    pub loot_table: String,
    /// Immunites (pour NM/Hell).
    pub immunities: Vec<DamageElement>,
    /// Affixes de champion/unique (si applicable).
    pub affixes: Vec<String>,
}

/// Type de monstre.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonsterType {
    Normal,
    Minion,
    Champion,
    Unique,
    SuperUnique,
    ActBoss,
}

/// Parametres specifiques de l'IA du monstre (charges dans le TOML).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiParams {
    /// Seuil de fuite (% de vie).
    pub flee_hp_pct: Option<f32>,
    /// Peut rallier d'autres monstres en fuyant.
    pub rally_on_flee: bool,
    /// Delai entre deux attaques (frames).
    pub attack_cooldown: u32,
    /// Liste des skills utilisables par l'IA.
    pub skills: Vec<AiSkillEntry>,
    /// Comportement specifique.
    pub special_behavior: Option<String>,
}

/// Entree de skill dans l'IA.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSkillEntry {
    /// ID du skill.
    pub skill_id: String,
    /// Priorite (plus haut = utilise en premier si conditions remplies).
    pub priority: u8,
    /// Condition d'utilisation.
    pub condition: AiSkillCondition,
    /// Cooldown en frames.
    pub cooldown: u32,
}

/// Conditions pour qu'un monstre utilise un skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AiSkillCondition {
    /// Toujours utilisable.
    Always,
    /// Cible dans le rayon donne (tiles).
    TargetInRange(f32),
    /// Vie sous un seuil (%).
    HealthBelow(f32),
    /// Timer ecoule (frames depuis le dernier usage).
    CooldownReady,
    /// Nombre d'allies a proximite superieur a N.
    AlliesNearby(u32),
}
```

---

## 9. Composants de reseau

```rust
// @id: comp-network @do: define @role: engine @layer: 2
// Crate: mge-net

/// Identifiant de joueur en reseau.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerId(pub u8);

/// Composant de synchronisation reseau (attache aux entites repliquees).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NetSync {
    /// ID reseau unique de l'entite (attribue par le host).
    pub net_id: u32,
    /// Le host est-il autoritaire sur cette entite ?
    pub authoritative: bool,
    /// Derniere frame de mise a jour recue.
    pub last_update_frame: u64,
}

/// Etat d'input du joueur (envoye par le client au host).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputState {
    /// Position du clic de deplacement (si applicable).
    pub move_target: Option<(f32, f32)>,
    /// Skill actif main gauche.
    pub lmb_skill: Option<String>,
    /// Cible du skill main gauche.
    pub lmb_target: Option<AttackTarget>,
    /// Skill actif main droite.
    pub rmb_skill: Option<String>,
    /// Cible du skill main droite.
    pub rmb_target: Option<AttackTarget>,
    /// Touches de potion (1-4 = colonnes de belt).
    pub potion_use: Option<u8>,
    /// Frame du client (pour la compensation de latence).
    pub client_frame: u64,
}

/// Interpolation reseau pour le rendu smooth des entites distantes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct NetInterpolation {
    /// Position precedente (pour interpolation).
    pub prev_x: f32,
    pub prev_y: f32,
    /// Position cible (derniere update recue).
    pub target_x: f32,
    pub target_y: f32,
    /// Facteur d'interpolation (0.0 a 1.0).
    pub lerp_factor: f32,
}
```

---

## 10. Composants d'interface

```rust
// @id: comp-ui @do: define @role: engine @layer: 2
// Crate: mge-ui

/// Etat des fenetres ouvertes du joueur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiState {
    pub inventory_open: bool,
    pub character_sheet_open: bool,
    pub skill_tree_open: bool,
    pub quest_log_open: bool,
    pub party_panel_open: bool,
    pub automap_visible: bool,
    pub chat_open: bool,
    pub cube_open: bool,
    pub stash_open: bool,
    pub trade_open: bool,
    pub shop_open: bool,
    /// NPC avec lequel on interagit.
    pub interacting_npc: Option<EntityId>,
}

/// Item "en main" (drag and drop en cours).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorItem {
    /// UUID de l'item tenu par le curseur.
    pub item_id: uuid::Uuid,
    /// Source de l'item (pour annulation).
    pub source: CursorItemSource,
}

/// Source d'un item en curseur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CursorItemSource {
    Inventory { col: u8, row: u8 },
    Equipment { slot: String },
    Stash { tab: u8, col: u8, row: u8 },
    Cube { col: u8, row: u8 },
    Belt { col: u8, row: u8 },
    Ground { x: f32, y: f32 },
    Shop { index: u32 },
}

/// Tooltip actuellement affiche.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTooltip {
    /// Contenu du tooltip.
    pub lines: Vec<TooltipLine>,
    /// Position ecran (pixels).
    pub screen_x: f32,
    pub screen_y: f32,
}

/// Ligne de tooltip avec couleur.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipLine {
    pub text: String,
    /// Couleur RGBA hex (ex: "#6969FF" pour Magic).
    pub color: String,
    pub bold: bool,
}

/// Filtre de loot (regles de visibilite des items au sol).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootFilter {
    /// Regles ordonnees (premiere regle qui match = appliquee).
    pub rules: Vec<LootFilterRule>,
    /// Actif ou non.
    pub enabled: bool,
}

/// Regle de filtre de loot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootFilterRule {
    /// Conditions de match.
    pub conditions: Vec<LootFilterCondition>,
    /// Action si match.
    pub action: LootFilterAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LootFilterCondition {
    Quality(ItemQuality),
    Category(ItemCategory),
    BaseType(String),
    ItemLevelMin(u8),
    ItemLevelMax(u8),
    Rune,
    Gem,
    Gold,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LootFilterAction {
    Show,
    Hide,
    Highlight,
    PlaySound,
}
```

---

## 11. Composants audio

```rust
// @id: comp-audio @do: define @role: engine @layer: 2
// Crate: mge-audio

/// Source sonore spatialisee attachee a une entite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSource {
    /// ID du son dans le registry (ex: "sfx/sword_hit_01").
    pub sound_id: String,
    /// Couche audio.
    pub layer: AudioLayer,
    /// Volume (0.0 a 1.0).
    pub volume: f32,
    /// Portee de spatialisation (tiles). 0 = non-spatial.
    pub spatial_range: f32,
    /// Boucle ?
    pub looping: bool,
}

/// Couches audio (5 couches D2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioLayer {
    Music,
    Ambient,
    Sfx,
    Ui,
    Voice,
}

/// Musique de zone courante.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneMusic {
    /// ID du track de musique.
    pub track_id: String,
    /// En crossfade ? (transition entre zones).
    pub crossfading: bool,
    /// Volume cible du crossfade (0.0 a 1.0).
    pub crossfade_target: f32,
    /// Duree du crossfade en secondes.
    pub crossfade_duration: f32,
}
```

---

## 12. Composants de monde et zones

```rust
// @id: comp-world @do: define @role: arpg @layer: 3
// Crate: mge-arpg-world

/// Zone courante de l'entite.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentZone {
    /// ID de la zone (ex: "act1/blood_moor").
    pub zone_id: String,
    /// Acte.
    pub act: u8,
    /// Difficulte courante.
    pub difficulty: Difficulty,
    /// Niveau de la zone (alvl) pour la difficulte courante.
    pub area_level: u8,
}

/// Difficulte du jeu.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Normal,
    Nightmare,
    Hell,
}

/// Waypoint decouvert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointData {
    /// ID du waypoint.
    pub waypoint_id: String,
    /// Zone associee.
    pub zone_id: String,
    /// Acte.
    pub act: u8,
    /// Nom affiche.
    pub display_name: String,
    /// Actif (decouvert par le joueur).
    pub activated: bool,
}

/// Donnees d'un waypoint decouvert par un personnage (pour sauvegarde).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredWaypoints {
    /// Set des waypoint_id decouverts, par difficulte.
    pub normal: Vec<String>,
    pub nightmare: Vec<String>,
    pub hell: Vec<String>,
}

/// Portal de ville actif.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownPortal {
    /// Joueur proprietaire du portal.
    pub owner: EntityId,
    /// Zone source (ou le portal a ete ouvert).
    pub source_zone: String,
    /// Position source.
    pub source_position: (f32, f32),
    /// Zone cible (ville de l'acte courant).
    pub target_zone: String,
    /// Duree de vie restante (frames). None = permanent jusqu'a utilisation.
    pub lifetime: Option<u32>,
}

/// Shrine actif dans la zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShrineData {
    /// Type de shrine (15 types identifies).
    pub shrine_type: ShrineType,
    /// Duree de l'effet en frames (si applicable).
    pub effect_duration: u32,
    /// Deja utilise ?
    pub used: bool,
}

/// Types de shrines.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShrineType {
    Health,
    Mana,
    Stamina,
    Experience,
    Fire,
    Resist,
    Skill,
    ManaRecharge,
    Armor,
    Combat,
    Gem,
    Monster,
    Exploding,
    Poison,
    Wells,
}

/// Point d'interaction dans le monde (coffre, barrel, door, lever...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interactable {
    /// Type d'interaction.
    pub interaction_type: InteractionType,
    /// Etat (ouvert, ferme, detruit...).
    pub state: InteractionState,
    /// Contenu lootable (si applicable).
    pub loot_table: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    Chest,
    SuperChest,
    Barrel,
    Urn,
    Door,
    Lever,
    StairUp,
    StairDown,
    Well,
    Shrine,
    Waypoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionState {
    Closed,
    Open,
    Destroyed,
    Locked,
    Activated,
}
```

---

## 13. Composants de quetes et scripts

```rust
// @id: comp-quests @do: define @role: arpg @layer: 3
// Crate: mge-arpg-quest

/// Etat de progression des quetes d'un personnage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestLog {
    /// Quetes par difficulte. Cle = quest_id, Valeur = etat.
    pub normal: HashMap<String, QuestState>,
    pub nightmare: HashMap<String, QuestState>,
    pub hell: HashMap<String, QuestState>,
}

/// Etat d'une quete individuelle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestState {
    /// Completee ?
    pub completed: bool,
    /// Recompense reclamee ?
    pub reward_claimed: bool,
    /// Etape courante (pour les quetes multi-etapes).
    pub current_step: u8,
    /// Flags specifiques a la quete.
    pub flags: HashMap<String, bool>,
    /// Compteurs (kill counts, items collectes...).
    pub counters: HashMap<String, i32>,
}

/// Reference vers un script Rhai attache a une entite (trigger, quest, dialogue).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptRef {
    /// Chemin du script Rhai (ex: "quests/act1/den_of_evil.rhai").
    pub script_path: String,
    /// Fonction a appeler dans le script.
    pub entry_function: String,
}

/// Zone de trigger (declenche un script quand un joueur entre).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerZone {
    /// Bounds rectangulaires de la zone (tiles).
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    /// Script a executer.
    pub script: ScriptRef,
    /// One-shot (se desactive apres le premier trigger) ou repeatable.
    pub one_shot: bool,
    /// Deja declenche ?
    pub triggered: bool,
}

/// Donnees NPC (vendeur, sage, quete...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcData {
    /// ID du NPC (ex: "npc_akara", "npc_aldric").
    pub npc_id: String,
    /// Type de NPC.
    pub npc_type: NpcType,
    /// Services disponibles.
    pub services: Vec<NpcService>,
    /// Script de dialogue.
    pub dialogue_script: Option<ScriptRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcType {
    QuestGiver,
    Vendor,
    Healer,
    Identifier,
    Blacksmith,
    Gambler,
    Mercenary,
    Lore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NpcService {
    Buy,
    Sell,
    Repair,
    Identify,
    IdentifyAll,
    Gamble,
    HireMercenary,
    Heal,
    Resurrect,
}
```

---

## 14. Sparse overlay -- etats ephemeres

Les etats ephemeres sont stockes dans un **sparse overlay** separe de l'archetype principal. L'ajout ou la suppression d'un etat ephemere ne cause PAS de migration d'archetype.

```rust
// @id: comp-sparse-overlay @do: define @role: arpg @layer: 3
// Crate: mge-arpg-combat (overlay states)

/// Overlay complet des etats ephemeres.
pub struct StatusOverlay {
    pub poison:       SparseMap<EntityId, PoisonState>,
    pub frozen:        SparseMap<EntityId, FrozenState>,
    pub chilled:       SparseMap<EntityId, ChilledState>,
    pub stunned:       SparseMap<EntityId, StunnedState>,
    pub cursed:        SparseMap<EntityId, CursedState>,
    pub amplified:     SparseMap<EntityId, AmplifiedState>,
    pub decrepified:   SparseMap<EntityId, DecrepifiedState>,
    pub lower_resist:  SparseMap<EntityId, LowerResistState>,
    pub conviction:    SparseMap<EntityId, ConvictionState>,
    pub open_wounds:   SparseMap<EntityId, OpenWoundsState>,
    pub blinded:       SparseMap<EntityId, BlindedState>,
    pub confused:      SparseMap<EntityId, ConfusedState>,
    pub slowed:        SparseMap<EntityId, SlowedState>,
    pub knocked_back:  SparseMap<EntityId, KnockedBackState>,
    pub converted:     SparseMap<EntityId, ConvertedState>,
    pub iron_maiden:   SparseMap<EntityId, IronMaidenState>,
    pub life_tap:      SparseMap<EntityId, LifeTapState>,
}

/// Etat de poison (DoT).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PoisonState {
    /// Dommage total restant a appliquer.
    pub total_damage_remaining: i32,
    /// Frames restantes.
    pub frames_remaining: u32,
    /// Dommage par frame.
    pub damage_per_frame: f32,
    /// Source (pour attribution).
    pub source: EntityId,
}

/// Etat de gel (Frozen).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FrozenState {
    /// Frames restantes de gel.
    pub frames_remaining: u32,
    /// L'entite est completement immobilisee.
    pub immobilized: bool,
}

/// Etat de ralentissement par le froid (Chilled).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChilledState {
    /// Frames restantes.
    pub frames_remaining: u32,
    /// Facteur de ralentissement (ex: 0.5 = 50% de la vitesse normale).
    pub slow_factor: f32,
}

/// Etat de stun.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StunnedState {
    pub frames_remaining: u32,
}

/// Etat de curse generique (Amplify Damage, Decrepify, etc.).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CursedState {
    pub curse_type: CurseType,
    pub frames_remaining: u32,
    /// Niveau du curse (affecte l'intensite).
    pub level: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CurseType {
    AmplifyDamage,
    Decrepify,
    LowerResist,
    Weaken,
    Terror,
    IronMaiden,
    LifeTap,
    DimVision,
    Confuse,
    Attract,
}

/// Amplify Damage specifique (pour acces rapide).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AmplifiedState {
    /// Frames restantes.
    pub frames_remaining: u32,
    /// Reduction de resistance physique (%).
    pub phys_resist_reduction: i32,
}

/// Decrepify specifique.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DecrepifiedState {
    pub frames_remaining: u32,
    /// Reduction de vitesse (%).
    pub slow_pct: i32,
    /// Reduction de resistance physique (%).
    pub phys_resist_reduction: i32,
}

/// Lower Resist specifique.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LowerResistState {
    pub frames_remaining: u32,
    /// Reduction de toutes les resistances elementaires (%).
    pub resist_reduction: i32,
}

/// Conviction aura (Paladin).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConvictionState {
    /// Reduction de defense (%).
    pub defense_reduction: i32,
    /// Reduction de resistance elementaire (%).
    pub resist_reduction: i32,
}

/// Open Wounds DoT.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OpenWoundsState {
    pub frames_remaining: u32,
    /// DPS calcule selon clvl du lanceur.
    pub damage_per_frame: f32,
    pub source: EntityId,
}

/// Blinded (Dim Vision).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlindedState {
    pub frames_remaining: u32,
    /// Rayon de vision reduit (tiles).
    pub reduced_sight_radius: f32,
}

/// Confused (Confuse curse).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConfusedState {
    pub frames_remaining: u32,
}

/// Ralentissement generique (Decrepify, Clay Golem, Holy Freeze...).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SlowedState {
    pub frames_remaining: u32,
    pub slow_factor: f32,
}

/// Repousse (Knockback).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KnockedBackState {
    /// Direction du knockback.
    pub direction_x: f32,
    pub direction_y: f32,
    /// Distance restante (tiles).
    pub distance_remaining: f32,
}

/// Converti (Attract, Conversion skill).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ConvertedState {
    pub frames_remaining: u32,
    /// L'entite attaque ses anciens allies.
    pub converted_owner: EntityId,
}

/// Iron Maiden (retour de dommage).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct IronMaidenState {
    pub frames_remaining: u32,
    /// Pourcentage de dommage physique retourne a l'attaquant.
    pub damage_return_pct: i32,
}

/// Life Tap (vol de vie sur hit).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LifeTapState {
    pub frames_remaining: u32,
    /// Pourcentage de dommage physique vole en vie.
    pub life_steal_pct: i32,
}
```

---

## 15. Tags (zero-sized components)

Les tags sont des composants de taille zero (ZST) utilises comme filtres dans les queries ECS. Ils n'occupent aucune memoire par entite et ne causent pas de cout en stockage.

```rust
// @id: comp-tags @do: define @role: arpg @layer: 3

/// Marqueur: cette entite est un joueur.
#[derive(Debug, Clone, Copy)]
pub struct PlayerTag;

/// Marqueur: cette entite est un monstre.
#[derive(Debug, Clone, Copy)]
pub struct MonsterTag;

/// Marqueur: cette entite est un item au sol.
#[derive(Debug, Clone, Copy)]
pub struct GroundItemTag;

/// Marqueur: cette entite est un projectile.
#[derive(Debug, Clone, Copy)]
pub struct ProjectileTag;

/// Marqueur: cette entite est un effet visuel.
#[derive(Debug, Clone, Copy)]
pub struct EffectTag;

/// Marqueur: cette entite est un NPC (non hostile).
#[derive(Debug, Clone, Copy)]
pub struct NpcTag;

/// Marqueur: cette entite est un mercenaire.
#[derive(Debug, Clone, Copy)]
pub struct MercenaryTag;

/// Marqueur: cette entite est une invocation (Raise Skeleton, Golem...).
#[derive(Debug, Clone, Copy)]
pub struct SummonTag;

/// Marqueur: cette entite est un waypoint.
#[derive(Debug, Clone, Copy)]
pub struct WaypointTag;

/// Marqueur: cette entite est un portal de ville.
#[derive(Debug, Clone, Copy)]
pub struct TownPortalTag;

/// Marqueur: cette entite est un shrine.
#[derive(Debug, Clone, Copy)]
pub struct ShrineTag;

/// Marqueur: cette entite est un interactable (chest, barrel...).
#[derive(Debug, Clone, Copy)]
pub struct InteractableTag;

/// Marqueur: cette entite est un trigger zone (script).
#[derive(Debug, Clone, Copy)]
pub struct TriggerTag;

/// Marqueur: cette entite est dead (en attente de cleanup ou corpse).
#[derive(Debug, Clone, Copy)]
pub struct DeadTag;

/// Marqueur: cette entite est marquee pour destruction (fin de frame).
#[derive(Debug, Clone, Copy)]
pub struct DespawnTag;

/// Marqueur: cette entite est en mode Hardcore.
#[derive(Debug, Clone, Copy)]
pub struct HardcoreTag;

/// Marqueur: cette entite est ethereal.
#[derive(Debug, Clone, Copy)]
pub struct EtherealTag;

/// Marqueur: cette entite est identifiee.
#[derive(Debug, Clone, Copy)]
pub struct IdentifiedTag;

/// Marqueur: entite hostile au joueur.
#[derive(Debug, Clone, Copy)]
pub struct HostileTag;

/// Marqueur: entite alliee au joueur (invocations, mercenaire).
#[derive(Debug, Clone, Copy)]
pub struct AlliedTag;

/// Marqueur: entite invulnerable (en ville, pendant cinematique...).
#[derive(Debug, Clone, Copy)]
pub struct InvulnerableTag;

/// Marqueur: entite en zone de ville (pas d'hostilite, pas de combat).
#[derive(Debug, Clone, Copy)]
pub struct InTownTag;
```

---

## 16. Ressources globales

Les ressources sont des singletons globaux accessibles par tous les systemes. Elles ne sont PAS attachees a une entite.

```rust
// @id: res-globals @do: define @role: engine @layer: 2

/// Temps de jeu global.
pub struct GameTime {
    /// Frame courante depuis le debut de la partie.
    pub frame: u64,
    /// Delta time en secondes depuis la derniere frame.
    pub dt: f32,
    /// Temps total ecoule depuis le debut (secondes).
    pub total_elapsed: f64,
    /// Tick rate fixe (25 fps pour la logique de jeu).
    pub tick_rate: u32,
}

/// Configuration de la difficulte courante.
pub struct DifficultyConfig {
    pub difficulty: Difficulty,
    /// Penalite de resistance (0, -40, -100).
    pub resistance_penalty: i32,
    /// Multiplicateur d'experience.
    pub experience_multiplier: f32,
    /// Les monstres ont-ils des immunites ?
    pub monster_immunities_enabled: bool,
}

/// Etat de la partie en reseau.
pub struct NetworkState {
    /// Nombre de joueurs connectes.
    pub player_count: u8,
    /// Maximum de joueurs (8).
    pub max_players: u8,
    /// Mode de reseau.
    pub mode: NetworkMode,
    /// Latence moyenne vers le host (ms).
    pub average_latency_ms: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMode {
    Solo,
    ListenServerHost,
    ListenServerClient,
    DedicatedServerClient,
}

/// Asset Registry (reference vers les assets charges).
pub struct AssetRegistry {
    /// Sprites charges.
    pub sprites: HashMap<SpriteId, SpriteAsset>,
    /// Atlas charges.
    pub atlases: HashMap<String, AtlasAsset>,
    /// Sons charges.
    pub sounds: HashMap<String, SoundAsset>,
    /// Maps charges.
    pub maps: HashMap<String, MapAsset>,
    /// Hot-reload watcher actif ?
    pub hot_reload_enabled: bool,
}

/// Registre global de tous les items instancies dans le monde.
pub struct ItemRegistry {
    /// Tous les items par UUID.
    pub items: HashMap<uuid::Uuid, ItemData>,
    /// Affixes par UUID.
    pub affixes: HashMap<uuid::Uuid, ItemAffixes>,
    /// Sockets par UUID.
    pub sockets: HashMap<uuid::Uuid, Sockets>,
    /// Durabilite par UUID.
    pub durability: HashMap<uuid::Uuid, Durability>,
    /// Runewords actifs par UUID.
    pub runewords: HashMap<uuid::Uuid, ActiveRuneword>,
}

/// Tables de donnees chargees depuis les TOML.
pub struct GameData {
    /// Definitions de monstres par ID.
    pub monster_defs: HashMap<String, MonsterDef>,
    /// Definitions d'items par ID.
    pub item_defs: HashMap<String, ItemDef>,
    /// Definitions de skills par ID.
    pub skill_defs: HashMap<String, SkillDef>,
    /// Definitions de zones par ID.
    pub zone_defs: HashMap<String, ZoneDef>,
    /// Definitions de quetes par ID.
    pub quest_defs: HashMap<String, QuestDef>,
    /// Tables de loot par ID.
    pub loot_tables: HashMap<String, LootTable>,
    /// Runewords par ID.
    pub runeword_defs: HashMap<String, RunewordDef>,
    /// Prefixes par ID.
    pub prefix_defs: HashMap<String, AffixDef>,
    /// Suffixes par ID.
    pub suffix_defs: HashMap<String, AffixDef>,
    /// Runes par ID.
    pub rune_defs: HashMap<String, RuneDef>,
    /// Sets par ID.
    pub set_defs: HashMap<String, SetDef>,
    /// Breakpoint tables par classe.
    pub breakpoint_tables: HashMap<CharacterClass, BreakpointTables>,
}

/// Tables de breakpoints pour une classe.
pub struct BreakpointTables {
    /// FCR: vec de (seuil%, frames)
    pub fcr: Vec<(i32, u32)>,
    /// FHR: vec de (seuil%, frames)
    pub fhr: Vec<(i32, u32)>,
    /// FBR: vec de (seuil%, frames)
    pub fbr: Vec<(i32, u32)>,
}

/// Etat du RNG global (seed pour reproductibilite en reseau).
pub struct GameRng {
    /// Seed de la partie.
    pub seed: u64,
    /// Etat courant du PRNG.
    pub state: u64,
}

/// Configuration de la camera.
pub struct CameraState {
    /// Position de la camera en coordonnees monde (tiles).
    pub center_x: f32,
    pub center_y: f32,
    /// Entite suivie par la camera.
    pub follow_target: Option<EntityId>,
    /// Mode de resolution.
    pub resolution_mode: ResolutionMode,
    /// Facteur de zoom (1.0 = normal).
    pub zoom: f32,
    /// Largeur/hauteur virtuelle (800x600 en pixel-perfect).
    pub virtual_width: u32,
    pub virtual_height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionMode {
    PixelPerfect,
    Hd,
}

/// Etat de l'audio global.
pub struct AudioState {
    /// Volume master (0.0 a 1.0).
    pub master_volume: f32,
    /// Volume par couche.
    pub layer_volumes: HashMap<AudioLayer, f32>,
    /// Musique courante.
    pub current_music: Option<String>,
}
```

---

## 17. Archetypes principaux

Les archetypes definissent les ensembles de composants stables pour chaque type d'entite. Les composants ephemeres (Section 14) sont geres via le sparse overlay.

### 17.1 Table des archetypes

| Archetype | Composants stables | Overlay ephemere |
|-----------|-------------------|-----------------|
| **Player** | Position, Velocity, Facing, Locomotion, MovementSpeed, Sprite, AnimState, CharacterInfo, BaseAttributes, UnspentPoints, VitalPools, Regeneration, Defense, AttackRating, Resistances, ResistanceCaps, Breakpoints, BlockChance, CombatBonuses, ElementalDamage, EquippedWeapon, CircleHitbox, Inventory, Equipment, PotionBelt, QuestLog, DiscoveredWaypoints, UiState, InputState, NetSync, PlayerTag | PoisonState, FrozenState, ChilledState, CursedState, AmplifiedState, DecrepifiedState, LowerResistState, OpenWoundsState, SlowedState, KnockedBackState |
| **Monster** | Position, Velocity, Facing, Locomotion, MovementSpeed, Sprite, AnimState, VitalPools, Defense, AttackRating, Resistances, CircleHitbox, AiState, AiParams, MonsterData, MonsterTag | PoisonState, FrozenState, ChilledState, StunnedState, CursedState, AmplifiedState, DecrepifiedState, LowerResistState, ConvictionState, ConvertedState, BlindedState, ConfusedState |
| **Item (sol)** | Position, Sprite, ItemData, DropAge, GroundItemTag | -- |
| **Projectile** | Position, Velocity, Facing, Sprite, AnimState, ProjectileData, Lifetime, CircleHitbox, ProjectileTag | -- |
| **Effect** | Position, Sprite, AnimState, VisualEffect, Lifetime, EffectTag | -- |
| **NPC** | Position, Sprite, AnimState, Facing, NpcData, NpcTag | -- |
| **Mercenary** | Position, Velocity, Facing, Locomotion, MovementSpeed, Sprite, AnimState, VitalPools, Defense, AttackRating, Resistances, CircleHitbox, AiState, AiParams, MercenaryTag, AlliedTag | PoisonState, FrozenState, ChilledState, CursedState |
| **Summon** | Position, Velocity, Facing, Locomotion, MovementSpeed, Sprite, AnimState, VitalPools, Defense, AttackRating, CircleHitbox, AiState, Parent, SummonTag, AlliedTag | PoisonState, FrozenState, CursedState |
| **Waypoint** | Position, Sprite, WaypointData, WaypointTag | -- |
| **TownPortal** | Position, Sprite, TownPortal, TownPortalTag | -- |
| **Shrine** | Position, Sprite, ShrineData, ShrineTag, InteractableTag | -- |
| **Interactable** | Position, Sprite, AnimState, Interactable, InteractableTag | -- |
| **TriggerZone** | TriggerZone, TriggerTag | -- |
| **AreaOfEffect** | Position, AreaOfEffect, VisualEffect, EffectTag | -- |

### 17.2 Comptabilite des composants

| Categorie | Nombre de composants | Crate(s) |
|-----------|---------------------|----------|
| Base (identification, hierarchie, cycle de vie) | 6 | mge-ecs |
| Mouvement et position | 7 | mge-ecs, mge-arpg-entity |
| Rendu et animation | 7 | mge-render |
| Stats et attributs | 14 | mge-arpg-stats |
| Combat | 9 | mge-arpg-combat |
| Items et inventaire | 16 | mge-arpg-items |
| IA | 7 | mge-arpg-ai |
| Reseau | 3 | mge-net |
| Interface | 7 | mge-ui |
| Audio | 2 | mge-audio |
| Monde et zones | 8 | mge-arpg-world |
| Quetes et scripts | 5 | mge-arpg-quest |
| Overlay ephemere | 17 | mge-arpg-combat |
| Tags (ZST) | 20 | mge-arpg-entity |
| **Total** | **128** | |

---

## 18. Invariants et regles

### 18.1 Invariants de composants

1. **EntityId unique** : Aucun EntityId ne peut etre reutilise sans incrementer la generation.
2. **Position obligatoire** : Toute entite visible dans le monde possede un composant Position.
3. **Sprite requis** : Toute entite rendue possede Sprite + AnimState.
4. **Stats coherentes** : `life_current <= life_max`, `mana_current <= mana_max`, `stamina_current <= stamina_max`.
5. **Inventaire borne** : La grille Inventory est 10x4, aucun item ne peut deborder.
6. **Gold cap** : Gold inventaire <= 10000 * clvl, Gold stash <= 2 500 000.
7. **Skill cap** : Aucun skill ne peut avoir plus de 20 hard points. Les soft points (items) sont illimites.
8. **Resistance cap** : En normal, les resistances effectives sont cappees a 75 (sauf bonus explicites de cap).
9. **CTH borne** : La chance de toucher est toujours dans [5%, 95%].
10. **CTB borne** : La chance de bloquer est cappee a 75% debout, 25% en course.

### 18.2 Regles de serialisation

- Tous les composants destines a la sauvegarde derivent `Serialize, Deserialize`.
- Les composants ephemeres (overlay) ne sont PAS sauvegardes (ils disparaissent au rechargement).
- Les Tags (ZST) ne sont pas serialises directement; ils sont reconstruits depuis les donnees.
- Les UUIDs sont serialises en format string hyphenated (`"550e8400-e29b-41d4-a716-446655440000"`).

### 18.3 Regles de securite

- Aucun composant ne stocke de donnees sensibles en clair (pas de mots de passe, pas de tokens).
- Les composants reseau (InputState, NetSync) sont valides cote host avant application.
- Le host ne fait JAMAIS confiance aux donnees client pour les calculs de combat.
- Les items et inventaires sont stockes et valides exclusivement cote host/serveur.

---

*Document genere par Denis -- Miyukini AI Studio*
*Revision prevue apres Sprint 0*
