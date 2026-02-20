# mge-pl-collision

> @id mge.platformer.collision.v1  
> @role plugin  
> @domain platformer  
> @do resolve_platform_collisions_one_way_slopes_moving  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-pl-collision` |
| @id MSCM | `mge.platformer.collision.v1` |
| Domaine | platformer |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-pl-movement`, `mge-plugin-physics` |
| Hot path | Oui (resolve_platform_collisions chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * p) n = entites, p = plateformes proches |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `PlatformType` | `Solid, OneWay, Moving, Crumbling` | Type de plateforme |
| `SlopeType` | `None, Gentle, Steep` | Inclinaison de la pente |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Platform` | `mge.platformer.collision.v1.component.platform` | `platform_type: PlatformType, slope: SlopeType, slope_angle: f32` | Plateforme. slope_angle en degres (0 = plat) |
| `MovingPlatform` | `mge.platformer.collision.v1.component.moving_platform` | `waypoints: Vec<(f32, f32)>, speed: f32, current_waypoint: u32, ping_pong: bool` | Plateforme mobile. ping_pong = aller-retour |
| `CrumblingPlatform` | `mge.platformer.collision.v1.component.crumbling_platform` | `delay_ticks: u32, respawn_ticks: u32, timer: u32, crumbled: bool` | Plateforme qui s'effondre apres contact. Respawn apres delai |
| `PlatformCollider` | `mge.platformer.collision.v1.component.platform_collider` | `width: f32, height: f32` | AABB de la plateforme |

---

## 4. Formules

```
OneWay collision:
  Resoudre uniquement si velocity.y <= 0 (descente) ET
  entity.bottom >= platform.top - tolerance

Slope correction:
  velocity.y = velocity.x * tan(slope_angle) (snap au sol sur pente)
  Gentle: angle < 30deg → pas de glissement
  Steep: angle >= 30deg → glissement si pas d'input

Moving platform carry:
  entity.position += platform.velocity * dt (entraine le joueur)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `resolve_platform_collisions` | `mge.platformer.collision.v1.fn.resolve_platform_collisions` | 1610 | Position, Velocity, Platform, PlatformCollider, GroundDetection | Position, Velocity, GroundDetection | PlatformLanded | O(n*p) | Resout les collisions AABB. OneWay = top uniquement. Met a jour GroundDetection |
| `update_moving_platforms` | `mge.platformer.collision.v1.fn.update_moving_platforms` | 1611 | MovingPlatform, Position | Position, MovingPlatform | none | O(p) | Deplace les plateformes mobiles entre waypoints. Entraine les entites dessus |
| `update_crumbling_platforms` | `mge.platformer.collision.v1.fn.update_crumbling_platforms` | 1612 | CrumblingPlatform | CrumblingPlatform, PlatformCollider | PlatformCrumbled, PlatformRespawned | O(p) | Timer apres contact. Crumble → desactive collision. Respawn apres delai |
| `apply_slope_physics` | `mge.platformer.collision.v1.fn.apply_slope_physics` | 1613 | Platform, GroundDetection, Velocity | Velocity | none | O(n) | Ajuste velocity pour les pentes. Snap au sol. Glissement sur Steep |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `PlatformLanded` | `mge.platformer.collision.v1.event.platform_landed` | `entity: EntityId, platform: EntityId, platform_type: PlatformType` | `resolve_platform_collisions` | jump (reset jumps), audio, crumbling (start timer) |
| `PlatformCrumbled` | `mge.platformer.collision.v1.event.platform_crumbled` | `platform: EntityId` | `update_crumbling_platforms` | ui (animation), audio |
| `PlatformRespawned` | `mge.platformer.collision.v1.event.platform_respawned` | `platform: EntityId` | `update_crumbling_platforms` | ui (animation) |

---

## 7. Invariants

