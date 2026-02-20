# MGE — Gameplay Foundation Pack (GFP)

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 1 (Foundation Pack)  
**Repertoire** : `mge/crates/foundation/`  
**Nombre de crates** : 35  

---

## 1. Contexte

Le Gameplay Foundation Pack (GFP) est le socle gameplay universel du MGE. Il se situe au-dessus du Kernel (Layer 0) et en dessous de tous les Genre Packs (Layer 2). Chaque pack genre (RPG, Platformer, Shooter, RTS, etc.) depend du GFP pour le spatial, le mouvement, la collision, la physique simple, la camera, les interactions, les triggers, l'animation de base et les utilitaires gameplay.

Sans le GFP, chaque pack genre devrait redefinir ses propres primitives de mouvement, collision et camera, causant duplication, incoherence et chaos architectural.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Tous. Le GFP est 100% generique et ne contient aucune logique specifique a un genre.
- **Hors portee** : Combat, inventaire, stats, quetes, dialogue, IA comportementale, rendu, audio, reseau.
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM, tous les Genre Packs.
- **Prerequis** : Kernel Layer 0 (`mge-ecs`, `mge-event`).

---

## 3. Vision

Le GFP fournit les primitives universelles dont tout jeu a besoin :

- **Position, rotation, echelle** (Transform / Spatial)
- **Deplacement par vecteur** (Velocity / Acceleration)
- **Detection de collision** (Collider / Raycast)
- **Physique simplifiee deterministe** (Gravite, friction, rebond)
- **Camera 2D** (Suivi, contraintes, shake)
- **Interaction joueur ↔ monde** (Proximity, activation)
- **Zones de declenchement** (Trigger enter/exit)
- **Machine a etats d'animation basique** (State, flip, frame timer)
- **Utilitaires gameplay** (Timer, cooldown, lifetime, despawn)

Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique a un genre.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/foundation/
├── spatial/
│   ├── mge-gfp-transform/        # Position, rotation, scale, local→world
│   ├── mge-gfp-spatial2d/        # Grid spatial, indexation 2D
│   ├── mge-gfp-hierarchy/        # Parent/child, propagation transform
│   └── mge-gfp-bounds/           # AABB, bounding boxes
├── motion/
│   ├── mge-gfp-velocity/         # Deplacement par vecteur vitesse
│   ├── mge-gfp-acceleration/     # Application acceleration
│   ├── mge-gfp-directional-move/ # Conversion intent→velocity
│   └── mge-gfp-kinematic-controller/ # Mouvement avec collision
├── collision/
│   ├── mge-gfp-collider/         # Formes de collision (AABB, circle)
│   ├── mge-gfp-collision-detection/ # Broad + narrow phase
│   ├── mge-gfp-layer-mask/       # Filtrage par couches
│   └── mge-gfp-raycast/          # Lancer de rayon 2D
├── physics/
│   ├── mge-gfp-physics-basic/    # Integration physique simple
│   ├── mge-gfp-gravity/          # Force gravitationnelle
│   ├── mge-gfp-friction/         # Friction sol/air
│   └── mge-gfp-bounce/           # Rebond sur collision
├── camera/
│   ├── mge-gfp-camera2d/         # Viewport, zoom, matrice vue
│   ├── mge-gfp-follow-camera/    # Suivi de cible
│   ├── mge-gfp-constraint-camera/ # Limites et clamping
│   └── mge-gfp-screen-shake/     # Tremblement camera
├── interaction/
│   ├── mge-gfp-interactable/     # Marqueur interactable
│   ├── mge-gfp-interaction-system/ # Resolution interactions
│   ├── mge-gfp-proximity-check/  # Detection proximite
│   └── mge-gfp-activation-event/ # Evenements activation
├── trigger/
│   ├── mge-gfp-trigger-zone/     # Zones de declenchement
│   ├── mge-gfp-enter-exit-event/ # Evenements entree/sortie
│   └── mge-gfp-area-condition/   # Conditions de zone
├── animation/
│   ├── mge-gfp-animation-state/  # Machine a etats animation
│   ├── mge-gfp-sprite-flip/      # Retournement horizontal/vertical
│   └── mge-gfp-frame-timer/      # Timer de frames animation
└── utility/
    ├── mge-gfp-timer/            # Timers generiques
    ├── mge-gfp-cooldown/         # Cooldowns rechargeables
    ├── mge-gfp-lifetime/         # Duree de vie entite
    └── mge-gfp-despawn/          # Suppression differee
