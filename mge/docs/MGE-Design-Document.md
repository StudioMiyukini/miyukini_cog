# MGE — Miyukini Game Engine
## Document de Design — v0.1

**Statut :** Brainstorm consolidé — référence de l'équipe
**Date :** 2026-02-28
**Scope :** Architecture moteur, jeux Phase 1 (Sodomight) et Phase 2 (Allumina)

---

## Table des matières

1. [Vision et périmètre](#1-vision-et-périmètre)
2. [Jeux cibles](#2-jeux-cibles)
3. [Décisions architecturales verrouillées](#3-décisions-architecturales-verrouillées)
4. [Structure des crates](#4-structure-des-crates)
5. [Système ECS — Archetype maison](#5-système-ecs--archetype-maison)
6. [Pipeline de rendu](#6-pipeline-de-rendu)
7. [Pipeline d'assets](#7-pipeline-dassets)
8. [Système audio](#8-système-audio)
9. [Collision et pathfinding](#9-collision-et-pathfinding)
10. [Persistance — KindMother](#10-persistance--kindmother)
11. [Réseau](#11-réseau)
12. [Scripting — Rhai](#12-scripting--rhai)
13. [Game data — TOML](#13-game-data--toml)
14. [mge-studio — Outils](#14-mge-studio--outils)
15. [Plan d'implémentation — Sprints](#15-plan-dimplémentation--sprints)
16. [Analyse concurrentielle — résumé Fabrice](#16-analyse-concurrentielle--résumé-fabrice)

---

## 1. Vision et périmètre

Le **MGE (Miyukini Game Engine)** est un moteur de jeu 2D isométrique écrit entièrement en Rust, développé en interne dans l'écosystème Miyukini COG. Il est conçu pour produire deux jeux distincts :

- **Sodomight** (Phase 1) — clone fidèle de Diablo 2, assets 100% maison
- **Allumina** (Phase 2) — MMO-ARPG sur la même base moteur, lore Miyukini, gameplay étendu

Le MGE n'utilise pas Bevy. Toutes les couches fondamentales (ECS, renderer, audio wrapper, networking) sont développées en interne pour garantir un contrôle total, l'absence de breaking changes externes, et la cohérence architecturale long terme.

**Principes directeurs :**
- Séparation stricte `engine` / `game content` / `lore`
- Data-driven : toute la logique de jeu est configurable via TOML, pas hardcodée
- IDs symboliques stables : aucun chemin de fichier dans le code de jeu
- Hot-reload en développement (assets + TOML via `notify`)
- Cross-platform : Windows, Linux, macOS

---

## 2. Jeux cibles

### 2.1 Sodomight (Phase 1)

| Propriété | Valeur |
|-----------|--------|
| Genre | ARPG isométrique |
| Modèle | Diablo 2 fidèle — même experience, assets maison |
| Classes | 7 (archétypes D2, noms originaux pour release) |
| Contenu | 5 Actes, 3 difficultés (Normal/Nightmare/Hell) |
| Multijoueur | 8 joueurs, Listen Server MVP → Serveur dédié |
| Réseau | Serveur COG dédié `sodomight-server` (codebase séparée d'Allumina) |
| Persistance | KindMother (SQLite), saves côté host/serveur |
| Plateforme | Windows + Linux + macOS |

**Règle fondamentale :** Sodomight doit offrir l'expérience exacte de D2. Chaque mécanique (synergies, runewords, breakpoints FCR/FHR, loot partagé dans le monde, trade libre P2P, hardcore mode) doit être reproduite fidèlement. Les QoL modernes (filtre de loot, stash multi-pages) sont ajoutées sans altérer la profondeur.

**Note légale :** Les noms propres Blizzard (Diablo, Tristram, Horadric, etc.) sont interdits en release publique. Développement interne avec noms D2 toléré ; renommage complet avant toute publication.

### 2.2 Allumina (Phase 2)

| Propriété | Valeur |
|-----------|--------|
| Genre | MMO-ARPG isométrique |
| Base | Moteur MGE — même fondation que Sodomight |
| Différenciation | Lore Miyukini, classes originales, open world, MMO layer |
| MMO Features | Gathering, crafting, housing, guerres de nations (Dark Age of Camelot style) |
| Réseau | Serveur dédié COG `allumina-server` (codebase séparée) |
| Timeline | Après Sodomight validé |

---

## 3. Décisions architecturales verrouillées

| Domaine | Décision |
|---------|----------|
| Langage | Rust (workspace Cargo) |
| Fenêtre/Input | `winit` |
| GPU backend | `wgpu` (Vulkan / Metal / DX12 / WebGPU) |
| ECS | Archetype maison — SoA, sparse overlay pour états éphémères |
| Isométrique | Dimetric 2:1 — tiles 64×32px |
| Résolution | Dual mode : pixel-perfect 800×600 upscale OU HD 1080p+, configurable |
| Audio | `kira` crate |
| Game data | TOML + `serde` |
| Scripting | `rhai` (quêtes, triggers) + Rust pur (IA combat) |
| Assets | PNG raw, PNG spritesheet, PNG+JSON (TexturePacker), Aseprite `.ase`, LDtk `.ldtk` |
| Maps | LDtk (principal) + Tiled `.tmx` (compat) |
| Persistance | KindMother (SQLite) via `mge-save` |
| Hot-reload | `notify` crate — TOML + assets en dev |
| Collision Phase 1 | Tiles walkable/non-walkable + hitboxes circulaires |
| Collision Phase 2 | Couche rich : polygones, shapes composées (Allumina) |
| Pathfinding | A* tile-based isométrique |
| Réseau Phase 1 | Listen Server → migration Serveur dédié |
| Outils | `mge-studio` en Dioxus 0.6 |
| Plateformes | Windows + Linux + macOS |

---

## 4. Structure des crates

```
mge/
├── Cargo.toml                      # workspace
│
├── crates/
│   │
│   ├── kernel/                     # COUCHE KERNEL
│   │   ├── mge-core/               # game loop, time, event bus, scheduler
│   │   ├── mge-ecs/                # archetype ECS maison (World, Query, System)
│   │   ├── mge-math/               # Vec2, IVec2, Rect, dimetric iso math
│   │   ├── mge-asset/              # asset registry, hot-reload, loaders
│   │   └── mge-platform/           # winit window, wgpu device init, input mapping
│   │
│   ├── engine/                     # COUCHE MOTEUR
│   │   ├── mge-render/             # wgpu renderer, sprite batching, tilemap iso, dual-res
│   │   ├── mge-audio/              # kira wrapper (sfx, ambient, musique adaptive)
│   │   ├── mge-ui/                 # HUD in-game (orbes, belt, paperdoll, grille inventaire)
│   │   ├── mge-pathfinding/        # A* tile-based isométrique
│   │   ├── mge-collision/          # tile walkable + hitboxes circulaires (Phase 1)
│   │   ├── mge-collision-rich/     # collision riche polygones/compounds (Allumina)
│   │   ├── mge-script/             # Rhai scripting engine wrapper
│   │   ├── mge-net/                # networking (listen server + dédié)
│   │   └── mge-save/               # KindMother save/load, format personnage
│   │
│   ├── arpg/                       # PACK ARPG (Sodomight + Allumina base)
│   │   ├── mge-arpg-world/         # zones, tiles, chunks, waypoints, portails
│   │   ├── mge-arpg-entity/        # Character, Monster, Item, Projectile, Effect
│   │   ├── mge-arpg-combat/        # attack, skills, projectiles, AoE, formules D2
│   │   ├── mge-arpg-items/         # affixes, sockets, runewords, génération loot
│   │   ├── mge-arpg-stats/         # attributs, résistances, ASPD, breakpoints FCR/FHR/FBR/IAS
│   │   ├── mge-arpg-skills/        # arbres de compétences, synergies, charges
│   │   ├── mge-arpg-loot/          # drop tables, magic find, distribution loot monde
│   │   ├── mge-arpg-ai/            # behaviour trees Rust, IA monstres, aggro
│   │   ├── mge-arpg-quest/         # système de quêtes, triggers Rhai
│   │   └── mge-arpg-trade/         # fenêtre trade P2P, hardcore mode
│   │
│   └── mmo/                        # PACK MMO (Allumina uniquement)
│       ├── mge-mmo-world/          # open world, chunk streaming, instances
│       ├── mge-mmo-social/         # guildes, nations, réputation, RvR
│       ├── mge-mmo-craft/          # gathering, recettes, crafting
│       ├── mge-mmo-housing/        # placement objets, maisons, territoires
│       └── mge-mmo-siege/          # guerres de nations, sièges, points de capture
│
├── tools/                          # OUTILS
│   ├── mge-studio/                 # Dioxus 0.6 — IDE contenu MGE
│   │   ├── atlas-viewer/           # visualise spritesheets + frames
│   │   ├── anim-preview/           # prévisualise animations Aseprite
│   │   ├── map-inspector/          # ouvre et inspecte fichiers LDtk
│   │   ├── gui-builder/            # layout UI in-game
│   │   └── data-editor/            # édite les TOML (items, skills, monstres)
│   ├── mge-packer/                 # CLI : PNG(s) → atlas optimisé PNG + TOML
│   ├── mge-slicer/                 # CLI : spritesheet PNG → frames individuelles
│   ├── mge-rescale/                # CLI : normalise tailles tiles entre packs
│   ├── mge-mirror/                 # CLI : génère directions manquantes par flip
│   └── mge-remap/                  # CLI : remappage de palette couleur
│
├── games/                          # CONTENU DES JEUX
│   ├── sodomight/                  # assets, TOML data, config Sodomight
│   └── allumina/                   # assets, TOML data, config Allumina (Phase 2)
│
├── assets/
│   ├── Dev_assets/                 # placeholders existants ✓
│   └── fonts/                      # polices existantes ✓
│
└── docs/
    └── MGE-Design-Document.md      # ce fichier
```

---

## 5. Système ECS — Archetype maison

### 5.1 Principes

L'ECS MGE utilise le modèle **archetype** : les entités partageant le même ensemble de composants sont stockées ensemble en mémoire (SoA — Struct of Arrays), maximisant la cohérence de cache pour l'itération bulk.

Pour les états éphémères (buffs, debuffs, statuts : Poison, Frozen, Cursed, Amplified...), un **sparse overlay** séparé évite les migrations d'archetype coûteuses.

### 5.2 Identification d'un archetype

```rust
// Archetype identifié par un Vec<TypeId> trié (hashé pour comparaison rapide)
struct ArchetypeId(u64); // hash d'un Vec<TypeId> sorted

struct Archetype {
    id: ArchetypeId,
    component_types: Vec<TypeId>,    // sorted
    // SoA storage : une colonne par composant
    columns: HashMap<TypeId, ComponentColumn>,
    entity_count: usize,
}

struct ComponentColumn {
    data: Vec<u8>,      // bytes contigus, reinterpret-cast au type correct
    layout: Layout,     // size + align du composant
}
```

### 5.3 Archetypes principaux de Sodomight

| Archetype | Composants stables | Overlay éphémère |
|-----------|-------------------|-----------------|
| Player | Position, Velocity, Sprite, AnimState, Stats, Inventory, InputState | Frozen, Cursed, Amplified... |
| Monster | Position, Velocity, Sprite, AnimState, Health, AI, MonsterData | Poison, Stunned, Decrepified... |
| Item (sol) | Position, Sprite, ItemData, DropAge | — |
| Projectile | Position, Velocity, Sprite, ProjectileData, Lifetime | — |
| Effect | Position, Sprite, AnimState, Lifetime | — |
| Waypoint | Position, WaypointData | — |
| TriggerZone | Bounds, TriggerData, ScriptRef | — |

### 5.4 Sparse Overlay (états éphémères)

```rust
// Séparé de l'archetype — pas de migration
struct StatusOverlay {
    poison:      SparseMap<EntityId, PoisonState>,
    frozen:      SparseMap<EntityId, FrozenState>,
    cursed:      SparseMap<EntityId, CursedState>,
    amplified:   SparseMap<EntityId, AmplifiedState>,
    // ...
}
```

### 5.5 Query system

```rust
// Itération sur toutes les entités ayant Position + Health + AI
world.query::<(&Position, &mut Health, &AI)>()
    .for_each(|(pos, hp, ai)| { /* system logic */ });
```

### 5.6 Scheduler

- Systèmes organisés en **stages** (PreUpdate, Update, PostUpdate, Render)
- Systèmes sans dépendances partagées → exécution parallèle via `rayon`
- Graphe de dépendances déclaré par les systèmes (lecture/écriture de composants)

---

## 6. Pipeline de rendu

### 6.1 Stack

```
winit (fenêtre, input)
  └── wgpu (GPU backend : Vulkan/Metal/DX12/WebGPU)
        └── mge-render
              ├── SpriteBatcher      (draw calls groupés, 1 draw call / atlas)
              ├── TilemapRenderer    (tiles iso, culling frustum)
              ├── UIRenderer         (HUD overlay, panels)
              └── PostProcess        (upscale pixel-perfect, effets)
```

### 6.2 Coordonnées isométriques dimetric 2:1

```rust
// Tile (tx, ty) → Screen (sx, sy)
fn tile_to_screen(tx: i32, ty: i32, tile_w: i32, tile_h: i32) -> (i32, i32) {
    let sx = (tx - ty) * (tile_w / 2);
    let sy = (tx + ty) * (tile_h / 2);
    (sx, sy)
}

// tile_w = 64, tile_h = 32 (standard dimetric 2:1)
```

### 6.3 Dual résolution

```toml
# config du joueur
[display]
mode = "pixel_perfect"   # ou "hd"
virtual_width  = 800
virtual_height = 600
scale_factor   = 3        # 800×600 → 2400×1800 (pixel-perfect ×3)
```

- **Pixel-perfect** : renderer cible une surface 800×600 → upscale entier (×2, ×3...) → natif. Zéro blur. Look rétro assumé.
- **HD** : renderer cible la résolution native (1080p/1440p), sprites HD chargés, caméra smooth.

### 6.4 Z-order isométrique

```
Z = tile_y * TILE_ROWS + tile_x   // tri par profondeur isométrique
Entités triées par Z avant draw pour occlusion correcte
```

### 6.5 Sprite batching

Tous les sprites du même atlas sont dessinés en un seul draw call GPU. L'atlas registry groupe les sprites par usage (personnages, tiles, UI, effets).

---

## 7. Pipeline d'assets

### 7.1 Formats supportés

| Format | Usage | Loader |
|--------|-------|--------|
| PNG raw | UI, icônes, tiles individuels | `image` crate |
| PNG spritesheet uniforme | Animations simples, tileset | slicer intégré |
| PNG + JSON TexturePacker | Atlas itch.io standard | parser JSON |
| PNG + TOML MGE | Format natif MGE | `serde` |
| Aseprite `.ase` | Animations pixel art directionnelles | `aseprite` crate |
| LDtk `.ldtk` | Maps isométriques | `ldtk_rust` crate |
| Tiled `.tmx` | Maps (compat packs itch.io) | `tiled` crate |
| OGG / WAV | Audio | `kira` |
| BMFont `.fnt` + PNG | Texte in-game | parser intégré |

### 7.2 Asset Registry (IDs stables)

```toml
# assets/sodomight/registry.toml
# L'ID symbolique ne change jamais.
# Seul le fichier source change lors du swap placeholder → final.

[sprite.necro_walk_north]
source = "Dev_assets/Test_joueur.png"   # placeholder actuel
frame_w = 64
frame_h = 64
frames  = 8
fps     = 12

[sprite.goblin_idle]
source = "Dev_assets/orc.png"
frame_w = 48
frame_h = 48
frames  = 4
fps     = 8

[tileset.grass_summer]
source = "Dev_assets/Grass_a.png"
tile_w = 64
tile_h = 32
```

```rust
// Dans le code jeu — jamais de chemin de fichier
let sprite = assets.get(SpriteId::NecroWalkNorth);
```

### 7.3 Hot-reload (dev uniquement)

```rust
// mge-asset surveille les fichiers via `notify`
// TOML modifié → rechargement automatique sans recompiler
// Sprite swappé → texture GPU mise à jour à la prochaine frame
```

### 7.4 Pipeline d'adaptation packs itch.io

```
Pack itch.io (PNG brut)
  → mge-slicer    (découpe en frames si spritesheet)
  → mge-rescale   (normalise vers 64×32 si tile, ou taille cible)
  → mge-mirror    (génère directions manquantes par flip horizontal)
  → mge-remap     (palette : ajuste couleurs pour cohérence visuelle)
  → mge-packer    (atlas final PNG + TOML MGE)
  → registry.toml (enregistrement avec ID symbolique)
```

---

## 8. Système audio

### 8.1 Stack

`kira` crate — spatial audio, musique adaptive, multi-layer.

### 8.2 Couches audio Sodomight

| Couche | Usage | Comportement |
|--------|-------|-------------|
| Music | Musique de zone | Cross-fade entre zones, loop |
| Ambient | Ambiance (vent, foule, nature) | Spatialisé, persistant |
| SFX | Sons de combat, loot, skills | One-shot, priorité |
| UI | Sons interface (clic, fenêtre, level up) | One-shot, non-spatial |
| Voice | VO NPC (si intégrée) | Séquentiel, non-interruptible |

### 8.3 Sons iconiques à reproduire (Sodomight)

- Son de drop d'item unique / set (distinct du commun)
- Son de rune qui tombe (grave, métallique)
- Son de level up
- Sons distinctifs par type de skill (froid, feu, foudre, os, invocation...)
- Ambiances de zone distinctes : ville, forêt, donjon, désert, jungle

---

## 9. Collision et pathfinding

### 9.1 Phase 1 — Sodomight (D2 style)

**Collision tiles :**
```toml
# Dans la map LDtk : chaque tile a un flag walkable
# mge-collision lit la collision layer de la map
[tile.floor_dirt]
walkable = true

[tile.wall_stone]
walkable = false
```

**Hitboxes combat (cercles) :**
```rust
struct CircleHitbox {
    radius: f32,
    offset: Vec2,   // depuis la position de l'entité
}
// Combat : overlap entre cercles attaquant et défenseur
```

**Pathfinding :**
- A* sur grille de tiles walkable
- Heuristique : distance de Manhattan ou octile
- Groupes de monstres : pathfinding partagé (un chemin calculé pour le groupe)

### 9.2 Phase 2 — Allumina (riche)

Extension non-destructive : `mge-collision-rich` ajoute des shapes composées (polygones, capsules, AABB) pour le housing (placement précis d'objets) et les zones du monde ouvert.

---

## 10. Persistance — KindMother

### 10.1 Schéma personnage Sodomight

```sql
-- Un compte peut avoir plusieurs personnages
CREATE TABLE characters (
    id          TEXT PRIMARY KEY,   -- UUID v4
    account_id  TEXT NOT NULL,
    name        TEXT NOT NULL,
    class       TEXT NOT NULL,      -- "necromancer", "sorceress"...
    level       INTEGER DEFAULT 1,
    experience  INTEGER DEFAULT 0,
    hardcore    BOOLEAN DEFAULT FALSE,
    is_dead     BOOLEAN DEFAULT FALSE,  -- HC mort permanent
    created_at  TEXT NOT NULL,          -- ISO 8601
    updated_at  TEXT NOT NULL
);

CREATE TABLE character_stats (
    character_id TEXT PRIMARY KEY REFERENCES characters(id),
    strength     INTEGER,
    dexterity    INTEGER,
    vitality     INTEGER,
    energy       INTEGER,
    life         INTEGER,
    mana         INTEGER,
    stat_points  INTEGER,
    skill_points INTEGER
);

CREATE TABLE character_skills (
    character_id TEXT REFERENCES characters(id),
    skill_id     TEXT,               -- "bone_spear", "corpse_explosion"...
    level        INTEGER DEFAULT 0,
    PRIMARY KEY (character_id, skill_id)
);

CREATE TABLE inventory_items (
    id           TEXT PRIMARY KEY,  -- UUID v4
    character_id TEXT REFERENCES characters(id),
    container    TEXT,              -- "inventory" | "stash" | "belt" | "equipped"
    slot         TEXT,              -- slot ID ou coordonnées grille "3,2"
    item_def_id  TEXT NOT NULL,     -- référence TOML
    quality      TEXT,              -- "normal"|"magic"|"rare"|"unique"|"set"
    affixes      TEXT,              -- JSON des affixes générés
    sockets      TEXT,              -- JSON des socketed items
    durability   INTEGER
);

CREATE TABLE waypoints (
    character_id TEXT REFERENCES characters(id),
    act          INTEGER,
    zone_id      TEXT,
    difficulty   TEXT,              -- "normal"|"nightmare"|"hell"
    PRIMARY KEY (character_id, act, zone_id, difficulty)
);

CREATE TABLE quest_flags (
    character_id TEXT REFERENCES characters(id),
    quest_id     TEXT,
    difficulty   TEXT,
    completed    BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (character_id, quest_id, difficulty)
);
```

### 10.2 Propriété des saves en multijoueur

- **Listen Server (Sodomight MVP)** : le host possède les saves de tous les joueurs de la partie. Les clients reçoivent un snapshot à la fin de session.
- **Serveur dédié** : le serveur autoritaire possède toutes les saves. Les clients ne stockent rien de critique.

---

## 11. Réseau

### 11.1 Phase 1 — Listen Server (Sodomight MVP)

```
Joueur 1 (host) = game logic authoritative + render
Joueurs 2-8     = clients → envoient inputs, reçoivent state
```

- Protocole : TCP (suffisant pour ARPG D2-style, latence actions faible vs FPS)
- Crate : `tokio` + `axum` pour les connexions, sérialisation `bincode` pour les messages
- Maximum 8 joueurs par partie (D2 standard)
- Loot partagé dans le monde (pas d'instanciation) — timer de priorité 30s pour le joueur qui a kill

### 11.2 Architecture réseau des messages

```rust
// Messages Client → Host
enum ClientMessage {
    Move { target: Vec2 },
    UseSkill { skill_id: SkillId, target: EntityTarget },
    PickupItem { entity_id: EntityId },
    OpenTrade { target_player: PlayerId },
    // ...
}

// Messages Host → Clients
enum ServerMessage {
    EntitySpawned { entity: EntitySnapshot },
    EntityMoved { id: EntityId, pos: Vec2 },
    EntityDied { id: EntityId },
    ItemDropped { item: ItemSnapshot, pos: Vec2 },
    StatChanged { player: PlayerId, stat: StatType, value: i32 },
    // ...
}
```

### 11.3 Phase 2 — Serveur dédié Sodomight

Migration Listen Server → Serveur dédié COG `sodomight-server` (codebase séparée d'Allumina). Le serveur est un binaire Rust autonome : `mge-net` + logique de jeu + KindMother.

---

## 12. Scripting — Rhai

### 12.1 Usages

| Usage | Technologie | Raison |
|-------|------------|--------|
| Triggers de quêtes | Rhai | Conditions dynamiques, accessible aux designers |
| Événements de zone | Rhai | "On player enter → spawn wave" |
| Dialogues NPC | Rhai | Branches conditionnelles |
| IA simple (patrouille) | Rhai | Comportements basiques sans recompiler |
| IA combat complexe | Rust ECS systems | Performance, behaviour trees |
| Formules de stats | Rust pur | Précision, performance critique |

### 12.2 Exemple script de quête

```rhai
// quests/act1/den_of_evil.rhai
fn on_quest_complete(player) {
    player.add_stat_point(1);
    player.set_flag("den_cleared_normal", true);
    world.play_sound("quest_complete");
    world.show_notification("Den of Evil cleared — +1 Stat Point");
}

fn can_complete(player) {
    player.get_kill_count("zone.den_of_evil") >= 1
}
```

### 12.3 Intégration ECS

```rust
// mge-script expose une API safe au Rhai engine
// Les scripts ne peuvent accéder qu'aux fonctions explicitement exposées
let engine = rhai::Engine::new();
engine.register_fn("get_kill_count", |player: &PlayerCtx, zone: &str| {
    player.kill_counts.get(zone).copied().unwrap_or(0)
});
```

---

## 13. Game data — TOML

Toute la logique de jeu de Sodomight est définie dans `games/sodomight/data/`.

### 13.1 Structure

```
games/sodomight/data/
  classes/
    necromancer.toml
    sorceress.toml
    ...
  skills/
    necromancer/
      bone_spear.toml
      corpse_explosion.toml
      ...
  items/
    uniques/
      shako.toml
      maras_kaleidoscope.toml
    sets/
      tal_rashas_wrappings.toml
    affixes/
      prefixes.toml
      suffixes.toml
  monsters/
    act1/
      fallen.toml
      zombie.toml
      andariel.toml
    ...
  zones/
    act1/
      blood_moor.toml
      den_of_evil.toml
      ...
  loot_tables/
    act1.toml
    bosses.toml
  runewords/
    runewords.toml
  quests/
    act1/
      den_of_evil.toml
```

### 13.2 Exemples TOML

```toml
# skills/necromancer/bone_spear.toml
[skill]
id             = "bone_spear"
class          = "necromancer"
tree           = "poison_bone"
prerequisites  = ["teeth"]
max_level      = 20
mana_base      = 9
mana_per_level = 0.5

[damage]
min_base = 26
max_base = 30
per_level_min = 10
per_level_max = 12
type = "magic"

[[synergies]]
source    = "teeth"
bonus     = 0.07        # +7% magic damage par hard point
type      = "damage_pct"

[[synergies]]
source    = "bone_spirit"
bonus     = 0.07
type      = "damage_pct"
```

```toml
# monsters/act1/fallen.toml
[monster]
id          = "fallen"
name        = "Fallen"
type        = "normal"
act         = 1

[stats]
life_base   = 8
life_per_player = 4      # +4 HP par joueur additionnel dans la partie
damage_min  = 1
damage_max  = 4
defense     = 6
walk_speed  = 4.0
run_speed   = 8.0

[ai]
type        = "melee_follower"
aggro_range = 8           # en tiles
flee_hp_pct = 0.25        # fuit si < 25% HP
rally_fallen = true       # invoque ses alliés en fuyant

[loot]
table = "act1_fallen"
```

```toml
# runewords/runewords.toml
[[runeword]]
id     = "spirit_shield"
name   = "Spirit"
runes  = ["Tal", "Thul", "Ort", "Amn"]
sockets = 4
item_types = ["shield", "monarch"]
properties = [
  { stat = "faster_cast_rate", value = 35 },
  { stat = "faster_hit_recovery", value = 55 },
  { stat = "skill_all", value = 2 },
  { stat = "vitality", value = 22 },
  { stat = "mana", value = 89 },
  { stat = "cold_resist", value = 35 },
  { stat = "lightning_resist", value = 35 },
  { stat = "absorb_lightning_pct", value = 0.25 },
]
```

---

## 14. mge-studio — Outils

Application desktop Dioxus 0.6. Interface principale de gestion du contenu MGE.

### 14.1 Modules

| Module | Fonction | Priorité |
|--------|----------|----------|
| `atlas-viewer` | Visualise spritesheets, frames, animations | HAUTE — Sprint 0 |
| `anim-preview` | Prévisualise animations Aseprite avec tags | HAUTE — Sprint 0 |
| `data-editor` | Édite les TOML (items, skills, monstres) | HAUTE — Sprint 1 |
| `map-inspector` | Ouvre et inspecte fichiers LDtk | MOYENNE — Sprint 2 |
| `gui-builder` | Layout des UI in-game (orbes, belt, etc.) | MOYENNE — Sprint 2 |
| `asset-packer` | Lance mge-packer CLI depuis l'UI | BASSE — Sprint 3 |

### 14.2 CLI Tools

| Outil | Commande | Usage |
|-------|---------|-------|
| `mge-packer` | `mge-packer pack -i sprites/ -o atlas.png` | Génère atlas PNG + TOML |
| `mge-slicer` | `mge-slicer slice -i sheet.png -fw 64 -fh 64` | Découpe spritesheet |
| `mge-rescale` | `mge-rescale -i tile.png -w 64 -h 32` | Normalise taille tiles |
| `mge-mirror` | `mge-mirror -i walk_east.png -dirs NE,SE` | Génère directions miroir |
| `mge-remap` | `mge-remap -i sprite.png -p palette.toml` | Remappage palette |

---

## 15. Plan d'implémentation — Sprints

### Sprint 0 — Fondations moteur (priorité absolue)

**Objectif :** Fenêtre + tile iso + sprite animé + pathfinding. Rien de gameplay.

- [ ] `mge-platform` : window winit, device wgpu, input events
- [ ] `mge-math` : Vec2, IVec2, Rect, fonctions dimetric iso
- [ ] `mge-render` : sprite batcher basique, tilemap iso (tiles `Dev_assets/Grass_*.png`)
- [ ] `mge-ecs` : World, archetype basique, query, system stages
- [ ] `mge-asset` : PNG loader, registry TOML, hot-reload `notify`
- [ ] `mge-pathfinding` : A* tile-based
- [ ] `mge-studio/atlas-viewer` + `mge-studio/anim-preview` : débloquer les artistes
- [ ] Scène de test : map herbe dimetric + entité qui se déplace au clic

**Livrable Sprint 0 :** Cliquer sur une map iso → une entité se déplace via pathfinding.

---

### Sprint 1 — Boucle de combat

**Objectif :** 1 classe jouable (Necromancer), combat fonctionnel, loot basique.

- [ ] `mge-arpg-entity` : Character, Monster, Item archetypes
- [ ] `mge-arpg-stats` : attributs, calcul life/mana, formules D2
- [ ] `mge-arpg-combat` : auto-attack, 3 skills Necro (Bone Spear, Corpse Explosion, Raise Skeleton)
- [ ] `mge-arpg-ai` : IA monster basique (follow + attack)
- [ ] `mge-arpg-items` : Normal/Magic items, affixes simples
- [ ] `mge-arpg-loot` : drop table Act 1 basique
- [ ] `mge-ui` : barre vie/mana, belt, hotkeys skills
- [ ] `mge-collision` : tiles walkable + hitboxes circulaires
- [ ] `mge-audio` : intégration kira, sons de combat basiques
- [ ] `mge-save` : sauvegarde/chargement personnage KindMother

**Livrable Sprint 1 :** Necromancer dans une arène, kill monstres, loot items, sauvegarder.

---

### Sprint 2 — Progression et inventaire

**Objectif :** Système de progression D2 complet, inventaire grille, arbres de compétences.

- [ ] `mge-arpg-skills` : arbre complet Necromancer (3 arbres, synergies)
- [ ] `mge-arpg-items` : Rare/Unique items, sockets, identification
- [ ] `mge-ui` : inventaire grille 10×4 drag-and-drop, paperdoll, fenêtre skills
- [ ] `mge-arpg-stats` : breakpoints FCR/FHR/FBR/IAS (tables complètes D2)
- [ ] `mge-script` : Rhai intégré, premiers scripts de quête
- [ ] `mge-studio/data-editor` : édition TOML items et skills

**Livrable Sprint 2 :** Progression complète Necromancer, gestion inventaire, arbre de skills.

---

### Sprint 3 — Acte 1 complet

**Objectif :** Sodomight Acte 1 jouable du début à Andariel.

- [ ] `mge-arpg-world` : zones Act 1 complètes (Blood Moor → Catacombs)
- [ ] `mge-arpg-quest` : 6 quêtes Act 1, flags, récompenses
- [ ] Waypoints Act 1, portails en ville
- [ ] Tous les monstres Act 1 (fallen, zombie, corrupted rogue, quill rat, dark elder...)
- [ ] Boss Andariel (phases, immunités NM/Hell)
- [ ] 15 items Uniques Act 1, 2-3 runewords low-tier
- [ ] Filtre de loot (configurable règles textuelles)
- [ ] Audio : musique Act 1, ambiances par zone, sons de loot distinctifs
- [ ] `mge-studio/map-inspector` : inspecter maps LDtk Act 1

**Livrable Sprint 3 :** Acte 1 jouable solo du début à la fin avec boss.

---

### Sprint 4 — Multijoueur

**Objectif :** 2-8 joueurs en réseau (Listen Server).

- [ ] `mge-net` : listen server TCP, messages Client/Server
- [ ] Synchronisation entités (positions, combat, loot)
- [ ] Loot partagé monde (timer priorité 30s)
- [ ] Trade P2P basique (fenêtre de trade)
- [ ] Scaling HP monstres par nombre de joueurs (D2 standard)
- [ ] Parties nommées, mot de passe optionnel

**Livrable Sprint 4 :** 2-4 joueurs en LAN jouent Acte 1 ensemble.

---

### Sprints 5+ — Contenu étendu

- Classes 2-7 (Sorceress, Barbarian, Paladin, Amazon, Druid, Assassin)
- Actes 2-5
- Set items complets, runewords rares
- 3 difficultés (Normal/Nightmare/Hell) avec immunités
- Hardcore mode
- Stash multi-pages
- Cube Alchimique + recettes
- Uber bosses
- Serveur dédié `sodomight-server`

---

## 16. Analyse concurrentielle — résumé Fabrice

*Rapport complet : Fabrice PR Analysis 2026-02-28*

**Menace directe :** Darkhaven (Moon Beast Productions) — créateurs originaux D1/D2, démo Steam depuis février 2026. Différenciation Miyukini : Rust open-source, transparence tech, lore Allumina fort.

**Marché validé :** D2R maintient ~54 000 joueurs quotidiens en 2026. Le segment existe.

**Leçons critiques :**
- **Last Epoch** : bon lancement, effondrement post-launch. La rétention endgame est le vrai défi.
- **PoE 2** : nerfs agressifs = review bomb. Toute modification de balance majeure doit être communiquée 2 semaines à l'avance.
- **Wolcen** : scope creep fatal. Ne pas lancer avant stabilité des systèmes core.

**Red flags identifiés :**
1. Ne pas développer 7 classes en parallèle — une classe polished vaut 10 classes à 30%
2. Ne pas instancier le loot (détruirait la dynamique sociale D2)
3. Ne pas annoncer de monétisation avant que le jeu soit aimé
4. Renommer TOUS les noms propres Blizzard avant toute release publique

---

*Document maintenu par l'équipe MGE — Miyukini COG*
*Prochaine révision : après Sprint 0*
