# mge-sh-target

> @id mge.sh.target.v1  
> @role plugin  
> @domain shooter  
> @do manage_auto_targeting_lock_on  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sh-target` |
| @id MSCM | `mge.sh.target.v1` |
| Domaine | shooter |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-sh-aim` |
| Hot path | Oui (scan chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * e) par tick, n = entites avec AutoAimConfig, e = cibles potentielles |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `TargetPriority` | `Nearest, LowestHealth, HighestThreat, Manual` | Mode de selection de cible. Manual = joueur selectionne |
| `LockState` | `None, Acquiring, Locked, Breaking` | Etat du verrou. Acquiring = en cours d'acquisition |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `TargetLock` | `mge.sh.target.v1.component.target_lock` | `target: Option<EntityId>, lock_state: LockState, lock_timer: u32` | Verrou sur cible. lock_timer = ticks depuis acquisition |
| `AutoAimConfig` | `mge.sh.target.v1.component.auto_aim_config` | `scan_radius: f32, lock_angle: f32, priority: TargetPriority, acquire_ticks: u32, break_ticks: u32` | Configuration ciblage. lock_angle = cone de detection (radians) |
| `ThreatLevel` | `mge.sh.target.v1.component.threat_level` | `value: f32` | Niveau de menace de l'entite. Utilise par priorite HighestThreat |

---

## 4. Formules

```
Distance cible :
  dist = distance(source.position, candidate.position)
  in_range = dist <= config.scan_radius

Angle cible :
  angle_to_target = atan2(candidate.y - source.y, candidate.x - source.x)
  angle_diff = abs(normalize_angle(angle_to_target - aim.angle_rad))
  in_cone = angle_diff <= config.lock_angle / 2.0

Score selection (Nearest) :
  score = -dist    (plus proche = meilleur)

Score selection (LowestHealth) :
  score = -candidate.health.current

Score selection (HighestThreat) :
  score = candidate.threat_level.value
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `scan_targets` | `mge.sh.target.v1.fn.scan_targets` | 1730 | AutoAimConfig, Position2D, AimDirection | (interne : candidats) | none | O(n*e) | Collecte les entites dans scan_radius et lock_angle. Filtre les allies |
| `select_best_target` | `mge.sh.target.v1.fn.select_best_target` | 1731 | AutoAimConfig, TargetLock, (candidats internes) | TargetLock | TargetAcquired | O(c) | Parmi les candidats, selectionne le meilleur selon TargetPriority. Demarre acquisition |
| `maintain_lock` | `mge.sh.target.v1.fn.maintain_lock` | 1732 | TargetLock, AutoAimConfig, Position2D | TargetLock | none | O(n) | Verifie que la cible verrouillee est toujours valide (vivante, en range, en cone) |
| `break_lock` | `mge.sh.target.v1.fn.break_lock` | 1733 | TargetLock | TargetLock | TargetLost, LockBroken | O(n) | Si cible invalide pendant break_ticks, casse le verrou. Emet TargetLost |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `TargetAcquired` | `mge.sh.target.v1.event.target_acquired` | `source: EntityId, target: EntityId, priority: TargetPriority` | `select_best_target` | aim (mode Auto), ui (indicateur cible) |
| `TargetLost` | `mge.sh.target.v1.event.target_lost` | `source: EntityId, previous_target: EntityId, reason: LockState` | `break_lock` | aim (retour mode libre), ui |
| `LockBroken` | `mge.sh.target.v1.event.lock_broken` | `source: EntityId, target: EntityId` | `break_lock` | ui (feedback visuel) |

---

## 7. Invariants

- Un TargetLock ne peut pointer que vers une entite vivante et existante.
- Si la cible meurt, le lock passe en Breaking au prochain tick.
- En mode Manual, scan_targets ne change pas la cible — seul l'input joueur le fait.
- `lock_timer` est incremente chaque tick ou le lock est Locked. Reset a 0 sur nouveau lock.
- Un seul TargetLock par entite (pas de multi-lock en v1).
- Les entites sans composant marqueur "cibleable" sont ignorees par scan_targets.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_scan_radius` | `f32` | 300.0 | [50.0, 2000.0] | Rayon de detection cibles |
| `default_lock_angle` | `f32` | 0.5 | [0.1, 3.14] | Cone de detection (radians, demi-angle) |
| `default_acquire_ticks` | `u32` | 10 | [1, 60] | Ticks pour acquerir le verrou |
| `default_break_ticks` | `u32` | 15 | [1, 60] | Ticks hors cone avant perte du verrou |
| `lock_through_walls` | `bool` | false | {true, false} | Permet le lock a travers obstacles |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Scanne les cibles dans un rayon/cone | Ne gere pas la visee (→ aim) |
| Selectionne la meilleure cible | Ne tire pas sur la cible (→ weapon) |
| Maintient et casse le verrou | Ne gere pas les degats (→ health) |
| Supporte plusieurs modes de priorite | Ne calcule pas le pathfinding vers la cible |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | AutoAimConfig, Position2D, AimDirection, TargetLock, ThreatLevel, ShooterHealth (opt) |
| Ecrit | TargetLock |
| Emet | TargetAcquired, TargetLost, LockBroken |
| Ne touche jamais | Weapon, Magazine, Spread, Recoil, Shield, DamageBuffer |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sh-target/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sh.target.v1, trait Plugin impl
    ├── components.rs     # TargetLock, AutoAimConfig, ThreatLevel
    ├── systems.rs        # scan_targets, select_best_target, maintain_lock, break_lock
    └── events.rs         # TargetAcquired, TargetLost, LockBroken
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (scan_targets) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (TargetPriority, LockState)
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : scan radius, cone filter, priority selection, lock break
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sh.target.v1","k":"p","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.target.v1.component.target_lock","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.target.v1.component.auto_aim_config","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.target.v1.component.threat_level","k":"d","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.target.v1.fn.scan_targets","k":"s","d":"shooter","r":["AutoAimConfig","Position2D","AimDirection"],"w":[],"e":[],"p":1730,"c":"O(n*e)"},
  {"i":"mge.sh.target.v1.fn.select_best_target","k":"s","d":"shooter","r":["AutoAimConfig","TargetLock"],"w":["TargetLock"],"e":["TargetAcquired"],"p":1731,"c":"O(c)"},
  {"i":"mge.sh.target.v1.fn.maintain_lock","k":"s","d":"shooter","r":["TargetLock","AutoAimConfig","Position2D"],"w":["TargetLock"],"e":[],"p":1732,"c":"O(n)"},
  {"i":"mge.sh.target.v1.fn.break_lock","k":"s","d":"shooter","r":["TargetLock"],"w":["TargetLock"],"e":["TargetLost","LockBroken"],"p":1733,"c":"O(n)"},
  {"i":"mge.sh.target.v1.event.target_acquired","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.target.v1.event.target_lost","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sh.target.v1.event.lock_broken","k":"e","d":"shooter","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let turret = world.spawn();
world.insert(turret, TargetLock {
    target: None,
    lock_state: LockState::None,
    lock_timer: 0,
});
world.insert(turret, AutoAimConfig {
    scan_radius: 300.0,
    lock_angle: 0.5,
    priority: TargetPriority::Nearest,
    acquire_ticks: 10,
    break_ticks: 15,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Shooter - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sh-aim](mge-sh-aim.md) | Plugin visee (fournit AimDirection) |