```

### Graphe de dependances inter-layers

```
Spatial ◄─── Motion ◄─── Collision ◄─── Physics
   │              │            │
   ▼              ▼            ▼
Camera       Interaction    Trigger
                  │
                  ▼
             Animation      Utility (independant)
```

---

## 5. Sous-packs (Layers)

Le GFP est organise en 9 layers, chacun representant un sous-pack thematique :

| # | Layer | Crates | Role |
|---|-------|--------|------|
| 1 | Spatial | 4 | Position, rotation, echelle, hierarchie, AABB |
| 2 | Motion | 4 | Velocity, acceleration, directional input, kinematic |
| 3 | Collision | 4 | Colliders, detection, layer mask, raycast |
| 4 | Physics | 4 | Gravite, friction, rebond, integration |
| 5 | Camera | 4 | Viewport, follow, contraintes, shake |
| 6 | Interaction | 4 | Interactable, proximity, activation |
| 7 | Trigger | 3 | Zones, enter/exit, conditions |
| 8 | Animation | 3 | State machine, sprite flip, frame timer |
| 9 | Utility | 4 | Timer, cooldown, lifetime, despawn |

---

## 6. Liste des plugins

### Spatial Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-gfp-transform` | `mge.foundation.transform.v1` | [mge-gfp-transform.md](spatial/mge-gfp-transform.md) | Position, rotation, scale, matrice local→world |
| 2 | `mge-gfp-spatial2d` | `mge.foundation.spatial2d.v1` | [mge-gfp-spatial2d.md](spatial/mge-gfp-spatial2d.md) | Grille spatiale, indexation 2D, requetes zone |
| 3 | `mge-gfp-hierarchy` | `mge.foundation.hierarchy.v1` | [mge-gfp-hierarchy.md](spatial/mge-gfp-hierarchy.md) | Parent/child, propagation transform, reparentage |
| 4 | `mge-gfp-bounds` | `mge.foundation.bounds.v1` | [mge-gfp-bounds.md](spatial/mge-gfp-bounds.md) | AABB, bounding box, cache de bornes |

### Motion Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 5 | `mge-gfp-velocity` | `mge.foundation.velocity.v1` | [mge-gfp-velocity.md](motion/mge-gfp-velocity.md) | Application velocity → position |
| 6 | `mge-gfp-acceleration` | `mge.foundation.acceleration.v1` | [mge-gfp-acceleration.md](motion/mge-gfp-acceleration.md) | Application acceleration → velocity |
| 7 | `mge-gfp-directional-move` | `mge.foundation.directional_move.v1` | [mge-gfp-directional-move.md](motion/mge-gfp-directional-move.md) | Conversion input directionnel → velocity |
| 8 | `mge-gfp-kinematic-controller` | `mge.foundation.kinematic_controller.v1` | [mge-gfp-kinematic-controller.md](motion/mge-gfp-kinematic-controller.md) | Mouvement avec resolution collision |

### Collision Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 9 | `mge-gfp-collider` | `mge.foundation.collider.v1` | [mge-gfp-collider.md](collision/mge-gfp-collider.md) | Formes de collision (AABB, circle, capsule) |
| 10 | `mge-gfp-collision-detection` | `mge.foundation.collision_detection.v1` | [mge-gfp-collision-detection.md](collision/mge-gfp-collision-detection.md) | Broad phase + narrow phase |
| 11 | `mge-gfp-layer-mask` | `mge.foundation.layer_mask.v1` | [mge-gfp-layer-mask.md](collision/mge-gfp-layer-mask.md) | Filtrage collision par couches/masques |
| 12 | `mge-gfp-raycast` | `mge.foundation.raycast.v1` | [mge-gfp-raycast.md](collision/mge-gfp-raycast.md) | Lancer de rayon 2D, resultats tries |

