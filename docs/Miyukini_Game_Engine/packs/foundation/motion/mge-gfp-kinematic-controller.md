# mge-gfp-kinematic-controller

> @id mge.foundation.kinematic_controller.v1  
> @role plugin  
> @domain foundation  
> @do movement_with_collision_resolution_ground_wall  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-kinematic-controller` |
| @id MSCM | `mge.foundation.kinematic_controller.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-velocity`, `mge-gfp-collider`, `mge-gfp-collision-detection` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(n*c), n = entites kinematiques, c = colliders proches |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `GroundType` | `None, Solid, Slope, OneWay` | Type de surface sous l'entite. None = en l'air |
| `WallSide` | `None, Left, Right` | Cote du mur en contact. None = pas de mur |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `KinematicController` | `mge.foundation.kinematic_controller.v1.component.kinematic_controller` | `step_height: f32, skin_width: f32` | Configuration du controleur. step_height = hauteur max de marche auto-montee. skin_width = marge interne pour eviter le chevauchement |
| `GroundState` | `mge.foundation.kinematic_controller.v1.component.ground_state` | `grounded: bool, ground_type: GroundType, ground_normal_x: f32, ground_normal_y: f32` | Etat du contact sol. grounded = true si en contact. ground_normal = normale de la surface |
| `WallState` | `mge.foundation.kinematic_controller.v1.component.wall_state` | `touching_wall: bool, wall_side: WallSide` | Etat du contact mur. touching_wall = true si en contact lateral |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `resolve_kinematic_step` | `mge.foundation.kinematic_controller.v1.fn.resolve_kinematic_step` | 114 | KinematicController, Velocity2D, Transform2D, Collider | Transform2D, Velocity2D, WallState | WallContactChanged | O(n*c) | Deplace l'entite selon la velocite. Resout les collisions par slide (projection le long des surfaces). Gere le step-up pour les petites marches |
| `update_ground_state` | `mge.foundation.kinematic_controller.v1.fn.update_ground_state` | 115 | KinematicController, Transform2D, Collider | GroundState | GroundLanded, GroundLeft | O(n*c) | Lance un raycast vers le bas (skin_width) pour detecter le sol. Met a jour grounded, ground_type et ground_normal |

---

## 5. Flux de donnees

```
Velocity2D + Transform2D + Collider + KinematicController
              │
              ▼
 ┌────────────────────────────┐
 │  resolve_kinematic_step    │  Phase 114
 │  (move + collision slide)  │
 └─────────────┬──────────────┘
               │
               ▼
   Transform2D' (position corrigee)
   Velocity2D' (projete sur surfaces)
   WallState (contact mur)
   WallContactChanged (event)
               │
               ▼
 ┌────────────────────────────┐
 │   update_ground_state      │  Phase 115
 │  (raycast sol → grounded)  │
 └─────────────┬──────────────┘
               │
               ▼
   GroundState (grounded, type, normal)
   GroundLanded / GroundLeft (events)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `GroundLanded` | `mge.foundation.kinematic_controller.v1.event.ground_landed` | `entity: EntityId, ground_type: GroundType, ground_normal_x: f32, ground_normal_y: f32` | `update_ground_state` | animation (atterrissage), audio (impact), particules |
| `GroundLeft` | `mge.foundation.kinematic_controller.v1.event.ground_left` | `entity: EntityId, last_ground_type: GroundType` | `update_ground_state` | jump (coyote time), animation (chute) |
| `WallContactChanged` | `mge.foundation.kinematic_controller.v1.event.wall_contact_changed` | `entity: EntityId, touching_wall: bool, wall_side: WallSide` | `resolve_kinematic_step` | wall jump, wall slide, animation |

---

## 7. Invariants

