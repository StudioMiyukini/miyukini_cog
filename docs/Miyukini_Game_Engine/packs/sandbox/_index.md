# MGE — Pack Sandbox

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  
**Couche** : Layer 2 (Genre Pack)  
**Repertoire** : `mge/crates/sandbox/`  
**Nombre de crates** : 9  

---

## 1. Contexte

Le Pack Sandbox modelise les mondes ouverts et simulations de vie : monde persistant par chunks, terrain modifiable, construction, crafting, besoins de survie, agents autonomes, saisons, meteo et faune. Il s'appuie sur le Core Universal (spatial, physics) et le Pack RPG (inventory) pour le crafting.

Tous les crates sont scaffoldes (v0.1.0). Les composants, systemes et evenements decrits dans les fichiers plugin constituent la specification d'implementation cible.

---

## 2. Portee

- **Types de jeux** : Survival, crafting, simulation de vie, city builder, farming sim.
- **Hors portee** : Logique specifique a un jeu, rendu, audio, reseau, combat (→ Pack RPG).
- **Audience** : Developpeurs moteur, developpeurs de contenu, LLM.
- **Prerequis** : Kernel Layer 0 (mge-ecs, mge-event). Core Universal Pack (spatial, physics). Pack RPG (inventory) pour le crafting.

---

## 3. Vision

Le Pack Sandbox est un ensemble de plugins simulation-first. Chaque plugin :

- Fournit des composants (donnees pures) et des systemes (1 fn = 1 effet).
- Ne contient aucune logique de jeu specifique.
- S'execute en headless sans rendu.
- Produit un comportement deterministe a seed et input identiques.
- Expose ses parametres via GCL pour configuration sans recompilation.

---

## 4. Architecture globale

```
mge/crates/sandbox/
├── mge-sb-world/            # Monde, chunks, persistance
├── mge-sb-terrain/          # Tiles terrain, modification, fertilite
├── mge-sb-building/         # Placement, construction, demolition
├── mge-sb-crafting/         # Recettes, fabrication, ateliers
├── mge-sb-need/             # Besoins survie (faim, soif, repos)
├── mge-sb-agent/            # Agents autonomes, routines, decisions
├── mge-sb-season/           # Saisons, cycle annuel, croissance
├── mge-sb-weather/          # Meteo, temperature, precipitation
└── mge-sb-wildlife/         # Faune, spawn, comportements, migration
```

### Graphe de dependances intra-pack

```
mge-sb-agent ──────► mge-sb-need
mge-sb-world ──────► mge-sb-terrain
mge-sb-building ───► mge-sb-terrain
mge-sb-crafting ───► (mge-rpg-inventory, externe)
mge-sb-wildlife ───► mge-sb-world
mge-sb-season ─────► mge-sb-terrain
mge-sb-weather ────► mge-sb-season
```

Crates feuilles (sans dependance intra-pack) : `mge-sb-need`, `mge-sb-terrain`, `mge-sb-crafting`.

---

## 5. Sous-packs

Aucun. Les 9 crates forment un seul pack plat.

---

## 6. Liste des plugins

| # | Crate | @id MSCM | Documentation | Role |
|---|-------|----------|---------------|------|
| 1 | `mge-sb-world` | `mge.sandbox.world.v1` | [mge-sb-world.md](mge-sb-world.md) | Monde, chunks, chargement, persistance |
| 2 | `mge-sb-terrain` | `mge.sandbox.terrain.v1` | [mge-sb-terrain.md](mge-sb-terrain.md) | Tiles terrain, modification, fertilite |
| 3 | `mge-sb-building` | `mge.sandbox.building.v1` | [mge-sb-building.md](mge-sb-building.md) | Placement, construction, demolition |
| 4 | `mge-sb-crafting` | `mge.sandbox.crafting.v1` | [mge-sb-crafting.md](mge-sb-crafting.md) | Recettes, fabrication, ateliers |
| 5 | `mge-sb-need` | `mge.sandbox.need.v1` | [mge-sb-need.md](mge-sb-need.md) | Besoins survie (faim, soif, repos, confort) |
| 6 | `mge-sb-agent` | `mge.sandbox.agent.v1` | [mge-sb-agent.md](mge-sb-agent.md) | Agents autonomes, routines, decisions |
| 7 | `mge-sb-season` | `mge.sandbox.season.v1` | [mge-sb-season.md](mge-sb-season.md) | Saisons, cycle annuel, effets croissance |
| 8 | `mge-sb-weather` | `mge.sandbox.weather.v1` | [mge-sb-weather.md](mge-sb-weather.md) | Meteo, temperature, precipitation |
| 9 | `mge-sb-wildlife` | `mge.sandbox.wildlife.v1` | [mge-sb-wildlife.md](mge-sb-wildlife.md) | Faune, spawn, comportements, migration |

---

## 7. Composants cles (resume)

| Plugin | Composants runtime | Composants donnees statiques |
|--------|-------------------|------------------------------|
| world | WorldConfig, Chunk, ActiveChunks | aucun |
| terrain | TerrainTile, TerrainModification, Fertility | aucun |
| building | Building, Blueprint, ConstructionProgress | aucun |
| crafting | CraftingStation, CraftingAction | Recipe |
| need | Need, NeedSet, NeedSatisfier | aucun |
| agent | Agent, Routine, Decision | aucun |
| season | SeasonClock, SeasonEffect, GrowthModifier | aucun |
| weather | Weather, Temperature, Wind | aucun |
| wildlife | Wildlife, SpawnZone, MigrationPath, HerdMember | aucun |

---

## 8. Systemes cles (resume)

