# mge-race-lap

> @id mge.race.lap.v1  
> @role plugin  
> @domain racing  
> @do manage_laps_positions_timing_race_results  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-race-lap` |
| @id MSCM | `mge.race.lap.v1` |
| Domaine | racing |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial` |
| Hot path | Oui (positions recalculees chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n log n) par tick, n = vehicules (tri positions) |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `RaceState` | `Countdown, Racing, Finished, Aborted` | Etat global de la course |
| `FinishReason` | `Completed, DNF, Disqualified, TimedOut` | Raison de fin pour un participant |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `LapState` | `mge.race.lap.v1.component.lap_state` | `current_lap: u32, total_laps: u32, last_checkpoint: u32, lap_start_tick: u32` | Etat de progression du vehicule dans la course |
| `RacePosition` | `mge.race.lap.v1.component.race_position` | `position: u32, total_participants: u32, distance_to_leader: f32, distance_to_next: f32` | Position dans le classement. position = 1 pour le leader |
| `RaceTimer` | `mge.race.lap.v1.component.race_timer` | `race_state: RaceState, elapsed_ticks: u32, countdown_ticks: u32, best_lap_ticks: u32, current_lap_ticks: u32` | Chrono global et par tour. best_lap_ticks = meilleur tour personnel |
| `RaceResult` | `mge.race.lap.v1.component.race_result` | `finish_position: u32, finish_reason: FinishReason, total_time_ticks: u32, best_lap_ticks: u32, laps_completed: u32` | Resultat final du participant. Ecrit a la fin de la course |

---

## 4. Formules

```
Progression normalise :
  progress = (current_lap - 1) * total_checkpoints + last_checkpoint
  progress_ratio = progress / (total_laps * total_checkpoints)

Distance au leader :
  leader_progress = max(all_participants.progress)
  distance_to_leader = leader_progress - self.progress

Position (tri) :
  sort participants by (current_lap DESC, last_checkpoint DESC, distance_to_next_checkpoint ASC)

Temps du tour :
  current_lap_ticks = elapsed_ticks - lap_start_tick
  if lap_completed:
    best_lap_ticks = min(best_lap_ticks, current_lap_ticks)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_lap` | `mge.race.lap.v1.fn.update_lap` | 1940 | CheckpointReached (event), LapState, RaceTimer | LapState, RaceTimer | LapCompleted | O(e) | A chaque CheckpointReached type Finish, incremente current_lap. Met a jour lap_start_tick |
| `compute_positions` | `mge.race.lap.v1.fn.compute_positions` | 1941 | LapState, Position2D, Checkpoint | RacePosition | PositionChanged | O(n log n) | Trie les participants par progression et distance. Emet si changement de position |
| `check_race_finish` | `mge.race.lap.v1.fn.check_race_finish` | 1942 | LapState, RaceTimer, Track | RaceResult, RaceTimer | RaceFinished | O(n) | Verifie si un participant a complete tous les tours. Ecrit RaceResult |
| `record_best_lap` | `mge.race.lap.v1.fn.record_best_lap` | 1943 | LapCompleted (event), RaceTimer | RaceTimer | BestLapSet | O(e) | Compare le tour complete avec le meilleur temps. Emet BestLapSet si amelioration |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `LapCompleted` | `mge.race.lap.v1.event.lap_completed` | `entity: EntityId, lap_number: u32, lap_time_ticks: u32` | `update_lap` | record_best_lap, ui (tour display) |
| `RaceFinished` | `mge.race.lap.v1.event.race_finished` | `entity: EntityId, finish_position: u32, total_time_ticks: u32, finish_reason: FinishReason` | `check_race_finish` | ui (resultats), game logic |
| `BestLapSet` | `mge.race.lap.v1.event.best_lap_set` | `entity: EntityId, lap_time_ticks: u32, lap_number: u32` | `record_best_lap` | ui (best lap indicator) |
| `PositionChanged` | `mge.race.lap.v1.event.position_changed` | `entity: EntityId, old_position: u32, new_position: u32` | `compute_positions` | ui (position overlay), audio |

---

## 7. Invariants

