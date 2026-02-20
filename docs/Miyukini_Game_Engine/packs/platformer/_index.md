# MGE — Pack Platformer

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/platformer/`  
**Nombre de crates** : 6  

---

## 1. Contexte

Le Pack Platformer fournit les mecaniques fondamentales des jeux de plateforme 2D : mouvement horizontal, saut (double saut, coyote time), collisions plateforme (one-way, pentes, mobiles), camera (suivi, dead zone), checkpoints (sauvegarde, respawn) et dangers (pics, lave, broyeurs). Il s'appuie sur le Core Universal (spatial, physics, input).

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Plateforme 2D, metroidvania, precision platformer, endless runner.
- **Hors portee** : Plateforme 3D, combat (→ Pack RPG), inventaire, rendu, audio.
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack (spatial, physics, input).

---

## 3. Vision

Le Pack Platformer est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/platformer/
├── mge-pl-movement/         # Deplacement horizontal, acceleration, friction
├── mge-pl-jump/             # Saut, double saut, coyote time, variable jump
├── mge-pl-collision/        # Collisions plateforme, one-way, pentes, mobiles
├── mge-pl-camera/           # Suivi camera, dead zone, limites, shake
├── mge-pl-checkpoint/       # Points de sauvegarde, respawn, invincibilite
└── mge-pl-hazard/           # Zones mortelles, degats environnement, broyeurs
```

### Graphe de dependances intra-pack

```
mge-pl-jump ──────► mge-pl-movement
mge-pl-collision ──► mge-pl-movement
mge-pl-camera ────► mge-pl-movement
mge-pl-checkpoint ► mge-pl-collision
mge-pl-hazard ────► mge-pl-collision
```

Crate feuille (sans dependance intra-pack) : `mge-pl-movement`.

---

## 5. Sous-packs

Aucun. Les 6 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-pl-movement` | `mge.platformer.movement.v1` | [mge-pl-movement.md](mge-pl-movement.md) | Deplacement horizontal, acceleration, friction |
| 2 | `mge-pl-jump` | `mge.platformer.jump.v1` | [mge-pl-jump.md](mge-pl-jump.md) | Saut, double saut, coyote time, variable jump |
| 3 | `mge-pl-collision` | `mge.platformer.collision.v1` | [mge-pl-collision.md](mge-pl-collision.md) | Collisions plateforme, one-way, pentes, mobiles |
| 4 | `mge-pl-camera` | `mge.platformer.camera.v1` | [mge-pl-camera.md](mge-pl-camera.md) | Suivi camera, dead zone, limites, shake |
| 5 | `mge-pl-checkpoint` | `mge.platformer.checkpoint.v1` | [mge-pl-checkpoint.md](mge-pl-checkpoint.md) | Points de sauvegarde, respawn, invincibilite |
| 6 | `mge-pl-hazard` | `mge.platformer.hazard.v1` | [mge-pl-hazard.md](mge-pl-hazard.md) | Zones mortelles, degats environnement, broyeurs |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|-------------------|------------------------------|
| movement | PlatformerMovement, GroundDetection, MovementInput | aucun |
| jump | JumpAbility, CoyoteTime, JumpBuffer, VariableJump | aucun |
| collision | Platform, MovingPlatform, CrumblingPlatform, PlatformCollider | aucun |
| camera | CameraTarget, CameraBounds, CameraSmoothing | aucun |
| checkpoint | Checkpoint, LastCheckpoint, RespawnState | aucun |
| hazard | HazardZone, HazardTrigger, Crusher | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1600-1602 | movement | read_movement_input, apply_horizontal_movement, apply_friction |
| 1605-1608 | jump | update_coyote_time, process_jump_input, apply_jump_force, apply_variable_gravity |
| 1610-1613 | collision | resolve_platform_collisions, update_moving_platforms, update_crumbling_platforms, apply_slope_physics |
| 1620-1622 | camera | update_camera_position, clamp_camera_bounds, apply_camera_shake |
| 1625-1627 | checkpoint | detect_checkpoint_activation, process_respawn, tick_invincibility |
| 1630-1632 | hazard | detect_hazard_contact, apply_hazard_damage, update_crushers |

**Ordre d'execution** : movement (1600) → jump (1605) → collision (1610) → camera (1620) → checkpoint (1625) → hazard (1630).

**Justification** : L'input est lu en premier. Le saut ajoute la velocite verticale. Les collisions resolvent les intersections. La camera suit la position finale. Les checkpoints sont verifies. Les dangers sont detectes en dernier sur la position resolue.

**Total** : 21 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| movement | MovementInput (composant) | GroundStateChanged, DirectionChanged |
| jump | MovementInput.jump_pressed (lu) | JumpStarted, JumpLanded, DoubleJumped |
| collision | (aucun, lit Position/Velocity) | PlatformLanded, PlatformCrumbled, PlatformRespawned |
| camera | (aucun, lit Position) | CameraTargetChanged, CameraBoundsReached |
| checkpoint | (aucun, lit collision) | CheckpointActivated, RespawnTriggered, RespawnCompleted |
| hazard | (aucun, lit collision) | HazardContactDetected, HazardDamageApplied, PlayerKilled |

**Total** : 17 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 6 crates | `mge-ecs`, `mge-event` |

### Dependances vers Core Universal

| Crate | Depend de |
|-------|-----------|
| movement | `mge-plugin-spatial`, `mge-plugin-input` |
| jump, collision | `mge-plugin-spatial`, `mge-plugin-physics` |
| camera | `mge-plugin-spatial` |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-pl-jump` | `mge-pl-movement` |
| `mge-pl-collision` | `mge-pl-movement` |
| `mge-pl-camera` | `mge-pl-movement` |
| `mge-pl-checkpoint` | `mge-pl-collision` |
| `mge-pl-hazard` | `mge-pl-collision` |

### Dependances externes (aucune)

Le Pack Platformer n'a aucune dependance vers des crates externes.

---

## 11. Exemple d'assemblage

### Minimal (headless, movement + jump)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginPhysics::default());
engine.add_plugin(MgePlMovementPlugin);
engine.add_plugin(MgePlJumpPlugin);
engine.build();
```

### Complet (Platformer jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginPhysics::default());
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginRender2d::default());
// Pack Platformer
engine.add_plugin(MgePlMovementPlugin);
engine.add_plugin(MgePlJumpPlugin);
engine.add_plugin(MgePlCollisionPlugin);
engine.add_plugin(MgePlCameraPlugin);
engine.add_plugin(MgePlCheckpointPlugin);
engine.add_plugin(MgePlHazardPlugin);
engine.build();
```

---

## 12. Organisation des crates

```
mge/crates/platformer/
├── mge-pl-movement/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.platformer.movement.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-pl-jump/
│   └── (meme structure)
├── mge-pl-collision/
│   └── (meme structure)
├── mge-pl-camera/
│   └── (meme structure)
├── mge-pl-checkpoint/
│   └── (meme structure)
└── mge-pl-hazard/
    └── (meme structure)
```

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
