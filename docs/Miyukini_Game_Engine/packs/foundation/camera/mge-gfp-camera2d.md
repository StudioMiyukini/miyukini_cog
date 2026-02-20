# mge-gfp-camera2d

> @id mge.foundation.camera2d.v1  
> @role plugin  
> @domain foundation  
> @do viewport_zoom_view_matrix_2d  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-camera2d` |
| @id MSCM | `mge.foundation.camera2d.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-transform` |
| Hot path | Non (1-2 cameras max) |
| Headless safe | Oui |
| Complexite globale | O(c), c = cameras actives (generalement 1) |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Camera2D` | `mge.foundation.camera2d.v1.component.camera2d` | `zoom: f32, rotation: f32, active: bool` | Configuration de la camera 2D : niveau de zoom, rotation en radians et flag d'activation |
| `Viewport` | `mge.foundation.camera2d.v1.component.viewport` | `width: f32, height: f32, world_x: f32, world_y: f32` | Rectangle visible du monde : dimensions en pixels et position monde du coin superieur gauche |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `compute_view_matrix` | `mge.foundation.camera2d.v1.fn.compute_view_matrix` | 140 | Camera2D, Transform2D | Camera2D | none | O(c) | Calcule la matrice de vue a partir de la position, du zoom et de la rotation de la camera |
| `update_viewport` | `mge.foundation.camera2d.v1.fn.update_viewport` | 141 | Camera2D, Transform2D | Viewport | none | O(c) | Calcule le rectangle monde visible en fonction de la position camera et du zoom |

---

## 5. Flux de donnees

```
Transform2D (position camera)
       │
       ├──── Camera2D (zoom, rotation, active)
       │
       ▼
 ┌──────────────────────┐
 │ compute_view_matrix   │  Phase 140
 │ (position+zoom+rot → │
 │  matrice de vue)      │
 └──────────┬────────────┘
            │
            ▼
      Camera2D (matrice mise a jour)
            │
            ▼
 ┌──────────────────────┐
 │ update_viewport       │  Phase 141
 │ (camera → rect monde) │
 └──────────┬────────────┘
            │
            ▼
      Viewport (world_x, world_y, width, height)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- Seules les cameras avec `active: true` sont traitees par les systemes.
- `Viewport` est toujours synchronise avec la position camera apres la Phase 141.
- `Camera2D.zoom` ne doit jamais etre 0.0 ou negatif (division par zero).
- La matrice de vue est recalculee chaque tick, jamais accumulee.
- Sans follow-camera ni constraint-camera, la camera reste statique a sa position Transform2D.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Stocke la configuration camera (zoom, rotation) | Ne gere pas le suivi de cible (→ follow-camera) |
| Calcule la matrice de vue 2D | Ne gere pas les limites camera (→ constraint-camera) |
| Calcule le rectangle monde visible (Viewport) | Ne gere pas le tremblement (→ screen-shake) |
| Fournit la primitive camera de base pour tout le pipeline | Ne gere pas le rendu |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Transform2D, Camera2D |
| Ecrit | Camera2D, Viewport |
| Emet | rien |
| Ne touche jamais | Velocity2D, Collider, SpatialIndex, FollowTarget |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-camera2d/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.camera2d.v1, trait Plugin impl
    ├── components.rs     # Camera2D, Viewport
    ├── systems.rs        # compute_view_matrix, update_viewport
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
- [ ] Parametres GCL : aucun requis
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : compute view matrix, update viewport, zoom != 0, rotation, camera inactive ignoree
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.camera2d.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.camera2d.v1.component.camera2d","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.camera2d.v1.component.viewport","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.camera2d.v1.fn.compute_view_matrix","k":"s","d":"foundation","r":["Camera2D","Transform2D"],"w":["Camera2D"],"e":[],"p":140,"c":"O(c)"},
  {"i":"mge.foundation.camera2d.v1.fn.update_viewport","k":"s","d":"foundation","r":["Camera2D","Transform2D"],"w":["Viewport"],"e":[],"p":141,"c":"O(c)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let camera = world.spawn();
world.insert(camera, Transform2D {
    x: 0.0, y: 0.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(camera, Camera2D {
    zoom: 1.0, rotation: 0.0, active: true,
});
world.insert(camera, Viewport {
    width: 1280.0, height: 720.0, world_x: 0.0, world_y: 0.0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-follow-camera](mge-gfp-follow-camera.md) | Suivi de cible (depend de camera2d) |
| [mge-gfp-constraint-camera](mge-gfp-constraint-camera.md) | Limites camera (depend de camera2d) |
| [mge-gfp-screen-shake](mge-gfp-screen-shake.md) | Tremblement camera (depend de camera2d) |