| Phase | Plugin | Systemes |
|-------|--------|----------|
| 1500-1502 | world | update_active_chunks, load_chunks, save_modified_chunks |
| 1505-1507 | terrain | apply_terrain_modifications, update_fertility, erode_terrain |
| 1510-1512 | building | validate_placement, advance_construction, process_demolition |
| 1515-1517 | crafting | validate_craft, advance_crafting, complete_crafting |
| 1520-1523 | need | decay_needs, apply_satisfiers, evaluate_urgency, check_critical_needs |
| 1525-1528 | agent | evaluate_decisions, execute_routine_step, update_agent_state, process_agent_idle |
| 1535-1537 | season | advance_season_clock, apply_season_effects, update_growth_modifiers |
| 1540-1542 | weather | update_weather, update_temperature, apply_weather_effects |
| 1545-1548 | wildlife | spawn_wildlife, update_wildlife_behavior, process_migration, cull_wildlife |

**Ordre d'execution** : world (1500) → terrain (1505) → building (1510) → crafting (1515) → need (1520) → agent (1525) → season (1535) → weather (1540) → wildlife (1545).

**Justification** : Le monde est charge en premier. Le terrain est mis a jour. Les constructions avancent sur le terrain stable. Le crafting consomme l'inventaire. Les besoins decayent. Les agents prennent des decisions basees sur les besoins. Les saisons avancent. La meteo suit les saisons. La faune reagit a l'environnement final.

**Total** : 31 systemes.

---

## 9. Evenements cles (resume)

| Plugin | Requests (entree) | Events (sortie) |
|--------|-------------------|------------------|
| world | (aucun) | ChunkLoaded, ChunkUnloaded, WorldSeedChanged |
| terrain | TerrainModification (composant) | TerrainModified, FertilityChanged |
| building | Blueprint (composant) | BuildingPlaced, ConstructionCompleted, BuildingDemolished |
| crafting | CraftingAction (composant) | CraftingStarted, CraftingCompleted, CraftingFailed |
| need | NeedSatisfier (composant) | NeedUrgencyChanged, NeedSatisfied, NeedCritical, NeedDepleted |
| agent | (aucun, lit NeedUrgency) | AgentStateChanged, RoutineCompleted, DecisionMade, AgentStuck |
| season | (aucun) | SeasonChanged, DayAdvanced |
| weather | (aucun) | WeatherChanged, TemperatureChanged, StormStarted, StormEnded |
| wildlife | (aucun) | WildlifeSpawned, WildlifeDespawned, MigrationStarted, HerdFormed |

**Total** : 26 evenements.

---

## 10. Dependances

### Dependances vers Kernel (Layer 0)

| Crate | Depend de |
|-------|-----------|
| Tous les 9 crates | `mge-ecs`, `mge-event` |

### Dependances vers Core Universal

| Crate | Depend de |
|-------|-----------|
| world, wildlife | `mge-plugin-spatial` |
| building | `mge-plugin-physics` (collision placement) |

### Dependances vers Pack RPG (Layer 2)

| Crate | Depend de |
|-------|-----------|
| `mge-sb-crafting` | `mge-rpg-inventory` (consommation/production items) |

### Dependances intra-pack

| Crate | Depend de |
|-------|-----------|
| `mge-sb-world` | `mge-sb-terrain` |
| `mge-sb-building` | `mge-sb-terrain` |
| `mge-sb-agent` | `mge-sb-need` |
| `mge-sb-season` | `mge-sb-terrain` |
| `mge-sb-weather` | `mge-sb-season` |
| `mge-sb-wildlife` | `mge-sb-world` |

### Dependances externes (aucune)

Le Pack Sandbox n'a aucune dependance vers des crates externes.

---

## 11. Interaction avec GCL

**Parametres exposables :**

- Taille chunks, rayon de chargement, seed monde
- Taux d'erosion, fertilite
- Materiaux construction, durees
- Recettes crafting, durees fabrication
- Taux decay besoins, seuils critiques
- Planning agents, priorites decisions
- Duree saisons, multiplicateurs croissance
- Probabilites meteo, plages temperature
- Populations faune, zones spawn, migration

---

## 12. Exemple d'assemblage

### Minimal (headless, monde + terrain)

```rust
let mut engine = Engine::new(EngineConfig::default());
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgeSbWorldPlugin);
engine.add_plugin(MgeSbTerrainPlugin);
engine.build();
```

### Complet (Sandbox jouable)

```rust
let mut engine = Engine::new(EngineConfig::default());
// Core Universal
engine.add_plugin(MgePluginSpatial::default());
engine.add_plugin(MgePluginPhysics::default());
engine.add_plugin(MgePluginInput::default());
engine.add_plugin(MgePluginRender2d::default());
// Pack RPG (inventory pour crafting)
engine.add_plugin(MgeRpgInventoryPlugin);
// Pack Sandbox
engine.add_plugin(MgeSbWorldPlugin);
engine.add_plugin(MgeSbTerrainPlugin);
engine.add_plugin(MgeSbBuildingPlugin);
engine.add_plugin(MgeSbCraftingPlugin);
engine.add_plugin(MgeSbNeedPlugin);
engine.add_plugin(MgeSbAgentPlugin);
engine.add_plugin(MgeSbSeasonPlugin);
engine.add_plugin(MgeSbWeatherPlugin);
engine.add_plugin(MgeSbWildlifePlugin);
engine.build();
```

---

## References

| Document | Role |
|----------|------|
| [MGE - Pack Architecture](../MGE%20-%20Pack%20Architecture.md) | Couches, composition |
| [MGE - Architecture Generale](../MGE%20-%20Architecture%20Generale.md) | Couches globales |
| [MGE - Plugin Contract](../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
| [MGE - AI-Native Writing Standard v1](../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [Pack RPG - Index](../rpg/_index.md) | Pack RPG (dependance crafting) |
