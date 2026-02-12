# MiyukiniLifeGame — Architecture Technique

## Contexte

Ce document détaille l'**architecture technique** de Miyukini Life Game : organisation des modules, flux de données, Toolkits requis, intégration avec les Cores Miyukini COG.

## Portée / Scope

- Architecture des crates et modules
- Flux de données et événements
- Intégration dans la pyramide Miyukini COG
- Toolkits requis et leur responsabilité
- Format de sauvegarde
- Performance et optimisations

## Vue d'ensemble architecturale

### Position dans Miyukini COG

```
┌─────────────────────────────────────────┐
│  Strate 9 — MiyukiniAdmin (Souverain)   │
└─────────────────────────────────────────┘
                    │
┌─────────────────────────────────────────┐
│  Strate 7 — Services                     │
│  ┌─────────────────────────────────┐    │
│  │  MiyukiniLifeGame (Service)     │    │
│  │  - UI Dioxus                     │    │
│  │  - Orchestration gameplay        │    │
│  └─────────────────────────────────┘    │
└─────────────────────────────────────────┘
                    │
┌─────────────────────────────────────────┐
│  Strate 7 — Opérateurs                  │
│  ┌──────────┬──────────┬──────────┬───┐ │
│  │Simulation│Entities  │World     │Pow│ │
│  └──────────┴──────────┴──────────┴───┘ │
└─────────────────────────────────────────┘
                    │
┌─────────────────────────────────────────┐
│  Strate 6 — Toolkits                    │
│  ┌────────────────────────────────────┐ │
│  │  MiyuWorldGen, MiyuPixelCanvas,    │ │
│  │  MiyuEntitySim, MiyuDiplomacy,     │ │
│  │  MiyuPathfinding, MiyuParticles,   │ │
│  │  MiyuTimeControl, MiyuSaveFormat   │ │
│  └────────────────────────────────────┘ │
└─────────────────────────────────────────┘
                    │
┌─────────────────────────────────────────┐
│  Strate 4 — Cores                       │
│  StrongFather, KindMother, CaringNanny, │
│  MasterButler, EverBuddy, WorrySentinel │
└─────────────────────────────────────────┘
```

## Organisation des crates

### Structure des dossiers

```
crates/
├── miyukini-life-game/          # Service principal
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── app.rs               # App Dioxus
│   │   ├── state.rs             # État global
│   │   ├── ui/
│   │   │   ├── mod.rs
│   │   │   ├── power_palette.rs
│   │   │   ├── world_canvas.rs
│   │   │   ├── stats_panel.rs
│   │   │   └── timeline.rs
│   │   └── services/
│   │       ├── mod.rs
│   │       ├── game_loop.rs
│   │       └── power_handler.rs
│   └── assets/
│       ├── sprites/
│       └── sounds/
│
├── lifegame-simulation/         # Opérateur Simulation
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── world_tick.rs        # Tick du monde
│       ├── physics.rs           # Physique simple
│       └── ai/
│           ├── unit_ai.rs
│           └── kingdom_ai.rs
│
├── lifegame-entities/           # Opérateur Entities
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── unit.rs
│       ├── building.rs
│       ├── creature.rs
│       └── entity_manager.rs
│
├── lifegame-world/              # Opérateur World
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── terrain.rs
│       ├── biome.rs
│       ├── resources.rs
│       └── generation.rs
│
├── lifegame-powers/             # Opérateur Powers
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── creation.rs
│       ├── destruction.rs
│       ├── creatures.rs
│       └── magic.rs
│
└── miyukini-toolkits/           # Toolkits
    ├── miyu-worldgen/
    ├── miyu-pixelcanvas/
    ├── miyu-entitysim/
    ├── miyu-diplomacy/
    ├── miyu-pathfinding/
    ├── miyu-particles/
    ├── miyu-timecontrol/
    └── miyu-saveformat/
```

### Dépendances entre crates