- OneWay platforms ne bloquent jamais par en dessous ou sur les cotes.
- Une CrumblingPlatform ne respawn que si `crumbled == true` et timer >= respawn_ticks.
- `MovingPlatform.current_waypoint` est dans [0, waypoints.len()).
- `GroundDetection.ground_entity` est mis a jour a chaque tick par `resolve_platform_collisions`.
- Les entites transportees par une plateforme mobile sont deplacees meme sans input.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `collision_tolerance` | `f32` | 2.0 | [0.5, 5.0] | Tolerance pour detection OneWay (pixels) |
| `steep_slope_threshold` | `f32` | 30.0 | [15.0, 60.0] | Angle a partir duquel la pente est Steep (degres) |
| `crumble_default_delay` | `u32` | 30 | [5, 120] | Delai avant effondrement (ticks) |
| `crumble_default_respawn` | `u32` | 180 | [60, 600] | Delai avant respawn (ticks) |
| `moving_platform_speed` | `f32` | 3.0 | [0.5, 20.0] | Vitesse par defaut des plateformes mobiles |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Resout les collisions plateforme (solid, one-way) | Ne gere pas le mouvement du joueur (→ movement) |
| Deplace les plateformes mobiles | Ne gere pas le saut (→ jump) |
| Gere les plateformes crumbling | Ne gere pas les dangers (→ hazard) |
| Corrige la physique des pentes | Ne fait pas le rendu (→ core render) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Position, Velocity, Platform, PlatformCollider, MovingPlatform, CrumblingPlatform, GroundDetection |
| Ecrit | Position, Velocity, GroundDetection, MovingPlatform, CrumblingPlatform, PlatformCollider |
| Emet | PlatformLanded, PlatformCrumbled, PlatformRespawned |
| Ne touche jamais | JumpAbility, CameraTarget, Checkpoint, HazardZone, PlatformerMovement |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-pl-collision/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.platformer.collision.v1
    ├── components.rs     # Platform, MovingPlatform, CrumblingPlatform, PlatformCollider
    ├── systems.rs        # resolve_platform_collisions, update_moving_platforms, update_crumbling_platforms, apply_slope_physics
    └── events.rs         # PlatformLanded, PlatformCrumbled, PlatformRespawned
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (resolve_platform_collisions) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (PlatformType, SlopeType)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : solid collision, one-way, moving platform carry, crumble/respawn, slopes
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.platformer.collision.v1","k":"p","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.collision.v1.component.platform","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.collision.v1.component.moving_platform","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.collision.v1.component.crumbling_platform","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.collision.v1.component.platform_collider","k":"d","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.collision.v1.fn.resolve_platform_collisions","k":"s","d":"platformer","r":["Position","Velocity","Platform","PlatformCollider","GroundDetection"],"w":["Position","Velocity","GroundDetection"],"e":["PlatformLanded"],"p":1610,"c":"O(n*p)"},
  {"i":"mge.platformer.collision.v1.fn.update_moving_platforms","k":"s","d":"platformer","r":["MovingPlatform","Position"],"w":["Position","MovingPlatform"],"e":[],"p":1611,"c":"O(p)"},
  {"i":"mge.platformer.collision.v1.fn.update_crumbling_platforms","k":"s","d":"platformer","r":["CrumblingPlatform"],"w":["CrumblingPlatform","PlatformCollider"],"e":["PlatformCrumbled","PlatformRespawned"],"p":1612,"c":"O(p)"},
  {"i":"mge.platformer.collision.v1.fn.apply_slope_physics","k":"s","d":"platformer","r":["Platform","GroundDetection","Velocity"],"w":["Velocity"],"e":[],"p":1613,"c":"O(n)"},
  {"i":"mge.platformer.collision.v1.event.platform_landed","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.collision.v1.event.platform_crumbled","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.platformer.collision.v1.event.platform_respawned","k":"e","d":"platformer","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let platform = world.spawn();
world.insert(platform, Platform { platform_type: PlatformType::Solid, slope: SlopeType::None, slope_angle: 0.0 });
world.insert(platform, PlatformCollider { width: 64.0, height: 16.0 });

let moving = world.spawn();
world.insert(moving, Platform { platform_type: PlatformType::Moving, slope: SlopeType::None, slope_angle: 0.0 });
world.insert(moving, MovingPlatform { waypoints: vec![(100.0, 200.0), (300.0, 200.0)], speed: 3.0, current_waypoint: 0, ping_pong: true });
world.insert(moving, PlatformCollider { width: 48.0, height: 16.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Platformer - Index](_index.md) | Vue d'ensemble du pack |
| [mge-pl-movement](mge-pl-movement.md) | Plugin movement (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
