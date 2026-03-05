---
name: miyukini-mge
description: Miyukini Game Engine (MGE) - moteur de jeu modulaire tick-based en Rust. Architecture ECS (Entity-Component-System), systeme de plugins, 113 crates (kernel, core, plugins, genre packs), scheduler par phases, evenements double-buffer, RNG deterministe, profiling integre. 16 genre packs (RPG, Idle, Factory, RTS, Sandbox, etc.). Utiliser quand on travaille dans mge/, quand on cree un composant/systeme/plugin de jeu, quand on implemente une mecanique de gameplay, quand on ajoute un genre pack, ou quand on travaille sur Allumina ou LordOfTheCastle.
---

# Miyukini Game Engine (MGE)

Moteur de jeu modulaire, tick-based, 100% Rust, zero unsafe. Workspace independant dans `mge/`.

## Architecture globale

```
mge/
├── crates/
│   ├── mge-core/          # Engine, Plugin, Scheduler, Context
│   ├── mge-ecs/           # World, EntityId, Component, Storage
│   ├── mge-event/         # EventQueue double-buffer
│   ├── mge-time/          # Time tick-based
│   ├── mge-rng/           # RNG deterministe (SmallRng)
│   ├── mge-profiler/      # Metriques par phase/systeme
│   ├── mge-plugin-spatial/    # Position2D, Velocity2D, SpatialHash
│   ├── mge-plugin-render-2d/  # Sprite, Camera2D, RenderLayer
│   ├── mge-plugin-input/      # Gestion d'entree
│   ├── mge-plugin-audio/      # Audio
│   ├── mge-plugin-basic-physics/  # Physique basique
│   ├── mge-plugin-save-load/     # Sauvegarde/chargement
│   ├── rpg/               # 8 crates (stats, combat, inventory, quest...)
│   ├── idle/              # 5 crates (producer, upgrade, multiplier...)
│   ├── factory/           # 4 crates (machine, recipe, conveyor...)
│   ├── grand-strategy/    # 11 crates (diplomacy, economy, military...)
│   ├── sandbox/           # 8 crates
│   ├── rts/               # 6 crates
│   └── ... (16 genres total, 100+ crates)
└── examples/              # 5 demos
```

**Total** : 113 crates workspace | Edition 2021 | `unsafe_code = "forbid"`

---

## Engine Core (mge-core)

### Engine

```rust
pub struct Engine {
    world: World,
    scheduler: Scheduler,
    events: EventQueue,
    time: Time,
    rng: Rng,
    config: EngineConfig,
    plugins: Vec<Box<dyn Plugin>>,
    built: bool,
}

pub struct EngineConfig {
    pub seed: u64,                      // Graine RNG globale
    pub headless: bool,                 // Sans fenetre (serveur)
    pub fixed_timestep_ms: Option<u32>, // Delta fixe (16 = 60 ticks/s)
    pub tick_budget_ms: Option<u32>,    // Budget CPU par tick
}

impl Engine {
    pub fn new(config: EngineConfig) -> Self
    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P)
    pub fn build(&mut self)  // Appel unique, initialise tous les plugins
    pub fn tick(&mut self, delta_secs: f32)  // Avance la simulation
    pub fn register_component<T: Component>(&mut self)
    pub fn add_system<F>(&mut self, phase: PhaseId, system: F)
        where F: FnMut(&mut World, &mut Context) + Send + 'static
    pub fn add_named_system<F>(&mut self, phase: PhaseId, name: impl Into<String>, system: F)
        where F: FnMut(&mut World, &mut Context) + Send + 'static
    pub fn world(&self) -> &World
    pub fn world_mut(&mut self) -> &mut World
    pub fn emit<E: Event>(&mut self, event: E)
    pub fn set_seed(&mut self, seed: u64)
    pub fn time(&self) -> &Time
    pub fn last_tick_metrics(&self) -> Option<&TickMetrics>
}
```

### Plugin trait

```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn build(&self, engine: &mut Engine);
    fn dependencies(&self) -> &[&str] { &[] }
}
```

### Context (passe a chaque systeme)

```rust
pub struct Context<'a> {
    pub time: &'a Time,
    pub rng: &'a mut Rng,
    pub events: &'a mut EventQueue,
}

impl Context<'_> {
    pub fn emit<E: Event>(&mut self, event: E)
    pub fn delta_secs(&self) -> f32
    pub fn tick_count(&self) -> u64
}
```

### Scheduler

Les systemes s'executent par `PhaseId` croissant (u32). Ordre deterministe.

```rust
pub struct PhaseId(pub u32);
// Phase 0-99: init/setup
// Phase 100-199: input/logic
// Phase 200-299: combat/interaction
// Phase 300+: render/cleanup
```

