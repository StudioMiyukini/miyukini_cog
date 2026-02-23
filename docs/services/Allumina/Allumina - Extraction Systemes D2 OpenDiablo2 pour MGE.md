# Allumina — Extraction Systèmes Diablo II (OpenDiablo2) — Implémentation MGE

## Contexte

Ce document est l'extraction technique complète des **10 systèmes fondamentaux** de Diablo II, basée sur :
- Le code source de **OpenDiablo2** (Go, ~11k stars GitHub)
- Le reverse-engineering **D2MOO** (C++, Phrozen Keep)
- Les fichiers de données moddés (.txt) documentés sur d2mods.info
- Les formules vérifiées par maxroll.gg, theamazonbasin.com, mannm.org

Chaque système est présenté en **3 versions** :
1. **Fidèle à Diablo II** — reconstruction exacte
2. **Modernisée** — améliorations sans changer le game feel
3. **Optimisée MGE ECS** — adaptation pour le moteur Miyukini Game Engine

## Portée / Scope

- **Applicable à :** Implémentation Allumina, plugins MGE Phase 2+, configuration game design.
- **Audience :** Développement moteur, game design, architecture.
- **Statut :** Document de référence technique exhaustif.

### Sources du code analysé

| Source | Langage | Rôle |
|--------|---------|------|
| `OpenDiablo2/d2core/d2map/` | Go | Pathfinding, entités, missiles, génération de cartes |
| `OpenDiablo2/d2core/d2records/` | Go | Structures de données (MonStats, Missiles, Skills, Items, TC) |
| `OpenDiablo2/d2common/d2path/` | Go | Structure Path |
| `OpenDiablo2/d2core/d2hero/` | Go | Skills, stats de héros |
| `OpenDiablo2/d2core/d2item/` | Go | Génération d'items |
| `D2MOO/source/D2Game/src/AI/` | C++ | IA des monstres (AiThink.cpp) |

---

# SYSTÈME 1 — DÉPLACEMENT DES ENTITÉS

## 1.1 Description fonctionnelle

Le déplacement dans D2 est un système hybride : **grille de subtiles** pour la passabilité + **coordonnées flottantes** pour le positionnement visuel. Le moteur tourne à **25 FPS fixe** (40ms/frame). Toute la logique de mouvement est liée à ce tick rate.

## 1.2 Architecture interne (OpenDiablo2)

### Structure `mapEntity` (d2mapentity/map_entity.go)

```go
type mapEntity struct {
    uuid      string
    Position  d2vector.Position
    Target    d2vector.Position
    velocity  d2vector.Vector
    Speed     float64
    path      []d2vector.Position
    drawLayer int
    done      func()
    directioner func(direction int)
    highlight bool
}
```

### Boucle de mouvement — `Step(tickTime)`

```go
func (m *mapEntity) Step(tickTime float64) {
    if m.atTarget() && !m.hasPath() {
        if m.done != nil { m.done(); m.done = nil }
        m.velocity.SetLength(0)
        return
    }
    m.setVelocity(tickTime * m.Speed)
    v := m.velocity.Clone()
    for {
        applyVelocity(&m.Position.Vector, v, &m.Target.Vector)
        if m.atTarget() { m.nextPath() }
        if v.IsZero() { break }
    }
}
```

### Fonction `applyVelocity`

```go
func applyVelocity(position, velocity, target *d2vector.Vector) {
    x, y := position.CompareApprox(target)
    vx, vy := velocity.X(), velocity.Y()
    if x == 0 { vx = 0 }
    if y == 0 { vy = 0 }
    velocity.Set(vx, vy)
    dest := position.Clone()
    dest.Add(velocity)
    destDistance := position.Distance(dest)
    targetDistance := position.Distance(target)
    if destDistance > targetDistance {
        position.Copy(target)
        velocity.Copy(dest.Subtract(target))
    } else {
        position.Copy(dest)
        velocity.Set(0, 0)
    }
}
```

**Analyse :** quand la vitesse dépasse la distance restante, l'entité est placée exactement sur la cible, et le surplus de vélocité est reporté vers le prochain waypoint. Pas d'interpolation continue — c'est un déplacement par pas discrets.

## 1.3 Pathfinding (d2mapengine/pathfind.go)

### Code OpenDiablo2 — Line-of-Sight simplifié

```go
func (m *MapEngine) PathFind(start, dest d2vector.Position) []d2vector.Position {
    points := make([]d2vector.Position, 0)
    _, point := m.checkLos(start, dest)
    points = append(points, point)
    return points
}

func (m *MapEngine) checkLos(start, end d2vector.Position) (bool, d2vector.Position) {
    dv := d2vector.Position{Vector: *end.Clone()}
    dv.Subtract(&start.Vector)
    dx := dv.X()
    dy := dv.Y()
    N := math.Max(math.Abs(dx), math.Abs(dy))
    divN := 1.0 / N
    xstep := dx * divN
    ystep := dy * divN
    x := start.X()
    y := start.Y()
    for i := 0; i <= int(N); i++ {
        x += xstep
        y += ystep
        if m.SubTileAt(int(math.Floor(x)), int(math.Floor(y))).BlockWalk {
            return false, d2vector.NewPosition(x-xstep, y-ystep)
        }
    }
    return true, end
}
```

**Analyse :** OpenDiablo2 implémente un **raycasting de Bresenham simplifié**, pas un A* complet. Le vrai D2 utilise A* avec portée limitée (~200 nœuds). OpenDiablo2 est un projet incomplet sur ce point.

### Algorithme A* du D2 original (reconstitué)

```
CONSTANTES:
  MAX_PATH_NODES = 200
  SUBTILE_SIZE = 5 par tile
  
fn a_star(start: SubTile, goal: SubTile, walk_flags: WalkType) -> Vec<SubTile>:
    open = PriorityQueue::new()
    open.push(start, heuristic(start, goal))
    came_from = HashMap::new()
    g_score = HashMap::new()
    g_score[start] = 0
    nodes_explored = 0
    
    while !open.is_empty() && nodes_explored < MAX_PATH_NODES:
        current = open.pop()
        nodes_explored += 1
        
        if current == goal:
            return reconstruct_path(came_from, current)
        
        for neighbor in get_walkable_neighbors(current, walk_flags):
            tentative_g = g_score[current] + distance(current, neighbor)
            if tentative_g < g_score.get(neighbor, INFINITY):
                came_from[neighbor] = current
                g_score[neighbor] = tentative_g
                f = tentative_g + heuristic(neighbor, goal)
                open.push(neighbor, f)
    
    return []  // pas de chemin trouvé

fn heuristic(a: SubTile, b: SubTile) -> f32:
    // Distance de Chebyshev (mouvement 8 directions)
    dx = abs(a.x - b.x)
    dy = abs(a.y - b.y)
    return max(dx, dy) + 0.414 * min(dx, dy)
```

## 1.4 Grille et subtiles

| Paramètre | D2 Original | OpenDiablo2 |
|-----------|-------------|-------------|
| Tile size | 160×80 px (isométrique) | Identique |
| Subtile | 32×16 px | Identique |
| Subtiles par tile | 5×5 = 25 | 5×5 = 25 |
| Walk flags par subtile | 16 bits (joueur, merc, missile, etc.) | `BlockWalk` boolean simplifié |
| Coordonnées | Entières (subtiles) + offset flottant | `d2vector.Position` (flottant) |

### Subtile flags (D2 original, par bit)

| Bit | Flag | Usage |
|-----|------|-------|
| 0 | Block Walk | Bloque mouvement joueur |
| 1 | Block LoS | Bloque ligne de vue |
| 2 | Block Jump | Bloque saut |
| 3 | Block Fly | Bloque vol |
| 4 | Block Projectile | Bloque missiles |
| 5 | Block Light | Bloque lumière |
| 6–15 | Réservés | Collision spécifique, walkable merc, etc. |

## 1.5 Vitesse de déplacement en frames

### D2 Original — vitesse fixe par frame

```
displacement_per_frame = velocity / 256
position += displacement_per_frame  (en subtiles)
```

La vitesse est stockée en **256ièmes de subtile par frame** dans MonStats.txt (colonnes `Velocity`, `Run`).

| Entité | Velocity | Unité | Réel (subtiles/s) |
|--------|----------|-------|--------------------|
| Joueur (walk) | 6 | 256ths/frame | 6/256 × 25 = 0.586 st/s |
| Joueur (run) | 9 | 256ths/frame | 9/256 × 25 = 0.879 st/s |
| Fallen (walk) | 5 | 256ths/frame | 0.488 st/s |
| Fallen (run) | 9 | 256ths/frame | 0.879 st/s |

### Breakpoints de vitesse

```
FRW% = Faster Run/Walk %
EffectiveSpeed = BaseSpeed × (100 + FRW%) / 100
```