### Physics Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 13 | `mge-gfp-physics-basic` | `mge.foundation.physics_basic.v1` | [mge-gfp-physics-basic.md](physics/mge-gfp-physics-basic.md) | Integration physique simplifiee |
| 14 | `mge-gfp-gravity` | `mge.foundation.gravity.v1` | [mge-gfp-gravity.md](physics/mge-gfp-gravity.md) | Force gravitationnelle configurable |
| 15 | `mge-gfp-friction` | `mge.foundation.friction.v1` | [mge-gfp-friction.md](physics/mge-gfp-friction.md) | Friction sol et air |
| 16 | `mge-gfp-bounce` | `mge.foundation.bounce.v1` | [mge-gfp-bounce.md](physics/mge-gfp-bounce.md) | Rebond sur collision (restitution) |

### Camera Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 17 | `mge-gfp-camera2d` | `mge.foundation.camera2d.v1` | [mge-gfp-camera2d.md](camera/mge-gfp-camera2d.md) | Viewport, zoom, matrice de vue |
| 18 | `mge-gfp-follow-camera` | `mge.foundation.follow_camera.v1` | [mge-gfp-follow-camera.md](camera/mge-gfp-follow-camera.md) | Suivi de cible avec lissage |
| 19 | `mge-gfp-constraint-camera` | `mge.foundation.constraint_camera.v1` | [mge-gfp-constraint-camera.md](camera/mge-gfp-constraint-camera.md) | Limites camera, dead zone, clamping |
| 20 | `mge-gfp-screen-shake` | `mge.foundation.screen_shake.v1` | [mge-gfp-screen-shake.md](camera/mge-gfp-screen-shake.md) | Tremblement camera (trauma, decay) |

### Interaction Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 21 | `mge-gfp-interactable` | `mge.foundation.interactable.v1` | [mge-gfp-interactable.md](interaction/mge-gfp-interactable.md) | Marqueur entite interactable |
| 22 | `mge-gfp-interaction-system` | `mge.foundation.interaction_system.v1` | [mge-gfp-interaction-system.md](interaction/mge-gfp-interaction-system.md) | Resolution des demandes d'interaction |
| 23 | `mge-gfp-proximity-check` | `mge.foundation.proximity_check.v1` | [mge-gfp-proximity-check.md](interaction/mge-gfp-proximity-check.md) | Detection entites proches |
| 24 | `mge-gfp-activation-event` | `mge.foundation.activation_event.v1` | [mge-gfp-activation-event.md](interaction/mge-gfp-activation-event.md) | Evenements d'activation contextuelle |

### Trigger Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 25 | `mge-gfp-trigger-zone` | `mge.foundation.trigger_zone.v1` | [mge-gfp-trigger-zone.md](trigger/mge-gfp-trigger-zone.md) | Definition de zones de declenchement |
| 26 | `mge-gfp-enter-exit-event` | `mge.foundation.enter_exit_event.v1` | [mge-gfp-enter-exit-event.md](trigger/mge-gfp-enter-exit-event.md) | Evenements OnEnter / OnExit |
| 27 | `mge-gfp-area-condition` | `mge.foundation.area_condition.v1` | [mge-gfp-area-condition.md](trigger/mge-gfp-area-condition.md) | Conditions d'activation de zone |

### Animation Core Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 28 | `mge-gfp-animation-state` | `mge.foundation.animation_state.v1` | [mge-gfp-animation-state.md](animation/mge-gfp-animation-state.md) | Machine a etats animation basique |
| 29 | `mge-gfp-sprite-flip` | `mge.foundation.sprite_flip.v1` | [mge-gfp-sprite-flip.md](animation/mge-gfp-sprite-flip.md) | Retournement sprite horizontal/vertical |
| 30 | `mge-gfp-frame-timer` | `mge.foundation.frame_timer.v1` | [mge-gfp-frame-timer.md](animation/mge-gfp-frame-timer.md) | Timer de frames pour animations |

