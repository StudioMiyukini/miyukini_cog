# mge-sb-need

> @id mge.sandbox.need.v1  
> @role plugin  
> @domain sandbox  
> @do manage_survival_needs_hunger_thirst_rest  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-sb-need` |
| @id MSCM | `mge.sandbox.need.v1` |
| Domaine | sandbox |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event` |
| Hot path | Oui (decay_needs chaque tick) |
| Headless safe | Oui |
| Complexite globale | O(n * k) n = entites, k = besoins par entite |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `NeedType` | `Hunger, Thirst, Rest, Comfort, Social` | Type de besoin |
| `NeedUrgency` | `Satisfied, Moderate, Urgent, Critical` | Niveau d'urgence du besoin |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Need` | `mge.sandbox.need.v1.component.need` | `need_type: NeedType, current: f32, max: f32, decay_rate: f32` | Besoin individuel. current diminue via decay_rate/tick |
| `NeedSet` | `mge.sandbox.need.v1.component.need_set` | `needs: Vec<EntityId>` | Ensemble des besoins d'une entite. Regroupe les Need |
| `NeedSatisfier` | `mge.sandbox.need.v1.component.need_satisfier` | `satisfies: NeedType, amount: f32, duration_ticks: u32` | Objet/action satisfaisant un besoin. Consomme sur application |

---

## 4. Formules

```
decay:
  current -= decay_rate * (1.0 + weather_modifier) per tick
  current = clamp(current, 0.0, max)

urgency:
  current > max * 0.7  → Satisfied
  current > max * 0.4  → Moderate
  current > max * 0.15 → Urgent
  current <= max * 0.15 → Critical
```

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `decay_needs` | `mge.sandbox.need.v1.fn.decay_needs` | 1520 | Need | Need | none | O(n*k) | Diminue current selon decay_rate pour chaque Need |
| `apply_satisfiers` | `mge.sandbox.need.v1.fn.apply_satisfiers` | 1521 | Need, NeedSatisfier | Need | NeedSatisfied | O(s) | Ajoute amount a current du Need correspondant. Consomme NeedSatisfier. s = satisfiers |
| `evaluate_urgency` | `mge.sandbox.need.v1.fn.evaluate_urgency` | 1522 | Need | Need | NeedUrgencyChanged | O(n*k) | Recalcule l'urgence selon les seuils. Emet si changement |
| `check_critical_needs` | `mge.sandbox.need.v1.fn.check_critical_needs` | 1523 | Need | Need | NeedCritical, NeedDepleted | O(n*k) | Si current <= 0 → NeedDepleted. Si Critical → NeedCritical |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `NeedUrgencyChanged` | `mge.sandbox.need.v1.event.need_urgency_changed` | `entity: EntityId, need_type: NeedType, old: NeedUrgency, new: NeedUrgency` | `evaluate_urgency` | agent (decision), ui |
| `NeedSatisfied` | `mge.sandbox.need.v1.event.need_satisfied` | `entity: EntityId, need_type: NeedType, amount: f32` | `apply_satisfiers` | agent (stop seeking), ui |
| `NeedCritical` | `mge.sandbox.need.v1.event.need_critical` | `entity: EntityId, need_type: NeedType, current: f32` | `check_critical_needs` | agent (emergency), ui (warning) |
| `NeedDepleted` | `mge.sandbox.need.v1.event.need_depleted` | `entity: EntityId, need_type: NeedType` | `check_critical_needs` | agent (death risk), health (damage) |

---

## 7. Invariants

- `Need.current` est toujours dans [0.0, Need.max].
- `Need.decay_rate` est toujours >= 0.0.
- Un NeedSatisfier est consomme dans le tick ou il est applique.
- Les seuils d'urgence sont fixes et ordonnes : Critical < Urgent < Moderate < Satisfied.
- `NeedDepleted` est emis une seule fois quand current atteint 0 (pas a chaque tick).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `hunger_decay_rate` | `f32` | 0.02 | [0.001, 0.1] | Decay faim/tick |
| `thirst_decay_rate` | `f32` | 0.03 | [0.001, 0.1] | Decay soif/tick |
| `rest_decay_rate` | `f32` | 0.015 | [0.001, 0.1] | Decay repos/tick |
| `critical_threshold_pct` | `f32` | 0.15 | [0.05, 0.3] | Seuil % pour niveau Critical |
| `need_max_default` | `f32` | 100.0 | [50.0, 500.0] | Valeur max par defaut |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Fait decroitre les besoins | Ne prend pas de decisions (→ agent) |
| Applique les satisfiers | Ne gere pas la nourriture comme item (→ inventory) |
| Evalue les niveaux d'urgence | Ne cause pas la mort (→ health/stats externe) |
| Emet les alertes critiques | Ne gere pas les routines (→ agent) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Need, NeedSet, NeedSatisfier |
| Ecrit | Need |
| Emet | NeedUrgencyChanged, NeedSatisfied, NeedCritical, NeedDepleted |
| Ne touche jamais | Agent, Building, CraftingStation, TerrainTile, Weather, Wildlife |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-sb-need/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.sandbox.need.v1
    ├── components.rs     # Need, NeedSet, NeedSatisfier
    ├── systems.rs        # decay_needs, apply_satisfiers, evaluate_urgency, check_critical_needs
    └── events.rs         # NeedUrgencyChanged, NeedSatisfied, NeedCritical, NeedDepleted
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (decay_needs) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec annotations completes
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (NeedType, NeedUrgency)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : decay, satisfy, urgency transitions, critical, depletion
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.sandbox.need.v1","k":"p","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.need.v1.component.need","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.need.v1.component.need_set","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.need.v1.component.need_satisfier","k":"d","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.need.v1.fn.decay_needs","k":"s","d":"sandbox","r":["Need"],"w":["Need"],"e":[],"p":1520,"c":"O(n*k)"},
  {"i":"mge.sandbox.need.v1.fn.apply_satisfiers","k":"s","d":"sandbox","r":["Need","NeedSatisfier"],"w":["Need"],"e":["NeedSatisfied"],"p":1521,"c":"O(s)"},
  {"i":"mge.sandbox.need.v1.fn.evaluate_urgency","k":"s","d":"sandbox","r":["Need"],"w":["Need"],"e":["NeedUrgencyChanged"],"p":1522,"c":"O(n*k)"},
  {"i":"mge.sandbox.need.v1.fn.check_critical_needs","k":"s","d":"sandbox","r":["Need"],"w":["Need"],"e":["NeedCritical","NeedDepleted"],"p":1523,"c":"O(n*k)"},
  {"i":"mge.sandbox.need.v1.event.need_urgency_changed","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.need.v1.event.need_satisfied","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.need.v1.event.need_critical","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.sandbox.need.v1.event.need_depleted","k":"e","d":"sandbox","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let hunger = world.spawn();
world.insert(hunger, Need { need_type: NeedType::Hunger, current: 100.0, max: 100.0, decay_rate: 0.02 });

let thirst = world.spawn();
world.insert(thirst, Need { need_type: NeedType::Thirst, current: 100.0, max: 100.0, decay_rate: 0.03 });

let agent = world.spawn();
world.insert(agent, NeedSet { needs: vec![hunger, thirst] });
```

---

## References

| Document | Role |
|----------|------|
| [Pack Sandbox - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
