# mge-gfp-constraint-camera

> @id mge.foundation.constraint_camera.v1  
> @role plugin  
> @domain foundation  
> @do camera_bounds_deadzone_clamping  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-constraint-camera` |
| @id MSCM | `mge.foundation.constraint_camera.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-camera2d` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(c), c = cameras avec contraintes |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `CameraBounds` | `mge.foundation.constraint_camera.v1.component.camera_bounds` | `min_x: f32, min_y: f32, max_x: f32, max_y: f32` | Limites rectangulaires du monde au-dela desquelles la camera ne peut pas se deplacer |
| `DeadZone` | `mge.foundation.constraint_camera.v1.component.dead_zone` | `width: f32, height: f32` | Zone morte centrale : la camera ne bouge pas tant que la cible reste dans cette zone |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `clamp_camera_bounds` | `mge.foundation.constraint_camera.v1.fn.clamp_camera_bounds` | 143 | CameraBounds, DeadZone, Camera2D, Transform2D | Transform2D (camera) | none | O(c) | Clamp la position camera dans les limites definies par CameraBounds et applique la logique de dead zone |

---

## 5. Flux de donnees

```
Transform2D (camera, apres follow)
       │
       ├──── CameraBounds (min_x, min_y, max_x, max_y)
       ├──── DeadZone (width, height)
       ├──── Camera2D (zoom pour ajuster limites)
       │
       ▼
 ┌─────────────────────────┐
 │ clamp_camera_bounds      │  Phase 143
 │ (dead zone + clamp      │
 │  dans les limites)       │
 └──────────┬──────────────┘
            │
            ▼
      Transform2D (camera, position corrigee)
```

---

## 6. Evenements

Aucun.

---

## 7. Invariants

- `CameraBounds.min_x < CameraBounds.max_x` et `CameraBounds.min_y < CameraBounds.max_y` (limites valides).
- `DeadZone.width >= 0.0` et `DeadZone.height >= 0.0` (dimensions non negatives).
- Le clamping s'applique apres le suivi (Phase 142) et avant le screen shake (Phase 144).
- Si la dead zone est plus grande que les limites, le comportement de clamping prime.
- Les limites tiennent compte du zoom : le viewport visible ne deborde jamais au-dela des bounds.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Clamp la position camera dans des limites rectangulaires | Ne gere pas le suivi de cible (→ follow-camera) |
| Applique une dead zone pour eviter les micro-mouvements | Ne gere pas le viewport ni le zoom (→ camera2d) |
| Corrige la position finale apres le suivi | Ne gere pas le tremblement (→ screen-shake) |
| Garantit que le viewport reste dans les limites du monde | Ne gere pas le rendu |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | CameraBounds, DeadZone, Camera2D, Transform2D |
| Ecrit | Transform2D (camera) |
| Emet | rien |
| Ne touche jamais | Viewport, Velocity2D, Collider, FollowTarget, ShakeTrauma |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-constraint-camera/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.constraint_camera.v1, trait Plugin impl
    ├── components.rs     # CameraBounds, DeadZone
    ├── systems.rs        # clamp_camera_bounds
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
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] `events.rs` present (vide)
- [ ] Parametres GCL : aucun requis
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : clamp gauche/droite/haut/bas, dead zone active, dead zone nulle, zoom pris en compte
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.constraint_camera.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.constraint_camera.v1.component.camera_bounds","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.constraint_camera.v1.component.dead_zone","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.constraint_camera.v1.fn.clamp_camera_bounds","k":"s","d":"foundation","r":["CameraBounds","DeadZone","Camera2D","Transform2D"],"w":["Transform2D"],"e":[],"p":143,"c":"O(c)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let camera = world.spawn();
world.insert(camera, Transform2D {
    x: 0.0, y: 0.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(camera, Camera2D { zoom: 1.0, rotation: 0.0, active: true });
world.insert(camera, CameraBounds {
    min_x: 0.0, min_y: 0.0, max_x: 2000.0, max_y: 1000.0,
});
world.insert(camera, DeadZone { width: 64.0, height: 48.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-camera2d](mge-gfp-camera2d.md) | Camera de base (prerequis) |
| [mge-gfp-follow-camera](mge-gfp-follow-camera.md) | Suivi cible (s'execute avant constraint) |
