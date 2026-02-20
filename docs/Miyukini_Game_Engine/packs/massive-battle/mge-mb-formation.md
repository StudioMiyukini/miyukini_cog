# mge-mb-formation

> @id mge.mb.formation.v1  
> @role plugin  
> @domain massive-battle  
> @do manage_formation_shapes_ranks_columns_slots  

**Statut** : Specification normative v1  
**Version** : 1.0  
**Date** : 2026-02-20  

---

## 1. Identite

| Champ | Valeur |
|-------|--------|
| Crate | `mge-mb-formation` |
| @id MSCM | `mge.mb.formation.v1` |
| Domaine | massive-battle |
| Couche | Layer 2 (Genre Pack) |
| Dependances | `mge-ecs`, `mge-event`, `mge-plugin-spatial` |
| Hot path | Oui (positions recalculees chaque tick en mouvement) |
| Headless safe | Oui |
| Complexite globale | O(n * s) ou n=formations, s=slots par formation |

---

## 2. Enumerations

| Enum | Valeurs | Usage |
|------|---------|-------|
| `FormationShape` | `Line, Square, Wedge, Circle, Skirmish` | Forme geometrique de la formation |
| `FormationRank` | `Front, Middle, Rear` | Rang dans la formation |
| `SlotState` | `Occupied, Vacant, Reserved` | Etat d'un slot de formation |

---

## 3. Composants

| Composant | @id MSCM | Champs | Description |
|-----------|----------|--------|-------------|
| `Formation` | `mge.mb.formation.v1.component.formation` | `shape: FormationShape, spacing: f32, facing: f32, center: EntityId` | Configuration globale d'une formation. facing en radians |
| `FormationSlot` | `mge.mb.formation.v1.component.formation_slot` | `rank: FormationRank, column: u16, row: u16, occupant: Option<EntityId>, state: SlotState` | Slot individuel dans la grille de formation |
| `FormationOffset` | `mge.mb.formation.v1.component.formation_offset` | `local_x: f32, local_y: f32, world_x: f32, world_y: f32` | Position calculee du slot en coordonnees locales et monde |
| `FormationMembership` | `mge.mb.formation.v1.component.formation_membership` | `formation_id: EntityId, slot_index: u16` | Lie une entite a sa formation et son slot |

---

## 4. Formules de derivation

```
local_x     = column * spacing
local_y     = row * spacing
world_x     = formation_center_x + local_x * cos(facing) - local_y * sin(facing)
world_y     = formation_center_y + local_x * sin(facing) + local_y * cos(facing)
```

Pour Wedge : `local_x += row * wedge_offset` (decalage progressif par rang).  
Pour Circle : `local_x = radius * cos(2π * index / total)`, `local_y = radius * sin(2π * index / total)`.

---

## 5. Systemes

| Systeme | @id MSCM | Phase | @requires | @writes | @emits | Complexite | Description |
|---------|----------|-------|-----------|---------|--------|------------|-------------|
| `assign_formation_slots` | `mge.mb.formation.v1.fn.assign_formation_slots` | Logic (900) | Formation, FormationSlot | FormationSlot, FormationMembership | SlotAssigned | O(n * s) | Assigne les entites sans slot aux slots vacants. Front en priorite |
| `compute_formation_positions` | `mge.mb.formation.v1.fn.compute_formation_positions` | Logic (901) | Formation, FormationSlot | FormationOffset | none | O(n * s) | Recalcule les positions world de chaque slot selon shape et facing |
| `rotate_formation` | `mge.mb.formation.v1.fn.rotate_formation` | Logic (902) | Formation | Formation | FormationRotated | O(n) | Applique la rotation demandee au facing de la formation |
| `compact_formation` | `mge.mb.formation.v1.fn.compact_formation` | Logic (903) | Formation, FormationSlot | FormationSlot | FormationCompacted | O(n * s) | Comble les trous laisses par les morts en preservant l'ordre des rangs |

---

## 6. Evenements