---

## ECS (mge-ecs)

### EntityId — Index generationnel

```rust
pub struct EntityId { index: u32, generation: u32 }

impl EntityId {
    pub fn index(&self) -> u32
    pub fn generation(&self) -> u32
    pub fn to_bits(&self) -> u64  // Pour derivation RNG
}
```

### Component — Trait marqueur

```rust
pub trait Component: Send + Sync + 'static {}
```

Tous les composants sont des donnees pures. Pas de logique.

### World — Stockage sparse-set

```rust
impl World {
    // Entites
    pub fn spawn(&mut self) -> EntityId
    pub fn despawn(&mut self, id: EntityId)
    pub fn is_alive(&self, id: EntityId) -> bool
    pub fn entity_count(&self) -> u32

    // Composants
    pub fn register_component<T: Component>(&mut self)
    pub fn insert<T: Component>(&mut self, id: EntityId, component: T)
    pub fn get<T: Component>(&self, id: EntityId) -> Option<&T>
    pub fn get_mut<T: Component>(&mut self, id: EntityId) -> Option<&mut T>
    pub fn remove_component<T: Component>(&mut self, id: EntityId)
    pub fn has_component<T: Component>(&self, id: EntityId) -> bool

    // Iteration lecture
    pub fn iter1<A: Component>(&self) -> impl Iterator<Item = (EntityId, &A)>
    pub fn iter2<A, B>(&self) -> impl Iterator<Item = (EntityId, &A, &B)>
    pub fn iter3<A, B, C>(&self) -> impl Iterator<Item = (EntityId, &A, &B, &C)>

    // Iteration mutable
    pub fn for_each1_mut<A, F>(&mut self, f: F) where F: FnMut(EntityId, &mut A)
    pub fn for_each_mut<A, B, F>(&mut self, f: F) where F: FnMut(EntityId, &mut A, &mut B)
}
```

---

## Evenements (mge-event) — Double buffer

```rust
pub trait Event: Send + Sync + 'static {}

pub struct EventQueue { write_buffer, read_buffer }

impl EventQueue {
    pub fn emit<E: Event>(&mut self, event: E)  // Ecrit tick N
    pub fn iter<E: Event>(&self) -> EventIter<E> // Lit tick N+1
    pub fn has<E: Event>(&self) -> bool
    pub fn count<E: Event>(&self) -> usize
    pub fn swap(&mut self)  // Appele au debut de chaque tick
}
```

**Regle** : Un evenement emis au tick N est lisible uniquement au tick N+1.

---

## Temps (mge-time)

```rust
pub struct Time {
    pub delta_secs: f32,   // Temps depuis dernier tick
    pub tick_count: u64,   // Compteur monotone
    pub time_scale: f32,   // Multiplicateur (0.5 = lent, 2.0 = rapide)
    pub paused: bool,      // Si true, delta_secs = 0
}
```

---

## RNG deterministe (mge-rng)

```rust
pub struct Rng { inner: SmallRng, global_seed: u64 }

impl Rng {
    pub fn new(seed: u64) -> Self
    pub fn seed(&mut self, seed: u64)
    pub fn derive_for_bits(&self, bits: u64) -> SmallRng  // RNG par entite
    pub fn f32(&mut self) -> f32          // [0.0, 1.0)
    pub fn f32_range(&mut self, min: f32, max: f32) -> f32
    pub fn u32(&mut self) -> u32
    pub fn u64(&mut self) -> u64
}
```

**Pattern** : `ctx.rng.derive_for_bits(entity_id.to_bits())` pour RNG unique par entite.

---

## Profiling (mge-profiler)

```rust
pub struct TickMetrics {
    pub total: Duration,
    pub phases: Vec<PhaseMetrics>,
    pub budget_exceeded: bool,
}

pub struct PhaseMetrics {
    pub phase: PhaseId,
    pub duration: Duration,
    pub systems: Vec<SystemMetrics>,
}

pub struct SystemMetrics {
    pub name: Option<String>,
    pub duration: Duration,
}
```

Acces via `engine.last_tick_metrics()`.

---

## Creer un plugin

```rust
pub struct MonPlugin;

impl Plugin for MonPlugin {
    fn name(&self) -> &str { "mon_plugin" }

    fn build(&self, engine: &mut Engine) {
        // 1. Enregistrer les composants
        engine.register_component::<MonComposant>();

        // 2. Ajouter les systemes par phase
        engine.add_named_system(PhaseId(100), "update_logic", |world, ctx| {
            for (id, comp) in world.iter1::<MonComposant>() {
                // Logique
            }
        });

        engine.add_named_system(PhaseId(200), "process_events", |world, ctx| {
            for event in ctx.events.iter::<MonEvent>() {
                // Reagir
            }
        });
    }
}
```