### Gameplay Utility Layer

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 31 | `mge-gfp-timer` | `mge.foundation.timer.v1` | [mge-gfp-timer.md](utility/mge-gfp-timer.md) | Timers generiques (one-shot, repeating) |
| 32 | `mge-gfp-cooldown` | `mge.foundation.cooldown.v1` | [mge-gfp-cooldown.md](utility/mge-gfp-cooldown.md) | Cooldowns rechargeables |
| 33 | `mge-gfp-lifetime` | `mge.foundation.lifetime.v1` | [mge-gfp-lifetime.md](utility/mge-gfp-lifetime.md) | Duree de vie, auto-expiration |
| 34 | `mge-gfp-despawn` | `mge.foundation.despawn.v1` | [mge-gfp-despawn.md](utility/mge-gfp-despawn.md) | Suppression differee d'entites |

---

## 7. Composants cles (resume)

| Layer | Plugin | Composants runtime | Composants donnees statiques |
|-------|--------|-------------------|------------------------------|
| Spatial | transform | Transform2D, WorldTransform, PreviousTransform | aucun |
| Spatial | spatial2d | SpatialIndex, GridCell | SpatialConfig |
| Spatial | hierarchy | Parent, Children | aucun |
| Spatial | bounds | AABB, BoundsCache | aucun |
| Motion | velocity | Velocity2D | aucun |
| Motion | acceleration | Acceleration2D | aucun |
| Motion | directional-move | DirectionalInput, MoveIntent, MaxSpeed | aucun |
| Motion | kinematic-controller | KinematicController, GroundState, WallState | aucun |
| Collision | collider | Collider, ColliderShape | aucun |
| Collision | collision-detection | CollisionPair, CollisionManifold | aucun |
| Collision | layer-mask | CollisionLayer, CollisionMask | aucun |
| Collision | raycast | RaycastRequest, RaycastHit | aucun |
| Physics | physics-basic | PhysicsBody, PhysicsMaterial | aucun |
| Physics | gravity | GravityAffected, GravityScale | GravityConfig |
| Physics | friction | FrictionCoefficient, SurfaceFriction | aucun |
| Physics | bounce | Bounciness, BounceState | aucun |
| Camera | camera2d | Camera2D, Viewport | aucun |
| Camera | follow-camera | FollowTarget, FollowSmoothing, FollowOffset | aucun |
| Camera | constraint-camera | CameraBounds, DeadZone | aucun |
| Camera | screen-shake | ShakeTrauma, ShakeDecay, ShakeIntensity | aucun |
| Interaction | interactable | Interactable, InteractableState | aucun |
| Interaction | interaction-system | InteractionRequest, ActiveInteraction | aucun |
| Interaction | proximity-check | ProximityRadius, NearbyEntities | aucun |
| Interaction | activation-event | ActivationTrigger, ActivationCooldown | aucun |
| Trigger | trigger-zone | TriggerZone, TriggerOccupants | aucun |
| Trigger | enter-exit-event | TriggerTracker, EntityPresence | aucun |
| Trigger | area-condition | AreaCondition, ConditionState | aucun |
| Animation | animation-state | AnimationStateMachine, CurrentState, StateTransition | AnimationDef |
| Animation | sprite-flip | SpriteFlip | aucun |
| Animation | frame-timer | FrameTimer, FrameIndex | aucun |
| Utility | timer | Timer, TimerMode | aucun |
| Utility | cooldown | Cooldown | aucun |
| Utility | lifetime | Lifetime | aucun |
| Utility | despawn | DespawnMarker, DespawnDelay | aucun |

---

## 8. Systemes cles (resume)

