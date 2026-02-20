# mge-sb-crafting

> @id mge.sandbox.crafting.v1  
> @role plugin  
> @domain sandbox  
> @do manage_recipes_crafting_stations_production  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-crafting` |
| @id MSCM | `mge.sandbox.crafting.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-rpg-inventory` |
| Hot path | Non (actions ponctuelles) |
| Headless safe | Oui |
| Complexite globale | O(a) par tick, a = crafts actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `CraftingResult` | `Success, InsufficientMaterials, StationRequired, SkillTooLow` | Resultat de validation |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Recipe` | `mge.sandbox.crafting.v1.component.recipe` | `recipe_id: u32, inputs: Vec<(u32, u32)>, outputs: Vec<(u32, u32)>, craft_ticks: u32, station_type: Option<u32>` | Definition de recette (donnee statique). inputs/outputs = (item_id, quantity) |
| `CraftingStation` | `mge.sandbox.crafting.v1.component.crafting_station` | `station_type: u32, efficiency: f32` | Station de fabrication. efficiency multiplie la vitesse |
| `CraftingAction` | `mge.sandbox.crafting.v1.component.crafting_action` | `recipe_id: u32, station: Option<EntityId>, progress: f32` | Action de craft en cours. progress 0.0-1.0 |

---

## 4. Formules

```
craft_speed = (1.0 / recipe.craft_ticks) * station.efficiency
progress += craft_speed per tick

Validation : tous inputs presents dans Inventory, station_type match si requis
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `validate_craft` | `mge.sandbox.crafting.v1.fn.validate_craft` | 1515 | CraftingAction, Recipe, Inventory, CraftingStation | CraftingAction | CraftingStarted, CraftingFailed | O(1) | Verifie materiaux et station. Consomme inputs si valide. Emet CraftingStarted ou CraftingFailed |
| `advance_crafting` | `mge.sandbox.crafting.v1.fn.advance_crafting` | 1516 | CraftingAction, Recipe, CraftingStation | CraftingAction | none | O(a) | Avance progress selon craft_speed |
| `complete_crafting` | `mge.sandbox.crafting.v1.fn.complete_crafting` | 1517 | CraftingAction, Recipe, Inventory | Inventory, CraftingAction | CraftingCompleted | O(a) | Si progress >= 1.0, ajoute outputs a Inventory. Retire CraftingAction |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `CraftingStarted` | `mge.sandbox.crafting.v1.event.crafting_started` | `entity: EntityId, recipe_id: u32` | `validate_craft` | agent (wait), ui |
| `CraftingCompleted` | `mge.sandbox.crafting.v1.event.crafting_completed` | `entity: EntityId, recipe_id: u32, outputs: Vec<(u32, u32)>` | `complete_crafting` | agent, quest, ui |
| `CraftingFailed` | `mge.sandbox.crafting.v1.event.crafting_failed` | `entity: EntityId, recipe_id: u32, reason: CraftingResult` | `validate_craft` | agent (retry), ui |

---

## 7. Invariants

- `CraftingAction.progress` est dans [0.0, 1.0].
- Les inputs sont consommes a la validation (pas a la completion).
- Si un CraftingAction est annule, les inputs ne sont PAS rendus (design choice : engagement).
- Une Recipe avec `station_type = None` peut etre craftee sans station.
- `CraftingStation.efficiency` est toujours > 0.0.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_craft_ticks` | `u32` | 120 | [10, 6000] | Duree par defaut si Recipe n'en specifie pas |
| `allow_cancel_refund` | `bool` | false | {true, false} | Si true, annulation rend les materiaux |
| `max_concurrent_crafts` | `u32` | 3 | [1, 10] | Crafts simultanes par entite |
| `station_efficiency_bonus` | `f32` | 1.0 | [0.5, 3.0] | Bonus global d'efficacite stations |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Valide les recettes et materiaux | Ne gere pas l'inventaire (→ rpg-inventory) |
| Fait avancer les crafts en cours | Ne gere pas les stations comme batiments (→ building) |
| Produit les items finis | Ne gere pas les besoins (→ need) |
| Respecte les stations requises | Ne gere pas les competences (→ rpg-progression) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | CraftingAction, Recipe, CraftingStation, Inventory |
| Ecrit | CraftingAction, Inventory |
| Emet | CraftingStarted, CraftingCompleted, CraftingFailed |
| Ne touche jamais | TerrainTile, Building, Need, Agent, Weather, Wildlife |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-crafting/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.crafting.v1
    ├── components.rs     # Recipe, CraftingStation, CraftingAction
    ├── systems.rs        # validate_craft, advance_crafting, complete_crafting
    └── events.rs         # CraftingStarted, CraftingCompleted, CraftingFailed
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
- [ ] 1 enumeration (CraftingResult)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : craft valid, insufficient materials, station required, completion
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.crafting.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.crafting.v1.component.recipe","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.crafting.v1.component.crafting_station","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.crafting.v1.component.crafting_action","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.crafting.v1.fn.validate_craft","k":"s","d":"sandbox","r":["CraftingAction","Recipe","Inventory","CraftingStation"],"w":["CraftingAction"],"e":["CraftingStarted","CraftingFailed"],"p":1515,"c":"O(1)"},
  {"i":"mge.sandbox.crafting.v1.fn.advance_crafting","k":"s","d":"sandbox","r":["CraftingAction","Recipe","CraftingStation"],"w":["CraftingAction"],"e":[],"p":1516,"c":"O(a)"},
  {"i":"mge.sandbox.crafting.v1.fn.complete_crafting","k":"s","d":"sandbox","r":["CraftingAction","Recipe","Inventory"],"w":["Inventory","CraftingAction"],"e":["CraftingCompleted"],"p":1517,"c":"O(a)"},
  {"i":"mge.sandbox.crafting.v1.event.crafting_started","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.crafting.v1.event.crafting_completed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.crafting.v1.event.crafting_failed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let recipe = world.spawn();
world.insert(recipe, Recipe {
    recipe_id: 1,
    inputs: vec![(10, 3), (11, 2)],
    outputs: vec![(20, 1)],
    craft_ticks: 120,
    station_type: Some(1),
});

let action = world.spawn();
world.insert(action, CraftingAction { recipe_id: 1, station: Some(station_entity), progress: 0.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [Pack RPG - mge-rpg-inventory](../rpg/mge-rpg-inventory.md) | Plugin inventaire (dependance) |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
