# mge-pl-jump

> @id mge.platformer.jump.v1  
> @role plugin  
> @domain platformer  
> @do manage_jump_double_jump_coyote_time_variable_height  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-pl-jump` |
| @id MSCM | `mge.platformer.jump.v1` |
| Domaine | platformer |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-pl-movement`, `mge-plugin-spatial` |
| Hot path | Oui (chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n) sur entites avec JumpAbility |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `JumpState` | `Grounded, Ascending, Falling` | Phase du saut |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `JumpAbility` | `mge.platformer.jump.v1.component.jump_ability` | `jump_force: f32, max_jumps: u32, current_jumps: u32, state: JumpState` | Capacite de saut. max_jumps = 2 pour double saut |
| `CoyoteTime` | `mge.platformer.jump.v1.component.coyote_time` | `grace_ticks: u32, remaining_ticks: u32` | Fenetre de grace apres quitter le sol. Permet saut tardif |
| `JumpBuffer` | `mge.platformer.jump.v1.component.jump_buffer` | `buffer_ticks: u32, remaining_ticks: u32` | Buffer d'input. Saut presse avant atterrissage → execute a l'atterrissage |
| `VariableJump` | `mge.platformer.jump.v1.component.variable_jump` | `min_height_ratio: f32, gravity_multiplier_fall: f32` | Saut variable. Relacher tot → gravite augmentee |

---

## 4. Formules

```
jump_velocity = sqrt(2.0 * jump_force * gravity)

Coyote time:
  Quand GroundState passe de Grounded a Airborne (sans saut) → remaining = grace_ticks
  remaining -= 1 par tick. Si > 0, le saut est encore autorise

Jump buffer:
  Quand jump_pressed et Airborne → remaining = buffer_ticks
  remaining -= 1 par tick. Si > 0 et GroundState → Grounded, execute le saut

Variable jump:
  Si Ascending et !jump_held → velocity.y *= min_height_ratio, gravity *= gravity_multiplier_fall
  Si Falling → gravity *= gravity_multiplier_fall (chute plus rapide)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_coyote_time` | `mge.platformer.jump.v1.fn.update_coyote_time` | 1605 | CoyoteTime, GroundDetection | CoyoteTime | none | O(n) | Decremente remaining. Reset a grace_ticks quand quitte le sol sans saut |
| `process_jump_input` | `mge.platformer.jump.v1.fn.process_jump_input` | 1606 | MovementInput, JumpAbility, CoyoteTime, JumpBuffer, GroundDetection | JumpAbility, JumpBuffer | none | O(n) | Determine si le saut est valide (coyote, buffer, current_jumps < max_jumps) |
| `apply_jump_force` | `mge.platformer.jump.v1.fn.apply_jump_force` | 1607 | JumpAbility | Velocity, JumpAbility | JumpStarted, DoubleJumped | O(n) | Applique jump_velocity. Incremente current_jumps. Emet JumpStarted ou DoubleJumped |
| `apply_variable_gravity` | `mge.platformer.jump.v1.fn.apply_variable_gravity` | 1608 | JumpAbility, VariableJump, MovementInput, Velocity | Velocity | JumpLanded | O(n) | Ajuste la gravite selon l'etat. Detecte l'atterrissage (JumpLanded) |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `JumpStarted` | `mge.platformer.jump.v1.event.jump_started` | `entity: EntityId, jump_number: u32` | `apply_jump_force` | animation, audio (jump sfx) |
| `JumpLanded` | `mge.platformer.jump.v1.event.jump_landed` | `entity: EntityId, landing_velocity: f32` | `apply_variable_gravity` | animation, audio (land sfx), camera (shake) |
| `DoubleJumped` | `mge.platformer.jump.v1.event.double_jumped` | `entity: EntityId` | `apply_jump_force` | animation (flip), audio, ui |

---

## 7. Invariants

