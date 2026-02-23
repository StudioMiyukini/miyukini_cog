# Allumina — Extraction Architecture Ultima Online pour MGE

## Contexte

Ce document est une extraction technique exhaustive des systèmes fondamentaux d'**Ultima Online** (1997, Origin Systems), basée sur l'analyse de 4 codebases open source :
- **ServUO** (C# .NET, serveur gameplay complet — héritier de RunUO)
- **ModernUO** (C# .NET moderne, fork optimisé MMO de ServUO)
- **ClassicUO** (C# MonoGame/FNA, client open source)
- **UOX3** (C++, émulateur historique avec scripting JavaScript)

L'objectif est de **formaliser les patterns MMO sandbox** et de produire une architecture engine-neutral réimplémentable dans MGE pour le projet Allumina.

## Portée / Scope

- **Applicable à :** Architecture MMO sandbox, persistance monde, économie player-driven, housing, progression par usage.
- **Audience :** Architecture moteur, game design, développement serveur.
- **Statut :** Document de référence technique exhaustif.

---

# I — FONDATIONS PHILOSOPHIQUES

## Principes systémiques profonds

| Principe | Description | Implication technique |
|----------|-------------|----------------------|
| **Simulation persistante** | Le monde existe et évolue même sans joueurs | Serveur autoritaire continu, timers, decay |
| **Monde non instancié** | Un seul monde partagé par tous les joueurs d'un shard | Pas de génération à la demande, tout est préchargé |
| **Économie player-driven** | Les joueurs créent la valeur (craft, trade, housing) | Système économique fermé avec sinks et faucets |
| **Progression par usage** | Pas d'XP — les compétences montent en les utilisant | Skill gain probabiliste, fenêtre de difficulté |
| **Housing dans le monde réel** | Les maisons sont placées dans le monde ouvert | Multi-tile collision, decay, access control |
| **PvP émergent** | Les interactions hostiles émergent du gameplay | Flagging system, karma, murder counts |
| **Craft à valeur réelle** | Les objets craftés ont une valeur économique | Qualité variable, signature crafter, resource rareté |

## Invariants architecturaux

```
1. Monde unique par shard (pas d'instances)
2. Serveur 100% autoritaire (pas de client prediction pour le gameplay)
3. Simulation déterministe tick-based
4. Persistance continue (World.Save périodique)
5. Économie fermée (tout or et item a une origine traçable)
6. Identité objet permanente (Serial number unique)
7. État mutable de tout objet du monde (rien d'immuable)
```

## Citation fondatrice (Richard Garriott)

> "UO, to this day, is the only MMO that did such a good job of giving players non-combat roles that were so thoroughly simulated that people had entire lives that they would live out in the virtual world that had little or nothing to do with adventuring."

---

# II — ARCHITECTURE GLOBALE DU MOTEUR

## Diagramme macro

```
┌─────────────────────────────────────────────────────────────┐
│                     TOOLS & SCRIPTING                       │
│  (UOX3: JavaScript/SpiderMonkey | ServUO: C# Scripts)       │
├─────────────────────────────────────────────────────────────┤
│                     NETWORK LAYER                           │
│  (Packets, Huffman, Login Gateway, Stateless IO)            │
├─────────────────────────────────────────────────────────────┤
│                     SOCIAL LAYER                            │
│  (Guilds, Parties, PvP Flagging, Karma/Fame)                │
├─────────────────────────────────────────────────────────────┤
│                     ECONOMIC LAYER                          │
│  (Gold, Vendors, Trade, Auction, Sinks)                     │
├─────────────────────────────────────────────────────────────┤
│                     INTERACTION LAYER                       │
│  (Combat, Magic, Skills, Crafting, Harvesting)              │
├─────────────────────────────────────────────────────────────┤
│                     ENTITY LAYER                            │
│  (Mobile, Item, BaseMulti, BaseHouse)                       │
├─────────────────────────────────────────────────────────────┤
│                     WORLD LAYER                             │
│  (Maps, Sectors, Tiles, Statics, Spawners, Regions)         │
├─────────────────────────────────────────────────────────────┤
│                     SIMULATION CORE                         │
│  (Timer Wheel, Tick Loop, Spatial Index, Persistence)       │
└─────────────────────────────────────────────────────────────┘
```

---

# III — ENTITY MODEL

## Hiérarchie ServUO/ModernUO

```
IEntity
  └── Entity (Serial, Location, Map)
        ├── Mobile (HP, Stats, Skills, Equipment, AI)
        │     ├── PlayerMobile (Account, Housing, Karma, Fame)
        │     └── BaseCreature (AI, Spawning, Taming)
        │           ├── BaseVendor (Shop, Pricing)
        │           └── [centaines de créatures]
        └── Item (Weight, Layer, Parent, Hue)
              ├── BaseWeapon (Damage, Speed, Durability)
              ├── BaseArmor (AR, Durability, Meditation)
              ├── Container (Capacity, Items)
              │     └── BankBox, Backpack
              └── BaseMulti (Components[])
                    └── BaseHouse (Access, Decay, Lockdowns, Secures)
```

## Serial System (identité objet)

```csharp
// Plages d'allocation :
Mobiles : 0x00000001 — 0x3FFFFFFF  (~1.07 milliard)
Items   : 0x40000000 — 0x7FFFFFFF  (~1.07 milliard)
Invalid : 0x00000000 (Zero), -1 (MinusOne)

// Allocation incrémentale avec vérification d'unicité
Serial.NewMobile : ++lastMobile, skip si déjà utilisé
Serial.NewItem   : ++lastItem, skip si déjà utilisé
```

## Version MGE ECS

### Components

```rust
pub struct WorldObject {
    pub serial: u64,
    pub position: IVec3,         // x, y, z (entier, z = altitude)
    pub map_id: u32,
    pub hue: u16,
    pub name: String,
}

pub struct MobileStats {
    pub hp: (i32, i32),          // (current, max)
    pub mana: (i32, i32),
    pub stamina: (i32, i32),
    pub str_: i32,
    pub dex: i32,
    pub int_: i32,
}

pub struct SkillSet {
    pub skills: Vec<SkillValue>,  // 58 skills dans UO
    pub total_cap: f64,           // 720.0 par défaut
}

pub struct SkillValue {
    pub skill_id: SkillId,
    pub base: f64,               // 0.0–100.0 (ou 120.0 avec power scroll)
    pub cap: f64,                // 100.0 par défaut (120.0 avec PS)
    pub lock: SkillLock,         // Up, Down, Locked
}

pub struct ItemProperties {
    pub weight: f32,
    pub layer: Option<EquipLayer>,
    pub parent: Option<EntityId>,
    pub stackable: bool,
    pub amount: u32,
    pub durability: Option<(u32, u32)>,
}

pub struct HouseData {
    pub owner: EntityId,
    pub co_owners: Vec<EntityId>,
    pub friends: Vec<EntityId>,
    pub bans: Vec<EntityId>,
    pub decay_stage: DecayStage,
    pub last_refresh: u64,        // timestamp
    pub lockdowns: Vec<EntityId>,
    pub secures: Vec<EntityId>,
    pub sign_pos: IVec3,
    pub is_public: bool,
}

pub enum DecayStage {
    LikeNew, SlightlyWorn, SomewhatWorn,
    FairlyWorn, GreatlyWorn, IDOC, Collapsed,
}
```

---

# IV — SIMULATION LOOP

## Tick model

ServUO/ModernUO tournent sur un **single-threaded game loop** :

```
while running:
    process_network_input()      // Lire les paquets clients
    process_timers()             // Exécuter les timers échus
    process_movement()           // Mouvement des mobiles
    process_ai()                 // IA des créatures
    process_decay()              // Decay items/maisons
    process_spawners()           // Vérifier les spawners
    process_network_output()     // Envoyer les paquets
    
    if save_interval_elapsed:
        world_save()
    
    sleep_remaining_tick()       // 25ms tick (~40 TPS)
```

## Timer Wheel (ModernUO)

### Architecture

```
┌─────────────────────────────────────────┐
│          HIERARCHICAL TIMER WHEEL        │
│                                          │
│  Ring 0 (core)  : 4096 slots × 8ms     │
│  Ring 1 (mid)   : 4096 slots × 32.8s   │
│  Ring 2 (outer) : 4096 slots × 37.3h   │
│                                          │
│  Résolution : 8ms                        │
│  Portée max : ~6.3 jours                │
│  Complexité : O(1) insert/remove        │
│  Threading : Single-threaded (no locks)  │
└─────────────────────────────────────────┘
```

### Fonctionnement

```
fn advance_timer_wheel(current_tick):
    // Core ring : exécuter tous les timers du slot courant
    slot = current_tick % 4096
    for timer in ring0[slot]:
        timer.execute()
    
    // Si core ring a fait un tour complet
    if slot == 0:
        // Promouvoir les timers du ring 1 vers ring 0
        mid_slot = (current_tick / 4096) % 4096
        for timer in ring1[mid_slot]:
            new_slot = timer.deadline % 4096
            ring0[new_slot].push(timer)
        
        // Si ring 1 a fait un tour complet
        if mid_slot == 0:
            // Promouvoir ring 2 vers ring 1
            outer_slot = (current_tick / (4096*4096)) % 4096
            for timer in ring2[outer_slot]:
                promote_to_ring1(timer)
```

### Comparaison ServUO vs ModernUO

| Aspect | ServUO | ModernUO |
|--------|--------|----------|
| Structure | 8 priority buckets + thread dédié | Timer wheel hiérarchique 3 anneaux |
| Threading | Thread séparé + locks | Single-threaded (no locks) |
| Complexité insert | O(n) (tri par priorité) | O(1) |
| Complexité execute | O(1) | O(1) |
| Résolution | Variable par priorité | 8ms fixe |
| Mémoire | Heap allocations | Ring buffer pré-alloué |

## Spatial partitioning (Sectors)

```
Monde divisé en Sectors (grille fixe)
Chaque Sector contient :
  - List<Mobile> clients
  - List<Mobile> mobiles
  - List<Item> items
  - List<BaseMulti> multis

Requêtes spatiales via PooledEnumerable<T>
  → itère sur les sectors dans le rayon demandé
  → filtre par distance euclidienne
```

---

# V — SKILL ENGINE

## Formule de gain exacte (ServUO SkillCheck.cs)

```csharp
double gc = (double)(from.Skills.Cap - from.Skills.Total) / from.Skills.Cap;
gc += (skill.Cap - skill.Base) / skill.Cap;
gc /= 4;
gc += (4.0 - chance) * (success ? 0.5 : (Core.AOS ? 0.0 : 0.2));
gc /= 4;

// Multiplicateur par skill (GainFactor)
gc *= skill.GainFactor;

// Pénalités par palier
if (skill.Base > 70.0) gc /= 2;
if (skill.Base > 80.0) gc /= 2;
if (skill.Base > 90.0) gc /= 2;

// Minimum garanti
gc = Math.Max(gc, 0.01);  // 1% minimum
```

### Décomposition des facteurs

| Facteur | Formule | Effet |
|---------|---------|-------|
| **Room to total cap** | `(SkillsCap - SkillsTotal) / SkillsCap` | Plus de place = plus de gain |
| **Room to individual cap** | `(SkillCap - SkillBase) / SkillCap` | Plus de place dans le skill = plus de gain |
| **Difficulty** | `(4.0 - chance) × modifier` | Action difficile = plus de gain |
| **Success modifier** | `0.5` si succès, `0.0` si échec (AOS+) | Succès gagne plus |
| **GainFactor** | Par skill (ex: 0.5 pour Magery) | Certains skills montent plus lentement |
| **Level penalty** | `/2` à 70, `/2` à 80, `/2` à 90 | Gain ÷8 au-dessus de 90 |

### Caps

| Cap | Valeur | Modifiable par |
|-----|--------|----------------|
| **Total skill cap** | 720.0 points | — |
| **Individual cap** | 100.0 par défaut | Power Scroll (+5, +10, +15, +20) → max 120.0 |
| **Skill lock** | Up / Down / Locked | Joueur choisit |
| **Anti-macro** | Location + target tracking | Empêche le macroing stupide |

### Guaranteed Gain System (GGS)

Quand un joueur n'a pas gagné depuis longtemps, le GGS garantit un gain :

```
GGS Cooldown table (secondes) :
  0-49.9 skill  → 0s (gain instantané)
  50-59.9       → 300s (5 min)
  60-69.9       → 900s (15 min)
  70-79.9       → 2700s (45 min)
  80-89.9       → 5400s (90 min)
  90-99.9       → 10800s (3h)
  100-109.9     → 21600s (6h)
  110-120       → 43200s (12h)
```

### Skill Difficulty Window

Chaque utilisation de skill passe par `SkillCheck.CheckSkill()` avec une **fenêtre de difficulté** :

```
fn check_skill(mobile, skill_id, min_skill, max_skill):
    let skill_value = mobile.skills[skill_id].base
    
    // Calcul de la chance de succès
    if skill_value < min_skill:
        chance = 0.0  // impossible
    elif skill_value >= max_skill:
        chance = 1.0  // garanti
    else:
        chance = (skill_value - min_skill) / (max_skill - min_skill)
    
    let success = random() < chance
    
    // Tenter le gain (même en cas d'échec dans certains cas)
    if mobile.skills[skill_id].lock == SkillLock::Up:
        try_skill_gain(mobile, skill_id, chance, success)
    
    return success
```

## Version MGE ECS

```rust
pub struct SkillDef {
    pub id: SkillId,
    pub name: String,
    pub gain_factor: f64,
    pub stat_primary: StatType,
    pub stat_secondary: StatType,
}

pub struct SkillGainConfig {
    pub total_cap: f64,
    pub default_individual_cap: f64,
    pub level_penalties: Vec<(f64, f64)>,  // (threshold, divisor)
    pub min_gain_chance: f64,
    pub ggs_cooldowns: Vec<(f64, f64)>,    // (skill_threshold, cooldown_secs)
    pub anti_macro_enabled: bool,
}

// System
// @phase 500
pub fn skill_gain_system(world: &mut World, ctx: &mut Context) {
    // Pour chaque SkillCheckEvent reçu :
    // 1. Calculer la chance de succès (difficulty window)
    // 2. Calculer la probabilité de gain (formule gc)
    // 3. Roll gain
    // 4. Si gain : ajuster le skill + émettre SkillGainEvent
    // 5. Si lock == Down sur un autre skill : réduire ce skill
}
```

---

# VI — RESOURCE & HARVEST ENGINE

## Architecture HarvestSystem (ServUO)

```
HarvestSystem (abstract singleton)
  ├── Mining     → HarvestDefinition(Ore) + HarvestDefinition(Sand)
  ├── Lumberjacking → HarvestDefinition(Wood)
  └── Fishing    → HarvestDefinition(Fish)

HarvestDefinition
  ├── BankWidth/Height (8×8 pour mining)
  ├── MinTotal/MaxTotal (10-34 pour ore)
  ├── MinRespawn/MaxRespawn (10-20 min)
  ├── Skill requis
  ├── HarvestResource[] (types de ressources)
  ├── HarvestVein[] (distribution probabiliste)
  └── BonusHarvestResource[] (drops rares)
```

## Resource Bank Grid

Le monde est divisé en **banques de ressources** invisibles :

```
fn get_bank(position, definition):
    bank_x = position.x / (definition.bank_width * 8)  // en tiles
    bank_y = position.y / (definition.bank_height * 8)
    return resource_banks[bank_x][bank_y]
```

Chaque banque contient `random(MinTotal, MaxTotal)` unités. Quand épuisée, respawn dans `random(MinRespawn, MaxRespawn)`.

## Distribution des veines (Mining)

| Veine | Probabilité | Skill min | Fallback |
|-------|-------------|-----------|----------|
| Iron | 49.6% | 0.0 | — |
| Dull Copper | 11.2% | 65.0 | Iron |
| Shadow Iron | 9.8% | 70.0 | Iron |
| Copper | 8.4% | 75.0 | Iron |
| Bronze | 7.0% | 80.0 | Iron |
| Gold | 5.6% | 85.0 | Iron |
| Agapite | 4.2% | 90.0 | Iron |
| Verite | 2.8% | 95.0 | Iron |
| Valorite | 1.4% | 99.0 | Iron |

**Bonus Felucca :** ×2 ressources par récolte (incitation PvP).

## Version MGE ECS

```rust
pub struct HarvestNode {
    pub definition_id: HarvestDefId,
    pub bank_pos: IVec2,
    pub remaining: u32,
    pub max_capacity: u32,
    pub respawn_at: Option<u64>,
}

pub struct HarvestDef {
    pub id: HarvestDefId,
    pub bank_size: IVec2,
    pub capacity_range: (u32, u32),
    pub respawn_range: (f32, f32),       // secondes
    pub skill_required: SkillId,
    pub tool_required: ItemTypeId,
    pub max_range: f32,
    pub consume_per_harvest: u32,
    pub veins: Vec<HarvestVein>,
    pub bonus_resources: Vec<BonusResource>,
}

pub struct HarvestVein {
    pub resource_type: ItemTypeId,
    pub probability: f32,
    pub min_skill: f64,
    pub fallback: Option<ItemTypeId>,
}
```

---

# VII — CRAFT ENGINE

## Architecture CraftSystem (ServUO)

```
CraftSystem (abstract)
  ├── DefBlacksmithy
  ├── DefTailoring
  ├── DefCarpentry
  ├── DefTinkering
  ├── DefAlchemy
  ├── DefCooking
  ├── DefInscription
  ├── DefBowFletching
  ├── DefCartography
  ├── DefMasonry
  └── DefGlassblowing
```

### Formule de succès

```
fn craft_chance(skill_value, min_skill, max_skill):
    if skill_value < min_skill: return 0.0
    if skill_value >= max_skill: return 1.0
    return (skill_value - min_skill) / (max_skill - min_skill)
```

### Formule Exceptional

```
fn exceptional_chance(skill_value, min_skill, max_skill):
    let success = craft_chance(skill_value, min_skill, max_skill)
    return max(0.0, success - 0.60)  // 60% de moins que le succès
    
    // Avec Exceptional Bonus (Arms Lore >= 80):
    // +20% chance
```

### Pipeline UI → Server

```
1. Joueur ouvre le menu craft → Gump envoyé au client
2. Joueur sélectionne une recette → Packet CraftItem(recipeId)
3. Serveur valide:
   a. Outil présent et non cassé
   b. Ressources suffisantes (type + quantité)
   c. Skill suffisant (min_skill check)
4. Roll succès
5. Si succès:
   a. Consommer ressources
   b. Roll exceptional
   c. Créer l'item (avec signature crafter si exceptional)
   d. Réduire durabilité outil
6. Si échec:
   a. Consommer une partie des ressources (typiquement 50%)
   b. Message d'échec
7. Tenter skill gain
```

### Signature crafter

Les items exceptional portent la signature du crafter : `"Exceptionally crafted by [PlayerName]"`. Cette signature a une **valeur économique réelle** — les joueurs recherchent des crafters réputés.

## Version MGE ECS

```rust
pub struct CraftRecipe {
    pub id: RecipeId,
    pub craft_system: CraftSystemId,
    pub result_item: ItemTypeId,
    pub skill_required: SkillId,
    pub min_skill: f64,
    pub max_skill: f64,
    pub resources: Vec<(ItemTypeId, u32)>,
    pub tool_required: ItemTypeId,
    pub failure_resource_loss: f32,      // 0.0–1.0 (50% typique)
}

pub struct CraftResult {
    pub item: EntityId,
    pub is_exceptional: bool,
    pub crafter_name: Option<String>,
    pub resource_type_used: ItemTypeId,  // détermine la couleur/propriétés
}
```

---

# VIII — HOUSING ENGINE

## Architecture BaseHouse (ServUO)

```csharp
class BaseHouse : BaseMulti {
    // Identité
    Serial serial;
    Mobile Owner;
    
    // Accès (5 niveaux)
    List<Mobile> CoOwners;
    List<Mobile> Friends;
    List<Mobile> Access;
    List<Mobile> Bans;
    bool IsPublic;
    
    // Contenu
    List<Item> LockDowns;      // items fixés au sol
    List<SecureInfo> Secures;  // containers sécurisés
    int MaxLockDowns;
    int MaxSecures;
    
    // Decay
    DecayLevel LastDecayLevel;
    DateTime LastRefreshed;
    
    // Structure
    MultiComponentList Components;  // tuiles composant la maison
    Point3D Sign;                   // position du panneau
}
```

## Decay lifecycle

```
┌────────────┐ refresh ┌────────────┐
│  Like New  │←────────│ Joueur     │
│  (0-5 jrs) │         │ entre      │
└─────┬──────┘         └────────────┘
      │ 5 jours sans refresh
      ↓
┌────────────┐
│ Slightly   │
│ Worn       │
└─────┬──────┘
      ↓
┌────────────┐
│ Somewhat   │
│ Worn       │
└─────┬──────┘
      ↓
┌────────────┐
│ Fairly     │
│ Worn       │
└─────┬──────┘
      ↓
┌────────────┐
│ Greatly    │
│ Worn       │
└─────┬──────┘
      ↓
┌────────────┐ 5-15h ┌────────────┐
│   IDOC     │───────→│ COLLAPSED  │
│ (condamné) │        │ (barils)   │
└────────────┘        └────────────┘
```

**IDOC** (In Danger Of Collapsing) :
- Accès privé forcé
- Listes co-owners/friends vidées
- Items deviennent accessibles dans des barils après collapse
- Événement PvP émergent (joueurs campent les maisons IDOC)

## Placement validation

```
fn validate_house_placement(house_deed, target_location, map):
    // 1. Vérifier la zone
    if is_guard_zone(target_location): return Error("Cannot place in town")
    if is_no_housing_region(target_location): return Error("No housing here")
    
    // 2. Vérifier l'espace
    let bounds = house_deed.multi_components.bounds()
    let expanded = bounds.expand(front=6, back=5, sides=1)
    
    for tile in expanded:
        if has_other_house(tile): return Error("Too close to another house")
        if has_blocking_static(tile): return Error("Obstructed")
        if !is_level_ground(tile): return Error("Ground not level")
    
    // 3. Vérifier le propriétaire
    if player.house_count >= max_houses_per_account:
        return Error("Max houses reached")
    
    // 4. Placer
    place_house(house_deed, target_location, map)
```

## Access Control Matrix

| Niveau | Peut entrer | Peut utiliser containers | Peut lockdown | Peut modifier accès |
|--------|-------------|------------------------|---------------|-------------------|
| **Owner** | Oui | Oui | Oui | Oui |
| **Co-Owner** | Oui | Oui | Oui | Oui (sauf owner) |
| **Friend** | Oui | Oui (certains) | Non | Non |
| **Access** | Oui | Non | Non | Non |
| **Banned** | Non (téléporté dehors) | Non | Non | Non |
| **Public** | Oui (tous) | Non | Non | Non |

## Version MGE ECS

```rust
pub struct MultiStructure {
    pub components: Vec<MultiComponent>,
    pub bounds: IRect,
}

pub struct MultiComponent {
    pub offset: IVec3,          // offset relatif au centre
    pub tile_id: u16,
    pub flags: TileFlags,
}

// Event
pub struct HousePlacementRequest {
    pub player: EntityId,
    pub deed: EntityId,
    pub target: IVec3,
    pub map: MapId,
}

// System
pub fn housing_decay_system(world: &mut World, ctx: &mut Context) {
    // Itérer sur toutes les maisons
    // Calculer le temps depuis last_refresh
    // Avancer le decay_stage si nécessaire
    // Si IDOC et timer expiré → collapse
}
```

---

# IX — MAP & SHARDING

## Facet system (ServUO)

```csharp
// MapRules (flags combinables)
enum MapRules {
    None              = 0x0000,   // Felucca (PvP total)
    FreeMovement      = 0x0002,   // Pas de stamina loss en se bousculant
    BeneficialRestrictions = 0x0004,   // Interdit soins sur criminels
    HarmfulRestrictions    = 0x0008,   // Interdit attaque sur innocents
    TrammelRules = FreeMovement | BeneficialRestrictions | HarmfulRestrictions,
}
```

### Configuration des maps

| Map | ID | Dimensions | Surface | Règles |
|-----|-----|-----------|---------|--------|
| Felucca | 0 | 7168×4096 | ~29M tiles | PvP libre |
| Trammel | 1 | 7168×4096 | ~29M tiles | PvE protégé |
| Ilshenar | 2 | 2304×1600 | ~3.7M tiles | PvE |
| Malas | 3 | 2560×2048 | ~5.2M tiles | PvE, housing illimité |
| Tokuno | 4 | 1448×1448 | ~2.1M tiles | PvE |
| TerMur | 5 | 1280×4096 | ~5.2M tiles | PvE |

### Structure de la carte (format binaire)

```
Carte = 768×512 blocs
Bloc = 8×8 cellules = 196 octets (4 header + 64 × 3 octets)
Cellule = 3 octets : u16 tile_graphic + i8 altitude

Total : ~6144×4096 = 25,165,824 cellules par map
        × 3 octets = ~75 MB par map (terrain seul)
```

### Tile Flags (TILEDATA.MUL)

| Flag | Bit | Effet |
|------|-----|-------|
| Impassable | 0x0040 | Bloque le mouvement |
| Surface | 0x0200 | Permet de se tenir dessus |
| Bridge | 0x0400 | Surface traversable |
| Wet | 0x0080 | Eau/liquide |
| Door | 0x20000000 | Porte ouvrable |
| Wall | 0x0010 | Mur |
| Roof | 0x10000000 | Toit |
| NoShoot | 0x2000 | Bloque projectiles |

## Transposition Allumina

Allumina vise un **monde unique type UO** (Document Conceptuel). L'architecture facet peut être adaptée :

```rust
pub struct MapDef {
    pub id: MapId,
    pub name: String,
    pub dimensions: IVec2,
    pub rules: MapRules,
    pub spawn_tables: Vec<SpawnTableId>,
}

bitflags! {
    pub struct MapRules: u32 {
        const NONE = 0;
        const PVP_FREE = 1;
        const PVP_CONSENSUAL = 2;
        const HOUSING_ALLOWED = 4;
        const BONUS_HARVEST = 8;
    }
}
```

---

# X — ÉCONOMIE SYSTÉMIQUE

## Modèle de monnaie

| Type | Limite | Stockage |
|------|--------|----------|
| **Gold coin** | Stack max 60,000 | Item physique dans l'inventaire |
| **Bank check** | Montant variable | Item convertible |
| **Account Gold** | 1,000,000,000 max | Dématérialisé (post-SA) |

## Gold Sinks (mécanismes de retrait)

| Sink | Type | Impact estimé |
|------|------|--------------|
| Item decay (sol) | Passif | Massif (60 min timer) |
| House decay/collapse | Passif | Modéré |
| Repair costs | Actif | Continu |
| Insurance (600gp/item) | Actif par mort | Continu |
| Reagent consumption | Actif par sort | Élevé |
| Player vendor maintenance | Passif quotidien | Modéré |
| NPC buy/sell spread | Actif par trade | ~40% par transaction |
| Tithing points (Paladin) | Actif par sort | Faible |

## Gold Faucets (sources de monnaie)

| Faucet | Type |
|--------|------|
| Monster loot (gold) | Direct |
| NPC vendor sell prices | Direct |
| Quest rewards | Direct |
| Champion spawn rewards | Indirect (items vendables) |

## Leçon clé (citation Raph Koster)

L'économie UO **"did not behave as expected in many ways"**. Le concept original voulait un cycle écologique fermé (épées fondues → fer retourné au monde). En pratique :
- L'inflation a été incontrôlable
- Les développeurs ont dû ajouter des sinks agressifs post-launch
- La séparation Felucca/Trammel a cassé l'interdépendance économique PvP/PvE

## Version MGE ECS

```rust
pub struct Economy {
    pub gold_in_circulation: u64,
    pub gold_sinks_daily: u64,
    pub gold_faucets_daily: u64,
    pub inflation_index: f64,
}

pub struct VendorShop {
    pub owner: EntityId,
    pub items_for_sale: Vec<(EntityId, u64)>,  // (item, price)
    pub daily_maintenance: u64,
    pub gold_held: u64,
}
```

---

# XI — PERSISTENCE ENGINE

## World.Save() (ServUO)

```
1. NetState.Pause()         — Pause tous les clients
2. WaitForWriteCompletion() — Attend le write précédent
3. Saving = true
4. SaveStrategy.Save()      — Sérialise tout
5. ProcessSafetyQueues()    — Traite add/delete en attente
6. ProcessDecay()           — Traite le decay
7. NetState.Resume()        — Reprend le réseau
```

### Format de fichiers

```
Saves/
├── Mobiles/
│   ├── Mobiles.idx   (TypeID, Serial, Position, Length)
│   ├── Mobiles.tdb   (noms de types complets)
│   └── Mobiles.bin   (données sérialisées)
├── Items/
│   ├── Items.idx
│   ├── Items.tdb
│   └── Items.bin
└── Guilds/
    ├── Guilds.idx
    └── Guilds.bin
```

### Sérialisation versionnée

```csharp
// Chaque objet commence par écrire sa version
public override void Serialize(GenericWriter writer) {
    base.Serialize(writer);
    writer.Write(3);  // version
    writer.Write(m_NewField);
    // ... champs par version
}

public override void Deserialize(GenericReader reader) {
    base.Deserialize(reader);
    int version = reader.ReadInt();
    switch (version) {
        case 3: m_NewField = reader.ReadInt(); goto case 2;
        case 2: /* ... */ goto case 1;
        case 1: /* ... */ goto case 0;
        case 0: /* ... */ break;
    }
}
```

### ModernUO : Améliorations

| Aspect | ServUO | ModernUO |
|--------|--------|----------|
| Sérialisation | Manuelle (Serialize/Deserialize) | Source Generator automatique |
| Threading | Single-threaded | Multi-threaded round-robin |
| Migration | Manuelle (switch version) | JSON migration automatique |
| Format | Binaire custom | Binaire custom + metadata |
| Snapshot | Pause réseau complète | Snapshot rapide puis write async |

---

# XII — NETWORK ARCHITECTURE

## Protocole UO

### Séquence de login

```
Client → LoginServer : Encrypted Seed (4 octets)
Client → LoginServer : 0x80 Account Login (62 octets)
Server → Client     : 0x82 Login Confirm/Error
Server → Client     : 0xB9 Client Flags
Server → Client     : 0xA9 Character List
Client → GameServer : 0x91 Game Login
--- Huffman compression activée ---
Server → Client     : 0x1B Login Confirm (37 octets)
```

### Serveur autoritaire

Le serveur UO est **100% autoritaire** :
- Mouvement : le client envoie une requête (0x02), le serveur valide et confirme
- Combat : entièrement calculé serveur-side
- Items : pas de modification client-side
- Trade : validé par les deux parties + serveur

### ModernUO : Optimisations réseau

| Aspect | ServUO | ModernUO |
|--------|--------|----------|
| Packet allocation | Heap (new byte[]) | `stackalloc` (zero-alloc) |
| Socket I/O | Select/Poll classique | `epoll`/`wepoll`/`kqueue` via PollGroup |
| Compression | Huffman (après login) | Identique |
| Encryption | Login seed | Identique |

---

# XIII — SCRIPTING & EXTENSIBILITÉ

## Modèles comparés

| Émulateur | Scripting | Hot reload | Performance |
|-----------|-----------|------------|-------------|
| ServUO | C# natif (compilé au démarrage) | Non (recompile) | Bon |
| ModernUO | C# natif (source generators) | Non | Excellent |
| UOX3 | JavaScript (SpiderMonkey 1.7) | Oui (runtime) | Moyen |

### Pattern ServUO/ModernUO : C# comme langage de scripting

```
Scripts/
├── Items/
│   ├── Weapons/
│   ├── Armor/
│   └── Containers/
├── Mobiles/
│   ├── Creatures/
│   ├── Vendors/
│   └── PlayerMobile.cs
├── Engines/
│   ├── Craft/
│   ├── Harvest/
│   └── Spawner/
└── Gumps/
```

Les "scripts" sont en réalité du C# compilé au démarrage du serveur. La frontière code/script est inexistante — tout est du même langage.

---

# XIV — MGE ADAPTATION

## Component list complète

```rust
// === WORLD ===
WorldObject         // serial, position, map, hue, name
TileFlags           // impassable, surface, wet, door, etc.
MapRegion           // zone rules, spawn tables
Sector              // spatial partition bucket

// === ENTITY ===
MobileStats         // hp, mana, stamina, str, dex, int
SkillSet            // 58+ skills avec caps et locks
Equipment           // slots d'équipement
Inventory           // container avec capacité
Karma               // karma + fame + murder counts
PvpFlags            // innocent, criminal, murderer, guild_enemy

// === ITEMS ===
ItemProperties      // weight, layer, parent, stackable, durability
WeaponProperties    // damage, speed, type, material
ArmorProperties     // AR, material, meditation penalty
CraftSignature      // crafter name, exceptional flag, resource type

// === HOUSING ===
HouseData           // owner, access lists, decay, lockdowns, secures
MultiStructure      // components[], bounds
DecayTimer          // last_refresh, current_stage

// === ECONOMY ===
VendorShop          // items_for_sale, maintenance
GoldBalance         // account-level gold
TradeSession        // between two players

// === HARVEST ===
HarvestNode         // bank_pos, remaining, respawn_at
ResourceVein        // type, probability, min_skill

// === CRAFT ===
CraftRecipe         // result, skill, resources, tool
CraftSession        // in-progress craft

// === SOCIAL ===
GuildMembership     // guild_id, rank
PartyMembership     // party_id

// === SPAWN ===
SpawnerConfig       // types, range, delay, max_count
SpawnedEntity       // spawner_id, lifetime
```

## System list

```rust
// Phase 100 — Input
network_input_system          // Lire paquets clients
command_dispatch_system       // Router vers les handlers

// Phase 200 — Simulation
movement_system               // Valider et appliquer le mouvement
combat_system                 // Résoudre les combats
magic_system                  // Résoudre les sorts
skill_check_system            // Vérifier les skill checks
skill_gain_system             // Calculer les gains de skill

// Phase 300 — World
harvest_system                // Récolte de ressources
craft_system                  // Fabrication d'objets
spawn_system                  // Spawner des créatures
decay_system                  // Decay items + maisons
respawn_system                // Respawn ressources

// Phase 400 — Economy
vendor_system                 // Achats/ventes NPC
trade_system                  // Échanges joueur-joueur
gold_sink_system              // Drains économiques

// Phase 500 — Social
pvp_flag_system               // Mise à jour des flags PvP
karma_system                  // Calcul karma/fame
guild_system                  // Gestion des guildes

// Phase 600 — AI
creature_ai_system            // IA des créatures
vendor_ai_system              // IA des marchands
guard_system                  // Gardes de ville

// Phase 900 — Persistence
world_save_system             // Sauvegarde périodique
incremental_save_system       // Save incrémentale

// Phase 950 — Network Output
network_output_system         // Envoyer les paquets
interest_management_system    // Filtrer par zone d'intérêt
```

## Event flow global

```
[Client Packet] → network_input_system
  ↓
[Command Dispatch] → route vers le système approprié
  ↓ (ex: mouvement)
[movement_system] → valide → met à jour position → émet MovementEvent
  ↓
[interest_management_system] → détermine qui voit le mouvement
  ↓
[network_output_system] → envoie le packet de mouvement aux clients concernés
```

## Architecture serveur

```
┌─────────────────────────────────────────┐
│              LOGIN GATEWAY              │
│  (Stateless, authentification)          │
├─────────────────────────────────────────┤
│           GAME SERVER (MWS Lobby)       │
│                                         │
│  ┌─────────┐  ┌─────────┐  ┌────────┐ │
│  │ World   │  │ Entity  │  │Network │ │
│  │ Layer   │  │ Layer   │  │ Layer  │ │
│  └────┬────┘  └────┬────┘  └────┬───┘ │
│       │            │             │      │
│  ┌────┴────────────┴─────────────┴───┐ │
│  │        SIMULATION CORE            │ │
│  │  (Timer Wheel, Tick Loop, ECS)    │ │
│  └───────────────┬───────────────────┘ │
│                  │                      │
│  ┌───────────────┴───────────────────┐ │
│  │       PERSISTENCE (KindMother)    │ │
│  └───────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

### Intégration MWS (Allumina)

| Concept UO | Allumina/MWS |
|------------|-------------|
| Shard (serveur séparé) | COG exposant un Lobby Allumina |
| Login Gateway | MWS Tracker (découverte) |
| Server List | Catalogue de Lobbys |
| Character Transfer | Non applicable (données souveraines par COG) |

## Config JSON exemple

```json
{
  "world": {
    "maps": [
      {
        "id": 0,
        "name": "Allumina World",
        "dimensions": [4096, 4096],
        "rules": ["PVP_FREE", "HOUSING_ALLOWED", "BONUS_HARVEST"]
      }
    ],
    "tick_rate": 40,
    "save_interval_seconds": 300,
    "sector_size": 16
  },
  "skills": {
    "total_cap": 720.0,
    "individual_cap": 100.0,
    "gain_penalties": [
      { "threshold": 70.0, "divisor": 2.0 },
      { "threshold": 80.0, "divisor": 2.0 },
      { "threshold": 90.0, "divisor": 2.0 }
    ],
    "ggs_enabled": true
  },
  "housing": {
    "max_per_account": 1,
    "decay_stages": [
      { "name": "LikeNew", "duration_hours": 120 },
      { "name": "SlightlyWorn", "duration_hours": 24 },
      { "name": "SomewhatWorn", "duration_hours": 24 },
      { "name": "FairlyWorn", "duration_hours": 24 },
      { "name": "GreatlyWorn", "duration_hours": 24 },
      { "name": "IDOC", "duration_hours": [5, 15] }
    ]
  },
  "economy": {
    "vendor_maintenance_daily": 50,
    "insurance_cost": 600,
    "npc_sell_ratio": 0.6,
    "item_decay_seconds": 3600
  }
}
```

---

# XV — MODERNISATION ET VERSION 2030-READY

## Comparaison architecturale

| Aspect | UO (1997) | Eve Online | WoW Vanilla | Allumina (cible) |
|--------|-----------|------------|-------------|-------------------|
| **World** | Non instancié | Single shard | Shardé + instances | Lobbys MWS (hybride) |
| **Économie** | Player-driven | Player-driven | NPC-driven | Player-driven (Allumina) |
| **Craft** | Valeur réelle | Valeur réelle | Presque aucune | Valeur réelle |
| **Housing** | Monde ouvert | Pas de housing | Pas de housing (WoD Garrison) | Monde ouvert |
| **PvP** | Émergent | Emergent | Consensuel (BGs) | Émergent (Felucca-like) |
| **Progression** | Usage-based | Temps-based (queue) | XP-based | Usage-based (Allumina) |
| **Serveur** | ~2000 joueurs/shard | ~40,000/shard | ~3000/realm | Lobby scalable |
| **Persistence** | Snapshot périodique | Snapshot + event log | DB relationnelle | KindMother + snapshot |
| **Langage** | C# (RunUO) | Stackless Python | C++ | Rust (MGE) |

## Limites d'UO à dépasser

| Limite | Cause | Solution moderne |
|--------|-------|-----------------|
| **2000 joueurs max** | Single-threaded, polling naïf | Multi-threaded + timer wheel + interest management |
| **Housing = consomme la map** | Placement physique permanent | Zones dédiées + instanciation intérieur |
| **Économie inflationniste** | Sinks insuffisants | Dynamic pricing + algorithme d'inflation |
| **Macroing** | Skill gain trop prévisible | Anti-macro + variation d'activité requise |
| **Pathfinding serveur** | Coûteux sur 29M tiles | A* limité + navmesh simplifié |
| **Save bloque le serveur** | Pause réseau pendant save | Multi-threaded save (ModernUO pattern) |
| **Client-server lag** | Pas de prediction | Client prediction + réconciliation |

## Propositions pour dépasser UO sans casser le sandbox

### 1. Simulation économique dynamique

```rust
pub struct DynamicEconomy {
    pub price_index: HashMap<ItemTypeId, f64>,    // prix moyen observé
    pub supply_tracker: HashMap<ItemTypeId, u64>,  // items créés/jour
    pub demand_tracker: HashMap<ItemTypeId, u64>,  // items consommés/jour
    pub inflation_rate: f64,
    pub sink_multiplier: f64,                      // auto-adjust
}

fn auto_balance_economy(economy: &mut DynamicEconomy):
    if economy.inflation_rate > TARGET_INFLATION:
        economy.sink_multiplier *= 1.05  // augmenter les sinks
    else:
        economy.sink_multiplier *= 0.98  // relâcher
```

### 2. IA émergente

Remplacer les PNJ statiques par des agents à comportement émergent :
- Les marchands NPC ajustent leurs prix selon l'offre/demande réelle
- Les monstres migrent vers les zones moins chassées
- Les factions NPC réagissent aux actions des joueurs

### 3. Anti-exploit

| Exploit | Prévention |
|---------|-----------|
| Duplication d'items | Serial unique + validation bidirectionnelle |
| Speed hack | Serveur autoritaire + rate limiting |
| Macro farming | Variation d'activité + CAPTCHA subtil |
| Gold farming | Monitoring automatique des patterns |
| Housing griefing | Règles de placement + cooldowns |

### 4. Scalabilité 10k+ joueurs

```
Architecture cluster :
┌─────────────────────────────────────────────┐
│              GATEWAY CLUSTER                │
│  (Load balancer, login, authentification)    │
├─────────────────────────────────────────────┤
│         ZONE SERVERS (par région)           │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐      │
│  │ Zone A  │ │ Zone B  │ │ Zone C  │      │
│  │ 2k max  │ │ 2k max  │ │ 2k max  │      │
│  └────┬────┘ └────┬────┘ └────┬────┘      │
│       │           │           │             │
│  ┌────┴───────────┴───────────┴──────────┐ │
│  │       SHARED STATE (DB + Message Bus) │ │
│  │  (Items, Housing, Economy, Guilds)    │ │
│  └───────────────────────────────────────┘ │
└─────────────────────────────────────────────┘

Handoff quand un joueur traverse une frontière de zone :
  1. Zone A sérialise l'état du joueur
  2. Message bus envoie l'état à Zone B
  3. Zone B instancie le joueur
  4. Client redirigé vers Zone B
  5. Délai total < 200ms
```

---

## Références

| Source | URL | Données |
|--------|-----|---------|
| ServUO | github.com/ServUO/ServUO | Code serveur C# complet |
| ModernUO | github.com/modernuo/ModernUO | Timer wheel, save multi-thread, networking |
| ClassicUO | github.com/ClassicUO/ClassicUO | Client C# MonoGame, rendu isométrique |
| UOX3 | github.com/UOX3DevTeam/UOX3 | Émulateur C++ avec scripting JS |
| UOGuide | uoguide.com | Housing decay, PvP, economy |
| Raph Koster | raphkoster.com | Design philosophy, economy evolution |
| Maxroll UO | — | Skill system, formulas |
| ModernUO docs | modernuo.com | Timer wheel documentation |
| Stratics | — | Formats MUL/UOP, tile system |
| Eve GDC | gdcvault.com | Architecture single-shard comparison |

---

**Document** : Allumina — Extraction Architecture UO pour MGE  
**Version** : 1.0  
**Date** : 2026-02-22  
**Statut** : Document de référence technique exhaustif