| Phase | Layer | Plugin | Systemes |
|-------|-------|--------|----------|
| 100-101 | Spatial | transform | sync_local_to_world, store_previous_transform |
| 102-103 | Spatial | spatial2d | update_spatial_index, rebuild_grid |
| 104-105 | Spatial | hierarchy | propagate_parent_transform, process_reparent |
| 106-107 | Spatial | bounds | compute_aabb, update_bounds_cache |
| 110 | Motion | velocity | apply_velocity |
| 111 | Motion | acceleration | apply_acceleration |
| 112-113 | Motion | directional-move | read_directional_input, apply_move_intent |
| 114-115 | Motion | kinematic-controller | resolve_kinematic_step, update_ground_state |
| 120 | Collision | collider | sync_collider_transform |
| 121-122 | Collision | collision-detection | detect_broad_phase, detect_narrow_phase |
| 123 | Collision | layer-mask | filter_collision_pairs |
| 124 | Collision | raycast | process_raycast_requests |
| 130 | Physics | physics-basic | integrate_physics |
| 131 | Physics | gravity | apply_gravity |
| 132 | Physics | friction | apply_ground_friction, apply_air_friction |
| 133 | Physics | bounce | resolve_bounce |
| 140-141 | Camera | camera2d | compute_view_matrix, update_viewport |
| 142 | Camera | follow-camera | update_follow_target |
| 143 | Camera | constraint-camera | clamp_camera_bounds |
| 144 | Camera | screen-shake | tick_screen_shake |
| 150 | Interaction | interactable | update_interactable_state |
| 151 | Interaction | interaction-system | process_interaction_requests |
| 152 | Interaction | proximity-check | update_proximity |
| 153 | Interaction | activation-event | process_activations |
| 160 | Trigger | trigger-zone | update_trigger_occupancy |
| 161 | Trigger | enter-exit-event | emit_enter_exit_events |
| 162 | Trigger | area-condition | evaluate_area_conditions |
| 170 | Animation | animation-state | tick_animation_state |
| 171 | Animation | sprite-flip | update_sprite_flip |
| 172 | Animation | frame-timer | tick_frame_timer |
| 180 | Utility | timer | tick_timers |
| 181 | Utility | cooldown | tick_cooldowns |
| 182 | Utility | lifetime | tick_lifetimes |
| 183 | Utility | despawn | process_despawn |

**Ordre d'execution** : spatial (100) → motion (110) → collision (120) → physics (130) → camera (140) → interaction (150) → trigger (160) → animation (170) → utility (180).

**Justification** : Le spatial etablit les positions et hierarchies. Le motion applique les vitesses. La collision detecte les intersections. La physique resout les reponses. La camera suit les positions finales. Les interactions et triggers lisent les positions resolues. L'animation reagit aux etats. Les utilitaires nettoient en fin de frame.

**Total** : 39 systemes.

---

## 9. Evenements cles (resume)

| Layer | Plugin | Requests (entree) | Events (sortie) |
|-------|--------|-------------------|------------------|
| Spatial | hierarchy | ReparentRequest | ParentChanged, ChildAdded, ChildRemoved |
| Motion | directional-move | DirectionalInput (composant) | MoveIntentChanged |
| Motion | kinematic-controller | (lit Velocity) | GroundLanded, GroundLeft, WallContactChanged |
| Collision | collision-detection | (lit Collider, Transform) | CollisionEnter, CollisionExit, CollisionStay |
| Collision | raycast | RaycastRequest (composant) | RaycastCompleted |
| Physics | bounce | (lit CollisionEnter) | BounceTriggered |
| Camera | follow-camera | (lit FollowTarget) | FollowTargetChanged |
| Camera | screen-shake | ShakeRequest | ShakeStarted, ShakeEnded |
| Interaction | interaction-system | InteractionRequest (composant) | InteractionStarted, InteractionCompleted, InteractionFailed |
| Interaction | activation-event | (lit InteractionCompleted) | ActivationTriggered |
| Trigger | enter-exit-event | (lit TriggerOccupants) | TriggerEnter, TriggerExit |
| Trigger | area-condition | (lit TriggerEnter) | AreaConditionMet, AreaConditionLost |
| Animation | animation-state | StateChangeRequest | AnimationStateChanged |
| Utility | timer | (aucun) | TimerFinished, TimerTick |
| Utility | lifetime | (aucun) | LifetimeExpired |
| Utility | despawn | DespawnMarker (composant) | EntityDespawned |

