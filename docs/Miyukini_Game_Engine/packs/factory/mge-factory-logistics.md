# mge-factory-logistics

> @id mge.factory.logistics.v1  
> @role plugin  
> @domain factory  
> @do manage_routing_priorities_storage_distribution  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-factory-logistics` |
| @id MSCM | `mge.factory.logistics.v1` |
| Domaine | factory |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-factory-machine`, `mge-factory-conveyor` |
| Hot path | Non (recomputation de routes periodique) |
| Headless safe | Oui |
| Complexite globale | O(n * e) par recomputation, n = noeuds, e = aretes |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `RouteAlgorithm` | `ShortestPath, LeastLoaded, RoundRobin, Priority` | Algorithme de routage. Priority utilise les poids assignes |
| `StorageMode` | `FIFO, LIFO, Priority, Balanced` | Mode de stockage/destockage du conteneur |
| `NodeType` | `Source, Sink, Junction, Storage` | Type de noeud logistique. Source = machine output, Sink = machine input |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `LogisticsNode` | `mge.factory.logistics.v1.component.logistics_node` | `node_type: NodeType, entity_ref: EntityId, throughput_limit: f32, active: bool` | Noeud dans le graphe logistique. Lie a une entite (machine, storage) |
| `LogisticsRoute` | `mge.factory.logistics.v1.component.logistics_route` | `from_node: EntityId, to_node: EntityId, item_id: u32, algorithm: RouteAlgorithm, cost: f32, hops: Vec<EntityId>` | Route calculee entre deux noeuds. hops = chemin intermediaire |
| `StorageContainer` | `mge.factory.logistics.v1.component.storage_container` | `slots: Vec<StorageSlot>, max_slots: u32, mode: StorageMode, total_items: u32, max_capacity: u32` | Conteneur de stockage. StorageSlot = {item_id, quantity} |
| `RoutePriority` | `mge.factory.logistics.v1.component.route_priority` | `item_id: u32, priority: f32, min_reserve: u32, max_throughput: f32` | Priorite de routage par type d'item. min_reserve = stock minimum garanti |

---

## 4. Formules

