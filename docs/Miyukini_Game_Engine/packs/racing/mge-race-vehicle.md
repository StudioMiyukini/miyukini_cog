# mge-race-vehicle

> @id mge.race.vehicle.v1  
> @role plugin  
> @domain racing  
> @do manage_vehicle_physics_acceleration_drift_nitro  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-race-vehicle` |
| @id MSCM | `mge.race.vehicle.v1` |
| Domaine | racing |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-plugin-basic-physics`, `mge-plugin-input` |
| Hot path | Oui (physique vehicule chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n) par tick, n = vehicules actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `DriveType` | `FWD, RWD, AWD` | Transmission. Influe sur la traction et le comportement en drift |
| `SurfaceType` | `Asphalt, Dirt, Ice, Grass, Sand` | Type de surface sous les roues. Modifie friction et traction |
| `DriftPhase` | `None, Initiating, Drifting, Recovering` | Phase du drift. Bonus possible en sortie (recovering) |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Vehicle` | `mge.race.vehicle.v1.component.vehicle` | `mass: f32, drive_type: DriveType, max_speed: f32, reverse_max_speed: f32` | Proprietes de base du vehicule |
| `VehicleEngine` | `mge.race.vehicle.v1.component.vehicle_engine` | `acceleration: f32, braking_force: f32, current_speed: f32, throttle: f32` | Moteur. throttle = [-1.0, 1.0], negatif = marche arriere |
| `Steering` | `mge.race.vehicle.v1.component.steering` | `angle: f32, max_angle: f32, responsiveness: f32, input_steer: f32` | Direction. input_steer = [-1.0, 1.0]. angle lisse vers input_steer |
| `Wheels` | `mge.race.vehicle.v1.component.wheels` | `grip: f32, surface: SurfaceType, slip_ratio: f32` | Etat des roues. grip modifie par surface. slip_ratio pour drift |
| `Nitro` | `mge.race.vehicle.v1.component.nitro` | `fuel: f32, max_fuel: f32, boost_multiplier: f32, consumption_rate: f32, is_active: bool` | Systeme nitro. Boost temporaire de vitesse max et acceleration |
| `DriftState` | `mge.race.vehicle.v1.component.drift_state` | `phase: DriftPhase, drift_angle: f32, drift_timer: u32, accumulated_score: f32` | Etat du drift. Score accumule pour bonus eventuel |

---

## 4. Formules

```
Acceleration :
  effective_accel = engine.acceleration * engine.throttle * wheels.grip
  if nitro.is_active:
    effective_accel *= nitro.boost_multiplier

Friction par surface :
  friction_table = { Asphalt: 1.0, Dirt: 0.7, Ice: 0.3, Grass: 0.6, Sand: 0.5 }
  effective_friction = base_friction * friction_table[surface]

Vitesse :
  engine.current_speed += (effective_accel - effective_friction * engine.current_speed) * dt
  engine.current_speed = clamp(engine.current_speed, -vehicle.reverse_max_speed, effective_max_speed)
  effective_max_speed = vehicle.max_speed * (1.0 + if nitro.is_active { 0.3 } else { 0.0 })

Traction :
  slip_ratio = lateral_velocity / max(1.0, forward_velocity)
  if slip_ratio > drift_threshold:
    drift_state.phase = Drifting

Direction effective :
  steering.angle = lerp(steering.angle, steering.input_steer * steering.max_angle, steering.responsiveness * dt)
  turn_radius = wheelbase / tan(steering.angle)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `apply_throttle` | `mge.race.vehicle.v1.fn.apply_throttle` | 1900 | VehicleEngine, Vehicle, InputAction, Wheels | VehicleEngine | none | O(n) | Applique l'acceleration selon throttle, grip et surface. Met a jour current_speed |
| `apply_steering` | `mge.race.vehicle.v1.fn.apply_steering` | 1901 | Steering, InputAction, VehicleEngine | Steering, Rotation2D | none | O(n) | Lisse l'angle de direction et applique la rotation au vehicule |
| `compute_traction` | `mge.race.vehicle.v1.fn.compute_traction` | 1902 | Wheels, VehicleEngine, Vehicle, Velocity2D | Wheels, DriftState | DriftStarted, DriftEnded | O(n) | Calcule le slip_ratio. Detecte debut/fin de drift |
| `apply_drift` | `mge.race.vehicle.v1.fn.apply_drift` | 1903 | DriftState, Steering, VehicleEngine | DriftState, Velocity2D | none | O(n) | Applique l'angle de drift et accumule le score drift |
| `consume_nitro` | `mge.race.vehicle.v1.fn.consume_nitro` | 1904 | Nitro, InputAction | Nitro | NitroActivated, NitroExhausted | O(n) | Consomme le fuel nitro si actif. Emet NitroExhausted quand vide |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `DriftStarted` | `mge.race.vehicle.v1.event.drift_started` | `entity: EntityId, initial_angle: f32` | `compute_traction` | ui (drift indicator), score |
| `DriftEnded` | `mge.race.vehicle.v1.event.drift_ended` | `entity: EntityId, duration_ticks: u32, accumulated_score: f32` | `compute_traction` | score, ui (bonus display) |
| `NitroActivated` | `mge.race.vehicle.v1.event.nitro_activated` | `entity: EntityId, fuel_remaining: f32` | `consume_nitro` | ui (nitro gauge), audio |
| `NitroExhausted` | `mge.race.vehicle.v1.event.nitro_exhausted` | `entity: EntityId` | `consume_nitro` | ui (gauge empty), audio |

---

## 7. Invariants