| Evenement | @id MSCM | Champs | Emetteur | Consommateur typique | Description |
|-----------|----------|--------|----------|----------------------|-------------|
| `SlotAssigned` | `mge.mb.formation.v1.event.slot_assigned` | `target: EntityId, formation: EntityId, slot_index: u16` | `assign_formation_slots` | unit, ai | Entite assignee a un slot de formation |
| `FormationRotated` | `mge.mb.formation.v1.event.formation_rotated` | `formation: EntityId, new_facing: f32` | `rotate_formation` | unit, tactics | La formation a change d'orientation |
| `FormationCompacted` | `mge.mb.formation.v1.event.formation_compacted` | `formation: EntityId, vacancies_filled: u16` | `compact_formation` | unit | Trous combles dans la formation |
| `FormationBroken` | `mge.mb.formation.v1.event.formation_broken` | `formation: EntityId, reason: String` | externe (morale) | morale, tactics, ai | Formation dissoute par pertes ou panique |

---

## 7. Invariants

- Un slot avec `state = Occupied` a toujours un `occupant = Some(EntityId)` valide.
- Une entite ne peut appartenir qu'a une seule formation (`FormationMembership` unique).
- `FormationOffset` est coherent avec `Formation.facing` et `FormationSlot.rank/column` apres `compute_formation_positions`.
- Le nombre de slots `Occupied` ne depasse jamais `max_slots_per_formation`.
- `compact_formation` preserve l'ordre des rangs (Front avant Middle avant Rear).
- Un slot `Reserved` n'est jamais ecrase par `assign_formation_slots`.

---

## 8. Parametres GCL

| Parametre | Type | Defaut | Plage valide | Description |
|-----------|------|--------|--------------|-------------|
| `max_slots_per_formation` | `u32` | 64 | [4, 256] | Nombre max de slots par formation |
| `default_spacing` | `f32` | 2.0 | [0.5, 10.0] | Espacement par defaut entre slots en unites monde |
| `compact_threshold` | `f32` | 0.5 | [0.1, 0.9] | Ratio vacants/total declenchant compaction auto |
| `wedge_offset` | `f32` | 0.5 | [0.1, 2.0] | Decalage lateral par rang pour formation Wedge |

---

## 9. Bornage

### Bornes fonctionnelles

| Ce plugin fait | Ce plugin ne fait pas |
|----------------|----------------------|
| Gere les formes de formation et positions | Ne gere pas les ordres de groupe (→ unit) |
| Place les entites dans des slots | Ne deplace pas les entites (→ spatial) |
| Compacte les formations trouees | Ne decide pas de la dissolution (→ morale) |
| Calcule les offsets world des slots | Ne gere pas le pathfinding (→ spatial) |
| Supporte 5 formes geometriques | Ne gere pas les formations 3D (v2) |

### Bornes d'interface

| Direction | Elements |
|-----------|----------|
| Lit | Formation, FormationSlot |
| Ecrit | FormationSlot, FormationOffset, FormationMembership |
| Emet | SlotAssigned, FormationRotated, FormationCompacted |
| Ne touche jamais | Squad, Morale, TacticalStance, SupplyStock, WallSection |

---

## 10. Guide d'implementation

### Structure fichiers

```
mge-mb-formation/
├── Cargo.toml
├── index.md              # Resume max 80 lignes + AI-Native Score
└── src/
    ├── lib.rs            # @id mge.mb.formation.v1, trait Plugin impl
    ├── components.rs     # Formation, FormationSlot, FormationOffset, FormationMembership
    ├── systems.rs        # assign_formation_slots, compute_formation_positions, rotate_formation, compact_formation
    └── events.rs         # SlotAssigned, FormationRotated, FormationCompacted, FormationBroken
```

### Annotations MSCM requises

**lib.rs** :
```rust
//! @id mge.mb.formation.v1
//! @role plugin
//! @layer plugin
//! @domain massive-battle
//! @do manage_formation_shapes_ranks_columns_slots
```

**Chaque composant** dans components.rs :
```rust
//! @id mge.mb.formation.v1.component.{name}
//! @role data
//! @layer plugin
//! @do {description}
//! @fields {champ1}:{type1},{champ2}:{type2}
```

**Chaque systeme** dans systems.rs :
```rust
//! @id mge.mb.formation.v1.fn.{name}
//! @role system
//! @layer plugin
//! @do {description}
//! @requires {Comp1},{Comp2}
//! @writes {Comp1}
//! @emits {Event1} | none
//! @phase {N}
//! @complexity O(n)
```

