# mge-sb-weather

> @id mge.sandbox.weather.v1  
> @role plugin  
> @domain sandbox  
> @do simulate_weather_temperature_precipitation  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-weather` |
| @id MSCM | `mge.sandbox.weather.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-sb-season` |
| Hot path | Non (O(1) par tick) |
| Headless safe | Oui |
| Complexite globale | O(1) |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `WeatherType` | `Clear, Cloudy, Rain, HeavyRain, Snow, Storm, Fog` | Type de meteo courant |
| `WindDirection` | `North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest` | Direction du vent |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Weather` | `mge.sandbox.weather.v1.component.weather` | `weather_type: WeatherType, intensity: f32, duration_ticks: u32` | Meteo courante. intensity 0.0-1.0. duration restante |
| `Temperature` | `mge.sandbox.weather.v1.component.temperature` | `current: f32, base: f32, seasonal_offset: f32` | Temperature en degres. current = base + seasonal_offset + variations |
| `Wind` | `mge.sandbox.weather.v1.component.wind` | `direction: WindDirection, speed: f32` | Vent. speed en unites/tick. Affecte erosion et confort |

---

## 4. Formules

```
temperature:
  current = base + seasonal_offset + weather_offset
  weather_offset:
    Rain      = -2.0
    HeavyRain = -5.0
    Snow      = -10.0
    Storm     = -8.0
    Clear     = +3.0

weather_transition_probability (per tick):
  Clear  → Cloudy : 0.001
  Cloudy → Rain   : 0.005 (Spring/Autumn), 0.002 (Summer), 0.008 (Winter→Snow)
  Rain   → Clear  : 0.01
  Storm duration   : 60-300 ticks
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `update_weather` | `mge.sandbox.weather.v1.fn.update_weather` | 1540 | Weather, SeasonClock | Weather | WeatherChanged, StormStarted, StormEnded | O(1) | Decremente duration. Si 0, transition aleatoire. Emet si changement |
| `update_temperature` | `mge.sandbox.weather.v1.fn.update_temperature` | 1541 | Temperature, Weather, SeasonEffect | Temperature | TemperatureChanged | O(1) | Recalcule current selon base, season et weather |
| `apply_weather_effects` | `mge.sandbox.weather.v1.fn.apply_weather_effects` | 1542 | Weather, Fertility | Fertility | none | O(t) | Rain → augmente moisture. Snow → gele. Storm → erosion. t = tiles affectees |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `WeatherChanged` | `mge.sandbox.weather.v1.event.weather_changed` | `old_type: WeatherType, new_type: WeatherType, intensity: f32` | `update_weather` | terrain (moisture), need (comfort), ui |
| `TemperatureChanged` | `mge.sandbox.weather.v1.event.temperature_changed` | `old_temp: f32, new_temp: f32` | `update_temperature` | need (comfort), wildlife (migration), ui |
| `StormStarted` | `mge.sandbox.weather.v1.event.storm_started` | `intensity: f32, duration_ticks: u32` | `update_weather` | building (damage), agent (shelter), ui |
| `StormEnded` | `mge.sandbox.weather.v1.event.storm_ended` | `total_duration: u32` | `update_weather` | agent (resume), ui |

---

## 7. Invariants

- `Weather.intensity` est dans [0.0, 1.0].
- `Weather.duration_ticks` decremente de 1 par tick. Transition quand = 0.
- `Temperature.current` est coherent avec base + offsets apres `update_temperature`.
- `Wind.speed` est >= 0.0.
- `Snow` ne peut apparaitre que si `Temperature.current < 0.0`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `base_temperature` | `f32` | 20.0 | [-20.0, 50.0] | Temperature de base (degres) |
| `storm_probability` | `f32` | 0.001 | [0.0, 0.01] | Probabilite de tempete/tick |
| `rain_moisture_rate` | `f32` | 0.05 | [0.0, 0.2] | Augmentation moisture sous la pluie |
| `min_weather_duration` | `u32` | 120 | [60, 600] | Duree minimum d'un type de meteo |
| `max_wind_speed` | `f32` | 5.0 | [1.0, 20.0] | Vitesse max du vent |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Simule les transitions meteo | Ne gere pas les saisons (→ season) |
| Calcule la temperature | Ne modifie pas le terrain directement (→ terrain) |
| Affecte la moisture des sols | Ne gere pas les batiments (→ building) |
| Emet tempetes et changements | Ne gere pas le confort des agents (→ need) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Weather, Temperature, Wind, SeasonClock, SeasonEffect, Fertility |
| Ecrit | Weather, Temperature, Wind, Fertility (moisture) |
| Emet | WeatherChanged, TemperatureChanged, StormStarted, StormEnded |
| Ne touche jamais | Building, Agent, CraftingStation, TerrainTile (type), Wildlife |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-weather/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.weather.v1
    ├── components.rs     # Weather, Temperature, Wind
    ├── systems.rs        # update_weather, update_temperature, apply_weather_effects
    └── events.rs         # WeatherChanged, TemperatureChanged, StormStarted, StormEnded
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
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (WeatherType, WindDirection)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : weather transition, temperature calc, storm start/end, snow threshold
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.weather.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.weather.v1.component.weather","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.weather.v1.component.temperature","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.weather.v1.component.wind","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.weather.v1.fn.update_weather","k":"s","d":"sandbox","r":["Weather","SeasonClock"],"w":["Weather"],"e":["WeatherChanged","StormStarted","StormEnded"],"p":1540,"c":"O(1)"},
  {"i":"mge.sandbox.weather.v1.fn.update_temperature","k":"s","d":"sandbox","r":["Temperature","Weather","SeasonEffect"],"w":["Temperature"],"e":["TemperatureChanged"],"p":1541,"c":"O(1)"},
  {"i":"mge.sandbox.weather.v1.fn.apply_weather_effects","k":"s","d":"sandbox","r":["Weather","Fertility"],"w":["Fertility"],"e":[],"p":1542,"c":"O(t)"},
  {"i":"mge.sandbox.weather.v1.event.weather_changed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.weather.v1.event.temperature_changed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.weather.v1.event.storm_started","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.weather.v1.event.storm_ended","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let weather_entity = world.spawn();
world.insert(weather_entity, Weather { weather_type: WeatherType::Clear, intensity: 0.0, duration_ticks: 300 });
world.insert(weather_entity, Temperature { current: 22.0, base: 20.0, seasonal_offset: 2.0 });
world.insert(weather_entity, Wind { direction: WindDirection::North, speed: 1.5 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sb-season](mge-sb-season.md) | Plugin season (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
