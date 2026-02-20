# mge-factory-machine

> @id mge.factory.machine.v1  
> @role plugin  
> @domain factory  
> @do manage_machines_states_input_output_production  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-factory-machine` |
| @id MSCM | `mge.factory.machine.v1` |
| Domaine | factory |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-factory-recipe` |
| Hot path | Oui (tick machine chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(m) par tick, m = machines actives |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `MachineType` | `Assembler, Smelter, Refiner, Packager, Splitter, Merger` | Classe de la machine. Influe sur les recettes acceptees |
| `MachineStatus` | `Idle, Working, Jammed, Disabled, WaitingInput, WaitingOutput` | Etat courant. Jammed = output plein et input bloque |
| `SlotType` | `Input, Output` | Type de slot sur la machine |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Machine` | `mge.factory.machine.v1.component.machine` | `machine_type: MachineType, tier: u8, speed_multiplier: f32, active_recipe_id: Option<u32>` | Definition de la machine. tier influe sur la vitesse |
| `MachineState` | `mge.factory.machine.v1.component.machine_state` | `status: MachineStatus, progress_ticks: u32, total_ticks: u32, cycles_completed: u64` | Etat de production. progress_ticks / total_ticks = avancement |
| `InputSlot` | `mge.factory.machine.v1.component.input_slot` | `slot_index: u8, item_id: Option<u32>, quantity: u32, max_capacity: u32` | Slot d'entree. Accueille les ingredients |
| `OutputSlot` | `mge.factory.machine.v1.component.output_slot` | `slot_index: u8, item_id: Option<u32>, quantity: u32, max_capacity: u32` | Slot de sortie. Contient les produits finis |
| `ProcessingTimer` | `mge.factory.machine.v1.component.processing_timer` | `base_duration_ticks: u32, effective_duration_ticks: u32, elapsed_ticks: u32` | Timer de production. effective = base / speed_multiplier |
| `MachineDef` | `mge.factory.machine.v1.component.machine_def` | `id: u32, name_hash: u64, machine_type: MachineType, input_slot_count: u8, output_slot_count: u8, base_speed: f32` | Definition statique (catalogue) |

---

## 4. Formules

```
Duree effective :
  effective_duration = base_duration_ticks / (machine.speed_multiplier * tier_bonus(machine.tier))
  tier_bonus = 1.0 + (tier - 1) * 0.25

Progression :
  if status == Working:
    processing_timer.elapsed_ticks += 1
    if elapsed_ticks >= effective_duration_ticks:
      production_complete = true

Etat machine :
  if inputs_empty AND status != Working:
    status = WaitingInput
  else if outputs_full AND production_complete:
    status = Jammed
  else if production_complete:
    status = Idle (reset pour prochain cycle)

Throughput :
  items_per_minute = 60.0 * 60.0 / effective_duration_ticks * output_quantity
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_machine` | `mge.factory.machine.v1.fn.tick_machine` | 2000 | MachineState, ProcessingTimer, Machine | ProcessingTimer, MachineState | none | O(m) | Incremente le timer des machines en Working. Detecte la completion |
| `consume_inputs` | `mge.factory.machine.v1.fn.consume_inputs` | 2001 | MachineState, InputSlot, Machine, Recipe | InputSlot, MachineState | MachineStarted | O(m) | Quand Idle avec inputs suffisants, consomme les ingredients et passe en Working |
| `produce_outputs` | `mge.factory.machine.v1.fn.produce_outputs` | 2002 | MachineState, OutputSlot, Machine, Recipe | OutputSlot, MachineState, ProcessingTimer | ProductionCompleted, MachineJammed | O(m) | Quand le timer est complete, place les produits dans les output slots. Jam si plein |
| `update_machine_state` | `mge.factory.machine.v1.fn.update_machine_state` | 2003 | MachineState, InputSlot, OutputSlot, Machine | MachineState | MachineStopped | O(m) | Met a jour le statut global (WaitingInput, Jammed, Disabled). Emet si arret |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `MachineStarted` | `mge.factory.machine.v1.event.machine_started` | `entity: EntityId, recipe_id: u32, duration_ticks: u32` | `consume_inputs` | ui (progress bar), audio |
| `MachineStopped` | `mge.factory.machine.v1.event.machine_stopped` | `entity: EntityId, reason: MachineStatus` | `update_machine_state` | ui (status icon), logistics |
| `ProductionCompleted` | `mge.factory.machine.v1.event.production_completed` | `entity: EntityId, recipe_id: u32, output_item_id: u32, output_quantity: u32` | `produce_outputs` | conveyor (pick up), ui, analytics |
| `MachineJammed` | `mge.factory.machine.v1.event.machine_jammed` | `entity: EntityId, blocked_item_id: u32` | `produce_outputs` | ui (warning), logistics (reroute) |

---

## 7. Invariants

- Une machine en `Working` ne peut pas changer de recette (verrouillee jusqu'a completion ou annulation).
- `InputSlot.quantity` ne depasse jamais `max_capacity`.
- `OutputSlot.quantity` ne depasse jamais `max_capacity`.
- `progress_ticks` est reset a 0 a chaque nouveau cycle de production.
- Une machine `Disabled` ne consomme pas et ne produit pas.
- `cycles_completed` ne peut qu'augmenter (compteur monotone).
- Le `speed_multiplier` est toujours > 0 (minimum 0.1).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_input_capacity` | `u32` | 10 | [1, 100] | Capacite par defaut d'un slot d'entree |
| `default_output_capacity` | `u32` | 10 | [1, 100] | Capacite par defaut d'un slot de sortie |
| `tier_speed_bonus` | `f32` | 0.25 | [0.0, 1.0] | Bonus de vitesse par tier supplementaire |
| `jam_retry_ticks` | `u32` | 30 | [1, 120] | Ticks entre chaque tentative de deblocage |
| `max_tier` | `u8` | 5 | [1, 10] | Tier maximum des machines |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere le cycle de production (input → processing → output) | Ne definit pas les recettes (→ recipe) |
| Gere les etats machine (Idle, Working, Jammed...) | Ne transporte pas les items (→ conveyor) |
| Consomme les ingredients et produit les resultats | Ne gere pas le routage global (→ logistics) |
| Gere le speed multiplier et les tiers | Ne gere pas le rendu des machines |
| Emet les evenements de production | Ne gere pas le cout financier (→ tycoon) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Machine, MachineState, InputSlot, OutputSlot, ProcessingTimer, Recipe |
| Ecrit | MachineState, InputSlot, OutputSlot, ProcessingTimer |
| Emet | MachineStarted, MachineStopped, ProductionCompleted, MachineJammed |
| Ne touche jamais | Conveyor, ConveyorBuffer, LogisticsNode, LogisticsRoute, StorageContainer |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-factory-machine/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.factory.machine.v1, trait Plugin impl
    ├── components.rs     # Machine, MachineState, InputSlot, OutputSlot, ProcessingTimer, MachineDef
    ├── systems.rs        # tick_machine, consume_inputs, produce_outputs, update_machine_state
    └── events.rs         # MachineStarted, MachineStopped, ProductionCompleted, MachineJammed
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
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (MachineType, MachineStatus, SlotType)
- [ ] Formules de production documentees
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : production cycle, jam detection, tier speed bonus, input consumption
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.factory.machine.v1","k":"p","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.component.machine","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.component.machine_state","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.component.input_slot","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.component.output_slot","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.component.processing_timer","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.component.machine_def","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.fn.tick_machine","k":"s","d":"factory","r":["MachineState","ProcessingTimer","Machine"],"w":["ProcessingTimer","MachineState"],"e":[],"p":2000,"c":"O(m)"},
  {"i":"mge.factory.machine.v1.fn.consume_inputs","k":"s","d":"factory","r":["MachineState","InputSlot","Machine","Recipe"],"w":["InputSlot","MachineState"],"e":["MachineStarted"],"p":2001,"c":"O(m)"},
  {"i":"mge.factory.machine.v1.fn.produce_outputs","k":"s","d":"factory","r":["MachineState","OutputSlot","Machine","Recipe"],"w":["OutputSlot","MachineState","ProcessingTimer"],"e":["ProductionCompleted","MachineJammed"],"p":2002,"c":"O(m)"},
  {"i":"mge.factory.machine.v1.fn.update_machine_state","k":"s","d":"factory","r":["MachineState","InputSlot","OutputSlot","Machine"],"w":["MachineState"],"e":["MachineStopped"],"p":2003,"c":"O(m)"},
  {"i":"mge.factory.machine.v1.event.machine_started","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.event.machine_stopped","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.event.production_completed","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.machine.v1.event.machine_jammed","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let smelter = world.spawn();
world.insert(smelter, Machine {
    machine_type: MachineType::Smelter,
    tier: 1,
    speed_multiplier: 1.0,
    active_recipe_id: Some(101),
});
world.insert(smelter, MachineState {
    status: MachineStatus::Idle,
    progress_ticks: 0,
    total_ticks: 120,
    cycles_completed: 0,
});
world.insert(smelter, InputSlot {
    slot_index: 0,
    item_id: Some(1),
    quantity: 5,
    max_capacity: 10,
});
world.insert(smelter, OutputSlot {
    slot_index: 0,
    item_id: None,
    quantity: 0,
    max_capacity: 10,
});
world.insert(smelter, ProcessingTimer {
    base_duration_ticks: 120,
    effective_duration_ticks: 120,
    elapsed_ticks: 0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Factory - Index](_index.md) | Vue d'ensemble du pack |
| [mge-factory-recipe](mge-factory-recipe.md) | Plugin recettes (dependance) |