- `JumpAbility.current_jumps` ne depasse jamais `max_jumps`.
- `current_jumps` est reset a 0 quand `GroundState` passe a `Grounded`.
- `CoyoteTime.remaining_ticks` ne depasse jamais `grace_ticks`.
- Le coyote time ne s'active que si l'entite quitte le sol SANS sauter (tomber d'un rebord).
- `VariableJump.gravity_multiplier_fall` est toujours >= 1.0.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_jump_force` | `f32` | 15.0 | [5.0, 50.0] | Force de saut |
| `default_max_jumps` | `u32` | 2 | [1, 5] | Nombre max de sauts (1 = pas de double saut) |
| `coyote_time_ticks` | `u32` | 6 | [0, 15] | Fenetre coyote time en ticks |
| `jump_buffer_ticks` | `u32` | 6 | [0, 15] | Buffer input saut en ticks |
| `fall_gravity_multiplier` | `f32` | 2.5 | [1.0, 5.0] | Multiplicateur gravite en chute |
| `min_jump_height_ratio` | `f32` | 0.4 | [0.1, 1.0] | Ratio hauteur minimum si relache tot |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le saut et le double saut | Ne gere pas le mouvement horizontal (→ movement) |
| Implemente coyote time et jump buffer | Ne resout pas les collisions (→ collision) |
| Applique le saut variable (hauteur) | Ne gere pas la camera (→ camera) |
| Detecte l'atterrissage | Ne gere pas les dommages (→ hazard) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | JumpAbility, CoyoteTime, JumpBuffer, VariableJump, MovementInput, GroundDetection, Velocity |
| Ecrit | JumpAbility, CoyoteTime, JumpBuffer, Velocity |
| Emet | JumpStarted, JumpLanded, DoubleJumped |
| Ne touche jamais | Platform, CameraTarget, Checkpoint, HazardZone, PlatformerMovement |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-pl-jump/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.platformer.jump.v1, trait Plugin impl
    ├── components.rs     # JumpAbility, CoyoteTime, JumpBuffer, VariableJump
    ├── systems.rs        # update_coyote_time, process_jump_input, apply_jump_force, apply_variable_gravity
    └── events.rs         # JumpStarted, JumpLanded, DoubleJumped
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
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (JumpState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : single jump, double jump, coyote time, jump buffer, variable jump, landing
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.platformer.jump.v1","k":"p","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.jump.v1.component.jump_ability","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.jump.v1.component.coyote_time","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.jump.v1.component.jump_buffer","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.jump.v1.component.variable_jump","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.jump.v1.fn.update_coyote_time","k":"s","d":"platformer","r":["CoyoteTime","GroundDetection"],"w":["CoyoteTime"],"e":[],"p":1605,"c":"O(n)"},
  {"i":"mge.platformer.jump.v1.fn.process_jump_input","k":"s","d":"platformer","r":["MovementInput","JumpAbility","CoyoteTime","JumpBuffer","GroundDetection"],"w":["JumpAbility","JumpBuffer"],"e":[],"p":1606,"c":"O(n)"},
  {"i":"mge.platformer.jump.v1.fn.apply_jump_force","k":"s","d":"platformer","r":["JumpAbility"],"w":["Velocity","JumpAbility"],"e":["JumpStarted","DoubleJumped"],"p":1607,"c":"O(n)"},
  {"i":"mge.platformer.jump.v1.fn.apply_variable_gravity","k":"s","d":"platformer","r":["JumpAbility","VariableJump","MovementInput","Velocity"],"w":["Velocity"],"e":["JumpLanded"],"p":1608,"c":"O(n)"},
  {"i":"mge.platformer.jump.v1.event.jump_started","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.jump.v1.event.jump_landed","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.jump.v1.event.double_jumped","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, JumpAbility { jump_force: 15.0, max_jumps: 2, current_jumps: 0, state: JumpState::Grounded });
world.insert(player, CoyoteTime { grace_ticks: 6, remaining_ticks: 0 });
world.insert(player, JumpBuffer { buffer_ticks: 6, remaining_ticks: 0 });
world.insert(player, VariableJump { min_height_ratio: 0.4, gravity_multiplier_fall: 2.5 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Platformer - Index](_index.md) | Vue d'ensemble du pack |
| [mge-pl-movement](mge-pl-movement.md) | Plugin movement (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
