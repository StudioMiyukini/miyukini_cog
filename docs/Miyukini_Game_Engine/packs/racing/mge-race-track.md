# mge-race-track

> @id mge.race.track.v1  
> @role plugin  
> @domain racing  
> @do manage_circuits_checkpoints_surfaces_offtrack  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-race-track` |
| @id MSCM | `mge.race.track.v1` |
| Domaine | racing |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-race-lap` |
| Hot path | Oui (detection surface chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * s) par tick, n = vehicules, s = segments proches |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `CheckpointType` | `Normal, Finish, Sector` | Type de checkpoint. Finish = ligne d'arrivee, Sector = split-time |
| `TrackSurfaceType` | `Asphalt, Dirt, Ice, Grass, Sand, Gravel` | Surface du segment. Modifie la friction du vehicule |
| `ShortcutSeverity` | `Minor, Major, Disqualify` | Gravite du raccourci detecte |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Track` | `mge.race.track.v1.component.track` | `name_hash: u64, total_checkpoints: u32, total_sectors: u32, lap_count: u32, track_length: f32` | Definition globale du circuit. track_length en unites world |
| `Checkpoint` | `mge.race.track.v1.component.checkpoint` | `index: u32, checkpoint_type: CheckpointType, position: Vec2, radius: f32, sector_index: u32` | Point de passage. radius = zone de detection |
| `TrackSegment` | `mge.race.track.v1.component.track_segment` | `start: Vec2, end: Vec2, width: f32, surface: TrackSurfaceType, segment_index: u32` | Portion de piste. Definit la surface et les limites |
| `TrackSurface` | `mge.race.track.v1.component.track_surface` | `current_surface: TrackSurfaceType, friction_modifier: f32, is_off_track: bool` | Surface sous le vehicule. Mise a jour chaque tick |
| `RaceStartGrid` | `mge.race.track.v1.component.race_start_grid` | `positions: Vec<Vec2>, orientations: Vec<f32>, grid_size: u32` | Grille de depart. positions[0] = pole position |

---

## 4. Formules

```
Detection surface :
  segment = nearest_segment(vehicle.position, track.segments)
  lateral_dist = perpendicular_distance(vehicle.position, segment.start, segment.end)
  is_off_track = lateral_dist > segment.width / 2.0

Friction modifier :
  friction_table = { Asphalt: 1.0, Dirt: 0.7, Ice: 0.3, Grass: 0.6, Sand: 0.5, Gravel: 0.65 }
  friction_modifier = friction_table[surface]
  if is_off_track:
    friction_modifier *= 0.5

Detection checkpoint :
  distance = (vehicle.position - checkpoint.position).length()
  reached = distance < checkpoint.radius

Detection raccourci :
  missed_checkpoints = expected_next - last_reached
  if missed_checkpoints > 1:
    severity = if missed_checkpoints > 3 { Disqualify } else { Major }
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `load_track` | `mge.race.track.v1.fn.load_track` | 1920 | LoadTrackRequest (event), Track | World (spawn checkpoints, segments) | none | O(c+s) | Charge le circuit : spawne checkpoints et segments. One-shot |
| `detect_surface` | `mge.race.track.v1.fn.detect_surface` | 1921 | Position2D, TrackSegment | TrackSurface | SurfaceChanged, OffTrack | O(n*s) | Detecte la surface sous chaque vehicule. Emet si changement |
| `process_checkpoint` | `mge.race.track.v1.fn.process_checkpoint` | 1922 | Position2D, Checkpoint, LapState | LapState | CheckpointReached | O(n*c) | Verifie si un vehicule atteint un checkpoint. Met a jour LapState |
| `detect_shortcut` | `mge.race.track.v1.fn.detect_shortcut` | 1923 | LapState, Checkpoint | LapState | ShortcutDetected | O(n) | Detecte les raccourcis (checkpoints sautes). Emet severite |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `CheckpointReached` | `mge.race.track.v1.event.checkpoint_reached` | `entity: EntityId, checkpoint_index: u32, checkpoint_type: CheckpointType, sector_time: u32` | `process_checkpoint` | lap (tour update), ui (split-time) |
| `SurfaceChanged` | `mge.race.track.v1.event.surface_changed` | `entity: EntityId, from: TrackSurfaceType, to: TrackSurfaceType` | `detect_surface` | vehicle (friction update), audio |
| `ShortcutDetected` | `mge.race.track.v1.event.shortcut_detected` | `entity: EntityId, missed_count: u32, severity: ShortcutSeverity` | `detect_shortcut` | lap (penalite), ui (avertissement) |
| `OffTrack` | `mge.race.track.v1.event.off_track` | `entity: EntityId, distance_from_edge: f32` | `detect_surface` | vehicle (ralentissement), ui |

