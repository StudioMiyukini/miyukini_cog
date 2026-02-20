# mge-pl-checkpoint

> @id mge.platformer.checkpoint.v1  
> @role plugin  
> @domain platformer  
> @do manage_checkpoints_respawn_save_positions  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-pl-checkpoint` |
| @id MSCM | `mge.platformer.checkpoint.v1` |
| Domaine | platformer |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-pl-collision` |
| Hot path | Non (verification ponctuelle) |
| Headless safe | Oui |
| Complexite globale | O(c) par tick, c = checkpoints |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `CheckpointState` | `Inactive, Active, Used` | Etat du checkpoint. Active = dernier atteint |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Checkpoint` | `mge.platformer.checkpoint.v1.component.checkpoint` | `position_x: f32, position_y: f32, state: CheckpointState, order: u32` | Point de sauvegarde. order = progression dans le niveau |
| `LastCheckpoint` | `mge.platformer.checkpoint.v1.component.last_checkpoint` | `checkpoint_entity: Option<EntityId>, respawn_x: f32, respawn_y: f32` | Dernier checkpoint atteint. Singleton sur le joueur |
| `RespawnState` | `mge.platformer.checkpoint.v1.component.respawn_state` | `respawning: bool, invincible_ticks: u32` | Etat de respawn. invincible_ticks = frames d'invincibilite apres respawn |

---

## 4. Formules

Aucune formule de derivation. Les checkpoints sont des declencheurs discrets.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `detect_checkpoint_activation` | `mge.platformer.checkpoint.v1.fn.detect_checkpoint_activation` | 1625 | Position, Checkpoint, LastCheckpoint | Checkpoint, LastCheckpoint | CheckpointActivated | O(c) | Detecte collision joueur/checkpoint. Active le checkpoint. Met a jour LastCheckpoint |
| `process_respawn` | `mge.platformer.checkpoint.v1.fn.process_respawn` | 1626 | LastCheckpoint, RespawnState, Position | Position, Velocity, RespawnState | RespawnTriggered, RespawnCompleted | O(1) | Teleporte le joueur a la position du dernier checkpoint. Reset velocity. Active invincibilite |
| `tick_invincibility` | `mge.platformer.checkpoint.v1.fn.tick_invincibility` | 1627 | RespawnState | RespawnState | none | O(1) | Decremente invincible_ticks. Quand 0 → respawning = false |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `CheckpointActivated` | `mge.platformer.checkpoint.v1.event.checkpoint_activated` | `entity: EntityId, checkpoint: EntityId, order: u32` | `detect_checkpoint_activation` | ui (animation flag), audio |
| `RespawnTriggered` | `mge.platformer.checkpoint.v1.event.respawn_triggered` | `entity: EntityId, respawn_x: f32, respawn_y: f32` | `process_respawn` | camera (snap), ui (fade), audio |
| `RespawnCompleted` | `mge.platformer.checkpoint.v1.event.respawn_completed` | `entity: EntityId` | `process_respawn` | hazard (re-enable), ui |

---

## 7. Invariants

- Un seul Checkpoint peut etre `Active` a la fois. Les precedents passent a `Used`.
- `LastCheckpoint` contient toujours le checkpoint avec le `order` le plus eleve atteint.
- Pendant `RespawnState.respawning == true`, le joueur est invincible (hazards ignores).
- `RespawnState.invincible_ticks` ne remonte jamais (decremente monotone).
- Si `LastCheckpoint.checkpoint_entity == None`, respawn au spawn initial du niveau.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `invincibility_ticks` | `u32` | 120 | [30, 300] | Duree invincibilite apres respawn (2s@60fps) |
| `checkpoint_activation_radius` | `f32` | 16.0 | [8.0, 64.0] | Rayon de detection checkpoint |
| `respawn_fade_ticks` | `u32` | 30 | [0, 60] | Duree du fade avant teleportation |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Detecte l'activation des checkpoints | Ne gere pas les collisions (→ collision) |
| Gere le respawn et l'invincibilite | Ne cause pas la mort (→ hazard) |
| Sauvegarde la position de respawn | Ne sauvegarde pas la progression (→ save-load) |
| Teleporte le joueur au checkpoint | Ne gere pas la camera (→ camera) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Position, Checkpoint, LastCheckpoint, RespawnState |
| Ecrit | Checkpoint, LastCheckpoint, RespawnState, Position, Velocity |
| Emet | CheckpointActivated, RespawnTriggered, RespawnCompleted |
| Ne touche jamais | PlatformerMovement, JumpAbility, Platform, HazardZone, CameraTarget |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-pl-checkpoint/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.platformer.checkpoint.v1
    ├── components.rs     # Checkpoint, LastCheckpoint, RespawnState
    ├── systems.rs        # detect_checkpoint_activation, process_respawn, tick_invincibility
    └── events.rs         # CheckpointActivated, RespawnTriggered, RespawnCompleted
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (CheckpointState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : activation, respawn teleport, invincibility decay, order progression
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.platformer.checkpoint.v1","k":"p","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.checkpoint.v1.component.checkpoint","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.checkpoint.v1.component.last_checkpoint","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.checkpoint.v1.component.respawn_state","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.checkpoint.v1.fn.detect_checkpoint_activation","k":"s","d":"platformer","r":["Position","Checkpoint","LastCheckpoint"],"w":["Checkpoint","LastCheckpoint"],"e":["CheckpointActivated"],"p":1625,"c":"O(c)"},
  {"i":"mge.platformer.checkpoint.v1.fn.process_respawn","k":"s","d":"platformer","r":["LastCheckpoint","RespawnState","Position"],"w":["Position","Velocity","RespawnState"],"e":["RespawnTriggered","RespawnCompleted"],"p":1626,"c":"O(1)"},
  {"i":"mge.platformer.checkpoint.v1.fn.tick_invincibility","k":"s","d":"platformer","r":["RespawnState"],"w":["RespawnState"],"e":[],"p":1627,"c":"O(1)"},
  {"i":"mge.platformer.checkpoint.v1.event.checkpoint_activated","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.checkpoint.v1.event.respawn_triggered","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.checkpoint.v1.event.respawn_completed","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let cp = world.spawn();
world.insert(cp, Checkpoint { position_x: 500.0, position_y: 300.0, state: CheckpointState::Inactive, order: 1 });

let player = world.spawn();
world.insert(player, LastCheckpoint { checkpoint_entity: None, respawn_x: 0.0, respawn_y: 0.0 });
world.insert(player, RespawnState { respawning: false, invincible_ticks: 0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Platformer - Index](_index.md) | Vue d'ensemble du pack |
| [mge-pl-collision](mge-pl-collision.md) | Plugin collision (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
