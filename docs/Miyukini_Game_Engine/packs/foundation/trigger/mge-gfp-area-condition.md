# mge-gfp-area-condition

> @id mge.foundation.area_condition.v1  
> @role plugin  
> @domain foundation  
> @do evaluate_conditional_activation_on_area  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-gfp-area-condition` |
| @id MSCM | `mge.foundation.area_condition.v1` |
| Domaine | foundation |
| Couche | Layer 1 (Foundation Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-gfp-trigger-zone`, `mge-gfp-enter-exit-event` |
| Hot path | Non |
| Headless safe | Oui |
| Complexite globale | O(t), t = zones avec conditions |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `ConditionType` | `AnyEntity`, `SpecificEntity`, `MinCount`, `TagMatch` | Type de condition a evaluer sur la zone trigger |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `AreaCondition` | `mge.foundation.area_condition.v1.component.area_condition` | `condition_type: ConditionType, required_count: u32, required_tag: Option<u32>` | Definition de la condition d'activation. `required_count` utilise par MinCount, `required_tag` utilise par TagMatch |
| `ConditionState` | `mge.foundation.area_condition.v1.component.condition_state` | `met: bool, last_change_tick: u32` | Etat courant de la condition : remplie ou non, et tick du dernier changement |

---

## 4. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `evaluate_area_conditions` | `mge.foundation.area_condition.v1.fn.evaluate_area_conditions` | 162 | AreaCondition, TriggerOccupants, TriggerEnter | ConditionState | AreaConditionMet, AreaConditionLost | O(t) | Evalue les conditions de chaque zone en fonction des occupants et des evenements TriggerEnter, emet AreaConditionMet ou AreaConditionLost lors d'un changement d'etat |

---

## 5. Flux de donnees

```
TriggerOccupants ──► AreaCondition
       │                    │
       ▼                    ▼
 TriggerEnter (events)      │
       │                    │
       ▼                    ▼
 ┌─────────────────────────────────────┐
 │     evaluate_area_conditions        │  Phase 162
 │  (check condition vs occupants)     │
 └────┬──────────────────────┬─────────┘
      │                      │
      ▼                      ▼
 ConditionState (maj)   AreaConditionMet /
                        AreaConditionLost (events)
```

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `AreaConditionMet` | `mge.foundation.area_condition.v1.event.area_condition_met` | `zone: EntityId` | `evaluate_area_conditions` | Scripts gameplay, systemes de portes/pieges |
| `AreaConditionLost` | `mge.foundation.area_condition.v1.event.area_condition_lost` | `zone: EntityId` | `evaluate_area_conditions` | Scripts gameplay, systemes de portes/pieges |

---

## 7. Invariants

- `ConditionState.met` ne change que si l'etat reel de la condition a change par rapport au tick precedent.
- Un evenement `AreaConditionMet` est emis une seule fois lors du passage de `met: false` a `met: true`.
- Un evenement `AreaConditionLost` est emis une seule fois lors du passage de `met: true` a `met: false`.
- `ConditionState.last_change_tick` est mis a jour uniquement lors d'un changement d'etat.
- `ConditionType::AnyEntity` est satisfaite des qu'au moins une entite est presente dans la zone.
- `ConditionType::MinCount` est satisfaite quand `TriggerOccupants.entities.len() >= required_count`.

---

## 8. Parametres GCL

Aucun.

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Evalue des conditions sur les zones trigger | Ne definit pas les zones trigger (→ trigger-zone) |
| Emet des evenements quand une condition est remplie ou perdue | Ne detecte pas les entrees/sorties (→ enter-exit-event) |
| Supporte 4 types de conditions | Ne gere pas les actions declenchees (responsabilite gameplay) |
| Maintient l'etat de chaque condition | Ne filtre pas par type d'entite au-dela du tag |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | AreaCondition, TriggerOccupants, TriggerEnter |
| Ecrit | ConditionState |
| Emet | AreaConditionMet, AreaConditionLost |
| Ne touche jamais | TriggerZone, Collider, WorldTransform, TriggerTracker |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-gfp-area-condition/
├── Cargo.toml
├── index.md
└── src/
    ├── lib.rs            # @id mge.foundation.area_condition.v1, trait Plugin impl
    ├── components.rs     # AreaCondition, ConditionState, ConditionType
    ├── systems.rs        # evaluate_area_conditions
    └── events.rs         # AreaConditionMet, AreaConditionLost
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | N/A |
| No allocation hot path | N/A |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin
- [ ] 1 enum `ConditionType` dans `components.rs`
- [ ] 2 composants dans `components.rs` avec @id et @fields
- [ ] 1 systeme dans `systems.rs` avec annotations completes
- [ ] 2 evenements dans `events.rs` avec @id et @fields
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : AnyEntity, SpecificEntity, MinCount, TagMatch, passage met→lost
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.foundation.area_condition.v1","k":"p","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.area_condition.v1.component.area_condition","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.area_condition.v1.component.condition_state","k":"d","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.area_condition.v1.event.area_condition_met","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.area_condition.v1.event.area_condition_lost","k":"e","d":"foundation","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.foundation.area_condition.v1.fn.evaluate_area_conditions","k":"s","d":"foundation","r":["AreaCondition","TriggerOccupants","TriggerEnter"],"w":["ConditionState"],"e":["AreaConditionMet","AreaConditionLost"],"p":162,"c":"O(t)"}
]
```

---

## 12. Exemple d'utilisation

```rust
let zone = world.spawn();
world.insert(zone, TriggerZone {
    shape: TriggerShape::AABB, width: 128.0, height: 128.0, radius: 0.0, enabled: true,
});
world.insert(zone, TriggerOccupants { entities: vec![] });
world.insert(zone, AreaCondition {
    condition_type: ConditionType::MinCount, required_count: 2, required_tag: None,
});
world.insert(zone, ConditionState { met: false, last_change_tick: 0 });
// Quand 2 entites entrent dans la zone :
// → ConditionState { met: true, last_change_tick: current_tick }
// → AreaConditionMet { zone } est emis
```

---

## References

| Document | Role |
|----------|------|
| [Pack GFP - Index](../_index.md) | Vue d'ensemble du pack |
| [mge-gfp-trigger-zone](mge-gfp-trigger-zone.md) | Plugin zones trigger (fournit TriggerOccupants) |
| [mge-gfp-enter-exit-event](mge-gfp-enter-exit-event.md) | Plugin evenements enter/exit (fournit TriggerEnter) |