Puisque le tick est à 25 FPS, la vitesse est discrétisée. Mais contrairement au FCR/IAS, il n'y a **pas de breakpoints pour la vitesse de déplacement** — elle est interpolée en continu en fractions de subtile.

## 1.6 Version Fidèle D2

```rust
pub struct D2Movement {
    pub position: SubTilePos,      // entier (x, y en subtiles)
    pub sub_offset: Vec2,          // fraction 0.0–1.0 dans la subtile
    pub velocity: u16,             // 256ths de subtile par frame
    pub run_velocity: u16,
    pub is_running: bool,
    pub path: Vec<SubTilePos>,
    pub path_index: usize,
}

fn step_d2(entity: &mut D2Movement) {
    let speed = if entity.is_running { entity.run_velocity } else { entity.velocity };
    let displacement = speed as f32 / 256.0;
    // appliquer le déplacement vers le prochain waypoint
    // recalculer direction chaque frame
}
```

## 1.7 Version Modernisée

- Coordonnées flottantes pures (Vec2)
- A* avec spatial hash pour la broadphase
- Interpolation visuelle découplée du tick logique
- Support du running sans pénalité de defense automatique

## 1.8 Version MGE ECS

### Components

```rust
// @id mge.plugin.movement.v1.component.locomotion
pub struct Locomotion {
    pub max_speed: f32,           // px/s
    pub acceleration_rate: f32,
    pub friction: f32,
    pub stop_threshold: f32,
    pub is_running: bool,
    pub run_multiplier: f32,
}

// @id mge.plugin.movement.v1.component.pathfinding_state
pub struct PathfindingState {
    pub waypoints: Vec<Vec2>,
    pub waypoint_index: usize,
    pub recalc_cooldown: f32,
    pub max_path_nodes: u32,      // 200 pour fidélité D2
}

// @id mge.plugin.movement.v1.component.walk_flags
pub struct WalkFlags {
    pub walk_type: WalkType,       // Player, Mercenary, Monster, Flying
    pub can_open_doors: bool,
    pub ignores_terrain: bool,     // flying=1
}
```

### Systems

```rust
// @id mge.plugin.movement.v1.fn.pathfind_update
// @requires Position2D, PathfindingState, WalkFlags
// @writes PathfindingState
// @phase 100
// @complexity O(n × max_nodes)
pub fn pathfind_update(world: &mut World, ctx: &mut Context) {
    // Recalcule le chemin pour chaque entité dont le cooldown a expiré
}

// @id mge.plugin.movement.v1.fn.locomotion_update
// @requires Position2D, Velocity2D, Locomotion, PathfindingState
// @writes Position2D, Velocity2D
// @phase 110
// @complexity O(n)
pub fn locomotion_update(world: &mut World, ctx: &mut Context) {
    // Chaîne locomotion MGE standard :
    // direction → accel/friction → clamp → displacement → rotation
}
```

### Event flow

```
[Input / AI Decision] → PathfindingState.waypoints updated
  ↓
[pathfind_update] → Recalcule waypoints si nécessaire (A* limité)
  ↓
[locomotion_update] → direction = (waypoint - pos).normalize()
                    → velocity = lerp(velocity, target_vel, accel * dt)
                    → position += velocity * dt
  ↓
[collision_check] → Résout les collisions (spatial hash)
  ↓
[orientation_update] → Rotation vers direction de mouvement
```

---

# SYSTÈME 2 — IA DES MONSTRES

## 2.1 Description fonctionnelle

L'IA de D2 est une **FSM table-driven** : le comportement est déterminé par un identifiant d'IA (colonne `AI` de MonStats.txt) + 8 paramètres numériques (`aip1-8`). Chaque IA est implémentée en code C++ (D2MOO : `AiThink.cpp`). Les entités exécutent leur boucle IA à un rythme contrôlé par `aidel` (délai entre ticks IA).

## 2.2 Architecture interne (OpenDiablo2)

### NPC — Entité passive (d2mapentity/npc.go)

OpenDiablo2 implémente les NPC comme des entités passives qui suivent des chemins prédéfinis :

```go
type NPC struct {
    mapEntity
    Paths       []d2path.Path
    name        string
    composite   *d2asset.Composite
    action      int
    path        int
    repetitions int
    monstatRecord *d2records.MonStatRecord
    monstatEx     *d2records.MonStat2Record
    HasPaths    bool
    isDone      bool
}
```

**Limitation :** OpenDiablo2 n'implémente **pas** l'IA de combat des monstres. Les NPC suivent uniquement des chemins prédéfinis (DS1 paths). L'IA hostile est absente du code.

### Boucle NPC (Advance)

```go
func (v *NPC) Advance(tickTime float64) {
    v.Step(tickTime)
    v.composite.Advance(tickTime)
    if v.HasPaths && v.wait() {
        v.isDone = false
        path := v.NextPath()
        v.setTarget(path.Position, v.next)
        v.action = path.Action
    }
}
```

Les NPC alternent entre déplacement vers un waypoint et attente (animation idle répétée 3-5 fois).

## 2.3 MonStatRecord — Structure complète (d2records/monster_stats_record.go)

### Champs IA extraits du code OpenDiablo2

```go
// AiKey — identifiant de l'IA (ex: "Fallen", "Skeleton", "SandRaider")
AiKey string

// AiDelay — délai entre ticks IA [Normal, Nightmare, Hell]
AiDelay [3]int

// AiDistance — distance d'activation [Normal, Nightmare, Hell]  
AiDistance [3]int

// AiParameters — 8 paramètres par difficulté (% passés à l'IA)
AiParameters [3][8]int

// ThreatLevel — priorité de ciblage par les ennemis
ThreatLevel int

// Flags booléens
IsNPC          bool  // npc
IsInteractable bool  // interact
IsLowUndead    bool  // lUndead
IsHighUndead   bool  // hUndead
IsDemon        bool  // demon
IsFlying       bool  // flying
CanOpenDoors   bool  // opendoors
IsBoss         bool  // boss
IsPrimeEvil    bool  // primeevil
IsKillable     bool  // killable
CanSwitchAI    bool  // switchai — peut changer de camp
NoAura         bool  // noAura
PetIgnore      bool  // petIgnore — ignore pets/mercs
DeathDamage    bool  // deathDmg — explosion à la mort

// Combat
MeleeRange     int   // portée mêlée
IsRanged       bool  // rangedtype
IsMelee        bool  // IsMelee
```

## 2.4 Machine d'état (reconstitué depuis D2MOO + MonStats.txt)

### États IA complets

```
┌──────────────┐
│   SPAWN      │ ← spawnmode (NU, S1, etc.)
└──────┬───────┘
       │ init complete
       ↓
┌──────────────┐     aidist check      ┌──────────────┐
│    IDLE      │────────────────────── →│    CHASE     │
│  (neutral)   │← no target / leash    │  (pursuit)   │
└──────┬───────┘                        └──────┬───────┘
       │ aidel elapsed                         │ in melee range
       ↓                                       ↓
┌──────────────┐                        ┌──────────────┐
│   WANDER     │                        │   ATTACK     │
│  (patrol)    │                        │  (A1/A2/Sk)  │
└──────────────┘                        └──────┬───────┘
                                               │
                              ┌────────────────┤
                              │                │
                         hit taken        HP < threshold
                              ↓                ↓
                     ┌──────────────┐  ┌──────────────┐
                     │    STUN      │  │    FLEE      │
                     │ (hit recov)  │  │ (run away)   │
                     └──────┬───────┘  └──────┬───────┘
                            │                 │
                            └──── recover ────┘
                                    │
                               HP = 0
                                    ↓
                           ┌──────────────┐
                           │    DEAD      │
                           │  (DT/DD)     │
                           └──────┬───────┘
                                  │ deathDmg?
                                  ↓
                           ┌──────────────┐
                           │  CORPSE      │
                           │ (resurrectable?)│
                           └──────────────┘
```

### Paramètres aip1-8 par type d'IA (exemples reconstitués)

| IA | aip1 | aip2 | aip3 | aip4 | aip5 | aip6 | aip7 | aip8 |
|----|------|------|------|------|------|------|------|------|
| **Fallen** | % fuite à vue mort allié | % resurrection | — | — | — | — | — | — |
| **Skeleton** | % chance attaque A2 | — | — | — | — | — | — | — |
| **Scarab** | % raid | % burrow | — | — | % boss raid | — | — | — |
| **SandRaider** | % chance charge | % chance flurry | — | — | — | — | — | — |
| **Shaman** | % heal allié | % resurrection | distance heal | — | — | — | — | — |

## 2.5 Pseudo-algorithme IA complet

