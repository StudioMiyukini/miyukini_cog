# mge-tycoon-facility

> @id mge.tycoon.facility.v1  
> @role plugin  
> @domain tycoon  
> @do manage_facilities_capacity_levels_and_maintenance  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-tycoon-facility` |
| @id MSCM | `mge.tycoon.facility.v1` |
| Domaine | tycoon |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-tycoon-revenue` |
| Hot path | Oui (tick maintenance chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n) sur entites Facility |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `FacilityType` | `Shop, Restaurant, Factory, Office, Warehouse, Entertainment, Custom(u32)` | Type d'installation |
| `FacilityCondition` | `Excellent, Good, Fair, Poor, Broken` | Etat de degradation |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Facility` | `mge.tycoon.facility.v1.component.facility` | `facility_type: FacilityType, name_key: u32, active: bool` | Definition de l'installation. active = ouverte au public |
| `FacilityDef` | `mge.tycoon.facility.v1.component.facility_def` | `base_capacity: u32, base_maintenance_cost: f64, upgrade_cost_base: f64, upgrade_cost_scaling: f64` | Donnees statiques chargees depuis GCL |
| `FacilityLevel` | `mge.tycoon.facility.v1.component.facility_level` | `level: u32, max_level: u32` | Niveau courant et maximum de l'installation |
| `Capacity` | `mge.tycoon.facility.v1.component.capacity` | `current: u32, max: u32, occupied: u32` | Capacite d'accueil. occupied = clients en cours de service |
| `MaintenanceCost` | `mge.tycoon.facility.v1.component.maintenance_cost` | `cost_per_tick: f64, accumulated_debt: f64` | Cout de maintenance par tick et dette accumulee si non payee |
| `FacilityState` | `mge.tycoon.facility.v1.component.facility_state` | `condition: FacilityCondition, degradation: f64, last_repair_tick: u64` | Etat de degradation. 0.0 = neuf, 1.0 = casse |

---

## 4. Formules

```
maintenance_cost_per_tick = base_maintenance_cost * level * (1.0 + degradation * 0.5)
capacity_max = base_capacity + (level - 1) * capacity_per_level
upgrade_cost = upgrade_cost_base * upgrade_cost_scaling ^ level
degradation += degradation_rate_per_tick (si pas repare)
```

La degradation augmente passivement. Un batiment `Broken` ne sert plus de clients.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `tick_maintenance` | `mge.tycoon.facility.v1.fn.tick_maintenance` | 2200 | Facility, MaintenanceCost, FacilityState | MaintenanceCost | none | O(n) | Accumule le cout de maintenance. Ajoute a accumulated_debt si budget insuffisant |
| `apply_degradation` | `mge.tycoon.facility.v1.fn.apply_degradation` | 2201 | Facility, FacilityState | FacilityState | FacilityDegraded, FacilityBroken | O(n) | Augmente degradation. Transition Excellent→Good→Fair→Poor→Broken. Emet event au changement |
| `process_facility_upgrade` | `mge.tycoon.facility.v1.fn.process_facility_upgrade` | 2202 | FacilityLevel, FacilityDef, Budget | FacilityLevel, Budget, Capacity | FacilityUpgraded | O(k) | Traite les UpgradeFacilityRequest. Verifie budget et max_level |
| `update_capacity` | `mge.tycoon.facility.v1.fn.update_capacity` | 2203 | FacilityLevel, FacilityDef, FacilityState | Capacity | none | O(n) | Recalcule capacity_max selon le niveau. Broken → max = 0 |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `UpgradeFacilityRequest` | `mge.tycoon.facility.v1.event.upgrade_facility_request` | `facility: EntityId` | Externe (UI, input) | `process_facility_upgrade` |
| `FacilityUpgraded` | `mge.tycoon.facility.v1.event.facility_upgraded` | `facility: EntityId, new_level: u32, cost: f64` | `process_facility_upgrade` | UI, analytics |
| `FacilityDegraded` | `mge.tycoon.facility.v1.event.facility_degraded` | `facility: EntityId, new_condition: FacilityCondition` | `apply_degradation` | UI (alerte), employee (reaffectation) |
| `FacilityBroken` | `mge.tycoon.facility.v1.event.facility_broken` | `facility: EntityId` | `apply_degradation` | UI (alerte critique), customer (redirection) |

---

## 7. Invariants

- `Capacity.occupied` ne depasse jamais `Capacity.max`.
- `FacilityLevel.level` ne depasse jamais `FacilityLevel.max_level`.
- `FacilityState.degradation` est dans [0.0, 1.0]. 1.0 = Broken.
- Un Facility avec `condition = Broken` a toujours `Capacity.max = 0`.
- `MaintenanceCost.cost_per_tick` est toujours >= 0.0.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `degradation_rate_per_tick` | `f64` | 0.0001 | [0.0, 0.01] | Taux de degradation par tick |
| `repair_cost_factor` | `f64` | 0.5 | [0.1, 5.0] | Multiplicateur du cout de reparation (base * factor * degradation) |
| `capacity_per_level` | `u32` | 5 | [1, 100] | Capacite supplementaire par niveau |
| `max_facility_level` | `u32` | 10 | [1, 50] | Niveau max par defaut |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les installations et leur capacite | Ne gere pas les clients (→ customer) |
| Applique la degradation passive | Ne gere pas les employes (→ employee) |
| Traite les upgrades d'installation | Ne calcule pas les revenus (→ revenue) |
| Met a jour la capacite selon le niveau | Ne gere pas le placement spatial (→ Core spatial) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Facility, FacilityDef, FacilityLevel, FacilityState, MaintenanceCost, Budget |
| Ecrit | FacilityState, FacilityLevel, Capacity, MaintenanceCost, Budget |
| Emet | FacilityUpgraded, FacilityDegraded, FacilityBroken |
| Ne touche jamais | Employee, Customer, Satisfaction, Transaction |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-tycoon-facility/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.tycoon.facility.v1, trait Plugin impl
    ├── components.rs     # Facility, FacilityDef, FacilityLevel, Capacity, MaintenanceCost, FacilityState
    ├── systems.rs        # tick_maintenance, apply_degradation, process_facility_upgrade, update_capacity
    └── events.rs         # UpgradeFacilityRequest, FacilityUpgraded, FacilityDegraded, FacilityBroken
```

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 6 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs`
- [ ] 4 evenements dans `events.rs`
- [ ] 2 enumerations (FacilityType, FacilityCondition)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : degradation, upgrade, capacite, maintenance
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.tycoon.facility.v1","k":"p","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.component.facility","k":"d","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.component.facility_def","k":"d","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.component.facility_level","k":"d","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.component.capacity","k":"d","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.component.maintenance_cost","k":"d","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.component.facility_state","k":"d","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.fn.tick_maintenance","k":"s","d":"tycoon","r":["Facility","MaintenanceCost","FacilityState"],"w":["MaintenanceCost"],"e":[],"p":2200,"c":"O(n)"},
  {"i":"mge.tycoon.facility.v1.fn.apply_degradation","k":"s","d":"tycoon","r":["Facility","FacilityState"],"w":["FacilityState"],"e":["FacilityDegraded","FacilityBroken"],"p":2201,"c":"O(n)"},
  {"i":"mge.tycoon.facility.v1.fn.process_facility_upgrade","k":"s","d":"tycoon","r":["FacilityLevel","FacilityDef","Budget"],"w":["FacilityLevel","Budget","Capacity"],"e":["FacilityUpgraded"],"p":2202,"c":"O(k)"},
  {"i":"mge.tycoon.facility.v1.fn.update_capacity","k":"s","d":"tycoon","r":["FacilityLevel","FacilityDef","FacilityState"],"w":["Capacity"],"e":[],"p":2203,"c":"O(n)"},
  {"i":"mge.tycoon.facility.v1.event.upgrade_facility_request","k":"e","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.event.facility_upgraded","k":"e","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.event.facility_degraded","k":"e","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.tycoon.facility.v1.event.facility_broken","k":"e","d":"tycoon","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let shop = world.spawn();
world.insert(shop, Facility { facility_type: FacilityType::Shop, name_key: 1, active: true });
world.insert(shop, FacilityDef { base_capacity: 10, base_maintenance_cost: 5.0, upgrade_cost_base: 500.0, upgrade_cost_scaling: 1.5 });
world.insert(shop, FacilityLevel { level: 1, max_level: 10 });
world.insert(shop, Capacity { current: 10, max: 10, occupied: 0 });
world.insert(shop, MaintenanceCost { cost_per_tick: 5.0, accumulated_debt: 0.0 });
world.insert(shop, FacilityState { condition: FacilityCondition::Excellent, degradation: 0.0, last_repair_tick: 0 });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Tycoon - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
