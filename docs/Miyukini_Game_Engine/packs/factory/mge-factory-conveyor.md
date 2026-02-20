# mge-factory-conveyor

> @id mge.factory.conveyor.v1  
> @role plugin  
> @domain factory  
> @do manage_conveyors_item_transport_buffers  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-factory-conveyor` |
| @id MSCM | `mge.factory.conveyor.v1` |
| Domaine | factory |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-factory-machine` |
| Hot path | Oui (deplacement items chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(c * b) par tick, c = convoyeurs actifs, b = items par convoyeur |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ConveyorTier` | `Slow, Normal, Fast, Express` | Vitesse du convoyeur. Express = 4x la vitesse Slow |
| `ConveyorDirection` | `Forward, Left, Right` | Direction de sortie du segment. Forward = tout droit |
| `TransferResult` | `Success, BufferFull, NoTarget, IncompatibleItem` | Resultat d'un transfert d'item |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Conveyor` | `mge.factory.conveyor.v1.component.conveyor` | `tier: ConveyorTier, speed: f32, length: f32, source_entity: Option<EntityId>, target_entity: Option<EntityId>` | Definition du convoyeur. Connecte source a target |
| `ConveyorSegment` | `mge.factory.conveyor.v1.component.conveyor_segment` | `direction: ConveyorDirection, start_pos: Vec2, end_pos: Vec2, segment_index: u32` | Portion de convoyeur. Plusieurs segments forment un chemin |
| `ConveyorBuffer` | `mge.factory.conveyor.v1.component.conveyor_buffer` | `items: Vec<ConveyorItemEntry>, max_items: u32, total_progress: f32` | Buffer d'items en transit. ConveyorItemEntry = {item_id, progress} |
| `ConveyorItem` | `mge.factory.conveyor.v1.component.conveyor_item` | `item_id: u32, quantity: u32, progress: f32, conveyor_entity: EntityId` | Item individuel sur le convoyeur. progress = [0.0, 1.0] |

---

## 4. Formules

```
Deplacement item :
  speed_table = { Slow: 1.0, Normal: 2.0, Fast: 3.0, Express: 4.0 }
  delta_progress = (speed_table[tier] / conveyor.length) * dt
  item.progress += delta_progress

Transfert vers machine :
  if item.progress >= 1.0:
    target_slot = machine.input_slots[compatible_slot]
    if target_slot.quantity + item.quantity <= target_slot.max_capacity:
      transfer = Success
    else:
      transfer = BufferFull

Espacement items :
  min_spacing = 1.0 / max_items
  can_accept_new = buffer.items.last().progress > min_spacing

Debit :
  items_per_minute = speed * 60.0 / (length / max_items)
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `move_items` | `mge.factory.conveyor.v1.fn.move_items` | 2040 | Conveyor, ConveyorBuffer, ConveyorItem | ConveyorItem (progress) | none | O(c*b) | Avance chaque item selon la vitesse du convoyeur. Respecte l'espacement |
| `transfer_to_machine` | `mge.factory.conveyor.v1.fn.transfer_to_machine` | 2041 | ConveyorItem, Conveyor, InputSlot | InputSlot, ConveyorBuffer | ItemTransferred, BufferFull | O(t) | Transfere les items arrives (progress >= 1.0) vers la machine cible |
| `transfer_from_machine` | `mge.factory.conveyor.v1.fn.transfer_from_machine` | 2042 | OutputSlot, Conveyor, ConveyorBuffer | OutputSlot, ConveyorBuffer | ItemTransferred | O(t) | Recupere les items des output slots et les place sur le convoyeur |
| `tick_buffer` | `mge.factory.conveyor.v1.fn.tick_buffer` | 2043 | ConveyorBuffer | ConveyorBuffer | BufferEmpty | O(c) | Met a jour l'etat du buffer. Emet BufferEmpty quand le convoyeur se vide |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ItemTransferred` | `mge.factory.conveyor.v1.event.item_transferred` | `item_id: u32, quantity: u32, from_entity: EntityId, to_entity: EntityId` | `transfer_to_machine`, `transfer_from_machine` | ui (flow indicator), analytics |
| `BufferFull` | `mge.factory.conveyor.v1.event.buffer_full` | `conveyor_entity: EntityId, blocked_item_id: u32` | `transfer_to_machine` | logistics (reroute), ui (warning) |
| `BufferEmpty` | `mge.factory.conveyor.v1.event.buffer_empty` | `conveyor_entity: EntityId` | `tick_buffer` | logistics (supply needed), ui |

---

## 7. Invariants

- `ConveyorItem.progress` est borne entre 0.0 et 1.0 (inclus).
- Le nombre d'items dans un `ConveyorBuffer` ne depasse jamais `max_items`.
- Les items sont ordonnes par `progress` decroissant dans le buffer (le plus avance en premier).
- L'espacement minimum entre deux items est garanti (`1.0 / max_items`).
- Un convoyeur sans `target_entity` accumule les items a progress = 1.0 jusqu'au buffer full.
- `Conveyor.speed` est toujours > 0 (minimum 0.1).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `slow_speed` | `f32` | 1.0 | [0.5, 5.0] | Vitesse du tier Slow (units/sec) |
| `normal_speed` | `f32` | 2.0 | [1.0, 10.0] | Vitesse du tier Normal |
| `fast_speed` | `f32` | 3.0 | [2.0, 15.0] | Vitesse du tier Fast |
| `express_speed` | `f32` | 4.0 | [3.0, 20.0] | Vitesse du tier Express |
| `default_buffer_size` | `u32` | 8 | [1, 32] | Nombre max d'items par convoyeur |
| `transfer_cooldown_ticks` | `u32` | 1 | [1, 10] | Ticks entre deux transferts consecutifs |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Deplace les items sur les convoyeurs | Ne gere pas les machines (→ machine) |
| Transfere les items vers/depuis les machines | Ne definit pas les recettes (→ recipe) |
| Gere les buffers et l'espacement | Ne gere pas le routage global (→ logistics) |
| Detecte les blocages (buffer full) | Ne gere pas le rendu des convoyeurs |
| Supporte plusieurs tiers de vitesse | Ne gere pas le cout de construction |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Conveyor, ConveyorSegment, ConveyorBuffer, ConveyorItem, InputSlot, OutputSlot |
| Ecrit | ConveyorItem (progress), ConveyorBuffer, InputSlot, OutputSlot |
| Emet | ItemTransferred, BufferFull, BufferEmpty |
| Ne touche jamais | Machine, MachineState, Recipe, LogisticsNode, LogisticsRoute, StorageContainer |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-factory-conveyor/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.factory.conveyor.v1, trait Plugin impl
    ├── components.rs     # Conveyor, ConveyorSegment, ConveyorBuffer, ConveyorItem
    ├── systems.rs        # move_items, transfer_to_machine, transfer_from_machine, tick_buffer
    └── events.rs         # ItemTransferred, BufferFull, BufferEmpty
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No allocation hot path | Obligatoire (buffers pre-alloues) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 3 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (ConveyorTier, ConveyorDirection, TransferResult)
- [ ] Formules de deplacement et espacement documentees
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : item movement, spacing, transfer to/from machine, buffer full/empty
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.factory.conveyor.v1","k":"p","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.conveyor.v1.component.conveyor","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.conveyor.v1.component.conveyor_segment","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.conveyor.v1.component.conveyor_buffer","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.conveyor.v1.component.conveyor_item","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.conveyor.v1.fn.move_items","k":"s","d":"factory","r":["Conveyor","ConveyorBuffer","ConveyorItem"],"w":["ConveyorItem"],"e":[],"p":2040,"c":"O(c*b)"},
  {"i":"mge.factory.conveyor.v1.fn.transfer_to_machine","k":"s","d":"factory","r":["ConveyorItem","Conveyor","InputSlot"],"w":["InputSlot","ConveyorBuffer"],"e":["ItemTransferred","BufferFull"],"p":2041,"c":"O(t)"},
  {"i":"mge.factory.conveyor.v1.fn.transfer_from_machine","k":"s","d":"factory","r":["OutputSlot","Conveyor","ConveyorBuffer"],"w":["OutputSlot","ConveyorBuffer"],"e":["ItemTransferred"],"p":2042,"c":"O(t)"},
  {"i":"mge.factory.conveyor.v1.fn.tick_buffer","k":"s","d":"factory","r":["ConveyorBuffer"],"w":["ConveyorBuffer"],"e":["BufferEmpty"],"p":2043,"c":"O(c)"},
  {"i":"mge.factory.conveyor.v1.event.item_transferred","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.conveyor.v1.event.buffer_full","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.conveyor.v1.event.buffer_empty","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let belt = world.spawn();
world.insert(belt, Conveyor {
    tier: ConveyorTier::Normal,
    speed: 2.0,
    length: 10.0,
    source_entity: Some(smelter_entity),
    target_entity: Some(assembler_entity),
});
world.insert(belt, ConveyorBuffer {
    items: Vec::new(),
    max_items: 8,
    total_progress: 0.0,
});
world.insert(belt, ConveyorSegment {
    direction: ConveyorDirection::Forward,
    start_pos: Vec2::new(10.0, 0.0),
    end_pos: Vec2::new(20.0, 0.0),
    segment_index: 0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Factory - Index](_index.md) | Vue d'ensemble du pack |
| [mge-factory-machine](mge-factory-machine.md) | Plugin machines (source/destination des items) |
| [mge-factory-logistics](mge-factory-logistics.md) | Plugin logistique (routage global) |
