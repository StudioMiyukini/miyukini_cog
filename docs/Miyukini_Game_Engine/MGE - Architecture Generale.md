# MGE — Architecture Générale

Architecture en couches du Miyukini Game Engine : microkernel, plugins, rendu, bridge COG et couche jeu.

## Contexte

Le MGE adopte une architecture en couches avec un microkernel minimal au centre. Chaque couche a des responsabilités précises et des règles de dépendance strictes.

## Portée / Scope

- **Applicable à :** Conception du moteur, développement des crates, intégration des jeux.
- **Audience :** Architectes, développeurs moteur, développeurs tiers.
- **Statut :** Spécification normative.

---

## 1. Vue d'ensemble — Les cinq couches

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        GAME LAYER (Allumina, etc.)                       │
│  Logique métier jeu, règles, contenu. Dépend de mge-plugins + mge-core.  │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         mge-cog-bridge                                   │
│  CogService, lancement/stop, isolation, IPC, Cores (KindMother, MWS).    │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         mge-plugins                                      │
│  physics | render | input | audio | ai | network | ...                   │
│  Plugins officiels ; chacun est optionnel et remplaçable.                │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         mge-render                                       │
│  Abstraction rendu : headless-safe, backend-agnostic (minifb, wgpu...). │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                         mge-core (MICROKERNEL)                           │
│  Engine { world, scheduler, events, time, rng } + tick()                │
│  Cœur minimal. Aucune physique, rendu, audio ou réseau.                 │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Responsabilités par couche

### 2.1 mge-core (microkernel)

| Composant | Responsabilité |
|-----------|----------------|
| **Engine** | Cycle de vie (new, build), **tick** (simulation pure), seed RNG, configuration. Pas de `run()` — le core ne connaît pas la boucle ni le rendu. |
| **World** | Stockage entités, composants (SoA), archetypes, queries. |
| **Scheduler** | Ordre déterministe des systèmes, budget CPU, profiling hooks. |
| **EventQueue** | File d'événements typés ; lecture explicite par les systèmes (`events.iter::<E>()`), pas de callbacks. |
| **RNG** | Génération déterministe, seed global, seed par entité. |
| **Time** | Delta time, fixed timestep, time scale, pause. |

**Règle :** mge-core ne dépend d'aucun plugin. Il ne connaît pas la physique, le rendu ni le réseau.

### 2.2 mge-render

| Responsabilité | Détail |
|----------------|--------|
| **Abstraction** | API unifiée : draw sprite, draw rect, clear, présent. |
| **Backend-agnostic** | Minifb, wgpu, ou null (headless) selon configuration. |
| **Headless-safe** | En mode headless, les appels rendu sont no-op ou bufferisés sans effet. |
| **Culling** | Exclut les entités hors écran avant dessin. |

**Règle :** mge-render dépend uniquement de mge-core. Il lit le World pour le rendu, ne modifie pas la simulation.

### 2.3 mge-plugins

| Plugin | Rôle |
|-------|------|
| **physics** | Collisions, hitbox, déplacement, résolution AABB. |
| **render** | Intégration mge-render dans la boucle, caméra, sprites. |
| **input** | Clavier, souris, manette ; mapping vers événements. |
| **audio** | Sons, musique ; stub si absent. |
| **ai** | Pathfinding, comportements, decision trees. |
| **network** | Snapshot/delta, host auth, client replica ; transport MWS. |

**Règle :** Chaque plugin enregistre ses composants et systèmes via le trait Plugin. Dépendances entre plugins déclaratives.

### 2.4 mge-cog-bridge

| Responsabilité | Détail |
|----------------|--------|
| **CogService** | Implémentation du contrat service COG : start, stop, status, config. |
| **Lancement** | Depuis Central : exe séparé, `Command::new`. |
| **Stop** | Signal gracieux, sauvegarde état si nécessaire. |
| **Communication** | IPC (events, status) entre Central et jeu. |
| **Isolation** | Processus séparé ; pas de shared memory directe. |
| **Cores** | Médiation BondingBrother → KindMother (sauvegardes), MWS (réseau). |

**Règle :** mge-cog-bridge est optionnel. Un jeu standalone peut tourner sans.

### 2.5 Game layer

| Responsabilité | Détail |
|----------------|--------|
| **Logique métier** | Règles de jeu, progression, inventaire, combat. |
| **Contenu** | Données, assets, config. |
| **Composition** | Choix des plugins, ordre des systèmes, configuration. |

**Règle :** Le jeu dépend des plugins dont il a besoin. Il n'étend pas le core directement.

---

## 3. Règles de dépendance

```
game layer     →  mge-plugins  →  mge-render  →  mge-core
                    ↑
                mge-cog-bridge (optionnel, peer des plugins)
```

- **Jamais l'inverse :** mge-core ne dépend pas des plugins, du rendu ou du bridge.
- **Transitivité :** Le jeu peut dépendre de mge-cog-bridge s'il est lancé depuis Central.
- **Plugins entre eux :** Un plugin peut déclarer une dépendance à un autre (ex. render dépend de physics pour les hitbox visuelles).

---

## 4. Flux de vie — Boot à shutdown

