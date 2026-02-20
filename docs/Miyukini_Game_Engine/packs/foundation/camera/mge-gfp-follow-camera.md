# mge-gfp-follow-camera

> @id mge.foundation.follow_camera.v1  
> @role plugin  
> @domain foundation  
> @do camera_follow_target_smoothing_offset  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-follow-camera` |
| @id MSCM | `mge.foundation.follow_camera.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-camera2d`, `mge-gfp-transform` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(c), c = cameras avec suivi |

---

## 2. Enumerations

Aucune.

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `FollowTarget` | `mge.foundation.follow_camera.v1.component.follow_target` | `entity: EntityId` | Reference vers l'entite cible que la camera doit suivre |
| `FollowSmoothing` | `mge.foundation.follow_camera.v1.component.follow_smoothing` | `speed: f32` | Vitesse d'interpolation lineaire (lerp) pour le lissage du suivi |
| `FollowOffset` | `mge.foundation.follow_camera.v1.component.follow_offset` | `offset_x: f32, offset_y: f32` | Decalage constant applique a la position cible |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_follow_target` | `mge.foundation.follow_camera.v1.fn.update_follow_target` | 142 | FollowTarget, FollowSmoothing, FollowOffset, Transform2D (cible) | Transform2D (camera) | FollowTargetChanged | O(c) | Interpole (lerp) la position de la camera vers la position de la cible + offset |

---

## 5. Flux de donnees

```
FollowTarget (entity cible)
       │
       ├──── Transform2D (position cible)
       ├──── FollowSmoothing (vitesse lerp)
       ├──── FollowOffset (decalage x, y)
       │
       ▼
 ┌─────────────────────────┐
 │ update_follow_target     │  Phase 142
 │ (lerp camera → cible    │
 │  + offset)               │
 └──────────┬──────────────┘
            │
            ├──→ Transform2D (camera, position mise a jour)
            │
            └──→ FollowTargetChanged (si cible change)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `FollowTargetChanged` | `mge.foundation.follow_camera.v1.event.follow_target_changed` | `camera: EntityId, old_target: Option<EntityId>, new_target: EntityId` | `update_follow_target` | Systemes UI, logique de scene, debug |

---

## 7. Invariants

- `FollowTarget.entity` doit referencer une entite existante possedant un `Transform2D`.
- La vitesse de lerp `FollowSmoothing.speed` doit etre > 0.0 (sinon pas de deplacement).
- L'event `FollowTargetChanged` n'est emis que lorsque le champ `entity` de `FollowTarget` change reellement.
- Le suivi est applique apres `compute_view_matrix` (Phase 140) et avant `clamp_camera_bounds` (Phase 143).
- Si la cible est detruite, le comportement est indefini — le consommateur doit gerer ce cas.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_follow_speed` | f32 | 5.0 | ]0.0, +inf[ | Vitesse de lissage par defaut pour les nouvelles cameras follow |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Interpole la position camera vers une entite cible | Ne gere pas le viewport ni le zoom (→ camera2d) |
| Applique un decalage constant (offset) | Ne gere pas les limites de camera (→ constraint-camera) |
| Emet un evenement lors du changement de cible | Ne gere pas le tremblement (→ screen-shake) |
| Fournit le lissage (smoothing) du suivi | Ne gere pas le rendu |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | FollowTarget, FollowSmoothing, FollowOffset, Transform2D (cible) |
| Ecrit | Transform2D (camera) |
| Emet | FollowTargetChanged |
| Ne touche jamais | Camera2D, Viewport, Velocity2D, Collider, SpatialIndex |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-follow-camera/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.follow_camera.v1, trait Plugin impl
    ├── components.rs     # FollowTarget, FollowSmoothing, FollowOffset
    ├── systems.rs        # update_follow_target
    └── events.rs         # FollowTargetChanged
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
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] Parametre GCL `default_follow_speed` documente
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : lerp basique, offset applique, cible changee → event, speed = 0 gere
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.follow_camera.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.follow_camera.v1.component.follow_target","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.follow_camera.v1.component.follow_smoothing","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.follow_camera.v1.component.follow_offset","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.follow_camera.v1.fn.update_follow_target","k":"s","d":"foundation","r":["FollowTarget","FollowSmoothing","FollowOffset","Transform2D"],"w":["Transform2D"],"e":["FollowTargetChanged"],"p":142,"c":"O(c)"},
  {"i":"mge.foundation.follow_camera.v1.event.follow_target_changed","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, Transform2D {
    x: 200.0, y: 100.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});

let camera = world.spawn();
world.insert(camera, Transform2D {
    x: 0.0, y: 0.0, rotation: 0.0, scale_x: 1.0, scale_y: 1.0,
});
world.insert(camera, Camera2D { zoom: 1.0, rotation: 0.0, active: true });
world.insert(camera, FollowTarget { entity: player });
world.insert(camera, FollowSmoothing { speed: 5.0 });
world.insert(camera, FollowOffset { offset_x: 0.0, offset_y: -50.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-camera2d](mge-gfp-camera2d.md) | Camera de base (prerequis) |
| [mge-gfp-constraint-camera](mge-gfp-constraint-camera.md) | Limites camera (s'applique apres follow) |
