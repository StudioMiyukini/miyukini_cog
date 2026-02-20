# mge-gfp-velocity

> @id mge.foundation.velocity.v1  
> @role plugin  
> @domain foundation  
> @do apply_velocity_to_position  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-velocity` |
| @id MSCM | `mge.foundation.velocity.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-transform` |
| Hot path | Oui (mouvement de base) |
| Headless safe | Oui |
| Complexite globale | O(n), n = entites avec Velocity2D |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Velocity2D` | `mge.foundation.velocity.v1.component.velocity2d` | `vx: f32, vy: f32` | Vecteur vitesse en unites/seconde. Applique a la position chaque tick |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `apply_velocity` | `mge.foundation.velocity.v1.fn.apply_velocity` | 110 | Velocity2D, Transform2D | Transform2D | none | O(n) | Applique position += velocity * dt. Modifie Transform2D.x et Transform2D.y |

---

## 5. Flux de donnees

```
Velocity2D (vx, vy)  +  Transform2D (x, y)  +  dt
                │                │               │
                └────────┬───────┘───────────────┘
                         ▼
                ┌──────────────────┐
                │  apply_velocity  │  Phase 110
                │  pos += vel * dt │
                └────────┬─────────┘
                         │
                         ▼
                Transform2D (x', y')
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `apply_velocity` ne modifie que `Transform2D.x` et `Transform2D.y`. La rotation et l'echelle sont inchangees.
- Si `Velocity2D` est (0.0, 0.0), la position reste identique (pas de drift numerique).
- Le `dt` est fourni par le contexte du tick (fixe ou variable selon la configuration moteur).
- Le systeme ne borne pas la position. Le clamping est la responsabilite d'autres plugins.
- L'integration est Euler explicite : simple, deterministe, suffisant pour les cas GFP.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Applique la velocite a la position | Ne gere pas l'acceleration (→ acceleration) |
| Fournit le composant Velocity2D | Ne borne pas la vitesse (→ directional-move, kinematic) |
| Integration Euler explicite | Ne resout pas les collisions (→ kinematic-controller) |
| Modifie uniquement x et y | Ne gere pas la gravite (→ gravity) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Velocity2D, Transform2D |
| Ecrit | Transform2D (x, y) |
| Emet | rien |
| Ne touche jamais | Acceleration2D, Collider, AABB, Camera2D, WorldTransform |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-velocity/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.velocity.v1, trait Plugin impl
    ├── components.rs     # Velocity2D
    ├── systems.rs        # apply_velocity
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
- [ ] Tests unitaires : apply velocity, zero velocity, negative velocity, dt variable
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.velocity.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.velocity.v1.component.velocity2d","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.velocity.v1.fn.apply_velocity","k":"s","d":"foundation","r":["Velocity2D","Transform2D"],"w":["Transform2D"],"e":[],"p":110,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Transform2D { x: 0.0, y: 0.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
world.insert(entity, Velocity2D { vx: 100.0, vy: -50.0 });
// Apres 1 tick (dt = 1/60) :
// Transform2D.x = 0.0 + 100.0 * (1.0/60.0) ≈ 1.667
// Transform2D.y = 0.0 + (-50.0) * (1.0/60.0) ≈ -0.833
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-transform](../spatial/mge-gfp-transform.md) | Plugin transform (fournit Transform2D a modifier) |
| [mge-gfp-acceleration](mge-gfp-acceleration.md) | Plugin acceleration (modifie Velocity2D) |
