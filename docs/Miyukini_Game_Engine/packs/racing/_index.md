# MGE — Pack Racing

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/racing/`  
**Nombre de crates** : 4  

---

## 1. Contexte

Le Pack Racing fournit les mecaniques generiques des jeux de course : physique vehicule, circuits, tours/positions et IA pilote. Il s'appuie sur le Core Universal pour le spatial, la physique et l'input.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Arcade racing, kart, rally, F1 simplifie, course futuriste.
- **Hors portee** : Simulation physique realiste (suspensions, deformation carrosserie), open world driving, vehicules armes (voir Pack Shooter), gestion d'ecurie.
- **Audience** : Developpeurs moteur, designers, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack obligatoire (spatial, physics, input).

---

## 3. Vision

Le Pack Racing est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/racing/
├── mge-race-vehicle/       # Physique vehicule, acceleration, drift, nitro
├── mge-race-track/         # Circuit, checkpoints, surfaces
├── mge-race-lap/           # Tours, positions, chrono, resultats
└── mge-race-ai-driver/     # IA pilote, trajectoire, depassement
```

### Graphe de dependances intra-pack

```
mge-race-ai-driver ──────► mge-race-vehicle
        │                       │
        └──► mge-race-track     │
                  │              │
                  └──────► mge-race-lap
```

Crates feuilles (sans dependance intra-pack) : `mge-race-vehicle`.

---

## 5. Sous-packs

Aucun. Les 4 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-race-vehicle` | `mge.race.vehicle.v1` | [mge-race-vehicle.md](mge-race-vehicle.md) | Physique vehicule, acceleration, steering, drift, nitro |
| 2 | `mge-race-track` | `mge.race.track.v1` | [mge-race-track.md](mge-race-track.md) | Circuits, checkpoints, surfaces, detection hors-piste |
| 3 | `mge-race-lap` | `mge.race.lap.v1` | [mge-race-lap.md](mge-race-lap.md) | Tours, positions, chrono, resultats de course |
| 4 | `mge-race-ai-driver` | `mge.race.ai_driver.v1` | [mge-race-ai-driver.md](mge-race-ai-driver.md) | IA pilote, racing line, depassement, adaptation |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|--------------------|------------------------------|
| vehicle | Vehicle, VehicleEngine, Steering, Wheels, Nitro, DriftState | aucun |
| track | Track, Checkpoint, TrackSegment, TrackSurface, RaceStartGrid | aucun |
| lap | LapState, RacePosition, RaceTimer, RaceResult | aucun |
| ai-driver | AIDriver, RacingLine, AIPersonality, AIAwareness | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1900-1904 | vehicle | apply_throttle, apply_steering, compute_traction, apply_drift, consume_nitro |
| 1920-1923 | track | load_track, detect_surface, process_checkpoint, detect_shortcut |
| 1940-1943 | lap | update_lap, compute_positions, check_race_finish, record_best_lap |
| 1960-1963 | ai-driver | follow_racing_line, evaluate_overtake, adapt_speed, avoid_collision |

**Ordre d'execution** : vehicle (1900) → track (1920) → lap (1940) → ai-driver (1960).

**Justification** : la physique vehicule est calculee en premier pour obtenir les positions/velocites. Le track detecte les surfaces et checkpoints apres deplacement. Le lap met a jour tours et positions. L'AI est en dernier pour decider les actions du tick suivant.

**Total** : 17 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| vehicle | (input direct : throttle, steer) | DriftStarted, DriftEnded, NitroActivated, NitroExhausted |
| track | LoadTrackRequest | CheckpointReached, SurfaceChanged, ShortcutDetected, OffTrack |
| lap | (automatique via checkpoints) | LapCompleted, RaceFinished, BestLapSet, PositionChanged |
| ai-driver | (aucun, autonome) | OvertakeAttempted, OvertakeCompleted, AIBraking |

**Total** : 1 request + 13 events = 14 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 4 crates | `mge-ecs`, `mge-event` |

### Dependances vers Core Universal

| Crate | Depend de |
|-------|-----------|
| `mge-race-vehicle` | `mge-plugin-spatial`, `mge-plugin-basic-physics`, `mge-plugin-input` |
| `mge-race-track` | `mge-plugin-spatial` |
| `mge-race-lap` | `mge-plugin-spatial` |
| `mge-race-ai-driver` | `mge-plugin-spatial` |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-race-track` | `mge-race-lap` (checkpoint → lap update) |
| `mge-race-ai-driver` | `mge-race-vehicle`, `mge-race-track` |

