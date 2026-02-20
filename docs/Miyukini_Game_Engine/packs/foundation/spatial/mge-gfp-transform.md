# mge-gfp-transform

> @id mge.foundation.transform.v1  
> @role plugin  
> @domain foundation  
> @do manage_position_rotation_scale_local_world  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-transform` |
| @id MSCM | `mge.foundation.transform.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (chaque entite avec position) |
| Headless safe | Oui |
| Complexite globale | O(n), n = entites avec Transform2D |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Transform2D` | `mge.foundation.transform.v1.component.transform2d` | `x: f32, y: f32, rotation: f32, scale_x: f32, scale_y: f32` | Position locale, rotation (radians) et echelle de l'entite |
| `WorldTransform` | `mge.foundation.transform.v1.component.world_transform` | `matrix: [f32; 6]` | Matrice monde calculee (2x3 affine). Resultat de la propagation local→world |
| `PreviousTransform` | `mge.foundation.transform.v1.component.previous_transform` | `x: f32, y: f32, rotation: f32` | Position du tick precedent. Utilisee pour l'interpolation de rendu |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `sync_local_to_world` | `mge.foundation.transform.v1.fn.sync_local_to_world` | 100 | Transform2D | WorldTransform | none | O(n) | Propage Transform2D local vers la matrice WorldTransform pour chaque entite |
| `store_previous_transform` | `mge.foundation.transform.v1.fn.store_previous_transform` | 101 | Transform2D | PreviousTransform | none | O(n) | Cache la position courante dans PreviousTransform avant le tick suivant |

---

## 5. Flux de donnees

```
Transform2D (local)
       │
       ▼
 ┌─────────────────────┐
 │ sync_local_to_world  │  Phase 100
 │ (local → matrice)    │
 └──────────┬───────────┘
            │
            ▼
     WorldTransform (monde)
            │
            ▼
 ┌──────────────────────────┐
 │ store_previous_transform │  Phase 101
 │ (cache position courante)│
 └──────────┬───────────────┘
            │
            ▼
    PreviousTransform (t-1)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `WorldTransform` est toujours synchronise avec `Transform2D` apres la Phase 100.
- `PreviousTransform` contient les valeurs du tick precedent apres la Phase 101.
- `Transform2D.scale_x` et `scale_y` ne doivent jamais etre exactement 0.0 (matrice non inversible).
- La matrice `WorldTransform` est recalculee chaque tick, jamais accumulee.
- Sans le plugin hierarchy, `WorldTransform` = transformation locale pure.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Stocke position, rotation et echelle locale | Ne gere pas la hierarchie parent/child (→ hierarchy) |
| Calcule la matrice monde a partir du local | Ne gere pas le deplacement (→ velocity) |
| Cache la position precedente pour interpolation | Ne gere pas le rendu |
| Fournit la primitive de base pour tout positionnement | Ne gere pas les contraintes de position |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Transform2D |
| Ecrit | WorldTransform, PreviousTransform |
| Emet | rien |
| Ne touche jamais | Velocity2D, Collider, Camera2D, SpatialIndex |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-transform/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.transform.v1, trait Plugin impl
    ├── components.rs     # Transform2D, WorldTransform, PreviousTransform
    ├── systems.rs        # sync_local_to_world, store_previous_transform
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
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 2 systemes dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] Parametres GCL : aucun requis
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : sync local→world, store previous, echelle, rotation
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.transform.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.transform.v1.component.transform2d","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.transform.v1.component.world_transform","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.transform.v1.component.previous_transform","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.transform.v1.fn.sync_local_to_world","k":"s","d":"foundation","r":["Transform2D"],"w":["WorldTransform"],"e":[],"p":100,"c":"O(n)"},
  {"i":"mge.foundation.transform.v1.fn.store_previous_transform","k":"s","d":"foundation","r":["Transform2D"],"w":["PreviousTransform"],"e":[],"p":101,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let entity = world.spawn();
world.insert(entity, Transform2D {
    x: 100.0, y: 50.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(entity, WorldTransform { matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0] });
world.insert(entity, PreviousTransform { x: 100.0, y: 50.0, rotation: 0.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-spatial2d](mge-gfp-spatial2d.md) | Indexation spatiale (depend de transform) |
| [mge-gfp-hierarchy](mge-gfp-hierarchy.md) | Hierarchie parent/child (propage WorldTransform) |