```
fn ai_think(monster: &mut Monster, world: &World, frame: u32):
    // Vérifier le délai IA
    if frame - monster.last_ai_tick < monster.ai_delay:
        return
    monster.last_ai_tick = frame
    
    match monster.ai_state:
        IDLE:
            // Chercher une cible
            let targets = world.find_hostile_in_range(
                monster.pos, 
                monster.ai_dist
            )
            
            // Filtrer par pet_ignore
            let targets = if monster.pet_ignore:
                targets.filter(|t| t.is_player())
            else:
                targets
            
            // Trier par threat (décroissant), puis distance
            targets.sort_by(|a, b| {
                b.threat.cmp(&a.threat)
                    .then(distance(monster.pos, a.pos)
                        .cmp(&distance(monster.pos, b.pos)))
            })
            
            if let Some(target) = targets.first():
                monster.target = Some(target.id)
                monster.ai_state = CHASE
            else:
                // Wander aléatoire
                if random_percent(20):
                    let wander_pos = random_point_in_radius(monster.pos, 5)
                    monster.set_path(pathfind(monster.pos, wander_pos))
                    monster.ai_state = WANDER
        
        CHASE:
            if monster.target.is_none() || !is_alive(monster.target):
                monster.ai_state = IDLE
                return
            
            let target_pos = world.get_pos(monster.target)
            let dist = distance(monster.pos, target_pos)
            
            // Leash check
            if dist > LEASH_DISTANCE:
                monster.target = None
                monster.ai_state = IDLE
                return
            
            // In attack range?
            if dist <= monster.melee_range:
                monster.ai_state = ATTACK
                return
            
            // Ranged attack check
            if monster.is_ranged && dist <= monster.skill_range:
                if has_line_of_sight(monster.pos, target_pos):
                    monster.ai_state = RANGED_ATTACK
                    return
            
            // Pathfind vers la cible
            let path = pathfind(monster.pos, target_pos)
            monster.set_path(path)
            monster.is_running = true
        
        ATTACK:
            let target = monster.target
            
            // Choisir l'attaque (aip paramètres)
            let attack = select_attack(monster.ai_params)
            execute_attack(monster, target, attack)
            
            // Retour à chase après l'attaque
            monster.ai_state = CHASE
        
        FLEE:
            let flee_dir = (monster.pos - world.get_pos(monster.target)).normalize()
            let flee_target = monster.pos + flee_dir * FLEE_DISTANCE
            monster.set_path(pathfind(monster.pos, flee_target))
            
            // Vérifier si on peut arrêter de fuir
            if monster.hp_percent() > 0.3:
                monster.ai_state = CHASE
        
        STUN:
            // Hit recovery animation en cours
            if animation_complete(monster):
                monster.ai_state = CHASE
        
        DEAD:
            // deathDmg explosion
            if monster.death_damage:
                apply_death_explosion(monster)
            // Spawn minion1 si splEndDeath=1
            if monster.spl_end_death == 1:
                spawn(monster.minion1, monster.pos)

fn select_attack(params: &[f32; 8]) -> AttackType:
    // Roll aléatoire pondéré par les paramètres aip
    let roll = random_percent(100)
    if roll < params[0]:
        return AttackType::Skill1
    elif roll < params[0] + params[1]:
        return AttackType::Skill2
    // etc.
    return AttackType::BasicA1
```

## 2.6 Version MGE ECS

### Components

```rust
pub struct MonsterAi {
    pub ai_type: AiTypeId,
    pub state: AiState,
    pub ai_delay: u32,           // frames entre ticks IA
    pub ai_dist: f32,            // distance d'activation (px)
    pub ai_params: [f32; 8],
    pub target: Option<EntityId>,
    pub last_tick: u32,
    pub leash_origin: Vec2,      // position de spawn (pour leash)
    pub leash_distance: f32,
    pub flee_threshold: f32,     // % HP pour fuir
}

pub enum AiState {
    Idle, Wander, Chase, Attack, RangedAttack,
    Flee, Stun, Dead, Corpse, Special(u8),
}

pub struct ThreatInfo {
    pub threat_level: f32,
    pub pet_ignore: bool,
    pub prime_evil: bool,        // +300% dmg vs pets
}

pub struct PackInfo {
    pub leader: Option<EntityId>,
    pub members: Vec<EntityId>,
    pub boss_xfer: bool,
}
```

### Systems

```rust
// @phase 200 — AI tick (après mouvement)
pub fn monster_ai_think(world: &mut World, ctx: &mut Context) { }

// @phase 210 — Résolution d'aggro
pub fn aggro_resolve(world: &mut World, ctx: &mut Context) { }

// @phase 220 — Sélection de cible
pub fn target_selection(world: &mut World, ctx: &mut Context) { }

// @phase 230 — Exécution d'attaque
pub fn attack_execution(world: &mut World, ctx: &mut Context) { }
```

### Config JSON exemple (équivalent MonStats.txt)

```json
{
  "monster_defs": [
    {
      "id": "fallen1",
      "base_id": "fallen1",
      "ai_type": "Fallen",
      "ai_delay": [8, 6, 4],
      "ai_dist": [35.0, 35.0, 35.0],
      "ai_params": [[60, 30, 0, 0, 0, 0, 0, 0],
                     [70, 40, 0, 0, 0, 0, 0, 0],
                     [80, 50, 0, 0, 0, 0, 0, 0]],
      "threat": 1,
      "velocity": 5,
      "run_velocity": 9,
      "melee_range": 2,
      "hp": [[15, 25], [200, 350], [1500, 2800]],
      "flags": { "is_melee": true, "can_open_doors": true }
    }
  ]
}
```

---

# SYSTÈME 3 — SPAWN SYSTEM

## 3.1 Description fonctionnelle

Le spawn de D2 est **déterministe par seed** : même seed = même carte = mêmes monstres aux mêmes positions. Le spawn se produit à la création de la partie, pas en continu.

## 3.2 Pipeline de spawn complet

```
1. Carte générée (seed)
   ↓
2. Pour chaque zone (Levels.txt):
   a. Lire MonDen (densité de monstres)
   b. Lire M1-M25 (types de monstres éligibles)
   c. Lire U1-U25 (types uniques éligibles)
   ↓
3. Pour chaque point de spawn valide:
   a. Sélectionner le type de monstre (Rarity weighting)
   b. Déterminer la taille du pack (MinGrp-MaxGrp)
   c. Roll élite (Champion/Unique)
   d. Placer les monstres aux subtiles passables
   ↓
4. Pour chaque Unique généré:
   a. Roll affixes (1/2/3 selon difficulté)
   b. Générer minions (PartyMin-PartyMax)
   c. Appliquer bonus stats
```

## 3.3 Formules de spawn

### Sélection de monstre par Rarity

```
P(monstre_i) = rarity_i / Σ(rarity_j pour tous j éligibles)
```

### sparsePopulate

```
if random(0, 100) > sparsePopulate:
    skip_this_spawn()
```

### Taille de pack

```
group_size = random_range(MinGrp, MaxGrp)
minion_count = random_range(PartyMin, PartyMax)
```

### Génération d'élite

```
fn roll_elite(difficulty):
    roll = random(0, 100)
    
    match roll:
        0..=7   => generate_unique(difficulty)
        8..=27  => generate_champion_pack(difficulty)
        _       => generate_standard_pack()
```

### Affixes Unique — Pool complet

| Affix | ID | Effet principal |
|-------|----|----------------|
| Extra Strong | 5 | +dmg%, +AR% |
| Extra Fast | 6 | +velocity, +attack speed |
| Cursed | 7 | Amplify Damage on hit |
| Magic Resistant | 8 | +res all |
| Fire Enchanted | 9 | +fire dmg, fire explosion death |
| Lightning Enchanted | 17 | Charged bolts when hit, death nova |
| Cold Enchanted | 18 | +cold dmg, frost nova death |
| Mana Burn | 25 | Drain mana ×256 |
| Teleportation | 26 | Random teleport |
| Spectral Hit | 27 | Random elemental dmg |
| Stone Skin | 28 | +80% phys res, -50% speed |
| Multishot | 29 | Multiple projectiles (ranged only) |
| Aura Enchanted | 30 | Random aura |
| Conviction | 40 | Conviction aura (reduces res) |

### Règles de combinaison d'affixes

```
fn pick_affixes(count, monster):
    pool = ALL_AFFIXES.clone()
    
    // Retirer Multishot si mêlée
    if monster.is_melee: pool.remove(Multishot)
    
    selected = []
    for _ in 0..count:
        affix = pool.random()
        
        // Vérifier : pas de 3e immunité
        if would_create_third_immunity(monster.res + affix.res_bonus):
            continue
        
        // Vérifier : pas d'augmentation d'immunité existante
        if increases_existing_immunity(monster.res, affix.res_bonus):
            continue
        
        selected.push(affix)
        pool.remove(affix)
    
    return selected
```

## 3.4 Champion variants (données exactes)