```
┌──────────────────────────────────────────────────────────────────────────┐
│  BOOT                                                                     │
│  1. Engine::new(config)                                                   │
│  2. engine.add_plugin(PhysicsPlugin)                                      │
│  3. engine.add_plugin(RenderPlugin)                                       │
│  4. engine.add_plugin(InputPlugin)                                        │
│  5. engine.add_plugin(AlluminaGamePlugin)  // game layer                   │
│  6. engine.build()  // résolution des plugins, enregistrement             │
└──────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌──────────────────────────────────────────────────────────────────────────┐
│  GAME LOOP (orchestré par le Game Runtime, PAS par le core)               │
│  loop {                                                                    │
│    input();           // plugin input                                     │
│    engine.tick();     // simulation pure (Scheduler, World, EventQueue)   │
│    render();          // plugin render                                    │
│  }                                                                         │
│  Le core ne fournit que tick(). Il ne connaît pas input ni render.        │
└──────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌──────────────────────────────────────────────────────────────────────────┐
│  SHUTDOWN                                                                  │
│  1. Signal stop (Game Runtime)                                             │
│  2. Sauvegarde (KindMother via BondingBrother) si nécessaire              │
│  3. Libération ressources, fermeture fenêtre                              │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Diagramme de flux — Données

```
                    ┌─────────────┐
                    │   Input     │
                    │ (clavier,   │
                    │  souris)    │
                    └──────┬──────┘
                           │ events
                           ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│ EventQueue  │◄────│  Scheduler  │────►│    World    │
│ (lecture    │     │ (systemes   │     │ (entités,   │
│  explicite) │     │  lissent    │     │  composants)│
└──────┬──────┘     └──────┬──────┘     └──────┬──────┘
       │                   │                   │
       │                   │                   │ read
       │                   │                   ▼
       │                   │            ┌─────────────┐
       │                   │            │   Render     │
       │                   │            │ (sprites,    │
       │                   │            │  caméra)     │
       │                   │            └──────┬──────┘
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Plugins    │     │  RNG, Time  │     │   Fenêtre   │
│ (physics,   │     │ (déterministe)     │ (minifb,    │
│  ai, etc.)  │     └─────────────┘     │  wgpu)      │
└─────────────┘                          └─────────────┘
```

---

## 6. Structure crates — Workspace MGE indépendant

Le MGE est organisé en workspace indépendant `mge/` avec architecture en couches :

### Kernel (Layer 0) — 7 crates

| Crate | Responsabilité |
|-------|----------------|
| `mge-core` | Engine, boot, tick, plugin trait, config |
| `mge-time` | Delta, fixed timestep, time scale, pause |
| `mge-rng` | RNG déterministe, seed |
| `mge-event` | Event trait, EventQueue (double buffer) |
| `mge-ecs` | World, EntityId, Component, storage SoA |
| `mge-query` | Query helpers, Query2Mut |
| `mge-profiler` | TickMetrics, PhaseMetrics, budget |

### Core Universal Pack (Layer 1) — 6 crates

| Crate | Domain | Rôle |
|-------|--------|------|
| `mge-plugin-spatial` | spatial | Position2D, Velocity2D, SpatialHash |
| `mge-plugin-input` | input | InputState, KeyBinding |
| `mge-plugin-render-2d` | render | Sprite, Camera2D, RenderLayer |
| `mge-plugin-audio` | audio | AudioSource, AudioListener |
| `mge-plugin-basic-physics` | physics | Collider, RigidBody, CollisionEvent |
| `mge-plugin-save-load` | persistence | SaveState, Snapshot |

### Genre Packs (Layer 2) — 16 packs, ~99 crates

Voir [MGE - Pack Architecture](./MGE%20-%20Pack%20Architecture.md) pour la liste complète et les dépendances inter-packs.

**Packs principaux :** RPG, Massive Battle, Social Simulation, RTS, Grand Strategy, Puzzle, Sandbox, Platformer, Shooter, Roguelike, Racing, Factory, Idle, Tycoon, Visual Novel, TCG.

### Tooling Layer (hors runtime)

Autour du runtime, une couche d'outils d'édition : Data Authoring, Prefab Editor, Balance Lab, Battle Sandbox, Sprite Tool, Rule Editor, Export Pipeline, AI Assist. Voir [MGE - Platform Tooling Layer v1](./MGE%20-%20Platform%20Tooling%20Layer%20v1.md).

---

## 7. Références

| Document | Rôle |
|----------|------|
| [MGE - Document Fondateur](./MGE%20-%20Document%20Fondateur.md) | Vision, philosophie. |
| [MGE - Pack Architecture](./MGE%20-%20Pack%20Architecture.md) | Philosophie packs, dépendances, composition. |
| [MGE - Core Specification Technique](./MGE%20-%20Core%20Specification%20Technique.md) | Détail Engine, World, Scheduler, EventQueue, RNG, Time. |
| [MGE - Plugin Contract](./MGE%20-%20Plugin%20Contract.md) | Trait Plugin, enregistrement. |
| [MGE - Intégration COG](./MGE%20-%20Integration%20COG.md) | CogService, lancement, isolation. |
| [MGE - Référence Commune](./reference/MGE%20-%20Reference%20Commune.md) | Types Vec2, Rect, coordonnées. |

---

**Document** : MGE — Architecture Générale  
**Version** : 1.1  
**Date** : 2026-02-20  
**Statut** : Spécification normative
