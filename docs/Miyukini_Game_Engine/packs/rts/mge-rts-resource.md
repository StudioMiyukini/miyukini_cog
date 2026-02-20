# mge-rts-resource

> @id mge.rts.resource.v1  
> @role plugin  
> @domain rts  
> @do manage_resource_harvesting_deposits_storage  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rts-resource` |
| @id MSCM | `mge.rts.resource.v1` |
| Domaine | rts |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (recolte continue chaque tick pour les recolteurs actifs) |
| Headless safe | Oui |
| Complexite globale | O(h) ou h=recolteurs actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ResourceKind` | `Gold, Wood, Stone, Food, Custom(u16)` | Type de ressource recoltable et stockable |
| `HarvestState` | `Idle, Harvesting, Carrying, Depositing` | Etat du cycle recolte-depot |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `ResourceNode` | `mge.rts.resource.v1.component.resource_node` | `kind: ResourceKind, amount: u32, max_amount: u32, regen_rate: f32` | Source de ressources dans le monde. regen_rate en unites/sec (0 = non-renouvelable) |
| `ResourceCarrier` | `mge.rts.resource.v1.component.resource_carrier` | `carrying_kind: Option<ResourceKind>, carrying_amount: u32, capacity: u32, state: HarvestState` | Recolteur qui transporte des ressources entre node et depot |
| `ResourceDepot` | `mge.rts.resource.v1.component.resource_depot` | `stored: HashMap<ResourceKind, u32>, capacity: u32, accepted_kinds: Vec<ResourceKind>` | Depot de stockage. capacity est le total global |
| `ResourceCost` | `mge.rts.resource.v1.component.resource_cost` | `amounts: HashMap<ResourceKind, u32>` | Cout en ressources pour une action (production, construction, recherche) |

---

## 4. Formules

