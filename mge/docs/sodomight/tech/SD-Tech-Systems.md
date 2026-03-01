<!-- @id: SD-Tech-Systems @do: reference @role: tech-lead @layer: 3 @human: miyuk -->

# SD-Tech-Systems -- Systemes ECS Sodomight

**Auteur :** Denis (Chef Dev Senior, Miyukini AI Studio)
**Date :** 2026-02-28
**Statut :** Reference technique -- v1.0
**Projet :** Sodomight (clone fidele Diablo 2 LoD, assets maison)
**Moteur :** MGE (Miyukini Game Engine) -- ECS archetype maison

---

## Table des matieres

1. [Pipeline de systemes par phase](#1-pipeline-de-systemes-par-phase)
2. [Ordre d'execution et dependances](#2-ordre-dexecution-et-dependances)
3. [Systemes de mouvement](#3-systemes-de-mouvement)
4. [Systemes de combat](#4-systemes-de-combat)
5. [Systemes IA](#5-systemes-ia)
6. [Systemes d'items](#6-systemes-ditems)
7. [Systemes de progression](#7-systemes-de-progression)
8. [Systemes reseau](#8-systemes-reseau)
9. [Systemes UI](#9-systemes-ui)
10. [Systemes audio](#10-systemes-audio)
11. [Pipeline de combat complet -- Implementation Rust](#11-pipeline-de-combat-complet--implementation-rust)
12. [Invariants et regles des systemes](#12-invariants-et-regles-des-systemes)

---

## 1. Pipeline de systemes par phase

### 1.1 Vue d'ensemble des stages

La game loop MGE execute les systemes en 7 stages sequentiels. Chaque stage regroupe
des systemes qui partagent la meme semantique temporelle. Aucun systeme ne peut
s'executer dans un stage autre que celui auquel il est enregistre.

```
Frame N:
  PreUpdate    (variable, 60 Hz+)   -- Input, hot-reload, transitions de scene
  FixedUpdate  (25 Hz, dt = 40ms)   -- Logique de jeu deterministe
  Update       (variable, 60 Hz+)   -- Animation, interpolation, camera
  PostUpdate   (variable)            -- Resolution collision, nettoyage, mort
  Network      (variable)            -- Envoi/reception messages reseau
  Render       (variable, 60 Hz+)   -- Tri Z, batching sprites, rendu GPU
  Audio        (variable)            -- Declenchement sons, crossfade musique
```

### 1.2 Frequences d'execution

| Stage | Frequence | dt nominal | Justification |
|-------|-----------|-----------|---------------|
| PreUpdate | Variable (chaque frame rendu) | frame_dt variable | Reactivite input maximale |
| FixedUpdate | 25 Hz fixe | 40 ms | Standard D2, determinisme game logic |
| Update | Variable (chaque frame rendu) | frame_dt variable | Fluidite visuelle, interpolation |
| PostUpdate | Variable (chaque frame rendu) | frame_dt variable | Nettoyage post-logique |
| Network | Variable (chaque frame rendu) | frame_dt variable | Synchronisation opportuniste |
| Render | Variable (cible 60 Hz VSync) | frame_dt variable | Rendu GPU |
| Audio | Variable (chaque frame rendu) | frame_dt variable | Audio kira gere son propre timing |

### 1.3 Accumulation FixedUpdate

Le FixedUpdate utilise un accumulateur temporel. Plusieurs ticks peuvent s'executer
par frame si le rendu est en retard. Un cap de 4 ticks maximum par frame empeche
les spirales de mort.

```rust
// @id: sd-sys-accumulator @do: reference @role: tech-lead @layer: 3 @human: miyuk

const FIXED_DT: Duration = Duration::from_millis(40); // 25 Hz
const MAX_TICKS_PER_FRAME: u32 = 4;

pub fn tick_accumulator(accumulator: &mut Duration, frame_dt: Duration, world: &mut World) {
    *accumulator += frame_dt;
    let mut ticks = 0u32;
    while *accumulator >= FIXED_DT && ticks < MAX_TICKS_PER_FRAME {
        world.run_stage(Stage::FixedUpdate);
        *accumulator -= FIXED_DT;
        ticks += 1;
    }
    // Si on depasse le cap, on perd du temps de simulation (evite la spirale de mort).
    if ticks == MAX_TICKS_PER_FRAME && *accumulator > FIXED_DT {
        *accumulator = Duration::ZERO;
    }
}
```

---

## 2. Ordre d'execution et dependances

### 2.1 Tableau complet des systemes par stage

#### Stage PreUpdate

| # | Systeme | Crate | Description |
|---|---------|-------|-------------|
| 1 | `PlayerInputSystem` | `sodomight-game` | Traduit inputs clavier/souris en intentions |
| 2 | `HotReloadSystem` | `mge-asset` | Detecte fichiers modifies, recharge assets |
| 3 | `SceneTransitionSystem` | `sodomight-game` | Gere transitions MainMenu/Loading/Game |

#### Stage FixedUpdate (25 Hz -- ordre strict)

| # | Systeme | Crate | Dependances (doit s'executer apres) |
|---|---------|-------|-------------------------------------|
| 1 | `BuffSystem` | `mge-arpg-combat` | Aucune |
| 2 | `AuraSystem` | `mge-arpg-combat` | BuffSystem |
| 3 | `MonsterSpawnSystem` | `mge-arpg-ai` | Aucune |
| 4 | `MonsterRespawnSystem` | `mge-arpg-ai` | MonsterSpawnSystem |
| 5 | `AggroSystem` | `mge-arpg-ai` | Aucune |
| 6 | `AIBehaviorSystem` | `mge-arpg-ai` | AggroSystem |
| 7 | `AIPathUpdateSystem` | `mge-arpg-ai` | AIBehaviorSystem |
| 8 | `PathfindingSystem` | `mge-pathfinding` | AIPathUpdateSystem |
| 9 | `MovementSystem` | `mge-arpg-entity` | PathfindingSystem |
| 10 | `CollisionSystem` | `mge-collision` | MovementSystem |
| 11 | `SkillActivationSystem` | `mge-arpg-skills` | Aucune |
| 12 | `AttackSystem` | `mge-arpg-combat` | SkillActivationSystem, MovementSystem |
| 13 | `SkillProjectileSystem` | `mge-arpg-combat` | AttackSystem |
| 14 | `DamageApplicationSystem` | `mge-arpg-combat` | AttackSystem, SkillProjectileSystem |
| 15 | `HitRecoverySystem` | `mge-arpg-combat` | DamageApplicationSystem |
| 16 | `DeathSystem` | `mge-arpg-combat` | DamageApplicationSystem |
| 17 | `LootGenerationSystem` | `mge-arpg-loot` | DeathSystem |
| 18 | `ItemDropSystem` | `mge-arpg-loot` | LootGenerationSystem |
| 19 | `ItemPickupSystem` | `mge-arpg-items` | ItemDropSystem |
| 20 | `ItemEquipSystem` | `mge-arpg-items` | ItemPickupSystem |
| 21 | `InventorySystem` | `mge-arpg-items` | ItemEquipSystem |
| 22 | `StashSystem` | `mge-arpg-items` | InventorySystem |
| 23 | `CubeSystem` | `sodomight-game` | InventorySystem |
| 24 | `XPSystem` | `mge-arpg-stats` | DeathSystem |
| 25 | `SkillProgressionSystem` | `mge-arpg-skills` | XPSystem |
| 26 | `QuestSystem` | `mge-arpg-quest` | DeathSystem, ItemPickupSystem |
| 27 | `WaypointSystem` | `sodomight-game` | MovementSystem |
| 28 | `DifficultySystem` | `sodomight-game` | Aucune |

#### Stage Update (variable)

| # | Systeme | Crate | Description |
|---|---------|-------|-------------|
| 1 | `AnimationSystem` | `mge-render` | Avance les frames d'animation |
| 2 | `ProjectileMovementSystem` | `mge-arpg-combat` | Deplace projectiles (interpolation) |
| 3 | `ParticleSystem` | `mge-render` | Met a jour les particules |
| 4 | `CameraSystem` | `mge-render` | Suit le joueur, smooth |

#### Stage PostUpdate (variable)

| # | Systeme | Crate | Description |
|---|---------|-------|-------------|
| 1 | `CollisionResolutionSystem` | `mge-collision` | Resolution finale des collisions |
| 2 | `DamageNumberSpawnSystem` | `mge-ui` | Spawn des nombres de degats flottants |
| 3 | `DeathCleanupSystem` | `mge-arpg-entity` | Supprime les entites mortes apres animation |

#### Stage Network (variable)

| # | Systeme | Crate | Description |
|---|---------|-------|-------------|
| 1 | `NetworkSyncSystem` | `mge-net` | Envoi delta state aux clients |
| 2 | `PlayerSessionSystem` | `mge-net` | Connexion/deconnexion joueurs |
| 3 | `AuthoritativeStateSystem` | `mge-net` | Validation etat autoritaire |

#### Stage Render (variable)

| # | Systeme | Crate | Description |
|---|---------|-------|-------------|
| 1 | `ZOrderSortSystem` | `mge-render` | Tri des sprites par profondeur iso |
| 2 | `SpriteBatchSystem` | `mge-render` | Groupement par atlas, draw calls |
| 3 | `TilemapRenderSystem` | `mge-render` | Rendu de la tilemap iso |
| 4 | `UIRenderSystem` | `mge-ui` | Rendu HUD, inventaire, tooltips |
| 5 | `HUDUpdateSystem` | `mge-ui` | Mise a jour orbes HP/Mana, belt |
| 6 | `TooltipSystem` | `mge-ui` | Generation tooltips items |
| 7 | `MiniMapSystem` | `mge-ui` | Rendu automap |

#### Stage Audio (variable)

| # | Systeme | Crate | Description |
|---|---------|-------|-------------|
| 1 | `AudioTriggerSystem` | `mge-audio` | Declenche sons par evenements |
| 2 | `MusicSystem` | `mge-audio` | Crossfade musique par zone |

### 2.2 Graphe de dependances (FixedUpdate)

```
BuffSystem ──────────────────────────────────────────────────────────────
  │
AuraSystem ──────────────────────────────────────────────────────────────
  │
MonsterSpawnSystem ──> MonsterRespawnSystem
  │
AggroSystem ──> AIBehaviorSystem ──> AIPathUpdateSystem ──> PathfindingSystem
                                                               │
                                          MovementSystem <─────┘
                                              │
                                        CollisionSystem
                                              │
SkillActivationSystem ──> AttackSystem ──> SkillProjectileSystem
                              │                   │
                              └───> DamageApplicationSystem <──┘
                                         │           │
                                  HitRecoverySystem  DeathSystem
                                                        │
                                              ┌─────────┼──────────┐
                                        LootGeneration  XPSystem  QuestSystem
                                              │            │
                                         ItemDropSystem  SkillProgressionSystem
                                              │
                                        ItemPickupSystem
                                              │
                                        ItemEquipSystem
                                              │
                                        InventorySystem
                                           │      │
                                     StashSystem  CubeSystem
```

---

## 3. Systemes de mouvement

### 3.1 PlayerInputSystem

**Stage :** PreUpdate (variable)
**Crate :** `sodomight-game`
**Query :** `(&mut MoveTarget, &CharacterInfo, &Locomotion, &Position, Option<&UiState>)`
**Ressources :** `Res<InputState>, Res<Camera>`

Traduit les inputs bruts (clavier, souris) en intentions de mouvement et d'action
sur l'entite du joueur local.

```rust
// @id: sd-sys-player-input @do: define @role: arpg @layer: 3 @human: miyuk

pub fn player_input_system(
    input: Res<InputState>,
    camera: Res<Camera>,
    mut query: Query<(
        &mut MoveTarget,
        &CharacterInfo,
        &Locomotion,
        &Position,
        Option<&UiState>,
    ), With<LocalPlayer>>,
) {
    for (mut move_target, _char_info, locomotion, _pos, ui_state) in query.iter_mut() {
        // Ne pas traiter les inputs si une fenetre UI est ouverte et consomme le clic.
        if let Some(ui) = ui_state {
            if ui.inventory_open || ui.skill_tree_open || ui.stash_open || ui.trade_open {
                continue;
            }
        }

        // Ne pas bouger si l'entite est en hit recovery, cast, ou morte.
        if matches!(
            locomotion.state,
            LocomotionState::HitRecovery
                | LocomotionState::BlockRecovery
                | LocomotionState::Dead
        ) {
            continue;
        }

        // Clic gauche : deplacement vers la position monde.
        if input.left_click_pressed {
            let world_pos = camera.screen_to_world(input.mouse_x, input.mouse_y);
            *move_target = MoveTarget {
                x: world_pos.x,
                y: world_pos.y,
            };
        }
    }
}
```

### 3.2 MovementSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-entity`
**Query :** `(&mut Position, &Velocity, &MovementSpeed, &mut Locomotion, &mut Facing, Option<&PathPlan>)`
**dt :** 40 ms (fixe)

Applique la velocite aux positions en tenant compte de la vitesse de marche/course
et de l'etat de locomotion.

```rust
// @id: sd-sys-movement @do: define @role: arpg @layer: 3 @human: miyuk

const FIXED_DT_SECS: f32 = 0.04; // 40ms = 1/25

pub fn movement_system(
    mut query: Query<(
        &mut Position,
        &Velocity,
        &MovementSpeed,
        &mut Locomotion,
        &mut Facing,
        Option<&PathPlan>,
        Option<&MoveTarget>,
    )>,
) {
    for (mut pos, vel, speed, mut locomotion, mut facing, path, target) in query.iter_mut() {
        // Pas de mouvement si l'entite est immobilisee.
        if matches!(
            locomotion.state,
            LocomotionState::HitRecovery
                | LocomotionState::BlockRecovery
                | LocomotionState::Casting
                | LocomotionState::Attacking
                | LocomotionState::Dead
        ) {
            continue;
        }

        // Determiner la vitesse effective.
        let base_speed = match locomotion.state {
            LocomotionState::Running => speed.run_speed,
            LocomotionState::Walking => speed.walk_speed,
            _ => 0.0,
        };
        let effective_speed = base_speed * speed.speed_modifier;

        // Si un chemin est disponible, suivre le prochain waypoint.
        if let Some(path_plan) = path {
            if path_plan.current_index < path_plan.waypoints.len() {
                let (wx, wy) = path_plan.waypoints[path_plan.current_index];
                let dx = wx as f32 - pos.x;
                let dy = wy as f32 - pos.y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist > 0.05 {
                    let nx = dx / dist;
                    let ny = dy / dist;
                    pos.x += nx * effective_speed * FIXED_DT_SECS;
                    pos.y += ny * effective_speed * FIXED_DT_SECS;

                    // Mise a jour de la direction.
                    facing.direction = direction_from_delta(nx, ny);
                }
            }
        } else if let Some(mt) = target {
            // Deplacement direct vers la cible (sans pathfinding).
            let dx = mt.x - pos.x;
            let dy = mt.y - pos.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > 0.05 {
                let nx = dx / dist;
                let ny = dy / dist;
                pos.x += nx * effective_speed * FIXED_DT_SECS;
                pos.y += ny * effective_speed * FIXED_DT_SECS;
                facing.direction = direction_from_delta(nx, ny);
                locomotion.state = LocomotionState::Walking;
            } else {
                locomotion.state = LocomotionState::Idle;
            }
        } else {
            // Appliquer la velocite brute (pour les entites poussees, knockback...).
            pos.x += vel.dx * FIXED_DT_SECS;
            pos.y += vel.dy * FIXED_DT_SECS;
        }
    }
}

/// Convertit un delta de mouvement en direction 8 voies.
fn direction_from_delta(dx: f32, dy: f32) -> Direction {
    let angle = dy.atan2(dx);
    let octant = ((angle + std::f32::consts::PI) / (std::f32::consts::PI / 4.0)) as i32 % 8;
    match octant {
        0 => Direction::West,
        1 => Direction::NorthWest,
        2 => Direction::North,
        3 => Direction::NorthEast,
        4 => Direction::East,
        5 => Direction::SouthEast,
        6 => Direction::South,
        7 => Direction::SouthWest,
        _ => Direction::South,
    }
}
```

### 3.3 CollisionSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-collision`
**Query :** `(&mut Position, &CircleHitbox, &Velocity)`
**Ressources :** `Res<TileMap>`

Detection et resolution AABB sur grille de tiles. Phase 1 Sodomight utilise des
hitboxes circulaires simplifiees pour les entites, et une grille de walkability
pour les tiles.

```rust
// @id: sd-sys-collision @do: define @role: engine @layer: 2 @human: miyuk

pub fn collision_system(
    tilemap: Res<TileMap>,
    mut query: Query<(&mut Position, &CircleHitbox, &mut Velocity)>,
) {
    for (mut pos, hitbox, mut vel) in query.iter_mut() {
        // 1. Collision tile : verifier que la position cible est walkable.
        let tile_x = pos.x.floor() as i32;
        let tile_y = pos.y.floor() as i32;

        if !tilemap.is_walkable(tile_x, tile_y) {
            // Repousser l'entite vers la derniere position valide.
            pos.x -= vel.dx * FIXED_DT_SECS;
            pos.y -= vel.dy * FIXED_DT_SECS;
            vel.dx = 0.0;
            vel.dy = 0.0;
            continue;
        }

        // 2. Verifier les 8 tiles adjacentes pour le rayon de hitbox.
        for dx_tile in -1..=1 {
            for dy_tile in -1..=1 {
                let check_x = tile_x + dx_tile;
                let check_y = tile_y + dy_tile;
                if !tilemap.is_walkable(check_x, check_y) {
                    // Calculer la distance entre le centre de l'entite et le bord de la tile.
                    let tile_center_x = check_x as f32 + 0.5;
                    let tile_center_y = check_y as f32 + 0.5;
                    let diff_x = pos.x - tile_center_x;
                    let diff_y = pos.y - tile_center_y;
                    let dist = (diff_x * diff_x + diff_y * diff_y).sqrt();

                    if dist < hitbox.radius + 0.5 {
                        // Repousser hors de la tile non-walkable.
                        let overlap = hitbox.radius + 0.5 - dist;
                        if dist > 0.001 {
                            pos.x += (diff_x / dist) * overlap;
                            pos.y += (diff_y / dist) * overlap;
                        }
                    }
                }
            }
        }
    }
}
```

### 3.4 PathfindingSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-pathfinding`
**Query :** `(&Position, &MoveTarget, &mut PathPlan)`
**Ressources :** `Res<TileMap>`
**Budget :** 5 ms maximum par frame (eviter stutter)

A* tile-based isometrique. Heuristique octile pour la grille 8 directions.
Le systeme est budgetise : si le temps depasse 5 ms, les pathfinding restants
sont reportes au tick suivant.

```rust
// @id: sd-sys-pathfinding @do: define @role: engine @layer: 2 @human: miyuk

const PATHFINDING_BUDGET_MS: u128 = 5;

pub fn pathfinding_system(
    tilemap: Res<TileMap>,
    mut query: Query<(&Position, &MoveTarget, &mut PathPlan), Changed<MoveTarget>>,
) {
    let start_time = std::time::Instant::now();

    for (pos, target, mut path_plan) in query.iter_mut() {
        // Verifier le budget temporel.
        if start_time.elapsed().as_millis() > PATHFINDING_BUDGET_MS {
            break; // Reporter au tick suivant.
        }

        let start = (pos.x.floor() as i32, pos.y.floor() as i32);
        let goal = (target.x.floor() as i32, target.y.floor() as i32);

        // A* avec heuristique octile.
        match astar_pathfind(&tilemap, start, goal) {
            Some(waypoints) => {
                path_plan.waypoints = waypoints;
                path_plan.current_index = 0;
                path_plan.target = goal;
            }
            None => {
                // Aucun chemin trouve : vider le plan.
                path_plan.waypoints.clear();
                path_plan.current_index = 0;
            }
        }
    }
}

/// A* avec heuristique octile sur grille isometrique.
fn astar_pathfind(
    tilemap: &TileMap,
    start: (i32, i32),
    goal: (i32, i32),
) -> Option<Vec<(i32, i32)>> {
    use std::collections::BinaryHeap;
    use std::collections::HashMap;

    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut g_score: HashMap<(i32, i32), f32> = HashMap::new();

    g_score.insert(start, 0.0);
    open_set.push(AStarNode {
        pos: start,
        f_score: octile_heuristic(start, goal),
    });

    while let Some(current) = open_set.pop() {
        if current.pos == goal {
            return Some(reconstruct_path(&came_from, current.pos));
        }

        let current_g = g_score.get(&current.pos).copied().unwrap_or(f32::INFINITY);

        // 8 voisins.
        for (dx, dy) in &[
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ] {
            let neighbor = (current.pos.0 + dx, current.pos.1 + dy);

            if !tilemap.is_walkable(neighbor.0, neighbor.1) {
                continue;
            }

            // Cout : 1.0 pour cardinal, 1.414 pour diagonal.
            let cost = if *dx != 0 && *dy != 0 { 1.414 } else { 1.0 };
            let tentative_g = current_g + cost;

            let existing_g = g_score.get(&neighbor).copied().unwrap_or(f32::INFINITY);
            if tentative_g < existing_g {
                came_from.insert(neighbor, current.pos);
                g_score.insert(neighbor, tentative_g);
                open_set.push(AStarNode {
                    pos: neighbor,
                    f_score: tentative_g + octile_heuristic(neighbor, goal),
                });
            }
        }
    }

    None // Aucun chemin.
}

/// Heuristique octile (admissible pour grille 8 directions).
fn octile_heuristic(a: (i32, i32), b: (i32, i32)) -> f32 {
    let dx = (a.0 - b.0).unsigned_abs() as f32;
    let dy = (a.1 - b.1).unsigned_abs() as f32;
    let min = dx.min(dy);
    let max = dx.max(dy);
    min * 1.414 + (max - min)
}
```

---

## 4. Systemes de combat

### 4.1 AttackSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-combat`
**Query :** `(&mut ActiveAttack, &Position, &EquippedWeapon, &CombatBonuses, &Breakpoints, &Facing)`
**Ecrit :** `EventWriter<DamageEvent>`

Gere le cycle de vie d'une attaque (auto-attack ou skill melee). Avance la frame
d'animation, et au hit_frame, effectue le calcul CTH + damage pipeline complet.

```rust
// @id: sd-sys-attack @do: define @role: arpg @layer: 3 @human: miyuk

pub fn attack_system(
    mut damage_events: EventWriter<DamageEvent>,
    mut attacker_query: Query<(
        Entity,
        &mut ActiveAttack,
        &Position,
        &EquippedWeapon,
        &CombatBonuses,
        &ElementalDamage,
        &Breakpoints,
        &CharacterInfo,
        &AttackRating,
    )>,
    defender_query: Query<(
        Entity,
        &Position,
        &Defense,
        &Resistances,
        &VitalPools,
        &CircleHitbox,
        Option<&MonsterData>,
        Option<&BlockChance>,
    )>,
    mut rng: ResMut<GameRng>,
) {
    for (
        attacker_entity,
        mut attack,
        attacker_pos,
        weapon,
        bonuses,
        elem_dmg,
        breakpoints,
        char_info,
        ar,
    ) in attacker_query.iter_mut()
    {
        // Avancer la frame d'animation.
        attack.current_frame += 1;

        // Verifier si l'animation est terminee.
        if attack.current_frame >= attack.total_frames {
            // L'attaque est terminee, sera retiree par le cleanup.
            continue;
        }

        // Verifier si c'est le hit frame et si le hit n'a pas encore ete applique.
        if attack.current_frame != attack.hit_frame || attack.hit_applied {
            continue;
        }
        attack.hit_applied = true;

        // Trouver la cible.
        let target_entity = match attack.target {
            AttackTarget::Entity(id) => id,
            _ => continue, // AoE et direction traitees par SkillProjectileSystem.
        };

        // Recuperer les donnees du defenseur.
        let defender = match defender_query.get(target_entity) {
            Ok(d) => d,
            Err(_) => continue, // Cible disparue.
        };

        let (
            def_entity,
            def_pos,
            def_defense,
            def_resist,
            def_vitals,
            def_hitbox,
            def_monster,
            def_block,
        ) = defender;

        // Verifier la portee.
        let dx = attacker_pos.x - def_pos.x;
        let dy = attacker_pos.y - def_pos.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > weapon.range + def_hitbox.radius {
            continue; // Hors de portee.
        }

        // Calculer le resultat de combat via le pipeline complet.
        let attacker_level = char_info.level;
        let defender_level = def_monster
            .map(|m| m.monster_level)
            .unwrap_or(attacker_level);

        let result = calculate_damage(
            ar.total_ar,
            def_defense.total_defense,
            attacker_level,
            defender_level,
            weapon,
            bonuses,
            elem_dmg,
            def_resist,
            def_vitals,
            def_block,
            def_monster,
            &mut rng.0,
        );

        // Emettre l'evenement de dommage.
        damage_events.send(DamageEvent {
            source: attacker_entity,
            target: def_entity,
            result,
        });
    }
}
```

### 4.2 SkillActivationSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-skills`
**Query :** `(&mut SkillSlots, &mut VitalPools, &Breakpoints, &mut Locomotion, &CharacterInfo)`
**Ressources :** `Res<SkillDefinitions>`

Active un skill quand le joueur ou l'IA le declenche. Verifie le cout en mana,
le cooldown, le niveau du skill, et initie l'animation de cast.

```rust
// @id: sd-sys-skill-activation @do: define @role: arpg @layer: 3 @human: miyuk

pub fn skill_activation_system(
    skill_defs: Res<SkillDefinitions>,
    mut query: Query<(
        Entity,
        &mut SkillSlots,
        &mut VitalPools,
        &Breakpoints,
        &mut Locomotion,
        &CharacterInfo,
    )>,
    mut skill_events: EventWriter<SkillActivatedEvent>,
) {
    for (entity, mut slots, mut vitals, breakpoints, mut locomotion, char_info) in
        query.iter_mut()
    {
        // Verifier chaque slot de skill pour une activation en attente.
        let pending = slots.take_pending_activation();
        for (skill_id, target) in pending {
            // Recuperer la definition du skill.
            let def = match skill_defs.get(&skill_id) {
                Some(d) => d,
                None => continue,
            };

            // Verifier le niveau du skill.
            let skill_level = slots.get_level(&skill_id).unwrap_or(0);
            if skill_level == 0 {
                continue;
            }

            // Verifier le cooldown.
            if slots.is_on_cooldown(&skill_id) {
                continue;
            }

            // Calculer le cout en mana (avec synergies).
            let mana_cost = def.mana_cost_at_level(skill_level);
            if vitals.mana_current < mana_cost as i32 {
                continue; // Pas assez de mana.
            }

            // Verifier que l'entite n'est pas deja en action bloquante.
            if matches!(
                locomotion.state,
                LocomotionState::HitRecovery
                    | LocomotionState::BlockRecovery
                    | LocomotionState::Dead
            ) {
                continue;
            }

            // Depenser la mana.
            vitals.mana_current -= mana_cost as i32;

            // Demarrer le cooldown.
            let cooldown_frames = def.cooldown_at_level(skill_level);
            slots.start_cooldown(&skill_id, cooldown_frames);

            // Mettre l'entite en etat de cast.
            locomotion.state = LocomotionState::Casting;
            locomotion.animation_frames_remaining = breakpoints.cast_frames;

            // Emettre l'evenement.
            skill_events.send(SkillActivatedEvent {
                caster: entity,
                skill_id: skill_id.clone(),
                skill_level,
                target,
                cast_frames: breakpoints.cast_frames,
            });
        }
    }
}
```

### 4.3 SkillProjectileSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-combat`
**Query :** `(&mut Position, &mut ProjectileData, &Velocity, Option<&Homing>, &mut Lifetime)`
**Ecrit :** `EventWriter<DamageEvent>`

Deplace les projectiles magiques, verifie les collisions avec les entites cibles,
et applique les dommages a l'impact.

```rust
// @id: sd-sys-skill-projectile @do: define @role: arpg @layer: 3 @human: miyuk

pub fn skill_projectile_system(
    mut commands: Commands,
    mut damage_events: EventWriter<DamageEvent>,
    mut projectile_query: Query<(
        Entity,
        &mut Position,
        &mut ProjectileData,
        &Velocity,
        Option<&Homing>,
        &mut Lifetime,
    )>,
    target_query: Query<(Entity, &Position, &CircleHitbox, &VitalPools), Without<ProjectileData>>,
) {
    for (proj_entity, mut proj_pos, mut proj_data, vel, homing, mut lifetime) in
        projectile_query.iter_mut()
    {
        // Decrire la duree de vie.
        if lifetime.remaining_frames == 0 {
            commands.despawn(proj_entity);
            continue;
        }
        lifetime.remaining_frames -= 1;

        // Deplacement.
        proj_pos.x += vel.dx * FIXED_DT_SECS;
        proj_pos.y += vel.dy * FIXED_DT_SECS;

        // Si homing, ajuster la direction vers la cible.
        if let Some(homing_data) = homing {
            if let Ok((_, target_pos, _, _)) = target_query.get(homing_data.target) {
                let dx = target_pos.x - proj_pos.x;
                let dy = target_pos.y - proj_pos.y;
                let _dist = (dx * dx + dy * dy).sqrt();
                // Rotation progressive vers la cible (turn_rate radians/sec).
                // Implementation simplifiee : ajuster le vecteur vitesse.
            }
        }

        // Detection de collision avec les entites cibles.
        for (target_entity, target_pos, target_hitbox, target_vitals) in target_query.iter() {
            // Ne pas toucher la source.
            if target_entity == proj_data.source {
                continue;
            }

            // Ne pas toucher une entite deja dans la hit_list.
            if proj_data.hit_list.contains(&target_entity) {
                continue;
            }

            // Ignorer les entites mortes.
            if target_vitals.life_current <= 0 {
                continue;
            }

            // Test de collision circulaire.
            let dx = proj_pos.x - target_pos.x;
            let dy = proj_pos.y - target_pos.y;
            let dist = (dx * dx + dy * dy).sqrt();
            let collision_radius = 0.2 + target_hitbox.radius; // 0.2 = rayon projectile.

            if dist < collision_radius {
                // Impact.
                proj_data.hit_list.push(target_entity);
                proj_data.targets_hit += 1;

                // Emettre evenement de dommage.
                damage_events.send(DamageEvent {
                    source: proj_data.source,
                    target: target_entity,
                    result: DamageResult {
                        physical: rng_range(proj_data.phys_damage.0, proj_data.phys_damage.1),
                        fire: rng_range(proj_data.elem_damage.fire_min, proj_data.elem_damage.fire_max),
                        cold: rng_range(proj_data.elem_damage.cold_min, proj_data.elem_damage.cold_max),
                        lightning: rng_range(
                            proj_data.elem_damage.lightning_min,
                            proj_data.elem_damage.lightning_max,
                        ),
                        poison: proj_data.elem_damage.poison_total,
                        poison_duration: proj_data.elem_damage.poison_duration_frames,
                        magic: rng_range(proj_data.elem_damage.magic_min, proj_data.elem_damage.magic_max),
                        blocked: false,
                        missed: false,
                        critical: false,
                        crushing_blow: false,
                        open_wounds: false,
                    },
                });

                // Si non-piercing ou max pierce atteint, despawn.
                if !proj_data.piercing
                    || (proj_data.pierce_count >= 0
                        && proj_data.targets_hit > proj_data.pierce_count)
                {
                    commands.despawn(proj_entity);
                    break;
                }
            }
        }
    }
}
```

### 4.4 DamageApplicationSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-combat`
**Query :** `(&mut VitalPools, &Resistances)`
**Lit :** `EventReader<DamageEvent>`
**Ecrit :** `EventWriter<EntityDiedEvent>`

Lit les evenements de dommage emis par AttackSystem et SkillProjectileSystem, applique
les resistances, et met a jour les pools de vie.

```rust
// @id: sd-sys-damage-application @do: define @role: arpg @layer: 3 @human: miyuk

pub fn damage_application_system(
    mut damage_events: EventReader<DamageEvent>,
    mut death_events: EventWriter<EntityDiedEvent>,
    mut query: Query<(&mut VitalPools, &Resistances, &ResistanceCaps)>,
) {
    for event in damage_events.iter() {
        if event.result.missed || event.result.blocked {
            continue;
        }

        let (mut vitals, resist, caps) = match query.get_mut(event.target) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Appliquer les resistances a chaque type de dommage.
        let phys = apply_physical_reduction(event.result.physical, resist);
        let fire = apply_elemental_resist(event.result.fire, resist.fire, caps.fire_cap);
        let cold = apply_elemental_resist(event.result.cold, resist.cold, caps.cold_cap);
        let light = apply_elemental_resist(
            event.result.lightning,
            resist.lightning,
            caps.lightning_cap,
        );
        let poison = apply_elemental_resist(
            event.result.poison,
            resist.poison,
            caps.poison_cap,
        );
        let magic = apply_elemental_resist(event.result.magic, resist.magic, 75);

        let total_damage = phys + fire + cold + light + poison + magic;

        // Appliquer les dommages.
        vitals.life_current -= total_damage;

        // Verifier la mort.
        if vitals.life_current <= 0 {
            vitals.life_current = 0;
            death_events.send(EntityDiedEvent {
                entity: event.target,
                killer: Some(event.source),
            });
        }
    }
}

/// Applique la reduction physique (DR% et DR flat).
fn apply_physical_reduction(damage: i32, resist: &Resistances) -> i32 {
    let after_pct = damage - (damage * resist.physical_pct.min(50) / 100);
    let after_flat = (after_pct - resist.physical_flat).max(0);
    after_flat
}

/// Applique une resistance elementaire avec cap.
fn apply_elemental_resist(damage: i32, resist: i32, cap: i32) -> i32 {
    let capped_resist = resist.min(cap);
    let reduced = damage - (damage * capped_resist / 100);
    reduced.max(0)
}
```

### 4.5 DeathSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-combat`
**Lit :** `EventReader<EntityDiedEvent>`
**Ecrit :** `EventWriter<LootSpawnEvent>, EventWriter<XPDistributionEvent>`

Gere la mort d'une entite. Declenche le spawn de loot, la distribution d'XP,
et lance l'animation de mort.

```rust
// @id: sd-sys-death @do: define @role: arpg @layer: 3 @human: miyuk

pub fn death_system(
    mut death_events: EventReader<EntityDiedEvent>,
    mut loot_events: EventWriter<LootSpawnEvent>,
    mut xp_events: EventWriter<XPDistributionEvent>,
    mut query: Query<(
        &mut Locomotion,
        &mut AnimState,
        &Position,
        Option<&MonsterData>,
        Option<&CharacterInfo>,
    )>,
) {
    for event in death_events.iter() {
        let (mut locomotion, mut anim, pos, monster, char_info) = match query.get_mut(event.entity)
        {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Passer en etat Dead.
        locomotion.state = LocomotionState::Dead;
        anim.action = AnimAction::Death;
        anim.current_frame = 0;
        anim.looping = false;
        anim.finished = false;

        // Si c'est un monstre, declencher le loot et l'XP.
        if let Some(monster_data) = monster {
            loot_events.send(LootSpawnEvent {
                position: (pos.x, pos.y),
                loot_table: monster_data.loot_table.clone(),
                monster_level: monster_data.monster_level,
                monster_type: monster_data.monster_type,
                killer: event.killer,
            });

            xp_events.send(XPDistributionEvent {
                monster_experience: monster_data.experience,
                monster_level: monster_data.monster_level,
                killer: event.killer,
                position: (pos.x, pos.y),
            });
        }

        // Si c'est un joueur hardcore, gerer la mort permanente.
        if let Some(char) = char_info {
            if char.hardcore {
                // Le personnage est marque comme mort definitivement.
                // La sauvegarde sera mise a jour par le SaveSystem.
            }
        }
    }
}
```

### 4.6 HitRecoverySystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-combat`
**Query :** `(&mut Locomotion, &Breakpoints)`

Gere les frames d'invincibilite apres avoir ete touche (FHR -- Faster Hit Recovery).
L'entite ne peut pas agir pendant la duree du stagger.

```rust
// @id: sd-sys-hit-recovery @do: define @role: arpg @layer: 3 @human: miyuk

/// Seuil de dommage pour declencher le hit recovery.
/// Le hit recovery se declenche si le dommage depasse 1/12 de la vie max.
const HIT_RECOVERY_THRESHOLD_DIVISOR: i32 = 12;

pub fn hit_recovery_system(
    mut hit_events: EventReader<DamageEvent>,
    mut query: Query<(&mut Locomotion, &Breakpoints, &VitalPools)>,
) {
    for event in hit_events.iter() {
        if event.result.missed || event.result.blocked {
            continue;
        }

        let (mut locomotion, breakpoints, vitals) = match query.get_mut(event.target) {
            Ok(v) => v,
            Err(_) => continue,
        };

        // Ne pas declencher si deja en recovery ou dead.
        if matches!(
            locomotion.state,
            LocomotionState::HitRecovery | LocomotionState::Dead
        ) {
            continue;
        }

        // Calculer le seuil de dommage pour le stagger.
        let threshold = vitals.life_max / HIT_RECOVERY_THRESHOLD_DIVISOR;
        let total_damage =
            event.result.physical + event.result.fire + event.result.cold + event.result.lightning;

        if total_damage >= threshold {
            locomotion.state = LocomotionState::HitRecovery;
            locomotion.animation_frames_remaining = breakpoints.hit_recovery_frames;
        }
    }
}

/// Systeme complementaire qui decremente les frames de recovery.
pub fn recovery_countdown_system(mut query: Query<&mut Locomotion>) {
    for mut locomotion in query.iter_mut() {
        if matches!(
            locomotion.state,
            LocomotionState::HitRecovery
                | LocomotionState::BlockRecovery
                | LocomotionState::Casting
                | LocomotionState::Attacking
        ) {
            if locomotion.animation_frames_remaining > 0 {
                locomotion.animation_frames_remaining -= 1;
            } else {
                locomotion.state = LocomotionState::Idle;
            }
        }
    }
}
```

### 4.7 AuraSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-combat`
**Query :** `(&Position, &ActiveAuras)` pour les porteurs ; `(&Position, &mut BuffList)` pour les cibles
**Composants sparse overlay :** `BuffEntry`

Les auras appliquent en permanence des buffs aux entites dans leur rayon. Quand
une entite sort du rayon, le buff est retire au tick suivant.

```rust
// @id: sd-sys-aura @do: define @role: arpg @layer: 3 @human: miyuk

pub fn aura_system(
    aura_query: Query<(Entity, &Position, &ActiveAuras)>,
    mut target_query: Query<(Entity, &Position, &mut BuffList)>,
) {
    // Pour chaque porteur d'aura.
    for (aura_entity, aura_pos, auras) in aura_query.iter() {
        for aura in &auras.auras {
            let radius_sq = aura.radius * aura.radius;

            // Pour chaque cible potentielle.
            for (target_entity, target_pos, mut buffs) in target_query.iter_mut() {
                // Verifier si la cible est une alliee ou ennemie selon le type d'aura.
                let dx = aura_pos.x - target_pos.x;
                let dy = aura_pos.y - target_pos.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq <= radius_sq {
                    // Appliquer le buff s'il n'est pas deja present de cette source.
                    if !buffs.has_buff_from(aura.buff_id.as_str(), aura_entity) {
                        buffs.add(BuffEntry {
                            buff_id: aura.buff_id.clone(),
                            source: aura_entity,
                            remaining_frames: u32::MAX, // Permanent tant que dans le rayon.
                            properties: aura.properties.clone(),
                            is_aura: true,
                        });
                    }
                } else {
                    // Retirer le buff d'aura si hors rayon.
                    buffs.remove_aura_from(aura.buff_id.as_str(), aura_entity);
                }
            }
        }
    }
}
```

### 4.8 BuffSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-combat`
**Query :** `(&mut BuffList)`
**Sparse overlay :** Oui (les buffs/debuffs sont stockes dans le sparse overlay)

Decremente les durees des buffs/debuffs, applique les effets par tick (DoT poison,
regen), et retire les buffs expires.

```rust
// @id: sd-sys-buff @do: define @role: arpg @layer: 3 @human: miyuk

pub fn buff_system(
    mut query: Query<(&mut BuffList, &mut VitalPools)>,
) {
    for (mut buffs, mut vitals) in query.iter_mut() {
        let mut expired_indices = Vec::new();

        for (index, buff) in buffs.entries.iter_mut().enumerate() {
            // Decrire la duree (sauf pour les buffs permanents d'aura).
            if !buff.is_aura && buff.remaining_frames != u32::MAX {
                if buff.remaining_frames > 0 {
                    buff.remaining_frames -= 1;
                } else {
                    expired_indices.push(index);
                    continue;
                }
            }

            // Appliquer les effets par tick.
            for prop in &buff.properties {
                match prop.stat.as_str() {
                    "poison_damage_per_frame" => {
                        if let PropertyValue::Flat(dmg) = prop.value {
                            vitals.life_current -= dmg;
                        }
                    }
                    "life_regen_per_frame" => {
                        if let PropertyValue::Flat(regen) = prop.value {
                            vitals.life_current =
                                (vitals.life_current + regen).min(vitals.life_max);
                        }
                    }
                    _ => {} // Les bonus de stats sont recalcules par le stat aggregation system.
                }
            }
        }

        // Retirer les buffs expires (en ordre inverse pour maintenir les indices).
        for index in expired_indices.into_iter().rev() {
            buffs.entries.swap_remove(index);
        }
    }
}
```

---

## 5. Systemes IA

### 5.1 AIBehaviorSystem

**Stage :** FixedUpdate (25 Hz, mais execute tous les 2-3 ticks = ~10 Hz)
**Crate :** `mge-arpg-ai`
**Query :** `(&mut AiState, &AiParams, &Position, &VitalPools, &Locomotion)`

Machine a etats pour les 43 archetypes d'IA. Chaque monstre evalue ses transitions
d'etat en fonction de son archetype, de sa cible, et de ses parametres.

```rust
// @id: sd-sys-ai-behavior @do: define @role: arpg @layer: 3 @human: miyuk

const AI_UPDATE_INTERVAL: u32 = 3; // Toutes les 3 frames = ~8.3 Hz

pub fn ai_behavior_system(
    frame_counter: Res<FrameCounter>,
    mut query: Query<(
        Entity,
        &mut AiState,
        &AiParams,
        &Position,
        &VitalPools,
        &Locomotion,
    ), With<MonsterData>>,
    player_query: Query<(Entity, &Position), With<CharacterInfo>>,
) {
    // Ne s'executer que tous les AI_UPDATE_INTERVAL ticks.
    if frame_counter.frame % AI_UPDATE_INTERVAL as u64 != 0 {
        return;
    }

    for (entity, mut ai, params, pos, vitals, locomotion) in query.iter_mut() {
        // Ne pas mettre a jour les morts.
        if matches!(locomotion.state, LocomotionState::Dead) {
            ai.current_state = AiBehaviorState::Dead;
            continue;
        }

        // Decrire le timer d'etat.
        if ai.state_timer > 0 {
            ai.state_timer = ai.state_timer.saturating_sub(AI_UPDATE_INTERVAL);
        }

        // Evaluer les transitions.
        match ai.current_state {
            AiBehaviorState::Idle => {
                // Chercher un joueur dans le rayon d'aggro.
                if let Some((player_entity, _)) =
                    find_nearest_player(&player_query, pos, ai.aggro_radius)
                {
                    ai.target = Some(player_entity);
                    ai.current_state = AiBehaviorState::Aggro;
                }
            }
            AiBehaviorState::Patrol => {
                // Patrouiller et verifier l'aggro.
                if let Some((player_entity, _)) =
                    find_nearest_player(&player_query, pos, ai.aggro_radius)
                {
                    ai.target = Some(player_entity);
                    ai.current_state = AiBehaviorState::Aggro;
                }
            }
            AiBehaviorState::Aggro => {
                // Transition vers Chase.
                ai.current_state = AiBehaviorState::Chase;
            }
            AiBehaviorState::Chase => {
                // Verifier leash.
                let home_dx = pos.x - ai.home_position.0;
                let home_dy = pos.y - ai.home_position.1;
                let home_dist = (home_dx * home_dx + home_dy * home_dy).sqrt();

                if home_dist > ai.leash_radius {
                    ai.target = None;
                    ai.current_state = AiBehaviorState::ReturnHome;
                    continue;
                }

                // Verifier si la cible est encore valide et a portee d'attaque.
                if let Some(target_entity) = ai.target {
                    if let Ok((_, target_pos)) = player_query.get(target_entity) {
                        let dx = pos.x - target_pos.x;
                        let dy = pos.y - target_pos.y;
                        let dist = (dx * dx + dy * dy).sqrt();

                        // A portee d'attaque : transition vers Attack.
                        let attack_range = get_attack_range(ai.archetype);
                        if dist <= attack_range {
                            ai.current_state = AiBehaviorState::Attack;
                        }
                        // Trop loin de l'aggro : perdre la cible.
                        else if dist > ai.aggro_radius * 2.0 {
                            ai.target = None;
                            ai.current_state = AiBehaviorState::ReturnHome;
                        }
                    } else {
                        ai.target = None;
                        ai.current_state = AiBehaviorState::ReturnHome;
                    }
                } else {
                    ai.current_state = AiBehaviorState::ReturnHome;
                }

                // Archetype-specific : fuite si HP bas.
                if let Some(flee_pct) = params.flee_hp_pct {
                    let hp_pct =
                        vitals.life_current as f32 / vitals.life_max.max(1) as f32;
                    if hp_pct < flee_pct {
                        ai.current_state = AiBehaviorState::Flee;
                    }
                }
            }
            AiBehaviorState::Attack => {
                // Apres l'attaque, revenir en Chase pour repositionner.
                if ai.state_timer == 0 {
                    ai.state_timer = params.attack_cooldown;
                    ai.current_state = AiBehaviorState::Chase;
                }
            }
            AiBehaviorState::Flee => {
                // Fuir pendant un temps, puis revenir.
                if ai.state_timer == 0 {
                    ai.state_timer = 75; // 3 secondes.
                }
                if ai.state_timer <= 1 {
                    ai.current_state = AiBehaviorState::ReturnHome;
                }
            }
            AiBehaviorState::ReturnHome => {
                let home_dx = pos.x - ai.home_position.0;
                let home_dy = pos.y - ai.home_position.1;
                let home_dist = (home_dx * home_dx + home_dy * home_dy).sqrt();
                if home_dist < 1.0 {
                    ai.current_state = AiBehaviorState::Idle;
                }
            }
            AiBehaviorState::Staggered | AiBehaviorState::Dead => {
                // Rien a faire, gere par d'autres systemes.
            }
            AiBehaviorState::Cast => {
                if ai.state_timer == 0 {
                    ai.current_state = AiBehaviorState::Chase;
                }
            }
            AiBehaviorState::BossPhase(_) => {
                // Logique de phase de boss specifique par archetype.
            }
        }
    }
}

/// Trouve le joueur le plus proche dans un rayon donne.
fn find_nearest_player(
    player_query: &Query<(Entity, &Position), With<CharacterInfo>>,
    pos: &Position,
    radius: f32,
) -> Option<(Entity, f32)> {
    let radius_sq = radius * radius;
    let mut nearest: Option<(Entity, f32)> = None;

    for (entity, player_pos) in player_query.iter() {
        let dx = pos.x - player_pos.x;
        let dy = pos.y - player_pos.y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq <= radius_sq {
            match nearest {
                Some((_, best_dist)) if dist_sq < best_dist => {
                    nearest = Some((entity, dist_sq));
                }
                None => {
                    nearest = Some((entity, dist_sq));
                }
                _ => {}
            }
        }
    }

    nearest
}

/// Retourne la portee d'attaque par defaut selon l'archetype IA.
fn get_attack_range(archetype: AiArchetype) -> f32 {
    match archetype {
        AiArchetype::MeleeFollower
        | AiArchetype::MeleeCharger
        | AiArchetype::MeleeHitAndRun
        | AiArchetype::MeleeStationary
        | AiArchetype::MeleeBerserker
        | AiArchetype::MeleeCowardLeader
        | AiArchetype::MeleeSwarm
        | AiArchetype::MeleePack
        | AiArchetype::MeleeAmbush => 1.5,

        AiArchetype::RangedSkirmisher
        | AiArchetype::RangedStationary
        | AiArchetype::RangedBomber
        | AiArchetype::RangedSniper
        | AiArchetype::RangedMortar => 8.0,

        AiArchetype::CasterOffensive
        | AiArchetype::CasterSupport
        | AiArchetype::CasterSummoner
        | AiArchetype::CasterCurser
        | AiArchetype::CasterAreaDenial => 6.0,

        AiArchetype::HybridMeleeCast
        | AiArchetype::HybridRangedMelee
        | AiArchetype::HybridPhaseShifter => 3.0,

        AiArchetype::BossPhased
        | AiArchetype::BossEnraged
        | AiArchetype::BossSummoner
        | AiArchetype::BossAreaControl
        | AiArchetype::BossMultiForm
        | AiArchetype::SpecialMiniBoss => 2.0,

        _ => 1.5,
    }
}
```

### 5.2 AggroSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-ai`
**Query :** `(&mut AiState, &Position)`

Gere la table d'aggro. Quand un joueur attaque un monstre, l'aggro est transferee.
Les monstres gardent une priorite de cible basee sur les dommages infliges.

```rust
// @id: sd-sys-aggro @do: define @role: arpg @layer: 3 @human: miyuk

pub fn aggro_system(
    mut damage_events: EventReader<DamageEvent>,
    mut query: Query<(&mut AiState, &Position), With<MonsterData>>,
    player_query: Query<&Position, With<CharacterInfo>>,
) {
    for event in damage_events.iter() {
        // Si un joueur attaque un monstre, transferer l'aggro.
        if player_query.get(event.source).is_ok() {
            if let Ok((mut ai, _)) = query.get_mut(event.target) {
                // Transferer l'aggro si pas deja en aggro ou si la nouvelle cible est plus proche.
                ai.target = Some(event.source);
                if matches!(ai.current_state, AiBehaviorState::Idle | AiBehaviorState::Patrol) {
                    ai.current_state = AiBehaviorState::Aggro;
                }
            }
        }
    }
}
```

### 5.3 AIPathUpdateSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-ai`
**Query :** `(&AiState, &Position, &mut MoveTarget)`

Met a jour la cible de deplacement des monstres en fonction de leur etat IA.

```rust
// @id: sd-sys-ai-path-update @do: define @role: arpg @layer: 3 @human: miyuk

pub fn ai_path_update_system(
    mut query: Query<(&AiState, &Position, &mut MoveTarget), With<MonsterData>>,
    target_query: Query<&Position>,
) {
    for (ai, pos, mut move_target) in query.iter_mut() {
        match ai.current_state {
            AiBehaviorState::Chase | AiBehaviorState::Aggro | AiBehaviorState::Attack => {
                if let Some(target_entity) = ai.target {
                    if let Ok(target_pos) = target_query.get(target_entity) {
                        move_target.x = target_pos.x;
                        move_target.y = target_pos.y;
                    }
                }
            }
            AiBehaviorState::ReturnHome => {
                move_target.x = ai.home_position.0;
                move_target.y = ai.home_position.1;
            }
            AiBehaviorState::Flee => {
                // Fuir dans la direction opposee a la cible.
                if let Some(target_entity) = ai.target {
                    if let Ok(target_pos) = target_query.get(target_entity) {
                        let dx = pos.x - target_pos.x;
                        let dy = pos.y - target_pos.y;
                        let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                        move_target.x = pos.x + (dx / dist) * 5.0;
                        move_target.y = pos.y + (dy / dist) * 5.0;
                    }
                }
            }
            _ => {} // Idle, Patrol, Dead : pas de mise a jour de chemin.
        }
    }
}
```

### 5.4 MonsterSpawnSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-ai`
**Ressources :** `Res<ZoneData>, Res<MonsterDefinitions>, Res<Difficulty>`

Spawne les monstres initiaux quand une zone est chargee. Gere la densite par zone,
les groupes (champion packs, unique packs), et les super uniques.

```rust
// @id: sd-sys-monster-spawn @do: define @role: arpg @layer: 3 @human: miyuk

pub fn monster_spawn_system(
    mut commands: Commands,
    zone_loaded_events: EventReader<ZoneLoadedEvent>,
    zone_data: Res<ZoneData>,
    monster_defs: Res<MonsterDefinitions>,
    difficulty: Res<Difficulty>,
    mut rng: ResMut<GameRng>,
) {
    for event in zone_loaded_events.iter() {
        let zone = match zone_data.get(&event.zone_id) {
            Some(z) => z,
            None => continue,
        };

        // Spawner les monstres de la zone.
        for spawn_group in &zone.spawn_groups {
            let monster_def = match monster_defs.get(&spawn_group.monster_id) {
                Some(d) => d,
                None => continue,
            };

            let count = rng.0.gen_range(spawn_group.min_count..=spawn_group.max_count);

            for _ in 0..count {
                let spawn_pos = random_position_in_area(
                    spawn_group.area_center,
                    spawn_group.area_radius,
                    &mut rng.0,
                );

                spawn_monster(
                    &mut commands,
                    monster_def,
                    spawn_pos,
                    *difficulty,
                    &zone.zone_id,
                    MonsterType::Normal,
                );
            }
        }

        // Spawner les super uniques de la zone.
        for su in &zone.super_uniques {
            let su_def = match monster_defs.get_super_unique(&su.su_id) {
                Some(d) => d,
                None => continue,
            };
            spawn_monster(
                &mut commands,
                &su_def.base,
                su.position,
                *difficulty,
                &zone.zone_id,
                MonsterType::SuperUnique,
            );
        }
    }
}
```

### 5.5 MonsterRespawnSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-ai`
**Query :** `(&RespawnTimer, &SpawnData)`

Timer de respawn par zone. Les monstres normaux respawnent apres un delai configurable
par zone (par defaut : non-respawn en D2, mais configurable pour Sodomight).

```rust
// @id: sd-sys-monster-respawn @do: define @role: arpg @layer: 3 @human: miyuk

pub fn monster_respawn_system(
    mut commands: Commands,
    mut respawn_query: Query<(Entity, &mut RespawnTimer, &SpawnData)>,
    monster_defs: Res<MonsterDefinitions>,
    difficulty: Res<Difficulty>,
) {
    for (_entity, mut timer, spawn_data) in respawn_query.iter_mut() {
        if timer.remaining_frames > 0 {
            timer.remaining_frames -= 1;
            continue;
        }

        // Respawn le monstre.
        let monster_def = match monster_defs.get(&spawn_data.monster_id) {
            Some(d) => d,
            None => continue,
        };

        spawn_monster(
            &mut commands,
            monster_def,
            spawn_data.position,
            *difficulty,
            &spawn_data.zone_id,
            spawn_data.monster_type,
        );

        // Resetter le timer.
        timer.remaining_frames = spawn_data.respawn_delay_frames;
    }
}
```

---

## 6. Systemes d'items

### 6.1 LootGenerationSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-loot`
**Lit :** `EventReader<LootSpawnEvent>`
**Ecrit :** `EventWriter<ItemDropEvent>`
**Ressources :** `Res<TreasureClasses>, Res<AffixTables>, Res<UniqueItemDefs>, Res<SetItemDefs>`

Genere les items a partir des treasure classes quand un monstre meurt. Implemente
la cascade de qualite D2 : Unique -> Set -> Rare -> Magic -> Superior -> Normal.

```rust
// @id: sd-sys-loot-generation @do: define @role: arpg @layer: 3 @human: miyuk

pub fn loot_generation_system(
    mut loot_events: EventReader<LootSpawnEvent>,
    mut drop_events: EventWriter<ItemDropEvent>,
    treasure_classes: Res<TreasureClasses>,
    affix_tables: Res<AffixTables>,
    unique_defs: Res<UniqueItemDefs>,
    set_defs: Res<SetItemDefs>,
    player_query: Query<&CombatBonuses, With<CharacterInfo>>,
    mut rng: ResMut<GameRng>,
) {
    for event in loot_events.iter() {
        // Recuperer le MF du killer (si joueur).
        let magic_find = event
            .killer
            .and_then(|k| player_query.get(k).ok())
            .map(|b| b.magic_find)
            .unwrap_or(0);

        // Resoudre la treasure class.
        let tc = match treasure_classes.get(&event.loot_table) {
            Some(tc) => tc,
            None => continue,
        };

        // Determiner le nombre de drops.
        let drop_count = resolve_drop_count(tc, &mut rng.0);

        for _ in 0..drop_count {
            // Choisir un item de base depuis la treasure class.
            let base_item = match resolve_treasure_class(tc, &treasure_classes, &mut rng.0) {
                Some(item) => item,
                None => continue, // NoDrop.
            };

            // Determiner la qualite via la cascade.
            let quality = determine_quality(
                event.monster_level,
                base_item.qlvl,
                magic_find,
                &unique_defs,
                &set_defs,
                &base_item.base_type,
                &mut rng.0,
            );

            // Generer les affixes selon la qualite.
            let affixes = generate_affixes(
                quality,
                event.monster_level,
                base_item.qlvl,
                &affix_tables,
                &unique_defs,
                &set_defs,
                &base_item.base_type,
                &mut rng.0,
            );

            drop_events.send(ItemDropEvent {
                position: event.position,
                base_item,
                quality,
                affixes,
                priority_owner: event.killer,
            });
        }
    }
}
```

### 6.2 ItemDropSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-loot`
**Lit :** `EventReader<ItemDropEvent>`

Cree des entites Item sur le sol a la position du drop.

```rust
// @id: sd-sys-item-drop @do: define @role: arpg @layer: 3 @human: miyuk

pub fn item_drop_system(
    mut commands: Commands,
    mut drop_events: EventReader<ItemDropEvent>,
    frame_counter: Res<FrameCounter>,
) {
    for event in drop_events.iter() {
        let item_entity = commands.spawn((
            Position {
                x: event.position.0,
                y: event.position.1,
            },
            event.base_item.clone(),
            ItemData {
                instance_id: uuid::Uuid::new_v4(),
                quality: event.quality,
                // ... remplir tous les champs selon l'evenement.
                identified: event.quality == ItemQuality::Normal
                    || event.quality == ItemQuality::Superior,
                ..Default::default()
            },
            event.affixes.clone(),
            Sprite {
                sprite_id: SpriteId(format!("items/{}", event.base_item.base_type)),
                render_layer: RenderLayer::GroundItem,
                visible: true,
                ..Default::default()
            },
            DropAge {
                dropped_at_frame: frame_counter.frame,
                priority_owner: event.priority_owner,
            },
        ));
    }
}
```

### 6.3 ItemPickupSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-items`
**Query joueur :** `(&Position, &mut Inventory, &CharacterInfo)`
**Query item :** `(&Position, &DropAge, &ItemData)`

Ramassage d'items par le joueur. Verifie la proximite, le timer de priorite (30 sec),
et l'espace disponible dans l'inventaire.

```rust
// @id: sd-sys-item-pickup @do: define @role: arpg @layer: 3 @human: miyuk

const PICKUP_RANGE: f32 = 1.5; // tiles
const PRIORITY_DURATION_FRAMES: u64 = 750; // 30 sec a 25 Hz

pub fn item_pickup_system(
    mut commands: Commands,
    mut pickup_events: EventReader<PickupRequestEvent>,
    frame_counter: Res<FrameCounter>,
    player_query: Query<(Entity, &Position, &mut Inventory)>,
    item_query: Query<(Entity, &Position, &DropAge, &ItemData)>,
) {
    for event in pickup_events.iter() {
        let (player_entity, player_pos, mut inventory) =
            match player_query.get_mut(event.player) {
                Ok(p) => p,
                Err(_) => continue,
            };

        let (item_entity, item_pos, drop_age, item_data) = match item_query.get(event.item) {
            Ok(i) => i,
            Err(_) => continue,
        };

        // Verifier la portee.
        let dx = player_pos.x - item_pos.x;
        let dy = player_pos.y - item_pos.y;
        if (dx * dx + dy * dy).sqrt() > PICKUP_RANGE {
            continue;
        }

        // Verifier le timer de priorite.
        if let Some(priority_owner) = drop_age.priority_owner {
            let elapsed = frame_counter.frame.saturating_sub(drop_age.dropped_at_frame);
            if elapsed < PRIORITY_DURATION_FRAMES && priority_owner != player_entity {
                continue; // Le joueur n'a pas la priorite.
            }
        }

        // Verifier l'espace dans l'inventaire.
        if !inventory.has_space_for(item_data.grid_size) {
            continue; // Inventaire plein.
        }

        // Ramasser l'item.
        inventory.add_item(item_data.instance_id, item_data.grid_size);
        commands.despawn(item_entity);
    }
}
```

### 6.4 ItemEquipSystem, InventorySystem, StashSystem, CubeSystem

Ces systemes suivent le meme patron : ils lisent des evenements de requete
(EquipRequest, MoveItemRequest, TransmuteRequest) et modifient les composants
Inventory, Equipment, Stash, ou AlchemicalCube en consequence.

```rust
// @id: sd-sys-item-equip @do: define @role: arpg @layer: 3 @human: miyuk

pub fn item_equip_system(
    mut equip_events: EventReader<EquipRequestEvent>,
    mut query: Query<(&mut Equipment, &mut Inventory, &BaseAttributes, &CharacterInfo)>,
    item_store: Res<ItemStore>,
) {
    for event in equip_events.iter() {
        let (mut equipment, mut inventory, attrs, char_info) = match query.get_mut(event.player) {
            Ok(q) => q,
            Err(_) => continue,
        };

        let item = match item_store.get(&event.item_id) {
            Some(i) => i,
            None => continue,
        };

        // Verifier les prerequis.
        if attrs.strength < item.required_strength
            || attrs.dexterity < item.required_dexterity
            || char_info.level < item.required_level
        {
            continue; // Prerequis non remplis.
        }

        // Desequiper l'item actuellement dans le slot (si present).
        let current_in_slot = equipment.get_slot(&event.slot);
        if let Some(current_id) = current_in_slot {
            // Remettre dans l'inventaire.
            let current_item = match item_store.get(&current_id) {
                Some(i) => i,
                None => continue,
            };
            if !inventory.has_space_for(current_item.grid_size) {
                continue; // Pas de place pour le swap.
            }
            inventory.add_item(current_id, current_item.grid_size);
        }

        // Equiper le nouvel item.
        inventory.remove_item(&event.item_id);
        equipment.set_slot(&event.slot, Some(event.item_id));
    }
}
```

```rust
// @id: sd-sys-cube @do: define @role: arpg @layer: 3 @human: miyuk

pub fn cube_system(
    mut transmute_events: EventReader<TransmuteRequestEvent>,
    mut query: Query<(&mut AlchemicalCube, &mut Inventory)>,
    recipes: Res<CubeRecipes>,
    item_store: ResMut<ItemStore>,
    mut rng: ResMut<GameRng>,
) {
    for event in transmute_events.iter() {
        let (mut cube, mut inventory) = match query.get_mut(event.player) {
            Ok(q) => q,
            Err(_) => continue,
        };

        // Collecter les items dans le cube.
        let cube_items: Vec<_> = cube.collect_items(&item_store);

        // Chercher une recette correspondante.
        let recipe = match recipes.find_matching(&cube_items) {
            Some(r) => r,
            None => continue, // Pas de recette.
        };

        // Executer la recette.
        let result_items = recipe.execute(&cube_items, &mut rng.0);

        // Vider le cube.
        cube.clear();

        // Placer les resultats dans le cube.
        for result_item in result_items {
            cube.place_item(result_item);
        }
    }
}
```

---

## 7. Systemes de progression

### 7.1 XPSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-stats`
**Lit :** `EventReader<XPDistributionEvent>`
**Query :** `(&mut CharacterInfo, &mut UnspentPoints, &Position)`

Distribue l'experience aux joueurs proches du kill. Gere le level up et l'attribution
de points de stat et de skill.

```rust
// @id: sd-sys-xp @do: define @role: arpg @layer: 3 @human: miyuk

/// Rayon de partage d'XP en tiles (environ 2 ecrans).
const XP_SHARE_RADIUS: f32 = 25.0;

pub fn xp_system(
    mut xp_events: EventReader<XPDistributionEvent>,
    xp_tables: Res<ExperienceTables>,
    mut query: Query<(Entity, &mut CharacterInfo, &mut UnspentPoints, &Position)>,
    mut level_up_events: EventWriter<LevelUpEvent>,
) {
    for event in xp_events.iter() {
        // Collecter les joueurs dans le rayon de partage.
        let mut eligible_players: Vec<(Entity, u8)> = Vec::new();

        for (entity, char_info, _, pos) in query.iter() {
            let dx = pos.x - event.position.0;
            let dy = pos.y - event.position.1;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist <= XP_SHARE_RADIUS {
                eligible_players.push((entity, char_info.level));
            }
        }

        if eligible_players.is_empty() {
            continue;
        }

        // Calculer l'XP par joueur.
        // En D2, l'XP est partagee mais chaque joueur recoit un bonus de groupe.
        let base_xp = event.monster_experience as u64;
        let party_size = eligible_players.len() as u64;

        for (entity, player_level) in &eligible_players {
            // Penalite/bonus de niveau : difference entre mlvl et clvl.
            let level_diff = event.monster_level as i32 - *player_level as i32;
            let xp_multiplier = xp_level_penalty(level_diff);

            let xp_gained = ((base_xp as f64 * xp_multiplier) / party_size as f64) as u64;

            if let Ok((_, mut char_info, mut unspent, _)) = query.get_mut(*entity) {
                char_info.experience += xp_gained;

                // Verifier le level up.
                while char_info.experience >= char_info.experience_next_level
                    && char_info.level < 99
                {
                    char_info.level += 1;
                    char_info.experience_next_level =
                        xp_tables.xp_for_level(char_info.level + 1);
                    unspent.stat_points += 5;
                    unspent.skill_points += 1;

                    level_up_events.send(LevelUpEvent {
                        entity: *entity,
                        new_level: char_info.level,
                    });
                }
            }
        }
    }
}

/// Penalite d'XP selon la difference de niveau monstre/joueur.
fn xp_level_penalty(level_diff: i32) -> f64 {
    match level_diff {
        d if d >= 10 => 0.05,
        d if d >= 6 => 0.15,
        d if d >= 3 => 0.60,
        d if d >= 0 => 1.0,
        d if d >= -5 => 1.0,
        d if d >= -10 => 0.80,
        d if d >= -15 => 0.50,
        _ => 0.25,
    }
}
```

### 7.2 SkillProgressionSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-skills`
**Lit :** `EventReader<LevelUpEvent>`

Gere l'attribution de points de skill lors du level up et le recalcul des synergies.

```rust
// @id: sd-sys-skill-progression @do: define @role: arpg @layer: 3 @human: miyuk

pub fn skill_progression_system(
    mut level_up_events: EventReader<LevelUpEvent>,
    mut query: Query<(&mut SkillSlots, &mut UnspentPoints, &CharacterInfo)>,
    skill_defs: Res<SkillDefinitions>,
) {
    for event in level_up_events.iter() {
        if let Ok((mut slots, mut unspent, char_info)) = query.get_mut(event.entity) {
            // Debloquer les skills accessibles au nouveau niveau.
            for (skill_id, def) in skill_defs.iter_for_class(char_info.class) {
                if def.required_level <= char_info.level && !slots.is_unlocked(skill_id) {
                    slots.unlock(skill_id.clone());
                }
            }

            // Recalculer les synergies de tous les skills.
            slots.recalculate_synergies(&skill_defs);
        }
    }
}
```

### 7.3 QuestSystem

**Stage :** FixedUpdate (25 Hz)
**Crate :** `mge-arpg-quest`
**Ressources :** `Res<QuestDefinitions>, Res<ScriptEngine>`

Gere les flags de quete, les triggers, et les recompenses.

```rust
// @id: sd-sys-quest @do: define @role: arpg @layer: 3 @human: miyuk

pub fn quest_system(
    mut quest_events: EventReader<QuestTriggerEvent>,
    quest_defs: Res<QuestDefinitions>,
    script_engine: Res<ScriptEngine>,
    mut query: Query<(&mut QuestLog, &mut CharacterInfo, &mut UnspentPoints)>,
) {
    for event in quest_events.iter() {
        let (mut quest_log, mut char_info, mut unspent) = match query.get_mut(event.player) {
            Ok(q) => q,
            Err(_) => continue,
        };

        let quest_def = match quest_defs.get(&event.quest_id) {
            Some(q) => q,
            None => continue,
        };

        match event.trigger_type {
            QuestTrigger::MonsterKilled(ref monster_id) => {
                if quest_def.completion_monster.as_deref() == Some(monster_id) {
                    quest_log.set_flag(&event.quest_id, QuestState::Completed);
                    apply_quest_rewards(quest_def, &mut char_info, &mut unspent);
                }
            }
            QuestTrigger::ItemDelivered(ref item_id) => {
                if quest_def.required_item.as_deref() == Some(item_id) {
                    quest_log.set_flag(&event.quest_id, QuestState::Completed);
                    apply_quest_rewards(quest_def, &mut char_info, &mut unspent);
                }
            }
            QuestTrigger::ZoneEntered(ref zone_id) => {
                if quest_def.trigger_zone.as_deref() == Some(zone_id) {
                    quest_log.set_flag(&event.quest_id, QuestState::InProgress);
                }
            }
            QuestTrigger::NpcTalked(ref npc_id) => {
                // Executer le script Rhai de dialogue.
                if let Some(script) = &quest_def.dialogue_script {
                    let _ = script_engine.execute(script);
                }
            }
        }
    }
}
```

### 7.4 WaypointSystem et DifficultySystem

```rust
// @id: sd-sys-waypoint @do: define @role: game @layer: 4 @human: miyuk

pub fn waypoint_system(
    mut query: Query<(&Position, &mut DiscoveredWaypoints, &CurrentZone), With<CharacterInfo>>,
    waypoint_positions: Res<WaypointPositions>,
) {
    for (pos, mut discovered, zone) in query.iter_mut() {
        // Verifier si le joueur est proche d'un waypoint non decouvert.
        if let Some(wp) = waypoint_positions.get_in_zone(&zone.zone_id) {
            let dx = pos.x - wp.x;
            let dy = pos.y - wp.y;
            if (dx * dx + dy * dy).sqrt() < 2.0 {
                let wp_list = match zone.difficulty {
                    Difficulty::Normal => &mut discovered.normal,
                    Difficulty::Nightmare => &mut discovered.nightmare,
                    Difficulty::Hell => &mut discovered.hell,
                };
                if !wp_list.contains(&zone.zone_id) {
                    wp_list.push(zone.zone_id.clone());
                }
            }
        }
    }
}

// @id: sd-sys-difficulty @do: define @role: game @layer: 4 @human: miyuk

pub fn difficulty_system(
    difficulty: Res<Difficulty>,
    mut resist_query: Query<&mut ResistancePenalty, With<CharacterInfo>>,
) {
    let penalty = match *difficulty {
        Difficulty::Normal => 0,
        Difficulty::Nightmare => -40,
        Difficulty::Hell => -100,
    };

    for mut rp in resist_query.iter_mut() {
        rp.fire_penalty = penalty;
        rp.cold_penalty = penalty;
        rp.lightning_penalty = penalty;
        rp.poison_penalty = penalty;
    }
}
```

---

## 8. Systemes reseau

### 8.1 NetworkSyncSystem

**Stage :** Network (variable)
**Crate :** `mge-net`
**Query :** `(&Position, &VitalPools, &AnimState, &NetSync)`

Envoie les deltas d'etat aux clients connectes. Utilise la compression delta
pour minimiser la bande passante.

```rust
// @id: sd-sys-network-sync @do: define @role: engine @layer: 2 @human: miyuk

pub fn network_sync_system(
    net: ResMut<NetworkManager>,
    mut delta_compressor: ResMut<DeltaCompressor>,
    query: Query<(&Position, &VitalPools, &AnimState, &NetSync)>,
) {
    if !net.is_host() {
        return;
    }

    let mut messages: Vec<ServerMessage> = Vec::new();

    for (pos, vitals, anim, sync) in query.iter() {
        // Verifier les changements de position.
        if delta_compressor.position_changed(sync.net_id, pos) {
            messages.push(ServerMessage::EntityMoved {
                id: EntityId { index: sync.net_id, generation: 0 },
                pos: Vec2 { x: pos.x, y: pos.y },
                vel: Vec2 { x: 0.0, y: 0.0 },
            });
        }

        // Verifier les changements de vie.
        if delta_compressor.health_changed(sync.net_id, vitals) {
            messages.push(ServerMessage::EntityHealthChanged {
                id: EntityId { index: sync.net_id, generation: 0 },
                current: vitals.life_current as f32,
                max: vitals.life_max as f32,
            });
        }
    }

    // Envoyer les messages a tous les clients.
    for msg in messages {
        net.broadcast(&msg);
    }
}
```

### 8.2 PlayerSessionSystem

```rust
// @id: sd-sys-player-session @do: define @role: engine @layer: 2 @human: miyuk

pub fn player_session_system(
    mut net: ResMut<NetworkManager>,
    mut commands: Commands,
    mut session_events: EventReader<SessionEvent>,
) {
    for event in session_events.iter() {
        match event {
            SessionEvent::PlayerConnected { player_id, character } => {
                // Spawner l'entite du joueur distant.
                commands.spawn(create_remote_player_bundle(player_id, character));
            }
            SessionEvent::PlayerDisconnected { player_id } => {
                // Despawn l'entite du joueur deconnecte.
                // Sauvegarder son personnage d'abord.
            }
        }
    }
}
```

### 8.3 AuthoritativeStateSystem

```rust
// @id: sd-sys-authoritative-state @do: define @role: engine @layer: 2 @human: miyuk

pub fn authoritative_state_system(
    mut net: ResMut<NetworkManager>,
    mut client_messages: EventReader<ClientMessageEvent>,
    mut query: Query<(&mut MoveTarget, &mut SkillSlots, &Position, &CharacterInfo)>,
) {
    for msg_event in client_messages.iter() {
        let player = msg_event.player_id;

        match &msg_event.message {
            ClientMessage::MoveToPosition { target } => {
                // Valider que la position est raisonnable (anti-triche).
                if let Ok((mut move_target, _, pos, _)) = query.get_mut_by_player(player) {
                    let dx = target.x - pos.x;
                    let dy = target.y - pos.y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < 100.0 {
                        // Position raisonnable.
                        move_target.x = target.x;
                        move_target.y = target.y;
                    }
                }
            }
            ClientMessage::UseSkill { skill_id, target } => {
                // Valider le skill (le joueur le possede, assez de mana, etc.).
                if let Ok((_, mut slots, _, char_info)) = query.get_mut_by_player(player) {
                    slots.queue_activation(skill_id.clone(), target.clone());
                }
            }
            _ => {
                // Autres messages traites par leurs systemes respectifs.
            }
        }
    }
}
```

---

## 9. Systemes UI

### 9.1 HUDUpdateSystem

**Stage :** Render (variable)
**Crate :** `mge-ui`
**Query :** `(&VitalPools, &PotionBelt, &CharacterInfo, &Equipment)`

Met a jour les orbes de vie et mana, la ceinture de potions, et les informations
du HUD a chaque frame de rendu.

```rust
// @id: sd-sys-hud-update @do: define @role: engine @layer: 2 @human: miyuk

pub fn hud_update_system(
    query: Query<(&VitalPools, &PotionBelt, &CharacterInfo), With<LocalPlayer>>,
    mut hud_state: ResMut<HudState>,
) {
    for (vitals, belt, char_info) in query.iter() {
        // Orbes de vie et mana.
        hud_state.life_pct = vitals.life_current as f32 / vitals.life_max.max(1) as f32;
        hud_state.mana_pct = vitals.mana_current as f32 / vitals.mana_max.max(1) as f32;
        hud_state.stamina_pct = vitals.stamina_current as f32 / vitals.stamina_max.max(1) as f32;

        // Texte des orbes.
        hud_state.life_text = format!("{}/{}", vitals.life_current, vitals.life_max);
        hud_state.mana_text = format!("{}/{}", vitals.mana_current, vitals.mana_max);

        // Niveau et XP.
        hud_state.level = char_info.level;
        hud_state.experience = char_info.experience;
        hud_state.experience_next = char_info.experience_next_level;
    }
}
```

### 9.2 TooltipSystem

```rust
// @id: sd-sys-tooltip @do: define @role: engine @layer: 2 @human: miyuk

pub fn tooltip_system(
    input: Res<InputState>,
    camera: Res<Camera>,
    mut tooltip: ResMut<ActiveTooltip>,
    item_query: Query<(&Position, &ItemData, &ItemAffixes, Option<&Sockets>)>,
    item_store: Res<ItemStore>,
) {
    // Effacer le tooltip precedent.
    tooltip.lines.clear();

    // Verifier si la souris survole un item au sol.
    let mouse_world = camera.screen_to_world(input.mouse_x, input.mouse_y);

    for (item_pos, item_data, affixes, sockets) in item_query.iter() {
        let dx = mouse_world.x - item_pos.x;
        let dy = mouse_world.y - item_pos.y;
        if (dx * dx + dy * dy).sqrt() < 0.5 {
            // Generer le tooltip.
            generate_item_tooltip(&mut tooltip.lines, item_data, affixes, sockets);
            tooltip.screen_x = input.mouse_x;
            tooltip.screen_y = input.mouse_y;
            break;
        }
    }
}
```

### 9.3 MiniMapSystem

```rust
// @id: sd-sys-minimap @do: define @role: engine @layer: 2 @human: miyuk

pub fn minimap_system(
    player_query: Query<&Position, With<LocalPlayer>>,
    monster_query: Query<&Position, With<MonsterData>>,
    item_query: Query<&Position, With<ItemData>>,
    mut minimap: ResMut<MiniMapState>,
) {
    // Centrer la minimap sur le joueur.
    if let Ok(player_pos) = player_query.get_single() {
        minimap.center = (player_pos.x, player_pos.y);
    }

    // Collecter les points d'interet.
    minimap.monster_dots.clear();
    for pos in monster_query.iter() {
        minimap.monster_dots.push((pos.x, pos.y));
    }

    minimap.item_dots.clear();
    for pos in item_query.iter() {
        minimap.item_dots.push((pos.x, pos.y));
    }
}
```

---

## 10. Systemes audio

### 10.1 AudioTriggerSystem

**Stage :** Audio (variable)
**Crate :** `mge-audio`
**Lit :** Evenements de combat, mort, ramassage, level up

```rust
// @id: sd-sys-audio-trigger @do: define @role: engine @layer: 2 @human: miyuk

pub fn audio_trigger_system(
    mut audio: ResMut<AudioManager>,
    damage_events: EventReader<DamageEvent>,
    death_events: EventReader<EntityDiedEvent>,
    level_up_events: EventReader<LevelUpEvent>,
    pickup_events: EventReader<PickupRequestEvent>,
) {
    for event in damage_events.iter() {
        if event.result.missed {
            audio.play_sfx("sfx/miss_01");
        } else if event.result.blocked {
            audio.play_sfx("sfx/block_01");
        } else if event.result.critical {
            audio.play_sfx("sfx/critical_hit_01");
        } else {
            audio.play_sfx("sfx/hit_01");
        }
    }

    for _event in death_events.iter() {
        audio.play_sfx("sfx/monster_death_01");
    }

    for _event in level_up_events.iter() {
        audio.play_sfx("sfx/level_up");
    }

    for _event in pickup_events.iter() {
        audio.play_sfx("sfx/pickup_01");
    }
}
```

### 10.2 MusicSystem

```rust
// @id: sd-sys-music @do: define @role: engine @layer: 2 @human: miyuk

pub fn music_system(
    mut audio: ResMut<AudioManager>,
    zone_changed_events: EventReader<ZoneChangedEvent>,
    zone_data: Res<ZoneData>,
) {
    for event in zone_changed_events.iter() {
        if let Some(zone) = zone_data.get(&event.new_zone_id) {
            // Crossfade vers la musique de la nouvelle zone.
            audio.crossfade_music(&zone.music_track, 2.0); // 2 secondes de crossfade.
        }
    }
}
```

---

## 11. Pipeline de combat complet -- Implementation Rust

### 11.1 Fonction calculate_damage

Pipeline de degats D2 en 8 etapes. Cette fonction est appelee par AttackSystem
pour chaque attaque resolue.

```rust
// @id: sd-sys-damage-pipeline @do: define @role: arpg @layer: 3 @human: miyuk

/// Resultat complet d'un calcul de dommage.
#[derive(Debug, Clone)]
pub struct DamageResult {
    /// Dommage physique apres toutes les reductions.
    pub physical: i32,
    /// Dommage feu apres resistance.
    pub fire: i32,
    /// Dommage froid apres resistance.
    pub cold: i32,
    /// Dommage foudre apres resistance.
    pub lightning: i32,
    /// Dommage poison total.
    pub poison: i32,
    /// Duree du poison en frames.
    pub poison_duration: u32,
    /// Dommage magique apres resistance.
    pub magic: i32,
    /// L'attaque a-t-elle ete bloquee ?
    pub blocked: bool,
    /// L'attaque a-t-elle rate ?
    pub missed: bool,
    /// Critical Strike / Deadly Strike ?
    pub critical: bool,
    /// Crushing Blow applique ?
    pub crushing_blow: bool,
    /// Open Wounds applique ?
    pub open_wounds: bool,
    /// Dommage total avant resistances (pour affichage).
    pub total_before_resist: i32,
    /// Dommage total apres resistances.
    pub total_final: i32,
}

/// Cible type (pour les modificateurs de Crushing Blow et Open Wounds).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    NormalMonster,
    ChampionOrSuperUnique,
    ActBoss,
    Player,
}

/// Pipeline de calcul de dommage complet.
///
/// # Etapes
///
/// 1. CTH (Chance to Hit) -- check de toucher
/// 2. Block check -- chance de blocage du defenseur
/// 3. Base damage roll -- roll min-max de l'arme
/// 4. Skill modifiers -- bonus de skill actif
/// 5. Defense reduction -- non utilise en D2 (defense affecte CTH, pas les degats)
/// 6. Resistance reduction -- application des resistances par element
/// 7. Final modifiers -- Crushing Blow, Open Wounds, Critical Strike, Deadly Strike
/// 8. Apply damage -- resultat final
pub fn calculate_damage(
    attacker_ar: i32,
    defender_defense: i32,
    attacker_level: u8,
    defender_level: u8,
    weapon: &EquippedWeapon,
    bonuses: &CombatBonuses,
    elem_dmg: &ElementalDamage,
    defender_resist: &Resistances,
    defender_vitals: &VitalPools,
    defender_block: Option<&BlockChance>,
    defender_monster: Option<&MonsterData>,
    rng: &mut impl Rng,
) -> DamageResult {
    let mut result = DamageResult {
        physical: 0,
        fire: 0,
        cold: 0,
        lightning: 0,
        poison: 0,
        poison_duration: 0,
        magic: 0,
        blocked: false,
        missed: false,
        critical: false,
        crushing_blow: false,
        open_wounds: false,
        total_before_resist: 0,
        total_final: 0,
    };

    // ================================================================
    // ETAPE 1 : Chance to Hit (CTH)
    // ================================================================
    // Formule D2 : CTH = 200 * AR / (AR + DR) * clvl / (clvl + mlvl)
    // Cappe entre 5% et 95%.
    let ar = attacker_ar as f32;
    let dr = defender_defense as f32;
    let clvl = attacker_level as f32;
    let mlvl = defender_level as f32;

    let cth = if bonuses.ignore_target_defense {
        // ITD : ignore la defense, seul l'ecart de niveau compte.
        // Ne fonctionne PAS contre les Super Uniques et Act Bosses.
        let is_boss = defender_monster
            .map(|m| {
                matches!(
                    m.monster_type,
                    MonsterType::SuperUnique | MonsterType::ActBoss | MonsterType::Unique
                )
            })
            .unwrap_or(false);

        if is_boss {
            (200.0 * ar / (ar + dr) * clvl / (clvl + mlvl)).clamp(5.0, 95.0)
        } else {
            (200.0 * clvl / (clvl + mlvl)).clamp(5.0, 95.0)
        }
    } else {
        let divisor = if (ar + dr).abs() < 0.001 { 1.0 } else { ar + dr };
        (200.0 * ar / divisor * clvl / (clvl + mlvl).max(1.0)).clamp(5.0, 95.0)
    };

    // Roll de toucher.
    let hit_roll: f32 = rng.gen_range(0.0..100.0);
    if hit_roll >= cth {
        result.missed = true;
        return result;
    }

    // ================================================================
    // ETAPE 2 : Block check
    // ================================================================
    // Le defenseur a une chance de bloquer l'attaque (si bouclier equipe).
    if let Some(block) = defender_block {
        let block_chance = block.standing.min(75) as f32;
        let block_roll: f32 = rng.gen_range(0.0..100.0);
        if block_roll < block_chance {
            result.blocked = true;
            return result;
        }
    }

    // ================================================================
    // ETAPE 3 : Base damage roll
    // ================================================================
    // Roll entre les dommages min et max de l'arme.
    let mut weapon_min = weapon.base_damage_min;
    let mut weapon_max = weapon.base_damage_max;

    // Bonus ethereal : +50% damage.
    if weapon.ethereal {
        weapon_min = (weapon_min as f32 * 1.5) as i32;
        weapon_max = (weapon_max as f32 * 1.5) as i32;
    }

    let base_roll = rng.gen_range(weapon_min..=weapon_max);

    // ================================================================
    // ETAPE 4 : Skill modifiers + ED on-weapon + flat damage + ED off-weapon + STR/DEX bonus
    // ================================================================
    // 4a. Enhanced Damage on-weapon (multiplicatif).
    let after_ed_on = (base_roll as f64 * (1.0 + bonuses.ed_on_weapon as f64 / 100.0)) as i32;

    // 4b. Flat damage bonus.
    let after_flat_min = after_ed_on + bonuses.flat_damage_min;
    let after_flat_max = after_ed_on + bonuses.flat_damage_max;
    let after_flat = rng.gen_range(after_flat_min..=after_flat_max.max(after_flat_min));

    // 4c. Enhanced Damage off-weapon (multiplicatif).
    let after_ed_off =
        (after_flat as f64 * (1.0 + bonuses.ed_off_weapon as f64 / 100.0)) as i32;

    // 4d. STR/DEX bonus selon type d'arme.
    // DmgBonus% = (STR * strFactor + DEX * dexFactor) / 100
    // Ce bonus est deja integre dans ed_off_weapon par le stat aggregation system.
    let physical_before_crit = after_ed_off;

    // ================================================================
    // ETAPE 5 : Defense reduction
    // ================================================================
    // En D2, la defense n'affecte PAS les degats, seulement la CTH (etape 1).
    // Rien a faire ici.

    // ================================================================
    // ETAPE 6 : Resistance reduction (par element)
    // ================================================================
    // Dommages elementaires (rolls independants).
    let raw_fire = rng.gen_range(elem_dmg.fire_min..=elem_dmg.fire_max.max(elem_dmg.fire_min));
    let raw_cold = rng.gen_range(elem_dmg.cold_min..=elem_dmg.cold_max.max(elem_dmg.cold_min));
    let raw_light = rng.gen_range(
        elem_dmg.lightning_min..=elem_dmg.lightning_max.max(elem_dmg.lightning_min),
    );
    let raw_magic =
        rng.gen_range(elem_dmg.magic_min..=elem_dmg.magic_max.max(elem_dmg.magic_min));
    let raw_poison = elem_dmg.poison_total;

    // Appliquer les resistances (cap a 75 par defaut, mais peut etre modifie).
    let fire_after_resist = apply_resist(raw_fire, defender_resist.fire, 75);
    let cold_after_resist = apply_resist(raw_cold, defender_resist.cold, 75);
    let light_after_resist = apply_resist(raw_light, defender_resist.lightning, 75);
    let magic_after_resist = apply_resist(raw_magic, defender_resist.magic, 75);
    let poison_after_resist = apply_resist(raw_poison, defender_resist.poison, 75);

    // Appliquer la reduction physique (DR% + DR flat).
    let phys_after_dr_pct =
        physical_before_crit - (physical_before_crit * defender_resist.physical_pct.min(50) / 100);
    let phys_after_dr = (phys_after_dr_pct - defender_resist.physical_flat).max(0);

    // ================================================================
    // ETAPE 7 : Final modifiers
    // ================================================================

    // 7a. Critical Strike (skill) + Deadly Strike (item) = double damage physique.
    let cs_chance = bonuses.critical_strike_pct as f32 / 100.0;
    let ds_chance = bonuses.deadly_strike_pct as f32 / 100.0;
    let double_chance = 1.0 - ((1.0 - cs_chance) * (1.0 - ds_chance));
    let double_roll: f32 = rng.gen_range(0.0..1.0);
    let mut final_phys = phys_after_dr;

    if double_roll < double_chance {
        final_phys *= 2;
        result.critical = true;
    }

    // 7b. Crushing Blow -- retire une fraction de la vie actuelle.
    let cb_roll: f32 = rng.gen_range(0.0..100.0);
    if cb_roll < bonuses.crushing_blow_pct as f32 {
        let target_type = determine_target_type(defender_monster);
        let is_ranged = matches!(weapon.attack_type, AttackType::Ranged | AttackType::Thrown);
        let cb_damage = crushing_blow_damage(
            defender_vitals.life_current,
            target_type,
            is_ranged,
        );
        final_phys += cb_damage;
        result.crushing_blow = true;
    }

    // 7c. Open Wounds -- DoT physique sur 8 secondes.
    let ow_roll: f32 = rng.gen_range(0.0..100.0);
    if ow_roll < bonuses.open_wounds_pct as f32 {
        result.open_wounds = true;
        // Le DoT est gere par le BuffSystem via un debuff.
    }

    // 7d. Life Steal et Mana Steal (appliques cote appelant, pas ici).

    // ================================================================
    // ETAPE 8 : Apply damage -- assembler le resultat
    // ================================================================
    result.physical = final_phys.max(0);
    result.fire = fire_after_resist.max(0);
    result.cold = cold_after_resist.max(0);
    result.lightning = light_after_resist.max(0);
    result.poison = poison_after_resist.max(0);
    result.poison_duration = elem_dmg.poison_duration_frames;
    result.magic = magic_after_resist.max(0);

    result.total_before_resist = physical_before_crit
        + raw_fire
        + raw_cold
        + raw_light
        + raw_poison
        + raw_magic;

    result.total_final = result.physical
        + result.fire
        + result.cold
        + result.lightning
        + result.poison
        + result.magic;

    result
}

/// Applique une resistance elementaire avec cap.
fn apply_resist(damage: i32, resist: i32, cap: i32) -> i32 {
    if damage <= 0 {
        return 0;
    }
    let capped = resist.min(cap);
    // Resistance negative = damage amplifie.
    let reduced = damage as f64 * (1.0 - capped as f64 / 100.0);
    reduced.max(0.0) as i32
}

/// Calcule les dommages de Crushing Blow.
fn crushing_blow_damage(current_life: i32, target: TargetType, is_ranged: bool) -> i32 {
    let fraction = match (target, is_ranged) {
        (TargetType::NormalMonster, false) => 4,    // 1/4
        (TargetType::NormalMonster, true) => 8,     // 1/8
        (TargetType::ChampionOrSuperUnique, false) => 8,  // 1/8
        (TargetType::ChampionOrSuperUnique, true) => 16,  // 1/16
        (TargetType::ActBoss, false) => 8,          // 1/8
        (TargetType::ActBoss, true) => 16,          // 1/16
        (TargetType::Player, false) => 10,          // 1/10
        (TargetType::Player, true) => 20,           // 1/20
    };
    current_life / fraction
}

/// Determine le type de cible a partir des donnees de monstre.
fn determine_target_type(monster: Option<&MonsterData>) -> TargetType {
    match monster {
        Some(m) => match m.monster_type {
            MonsterType::Normal | MonsterType::Minion => TargetType::NormalMonster,
            MonsterType::Champion | MonsterType::Unique | MonsterType::SuperUnique => {
                TargetType::ChampionOrSuperUnique
            }
            MonsterType::ActBoss => TargetType::ActBoss,
        },
        None => TargetType::Player,
    }
}
```

---

## 12. Invariants et regles des systemes

### 12.1 Invariants d'execution

| Invariant | Description | Verification |
|-----------|-------------|-------------|
| Ordre strict FixedUpdate | Les systemes FixedUpdate s'executent dans l'ordre documente | Scheduler verifie les dependances |
| Budget pathfinding | Le PathfindingSystem ne depasse jamais 5ms par frame | Timer interne + break |
| Cap de ticks | Maximum 4 ticks FixedUpdate par frame rendu | Accumulateur avec cap |
| AI throttle | L'IA ne s'execute que tous les 2-3 ticks (~10 Hz) | Frame counter modulo |
| Damage pipeline deterministe | Le meme seed RNG + memes inputs = meme resultat | Tests de regression |
| Hit frame unique | Le hit d'une attaque n'est applique qu'une seule fois | Flag hit_applied |
| Loot cote serveur | LootGenerationSystem ne s'execute que sur le host | Guard is_host() |

### 12.2 Regles de securite

| Regle | Systeme | Mesure |
|-------|---------|--------|
| Validation inputs client | AuthoritativeStateSystem | Distance max de deplacement par tick |
| Anti speed-hack | MovementSystem | Vitesse max cappee par le serveur |
| Anti duplication items | ItemPickupSystem | Despawn atomique + verification |
| Anti loot snipe | ItemPickupSystem | Timer de priorite 30 secondes |
| Anti gold hack | InventorySystem | Gold valide cote serveur uniquement |
| Anti skill spam | SkillActivationSystem | Cooldowns verifies cote serveur |

### 12.3 Tests obligatoires par systeme

| Systeme | Type de test | Description |
|---------|-------------|-------------|
| calculate_damage | Unitaire | Verifier chaque etape du pipeline avec des valeurs connues |
| calculate_damage | Regression | Comparer avec les valeurs D2 de reference |
| PathfindingSystem | Unitaire | A* sur grilles predefinies, verifier les chemins |
| LootGenerationSystem | Statistique | 10000 drops, verifier les distributions vs D2 |
| XPSystem | Unitaire | Verifier les penalites de niveau |
| BuffSystem | Unitaire | Application, expiration, stacking |
| CollisionSystem | Unitaire | Detection sur cas limites |
| AIBehaviorSystem | Integration | State machine complete sur scenario type |

---

*Document redige par Denis, Chef Dev Senior -- Miyukini AI Studio*
*Revision : 2026-02-28 v1.0*