| Variante | HP mult | Dmg mult | AR mult | XP mult | Spécial |
|----------|---------|----------|---------|---------|---------|
| Champion | ×3/2.5/2 | +90/75/66% | +67/56/49% | ×3 | — |
| Berserker | ×0.75 champ | +270/225/198% | +270/225/198% | ×5 | Glass cannon |
| Fanatic | ×3/2.5/2 | +90/75/66% | +67/56/49% | ×3 | Similaire standard |
| Ghostly | ×3/2.5/2 | — | — | ×3 | 80% phys res, +cold dmg |
| Possessed | ×6 champ | Standard | Standard | ×3 | Immune malédictions |

## 3.5 Version MGE ECS

```rust
pub struct SpawnZone {
    pub zone_id: u32,
    pub area_level: u32,
    pub density: f32,              // MonDen / 100
    pub eligible_monsters: Vec<SpawnEntry>,
    pub elite_chance: f32,
    pub max_population: u32,
    pub respawn_delay: Option<f32>, // None pour fidélité D2, Some(s) pour Allumina
}

pub struct SpawnEntry {
    pub monster_def_id: String,
    pub rarity: u32,
    pub min_group: u32,
    pub max_group: u32,
    pub party_min: u32,
    pub party_max: u32,
    pub sparse_chance: f32,
}

pub struct EliteModifier {
    pub affix_id: u32,
    pub name: String,
    pub hp_mult: f32,
    pub dmg_mult: f32,
    pub res_bonus: ResistanceSet,
    pub special_effect: Option<EliteEffect>,
}
```

---

# SYSTÈME 4 — PROJECTILES

## 4.1 Description fonctionnelle

Les projectiles D2 sont des **entités discrètes** traversant l'espace frame par frame. Ils ne sont **pas** hitscan. Le système est entièrement défini par Missiles.txt (~400 types).

## 4.2 Architecture interne (OpenDiablo2)

### Structure Missile (d2mapentity/missile.go)

```go
type Missile struct {
    *AnimatedEntity
    record *d2records.MissileRecord
}

func (m *Missile) SetRadians(angle float64, done func()) {
    r := float64(m.record.Range)
    x := m.Position.X() + (r * math.Cos(angle))
    y := m.Position.Y() + (r * math.Sin(angle))
    m.setTarget(d2vector.NewPosition(x, y), done)
}

func (m *Missile) Advance(tickTime float64) {
    m.Step(tickTime)
    m.AnimatedEntity.Advance(tickTime)
}
```

### MissileRecord complet (d2records/missiles_record.go) — Champs critiques

```go
type MissileRecord struct {
    // Mouvement
    Velocity         int    // pixels/frame
    MaxVelocity      int
    LevelVelocityBonus int  // bonus par niveau
    Accel            int    // accélération/frame
    Range            int    // durée de vie en frames
    LevelRangeBonus  int    // bonus range par niveau
    Size             int    // diamètre collision (subtiles)
    
    // Collision
    Collision MissileCollision
    // CollisionType: 0=none, 1=players, 3=normal, 6=walls, 8=all
    
    // Fonctions
    ServerMovementFunc  int  // fonction de mouvement serveur
    ClientMovementFunc  int  // fonction de mouvement client
    ServerCollisionFunc int  // fonction de collision serveur
    ServerDamageFunc    int  // fonction de dégâts
    
    // Dégâts
    HitShift    int    // dégâts = damage × 2^hitshift / 256
    SourceDamage int   // 0-128 (128 = 100% des dégâts source)
    UseAttackRating bool
    
    // Comportement
    AffectedByPierce bool
    AlwaysExplode    bool
    CanBeSlowed      bool
    
    // Sous-missiles
    SubMissile       [3]string  // spawned par mouvement
    HitSubMissile    [4]string  // spawned par collision
    ExplosionMissile string     // missile d'explosion
}
```

## 4.3 Formules de missiles

### Vélocité effective

```
vel_effective = Velocity + (skill_level × LevelVelocityBonus)
range_effective = Range + (skill_level × LevelRangeBonus)
```

### Distance en pixels

```
distance_subtiles = vel_effective × range_effective / 65536
distance_yards = distance_subtiles × 2/3
distance_px_horizontal = distance_subtiles × 32
distance_px_vertical = distance_subtiles × 16
```

### HitShift — Dégâts réels

```
actual_damage = raw_damage × 2^HitShift
visual_damage = actual_damage / 256
```

Ceci est le système de dégâts en **fixed-point arithmetic** de D2 (les dégâts sont calculés en 256èmes pour la précision).

### 9 types de collision (CollideType)

| Type | Comportement | Usage typique |
|------|-------------|---------------|
| 0 | Pas de collision | Effets visuels purs |
| 1 | Joueurs uniquement | (buggé — ne touche pas les monstres) |
| 2 | Monstres ennemis uniquement | Projectiles joueur standard |
| 3 | Monstres + joueurs | Projectiles multi-cibles |
| 4 | Expire toujours | Missiles fantômes |
| 5 | Copie du type 2 | — |
| 6 | Murs uniquement | Impacts verticaux |
| 7 | Missiles destructibles | Déprécié |
| 8 | Tout (murs + unités + sol) | Projectiles bloqués par terrain |

### Move Functions connues

| ID | Fonction | Comportement |
|----|----------|-------------|
| 1 | Linear | Ligne droite, vitesse constante |
| 15 | Homing | Ajuste la direction vers la cible chaque frame |
| 32 | Spiral | Mouvement en spirale |
| 48 | Random | Direction aléatoire (Blizzard shards) |

### Algorithme de missile guidé (homing)

```
fn missile_move_homing(missile, target, dt):
    if target.is_alive():
        desired_dir = normalize(target.pos - missile.pos)
        current_dir = missile.direction
        
        // Interpolation avec turn rate
        angle_diff = angle_between(current_dir, desired_dir)
        max_turn = missile.turn_rate * dt
        actual_turn = clamp(angle_diff, -max_turn, max_turn)
        
        missile.direction = rotate(current_dir, actual_turn)
    
    missile.pos += missile.direction * missile.velocity
    missile.range_remaining -= 1
    
    if missile.range_remaining <= 0:
        if missile.always_explode:
            trigger_explosion(missile)
        destroy(missile)
```

## 4.4 Mémoire anti-spam et piercing

```
fn check_missile_collision(missile, entities):
    for entity in entities:
        if entity.id == missile.last_hit_entity:
            if missile.next_hit_timer > 0:
                continue  // anti-spam
        
        if circles_overlap(missile.pos, missile.size, entity.pos, entity.hitbox):
            apply_missile_hit(missile, entity)
            missile.last_hit_entity = entity.id
            missile.next_hit_timer = missile.next_hit_delay
            
            if !missile.pierce:
                if missile.collide_kill:
                    destroy(missile)
                return
            // Pierce : continue la boucle
```

## 4.5 Version MGE ECS

### Components

```rust
pub struct Projectile {
    pub def_id: u32,
    pub velocity: f32,
    pub max_velocity: f32,
    pub acceleration: f32,
    pub range_remaining: u32,
    pub size: f32,
    pub collide_type: u8,
    pub hit_shift: u8,
    pub source_damage_pct: f32,     // 0.0–1.0
}

pub struct ProjectileBehavior {
    pub pierce: bool,
    pub always_explode: bool,
    pub use_attack_rating: bool,
    pub homing: bool,
    pub homing_target: Option<EntityId>,
    pub turn_rate: f32,
}

pub struct MissileHitMemory {
    pub last_hit: Option<EntityId>,
    pub next_hit_delay: u32,
    pub hit_count: u32,
}

pub struct SubMissileConfig {
    pub on_move: [Option<String>; 3],
    pub on_hit: [Option<String>; 4],
    pub on_explode: Option<String>,
}
```

### Séparation client/serveur

| Opération | Serveur (Lobby hôte) | Client |
|-----------|---------------------|--------|
| Création missile | Autoritaire | Prédiction locale |
| Mouvement | Calcul serveur | Interpolation visuelle |
| Collision | Détection serveur | Effets visuels |
| Dégâts | Calcul serveur uniquement | Affichage dégâts |
| Destruction | Décision serveur | Animation de destruction |

---

# SYSTÈME 5 — FOLLOWERS / MERCENAIRES / INVOCATIONS

## 5.1 Architecture (OpenDiablo2)

OpenDiablo2 n'implémente pas les mercenaires mais définit les structures de données :

### HirelingRecord (d2records/hireling_record.go)

Le record contient les stats de base par niveau, les skills, et les paramètres d'IA des mercenaires des 4 actes.

## 5.2 IA des followers (reconstitué depuis D2 original)