---

## 7. Invariants

- Les checkpoints sont ordonnes par `index`. L'index 0 est toujours le premier apres la ligne de depart.
- Le checkpoint `Finish` a toujours l'index le plus eleve du circuit.
- Un vehicule ne peut valider un checkpoint que si le precedent a deja ete valide (pas de saut).
- `TrackSurface` est mis a jour chaque tick — il reflette toujours le segment le plus proche.
- `is_off_track` est vrai si la distance laterale depasse `segment.width / 2`.
- La grille de depart contient exactement `grid_size` positions.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `checkpoint_radius` | `f32` | 5.0 | [1.0, 20.0] | Rayon de detection des checkpoints |
| `off_track_friction_penalty` | `f32` | 0.5 | [0.1, 0.9] | Multiplicateur friction hors-piste |
| `shortcut_minor_threshold` | `u32` | 1 | [1, 3] | Checkpoints sautes pour Minor |
| `shortcut_disqualify_threshold` | `u32` | 3 | [2, 10] | Checkpoints sautes pour Disqualify |
| `segment_search_radius` | `f32` | 50.0 | [10.0, 200.0] | Rayon de recherche du segment le plus proche |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Definit la structure du circuit (checkpoints, segments) | Ne gere pas la physique vehicule (→ vehicle) |
| Detecte la surface sous chaque vehicule | Ne gere pas les tours/positions (→ lap) |
| Detecte le passage des checkpoints | Ne gere pas l'IA pilote (→ ai-driver) |
| Detecte les raccourcis et le hors-piste | Ne gere pas le rendu du circuit |
| Fournit la grille de depart | Ne gere pas le chrono ni le classement (→ lap) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Track, Checkpoint, TrackSegment, Position2D, LapState, LoadTrackRequest |
| Ecrit | TrackSurface, LapState (checkpoint validation), World (spawn) |
| Emet | CheckpointReached, SurfaceChanged, ShortcutDetected, OffTrack |
| Ne touche jamais | Vehicle, VehicleEngine, Steering, Nitro, AIDriver, RacePosition |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-race-track/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.race.track.v1, trait Plugin impl
    ├── components.rs     # Track, Checkpoint, TrackSegment, TrackSurface, RaceStartGrid
    ├── systems.rs        # load_track, detect_surface, process_checkpoint, detect_shortcut
    └── events.rs         # CheckpointReached, SurfaceChanged, ShortcutDetected, OffTrack
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
- [ ] 5 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (CheckpointType, TrackSurfaceType, ShortcutSeverity)
- [ ] Formules de detection surface documentees
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : checkpoint reach, surface detection, off-track, shortcut detection
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.race.track.v1","k":"p","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.component.track","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.component.checkpoint","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.component.track_segment","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.component.track_surface","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.component.race_start_grid","k":"d","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.fn.load_track","k":"s","d":"racing","r":["Track"],"w":["World"],"e":[],"p":1920,"c":"O(c+s)"},
  {"i":"mge.race.track.v1.fn.detect_surface","k":"s","d":"racing","r":["Position2D","TrackSegment"],"w":["TrackSurface"],"e":["SurfaceChanged","OffTrack"],"p":1921,"c":"O(n*s)"},
  {"i":"mge.race.track.v1.fn.process_checkpoint","k":"s","d":"racing","r":["Position2D","Checkpoint","LapState"],"w":["LapState"],"e":["CheckpointReached"],"p":1922,"c":"O(n*c)"},
  {"i":"mge.race.track.v1.fn.detect_shortcut","k":"s","d":"racing","r":["LapState","Checkpoint"],"w":["LapState"],"e":["ShortcutDetected"],"p":1923,"c":"O(n)"},
  {"i":"mge.race.track.v1.event.checkpoint_reached","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.event.surface_changed","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.event.shortcut_detected","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.race.track.v1.event.off_track","k":"e","d":"racing","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let track = world.spawn();
world.insert(track, Track {
    name_hash: 0xDEADBEEF,
    total_checkpoints: 12,
    total_sectors: 3,
    lap_count: 3,
    track_length: 5200.0,
});
world.insert(track, RaceStartGrid {
    positions: vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(-2.0, -5.0),
        Vec2::new(2.0, -5.0),
        Vec2::new(-2.0, -10.0),
    ],
    orientations: vec![0.0; 4],
    grid_size: 4,
});
world.push_event(LoadTrackRequest { track_entity: track });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Racing - Index](_index.md) | Vue d'ensemble du pack |
| [mge-race-vehicle](mge-race-vehicle.md) | Plugin vehicule (consomme TrackSurface) |
| [mge-race-lap](mge-race-lap.md) | Plugin tours (consomme CheckpointReached) |