- `VehicleEngine.current_speed` est borne entre `-reverse_max_speed` et `max_speed` (+ nitro bonus).
- `Steering.angle` est borne entre `-max_angle` et `max_angle`.
- `Nitro.fuel` ne descend jamais en dessous de 0. A 0, `is_active` passe a false.
- Un drift ne peut commencer que si `slip_ratio > drift_threshold`.
- Le vehicule a l'arret (`current_speed < epsilon`) ne peut pas drifter.
- `Wheels.grip` est toujours > 0 (minimum 0.1 sur glace).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_max_speed` | `f32` | 200.0 | [50.0, 1000.0] | Vitesse max par defaut (units/sec) |
| `default_acceleration` | `f32` | 80.0 | [10.0, 500.0] | Acceleration par defaut (units/sec^2) |
| `braking_force` | `f32` | 150.0 | [50.0, 500.0] | Force de freinage |
| `drift_threshold` | `f32` | 0.4 | [0.1, 0.9] | Ratio slip pour declencher le drift |
| `nitro_boost_multiplier` | `f32` | 1.3 | [1.1, 2.0] | Multiplicateur vitesse avec nitro |
| `nitro_duration_ticks` | `u32` | 180 | [30, 600] | Duree du nitro (plein → vide) |
| `steering_responsiveness` | `f32` | 5.0 | [1.0, 20.0] | Vitesse de reponse du volant |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere la physique arcade du vehicule | Ne gere pas le circuit (→ track) |
| Gere acceleration, freinage, marche arriere | Ne gere pas les tours/positions (→ lap) |
| Gere le drift (detection, angle, score) | Ne gere pas l'IA pilote (→ ai-driver) |
| Gere le nitro | Ne gere pas les collisions vehicule-vehicule (→ physics) |
| Applique les effets de surface sur la traction | Ne detecte pas la surface (→ track) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Vehicle, VehicleEngine, Steering, Wheels, Nitro, DriftState, InputAction, Velocity2D |
| Ecrit | VehicleEngine, Steering, Wheels, Nitro, DriftState, Velocity2D, Rotation2D |
| Emet | DriftStarted, DriftEnded, NitroActivated, NitroExhausted |
| Ne touche jamais | Track, Checkpoint, LapState, RacePosition, AIDriver |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-race-vehicle/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.race.vehicle.v1, trait Plugin impl
    ├── components.rs     # Vehicle, VehicleEngine, Steering, Wheels, Nitro, DriftState
    ├── systems.rs        # apply_throttle, apply_steering, compute_traction, apply_drift, consume_nitro
    └── events.rs         # DriftStarted, DriftEnded, NitroActivated, NitroExhausted
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
- [ ] 6 composants dans `components.rs` avec @id et @fields
- [ ] 5 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (DriveType, SurfaceType, DriftPhase)
- [ ] Formules physique arcade documentees
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : acceleration, braking, drift detection, nitro consumption, surface friction
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.race.vehicle.v1","k":"p","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.component.vehicle","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.component.vehicle_engine","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.component.steering","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.component.wheels","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.component.nitro","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.component.drift_state","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.fn.apply_throttle","k":"s","d":"racing","r":["VehicleEngine","Vehicle","InputAction","Wheels"],"w":["VehicleEngine"],"e":[],"p":1900,"c":"O(n)"},
  {"i":"mge.race.vehicle.v1.fn.apply_steering","k":"s","d":"racing","r":["Steering","InputAction","VehicleEngine"],"w":["Steering","Rotation2D"],"e":[],"p":1901,"c":"O(n)"},
  {"i":"mge.race.vehicle.v1.fn.compute_traction","k":"s","d":"racing","r":["Wheels","VehicleEngine","Vehicle","Velocity2D"],"w":["Wheels","DriftState"],"e":["DriftStarted","DriftEnded"],"p":1902,"c":"O(n)"},
  {"i":"mge.race.vehicle.v1.fn.apply_drift","k":"s","d":"racing","r":["DriftState","Steering","VehicleEngine"],"w":["DriftState","Velocity2D"],"e":[],"p":1903,"c":"O(n)"},
  {"i":"mge.race.vehicle.v1.fn.consume_nitro","k":"s","d":"racing","r":["Nitro","InputAction"],"w":["Nitro"],"e":["NitroActivated","NitroExhausted"],"p":1904,"c":"O(n)"},
  {"i":"mge.race.vehicle.v1.event.drift_started","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.event.drift_ended","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.event.nitro_activated","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.vehicle.v1.event.nitro_exhausted","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let car = world.spawn();
world.insert(car, Vehicle { mass: 1200.0, drive_type: DriveType::RWD, max_speed: 200.0, reverse_max_speed: 40.0 });
world.insert(car, VehicleEngine { acceleration: 80.0, braking_force: 150.0, current_speed: 0.0, throttle: 0.0 });
world.insert(car, Steering { angle: 0.0, max_angle: 0.6, responsiveness: 5.0, input_steer: 0.0 });
world.insert(car, Wheels { grip: 1.0, surface: SurfaceType::Asphalt, slip_ratio: 0.0 });
world.insert(car, Nitro { fuel: 100.0, max_fuel: 100.0, boost_multiplier: 1.3, consumption_rate: 0.55, is_active: false });
world.insert(car, DriftState { phase: DriftPhase::None, drift_angle: 0.0, drift_timer: 0, accumulated_score: 0.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Racing - Index](_index.md) | Vue d'ensemble du pack |
| [mge-race-track](mge-race-track.md) | Plugin circuit (fournit SurfaceType) |