**Total** : 6 requests + 23 events = 29 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 35 crates | `mge-ecs`, `mge-event` |

### Dependances intra-pack (inter-layer)

| Crate | Depend de |
|-------|-----------|
| `mge-gfp-velocity` | `mge-gfp-transform` |
| `mge-gfp-acceleration` | `mge-gfp-velocity` |
| `mge-gfp-directional-move` | `mge-gfp-velocity` |
| `mge-gfp-kinematic-controller` | `mge-gfp-velocity`, `mge-gfp-collider`, `mge-gfp-collision-detection` |
| `mge-gfp-collider` | `mge-gfp-transform`, `mge-gfp-bounds` |
| `mge-gfp-collision-detection` | `mge-gfp-collider`, `mge-gfp-spatial2d`, `mge-gfp-layer-mask` |
| `mge-gfp-raycast` | `mge-gfp-collider`, `mge-gfp-spatial2d`, `mge-gfp-layer-mask` |
| `mge-gfp-physics-basic` | `mge-gfp-velocity`, `mge-gfp-collision-detection` |
| `mge-gfp-gravity` | `mge-gfp-velocity` |
| `mge-gfp-friction` | `mge-gfp-velocity`, `mge-gfp-kinematic-controller` |
| `mge-gfp-bounce` | `mge-gfp-velocity`, `mge-gfp-collision-detection` |
| `mge-gfp-camera2d` | `mge-gfp-transform` |
| `mge-gfp-follow-camera` | `mge-gfp-camera2d`, `mge-gfp-transform` |
| `mge-gfp-constraint-camera` | `mge-gfp-camera2d` |
| `mge-gfp-screen-shake` | `mge-gfp-camera2d` |
| `mge-gfp-proximity-check` | `mge-gfp-transform`, `mge-gfp-spatial2d` |
| `mge-gfp-interaction-system` | `mge-gfp-interactable`, `mge-gfp-proximity-check` |
| `mge-gfp-activation-event` | `mge-gfp-interaction-system` |
| `mge-gfp-trigger-zone` | `mge-gfp-collider`, `mge-gfp-collision-detection` |
| `mge-gfp-enter-exit-event` | `mge-gfp-trigger-zone` |
| `mge-gfp-area-condition` | `mge-gfp-trigger-zone`, `mge-gfp-enter-exit-event` |
| `mge-gfp-sprite-flip` | `mge-gfp-velocity` |
| `mge-gfp-frame-timer` | `mge-gfp-animation-state` |

### Dependances externes (aucune)

Le GFP n'a aucune dependance vers des crates externes (pas de serde, pas de rand).

---

## 11. Interaction avec GCL

Le GCL (Game Composition Layer) configure les plugins GFP sans recompilation.

**Parametres exposables :**

