# mge-pl-camera

> @id mge.platformer.camera.v1  
> @role plugin  
> @domain platformer  
> @do manage_camera_follow_target_bounds_smoothing  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-pl-camera` |
| @id MSCM | `mge.platformer.camera.v1` |
| Domaine | platformer |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial` |
| Hot path | Oui (update_camera_position chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(1) par camera |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `FollowMode` | `Instant, Smooth, DeadZone, LookAhead` | Mode de suivi camera |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `CameraTarget` | `mge.platformer.camera.v1.component.camera_target` | `target: EntityId, follow_mode: FollowMode, offset_x: f32, offset_y: f32` | Cible et mode de suivi |
| `CameraBounds` | `mge.platformer.camera.v1.component.camera_bounds` | `min_x: f32, max_x: f32, min_y: f32, max_y: f32` | Limites du monde visible. La camera ne sort pas |
| `CameraSmoothing` | `mge.platformer.camera.v1.component.camera_smoothing` | `speed: f32, dead_zone_width: f32, dead_zone_height: f32, look_ahead_distance: f32` | Parametres lissage et dead zone |

---

## 4. Formules

```
Instant:
  camera.position = target.position + offset

Smooth:
  camera.position = lerp(camera.position, target.position + offset, speed * dt)

DeadZone:
  Si target dans dead zone → pas de mouvement
  Si target hors dead zone → smooth vers target

LookAhead:
  look_offset = facing_direction * look_ahead_distance
  camera.position = lerp(camera.position, target.position + offset + look_offset, speed * dt)

Bounds clamp:
  camera.x = clamp(camera.x, bounds.min_x, bounds.max_x)
  camera.y = clamp(camera.y, bounds.min_y, bounds.max_y)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_camera_position` | `mge.platformer.camera.v1.fn.update_camera_position` | 1620 | CameraTarget, CameraSmoothing, Position | Position (camera) | CameraTargetChanged | O(1) | Calcule la position camera selon le mode. Interpole si Smooth/DeadZone/LookAhead |
| `clamp_camera_bounds` | `mge.platformer.camera.v1.fn.clamp_camera_bounds` | 1621 | CameraBounds, Position (camera) | Position (camera) | CameraBoundsReached | O(1) | Clampe la position camera dans les bounds. Emet si atteint un bord |
| `apply_camera_shake` | `mge.platformer.camera.v1.fn.apply_camera_shake` | 1622 | CameraShake, Position (camera) | Position (camera), CameraShake | none | O(1) | Applique un offset aleatoire temporaire. Decroit sur la duree |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `CameraTargetChanged` | `mge.platformer.camera.v1.event.camera_target_changed` | `old_target: EntityId, new_target: EntityId` | `update_camera_position` | ui (transition) |
| `CameraBoundsReached` | `mge.platformer.camera.v1.event.camera_bounds_reached` | `edge: &str` | `clamp_camera_bounds` | ui (edge indicator) |

---

## 7. Invariants

- La position camera est toujours dans les CameraBounds apres `clamp_camera_bounds`.
- En mode `Instant`, la camera n'a aucun retard sur la cible.
- La dead zone ne s'applique que si `FollowMode == DeadZone`.
- `CameraSmoothing.speed` est > 0.0.
- Le camera shake ne modifie pas la position logique, seulement le rendu.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_follow_mode` | `FollowMode` | Smooth | {Instant, Smooth, DeadZone, LookAhead} | Mode par defaut |
| `default_smooth_speed` | `f32` | 5.0 | [1.0, 20.0] | Vitesse interpolation |
| `default_dead_zone_width` | `f32` | 40.0 | [0.0, 200.0] | Largeur dead zone (pixels) |
| `default_dead_zone_height` | `f32` | 20.0 | [0.0, 100.0] | Hauteur dead zone (pixels) |
| `default_look_ahead` | `f32` | 60.0 | [0.0, 200.0] | Distance look ahead (pixels) |
| `shake_max_intensity` | `f32` | 10.0 | [1.0, 50.0] | Intensite max du shake (pixels) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Suit la cible avec interpolation | Ne gere pas le mouvement du joueur (→ movement) |
| Respecte les limites du monde | Ne fait pas le rendu (→ core render) |
| Applique le camera shake | Ne gere pas les transitions entre zones |
| Supporte dead zone et look ahead | Ne gere pas le zoom |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | CameraTarget, CameraSmoothing, CameraBounds, Position (target) |
| Ecrit | Position (camera) |
| Emet | CameraTargetChanged, CameraBoundsReached |
| Ne touche jamais | PlatformerMovement, JumpAbility, Platform, Checkpoint, HazardZone |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-pl-camera/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.platformer.camera.v1
    ├── components.rs     # CameraTarget, CameraBounds, CameraSmoothing
    ├── systems.rs        # update_camera_position, clamp_camera_bounds, apply_camera_shake
    └── events.rs         # CameraTargetChanged, CameraBoundsReached
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (FollowMode)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : instant follow, smooth lerp, dead zone, look ahead, bounds clamp, shake
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.platformer.camera.v1","k":"p","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.camera.v1.component.camera_target","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.camera.v1.component.camera_bounds","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.camera.v1.component.camera_smoothing","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.camera.v1.fn.update_camera_position","k":"s","d":"platformer","r":["CameraTarget","CameraSmoothing","Position"],"w":["Position"],"e":["CameraTargetChanged"],"p":1620,"c":"O(1)"},
  {"i":"mge.platformer.camera.v1.fn.clamp_camera_bounds","k":"s","d":"platformer","r":["CameraBounds","Position"],"w":["Position"],"e":["CameraBoundsReached"],"p":1621,"c":"O(1)"},
  {"i":"mge.platformer.camera.v1.fn.apply_camera_shake","k":"s","d":"platformer","r":["Position"],"w":["Position"],"e":[],"p":1622,"c":"O(1)"},
  {"i":"mge.platformer.camera.v1.event.camera_target_changed","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.camera.v1.event.camera_bounds_reached","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let camera = world.spawn();
world.insert(camera, CameraTarget { target: player_entity, follow_mode: FollowMode::Smooth, offset_x: 0.0, offset_y: -20.0 });
world.insert(camera, CameraBounds { min_x: 0.0, max_x: 3200.0, min_y: 0.0, max_y: 600.0 });
world.insert(camera, CameraSmoothing { speed: 5.0, dead_zone_width: 40.0, dead_zone_height: 20.0, look_ahead_distance: 60.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Platformer - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