```rust
// Cargo.toml du service principal
[dependencies]
dioxus = "0.6"
dioxus-desktop = "0.6"

# Opérateurs
lifegame-simulation = { path = "../lifegame-simulation" }
lifegame-entities = { path = "../lifegame-entities" }
lifegame-world = { path = "../lifegame-world" }
lifegame-powers = { path = "../lifegame-powers" }

# Toolkits
miyu-worldgen = { path = "../miyukini-toolkits/miyu-worldgen" }
miyu-pixelcanvas = { path = "../miyukini-toolkits/miyu-pixelcanvas" }
miyu-entitysim = { path = "../miyukini-toolkits/miyu-entitysim" }
miyu-diplomacy = { path = "../miyukini-toolkits/miyu-diplomacy" }
miyu-pathfinding = { path = "../miyukini-toolkits/miyu-pathfinding" }
miyu-particles = { path = "../miyukini-toolkits/miyu-particles" }
miyu-timecontrol = { path = "../miyukini-toolkits/miyu-timecontrol" }
miyu-saveformat = { path = "../miyukini-toolkits/miyu-saveformat" }

# Cores
strongfather = { path = "../strongfather" }
kindmother = { path = "../kindmother" }
caringnanny = { path = "../caringnanny" }
masterbutler = { path = "../masterbutler" }
everbuddy = { path = "../everbuddy" }
worrysentinel = { path = "../worrysentinel" }

# Utilitaires
serde = { version = "1.0", features = ["derive"] }
rayon = "1.10"
rand = "0.8"
```

## Modèle de données principal

### Structure World

```rust
/// Représentation complète d'un monde
#[derive(Serialize, Deserialize, Clone)]
pub struct World {
    /// Métadonnées
    pub metadata: WorldMetadata,
    
    /// Terrain et environnement
    pub terrain: TerrainGrid,
    
    /// Toutes les entités
    pub entities: EntityManager,
    
    /// Royaumes et diplomatie
    pub kingdoms: Vec<Kingdom>,
    pub relations: RelationMatrix,
    
    /// Historique
    pub history: EventLog,
    pub timeline: Vec<HistoricalEvent>,
    
    /// Simulation
    pub tick: u64,              // Tick actuel
    pub speed: TimeSpeed,       // Vitesse de simulation
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WorldMetadata {
    pub id: WorldId,
    pub name: String,
    pub size: (u32, u32),
    pub seed: u64,
    pub created_at: Timestamp,
    pub last_played: Timestamp,
    pub playtime: Duration,
}
```

### Structure TerrainGrid

```rust
/// Grille de terrain
#[derive(Serialize, Deserialize, Clone)]
pub struct TerrainGrid {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<Tile>>,     // [y][x]
    pub biomes: BiomeMap,
    pub resources: ResourceMap,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Tile {
    pub terrain_type: TerrainType,
    pub elevation: u8,              // 0-255
    pub temperature: i8,            // -50 à +50°C
    pub moisture: u8,               // 0-100%
    pub fertility: u8,              // 0-100%
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum TerrainType {
    DeepWater,
    ShallowWater,
    Beach,
    Grass,
    Forest,
    Hill,
    Mountain,
    Snow,
    Desert,
    Swamp,
    Lava,
}
```

### Structure EntityManager

```rust
/// Gestionnaire centralisé d'entités
pub struct EntityManager {
    pub units: HashMap<EntityId, Unit>,
    pub buildings: HashMap<EntityId, Building>,
    pub creatures: HashMap<EntityId, Creature>,
    
    // Index spatiaux pour performance
    spatial_index: SpatialHashGrid,
    
    // Compteurs
    next_id: EntityId,
}

impl EntityManager {
    pub fn spawn_unit(&mut self, unit: Unit) -> EntityId { /* ... */ }
    pub fn kill_unit(&mut self, id: EntityId) { /* ... */ }
    pub fn units_in_radius(&self, pos: Vec2, radius: f32) -> Vec<&Unit> { /* ... */ }
    pub fn buildings_at(&self, pos: Vec2) -> Vec<&Building> { /* ... */ }
}
```

