# mge-sb-season

> @id mge.sandbox.season.v1  
> @role plugin  
> @domain sandbox  
> @do manage_seasonal_cycle_growth_effects  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-season` |
| @id MSCM | `mge.sandbox.season.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-sb-terrain` |
| Hot path | Non (advance_season_clock 1x/tick, O(1)) |
| Headless safe | Oui |
| Complexite globale | O(1) pour clock, O(t) pour effets (t = tiles) |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `Season` | `Spring, Summer, Autumn, Winter` | Saison courante |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `SeasonClock` | `mge.sandbox.season.v1.component.season_clock` | `current_season: Season, day_in_season: u32, days_per_season: u32` | Horloge saisonniere. Singleton. Avance de 1 jour/cycle |
| `SeasonEffect` | `mge.sandbox.season.v1.component.season_effect` | `growth_multiplier: f32, decay_multiplier: f32, comfort_modifier: f32` | Effets de la saison courante. Recalcule a chaque changement |
| `GrowthModifier` | `mge.sandbox.season.v1.component.growth_modifier` | `base_rate: f32, seasonal_bonus: f32` | Modificateur de croissance applique aux cultures/arbres |

---

## 4. Formules

```
Season effects:
  Spring : growth_multiplier=1.5, decay_multiplier=0.8, comfort_modifier=0.1
  Summer : growth_multiplier=2.0, decay_multiplier=1.0, comfort_modifier=0.0
  Autumn : growth_multiplier=0.5, decay_multiplier=1.5, comfort_modifier=-0.1
  Winter : growth_multiplier=0.0, decay_multiplier=2.0, comfort_modifier=-0.3

effective_growth = base_rate * (1.0 + seasonal_bonus) * growth_multiplier
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `advance_season_clock` | `mge.sandbox.season.v1.fn.advance_season_clock` | 1535 | SeasonClock | SeasonClock | SeasonChanged, DayAdvanced | O(1) | Incremente day_in_season. Si >= days_per_season → change saison. Emet SeasonChanged |
| `apply_season_effects` | `mge.sandbox.season.v1.fn.apply_season_effects` | 1536 | SeasonClock | SeasonEffect | none | O(1) | Recalcule SeasonEffect selon la saison courante |
| `update_growth_modifiers` | `mge.sandbox.season.v1.fn.update_growth_modifiers` | 1537 | SeasonEffect, GrowthModifier | GrowthModifier | none | O(g) | Met a jour seasonal_bonus selon SeasonEffect. g = entites avec GrowthModifier |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `SeasonChanged` | `mge.sandbox.season.v1.event.season_changed` | `old_season: Season, new_season: Season` | `advance_season_clock` | weather (recalcul), terrain (fertility), wildlife (migration), ui |
| `DayAdvanced` | `mge.sandbox.season.v1.event.day_advanced` | `day: u32, season: Season` | `advance_season_clock` | agent (schedule), need (daily), ui |

---

## 7. Invariants

- `SeasonClock.day_in_season` est dans [0, days_per_season).
- Les saisons suivent l'ordre cyclique : Spring → Summer → Autumn → Winter → Spring.
- `SeasonEffect` est coherent avec `SeasonClock.current_season` apres `apply_season_effects`.
- En Winter, `growth_multiplier` = 0.0 (pas de croissance).
- `SeasonChanged` n'est emis qu'une fois par transition.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `days_per_season` | `u32` | 28 | [7, 120] | Duree d'une saison en jours |
| `ticks_per_day` | `u32` | 1440 | [60, 7200] | Ticks par jour (24min@60fps) |
| `start_season` | `Season` | Spring | {Spring, Summer, Autumn, Winter} | Saison de depart |
| `enable_winter_growth_stop` | `bool` | true | {true, false} | Bloquer croissance en hiver |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Fait avancer le cycle des saisons | Ne gere pas la meteo (→ weather) |
| Calcule les effets saisonniers | Ne modifie pas le terrain directement (→ terrain) |
| Modifie les taux de croissance | Ne gere pas la faune (→ wildlife) |
| Emet les changements de jour et saison | Ne gere pas le temps reel (→ timer) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | SeasonClock, SeasonEffect, GrowthModifier |
| Ecrit | SeasonClock, SeasonEffect, GrowthModifier |
| Emet | SeasonChanged, DayAdvanced |
| Ne touche jamais | TerrainTile, Building, Need, Agent, Weather, Wildlife |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-season/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.season.v1
    ├── components.rs     # SeasonClock, SeasonEffect, GrowthModifier
    ├── systems.rs        # advance_season_clock, apply_season_effects, update_growth_modifiers
    └── events.rs         # SeasonChanged, DayAdvanced
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
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] 1 enumeration (Season)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : season cycle, day advance, effects calculation, growth modifiers
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.season.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.season.v1.component.season_clock","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.season.v1.component.season_effect","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.season.v1.component.growth_modifier","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.season.v1.fn.advance_season_clock","k":"s","d":"sandbox","r":["SeasonClock"],"w":["SeasonClock"],"e":["SeasonChanged","DayAdvanced"],"p":1535,"c":"O(1)"},
  {"i":"mge.sandbox.season.v1.fn.apply_season_effects","k":"s","d":"sandbox","r":["SeasonClock"],"w":["SeasonEffect"],"e":[],"p":1536,"c":"O(1)"},
  {"i":"mge.sandbox.season.v1.fn.update_growth_modifiers","k":"s","d":"sandbox","r":["SeasonEffect","GrowthModifier"],"w":["GrowthModifier"],"e":[],"p":1537,"c":"O(g)"},
  {"i":"mge.sandbox.season.v1.event.season_changed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.season.v1.event.day_advanced","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let clock = world.spawn();
world.insert(clock, SeasonClock { current_season: Season::Spring, day_in_season: 0, days_per_season: 28 });
world.insert(clock, SeasonEffect { growth_multiplier: 1.5, decay_multiplier: 0.8, comfort_modifier: 0.1 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sb-terrain](mge-sb-terrain.md) | Plugin terrain (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
