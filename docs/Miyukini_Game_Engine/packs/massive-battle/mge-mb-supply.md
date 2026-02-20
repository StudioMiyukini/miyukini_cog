# mge-mb-supply

> @id mge.mb.supply.v1  
> @role plugin  
> @domain massive-battle  
> @do manage_logistics_ammunition_resupply_depots  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-mb-supply` |
| @id MSCM | `mge.mb.supply.v1` |
| Domaine | massive-battle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-mb-unit` |
| Hot path | Non (tick supply toutes les N ticks) |
| Headless safe | Oui |
| Complexite globale | O(n + d) ou n=consommateurs, d=depots |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `SupplyType` | `Ammunition, Food, Medical` | Type de ressource logistique |
| `DepotState` | `Active, Depleted, Destroyed` | Etat d'un depot |
| `SupplyLineState` | `Connected, Disrupted, Cut` | Etat d'une ligne d'approvisionnement |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `SupplyStock` | `mge.mb.supply.v1.component.supply_stock` | `ammunition: f32, food: f32, medical: f32` | Stock de ressources portes par un squad |
| `SupplyLine` | `mge.mb.supply.v1.component.supply_line` | `source: EntityId, target: EntityId, state: SupplyLineState, throughput: f32` | Lien logistique entre depot et consommateur |
| `Depot` | `mge.mb.supply.v1.component.depot` | `stocks: [f32; 3], max_capacity: f32, state: DepotState` | Point de stockage central. Index: 0=ammo, 1=food, 2=medical |
| `SupplyConsumer` | `mge.mb.supply.v1.component.supply_consumer` | `consumption_rate: [f32; 3], connected_depot: Option<EntityId>` | Taux de consommation par type et depot connecte |

---

## 4. Formules de derivation