---

## Plugins spatiaux standard

```rust
// mge-plugin-spatial
pub struct Position2D { pub x: f32, pub y: f32 }
pub struct Velocity2D { pub x: f32, pub y: f32 }
pub struct Rotation { pub angle: f32 }
pub struct SpatialHash { pub cell_id: u64 }

// mge-plugin-render-2d
pub struct Sprite { pub texture_id: u32, pub width: f32, pub height: f32 }
pub struct Camera2D { pub zoom: f32, pub offset_x: f32, pub offset_y: f32 }
pub struct RenderLayer { pub layer: u32 }
```

---

## Genre Pack RPG — Exemple complet

### Stats (mge-rpg-stats)

```rust
pub const STAT_COUNT: usize = 16;
pub type StatId = u8;
pub struct StatBlock { pub values: [f64; STAT_COUNT] }
pub struct Health { pub current: f64, pub max: f64, pub regen_rate: f64 }
pub struct DeadTag;  // Marqueur d'entite morte

// Evenements
pub struct StatChangedEvent { pub entity: EntityId, pub stat_id: StatId, pub old_value: f64, pub new_value: f64 }
pub struct EntityDeathEvent { pub entity: EntityId }
```

### Combat (mge-rpg-combat)

```rust
pub struct CombatStats {
    pub atk: f64, pub esq: f64, pub par: f64,
    pub atk_speed: f64, pub damage_base: f64, pub damage_type: u8,
}
pub struct Armor { pub ar: [f64; 3], pub resistance: f64 }
pub struct AttackCooldown { pub remaining: f32, pub interval: f32 }

// Constantes
pub const DAMAGE_TYPE_SLASH: u8 = 0;
pub const DAMAGE_TYPE_BLUNT: u8 = 1;
pub const DAMAGE_TYPE_PIERCE: u8 = 2;

// Evenements
pub struct AttackRequestEvent { pub attacker: EntityId, pub target: EntityId }
pub struct DamageEvent { pub target: EntityId, pub amount: f64, pub crit: bool }
pub struct AttackMissEvent { pub attacker: EntityId, pub target: EntityId }

// Systeme de combat (PhaseId(200))
// Resolution: hit check (atk vs esq) -> crit (5%) -> parry (par vs atk)
// -> armure (ar%) -> degats -> mort si health <= 0
```

---

## Boucle de jeu standard

```rust
let config = EngineConfig {
    seed: 42,
    headless: false,
    fixed_timestep_ms: Some(16),
    tick_budget_ms: Some(8),
};
let mut engine = Engine::new(config);

engine.add_plugin(SpatialPlugin);
engine.add_plugin(CombatPlugin);
engine.build();  // Une seule fois

loop {
    let delta = frame_time.as_secs_f32();
    engine.tick(delta);

    if let Some(metrics) = engine.last_tick_metrics() {
        if metrics.budget_exceeded {
            // Alerte performance
        }
    }
}
```

---

## Metadata Cargo.toml

Chaque crate MGE inclut des metadonnees MSCM :

```toml
[package.metadata]
"@id" = "mge.rpg.combat.v1"
"@role" = "simulation"
"@layer" = "plugin"
"@domain" = "rpg"
"@human" = "Description en francais"
"@do" = "Verbe d'action"
```

---

## Genres disponibles (16)

| Genre | Crates | Etat |
|-------|--------|------|
| RPG | 8 (stats, combat, inventory, quest, progression, dialogue, AI) | Combat impl |
| Idle | 5 (producer, upgrade, multiplier, offline, prestige) | Scaffold |
| Factory | 4 (machine, recipe, conveyor, logistics) | Scaffold |
| Grand Strategy | 11 (diplomacy, economy, trade, military...) | Scaffold |
| Sandbox | 8 (agent, world, crafting, building...) | Scaffold |
| RTS | 6 (selection, production, resource, unit-AI, fog) | Scaffold |
| Massive Battle | 6 (formation, unit, morale, tactics, supply) | Scaffold |
| Social | 7 (relationship, faction, reputation, need...) | Scaffold |
| Shooter | 5 (weapon, aim, ammo, target, health) | Scaffold |
| Puzzle | 4 | Scaffold |
| Platformer | 5 | Scaffold |
| Racing | 5 | Scaffold |
| Roguelike | 6 | Scaffold |
| TCG | 5 | Scaffold |
| Tycoon | 5 | Scaffold |
| Visual Novel | 4 | Scaffold |