- `resolve_kinematic_step` ne permet jamais de penetrer un collider solide (separation garantie par skin_width).
- Le step-up ne s'applique que si l'obstacle est inferieur a `step_height` et qu'il y a de l'espace au-dessus.
- `GroundState.grounded` est coherent avec la detection par raycast vers le bas (distance <= skin_width).
- La velocite est projetee (slide) et jamais inversee par le resolve. L'inversion est la responsabilite du plugin bounce.
- `GroundLanded` et `GroundLeft` ne sont emis qu'aux transitions (pas chaque tick).
- `WallContactChanged` n'est emis qu'aux transitions de contact mur.
- Les plateformes `OneWay` ne bloquent que la composante descendante (vy > 0 vers le bas).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_step_height` | `f32` | 4.0 | [0.0, 32.0] | Hauteur de marche auto-montee par defaut |
| `default_skin_width` | `f32` | 0.1 | [0.01, 1.0] | Marge interne de separation par defaut |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Deplace avec resolution de collision (slide) | Ne detecte pas les collisions brutes (→ collision-detection) |
| Detecte l'etat du sol (grounded, type, normal) | Ne gere pas la gravite (→ gravity) |
| Detecte le contact mur (side) | Ne gere pas le saut (→ genre pack) |
| Gere le step-up pour petites marches | Ne gere pas le wall-jump / wall-slide (→ genre pack) |
| Supporte les plateformes one-way | Ne gere pas les plateformes mobiles (→ extension) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | KinematicController, Velocity2D, Transform2D, Collider (collision-detection) |
| Ecrit | Transform2D, Velocity2D, GroundState, WallState |
| Emet | GroundLanded, GroundLeft, WallContactChanged |
| Ne touche jamais | Acceleration2D, DirectionalInput, Camera2D, PhysicsBody, GravityAffected |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-kinematic-controller/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.kinematic_controller.v1, trait Plugin impl
    ├── components.rs     # KinematicController, GroundState, WallState
    ├── systems.rs        # resolve_kinematic_step, update_ground_state
    └── events.rs         # GroundLanded, GroundLeft, WallContactChanged
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No allocation hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 2 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (GroundType, WallSide)
- [ ] Collision slide correcte (projection vecteur)
- [ ] Step-up fonctionnel
- [ ] OneWay platform support
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : slide, step-up, ground detection, wall detection, one-way, transitions events
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.kinematic_controller.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.kinematic_controller.v1.component.kinematic_controller","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.kinematic_controller.v1.component.ground_state","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.kinematic_controller.v1.component.wall_state","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.kinematic_controller.v1.fn.resolve_kinematic_step","k":"s","d":"foundation","r":["KinematicController","Velocity2D","Transform2D","Collider"],"w":["Transform2D","Velocity2D","WallState"],"e":["WallContactChanged"],"p":114,"c":"O(n*c)"},
  {"i":"mge.foundation.kinematic_controller.v1.fn.update_ground_state","k":"s","d":"foundation","r":["KinematicController","Transform2D","Collider"],"w":["GroundState"],"e":["GroundLanded","GroundLeft"],"p":115,"c":"O(n*c)"},
  {"i":"mge.foundation.kinematic_controller.v1.event.ground_landed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.kinematic_controller.v1.event.ground_left","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.kinematic_controller.v1.event.wall_contact_changed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, Transform2D { x: 50.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
world.insert(player, Velocity2D { vx: 80.0, vy: -200.0 });
world.insert(player, KinematicController { step_height: 4.0, skin_width: 0.1 });
world.insert(player, GroundState {
    grounded: false, ground_type: GroundType::None,
    ground_normal_x: 0.0, ground_normal_y: 0.0,
});
world.insert(player, WallState { touching_wall: false, wall_side: WallSide::None });
// Apres Phase 114 : position corrigee, velocite projetee sur surfaces
// Apres Phase 115 : GroundState.grounded = true si sol detecte
//                   → GroundLanded emis a la transition air→sol
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-velocity](mge-gfp-velocity.md) | Plugin velocity (fournit Velocity2D) |
| [mge-gfp-directional-move](mge-gfp-directional-move.md) | Plugin directional move (genere la velocite d'entree) |
