# mge-rts-unit-ai

> @id mge.rts.unit-ai.v1  
> @role plugin  
> @domain rts  
> @do manage_unit_orders_queue_group_pathfinding  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-rts-unit-ai` |
| @id MSCM | `mge.rts.unit-ai.v1` |
| Domaine | rts |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial`, `mge-rts-selection` |
| Hot path | Oui (ordres executes chaque tick pour les unites actives) |
| Headless safe | Oui |
| Complexite globale | O(u) ou u=unites avec ordres actifs |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `OrderKind` | `Move, Attack, Patrol, Hold, Follow, Gather, Build, Stop` | Type d'ordre donne a une unite |
| `OrderState` | `Pending, Executing, Completed, Failed` | Etat courant d'un ordre dans la queue |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `OrderQueue` | `mge.rts.unit-ai.v1.component.order_queue` | `orders: Vec<UnitOrder>, max_size: u8` | File d'ordres d'une unite. Le premier est l'ordre courant |
| `UnitOrder` | `mge.rts.unit-ai.v1.component.unit_order` | `kind: OrderKind, target: Option<EntityId>, destination: Option<(f32, f32)>, state: OrderState` | Ordre individuel avec cible et/ou destination |
| `GroupMovement` | `mge.rts.unit-ai.v1.component.group_movement` | `leader: EntityId, members: Vec<EntityId>, destination: (f32, f32), formation_spacing: f32` | Mouvement de groupe avec leader et espacement |

---

## 4. Formules

Non applicable. L'IA d'unite est procedurale (machine a etats ordres).

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `process_orders` | `mge.rts.unit-ai.v1.fn.process_orders` | Logic (1140) | OrderQueue | OrderQueue | OrderIssued | O(u) | Dequeue les ordres pending et les passe en executing |
| `execute_current_order` | `mge.rts.unit-ai.v1.fn.execute_current_order` | Logic (1141) | OrderQueue, UnitOrder, Position2D | UnitOrder | none | O(u) | Execute l'ordre courant (deplacement, attaque, etc.) |
| `update_group_pathfinding` | `mge.rts.unit-ai.v1.fn.update_group_pathfinding` | Logic (1142) | GroupMovement, Position2D | GroupMovement | GroupArrived | O(g * m) | Calcule le pathfinding de groupe avec formation |
| `check_order_completion` | `mge.rts.unit-ai.v1.fn.check_order_completion` | Logic (1143) | OrderQueue, UnitOrder | OrderQueue | OrderCompleted, OrderFailed | O(u) | Verifie si l'ordre courant est termine ou echoue |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique |
|-----------|----------|--------|----------|----------------------|
| `OrderIssued` | `mge.rts.unit-ai.v1.event.order_issued` | `entity: EntityId, kind: OrderKind, target: Option<EntityId>` | `process_orders` | ui, audio |
| `OrderCompleted` | `mge.rts.unit-ai.v1.event.order_completed` | `entity: EntityId, kind: OrderKind` | `check_order_completion` | ai, ui |
| `OrderFailed` | `mge.rts.unit-ai.v1.event.order_failed` | `entity: EntityId, kind: OrderKind, reason: String` | `check_order_completion` | ai, ui |
| `GroupArrived` | `mge.rts.unit-ai.v1.event.group_arrived` | `leader: EntityId, members: Vec<EntityId>, destination: (f32, f32)` | `update_group_pathfinding` | ai |

---

## 7. Invariants

