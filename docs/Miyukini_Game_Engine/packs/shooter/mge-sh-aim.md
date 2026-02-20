# mge-sh-aim

> @id mge.sh.aim.v1  
> @role plugin  
> @domain shooter  
> @do manage_aim_direction_spread_recoil  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sh-aim` |
| @id MSCM | `mge.sh.aim.v1` |
| Domaine | shooter |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-input`, `mge-plugin-spatial` |
| Hot path | Oui (mise a jour direction chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n) par tick, n = entites avec AimDirection |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `AimMode` | `Mouse, Stick, Auto` | Source de la direction de visee. Auto = vers cible TargetLock |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `AimDirection` | `mge.sh.aim.v1.component.aim_direction` | `angle_rad: f32, normalized: Vec2` | Direction courante de visee. angle_rad en radians, normalized = vecteur unite |
| `AimConfig` | `mge.sh.aim.v1.component.aim_config` | `aim_mode: AimMode, sensitivity: f32, snap_angle: f32` | Configuration visee. snap_angle pour discretiser la direction (0 = libre) |
| `Spread` | `mge.sh.aim.v1.component.spread` | `current: f32, base: f32, max: f32, decay_rate: f32` | Deviation angulaire du tir. Augmente avec le recul, decroit naturellement |
| `Recoil` | `mge.sh.aim.v1.component.recoil` | `kick_angle: f32, recovery_rate: f32, accumulated: f32` | Recul par tir. accumulated = recul total non encore recupere |

---

## 4. Formules

```
Spread apres tir :
  spread.current = min(spread.current + recoil.kick_angle, spread.max)

Decay naturel du spread (par tick) :
  spread.current = max(spread.base, spread.current - spread.decay_rate * dt)

Recovery du recul (par tick) :
  recoil.accumulated = max(0.0, recoil.accumulated - recoil.recovery_rate * dt)

Direction effective (au moment du tir) :
  offset = rng.range(-spread.current, spread.current)
  effective_angle = aim.angle_rad + offset
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_aim_direction` | `mge.sh.aim.v1.fn.update_aim_direction` | 1710 | AimConfig, InputAction, Position2D, TargetLock (opt) | AimDirection | AimUpdated | O(n) | Calcule la direction selon AimMode : Mouse (curseur), Stick (joystick), Auto (vers TargetLock) |
| `apply_recoil` | `mge.sh.aim.v1.fn.apply_recoil` | 1711 | WeaponFired (event), Recoil, Spread | Spread, Recoil | none | O(f) | Pour chaque WeaponFired, augmente spread.current et recoil.accumulated |
| `decay_spread` | `mge.sh.aim.v1.fn.decay_spread` | 1712 | Spread, Recoil | Spread, Recoil | none | O(n) | Reduit spread vers base et recupere le recul accumule chaque tick |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `AimUpdated` | `mge.sh.aim.v1.event.aim_updated` | `entity: EntityId, angle_rad: f32, normalized: Vec2` | `update_aim_direction` | weapon (direction tir), ui (crosshair) |

---

## 7. Invariants

- `AimDirection.normalized` est toujours un vecteur unite (longueur 1.0 ± epsilon).
- `Spread.current` est borne entre `Spread.base` et `Spread.max` inclus.
- `Recoil.accumulated` ne descend jamais en dessous de 0.
- En mode `Auto`, si aucun TargetLock, la direction garde sa valeur precedente.
- `snap_angle > 0` discretise la direction en increments de snap_angle radians.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_sensitivity` | `f32` | 1.0 | [0.1, 5.0] | Sensibilite de base visee |
| `default_spread_base` | `f32` | 0.02 | [0.0, 0.5] | Spread minimal au repos (radians) |
| `default_spread_max` | `f32` | 0.3 | [0.05, 1.0] | Spread maximal (radians) |
| `default_decay_rate` | `f32` | 0.1 | [0.01, 1.0] | Vitesse retour spread vers base (rad/tick) |
| `default_recoil_recovery` | `f32` | 0.08 | [0.01, 0.5] | Vitesse recuperation recul (rad/tick) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Met a jour la direction de visee | Ne gere pas le tir (→ weapon) |
| Gere le spread et le recul | Ne gere pas les projectiles (→ weapon) |
| Supporte Mouse/Stick/Auto | Ne gere pas le ciblage automatique (→ target) |
| Emet AimUpdated | Ne gere pas les degats (→ health) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | AimConfig, InputAction, Position2D, TargetLock (optionnel), WeaponFired (event) |
| Ecrit | AimDirection, Spread, Recoil |
| Emet | AimUpdated |
| Ne touche jamais | Weapon, WeaponSlots, Magazine, ShooterHealth, Shield |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sh-aim/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sh.aim.v1, trait Plugin impl
    ├── components.rs     # AimDirection, AimConfig, Spread, Recoil
    ├── systems.rs        # update_aim_direction, apply_recoil, decay_spread
    └── events.rs         # AimUpdated
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 3 systemes dans `systems.rs` avec annotations completes
- [ ] 1 evenement dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (AimMode)
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : aim modes, spread decay, recoil accumulation, snap angle
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sh.aim.v1","k":"p","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.aim.v1.component.aim_direction","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.aim.v1.component.aim_config","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.aim.v1.component.spread","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.aim.v1.component.recoil","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.aim.v1.fn.update_aim_direction","k":"s","d":"shooter","r":["AimConfig","InputAction","Position2D"],"w":["AimDirection"],"e":["AimUpdated"],"p":1710,"c":"O(n)"},
  {"i":"mge.sh.aim.v1.fn.apply_recoil","k":"s","d":"shooter","r":["Recoil","Spread"],"w":["Spread","Recoil"],"e":[],"p":1711,"c":"O(f)"},
  {"i":"mge.sh.aim.v1.fn.decay_spread","k":"s","d":"shooter","r":["Spread","Recoil"],"w":["Spread","Recoil"],"e":[],"p":1712,"c":"O(n)"},
  {"i":"mge.sh.aim.v1.event.aim_updated","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, AimDirection { angle_rad: 0.0, normalized: Vec2::new(1.0, 0.0) });
world.insert(player, AimConfig {
    aim_mode: AimMode::Mouse,
    sensitivity: 1.0,
    snap_angle: 0.0,
});
world.insert(player, Spread {
    current: 0.02,
    base: 0.02,
    max: 0.3,
    decay_rate: 0.1,
});
world.insert(player, Recoil {
    kick_angle: 0.05,
    recovery_rate: 0.08,
    accumulated: 0.0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Shooter - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sh-weapon](mge-sh-weapon.md) | Plugin armes (consommateur principal) |
| [mge-sh-target](mge-sh-target.md) | Plugin ciblage (fournit TargetLock pour mode Auto) |
