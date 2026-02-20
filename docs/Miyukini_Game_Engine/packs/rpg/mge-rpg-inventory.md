# mge-rpg-inventory

> @id mge.rpg.inventory.v1  
> @role plugin  
> @domain rpg  
> @do manage_inventory_slots_equipment_stacking_loot  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rpg-inventory` |
| @id MSCM | `mge.rpg.inventory.v1` |
| Domaine | rpg |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Non (evenementiel) |
| Headless safe | Oui |
| Complexite globale | O(1) par operation, O(s) pour recherche slot libre |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ItemType` | `Weapon, Armor, Accessory, Consumable, Material, QuestItem, Misc` | Classification objet |
| `EquipSlot` | `Weapon, Armor, Accessory1, Accessory2` | Emplacement d'equipement |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Inventory` | `mge.rpg.inventory.v1.component.inventory` | `slots: Vec<Option<ItemStack>>, capacity: u32` | Inventaire a taille fixe. slots.len() == capacity |
| `ItemStack` | `mge.rpg.inventory.v1.component.item_stack` | `item_id: u32, quantity: u32, max_stack: u32` | Pile d'objets identiques. quantity dans [1, max_stack] |
| `Equipment` | `mge.rpg.inventory.v1.component.equipment` | `weapon: Option<u32>, armor: Option<u32>, accessory_1: Option<u32>, accessory_2: Option<u32>` | Slots d'equipement. u32 = item_id |
| `ItemDef` | `mge.rpg.inventory.v1.component.item_def` | `id: u32, name_key: u32, weight: f32, item_type: ItemType, stackable: bool, max_stack: u32` | Definition statique. Non modifiable a runtime |
| `Container` | `mge.rpg.inventory.v1.component.container` | `inventory: Inventory, locked: bool` | Conteneur interactable. locked = necesssite action externe |
| `LootDrop` | `mge.rpg.inventory.v1.component.loot_drop` | `item_id: u32, quantity: u32, drop_chance: f32` | Entree table de loot. drop_chance dans [0.0, 1.0] |
| `LootTable` | `mge.rpg.inventory.v1.component.loot_table` | `drops: Vec<LootDrop>, guaranteed_drops: Vec<LootDrop>` | Table complete. guaranteed_drops toujours produit |

---

## 4. Regles de stacking

```
1. Si item stackable ET slot existant avec meme item_id ET slot.quantity < max_stack :
     → ajouter au stack existant (min(quantity_restante, max_stack - slot.quantity))
2. Si reste a placer > 0 ET slot vide disponible :
     → creer nouveau stack
3. Si aucun slot libre → emettre InventoryFull, annuler le reste
```

Si `weight_limit_enabled` :
```
total_weight = sum(slots[i].quantity * ItemDef[slots[i].item_id].weight)
Si total_weight + new_weight > weight_limit → InventoryFull
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_item_pickup` | `mge.rpg.inventory.v1.fn.process_item_pickup` | Logic (400) | PickupRequest (event), Inventory, ItemDef | Inventory | ItemPickedUp, InventoryFull | O(s) | Cherche slot stackable ou libre. Ajoute item |
| `process_equip` | `mge.rpg.inventory.v1.fn.process_equip` | Logic (401) | EquipRequest (event), Inventory, Equipment, ItemDef | Inventory, Equipment | ItemEquipped | O(1) | Retire de l'inventaire, place dans Equipment |
| `process_unequip` | `mge.rpg.inventory.v1.fn.process_unequip` | Logic (402) | UnequipRequest (event), Inventory, Equipment | Inventory, Equipment | ItemUnequipped | O(s) | Retire de Equipment, cherche slot libre |
| `process_item_transfer` | `mge.rpg.inventory.v1.fn.process_item_transfer` | Logic (403) | TransferRequest (event), Inventory (x2) | Inventory (x2) | ItemTransferred | O(s) | Transfere entre deux Inventory |
| `resolve_loot` | `mge.rpg.inventory.v1.fn.resolve_loot` | Logic (404) | LootRequest (event), LootTable | none | PickupRequest | O(d) | Evalue chaque LootDrop via mge-rng |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `PickupRequest` | `mge.rpg.inventory.v1.event.pickup_request` | `entity: EntityId, item_id: u32, quantity: u32` | Externe, resolve_loot | process_item_pickup |
| `ItemPickedUp` | `mge.rpg.inventory.v1.event.item_picked_up` | `entity: EntityId, item_id: u32, quantity: u32` | process_item_pickup | quest, ui |
| `EquipRequest` | `mge.rpg.inventory.v1.event.equip_request` | `entity: EntityId, item_id: u32, slot: EquipSlot` | Externe (ui) | process_equip |
| `ItemEquipped` | `mge.rpg.inventory.v1.event.item_equipped` | `entity: EntityId, item_id: u32, slot: EquipSlot` | process_equip | stats (recalcul), ui |
| `UnequipRequest` | `mge.rpg.inventory.v1.event.unequip_request` | `entity: EntityId, slot: EquipSlot` | Externe (ui) | process_unequip |
| `ItemUnequipped` | `mge.rpg.inventory.v1.event.item_unequipped` | `entity: EntityId, item_id: u32, slot: EquipSlot` | process_unequip | stats, ui |
| `ItemDropped` | `mge.rpg.inventory.v1.event.item_dropped` | `entity: EntityId, item_id: u32, quantity: u32` | Externe | spatial (spawn sol), ui |
| `TransferRequest` | `mge.rpg.inventory.v1.event.transfer_request` | `source: EntityId, target: EntityId, item_id: u32, quantity: u32` | Externe | process_item_transfer |
| `ItemTransferred` | `mge.rpg.inventory.v1.event.item_transferred` | `source: EntityId, target: EntityId, item_id: u32, quantity: u32` | process_item_transfer | ui |
| `InventoryFull` | `mge.rpg.inventory.v1.event.inventory_full` | `entity: EntityId, item_id: u32, quantity_remaining: u32` | process_item_pickup | ui |
| `LootRequest` | `mge.rpg.inventory.v1.event.loot_request` | `source: EntityId, looted_by: EntityId` | Externe (DeathEvent handler) | resolve_loot |

