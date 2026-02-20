# mge-pl-movement

> @id mge.platformer.movement.v1  
> @role plugin  
> @domain platformer  
> @do manage_horizontal_movement_acceleration_friction  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-pl-movement` |
| @id MSCM | `mge.platformer.movement.v1` |
| Domaine | platformer |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-plugin-input` |
| Hot path | Oui (chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n) sur entites mobiles |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `FacingDirection` | `Left, Right` | Direction ou l'entite regarde |
| `GroundState` | `Grounded, Airborne, OnSlope` | Etat de contact avec le sol |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `PlatformerMovement` | `mge.platformer.movement.v1.component.platformer_movement` | `max_speed: f32, acceleration: f32, friction: f32, air_control: f32, facing: FacingDirection` | Parametres de mouvement. air_control = ratio acceleration en l'air (0.0-1.0) |
| `GroundDetection` | `mge.platformer.movement.v1.component.ground_detection` | `ground_state: GroundState, ground_normal: (f32, f32), ground_entity: Option<EntityId>` | Detection du sol. ground_entity = plateforme en contact |
| `MovementInput` | `mge.platformer.movement.v1.component.movement_input` | `horizontal: f32, jump_pressed: bool, jump_held: bool` | Input joueur normalise. horizontal dans [-1.0, 1.0] |

---

## 4. Formules

```
Grounded:
  velocity.x += horizontal * acceleration * dt
  velocity.x *= (1.0 - friction * dt)
  velocity.x = clamp(velocity.x, -max_speed, max_speed)

Airborne:
  velocity.x += horizontal * acceleration * air_control * dt
  velocity.x = clamp(velocity.x, -max_speed, max_speed)

Facing:
  if horizontal > 0.0 → facing = Right
  if horizontal < 0.0 → facing = Left
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `read_movement_input` | `mge.platformer.movement.v1.fn.read_movement_input` | 1600 | InputState (core) | MovementInput | none | O(n) | Lit l'input et ecrit dans MovementInput. Normalise horizontal |
| `apply_horizontal_movement` | `mge.platformer.movement.v1.fn.apply_horizontal_movement` | 1601 | MovementInput, PlatformerMovement, GroundDetection, Velocity | Velocity, PlatformerMovement | DirectionChanged | O(n) | Applique acceleration selon ground state. Met a jour facing |
| `apply_friction` | `mge.platformer.movement.v1.fn.apply_friction` | 1602 | PlatformerMovement, GroundDetection, Velocity | Velocity | GroundStateChanged | O(n) | Applique friction si Grounded et pas d'input. Detecte transitions ground state |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `GroundStateChanged` | `mge.platformer.movement.v1.event.ground_state_changed` | `entity: EntityId, old_state: GroundState, new_state: GroundState` | `apply_friction` | jump (coyote time), animation, audio |
| `DirectionChanged` | `mge.platformer.movement.v1.event.direction_changed` | `entity: EntityId, new_facing: FacingDirection` | `apply_horizontal_movement` | animation (flip sprite) |

---

## 7. Invariants

- `Velocity.x` ne depasse jamais `max_speed` en valeur absolue apres clamping.
- `MovementInput.horizontal` est dans [-1.0, 1.0].
- `PlatformerMovement.air_control` est dans [0.0, 1.0].
- `GroundDetection` est mis a jour par le systeme de collision, pas par movement.
- La friction ne s'applique que si `GroundState == Grounded` et `horizontal == 0.0`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_max_speed` | `f32` | 8.0 | [1.0, 30.0] | Vitesse horizontale max |
| `default_acceleration` | `f32` | 40.0 | [10.0, 200.0] | Acceleration au sol |
| `default_friction` | `f32` | 15.0 | [1.0, 50.0] | Friction au sol (deceleration) |
| `default_air_control` | `f32` | 0.6 | [0.0, 1.0] | Ratio controle en l'air |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Lit l'input et calcule la velocite horizontale | Ne gere pas le saut (→ jump) |
| Applique acceleration et friction | Ne resout pas les collisions (→ collision) |
| Detecte la direction (facing) | Ne gere pas la camera (→ camera) |
| Respecte air_control en l'air | Ne gere pas la gravite (→ core physics) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | InputState (core), MovementInput, PlatformerMovement, GroundDetection, Velocity |
| Ecrit | MovementInput, Velocity, PlatformerMovement (facing) |
| Emet | GroundStateChanged, DirectionChanged |
| Ne touche jamais | JumpAbility, Platform, CameraTarget, Checkpoint, HazardZone |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-pl-movement/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.platformer.movement.v1, trait Plugin impl
    ├── components.rs     # PlatformerMovement, GroundDetection, MovementInput
    ├── systems.rs        # read_movement_input, apply_horizontal_movement, apply_friction
    └── events.rs         # GroundStateChanged, DirectionChanged
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (FacingDirection, GroundState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : acceleration, friction, air control, facing, ground transitions
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.platformer.movement.v1","k":"p","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.movement.v1.component.platformer_movement","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.movement.v1.component.ground_detection","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.movement.v1.component.movement_input","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.movement.v1.fn.read_movement_input","k":"s","d":"platformer","r":["InputState"],"w":["MovementInput"],"e":[],"p":1600,"c":"O(n)"},
  {"i":"mge.platformer.movement.v1.fn.apply_horizontal_movement","k":"s","d":"platformer","r":["MovementInput","PlatformerMovement","GroundDetection","Velocity"],"w":["Velocity","PlatformerMovement"],"e":["DirectionChanged"],"p":1601,"c":"O(n)"},
  {"i":"mge.platformer.movement.v1.fn.apply_friction","k":"s","d":"platformer","r":["PlatformerMovement","GroundDetection","Velocity"],"w":["Velocity"],"e":["GroundStateChanged"],"p":1602,"c":"O(n)"},
  {"i":"mge.platformer.movement.v1.event.ground_state_changed","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.movement.v1.event.direction_changed","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, PlatformerMovement {
    max_speed: 8.0, acceleration: 40.0, friction: 15.0, air_control: 0.6,
    facing: FacingDirection::Right,
});
world.insert(player, GroundDetection { ground_state: GroundState::Grounded, ground_normal: (0.0, 1.0), ground_entity: None });
world.insert(player, MovementInput { horizontal: 0.0, jump_pressed: false, jump_held: false });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Platformer - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
