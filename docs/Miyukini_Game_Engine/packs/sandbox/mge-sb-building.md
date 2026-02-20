# mge-sb-building

> @id mge.sandbox.building.v1  
> @role plugin  
> @domain sandbox  
> @do manage_building_placement_construction_demolition  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-building` |
| @id MSCM | `mge.sandbox.building.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-sb-terrain` |
| Hot path | Non (actions ponctuelles) |
| Headless safe | Oui |
| Complexite globale | O(b) par tick, b = batiments en construction |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `BuildingState` | `Blueprint, UnderConstruction, Built, Damaged, Demolished` | Cycle de vie du batiment |
| `BuildingCategory` | `Residential, Production, Storage, Decoration, Infrastructure` | Classification fonctionnelle |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Building` | `mge.sandbox.building.v1.component.building` | `category: BuildingCategory, state: BuildingState, durability: f32, max_durability: f32` | Batiment place. Durability = etat structural |
| `Blueprint` | `mge.sandbox.building.v1.component.blueprint` | `building_def_id: u32, position_x: f32, position_z: f32, rotation: f32` | Plan de construction. Consomme a la validation |
| `ConstructionProgress` | `mge.sandbox.building.v1.component.construction_progress` | `progress: f32, required_materials: Vec<(u32, u32)>, materials_supplied: Vec<(u32, u32)>` | Avancement. progress 0.0-1.0. materials = (item_id, quantity) |

---

## 4. Formules

```
construction_speed = base_construction_speed * workers_count * weather_modifier
progress += construction_speed per tick (si materials_supplied >= required)

durability_decay = weather_damage_rate * (1.0 - shelter_bonus)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `validate_placement` | `mge.sandbox.building.v1.fn.validate_placement` | 1510 | Blueprint, TerrainTile | Building | BuildingPlaced | O(1) | Verifie terrain valide, pas de collision. Cree Building en etat Blueprint |
| `advance_construction` | `mge.sandbox.building.v1.fn.advance_construction` | 1511 | Building, ConstructionProgress | ConstructionProgress, Building | ConstructionCompleted | O(b) | Avance progress si materiaux fournis. A 1.0 → state = Built |
| `process_demolition` | `mge.sandbox.building.v1.fn.process_demolition` | 1512 | Building | Building | BuildingDemolished | O(d) | Passe state Demolished. Libere les materiaux partiels |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `BuildingPlaced` | `mge.sandbox.building.v1.event.building_placed` | `entity: EntityId, category: BuildingCategory, x: f32, z: f32` | `validate_placement` | terrain (mark occupied), agent, ui |
| `ConstructionCompleted` | `mge.sandbox.building.v1.event.construction_completed` | `entity: EntityId, category: BuildingCategory` | `advance_construction` | agent (use building), crafting (station ready), ui |
| `BuildingDemolished` | `mge.sandbox.building.v1.event.building_demolished` | `entity: EntityId, materials_recovered: Vec<(u32, u32)>` | `process_demolition` | inventory (recover), terrain (free), ui |

---

## 7. Invariants

- Un Building `Blueprint` ne peut pas etre utilise (pas encore construit).
- `ConstructionProgress.progress` est dans [0.0, 1.0].
- Un Building `Demolished` n'a plus de ConstructionProgress.
- Un Building ne peut etre place que sur un terrain non-Water et non-Hole.
- `Building.durability` ne depasse jamais `max_durability`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `base_construction_speed` | `f32` | 0.01 | [0.001, 0.1] | Vitesse de construction/tick |
| `weather_damage_rate` | `f32` | 0.001 | [0.0, 0.01] | Degats meteo sur durability/tick |
| `demolition_material_recovery` | `f32` | 0.5 | [0.0, 1.0] | Pourcentage materiaux recuperes |
| `max_buildings_per_chunk` | `u32` | 16 | [4, 64] | Limite de batiments par chunk |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Valide le placement des batiments | Ne gere pas le terrain (→ terrain) |
| Fait avancer la construction | Ne gere pas les materiaux (→ crafting/inventory) |
| Gere la demolition et recuperation | Ne gere pas les agents constructeurs (→ agent) |
| Suit la durabilite des batiments | Ne fait pas le rendu (→ core render) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Building, Blueprint, ConstructionProgress, TerrainTile |
| Ecrit | Building, ConstructionProgress |
| Emet | BuildingPlaced, ConstructionCompleted, BuildingDemolished |
| Ne touche jamais | CraftingStation, Need, Agent, Weather, Wildlife, Chunk |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-building/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.building.v1
    ├── components.rs     # Building, Blueprint, ConstructionProgress
    ├── systems.rs        # validate_placement, advance_construction, process_demolition
    └── events.rs         # BuildingPlaced, ConstructionCompleted, BuildingDemolished
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
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (BuildingState, BuildingCategory)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : placement valid/invalid, construction progress, demolition recovery
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.building.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.building.v1.component.building","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.building.v1.component.blueprint","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.building.v1.component.construction_progress","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.building.v1.fn.validate_placement","k":"s","d":"sandbox","r":["Blueprint","TerrainTile"],"w":["Building"],"e":["BuildingPlaced"],"p":1510,"c":"O(1)"},
  {"i":"mge.sandbox.building.v1.fn.advance_construction","k":"s","d":"sandbox","r":["Building","ConstructionProgress"],"w":["ConstructionProgress","Building"],"e":["ConstructionCompleted"],"p":1511,"c":"O(b)"},
  {"i":"mge.sandbox.building.v1.fn.process_demolition","k":"s","d":"sandbox","r":["Building"],"w":["Building"],"e":["BuildingDemolished"],"p":1512,"c":"O(d)"},
  {"i":"mge.sandbox.building.v1.event.building_placed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.building.v1.event.construction_completed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.building.v1.event.building_demolished","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let building = world.spawn();
world.insert(building, Blueprint { building_def_id: 1, position_x: 10.0, position_z: 20.0, rotation: 0.0 });
world.insert(building, Building {
    category: BuildingCategory::Residential,
    state: BuildingState::Blueprint,
    durability: 100.0, max_durability: 100.0,
});
world.insert(building, ConstructionProgress {
    progress: 0.0,
    required_materials: vec![(1, 10), (2, 5)],
    materials_supplied: vec![],
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [mge-sb-terrain](mge-sb-terrain.md) | Plugin terrain (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
