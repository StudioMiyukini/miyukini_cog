# mge-race-ai-driver

> @id mge.race.ai_driver.v1  
> @role plugin  
> @domain racing  
> @do manage_ai_driver_racing_line_overtake_adaptation  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-race-ai-driver` |
| @id MSCM | `mge.race.ai_driver.v1` |
| Domaine | racing |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-race-vehicle`, `mge-race-track` |
| Hot path | Oui (decisions IA chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * k) par tick, n = pilotes IA, k = vehicules proches |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `AIAggressiveness` | `Cautious, Normal, Aggressive, Reckless` | Profil d'agressivite. Influe sur la distance de depassement et le freinage |
| `OvertakeState` | `None, Planning, Executing, Aborting` | Phase du depassement en cours |
| `AISkillLevel` | `Beginner, Intermediate, Advanced, Expert` | Niveau de competence. Influe sur la precision de la trajectoire |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `AIDriver` | `mge.race.ai_driver.v1.component.ai_driver` | `skill_level: AISkillLevel, aggressiveness: AIAggressiveness, target_throttle: f32, target_steer: f32, is_braking: bool` | Pilote IA. target_throttle/steer sont les inputs generes |
| `RacingLine` | `mge.race.ai_driver.v1.component.racing_line` | `waypoints: Vec<Vec2>, current_waypoint: u32, lookahead_distance: f32` | Trajectoire ideale pre-calculee. Le pilote vise le waypoint courant |
| `AIPersonality` | `mge.race.ai_driver.v1.component.ai_personality` | `risk_tolerance: f32, overtake_distance: f32, brake_distance_factor: f32, line_precision: f32` | Parametres de personnalite derives du skill + aggressiveness |
| `AIAwareness` | `mge.race.ai_driver.v1.component.ai_awareness` | `nearby_vehicles: Vec<EntityId>, closest_distance: f32, overtake_state: OvertakeState, overtake_target: Option<EntityId>` | Perception des vehicules proches. Mise a jour chaque tick |

---

## 4. Formules