**Chaque evenement** dans events.rs :
```rust
//! @id mge.mb.formation.v1.event.{name}
//! @role event
//! @layer plugin
//! @do {description}
//! @fields {champ1}:{type1},{champ2}:{type2}
```

### Bornage code

| Regle | Valeur |
|-------|--------|
| Max lignes par fonction | 30 (ideal), 40 (absolu) |
| Max lignes par fichier | 300 |
| 1 fn = 1 effet | Obligatoire |
| No hidden state | Obligatoire (pas de static mut, lazy_static, thread_local) |
| No dynamic dispatch hot path | Obligatoire (compute_formation_positions) |
| No allocation hot path | Obligatoire (pre-allouer Vec<FormationSlot>) |
| No unsafe | Obligatoire |

### Checklist implementation

- [ ] `lib.rs` avec @id root et impl Plugin (register components + systems)
- [ ] 4 composants dans `components.rs` avec @id et @fields
- [ ] 4 systemes dans `systems.rs` avec @id, @requires, @writes, @emits, @phase, @complexity
- [ ] 4 evenements dans `events.rs` avec @id et @fields
- [ ] 3 enumerations (FormationShape, FormationRank, SlotState)
- [ ] `index.md` genere (max 80 lignes)
- [ ] Tests unitaires : slot assignment, position compute, rotation, compaction
- [ ] AI-Native Score >= 8/10

---

## 11. Bloc MIP (blocks.json)

```json
[
  {"i":"mge.mb.formation.v1","k":"p","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.formation.v1.component.formation","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.formation.v1.component.formation_slot","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.formation.v1.component.formation_offset","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.formation.v1.component.formation_membership","k":"d","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.formation.v1.fn.assign_formation_slots","k":"s","d":"massive-battle","r":["Formation","FormationSlot"],"w":["FormationSlot","FormationMembership"],"e":["SlotAssigned"],"p":900,"c":"O(n*s)"},
  {"i":"mge.mb.formation.v1.fn.compute_formation_positions","k":"s","d":"massive-battle","r":["Formation","FormationSlot"],"w":["FormationOffset"],"e":[],"p":901,"c":"O(n*s)"},
  {"i":"mge.mb.formation.v1.fn.rotate_formation","k":"s","d":"massive-battle","r":["Formation"],"w":["Formation"],"e":["FormationRotated"],"p":902,"c":"O(n)"},
  {"i":"mge.mb.formation.v1.fn.compact_formation","k":"s","d":"massive-battle","r":["Formation","FormationSlot"],"w":["FormationSlot"],"e":["FormationCompacted"],"p":903,"c":"O(n*s)"},
  {"i":"mge.mb.formation.v1.event.slot_assigned","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.formation.v1.event.formation_rotated","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.formation.v1.event.formation_compacted","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null},
  {"i":"mge.mb.formation.v1.event.formation_broken","k":"e","d":"massive-battle","r":[],"w":[],"e":[],"p":null,"c":null}
]
```

---

## 12. Exemple d'utilisation

```rust
let formation = world.spawn();
world.insert(formation, Formation {
    shape: FormationShape::Square,
    spacing: 2.0,
    facing: 0.0,
    center: formation,
});

for row in 0..8 {
    for col in 0..8 {
        let slot = world.spawn();
        world.insert(slot, FormationSlot {
            rank: match row { 0..=1 => FormationRank::Front, 2..=5 => FormationRank::Middle, _ => FormationRank::Rear },
            column: col,
            row,
            occupant: None,
            state: SlotState::Vacant,
        });
        world.insert(slot, FormationOffset { local_x: 0.0, local_y: 0.0, world_x: 0.0, world_y: 0.0 });
    }
}
```

---

## References

| Document | Role |
|----------|------|
| [Pack Massive Battle - Index](_index.md) | Vue d'ensemble du pack |
| [MGE - AI-Native Writing Standard v1](../../mge-kernel/MGE%20-%20AI-Native%20Writing%20Standard%20v1.md) | Norme code |
| [MGE - Plugin Contract](../../MGE%20-%20Plugin%20Contract.md) | Trait Plugin |
