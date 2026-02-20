# mge-gfp-bounds

> @id mge.foundation.bounds.v1  
> @role plugin  
> @domain foundation  
> @do aabb_computation_bounds_cache  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-bounds` |
| @id MSCM | `mge.foundation.bounds.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-transform` |
| Hot path | Oui (dependance collision) |
| Headless safe | Oui |
| Complexite globale | O(n) pour compute, O(d) pour cache update (d = dirty) |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `AABB` | `mge.foundation.bounds.v1.component.aabb` | `min_x: f32, min_y: f32, max_x: f32, max_y: f32` | Boite englobante alignee sur les axes. Calculee a partir de la transform et de la taille du collider |
| `BoundsCache` | `mge.foundation.bounds.v1.component.bounds_cache` | `dirty: bool, cached_aabb: AABB` | Cache d'AABB. dirty = true si la transform ou le collider a change depuis le dernier calcul |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `compute_aabb` | `mge.foundation.bounds.v1.fn.compute_aabb` | 106 | WorldTransform | AABB | none | O(n) | Calcule l'AABB a partir de WorldTransform et de la taille de l'entite. Marque BoundsCache.dirty = true si l'AABB a change |
| `update_bounds_cache` | `mge.foundation.bounds.v1.fn.update_bounds_cache` | 107 | AABB, BoundsCache | BoundsCache | none | O(d) d=dirty | Copie l'AABB courant dans le cache et remet dirty = false. Ne traite que les entites avec dirty = true |

---

## 5. Flux de donnees

```
WorldTransform
       │
       ▼
 ┌──────────────┐
 │ compute_aabb │  Phase 106
 │ (transform → │
 │  AABB)       │
 └──────┬───────┘
        │
        ▼
   AABB (min/max)
   BoundsCache.dirty = true
        │
        ▼
 ┌──────────────────────┐
 │ update_bounds_cache   │  Phase 107
 │ (AABB → cache, reset │
 │  dirty)               │
 └──────────┬────────────┘
            │
            ▼
   BoundsCache.dirty = false
   BoundsCache.cached_aabb = AABB
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `AABB.min_x <= AABB.max_x` et `AABB.min_y <= AABB.max_y` toujours.
- `BoundsCache.dirty` est remis a `false` apres Phase 107.
- L'AABB est recalculee chaque tick pour les entites dont la transform a change.
- L'AABB est une approximation englobante : elle peut etre plus large que la forme exacte (rotation d'un rectangle).
- Le cache est un mecanisme d'optimisation pour les consommateurs qui n'ont pas besoin de l'AABB chaque tick.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Calcule les AABB a partir de la transform | Ne detecte pas les collisions (→ collision-detection) |
| Maintient un cache de bornes avec flag dirty | Ne gere pas les formes de collider (→ collider) |
| Fournit les bornes pour le broad phase | Ne gere pas la hierarchie (→ hierarchy) |
| Optimise via dirty flag | Ne gere pas les transforms (→ transform) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | WorldTransform, AABB, BoundsCache |
| Ecrit | AABB, BoundsCache |
| Emet | rien |
| Ne touche jamais | Transform2D, Velocity2D, Collider, SpatialIndex, CollisionPair |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-bounds/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.bounds.v1, trait Plugin impl
    ├── components.rs     # AABB, BoundsCache
    ├── systems.rs        # compute_aabb, update_bounds_cache
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
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 2 systemes dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : AABB calcul, dirty flag, cache update, rotation englobante
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.bounds.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.bounds.v1.component.aabb","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.bounds.v1.component.bounds_cache","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.bounds.v1.fn.compute_aabb","k":"s","d":"foundation","r":["WorldTransform"],"w":["AABB"],"e":[],"p":106,"c":"O(n)"},
  {"i":"mge.foundation.bounds.v1.fn.update_bounds_cache","k":"s","d":"foundation","r":["AABB","BoundsCache"],"w":["BoundsCache"],"e":[],"p":107,"c":"O(d)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Transform2D { x: 50.0, y: 30.0, rotation: 0.0, scale_x: 2.0, scale_y: 2.0 });
world.insert(entity, WorldTransform { matrix: [2.0, 0.0, 0.0, 2.0, 50.0, 30.0] });
world.insert(entity, AABB { min_x: 0.0, min_y: 0.0, max_x: 0.0, max_y: 0.0 });
world.insert(entity, BoundsCache { dirty: true, cached_aabb: AABB { min_x: 0.0, min_y: 0.0, max_x: 0.0, max_y: 0.0 } });
// Apres Phase 106 : AABB recalculee selon WorldTransform
// Apres Phase 107 : BoundsCache.cached_aabb = AABB, dirty = false
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-transform](mge-gfp-transform.md) | Plugin transform (fournit WorldTransform) |
| [mge-gfp-spatial2d](mge-gfp-spatial2d.md) | Grille spatiale (utilise AABB pour les requetes) |
