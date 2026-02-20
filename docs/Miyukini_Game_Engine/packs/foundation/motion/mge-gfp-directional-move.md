# mge-gfp-directional-move

> @id mge.foundation.directional_move.v1  
> @role plugin  
> @domain foundation  
> @do convert_directional_input_to_velocity  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-directional-move` |
| @id MSCM | `mge.foundation.directional_move.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-velocity` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(n), n = entites avec DirectionalInput |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `DirectionalInput` | `mge.foundation.directional_move.v1.component.directional_input` | `dir_x: f32, dir_y: f32` | Direction brute d'entree. Valeurs typiquement dans [-1.0, 1.0] par axe |
| `MoveIntent` | `mge.foundation.directional_move.v1.component.move_intent` | `intent_x: f32, intent_y: f32` | Direction normalisee apres traitement. Magnitude <= 1.0 |
| `MaxSpeed` | `mge.foundation.directional_move.v1.component.max_speed` | `value: f32` | Vitesse maximale de l'entite en unites/seconde |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `read_directional_input` | `mge.foundation.directional_move.v1.fn.read_directional_input` | 112 | DirectionalInput | MoveIntent | none | O(n) | Normalise le vecteur DirectionalInput → MoveIntent. Si magnitude > 1.0, normalise a 1.0 |
| `apply_move_intent` | `mge.foundation.directional_move.v1.fn.apply_move_intent` | 113 | MoveIntent, MaxSpeed | Velocity2D | MoveIntentChanged | O(n) | Calcule velocity = intent * max_speed. Ecrase la velocite actuelle avec le resultat |

---

## 5. Flux de donnees

```
DirectionalInput (dir_x, dir_y)
              │
              ▼
 ┌──────────────────────────┐
 │  read_directional_input  │  Phase 112
 │  (normalise → intent)    │
 └────────────┬─────────────┘
              │
              ▼
     MoveIntent (intent_x, intent_y)
              │
              │ + MaxSpeed.value
              ▼
 ┌──────────────────────────┐
 │   apply_move_intent      │  Phase 113
 │  (intent * speed → vel)  │
 └────────────┬─────────────┘
              │
              ▼
     Velocity2D (vx, vy)
     MoveIntentChanged (event)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `MoveIntentChanged` | `mge.foundation.directional_move.v1.event.move_intent_changed` | `entity: EntityId, old_intent_x: f32, old_intent_y: f32, new_intent_x: f32, new_intent_y: f32` | `apply_move_intent` | animation (idle/walk/run), audio (pas) |

---

## 7. Invariants

- `MoveIntent` a toujours une magnitude <= 1.0 apres Phase 112.
- `MaxSpeed.value` doit etre >= 0.0. Une valeur de 0.0 immobilise l'entite.
- `apply_move_intent` ecrase `Velocity2D`, il ne l'accumule pas. C'est un choix de conception pour les entites controlees par input.
- `MoveIntentChanged` n'est emis que si l'intent a effectivement change (delta > epsilon).
- Les entites sans `DirectionalInput` ne sont pas affectees.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Normalise l'input directionnel | Ne lit pas les peripheriques (→ input layer) |
| Convertit intent * speed en velocite | Ne gere pas l'acceleration progressive (→ acceleration) |
| Borne la vitesse via MaxSpeed | Ne resout pas les collisions (→ kinematic-controller) |
| Emet MoveIntentChanged | Ne gere pas la friction (→ friction) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | DirectionalInput, MoveIntent, MaxSpeed |
| Ecrit | MoveIntent, Velocity2D |
| Emet | MoveIntentChanged |
| Ne touche jamais | Transform2D, Acceleration2D, Collider, GroundState, KinematicController |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-directional-move/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.directional_move.v1, trait Plugin impl
    ├── components.rs     # DirectionalInput, MoveIntent, MaxSpeed
    ├── systems.rs        # read_directional_input, apply_move_intent
    └── events.rs         # MoveIntentChanged
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
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] Normalisation correcte (eviter division par zero)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : normalisation, intent zero, max speed, event emission
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.directional_move.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.directional_move.v1.component.directional_input","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.directional_move.v1.component.move_intent","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.directional_move.v1.component.max_speed","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.directional_move.v1.fn.read_directional_input","k":"s","d":"foundation","r":["DirectionalInput"],"w":["MoveIntent"],"e":[],"p":112,"c":"O(n)"},
  {"i":"mge.foundation.directional_move.v1.fn.apply_move_intent","k":"s","d":"foundation","r":["MoveIntent","MaxSpeed"],"w":["Velocity2D"],"e":["MoveIntentChanged"],"p":113,"c":"O(n)"},
  {"i":"mge.foundation.directional_move.v1.event.move_intent_changed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, DirectionalInput { dir_x: 1.0, dir_y: 0.5 });
world.insert(player, MoveIntent { intent_x: 0.0, intent_y: 0.0 });
world.insert(player, MaxSpeed { value: 200.0 });
world.insert(player, Velocity2D { vx: 0.0, vy: 0.0 });
// Apres Phase 112 : MoveIntent = normalize(1.0, 0.5) ≈ (0.894, 0.447)
// Apres Phase 113 : Velocity2D = (0.894 * 200, 0.447 * 200) ≈ (178.9, 89.4)
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-velocity](mge-gfp-velocity.md) | Plugin velocity (ecrit par directional-move) |
| [mge-gfp-kinematic-controller](mge-gfp-kinematic-controller.md) | Plugin kinematic (resolution collision apres move) |
