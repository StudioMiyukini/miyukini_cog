<!-- @id: SD-Impl-01 @do: guide @role: back-end @layer: 3 @human: miyuk -->

# IMPL-01 -- Guide de Creation du Workspace Sodomight

**Auteur :** Francois (Dev Back-End, Miyukini AI Studio)
**Base :** SD-Tech-Architecture.md (Denis)
**Date :** 2026-02-28
**Statut :** Guide d'implementation -- v1.0

---

## Table des matieres

1. [Strategie de workspace](#1-strategie-de-workspace)
2. [Workspace Cargo.toml racine](#2-workspace-cargotoml-racine)
3. [Couche Kernel -- 5 crates](#3-couche-kernel--5-crates)
4. [Couche Engine -- 9 crates](#4-couche-engine--9-crates)
5. [Couche Pack ARPG -- 10 crates](#5-couche-pack-arpg--10-crates)
6. [Couche Game -- 3 crates](#6-couche-game--3-crates)
7. [Outils -- 6 crates](#7-outils--6-crates)
8. [Graphe de dependances complet](#8-graphe-de-dependances-complet)
9. [Ordre de compilation recommande](#9-ordre-de-compilation-recommande)
10. [Feature flags](#10-feature-flags)
11. [Regles obligatoires par Cargo.toml](#11-regles-obligatoires-par-cargotoml)
12. [Commandes de verification](#12-commandes-de-verification)

---

## 1. Strategie de workspace

Le workspace Sodomight est **independant** du workspace principal Miyukini-COG.
Le commentaire dans le Cargo.toml racine de COG le confirme :

```
# === MGE (Miyukini Game Engine) ===
# Workspace MGE independant : mge/ (voir mge/Cargo.toml)
```

Le workspace MGE vit sous `mge/` avec son propre `Cargo.toml` racine.
Sodomight est le premier jeu construit sur le moteur MGE. Les crates sont
organisees en 4 couches verticales strictes :

```
Couche 4 (Game)      : sodomight-game, sodomight-server, sodomight-client
Couche 3 (Pack ARPG) : mge-arpg-*  (10 crates)
Couche 2 (Engine)    : mge-render, mge-audio, mge-ui, ... (9 crates)
Couche 1 (Kernel)    : mge-core, mge-ecs, mge-math, mge-asset, mge-platform
```

**Regle d'or :** Aucune couche haute ne peut etre importee par une couche basse.
Une crate de Couche 1 ne peut jamais dependre d'une crate de Couche 2, 3 ou 4.

---

## 2. Workspace Cargo.toml racine

Le fichier `mge/Cargo.toml` est le point d'entree du workspace complet.

```toml
[workspace]
resolver = "2"
members = [
    # === Couche 1 : Kernel (5 crates) ===
    "crates/kernel/mge-core",
    "crates/kernel/mge-ecs",
    "crates/kernel/mge-math",
    "crates/kernel/mge-asset",
    "crates/kernel/mge-platform",

    # === Couche 2 : Engine (9 crates) ===
    "crates/engine/mge-render",
    "crates/engine/mge-audio",
    "crates/engine/mge-ui",
    "crates/engine/mge-pathfinding",
    "crates/engine/mge-collision",
    "crates/engine/mge-collision-rich",
    "crates/engine/mge-script",
    "crates/engine/mge-net",
    "crates/engine/mge-save",

    # === Couche 3 : Pack ARPG (10 crates) ===
    "crates/arpg/mge-arpg-world",
    "crates/arpg/mge-arpg-entity",
    "crates/arpg/mge-arpg-combat",
    "crates/arpg/mge-arpg-items",
    "crates/arpg/mge-arpg-stats",
    "crates/arpg/mge-arpg-skills",
    "crates/arpg/mge-arpg-loot",
    "crates/arpg/mge-arpg-ai",
    "crates/arpg/mge-arpg-quest",
    "crates/arpg/mge-arpg-trade",

    # === Couche 4 : Game (3 crates) ===
    "games/sodomight",
    "games/sodomight-server",
    "games/sodomight-client",

    # === Outils (6 crates) ===
    "tools/mge-studio",
    "tools/mge-packer",
    "tools/mge-slicer",
    "tools/mge-rescale",
    "tools/mge-mirror",
    "tools/mge-remap",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["Miyukini AI Studio"]
license = "MIT OR Apache-2.0"

[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
doc_markdown = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
must_use_candidate = "allow"
return_self_not_must_use = "allow"
cast_possible_truncation = "allow"
cast_sign_loss = "allow"
cast_precision_loss = "allow"
cast_lossless = "allow"
module_name_repetitions = "allow"
similar_names = "allow"

[workspace.dependencies]
# Serialisation
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
bincode = "1.3"

# Identifiants et temps
uuid = { version = "1.0", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }

# Erreurs
thiserror = "2.0"

# Random
rand = "0.8"
rand_chacha = "0.3"

# Graphique
wgpu = "24.0"
winit = "0.30"
image = { version = "0.25", default-features = false, features = ["png"] }

# Audio
kira = "0.9"

# Hot-reload
notify = "7.0"

# Scripting
rhai = "1.20"

# Reseau
tokio = { version = "1.0", features = ["rt-multi-thread", "net", "sync", "macros"] }

# Base de donnees
rusqlite = { version = "0.32", features = ["bundled"] }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Pathfinding (interne, pas de dep externe)
# Collision (interne, pas de dep externe)
```

---

## 3. Couche Kernel -- 5 crates

### 3.1 mge-core

**Chemin :** `crates/kernel/mge-core/`
**Role :** Game loop, time management, event bus, system scheduler, tick rate.

```toml
# crates/kernel/mge-core/Cargo.toml
[package]
name = "mge-core"
version.workspace = true
edition.workspace = true
description = "MGE kernel: game loop, time, events, system scheduler"

[dependencies]
tracing = { workspace = true }

[dev-dependencies]
# Pas de dependance de test externe

[lints]
workspace = true
```

**Structure de fichiers :**

```
crates/kernel/mge-core/src/
    lib.rs          # Re-exports publics
    game_loop.rs    # Boucle de jeu principale, accumulateur FixedUpdate
    time.rs         # DeltaTime, FixedTime, Instant wrapper
    event.rs        # Event bus generique type-erased
    stage.rs        # Stages PreUpdate/FixedUpdate/Update/PostUpdate/Network/Render/Audio
    scheduler.rs    # System registration et execution sequentielle par stage
    errors.rs       # GameError (thiserror)
```

**Dependances internes :** Aucune (crate racine).

---

### 3.2 mge-ecs

**Chemin :** `crates/kernel/mge-ecs/`
**Role :** ECS archetype maison -- World, Archetype, Query, System, SparseMap.

```toml
# crates/kernel/mge-ecs/Cargo.toml
[package]
name = "mge-ecs"
version.workspace = true
edition.workspace = true
description = "MGE ECS: archetype storage, queries, sparse overlay"

[dependencies]
# Zero dependance externe -- pure Rust ECS

[dev-dependencies]
# Aucune

[lints]
workspace = true
```

**Structure de fichiers :**

```
crates/kernel/mge-ecs/src/
    lib.rs              # Re-exports: World, EntityId, Query, etc.
    world.rs            # World: registre d'archetypes, spawn/despawn, resources
    entity.rs           # EntityId (index + generation), EntityLocation
    archetype.rs        # Archetype struct, SoA columns, migration
    component.rs        # Component trait, TypeId registre, ComponentColumn
    query.rs            # Query<(&A, &mut B)>, With<T>, Without<T>, Changed<T>
    system.rs           # System trait, SystemFn wrapper, stage bindings
    resource.rs         # Typed global resources (Res<T>, ResMut<T>)
    sparse_set.rs       # SparseSet<EntityId, T> pour overlay ephemere
    commands.rs         # Commands: spawn/despawn deferred
    event.rs            # EventWriter<T>, EventReader<T> pour communication inter-systemes
    errors.rs           # EcsError
```

**Dependances internes :** Aucune.

---

### 3.3 mge-math

**Chemin :** `crates/kernel/mge-math/`
**Role :** Types mathematiques, projection dimetric isometrique, interpolation.

```toml
# crates/kernel/mge-math/Cargo.toml
[package]
name = "mge-math"
version.workspace = true
edition.workspace = true
description = "MGE math: Vec2, IVec2, Rect, dimetric projection, lerp"

[dependencies]
serde = { workspace = true }

[dev-dependencies]
# Aucune

[lints]
workspace = true
```

**Structure de fichiers :**

```
crates/kernel/mge-math/src/
    lib.rs          # Re-exports
    vec2.rs         # Vec2 (f32, f32), operations, normalize, dot, distance
    ivec2.rs        # IVec2 (i32, i32), conversions
    rect.rs         # Rect { x, y, w, h }, intersection, contains
    iso.rs          # world_to_screen, screen_to_world, projection dimetric 2:1
    lerp.rs         # lerp, inverse_lerp, smoothstep
    color.rs        # Color { r, g, b, a } en f32 [0..1]
```

**Dependances internes :** Aucune.

---

### 3.4 mge-asset

**Chemin :** `crates/kernel/mge-asset/`
**Role :** Asset registry, hot-reload, loaders PNG/TOML/Aseprite/LDtk.

```toml
# crates/kernel/mge-asset/Cargo.toml
[package]
name = "mge-asset"
version.workspace = true
edition.workspace = true
description = "MGE asset registry: loading, caching, hot-reload via notify"

[dependencies]
mge-math = { path = "../mge-math" }
serde = { workspace = true }
toml = { workspace = true }
image = { workspace = true }
notify = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
tempfile = "3.14"

[lints]
workspace = true
```

**Structure de fichiers :**

```
crates/kernel/mge-asset/src/
    lib.rs              # Re-exports
    registry.rs         # AssetRegistry: HashMap<AssetId, AssetSlot>
    asset_id.rs         # AssetId (String symbolique, pas de chemin)
    loader.rs           # Trait AssetLoader, dispatch par extension
    loader_png.rs       # PNG loader -> TextureData
    loader_toml.rs      # TOML loader -> serde_json::Value ou struct typed
    loader_aseprite.rs  # Aseprite JSON metadata loader
    loader_ldtk.rs      # LDtk project loader (zones/tilemaps)
    hot_reload.rs       # Watcher notify, file changed -> reload callback
    texture.rs          # TextureData { width, height, rgba_pixels }
    errors.rs           # AssetError
```

**Dependances internes :** `mge-math`.

---

### 3.5 mge-platform

**Chemin :** `crates/kernel/mge-platform/`
**Role :** Fenetre winit, device wgpu, input mapping, fullscreen, cursor.

```toml
# crates/kernel/mge-platform/Cargo.toml
[package]
name = "mge-platform"
version.workspace = true
edition.workspace = true
description = "MGE platform: window, GPU init, input mapping"

[dependencies]
mge-math = { path = "../mge-math" }
wgpu = { workspace = true }
winit = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }
pollster = "0.4"

[dev-dependencies]
# Aucune (tests platform necessitent GPU)

[lints]
workspace = true
```

**Structure de fichiers :**

```
crates/kernel/mge-platform/src/
    lib.rs          # Re-exports
    window.rs       # Creation fenetre winit, fullscreen toggle
    gpu.rs          # GpuContext: Instance, Adapter, Device, Queue, Surface
    input.rs        # InputState: keyboard, mouse, gamepad
    keybind.rs      # Mapping input -> action symbolique
    cursor.rs       # Cursor mode (visible, hidden, custom sprite)
    errors.rs       # PlatformError, RenderError
```

**Dependances internes :** `mge-math`.

---

## 4. Couche Engine -- 9 crates

### 4.1 mge-render

**Chemin :** `crates/engine/mge-render/`
**Role :** Renderer wgpu -- sprite batching, tilemap iso, dual-res, z-order, post-process.

```toml
# crates/engine/mge-render/Cargo.toml
[package]
name = "mge-render"
version.workspace = true
edition.workspace = true
description = "MGE renderer: wgpu sprite batching, iso tilemap, dual-res"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-math = { path = "../../kernel/mge-math" }
mge-platform = { path = "../../kernel/mge-platform" }
mge-asset = { path = "../../kernel/mge-asset" }
wgpu = { workspace = true }
image = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

**Structure de fichiers :**

```
crates/engine/mge-render/src/
    lib.rs              # Re-exports
    renderer.rs         # Renderer: state, frame loop, render passes
    sprite_batch.rs     # SpriteBatcher: instance buffer, atlas grouping
    sprite_instance.rs  # SpriteInstance: GPU vertex/instance data
    tilemap_render.rs   # TilemapRenderer: iso grid, chunks, culling
    z_order.rs          # Z-order sorting: layer + iso depth + sub_order
    camera.rs           # Camera2D: follow, smooth, bounds, screen_to_world
    animation.rs        # AnimationSystem: frame advance, sprite lookup
    particle.rs         # ParticleSystem: emitter, update, render
    dual_res.rs         # DualResolution: offscreen render + upscale
    atlas.rs            # TextureAtlas: UV regions, atlas management
    pipeline.rs         # create_sprite_pipeline (wgpu RenderPipeline)
    shaders/
        sprite.wgsl     # Vertex + Fragment shader WGSL
        upscale.wgsl    # Upscale shader pour dual-res
    errors.rs           # RenderError
```

---

### 4.2 mge-audio

**Chemin :** `crates/engine/mge-audio/`
**Role :** Wrapper kira -- SFX, ambient, musique adaptative, spatial audio.

```toml
# crates/engine/mge-audio/Cargo.toml
[package]
name = "mge-audio"
version.workspace = true
edition.workspace = true
description = "MGE audio: kira wrapper, SFX, music, spatial"

[dependencies]
mge-asset = { path = "../../kernel/mge-asset" }
kira = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 4.3 mge-ui

**Chemin :** `crates/engine/mge-ui/`
**Role :** HUD in-game -- orbes vie/mana, belt, paperdoll, inventaire grille, arbres skills.

```toml
# crates/engine/mge-ui/Cargo.toml
[package]
name = "mge-ui"
version.workspace = true
edition.workspace = true
description = "MGE in-game UI: HUD, inventory, skill tree, tooltips"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-math = { path = "../../kernel/mge-math" }
mge-render = { path = "../mge-render" }
serde = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 4.4 mge-pathfinding

**Chemin :** `crates/engine/mge-pathfinding/`
**Role :** A* tile-based isometrique, heuristique octile, pathfinding budgete.

```toml
# crates/engine/mge-pathfinding/Cargo.toml
[package]
name = "mge-pathfinding"
version.workspace = true
edition.workspace = true
description = "MGE pathfinding: A* tile-based, octile heuristic, budgeted"

[dependencies]
mge-math = { path = "../../kernel/mge-math" }
mge-collision = { path = "../mge-collision" }

[lints]
workspace = true
```

---

### 4.5 mge-collision

**Chemin :** `crates/engine/mge-collision/`
**Role :** Tile walkable/non-walkable + hitboxes circulaires (Phase 1 Sodomight).

```toml
# crates/engine/mge-collision/Cargo.toml
[package]
name = "mge-collision"
version.workspace = true
edition.workspace = true
description = "MGE collision: tile walkability grid, circle hitboxes"

[dependencies]
mge-math = { path = "../../kernel/mge-math" }

[lints]
workspace = true
```

---

### 4.6 mge-collision-rich

**Chemin :** `crates/engine/mge-collision-rich/`
**Role :** Collision riche pour Phase 2 Allumina -- polygones, capsules, AABB.

```toml
# crates/engine/mge-collision-rich/Cargo.toml
[package]
name = "mge-collision-rich"
version.workspace = true
edition.workspace = true
description = "MGE collision rich: polygons, capsules, AABB (Phase 2)"

[dependencies]
mge-math = { path = "../../kernel/mge-math" }
mge-collision = { path = "../mge-collision" }

[lints]
workspace = true
```

---

### 4.7 mge-script

**Chemin :** `crates/engine/mge-script/`
**Role :** Wrapper Rhai -- scripting quetes, triggers, dialogues NPC, API safe.

```toml
# crates/engine/mge-script/Cargo.toml
[package]
name = "mge-script"
version.workspace = true
edition.workspace = true
description = "MGE scripting: Rhai wrapper, quest triggers, NPC dialogue"

[dependencies]
rhai = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 4.8 mge-net

**Chemin :** `crates/engine/mge-net/`
**Role :** Networking -- listen server TCP, messages client/server, serialisation bincode.

```toml
# crates/engine/mge-net/Cargo.toml
[package]
name = "mge-net"
version.workspace = true
edition.workspace = true
description = "MGE networking: listen server TCP, bincode messages"

[dependencies]
mge-core = { path = "../../kernel/mge-core" }
serde = { workspace = true }
bincode = { workspace = true }
tokio = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 4.9 mge-save

**Chemin :** `crates/engine/mge-save/`
**Role :** KindMother save/load, format personnage, SQLite governe.

```toml
# crates/engine/mge-save/Cargo.toml
[package]
name = "mge-save"
version.workspace = true
edition.workspace = true
description = "MGE persistence: KindMother SQLite save/load"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
serde = { workspace = true }
serde_json = { workspace = true }
rusqlite = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
tracing = { workspace = true }
thiserror = { workspace = true }

[features]
default = ["kindmother-only"]
legacy-sqlite = []
kindmother-only = []

[lints]
workspace = true
```

---

## 5. Couche Pack ARPG -- 10 crates

### 5.1 mge-arpg-entity

**Chemin :** `crates/arpg/mge-arpg-entity/`
**Role :** Archetypes Character, Monster, Item, Projectile, Effect, Shrine.

```toml
# crates/arpg/mge-arpg-entity/Cargo.toml
[package]
name = "mge-arpg-entity"
version.workspace = true
edition.workspace = true
description = "MGE ARPG entity archetypes: character, monster, item, projectile"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-math = { path = "../../kernel/mge-math" }
serde = { workspace = true }

[lints]
workspace = true
```

---

### 5.2 mge-arpg-stats

**Chemin :** `crates/arpg/mge-arpg-stats/`
**Role :** Attributs, resistances, breakpoints FCR/FHR/FBR/IAS, formules derivees.

```toml
# crates/arpg/mge-arpg-stats/Cargo.toml
[package]
name = "mge-arpg-stats"
version.workspace = true
edition.workspace = true
description = "MGE ARPG stats: attributes, resistances, breakpoints, derived formulas"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-math = { path = "../../kernel/mge-math" }
serde = { workspace = true }
toml = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 5.3 mge-arpg-skills

**Chemin :** `crates/arpg/mge-arpg-skills/`
**Role :** Arbres de competences, synergies, cooldowns, charges, hotkeys.

```toml
# crates/arpg/mge-arpg-skills/Cargo.toml
[package]
name = "mge-arpg-skills"
version.workspace = true
edition.workspace = true
description = "MGE ARPG skills: skill trees, synergies, cooldowns, activation"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-arpg-stats = { path = "../mge-arpg-stats" }
serde = { workspace = true }
toml = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 5.4 mge-arpg-combat

**Chemin :** `crates/arpg/mge-arpg-combat/`
**Role :** Pipeline attack, skills, projectiles, AoE, formules D2 (CTH, damage).

```toml
# crates/arpg/mge-arpg-combat/Cargo.toml
[package]
name = "mge-arpg-combat"
version.workspace = true
edition.workspace = true
description = "MGE ARPG combat: attack pipeline, CTH, damage, projectiles, buffs"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-math = { path = "../../kernel/mge-math" }
mge-arpg-stats = { path = "../mge-arpg-stats" }
mge-arpg-entity = { path = "../mge-arpg-entity" }
mge-arpg-skills = { path = "../mge-arpg-skills" }
serde = { workspace = true }
rand = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 5.5 mge-arpg-items

**Chemin :** `crates/arpg/mge-arpg-items/`
**Role :** Affixes, sockets, runewords, set items, unique items, generation loot, cube.

```toml
# crates/arpg/mge-arpg-items/Cargo.toml
[package]
name = "mge-arpg-items"
version.workspace = true
edition.workspace = true
description = "MGE ARPG items: affixes, sockets, runewords, sets, uniques, cube"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-arpg-stats = { path = "../mge-arpg-stats" }
mge-arpg-entity = { path = "../mge-arpg-entity" }
serde = { workspace = true }
serde_json = { workspace = true }
toml = { workspace = true }
rand = { workspace = true }
uuid = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 5.6 mge-arpg-loot

**Chemin :** `crates/arpg/mge-arpg-loot/`
**Role :** Drop tables, treasure classes, NoDrop, Magic Find, distribution monde.

```toml
# crates/arpg/mge-arpg-loot/Cargo.toml
[package]
name = "mge-arpg-loot"
version.workspace = true
edition.workspace = true
description = "MGE ARPG loot: drop tables, treasure classes, MF, NoDrop"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-arpg-items = { path = "../mge-arpg-items" }
mge-arpg-stats = { path = "../mge-arpg-stats" }
mge-arpg-entity = { path = "../mge-arpg-entity" }
serde = { workspace = true }
toml = { workspace = true }
rand = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 5.7 mge-arpg-ai

**Chemin :** `crates/arpg/mge-arpg-ai/`
**Role :** Behaviour trees Rust, 43 archetypes IA, aggro, leash, state machines.

```toml
# crates/arpg/mge-arpg-ai/Cargo.toml
[package]
name = "mge-arpg-ai"
version.workspace = true
edition.workspace = true
description = "MGE ARPG AI: behaviour trees, 43 archetypes, aggro, leash"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-math = { path = "../../kernel/mge-math" }
mge-arpg-entity = { path = "../mge-arpg-entity" }
mge-arpg-combat = { path = "../mge-arpg-combat" }
mge-pathfinding = { path = "../../engine/mge-pathfinding" }
serde = { workspace = true }
toml = { workspace = true }
rand = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 5.8 mge-arpg-world

**Chemin :** `crates/arpg/mge-arpg-world/`
**Role :** Zones, tiles, chunks, waypoints, portails, generation procedurale DOC.

```toml
# crates/arpg/mge-arpg-world/Cargo.toml
[package]
name = "mge-arpg-world"
version.workspace = true
edition.workspace = true
description = "MGE ARPG world: zones, tiles, chunks, waypoints, portals"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-math = { path = "../../kernel/mge-math" }
mge-asset = { path = "../../kernel/mge-asset" }
mge-collision = { path = "../../engine/mge-collision" }
serde = { workspace = true }
toml = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 5.9 mge-arpg-quest

**Chemin :** `crates/arpg/mge-arpg-quest/`
**Role :** Systeme de quetes, triggers Rhai, flags, recompenses, NPC dialogues.

```toml
# crates/arpg/mge-arpg-quest/Cargo.toml
[package]
name = "mge-arpg-quest"
version.workspace = true
edition.workspace = true
description = "MGE ARPG quests: triggers, flags, rewards, NPC dialogue"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-script = { path = "../../engine/mge-script" }
mge-arpg-entity = { path = "../mge-arpg-entity" }
serde = { workspace = true }
toml = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 5.10 mge-arpg-trade

**Chemin :** `crates/arpg/mge-arpg-trade/`
**Role :** Fenetre trade P2P, hardcore mode, gold drop, vendeur NPC.

```toml
# crates/arpg/mge-arpg-trade/Cargo.toml
[package]
name = "mge-arpg-trade"
version.workspace = true
edition.workspace = true
description = "MGE ARPG trade: P2P trade, NPC vendor, gold"

[dependencies]
mge-ecs = { path = "../../kernel/mge-ecs" }
mge-arpg-items = { path = "../mge-arpg-items" }
mge-net = { path = "../../engine/mge-net" }
serde = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

## 6. Couche Game -- 3 crates

### 6.1 sodomight-game (Phase 1 -- binaire principal)

**Chemin :** `games/sodomight/`
**Role :** Binaire client+host -- init, chargement TOML, lancement partie.

```toml
# games/sodomight/Cargo.toml
[package]
name = "sodomight-game"
version.workspace = true
edition.workspace = true
description = "Sodomight: D2-like ARPG built on MGE"

[[bin]]
name = "sodomight"
path = "src/main.rs"

[dependencies]
# Kernel
mge-core = { path = "../../crates/kernel/mge-core" }
mge-ecs = { path = "../../crates/kernel/mge-ecs" }
mge-math = { path = "../../crates/kernel/mge-math" }
mge-asset = { path = "../../crates/kernel/mge-asset" }
mge-platform = { path = "../../crates/kernel/mge-platform" }

# Engine
mge-render = { path = "../../crates/engine/mge-render" }
mge-audio = { path = "../../crates/engine/mge-audio" }
mge-ui = { path = "../../crates/engine/mge-ui" }
mge-pathfinding = { path = "../../crates/engine/mge-pathfinding" }
mge-collision = { path = "../../crates/engine/mge-collision" }
mge-script = { path = "../../crates/engine/mge-script" }
mge-net = { path = "../../crates/engine/mge-net" }
mge-save = { path = "../../crates/engine/mge-save" }

# ARPG pack
mge-arpg-world = { path = "../../crates/arpg/mge-arpg-world" }
mge-arpg-entity = { path = "../../crates/arpg/mge-arpg-entity" }
mge-arpg-combat = { path = "../../crates/arpg/mge-arpg-combat" }
mge-arpg-items = { path = "../../crates/arpg/mge-arpg-items" }
mge-arpg-stats = { path = "../../crates/arpg/mge-arpg-stats" }
mge-arpg-skills = { path = "../../crates/arpg/mge-arpg-skills" }
mge-arpg-loot = { path = "../../crates/arpg/mge-arpg-loot" }
mge-arpg-ai = { path = "../../crates/arpg/mge-arpg-ai" }
mge-arpg-quest = { path = "../../crates/arpg/mge-arpg-quest" }
mge-arpg-trade = { path = "../../crates/arpg/mge-arpg-trade" }

# Utilitaires
serde = { workspace = true }
toml = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
thiserror = { workspace = true }
rand = { workspace = true }
rand_chacha = { workspace = true }

[features]
default = []
dev-hotreload = []
dev-cheats = []

[lints]
workspace = true
```

---

### 6.2 sodomight-server (Phase 2)

**Chemin :** `games/sodomight-server/`

```toml
# games/sodomight-server/Cargo.toml
[package]
name = "sodomight-server"
version.workspace = true
edition.workspace = true
description = "Sodomight dedicated server: authoritative logic, no rendering"

[[bin]]
name = "sodomight-server"
path = "src/main.rs"

[dependencies]
mge-core = { path = "../../crates/kernel/mge-core" }
mge-ecs = { path = "../../crates/kernel/mge-ecs" }
mge-math = { path = "../../crates/kernel/mge-math" }
mge-asset = { path = "../../crates/kernel/mge-asset" }
mge-net = { path = "../../crates/engine/mge-net" }
mge-save = { path = "../../crates/engine/mge-save" }
mge-collision = { path = "../../crates/engine/mge-collision" }
mge-pathfinding = { path = "../../crates/engine/mge-pathfinding" }
mge-script = { path = "../../crates/engine/mge-script" }

# ARPG pack (logique uniquement, pas d'UI)
mge-arpg-world = { path = "../../crates/arpg/mge-arpg-world" }
mge-arpg-entity = { path = "../../crates/arpg/mge-arpg-entity" }
mge-arpg-combat = { path = "../../crates/arpg/mge-arpg-combat" }
mge-arpg-items = { path = "../../crates/arpg/mge-arpg-items" }
mge-arpg-stats = { path = "../../crates/arpg/mge-arpg-stats" }
mge-arpg-skills = { path = "../../crates/arpg/mge-arpg-skills" }
mge-arpg-loot = { path = "../../crates/arpg/mge-arpg-loot" }
mge-arpg-ai = { path = "../../crates/arpg/mge-arpg-ai" }
mge-arpg-quest = { path = "../../crates/arpg/mge-arpg-quest" }
mge-arpg-trade = { path = "../../crates/arpg/mge-arpg-trade" }

serde = { workspace = true }
toml = { workspace = true }
tokio = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

### 6.3 sodomight-client (Phase 2)

**Chemin :** `games/sodomight-client/`

```toml
# games/sodomight-client/Cargo.toml
[package]
name = "sodomight-client"
version.workspace = true
edition.workspace = true
description = "Sodomight client: rendering + input, no authoritative logic"

[[bin]]
name = "sodomight-client"
path = "src/main.rs"

[dependencies]
mge-core = { path = "../../crates/kernel/mge-core" }
mge-ecs = { path = "../../crates/kernel/mge-ecs" }
mge-math = { path = "../../crates/kernel/mge-math" }
mge-asset = { path = "../../crates/kernel/mge-asset" }
mge-platform = { path = "../../crates/kernel/mge-platform" }
mge-render = { path = "../../crates/engine/mge-render" }
mge-audio = { path = "../../crates/engine/mge-audio" }
mge-ui = { path = "../../crates/engine/mge-ui" }
mge-net = { path = "../../crates/engine/mge-net" }

serde = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

## 7. Outils -- 6 crates

### 7.1 mge-studio

```toml
# tools/mge-studio/Cargo.toml
[package]
name = "mge-studio"
version.workspace = true
edition.workspace = true
description = "MGE Studio: IDE Dioxus for atlas, animation, map, data editing"

[dependencies]
mge-math = { path = "../../crates/kernel/mge-math" }
mge-asset = { path = "../../crates/kernel/mge-asset" }
serde = { workspace = true }
toml = { workspace = true }
image = { workspace = true }
thiserror = { workspace = true }
# dioxus = "0.6" # a confirmer pour l'outil IDE

[lints]
workspace = true
```

### 7.2 a 7.6 -- Outils CLI

Les 5 outils CLI (`mge-packer`, `mge-slicer`, `mge-rescale`, `mge-mirror`, `mge-remap`)
suivent le meme schema minimal :

```toml
# tools/mge-{outil}/Cargo.toml
[package]
name = "mge-{outil}"
version.workspace = true
edition.workspace = true
description = "MGE CLI: {description}"

[dependencies]
image = { workspace = true }
serde = { workspace = true }
toml = { workspace = true }
thiserror = { workspace = true }

[lints]
workspace = true
```

---

## 8. Graphe de dependances complet

Represente sous forme textuelle. La fleche `->` signifie "depend de".

```
=== Couche 1 (zero dep interne) ===
mge-core       -> (rien)
mge-ecs        -> (rien)
mge-math       -> (rien)
mge-asset      -> mge-math
mge-platform   -> mge-math

=== Couche 2 ===
mge-render     -> mge-ecs, mge-math, mge-platform, mge-asset
mge-audio      -> mge-asset
mge-ui         -> mge-render, mge-ecs, mge-math
mge-pathfinding -> mge-math, mge-collision
mge-collision  -> mge-math
mge-collision-rich -> mge-math, mge-collision
mge-script     -> (rhai uniquement)
mge-net        -> mge-core
mge-save       -> mge-ecs

=== Couche 3 ===
mge-arpg-entity  -> mge-ecs, mge-math
mge-arpg-stats   -> mge-ecs, mge-math
mge-arpg-skills  -> mge-ecs, mge-arpg-stats
mge-arpg-combat  -> mge-ecs, mge-math, mge-arpg-stats, mge-arpg-entity, mge-arpg-skills
mge-arpg-items   -> mge-ecs, mge-arpg-stats, mge-arpg-entity
mge-arpg-loot    -> mge-ecs, mge-arpg-items, mge-arpg-stats, mge-arpg-entity
mge-arpg-ai      -> mge-ecs, mge-math, mge-arpg-entity, mge-arpg-combat, mge-pathfinding
mge-arpg-world   -> mge-ecs, mge-math, mge-asset, mge-collision
mge-arpg-quest   -> mge-ecs, mge-script, mge-arpg-entity
mge-arpg-trade   -> mge-ecs, mge-arpg-items, mge-net

=== Couche 4 ===
sodomight-game   -> TOUTES les crates Couche 1/2/3
sodomight-server -> Couche 1 + Couche 2 (sans render/audio/ui) + Couche 3
sodomight-client -> Couche 1 + mge-render, mge-audio, mge-ui, mge-net
```

---

## 9. Ordre de compilation recommande

L'ordre de compilation respecte le graphe de dependances. Les crates sans
dependances internes se compilent en premier, en parallele.

**Vague 1 (parallele) :** aucune dependance interne.
```
mge-core, mge-ecs, mge-math
```

**Vague 2 (parallele) :** depend uniquement de Vague 1.
```
mge-asset (-> mge-math)
mge-platform (-> mge-math)
mge-collision (-> mge-math)
mge-script (-> rhai)
mge-net (-> mge-core)
mge-save (-> mge-ecs)
```

**Vague 3 (parallele) :** depend de Vague 1+2.
```
mge-render (-> mge-ecs, mge-math, mge-platform, mge-asset)
mge-pathfinding (-> mge-math, mge-collision)
mge-collision-rich (-> mge-math, mge-collision)
mge-audio (-> mge-asset)
mge-arpg-entity (-> mge-ecs, mge-math)
mge-arpg-stats (-> mge-ecs, mge-math)
```

**Vague 4 (parallele) :** depend de Vague 3.
```
mge-ui (-> mge-render, mge-ecs, mge-math)
mge-arpg-skills (-> mge-ecs, mge-arpg-stats)
mge-arpg-items (-> mge-ecs, mge-arpg-stats, mge-arpg-entity)
mge-arpg-world (-> mge-ecs, mge-math, mge-asset, mge-collision)
mge-arpg-quest (-> mge-ecs, mge-script, mge-arpg-entity)
mge-arpg-trade (-> mge-ecs, mge-arpg-items, mge-net)
```

**Vague 5 (parallele) :** depend de Vague 4.
```
mge-arpg-combat (-> mge-arpg-stats, mge-arpg-entity, mge-arpg-skills)
mge-arpg-loot (-> mge-arpg-items, mge-arpg-stats, mge-arpg-entity)
```

**Vague 6 (sequentiel) :**
```
mge-arpg-ai (-> mge-arpg-entity, mge-arpg-combat, mge-pathfinding)
```

**Vague 7 (binaires) :**
```
sodomight-game (-> tout)
sodomight-server (-> logique uniquement)
sodomight-client (-> rendu uniquement)
```

---

## 10. Feature flags

### 10.1 Feature flags par crate

| Crate | Feature | Description | Defaut |
|-------|---------|-------------|--------|
| `mge-save` | `legacy-sqlite` | Utilise rusqlite directement | Non |
| `mge-save` | `kindmother-only` | Utilise KindMother governe | Oui |
| `sodomight-game` | `dev-hotreload` | Active le hot-reload des TOML et assets en developpement | Non |
| `sodomight-game` | `dev-cheats` | Active les commandes de triche (God mode, spawn item...) | Non |
| `mge-render` | `dual-res` | Active le rendu double resolution | Oui |
| `mge-render` | `post-process` | Active les post-process effects | Non |
| `mge-net` | `debug-json` | Serialise en JSON au lieu de bincode (debug reseau) | Non |

### 10.2 Configuration en development

```bash
# Build de dev avec hot-reload et cheats
cargo build -p sodomight-game --features "dev-hotreload,dev-cheats"

# Build de release sans features de dev
cargo build -p sodomight-game --release
```

---

## 11. Regles obligatoires par Cargo.toml

Chaque crate du workspace DOIT inclure les sections suivantes :

```toml
[lints]
workspace = true
```

Cette reference herite des reglages workspace qui imposent :
- `unsafe_code = "forbid"` -- aucun code unsafe, jamais, nulle part
- `clippy::all = "warn"` -- tous les lints clippy actifs
- `clippy::pedantic = "warn"` -- lints pedantiques actifs

De plus, les invariants suivants s'appliquent :
- **Pas de `unwrap()`** en code de production. Uniquement dans `#[cfg(test)]`.
- **Pas de `panic!()`** en code de production.
- **Pas de `todo!()`** en code merge. Accepte uniquement en branches de dev.
- **Types d'erreur explicites** via `thiserror` par module.
- **UUIDs v4** pour les identifiants primaires.
- **ISO 8601** pour les timestamps persistables.
- **Annotations MSCM** (`@id`, `@do`, `@role`, `@layer`, `@human`) sur les modules publics.

---

## 12. Commandes de verification

```bash
# Build complet du workspace
cargo build --workspace

# Tests complets
cargo test --workspace

# Lint complet (zero warning tolere)
cargo clippy --workspace -- -D warnings

# Tests d'une crate specifique avec output
cargo test -p mge-ecs -- --nocapture

# Tests filtres par nom
cargo test -p mge-arpg-combat test_cth

# Verifier l'arbre de dependances
cargo tree -p sodomight-game

# Verifier qu'aucun unsafe n'est present
cargo clippy --workspace -- -D unsafe_code

# Build release optimise
cargo build -p sodomight-game --release
```

---

**Total : 33 crates**

| Couche | Nombre | Crates |
|--------|--------|--------|
| Kernel | 5 | mge-core, mge-ecs, mge-math, mge-asset, mge-platform |
| Engine | 9 | mge-render, mge-audio, mge-ui, mge-pathfinding, mge-collision, mge-collision-rich, mge-script, mge-net, mge-save |
| Pack ARPG | 10 | mge-arpg-world, mge-arpg-entity, mge-arpg-combat, mge-arpg-items, mge-arpg-stats, mge-arpg-skills, mge-arpg-loot, mge-arpg-ai, mge-arpg-quest, mge-arpg-trade |
| Game | 3 | sodomight-game, sodomight-server, sodomight-client |
| Outils | 6 | mge-studio, mge-packer, mge-slicer, mge-rescale, mge-mirror, mge-remap |

---

*Document redige par Francois, Dev Back-End -- Miyukini AI Studio*
*Base sur SD-Tech-Architecture.md de Denis*
*Revision : 2026-02-28 v1.0*
