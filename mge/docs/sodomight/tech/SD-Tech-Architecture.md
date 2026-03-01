<!-- @id: SD-Tech-Architecture @do: reference @role: tech-lead @layer: 3 @human: miyuk -->

# SD-Tech-Architecture -- Architecture Technique Sodomight

**Auteur :** Denis (Chef Dev Senior, Miyukini AI Studio)
**Date :** 2026-02-28
**Statut :** Reference technique -- v1.0
**Projet :** Sodomight (clone fidele Diablo 2 LoD, assets maison)
**Moteur :** MGE (Miyukini Game Engine)

---

## Table des matieres

1. [Vue d'ensemble architecturale](#1-vue-densemble-architecturale)
2. [Liste exhaustive des crates](#2-liste-exhaustive-des-crates)
3. [Graphe de dependances inter-crates](#3-graphe-de-dependances-inter-crates)
4. [Structure des crates Sodomight-specific](#4-structure-des-crates-sodomight-specific)
5. [Pipeline de demarrage du jeu](#5-pipeline-de-demarrage-du-jeu)
6. [Boucle de jeu principale](#6-boucle-de-jeu-principale)
7. [Separation client/serveur](#7-separation-clientserveur)
8. [Architecture reseau](#8-architecture-reseau)
9. [Plan de developpement par sprint](#9-plan-de-developpement-par-sprint)
10. [Invariants de securite et qualite](#10-invariants-de-securite-et-qualite)

---

## 1. Vue d'ensemble architecturale

### 1.1 Couches

L'architecture MGE pour Sodomight s'organise en 4 couches verticales strictes. Aucune couche haute ne peut etre importee par une couche basse.

```
Couche 4 (Game)      : sodomight-game, sodomight-server, sodomight-client
Couche 3 (Pack ARPG) : mge-arpg-*  (combat, items, loot, AI, world, quest, trade, skills, stats, entity)
Couche 2 (Engine)    : mge-render, mge-audio, mge-ui, mge-pathfinding, mge-collision, mge-script, mge-net, mge-save
Couche 1 (Kernel)    : mge-core, mge-ecs, mge-math, mge-asset, mge-platform
```

### 1.2 Principes directeurs

| Principe | Description |
|----------|-------------|
| Data-driven | Toute la logique de jeu configurable via TOML, pas hardcodee |
| IDs symboliques | Aucun chemin de fichier dans le code de jeu |
| Hot-reload | Assets + TOML via `notify` en developpement |
| unsafe_code = "forbid" | Dans TOUS les Cargo.toml sans exception |
| Clippy pedantic | `all = "warn"`, `pedantic = "warn"` |
| Pas de unwrap() | En production, uniquement dans les tests |
| ECS archetype maison | SoA storage + sparse overlay pour etats ephemeres |
| Separation engine/content | Le moteur ne connait pas le contenu de Sodomight |

### 1.3 Lois d'Autonomie (NON NEGOCIABLES)

1. Aucune dependance externe critique a l'execution
2. Isolement = etat normal
3. Etat local souverain
4. Pas de temps global requis
5. Cout proportionnel au hardware
6. Autonomie n'empeche pas la federation
7. Strate Cores immuable
8. Migration = diplomatie entre environnements

---

## 2. Liste exhaustive des crates

### 2.1 Couche Kernel (5 crates)

| Crate | Chemin | Role | Dependances externes |
|-------|--------|------|---------------------|
| `mge-core` | `crates/kernel/mge-core/` | Game loop, time management, event bus, system scheduler, tick rate | Aucune |
| `mge-ecs` | `crates/kernel/mge-ecs/` | ECS archetype maison : World, Archetype, Query, System, SparseMap | Aucune |
| `mge-math` | `crates/kernel/mge-math/` | Vec2, IVec2, Rect, fonctions dimetric isometrique, interpolation | Aucune |
| `mge-asset` | `crates/kernel/mge-asset/` | Asset registry, hot-reload via notify, loaders PNG/TOML/Aseprite/LDtk | `notify`, `image`, `serde`, `toml` |
| `mge-platform` | `crates/kernel/mge-platform/` | Fenetre winit, device wgpu init, input mapping, fullscreen, cursor | `winit`, `wgpu` |

### 2.2 Couche Engine (9 crates)

| Crate | Chemin | Role | Dependances externes |
|-------|--------|------|---------------------|
| `mge-render` | `crates/engine/mge-render/` | Renderer wgpu : sprite batching, tilemap iso, dual-res, z-order, post-process | `wgpu`, `image` |
| `mge-audio` | `crates/engine/mge-audio/` | Wrapper kira : SFX, ambient, musique adaptative, spatial audio | `kira` |
| `mge-ui` | `crates/engine/mge-ui/` | HUD in-game : orbes vie/mana, belt, paperdoll, inventaire grille, arbres skills | Aucune (depend mge-render) |
| `mge-pathfinding` | `crates/engine/mge-pathfinding/` | A* tile-based isometrique, heuristique Manhattan/octile, pathfinding partage | Aucune |
| `mge-collision` | `crates/engine/mge-collision/` | Tile walkable/non-walkable + hitboxes circulaires (Phase 1 Sodomight) | Aucune |
| `mge-collision-rich` | `crates/engine/mge-collision-rich/` | Collision riche : polygones, capsules, AABB (Phase 2 Allumina) | Aucune |
| `mge-script` | `crates/engine/mge-script/` | Wrapper Rhai : scripting quetes, triggers, dialogues NPC, API safe | `rhai` |
| `mge-net` | `crates/engine/mge-net/` | Networking : listen server TCP, messages client/server, serialisation bincode | `tokio`, `bincode` |
| `mge-save` | `crates/engine/mge-save/` | KindMother save/load, format personnage, SQLite governe | `rusqlite`, `serde` |

### 2.3 Couche Pack ARPG (10 crates)

| Crate | Chemin | Role |
|-------|--------|------|
| `mge-arpg-world` | `crates/arpg/mge-arpg-world/` | Zones, tiles, chunks, waypoints, portails, generation procedurale DOC |
| `mge-arpg-entity` | `crates/arpg/mge-arpg-entity/` | Archetypes Character, Monster, Item, Projectile, Effect, Shrine |
| `mge-arpg-combat` | `crates/arpg/mge-arpg-combat/` | Pipeline attack, skills, projectiles, AoE, formules D2 (CTH, damage) |
| `mge-arpg-items` | `crates/arpg/mge-arpg-items/` | Affixes, sockets, runewords, set items, unique items, generation loot, cube |
| `mge-arpg-stats` | `crates/arpg/mge-arpg-stats/` | Attributs, resistances, breakpoints FCR/FHR/FBR/IAS, formules derivees |
| `mge-arpg-skills` | `crates/arpg/mge-arpg-skills/` | Arbres de competences, synergies, cooldowns, charges, hotkeys |
| `mge-arpg-loot` | `crates/arpg/mge-arpg-loot/` | Drop tables, treasure classes, NoDrop, Magic Find, distribution monde |
| `mge-arpg-ai` | `crates/arpg/mge-arpg-ai/` | Behaviour trees Rust, 43 archetypes IA, aggro, leash, state machines |
| `mge-arpg-quest` | `crates/arpg/mge-arpg-quest/` | Systeme de quetes, triggers Rhai, flags, recompenses, NPC dialogues |
| `mge-arpg-trade` | `crates/arpg/mge-arpg-trade/` | Fenetre trade P2P, hardcore mode, gold drop, vendeur NPC |

### 2.4 Couche Game (3 crates)

| Crate | Chemin | Role |
|-------|--------|------|
| `sodomight-game` | `games/sodomight/` | Binaire client+host : init, chargement TOML, lancement partie |
| `sodomight-server` | `games/sodomight-server/` | Binaire serveur dedie (Phase 2) : logique autorit. sans rendu |
| `sodomight-client` | `games/sodomight-client/` | Binaire client leger (Phase 2) : rendu + input, pas de logique |

### 2.5 Outils (6 crates)

| Crate | Chemin | Role |
|-------|--------|------|
| `mge-studio` | `tools/mge-studio/` | IDE Dioxus 0.6 : atlas-viewer, anim-preview, map-inspector, data-editor, gui-builder |
| `mge-packer` | `tools/mge-packer/` | CLI : PNG(s) vers atlas optimise PNG + TOML |
| `mge-slicer` | `tools/mge-slicer/` | CLI : spritesheet PNG vers frames individuelles |
| `mge-rescale` | `tools/mge-rescale/` | CLI : normalise tailles tiles entre packs |
| `mge-mirror` | `tools/mge-mirror/` | CLI : genere directions manquantes par flip |
| `mge-remap` | `tools/mge-remap/` | CLI : remappage de palette couleur |

**Total : 33 crates**

---

## 3. Graphe de dependances inter-crates

### 3.1 Dependances directes (notation A -> B signifie "A depend de B")

```
sodomight-game -> mge-arpg-* (tous), mge-core, mge-platform, mge-render, mge-audio, mge-ui
sodomight-game -> mge-save, mge-net, mge-script, mge-asset

mge-arpg-combat  -> mge-ecs, mge-math, mge-arpg-stats, mge-arpg-entity, mge-arpg-skills
mge-arpg-items   -> mge-ecs, mge-arpg-stats, mge-arpg-entity
mge-arpg-loot    -> mge-ecs, mge-arpg-items, mge-arpg-stats, mge-arpg-entity
mge-arpg-ai      -> mge-ecs, mge-math, mge-arpg-entity, mge-arpg-combat, mge-pathfinding
mge-arpg-world   -> mge-ecs, mge-math, mge-asset, mge-collision
mge-arpg-quest   -> mge-ecs, mge-script, mge-arpg-entity
mge-arpg-skills  -> mge-ecs, mge-arpg-stats
mge-arpg-stats   -> mge-ecs, mge-math
mge-arpg-entity  -> mge-ecs, mge-math
mge-arpg-trade   -> mge-ecs, mge-arpg-items, mge-net

mge-render     -> mge-ecs, mge-math, mge-platform, mge-asset
mge-audio      -> mge-asset
mge-ui         -> mge-render, mge-ecs, mge-math
mge-pathfinding -> mge-math, mge-collision
mge-collision  -> mge-math
mge-script     -> (rhai)
mge-net        -> mge-core
mge-save       -> mge-ecs

mge-core       -> (aucun crate interne)
mge-ecs        -> (aucun crate interne)
mge-math       -> (aucun crate interne)
mge-asset      -> mge-math
mge-platform   -> mge-math
```

### 3.2 Arbre de dependances simplifie

```
                      sodomight-game
                     /       |       \
              mge-arpg-*   mge-ui   mge-net
             /    |    \     |        |
    mge-ecs  mge-math  mge-render  mge-core
              |    \      |
         mge-asset  mge-platform
              |
           mge-collision
              |
         mge-pathfinding
```

---

## 4. Structure des crates Sodomight-specific

### 4.1 sodomight-game (binaire principal Phase 1)

```
games/sodomight/
  Cargo.toml
  src/
    main.rs             # Point d'entree, init fenetre, game loop
    app.rs              # Application state machine
    config.rs           # Chargement config.toml, resolution, audio, keybinds
    class_loader.rs     # Charge les TOML de classes (7 classes)
    skill_loader.rs     # Charge les TOML de skills (210 skills)
    monster_loader.rs   # Charge les TOML monstres (catalogue complet)
    item_loader.rs      # Charge les TOML items (bases, affixes, uniques, sets, runewords)
    zone_loader.rs      # Charge les TOML zones (5 actes, 100+ zones)
    quest_loader.rs     # Charge les TOML quetes (27 quetes)
    loot_loader.rs      # Charge les loot tables
    scene/
      main_menu.rs      # Ecran titre, selection personnage
      character_select.rs  # Creation/selection personnage
      game_scene.rs     # Scene de jeu principale
      loading.rs        # Ecran de chargement
    systems/
      mod.rs            # Registration de tous les systemes Sodomight
      difficulty.rs     # Gestion Normal/Nightmare/Hell
      shrine.rs         # Sanctuaires interactables
      waypoint.rs       # Systeme de waypoints
      town_portal.rs    # Portails de ville
      cube.rs           # Cube Alchimique (Horadric Cube)
      mercenary.rs      # Systeme de mercenaires
  data/                 # TOML game data
    classes/            # 7 fichiers TOML (1 par classe)
    skills/             # 210 fichiers TOML (30 par classe)
    items/
      bases/            # Normal/Exceptional/Elite bases
      affixes/          # prefixes.toml, suffixes.toml
      uniques/          # 1 fichier par unique majeur
      sets/             # 1 fichier par set complet
      runewords/        # runewords.toml
      runes/            # runes.toml
      gems/             # gems.toml
      charms/           # charms.toml
    monsters/
      act1/ act2/ act3/ act4/ act5/
      affixes/          # champion_affixes.toml
      super_uniques/    # 1 fichier par SU majeur
    zones/
      act1/ act2/ act3/ act4/ act5/
    loot_tables/
      treasure_classes.toml
      bosses.toml
      rune_drops.toml
    quests/
      act1/ act2/ act3/ act4/ act5/
    config/
      difficulty.toml
      breakpoints.toml
      experience.toml
      shrines.toml
  assets/               # Sprites, tilesets, audio
    sprites/
    tiles/
    audio/
    fonts/
  scripts/              # Rhai scripts
    quests/
    triggers/
    dialogues/
```

### 4.2 sodomight-server (binaire serveur dedie Phase 2)

```
games/sodomight-server/
  Cargo.toml
  src/
    main.rs             # Point d'entree serveur
    server.rs           # Accept connections, manage sessions
    session.rs          # Session joueur (auth, personnage)
    world_host.rs       # World ECS autorit. sans rendu
    tick.rs             # Fixed tick 25 Hz
    loot_authority.rs   # Drop generation cote serveur uniquement
    save_authority.rs   # Sauvegarde autorit.
```

### 4.3 sodomight-client (binaire client Phase 2)

```
games/sodomight-client/
  Cargo.toml
  src/
    main.rs             # Point d'entree client
    client.rs           # Connection au serveur, envoi inputs
    prediction.rs       # Client-side prediction (mouvement)
    interpolation.rs    # Interpolation entites distantes
    render_world.rs     # World ECS local pour rendu uniquement
```

---

## 5. Pipeline de demarrage du jeu

### 5.1 Sequence d'initialisation (sodomight-game)

```rust
// @id: sd-init-pipeline @do: reference @role: tech-lead @layer: 3 @human: miyuk

fn main() -> Result<(), SodomightError> {
    // 1. Platform init
    let event_loop = mge_platform::create_event_loop()?;
    let window = mge_platform::create_window(&event_loop, "Sodomight", 800, 600)?;
    let gpu = mge_platform::init_wgpu(&window)?;

    // 2. Asset registry
    let mut assets = mge_asset::AssetRegistry::new();
    assets.load_registry("games/sodomight/assets/registry.toml")?;
    assets.enable_hot_reload()?; // notify watcher

    // 3. ECS World
    let mut world = mge_ecs::World::new();

    // 4. Game data loading (TOML)
    let class_defs = class_loader::load_all("games/sodomight/data/classes/")?;
    let skill_defs = skill_loader::load_all("games/sodomight/data/skills/")?;
    let monster_defs = monster_loader::load_all("games/sodomight/data/monsters/")?;
    let item_defs = item_loader::load_all("games/sodomight/data/items/")?;
    let zone_defs = zone_loader::load_all("games/sodomight/data/zones/")?;
    let quest_defs = quest_loader::load_all("games/sodomight/data/quests/")?;
    let loot_tables = loot_loader::load_all("games/sodomight/data/loot_tables/")?;

    // 5. Insert resources into ECS World
    world.insert_resource(class_defs);
    world.insert_resource(skill_defs);
    world.insert_resource(monster_defs);
    world.insert_resource(item_defs);
    world.insert_resource(zone_defs);
    world.insert_resource(quest_defs);
    world.insert_resource(loot_tables);

    // 6. Audio init
    let audio = mge_audio::AudioManager::new()?;
    world.insert_resource(audio);

    // 7. Renderer init
    let renderer = mge_render::Renderer::new(gpu, &assets)?;
    world.insert_resource(renderer);

    // 8. Script engine init
    let script_engine = mge_script::ScriptEngine::new()?;
    script_engine.load_directory("games/sodomight/scripts/")?;
    world.insert_resource(script_engine);

    // 9. Save system init
    let save_system = mge_save::SaveSystem::open("sodomight.db")?;
    world.insert_resource(save_system);

    // 10. Network init (si multijoueur)
    // let net = mge_net::NetworkManager::new(NetworkMode::ListenServer)?;
    // world.insert_resource(net);

    // 11. Register systems (voir section 6)
    register_all_systems(&mut world);

    // 12. Scene: main menu
    world.insert_resource(SceneState::MainMenu);

    // 13. Game loop
    mge_core::run_game_loop(event_loop, world)?;

    Ok(())
}
```

### 5.2 Ordre de chargement des donnees

```
1. Config globale (resolution, audio volume, keybinds)
2. Breakpoints tables (FCR, FHR, FBR, IAS par classe)
3. Classes definitions (7 classes, stats de base, gains par niveau)
4. Skills definitions (210 skills, formules, synergies)
5. Item bases (armes, armures, tous tiers Normal/Exceptional/Elite)
6. Affixes (prefixes, suffixes, tables completes)
7. Unique items (stats fixes, ilvl drop min)
8. Set items (16 sets normaux + 18 sets LoD, bonus partiels/complets)
9. Runewords (33 runes, 60+ runewords, sequences, types requis)
10. Runes (33 runes, effets par slot, recettes upgrade)
11. Gems (7 types, 5 qualites, effets par slot)
12. Monster bases (catalogue par acte, stats N/NM/H)
13. Champion affixes (~34 affixes, effets, combinaisons)
14. Super Uniques (~60 SU, affixes fixes, zones)
15. Act Bosses (5 bosses, phases, attaques, resistances par difficulte)
16. AI archetypes (43 comportements, parametres, state machines)
17. Zones (100+ zones, alvl par difficulte, connexions, waypoints)
18. Loot tables (treasure classes, NoDrop, boss tables)
19. Quetes (27 quetes, 5 actes, objectifs, recompenses, triggers)
20. Cube recipes (gemmes, runes, sockets, crafting, upgrade tier)
21. Shrines (15 types, effets, durees)
22. Difficulty modifiers (resistances penalty, immunities Hell)
23. Experience tables (niveaux 1-99, XP par niveau, penalites)
24. Scripts Rhai (quetes, triggers, dialogues)
```

---

## 6. Boucle de jeu principale

### 6.1 Architecture de la game loop

```rust
// @id: sd-game-loop @do: reference @role: tech-lead @layer: 3 @human: miyuk

pub fn run_game_loop(
    event_loop: EventLoop<()>,
    mut world: World,
) -> Result<(), GameError> {
    let mut accumulator = Duration::ZERO;
    let fixed_dt = Duration::from_millis(40); // 25 Hz fixed update (D2 standard)
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => {
                // Input handling -> InputState resource
                world.resource_mut::<InputState>().process_event(&event);
            }
            Event::MainEventsCleared => {
                let now = Instant::now();
                let frame_dt = now - last_frame;
                last_frame = now;
                accumulator += frame_dt;

                // === PRE-UPDATE STAGE ===
                world.run_stage(Stage::PreUpdate);

                // === FIXED UPDATE (25 Hz) ===
                while accumulator >= fixed_dt {
                    world.run_stage(Stage::FixedUpdate);
                    accumulator -= fixed_dt;
                }

                // === UPDATE STAGE (variable) ===
                world.run_stage(Stage::Update);

                // === POST-UPDATE STAGE ===
                world.run_stage(Stage::PostUpdate);

                // === NETWORK STAGE ===
                world.run_stage(Stage::Network);

                // === RENDER STAGE ===
                world.run_stage(Stage::Render);

                // === AUDIO STAGE ===
                world.run_stage(Stage::Audio);
            }
            _ => {}
        }
    });
}
```

### 6.2 Stages et systemes par stage

| Stage | Frequence | Systemes | Description |
|-------|-----------|----------|-------------|
| **PreUpdate** | Variable (60 Hz+) | InputSystem, HotReloadSystem, SceneTransition | Traitement inputs, hot-reload assets |
| **FixedUpdate** | 25 Hz fixe | CombatSystem, AISystem, PathfindingSystem, SkillSystem, LootSystem, QuestSystem, StatusEffectSystem, MercenarySystem, ShrineSystem, WaypointSystem, PortalSystem, CubeSystem, TradeSystem | Logique de jeu deterministe |
| **Update** | Variable | AnimationSystem, ProjectileMovement, ParticleSystem, CameraSystem | Interpolation, animation, camera |
| **PostUpdate** | Variable | CollisionResolution, DamageNumberSpawn, DeathCleanup | Nettoyage, resolution finale |
| **Network** | Variable | NetworkSend, NetworkReceive, StateSynchronization | Envoi/reception messages reseau |
| **Render** | Variable | ZOrderSort, SpriteBatch, TilemapRender, UIRender, PostProcess | Rendu GPU |
| **Audio** | Variable | AudioTrigger, MusicTransition, AmbientUpdate | Son et musique |

### 6.3 Tick rate et timing

| Parametre | Valeur | Justification |
|-----------|--------|---------------|
| Fixed tick rate | 25 Hz (40ms) | Standard D2, suffisant pour ARPG |
| Render target | 60 Hz minimum (VSync) | Fluidite visuelle |
| Network tick | 25 Hz (synchronise au fixed tick) | Coherence logique |
| Input poll | Chaque frame (variable) | Reactivite maximale |
| AI update | 10 Hz (tous les 2.5 fixed ticks) | Performance IA |
| Pathfinding budget | 5ms max par frame | Eviter stutter |

---

## 7. Separation client/serveur

### 7.1 Phase 1 : Listen Server (MVP)

En Phase 1, le joueur hote execute a la fois la logique de jeu et le rendu. Les clients distants envoient leurs inputs et recoivent l'etat autorit.

```
Host (Joueur 1) :
  - World ECS complet (toutes les entites)
  - Logique de jeu autorit. (combat, loot, AI, quetes)
  - Rendu local
  - Accept connexions TCP des clients

Client (Joueurs 2-8) :
  - World ECS local (copie partielle pour rendu)
  - Envoi inputs uniquement
  - Reception snapshots d'etat
  - Rendu local avec interpolation
```

### 7.2 Donnees cote serveur (autoritaires)

| Donnee | Localisation | Justification |
|--------|-------------|---------------|
| HP/Mana de tous les personnages | Host uniquement | Anti-triche |
| Inventaire de chaque joueur | Host uniquement | Prevention duplication |
| Etat des monstres (HP, position, AI) | Host uniquement | Coherence multi-joueurs |
| Loot generation (rolls, affixes) | Host uniquement | Equite de loot |
| Quest flags | Host uniquement | Prevention de progression illegitime |
| Gold | Host uniquement | Prevention duplication |
| Durabilite des items | Host uniquement | Coherence |

### 7.3 Donnees cote client (cosmetiques/predictives)

| Donnee | Localisation | Justification |
|--------|-------------|---------------|
| Position du joueur local (prediction) | Client | Reactivite |
| Animations et sprites | Client | Rendu local |
| Particules et effets visuels | Client | Cosmetique |
| Sons et musique | Client | Audio local |
| UI state (fenetres ouvertes, drag) | Client | Interaction locale |
| Camera position | Client | Smooth |

### 7.4 Phase 2 : Serveur dedie

Migration Listen Server vers serveur dedie COG :

```
sodomight-server (binaire) :
  - mge-ecs + tous mge-arpg-* (logique sans rendu)
  - mge-net (accept connexions)
  - mge-save (KindMother authoritative)
  - Pas de mge-render, mge-audio, mge-ui
  - Tick 25 Hz pur logique

sodomight-client (binaire) :
  - mge-ecs (monde local pour rendu)
  - mge-render + mge-audio + mge-ui
  - mge-net (connect au serveur)
  - Client-side prediction
  - Interpolation entites distantes
```

---

## 8. Architecture reseau

### 8.1 Protocole

| Propriete | Valeur |
|-----------|--------|
| Transport | TCP (suffisant pour ARPG, pas de FPS) |
| Serialisation | `bincode` (compact, rapide) |
| Max joueurs | 8 par partie (standard D2) |
| Modele | Client-serveur autoritaire (host = serveur MVP) |
| Loot | Partage dans le monde, timer priorite 30s pour le killer |
| Trade | P2P direct entre joueurs presents |

### 8.2 Messages Client vers Host

```rust
// @id: sd-net-client-msg @do: reference @role: tech-lead @layer: 3 @human: miyuk

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ClientMessage {
    // Mouvement
    MoveToPosition { target: Vec2 },
    MoveToEntity { entity_id: EntityId },
    StopMoving,

    // Combat
    UseSkill { skill_id: SkillId, target: SkillTarget },
    NormalAttack { target: EntityId },
    SwapWeaponSet,

    // Items
    PickupItem { entity_id: EntityId },
    DropItem { item_id: ItemId, position: Vec2 },
    EquipItem { item_id: ItemId, slot: EquipSlot },
    UnequipItem { slot: EquipSlot },
    UsePotion { belt_slot: u8 },
    IdentifyItem { item_id: ItemId },
    MoveItem { item_id: ItemId, container: Container, slot: SlotCoord },

    // Trade
    OpenTrade { target_player: PlayerId },
    AddToTrade { item_id: ItemId },
    RemoveFromTrade { item_id: ItemId },
    SetTradeGold { amount: u32 },
    AcceptTrade,
    CancelTrade,

    // Cube
    TransmuteCube,

    // World
    UseWaypoint { zone_id: ZoneId },
    EnterPortal { portal_id: EntityId },
    CastTownPortal,
    InteractNpc { npc_id: EntityId },

    // Quests
    AcceptQuest { quest_id: QuestId },
    CompleteQuest { quest_id: QuestId },

    // Chat
    ChatMessage { channel: ChatChannel, text: String },
}
```

### 8.3 Messages Host vers Clients

```rust
// @id: sd-net-server-msg @do: reference @role: tech-lead @layer: 3 @human: miyuk

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServerMessage {
    // Entites
    EntitySpawned { entity: EntitySnapshot },
    EntityDespawned { id: EntityId },
    EntityMoved { id: EntityId, pos: Vec2, vel: Vec2 },
    EntityDied { id: EntityId, killer: Option<EntityId> },
    EntityHealthChanged { id: EntityId, current: f32, max: f32 },
    EntityManaChanged { id: EntityId, current: f32, max: f32 },
    EntityAnimationChanged { id: EntityId, anim: AnimationId },

    // Combat
    DamageDealt { source: EntityId, target: EntityId, amount: f32, dtype: DamageType },
    SkillActivated { source: EntityId, skill: SkillId, target: SkillTarget },
    ProjectileSpawned { projectile: ProjectileSnapshot },
    StatusApplied { target: EntityId, status: StatusType, duration: f32 },
    StatusRemoved { target: EntityId, status: StatusType },

    // Items
    ItemDropped { item: ItemSnapshot, pos: Vec2, priority_player: Option<PlayerId>, timer: f32 },
    ItemPickedUp { item_id: ItemId, player: PlayerId },
    ItemEquipped { player: PlayerId, slot: EquipSlot, item: ItemSnapshot },
    ItemUnequipped { player: PlayerId, slot: EquipSlot },
    InventoryUpdate { player: PlayerId, items: Vec<InventorySlot> },

    // Player
    StatChanged { player: PlayerId, stat: StatType, value: i32 },
    ExperienceGained { player: PlayerId, amount: u64, current: u64 },
    LevelUp { player: PlayerId, new_level: u32 },
    PlayerDied { player: PlayerId, hardcore: bool },
    GoldChanged { player: PlayerId, amount: u32 },

    // World
    ZoneLoaded { zone: ZoneSnapshot },
    WaypointActivated { player: PlayerId, zone_id: ZoneId },
    PortalSpawned { pos: Vec2, owner: PlayerId },
    ShrineActivated { shrine_type: ShrineType, player: PlayerId },

    // Quest
    QuestUpdated { player: PlayerId, quest_id: QuestId, state: QuestState },
    QuestCompleted { player: PlayerId, quest_id: QuestId, rewards: Vec<Reward> },

    // Network
    PlayerJoined { player: PlayerSnapshot },
    PlayerLeft { player: PlayerId },
    FullStateSync { state: WorldSnapshot },
}
```

### 8.4 Delta compression

Pour optimiser la bande passante, seuls les changements sont envoyes :

```rust
// @id: sd-net-delta @do: reference @role: tech-lead @layer: 3 @human: miyuk

pub struct DeltaCompressor {
    last_sent: HashMap<EntityId, EntitySnapshot>,
}

impl DeltaCompressor {
    pub fn compute_delta(&mut self, current: &WorldSnapshot) -> Vec<ServerMessage> {
        let mut messages = Vec::new();
        for (id, entity) in &current.entities {
            match self.last_sent.get(id) {
                Some(prev) => {
                    if prev.position != entity.position {
                        messages.push(ServerMessage::EntityMoved {
                            id: *id,
                            pos: entity.position,
                            vel: entity.velocity,
                        });
                    }
                    if prev.health != entity.health {
                        messages.push(ServerMessage::EntityHealthChanged {
                            id: *id,
                            current: entity.health,
                            max: entity.max_health,
                        });
                    }
                    // ... autres champs
                }
                None => {
                    messages.push(ServerMessage::EntitySpawned {
                        entity: entity.clone(),
                    });
                }
            }
            self.last_sent.insert(*id, entity.clone());
        }
        messages
    }
}
```

---

## 9. Plan de developpement par sprint

### Sprint 0 -- Fondations moteur (4 semaines)

**Objectif :** Fenetre + tile iso + sprite anime + pathfinding. Rien de gameplay.

| Tache | Crate | Assignation | Priorite |
|-------|-------|-------------|----------|
| Fenetre winit, device wgpu, input events | `mge-platform` | Francois | CRITIQUE |
| Vec2, IVec2, Rect, fonctions dimetric iso | `mge-math` | Francois | CRITIQUE |
| Sprite batcher basique, tilemap iso | `mge-render` | Francois | CRITIQUE |
| World, archetype basique, query, system stages | `mge-ecs` | Francois | CRITIQUE |
| PNG loader, registry TOML, hot-reload notify | `mge-asset` | Francois | HAUTE |
| A* tile-based | `mge-pathfinding` | Francois | HAUTE |
| atlas-viewer, anim-preview | `mge-studio` | Lise | HAUTE |
| Scene de test : map herbe dimetric + mouvement au clic | Integration | Francois + Lise | HAUTE |

**Livrable :** Cliquer sur une map iso -> une entite se deplace via pathfinding.

### Sprint 1 -- Boucle de combat (6 semaines)

**Objectif :** 1 classe jouable (Necromancer), combat fonctionnel, loot basique.

| Tache | Crate | Assignation |
|-------|-------|-------------|
| Character, Monster, Item archetypes | `mge-arpg-entity` | Francois |
| Attributs, calcul life/mana, formules D2 | `mge-arpg-stats` | Francois |
| Auto-attack, 3 skills Necro | `mge-arpg-combat` | Francois |
| IA monstre basique (follow + attack) | `mge-arpg-ai` | Francois |
| Normal/Magic items, affixes simples | `mge-arpg-items` | Francois |
| Drop table Act 1 basique | `mge-arpg-loot` | Francois |
| Barre vie/mana, belt, hotkeys skills | `mge-ui` | Lise |
| Tiles walkable + hitboxes circulaires | `mge-collision` | Francois |
| Integration kira, sons de combat basiques | `mge-audio` | Lise |
| Sauvegarde/chargement personnage KindMother | `mge-save` | Francois |

**Livrable :** Necromancer dans une arene, kill monstres, loot items, sauvegarder.

### Sprint 2 -- Progression et inventaire (6 semaines)

**Objectif :** Systeme de progression D2 complet, inventaire grille, arbres de competences.

| Tache | Crate | Assignation |
|-------|-------|-------------|
| Arbre complet Necromancer (3 arbres, synergies) | `mge-arpg-skills` | Francois |
| Rare/Unique items, sockets, identification | `mge-arpg-items` | Francois |
| Inventaire grille 10x4 drag-and-drop, paperdoll, fenetre skills | `mge-ui` | Lise |
| Breakpoints FCR/FHR/FBR/IAS (tables completes D2) | `mge-arpg-stats` | Francois |
| Rhai integre, premiers scripts de quete | `mge-script` | Francois |
| Edition TOML items et skills | `mge-studio/data-editor` | Lise |

**Livrable :** Progression complete Necromancer, gestion inventaire, arbre de skills.

### Sprint 3 -- Acte 1 complet (8 semaines)

**Objectif :** Sodomight Acte 1 jouable du debut a Andariel.

| Tache | Crate | Assignation |
|-------|-------|-------------|
| Zones Act 1 completes (Blood Moor -> Catacombs) | `mge-arpg-world` | Francois |
| 6 quetes Act 1, flags, recompenses | `mge-arpg-quest` | Francois |
| Waypoints Act 1, portails en ville | `mge-arpg-world` | Francois |
| Tous les monstres Act 1 | TOML data | Francois |
| Boss Andariel (phases, immunites NM/Hell) | `mge-arpg-ai` + TOML | Francois |
| 15 items Uniques Act 1, 2-3 runewords low-tier | TOML data | Francois |
| Filtre de loot (configurable) | `mge-ui` | Lise |
| Audio : musique Act 1, ambiances par zone | `mge-audio` | Lise |
| map-inspector : inspecter maps LDtk Act 1 | `mge-studio` | Lise |

**Livrable :** Acte 1 jouable solo du debut a la fin avec boss.

### Sprint 4 -- Multijoueur (6 semaines)

**Objectif :** 2-8 joueurs en reseau (Listen Server).

| Tache | Crate | Assignation |
|-------|-------|-------------|
| Listen server TCP, messages Client/Server | `mge-net` | Francois |
| Synchronisation entites (positions, combat, loot) | `mge-net` | Francois |
| Loot partage monde (timer priorite 30s) | `mge-arpg-loot` | Francois |
| Trade P2P basique (fenetre de trade) | `mge-arpg-trade` + `mge-ui` | Lise + Francois |
| Scaling HP monstres par nombre de joueurs | `mge-arpg-stats` | Francois |
| Parties nommees, mot de passe optionnel | `mge-net` | Francois |

**Livrable :** 2-4 joueurs en LAN jouent Acte 1 ensemble.

### Sprints 5+ -- Contenu etendu (timeline variable)

| Priorite | Contenu |
|----------|---------|
| Sprint 5 | Classes 2-3 (Sorceress, Barbarian) + Acte 2 |
| Sprint 6 | Classes 4-5 (Paladin, Amazon) + Acte 3 |
| Sprint 7 | Classes 6-7 (Druid, Assassin) + Acte 4 |
| Sprint 8 | Acte 5 + 3 difficultes (Normal/Nightmare/Hell) |
| Sprint 9 | Set items complets, runewords rares, Cube Alchimique |
| Sprint 10 | Uber bosses, Hardcore mode, Stash multi-pages |
| Sprint 11 | Serveur dedie `sodomight-server` |
| Sprint 12 | Polish, optimisation, tests de charge |

---

## 10. Invariants de securite et qualite

### 10.1 Invariants de code

| Invariant | Verification |
|-----------|-------------|
| `unsafe_code = "forbid"` | Cargo.toml de chaque crate, CI check |
| Pas de `unwrap()` en production | Clippy lint, code review |
| Pas de `panic!()` en production | Clippy lint, code review |
| Types d'erreur explicites par module | Code review |
| UUIDs v4 pour les IDs primaires | Type alias enforce |
| ISO 8601 pour les timestamps | Type alias enforce |
| Annotations MSCM obligatoires | CI check |

### 10.2 Invariants de securite

| Invariant | Mesure |
|-----------|--------|
| Loot generation cote serveur uniquement | Architecture reseau |
| Inventaire valide uniquement cote serveur | Validation a chaque modification |
| Pas de donnees sensibles en clair | Chiffrement des saves |
| Pas d'execution de code distant | Rhai sandboxe, API safe |
| Rate limiting sur les messages reseau | mge-net validation |
| Validation des inputs client | Serveur refuse les inputs invalides |
| Gold et items impossibles a dupliquer | Serveur autoritaire |

### 10.3 Invariants de gameplay (fidelite D2)

| Invariant | Source |
|-----------|--------|
| 7 classes avec 30 skills chacune | SD-Classes-Skills.md |
| 33 runes dans l'ordre exact | SD-Items-Itemization.md |
| Breakpoints FCR/FHR/FBR/IAS par classe | SD-Combat-Stats.md |
| Magic Find diminishing returns | SD-Items-Itemization.md |
| Loot partage dans le monde (pas instancie) | MGE-Design-Document.md |
| 5 actes, 3 difficultes | SD-World-Quests.md |
| 43 archetypes IA | SD-Monsters-AI.md |
| HP scaling multi-joueurs : HP * (N+1)/2 | SD-Monsters-AI.md |

### 10.4 Tests obligatoires

```bash
cargo test --workspace              # Tous les tests
cargo clippy --workspace -- -D warnings  # Lint complet
cargo build --workspace             # Build complet
```

| Type de test | Couverture |
|-------------|-----------|
| Unitaire | Chaque formule de combat, chaque calcul de stats |
| Integration | Pipeline de loot complet (monster -> drop -> generation item) |
| Regression | Breakpoints D2 (valeurs exactes verifiees contre reference) |
| Fuzz | Inputs reseau (messages malformes) |
| Performance | Tick rate 25 Hz avec 500 entites |

---

*Document redige par Denis, Chef Dev Senior -- Miyukini AI Studio*
*Revision : 2026-02-28 v1.0*