### Dependances externes (aucune)

Le Pack Racing n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

Le GCL configure les plugins Racing sans recompilation.

**Parametres exposables :**

- Acceleration, vitesse max, friction par surface
- Angle de drift, bonus drift
- Duree nitro, boost multiplicateur
- Nombre de tours, mode de course
- Agressivite IA, distance depassement

Le GCL ne modifie pas la structure des composants. Il parametre les systemes.

---

## 12. Interaction avec autres packs

| Pack dependant | Crates Racing utilises | Usage |
|----------------|------------------------|-------|
| (aucun actuellement) | — | — |

Packs pouvant s'integrer :

| Pack | Integration possible |
|------|----------------------|
| **Shooter** | Vehicules armes (kart + armes) |

Le Pack Racing ne depend d'aucun autre pack genre.

---

## 13. Contraintes determinisme

| Contrainte | Detail |
|------------|--------|
| **Fixed timestep** | Physique vehicule a dt fixe (pas de variable dt) |
| **Pas de float non deterministe** | Operations deterministes |
| **Seed RNG** | IA utilise mge-rng pour variation comportement |
| **Pas de static mut** | Interdit par la norme AI-Native |
| **Replay compatible** | Input replay produit le meme resultat |

---

## 14. Contraintes performance

| Contrainte | Detail |
|------------|--------|
| **Hot path** | vehicle (physique), track (surface detect) |
| **Budget cible** | < 1ms pour 20 vehicules a 60 FPS |
| **Pas de dynamic dispatch** | Dans le hot path |
| **SoA storage** | Composants stockes en SoA via mge-ecs |
| **Pas d'allocation** | Dans les systemes hot path |

---

## 15. Limites v1

| Limite | Raison |
|--------|--------|
| Pas de suspensions physiques | Simplification v1 (arcade focus) |
| Pas de deformation vehicule | Hors scope |
| Pas de meteo dynamique | Surfaces statiques en v1 |
| Pas de replay system | Hors scope (voir plugin save-load) |
| Pas de split-screen | Couche rendu, hors scope |

---

## 16. Extensions possibles v2

| Extension | Description |
|-----------|-------------|
| Meteo dynamique | Pluie, neige affectant les surfaces en temps reel |
| Suspensions | Modele physique amortisseurs |
| Drafting/slipstream | Bonus vitesse derriere un vehicule |
| Degradation pneus | Usure influencant la traction |
| Custom tracks | Editeur de circuit via donnees |

---

## 17. Exemple d'assemblage

### Minimal (headless, vehicle + track uniquement)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginBasicPhysics::default());
engine.add_plugin(MgeRaceVehiclePlugin);
engine.add_plugin(MgeRaceTrackPlugin);
engine.build();
```

### Complet (course jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginRender2d::default());
engine.add_plugin(MgePluginBasicPhysics::default());
// Pack Racing
engine.add_plugin(MgeRaceVehiclePlugin);
engine.add_plugin(MgeRaceTrackPlugin);
engine.add_plugin(MgeRaceLapPlugin);
engine.add_plugin(MgeRaceAiDriverPlugin);
engine.build();
```

---

## 18. Organisation des crates

```
mge/crates/racing/
├── mge-race-vehicle/
│   ├── Cargo.toml
│   ├── index.md
│   └── src/
│       ├── lib.rs           # @id mge.race.vehicle.v1
│       ├── components.rs
│       ├── systems.rs
│       └── events.rs
├── mge-race-track/
│   └── (meme structure)
├── mge-race-lap/
│   └── (meme structure)
└── mge-race-ai-driver/
    └── (meme structure)
```

---

## 19. Resume strategique

Le Pack Racing est la brique fondamentale des jeux de course dans MGE. Il :

- Fournit 4 plugins couvrant vehicules, circuits, tours/positions et IA pilote.
- Reste generique : aucune logique specifique a un jeu.
- S'execute en headless, en deterministe, sans rendu.
- Focus arcade mais extensible vers simulation via GCL.
- Expose ses parametres via GCL pour iteration rapide.
- Respecte strictement la norme AI-Native (MSCM, 1 fn = 1 effet, max 30 lignes, pas de hidden state).

Les 4 crates sont scaffoldes (v0.1.0). L'implementation suit les specifications des fichiers plugin individuels.

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Platform Tooling Layer v1](../MGE%20-%20Platform%20Tooling%20Layer%20v1.md) | GCL, outils |
