# mge-gfp-acceleration

> @id mge.foundation.acceleration.v1  
> @role plugin  
> @domain foundation  
> @do apply_acceleration_to_velocity  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-acceleration` |
| @id MSCM | `mge.foundation.acceleration.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-velocity` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(n), n = entites avec Acceleration2D |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Acceleration2D` | `mge.foundation.acceleration.v1.component.acceleration2d` | `ax: f32, ay: f32` | Vecteur acceleration en unites/seconde². Applique a la velocite chaque tick |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `apply_acceleration` | `mge.foundation.acceleration.v1.fn.apply_acceleration` | 111 | Acceleration2D, Velocity2D | Velocity2D | none | O(n) | Applique velocity += acceleration * dt. Modifie Velocity2D.vx et Velocity2D.vy |

---

## 5. Flux de donnees

```
Acceleration2D (ax, ay)  +  Velocity2D (vx, vy)  +  dt
                │                    │                 │
                └─────────┬──────────┘─────────────────┘
                          ▼
                ┌──────────────────────┐
                │  apply_acceleration  │  Phase 111
                │  vel += accel * dt   │
                └──────────┬───────────┘
                           │
                           ▼
                  Velocity2D (vx', vy')
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `apply_acceleration` ne modifie que `Velocity2D.vx` et `Velocity2D.vy`.
- Si `Acceleration2D` est (0.0, 0.0), la velocite reste identique.
- Le systeme ne borne pas la velocite. Le clamping est la responsabilite de `directional-move` ou `kinematic-controller`.
- L'integration est Euler explicite, identique a `apply_velocity`.
- L'acceleration est appliquee avant la velocite dans l'ordre des phases (111 avant 110 au tick suivant).

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Applique l'acceleration a la velocite | Ne gere pas la position (→ velocity) |
| Fournit le composant Acceleration2D | Ne borne pas la vitesse maximale (→ directional-move) |
| Integration Euler explicite | Ne gere pas la gravite (→ gravity) |
| Modifie uniquement vx et vy | Ne gere pas la friction (→ friction) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Acceleration2D, Velocity2D |
| Ecrit | Velocity2D (vx, vy) |
| Emet | rien |
| Ne touche jamais | Transform2D, Collider, AABB, GravityAffected, FrictionCoefficient |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-acceleration/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.acceleration.v1, trait Plugin impl
    ├── components.rs     # Acceleration2D
    ├── systems.rs        # apply_acceleration
    └── events.rs         # (vide)
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
- [ ] 1 composant dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : apply acceleration, zero accel, negative accel, dt variable
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.acceleration.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.acceleration.v1.component.acceleration2d","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.acceleration.v1.fn.apply_acceleration","k":"s","d":"foundation","r":["Acceleration2D","Velocity2D"],"w":["Velocity2D"],"e":[],"p":111,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Velocity2D { vx: 10.0, vy: 0.0 });
world.insert(entity, Acceleration2D { ax: 5.0, ay: -9.81 });
// Apres 1 tick (dt = 1/60) :
// Velocity2D.vx = 10.0 + 5.0 * (1.0/60.0) ≈ 10.083
// Velocity2D.vy = 0.0 + (-9.81) * (1.0/60.0) ≈ -0.164
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-velocity](mge-gfp-velocity.md) | Plugin velocity (ecrit par acceleration) |
| [mge-gfp-directional-move](mge-gfp-directional-move.md) | Plugin directional move (alternative haut niveau) |