---

## 7. Invariants

- Un ItemStack a toujours quantity >= 1. Un stack vide est supprime (slot = None).
- Equipment ne contient que des items dont ItemType correspond a EquipSlot.
- process_equip echoue silencieusement si ItemType incompatible.
- Les ItemDef ne sont jamais modifiees a runtime.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_inventory_capacity` | `u32` | 20 | [1, 999] | Nombre de slots |
| `weight_limit_enabled` | `bool` | false | {true, false} | Active systeme de poids |
| `weight_limit` | `f32` | 100.0 | [1.0, 99999.0] | Poids max si active |
| `auto_stack` | `bool` | true | {true, false} | false = chaque pickup cree un nouveau stack |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Stocke items dans des slots | Ne definit pas les effets des items (→ stats, combat) |
| Gere l'equipement (slots fixes) | Ne modifie pas les attributs (→ stats sur ItemEquipped) |
| Resout les loot tables | Ne decide pas quand loot (le code appelant emet LootRequest) |
| Transfere entre inventaires | Ne gere pas le commerce (prix, monnaie → code jeu) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Inventory, Equipment, ItemDef, LootTable |
| Ecrit | Inventory, Equipment |
| Emet | ItemPickedUp, ItemEquipped, ItemUnequipped, ItemDropped, ItemTransferred, InventoryFull, PickupRequest |
| Ne touche jamais | Health, Attributes, QuestLog, DialogueState, AIBehavior |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rpg-inventory/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.rpg.inventory.v1
    ├── components.rs     # Inventory, ItemStack, Equipment, ItemDef, Container, LootDrop, LootTable
    ├── systems.rs        # process_item_pickup, process_equip, process_unequip, process_item_transfer, resolve_loot
    └── events.rs         # PickupRequest, ItemPickedUp, EquipRequest, ItemEquipped, ...
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 (events.rs peut necessiter decoupe si > 300) |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 7 composants dans `components.rs` avec @id et @fields
- [ ] 5 systemes dans `systems.rs` avec annotations completes
- [ ] 11 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (ItemType, EquipSlot)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : pickup, stack, equip, transfer, loot, weight limit, full inventory
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rpg.inventory.v1","k":"p","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.component.inventory","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.component.item_stack","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.component.equipment","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.component.item_def","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.component.container","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.component.loot_drop","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.component.loot_table","k":"d","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.fn.process_item_pickup","k":"s","d":"rpg","r":["Inventory","ItemDef"],"w":["Inventory"],"e":["ItemPickedUp","InventoryFull"],"p":400,"c":"O(s)"},
  {"i":"mge.rpg.inventory.v1.fn.process_equip","k":"s","d":"rpg","r":["Inventory","Equipment","ItemDef"],"w":["Inventory","Equipment"],"e":["ItemEquipped"],"p":401,"c":"O(1)"},
  {"i":"mge.rpg.inventory.v1.fn.process_unequip","k":"s","d":"rpg","r":["Inventory","Equipment"],"w":["Inventory","Equipment"],"e":["ItemUnequipped"],"p":402,"c":"O(s)"},
  {"i":"mge.rpg.inventory.v1.fn.process_item_transfer","k":"s","d":"rpg","r":["Inventory"],"w":["Inventory"],"e":["ItemTransferred"],"p":403,"c":"O(s)"},
  {"i":"mge.rpg.inventory.v1.fn.resolve_loot","k":"s","d":"rpg","r":["LootTable"],"w":[],"e":["PickupRequest"],"p":404,"c":"O(d)"},
  {"i":"mge.rpg.inventory.v1.event.pickup_request","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.event.item_picked_up","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.event.item_equipped","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rpg.inventory.v1.event.inventory_full","k":"e","d":"rpg","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let player = world.spawn();
world.insert(player, Inventory { slots: vec![None; 20], capacity: 20 });
world.insert(player, Equipment { weapon: None, armor: None, accessory_1: None, accessory_2: None });
events.emit(PickupRequest { entity: player, item_id: 42, quantity: 3 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack RPG - Index](_index.md) | Vue d'ensemble du pack |