```
harvest_amount  = min(harvest_rate * dt, node.amount, carrier.capacity - carrier.carrying_amount)
node.amount     = node.amount - harvest_amount
carrier.carrying_amount = carrier.carrying_amount + harvest_amount

regen_amount    = min(regen_rate * dt, max_amount - amount)
node.amount     = node.amount + regen_amount
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `harvest_resource` | `mge.rts.resource.v1.fn.harvest_resource` | Logic (1120) | ResourceCarrier, ResourceNode | ResourceCarrier, ResourceNode | ResourceHarvested | O(h) | Extrait des ressources du node vers le carrier. Respecte capacity |
| `deposit_resource` | `mge.rts.resource.v1.fn.deposit_resource` | Logic (1121) | ResourceCarrier, ResourceDepot | ResourceCarrier, ResourceDepot | ResourceDeposited | O(h) | Depose les ressources transportees dans le depot le plus proche |
| `update_resource_node` | `mge.rts.resource.v1.fn.update_resource_node` | Logic (1122) | ResourceNode | ResourceNode | none | O(n) | Regenere les nodes renouvelables selon regen_rate |
| `check_resource_depletion` | `mge.rts.resource.v1.fn.check_resource_depletion` | Logic (1123) | ResourceNode | ResourceNode | ResourceDepleted | O(n) | Detecte les nodes vides et emet l'evenement de depletion |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `ResourceHarvested` | `mge.rts.resource.v1.event.resource_harvested` | `carrier: EntityId, node: EntityId, kind: ResourceKind, amount: u32` | `harvest_resource` | ui, audio |
| `ResourceDeposited` | `mge.rts.resource.v1.event.resource_deposited` | `carrier: EntityId, depot: EntityId, kind: ResourceKind, amount: u32` | `deposit_resource` | ui, economy |
| `ResourceDepleted` | `mge.rts.resource.v1.event.resource_depleted` | `node: EntityId, kind: ResourceKind` | `check_resource_depletion` | ai, ui |
| `InsufficientResources` | `mge.rts.resource.v1.event.insufficient_resources` | `requester: EntityId, missing: ResourceCost` | externe (production, building) | ui |

---

## 7. Invariants

- `ResourceNode.amount` est toujours dans [0, max_amount].
- `ResourceCarrier.carrying_amount` ne depasse jamais `capacity`.
- `ResourceDepot.stored` total ne depasse jamais `capacity`.
- Un depot n'accepte que les `accepted_kinds` configures.
- `ResourceDepleted` n'est emis qu'une seule fois par node (quand amount atteint 0).
- `regen_rate = 0` signifie une ressource non-renouvelable.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `default_harvest_rate` | `f32` | 5.0 | [1.0, 50.0] | Unites recoltees par seconde par defaut |
| `default_carrier_capacity` | `u32` | 20 | [5, 100] | Capacite par defaut d'un carrier |
| `default_depot_capacity` | `u32` | 1000 | [100, 10000] | Capacite par defaut d'un depot |
| `regen_tick_interval` | `f32` | 1.0 | [0.1, 10.0] | Intervalle de regeneration en secondes |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere la recolte, le transport et le depot | Ne deplace pas les carriers (→ spatial, unit-ai) |
| Regenere les nodes renouvelables | Ne gere pas la production (→ production) |
| Detecte l'epuisement des nodes | Ne gere pas le commerce (→ external) |
| Stocke les ressources dans les depots | Ne gere pas le budget/tresor (→ external) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | ResourceCarrier, ResourceNode, ResourceDepot |
| Ecrit | ResourceCarrier, ResourceNode, ResourceDepot |
| Emet | ResourceHarvested, ResourceDeposited, ResourceDepleted |
| Ne touche jamais | Selection, ProductionQueue, Building, OrderQueue, FogGrid, TechNode |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rts-resource/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rts.resource.v1, trait Plugin impl
    ├── components.rs     # ResourceNode, ResourceCarrier, ResourceDepot, ResourceCost
    ├── systems.rs        # harvest_resource, deposit_resource, update_resource_node, check_resource_depletion
    └── events.rs         # ResourceHarvested, ResourceDeposited, ResourceDepleted, InsufficientResources
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (harvest_resource) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (ResourceKind, HarvestState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : harvest, deposit, regen, depletion
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rts.resource.v1","k":"p","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.resource.v1.component.resource_node","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.resource.v1.component.resource_carrier","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.resource.v1.component.resource_depot","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.resource.v1.component.resource_cost","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.resource.v1.fn.harvest_resource","k":"s","d":"rts","r":["ResourceCarrier","ResourceNode"],"w":["ResourceCarrier","ResourceNode"],"e":["ResourceHarvested"],"p":1120,"c":"O(h)"},
  {"i":"mge.rts.resource.v1.fn.deposit_resource","k":"s","d":"rts","r":["ResourceCarrier","ResourceDepot"],"w":["ResourceCarrier","ResourceDepot"],"e":["ResourceDeposited"],"p":1121,"c":"O(h)"},
  {"i":"mge.rts.resource.v1.fn.update_resource_node","k":"s","d":"rts","r":["ResourceNode"],"w":["ResourceNode"],"e":[],"p":1122,"c":"O(n)"},
  {"i":"mge.rts.resource.v1.fn.check_resource_depletion","k":"s","d":"rts","r":["ResourceNode"],"w":["ResourceNode"],"e":["ResourceDepleted"],"p":1123,"c":"O(n)"},
  {"i":"mge.rts.resource.v1.event.resource_harvested","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.resource.v1.event.resource_deposited","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.resource.v1.event.resource_depleted","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.resource.v1.event.insufficient_resources","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let gold_mine = world.spawn();
world.insert(gold_mine, ResourceNode {
    kind: ResourceKind::Gold,
    amount: 5000,
    max_amount: 5000,
    regen_rate: 0.0,
});

let worker = world.spawn();
world.insert(worker, ResourceCarrier {
    carrying_kind: None,
    carrying_amount: 0,
    capacity: 20,
    state: HarvestState::Idle,
});

let town_center = world.spawn();
world.insert(town_center, ResourceDepot {
    stored: HashMap::from([(ResourceKind::Gold, 100), (ResourceKind::Food, 200)]),
    capacity: 5000,
    accepted_kinds: vec![ResourceKind::Gold, ResourceKind::Wood, ResourceKind::Stone, ResourceKind::Food],
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack RTS - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