```
fn follower_think(follower, owner, world):
    let dist_to_owner = distance(follower.pos, owner.pos)
    
    // TÉLÉPORTATION si trop loin
    if dist_to_owner > TELEPORT_THRESHOLD:  // ~40-50 subtiles
        follower.pos = find_free_pos_near(owner.pos, 3)
        follower.clear_target()
        return
    
    // Si le propriétaire utilise Teleport
    if owner.just_teleported:
        follower.pos = find_free_pos_near(owner.teleport_dest, 2)
        return
    
    // PRIORITÉ 1: Suivre le propriétaire si trop loin
    if dist_to_owner > MAX_FOLLOW_DIST:  // ~15 subtiles
        follower.pathfind_to(owner.pos)
        follower.target = None
        return
    
    // PRIORITÉ 2: Cible du propriétaire
    if owner.target.is_some():
        let target = owner.target
        if can_see(follower, target) && is_alive(target):
            follower.target = Some(target)
            engage(follower, target)
            return
    
    // PRIORITÉ 3: Ennemi le plus proche
    let nearest = find_nearest_hostile(follower.pos, FOLLOWER_AGGRO_RANGE)
    if nearest.is_some():
        follower.target = nearest
        engage(follower, nearest)
        return
    
    // PRIORITÉ 4: Suivre le propriétaire (déplacement passif)
    if dist_to_owner > IDLE_DIST:  // ~5 subtiles
        follower.pathfind_to(owner.pos)
    else:
        follower.idle()
```

## 5.3 Spécificités par type

| Type | IA | Pathfinding | Téléport | Spécial |
|------|-----|-------------|----------|---------|
| **Merc Act 1** | Ranged (bow) | Standard | Oui | Reste à distance |
| **Merc Act 2** | Melee + aura | Standard | Oui | Aura bénéficie au groupe |
| **Merc Act 3** | Ranged (sort) | Standard | Oui | Cast élémentaire |
| **Merc Act 5** | Melee aggressif | Standard | Oui | Dual-wield |
| **Squelettes** | Melee/ranged | Standard | Oui (>screen) | Suivent le necro |
| **Golems** | Selon type | Standard | Non (sauf Iron) | 1 seul actif |
| **Revives** | IA originale | Standard | Oui | Conservent skills originaux |
| **Valkyrie** | Melee aggressif | Standard | Oui | Invoquée une seule |
| **Shadow** | Copie joueur | Standard | Oui | Utilise les skills de l'assassin |

## 5.4 Dégâts primeevil vs followers

Les Act Bosses (Diablo, Baal, Mephisto) avec `primeevil=1` infligent **+300% dégâts** aux mercenaires et invocations.

```
fn calculate_damage_vs_target(attacker, target, base_damage):
    let multiplier = if attacker.is_prime_evil && target.is_pet_or_merc:
        4.0  // +300% = ×4
    else:
        1.0
    return base_damage * multiplier
```

## 5.5 Version MGE ECS

```rust
pub struct Follower {
    pub owner: EntityId,
    pub follower_type: FollowerType,
    pub follow_distance: f32,
    pub teleport_threshold: f32,
    pub idle_distance: f32,
    pub aggro_range: f32,
    pub retain_original_ai: bool,
}

pub enum FollowerType {
    Mercenary { act: u8, aura: Option<AuraId> },
    Skeleton { is_mage: bool },
    Golem { golem_type: GolemType },
    Revive { original_monster_id: String },
    Valkyrie,
    Shadow { shadow_level: u32 },
}
```

---

# SYSTÈME 6 — SYSTÈME DE STATS ET COMBAT

## 6.1 Formules de combat exactes

### Chance to Hit

```
ChanceToHit = clamp(
    200 × (AR / (AR + DR)) × (aLvl / (aLvl + dLvl)),
    5,    // minimum
    95    // maximum
)
```

| Variable | Source |
|----------|--------|
| AR | `(Dex × 5 - 35 + ClassBonus) × (1 + AR_bonus% / 100) + flat_AR` |
| DR | `(BaseDefense + BonusDef) × (1 + SkillDefBonus%)` |
| aLvl | Niveau attaquant |
| dLvl | Niveau défenseur |

**Running :** le défenseur qui court a DR = 0 → hit rate ~automatique.

### Class AR bonuses

| Classe | Bonus |
|--------|-------|
| Amazon | +5 |
| Necromancer | -10 |
| Barbarian | +20 |
| Sorceress | -15 |
| Paladin | +20 |
| Druid | 0 |
| Assassin | +15 |

### Physical Damage Chain

```
step1 = BaseDmg × (1 + OnWeaponED%)
step2 = step1 + FlatDmg
step3 = step2 × (1 + OffWeaponED%)

OffWeaponED% = StrBonus% + SkillED% + AuraED% + GearOffED%
```

### Str/Dex bonus par type d'arme

| Type | Formule |
|------|---------|
| Mêlée standard | +1% par Str |
| Hammers | +1.1% par Str |
| Daggers/Claws | +0.75% Str + 0.75% Dex |
| Amazon Jav/Spear | +0.80% Str + 0.50% Dex |
| Bows/Crossbows | +1% par Dex |

### Crushing Blow

```
fn crushing_blow(target_hp, is_melee, target_type):
    let fraction = match (is_melee, target_type):
        (true, Normal)      => 1.0 / 4.0
        (true, Boss)        => 1.0 / 8.0
        (true, Player)      => 1.0 / 10.0
        (false, Normal)     => 1.0 / 8.0
        (false, Boss)       => 1.0 / 16.0
        (false, Player)     => 1.0 / 20.0
    
    return target_hp * fraction
```

### Deadly Strike + Critical Strike

```
fn check_critical(cs_percent, ds_percent):
    // Un seul check : CS d'abord, puis DS
    if random_percent(cs_percent):
        return true  // double damage
    
    let adjusted_ds = ds_percent * (1.0 - cs_percent / 100.0)
    if random_percent(adjusted_ds):
        return true
    
    return false

// Formule combinée
total_crit_chance = CS + DS × (1 - CS/100)
```

### Open Wounds — Dégâts par frame (en HP)

```
fn open_wounds_per_frame(clvl):
    match clvl:
        1..=15  => (9 * clvl + 31) as f32 / 256.0
        16..=30 => (18 * clvl - 104) as f32 / 256.0
        31..=45 => (27 * clvl - 374) as f32 / 256.0
        46..=60 => (36 * clvl - 779) as f32 / 256.0
        _       => (45 * clvl - 1319) as f32 / 256.0

// Durée : 200 frames (8 secondes)
// Modificateurs cible :
//   Joueur hostile (mêlée) : ×0.25
//   Joueur hostile (distance) : ×0.125
//   Boss/Super Unique : ×0.5
//   Autres : ×1.0
```

### Block Chance (LoD)

```
BlockChance = clamp(
    (BlockShield + BlockBonus) × (Dex - 15) / (cLvl × 2),
    0,
    75
)

// Running : block_chance = block_chance / 3 (cap 25%)

// Dex requise pour 75% block :
Dex_needed = (75 × cLvl × 2) / (BlockShield + BlockBonus) + 15
```

### Résistances et immunité

```
fn apply_resistance(base_damage, resistance):
    if resistance >= 100:  // IMMUNITÉ
        return base_damage  // pas de dégâts
    
    return base_damage × (1.0 - resistance / 100.0)

fn break_immunity(monster_res, reduction):
    // Conviction + Lower Resist opèrent à 1/5 contre immunité
    let effective_reduction = reduction / 5
    let new_res = monster_res - effective_reduction
    
    if new_res < 100:
        // Immunité brisée — les réductions gear s'appliquent à 100%
        return new_res
    else:
        return monster_res  // immunité non brisée
```

### Pénalités de résistance par difficulté

| Difficulté | Pénalité |
|------------|----------|
| Normal | 0% |
| Nightmare | -40% |
| Hell | -100% |

### Life/Mana Steal

```
fn leech(physical_damage, leech_percent, difficulty, drain_effectiveness):
    let diff_penalty = match difficulty:
        Normal => 1.0,
        Nightmare => 0.5,
        Hell => 0.333
    
    return physical_damage × (leech_percent / 100.0) × diff_penalty × drain_effectiveness
```

### Attack Speed (EIAS)

```
EIAS = 120 × IAS / (120 + IAS)

AnimFrames = floor(
    (AnimLength × 256) / floor(
        256 × (100 + SIAS + EIAS - WSM) / 100
    )
) - 1

// WSM = Weapon Speed Modifier (-60 rapide à +20 lent)
// SIAS = Skill IAS (buffs, debuffs, cap -50%)
```

### Régénération de vie (monstres)

```
hp_regen_per_frame = (DamageRegen × MaxHP) / 4096
hp_regen_per_second = hp_regen_per_frame × 25
```

## 6.2 Version MGE ECS

### Components

