# mge-gfp-spatial2d

> @id mge.foundation.spatial2d.v1  
> @role plugin  
> @domain foundation  
> @do spatial_index_grid_zone_queries  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-spatial2d` |
| @id MSCM | `mge.foundation.spatial2d.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-transform` |
| Hot path | Oui (dependance broad phase collision) |
| Headless safe | Oui |
| Complexite globale | O(n) pour update, O(n) pour rebuild |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `SpatialIndex` | `mge.foundation.spatial2d.v1.component.spatial_index` | `grid_x: i32, grid_y: i32, bucket_id: u32` | Coordonnees de grille et identifiant du bucket auquel l'entite appartient |
| `GridCell` | `mge.foundation.spatial2d.v1.component.grid_cell` | `entities: Vec<EntityId>` | Liste des entites presentes dans cette cellule de grille |
| `SpatialConfig` | `mge.foundation.spatial2d.v1.component.spatial_config` | `cell_size: f32` | Configuration statique de la grille. cell_size = taille d'une cellule en unites monde |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_spatial_index` | `mge.foundation.spatial2d.v1.fn.update_spatial_index` | 102 | WorldTransform, SpatialConfig | SpatialIndex | none | O(n) | Calcule grid_x/grid_y a partir de WorldTransform et cell_size, assigne le bucket |
| `rebuild_grid` | `mge.foundation.spatial2d.v1.fn.rebuild_grid` | 103 | SpatialIndex | GridCell | none | O(n) | Reconstruit la structure de grille. Vide toutes les cellules puis reassigne les entites |

---

## 5. Flux de donnees

```
WorldTransform ──► SpatialConfig.cell_size
       │                    │
       ▼                    ▼
 ┌────────────────────────────┐
 │   update_spatial_index     │  Phase 102
 │ (position → grid coords)  │
 └─────────────┬──────────────┘
               │
               ▼
        SpatialIndex (grid_x, grid_y, bucket_id)
               │
               ▼
 ┌────────────────────────────┐
 │       rebuild_grid         │  Phase 103
 │ (reindex cells)            │
 └─────────────┬──────────────┘
               │
               ▼
        GridCell (entities par cellule)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- Chaque entite avec `WorldTransform` et `SpatialIndex` est affectee a exactement une cellule de grille.
- `GridCell.entities` est reconstruit entierement chaque tick (pas d'accumulation).
- `SpatialConfig.cell_size` doit etre > 0.0. Une valeur trop petite degrade les performances (trop de cellules).
- Les coordonnees de grille sont calculees par `floor(position / cell_size)`.
- Apres Phase 103, la grille reflete exactement les positions du tick courant.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `cell_size` | `f32` | 64.0 | [8.0, 1024.0] | Taille d'une cellule de la grille spatiale en unites monde |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Indexe les entites dans une grille 2D | Ne detecte pas les collisions (→ collision-detection) |
| Fournit des requetes spatiales par zone | Ne gere pas les transforms (→ transform) |
| Permet le broad phase en O(n) au lieu de O(n²) | Ne gere pas les layer masks (→ layer-mask) |
| Reconstruit la grille a chaque tick | Ne fait pas de raycast (→ raycast) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | WorldTransform, SpatialConfig, SpatialIndex |
| Ecrit | SpatialIndex, GridCell |
| Emet | rien |
| Ne touche jamais | Transform2D, Velocity2D, Collider, CollisionPair |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-spatial2d/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.spatial2d.v1, trait Plugin impl
    ├── components.rs     # SpatialIndex, GridCell, SpatialConfig
    ├── systems.rs        # update_spatial_index, rebuild_grid
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
| No allocation hot path | Obligatoire (GridCell pre-alloue) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields (dont 1 statique)
- [ ] 2 systemes dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] Parametre GCL `cell_size` expose
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : assignation grille, rebuild, changement position, cell_size variable
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.spatial2d.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.spatial2d.v1.component.spatial_index","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.spatial2d.v1.component.grid_cell","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.spatial2d.v1.component.spatial_config","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.spatial2d.v1.fn.update_spatial_index","k":"s","d":"foundation","r":["WorldTransform","SpatialConfig"],"w":["SpatialIndex"],"e":[],"p":102,"c":"O(n)"},
  {"i":"mge.foundation.spatial2d.v1.fn.rebuild_grid","k":"s","d":"foundation","r":["SpatialIndex"],"w":["GridCell"],"e":[],"p":103,"c":"O(n)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let config = world.spawn();
world.insert(config, SpatialConfig { cell_size: 64.0 });

let entity = world.spawn();
world.insert(entity, Transform2D { x: 130.0, y: 70.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0 });
world.insert(entity, WorldTransform { matrix: [1.0, 0.0, 0.0, 1.0, 130.0, 70.0] });
world.insert(entity, SpatialIndex { grid_x: 0, grid_y: 0, bucket_id: 0 });
// Apres Phase 102 : SpatialIndex { grid_x: 2, grid_y: 1, bucket_id: ... }
// Apres Phase 103 : GridCell(2,1).entities contient entity
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-transform](mge-gfp-transform.md) | Plugin transform (fournit WorldTransform) |
| [mge-gfp-bounds](mge-gfp-bounds.md) | Plugin AABB (utilise la grille pour les requetes) |