- `OrderQueue.orders.len()` ne depasse jamais `max_size`.
- Un seul ordre est en state `Executing` a la fois (le premier de la queue).
- Un ordre `Completed` ou `Failed` est retire de la queue au tick suivant.
- `GroupMovement.members` ne contient que des entites vivantes.
- Un ordre `Stop` vide la queue sauf si `shift` est actif.
- Le pathfinding de groupe ne modifie pas les positions directement (ecriture via spatial).

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage | Description |
|-----------|------|--------|-------|-------------|
| `default_order_queue_size` | `u8` | 10 | [1, 20] | Taille par defaut de la file d'ordres |
| `arrival_threshold` | `f32` | 2.0 | [0.5, 10.0] | Distance seuil pour considerer un Move comme complete |
| `group_formation_spacing` | `f32` | 3.0 | [1.0, 10.0] | Espacement entre unites en mouvement de groupe |
| `patrol_wait_time` | `f32` | 1.0 | [0.0, 10.0] | Temps d'attente a chaque point de patrouille (secondes) |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les ordres et leur file d'attente | Ne gere pas la selection (→ selection) |
| Execute les ordres (move, attack, patrol) | Ne gere pas le combat (→ combat externe) |
| Calcule le pathfinding de groupe | Ne gere pas le pathfinding individuel (→ spatial) |
| Detecte la completion et l'echec | Ne gere pas le spawn/despawn des unites (→ production) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | OrderQueue, UnitOrder, GroupMovement, Position2D |
| Ecrit | OrderQueue, UnitOrder, GroupMovement |
| Emet | OrderIssued, OrderCompleted, OrderFailed, GroupArrived |
| Ne touche jamais | ProductionQueue, ResourceNode, Building, FogGrid, TechNode, SelectionBox |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-rts-unit-ai/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.rts.unit-ai.v1, trait Plugin impl
    ├── components.rs     # OrderQueue, UnitOrder, GroupMovement
    ├── systems.rs        # process_orders, execute_current_order, update_group_pathfinding, check_order_completion
    └── events.rs         # OrderIssued, OrderCompleted, OrderFailed, GroupArrived
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire |
| No dynamic dispatch hot path | Obligatoire (execute_current_order) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 3 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 2 enumerations (OrderKind, OrderState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : order queue, execution, group pathfinding, completion
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.rts.unit-ai.v1","k":"p","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.unit-ai.v1.component.order_queue","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.unit-ai.v1.component.unit_order","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.unit-ai.v1.component.group_movement","k":"d","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.unit-ai.v1.fn.process_orders","k":"s","d":"rts","r":["OrderQueue"],"w":["OrderQueue"],"e":["OrderIssued"],"p":1140,"c":"O(u)"},
  {"i":"mge.rts.unit-ai.v1.fn.execute_current_order","k":"s","d":"rts","r":["OrderQueue","UnitOrder","Position2D"],"w":["UnitOrder"],"e":[],"p":1141,"c":"O(u)"},
  {"i":"mge.rts.unit-ai.v1.fn.update_group_pathfinding","k":"s","d":"rts","r":["GroupMovement","Position2D"],"w":["GroupMovement"],"e":["GroupArrived"],"p":1142,"c":"O(g*m)"},
  {"i":"mge.rts.unit-ai.v1.fn.check_order_completion","k":"s","d":"rts","r":["OrderQueue","UnitOrder"],"w":["OrderQueue"],"e":["OrderCompleted","OrderFailed"],"p":1143,"c":"O(u)"},
  {"i":"mge.rts.unit-ai.v1.event.order_issued","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.unit-ai.v1.event.order_completed","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.unit-ai.v1.event.order_failed","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.rts.unit-ai.v1.event.group_arrived","k":"e","d":"rts","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let soldier = world.spawn();
world.insert(soldier, OrderQueue {
    orders: vec![
        UnitOrder {
            kind: OrderKind::Move,
            target: None,
            destination: Some((300.0, 200.0)),
            state: OrderState::Pending,
        },
        UnitOrder {
            kind: OrderKind::Attack,
            target: Some(enemy_id),
            destination: None,
            state: OrderState::Pending,
        },
    ],
    max_size: 10,
});

let group = world.spawn();
world.insert(group, GroupMovement {
    leader: soldier,
    members: vec![soldier, soldier2, soldier3],
    destination: (300.0, 200.0),
    formation_spacing: 3.0,
});
```

---

## References

| Document | Role |
|----------|------|
| [Pack RTS - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