```rust
pub struct CombatStats {
    pub attack_rating: i32,
    pub defense: i32,
    pub level: u32,
    pub min_damage: [i32; 2],  // [A1, A2]
    pub max_damage: [i32; 2],
    pub block_chance: f32,
    pub critical_strike: f32,
    pub deadly_strike: f32,
    pub crushing_blow: f32,
    pub open_wounds: f32,
}

pub struct Resistances {
    pub physical: i32,    // -100 à 100+ (100+ = immune)
    pub fire: i32,
    pub cold: i32,
    pub lightning: i32,
    pub poison: i32,
    pub magic: i32,
}

pub struct Leech {
    pub life_steal_pct: f32,
    pub mana_steal_pct: f32,
    pub drain_effectiveness: f32,
}

pub struct AttackSpeed {
    pub base_anim_length: u32,
    pub ias_total: i32,
    pub weapon_speed_mod: i32,
    pub skill_ias: i32,
}
```

### Damage Resolution System

```rust
// @phase 300 — Résolution des dégâts
pub fn damage_resolution(world: &mut World, ctx: &mut Context) {
    // 1. Check hit (AR vs DR formula)
    // 2. Check block
    // 3. Calculate physical damage (full chain)
    // 4. Apply crushing blow
    // 5. Apply deadly strike / critical strike
    // 6. Apply elemental damage
    // 7. Apply resistances
    // 8. Apply leech
    // 9. Apply open wounds
    // 10. Apply on-hit effects
}
```

---

# SYSTÈME 7 — ARCHITECTURE DES ITEMS

## 7.1 Quality Determination Chain

### Algorithme exact

```
fn determine_quality(ilvl, magic_find):
    // Ordre de test : Unique → Set → Rare → Magic → Superior → Normal
    
    // Formules EMF (Effective Magic Find) avec Diminishing Returns
    let emf_unique  = magic_find × 250 / (magic_find + 250)
    let emf_set     = magic_find × 500 / (magic_find + 500)
    let emf_rare    = magic_find × 600 / (magic_find + 600)
    
    // Test Unique
    let chance_unique = max(
        (BaseChance_U - (ilvl - qlvl) / Divisor_U),
        MinChance_U
    )
    let effective_chance = chance_unique × 128 / (128 + emf_unique)
    if random(0, effective_chance) == 0:
        if check_unique_eligible(ilvl, item_type):
            return Quality::Unique
        else:
            return Quality::Rare  // downgrade, ×3 durability
    
    // Test Set
    let chance_set = max(
        (BaseChance_S - (ilvl - qlvl) / Divisor_S),
        MinChance_S
    )
    let effective_chance = chance_set × 128 / (128 + emf_set)
    if random(0, effective_chance) == 0:
        if check_set_eligible(ilvl, item_type):
            return Quality::Set
        else:
            return Quality::Rare  // downgrade, ×2 durability
    
    // Test Rare
    let chance_rare = max(
        (BaseChance_R - (ilvl - qlvl) / Divisor_R),
        MinChance_R
    )
    let effective_chance = chance_rare × 128 / (128 + emf_rare)
    if random(0, effective_chance) == 0:
        return Quality::Rare
    
    // Test Magic
    let chance_magic = max(
        (BaseChance_M - (ilvl - qlvl) / Divisor_M),
        MinChance_M
    )
    let effective_chance = chance_magic × 128 / (128 + emf_rare)  // uses rare EMF
    if random(0, effective_chance) == 0:
        return Quality::Magic
    
    // Test Superior (2%)
    if random(0, 50) == 0:
        return Quality::Superior
    
    // Test Normal vs Low Quality
    if random(0, 100) < 75:
        return Quality::Normal
    else:
        return Quality::LowQuality
```

### ItemRatio.txt — Valeurs exactes (v1.13)

| Qualité | BaseChance | Divisor | MinChance |
|---------|-----------|---------|-----------|
| Unique (Armor) | 400 | 1 | 6944 |
| Unique (Weapon) | 400 | 1 | 6944 |
| Set (Armor) | 160 | 2 | 5600 |
| Set (Weapon) | 160 | 2 | 5600 |
| Rare (Armor) | 100 | 2 | 6000 |
| Rare (Weapon) | 100 | 2 | 6000 |
| Magic (Armor) | 34 | 3 | 17 |
| Magic (Weapon) | 34 | 3 | 17 |

## 7.2 Affix Generation

### Calcul alvl (Affix Level)

```
fn calc_alvl(ilvl, qlvl, magic_lvl):
    let ilvl = min(ilvl, 99)
    let adjusted = ilvl + magic_lvl  // magic_lvl from item base type
    
    if adjusted < 99 - qlvl/2:
        return adjusted - qlvl/2
    else:
        return 2 × adjusted - 99
```

### Sélection d'affixes

```
fn pick_affixes(item, quality, alvl):
    let (num_prefix, num_suffix) = match quality:
        Magic => (random(0, 1).max(1), random(0, 1))  // au moins 1 affix total
        Rare  => {
            let np = count_successes(3, 0.25)  // 1-3 préfixes
            let ns = count_successes(3, 0.25)  // 1-3 suffixes
            (np.max(1), ns)                     // au moins 1 préfixe
        }
    
    let mut selected = Vec::new()
    let pool = get_affix_pool(item.type, quality)
    
    for _ in 0..num_prefix:
        let eligible = pool.prefixes
            .filter(|a| a.level <= alvl)
            .filter(|a| !conflicts_with(a, &selected))
        
        // Sélection pondérée par frequency
        let affix = weighted_random(eligible, |a| a.frequency)
        selected.push(affix)
    
    for _ in 0..num_suffix:
        // Identique pour les suffixes
```

## 7.3 Socket system

### Formule de sockets

```
fn determine_sockets(ilvl, item_type):
    let max_sockets = item_type.max_sockets_for_ilvl(ilvl)
    
    // Larzuk : donne toujours max_sockets sur Normal quality
    // Pour Magic : 1-2 aléatoire
    // Pour Rare/Set/Unique : 1
    
    match generation_method:
        Larzuk(Normal)  => max_sockets
        Larzuk(Magic)   => random(1, 2)
        Larzuk(Rare)    => 1
        Random          => random(1, min(6, max_sockets))

fn max_sockets_for_ilvl(item, ilvl):
    if ilvl <= 25:
        item.max_sockets_low    // MaxSock1
    elif ilvl <= 40:
        item.max_sockets_mid    // MaxSock25
    else:
        item.max_sockets_high   // MaxSock40
```

## 7.4 Structure OpenDiablo2 (d2records/treasure_class_record.go)

```go
type TreasureClassRecord struct {
    Name       string
    Group      int
    Level      int
    NumPicks   int
    FreqUnique int
    FreqSet    int
    FreqRare   int
    FreqMagic  int
    FreqNoDrop int
    Treasures  []*Treasure
}

type Treasure struct {
    Code        string
    Probability int
}
```

## 7.5 Version MGE ECS

```rust
pub struct Item {
    pub base_type: ItemBaseId,
    pub quality: ItemQuality,
    pub ilvl: u32,
    pub prefixes: Vec<AffixInstance>,
    pub suffixes: Vec<AffixInstance>,
    pub sockets: Vec<Option<SocketedItem>>,
    pub durability: (u32, u32),   // (current, max)
    pub is_ethereal: bool,
    pub is_identified: bool,
}

pub enum ItemQuality {
    LowQuality, Normal, Superior, Magic, Rare, Set(SetId), Unique(UniqueId),
}

pub struct AffixInstance {
    pub affix_def: AffixId,
    pub value: i32,              // rolled value within [min, max]
}

pub struct ItemGenerator {
    pub affix_pool: AffixPool,
    pub treasure_classes: HashMap<String, TreasureClass>,
    pub unique_items: Vec<UniqueItemDef>,
    pub set_items: Vec<SetItemDef>,
}
```

---

# SYSTÈME 8 — SYSTÈME DE SKILLS

## 8.1 Architecture (OpenDiablo2)

### SkillRecord (d2records/skill_details_record.go) — Champs critiques

```go
type SkillRecord struct {
    Skill     string
    Charclass string
    ID        int
    
    // Dégâts
    MinDam    int
    MaxDam    int
    MinLevDam1-5 int  // bonus par bracket de niveau
    MaxLevDam1-5 int
    EMin, EMax int     // dégâts élémentaires
    EType     string   // type d'élément
    
    // Coût
    Mana      int      // coût mana base
    Lvlmana   int      // coût additionnel par niveau
    Minmana   int      // coût minimum
    Startmana int      // coût au niveau 1
    Manashift int      // diviseur (Mana × 2^Manashift)
    
    // Missiles
    Srvmissile  string  // missile serveur
    Srvmissilea string
    Srvmissileb string
    Cltmissile  string  // missile client
    
    // Invocations
    Summon    string    // monstre invoqué
    Petmax    Calculation  // max invocations
    
    // Synergies (via CalcString)
    DmgSymPerCalc  Calculation
    EDmgSymPerCalc Calculation
    
    // Flags
    Passive    bool
    Aura       bool
    Periodic   bool
    Leftskill  bool
}
```