```
Cout de route :
  cost = sum(hop.distance for hop in hops) + congestion_penalty
  congestion_penalty = sum(hop.buffer_fill_ratio * congestion_weight for hop in hops)

Distribution (Priority) :
  for each route sorted by priority DESC:
    available = source.quantity - source.min_reserve
    demand = target.max_capacity - target.total_items
    transfer = min(available, demand, max_throughput)

Distribution (Balanced) :
  total_demand = sum(target.demand for each target)
  for each target:
    share = target.demand / total_demand
    transfer = available * share

Equilibrage stockage :
  if container.total_items > max_capacity * 0.9:
    emit StorageFull
  if container.total_items == 0:
    emit StorageEmpty
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `compute_routes` | `mge.factory.logistics.v1.fn.compute_routes` | 2060 | LogisticsNode, Conveyor, RoutePriority | LogisticsRoute | RouteComputed | O(n*e) | Recalcule les routes optimales entre noeuds. Dijkstra simplifie |
| `assign_priorities` | `mge.factory.logistics.v1.fn.assign_priorities` | 2061 | RoutePriority, StorageContainer, LogisticsRoute | LogisticsRoute (cost) | none | O(r) | Ajuste les couts de route en fonction des priorites et niveaux de stock |
| `distribute_items` | `mge.factory.logistics.v1.fn.distribute_items` | 2062 | LogisticsRoute, StorageContainer, OutputSlot, InputSlot, RoutePriority | StorageContainer, InputSlot | ItemDistributed | O(r) | Distribue les items selon les routes calculees et les priorites |
| `balance_storage` | `mge.factory.logistics.v1.fn.balance_storage` | 2063 | StorageContainer | StorageContainer | StorageFull, StorageEmpty | O(s) | Equilibre les conteneurs de stockage. Emet alertes de capacite |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `RouteComputed` | `mge.factory.logistics.v1.event.route_computed` | `from_node: EntityId, to_node: EntityId, item_id: u32, cost: f32, hop_count: u32` | `compute_routes` | ui (route overlay), debug |
| `ItemDistributed` | `mge.factory.logistics.v1.event.item_distributed` | `item_id: u32, quantity: u32, from_node: EntityId, to_node: EntityId` | `distribute_items` | ui (flow indicator), analytics |
| `StorageFull` | `mge.factory.logistics.v1.event.storage_full` | `container_entity: EntityId, item_id: u32, total_items: u32` | `balance_storage` | ui (warning), production (throttle) |
| `StorageEmpty` | `mge.factory.logistics.v1.event.storage_empty` | `container_entity: EntityId` | `balance_storage` | production (pause), logistics (reroute supply) |

---

## 7. Invariants

- Chaque `LogisticsRoute` relie deux `LogisticsNode` existants.
- `StorageContainer.total_items` ne depasse jamais `max_capacity`.
- `RoutePriority.priority` est borne entre 0.0 et 1.0.
- Les routes sont recalculees uniquement quand la topologie change (ajout/suppression de noeud).
- Un noeud `Source` ne peut pas etre destination d'une route.
- Un noeud `Sink` ne peut pas etre source d'une route.
- `min_reserve` garantit qu'un stockage ne descend jamais en dessous de cette quantite via la distribution.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `default_route_algorithm` | `RouteAlgorithm` | ShortestPath | {ShortestPath, LeastLoaded, RoundRobin, Priority} | Algorithme par defaut |
| `route_recompute_interval` | `u32` | 60 | [10, 600] | Ticks entre recomputations de routes |
| `congestion_weight` | `f32` | 0.5 | [0.0, 2.0] | Poids de la congestion dans le cout de route |
| `default_storage_capacity` | `u32` | 100 | [10, 10000] | Capacite par defaut d'un conteneur |
| `storage_full_threshold` | `f32` | 0.9 | [0.5, 1.0] | Seuil de remplissage pour StorageFull |
| `default_min_reserve` | `u32` | 0 | [0, 100] | Reserve minimum par defaut |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Calcule les routes entre noeuds logistiques | Ne deplace pas les items physiquement (→ conveyor) |
| Distribue les items selon les priorites | Ne gere pas les machines (→ machine) |
| Gere les conteneurs de stockage | Ne definit pas les recettes (→ recipe) |
| Detecte les surcharges et penuries | Ne gere pas le rendu du reseau |
| Supporte plusieurs algorithmes de routage | Ne gere pas le cout financier (→ tycoon) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | LogisticsNode, LogisticsRoute, StorageContainer, RoutePriority, Conveyor, OutputSlot, InputSlot |
| Ecrit | LogisticsRoute, StorageContainer, InputSlot |
| Emet | RouteComputed, ItemDistributed, StorageFull, StorageEmpty |
| Ne touche jamais | Machine, MachineState, ProcessingTimer, Recipe, ConveyorItem, ConveyorBuffer |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-factory-logistics/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.factory.logistics.v1, trait Plugin impl
    ├── components.rs     # LogisticsNode, LogisticsRoute, StorageContainer, RoutePriority
    ├── systems.rs        # compute_routes, assign_priorities, distribute_items, balance_storage
    └── events.rs         # RouteComputed, ItemDistributed, StorageFull, StorageEmpty
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (RouteAlgorithm, StorageMode, NodeType)
- [ ] Formules de routage et distribution documentees
- [ ] Parametres GCL exposes
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : route computation, priority distribution, storage balance, full/empty alerts
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.factory.logistics.v1","k":"p","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.logistics.v1.component.logistics_node","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.logistics.v1.component.logistics_route","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.logistics.v1.component.storage_container","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.logistics.v1.component.route_priority","k":"d","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.logistics.v1.fn.compute_routes","k":"s","d":"factory","r":["LogisticsNode","Conveyor","RoutePriority"],"w":["LogisticsRoute"],"e":["RouteComputed"],"p":2060,"c":"O(n*e)"},
  {"i":"mge.factory.logistics.v1.fn.assign_priorities","k":"s","d":"factory","r":["RoutePriority","StorageContainer","LogisticsRoute"],"w":["LogisticsRoute"],"e":[],"p":2061,"c":"O(r)"},
  {"i":"mge.factory.logistics.v1.fn.distribute_items","k":"s","d":"factory","r":["LogisticsRoute","StorageContainer","OutputSlot","InputSlot","RoutePriority"],"w":["StorageContainer","InputSlot"],"e":["ItemDistributed"],"p":2062,"c":"O(r)"},
  {"i":"mge.factory.logistics.v1.fn.balance_storage","k":"s","d":"factory","r":["StorageContainer"],"w":["StorageContainer"],"e":["StorageFull","StorageEmpty"],"p":2063,"c":"O(s)"},
  {"i":"mge.factory.logistics.v1.event.route_computed","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.logistics.v1.event.item_distributed","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.logistics.v1.event.storage_full","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.factory.logistics.v1.event.storage_empty","k":"e","d":"factory","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let storage = world.spawn();
world.insert(storage, LogisticsNode {
    node_type: NodeType::Storage,
    entity_ref: storage,
    throughput_limit: 10.0,
    active: true,
});
world.insert(storage, StorageContainer {
    slots: vec![
        StorageSlot { item_id: 10, quantity: 50 },
        StorageSlot { item_id: 20, quantity: 30 },
    ],
    max_slots: 8,
    mode: StorageMode::FIFO,
    total_items: 80,
    max_capacity: 200,
});

let priority = world.spawn();
world.insert(priority, RoutePriority {
    item_id: 10, // iron_bar
    priority: 0.8,
    min_reserve: 10,
    max_throughput: 5.0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack Factory - Index](_index.md) | Vue d'ensemble du pack |
| [mge-factory-machine](mge-factory-machine.md) | Plugin machines (noeuds source/sink) |
| [mge-factory-conveyor](mge-factory-conveyor.md) | Plugin convoyeurs (transport physique) |