```
consumption_per_tick = consumption_rate[type] * squad_member_count
supply_remaining     = supply_stock[type] - consumption_per_tick
resupply_amount      = min(throughput, depot_stock[type], max_carry - current_stock)

Penalite sans supply:
  ammunition = 0 → attack_power * 0.2 (armes de melee seulement)
  food = 0       → morale -5.0/tick, speed * 0.7
  medical = 0    → pas de regeneration HP
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `consume_supplies` | `mge.mb.supply.v1.fn.consume_supplies` | Logic (940) | SupplyStock, SupplyConsumer | SupplyStock | SupplyDepleted | O(n) | Decremente les stocks selon consumption_rate. Emet si un type atteint 0 |
| `update_supply_lines` | `mge.mb.supply.v1.fn.update_supply_lines` | Logic (941) | SupplyLine, Depot | SupplyLine | SupplyLineCut | O(l) | Verifie l'etat des lignes. Marque Cut si depot detruit ou route bloquee |
| `check_depot_status` | `mge.mb.supply.v1.fn.check_depot_status` | Logic (942) | Depot | Depot | DepotDestroyed | O(d) | Met a jour l'etat des depots. Depleted si tous stocks a 0 |
| `apply_supply_penalty` | `mge.mb.supply.v1.fn.apply_supply_penalty` | Logic (943) | SupplyStock | SupplyStock | ResupplyReceived | O(n) | Applique les malus aux squads sans supply. Reapprovisionne via lignes actives |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique | Description |
|-----------|----------|--------|----------|----------------------|-------------|
| `SupplyDepleted` | `mge.mb.supply.v1.event.supply_depleted` | `squad: EntityId, supply_type: SupplyType` | `consume_supplies` | morale, ai | Un type de supply atteint 0 pour un squad |
| `DepotDestroyed` | `mge.mb.supply.v1.event.depot_destroyed` | `depot: EntityId` | `check_depot_status` | ai, tactics | Depot detruit par l'ennemi |
| `SupplyLineCut` | `mge.mb.supply.v1.event.supply_line_cut` | `line: EntityId, source: EntityId, target: EntityId` | `update_supply_lines` | ai | Ligne d'approvisionnement coupee |
| `ResupplyReceived` | `mge.mb.supply.v1.event.resupply_received` | `squad: EntityId, amounts: [f32; 3]` | `apply_supply_penalty` | ui | Squad reapprovisionne depuis un depot |

---

## 7. Invariants

- `SupplyStock` n'a jamais de valeur negative apres `consume_supplies`.
- Un `Depot` avec `state = Destroyed` n'est jamais source d'une `SupplyLine` active.
- `SupplyLine.state = Connected` implique que le depot source a `state = Active`.
- `SupplyConsumer.connected_depot = None` si aucune `SupplyLine` active ne le relie.
- Le throughput effectif ne depasse jamais `SupplyLine.throughput`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `ammo_consumption_rate` | `f32` | 1.0 | [0.1, 10.0] | Consommation ammo par soldat par tick de combat |
| `food_consumption_rate` | `f32` | 0.1 | [0.01, 1.0] | Consommation food par soldat par tick |
| `supply_tick_interval` | `u32` | 10 | [1, 60] | Nombre de ticks entre chaque update supply |
| `depot_max_capacity` | `f32` | 10000.0 | [100.0, 100000.0] | Capacite max d'un depot |
| `line_throughput_default` | `f32` | 50.0 | [10.0, 500.0] | Debit par defaut d'une ligne supply |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les stocks de supply par squad | Ne calcule pas les degats (→ rpg-combat) |
| Decremente les consommations | Ne gere pas le moral (→ morale) |
| Relie depots et consommateurs | Ne deplace pas les convois (v2, → spatial) |
| Applique les penalites de manque | Ne gere pas le commerce (→ gs-trade) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | SupplyStock, SupplyLine, Depot, SupplyConsumer |
| Ecrit | SupplyStock, SupplyLine, Depot |
| Emet | SupplyDepleted, DepotDestroyed, SupplyLineCut, ResupplyReceived |
| Ne touche jamais | Formation, Morale, TacticalStance, WallSection, SiegeEngine |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-mb-supply/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.mb.supply.v1
    ├── components.rs     # SupplyStock, SupplyLine, Depot, SupplyConsumer
    ├── systems.rs        # consume_supplies, update_supply_lines, check_depot_status, apply_supply_penalty
    └── events.rs         # SupplyDepleted, DepotDestroyed, SupplyLineCut, ResupplyReceived
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
- [ ] 4 composants dans `components.rs`
- [ ] 4 systemes dans `systems.rs`
- [ ] 4 evenements dans `events.rs`
- [ ] 3 enumerations (SupplyType, DepotState, SupplyLineState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : consumption, depot depletion, line cut, resupply, penalties
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.mb.supply.v1","k":"p","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.supply.v1.component.supply_stock","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.supply.v1.component.supply_line","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.supply.v1.component.depot","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.supply.v1.component.supply_consumer","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.supply.v1.fn.consume_supplies","k":"s","d":"massive-battle","r":["SupplyStock","SupplyConsumer"],"w":["SupplyStock"],"e":["SupplyDepleted"],"p":940,"c":"O(n)"},
  {"i":"mge.mb.supply.v1.fn.update_supply_lines","k":"s","d":"massive-battle","r":["SupplyLine","Depot"],"w":["SupplyLine"],"e":["SupplyLineCut"],"p":941,"c":"O(l)"},
  {"i":"mge.mb.supply.v1.fn.check_depot_status","k":"s","d":"massive-battle","r":["Depot"],"w":["Depot"],"e":["DepotDestroyed"],"p":942,"c":"O(d)"},
  {"i":"mge.mb.supply.v1.fn.apply_supply_penalty","k":"s","d":"massive-battle","r":["SupplyStock"],"w":["SupplyStock"],"e":["ResupplyReceived"],"p":943,"c":"O(n)"},
  {"i":"mge.mb.supply.v1.event.supply_depleted","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.supply.v1.event.depot_destroyed","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.supply.v1.event.supply_line_cut","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.supply.v1.event.resupply_received","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let depot = world.spawn();
world.insert(depot, Depot { stocks: [5000.0, 3000.0, 1000.0], max_capacity: 10000.0, state: DepotState::Active });

let squad = world.spawn();
world.insert(squad, SupplyStock { ammunition: 100.0, food: 50.0, medical: 20.0 });
world.insert(squad, SupplyConsumer { consumption_rate: [1.0, 0.1, 0.05], connected_depot: Some(depot) });

let line = world.spawn();
world.insert(line, SupplyLine { source: depot, target: squad, state: SupplyLineState::Connected, throughput: 50.0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Massive Battle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