- Gravite globale (direction, force)
- Coefficient de friction par defaut
- Restitution par defaut (bounce)
- Taille de la grille spatiale
- Portee de detection proximite
- Decay du screen shake
- Duree cooldown par defaut
- Vitesse maximale par defaut

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Layers GFP utilises | Usage |
|----------------|---------------------|-------|
| **RPG** | Spatial, Motion | Position entites, deplacement PNJ |
| **Platformer** | Spatial, Motion, Collision, Physics, Camera | Fondation complete du platformer |
| **Shooter** | Spatial, Motion, Collision, Camera | Projectiles, visee, camera |
| **RTS** | Spatial, Motion, Collision | Unites, pathfinding, selection |
| **Massive Battle** | Spatial, Motion, Collision | Bataillons, collisions masse |
| **Racing** | Spatial, Motion, Physics, Camera | Vehicules, friction, suivi camera |
| **Roguelike** | Spatial, Collision, Trigger, Interaction | Donjons, pieges, interactions |
| **Sandbox** | Spatial, Collision, Physics, Interaction | Monde interactif |
| **Factory** | Spatial, Motion, Trigger | Convoyeurs, zones, deplacements |
| **Puzzle** | Spatial, Collision, Trigger | Deplacements, zones, declencheurs |
| **Social** | Spatial, Interaction, Proximity | Monde social, interactions PNJ |
| **Grand Strategy** | Spatial | Carte, positions |
| **Idle** | Utility | Timers, cooldowns |
| **Tycoon** | Spatial, Trigger, Utility | Zones, timers, placement |

Le GFP ne depend d'aucun pack genre. Il est 100% autonome.

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Pas de float non deterministe** | Utiliser operations deterministes, pas de NaN |
| **Pas de HashMap order-dependent** | Iteration ordonnee si necessaire |
| **Pas de thread-local** | Aucun etat cache |
| **Pas de static mut** | Interdit par la norme AI-Native |
| **Physique deterministe** | Pas de physics engine externe, simulation interne simple |
| **Collision deterministe** | Broad phase ordonnee, narrow phase symetrique |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | spatial (propagate), collision (broad/narrow), physics (integrate) |
| **Budget cible** | < 1ms pour 10000 entites a 60 FPS (spatial + collision) |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path (pre-allouer) |
| **Spatial indexing** | Grille pour broad phase O(n) au lieu de O(n²) |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de physique 3D | GFP v1 est 2D uniquement |
| Pas de joints/contraintes physiques | Simplification v1, extension v2 |
| Pas de continuous collision detection | Discrete uniquement, CCD en v2 |
| Pas de tilemap integration | Hors scope (layer rendu) |
| Pas de navmesh | Pathfinding dans un pack dedie |
| Pas de bone animation | Sprite-based uniquement |
| Pas de spatial audio | Hors scope (audio layer) |
| Pas de serialisation transform | Utiliser mge-plugin-save-load |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| 3D Transform | Position3D, Rotation3D, matrice 4x4 |
| CCD (Continuous Collision) | Detection collision continue pour projectiles rapides |
| Joints physiques | Pivot, distance, spring joints |
| Spatial hashing avance | Quadtree, spatial hash configurable |
| Camera 3D | Perspective, orbital, FPS camera |
| Bone animation | Squelette 2D, interpolation |
| Navmesh integration | Pathfinding sur navmesh |
| Network transform sync | Synchronisation transform reseau |

---

## 17. Exemple d'assemblage

### Minimal (headless, spatial + motion uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeGfpTransformPlugin);
engine.add_plugin(MgeGfpVelocityPlugin);
engine.build();
```

### Spatial + Collision (detection uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgeGfpTransformPlugin);
engine.add_plugin(MgeGfpSpatial2dPlugin);
engine.add_plugin(MgeGfpBoundsPlugin);
engine.add_plugin(MgeGfpColliderPlugin);
engine.add_plugin(MgeGfpLayerMaskPlugin);
engine.add_plugin(MgeGfpCollisionDetectionPlugin);
engine.build();
```