## 8.2 Formules de skills

### Mana Cost

```
fn mana_cost(skill, level):
    let base = skill.startmana + (level - 1) × skill.lvlmana
    let shifted = base >> skill.manashift
    return max(shifted, skill.minmana)
```

### Damage Scaling

```
fn skill_damage(skill, level):
    let min_bonus = match level:
        1..=8   => (level - 1) × skill.min_lev_dam_1
        9..=16  => 7 × skill.min_lev_dam_1 + (level - 8) × skill.min_lev_dam_2
        17..=22 => 7×d1 + 8×d2 + (level-16) × skill.min_lev_dam_3
        23..=28 => 7×d1 + 8×d2 + 6×d3 + (level-22) × skill.min_lev_dam_4
        _       => 7×d1 + 8×d2 + 6×d3 + 6×d4 + (level-28) × skill.min_lev_dam_5
    
    return (skill.min_dam + min_bonus, skill.max_dam + max_bonus)
```

### Synergies

```
synergy_bonus = hard_points_in_synergy × synergy_percent_per_point
final_damage = base_damage × (1 + synergy_bonus / 100)

// RÈGLE FONDAMENTALE : seuls les HARD POINTS comptent pour les synergies
// Les +skills d'items N'AJOUTENT PAS aux synergies
```

### Skill Tree Structure

```
Par classe : 3 tabs, ~10 skills par tab, ~30 skills total
Chaque skill a :
  - reqlevel (niveau requis)
  - reqskill1-3 (prérequis dans l'arbre)
  - maxlvl (20 natif, 20+ via +skills)
  
Points de skill : 1 par niveau (niveaux 2-99 = 98 points)
```

## 8.3 Version MGE ECS

```rust
pub struct SkillInstance {
    pub skill_def: SkillDefId,
    pub hard_points: u32,        // investis par le joueur
    pub soft_bonus: i32,         // via +skills items
    pub effective_level: u32,    // hard + soft
}

pub struct SkillDef {
    pub id: String,
    pub class: Option<ClassId>,
    pub tab: u8,
    pub row: u8,
    pub col: u8,
    pub req_level: u32,
    pub prerequisites: Vec<SkillDefId>,
    pub max_level: u32,
    pub mana_cost: ManaCostDef,
    pub damage: DamageDef,
    pub synergies: Vec<Synergy>,
    pub missile: Option<String>,
    pub is_passive: bool,
    pub is_aura: bool,
}

pub struct Synergy {
    pub source_skill: SkillDefId,
    pub percent_per_point: f32,   // ex: 14% par point
}

pub struct ManaCostDef {
    pub start_mana: i32,
    pub lvl_mana: i32,
    pub min_mana: i32,
    pub mana_shift: u8,
}
```

---

# SYSTÈME 9 — LOOT ET TREASURE CLASSES

## 9.1 Hiérarchie des TC

```
Boss Kill
  → Roll TC (ex: "Baal (H)")
    → Roll Picks (NumPicks fois)
      → Pour chaque pick:
        → Roll NoDrop vs items
        → Si item:
          → Résoudre TC récursive (peut pointer vers autre TC)
          → Atteindre une TC atomique (ex: "Weap87", "Armo87")
          → Générer l'item base
          → Déterminer la qualité (§7.1)
          → Générer les affixes (§7.2)
```

## 9.2 Formule NoDrop exacte

### Single player

```
P(NoDrop) = NoDrop / (NoDrop + ΣProb_i)
P(Drop_i) = Prob_i / (NoDrop + ΣProb_i)
```

### Multiplayer scaling

```
N = floor(1 + AdditionalPlayers/2 + ClosePartiedPlayers/2)

NewNoDrop = floor(
    ΣProb / (1 / ((NoDrop / (NoDrop + ΣProb))^N) - 1)
)
```

Où N est le nombre effectif de joueurs. Plus N augmente, plus NoDrop diminue.

### Exemple : Mephisto Hell (TC = "Mephisto (H)")

```
NoDrop = 15
Prob1 (Mephisto TC) = 78
Prob2 (Gold) = 5
Prob3 (Junk) = 7
ΣProb = 90

P(NoDrop) = 15 / (15 + 90) = 14.3%

Avec 8 joueurs (N=4):
NewNoDrop = floor(90 / (1/(0.143^4) - 1)) = floor(90 / (2387 - 1)) = floor(0.0377) = 0
→ NoDrop = 0 → toujours un drop
```

## 9.3 Quest Drop

Les Act Bosses ont une TC spéciale pour le **premier kill** (quest drop) :

```
Quest drop multiplier: QualityFactor = 983/1024 ≈ 96%
Appliqué à la chance Unique/Set/Rare (multiplie la chance)
```

### Bug Andariel

Andariel utilise toujours la quest drop table si le joueur complète sa quête et sauvegarde/quitte **dans la même session**. Ce bug permet de farmer indéfiniment avec des taux de drop quest.

## 9.4 TC Upgrade (Nightmare/Hell)

En NM et Hell, les TC atomiques sont **upgradées** :

```
fn upgrade_tc(base_tc, difficulty):
    match difficulty:
        Nightmare => base_tc.upgrade(3)  // +3 tiers
        Hell      => base_tc.upgrade(6)  // +6 tiers
```

Ex : "Weap3" en Normal → "Weap6" en NM → "Weap9" en Hell.

## 9.5 Version MGE ECS

```rust
pub struct TreasureClass {
    pub name: String,
    pub level: u32,
    pub num_picks: i32,        // négatif = chaque item indépendant
    pub no_drop: u32,
    pub entries: Vec<TcEntry>,
    pub quality_factors: QualityFactors,
}

pub struct TcEntry {
    pub code: String,          // item code ou TC reference
    pub probability: u32,
}

pub struct QualityFactors {
    pub unique_mult: u32,
    pub set_mult: u32,
    pub rare_mult: u32,
    pub magic_mult: u32,
}
```

---

# SYSTÈME 10 — GÉNÉRATION PROCÉDURALE DES DONJONS

## 10.1 Seed System

### Stockage

```
Game seed : u32 (stocké dans .d2s à offset 0xAB, Little Endian)
```

### Chaîne de dérivation

```
Game Seed (32-bit)
  → Act Seed = derive(game_seed, act_id)
    → Level Seed = derive(act_seed, level_id)
      → Room Seeds = derive(level_seed, room_index)
```

Le même seed produit **toujours** la même carte. Les monstres sont placés après la génération de carte.

## 10.2 Types de génération (DrlgType)

### Type 1 — Random Maze (donjons, caves)

```
Rooms = DS1 presets de 25×25 tiles
Chaque room a des ouvertures codées sur 4 bits (NSEW):
  N=8, S=4, E=2, W=1
  
Exemples:
  0101 (N+E) = ouvertures Nord et Est
  1111 (NSEW) = carrefour complet
  
15 combinaisons possibles × jusqu'à 6 variantes par type

Algorithme:
  1. Placer la room d'entrée
  2. Pour chaque ouverture non connectée:
     a. Choisir un preset compatible (ouverture alignée)
     b. Vérifier qu'il ne chevauche pas de rooms existantes
     c. Placer la room
  3. Répéter jusqu'à ce que toutes les ouvertures soient connectées
  4. Boucher les ouvertures restantes avec des dead-ends
```

### Type 2 — Preset (villes, zones fixes)

```
Un seul DS1 fixe, avec jusqu'à 6 variantes aléatoires.
La variante est choisie par le seed.
```

### Type 3 — Random Wilderness (extérieurs)

```
Algorithme en 4 étapes:
  1. Placer les bordures (bords de carte)
  2. Tracer les chemins et waypoints
  3. Placer les presets thématiques (camps, structures)
  4. Remplir le reste avec des tuiles aléatoires (LvlSub.txt)
```

## 10.3 Format DT1 (tuiles)

| Champ | Taille | Description |
|-------|--------|-------------|
| File Header | 276 octets | Version, nombre de tuiles |
| Tile Header | 96 octets par tuile | Direction, main index, sub index, rarity, flags |
| Sub-tile flags | 25 × u16 par tuile | Passabilité par subtile (16 bits) |
| Block data | Variable | Pixels (RAW isométrique 256 oct ou RLE 32×32) |

### Identification d'une tuile

```
Triplet unique : (Orientation 0-19, MainIndex 0-63, SubIndex 0-63)
```

## 10.4 Map Generator (OpenDiablo2 — d2mapgen/map_generator.go)

