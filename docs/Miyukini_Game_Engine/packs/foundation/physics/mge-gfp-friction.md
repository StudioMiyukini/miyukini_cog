# mge-gfp-friction

> @id mge.foundation.friction.v1  
> @role plugin  
> @domain foundation  
> @do ground_air_friction_velocity_damping  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-friction` |
| @id MSCM | `mge.foundation.friction.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-velocity`, `mge-gfp-kinematic-controller` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(n) |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `FrictionCoefficient` | `mge.foundation.friction.v1.component.friction_coefficient` | `ground: f32, air: f32` | Coefficients de friction de l'entite. `ground` = friction au sol (plus forte), `air` = friction en l'air (plus faible) |
| `SurfaceFriction` | `mge.foundation.friction.v1.component.surface_friction` | `value: f32` | Friction de la surface sur laquelle l'entite se deplace. Multiplie le coefficient de friction de l'entite |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `apply_ground_friction` | `mge.foundation.friction.v1.fn.apply_ground_friction` | 132 | FrictionCoefficient, SurfaceFriction, GroundState, Velocity2D | Velocity2D | none | O(n) | Pour les entites au sol (GroundState.grounded=true), applique `velocity.x *= ground_friction * surface_friction`. Amortit la vitesse horizontale |
| `apply_air_friction` | `mge.foundation.friction.v1.fn.apply_air_friction` | 132 | FrictionCoefficient, GroundState, Velocity2D | Velocity2D | none | O(n) | Pour les entites en l'air (GroundState.grounded=false), applique `velocity.x *= air_friction`. Amortissement aerien plus faible |

---

## 5. Flux de donnees

```
GroundState ──────────────┐
FrictionCoefficient ─────►│
SurfaceFriction ─────────►│ apply_ground_friction ──► Velocity2D (amorti au sol)
Velocity2D ──────────────►│
                          │
GroundState ──────────────┐
FrictionCoefficient ─────►│ apply_air_friction ──► Velocity2D (amorti en l'air)
Velocity2D ──────────────►│
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `FrictionCoefficient.ground` est borne entre 0.0 (aucune friction) et 1.0 (arret instantane).
- `FrictionCoefficient.air` est borne entre 0.0 et 1.0. Typiquement plus proche de 1.0 (peu de friction).
- `SurfaceFriction.value` est borne entre 0.0 (glace) et 2.0 (surface tres rugueuse).
- Si `SurfaceFriction` est absent, la valeur par defaut 1.0 est utilisee.
- Si `FrictionCoefficient` est absent, les valeurs GCL par defaut sont utilisees.
- La friction est multiplicative : `vel *= coefficient`. Pas additive.
- La friction sol n'affecte que la composante horizontale (x) de la velocite.
- La friction air n'affecte que la composante horizontale (x) de la velocite.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_ground_friction` | `f32` | 0.85 | [0.0, 1.0] | Coefficient de friction sol par defaut. 0.85 = deceleration progressive |
| `default_air_friction` | `f32` | 0.98 | [0.0, 1.0] | Coefficient de friction air par defaut. 0.98 = tres peu de friction |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Applique la friction sol (amortissement fort) | Ne detecte pas le sol (→ kinematic-controller) |
| Applique la friction air (amortissement faible) | Ne gere pas la gravite (→ gravity) |
| Supporte la friction de surface variable | Ne resout pas les collisions (→ physics-basic) |
| Coefficients configurables par entite et par surface | Ne gere pas le rebond (→ bounce) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | FrictionCoefficient, SurfaceFriction, GroundState |
| Ecrit | Velocity2D |
| Emet | Aucun |
| Ne touche jamais | PhysicsBody, CollisionPair, CollisionManifold, GravityAffected, Bounciness, Camera2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-friction/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs           # @id mge.foundation.friction.v1, trait Plugin impl
    ├── components.rs    # FrictionCoefficient, SurfaceFriction
    ├── systems.rs       # apply_ground_friction, apply_air_friction
    └── events.rs        # (vide)
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
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 2 systemes dans `systems.rs` avec annotations completes
- [ ] `events.rs` vide (aucun evenement)
- [ ] Friction multiplicative (vel *= coeff)
- [ ] Distinction sol/air via GroundState
- [ ] Fallback vers valeurs GCL si composants absents
- [ ] SurfaceFriction optionnel (defaut 1.0)
- [ ] Parametres GCL exposes (default_ground_friction, default_air_friction)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : friction sol, friction air, surface glace, surface rugueuse, composants absents
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.friction.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.friction.v1.component.friction_coefficient","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.friction.v1.component.surface_friction","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.friction.v1.fn.apply_ground_friction","k":"s","d":"foundation","r":["FrictionCoefficient","SurfaceFriction","GroundState","Velocity2D"],"w":["Velocity2D"],"e":[],"p":132,"c":"O(n)"},
  {"i":"mge.foundation.friction.v1.fn.apply_air_friction","k":"s","d":"foundation","r":["FrictionCoefficient","GroundState","Velocity2D"],"w":["Velocity2D"],"e":[],"p":132,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, FrictionCoefficient {
    ground: 0.85,
    air: 0.98,
});
world.insert(player, Velocity2D { x: 200.0, y: 0.0 });
// Au sol : chaque tick, velocity.x *= 0.85 → deceleration progressive
// En l'air : chaque tick, velocity.x *= 0.98 → presque pas de friction

// Surface glacee
let ice_platform = world.spawn();
world.insert(ice_platform, SurfaceFriction { value: 0.1 });
// Quand le joueur est sur cette surface :
// velocity.x *= ground_friction * surface_friction = 0.85 * 0.1 = 0.085
// → quasiment pas de friction, le joueur glisse

// Surface rugueuse
let sand = world.spawn();
world.insert(sand, SurfaceFriction { value: 1.5 });
// velocity.x *= 0.85 * 1.5 = clampe → deceleration tres forte
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