### Complet (tous les layers GFP)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Spatial Layer
engine.add_plugin(MgeGfpTransformPlugin);
engine.add_plugin(MgeGfpSpatial2dPlugin);
engine.add_plugin(MgeGfpHierarchyPlugin);
engine.add_plugin(MgeGfpBoundsPlugin);
// Motion Layer
engine.add_plugin(MgeGfpVelocityPlugin);
engine.add_plugin(MgeGfpAccelerationPlugin);
engine.add_plugin(MgeGfpDirectionalMovePlugin);
engine.add_plugin(MgeGfpKinematicControllerPlugin);
// Collision Layer
engine.add_plugin(MgeGfpColliderPlugin);
engine.add_plugin(MgeGfpCollisionDetectionPlugin);
engine.add_plugin(MgeGfpLayerMaskPlugin);
engine.add_plugin(MgeGfpRaycastPlugin);
// Physics Layer
engine.add_plugin(MgeGfpPhysicsBasicPlugin);
engine.add_plugin(MgeGfpGravityPlugin);
engine.add_plugin(MgeGfpFrictionPlugin);
engine.add_plugin(MgeGfpBouncePlugin);
// Camera Layer
engine.add_plugin(MgeGfpCamera2dPlugin);
engine.add_plugin(MgeGfpFollowCameraPlugin);
engine.add_plugin(MgeGfpConstraintCameraPlugin);
engine.add_plugin(MgeGfpScreenShakePlugin);
// Interaction Layer
engine.add_plugin(MgeGfpInteractablePlugin);
engine.add_plugin(MgeGfpInteractionSystemPlugin);
engine.add_plugin(MgeGfpProximityCheckPlugin);
engine.add_plugin(MgeGfpActivationEventPlugin);
// Trigger Layer
engine.add_plugin(MgeGfpTriggerZonePlugin);
engine.add_plugin(MgeGfpEnterExitEventPlugin);
engine.add_plugin(MgeGfpAreaConditionPlugin);
// Animation Core Layer
engine.add_plugin(MgeGfpAnimationStatePlugin);
engine.add_plugin(MgeGfpSpriteFlipPlugin);
engine.add_plugin(MgeGfpFrameTimerPlugin);
// Utility Layer
engine.add_plugin(MgeGfpTimerPlugin);
engine.add_plugin(MgeGfpCooldownPlugin);
engine.add_plugin(MgeGfpLifetimePlugin);
engine.add_plugin(MgeGfpDespawnPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/foundation/
├── spatial/
│   ├── mge-gfp-transform/
│   │   ├── Cargo.toml
│   │   ├── index.md
│   │   └── src/
│   │       ├── lib.rs           # @id mge.foundation.transform.v1
│   │       ├── components.rs
│   │       ├── systems.rs
│   │       └── events.rs
│   ├── mge-gfp-spatial2d/
│   │   └── (meme structure)
│   ├── mge-gfp-hierarchy/
│   │   └── (meme structure)
│   └── mge-gfp-bounds/
│       └── (meme structure)
├── motion/
│   ├── mge-gfp-velocity/
│   │   └── (meme structure)
│   ├── mge-gfp-acceleration/
│   │   └── (meme structure)
│   ├── mge-gfp-directional-move/
│   │   └── (meme structure)
│   └── mge-gfp-kinematic-controller/
│       └── (meme structure)
├── collision/
│   ├── mge-gfp-collider/
│   │   └── (meme structure)
│   ├── mge-gfp-collision-detection/
│   │   └── (meme structure)
│   ├── mge-gfp-layer-mask/
│   │   └── (meme structure)
│   └── mge-gfp-raycast/
│       └── (meme structure)
├── physics/
│   └── (4 crates, meme structure)
├── camera/
│   └── (4 crates, meme structure)
├── interaction/
│   └── (4 crates, meme structure)
├── trigger/
│   └── (3 crates, meme structure)
├── animation/
│   └── (3 crates, meme structure)
└── utility/
    └── (4 crates, meme structure)
```

---

## 19. Resume strategique

Le Gameplay Foundation Pack est le vrai socle gameplay du MGE. Il :

- Fournit 35 plugins couvrant spatial, motion, collision, physics, camera, interaction, trigger, animation et utility.
- Est 100% universel : aucune logique specifique a un genre.
- Se situe en Layer 1, entre le Kernel (Layer 0) et les Genre Packs (Layer 2).
- Garantit que tous les packs parlent le meme langage : meme Transform, meme Collider, meme Camera.
- Elimine la duplication : sans lui, chaque pack redefinirait ses propres primitives.
- S'execute en headless, en deterministe, sans rendu.
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 35 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