```
Throttle cible :
  curve_angle = angle_between(current_waypoint, next_waypoint)
  speed_factor = 1.0 - (curve_angle / max_curve_angle) * brake_distance_factor
  target_throttle = clamp(speed_factor, 0.2, 1.0)
  if is_braking:
    target_throttle = -braking_intensity

Steering cible :
  target_point = waypoints[current_waypoint]
  angle_to_target = atan2(target_point.y - pos.y, target_point.x - pos.x) - rotation
  target_steer = clamp(angle_to_target * line_precision, -1.0, 1.0)

Depassement :
  can_overtake = closest_distance < overtake_distance AND lateral_space > vehicle_width * 1.5
  if can_overtake AND risk_tolerance > 0.5:
    overtake_state = Planning

Skill noise :
  precision_noise = match skill_level {
    Beginner: rng.range(-0.15, 0.15),
    Intermediate: rng.range(-0.08, 0.08),
    Advanced: rng.range(-0.03, 0.03),
    Expert: 0.0,
  }
  target_steer += precision_noise
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `follow_racing_line` | `mge.race.ai_driver.v1.fn.follow_racing_line` | 1960 | AIDriver, RacingLine, Position2D, Rotation2D, AIPersonality | AIDriver (target_steer, target_throttle) | none | O(n) | Calcule throttle et steer pour suivre la racing line. Applique noise selon skill |
| `evaluate_overtake` | `mge.race.ai_driver.v1.fn.evaluate_overtake` | 1961 | AIAwareness, AIPersonality, RacePosition, Velocity2D | AIAwareness (overtake_state, overtake_target) | OvertakeAttempted, OvertakeCompleted | O(n*k) | Evalue la faisabilite d'un depassement. Transition de phase overtake |
| `adapt_speed` | `mge.race.ai_driver.v1.fn.adapt_speed` | 1962 | AIDriver, AIAwareness, RacingLine, Wheels | AIDriver (target_throttle, is_braking) | AIBraking | O(n) | Adapte la vitesse en fonction des virages a venir et des vehicules proches |
| `avoid_collision` | `mge.race.ai_driver.v1.fn.avoid_collision` | 1963 | AIAwareness, AIDriver, Position2D, Velocity2D | AIDriver (target_steer) | none | O(n*k) | Corrige la trajectoire pour eviter les collisions imminentes |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `OvertakeAttempted` | `mge.race.ai_driver.v1.event.overtake_attempted` | `entity: EntityId, target: EntityId, side: f32` | `evaluate_overtake` | ui (indicator), analytics |
| `OvertakeCompleted` | `mge.race.ai_driver.v1.event.overtake_completed` | `entity: EntityId, target: EntityId, duration_ticks: u32` | `evaluate_overtake` | ui (notification), analytics |
| `AIBraking` | `mge.race.ai_driver.v1.event.ai_braking` | `entity: EntityId, reason: u8, intensity: f32` | `adapt_speed` | debug ui, analytics |

---

## 7. Invariants

- Un pilote IA ne peut avoir qu'un seul `overtake_target` a la fois.
- `target_throttle` est borne entre -1.0 et 1.0.
- `target_steer` est borne entre -1.0 et 1.0.
- `RacingLine.waypoints` contient au minimum 4 points (boucle fermee).
- Le `current_waypoint` avance cycliquement dans la liste de waypoints.
- Un depassement en phase `Aborting` ne peut pas repasser directement a `Executing`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_skill_level` | `AISkillLevel` | Intermediate | {Beginner, Intermediate, Advanced, Expert} | Niveau de competence par defaut |
| `default_aggressiveness` | `AIAggressiveness` | Normal | {Cautious, Normal, Aggressive, Reckless} | Agressivite par defaut |
| `overtake_distance` | `f32` | 15.0 | [5.0, 50.0] | Distance minimale pour considerer un depassement |
| `awareness_radius` | `f32` | 30.0 | [10.0, 100.0] | Rayon de perception des vehicules proches |
| `lookahead_waypoints` | `u32` | 5 | [2, 15] | Nombre de waypoints a anticiper pour le freinage |
| `max_curve_angle` | `f32` | 1.57 | [0.5, 3.14] | Angle max de virage (radians). Au-dela, freinage total |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Genere les inputs IA (throttle, steer) | N'applique pas la physique (→ vehicle) |
| Suit la racing line pre-calculee | Ne genere pas la racing line (donnee externe) |
| Decide et execute les depassements | Ne gere pas le circuit (→ track) |
| Adapte la vitesse aux virages | Ne gere pas les tours/positions (→ lap) |
| Evite les collisions imminentes | Ne gere pas les collisions physiques (→ physics) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | AIDriver, RacingLine, AIPersonality, AIAwareness, Position2D, Rotation2D, Velocity2D, Wheels, RacePosition |
| Ecrit | AIDriver (target_throttle, target_steer, is_braking), AIAwareness (overtake_state, overtake_target, nearby_vehicles) |
| Emet | OvertakeAttempted, OvertakeCompleted, AIBraking |
| Ne touche jamais | Vehicle, VehicleEngine, Track, Checkpoint, LapState, RaceResult |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-race-ai-driver/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.race.ai_driver.v1, trait Plugin impl
    ├── components.rs     # AIDriver, RacingLine, AIPersonality, AIAwareness
    ├── systems.rs        # follow_racing_line, evaluate_overtake, adapt_speed, avoid_collision
    └── events.rs         # OvertakeAttempted, OvertakeCompleted, AIBraking
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
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (AIAggressiveness, OvertakeState, AISkillLevel)
- [ ] Formules IA documentees
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : racing line follow, overtake decision, speed adaptation, collision avoidance
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.race.ai_driver.v1","k":"p","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.ai_driver.v1.component.ai_driver","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.ai_driver.v1.component.racing_line","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.ai_driver.v1.component.ai_personality","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.ai_driver.v1.component.ai_awareness","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.ai_driver.v1.fn.follow_racing_line","k":"s","d":"racing","r":["AIDriver","RacingLine","Position2D","Rotation2D","AIPersonality"],"w":["AIDriver"],"e":[],"p":1960,"c":"O(n)"},
  {"i":"mge.race.ai_driver.v1.fn.evaluate_overtake","k":"s","d":"racing","r":["AIAwareness","AIPersonality","RacePosition","Velocity2D"],"w":["AIAwareness"],"e":["OvertakeAttempted","OvertakeCompleted"],"p":1961,"c":"O(n*k)"},
  {"i":"mge.race.ai_driver.v1.fn.adapt_speed","k":"s","d":"racing","r":["AIDriver","AIAwareness","RacingLine","Wheels"],"w":["AIDriver"],"e":["AIBraking"],"p":1962,"c":"O(n)"},
  {"i":"mge.race.ai_driver.v1.fn.avoid_collision","k":"s","d":"racing","r":["AIAwareness","AIDriver","Position2D","Velocity2D"],"w":["AIDriver"],"e":[],"p":1963,"c":"O(n*k)"},
  {"i":"mge.race.ai_driver.v1.event.overtake_attempted","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.ai_driver.v1.event.overtake_completed","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.ai_driver.v1.event.ai_braking","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let ai_car = world.spawn();
world.insert(ai_car, AIDriver {
    skill_level: AISkillLevel::Advanced,
    aggressiveness: AIAggressiveness::Aggressive,
    target_throttle: 0.0,
    target_steer: 0.0,
    is_braking: false,
});
world.insert(ai_car, RacingLine {
    waypoints: track_waypoints.clone(),
    current_waypoint: 0,
    lookahead_distance: 20.0,
});
world.insert(ai_car, AIPersonality {
    risk_tolerance: 0.7,
    overtake_distance: 12.0,
    brake_distance_factor: 0.8,
    line_precision: 4.0,
});
world.insert(ai_car, AIAwareness {
    nearby_vehicles: Vec::new(),
    closest_distance: f32::MAX,
    overtake_state: OvertakeState::None,
    overtake_target: None,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Racing - Index](_index.md) | Vue d'ensemble du pack |
| [mge-race-vehicle](mge-race-vehicle.md) | Plugin vehicule (dependance) |
| [mge-race-track](mge-race-track.md) | Plugin circuit (dependance) |