```go
type MapGenerator struct {
    asset  *d2asset.AssetManager
    engine *d2mapengine.MapEngine
}

func (g *MapGenerator) loadPreset(id, index int) *d2mapstamp.Stamp {
    for _, file := range g.asset.Records.LevelPreset(id).Files {
        g.engine.AddDS1(file)
    }
    return g.engine.LoadStamp(d2enum.RegionAct1Wilderness, id, index)
}

func areaEmpty(mapEngine *d2mapengine.MapEngine, rect d2geom.Rectangle) bool {
    // Vérifie qu'une zone est vide avant d'y placer un preset
    for y := rect.Top; y <= rect.Bottom(); y++ {
        for x := rect.Left; x <= rect.Right(); x++ {
            floor := mapEngine.Tile(x, y).Components.Floors[0]
            if floor.Style != 0 || floor.Sequence != 0 || floor.Prop1 != 1 {
                return false
            }
        }
    }
    return true
}
```

## 10.5 Levels.txt — Paramètres de génération clés

| Colonne | Rôle |
|---------|------|
| `DrlgType` | 1=Maze, 2=Preset, 3=Wilderness |
| `MonDen` / `MonDen(N)` / `MonDen(H)` | Densité de monstres (% du max) |
| `MonUMin` / `MonUMax` | Min/max groupes Unique dans la zone |
| `NumMon` | Nombre de types de monstres différents |
| `M1-M25` | Types de monstres normaux éligibles |
| `U1-U25` | Types uniques éligibles |
| `S1-S25` | Types spéciaux (critters) |
| `SizeX` / `SizeY` | Taille de la carte en tiles |
| `IsInside` | 0=extérieur, 1=intérieur |
| `Waypoint` | ID du waypoint dans la zone |

## 10.6 Version MGE ECS

### Adaptation pour Allumina (monde persistant)

Contrairement à D2 (carte régénérée par partie), Allumina vise un monde persistant (type UO). L'adaptation :

| Concept D2 | Adaptation Allumina |
|------------|---------------------|
| Seed par partie | Seed par zone (stocké dans KindMother) |
| Carte jetable | Carte persistante, sauvegardée |
| Monstres one-shot | Respawn par timer configurable |
| DrlgType | Éditeur de carte + procédural hybride |

### Structures

```rust
pub struct LevelDef {
    pub id: u32,
    pub name: String,
    pub drlg_type: DrlgType,
    pub size: (u32, u32),
    pub monster_density: f32,
    pub eligible_monsters: Vec<String>,
    pub elite_min: u32,
    pub elite_max: u32,
    pub area_level: [u32; 3],
    pub tile_set: TileSetId,
    pub presets: Vec<PresetRef>,
}

pub enum DrlgType {
    RandomMaze { room_size: u32, max_rooms: u32 },
    Preset { ds1_variants: Vec<String> },
    Wilderness { border_preset: String, path_preset: String },
}

pub struct MapChunk {
    pub tiles: Vec<Vec<Tile>>,
    pub subtile_flags: Vec<Vec<u16>>,
    pub spawn_points: Vec<SpawnPoint>,
    pub waypoint: Option<Vec2>,
}
```

---

# SYNTHÈSE — MAPPING COMPLET D2 → MGE

## Fichiers D2 → Plugins MGE

| Fichier D2 | Plugin MGE | Composants principaux |
|------------|------------|----------------------|
| MonStats.txt | `mge-plugin-monster-stats.v1` | `MonsterDef`, `MonsterInstance`, `CombatStats` |
| MonStats2.txt | `mge-plugin-monster-collision.v1` | `MonsterHitbox`, `MonsterSize` |
| Missiles.txt | `mge-plugin-projectile.v1` | `Projectile`, `ProjectileBehavior`, `MissileHitMemory` |
| Skills.txt | `mge-plugin-skills.v1` | `SkillDef`, `SkillInstance`, `Synergy` |
| Levels.txt | `mge-plugin-zone.v1` | `LevelDef`, `SpawnZone` |
| TreasureClassEx.txt | `mge-plugin-loot.v1` | `TreasureClass`, `TcEntry` |
| ItemTypes.txt | `mge-plugin-items.v1` | `Item`, `ItemQuality`, `AffixInstance` |
| UniqueItems.txt | `mge-plugin-items.v1` | `UniqueItemDef` |
| SetItems.txt | `mge-plugin-items.v1` | `SetItemDef` |
| SuperUniques.txt | `mge-plugin-elite-gen.v1` | `SuperUniqueDef` |
| CharStats.txt | `mge-plugin-character.v1` | `CharacterDef`, `ClassBonus` |
| Experience.txt | `mge-plugin-progression.v1` | `ExpTable` |
| DifficultyLevels.txt | `mge-plugin-difficulty.v1` | `DifficultyConfig` |
| Hireling.txt | `mge-plugin-followers.v1` | `HirelingDef`, `Follower` |

## Architecture event flow globale

```
[FRAME START]
  ↓
[Phase 100] Input Processing
  ↓
[Phase 110] Pathfinding Update (A* recalc)
  ↓
[Phase 120] Locomotion Update (movement)
  ↓
[Phase 130] Collision Resolution (spatial hash)
  ↓
[Phase 140] Orientation Update (facing)
  ↓
[Phase 200] Monster AI Think (FSM tick)
  ↓
[Phase 210] Aggro Resolution
  ↓
[Phase 220] Target Selection
  ↓
[Phase 230] Attack Execution → emit DamageEvent
  ↓
[Phase 240] Skill Execution → emit MissileSpawnEvent
  ↓
[Phase 250] Follower AI Think
  ↓
[Phase 300] Damage Resolution (hit check, damage calc, resistances)
  ↓
[Phase 310] Status Effect Application (OW, freeze, stun)
  ↓
[Phase 320] Death Processing (loot generation, corpse)
  ↓
[Phase 400] Projectile Movement
  ↓
[Phase 410] Projectile Collision Detection
  ↓
[Phase 420] Projectile Hit Processing → emit DamageEvent (loop to 300)
  ↓
[Phase 500] Spawn System (respawn timer check)
  ↓
[Phase 600] Loot Generation (on DamageEvent.kill)
  ↓
[Phase 900] Network Sync (send state to clients)
  ↓
[Phase 950] Animation Update
  ↓
[FRAME END]
```

## Déterminisme réseau

Pour le multijoueur via MWS (Lobby hôte autoritaire) :

| Aspect | Stratégie |
|--------|-----------|
| **Tick rate** | 30 FPS logique fixe |
| **RNG** | Seed partagé, RNG déterministe (PCG) |
| **Mouvement** | Serveur autoritaire, client prédit |
| **Combat** | Serveur uniquement (pas de client damage) |
| **Loot** | Serveur uniquement (pas de client loot) |
| **IA** | Serveur uniquement |
| **Projectiles** | Serveur calcule, client interpole |
| **Input** | Client envoie inputs, serveur exécute |

---

## Risques techniques par système

| Système | Risque | Mitigation |
|---------|--------|------------|
| **Pathfinding** | O(n²) si pas de spatial hash | Spatial hash obligatoire |
| **IA** | 200+ mobs × FSM par frame | aidel + skip quand hors écran |
| **Projectiles** | Tunneling (traverser les murs) | CCD (Continuous Collision Detection) pour missiles rapides |
| **Loot** | Génération trop lente | Pré-calculer les pools d'affixes éligibles |
| **Combat** | Floating point inconsistency réseau | Fixed-point arithmetic (comme D2) |
| **Map gen** | Mémoire pour cartes larges | Streaming par chunks |
| **Stats** | Formules complexes = bugs subtils | Tests unitaires exhaustifs pour chaque formule |
| **Items** | Combinatoire d'affixes explosive | Limiter les affixes par catégorie |
| **Skills** | Synergie hard/soft points confusion | Validation stricte (seuls hard points comptent) |
| **Followers** | Pathfinding bloqué | Téléportation automatique (threshold configurable) |

---

## Références

| Source | URL | Données |
|--------|-----|---------|
| OpenDiablo2 | github.com/OpenDiablo2/OpenDiablo2 | Code source Go |
| D2MOO | github.com/ThePhrozenKeep/D2MOO | Reverse-engineering C++ |
| Phrozen Keep KB | d2mods.info/forum/kb | MonStats, Missiles, Items |
| Maxroll.gg | maxroll.gg/d2/resources | Formules combat, breakpoints |
| Amazon Basin | theamazonbasin.com/wiki | Formules détaillées |
| mannm.org | mannm.org/d2library | Open Wounds, IAS, Missiles |
| Paul SIRAMY | paul.siramy.free.fr | Spécification DT1 |
| d2mods data | github.com/fabd/diablo2 | Fichiers .txt v1.13 |

---

**Document** : Allumina — Extraction Systèmes D2 (OpenDiablo2) pour MGE  
**Version** : 1.0  
**Date** : 2026-02-22  
**Statut** : Document de référence technique exhaustif