- `current_lap` commence a 1 et ne peut qu'augmenter.
- `position` est unique parmi les participants actifs (pas de doublons).
- `RaceResult` n'est ecrit qu'une seule fois par participant — il est immutable apres ecriture.
- Le `best_lap_ticks` ne peut que diminuer (ou rester identique).
- Un vehicule disqualifie recoit `FinishReason::Disqualified` et sa position est la derniere.
- `elapsed_ticks` est incremente chaque tick uniquement si `race_state == Racing`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_lap_count` | `u32` | 3 | [1, 100] | Nombre de tours par defaut |
| `countdown_duration_ticks` | `u32` | 180 | [60, 600] | Duree du decompte avant depart (3 sec a 60 FPS) |
| `dnf_timeout_ticks` | `u32` | 18000 | [3600, 108000] | Temps max avant DNF automatique (5 min a 60 FPS) |
| `position_update_interval` | `u32` | 1 | [1, 10] | Recalcul positions tous les N ticks |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les tours (increment, completion) | Ne gere pas les checkpoints (→ track) |
| Calcule le classement en temps reel | Ne gere pas la physique vehicule (→ vehicle) |
| Chronometre la course et les tours | Ne gere pas l'IA pilote (→ ai-driver) |
| Genere les resultats finaux | Ne gere pas la detection de surface (→ track) |
| Detecte les meilleurs tours | Ne gere pas le rendu du classement |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | LapState, RaceTimer, RacePosition, Position2D, Checkpoint, Track, CheckpointReached, LapCompleted |
| Ecrit | LapState, RacePosition, RaceTimer, RaceResult |
| Emet | LapCompleted, RaceFinished, BestLapSet, PositionChanged |
| Ne touche jamais | Vehicle, VehicleEngine, Steering, TrackSegment, TrackSurface, AIDriver |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-race-lap/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.race.lap.v1, trait Plugin impl
    ├── components.rs     # LapState, RacePosition, RaceTimer, RaceResult
    ├── systems.rs        # update_lap, compute_positions, check_race_finish, record_best_lap
    └── events.rs         # LapCompleted, RaceFinished, BestLapSet, PositionChanged
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No allocation hot path | Obligatoire (positions pre-allouees) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (RaceState, FinishReason)
- [ ] Formules de progression et tri documentees
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : lap increment, position sort, race finish, best lap
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.race.lap.v1","k":"p","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.lap.v1.component.lap_state","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.lap.v1.component.race_position","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.lap.v1.component.race_timer","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.lap.v1.component.race_result","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.lap.v1.fn.update_lap","k":"s","d":"racing","r":["LapState","RaceTimer"],"w":["LapState","RaceTimer"],"e":["LapCompleted"],"p":1940,"c":"O(e)"},
  {"i":"mge.race.lap.v1.fn.compute_positions","k":"s","d":"racing","r":["LapState","Position2D","Checkpoint"],"w":["RacePosition"],"e":["PositionChanged"],"p":1941,"c":"O(n log n)"},
  {"i":"mge.race.lap.v1.fn.check_race_finish","k":"s","d":"racing","r":["LapState","RaceTimer","Track"],"w":["RaceResult","RaceTimer"],"e":["RaceFinished"],"p":1942,"c":"O(n)"},
  {"i":"mge.race.lap.v1.fn.record_best_lap","k":"s","d":"racing","r":["RaceTimer"],"w":["RaceTimer"],"e":["BestLapSet"],"p":1943,"c":"O(e)"},
  {"i":"mge.race.lap.v1.event.lap_completed","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.lap.v1.event.race_finished","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.lap.v1.event.best_lap_set","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.lap.v1.event.position_changed","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let racer = world.spawn();
world.insert(racer, LapState {
    current_lap: 1,
    total_laps: 3,
    last_checkpoint: 0,
    lap_start_tick: 0,
});
world.insert(racer, RaceTimer {
    race_state: RaceState::Countdown,
    elapsed_ticks: 0,
    countdown_ticks: 180,
    best_lap_ticks: u32::MAX,
    current_lap_ticks: 0,
});
world.insert(racer, RacePosition {
    position: 1,
    total_participants: 8,
    distance_to_leader: 0.0,
    distance_to_next: 0.0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Racing - Index](_index.md) | Vue d'ensemble du pack |
| [mge-race-track](mge-race-track.md) | Plugin circuit (fournit CheckpointReached) |
| [mge-race-vehicle](mge-race-vehicle.md) | Plugin vehicule |