### Structure Unit

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: EntityId,
    pub name: String,
    pub race: Race,
    pub kingdom: Option<KingdomId>,
    
    // Position
    pub position: Vec2,
    pub velocity: Vec2,
    pub target_position: Option<Vec2>,
    
    // Stats
    pub health: u32,
    pub max_health: u32,
    pub attack: u32,
    pub defense: u32,
    pub speed: f32,
    
    // Besoins (0-100)
    pub hunger: u8,
    pub thirst: u8,
    pub energy: u8,
    pub happiness: i8,      // -100 à +100
    
    // Âge et cycle de vie
    pub age: u32,           // en jours
    pub max_age: u32,
    
    // Rôle
    pub role: UnitRole,
    
    // Équipement
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
    
    // Traits
    pub traits: Vec<Trait>,
    
    // Relations
    pub family: FamilyTree,
    pub friends: Vec<EntityId>,
    pub enemies: Vec<EntityId>,
    
    // Expérience
    pub level: u8,
    pub experience: u32,
    pub kills: u32,
}
```

### Structure Kingdom

```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Kingdom {
    pub id: KingdomId,
    pub name: String,
    pub race: Race,
    
    // Dirigeant
    pub king: Option<EntityId>,
    
    // Territoires
    pub capital: Option<BuildingId>,
    pub cities: Vec<BuildingId>,
    pub territory: Vec<Vec2>,       // Polygone de frontière
    
    // Population
    pub population: u32,
    pub units: Vec<EntityId>,
    
    // Ressources
    pub resources: ResourceStorage,
    
    // Militaire
    pub army: Vec<EntityId>,
    pub at_war_with: Vec<KingdomId>,
    
    // Technologies
    pub tech_level: TechLevel,
    pub researched_techs: Vec<TechId>,
    
    // Diplomatie
    pub alliances: Vec<KingdomId>,
    pub trade_partners: Vec<KingdomId>,
    
    // Culture
    pub flag: Flag,
    pub language: Language,
    pub religion: Option<Religion>,
}
```

## Flux de simulation

### Boucle principale (Game Loop)

```rust
/// Boucle de jeu principale
pub async fn game_loop(world: Arc<Mutex<World>>) {
    let mut last_tick = Instant::now();
    let tick_duration = Duration::from_millis(100); // 10 ticks/sec
    
    loop {
        let now = Instant::now();
        if now.duration_since(last_tick) >= tick_duration {
            let mut world = world.lock().unwrap();
            
            if !world.paused {
                // Applique multiplicateur de vitesse
                let ticks = match world.speed {
                    TimeSpeed::Normal => 1,
                    TimeSpeed::Fast2x => 2,
                    TimeSpeed::Fast5x => 5,
                    TimeSpeed::Fast10x => 10,
                };
                
                for _ in 0..ticks {
                    tick_world(&mut world).await;
                }
            }
            
            last_tick = now;
        }
        
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
```

### Tick du monde

```rust
/// Simule 1 tick (= 1 minute de jeu)
pub async fn tick_world(world: &mut World) {
    // 1. Physique basique
    physics::update_positions(&mut world.entities);
    physics::apply_gravity(&mut world.entities);
    
    // 2. Besoins des unités
    for unit in world.entities.units.values_mut() {
        unit.hunger = unit.hunger.saturating_sub(1);
        unit.thirst = unit.thirst.saturating_sub(2);
        unit.energy = unit.energy.saturating_sub(1);
        
        if unit.hunger == 0 {
            unit.health = unit.health.saturating_sub(5); // Famine
        }
        if unit.health == 0 {
            world.entities.kill_unit(unit.id);
        }
    }
    
    // 3. AI des unités
    ai::unit_decisions(world);
    
    // 4. Production (fermes, mines)
    production::tick_buildings(&mut world.entities, &mut world.kingdoms);
    
    // 5. Construction
    construction::tick_construction(&mut world.entities);
    
    // 6. Combat
    combat::resolve_battles(&mut world.entities);
    
    // 7. Diplomatie
    diplomacy::tick_relations(&mut world.kingdoms);
    
    // 8. Technologies
    research::tick_research(&mut world.kingdoms);
    
    // 9. Événements aléatoires
    if rand::random::<f32>() < 0.01 {
        events::trigger_random_event(world);
    }
    
    // 10. Mise à jour de l'historique
    world.tick += 1;
    
    // 11. Sauvegarde auto (toutes les 5 minutes de jeu = 300 ticks)
    if world.tick % 300 == 0 {
        kindmother::auto_save(world).await;
    }
}
```

### Pathfinding (MiyuPathfinding)

```rust
/// Trouve le chemin entre deux points
pub fn find_path(
    world: &World,
    start: Vec2,
    end: Vec2,
    unit: &Unit,
) -> Option<Vec<Vec2>> {
    // A* classique avec heuristique Manhattan
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    
    open_set.push(Node { pos: start, f_score: heuristic(start, end) });
    g_score.insert(start, 0.0);
    
    while let Some(current) = open_set.pop() {
        if current.pos == end {
            return Some(reconstruct_path(came_from, current.pos));
        }
        
        for neighbor in get_neighbors(current.pos, world) {
            if !is_walkable(neighbor, world, unit) {
                continue;
            }
            
            let tentative_g = g_score[&current.pos] + distance(current.pos, neighbor);
            
            if tentative_g < *g_score.get(&neighbor).unwrap_or(&f32::INFINITY) {
                came_from.insert(neighbor, current.pos);
                g_score.insert(neighbor, tentative_g);
                let f = tentative_g + heuristic(neighbor, end);
                open_set.push(Node { pos: neighbor, f_score: f });
            }
        }
    }
    
    None
}
```

## Intégration avec les Cores

### StrongFather — Autorisations

```rust
/// Demande permission pour utiliser un pouvoir
pub async fn request_power_permission(
    power: PowerType,
    target: Target,
    user: UserId,
) -> Result<Permission, Error> {
    let request = PermissionRequest {
        requestor: user,
        action: format!("use_power_{:?}", power),
        context: json!({
            "power": power,
            "target": target,
        }),
    };
    
    strongfather::request_decision(request).await
}
```

### KindMother — Sauvegarde

```rust
/// Sauvegarde le monde via KindMother
pub async fn save_world(world: &World) -> Result<(), Error> {
    // 1. Sérialiser en binaire compressé
    let data = miyu_saveformat::serialize(world)?;
    
    // 2. Compresser (zstd)
    let compressed = compress(&data)?;
    
    // 3. Envoyer à KindMother
    kindmother::store_blob(
        &format!("lifegame/world/{}", world.metadata.id),
        compressed,
    ).await?;
    
    Ok(())
}

/// Charge un monde depuis KindMother
pub async fn load_world(world_id: WorldId) -> Result<World, Error> {
    // 1. Récupérer depuis KindMother
    let compressed = kindmother::retrieve_blob(
        &format!("lifegame/world/{}", world_id)
    ).await?;
    
    // 2. Décompresser
    let data = decompress(&compressed)?;
    
    // 3. Désérialiser
    let world = miyu_saveformat::deserialize(&data)?;
    
    Ok(world)
}
```

### CaringNanny — Observation

```rust
/// Envoie des métriques à CaringNanny
pub async fn report_metrics(world: &World) {
    let metrics = WorldMetrics {
        tick: world.tick,
        total_population: world.entities.units.len(),
        kingdoms: world.kingdoms.len(),
        buildings: world.entities.buildings.len(),
        creatures: world.entities.creatures.len(),
        fps: measure_fps(),
        memory_mb: measure_memory(),
    };
    
    caringnanny::report("miyukini_life_game", metrics).await;
}
```

### WorrySentinel — Limites

```rust
/// Vérifie les limites de sécurité
pub fn check_limits(world: &World, power: PowerType) -> Result<(), Error> {
    match power {
        PowerType::AtomicBomb => {
            let recent_nukes = count_recent_powers(world, PowerType::AtomicBomb, 60);
            if recent_nukes >= 5 {
                return Err(Error::RateLimitExceeded);
            }
        }
        PowerType::SpawnCreature(CreatureType::Crabzilla) => {
            let crabzilla_count = world.entities.count_creatures(CreatureType::Crabzilla);
            if crabzilla_count >= 1 {
                return Err(Error::LimitExceeded("Only 1 Crabzilla allowed"));
            }
        }
        _ => {}
    }
    
    worrysentinel::check_resource_limits(world).await?;
    
    Ok(())
}
```

## Performance et optimisations

### Optimisations prévues

**Spatial Hashing :**
```rust
/// Index spatial pour requêtes rapides
pub struct SpatialHashGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<EntityId>>,
}

impl SpatialHashGrid {
    pub fn insert(&mut self, entity_id: EntityId, pos: Vec2) {
        let cell = self.pos_to_cell(pos);
        self.cells.entry(cell).or_default().push(entity_id);
    }
    
    pub fn query_radius(&self, pos: Vec2, radius: f32) -> Vec<EntityId> {
        let min_cell = self.pos_to_cell(pos - Vec2::splat(radius));
        let max_cell = self.pos_to_cell(pos + Vec2::splat(radius));
        
        let mut results = Vec::new();
        for y in min_cell.1..=max_cell.1 {
            for x in min_cell.0..=max_cell.0 {
                if let Some(entities) = self.cells.get(&(x, y)) {
                    results.extend(entities);
                }
            }
        }
        results
    }
}
```

**Parallélisation avec Rayon :**
```rust
use rayon::prelude::*;

/// Mise à jour parallèle des unités
pub fn parallel_unit_update(units: &mut [Unit]) {
    units.par_iter_mut().for_each(|unit| {
        update_unit_needs(unit);
        update_unit_ai(unit);
    });
}
```

**Chunking du terrain :**
```rust
/// Divise le monde en chunks pour rendu et simulation
pub struct ChunkedWorld {
    chunk_size: u32,
    chunks: HashMap<(i32, i32), Chunk>,
}

pub struct Chunk {
    tiles: Vec<Vec<Tile>>,
    entities: Vec<EntityId>,
    dirty: bool,  // Nécessite re-render
}
```

### Métriques cibles

| Métrique | Objectif MVP | Objectif v1.0 |
|----------|--------------|---------------|
| **FPS** | 60 (500 entités) | 60 (2000 entités) |
| **Taille monde** | 512×512 | 1024×1024 |
| **Sauvegarde** | <2s | <1s |
| **Chargement** | <5s | <3s |
| **Mémoire** | <300 MB | <500 MB |
| **Tick time** | <16ms | <10ms |

## Format de sauvegarde

### Structure fichier `.lifegame`

```
MiyukiniLifeGame Save File v1.0
═══════════════════════════════
[HEADER]
  Magic: 0x4D4C4746 ('MLFG')
  Version: 1
  Size: 12,345,678 bytes
  Compressed: true (zstd level 3)
  Checksum: SHA-256

[METADATA] (JSON)
  {
    "world_id": "uuid-1234",
    "name": "Mon Monde",
    "created_at": "2026-02-12T10:30:00Z",
    "last_played": "2026-02-12T12:45:00Z",
    "playtime": "2h 15m",
    "version": "0.1.0"
  }

[TERRAIN] (Binary)
  Width: 512
  Height: 512
  Tiles: [262,144 bytes]

[ENTITIES] (MessagePack)
  Units: [...]
  Buildings: [...]
  Creatures: [...]

[KINGDOMS] (MessagePack)
  [...]

[HISTORY] (MessagePack)
  [...]
```

### Compression

- **Algorithme :** Zstd niveau 3
- **Ratio attendu :** 5:1
- **Monde moyen (512×512) :**
  - Non compressé : ~50 MB
  - Compressé : ~10 MB

## Conclusion

L'architecture de Miyukini Life Game suit strictement la **pyramide Miyukini COG** :
- Service UI en Strate 7
- Opérateurs métier en Strate 7
- Toolkits réutilisables en Strate 6
- Cores gouvernent en Strate 4

Chaque composant a une responsabilité claire. La simulation est optimisée avec spatial hashing et parallélisation. La sauvegarde est gérée par KindMother avec compression efficace.

**Phase suivante :** Lire le Guide d'Implémentation MVP pour le plan d'action concret.
