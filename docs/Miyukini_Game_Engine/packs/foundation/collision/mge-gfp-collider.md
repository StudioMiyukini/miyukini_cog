# mge-gfp-collider

> @id mge.foundation.collider.v1  
> @role plugin  
> @domain foundation  
> @do collision_shapes_aabb_circle_capsule  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-collider` |
| @id MSCM | `mge.foundation.collider.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-transform`, `mge-gfp-bounds` |
| Hot path | Oui |
| Headless safe | Oui |
| Complexite globale | O(n) |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ColliderShape` | `AABB, Circle, Capsule` | Type de forme geometrique du collider. Determine l'algorithme de detection utilise en narrow phase |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Collider` | `mge.foundation.collider.v1.component.collider` | `shape: ColliderShape, offset_x: f32, offset_y: f32, is_trigger: bool` | Collider principal. Definit la forme, le decalage par rapport au Transform et le mode trigger (pas de reponse physique) |
| `ColliderAABB` | `mge.foundation.collider.v1.component.collider_aabb` | `half_width: f32, half_height: f32` | Parametres specifiques a la forme AABB. Demi-largeur et demi-hauteur |
| `ColliderCircle` | `mge.foundation.collider.v1.component.collider_circle` | `radius: f32` | Parametres specifiques a la forme cercle. Rayon |
| `ColliderCapsule` | `mge.foundation.collider.v1.component.collider_capsule` | `radius: f32, height: f32` | Parametres specifiques a la forme capsule. Rayon et hauteur du segment central |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `sync_collider_transform` | `mge.foundation.collider.v1.fn.sync_collider_transform` | 120 | Collider, WorldTransform | Collider | none | O(n) | Synchronise la position monde du collider a partir du WorldTransform de l'entite. Applique l'offset du collider |

---

## 5. Flux de donnees

```
WorldTransform ──► sync_collider_transform ──► Collider (position monde mise a jour)
                         │
                         ├── lit ColliderAABB / ColliderCircle / ColliderCapsule
                         │
                         └── position monde = WorldTransform.position + Collider.offset
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- Un `Collider` doit toujours avoir exactement un composant de forme associe (`ColliderAABB`, `ColliderCircle` ou `ColliderCapsule`).
- `ColliderAABB.half_width` et `ColliderAABB.half_height` sont strictement positifs.
- `ColliderCircle.radius` est strictement positif.
- `ColliderCapsule.radius` et `ColliderCapsule.height` sont strictement positifs.
- La position monde du collider est recalculee a chaque tick par `sync_collider_transform`. Jamais mise en cache entre ticks.
- Un collider avec `is_trigger = true` ne genere pas de reponse physique (penetration ignoree).

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Definit les formes de collision (AABB, Circle, Capsule) | Ne detecte pas les collisions (→ collision-detection) |
| Synchronise la position monde du collider | Ne resout pas la penetration (→ physics-basic) |
| Fournit le flag trigger | Ne gere pas les couches de collision (→ layer-mask) |
| Stocke les parametres geometriques | Ne fait pas de raycast (→ raycast) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | WorldTransform, ColliderAABB, ColliderCircle, ColliderCapsule |
| Ecrit | Collider (position monde) |
| Emet | Aucun |
| Ne touche jamais | Velocity2D, PhysicsBody, CollisionPair, CollisionManifold, Camera2D |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-collider/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs           # @id mge.foundation.collider.v1, trait Plugin impl
    ├── components.rs    # Collider, ColliderAABB, ColliderCircle, ColliderCapsule
    ├── systems.rs       # sync_collider_transform
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
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 1 enum `ColliderShape` (AABB, Circle, Capsule)
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` vide (aucun evenement)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : sync AABB, sync Circle, sync Capsule, offset application, trigger flag
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.collider.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collider.v1.component.collider","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collider.v1.component.collider_aabb","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collider.v1.component.collider_circle","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collider.v1.component.collider_capsule","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.collider.v1.fn.sync_collider_transform","k":"s","d":"foundation","r":["Collider","WorldTransform"],"w":["Collider"],"e":[],"p":120,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let wall = world.spawn();
world.insert(wall, Collider {
    shape: ColliderShape::AABB,
    offset_x: 0.0,
    offset_y: 0.0,
    is_trigger: false,
});
world.insert(wall, ColliderAABB {
    half_width: 16.0,
    half_height: 8.0,
});

let player = world.spawn();
world.insert(player, Collider {
    shape: ColliderShape::Circle,
    offset_x: 0.0,
    offset_y: 2.0,
    is_trigger: false,
});
world.insert(player, ColliderCircle { radius: 6.0 });

let trigger_zone = world.spawn();
world.insert(trigger_zone, Collider {
    shape: ColliderShape::AABB,
    offset_x: 0.0,
    offset_y: 0.0,
    is_trigger: true,
});
world.insert(trigger_zone, ColliderAABB {
    half_width: 32.0,
    half_height: 32.0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
